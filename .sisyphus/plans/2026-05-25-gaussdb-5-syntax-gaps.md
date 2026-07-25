# GaussDB 5 类语法缺口修复 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 修复 ogsql-parser 对 GaussDB 5 类尚不支持的 SQL 语法的解析，消除 error-5.txt 中的全部 15 个错误。

**Architecture:** 5 类修复均为 parser 层面的改动，涉及 dml.rs（INSERT）、expr.rs（(+)、IN+UNION、ANY+VALUES）、select.rs（PIVOT/UNPIVOT 上下文检测）。大部分特性已有部分实现，修复以补全为主。

**Tech Stack:** Rust, 递归下降 parser, Pratt expression parser

---

## 现状摘要

| 类别 | 语法 | 现有实现 | 缺口 |
|------|------|----------|------|
| A | `INSERT INTO t (SELECT...)` / `INSERT INTO t (cols) ((SELECT...))` | dml.rs:117-158 已处理 `(SELECT...)`，但仅前瞻 1 个 token | `((SELECT...))` 双括号 + `(SELECT` 被误入列名解析 |
| B | Oracle `(+)` 外连接 | expr.rs:547-576 已检测 `(+)` 并 warn | 某些上下文（如关键字开头的列名 `LANGUAGE(+)`）未触发检测 |
| C | PIVOT / UNPIVOT | select.rs:714-737 + AST TableRef::Pivot/Unpivot 已完整实现 | 子查询 `)` 后的 PIVOT/UNPIVOT 检测未触发 |
| D | `IN ((SELECT...) UNION (SELECT...))` | parse_in_expr 仅检查直接 SELECT/WITH | `(` 后又是 `(` 的情况未处理 |
| E | `ANY(VALUES(...))` | ANY/ALL 已解析 subquery，但 VALUES 不被识别为 subquery 起始 | 需在 ANY 上下文中识别 VALUES |

---

## Task 1: 修复 INSERT INTO table (SELECT ...) — 列名解析歧义

**问题**: `INSERT INTO t (SELECT ...)` 中 `(` 被误识别为列名列表起始，`SELECT` 被当作列名。

**Files:**
- Modify: `src/parser/dml.rs:56-67` (columns 解析逻辑)
- Test: `src/parser/tests.rs`

**Step 1: 在列名解析前添加 SELECT/WITH 前瞻**

在 `parse_insert` 函数中，当前代码（约 line 56）：
```rust
let columns = if self.match_token(&Token::LParen) {
    self.advance();
    let mut cols = vec![self.parse_identifier()?];
    // ...
```

修改为：在 `self.match_token(&Token::LParen)` 为 true 时，先前瞻检查 `(SELECT` 或 `(WITH`，如果是则跳过列名解析（columns 留空），让后续 source 解析逻辑处理 `(SELECT...)`：

```rust
let columns = if self.match_token(&Token::LParen) {
    // Look ahead: if (SELECT or (WITH, this is NOT a column list
    // but a parenthesized INSERT source
    if let Some(Token::Keyword(kw)) = self.tokens.get(self.pos + 1).map(|tws| &tws.token) {
        if *kw == Keyword::SELECT || *kw == Keyword::WITH {
            vec![]  // No column list — let source parsing handle (SELECT...)
        } else {
            // Normal column list parsing
            self.advance();
            let mut cols = vec![self.parse_identifier()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                cols.push(self.parse_identifier()?);
            }
            self.expect_token(&Token::RParen)?;
            cols
        }
    } else {
        // Non-keyword after (, parse as column list
        self.advance();
        let mut cols = vec![self.parse_identifier()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            cols.push(self.parse_identifier()?);
        }
        self.expect_token(&Token::RParen)?;
        cols
    }
} else {
    vec![]
};
```

**Step 2: 修复 INSERT INTO table (cols) ((SELECT...)) 双括号处理**

当前 line 117-158 的 `(SELECT...)` 处理仅前瞻 1 个 token。当遇到 `((SELECT...))` 时，`self.tokens.get(self.pos + 1)` 看到的是 `(` 而非 `SELECT`。

修改 line 117 附近的逻辑，用更灵活的方式检测：不手动前瞻，而是尝试调用 `parse_select_statement()` 并回退：

```rust
} else if self.match_token(&Token::LParen) {
    // Try to parse as parenthesized SELECT/WITH (handles nested parens like ((SELECT...)))
    let save_pos = self.pos;
    let save_err_len = self.errors.len();
    self.advance(); // consume (
    if let Ok(mut select) = self.parse_select_statement() {
        if self.match_token(&Token::RParen) {
            self.advance(); // consume )
            // Check for set operations after )
            select = self.parse_set_operations(select)?;
            InsertSource::Select(Box::new(select))
        } else {
            // Not a parenthesized select — restore and try other interpretations
            self.pos = save_pos;
            self.errors.truncate(save_err_len);
            return Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "VALUES, SELECT, DEFAULT VALUES".to_string(),
                got: format!("{:?}", self.peek()),
            });
        }
    } else {
        // Failed to parse as select — restore and error
        self.pos = save_pos;
        self.errors.truncate(save_err_len);
        return Err(ParserError::UnexpectedToken {
            location: self.current_location(),
            expected: "VALUES, SELECT, DEFAULT VALUES".to_string(),
            got: format!("{:?}", self.peek()),
        });
    }
```

**Step 3: 写测试**

```rust
#[test]
fn test_insert_select_parenthesized() {
    // INSERT INTO table (SELECT ...)
    let sql = "INSERT INTO t1 (SELECT * FROM t2)";
    // should parse without error

    // INSERT INTO table (cols) (SELECT ...)
    let sql2 = "INSERT INTO t1 (a, b) (SELECT x, y FROM t2)";
    // should parse without error

    // INSERT INTO table (cols) ((SELECT ...))
    let sql3 = "INSERT INTO t1 (a, b) ((SELECT x, y FROM t2))";
    // should parse without error (GaussDB compatibility)
}
```

**Step 4: 运行测试验证**
```bash
cargo test test_insert_select_parenthesized
```

**Step 5: Commit**
```bash
git add -A && git commit -m "fix: support INSERT INTO table (SELECT...) and ((SELECT...)) syntax"
```

---

## Task 2: 修复 Oracle `(+)` 外连接在所有表达式上下文中的检测

**问题**: `(+)` 在某些上下文中（如关键字作为列名 `LANGUAGE(+)`，或在复杂表达式右侧 `exchange.coin_code(+)`) 未被正确检测，导致 `expected expression, got RParen`。

**Files:**
- Modify: `src/parser/expr.rs` — `try_postfix_op` 函数中 LParen 分支

**Step 1: 审查现有 `(+)` 检测代码**

当前 expr.rs:547-576 已在 `try_postfix_op` 的 LParen 分支中检测 `(+)`。但存在以下问题：

1. 当 left 是关键字解析的结果（如 `LANGUAGE` 作为 `Keyword` token 而非 `Ident`）时，`try_postfix_op` 可能未被调用
2. `parse_primary` 中对 `(` 的处理可能在 `try_postfix_op` 之前拦截了 token

需要检查：
- `LANGUAGE` 在表达式中如何被解析（是作为 Ident 还是 Keyword？）
- `try_postfix_op` 是否在所有表达式解析路径的 postfix 循环中被调用

**Step 2: 确保 `(+)` 检测覆盖所有路径**

最可靠的方案：在 `parse_primary` 的 `Token::LParen` 分支中**也**添加 `(+)` 检测，作为 fallback：

在 `parse_primary` 中处理 `Token::LParen` 的位置，在尝试解析子查询/表达式之前，先检查是否是 `(+)`：

```rust
Token::LParen => {
    // Check for Oracle outer join (+) syntax
    if let Some(next) = self.tokens.get(self.pos + 1) {
        if matches!(&next.token, Token::Plus) {
            if let Some(next2) = self.tokens.get(self.pos + 2) {
                if matches!(&next2.token, Token::RParen) {
                    // This shouldn't happen in primary position — (+) is postfix
                    // But if it does, treat as no-op with warning
                    let loc = self.current_location();
                    self.advance(); // (
                    self.advance(); // +
                    self.advance(); // )
                    self.add_error(ParserError::Warning {
                        message: "Oracle-style outer join operator '(+)' in unexpected position".to_string(),
                        location: loc,
                    });
                    return self.parse_primary(); // Parse what comes next
                }
            }
        }
    }
    // ... existing LParen handling for subquery/grouped expression
}
```

但主要修复应在 `try_postfix_op`：确保对于 `LANGUAGE(+)` 这种情况，`LANGUAGE` 被解析为 ColumnRef 后，`try_postfix_op` 能正确检测 `(+)`。需要验证 `LANGUAGE` 关键字在表达式上下文中是否被接受为标识符。

如果 `parse_primary` 对 `Keyword(LANGUAGE)` 有特殊处理（如不将其当作列名），则需要确保 `Keyword` 类型的列名也能进入 `try_postfix_op` 的 postfix 循环。

**Step 3: 写测试**

```rust
#[test]
fn test_oracle_outer_join_plus() {
    // Column reference with (+)
    let sql1 = "SELECT * FROM t1, t2 WHERE t1.id = t2.id(+)";
    // Keyword column with (+)
    let sql2 = "SELECT * FROM t1, t2 WHERE LANGUAGE(+) = '02'";
    // Complex expression with (+)
    let sql3 = "SELECT * FROM t1, t2 WHERE t1.code = exchange.coin_code(+)";
}
```

**Step 4: 运行测试验证**
```bash
cargo test test_oracle_outer_join_plus
```

**Step 5: Commit**
```bash
git add -A && git commit -m "fix: Oracle (+) outer join detection in all expression contexts"
```

---

## Task 3: 修复 PIVOT/UNPIVOT 在子查询后的检测

**问题**: `FROM (SELECT ...) PIVOT(...)` — 子查询 `)` 后的 PIVOT 未被检测到，报 `expected RParen, got LParen`。

**Files:**
- Modify: `src/parser/select.rs` — `parse_table_ref` 函数

**Step 1: 定位子查询表引用解析**

在 `parse_table_ref` 中，当解析 `FROM (subquery)` 时，子查询的 `)` 被消耗后，需要继续检查 PIVOT/UNPIVOT。

当前代码在 select.rs:714-737 已有 PIVOT/UNPIVOT 检测逻辑，但可能在子查询解析路径中未被调用。

需要找到 `TableRef::Subquery` 的解析代码，确认在解析完子查询后是否调用了 PIVOT/UNPIVOT 检测。

**Step 2: 在子查询解析后添加 PIVOT/UNPIVOT 检测**

在 `parse_table_ref` 中构建 `TableRef::Subquery` 后，添加与普通表引用相同的 PIVOT/UNPIVOT 后续检测：

```rust
// After building TableRef::Subquery { query, alias, lateral }
// Check for PIVOT/UNPIVOT (same logic as lines 714-737)
if self.match_ident_str("PIVOT") {
    self.advance();
    let pivot = self.parse_pivot()?;
    left = TableRef::Pivot { source: Box::new(left), pivot };
} else if self.match_ident_str("UNPIVOT") {
    self.advance();
    let unpivot = self.parse_unpivot()?;
    left = TableRef::Unpivot { source: Box::new(left), unpivot };
}
```

**Step 3: 写测试**

```rust
#[test]
fn test_pivot_after_subquery() {
    let sql = "SELECT * FROM (SELECT a, b FROM t) PIVOT(MAX(b) FOR a IN ('x', 'y'))";
    // should parse without error
}

#[test]
fn test_unpivot_after_subquery() {
    let sql = "SELECT * FROM (SELECT * FROM t1 WHERE rownum = 1) UNPIVOT(val FOR name IN(col1, col2))";
    // should parse without error
}
```

**Step 4: 运行测试验证**
```bash
cargo test test_pivot_after_subquery test_unpivot_after_subquery
```

**Step 5: Commit**
```bash
git add -A && git commit -m "fix: PIVOT/UNPIVOT detection after subquery in FROM clause"
```

---

## Task 4: 修复 IN 子句中的 UNION 支持

**问题**: `IN ((SELECT...) UNION (SELECT...))` — `parse_in_expr` 在 `(` 后只检查直接的 `SELECT`/`WITH`，遇到 `((` 时无法识别为子查询。

**Files:**
- Modify: `src/parser/expr.rs` — `parse_in_expr` 函数 (约 line 651-673)

**Step 1: 修改 parse_in_expr 支持 UNION 子查询**

当前代码：
```rust
fn parse_in_expr(&mut self, left: Expr, negated: bool) -> Result<Expr, ParserError> {
    self.expect_token(&Token::LParen)?;
    if self.match_keyword(Keyword::SELECT) || self.match_keyword(Keyword::WITH) {
        let subquery = self.parse_select_statement()?;
        self.expect_token(&Token::RParen)?;
        return Ok(Expr::InSubquery { ... });
    }
    // ... fall through to expression list
}
```

修改为：在 `(` 后不仅检查直接 `SELECT`/`WITH`，也检查 `(` (嵌套括号子查询)。使用 `parse_select_statement()` 的已有能力（它已经能处理 `select_with_parens` 即 `(SELECT...) UNION (SELECT...)`)：

```rust
fn parse_in_expr(&mut self, left: Expr, negated: bool) -> Result<Expr, ParserError> {
    self.expect_token(&Token::LParen)?;

    // Check if this is a subquery (SELECT, WITH, or parenthesized subquery)
    let is_subquery = self.match_keyword(Keyword::SELECT)
        || self.match_keyword(Keyword::WITH)
        || self.match_token(&Token::LParen);  // (SELECT...) or ((SELECT...) UNION (...))

    if is_subquery {
        let save_pos = self.pos;
        let save_err_len = self.errors.len();
        if let Ok(subquery) = self.parse_select_statement() {
            if self.match_token(&Token::RParen) {
                self.advance();
                return Ok(Expr::InSubquery {
                    expr: Box::new(left),
                    subquery: Box::new(subquery),
                    negated,
                });
            }
        }
        // Not a valid subquery — restore and try as expression list
        self.pos = save_pos;
        self.errors.truncate(save_err_len);
    }

    // Expression list
    let mut list = vec![self.parse_expr()?];
    while self.match_token(&Token::Comma) {
        self.advance();
        list.push(self.parse_expr()?);
    }
    self.expect_token(&Token::RParen)?;
    Ok(Expr::InList { expr: Box::new(left), list, negated })
}
```

**Step 2: 写测试**

```rust
#[test]
fn test_in_union_subquery() {
    let sql = "SELECT * FROM t WHERE id IN ((SELECT id FROM t1) UNION (SELECT id FROM t2))";
    // should parse without error
}

#[test]
fn test_in_parenthesized_select() {
    let sql = "SELECT * FROM t WHERE id IN (SELECT id FROM t1)";
    // should still parse correctly (regression test)
}
```

**Step 3: 运行测试验证**
```bash
cargo test test_in_union_subquery test_in_parenthesized_select
```

**Step 4: Commit**
```bash
git add -A && git commit -m "fix: support UNION inside IN subquery"
```

---

## Task 5: 修复 ANY(VALUES(...)) 语法

**问题**: `0 <> ANY(VALUES(expr1), (expr2), ...)` — VALUES 不被识别为 ANY 内的子查询起始。

**Files:**
- Modify: `src/parser/expr.rs` — ANY/ALL 处理逻辑 (约 line 84-147)

**Step 1: 在 ANY/ALL 的子查询解析中添加 VALUES 支持**

当前代码在 line 84-147 处理 `expr op ANY/ALL/SOME (subquery)`。解析 `(SELECT...)` 后如果有更多内容（如逗号），会失败。

修改：在尝试 `parse_select_statement` 之前，也检查 `VALUES` 关键字，将其构造为 `values_clause` 类型的 SelectStatement：

```rust
// In the ANY/ALL handler, after consuming ( and before parse_select_statement:
if self.match_keyword(Keyword::VALUES) {
    // ANY(VALUES(expr), (expr), ...) — parse VALUES as a subquery
    self.advance(); // consume VALUES
    let mut rows = Vec::new();
    self.expect_token(&Token::LParen)?;
    let mut row = vec![self.parse_expr()?];
    while self.match_token(&Token::Comma) {
        self.advance();
        row.push(self.parse_expr()?);
    }
    self.expect_token(&Token::RParen)?;
    rows.push(row);
    while self.match_token(&Token::Comma) {
        self.advance();
        self.expect_token(&Token::LParen)?;
        let mut row = vec![self.parse_expr()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            row.push(self.parse_expr()?);
        }
        self.expect_token(&Token::RParen)?;
        rows.push(row);
    }
    self.expect_token(&Token::RParen)?;
    // Construct a SelectStatement wrapping the VALUES
    left = Expr::ScalarSublink {
        expr: Box::new(left),
        op: op_str,
        sublink_type,
        subquery: Box::new(SelectStatement {
            targets: vec![],
            from: vec![],
            // Use ValuesStatement or equivalent to represent VALUES(...)
            // ... depends on existing AST structure
            ..Default::default()  // or construct manually
        }),
    };
    continue;
}
```

**注意**: 需要检查现有 AST 是否有 `ValuesStatement` 或类似类型来表示 `VALUES(...)` 子查询。如果已有 `TableRef::Values`，可以复用。如果没有，需要用 `SelectStatement` + 标记字段来表示。

**Step 2: 写测试**

```rust
#[test]
fn test_any_values() {
    let sql = "SELECT * FROM t WHERE 0 <> ANY(VALUES(1), (2), (3))";
    // should parse without error

    let sql2 = "SELECT * FROM vv WHERE (0 <> ANY(VALUES(to_number(REPLACE(vv.deal_amount1, ',', ''))), (to_number(REPLACE(vv.deal_amount2, ',', '')))))";
    // real-world case from error-5.txt
}
```

**Step 3: 运行测试验证**
```bash
cargo test test_any_values
```

**Step 4: Commit**
```bash
git add -A && git commit -m "fix: support ANY(VALUES(...)) syntax"
```

---

## Task 6: 回归测试 + error-5.txt 验证

**Files:**
- Test: `src/parser/tests.rs`

**Step 1: 添加 error-5.txt 中全部 15 个错误对应的测试用例**

```rust
#[test]
fn test_error5_all_cases() {
    // Category A: INSERT (SELECT...) variants
    parse("INSERT INTO dat_fax_receive_info (col1) ((SELECT v_fax_seq FROM t))");
    parse("INSERT INTO par_fund_accnt_relation (SELECT v_row.seq_id, v_row.fund_code FROM t)");
    parse("INSERT INTO FUNDCODE_PRIV_FUNDKIND (SELECT p_targetuser_id, role_id FROM t)");

    // Category B: (+) outer join
    parse("SELECT * FROM t WHERE t.coin_code = exchange.coin_code(+)");
    parse("SELECT * FROM t WHERE LANGUAGE(+) = '02'");

    // Category C: PIVOT/UNPIVOT
    parse("SELECT * FROM (SELECT * FROM t WHERE user_code = p) PIVOT(MIN(remark12) FOR remark11 IN ('1','2'))");
    parse("SELECT * FROM (SELECT * FROM t WHERE rownum = 1) UNPIVOT(val FOR name IN(col1, col2))");

    // Category D: UNION in IN
    parse("SELECT * FROM t WHERE code IN ((SELECT code FROM t1) UNION (SELECT code FROM t2))");

    // Category E: ANY(VALUES(...))
    parse("SELECT * FROM t WHERE 0 <> ANY(VALUES(1), (2), (3))");
}
```

**Step 2: 运行全部测试**
```bash
cargo test
```

**Step 3: 对 error-5.txt 中涉及的 10 个 SQL 文件运行 parser 验证**
```bash
cargo run -- validate -f <each-sql-file>
```

**Step 4: Commit**
```bash
git add -A && git commit -m "test: add regression tests for GaussDB 5 syntax gap fixes"
```

---

## 实施优先级

| Task | 难度 | 影响面 | 建议顺序 |
|------|------|--------|----------|
| Task 1 (INSERT) | 中 | 5 个错误 | 1 |
| Task 4 (IN+UNION) | 低 | 1 个错误 | 2 |
| Task 5 (ANY+VALUES) | 中 | 2 个错误 | 3 |
| Task 3 (PIVOT/UNPIVOT) | 低 | 4 个错误 | 4 |
| Task 2 ((+) 外连接) | 中 | 2 个错误 | 5 |
| Task 6 (回归测试) | 低 | 验证全部 | 6 |
