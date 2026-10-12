//! Structured-mapper lint rules — rules that need the pre-expansion `SqlNode`
//! tree (dynamic SQL), which the flat `SqlLinter::lint(&[StatementInfo])` API
//! cannot see.
//!
//! Currently houses rule C018 (foreach-in-INSERT-VALUES) and rule C020
//! (dynamic-SQL degenerate full-table DELETE/UPDATE).

use crate::ast::{Expr, Statement};
use crate::ibatis::types::{
    ExpandConfig, IfExpandStrategy, SqlNode, StatementKind, StructuredMapper, StructuredStatement,
};
use crate::linter::{Confidence, LintConfig, SqlWarning, WarningLevel};
use crate::parser::ParserError;

/// Lint a structured mapper for foreach-in-INSERT-VALUES (rule C018,
/// dynamic-SQL variant). Flat SQL collapses `<foreach>` to a single iteration,
/// so this must run on the `SqlNode` tree before expansion.
pub fn lint_structured_mapper(mapper: &StructuredMapper, config: &LintConfig) -> Vec<SqlWarning> {
    if !mapper.errors.is_empty() {
        return vec![];
    }
    let mut warnings = Vec::new();
    // 本模块现有规则（C018/C020）均为 Caution 级，与 flat 侧 `rule.level < min_level` 同口径
    if WarningLevel::Caution < config.min_level {
        return warnings;
    }

    for stmt in &mapper.statements {
        check_c020(stmt, config, &mut warnings);

        if let Some(foreach_node) = find_foreach_in_insert_values(&stmt.body) {
            let params_per_row = count_params_in_foreach_body(foreach_node);
            if params_per_row == 0 {
                continue;
            }
            let estimated_rows = config.foreach_estimated_rows;
            let total_params = params_per_row.saturating_mul(estimated_rows);

            // FIXED: no `|| is_insert_values` clause — the threshold is the sole trigger
            if total_params > config.max_insert_values_rows && !is_suppressed(config, "C018") {
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

/// structured 规则的 suppress 判定，口径与 flat 侧一致（`LintConfig::suppress` 精确匹配）。
fn is_suppressed(config: &LintConfig, rule_id: &str) -> bool {
    config.suppress.iter().any(|s| s == rule_id)
}

/// C020 `dynamic-delete-update-full-table`：枚举运行时条件组合（Both 策略、foreach
/// 零/单迭代），若存在某个组合使 DELETE/UPDATE 以"WHERE 缺失或无数据依赖谓词
/// （ColumnRef/Parameter）"的形态执行，即静默全表操作风险。解析失败的变体分两类、
/// 均不告警：缺 SET 的 UPDATE 与空 `<set>`（运行时必然 SQL 报错，响亮失败）、
/// foreach 零迭代的 `IN ()`（同理）。
fn check_c020(stmt: &StructuredStatement, config: &LintConfig, warnings: &mut Vec<SqlWarning>) {
    if !stmt.has_dynamic_elements {
        return;
    }
    if !matches!(stmt.kind, StatementKind::Delete | StatementKind::Update) {
        return;
    }
    let expand_config = ExpandConfig {
        if_strategy: IfExpandStrategy::Both,
        foreach_sizes: vec![0, 1],
        generate_parse_results: true,
        ..ExpandConfig::default()
    };
    let mut degenerate = false;
    for variant in stmt.expand_variants(&expand_config) {
        if variant.sql.trim().is_empty() {
            continue;
        }
        let Some((infos, errors)) = variant.parse_result else { continue };
        let has_fatal_error = errors
            .iter()
            .any(|e| !matches!(e, ParserError::Warning { .. } | ParserError::ReservedKeywordAsIdentifier { .. }));
        if has_fatal_error {
            continue;
        }
        for info in &infos {
            if !matches!(info.statement, Statement::Delete(_) | Statement::Update(_)) {
                continue;
            }
            let no_data_predicate = match crate::linter::extract_where(&info.statement) {
                None => true,
                Some(where_expr) => !expr_has_data_dependency(where_expr),
            };
            if no_data_predicate {
                degenerate = true;
                break;
            }
        }
        if degenerate {
            break;
        }
    }
    if degenerate && !is_suppressed(config, "C020") {
        warnings.push(SqlWarning {
            level: WarningLevel::Caution,
            rule_id: "C020".to_string(),
            rule_name: "dynamic-delete-update-full-table".to_string(),
            message: format!(
                "语句 {} 存在运行时条件组合，使 WHERE 退化为恒真或缺失\
                 （动态条件不命中而其余子句照常执行），将静默影响全表数据\
                 （DELETE 全删 / UPDATE 全改）。",
                stmt.id
            ),
            suggestion: Some(
                "在 WHERE 中保留至少一个非动态的数据依赖谓词（如主键条件）；\
                 若确属刻意的可选全表操作，请人工确认或经 .ogsql-lint.toml 抑制 C020"
                    .to_string(),
            ),
            location: crate::SourceLocation::default(),
            gaussdb_ref: None,
            confidence: Confidence::Partial,
        });
    }
}

/// WHERE 中是否存在数据依赖谓词：引用了任一 ColumnRef / Parameter 即实质谓词；
/// 纯常量谓词（`1=1` 等）不含列引用，天然归零。子查询类节点（Subquery/Exists/
/// InSubquery/ScalarSublink）保守视为实质谓词（漏报方向，安全）——walk_expr 不下钻
/// 其内部，故必须匹配容器节点本身；`NOT EXISTS` 经 UnaryOp 下钻后命中 Exists。
fn expr_has_data_dependency(expr: &Expr) -> bool {
    let mut found = false;
    crate::linter::walk_expr(expr, &mut |e| {
        if matches!(
            e,
            Expr::ColumnRef(_)
                | Expr::ColumnRefOuterJoin(_)
                | Expr::Parameter(_)
                | Expr::Subquery(_)
                | Expr::Exists(_)
                | Expr::InSubquery { .. }
                | Expr::ScalarSublink { .. }
        ) {
            found = true;
            return false;
        }
        true
    });
    found
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

    #[test]
    fn c020_fires_when_degenerate_where_is_tautology() {
        // 全部 <if> 不命中时退化为 DELETE ... WHERE 1=1 —— 静默全表删除。
        let xml = br#"<mapper namespace="t">
            <delete id="deleteByCondition">
                DELETE FROM users
                WHERE 1=1
                <if test="param.c1 != null">
                    AND c1 = #{param.c1}
                </if>
            </delete>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let warnings = lint_structured_mapper(&mapper, &LintConfig::default());
        let c020: Vec<_> = warnings.iter().filter(|w| w.rule_id == "C020").collect();
        assert_eq!(c020.len(), 1, "退化 WHERE 1=1 必须触发且仅触发一次 C020，实际: {warnings:?}");
        assert_eq!(c020[0].level, WarningLevel::Caution);
        assert_eq!(c020[0].rule_name, "dynamic-delete-update-full-table");
    }

    #[test]
    fn c020_fires_for_update_tautology() {
        let xml = br#"<mapper namespace="t">
            <update id="updateByCondition">
                UPDATE users SET name = #{name}
                WHERE 1=1
                <if test="param.status != null">
                    AND status = #{param.status}
                </if>
            </update>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let warnings = lint_structured_mapper(&mapper, &LintConfig::default());
        assert_eq!(warnings.iter().filter(|w| w.rule_id == "C020").count(), 1);
    }

    #[test]
    fn c020_fires_when_update_set_is_dynamic() {
        // 审核发现的典型部分更新 mapper：<set> 全部由 <if> 组成，WHERE 1=1 + 可选 <if>。
        // 全 <if> 剥除的退化形态 UPDATE t WHERE 1=1 解析失败（缺 SET），但运行时 SET 条件
        // 命中、WHERE 条件不命中 => UPDATE t SET col=? WHERE 1=1 —— 静默全表写。
        let xml = br#"<mapper namespace="t">
            <update id="updateByCondition">
                UPDATE users
                <set>
                    <if test="name != null">name = #{name},</if>
                    <if test="age != null">age = #{age},</if>
                </set>
                WHERE 1=1
                <if test="param.status != null">
                    AND status = #{param.status}
                </if>
            </update>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let warnings = lint_structured_mapper(&mapper, &LintConfig::default());
        assert_eq!(
            warnings.iter().filter(|w| w.rule_id == "C020").count(),
            1,
            "SET 条件命中而 WHERE 条件全不命中的组合必须触发 C020，实际: {warnings:?}"
        );
    }

    #[test]
    fn c020_silent_when_data_predicate_survives() {
        // 退化后 WHERE 仍含 id = #{id}（ColumnRef 判定）—— 安全，不告警
        let xml = br#"<mapper namespace="t">
            <delete id="deleteById">
                DELETE FROM users
                WHERE id = #{id}
                <if test="param.c1 != null">
                    AND c1 = #{param.c1}
                </if>
            </delete>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let warnings = lint_structured_mapper(&mapper, &LintConfig::default());
        assert!(
            warnings.iter().all(|w| w.rule_id != "C020"),
            "退化后存在数据依赖谓词时不得触发 C020，实际: {warnings:?}"
        );
    }

    #[test]
    fn c020_silent_when_where_has_exists_subquery() {
        // 关联 EXISTS 是数据依赖谓词（子查询引用外层表）；walk_expr 不下钻 Exists，
        // 但容器节点本身必须计为实质谓词，否则误报
        let xml = br#"<mapper namespace="t">
            <delete id="deleteByExists">
                DELETE FROM users
                WHERE EXISTS (SELECT 1 FROM orders o WHERE o.user_id = users.id)
                <if test="param.c1 != null">
                    AND c1 = #{param.c1}
                </if>
            </delete>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let warnings = lint_structured_mapper(&mapper, &LintConfig::default());
        assert!(
            warnings.iter().all(|w| w.rule_id != "C020"),
            "EXISTS 子查询是数据依赖谓词，不得触发 C020，实际: {warnings:?}"
        );
    }

    #[test]
    fn c020_silent_when_where_has_in_subquery() {
        // WHERE 1 IN (SELECT ...) / IN 子查询形态同样必须计为实质谓词
        let xml = br#"<mapper namespace="t">
            <delete id="deleteByInSub">
                DELETE FROM users
                WHERE 1 IN (SELECT 1 FROM orders WHERE orders.user_id = users.id)
                <if test="param.c1 != null">
                    AND c1 = #{param.c1}
                </if>
            </delete>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let warnings = lint_structured_mapper(&mapper, &LintConfig::default());
        assert!(warnings.iter().all(|w| w.rule_id != "C020"), "实际: {warnings:?}");
    }

    #[test]
    fn c020_silent_with_otherwise() {
        // otherwise 分支在运行时必然命中，退化形态仍带 id 谓词
        let xml = br#"<mapper namespace="t">
            <delete id="deleteByChoose">
                DELETE FROM users
                WHERE 1=1
                <choose>
                    <when test="param.x != null">
                        AND x = #{param.x}
                    </when>
                    <otherwise>
                        AND id = #{id}
                    </otherwise>
                </choose>
            </delete>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let warnings = lint_structured_mapper(&mapper, &LintConfig::default());
        assert!(
            warnings.iter().all(|w| w.rule_id != "C020"),
            "otherwise 提供数据谓词，不得触发 C020，实际: {warnings:?}"
        );
    }

    #[test]
    fn c020_respects_suppress() {
        // suppress 机制必须覆盖 structured 规则（与 flat 规则口径一致）
        let xml = br#"<mapper namespace="t">
            <delete id="deleteByCondition">
                DELETE FROM users
                WHERE 1=1
                <if test="param.c1 != null">
                    AND c1 = #{param.c1}
                </if>
            </delete>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let config = LintConfig { suppress: vec!["C020".to_string()], ..LintConfig::default() };
        let warnings = lint_structured_mapper(&mapper, &config);
        assert!(warnings.iter().all(|w| w.rule_id != "C020"), "suppress 后不得输出 C020，实际: {warnings:?}");
    }

    #[test]
    fn c018_respects_suppress() {
        // 同一过滤行为锁 C018（修复前 structured 告警不受 suppress 控制）
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
        config.max_insert_values_rows = 1000; // 5000 > 1000 → 未 suppress 时必触发
        let warnings = lint_structured_mapper(&mapper, &config);
        assert_eq!(warnings.iter().filter(|w| w.rule_id == "C018").count(), 1, "未 suppress 时应触发 C018");

        let config_suppressed = LintConfig { suppress: vec!["C018".to_string()], ..config };
        let warnings = lint_structured_mapper(&mapper, &config_suppressed);
        assert!(warnings.iter().all(|w| w.rule_id != "C018"), "suppress 后不得输出 C018，实际: {warnings:?}");
    }

    #[test]
    fn c020_respects_min_level() {
        // structured 告警必须与 flat 侧同口径过滤 min_level（Caution < Prohibition → 跳过）
        let xml = br#"<mapper namespace="t">
            <delete id="deleteByCondition">
                DELETE FROM users
                WHERE 1=1
                <if test="param.c1 != null">
                    AND c1 = #{param.c1}
                </if>
            </delete>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let config = LintConfig { min_level: WarningLevel::Prohibition, ..LintConfig::default() };
        let warnings = lint_structured_mapper(&mapper, &config);
        assert!(warnings.is_empty(), "min_level=Prohibition 时 Caution 级告警必须被过滤，实际: {warnings:?}");
    }

    #[test]
    fn c020_registered_in_rule_catalog() {
        // C020 必须出现在共享规则目录中（SARIF driver / 规则计数依赖其对齐）
        let entry = crate::linter::all_rules_metadata().into_iter().find(|r| r.id == "C020");
        let entry = entry.expect("C020 必须注册进 all_rules_metadata");
        assert_eq!(entry.name, "dynamic-delete-update-full-table");
        assert_eq!(entry.level, WarningLevel::Caution);
    }

    #[test]
    fn c020_silent_when_degenerate_parse_fails() {
        // foreach 零迭代退化为 IN ()，运行时必然 SQL 报错（响亮失败），不告警
        let xml = br#"<mapper namespace="t">
            <delete id="deleteByIds">
                DELETE FROM users
                WHERE 1=1
                AND id IN
                <foreach collection="ids" item="id" open="(" separator="," close=")">
                    #{id}
                </foreach>
            </delete>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let warnings = lint_structured_mapper(&mapper, &LintConfig::default());
        assert!(warnings.iter().all(|w| w.rule_id != "C020"), "实际: {warnings:?}");
    }

    #[test]
    fn c020_skips_static_statements() {
        // 静态无 WHERE 的 DELETE 由 C008 辖区负责，C020 不得重复告警
        let xml = br#"<mapper namespace="t">
            <delete id="deleteAll">
                DELETE FROM users
            </delete>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let warnings = lint_structured_mapper(&mapper, &LintConfig::default());
        assert!(warnings.iter().all(|w| w.rule_id != "C020"), "实际: {warnings:?}");
    }

    #[test]
    fn c020_ignores_select_statements() {
        let xml = br#"<mapper namespace="t">
            <select id="find">
                SELECT * FROM users
                WHERE 1=1
                <if test="param.c1 != null">
                    AND c1 = #{param.c1}
                </if>
            </select>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let warnings = lint_structured_mapper(&mapper, &LintConfig::default());
        assert!(warnings.iter().all(|w| w.rule_id != "C020"), "实际: {warnings:?}");
    }

    #[test]
    fn c020_fires_when_where_tag_pure_if() {
        // `<where>` 内只有 `<if>`：全部不命中时 where 标签整体裁剪，退化无 WHERE
        let xml = br#"<mapper namespace="t">
            <delete id="deleteByOptional">
                DELETE FROM users
                <where>
                    <if test="param.c1 != null">
                        AND c1 = #{param.c1}
                    </if>
                </where>
            </delete>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let warnings = lint_structured_mapper(&mapper, &LintConfig::default());
        assert_eq!(warnings.iter().filter(|w| w.rule_id == "C020").count(), 1, "实际: {warnings:?}");
    }

    #[test]
    fn c020_fires_on_choose_without_otherwise() {
        // 零分支命中变体（Loop 1a 引入）下 WHERE 退化为 1=1
        let xml = br#"<mapper namespace="t">
            <delete id="deleteByChoose">
                DELETE FROM users
                WHERE 1=1
                <choose>
                    <when test="param.x != null">
                        AND x = #{param.x}
                    </when>
                </choose>
            </delete>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let warnings = lint_structured_mapper(&mapper, &LintConfig::default());
        assert_eq!(warnings.iter().filter(|w| w.rule_id == "C020").count(), 1, "实际: {warnings:?}");
    }

    #[test]
    fn c020_fires_on_nested_choose_degeneration() {
        // 外层 when 命中而内层 choose（无 otherwise）零命中 => WHERE 1=1。
        // 外层 otherwise 虽安全，但运行时组合存在全表删除路径。
        let xml = br#"<mapper namespace="t">
            <delete id="deleteByNestedChoose">
                DELETE FROM users
                WHERE 1=1
                <choose>
                    <when test="a != null">
                        <choose>
                            <when test="b != null">
                                AND b = #{b}
                            </when>
                        </choose>
                    </when>
                    <otherwise>
                        AND id = #{id}
                    </otherwise>
                </choose>
            </delete>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let warnings = lint_structured_mapper(&mapper, &LintConfig::default());
        assert_eq!(
            warnings.iter().filter(|w| w.rule_id == "C020").count(),
            1,
            "嵌套 choose 内层零命中组合必须触发 C020，实际: {warnings:?}"
        );
    }

    #[test]
    fn c020_fires_with_raw_interpolation_if() {
        // issue #335 原始样例：${param.c1} 是字符串拼接而非参数化
        let xml = br#"<mapper namespace="t">
            <delete id="deleteByCondition">
                DELETE FROM users
                WHERE 1=1
                <if test="param.c1 != null">
                    AND c1=${param.c1}
                </if>
            </delete>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let warnings = lint_structured_mapper(&mapper, &LintConfig::default());
        assert_eq!(warnings.iter().filter(|w| w.rule_id == "C020").count(), 1);
    }
}
