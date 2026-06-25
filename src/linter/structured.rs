//! Structured-mapper lint rules — rules that need the pre-expansion `SqlNode`
//! tree (dynamic SQL), which the flat `SqlLinter::lint(&[StatementInfo])` API
//! cannot see.
//!
//! Currently houses the foreach-in-INSERT-VALUES flavor of rule C018.

use crate::ibatis::types::{SqlNode, StructuredMapper};
use crate::linter::{Confidence, LintConfig, SqlWarning, WarningLevel};

/// Lint a structured mapper for foreach-in-INSERT-VALUES (rule C018,
/// dynamic-SQL variant). Flat SQL collapses `<foreach>` to a single iteration,
/// so this must run on the `SqlNode` tree before expansion.
pub fn lint_structured_mapper(mapper: &StructuredMapper, config: &LintConfig) -> Vec<SqlWarning> {
    if !mapper.errors.is_empty() {
        return vec![];
    }
    let mut warnings = Vec::new();

    for stmt in &mapper.statements {
        if let Some(foreach_node) = find_foreach_in_insert_values(&stmt.body) {
            let params_per_row = count_params_in_foreach_body(foreach_node);
            if params_per_row == 0 {
                continue;
            }
            let estimated_rows = config.foreach_estimated_rows;
            let total_params = params_per_row.saturating_mul(estimated_rows);

            // FIXED: no `|| is_insert_values` clause — the threshold is the sole trigger
            if total_params > config.max_insert_values_rows {
                warnings.push(SqlWarning {
                    level: WarningLevel::Caution,
                    rule_id: "C018".to_string(),
                    rule_name: "excessive-insert-values".to_string(),
                    message: format!(
                        "INSERT VALUES 包含 foreach 动态批量插入，每行 {} 个参数。\
                         若运行时集合包含约 {} 行，总绑定参数将达 {}，超过阈值 {}。\
                         建议分批提交或使用 COPY。",
                        params_per_row, estimated_rows, total_params, config.max_insert_values_rows
                    ),
                    suggestion: Some("拆分为更小批次插入以减少锁持有时间，或使用 COPY 替代".to_string()),
                    location: crate::SourceLocation::default(),
                    gaussdb_ref: None,
                    confidence: Confidence::Partial,
                });
            }
        }
    }
    warnings
}

/// Find a ForEach node nested in the SqlNode tree (typically inside INSERT VALUES).
fn find_foreach_in_insert_values(node: &SqlNode) -> Option<&SqlNode> {
    match node {
        SqlNode::ForEach { .. } => Some(node),
        SqlNode::Sequence { children }
        | SqlNode::Trim { children, .. }
        | SqlNode::Where { children, .. }
        | SqlNode::Set { children, .. }
        | SqlNode::If { children, .. } => {
            for child in children {
                if let Some(f) = find_foreach_in_insert_values(child) {
                    return Some(f);
                }
            }
            None
        }
        SqlNode::Choose { branches } => {
            for (_, ch) in branches {
                for c in ch {
                    if let Some(f) = find_foreach_in_insert_values(c) {
                        return Some(f);
                    }
                }
            }
            None
        }
        _ => None,
    }
}

/// Count the number of Parameter nodes inside a ForEach body.
fn count_params_in_foreach_body(node: &SqlNode) -> usize {
    match node {
        SqlNode::Parameter { .. } => 1,
        SqlNode::Sequence { children }
        | SqlNode::Trim { children, .. }
        | SqlNode::Where { children, .. }
        | SqlNode::Set { children, .. }
        | SqlNode::If { children, .. }
        | SqlNode::ForEach { children, .. } => children.iter().map(count_params_in_foreach_body).sum(),
        SqlNode::Choose { branches } => {
            branches.iter().flat_map(|(_, ch)| ch.iter().map(count_params_in_foreach_body)).sum()
        }
        SqlNode::RawExpr { .. } => 1, // ${expr} also counts as a parameter
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_mapper(xml: &[u8]) -> StructuredMapper {
        crate::ibatis::parse_mapper_bytes_structured(xml)
    }

    #[test]
    fn c018_fires_when_estimated_params_exceed_threshold() {
        // 5 params per row × estimated_rows (1000) = 5000. We set the
        // threshold explicitly to make the test deterministic and decoupled
        // from the LintConfig default (which is 65535).
        let xml = br#"<mapper namespace="t">
            <insert id="batch">
                INSERT INTO t (a, b, c, d, e) VALUES
                <foreach collection="rows" item="r" separator=",">
                    (#{r.a}, #{r.b}, #{r.c}, #{r.d}, #{r.e})
                </foreach>
            </insert>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let mut config = LintConfig::default();
        config.max_insert_values_rows = 1000; // 5000 > 1000 → must fire
        let warnings = lint_structured_mapper(&mapper, &config);
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].rule_id, "C018");
    }

    #[test]
    fn c018_does_NOT_fire_when_estimated_params_below_threshold() {
        // BUG REPRODUCTION: 1 param per row. With the bug present
        // (`|| is_insert_values`), this would still fire because the foreach
        // is inside INSERT VALUES. The fix removes that clause.
        let xml = br#"<mapper namespace="t">
            <insert id="batch">
                INSERT INTO t (a) VALUES
                <foreach collection="rows" item="r" separator=",">
                    (#{r.a})
                </foreach>
            </insert>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let mut config = LintConfig::default();
        config.max_insert_values_rows = usize::MAX; // 1000 << usize::MAX → must NOT fire
        let warnings = lint_structured_mapper(&mapper, &config);
        assert_eq!(warnings.len(), 0, "threshold is usize::MAX — single-param foreach must NOT fire C018");
    }

    #[test]
    fn c018_no_foreach_no_warning() {
        let xml = br#"<mapper namespace="t">
            <insert id="one">
                INSERT INTO t (a) VALUES (1)
            </insert>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let config = LintConfig::default();
        let warnings = lint_structured_mapper(&mapper, &config);
        assert!(warnings.is_empty());
    }
}
