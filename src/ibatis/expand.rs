//! 动态 SQL 变体展开器。
//!
//! 从 `SqlNode` 树枚举所有可能的 SQL 变体，内部处理
//! `<where>`/`<set>`/`<trim>` 的运行时语义（前缀/后缀裁剪）。

use crate::ibatis::flatten::{self, sanitize_param_name};
use crate::ibatis::types::{
    BranchStep, ExpandConfig, ExpandedVariant, IfExpandStrategy, ParamMeta, PlaceholderStrategy,
    SqlNode, StructuredStatement,
};

const PARAM_PREFIX: &str = "__XML_PARAM_";
const RAW_PREFIX: &str = "__XML_RAW_";
const PLACEHOLDER_SUFFIX: &str = "__";

#[derive(Debug, Clone)]
struct ExpansionState {
    sql_buffer: String,
    branch_path: Vec<BranchStep>,
    params: Vec<ParamMeta>,
}

/// 展开 `StructuredStatement` 的所有 SQL 变体。
pub fn expand_variants(stmt: &StructuredStatement, config: &ExpandConfig) -> Vec<ExpandedVariant> {
    let initial = ExpansionState {
        sql_buffer: String::new(),
        branch_path: Vec::new(),
        params: Vec::new(),
    };
    let mut variants = vec![initial];
    expand_node(&stmt.body, &mut variants, 0, config);

    let mut results: Vec<ExpandedVariant> = variants
        .into_iter()
        .map(|state| {
            let parse_result = if config.generate_parse_results && !state.sql_buffer.trim().is_empty()
            {
                Some(crate::parser::Parser::parse_sql(&state.sql_buffer))
            } else {
                None
            };
            ExpandedVariant {
                sql: state.sql_buffer,
                branch_path: state.branch_path,
                parameters: state.params,
                parse_result,
            }
        })
        .collect();

    results.truncate(config.max_variants);
    results
}

fn expand_node(
    node: &SqlNode,
    variants: &mut Vec<ExpansionState>,
    depth: usize,
    config: &ExpandConfig,
) {
    if depth > config.max_depth || variants.len() >= config.max_variants {
        return;
    }
    match node {
        SqlNode::Text { content } => {
            let rendered = render_text_params(content, config.placeholder);
            for v in variants.iter_mut() {
                v.sql_buffer.push_str(&rendered);
            }
        }
        SqlNode::Parameter { name, java_type } => {
            for v in variants.iter_mut() {
                v.sql_buffer.push_str(&render_parameter(name, java_type, config.placeholder));
                if !v.params.iter().any(|p| p.name == *name) {
                    v.params.push(ParamMeta {
                        name: name.clone(),
                        jdbc_type: None,
                        source: None,
                        position: 0,
                        raw: format!("#{{{}}}", name),
                    });
                }
            }
        }
        SqlNode::RawExpr { expr, java_type } => {
            for v in variants.iter_mut() {
                v.sql_buffer.push_str(&render_raw(expr, java_type, config.placeholder));
                if !v.params.iter().any(|p| p.name == *expr) {
                    v.params.push(ParamMeta {
                        name: expr.clone(),
                        jdbc_type: None,
                        source: None,
                        position: 0,
                        raw: format!("${{{}}}", expr),
                    });
                }
            }
        }
        SqlNode::Sequence { children } => {
            for child in children {
                expand_node(child, variants, depth, config);
            }
        }
        SqlNode::If { test, prepend, children } => {
            expand_if(test, prepend.as_deref(), children, variants, depth, config);
        }
        SqlNode::Choose { branches } => {
            expand_choose(branches, variants, depth, config);
        }
        SqlNode::Where { children } => {
            let mut sub_variants = variants
                .drain(..)
                .map(|v| ExpansionState {
                    sql_buffer: String::new(),
                    branch_path: v.branch_path,
                    params: v.params,
                })
                .collect::<Vec<_>>();
            for child in children {
                expand_node(child, &mut sub_variants, depth + 1, config);
            }
            let prefix_overrides = Some("AND |OR ");
            for sub in sub_variants {
                let trimmed = apply_trim(&sub.sql_buffer, Some("WHERE"), None, prefix_overrides, None);
                variants.push(ExpansionState {
                    sql_buffer: trimmed,
                    branch_path: sub.branch_path,
                    params: sub.params,
                });
            }
        }
        SqlNode::Set { children } => {
            let mut sub_variants = variants
                .drain(..)
                .map(|v| ExpansionState {
                    sql_buffer: String::new(),
                    branch_path: v.branch_path,
                    params: v.params,
                })
                .collect::<Vec<_>>();
            for child in children {
                expand_node(child, &mut sub_variants, depth + 1, config);
            }
            for sub in sub_variants {
                let trimmed = apply_trim(&sub.sql_buffer, Some("SET"), None, None, Some(","));
                variants.push(ExpansionState {
                    sql_buffer: trimmed,
                    branch_path: sub.branch_path,
                    params: sub.params,
                });
            }
        }
        SqlNode::Trim {
            prefix,
            suffix,
            prefix_overrides,
            suffix_overrides,
            children,
        } => {
            let mut sub_variants = variants
                .drain(..)
                .map(|v| ExpansionState {
                    sql_buffer: String::new(),
                    branch_path: v.branch_path,
                    params: v.params,
                })
                .collect::<Vec<_>>();
            for child in children {
                expand_node(child, &mut sub_variants, depth + 1, config);
            }
            for sub in sub_variants {
                let trimmed = apply_trim(
                    &sub.sql_buffer,
                    prefix.as_deref(),
                    suffix.as_deref(),
                    prefix_overrides.as_deref(),
                    suffix_overrides.as_deref(),
                );
                variants.push(ExpansionState {
                    sql_buffer: trimmed,
                    branch_path: sub.branch_path,
                    params: sub.params,
                });
            }
        }
        SqlNode::ForEach {
            collection,
            open,
            separator,
            close,
            children,
            ..
        } => {
            expand_foreach(collection, open, separator, close, children, variants, depth, config);
        }
        SqlNode::Bind { .. } => {}
        SqlNode::Include { .. } => {}
    }
}

fn expand_if(
    test: &str,
    prepend: Option<&str>,
    children: &[SqlNode],
    variants: &mut Vec<ExpansionState>,
    depth: usize,
    config: &ExpandConfig,
) {
    match config.if_strategy {
        IfExpandStrategy::IncludeOnly => {
            let mut sub_variants = variants
                .drain(..)
                .map(|v| ExpansionState {
                    sql_buffer: String::new(),
                    branch_path: {
                        let mut p = v.branch_path;
                        p.push(BranchStep::If { test: test.to_string(), included: true });
                        p
                    },
                    params: v.params,
                })
                .collect::<Vec<_>>();
            for child in children {
                expand_node(child, &mut sub_variants, depth + 1, config);
            }
            let prepend_str = prepend.unwrap_or("");
            for mut sub in sub_variants {
                let content = apply_prepend_text(prepend_str, &sub.sql_buffer);
                sub.sql_buffer = content;
                variants.push(sub);
            }
        }
        IfExpandStrategy::ExcludeOnly => {
            for v in variants.iter_mut() {
                v.branch_path.push(BranchStep::If { test: test.to_string(), included: false });
            }
        }
        IfExpandStrategy::Both => {
            let mut new_variants = Vec::new();

            for v in variants.drain(..) {
                // included = true
            let mut inc = ExpansionState {
                sql_buffer: String::new(),
                branch_path: {
                    let mut p = v.branch_path.clone();
                    p.push(BranchStep::If { test: test.to_string(), included: true });
                    p
                },
                params: v.params.clone(),
            };
            let mut inc_vec = vec![inc];
            for child in children {
                expand_node(child, &mut inc_vec, depth + 1, config);
            }
            let prepend_str = prepend.unwrap_or("");
            inc_vec[0].sql_buffer = apply_prepend_text(prepend_str, &inc_vec[0].sql_buffer);
            new_variants.push(inc_vec.remove(0));

            // included = false
            let mut exc = ExpansionState {
                    sql_buffer: String::new(),
                    branch_path: v.branch_path,
                    params: v.params,
                };
                exc.branch_path.push(BranchStep::If { test: test.to_string(), included: false });
                new_variants.push(exc);
            }
            *variants = new_variants;
        }
    }
}

fn expand_choose(
    branches: &[(Option<String>, Vec<SqlNode>)],
    variants: &mut Vec<ExpansionState>,
    depth: usize,
    config: &ExpandConfig,
) {
    let mut new_variants = Vec::new();

    for v in variants.drain(..) {
        for (idx, (_test, branch_children)) in branches.iter().enumerate() {
            let mut sub = ExpansionState {
                sql_buffer: String::new(),
                branch_path: {
                    let mut p = v.branch_path.clone();
                    p.push(BranchStep::Choose { branch_index: idx });
                    p
                },
                params: v.params.clone(),
            };
            let mut sub_vec = vec![sub];
            for child in branch_children {
                expand_node(child, &mut sub_vec, depth + 1, config);
            }
            new_variants.push(sub_vec.remove(0));
            if new_variants.len() >= config.max_variants {
                break;
            }
        }
        if new_variants.len() >= config.max_variants {
            break;
        }
    }

    *variants = new_variants;
}

fn expand_foreach(
    collection: &str,
    open: &Option<String>,
    separator: &Option<String>,
    close: &Option<String>,
    children: &[SqlNode],
    variants: &mut Vec<ExpansionState>,
    depth: usize,
    config: &ExpandConfig,
) {
    let mut new_variants = Vec::new();

    for v in variants.drain(..) {
        for &size in &config.foreach_sizes {
            let mut sub = ExpansionState {
                sql_buffer: String::new(),
                branch_path: {
                    let mut p = v.branch_path.clone();
                    p.push(BranchStep::Foreach {
                        collection: collection.to_string(),
                        size,
                    });
                    p
                },
                params: v.params.clone(),
            };

            for i in 0..size {
                if i > 0 {
                    if let Some(sep) = separator {
                        sub.sql_buffer.push_str(sep);
                    }
                }
                let mut body_buf = vec![ExpansionState {
                    sql_buffer: String::new(),
                    branch_path: Vec::new(),
                    params: Vec::new(),
                }];
                for child in children {
                    expand_node(child, &mut body_buf, depth + 1, config);
                }
                if let Some(rendered) = body_buf.into_iter().next() {
                    sub.sql_buffer.push_str(&rendered.sql_buffer);
                    for p in rendered.params {
                        if !sub.params.iter().any(|ep| ep.name == p.name) {
                            sub.params.push(p);
                        }
                    }
                }
            }

            let open_str = open.as_deref().unwrap_or("");
            let close_str = close.as_deref().unwrap_or("");
            let wrapped = format!("{}{}{}", open_str, sub.sql_buffer, close_str);
            sub.sql_buffer = wrapped;

            new_variants.push(sub);
            if new_variants.len() >= config.max_variants {
                break;
            }
        }
        if new_variants.len() >= config.max_variants {
            break;
        }
    }

    *variants = new_variants;
}

fn render_text_params(text: &str, strategy: PlaceholderStrategy) -> String {
    match strategy {
        PlaceholderStrategy::PreserveInternalMarkers => flatten::replace_params(text),
        PlaceholderStrategy::QuestionMark => replace_params_question_mark(text),
    }
}

fn render_parameter(name: &str, java_type: &Option<String>, strategy: PlaceholderStrategy) -> String {
    match strategy {
        PlaceholderStrategy::PreserveInternalMarkers => {
            let sanitized = sanitize_param_name(name);
            match java_type {
                Some(t) => format!("{}{}_{}{}", PARAM_PREFIX, t.to_uppercase(), sanitized, PLACEHOLDER_SUFFIX),
                None => format!("{}{}{}", PARAM_PREFIX, sanitized, PLACEHOLDER_SUFFIX),
            }
        }
        PlaceholderStrategy::QuestionMark => "?".to_string(),
    }
}

fn render_raw(expr: &str, java_type: &Option<String>, strategy: PlaceholderStrategy) -> String {
    match strategy {
        PlaceholderStrategy::PreserveInternalMarkers => {
            let sanitized = sanitize_param_name(expr);
            match java_type {
                Some(t) => format!("{}{}_{}{}", RAW_PREFIX, t.to_uppercase(), sanitized, PLACEHOLDER_SUFFIX),
                None => format!("{}{}{}", RAW_PREFIX, sanitized, PLACEHOLDER_SUFFIX),
            }
        }
        PlaceholderStrategy::QuestionMark => "?".to_string(),
    }
}

/// Replace #{param} and ${expr} with `?` (question mark mode).
fn replace_params_question_mark(sql: &str) -> String {
    let mut result = String::with_capacity(sql.len());
    let chars: Vec<char> = sql.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut in_string = false;

    while i < len {
        let c = chars[i];
        if c == '\'' {
            if in_string && i + 1 < len && chars[i + 1] == '\'' {
                result.push_str("''");
                i += 2;
                continue;
            }
            in_string = !in_string;
            result.push(c);
            i += 1;
            continue;
        }
        if in_string {
            result.push(c);
            i += 1;
            continue;
        }
        if (c == '#' || c == '$') && i + 1 < len && chars[i + 1] == '{' {
            if let Some(end) = crate::ibatis::util::find_closing_brace(&chars, i + 2) {
                result.push('?');
                i = end + 1;
                continue;
            }
        }
        result.push(c);
        i += 1;
    }
    result
}

fn apply_prepend_text(prepend: &str, content: &str) -> String {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if prepend.is_empty() {
        return content.to_string();
    }
    format!("{} {}", prepend, content)
}

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
