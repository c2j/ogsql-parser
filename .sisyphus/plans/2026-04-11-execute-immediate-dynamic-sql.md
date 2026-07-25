# GaussDB EXECUTE IMMEDIATE 动态 SQL 解析 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 完整支持 GaussDB PL/pgSQL 中的 `EXECUTE IMMEDIATE ... INTO ... USING [IN|OUT|IN OUT] ...` 语法解析，以及 `FOR ... IN EXECUTE ... USING ... LOOP` 游标形式。

**Architecture:** 在现有 PL/pgSQL 解析框架内扩展。AST 层修改 `PlExecuteStmt` 以支持多 INTO 目标和带模式的 USING 参数；解析器层重写 `parse_pl_execute()` 和增强 `parse_pl_for_kind()`；格式化器层更新输出逻辑。所有 serde 变更使用 `#[serde(default)]` 保证向后兼容。

**Tech Stack:** Rust 2021, serde, thiserror

---

## GaussDB 语法参考

```
-- 非查询
EXECUTE IMMEDIATE dynamic_string
  USING [IN | OUT | IN OUT] bind_arg [, [IN | OUT | IN OUT] bind_arg ...];

-- 查询
EXECUTE IMMEDIATE dynamic_select_string
  INTO define_var [, define_var] ...
  [USING [IN | OUT | IN OUT] bind_arg [, [IN | OUT | IN OUT] bind_arg ...]];

-- FOR-IN-EXECUTE 游标
FOR rec IN EXECUTE dynamic_string [USING ...] LOOP ... END LOOP;
```

**关键细节：**
- `IMMEDIATE` 关键字：在 GaussDB 中是可选的，`EXECUTE 'SELECT 1'` 和 `EXECUTE IMMEDIATE 'SELECT 1'` 等价
- 占位符 `:1`, `:name` 等在**字符串字面量内部**，不需要 tokenizer 修改
- USING 参数的默认模式是 `IN`（可省略）
- INTO 和 USING 可以同时出现（INTO 在前，USING 在后）
- `EXECUTE expr || expr` 形式：字符串表达式可以是拼接表达式，不仅仅是字面量

---

## Task 1: 扩展 AST 类型定义

**Files:**
- Modify: `src/ast/plpgsql.rs:340-345` (PlExecuteStmt)
- Modify: `src/ast/plpgsql.rs:278-298` (PlForKind::Query)

**Step 1: 新增 PlUsingMode 和 PlUsingArg 类型**

在 `src/ast/plpgsql.rs` 的 `PlExecuteStmt` 定义**之前**（约第 339 行），添加：

```rust
/// Parameter passing mode for EXECUTE IMMEDIATE ... USING arguments.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PlUsingMode {
    In,
    Out,
    InOut,
}

/// A single argument in a USING clause with its passing mode.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlUsingArg {
    pub mode: PlUsingMode,
    pub argument: crate::ast::Expr,
}
```

**Step 2: 修改 PlExecuteStmt 结构体**

将第 341-345 行的 `PlExecuteStmt` 从：

```rust
pub struct PlExecuteStmt {
    pub string_expr: crate::ast::Expr,
    pub into_target: Option<crate::ast::Expr>,
    pub using_args: Vec<crate::ast::Expr>,
}
```

改为：

```rust
pub struct PlExecuteStmt {
    /// Whether the IMMEDIATE keyword was present
    pub immediate: bool,
    /// The dynamic SQL string expression (may be a concatenation)
    pub string_expr: crate::ast::Expr,
    /// INTO target variables for query results
    #[serde(default)]
    pub into_targets: Vec<crate::ast::Expr>,
    /// USING arguments with IN/OUT/INOUT mode
    #[serde(default)]
    pub using_args: Vec<PlUsingArg>,
}
```

**Serde 兼容性说明：**
- `into_target: Option<Expr>` → `into_targets: Vec<Expr>`：旧 JSON 中 `"into_target": null` 被忽略（字段名变了，旧字段不匹配），新字段 `into_targets` 使用 `#[serde(default)]` 反序列化时得到空 Vec。由于旧值始终为 `None`，不会丢失数据。
- `using_args: Vec<Expr>` → `Vec<PlUsingArg>`：旧 JSON 中 `"using_args": []` 反序列化为空 Vec（因为 `#[serde(default)]`）。由于旧值始终为空数组，不会有类型冲突。
- `immediate: bool`：新增字段使用 `#[serde(default)]`，旧 JSON 反序列化时得到 `false`。

**Step 3: 修改 PlForKind::Query 增加 using_args**

将第 279-298 行的 `PlForKind` 从：

```rust
pub enum PlForKind {
    Range { ... },
    Query {
        query: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        parsed_query: Option<Box<crate::ast::Statement>>,
    },
    Cursor { ... },
}
```

改为：

```rust
pub enum PlForKind {
    Range {
        low: crate::ast::Expr,
        high: crate::ast::Expr,
        step: Option<crate::ast::Expr>,
        reverse: bool,
    },
    Query {
        query: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        parsed_query: Option<Box<crate::ast::Statement>>,
        /// USING arguments for dynamic queries (FOR ... IN EXECUTE ... USING ...)
        #[serde(default)]
        using_args: Vec<PlUsingArg>,
    },
    Cursor {
        cursor_name: String,
        arguments: Vec<crate::ast::Expr>,
    },
}
```

**Step 4: 编译验证 AST 改动**

Run: `cargo check 2>&1 | head -50`

预期：编译报错指向所有使用 `PlExecuteStmt` 和 `PlForKind` 的地方（解析器和格式化器），这正是后续 Task 需要修改的位置。记录这些报错位置。

**Step 5: Commit**

```bash
git add src/ast/plpgsql.rs
git commit -m "feat(ast): extend PlExecuteStmt and PlForKind for EXECUTE IMMEDIATE INTO USING support"
```

---

## Task 2: 重写 parse_pl_execute()

**Files:**
- Modify: `src/parser/plpgsql.rs:1118-1128` (parse_pl_execute)

**Step 1: 更新现有测试以匹配新 AST**

在 `src/parser/tests.rs` 中修改第 443-454 行的 `test_plpgsql_execute`：

```rust
#[test]
fn test_plpgsql_execute() {
    let block = parse_do_block("DO $$ BEGIN EXECUTE 'SELECT 1'; END $$");
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(
                matches!(&e.string_expr, Expr::Literal(Literal::String(s)) if s.contains("SELECT 1"))
            );
            assert!(!e.immediate);  // 没有 IMMEDIATE 关键字
            assert!(e.into_targets.is_empty());
            assert!(e.using_args.is_empty());
        }
        _ => panic!("expected Execute"),
    }
}
```

**Step 2: 添加新测试用例（先写测试，后实现）**

在 `test_plpgsql_execute` 之后添加：

```rust
#[test]
fn test_plpgsql_execute_immediate_simple() {
    // EXECUTE IMMEDIATE 'INSERT INTO t VALUES(:1, :2)' USING a, b;
    let block = parse_do_block(
        "DO $$ BEGIN EXECUTE IMMEDIATE 'INSERT INTO t VALUES(:1, :2)' USING a, b; END $$"
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert!(e.into_targets.is_empty());
            assert_eq!(e.using_args.len(), 2);
            assert!(matches!(e.using_args[0].mode, PlUsingMode::In));  // 默认 IN
            assert!(matches!(e.using_args[1].mode, PlUsingMode::In));
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_immediate_into() {
    // EXECUTE IMMEDIATE 'SELECT count(*) FROM t' INTO v_count;
    let block = parse_do_block(
        "DO $$ BEGIN EXECUTE IMMEDIATE 'SELECT count(*) FROM t' INTO v_count; END $$"
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert_eq!(e.into_targets.len(), 1);
            assert!(e.using_args.is_empty());
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_immediate_into_using() {
    // EXECUTE IMMEDIATE 'SELECT name FROM t WHERE id=:1' INTO v_name USING IN v_id;
    let block = parse_do_block(
        "DO $$ BEGIN EXECUTE IMMEDIATE 'SELECT name FROM t WHERE id=:1' INTO v_name USING IN v_id; END $$"
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert_eq!(e.into_targets.len(), 1);
            assert_eq!(e.using_args.len(), 1);
            assert!(matches!(e.using_args[0].mode, PlUsingMode::In));
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_immediate_using_in_out() {
    // EXECUTE IMMEDIATE stmt USING OUT v1, IN v2, IN OUT v3;
    let block = parse_do_block(
        "DO $$ DECLARE stmt VARCHAR2(200); v1 INT; v2 INT; v3 INT; BEGIN EXECUTE IMMEDIATE stmt USING OUT v1, IN v2, IN OUT v3; END $$"
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert!(e.into_targets.is_empty());
            assert_eq!(e.using_args.len(), 3);
            assert!(matches!(e.using_args[0].mode, PlUsingMode::Out));
            assert!(matches!(e.using_args[1].mode, PlUsingMode::In));
            assert!(matches!(e.using_args[2].mode, PlUsingMode::InOut));
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_immediate_multi_into() {
    // EXECUTE IMMEDIATE 'SELECT name, salary FROM t WHERE id=:1' INTO v_name, v_salary USING v_id;
    let block = parse_do_block(
        "DO $$ BEGIN EXECUTE IMMEDIATE 'SELECT name, salary FROM t WHERE id=:1' INTO v_name, v_salary USING v_id; END $$"
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert_eq!(e.into_targets.len(), 2);
            assert_eq!(e.using_args.len(), 1);
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_concat_expr() {
    // EXECUTE IMMEDIATE 'ALTER TABLE ' || tab_name || ' ADD COLUMN c INT';
    let block = parse_do_block(
        "DO $$ BEGIN EXECUTE IMMEDIATE 'ALTER TABLE ' || tab_name || ' ADD COLUMN c INT'; END $$"
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert!(matches!(e.string_expr, Expr::BinaryOp { .. }));
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_for_in_execute() {
    // FOR rec IN EXECUTE 'SELECT * FROM ' || tab_name LOOP ... END LOOP;
    let block = parse_do_block(
        "DO $$ BEGIN FOR rec IN EXECUTE 'SELECT * FROM ' || tab_name LOOP NULL; END LOOP; END $$"
    );
    match &block.body[0] {
        PlStatement::For(f) => match &f.kind {
            PlForKind::Query { query, using_args, .. } => {
                assert!(query.contains("EXECUTE"));
                assert!(using_args.is_empty());
            }
            _ => panic!("expected Query kind"),
        },
        _ => panic!("expected For"),
    }
}

#[test]
fn test_plpgsql_for_in_execute_using() {
    // FOR rec IN EXECUTE 'SELECT * FROM t WHERE id=:1' USING v_id LOOP NULL; END LOOP;
    let block = parse_do_block(
        "DO $$ BEGIN FOR rec IN EXECUTE 'SELECT * FROM t WHERE id=:1' USING v_id LOOP NULL; END LOOP; END $$"
    );
    match &block.body[0] {
        PlStatement::For(f) => match &f.kind {
            PlForKind::Query { using_args, .. } => {
                assert_eq!(using_args.len(), 1);
            }
            _ => panic!("expected Query kind"),
        },
        _ => panic!("expected For"),
    }
}
```

**Step 3: 运行测试确认失败**

Run: `cargo test test_plpgsql_execute 2>&1 | tail -20`

预期：现有测试 `test_plpgsql_execute` 失败（AST 结构变了），新测试全部编译失败或 panic。

**Step 4: 重写 parse_pl_execute()**

将 `src/parser/plpgsql.rs` 第 1118-1128 行从：

```rust
fn parse_pl_execute(&mut self) -> Result<PlStatement, ParserError> {
    self.advance();
    let string_expr = self.parse_expr()?;
    self.try_consume_semicolon();
    Ok(PlStatement::Execute(PlExecuteStmt {
        string_expr,
        into_target: None,
        using_args: Vec::new(),
    }))
}
```

改为：

```rust
fn parse_pl_execute(&mut self) -> Result<PlStatement, ParserError> {
    self.advance(); // consume "execute"

    // Optional IMMEDIATE keyword
    let immediate = self.try_consume_ident_str("immediate");

    // Parse the dynamic SQL string expression
    let string_expr = self.parse_expr()?;

    // Optional INTO var [, var] ...
    let mut into_targets = Vec::new();
    if self.match_ident_str("into") {
        self.advance();
        loop {
            into_targets.push(self.parse_expr()?);
            if self.match_token(&Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }
    }

    // Optional USING [IN | OUT | IN OUT] arg [, [IN | OUT | IN OUT] arg] ...
    let mut using_args = Vec::new();
    if self.match_ident_str("using") {
        self.advance();
        loop {
            let mode = if self.match_ident_str("out") {
                self.advance();
                if self.match_ident_str("in") {
                    // "OUT IN" — unlikely but handle: treat as IN OUT
                    // Actually GaussDB syntax is "IN OUT", not "OUT IN"
                    // Let's re-check: the syntax is IN, OUT, or IN OUT
                    // "out" followed by "in" shouldn't happen in valid SQL
                    PlUsingMode::Out
                } else {
                    PlUsingMode::Out
                }
            } else if self.match_ident_str("in") {
                self.advance();
                if self.match_ident_str("out") {
                    self.advance();
                    PlUsingMode::InOut
                } else {
                    PlUsingMode::In
                }
            } else {
                // Default is IN
                PlUsingMode::In
            };
            using_args.push(PlUsingArg {
                mode,
                argument: self.parse_expr()?,
            });
            if self.match_token(&Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }
    }

    self.try_consume_semicolon();

    Ok(PlStatement::Execute(PlExecuteStmt {
        immediate,
        string_expr,
        into_targets,
        using_args,
    }))
}
```

**Step 5: 在文件顶部确保导入 PlUsingMode 和 PlUsingArg**

检查 `src/parser/plpgsql.rs` 的 `use` 块，确保引入了新增的类型。现有导入应类似：

```rust
use crate::ast::plpgsql::*;
```

如果是通配符导入则无需修改。否则需添加 `PlUsingMode`, `PlUsingArg` 到导入列表。

**Step 6: 运行测试验证**

Run: `cargo test test_plpgsql_execute 2>&1 | tail -30`

预期：所有 `test_plpgsql_execute*` 测试通过。

**Step 7: Commit**

```bash
git add src/parser/plpgsql.rs src/parser/tests.rs
git commit -m "feat(parser): rewrite parse_pl_execute for EXECUTE IMMEDIATE INTO USING support"
```

---

## Task 3: 增强 FOR-IN-EXECUTE 支持 USING 子句

**Files:**
- Modify: `src/parser/plpgsql.rs:898-924` (parse_pl_for_kind 中的 execute 分支)

**Step 1: 添加 FOR-IN-EXECUTE-USING 测试（已在 Task 2 添加）**

测试 `test_plpgsql_for_in_execute` 和 `test_plpgsql_for_in_execute_using` 已在 Task 2 中添加。

**Step 2: 修改 parse_pl_for_kind() 的 execute 分支**

将第 916-924 行的 else 分支（`// "execute" — dynamic SQL`）从：

```rust
} else {
    // "execute" — dynamic SQL, skip structured parse
    (self.collect_until_ident_str("loop")?, None)
}
```

改为：

```rust
} else {
    // "execute" — dynamic SQL
    // Collect the expression and optional USING clause up to LOOP
    let query = self.collect_until_ident_str("loop")?;
    // Note: We store the full text including USING in the query string.
    // For structured USING args, we'd need a more complex approach.
    // TODO: Parse USING clause into structured using_args if needed.
    (query, None)
}
```

**重要设计决策：** `FOR ... IN EXECUTE ... USING ... LOOP` 中的 USING 参数，由于当前 `PlForKind::Query` 的 `query` 字段是原始字符串（`String`），且 `collect_until_ident_str("loop")` 会收集 `EXECUTE` 到 `LOOP` 之间的所有原始文本，所以 USING 子句自然包含在 query 字符串中。

对于结构化的 `using_args` 字段，在 `collect_until_ident_str` 之后已经无法回溯解析（因为使用的是原始文本收集而非 token 解析）。因此有两种策略：

**策略 A（推荐，简单）：** `PlForKind::Query` 的 `using_args` 留空，USING 信息保留在 `query` 字符串中。与现有 PERFORM 的处理方式一致。

**策略 B（完整）：** 在 `FOR ... IN EXECUTE` 分支中使用 token 级解析而非 `collect_until_ident_str`，类似 `parse_pl_execute` 的做法。这需要更大的重构。

**本计划采用策略 A。** Task 2 中 `test_plpgsql_for_in_execute_using` 测试的断言需要调整为：

```rust
#[test]
fn test_plpgsql_for_in_execute_using() {
    // FOR rec IN EXECUTE 'SELECT * FROM t WHERE id=:1' USING v_id LOOP NULL; END LOOP;
    let block = parse_do_block(
        "DO $$ BEGIN FOR rec IN EXECUTE 'SELECT * FROM t WHERE id=:1' USING v_id LOOP NULL; END LOOP; END $$"
    );
    match &block.body[0] {
        PlStatement::For(f) => match &f.kind {
            PlForKind::Query { query, using_args, .. } => {
                // USING is captured in query string (strategy A)
                assert!(query.to_lowercase().contains("using"));
                assert!(using_args.is_empty());  // structured args not parsed for FOR-IN-EXECUTE
            }
            _ => panic!("expected Query kind"),
        },
        _ => panic!("expected For"),
    }
}
```

**Step 3: 运行测试**

Run: `cargo test test_plpgsql_for_in 2>&1 | tail -20`

预期：所有 FOR-IN-EXECUTE 测试通过。

**Step 4: Commit**

```bash
git add src/parser/plpgsql.rs src/parser/tests.rs
git commit -m "feat(parser): update FOR-IN-EXECUTE comment for USING support"
```

---

## Task 4: 更新格式化器

**Files:**
- Modify: `src/formatter.rs:1871-1885` (format_pl_statement 的 Execute 分支)

**Step 1: 添加格式化往返测试**

在 `src/parser/tests.rs` 中添加：

```rust
#[test]
fn test_execute_immediate_roundtrip() {
    use crate::SqlFormatter;

    let cases = vec![
        (
            "DO $$ BEGIN EXECUTE 'SELECT 1'; END $$",
            "DO $$ BEGIN EXECUTE 'SELECT 1'; END $$"
        ),
        (
            "DO $$ BEGIN EXECUTE IMMEDIATE 'INSERT INTO t VALUES(:1, :2)' USING a, b; END $$",
            "DO $$ BEGIN EXECUTE IMMEDIATE 'INSERT INTO t VALUES(:1, :2)' USING a, b; END $$"
        ),
        (
            "DO $$ BEGIN EXECUTE IMMEDIATE 'SELECT count(*) FROM t' INTO v_count; END $$",
            "DO $$ BEGIN EXECUTE IMMEDIATE 'SELECT count(*) FROM t' INTO v_count; END $$"
        ),
        (
            "DO $$ BEGIN EXECUTE IMMEDIATE 'SELECT name FROM t WHERE id=:1' INTO v_name USING IN v_id; END $$",
            "DO $$ BEGIN EXECUTE IMMEDIATE 'SELECT name FROM t WHERE id=:1' INTO v_name USING IN v_id; END $$"
        ),
        (
            "DO $$ BEGIN EXECUTE IMMEDIATE stmt USING OUT v1, IN v2, IN OUT v3; END $$",
            "DO $$ BEGIN EXECUTE IMMEDIATE stmt USING OUT v1, IN v2, IN OUT v3; END $$"
        ),
    ];

    let formatter = SqlFormatter::new();
    for (input, expected_output) in cases {
        let tokens = crate::Tokenizer::new(input).tokenize().unwrap();
        let stmts = crate::parser::Parser::new(tokens).parse().unwrap();
        let output = formatter.format_statement(&stmts[0]);
        assert_eq!(output, expected_output, "roundtrip failed for: {}", input);
    }
}
```

**Step 2: 重写格式化器的 Execute 分支**

将 `src/formatter.rs` 第 1871-1885 行从：

```rust
PlStatement::Execute(e) => {
    let mut s = format!(
        "{} {}",
        self.kw("EXECUTE"),
        self.format_expr(&e.string_expr)
    );
    if let Some(ref into) = e.into_target {
        s.push_str(&format!(" {} {}", self.kw("INTO"), self.format_expr(into)));
    }
    if !e.using_args.is_empty() {
        let args: Vec<String> =
            e.using_args.iter().map(|a| self.format_expr(a)).collect();
        s.push_str(&format!(" {} {}", self.kw("USING"), args.join(", ")));
    }
    format!("{};", s)
}
```

改为：

```rust
PlStatement::Execute(e) => {
    let mut s = if e.immediate {
        format!("{} {}", self.kw("EXECUTE"), self.kw("IMMEDIATE"))
    } else {
        self.kw("EXECUTE").to_string()
    };
    s.push_str(&format!(" {}", self.format_expr(&e.string_expr)));
    if !e.into_targets.is_empty() {
        let targets: Vec<String> =
            e.into_targets.iter().map(|t| self.format_expr(t)).collect();
        s.push_str(&format!(" {} {}", self.kw("INTO"), targets.join(", ")));
    }
    if !e.using_args.is_empty() {
        let args: Vec<String> = e.using_args.iter().map(|a| {
            let mode = match a.mode {
                PlUsingMode::In => format!("{} ", self.kw("IN")),
                PlUsingMode::Out => format!("{} ", self.kw("OUT")),
                PlUsingMode::InOut => format!("{} {} ", self.kw("IN"), self.kw("OUT")),
            };
            format!("{}{}", mode, self.format_expr(&a.argument))
        }).collect();
        s.push_str(&format!(" {} {}", self.kw("USING"), args.join(", ")));
    }
    format!("{};", s)
}
```

**Step 3: 确保格式化器导入了 PlUsingMode**

检查 `src/formatter.rs` 的 `use` 块，确保有：

```rust
use crate::ast::plpgsql::{PlBlock, PlStatement, ..., PlUsingMode};
```

**Step 4: 运行往返测试**

Run: `cargo test test_execute_immediate_roundtrip 2>&1 | tail -20`

预期：测试通过。

**Step 5: Commit**

```bash
git add src/formatter.rs src/parser/tests.rs
git commit -m "feat(formatter): update EXECUTE IMMEDIATE formatting with INTO and USING support"
```

---

## Task 5: 全量回归测试

**Files:**
- No modifications

**Step 1: 运行全部单元测试**

Run: `cargo test 2>&1 | tail -30`

预期：所有 230+ 测试通过。如果有因为 `PlExecuteStmt` 字段名变更导致的 JSON 反序列化测试失败，需要检查并修复。

**Step 2: 运行回归测试**

Run: `cargo run --example regression 2>&1 | tail -20`

预期：1409/1409 通过。EXECUTE 语句在回归测试中的使用通常在 PL/pgSQL 块内部，且当前解析只捕获表达式不捕获子句，改动后行为等价（旧格式 `into_target: None` + `using_args: []` 变成新格式 `into_targets: []` + `using_args: []`）。

**Step 3: 检查 LSP 诊断**

Run: `cargo clippy 2>&1 | tail -20`

预期：无新警告。

**Step 4: Commit（如有修复）**

如果有任何需要修复的问题：

```bash
git add -A
git commit -m "fix: resolve test regressions from EXECUTE IMMEDIATE changes"
```

---

## 影响范围总结

| 文件 | 改动类型 | 改动量 |
|------|---------|--------|
| `src/ast/plpgsql.rs` | 修改 PlExecuteStmt + 新增 PlUsingMode/PlUsingArg + 修改 PlForKind::Query | ~40行 |
| `src/parser/plpgsql.rs` | 重写 parse_pl_execute() + 更新 FOR-IN-EXECUTE 注释 | ~60行 |
| `src/parser/tests.rs` | 修改现有测试 + 新增 8 个测试用例 + 1 个往返测试 | ~120行 |
| `src/formatter.rs` | 重写 Execute 格式化分支 | ~25行 |
| **合计** | **4 个文件** | **~245行** |

## 不需要改动的文件

- `src/token/tokenizer.rs` — 占位符 `:N` 在字符串字面量内部
- `src/token/mod.rs` — 无需新 Token 类型
- `src/token/keyword.rs` — `IMMEDIATE` 已定义
- `src/parser/expr.rs` — USING 参数用现有表达式解析
- `src/parser/mod.rs` — 无需修改分发逻辑
- `src/ast/visitor.rs` — Execute 已通过 PlStatement 覆盖
- `Cargo.toml` — 无新依赖
