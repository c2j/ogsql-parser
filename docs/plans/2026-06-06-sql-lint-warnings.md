# SQL Lint / Warning System Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 在 parse / parse-java / parse-xml / validate 四个入口中增加 SQL 反模式检测能力，通过分级 Warning 揭示 GaussDB 文档中记载的危险、低效、有隐患的 SQL 写法。

**Architecture:** 新增 `src/linter/` 模块，定义 `SqlWarning` 类型和 `LintRule` trait。每条检测规则实现为一个独立的 `fn check()` 函数（或 Visitor），由 `SqlLinter` 统一调度。所有规则基于已有的 AST + Visitor 基础设施，无需修改解析器核心。通过 CLI `--lint` flag 激活，结果汇入 JSON / CSV / 终端输出。

**Tech Stack:** Rust, 已有 AST (`src/ast/`), 已有 Visitor (`src/ast/visitor.rs`), 已有 Hint 验证 (`src/parser/hint_validator.rs`), 已有函数注册表 (`src/parser/function_registry.rs`), 已有 Schema 加载 (`src/analyzer/schema.rs`)

---

## Background / 背景说明

### 问题

GaussDB 2.23.07.210 产品文档（华为云Stack 8.3.0）中明确记载了大量 SQL "规则"（强制禁止）和"建议"（应优化），涵盖：

- 数据安全风险：`LOCK TABLE` 死锁、`DROP ... CASCADE` 误删、隐式类型转换、unlogged table 数据丢失
- 性能陷阱：`SELECT *`、`UNION`（非 ALL）、`NOT IN`、标量子查询、`now()` 不可下推、`LIKE '%...'`
- 低效语法：`UPDATE SET (c1,c2) = (SELECT ...)`、关联子查询、`count(*)` 大表
- Hint 隐患：未知 Hint 被静默忽略、矛盾 Hint 导致预期外执行计划

开发者在编写 SQL 时往往意识不到这些隐患。现有 parser 只做语法检查，不检测语义级别的反模式。

### 现有基础设施

| 组件 | 位置 | 状态 |
|---|---|---|
| AST（180+ 语句类型，完整表达式树） | `src/ast/mod.rs` | ✅ 完整 |
| Visitor 模式（11 个 hook，完整遍历） | `src/ast/visitor.rs` | ✅ 完整 |
| Hint 验证（69 个已知 Hint，5 种验证） | `src/parser/hint_validator.rs` | ✅ 已有，产 `ParserError::Warning` |
| 函数注册表（449 内置函数，参数/类型验证） | `src/parser/function_registry.rs` | ✅ 已有 |
| Schema 加载（JSON → SchemaMap） | `src/analyzer/schema.rs` | ✅ 已有 |
| `ParserError::Warning` | `src/parser/mod.rs:22` | ✅ 已有 |
| validate 命令管道 | `src/bin/ogsql.rs:3264` | ✅ 已有 |

### 设计原则

1. **不破坏现有 API** — `ParseOutput.errors` 语义不变，warnings 作为新字段加入
2. **每条规则独立可测** — 单个函数，纯输入输出，无全局状态
3. **可选启用** — `--lint` flag 控制，不影响默认 parse 行为
4. **分级清晰** — 不使用 "Error"（留给语法错误），用 Prohibition/Performance/Caution/Suggestion 四级
5. **复用已有验证** — Hint 验证结果转为 `SqlWarning`，不重写

---

## Warning Levels / 级别定义

```rust
/// SQL 警告严重级别。
/// 注意：不使用 "Error"，因为该词在 ParserError 中已用于语法级错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum WarningLevel {
    /// 🔴 强烈禁止 — 违反 GaussDB 文档"规则"级要求，可能导致数据安全风险/死锁/数据丢失。
    ///   例: LOCK TABLE、隐式类型转换、DROP CASCADE
    Prohibition = 3,

    /// 🟡 性能警告 — 存在可识别的性能隐患，有明确的优化方案。
    ///   例: UNION → UNION ALL、NOT IN → NOT EXISTS、SELECT *
    Performance = 2,

    /// 🔵 风险提示 — 语法合法但容易被忽视的隐患，需结合场景判断。
    ///   例: Hint 被静默忽略、低效 GaussDB 专用语法、UPDATE 无 WHERE
    Caution = 1,

    /// ⚪ 编码建议 — 不影响正确性，遵循后可提高可维护性和健壮性。
    ///   例: 用 TRUNCATE 替代 DELETE 全表、LIMIT 1 无 ORDER BY
    Suggestion = 0,
}
```

级别支持排序：`Prohibition > Performance > Caution > Suggestion`。CLI `--min-level` 过滤基于此排序。

---

## Warning Output Type / 输出类型

```rust
/// 检测结果的可信度
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Confidence {
    /// 完整 SQL — 检测结果完全可信
    Full,
    /// iBatis/MyBatis 动态 SQL 片段 — 可能因 <if>/<choose> 导致 SQL 不完整，存在误报可能
    Partial,
}

/// 单条 SQL 警告。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SqlWarning {
    /// 严重级别
    pub level: WarningLevel,
    /// 规则编号，如 "R001"、"P002"
    pub rule_id: String,
    /// 人类可读的规则名称
    pub rule_name: String,
    /// 警告描述（含具体 SQL 上下文）
    pub message: String,
    /// 优化建议（可选）
    pub suggestion: Option<String>,
    /// 源码位置
    pub location: SourceLocation,
    /// GaussDB 文档引用（可选）
    pub gaussdb_ref: Option<String>,
    /// 检测可信度
    pub confidence: Confidence,
}
```

- `parse` / `validate` 产生的 Warning: `Confidence::Full`
- `parse-xml` 提取的动态 SQL 产生的 Warning: `Confidence::Partial`
- `parse-java` 拼接的 SQL 产生的 Warning: `Confidence::Partial`
- JSON 输出中始终包含 confidence 字段
- 终端输出中 Partial 级别标注 `[partial]` 标记
- 可通过 `--min-confidence full` 过滤掉 Partial 结果

---

## Complete Rule Catalog / 完整规则清单

### A. Prohibition（🔴 强烈禁止）— GaussDB 文档"规则"级

共 9 条，8 条纯 AST 可检。

| ID | 规则名 | AST 检测点 | GaussDB 文档 | 需 Schema |
|---|---|---|---|---|
| R001 | select-star | `SelectTarget::Star(None)` — 仅无表限定的 `*`，`t.*` 不触发 | SELECT 规范 | 否 |
| R002 | large-column-sort | SELECT 的 group_by / order_by / distinct / union（非 all）涉及表达式 — 仅提示存在性 | SELECT 规范 | 是(列大小) |
| R003 | lock-table | `Statement::Lock(_)` | SELECT 规范 | 否 |
| R004 | drop-cascade | `DropStatement { cascade: true, .. }` | SQL 编写 | 否 |
| R005 | implicit-type-conversion | WHERE 中类型不一致（需 schema 辅助，暂降级为 Caution） | WHERE 规范 | 是 |
| R006 | function-on-where-column | WHERE 中 `Expr::FunctionCall` 直接包装 `ColumnRef` 作为过滤条件 | WHERE 规范 | 否 |
| R007 | like-leading-wildcard | `Expr::Like { pattern: Literal::String(s) if s.starts_with('%'), .. }` | WHERE 规范 | 否 |
| R008 | same-table-column-compare | WHERE 中 `Expr::BinaryOp` 两边均为 `ColumnRef` 且前缀相同 | WHERE 规范 | 否 |
| R009 | scalar-subquery-in-select | `Expr::Subquery` 出现在 `SelectTarget::Expr` 中 | SQL 编写 | 否 |

### B. Performance（🟡 性能警告）— 有明确优化方案

共 22 条，18 条纯 AST 可检。

| ID | 规则名 | AST 检测点 | 优化建议 | 需 Schema |
|---|---|---|---|---|
| P001 | union-without-all | `SetOperation::Union { all: false, .. }` | 改用 UNION ALL | 否 |
| P002 | not-in-subquery | `Expr::InSubquery { negated: true, .. }` 及匹配的 `Expr::ScalarSublink` 变体 | 改为 NOT EXISTS | 否 |
| P003 | in-list-too-large | `Expr::InList { list, .. }` where `list.len() > threshold` | 改用 INNER JOIN | 否 |
| P004 | or-to-union-all | WHERE 中 `Expr::BinaryOp { op: "OR", .. }` 作为顶层条件 | 纯 OR 改写为 UNION ALL | 否 |
| P005 | now-function-non-pushable | `Expr::FunctionCall { name: ["now"], .. }` | 用时间宏替代 | 否 |
| P006 | count-star-large-table | `Expr::FunctionCall { name: ["count"], args: [Star], .. }` | 用 pg_class.reltuples | 否 |
| P007 | too-many-non-equi-joins | 非等值 `BinaryOp` 数量 > threshold | 优先使用等值查询 | 否 |
| P008 | group-by-without-hashagg | `group_by` 非空，无 use_hash_agg hint | 考虑调大 work_mem | 否 |
| P009 | function-instead-of-case | WHERE 中 `FunctionCall` 可替换为 `Case` | 用 CASE 表达式替代 | 否 |
| P010 | multi-column-update-subquery | `UpdateAssignment { columns: len>1, value: Subquery }` | 改为 UPDATE FROM JOIN | 否 |
| P011 | correlated-subquery | `Expr::InSubquery` / `Expr::ScalarSublink` / `Expr::Subquery` 中引用外表列（需要遍历子查询的 ColumnRef 并与外层 FROM 表对比） | 改写为等值 JOIN | 否 |
| P012 | unnecessary-distinct | `distinct: true` 且 SELECT 含唯一列 | 检查去重必要性 | 是 |
| P013 | cartesian-product | `TableRef::Join { condition: None, .. }` 或多 FROM 无 JOIN | 补充 JOIN 条件 | 否 |
| P014 | deeply-nested-subquery | `Expr::Subquery` 嵌套深度 > threshold | 拆分为临时表 | 否 |
| P015 | range-equals-same-value | WHERE 中同一列 `>= val AND <= val` 且值相同 | 简化为 `=` | 否 |
| P016 | update-from-no-join-condition | `UpdateStatement { from, where_clause: None/无关联 }` | WHERE 中关联 FROM 表 | 否 |
| P017 | merge-without-unique-index | `MergeStatement { on_condition }` | 确保 ON 有唯一索引 | 是 |
| P018 | insert-select-no-columns | `InsertStatement { columns: [], source: Select }` | 指定目标列名 | 否 |
| P019 | multi-table-update | `UpdateStatement { tables: len>1 }` | 拆分为多条单表 UPDATE | 否 |
| P020 | insert-all-multi-table | `Statement::InsertAll` / `Statement::InsertFirst` | 评估是否可用单条 INSERT...SELECT | 否 |
| P021 | row-by-row-insert-in-loop | 循环体内含 INSERT 的 `PlStatement::SqlStatement` (Phase 2) | 使用 FORALL 批量操作 | 否 |
| P022 | explain-in-production | `Statement::Explain` | 不应出现在生产代码中 | 否 |

### C. Caution（🔵 风险提示）— 合法但易忽视

共 16 条，14 条纯 AST 可检。

| ID | 规则名 | AST 检测点 | 说明 | 需 Schema |
|---|---|---|---|---|
| C001 | hint-unknown | hints 不在 `KNOWN_HINTS` | **已有 hint_validator.rs** | 否 |
| C002 | hint-invalid-negation | hint 不支持 no 前缀却使用了 | **已有 hint_validator.rs** | 否 |
| C003 | hint-missing-args | 需要括号参数的 hint 缺失参数 | **已有 hint_validator.rs** | 否 |
| C004 | hint-extra-args | 不需要括号的 hint 给了参数 | **已有 hint_validator.rs** | 否 |
| C005 | hint-contradictory | 同语句中矛盾 hint（同表+跨表全量检测） | **需新增** | 否 |
| C006 | hint-table-not-in-from | hint 引用的表不在 FROM 子句中 | **需新增** | 否 |
| C007 | update-without-where | `UpdateStatement.where_clause.is_none()` | 可能是全表更新 | 否 |
| C008 | delete-without-where | `DeleteStatement.where_clause.is_none()` | 可能是全表删除 | 否 |
| C009 | insert-no-column-list | `InsertStatement.columns.is_empty()` | 依赖列顺序 | 否 |
| C010 | unlogged-table | `CreateTableStatement.unlogged == true` | 数据安全风险 | 否 |
| C011 | goto-statement | `PlStatement::Goto { .. }` | 结构化编程不推荐 | 否 |
| C012 | execute-concat-sql-injection | `PlStatement::Execute` 中 string_expr 含 `||` 拼接 | SQL 注入风险 | 否 |
| C013 | exception-swallow | WHEN OTHERS THEN 无 RAISE | 静默吞错 | 否 |
| C014 | pl-commit-rollback | PL 块中 `Commit` / `Rollback` | 事务控制复杂化 | 否 |
| C015 | select-for-update-blocking | `LockClause::Update { nowait: false, skip_locked: false, wait: None }` | 可能长时间阻塞 | 否 |
| C016 | autonomous-transaction | `Pragma { name: "AUTONOMOUS_TRANSACTION" }` | 性能开销大 | 否 |

### D. Suggestion（⚪ 编码建议）

共 8 条，5 条纯 AST 可检。

| ID | 规则名 | AST 检测点 | 建议 | 需 Schema |
|---|---|---|---|---|
| S001 | delete-full-table-use-truncate | `DeleteStatement.where_clause.is_none()` | 用 TRUNCATE 释放空间 | 否 |
| S002 | limit-offset-use-cursor | `SelectStatement.offset.is_some()` | 用游标翻页 | 否 |
| S003 | index-column-too-wide | VARCHAR(n) n > 50 的列出现在索引中 | 控制索引列长度 | 是 |
| S004 | analyze-after-bulk-insert | 大量 INSERT 后无 ANALYZE（需上下文） | 防止统计信息不准 | 是 |
| S005 | prefer-percent-type | `PlDataType::TypeName` 而非 `PercentType` | 用 %TYPE 锚定类型 | 否 |
| S006 | limit-without-order-by | `limit` 存在但 `order_by` 为空 | 结果不确定 | 否 |
| S007 | explicit-type-for-literals | WHERE 中 `Literal` 无 `TypeCast` | 避免隐式转换 | 否 |
| S008 | complex-sql-consider-split | SQL 文本长度 > threshold 或 子查询数 > threshold | 拆分简化 | 否 |

### 汇总

| 级别 | 数量 | 纯 AST | 需 Schema |
|---|---|---|---|
| Prohibition | 9 | 8 | 1 |
| Performance | 22 | 18 | 4 |
| Caution | 16 | 14 | 2 |
| Suggestion | 8 | 5 | 3 |
| **Total** | **55** | **45** | **10** |

---

## Module Structure / 模块结构

```
src/linter/
├── mod.rs                    # SqlWarning, WarningLevel, Confidence, LintRule trait, SqlLinter orchestrator
├── rules_prohibition.rs      # R001-R009
├── rules_performance.rs      # P001-P022
├── rules_caution.rs          # C001-C016 (含 Hint 增强规则)
├── rules_suggestion.rs       # S001-S008
└── tests.rs                  # 所有规则的单元测试
```

### SqlLinter 调度器

```rust
pub struct SqlLinter {
    rules: Vec<LintRuleEntry>,
    config: LinterConfig,
}

pub struct LinterConfig {
    pub min_level: WarningLevel,      // 最低报告级别
    pub min_confidence: Confidence,   // 最低可信度 (Full/Partial)
    pub suppress: Vec<String>,        // 禁用的规则 ID 列表
    pub in_list_threshold: usize,     // P003 阈值，默认 500
    pub subquery_depth_limit: usize,  // P014 深度阈值，默认 3
    pub sql_length_limit: usize,      // S008 SQL 长度阈值，默认 2000
    pub non_equi_join_limit: usize,   // P007 非等值操作上限，默认 2
}

pub struct LintRuleEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub level: WarningLevel,
    /// 对应的语句类型（None = 对所有语句执行）
    pub stmt_kind: Option<StatementKind>,
    pub check_fn: fn(&StatementInfo, Option<&SchemaMap>, &LinterConfig, Confidence, &mut Vec<SqlWarning>),
}

/// 语句类型枚举，用于 SqlLinter 一级分发
/// 避免每条规则内部重复 match Statement
pub enum StatementKind {
    Select, Update, Delete, Insert, Merge, DmlAll, HintOnly,
    PlBlock, Dml, Ddl, All,
}
```

### SqlLinter 一级分发策略

每条规则注册时声明 `stmt_kind`。`SqlLinter.lint()` 遍历语句时，先按 `StatementKind` 分发到匹配的规则组，再将当前 `StatementInfo` 传入规则函数。这样规则函数内部只需直接解构自己关心的子类型，无需写 match 分发：

```rust
impl SqlLinter {
    pub fn lint(&self, stmts: &[StatementInfo], schema: Option<&SchemaMap>, confidence: Confidence) -> Vec<SqlWarning> {
        let mut warnings = Vec::new();
        for info in stmts {
            let kind = classify_statement(&info.statement);
            for rule in &self.rules {
                if let Some(ref rule_kind) = rule.stmt_kind {
                    if !rule_kind.matches(kind) { continue; }
                }
                if self.config.suppress.contains(&rule.id) { continue; }
                (rule.check_fn)(info, schema, &self.config, confidence, &mut warnings);
            }
        }
        // 聚合、排序、按级别过滤...
        warnings
    }
}
```

规则检测有三种实现方式：

1. **直接模式匹配**（最简单）— 直接 match AST 节点，如 R003 `Statement::Lock`
2. **Visitor 遍历**（中等）— 实现 `Visitor` trait，遍历表达式树收集模式，如 R007 搜索所有 `Expr::Like`
3. **上下文感知**（较复杂）— 需要理解语句结构上下文，如 R009 需区分"SELECT 列表中的子查询"和"WHERE 中的子查询"

---

## Integration Points / 集成点

### 1. ParseOutput 扩展

```rust
// src/parser/mod.rs
pub struct ParseOutput {
    pub statements: Vec<crate::ast::StatementInfo>,
    pub errors: Vec<ParserError>,
    pub comments: Vec<CommentInfo>,
    // 新增 — 仅当调用方请求 lint 时填充
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<crate::linter::SqlWarning>,
}
```

### 2. CLI 入口

```
ogsql parse       --lint [--min-level <level>] [--min-confidence <level>] [--suppress <ids>] [--lint-config <path>]
ogsql validate     --lint [--min-level <level>] [--min-confidence <level>] [--suppress <ids>] [--lint-config <path>]
ogsql parse-xml    --lint [--min-level <level>] [--min-confidence <level>] [--suppress <ids>] [--lint-config <path>]
ogsql parse-java   --lint [--min-level <level>] [--min-confidence <level>] [--suppress <ids>] [--lint-config <path>]
```

阈值 CLI 参数（优先级高于配置文件）：

```
--in-list-threshold <N>         P003 IN 列表上限 (默认 500)
--subquery-depth-limit <N>      P014 子查询嵌套深度 (默认 3)
--sql-length-limit <N>          S008 SQL 长度上限 (默认 2000)
--non-equi-join-limit <N>       P007 非等值操作上限 (默认 2)
```

### 3. Warning 与 ParserError::Warning 的关系

两套体系并存，各自服务于不同目的：

| 体系 | 类型 | 来源 | 语义 |
|---|---|---|---|
| 语法级 | `ParserError::Warning` | hint_validator.rs, function_registry.rs | 解析阶段的语法/语义提示 |
| 语义级 | `SqlWarning` (新) | linter 模块 | AST 级别的反模式/性能/风险检测 |

- `ParseOutput.errors` 不变，继续包含 `ParserError::Warning`
- `ParseOutput.warnings` (新) 包含 `SqlWarning`
- 两者在 JSON 输出中分别呈现为 `errors` 和 `warnings` 数组
- 两者在终端输出中使用不同的前缀标记：`[W-语法]` vs `[W-P001]`
- hint_validator 的结果未来可渐进式迁移到 SqlWarning（不阻塞本次实现）

### 4. 多语句 Warning 归属 — 按语句聚合

JSON 输出格式：
```json
{
  "statements": [
    {
      "sql_text": "SELECT * FROM t1 UNION SELECT * FROM t2",
      "statement": { "..." : "..." },
      "warnings": [
        { "rule_id": "R001", "level": "prohibition", "confidence": "full", "message": "..." },
        { "rule_id": "P001", "level": "performance", "confidence": "full", "message": "..." }
      ]
    }
  ],
  "summary": {
    "total_warnings": 5,
    "by_level": { "prohibition": 1, "performance": 2, "caution": 1, "suggestion": 1 },
    "by_rule": { "R001": 1, "P001": 2, "C007": 1, "S006": 1 }
  }
}
```

终端输出格式：
```
── Statement 1 (line 1) ──
  🔴 R001  SELECT * 违反 GaussDB 编码规范
          → 明确列出所需字段名
  🟡 P001  UNION 未使用 ALL，存在不必要的去重排序
          → 如果确认无重叠，改用 UNION ALL

── Statement 2 (line 5) ──
  🔵 C007  UPDATE 无 WHERE 子句，可能影响全表数据

── Summary ──
  Prohibition: 1 | Performance: 1 | Caution: 1 | Suggestion: 0
  Total: 3 warnings
```

### 5. 配置文件

查找优先级（高到低）：

1. `--lint-config <path>` — CLI 显式指定
2. `.ogsql-lint.toml` — 当前工作目录
3. `~/.config/ogsql/lint.toml` — 用户全局配置（XDG 标准）
4. 内置默认值

找到配置文件后与 CLI 参数合并，CLI 参数覆盖配置文件中的同名项。

```toml
# .ogsql-lint.toml
min_level = "performance"
min_confidence = "full"
suppress = ["R001", "S006"]

[thresholds]
in_list = 500
subquery_depth = 3
sql_length = 2000
non_equi_join = 2
```

### 6. MCP / HTTP API

- `validate` 工具/端点：新增 `lint: bool`、`min_level: string`、`min_confidence: string` 参数
- 返回 JSON 中新增 `warnings: SqlWarning[]` 字段
- 按语句聚合输出

---

## Design Decisions / 设计决策记录

以下 7 项已确认。

### D1: 规则阈值参数化 — CLI 参数 + 配置文件双重支持

阈值参数同时暴露为 CLI 参数和配置文件字段。优先级链：

`CLI --in-list-threshold 300` > `.ogsql-lint.toml` 的 `thresholds.in_list` > 代码内置默认值 (500)

涉及的阈值参数：

| 参数 | CLI flag | 配置文件字段 | 默认值 | 影响规则 |
|---|---|---|---|---|
| IN 列表上限 | `--in-list-threshold` | `thresholds.in_list` | 500 | P003 |
| 子查询嵌套深度 | `--subquery-depth-limit` | `thresholds.subquery_depth` | 3 | P014 |
| SQL 长度上限 | `--sql-length-limit` | `thresholds.sql_length` | 2000 | S008 |
| 非等值操作数上限 | `--non-equi-join-limit` | `thresholds.non_equi_join` | 2 | P007 |

### D2: iBatis 动态 SQL — 增加 confidence 字段

`SqlWarning` 结构体包含 `confidence: Confidence` 字段（`Full` / `Partial`）。parse-xml 和 parse-java 产出的 Warning 标记为 `Partial`，parse 和 validate 产出的标记为 `Full`。支持 `--min-confidence full` 过滤。

### D3: Hint 矛盾检测 — 全量检测（同表 + 跨表）

同时检测以下五类矛盾：

| 矛盾类型 | 示例 | 严重性 |
|---|---|---|
| 同表正反矛盾 | `hashjoin(t1)` + `nohashjoin(t1)` | 高 |
| 同表互斥扫描方式 | `tablescan(t1)` + `indexscan(t1)` | 高 |
| 同表多种 JOIN 方式 | `hashjoin(t1)` + `nestloop(t1)` | 中 |
| 跨表 leading 冲突 | `leading(t1 t2)` + `leading(t2 t1)` | 中 |
| Hint 与 GUC 矛盾 | `set(enable_hashjoin off)` + `hashjoin(t1)` | 低 |

实现时复用 `hint_validator.rs` 的 `parse_hint_list()` 解析结果。

### D4: Warning 与 ParserError::Warning — 并存

两套体系并存。语法级 `ParserError::Warning` 由 hint_validator/function_registry 产出，语义级 `SqlWarning` 由 linter 产出。分别位于 `ParseOutput.errors` 和 `ParseOutput.warnings`。

### D5: 多语句 Warning 归属 — 按语句聚合

Warning 按语句分组聚合展示。JSON 中每条语句的 warnings 为数组。终端输出按语句分块显示。末尾附 summary 统计。

### D6: 配置文件路径 — 默认程序内路径 + `--lint-config` 优先

配置文件查找顺序：`--lint-config <path>` > `.ogsql-lint.toml` (CWD) > `~/.config/ogsql/lint.toml` (XDG) > 内置默认值。

### D7: PL/pgSQL 循环内 INSERT 检测 (P021) — 作为 Phase 2 高级规则

P021 归入 Phase 2。需要在 Visitor 遍历 PL 块时维护"是否在循环体内"的上下文栈，复杂度高于简单的 AST 模式匹配。

---

## Implementation Phases / 实施阶段

### Phase 1: 核心框架 + 纯 AST 的 Prohibition/Performance 规则

**范围**: 框架搭建 + 31 条规则（Prohibition R001-R009 + Performance P001-P022，其中 26 条纯 AST 可检、5 条需 Schema 暂降级为纯 AST 有限检测）

**产出**:
- `src/linter/mod.rs` — 核心类型（`SqlWarning`、`WarningLevel`、`Confidence`、`LintRuleEntry`、`SqlLinter`）+ 调度器
- `src/linter/rules_prohibition.rs` — R001-R009
- `src/linter/rules_performance.rs` — P001-P022
- `src/lib.rs` — 新增 `pub mod linter;`
- `src/bin/ogsql.rs` — `--lint` / `--min-level` / `--min-confidence` / `--suppress` / 阈值 CLI flags 集成到 validate + parse
- `src/linter/tests.rs` — 每条规则的单元测试

**预估**: 5-7 天

### Phase 2: Caution/Suggestion 规则 + Hint 增强 + parse-xml/parse-java

**范围**: C001-C016（含 Hint 矛盾检测）+ S001-S008 + parse-xml/parse-java `--lint` 集成

**产出**:
- `src/linter/rules_caution.rs` — C001-C016
- `src/linter/rules_suggestion.rs` — S001-S008
- `src/bin/ogsql.rs` — parse-xml / parse-java 的 `--lint` 集成
- Hint 矛盾检测（C005）和表名存在性检测（C006）
- P021 循环内 INSERT 检测

**预估**: 3-4 天

### Phase 3: Schema 依赖规则 + 配置文件 + MCP/HTTP API

**范围**: 10 条需 Schema 的规则 + `.ogsql-lint.toml` 配置文件 + API 集成

**产出**:
- Schema 辅助检测（R005 隐式转换、P012 不必要 DISTINCT、P017 MERGE 索引等）
- 配置文件查找、加载、合并逻辑
- MCP `validate` 工具更新
- HTTP `/api/validate` 更新

**预估**: 2-3 天

---

## Testing Strategy / 测试策略

### 单元测试

每条规则独立的测试用例，覆盖三种状态：

```rust
#[test]
fn r001_select_star_detected() {
    let stmts = parse("SELECT * FROM t1");
    let warnings = lint(&stmts, None, &default_config());
    assert!(warnings.iter().any(|w| w.rule_id == "R001"));
}

#[test]
fn r001_table_qualified_star_not_detected() {
    let stmts = parse("SELECT t1.* FROM t1");
    let warnings = lint(&stmts, None, &default_config());
    assert!(!warnings.iter().any(|w| w.rule_id == "R001"));
}

#[test]
fn r001_no_warning_on_explicit_cols() {
    let stmts = parse("SELECT id, name FROM t1");
    let warnings = lint(&stmts, None, &default_config());
    assert!(!warnings.iter().any(|w| w.rule_id == "R001"));
}
```

### 集成测试

在每个 Phase 结束时，使用项目中已有的回归测试 SQL 文件进行端到端测试：

- 从 `testcases/` 或 `lib/openGauss-server/src/test/regress/sql/` 选取代表性 SQL 文件
- 以 `--lint` 模式运行 `validate`，验证：
  1. 不会 panic 或崩溃
  2. 不会产生假阳性报警（纯语法正确的 SQL 不应触发 Prohibition 级警告）
  3. 已知反模式用例确实被检出
- 如果引入 regression 测试中的 SQL 触发大量 Warning（如 `SELECT *` 在测试用例中广泛存在），需评估是否需要调整规则敏感度或降低这些规则的初始级别

### 多规则交互测试

```rust
#[test]
fn multiple_rules_same_statement() {
    // SELECT * + UNION (非 ALL) — 应同时触发 R001 + P001
    let stmts = parse("SELECT * FROM t1 UNION SELECT * FROM t2");
    let warnings = lint(&stmts, None, &default_config());
    assert_eq!(warnings.len(), 2);
}
```

### Confidence 传播测试

```rust
#[test]
fn confidence_partial_via_xml_source() {
    // 模拟 parse-xml 场景：confidence = Partial
    let stmts = parse("SELECT * FROM t WHERE id = #{id}");
    let warnings = lint_with_confidence(&stmts, Confidence::Partial, &default_config());
    assert!(warnings.iter().all(|w| w.confidence == Confidence::Partial));
}
```

---

## Rule Detection Patterns (Key Examples) / 关键规则检测示例

### R001: SELECT *

```rust
fn check_select_star(select: &SelectStatement, warnings: &mut Vec<SqlWarning>) {
    for target in &select.targets {
        // SelectTarget::Star(Some(table)) = t.* — 限定表的星号不触发
        // SelectTarget::Star(None) = * — 无表限定的星号才触发
        if matches!(target, SelectTarget::Star(None)) {
            warnings.push(SqlWarning {
                level: WarningLevel::Prohibition,
                rule_id: "R001".into(),
                rule_name: "select-star".into(),
                message: "SELECT * 违反 GaussDB 编码规范：表结构变化时可能导致不兼容".into(),
                suggestion: Some("明确列出所需字段名".into()),
                location: /* from span */,
                gaussdb_ref: Some("开发设计建议 > SELECT 规范".into()),
                confidence: Confidence::Full,
            });
        }
    }
}
```

### P010: Multi-column UPDATE from subquery

```rust
fn check_multi_col_update_subquery(update: &UpdateStatement, warnings: &mut Vec<SqlWarning>) {
    for assignment in &update.assignments {
        if assignment.columns.len() > 1 {
            if matches!(&assignment.value, Expr::Subquery(_)) {
                warnings.push(SqlWarning {
                    level: WarningLevel::Performance,
                    rule_id: "P010".into(),
                    rule_name: "multi-column-update-subquery".into(),
                    message: format!(
                        "UPDATE SET ({}列) = (SELECT ...) 效率较低",
                        assignment.columns.len()
                    ),
                    suggestion: Some("改用 UPDATE ... FROM ... WHERE ... 的 JOIN 风格".into()),
                    location: /* from span */,
                    gaussdb_ref: None,
                    confidence: Confidence::Full,
                });
            }
        }
    }
}
```

### C005: Contradictory hints

```rust
fn check_hint_contradictions(hints: &[String], warnings: &mut Vec<SqlWarning>) {
    // 复用 hint_validator.rs 的 parse_hint_list() 解析
    let parsed = parse_hint_list_for_conflicts(hints);

    // 检测五类矛盾:
    // 1. 同表正反: hashjoin(t1) + nohashjoin(t1)
    // 2. 同表互斥扫描: tablescan(t1) + indexscan(t1)
    // 3. 同表多种 JOIN: hashjoin(t1) + nestloop(t1)
    // 4. 跨表 leading 冲突: leading(t1 t2) + leading(t2 t1)
    // 5. Hint 与 GUC 矛盾: set(enable_hashjoin off) + hashjoin(t1)
    // ...
}
```

### C007: UPDATE without WHERE

```rust
fn check_update_without_where(update: &UpdateStatement, warnings: &mut Vec<SqlWarning>) {
    if update.where_clause.is_none() {
        warnings.push(SqlWarning {
            level: WarningLevel::Caution,
            rule_id: "C007".into(),
            rule_name: "update-without-where".into(),
            message: "UPDATE 无 WHERE 子句，可能影响全表数据".into(),
            suggestion: Some("确认是否需要全表更新；若需清空，考虑 TRUNCATE".into()),
            location: /* from span */,
            gaussdb_ref: None,
            confidence: Confidence::Full,
        });
    }
}
```

---

## References / 参考

- GaussDB 2.23.07.210 文档: `GaussDB-2.23.07.210/term/`
- GaussDB SQL 查询最佳实践（分布式）: https://support.huaweicloud.com/bestpractice-gaussdb/gaussdb-22-0013.html
- GaussDB SQL 编写规范: https://support.huaweicloud.com/intl/zh-cn/distributed-devg-v2-gaussdb/gaussdb-12-0052.html
- GaussDB SELECT 规范: https://support.huaweicloud.com/centralized-devg-v8-gaussdb/gaussdb-42-2082.html
- GaussDB WHERE 规范: https://support.huaweicloud.com/intl/zh-cn/distributed-devg-v2-gaussdb/gaussdb-12-1204.html
- GaussDB SQL 语句改写规则: https://support.huaweicloud.com/intl/zh-cn/distributed-devg-v3-gaussdb/gaussdb-12-0270.html
