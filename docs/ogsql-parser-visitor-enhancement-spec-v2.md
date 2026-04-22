# ogsql-parser Visitor 增强需求规格书 v2

**目标仓库**: https://github.com/c2j/ogsql-parser
**基线版本**: commit `85ac383f`
**需求方**: codeweb 项目 — 基于 ogsql-parser 构建存储过程调用图分析工具
**版本**: v2（2026-04-23 修订）

---

## 1. 背景与动机

### 1.1 现状

当前 `Visitor` trait（`src/ast/visitor.rs`，337 行）仅覆盖 SQL 顶层语句的浅层遍历：

- **7 个 trait 方法**：`visit_statement`、`visit_expr`、`visit_select`、`visit_insert`、`visit_update`、`visit_delete`、`visit_create_table`
- **`walk_statement` 仅处理 5 个 `Statement` 变体**（Select/Insert/Update/Delete/CreateTable），其余 **182 个**变体落入 `_ => VisitorResult::Continue` 通配
- **完全不感知 PL/pgSQL**：无 `visit_pl_block`、`visit_pl_statement`、`visit_pl_declaration` 等方法
- **`walk_expr` 覆盖不完整**：跳过 `Like`、`Between`、`InList`、`InSubquery`、`Exists`、`ScalarSublink`、`TypeCast`、`Array`、`Subscript`、`FieldAccess` 等大量变体
- **`walk_select` 跳过 CTEs**（`with`）、`group_by`、`having`、`order_by`、`set_operation`

### 1.2 影响范围

以下 6 个承载 PL/pgSQL 块体的 `Statement` 变体，遍历时完全被忽略：

| Statement 变体 | PL/pgSQL 入口 | 源码位置 |
|---|---|---|
| `CreateFunction` | `.block: Option<PlBlock>` | mod.rs:1857 |
| `CreateProcedure` | `.block: Option<PlBlock>` | mod.rs:1866 |
| `Do` | `.block: Option<PlBlock>` | mod.rs:2141 |
| `AnonyBlock` | `.block: PlBlock`（非 Option） | mod.rs:1691 |
| `CreatePackage` | `.items → PackageProcedure/PackageFunction → .block` | mod.rs:1876-1912 |
| `CreatePackageBody` | 同上 | mod.rs:1885-1890 |

此外，`PlStatement` 的 31 个变体中嵌入的 `Expr` 和 `Statement`（如 `PlExecuteStmt.parsed_query`、`PlForKind::Query.parsed_query`、`PlStatement::SqlStatement.statement`）也无法被遍历到。

### 1.3 目标

使 `Visitor` trait 能够**完整遍历 ogsql-parser 已有的全部 AST 类型**，包括 PL/pgSQL 块体，从而使下游用户（如调用图提取、代码分析、格式化等）无需自行编写遍历器。

---

## 2. 设计原则

1. **向后兼容**：现有 `Visitor` 实现必须无需修改即可编译通过。新增 trait 方法全部提供默认空实现。
2. **不引入外部依赖**：控制流机制保持现有 `VisitorResult`。
3. **关注点分离**：`Visitor` trait 负责通知，`walk_*` 函数负责驱动遍历。两者独立演进。
4. **覆盖完整性**：对 AST 类型做到无遗漏遍历。任何 `Expr`/`Statement`/`PlStatement`/`PlDeclaration` 的子节点均应被遍历到。
5. **可组合**：用户可以通过覆盖 visit 方法返回 `SkipChildren` 来跳过子树。

---

## 3. 需求项

按优先级排列。P0 = 必须实现；P1 = 强烈建议；P2 = 建议实现。

---

### P0-1: PL/pgSQL Visitor Trait 方法

在 `Visitor` trait 中新增以下方法（均提供默认空实现，返回 `VisitorResult::Continue`）：

```rust
// PlBlock 入口
fn visit_pl_block(&mut self, _block: &PlBlock) -> VisitorResult {
    VisitorResult::Continue
}

// PL/pgSQL 语句
fn visit_pl_statement(&mut self, _stmt: &PlStatement) -> VisitorResult {
    VisitorResult::Continue
}

// PL/pgSQL 声明
fn visit_pl_declaration(&mut self, _decl: &PlDeclaration) -> VisitorResult {
    VisitorResult::Continue
}

// PL/pgSQL 异常处理器
fn visit_pl_exception_handler(&mut self, _handler: &PlExceptionHandler) -> VisitorResult {
    VisitorResult::Continue
}
```

**验收标准**：
- 所有方法均有默认实现，现有 `Visitor` impl 无需修改
- `PlBlock`、`PlStatement`、`PlDeclaration`、`PlExceptionHandler` 类型已正确 import

---

### P0-2: PL/pgSQL Walk 函数

新增以下 `pub` walk 函数，由 `walk_statement` 在遇到承载 PL/pgSQL 块的 `Statement` 变体时调用：

```rust
/// 遍历 PlBlock：declarations → body → exception_block
pub fn walk_pl_block(visitor: &mut dyn Visitor, block: &PlBlock) -> VisitorResult

/// 遍历 PlStatement：先调用 visit_pl_statement，再根据变体递归遍历子节点
pub fn walk_pl_statement(visitor: &mut dyn Visitor, stmt: &PlStatement) -> VisitorResult

/// 遍历 PlDeclaration：先调用 visit_pl_declaration，再遍历子 Expr/Statement/PlBlock
pub fn walk_pl_declaration(visitor: &mut dyn Visitor, decl: &PlDeclaration) -> VisitorResult
```

**遍历顺序**（`walk_pl_block`）：
1. 调用 `visitor.visit_pl_block(block)` → 检查返回值
2. 遍历 `block.declarations` → 每个调用 `walk_pl_declaration`
3. 遍历 `block.body` → 每个调用 `walk_pl_statement`
4. 如果存在 `block.exception_block`（类型为 `PlExceptionBlock`）→ 遍历其 `handlers`，每个 handler：
   a. 调用 `visitor.visit_pl_exception_handler(handler)`
   b. 遍历 `handler.statements` → 每个调用 `walk_pl_statement`

**`walk_pl_statement` 需覆盖的 PlStatement 变体**：

| 变体 | 需遍历的子节点 |
|---|---|
| `Block(PlBlock)` | `walk_pl_block` |
| `Assignment { expression }` | `walk_expr(expression)` |
| `If(PlIfStmt)` | `.condition` → `walk_expr`, `.then_stmts` / `.elsifs[].stmts` / `.else_stmts` → `walk_pl_statement` |
| `Case(PlCaseStmt)` | `.expression` → `walk_expr`, `.whens[].condition` → `walk_expr`, `.whens[].stmts` / `.else_stmts` → `walk_pl_statement` |
| `Loop(PlLoopStmt)` | `.body` → `walk_pl_statement` |
| `While(PlWhileStmt)` | `.condition` → `walk_expr`, `.body` → `walk_pl_statement` |
| `For(PlForStmt)` | `.kind` 见下表，`.body` → `walk_pl_statement` |
| `ForEach(PlForEachStmt)` | `.expression` → `walk_expr`, `.body` → `walk_pl_statement` |
| `Exit { condition }` | `condition` → `walk_expr`（若存在） |
| `Continue { condition }` | `condition` → `walk_expr`（若存在） |
| `Return { expression }` | `expression` → `walk_expr`（若存在） |
| `ReturnNext { expression }` | `expression` → `walk_expr` |
| `ReturnQuery(PlReturnQueryStmt)` | `.dynamic_expr` → `walk_expr`（若存在），`.using_args[].argument` → `walk_expr` |
| `Raise(PlRaiseStmt)` | `.params` 每个 → `walk_expr`，`.options[].value` → `walk_expr` |
| `Execute(PlExecuteStmt)` | `.string_expr` → `walk_expr`，`.into_targets` 每个 → `walk_expr`，`.using_args[].argument` → `walk_expr`，`.parsed_query` → `walk_statement`（若存在） |
| `Perform { parsed_query }` | `parsed_query` → `walk_statement`（若存在） |
| `Open(PlOpenStmt)` | `.kind` 见下表 |
| `Fetch(PlFetchStmt)` | `.into` → `walk_expr` |
| `Close { .. }` | 无子节点 |
| `Move { .. }` | 无子节点 |
| `GetDiagnostics(PlGetDiagStmt)` | 无子节点 |
| `Commit` | 无子节点 |
| `Rollback { .. }` | 无子节点 |
| `Savepoint { .. }` | 无子节点 |
| `ReleaseSavepoint { .. }` | 无子节点 |
| `Null` | 无子节点 |
| `Goto { .. }` | 无子节点 |
| `ProcedureCall(PlProcedureCall)` | `.arguments` 每个 → `walk_expr` |
| `Sql(String)` | 无结构化子节点（原始字符串） |
| `SqlStatement { statement }` | `statement` → `walk_statement` |
| `ForAll(PlForAllStmt)` | 无结构化子节点（原始字符串） |
| `PipeRow { expression }` | `expression` → `walk_expr` |

**`walk_pl_statement` 需覆盖的 PlForKind 变体**：

| 变体 | 需遍历的子节点 |
|---|---|
| `Range { low, high, step }` | `low` / `high` / `step`（若存在）→ `walk_expr` |
| `Query { parsed_query, using_args }` | `parsed_query` → `walk_statement`（若存在），`using_args[].argument` → `walk_expr` |
| `Cursor { arguments }` | `arguments` 每个 → `walk_expr` |

**`walk_pl_statement` 需覆盖的 PlOpenKind 变体**：

| 变体 | 需遍历的子节点 |
|---|---|
| `Simple { arguments }` | `arguments` 每个 → `walk_expr` |
| `ForQuery { parsed_query }` | `parsed_query` → `walk_statement`（若存在） |
| `ForExecute { query, using_args }` | `query` → `walk_expr`，`using_args` 每个 → `walk_expr` |
| `ForUsing { expressions }` | `expressions` 每个 → `walk_expr` |

**`walk_pl_declaration` 需覆盖的 PlDeclaration 变体**：

| 变体 | 需遍历的子节点 |
|---|---|
| `Variable(PlVarDecl)` | `.default` → `walk_expr`（若存在） |
| `Cursor(PlCursorDecl)` | `.parsed_query` → `walk_statement`（若存在） |
| `Record(PlRecordDecl)` | 无子节点 |
| `Type(PlTypeDecl)` | `VarrayOf.size` → `walk_expr` |
| `NestedProcedure(PackageProcedure)` | `.block` → `walk_pl_block`（若存在） |
| `NestedFunction(PackageFunction)` | `.block` → `walk_pl_block`（若存在） |
| `Pragma { .. }` | 无子节点 |

**验收标准**：
- `walk_pl_statement` 能递归进入所有包含子 `PlStatement` 的变体（`If.then_stmts/elsifs/else_stmts`、`Case.whens/else_stmts`、`Loop.body`、`While.body`、`For.body`、`ForEach.body`、`Block(PlBlock)`）
- `walk_pl_statement` 能遍历所有包含 `Expr` 的变体（调用 `walk_expr`）
- `walk_pl_statement` 能遍历所有包含 `Statement` 的变体（调用 `walk_statement`）
- `walk_pl_declaration` 能进入嵌套的 `NestedProcedure`/`NestedFunction` 的 `block`

---

### P0-3: 修正 `walk_statement` 对 PL/pgSQL 载体变体的处理

当前 `walk_statement` 的 `_ => VisitorResult::Continue` 通配分支忽略了 PL/pgSQL 载体。需改为显式匹配：

```rust
Statement::CreateFunction(s) => {
    // visit_statement 已在外层调用，内层只负责遍历子节点
    if let Some(ref block) = s.block {
        walk_pl_block(visitor, block)?;
    }
    VisitorResult::Continue
}
Statement::CreateProcedure(s) => { /* 同上模式 */ }
Statement::Do(s) => { /* 同上模式 */ }
Statement::AnonyBlock(s) => {
    // block 不是 Option，直接遍历
    walk_pl_block(visitor, &s.block)?;
    VisitorResult::Continue
}
Statement::CreatePackage(s) => {
    for item in &s.items {
        match item {
            PackageItem::Procedure(p) => {
                if let Some(ref block) = p.block {
                    walk_pl_block(visitor, block)?;
                }
            }
            PackageItem::Function(f) => {
                if let Some(ref block) = f.block {
                    walk_pl_block(visitor, block)?;
                }
            }
            PackageItem::Raw(_) => {}
        }
    }
    VisitorResult::Continue
}
Statement::CreatePackageBody(s) => { /* 同上模式 */ }
```

**注意**：`visit_statement` 已在外层 `walk_statement` 入口调用，内层不应重复调用。内层只负责遍历子节点。

**验收标准**：
- 解析包含 `CREATE FUNCTION ... AS $$ ... $$` 的 SQL 并用 `walk_statement` 遍历，能触发 `visit_pl_block` 和 `visit_pl_statement`
- 解析 `CREATE PACKAGE BODY` 并遍历，能进入包内过程/函数的 `block`

---

### P0-4: 新增 `visit_call` 和 `visit_procedure_call` 方法

调用图提取的核心需求——对过程调用和函数调用提供专门的 hook：

```rust
/// 顶层 CALL 语句（Statement::Call）
fn visit_call(&mut self, _call: &CallFuncStatement) -> VisitorResult {
    VisitorResult::Continue
}

/// PL/pgSQL 块内的过程调用（PlStatement::ProcedureCall）
fn visit_procedure_call(&mut self, _call: &PlProcedureCall) -> VisitorResult {
    VisitorResult::Continue
}
```

**遍历行为**：
- `walk_statement` 遇到 `Statement::Call(s)` → 调用 `visitor.visit_call(&s)`
- `walk_pl_statement` 遇到 `PlStatement::ProcedureCall(c)` → 调用 `visitor.visit_procedure_call(&c)`，然后遍历 `c.arguments` 中的 `Expr`

**验收标准**：
- 遍历 `CALL schema.run_maintenance()` 能触发 `visit_call`，并可在回调中读取 `call.func_name`
- 遍历 PL/pgSQL 块体内的 `schema.proc_name(arg1)` 能触发 `visit_procedure_call`，并可在回调中读取 `call.name`

---

### P1-1: 增强 `walk_expr` 覆盖率

当前 `walk_expr` 跳过大量 `Expr` 变体。应补全以下遍历：

> **注意**：以下变体中的 `negated` 字段用于表示 NOT 形式（如 `NOT LIKE`、`NOT BETWEEN`、`NOT IN`、`IS NOT NULL`）。NOT 形式不对应独立的 Expr 变体，处理 `Like`/`Between`/`InList`/`InSubquery`/`IsNull` 时已经覆盖。

| Expr 变体 | 应遍历的子节点 |
|---|---|
| `Like { expr, pattern, escape }` | `expr`、`pattern`、`escape`（若存在） |
| `Between { expr, low, high }` | `expr`、`low`、`high` |
| `InList { expr, list }` | `expr`、`list` 中每个 |
| `InSubquery { expr, subquery }` | `expr`、`subquery` → `walk_select` |
| `Exists(subquery)` | `subquery` → `walk_select` |
| `ScalarSublink { expr, subquery }` | `expr`、`subquery` → `walk_select` |
| `IsNull { expr }` | `expr` |
| `TypeCast { expr, default, format }` | `expr`、`default`（若存在）、`format`（若存在） |
| `Treat { expr }` | `expr` |
| `Array(elems)` | `elems` 中每个 |
| `Subscript { object, index }` | `object`、`index` |
| `FieldAccess { object }` | `object` |
| `Parenthesized(expr)` | `expr` |
| `RowConstructor(exprs)` | `exprs` 中每个 |
| `CollationFor { expr }` | `expr` |
| `Prior(expr)` | `expr` |
| `SpecialFunction { args }` | `args` 中每个 |

**补充**：`FunctionCall` 变体虽已处理 `args`，但还需遍历以下子节点：

| FunctionCall 子字段 | 应遍历的子节点 |
|---|---|
| `over` | `WindowSpec.partition_by` / `order_by` / `frame` 中的 `Expr` |
| `filter` | `Expr` |
| `within_group` | `OrderByItem.expr` |
| `separator` | `Expr` |
| `default` | `Expr` |
| `conversion_format` | `Expr` |

**验收标准**：
- 补全后 `walk_expr` 的 match 分支应覆盖 `Expr` 枚举的全部变体（`_` 通配分支可保留用于未来新增变体，但当前所有已知变体均需显式处理）
- 每个 `Expr` 子节点均被递归遍历

---

### P1-2: 增强 `walk_select` 覆盖率

当前 `walk_select` 跳过 CTEs、`group_by`、`having`、`order_by`、`set_operation` 等。应补全：

| 被跳过的 Select 字段 | 应遍历的子节点 |
|---|---|
| `with` (CTEs) | 每个 CTE 的 `query` → `walk_select`；`column_names` 无需遍历 |
| `distinct_on` | 每个 `Expr` → `walk_expr` |
| `connect_by` | `.condition` → `walk_expr`，`.start_with` → `walk_expr`（若存在） |
| `group_by` | 每个 `GroupByItem` → `GroupByItem::Expr` / `Rollup` / `Cube` / `GroupingSets` 中的 `Expr` |
| `having` | `Expr` → `walk_expr` |
| `order_by` | 每个 `OrderByItem.expr` → `walk_expr` |
| `limit` | `Expr` → `walk_expr`（若存在） |
| `offset` | `Expr` → `walk_expr`（若存在） |
| `fetch` | `FetchClause.count` → `walk_expr`（若存在） |
| `set_operation` (UNION/INTERSECT/EXCEPT) | `.right` → `walk_select` |
| `window_clause` | 每个 `NamedWindow` → `WindowSpec` 中的 `partition_by` / `order_by` / `frame` 中的 `Expr` |
| `into_targets` | 每个 `SelectTarget` 中的 `Expr`（若存在） |

**验收标准**：
- 含 CTE 的 `WITH cte AS (...) SELECT ...` 语句能被完整遍历
- 含 `UNION ALL SELECT` 的复合查询能递归遍历两侧
- 含 `CONNECT BY` 的层次查询能遍历条件表达式

---

### P1-3: 增强 `walk_table_ref` 覆盖率

当前 `walk_table_ref` 仅处理 `TableRef::Subquery`。应补全：

| 被跳过的 TableRef 变体 | 应遍历的子节点 |
|---|---|
| `TableRef::Table { timecapsule }` | `timecapsule` → `walk_expr`（若存在） |
| `TableRef::Join { left, right, condition }` | `left` → `walk_table_ref`、`right` → `walk_table_ref`、`condition` → `walk_expr`（若存在） |
| `TableRef::FunctionCall { args }` | `args` 中每个 → `walk_expr` |
| `TableRef::Values { values }` | `ValuesStatement` 中的每行每列 `Expr` → `walk_expr` |
| `TableRef::Pivot { source, pivot }` | `source` → `walk_table_ref`，`pivot.aggregate` → `walk_expr`，`pivot.values` 中每个 `.value` → `walk_expr` |
| `TableRef::Unpivot { source, unpivot }` | `source` → `walk_table_ref`，`unpivot.value_column` 无需遍历（ObjectName） |

---

### P2-1: 统一 `SkipChildren` 处理风格

当前各 `walk_*` 函数对 `SkipChildren` 的处理风格不统一：
- `walk_statement` 使用 `match` 风格
- `walk_select` / `walk_expr` 使用 `if/else` 风格

建议统一为 `match` 风格，确保所有 walk 函数中 `SkipChildren` 语义一致（跳过当前节点子节点，继续遍历兄弟节点）。

**注意**：当前 `SkipChildren` 语义已正确，无需重命名为 `Skip`，也无需增加第四种状态。

---

### P2-2: 导出新增类型

在 `src/lib.rs` 中导出新增的公开类型：

```rust
pub use ast::visitor::{
    walk_statement,
    walk_pl_block,       // 新增
    walk_pl_statement,   // 新增
    walk_pl_declaration, // 新增
    Visitor, VisitorResult,
};
```

---

## 4. 已知限制

以下内容**明确排除**，建议不在本次增强中实现：

- **可变遍历（VisitorMut / Fold）**：当前需求为只读遍历，可变遍历是独立的大型特性
- **AST 变换 / 重写**：不属于 Visitor trait 的职责
- **Proc macro 自动派生**：手动编写 walk 函数已足够，codegen 是后续优化
- **新的控制流机制**：保持现有 `VisitorResult`，不引入 `ControlFlow` 或 `Result` 包装
- **性能优化**（如 arena 分配、零拷贝）：不影响功能正确性

以下 AST 节点**无法结构化遍历**，需在实现中记录为已知限制：

| 节点 | 原因 |
|---|---|
| `PlStatement::Sql(String)` | 仅含原始 SQL 字符串，无结构化 AST |
| `PlStatement::ForAll(PlForAllStmt)` | `bounds` 和 `body` 均为 `String`，无结构化 AST |

---

## 5. 验收测试建议

### 5.1 P0 验收用例

**TC-P0-01**: `CREATE FUNCTION` 体遍历

```sql
CREATE OR REPLACE FUNCTION foo(a INTEGER) RETURNS INTEGER AS $$
BEGIN
    PERFORM bar(a);
    RETURN SELECT baz(a);
END;
$$ LANGUAGE plpgsql;
```

**预期**：`walk_statement` 触发 → `visit_pl_block` → `visit_pl_statement`（Perform → walk_statement 触及 baz 的 FunctionCall）、`visit_pl_statement`（Return）。

**TC-P0-02**: `CREATE PROCEDURE` 体遍历 + 过程调用

```sql
CREATE PROCEDURE load_data(p_id INT) AS $$
BEGIN
    insert_log(p_id, 'start');
    data_core.process(p_id);
    insert_log(p_id, 'done');
EXCEPTION
    WHEN OTHERS THEN
        log_error(SQLERRM);
END;
$$;
```

**预期**：`visit_procedure_call` 被调用 3 次（`insert_log`、`data_core.process`、`insert_log`），异常块中的 `log_error` 也能被遍历到。

**TC-P0-03**: `CREATE PACKAGE BODY` 包内过程遍历

```sql
CREATE OR REPLACE PACKAGE BODY pkg_api AS
    PROCEDURE inner_proc IS
    BEGIN
        helper.do_stuff();
    END;

    FUNCTION get_val RETURN NUMBER IS
    BEGIN
        RETURN compute_val();
    END;
END pkg_api;
```

**预期**：`walk_statement` 能进入 `pkg_api` 的 `items`，对 `inner_proc` 和 `get_val` 的 `block` 各触发 `visit_pl_block` 和对应的 `visit_pl_statement` / `visit_procedure_call`。

**TC-P0-04**: 顶层 `CALL` 语句

```sql
CALL schema.run_maintenance();
```

**预期**：`visit_call` 被触发，`call.func_name` 为 `schema.run_maintenance`。

**TC-P0-05**: 嵌套块递归

```sql
BEGIN
    BEGIN
        inner_call();
    END;
    outer_call();
END;
```

**预期**：`visit_procedure_call` 先 `inner_call` 后 `outer_call`。

**TC-P0-06**: `EXECUTE` 动态 SQL

```sql
BEGIN
    EXECUTE IMMEDIATE 'CALL ' || v_proc || '()';
END;
```

**预期**：`visit_pl_statement` 触发（`PlStatement::Execute`），`string_expr` 可被遍历。`parsed_query` 为 None（动态 SQL 无法解析）时不崩溃。

### 5.2 P1 验收用例

**TC-P1-01**: `walk_expr` 完整性 — `BETWEEN` 表达式

```sql
SELECT * FROM t WHERE x BETWEEN 1 AND 10;
```

**预期**：`visit_expr` 对 `BETWEEN` 的 `expr`、`low`、`high` 三个子节点均触发。

**TC-P1-02**: `walk_select` CTE 遍历

```sql
WITH cte AS (SELECT id FROM users WHERE active = 1)
SELECT * FROM cte WHERE id > 100;
```

**预期**：CTE 内的 `SELECT` 和外层 `SELECT` 的 `WHERE` 均被遍历。

**TC-P1-03**: `walk_table_ref` JOIN 遍历

```sql
SELECT * FROM a JOIN b ON a.id = b.aid WHERE b.val > 0;
```

**预期**：`walk_table_ref` 递归进入 JOIN 的左右表和 ON 条件。

---

## 6. 实现影响评估

| 需求项 | 修改文件 | 预估工作量（v2 修正） |
|---|---|---|
| P0-1: PL/pgSQL trait 方法 | `visitor.rs` | ~20 行新增 |
| P0-2: PL/pgSQL walk 函数 | `visitor.rs` | ~180-220 行新增（walk_pl_block ~30 行, walk_pl_statement ~100-120 行, walk_pl_declaration ~50-70 行） |
| P0-3: 修正 walk_statement | `visitor.rs` | ~40 行修改 |
| P0-4: visit_call / visit_procedure_call | `visitor.rs` | ~20 行新增 |
| P1-1: 增强 walk_expr | `visitor.rs` | ~100-130 行新增（补全 ~18 个 Expr 变体 + FunctionCall 子字段） |
| P1-2: 增强 walk_select | `visitor.rs` | ~45-55 行新增 |
| P1-3: 增强 walk_table_ref | `visitor.rs` | ~30-40 行新增 |
| P2-1: 统一 SkipChildren 风格 | `visitor.rs` | ~20 行修改 |
| P2-2: 导出新类型 | `lib.rs` | ~3 行修改 |

**总预估**：`visitor.rs` 从 337 行增长至 ~770-860 行。无其他文件需大规模修改。

---

## 7. v2 修订记录

| 修订项 | v1 描述 | v2 修正 | 原因 |
|---|---|---|---|
| Statement 变体数 | "~175 个" | "182 个" | 实际计数 182 |
| Expr 变体（P1-1） | 列出 NotLike、NotBetween、NotInList、NotInSubquery、IsNotNull、IsNotDistinctFrom、IsDistinctFrom | 移除上述 7 个不存在的变体，说明其通过 `negated` 字段或 `BinaryOp` 表达 | 代码中无独立变体 |
| AnonyBlock.block | "`Option<PlBlock>`" | "`PlBlock`（非 Option）" | 代码实际为非可选 |
| PlExceptionHandler | 直接假设 PlBlock 含 handlers | 明确 `exception_block: Option<PlExceptionBlock>` → `PlExceptionBlock.handlers` | 多一层间接 |
| walk_pl_statement 覆盖 | 仅列出主要变体 | 完整列出全部 31 个 PlStatement 变体的遍历规则 | 确保无遗漏 |
| walk_pl_declaration 覆盖 | 仅提及 NestedProcedure/NestedFunction | 完整列出全部 7 个 PlDeclaration 变体的遍历规则 | 确保无遗漏 |
| walk_expr 补充 | 遗漏 FunctionCall 子字段 | 新增 FunctionCall 的 over/filter/within_group/separator/default/conversion_format 遍历 | 调用图提取关键遗漏 |
| walk_select 补充 | 遗漏 connect_by、distinct_on、window_clause、fetch、into_targets | 新增上述字段的遍历规则 | 完整覆盖 |
| walk_table_ref 补充 | 遗漏 Table.timecapsule | 新增 Table 变体的 timecapsule Expr 遍历 | 完整覆盖 |
| P2-1 | 建议重命名为三态 | 改为统一 SkipChildren 处理风格，不更名 | SkipChildren 语义已正确 |
| 已知限制 | 无 | 新增 PlStatement::Sql 和 PlForAll 无法结构化遍历的说明 | 文档化不可遍历节点 |
| 工作量估算 | ~345-435 行 | ~435-525 行 | 修正遗漏项后更准确 |
