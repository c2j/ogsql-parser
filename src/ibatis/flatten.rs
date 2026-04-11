//! SqlNode 树 → 扁平 SQL 字符串。
//!
//! 遍历 SqlNode 树，将动态 SQL 元素转换为具体 SQL 文本。
//! - #{param} → $1
//! - ${expr} → __IBATIS_DOLLAR_expr__

use crate::ibatis::types::SqlNode;

const DOLLAR_PREFIX: &str = "__IBATIS_DOLLAR_";
const DOLLAR_SUFFIX: &str = "__";

/// 将 SqlNode 树扁平化为 SQL 字符串。
///
/// Phase 1: 只处理 Text 节点。动态元素在 Task 6 后实现。
pub fn flatten_sql(node: &SqlNode) -> String {
    match node {
        SqlNode::Text { content } => replace_params(content),
        SqlNode::Parameter { .. } => "$1".to_string(),
        SqlNode::RawExpr { expr } => format!("{}{}{}", DOLLAR_PREFIX, expr, DOLLAR_SUFFIX),
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
    }
}

fn flatten_children(children: &[SqlNode]) -> String {
    children.iter().map(|c| flatten_sql(c)).collect()
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
        for ov in &ov_list {
            let trimmed = result.trim_start();
            if trimmed.len() >= ov.len() && trimmed[..ov.len()].eq_ignore_ascii_case(ov) {
                result = trimmed[ov.len()..].to_string();
            }
        }
    }

    if let Some(overrides) = suffix_overrides {
        let ov_list: Vec<&str> = overrides
            .split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        for ov in &ov_list {
            if result.len() >= ov.len()
                && result[result.len() - ov.len()..].eq_ignore_ascii_case(ov)
            {
                result.truncate(result.len() - ov.len());
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
/// - `#{param}` → `?` (含类型注解时截断: `#{name,javaType=...}` → `?`)
/// - `${expr}` → `__IBATIS_DOLLAR_expr__`
/// - `\${...}` → `${...}` (反斜杠转义)
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
                result.push_str("$1");
                i = end + 1;
                continue;
            }
        }

        // 处理 ${
        if c == '$' && i + 1 < len && chars[i + 1] == '{' {
            if let Some(end) = find_closing_brace(&chars, i + 2) {
                let expr: String = chars[i + 2..end].iter().collect();
                result.push_str(DOLLAR_PREFIX);
                result.push_str(&expr);
                result.push_str(DOLLAR_SUFFIX);
                i = end + 1;
                continue;
            }
        }

        result.push(c);
        i += 1;
    }

    result
}

/// 从位置 start 开始查找匹配的 `}`，考虑嵌套。
fn find_closing_brace(chars: &[char], start: usize) -> Option<usize> {
    let mut depth = 1;
    let mut i = start;
    while i < chars.len() {
        match chars[i] {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod param_tests {
    use super::*;

    #[test]
    fn test_hash_param() {
        assert_eq!(replace_params("WHERE id = #{id}"), "WHERE id = $1");
    }

    #[test]
    fn test_dollar_param() {
        assert_eq!(
            replace_params("ORDER BY ${col}"),
            format!("ORDER BY {}col{}", DOLLAR_PREFIX, DOLLAR_SUFFIX)
        );
    }

    #[test]
    fn test_mixed_params() {
        assert_eq!(
            replace_params("WHERE id = #{id} AND name = #{name}"),
            "WHERE id = $1 AND name = $1"
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
            "$1"
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
        assert!(result.contains("__IBATIS_DOLLAR_table__"));
        assert_eq!(result.matches("$1").count(), 2);
    }
}
