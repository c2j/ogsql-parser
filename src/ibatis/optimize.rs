//! 互斥 `<if>` 条件自动合并优化。
//!
//! 检测 `SqlNode` 树中相邻的互斥 `<if>` 节点，将其合并为 `<choose>`，
//! 消除 `expand_variants` 中的组合爆炸问题。
//!
//! # 支持的互斥模式
//!
//! | 模式 | 示例 |
//! |------|------|
//! | 同变量 == vs != 同值 | `roleId == 3` vs `roleId != 3` |
//! | 同变量 == 不同值 | `type == 'A'` vs `type == 'B'` |
//! | AND 合取中含互斥对 | `roleId != null and roleId != 3` vs `roleId != null and roleId == 3` |
//!
//! # 不处理的场景
//!
//! - 含 `or` 的表达式（复杂度太高）
//! - 有 `prepend` 属性的 `<if>`（`Choose` 不支持 per-branch prepend）
//! - `>`, `<`, `>=`, `<=` 操作符（仅处理 `==` / `!=`）

use crate::ibatis::types::SqlNode;

// ── OGNL 比较表达式解析 ──

#[derive(Debug, Clone, PartialEq)]
struct Comparison {
    variable: String,
    operator: CmpOp,
    value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CmpOp {
    Eq,
    Ne,
}

/// 解析 OGNL test 表达式为原子比较列表。
///
/// 支持 AND 合取表达式（如 `roleId != null and roleId != 3`），
/// 不支持含 OR 的表达式（返回空以跳过优化）。
fn parse_comparisons(expr: &str) -> Vec<Comparison> {
    let expr = expr.trim();
    if expr.is_empty() {
        return Vec::new();
    }

    // Skip expressions with OR — mutual exclusion reasoning too complex
    if contains_keyword(expr, "or") {
        return Vec::new();
    }

    let parts = split_on_keyword(expr, "and");

    let mut comparisons = Vec::new();
    for part in parts {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some(cmp) = parse_single_comparison(part) {
            comparisons.push(cmp);
        }
    }
    comparisons
}

/// 按 OGNL 关键字（AND/OR）拆分表达式，保留原始大小写。
///
/// 假设表达式为纯 ASCII（OGNL test 属性的常规情况），
/// 因此可直接用字节偏移切片。
fn split_on_keyword<'a>(expr: &'a str, keyword: &str) -> Vec<&'a str> {
    let lower_expr = expr.to_lowercase();
    let lower_keyword = format!(" {} ", keyword.to_lowercase());
    let kw_len = lower_keyword.len();

    let mut parts = Vec::new();
    let mut start = 0;
    let mut i = 0;

    while i + kw_len <= lower_expr.len() {
        if lower_expr[i..i + kw_len] == lower_keyword {
            parts.push(&expr[start..i]);
            start = i + kw_len;
            i += kw_len;
        } else {
            i += 1;
        }
    }
    parts.push(&expr[start..]);
    parts
}

/// 检查表达式中是否含某个关键字（以空格分隔）。
fn contains_keyword(expr: &str, keyword: &str) -> bool {
    let pattern = format!(" {} ", keyword.to_lowercase());
    expr.to_lowercase().contains(&pattern)
}

fn parse_single_comparison(part: &str) -> Option<Comparison> {
    let chars: Vec<char> = part.chars().collect();
    let len = chars.len();

    for i in 0..len.saturating_sub(1) {
        // Check != first (! comes before =, avoids == false match)
        if chars[i] == '!' && chars[i + 1] == '=' {
            let variable: String = chars[..i].iter().collect::<String>().trim().to_string();
            let value: String = chars[i + 2..].iter().collect::<String>().trim().to_string();
            if !variable.is_empty() && !value.is_empty() {
                return Some(Comparison { variable, operator: CmpOp::Ne, value });
            }
        }
        // Then check == (exclude !=, >=, <=)
        if chars[i] == '=' && chars[i + 1] == '=' {
            if i > 0 && (chars[i - 1] == '!' || chars[i - 1] == '<' || chars[i - 1] == '>') {
                continue;
            }
            let variable: String = chars[..i].iter().collect::<String>().trim().to_string();
            let value: String = chars[i + 2..].iter().collect::<String>().trim().to_string();
            if !variable.is_empty() && !value.is_empty() {
                return Some(Comparison { variable, operator: CmpOp::Eq, value });
            }
        }
    }
    None
}

// ── 互斥检测 ──

/// 检查两个 OGNL test 表达式是否互斥。
///
/// 判定规则：
/// 1. 同变量、同值、相反操作符（== vs !=）
/// 2. 同变量、均为 ==、不同值
pub fn are_mutually_exclusive(test1: &str, test2: &str) -> bool {
    let cmps1 = parse_comparisons(test1);
    let cmps2 = parse_comparisons(test2);

    if cmps1.is_empty() || cmps2.is_empty() {
        return false;
    }

    for c1 in &cmps1 {
        for c2 in &cmps2 {
            if c1.variable != c2.variable {
                continue;
            }
            // 模式 1: 同变量、同值、相反操作符
            if c1.value == c2.value && c1.operator != c2.operator {
                return true;
            }
            // 模式 2: 同变量、均为 ==、不同值
            if c1.operator == CmpOp::Eq && c2.operator == CmpOp::Eq && c1.value != c2.value {
                return true;
            }
        }
    }
    false
}

// ── 树遍历与合并 ──

/// 递归遍历 `SqlNode` 树，将相邻互斥 `<if>` 合并为 `<choose>`。
///
/// 在每个容器节点（Sequence, Where, Set, Trim, ForEach, Choose）的子节点列表中，
/// 检测连续的互斥 `<if>` 节点并将其替换为单个 `<choose>`。
pub fn optimize_exclusive_ifs(node: &mut SqlNode) {
    match node {
        SqlNode::Sequence { children }
        | SqlNode::Where { children }
        | SqlNode::Set { children }
        | SqlNode::Trim { children, .. }
        | SqlNode::ForEach { children, .. } => {
            for child in children.iter_mut() {
                optimize_exclusive_ifs(child);
            }
            merge_exclusive_ifs(children);
        }
        SqlNode::If { children, .. } => {
            for child in children.iter_mut() {
                optimize_exclusive_ifs(child);
            }
        }
        SqlNode::Choose { branches } => {
            for (_, branch_children) in branches.iter_mut() {
                for child in branch_children.iter_mut() {
                    optimize_exclusive_ifs(child);
                }
                merge_exclusive_ifs(branch_children);
            }
        }
        SqlNode::Text { .. }
        | SqlNode::Parameter { .. }
        | SqlNode::RawExpr { .. }
        | SqlNode::Bind { .. }
        | SqlNode::Include { .. } => {}
    }
}

/// 在子节点列表中合并相邻互斥 `<if>` 节点为 `<choose>`。
///
/// 扫描连续的 If 节点（无 prepend），若两两互斥则合并为一个 Choose。
/// 有 prepend 的 If 不参与合并（Choose 不支持 per-branch prepend）。
fn merge_exclusive_ifs(children: &mut Vec<SqlNode>) {
    let mut new_children = Vec::with_capacity(children.len());
    let mut i = 0;

    while i < children.len() {
        if let SqlNode::If { prepend: None, .. } = &children[i] {
            let mut run_if_indices = vec![i];
            let mut scan = i + 1;

            loop {
                // Skip whitespace-only Text nodes (XML indentation between <if> tags)
                while scan < children.len() && is_whitespace_only(&children[scan]) {
                    scan += 1;
                }
                if scan >= children.len() {
                    break;
                }

                if let SqlNode::If { test, prepend: None, .. } = &children[scan] {
                    let mut all_exclusive = true;
                    for &prev_idx in &run_if_indices {
                        if let SqlNode::If { test: prev_test, .. } = &children[prev_idx] {
                            if !are_mutually_exclusive(prev_test, test) {
                                all_exclusive = false;
                                break;
                            }
                        }
                    }
                    if all_exclusive {
                        run_if_indices.push(scan);
                        scan += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            if run_if_indices.len() > 1 {
                let branches: Vec<(Option<String>, Vec<SqlNode>)> = run_if_indices
                    .iter()
                    .map(|&idx| match &children[idx] {
                        SqlNode::If { test, children, .. } => (Some(test.clone()), children.clone()),
                        _ => unreachable!(),
                    })
                    .collect();
                new_children.push(SqlNode::Choose { branches });
                i = scan;
            } else {
                new_children.push(children[i].clone());
                i += 1;
            }
        } else {
            new_children.push(children[i].clone());
            i += 1;
        }
    }

    *children = new_children;
}

fn is_whitespace_only(node: &SqlNode) -> bool {
    matches!(node, SqlNode::Text { content } if content.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Comparison parsing ──

    #[test]
    fn test_parse_simple_eq() {
        let cmps = parse_comparisons("roleId == 3");
        assert_eq!(cmps.len(), 1);
        assert_eq!(cmps[0].variable, "roleId");
        assert_eq!(cmps[0].operator, CmpOp::Eq);
        assert_eq!(cmps[0].value, "3");
    }

    #[test]
    fn test_parse_simple_ne() {
        let cmps = parse_comparisons("roleId != null");
        assert_eq!(cmps.len(), 1);
        assert_eq!(cmps[0].variable, "roleId");
        assert_eq!(cmps[0].operator, CmpOp::Ne);
        assert_eq!(cmps[0].value, "null");
    }

    #[test]
    fn test_parse_compound_and() {
        let cmps = parse_comparisons("roleId != null and roleId != 3");
        assert_eq!(cmps.len(), 2);
        assert_eq!(cmps[0].variable, "roleId");
        assert_eq!(cmps[0].operator, CmpOp::Ne);
        assert_eq!(cmps[0].value, "null");
        assert_eq!(cmps[1].variable, "roleId");
        assert_eq!(cmps[1].operator, CmpOp::Ne);
        assert_eq!(cmps[1].value, "3");
    }

    #[test]
    fn test_parse_compound_and_no_space() {
        // "roleId !=3" — no space before 3
        let cmps = parse_comparisons("roleId != null and roleId !=3");
        assert_eq!(cmps.len(), 2);
        assert_eq!(cmps[1].variable, "roleId");
        assert_eq!(cmps[1].value, "3");
    }

    #[test]
    fn test_parse_or_returns_empty() {
        assert!(parse_comparisons("roleId == 3 or name == null").is_empty());
    }

    #[test]
    fn test_parse_empty() {
        assert!(parse_comparisons("").is_empty());
    }

    #[test]
    fn test_parse_no_comparison() {
        // Method call without == / !=
        assert!(parse_comparisons("list.size() > 0").is_empty());
    }

    #[test]
    fn test_parse_string_value() {
        let cmps = parse_comparisons("type == 'ACTIVE'");
        assert_eq!(cmps.len(), 1);
        assert_eq!(cmps[0].value, "'ACTIVE'");
    }

    // ── Mutual exclusion detection ──

    #[test]
    fn test_exclusive_eq_ne_same_value() {
        assert!(are_mutually_exclusive("roleId == 3", "roleId != 3"));
    }

    #[test]
    fn test_exclusive_eq_different_values() {
        assert!(are_mutually_exclusive("roleId == 3", "roleId == 5"));
    }

    #[test]
    fn test_exclusive_compound() {
        assert!(are_mutually_exclusive("roleId != null and roleId != 3", "roleId != null and roleId == 3"));
    }

    #[test]
    fn test_exclusive_compound_no_space() {
        assert!(are_mutually_exclusive("roleId != null and roleId !=3", "roleId != null and roleId == 3"));
    }

    #[test]
    fn test_not_exclusive_independent_vars() {
        assert!(!are_mutually_exclusive("name != null", "age != null"));
    }

    #[test]
    fn test_not_exclusive_same_condition() {
        assert!(!are_mutually_exclusive("roleId == 3", "roleId == 3"));
    }

    #[test]
    fn test_not_exclusive_both_ne_same() {
        assert!(!are_mutually_exclusive("roleId != 3", "roleId != 3"));
    }

    #[test]
    fn test_not_exclusive_or_expression() {
        // OR expressions are not parsed, so can't determine exclusivity
        assert!(!are_mutually_exclusive("roleId == 3", "roleId != 3 or name == null"));
    }

    // ── Tree transformation ──

    #[test]
    fn test_merge_two_exclusive_ifs() {
        let mut tree = SqlNode::Sequence {
            children: vec![
                SqlNode::If {
                    test: "roleId != null and roleId != 3".to_string(),
                    prepend: None,
                    children: vec![SqlNode::Text { content: "SELECT 1".to_string() }],
                },
                SqlNode::If {
                    test: "roleId != null and roleId == 3".to_string(),
                    prepend: None,
                    children: vec![SqlNode::Text { content: "SELECT 2".to_string() }],
                },
            ],
        };

        optimize_exclusive_ifs(&mut tree);

        if let SqlNode::Sequence { children } = &tree {
            assert_eq!(children.len(), 1, "should merge into single Choose");
            if let SqlNode::Choose { branches } = &children[0] {
                assert_eq!(branches.len(), 2);
                assert_eq!(branches[0].0.as_deref(), Some("roleId != null and roleId != 3"));
                assert_eq!(branches[1].0.as_deref(), Some("roleId != null and roleId == 3"));
                assert!(branches[0].1[0].clone().as_text().unwrap().contains("SELECT 1"));
                assert!(branches[1].1[0].clone().as_text().unwrap().contains("SELECT 2"));
            } else {
                panic!("expected Choose, got {:?}", children[0]);
            }
        } else {
            panic!("expected Sequence");
        }
    }

    #[test]
    fn test_merge_three_exclusive_ifs() {
        let mut tree = SqlNode::Sequence {
            children: vec![
                SqlNode::If {
                    test: "type == 1".to_string(),
                    prepend: None,
                    children: vec![SqlNode::Text { content: "A".to_string() }],
                },
                SqlNode::If {
                    test: "type == 2".to_string(),
                    prepend: None,
                    children: vec![SqlNode::Text { content: "B".to_string() }],
                },
                SqlNode::If {
                    test: "type == 3".to_string(),
                    prepend: None,
                    children: vec![SqlNode::Text { content: "C".to_string() }],
                },
            ],
        };

        optimize_exclusive_ifs(&mut tree);

        if let SqlNode::Sequence { children } = &tree {
            assert_eq!(children.len(), 1);
            if let SqlNode::Choose { branches } = &children[0] {
                assert_eq!(branches.len(), 3);
            }
        }
    }

    #[test]
    fn test_no_merge_independent_ifs() {
        let mut tree = SqlNode::Sequence {
            children: vec![
                SqlNode::If {
                    test: "name != null".to_string(),
                    prepend: None,
                    children: vec![SqlNode::Text { content: "AND name = #{name}".to_string() }],
                },
                SqlNode::If {
                    test: "age != null".to_string(),
                    prepend: None,
                    children: vec![SqlNode::Text { content: "AND age = #{age}".to_string() }],
                },
            ],
        };

        optimize_exclusive_ifs(&mut tree);

        if let SqlNode::Sequence { children } = &tree {
            assert_eq!(children.len(), 2, "independent Ifs should NOT be merged");
            assert!(matches!(children[0], SqlNode::If { .. }));
            assert!(matches!(children[1], SqlNode::If { .. }));
        }
    }

    #[test]
    fn test_no_merge_if_with_prepend() {
        let mut tree = SqlNode::Sequence {
            children: vec![
                SqlNode::If {
                    test: "roleId == 3".to_string(),
                    prepend: Some("AND".to_string()),
                    children: vec![SqlNode::Text { content: "name = 'admin'".to_string() }],
                },
                SqlNode::If {
                    test: "roleId != 3".to_string(),
                    prepend: None,
                    children: vec![SqlNode::Text { content: "name = 'user'".to_string() }],
                },
            ],
        };

        optimize_exclusive_ifs(&mut tree);

        if let SqlNode::Sequence { children } = &tree {
            assert_eq!(children.len(), 2, "Ifs with prepend should NOT be merged");
        }
    }

    #[test]
    fn test_no_merge_mixed_if_and_text() {
        let mut tree = SqlNode::Sequence {
            children: vec![
                SqlNode::If {
                    test: "roleId == 3".to_string(),
                    prepend: None,
                    children: vec![SqlNode::Text { content: "A".to_string() }],
                },
                SqlNode::Text { content: "SOME SQL".to_string() },
                SqlNode::If {
                    test: "roleId != 3".to_string(),
                    prepend: None,
                    children: vec![SqlNode::Text { content: "B".to_string() }],
                },
            ],
        };

        optimize_exclusive_ifs(&mut tree);

        if let SqlNode::Sequence { children } = &tree {
            assert_eq!(children.len(), 3, "non-whitespace Text should break If run");
        }
    }

    #[test]
    fn test_merge_ifs_with_whitespace_text_between() {
        let mut tree = SqlNode::Sequence {
            children: vec![
                SqlNode::If {
                    test: "roleId != null and roleId != 3".to_string(),
                    prepend: None,
                    children: vec![SqlNode::Text { content: "SELECT 1".to_string() }],
                },
                SqlNode::Text { content: "\n        ".to_string() },
                SqlNode::If {
                    test: "roleId != null and roleId == 3".to_string(),
                    prepend: None,
                    children: vec![SqlNode::Text { content: "SELECT 2".to_string() }],
                },
            ],
        };

        optimize_exclusive_ifs(&mut tree);

        if let SqlNode::Sequence { children } = &tree {
            assert_eq!(children.len(), 1, "whitespace-only Text should be consumed during merge");
            if let SqlNode::Choose { branches } = &children[0] {
                assert_eq!(branches.len(), 2);
            } else {
                panic!("expected Choose, got {:?}", children[0]);
            }
        }
    }

    #[test]
    fn test_recursive_into_where() {
        let mut tree = SqlNode::Where {
            children: vec![SqlNode::Sequence {
                children: vec![
                    SqlNode::If {
                        test: "type == 'A'".to_string(),
                        prepend: None,
                        children: vec![SqlNode::Text { content: "AND a = 1".to_string() }],
                    },
                    SqlNode::If {
                        test: "type == 'B'".to_string(),
                        prepend: None,
                        children: vec![SqlNode::Text { content: "AND b = 2".to_string() }],
                    },
                ],
            }],
        };

        optimize_exclusive_ifs(&mut tree);

        if let SqlNode::Where { children } = &tree {
            if let SqlNode::Sequence { children } = &children[0] {
                assert_eq!(children.len(), 1, "should merge Ifs inside Where > Sequence");
                assert!(matches!(children[0], SqlNode::Choose { .. }));
            }
        }
    }
}

impl SqlNode {
    fn as_text(&self) -> Option<&String> {
        match self {
            SqlNode::Text { content } => Some(content),
            _ => None,
        }
    }
}
