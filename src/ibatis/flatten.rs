//! SqlNode 树 → 扁平 SQL 字符串。
//!
//! 遍历 SqlNode 树，将动态 SQL 元素转换为具体 SQL 文本。
//! - #{param} → __XML_PARAM_param__
//! - #{param,javaType=double} → __XML_PARAM_DOUBLE_param__
//! - ${expr} → __XML_RAW_expr__

use crate::ibatis::types::SqlNode;
use crate::ibatis::util::{find_closing_brace, parse_param_type};

const PARAM_PREFIX: &str = "__XML_PARAM_";
const RAW_PREFIX: &str = "__XML_RAW_";
const PLACEHOLDER_SUFFIX: &str = "__";

/// 将 SqlNode 树扁平化为 SQL 字符串。
///
/// Phase 1: 只处理 Text 节点。动态元素在 Task 6 后实现。
pub fn flatten_sql(node: &SqlNode) -> String {
    match node {
        SqlNode::Text { content } => replace_params(content),
        SqlNode::Parameter { name, java_type } => match java_type {
            Some(t) => format!(
                "{}{}_{}{}",
                PARAM_PREFIX,
                t.to_uppercase(),
                name,
                PLACEHOLDER_SUFFIX
            ),
            None => format!("{}{}{}", PARAM_PREFIX, name, PLACEHOLDER_SUFFIX),
        },
        SqlNode::RawExpr { expr } => format!("{}{}{}", RAW_PREFIX, expr, PLACEHOLDER_SUFFIX),
        // 动态元素: "最完整"策略，取所有内容
        SqlNode::If { children, .. } => flatten_children(children),
        SqlNode::Choose { branches } => {
            // 取第一个分支
            if let Some((_, branch)) = branches.first() {
                flatten_children(branch)
            } else {
                String::new()
            }
        }
        SqlNode::Where { children } => {
            let content = flatten_children(children);
            apply_trim(&content, Some("WHERE"), None, Some("AND |OR "), None)
        }
        SqlNode::Set { children } => {
            let content = flatten_children(children);
            apply_trim(&content, Some("SET"), None, None, Some(","))
        }
        SqlNode::Trim {
            prefix,
            suffix,
            prefix_overrides,
            suffix_overrides,
            children,
        } => {
            let content = flatten_children(children);
            apply_trim(
                &content,
                prefix.as_deref(),
                suffix.as_deref(),
                prefix_overrides.as_deref(),
                suffix_overrides.as_deref(),
            )
        }
        SqlNode::ForEach {
            open,
            separator: _,
            close,
            children,
            ..
        } => {
            let content = flatten_children(children);
            let open_str = open.as_deref().unwrap_or("");
            let close_str = close.as_deref().unwrap_or("");
            format!("{}{}{}", open_str, content, close_str)
        }
        SqlNode::Bind { .. } => String::new(),
        SqlNode::Sequence { children } => flatten_children(children),
        SqlNode::Include { refid } => {
            format!("/* UNRESOLVED_INCLUDE({}) */", refid)
        }
    }
}

fn flatten_children(children: &[SqlNode]) -> String {
    children.iter().map(|c| flatten_sql(c)).collect()
}

/// 从 SqlNode 树中收集所有 Parameter 节点。
pub fn collect_params(node: &SqlNode) -> Vec<(String, Option<String>, String)> {
    let mut params = Vec::new();
    collect_params_recursive(node, &mut params);
    params
}

fn collect_params_recursive(
    node: &SqlNode,
    params: &mut Vec<(String, Option<String>, String)>,
) {
    match node {
        SqlNode::Parameter { name, java_type } => {
            let raw = match java_type {
                Some(t) => format!("#{{{},{}}}", name, format!("javaType={}", t)),
                None => format!("#{{{}}}", name),
            };
            params.push((name.clone(), java_type.clone(), raw));
        }
        SqlNode::If { children, .. }
        | SqlNode::Where { children, .. }
        | SqlNode::Set { children, .. }
        | SqlNode::Trim { children, .. }
        | SqlNode::ForEach { children, .. }
        | SqlNode::Sequence { children } => {
            for c in children { collect_params_recursive(c, params); }
        }
        SqlNode::Choose { branches } => {
            for (_, ch) in branches {
                for c in ch { collect_params_recursive(c, params); }
            }
        }
        SqlNode::Text { .. } | SqlNode::RawExpr { .. } | SqlNode::Bind { .. } | SqlNode::Include { .. } => {}
    }
}

/// 应用 trim 逻辑：添加前缀/后缀，移除前缀/后缀覆盖。
fn apply_trim(
    content: &str,
    prefix: Option<&str>,
    suffix: Option<&str>,
    prefix_overrides: Option<&str>,
    suffix_overrides: Option<&str>,
) -> String {
    let mut result = content.trim().to_string();

    if let Some(overrides) = prefix_overrides {
        let ov_list: Vec<&str> = overrides
            .split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        loop {
            let trimmed = result.trim_start();
            let mut stripped = false;
            for ov in &ov_list {
                if trimmed.len() >= ov.len() && trimmed[..ov.len()].eq_ignore_ascii_case(ov) {
                    result = trimmed[ov.len()..].to_string();
                    stripped = true;
                    break;
                }
            }
            if !stripped {
                break;
            }
        }
    }

    if let Some(overrides) = suffix_overrides {
        let ov_list: Vec<&str> = overrides
            .split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        loop {
            let trimmed = result.trim_end();
            let mut stripped = false;
            for ov in &ov_list {
                if trimmed.len() >= ov.len()
                    && trimmed[trimmed.len() - ov.len()..].eq_ignore_ascii_case(ov)
                {
                    result = trimmed[..trimmed.len() - ov.len()].to_string();
                    stripped = true;
                    break;
                }
            }
            if !stripped {
                break;
            }
        }
    }

    result = result.trim().to_string();

    if result.is_empty() {
        return String::new();
    }

    let mut final_result = String::new();
    if let Some(p) = prefix {
        final_result.push_str(p);
        final_result.push(' ');
    }
    final_result.push_str(&result);
    if let Some(s) = suffix {
        final_result.push(' ');
        final_result.push_str(s);
    }

    final_result
}

/// 替换 SQL 文本中的参数占位符。
///
/// - `#{param}` → `__XML_PARAM_param__`
/// - `#{name,javaType=double}` → `__XML_PARAM_DOUBLE_name__`
/// - `${expr}` → `__XML_RAW_expr__`
/// - `'...'` 内的 `#{}` 和 `${}` 不替换
fn replace_params(sql: &str) -> String {
    let mut result = String::with_capacity(sql.len());
    let chars: Vec<char> = sql.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut in_string = false;

    while i < len {
        let c = chars[i];

        // 跟踪 SQL 字符串字面量
        if c == '\'' && !in_string {
            in_string = true;
            result.push(c);
            i += 1;
            continue;
        }
        if c == '\'' && in_string {
            if i + 1 < len && chars[i + 1] == '\'' {
                result.push_str("''");
                i += 2;
                continue;
            }
            in_string = false;
            result.push(c);
            i += 1;
            continue;
        }

        if in_string {
            result.push(c);
            i += 1;
            continue;
        }

        // 处理 #{
        if c == '#' && i + 1 < len && chars[i + 1] == '{' {
            if let Some(end) = find_closing_brace(&chars, i + 2) {
                let param: String = chars[i + 2..end].iter().collect();
                let (name, java_type) = parse_param_type(&param);
                match java_type {
                    Some(t) => {
                        result.push_str(PARAM_PREFIX);
                        result.push_str(&t.to_uppercase());
                        result.push('_');
                        result.push_str(&name);
                        result.push_str(PLACEHOLDER_SUFFIX);
                    }
                    None => {
                        result.push_str(PARAM_PREFIX);
                        result.push_str(&name);
                        result.push_str(PLACEHOLDER_SUFFIX);
                    }
                }
                i = end + 1;
                continue;
            }
        }

        // 处理 ${
        if c == '$' && i + 1 < len && chars[i + 1] == '{' {
            if let Some(end) = find_closing_brace(&chars, i + 2) {
                let expr: String = chars[i + 2..end].iter().collect();
                result.push_str(RAW_PREFIX);
                result.push_str(&expr);
                result.push_str(PLACEHOLDER_SUFFIX);
                i = end + 1;
                continue;
            }
        }

        // 处理 iBatis 2.x #param# 格式
        if c == '#' && (i + 1 >= len || chars[i + 1] != '{') {
            let start = i + 1;
            let mut end = start;
            while end < len && chars[end] != '#' {
                end += 1;
            }
            if end < len && end > start {
                let param: String = chars[start..end].iter().collect();
                if !param.contains(' ') && !param.contains('\n') && !param.contains('\r') {
                    result.push_str(PARAM_PREFIX);
                    result.push_str(&param);
                    result.push_str(PLACEHOLDER_SUFFIX);
                    i = end + 1;
                    continue;
                }
            }
        }

        // 处理 iBatis 2.x $param$ 格式
        if c == '$' && (i + 1 >= len || chars[i + 1] != '{') {
            let start = i + 1;
            let mut end = start;
            while end < len && chars[end] != '$' {
                end += 1;
            }
            if end < len && end > start {
                let param: String = chars[start..end].iter().collect();
                if !param.contains(' ') && !param.contains('\n') && !param.contains('\r') {
                    result.push_str(RAW_PREFIX);
                    result.push_str(&param);
                    result.push_str(PLACEHOLDER_SUFFIX);
                    i = end + 1;
                    continue;
                }
            }
        }

        result.push(c);
        i += 1;
    }

    result
}

#[cfg(test)]
mod param_tests {
    use super::*;

    #[test]
    fn test_hash_param() {
        assert_eq!(
            replace_params("WHERE id = #{id}"),
            "WHERE id = __XML_PARAM_id__"
        );
    }

    #[test]
    fn test_dollar_param() {
        assert_eq!(
            replace_params("ORDER BY ${col}"),
            "ORDER BY __XML_RAW_col__"
        );
    }

    #[test]
    fn test_mixed_params() {
        assert_eq!(
            replace_params("WHERE id = #{id} AND name = #{name}"),
            "WHERE id = __XML_PARAM_id__ AND name = __XML_PARAM_name__"
        );
    }

    #[test]
    fn test_param_in_string_not_replaced() {
        assert_eq!(
            replace_params("WHERE name = '#{not_a_param}'"),
            "WHERE name = '#{not_a_param}'"
        );
    }

    #[test]
    fn test_dollar_in_string_not_replaced() {
        assert_eq!(
            replace_params("WHERE name = '${not_a_param}'"),
            "WHERE name = '${not_a_param}'"
        );
    }

    #[test]
    fn test_param_with_type_annotation() {
        assert_eq!(
            replace_params("#{price,javaType=double,jdbcType=NUMERIC}"),
            "__XML_PARAM_DOUBLE_price__"
        );
    }

    #[test]
    fn test_no_params() {
        assert_eq!(replace_params("SELECT 1"), "SELECT 1");
    }

    #[test]
    fn test_multiple_hash_and_dollar() {
        let sql = "SELECT * FROM ${table} WHERE id = #{id} AND name LIKE #{name}";
        let result = replace_params(sql);
        assert!(result.contains("__XML_RAW_table__"));
        assert!(result.contains("__XML_PARAM_id__"));
        assert!(result.contains("__XML_PARAM_name__"));
    }

    #[test]
    fn test_param_with_jdbc_type_only() {
        assert_eq!(
            replace_params("#{name,jdbcType=VARCHAR}"),
            "__XML_PARAM_VARCHAR_name__"
        );
    }

    #[test]
    fn test_param_no_type() {
        assert_eq!(replace_params("#{simple}"), "__XML_PARAM_simple__");
    }
}
