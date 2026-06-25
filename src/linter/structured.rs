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
pub fn lint_structured_mapper(
    mapper: &StructuredMapper,
    config: &LintConfig,
) -> Vec<SqlWarning> {
    unimplemented!("implemented in Step 3")
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
        assert_eq!(
            warnings.len(),
            0,
            "threshold is usize::MAX — single-param foreach must NOT fire C018"
        );
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
