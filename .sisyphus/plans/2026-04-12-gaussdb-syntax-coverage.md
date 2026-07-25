# GaussDB 语法覆盖扩展 — 分区/层级查询/多维聚合/高级特性

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 将 GaussDB 语法解析覆盖率从当前水平提升到生产级，补齐分区DDL、层级查询、多表插入、行列转换、多维聚合等关键缺失特性。

**Architecture:** 所有变更遵循现有架构 — AST 类型在 `src/ast/mod.rs`，Parser 方法在 `src/parser/` 各模块，Formatter 在 `src/formatter.rs`，测试在 `src/parser/tests.rs`。三种来源（SQL/XML/Java）共享同一 Parser 内核，自动受益。

**Tech Stack:** Rust 2021, thiserror, serde (Serialize + Deserialize)

**TDD 规则:** 先写失败测试 → 验证失败 → 最小实现 → 验证通过 → 回归测试。

**回归验证:** 每个波次完成后必须运行:
1. `cargo test` — 单元测试全部通过
2. `cargo test --features ibatis` — 含 XML 解析测试
3. `cargo test --features java` — 含 Java 提取测试
4. `cargo run --example regression` — openGauss 1409 回归测试全部通过

---

## Wave 1: GROUPING SETS / ROLLUP / CUBE — 多维聚合 (P1)

**背景:** GaussDB 支持在 GROUP BY 中使用 GROUPING SETS、ROLLUP、CUBE 实现多维聚合分析。当前 parser 只支持普通 GROUP BY 表达式列表。

**Files:**
- Modify: `src/ast/mod.rs` — 新增 `GroupByItem` 枚举替换 `Vec<Expr>`
- Modify: `src/parser/select.rs` — GROUP BY 解析增加 GROUPING SETS/ROLLUP/CUBE
- Modify: `src/formatter.rs` — 格式化新增语法
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
// === GROUPING SETS / ROLLUP / CUBE Tests ===

#[test]
fn test_grouping_sets() {
    let stmt = parse_one("SELECT dept, region, SUM(salary) FROM emp GROUP BY GROUPING SETS ((dept, region), (dept), (region), ())");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.group_by.len(), 1);
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_rollup() {
    let stmt = parse_one("SELECT year, month, SUM(amount) FROM sales GROUP BY ROLLUP (year, month)");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.group_by.len(), 1);
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_cube() {
    let stmt = parse_one("SELECT year, product, SUM(amount) FROM sales GROUP BY CUBE (year, product)");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.group_by.len(), 1);
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_mixed_group_by() {
    let stmt = parse_one("SELECT dept, region, SUM(salary) FROM emp GROUP BY dept, ROLLUP (region)");
    match stmt {
        Statement::Select(s) => {
            assert!(s.group_by.len() >= 2);
        }
        _ => panic!("expected Select"),
    }
}
```

**Step 2: 运行测试确认失败**

Run: `cargo test test_grouping_sets test_rollup test_cube test_mixed_group_by`
Expected: 编译失败或测试失败

**Step 3: AST 变更 — `src/ast/mod.rs`**

在 `SelectStatement` 中将 `group_by: Vec<Expr>` 替换为新的枚举:

```rust
/// GROUP BY 子项：普通表达式 或 GROUPING SETS / ROLLUP / CUBE
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GroupByItem {
    /// 普通表达式: GROUP BY a, b
    Expr(Expr),
    /// GROUPING SETS ((a, b), (a), ())
    GroupingSets(Vec<Vec<Expr>>),
    /// ROLLUP (a, b, c)
    Rollup(Vec<Expr>),
    /// CUBE (a, b, c)
    Cube(Vec<Expr>),
}
```

将 `SelectStatement.group_by` 类型改为 `Vec<GroupByItem>`。

**影响范围:** 所有引用 `group_by` 字段的代码需要适配：
- `src/parser/select.rs` — 解析逻辑
- `src/formatter.rs` — 格式化逻辑
- `src/parser/tests.rs` — 现有 group_by 相关测试

**Step 4: Parser 变更 — `src/parser/select.rs`**

修改 GROUP BY 解析段（当前在 `parse_simple_select` 第139-150行）:

```rust
let group_by = if self.match_keyword(Keyword::GROUP_P) {
    self.advance();
    self.expect_keyword(Keyword::BY)?;
    let mut items = vec![self.parse_group_by_item()?];
    while self.match_token(&Token::Comma) {
        self.advance();
        items.push(self.parse_group_by_item()?);
    }
    items
} else {
    vec![]
};
```

新增方法:
```rust
fn parse_group_by_item(&mut self) -> Result<GroupByItem, ParserError> {
    if self.match_keyword(Keyword::GROUPING) {
        // 不需要 consume Hints，直接检测
    }
    if self.match_ident_str("GROUPING") {
        // GROUPING SETS (...)
        // 但注意 GROUPING 可能是普通标识符
        // 需要前瞻判断
    }
    // 方案: 先尝试匹配关键字 GROUPING SETS, ROLLUP, CUBE
    // 如果不是，走普通表达式
}
```

实际实现需要注意：
- `GROUPING SETS` 中 GROUPING 不是保留关键字，需要用 `match_ident_str` 或检测后面跟着 SETS
- `ROLLUP` 和 `CUBE` 同理（在 keyword.rs 中已定义为 Unreserved）
- 需要解析嵌套括号组: `GROUPING SETS ((a, b), (a), ())`

**Step 5: Formatter 变更 — `src/formatter.rs`**

在 format_select_statement 中适配新的 group_by 类型。

**Step 6: 修复现有测试**

现有引用 `group_by` 字段的测试需要适配 `GroupByItem::Expr(e)` 包装。

**Step 7: 运行全部测试**

Run: `cargo test && cargo run --example regression`
Expected: 全部通过

**Step 8: Commit**

```bash
git add -A
git commit -m "feat: GROUPING SETS / ROLLUP / CUBE support in GROUP BY"
```

---

## Wave 2: CONNECT BY 层级查询 (P0)

**背景:** GaussDB/Oracle 兼容的层级查询语法 `SELECT ... CONNECT BY [NOCYCLE] PRIOR col = parent_col START WITH expr`。PRIOR 关键字已定义，但未实现层级查询解析。

**Files:**
- Modify: `src/ast/mod.rs` — `SelectStatement` 新增层级查询字段
- Modify: `src/parser/select.rs` — 解析 CONNECT BY / START WITH / PRIOR / NOCYCLE
- Modify: `src/parser/expr.rs` — PRIOR 一元运算符
- Modify: `src/formatter.rs` — 格式化层级查询
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
#[test]
fn test_connect_by_simple() {
    let stmt = parse_one("SELECT * FROM emp CONNECT BY PRIOR empno = mgr");
    match stmt {
        Statement::Select(s) => {
            assert!(s.connect_by.is_some());
            let cb = s.connect_by.as_ref().unwrap();
            assert!(!cb.nocycle);
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_connect_by_with_start_with() {
    let stmt = parse_one("SELECT * FROM emp START WITH mgr IS NULL CONNECT BY PRIOR empno = mgr");
    match stmt {
        Statement::Select(s) => {
            let cb = s.connect_by.as_ref().expect("should have CONNECT BY");
            assert!(cb.start_with.is_some());
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_connect_by_nocycle() {
    let stmt = parse_one("SELECT * FROM emp CONNECT BY NOCYCLE PRIOR empno = mgr");
    match stmt {
        Statement::Select(s) => {
            let cb = s.connect_by.as_ref().unwrap();
            assert!(cb.nocycle);
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_connect_by_level() {
    let stmt = parse_one("SELECT LEVEL, empno, ename FROM emp CONNECT BY PRIOR empno = mgr START WITH mgr IS NULL");
    match stmt {
        Statement::Select(s) => {
            assert!(s.connect_by.is_some());
        }
        _ => panic!("expected Select"),
    }
}
```

**Step 2: 运行测试确认失败**

Run: `cargo test test_connect_by`
Expected: 编译错误（`connect_by` 字段不存在）

**Step 3: AST 变更 — `src/ast/mod.rs`**

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ConnectByClause {
    pub nocycle: bool,
    pub condition: Expr,  // PRIOR col = parent_col
    pub start_with: Option<Expr>,
}
```

在 `SelectStatement` 中新增:
```rust
pub connect_by: Option<ConnectByClause>,
```

同时在 `Expr` 枚举中新增 PRIOR 一元运算符:
```rust
Prior(Box<Expr>),  // PRIOR expr
```

**Step 4: Parser 变更 — `src/parser/select.rs`**

在 `parse_simple_select` 中，在 `where_clause` 之后、`group_by` 之前插入 CONNECT BY / START WITH 解析:

```rust
// START WITH 可以出现在 CONNECT BY 之前或之后（GaussDB 兼容两种顺序）
let start_with = if self.match_keyword(Keyword::START) {
    self.advance();
    self.expect_keyword(Keyword::WITH)?;
    Some(self.parse_expr()?)
} else {
    None
};

let connect_by = if self.match_keyword(Keyword::CONNECT) {
    self.advance();
    self.expect_keyword(Keyword::BY)?;
    let nocycle = self.try_consume_keyword(Keyword::NOCYCLE);
    let condition = self.parse_expr()?;
    let sw = if start_with.is_none() && self.match_keyword(Keyword::START) {
        self.advance();
        self.expect_keyword(Keyword::WITH)?;
        Some(self.parse_expr()?)
    } else {
        start_with
    };
    Some(ConnectByClause { nocycle, condition, start_with: sw })
} else if start_with.is_some() {
    // START WITHOUT without CONNECT BY — error or treat as WHERE
    None
} else {
    None
};
```

在 `src/parser/expr.rs` 中，在 `parse_unary_expr` 里添加 PRIOR:
```rust
if self.match_keyword(Keyword::PRIOR) {
    self.advance();
    let expr = self.parse_expr_with_precedence(15)?;  // 高优先级
    return Ok(Expr::Prior(Box::new(expr)));
}
```

**Step 5: Formatter 变更**

添加 format_connect_by 方法。

**Step 6: 运行全部测试 + 回归**

Run: `cargo test && cargo run --example regression`

**Step 7: Commit**

```bash
git commit -m "feat: CONNECT BY hierarchical query support (Oracle/GaussDB compatible)"
```

---

## Wave 3: INSERT ALL / INSERT FIRST — 多表插入 (P1)

**背景:** GaussDB/Oracle 兼容的多表无条件插入语法。

**Files:**
- Modify: `src/ast/mod.rs` — 新增 `InsertAllStatement` / `InsertFirstStatement`
- Modify: `src/parser/dml.rs` — 解析多表插入
- Modify: `src/parser/mod.rs` — dispatch INSERT ALL / INSERT FIRST
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
#[test]
fn test_insert_all() {
    let stmt = parse_one("INSERT ALL INTO sales_east VALUES (1, 'a') INTO sales_west VALUES (2, 'b') SELECT * FROM source");
    match stmt {
        Statement::InsertAll(ia) => {
            assert_eq!(ia.targets.len(), 2);
            assert!(ia.conditions.is_empty());
        }
        _ => panic!("expected InsertAll, got {:?}", stmt),
    }
}

#[test]
fn test_insert_first() {
    let stmt = parse_one("INSERT FIRST WHEN dept = 'EAST' THEN INTO sales_east VALUES (1, 'a') WHEN dept = 'WEST' THEN INTO sales_west VALUES (2, 'b') ELSE INTO sales_other VALUES (3, 'c') SELECT * FROM source");
    match stmt {
        Statement::InsertFirst(if_stmt) => {
            assert_eq!(if_stmt.when_clauses.len(), 2);
        }
        _ => panic!("expected InsertFirst"),
    }
}

#[test]
fn test_insert_all_with_when() {
    let stmt = parse_one("INSERT ALL WHEN salary > 10000 THEN INTO high_earners VALUES (empno, name) WHEN salary <= 10000 THEN INTO low_earners VALUES (empno, name) SELECT empno, name, salary FROM emp");
    match stmt {
        Statement::InsertAll(ia) => {
            assert_eq!(ia.targets.len(), 0); // no unconditional targets
            assert_eq!(ia.conditions.len(), 2);
        }
        _ => panic!("expected InsertAll"),
    }
}
```

**Step 2: 确认失败**

**Step 3: AST 变更**

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InsertAllTarget {
    pub table: ObjectName,
    pub columns: Vec<String>,
    pub values: Vec<Vec<Expr>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InsertAllCondition {
    pub condition: Expr,
    pub targets: Vec<InsertAllTarget>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InsertAllStatement {
    pub targets: Vec<InsertAllTarget>,        // 无条件插入目标
    pub conditions: Vec<InsertAllCondition>,  // WHEN 条件插入
    pub else_targets: Vec<InsertAllTarget>,   // ELSE 目标
    pub source: Box<SelectStatement>,         // 子查询数据源
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InsertFirstStatement {
    pub when_clauses: Vec<InsertAllCondition>,
    pub else_targets: Vec<InsertAllTarget>,
    pub source: Box<SelectStatement>,
}
```

在 `Statement` 枚举中新增 `InsertAll(InsertAllStatement)` 和 `InsertFirst(InsertFirstStatement)`。

**Step 4: Parser 变更**

在 `src/parser/mod.rs` 的 `parse_statement` 中，INSERT dispatch 处增加 ALL/FIRST 判断:

```rust
Token::Keyword(Keyword::INSERT) => {
    self.advance();
    if self.match_keyword(Keyword::ALL) {
        self.advance();
        match self.parse_insert_all(false) {
            Ok(stmt) => { self.try_consume_semicolon(); Statement::InsertAll(stmt) }
            Err(e) => { self.add_error(e); self.skip_to_semicolon() }
        }
    } else if self.match_keyword(Keyword::FIRST_P) {
        self.advance();
        match self.parse_insert_all(true) {
            Ok(stmt) => { self.try_consume_semicolon(); Statement::InsertFirst(stmt) }
            Err(e) => { self.add_error(e); self.skip_to_semicolon() }
        }
    } else {
        // existing INSERT INTO logic
    }
}
```

**Step 5: Formatter 变更**

**Step 6: 测试 + 回归**

**Step 7: Commit**

---

## Wave 4: PIVOT / UNPIVOT — 行列转换 (P1)

**背景:** GaussDB 支持在 FROM 子句中使用 PIVOT 和 UNPIVOT 进行行列转换。

**Files:**
- Modify: `src/ast/mod.rs` — `TableRef` 新增 PIVOT / UNPIVOT 变体
- Modify: `src/parser/select.rs` — FROM 子句后解析 PIVOT / UNPIVOT
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
#[test]
fn test_pivot() {
    let stmt = parse_one("SELECT * FROM sales PIVOT (SUM(amount) FOR quarter IN ('Q1' AS q1, 'Q2' AS q2))");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.from.len(), 1);
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_unpivot() {
    let stmt = parse_one("SELECT * FROM pivoted UNPIVOT (amount FOR quarter IN (q1 AS 'Q1', q2 AS 'Q2'))");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.from.len(), 1);
        }
        _ => panic!("expected Select"),
    }
}
```

**Step 2: 确认失败**

**Step 3: AST 变更**

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PivotClause {
    pub aggregate: Expr,            // SUM(amount)
    pub for_column: ObjectName,     // quarter
    pub values: Vec<PivotValue>,    // IN ('Q1' AS q1, 'Q2' AS q2)
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PivotValue {
    pub value: Expr,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnpivotClause {
    pub value_column: ObjectName,   // amount
    pub for_column: ObjectName,     // quarter
    pub columns: Vec<PivotValue>,   // IN (q1 AS 'Q1', q2 AS 'Q2')
}
```

在 `TableRef` 中新增变体或在现有 TableRef 后附加 pivot 字段。

**Step 4: Parser 变更**

在 `parse_table_ref` 中，解析完主表后检查 PIVOT / UNPIVOT。

**Step 5-7: Formatter + 测试 + 回归 + Commit**

---

## Wave 5: ALTER TABLE PARTITION — 分区管理 DDL (P0)

**背景:** 这是 GaussDB DDL 最核心的差异。当前 `PARTITION BY RANGE/LIST/HASH(column)` 基础分区定义已支持，但缺少 ALTER TABLE PARTITION 操作和子分区。

**Files:**
- Modify: `src/ast/mod.rs` — `AlterTableAction` 新增分区操作
- Modify: `src/parser/ddl.rs` — 解析分区操作
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
#[test]
fn test_alter_table_add_partition() {
    let stmt = parse_one("ALTER TABLE sales ADD PARTITION p202601 VALUES LESS THAN ('2026-02-01')");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_drop_partition() {
    let stmt = parse_one("ALTER TABLE sales DROP PARTITION p202501");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_merge_partitions() {
    let stmt = parse_one("ALTER TABLE sales MERGE PARTITIONS p202501, p202502 INTO PARTITION p2025q1");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_split_partition() {
    let stmt = parse_one("ALTER TABLE sales SPLIT PARTITION p2025q1 AT ('2025-02-01') INTO (PARTITION p202501, PARTITION p202502)");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_exchange_partition() {
    let stmt = parse_one("ALTER TABLE sales EXCHANGE PARTITION p202501 WITH TABLE sales_temp");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_truncate_partition() {
    let stmt = parse_one("ALTER TABLE sales TRUNCATE PARTITION p202501");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
        }
        _ => panic!("expected AlterTable"),
    }
}
```

**Step 2: 确认失败**

**Step 3: AST 变更**

在 `AlterTableAction` 枚举中新增:

```rust
AddPartition {
    name: String,
    values: PartitionValues,
    tablespace: Option<String>,
},
DropPartition {
    name: String,
    if_exists: bool,
    cascade: bool,
},
MergePartitions {
    names: Vec<String>,
    into_name: String,
},
SplitPartition {
    name: String,
    at_value: Option<Expr>,
    into: Vec<PartitionDef>,
},
ExchangePartition {
    name: String,
    table: ObjectName,
    with_validation: bool,
},
TruncatePartition {
    name: String,
    cascade: bool,
},
RenamePartition {
    old_name: String,
    new_name: String,
},

// 辅助类型
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PartitionValues {
    LessThan(Vec<Expr>),           // VALUES LESS THAN (...)
    InValues(Vec<Expr>),           // VALUES IN (...)
    StartEnd { start: Expr, end: Expr }, // START(...) END(...)
}
```

**Step 4: Parser 变更**

在 `parse_alter_table` 的 action dispatch 中添加 PARTITION 关键字分支。需要先检查后面跟着的是否是 ADD/DROP/MERGE/SPLIT/EXCHANGE/TRUNCATE/RENAME 关键字。

**Step 5-7: 同上**

---

## Wave 6: CREATE TABLE PARTITION 增强 (P0)

**背景:** 完善分区表创建语法 — 间隔分区(INTERVAL)、子分区(SUBPARTITION)、模板分区。

**Files:**
- Modify: `src/ast/mod.rs` — `PartitionClause` 增强, `CreateTableStatement` 增加 SUBPARTITION
- Modify: `src/parser/ddl.rs` — 解析增强分区语法
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
#[test]
fn test_create_table_range_partition_with_values() {
    let stmt = parse_one("CREATE TABLE sales (id INT, sale_date DATE, amount DECIMAL) PARTITION BY RANGE (sale_date) (PARTITION p2025 VALUES LESS THAN ('2026-01-01'), PARTITION p2026 VALUES LESS THAN ('2027-01-01'))");
    match stmt {
        Statement::CreateTable(ct) => {
            assert!(ct.partition_by.is_some());
            // 需要检查分区定义中是否包含 VALUES LESS THAN
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_interval_partition() {
    let stmt = parse_one("CREATE TABLE t (id INT, created DATE) PARTITION BY RANGE (created) INTERVAL ('1 month') (PARTITION p0 VALUES LESS THAN ('2025-01-01'))");
    match stmt {
        Statement::CreateTable(ct) => {
            assert!(ct.partition_by.is_some());
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_list_partition() {
    let stmt = parse_one("CREATE TABLE region_sales (id INT, region VARCHAR(10)) PARTITION BY LIST (region) (PARTITION p_east VALUES IN ('EAST'), PARTITION p_west VALUES IN ('WEST'))");
    match stmt {
        Statement::CreateTable(ct) => {
            assert!(ct.partition_by.is_some());
        }
        _ => panic!("expected CreateTable"),
    }
}
```

**Step 2-7: 同上模式**

需要扩展 `PartitionClause`:
```rust
pub enum PartitionClause {
    Range {
        column: ObjectName,
        interval: Option<Expr>,              // INTERVAL ('1 month')
        partitions: Vec<PartitionDef>,       // (PARTITION p1 VALUES LESS THAN (...), ...)
    },
    List {
        column: ObjectName,
        partitions: Vec<PartitionDef>,
    },
    Hash {
        column: ObjectName,
        partitions: Option<u32>,             // PARTITIONS 4
        partition_names: Vec<PartitionDef>,
    },
}

pub struct PartitionDef {
    pub name: String,
    pub values: Option<PartitionValues>,
    pub tablespace: Option<String>,
    pub subpartitions: Option<SubpartitionClause>,
}
```

---

## Wave 7: FILTER / WITHIN GROUP — 聚合函数增强 (P2)

**背景:** `COUNT(*) FILTER (WHERE ...)` 和 `PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY x)` 是 GaussDB 支持的高级聚合语法。

**Files:**
- Modify: `src/ast/mod.rs` — `Expr::FunctionCall` 增加 filter / within_group 字段
- Modify: `src/parser/expr.rs` — 函数调用后解析 FILTER / WITHIN GROUP
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
#[test]
fn test_filter_clause() {
    let stmt = parse_one("SELECT COUNT(*) FILTER (WHERE status = 'active') FROM users");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_within_group() {
    let stmt = parse_one("SELECT PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY salary) FROM emp");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select"),
    }
}
```

**Step 2-7: 同上模式**

在 `Expr::FunctionCall` 中新增:
```rust
pub filter: Option<Box<Expr>>,                    // FILTER (WHERE ...)
pub within_group: Option<Vec<OrderByItem>>,       // WITHIN GROUP (ORDER BY ...)
```

---

## Wave 8: DATABASE LINK (P1)

**背景:** GaussDB 跨库查询 `CREATE DATABASE LINK` / 通过 `@link_name` 访问远程表。

**Files:**
- Modify: `src/ast/mod.rs` — 新增 DatabaseLink 相关类型
- Modify: `src/parser/ddl.rs` — 解析 CREATE/ALTER/DROP DATABASE LINK
- Modify: `src/parser/mod.rs` — dispatch
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
#[test]
fn test_create_database_link() {
    let stmt = parse_one("CREATE DATABASE LINK remote_db CONNECT TO user IDENTIFIED BY 'pass' USING 'host:port/db'");
    match stmt {
        Statement::CreateDatabaseLink(dbl) => {
            assert_eq!(dbl.name, "remote_db");
        }
        _ => panic!("expected CreateDatabaseLink"),
    }
}

#[test]
fn test_create_public_database_link() {
    let stmt = parse_one("CREATE PUBLIC DATABASE LINK remote_db CONNECT TO user IDENTIFIED BY 'pass' USING 'host:port/db'");
    match stmt {
        Statement::CreateDatabaseLink(dbl) => {
            assert!(dbl.public_link);
        }
        _ => panic!("expected CreateDatabaseLink"),
    }
}
```

**Step 2-7: 同上模式**

---

## Wave 9: GaussDB 特有 CREATE TABLE 选项 (P2)

**背景:** GaussDB CREATE TABLE 支持大量特有选项: DISTRIBUTE BY, TO GROUP, orientation=column, 压缩等。

**Files:**
- Modify: `src/ast/mod.rs` — `CreateTableStatement` 增加分布式选项
- Modify: `src/parser/ddl.rs` — 解析 GaussDB 特有选项
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
#[test]
fn test_create_table_distribute_by_hash() {
    let stmt = parse_one("CREATE TABLE t (id INT, name VARCHAR(100)) DISTRIBUTE BY HASH (id) TO GROUP group1");
    match stmt {
        Statement::CreateTable(ct) => {
            assert!(ct.options.len() > 0 || ct.distribute_by.is_some());
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_column_orientation() {
    let stmt = parse_one("CREATE TABLE t (id INT, name VARCHAR(100)) WITH (orientation = column)");
    match stmt {
        Statement::CreateTable(ct) => {
            // orientation 应在 options 中
        }
        _ => panic!("expected CreateTable"),
    }
}
```

**Step 2-7: 同上模式**

需要扩展 `CreateTableStatement`:
```rust
pub distribute_by: Option<DistributeClause>,
pub to_group: Option<String>,
```

```rust
pub enum DistributeClause {
    Hash { columns: Vec<String> },
    Replication,
    RoundRobin { columns: Vec<String> },
    Modulo { columns: Vec<String> },
}
```

---

## 执行顺序

| 顺序 | 波次 | 预计复杂度 | 预计新增测试 |
|------|------|-----------|-------------|
| 1 | Wave 1: GROUPING SETS/ROLLUP/CUBE | 中 | 8-10 |
| 2 | Wave 2: CONNECT BY 层级查询 | 中 | 6-8 |
| 3 | Wave 3: INSERT ALL/FIRST | 中 | 6-8 |
| 4 | Wave 4: PIVOT/UNPIVOT | 中 | 6-8 |
| 5 | Wave 5: ALTER TABLE PARTITION | 高 | 10-12 |
| 6 | Wave 6: CREATE TABLE PARTITION 增强 | 高 | 8-10 |
| 7 | Wave 7: FILTER/WITHIN GROUP | 低 | 4-6 |
| 8 | Wave 8: DATABASE LINK | 中 | 4-6 |
| 9 | Wave 9: CREATE TABLE 分布式选项 | 中 | 4-6 |

**每个波次完成后的验证检查点:**

```bash
# 单元测试
cargo test
cargo test --features ibatis
cargo test --features java

# 回归测试
cargo run --example regression

# 全特性测试
cargo test --features full
```

**注意:** 波次 5 和 6 可以并行开发（ALTER TABLE PARTITION 和 CREATE TABLE PARTITION 增强），但建议顺序执行以确保 AST 类型一致性。
