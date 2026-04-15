use crate::parser::ParserError;
use crate::token::SourceLocation;

pub const KNOWN_HINTS: &[&str] = &[
    "tablescan",
    "indexscan",
    "indexonlyscan",
    "gsi",
    "gsitable",
    "bitmapscan",
    "nestloop",
    "hashjoin",
    "mergejoin",
    "leading",
    "broadcast",
    "redistribute",
    "gather",
    "use_hash_agg",
    "use_sort_agg",
    "set",
    "use_cplan",
    "use_gplan",
    "choose_adaptive_gplan",
    "no_gpc",
    "material_subplan",
    "begin_outline_data",
    "end_outline_data",
    "blockname",
    "wlmrule",
    "expand_sublink",
    "no_expand_sublink",
    "expand_sublink_having",
    "no_expand_sublink_having",
    "expand_sublink_target",
    "no_expand_sublink_target",
    "expand_sublink_unique_check",
    "no_expand_sublink_unique_check",
    "sublink_disable_replicated",
    "no_sublink_disable_replicated",
    "sublink_disable_expr",
    "no_sublink_disable_expr",
    "enable_sublink_enhanced",
    "no_enable_sublink_enhanced",
    "use_magic_set",
    "no_use_magic_set",
    "partial_push",
    "no_partial_push",
    "reduce_order_by",
    "no_reduce_order_by",
    "remove_not_null",
    "no_remove_not_null",
    "lazy_agg",
    "no_lazy_agg",
    "expand_subquery",
    "no_expand_subquery",
    "pushdown_having",
    "no_pushdown_having",
    "inlist_to_join",
    "no_inlist_to_join",
    "rownum_pushdown",
    "no_rownum_pushdown",
    "rows",
    "no_expand",
    "predpush_same_level",
    "nestloop_index",
    "skew",
    "no_skew",
    "index_paramsel",
];

const HINTS_WITH_PARENS: &[&str] = &[
    "tablescan",
    "indexscan",
    "indexonlyscan",
    "gsi",
    "gsitable",
    "bitmapscan",
    "nestloop",
    "hashjoin",
    "mergejoin",
    "leading",
    "broadcast",
    "redistribute",
    "set",
    "blockname",
    "wlmrule",
    "rows",
    "predpush_same_level",
    "nestloop_index",
    "skew",
    "no_skew",
    "inlist_to_join",
    "index_paramsel",
];

const HINTS_SUPPORTING_NO_PREFIX: &[&str] = &[
    "tablescan",
    "indexscan",
    "indexonlyscan",
    "gsi",
    "gsitable",
    "bitmapscan",
    "nestloop",
    "hashjoin",
    "mergejoin",
    "broadcast",
    "redistribute",
];

const NO_PAREN_HINTS: &[&str] = &[
    "use_hash_agg",
    "use_sort_agg",
    "use_cplan",
    "use_gplan",
    "choose_adaptive_gplan",
    "no_gpc",
    "material_subplan",
    "begin_outline_data",
    "end_outline_data",
    "no_expand",
    "expand_sublink",
    "no_expand_sublink",
    "expand_sublink_having",
    "no_expand_sublink_having",
    "expand_sublink_target",
    "no_expand_sublink_target",
    "expand_sublink_unique_check",
    "no_expand_sublink_unique_check",
    "sublink_disable_replicated",
    "no_sublink_disable_replicated",
    "sublink_disable_expr",
    "no_sublink_disable_expr",
    "enable_sublink_enhanced",
    "no_enable_sublink_enhanced",
    "use_magic_set",
    "no_use_magic_set",
    "partial_push",
    "no_partial_push",
    "reduce_order_by",
    "no_reduce_order_by",
    "remove_not_null",
    "no_remove_not_null",
    "lazy_agg",
    "no_lazy_agg",
    "expand_subquery",
    "no_expand_subquery",
    "pushdown_having",
    "no_pushdown_having",
    "rownum_pushdown",
    "no_rownum_pushdown",
];

#[derive(Debug, Clone)]
struct ParsedHint {
    name: String,
    negated: bool,
    queryblock: Option<String>,
    args: Option<String>,
}

fn parse_hint_list(content: &str) -> Vec<ParsedHint> {
    let mut hints = Vec::new();
    let chars = content.chars().collect::<Vec<_>>();
    let len = chars.len();
    let mut pos = 0;

    while pos < len {
        while pos < len && chars[pos].is_whitespace() {
            pos += 1;
        }
        if pos >= len {
            break;
        }

        let mut name = String::new();
        while pos < len && chars[pos] != '(' && !chars[pos].is_whitespace() && chars[pos] != '@' {
            name.push(chars[pos]);
            pos += 1;
        }
        if name.is_empty() {
            break;
        }

        // Check for @queryblock between name and parens: hintname @qb (args)
        let mut queryblock = None;
        if pos < len && chars[pos] == '@' {
            pos += 1;
            let mut qb = String::from("@");
            while pos < len && chars[pos] != '(' && !chars[pos].is_whitespace() {
                qb.push(chars[pos]);
                pos += 1;
            }
            queryblock = Some(qb);
        }

        let mut args = None;
        if pos < len && chars[pos] == '(' {
            pos += 1;
            let mut arg_content = String::new();
            let mut depth = 1;
            while pos < len && depth > 0 {
                match chars[pos] {
                    '(' => {
                        depth += 1;
                        arg_content.push('(');
                    }
                    ')' => {
                        depth -= 1;
                        if depth > 0 {
                            arg_content.push(')');
                        }
                    }
                    c => arg_content.push(c),
                }
                pos += 1;
            }
            // Extract @queryblock from inside parens if present: tablescan(@sel$1 t1)
            if queryblock.is_none() {
                let trimmed = arg_content.trim();
                if let Some(rest) = trimmed.strip_prefix('@') {
                    if let Some(space_pos) = rest.find(char::is_whitespace) {
                        let qb = format!("@{}", &rest[..space_pos]);
                        let remaining = rest[space_pos..].trim().to_string();
                        queryblock = Some(qb);
                        arg_content = remaining;
                    }
                }
            }
            args = Some(arg_content);
        }

        let (actual_name, negated) = resolve_name_and_negation(&name);

        hints.push(ParsedHint {
            name: actual_name,
            negated,
            queryblock,
            args,
        });
    }

    hints
}

fn resolve_name_and_negation(name: &str) -> (String, bool) {
    let lower = name.to_lowercase();

    if lower.starts_with("no ") {
        return (lower[3..].trim().to_string(), true);
    }

    // Check if the full name with no_ prefix is a known hint (e.g. no_expand)
    if lower.starts_with("no_") {
        if KNOWN_HINTS.contains(&lower.as_str()) {
            return (lower, false);
        }
        let base = &lower[3..];
        if KNOWN_HINTS.contains(&base) {
            return (base.to_string(), true);
        }
    }

    // Check no-prefix without underscore: notablescan -> tablescan
    if lower.starts_with("no") && lower.len() > 2 {
        let base = &lower[2..];
        if HINTS_SUPPORTING_NO_PREFIX.contains(&base) {
            return (base.to_string(), true);
        }
    }

    (lower, false)
}

pub fn validate_hints(hint_content: &str, location: SourceLocation) -> Vec<ParserError> {
    let hints = parse_hint_list(hint_content);
    let mut warnings = Vec::new();
    for hint in &hints {
        warnings.extend(validate_single_hint(hint, location.clone()));
    }
    warnings
}

fn validate_single_hint(hint: &ParsedHint, location: SourceLocation) -> Vec<ParserError> {
    let mut warnings = Vec::new();

    if !KNOWN_HINTS.contains(&hint.name.as_str()) {
        warnings.push(ParserError::Warning {
            message: format!(
                "Unknown hint '{}' — not in GaussDB documented hint list",
                hint.name
            ),
            location: location.clone(),
        });
        return warnings;
    }

    if hint.negated && !HINTS_SUPPORTING_NO_PREFIX.contains(&hint.name.as_str()) {
        warnings.push(ParserError::Warning {
            message: format!(
                "Hint 'no {}' does not support 'no' prefix negation",
                hint.name
            ),
            location: location.clone(),
        });
    }

    let needs_parens = HINTS_WITH_PARENS.contains(&hint.name.as_str());
    if needs_parens && hint.args.is_none() {
        warnings.push(ParserError::Warning {
            message: format!(
                "Hint '{}' requires parenthesized arguments, e.g. {}(table_name)",
                hint.name, hint.name
            ),
            location: location.clone(),
        });
    }

    if !needs_parens && hint.args.is_some() && NO_PAREN_HINTS.contains(&hint.name.as_str()) {
        warnings.push(ParserError::Warning {
            message: format!("Hint '{}' does not take parenthesized arguments", hint.name),
            location: location.clone(),
        });
    }

    if hint.name == "wlmrule" {
        if let Some(ref args) = hint.args {
            let trimmed = args.trim();
            if !trimmed.starts_with('"') || !trimmed.ends_with('"') {
                warnings.push(ParserError::Warning {
                    message: "Hint 'wlmrule' requires quoted argument: wlmrule(\"time_limit,max_execute_time,max_iops\")".to_string(),
                    location: location.clone(),
                });
            }
        }
    }

    if hint.name == "set" {
        if let Some(ref args) = hint.args {
            let parts: Vec<&str> = args.split_whitespace().collect();
            if parts.len() < 2 {
                warnings.push(ParserError::Warning {
                    message: "Hint 'set' requires parameter name and value: set(param_name value)"
                        .to_string(),
                    location: location.clone(),
                });
            }
        }
    }

    if hint.name == "leading" {
        if let Some(ref args) = hint.args {
            let trimmed = args.trim();
            if trimmed.is_empty() {
                warnings.push(ParserError::Warning {
                    message: "Hint 'leading' requires at least one table name".to_string(),
                    location: location.clone(),
                });
            } else {
                let open = trimmed.chars().filter(|c| *c == '(').count();
                let close = trimmed.chars().filter(|c| *c == ')').count();
                if open != close {
                    warnings.push(ParserError::Warning {
                        message: "Hint 'leading' has unbalanced parentheses in table list"
                            .to_string(),
                        location: location.clone(),
                    });
                }
            }
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loc() -> SourceLocation {
        SourceLocation::default()
    }

    #[test]
    fn test_parse_simple() {
        let h = parse_hint_list("tablescan(t1)");
        assert_eq!(h.len(), 1);
        assert_eq!(h[0].name, "tablescan");
        assert_eq!(h[0].args.as_deref(), Some("t1"));
        assert!(!h[0].negated);
    }

    #[test]
    fn test_parse_multiple() {
        let h = parse_hint_list("tablescan(t1) leading(t1 t2)");
        assert_eq!(h.len(), 2);
    }

    #[test]
    fn test_parse_negated() {
        let h = parse_hint_list("nohashjoin(t1 t2)");
        assert_eq!(h.len(), 1);
        assert!(h[0].negated);
        assert_eq!(h[0].name, "hashjoin");
    }

    #[test]
    fn test_parse_queryblock() {
        let h = parse_hint_list("tablescan(@sel$1 t1)");
        assert_eq!(h[0].queryblock.as_deref(), Some("@sel$1"));
        assert_eq!(h[0].args.as_deref(), Some("t1"));
    }

    #[test]
    fn test_validate_known_ok() {
        assert!(validate_hints("tablescan(t1)", loc()).is_empty());
    }

    #[test]
    fn test_validate_unknown() {
        let w = validate_hints("invalid_hint(t1)", loc());
        assert_eq!(w.len(), 1);
        assert!(w[0].to_string().contains("Unknown hint"));
    }

    #[test]
    fn test_validate_missing_parens() {
        let w = validate_hints("tablescan", loc());
        assert!(w
            .iter()
            .any(|e| e.to_string().contains("requires parenthesized")));
    }

    #[test]
    fn test_validate_set_ok() {
        assert!(validate_hints("set(enable_hashjoin off)", loc()).is_empty());
    }

    #[test]
    fn test_validate_set_missing_value() {
        let w = validate_hints("set(enable_hashjoin)", loc());
        assert!(w
            .iter()
            .any(|e| e.to_string().contains("parameter name and value")));
    }

    #[test]
    fn test_validate_wlmrule_ok() {
        assert!(validate_hints("wlmrule(\"100,500,1\")", loc()).is_empty());
    }

    #[test]
    fn test_validate_wlmrule_bad() {
        let w = validate_hints("wlmrule(100,500,1)", loc());
        assert!(w.iter().any(|e| e.to_string().contains("quoted argument")));
    }

    #[test]
    fn test_validate_no_prefix_invalid() {
        let w = validate_hints("notablescan(t1)", loc());
        assert_eq!(w.len(), 0);

        let w2 = validate_hints("noleading(t1 t2)", loc());
        assert!(w2.iter().any(|e| e.to_string().contains("Unknown hint")));
    }

    #[test]
    fn test_no_expand_is_known() {
        assert!(validate_hints("no_expand", loc()).is_empty());
    }

    #[test]
    fn test_leading_unbalanced_parens() {
        let w = validate_hints("leading((t1 t2) t3)", loc());
        assert_eq!(w.len(), 0);
    }

    #[test]
    fn test_leading_empty_args() {
        let w = validate_hints("leading()", loc());
        assert!(w
            .iter()
            .any(|e| e.to_string().contains("at least one table")));
    }
}
