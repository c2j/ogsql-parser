# AST Reverse Engineering: JSON → SQL Round-Trip

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Modify the AST, parser, and formatter to enable lossless semantic JSON → SQL reverse engineering without relying on `sql_text`.

**Architecture:** Expand `Literal` enum to preserve string literal type markers (B'/E'/N'/X'/$$). Add `serde::Deserialize` to all AST types. Refactor PL/pgSQL `String` fields to structured `Expr` types. Improve DDL type representations.

**Tech Stack:** Rust, serde (Serialize + Deserialize), existing recursive-descent parser + Pratt expression parser + SqlFormatter.

**Baseline:** 188 passing tests, 0 failures. Every task MUST maintain this.

---

## P0: Literal Expansion + Deserialize (Unblocks DML/DDL round-trip)

### Task 1: Expand `Literal` enum with string type variants

**Files:**
- Modify: `src/token/mod.rs:63-64` (Token::DollarString — add tag field)
- Modify: `src/token/tokenizer.rs:632-637` (preserve dollar-quote tag)
- Modify: `src/ast/mod.rs:569-576` (Literal enum)
- Modify: `src/parser/expr.rs:257-280` (literal parsing)
- Modify: `src/formatter.rs:517-536` (format_literal + quote_string)

**IMPORTANT:** The tokenizer currently discards the dollar-quote tag (`$tag$body$tag$` → `DollarString("body")`). We must fix this at the tokenizer level to enable full round-trip.

**Step 1: Add test for special literal round-trip**

Add to `src/parser/tests.rs`:

```rust
// ========== Literal Type Preservation Tests ==========

#[test]
fn test_bit_string_literal() {
    let stmt = parse_one("SELECT B'10101'");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, None) => {
                    assert!(matches!(expr, Expr::Literal(Literal::BitString(s)) if s == "10101"));
                }
                _ => panic!("expected expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_hex_string_literal() {
    let stmt = parse_one("SELECT X'FF00'");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, None) => {
                    assert!(matches!(expr, Expr::Literal(Literal::HexString(s)) if s == "FF00"));
                }
                _ => panic!("expected expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_escape_string_literal() {
    let stmt = parse_one("SELECT E'hello\\nworld'");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, None) => {
                    assert!(matches!(expr, Expr::Literal(Literal::EscapeString(s)) if s == "hello\\nworld"));
                }
                _ => panic!("expected expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_national_string_literal() {
    let stmt = parse_one("SELECT N'你好'");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, None) => {
                    assert!(matches!(expr, Expr::Literal(Literal::NationalString(s)) if s == "你好"));
                }
                _ => panic!("expected expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_dollar_string_literal() {
    let stmt = parse_one("SELECT $$hello world$$");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, None) => {
                    assert!(matches!(expr, Expr::Literal(Literal::DollarString { tag: None, body }) if body == "hello world"));
                }
                _ => panic!("expected expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_tagged_dollar_string_literal() {
    let stmt = parse_one("SELECT $tag$hello$tag$");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, None) => {
                    assert!(matches!(expr, Expr::Literal(Literal::DollarString { tag: Some(t), body }) if t == "tag" && body == "hello"));
                }
                _ => panic!("expected expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_plain_string_literal_unchanged() {
    let stmt = parse_one("SELECT 'hello'");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, None) => {
                    assert!(matches!(expr, Expr::Literal(Literal::String(s)) if s == "hello"));
                }
                _ => panic!("expected expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --lib test_bit_string test_hex_string test_escape_string test_national_string test_dollar_string test_plain_string`
Expected: All 6 new tests FAIL (Literal has no such variants)

**Step 3: Expand the `Literal` enum and fix `Token::DollarString`**

First, update `Token::DollarString` in `src/token/mod.rs` to preserve the tag:

```rust
/// Dollar-quoted string literal ($$...$$ or $tag$...$tag$)
DollarString { tag: Option<String>, body: String },  // was DollarString(String)
```

Then update the tokenizer in `src/token/tokenizer.rs` (lines 632-637):

```rust
let tag = self.advance_while(|c| c != '$' && c != '\0');
if self.peek() == Some('$') {
    self.advance(); // consume closing $ of delimiter
    let delimiter = format!("${}$", tag);
    let content = self.scan_dollar_string_content(&delimiter);
    let tag_opt = if tag.is_empty() { None } else { Some(tag) };
    Token::DollarString { tag: tag_opt, body: content }  // was Token::DollarString(content)
} else {
```

Then expand the `Literal` enum in `src/ast/mod.rs` (lines 569-576):

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum Literal {
    Integer(i64),
    Float(String),
    String(String),
    EscapeString(String),       // E'...'
    BitString(String),          // B'...'
    HexString(String),          // X'...'
    NationalString(String),     // N'...'
    DollarString {              // $$...$$ or $tag$...$tag$
        tag: Option<String>,
        body: String,
    },
    Boolean(bool),
    Null,
}
```

**Step 4: Update parser/expr.rs literal parsing**

In `src/parser/expr.rs`, change lines 257-280:

```rust
Token::StringLiteral(s) => {
    self.advance();
    Ok(Expr::Literal(Literal::String(s)))
}
Token::EscapeString(s) => {
    self.advance();
    Ok(Expr::Literal(Literal::EscapeString(s)))
}
Token::BitString(s) => {
    self.advance();
    Ok(Expr::Literal(Literal::BitString(s)))
}
Token::HexString(s) => {
    self.advance();
    Ok(Expr::Literal(Literal::HexString(s)))
}
Token::NationalString(s) => {
    self.advance();
    Ok(Expr::Literal(Literal::NationalString(s)))
}
Token::DollarString { tag, body } => {
    self.advance();
    Ok(Expr::Literal(Literal::DollarString { tag, body }))
}
```

Also update the IS NULL / LIKE / ILIKE string matching (lines ~179-213 and ~345-390) — there are additional `Literal::String` usages in `expr.rs` for pattern matching. Check and update:

```rust
// Line ~349 and ~384: these construct Literal::String from string-like tokens
// in postfix IS/NOT LIKE/ILIKE parsing. Update same as above.
```

**Step 5: Update `format_literal` in `src/formatter.rs`**

Replace the `format_literal` method (lines 517-531):

```rust
fn format_literal(&self, lit: &Literal) -> String {
    match lit {
        Literal::Integer(n) => n.to_string(),
        Literal::Float(s) => s.clone(),
        Literal::String(s) => self.quote_string(s),
        Literal::EscapeString(s) => {
            let escaped = s.replace('\\', "\\\\").replace("'", "''");
            format!("E'{}'", escaped)
        }
        Literal::BitString(s) => format!("B'{}'", s),
        Literal::HexString(s) => format!("X'{}'", s),
        Literal::NationalString(s) => format!("N'{}'", s),
        Literal::DollarString { tag, body } => match tag {
            Some(t) => format!("${}${}${}${}", t, body, t),
            None => format!("$${}$$", body),
        },,
        Literal::Boolean(b) => {
            if *b {
                "TRUE".to_string()
            } else {
                "FALSE".to_string()
            }
        }
        Literal::Null => "NULL".to_string(),
    }
}
```

**Step 6: Fix any remaining `Literal::String` match arms**

After expanding the enum, the Rust compiler will flag every non-exhaustive `match` on `Literal`. Fix all of them. Known locations:
- `src/parser/expr.rs` — any match on Literal
- `src/parser/ddl.rs` — default value parsing (if any)
- `src/parser/utility.rs` — constraint default value parsing
- `src/parser/tests.rs` — any test that matches `Literal::String`

For each, add the new variants. In most parser code that matches `Literal::String`, you should fall through to `Literal::String`:

```rust
// Pattern for parser match sites that need plain strings:
Literal::String(s) | Literal::EscapeString(s) | Literal::BitString(s) | Literal::HexString(s) | Literal::NationalString(s) => { /* existing logic */ }
Literal::DollarString { body, .. } => { /* existing logic with body */ }
```

**IMPORTANT:** `Token::DollarString` changed from `DollarString(String)` to `DollarString { tag, body }`. Every `match` on `Token` in the codebase will also need updating. Known sites:
- `src/parser/expr.rs` — literal parsing (handled in Step 4)
- `src/parser/mod.rs` — statement dispatch (check for `Token::DollarString(_)` patterns at lines ~204, ~530-534)
- `src/parser/ddl.rs` — DDL parsing (lines ~1281)
- `src/parser/utility.rs` — utility parsing (lines ~1001-1002, ~1262-1263, ~2629-2630, ~2687-2688)
- `src/parser/plpgsql.rs` — PL/pgSQL parsing (if it matches on DollarString)
- `src/token/tokenizer.rs` — tokenizer tests (lines ~926, ~932)

**Step 7: Run all tests**

Run: `cargo test --lib`
Expected: ALL 195 tests pass (188 original + 7 new)

**Step 8: Commit**

```bash
git add src/ast/mod.rs src/parser/expr.rs src/formatter.rs src/parser/tests.rs
# plus any other files fixed for non-exhaustive match
git commit -m "feat: expand Literal enum to preserve B'/E'/N'/X'/$$ string types"
```

---

### Task 2: Add `serde::Deserialize` to all AST and token types

**Files:**
- Modify: `src/ast/mod.rs` (155 derive attributes)
- Modify: `src/ast/plpgsql.rs` (33 derive attributes)
- Modify: `src/token/mod.rs` (4 derive attributes)
- Modify: `src/parser/mod.rs` (1 derive attribute)

**Step 1: Write JSON deserialize round-trip test**

Add to `src/parser/tests.rs`:

```rust
// ========== JSON Deserialize Round-Trip Tests ==========

#[test]
fn test_json_roundtrip_select() {
    let sql = "SELECT id, name FROM users WHERE status = 'active' ORDER BY id DESC LIMIT 10";
    let stmt = parse_one(sql);
    let json = serde_json::to_string(&stmt).unwrap();
    let de: Statement = serde_json::from_str(&json).unwrap();
    assert_eq!(stmt, de);
}

#[test]
fn test_json_roundtrip_insert() {
    let sql = "INSERT INTO users (id, name) VALUES (1, 'Alice'), (2, 'Bob') RETURNING id";
    let stmt = parse_one(sql);
    let json = serde_json::to_string(&stmt).unwrap();
    let de: Statement = serde_json::from_str(&json).unwrap();
    assert_eq!(stmt, de);
}

#[test]
fn test_json_roundtrip_update() {
    let sql = "UPDATE users SET name = 'Bob' WHERE id = 1 RETURNING *";
    let stmt = parse_one(sql);
    let json = serde_json::to_string(&stmt).unwrap();
    let de: Statement = serde_json::from_str(&json).unwrap();
    assert_eq!(stmt, de);
}

#[test]
fn test_json_roundtrip_delete() {
    let sql = "DELETE FROM users WHERE id = 1";
    let stmt = parse_one(sql);
    let json = serde_json::to_string(&stmt).unwrap();
    let de: Statement = serde_json::from_str(&json).unwrap();
    assert_eq!(stmt, de);
}

#[test]
fn test_json_roundtrip_create_table() {
    let sql = "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name VARCHAR(100) NOT NULL)";
    let stmt = parse_one(sql);
    let json = serde_json::to_string(&stmt).unwrap();
    let de: Statement = serde_json::from_str(&json).unwrap();
    assert_eq!(stmt, de);
}

#[test]
fn test_json_roundtrip_special_literals() {
    let sql = "SELECT B'1010', X'FF', E'\\t', N'hello', $$world$$";
    let stmt = parse_one(sql);
    let json = serde_json::to_string(&stmt).unwrap();
    let de: Statement = serde_json::from_str(&json).unwrap();
    assert_eq!(stmt, de);
}

#[test]
fn test_json_roundtrip_complex_expressions() {
    let sql = "SELECT CASE WHEN x > 0 THEN 1 WHEN x < 0 THEN -1 ELSE 0 END FROM t WHERE a BETWEEN 1 AND 10 AND b IN (1, 2, 3)";
    let stmt = parse_one(sql);
    let json = serde_json::to_string(&stmt).unwrap();
    let de: Statement = serde_json::from_str(&json).unwrap();
    assert_eq!(stmt, de);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --lib test_json_roundtrip`
Expected: FAIL — types don't impl Deserialize

**Step 3: Add Deserialize to `src/ast/mod.rs`**

Global replace in the file:
- `serde::Serialize` → `serde::{Serialize, Deserialize}`

This changes all 155 derive lines. For types that can't auto-derive Deserialize (e.g., if there are `Vec<(String, String)>` tuple fields — these work fine with serde), check compilation.

**IMPORTANT:** `ObjectName = Vec<String>` is a type alias — serde handles this natively.

**Step 4: Add Deserialize to `src/ast/plpgsql.rs`**

Change `use serde::Serialize;` to `use serde::{Serialize, Deserialize};`
Change all `Serialize` in derives to `Serialize, Deserialize`.

**Step 5: Add Deserialize to `src/token/mod.rs`**

Change `serde::Serialize` to `serde::{Serialize, Deserialize}` in all 4 derive lines.

**Step 6: Add Deserialize to `src/parser/mod.rs`**

Change line 11 from `serde::Serialize` to `serde::{Serialize, Deserialize}`.

**Step 7: Handle potential issues**

- `StatementInfo` uses `#[serde(flatten)]` on `statement: Statement` — this should work with externally-tagged enums, but verify.
- If `skip_serializing_if` annotations cause Deserialize issues, they won't (serde ignores unknown fields by default with `flatten`).
- `stub_struct!` macro types (`_stub: ()`) — derive Deserialize works fine for unit structs.

**Step 8: Run all tests**

Run: `cargo test --lib`
Expected: ALL tests pass (195+ including new round-trip tests)

**Step 9: Commit**

```bash
git add src/ast/mod.rs src/ast/plpgsql.rs src/token/mod.rs src/parser/mod.rs src/parser/tests.rs
git commit -m "feat: add serde::Deserialize to all AST and token types for JSON round-trip"
```

---

### Task 3: Add reverse SQL round-trip integration tests

**Files:**
- Modify: `src/parser/tests.rs`

**Step 1: Write SQL format round-trip tests**

```rust
// ========== SQL Format Round-Trip Tests ==========

use crate::formatter::SqlFormatter;

fn roundtrip_sql(sql: &str) -> String {
    let stmt = parse_one(sql);
    let json = serde_json::to_string(&stmt).unwrap();
    let de: Statement = serde_json::from_str(&json).unwrap();
    SqlFormatter::new().format_statement(&de)
}

#[test]
fn test_sql_roundtrip_select_basic() {
    assert_eq!(
        roundtrip_sql("SELECT id FROM users"),
        "SELECT id FROM users"
    );
}

#[test]
fn test_sql_roundtrip_bit_string() {
    assert_eq!(
        roundtrip_sql("SELECT B'10101'"),
        "SELECT B'10101'"
    );
}

#[test]
fn test_sql_roundtrip_hex_string() {
    assert_eq!(
        roundtrip_sql("SELECT X'FF'"),
        "SELECT X'FF'"
    );
}

#[test]
fn test_sql_roundtrip_escape_string() {
    assert_eq!(
        roundtrip_sql("SELECT E'\\n'"),
        "SELECT E'\\n'"
    );
}

#[test]
fn test_sql_roundtrip_insert_values() {
    let result = roundtrip_sql("INSERT INTO t (a, b) VALUES (1, 'x')");
    assert!(result.contains("INSERT INTO"));
    assert!(result.contains("VALUES"));
    assert!(result.contains("'x'"));
}

#[test]
fn test_sql_roundtrip_join() {
    let result = roundtrip_sql("SELECT a.id FROM users AS a INNER JOIN orders AS o ON a.id = o.user_id");
    assert!(result.contains("INNER JOIN"));
    assert!(result.contains("ON"));
}
```

**Step 2: Run tests**

Run: `cargo test --lib test_sql_roundtrip`
Expected: ALL pass

**Step 3: Commit**

```bash
git add src/parser/tests.rs
git commit -m "test: add SQL and JSON round-trip integration tests"
```

---

## P1: PL/pgSQL String → Expr (Enables PL/pgSQL round-trip)

### Task 4: Restructure PL/pgSQL assignment and control flow fields

This is the largest task. The PL/pgSQL parser (`src/parser/plpgsql.rs`) currently stores most expressions as raw `String` by collecting tokens until a delimiter. We need to parse them into `Expr` instead.

**IMPORTANT STRATEGY:** The existing `String` fields work correctly for forward parsing (SQL → AST → formatter). We must NOT break this. The approach is:
1. Change AST field types from `String` to `Expr`
2. Change parser to call `self.parse_expr()` instead of collecting tokens as strings
3. Change formatter to call `self.format_expr()` instead of emitting raw strings
4. For cases where the expression parser can't handle PL/pgSQL-specific syntax, keep a `raw: Option<String>` fallback

**Files:**
- Modify: `src/ast/plpgsql.rs` (field type changes)
- Modify: `src/parser/plpgsql.rs` (parsing changes)
- Modify: `src/formatter.rs` (PL/pgSQL formatting changes)

**Step 1: Write failing test for PL/pgSQL expression round-trip**

```rust
#[test]
fn test_plpgsql_if_condition_roundtrip() {
    let block = parse_do_block("DO $$ BEGIN IF x > 0 THEN NULL; END IF; END $$");
    match &block.body[0] {
        PlStatement::If(i) => {
            // After P1, condition should be a structured Expr, not a String
            match &i.condition {
                Expr::BinaryOp { op, .. } => assert_eq!(op, ">"),
                other => panic!("expected BinaryOp, got: {:?}", other),
            }
        }
        _ => panic!("expected If"),
    }
}

#[test]
fn test_plpgsql_assignment_roundtrip() {
    let block = parse_do_block("DO $$ BEGIN x := 42; END $$");
    match &block.body[0] {
        PlStatement::Assignment { target, expression } => {
            assert_eq!(target, "x");
            match expression {
                Expr::Literal(Literal::Integer(42)) => {},
                other => panic!("expected Integer(42), got: {:?}", other),
            }
        }
        _ => panic!("expected Assignment"),
    }
}

#[test]
fn test_plpgsql_while_condition_roundtrip() {
    let block = parse_do_block("DO $$ BEGIN WHILE x > 0 LOOP NULL; END LOOP; END $$");
    match &block.body[0] {
        PlStatement::While(w) => {
            match &w.condition {
                Expr::BinaryOp { op, .. } => assert_eq!(op, ">"),
                other => panic!("expected BinaryOp, got: {:?}", other),
            }
        }
        _ => panic!("expected While"),
    }
}
```

**Step 2: Change AST field types**

In `src/ast/plpgsql.rs`, change the following fields. **Do them in batches**, testing after each batch:

**Batch A — Simple conditions (IF/ELSIF/WHILE/CASE WHEN):**

```rust
// PlIfStmt.condition: String → Expr
pub struct PlIfStmt {
    pub condition: Expr,                    // was String
    pub then_stmts: Vec<PlStatement>,
    pub elsifs: Vec<PlElsif>,
    pub else_stmts: Vec<PlStatement>,
}

// PlElsif.condition: String → Expr
pub struct PlElsif {
    pub condition: Expr,                    // was String
    pub stmts: Vec<PlStatement>,
}

// PlWhileStmt.condition: String → Expr
pub struct PlWhileStmt {
    pub label: Option<String>,
    pub condition: Expr,                    // was String
    pub body: Vec<PlStatement>,
    pub end_label: Option<String>,
}

// PlCaseWhen.condition: String → Expr
pub struct PlCaseWhen {
    pub condition: Expr,                    // was String
    pub stmts: Vec<PlStatement>,
}

// PlCaseStmt.expression: Option<String> → Option<Expr>
pub struct PlCaseStmt {
    pub expression: Option<Expr>,           // was Option<String>
    pub whens: Vec<PlCaseWhen>,
    pub else_stmts: Vec<PlStatement>,
}
```

**Batch B — Assignment and expressions:**

```rust
// PlStatement::Assignment expression: String → Expr
// In the enum:
Assignment { target: String, expression: Expr },  // expression was String

// PlStatement::Return expression: Option<String> → Option<Expr>
Return { expression: Option<Expr> },               // was Option<String>

// PlStatement::Exit condition: Option<String> → Option<Expr>
Exit { label: Option<String>, condition: Option<Expr> },

// PlStatement::Continue condition: Option<String> → Option<Expr>
Continue { label: Option<String>, condition: Option<Expr> },

// PlForEachStmt.expression: String → Expr
pub struct PlForEachStmt {
    pub expression: Expr,                   // was String
    ...
}

// PlVarDecl.default: Option<String> → Option<Expr>
pub struct PlVarDecl {
    pub default: Option<Expr>,              // was Option<String>
    ...
}
```

**Batch C — FOR range bounds and EXECUTE:**

```rust
// PlForKind::Range low/high/step: String → Expr
Range {
    low: Expr,                              // was String
    high: Expr,                             // was String
    step: Option<Expr>,                     // was Option<String>
    reverse: bool,
},

// PlExecuteStmt fields:
pub struct PlExecuteStmt {
    pub string_expr: Expr,                  // was String
    pub into_target: Option<Expr>,          // was Option<String>
    pub using_args: Vec<Expr>,              // was Vec<String>
}

// PlProcedureCall.arguments: Vec<String> → Vec<Expr>
pub struct PlProcedureCall {
    pub arguments: Vec<Expr>,               // was Vec<String>
}

// PlOpenKind variants:
Simple { arguments: Vec<Expr> },            // was Vec<String>
ForUsing { expressions: Vec<Expr> },        // was Vec<String>

// PlFetchStmt.into: String → Expr
pub struct PlFetchStmt {
    pub into: Expr,                         // was String
    ...
}

// RaiseOption.value: String → Expr
pub struct RaiseOption {
    pub value: Expr,                        // was String
}

// PlRaiseStmt.message: Option<String> → Option<Expr>
pub struct PlRaiseStmt {
    pub message: Option<Expr>,              // was Option<String>
    ...
}

// PlStatement::PipeRow: String → Expr
PipeRow { expression: Expr },               // was String
```

**Step 3: Update PL/pgSQL parser**

In `src/parser/plpgsql.rs`, the current approach is:
```rust
// Current: collect tokens as raw string
let condition = self.collect_until_keyword("THEN");
```

Change to:
```rust
// New: parse as Expr
let condition = self.parse_expr()?;
```

The PL/pgSQL parser (`src/parser/plpgsql.rs`) has access to the expression parser via `self.parse_expr()`. The key change is replacing `collect_until_*` calls with `parse_expr()` calls.

**CRITICAL:** The `parse_expr()` method exists on `Parser` which is the same struct. PL/pgSQL parsing methods already have `&mut self`. The method is at `src/parser/expr.rs` as `pub fn parse_expr(&mut self) -> Result<Expr, ParserError>`.

For each batch, find the corresponding parse method and replace:
- `collect_until_ident_str("THEN")` → `self.parse_expr()`
- `collect_until_ident_str("LOOP")` → `self.parse_expr()`
- `collect_until_ident_str(";")` → `self.parse_expr()`
- etc.

**Potential issue:** Some PL/pgSQL expressions use syntax that the SQL expression parser doesn't handle (e.g., `%TYPE`, `:=`, qualified names in assignments). For these cases, keep a fallback:

```rust
// If parse_expr fails for PL/pgSQL-specific syntax, keep the raw string
let condition = match self.parse_expr() {
    Ok(expr) => expr,
    Err(_) => {
        // Fallback: collect as raw string and wrap in a passthrough Expr
        Expr::ColumnRef(vec![self.collect_until_keyword("THEN")])
    }
};
```

**Step 4: Update formatter**

In `src/formatter.rs`, the PL/pgSQL formatting methods currently emit raw strings:
```rust
// Current:
format!("{} {} {} ", self.kw("IF"), i.condition, self.kw("THEN"))
```

Change to:
```rust
// New:
format!("{} {} {} ", self.kw("IF"), self.format_expr(&i.condition), self.kw("THEN"))
```

Apply this pattern to ALL formatter methods that use PL/pgSQL String fields.

**Step 5: Run tests after EACH batch**

Run: `cargo test --lib`
Expected: All tests pass after each batch

**Step 6: Commit after each batch**

```bash
git commit -m "feat(plpgsql): restructure <batch-name> fields from String to Expr"
```

---

## P2: Type/Structural Improvements

### Task 5: Change `Expr::TypeCast.type_name` from String to DataType

**Files:**
- Modify: `src/ast/mod.rs:554-557` (TypeCast variant)
- Modify: `src/parser/expr.rs` (typecast parsing)
- Modify: `src/formatter.rs` (TypeCast formatting)

**Step 1: Write test**

```rust
#[test]
fn test_typecast_preserves_type_info() {
    let stmt = parse_one("SELECT x::NUMERIC(10,2)");
    match stmt {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(Expr::TypeCast { type_name, .. }, _) => {
                assert!(matches!(type_name, DataType::Numeric(Some(10), Some(2))));
            }
            _ => panic!("expected TypeCast"),
        },
        _ => panic!("expected Select"),
    }
}
```

**Step 2: Change AST**

```rust
TypeCast {
    expr: Box<Expr>,
    type_name: DataType,  // was String
},
```

**Step 3: Update parser**

In `src/parser/expr.rs`, the typecast parsing (around the `::` operator) currently stores the type name as a string. Change to call the existing `parse_data_type()` method.

**Step 4: Update formatter**

```rust
Expr::TypeCast { expr, type_name } => {
    format!("{}::{}", self.format_expr(expr), self.format_data_type(type_name))
}
```

**Step 5: Run tests, commit**

---

### Task 6: Convert WindowFrame enums

**Files:**
- Modify: `src/ast/mod.rs` (WindowFrame, WindowFrameBound)
- Modify: `src/parser/select.rs` or `src/parser/expr.rs` (window parsing)
- Modify: `src/formatter.rs` (window formatting)

**Step 1: Add enums**

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum WindowFrameMode {
    Rows,
    Range,
    Groups,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum WindowFrameDirection {
    UnboundedPreceding,
    UnboundedFollowing,
    CurrentRow,
    Preceding { offset: i64 },
    Following { offset: i64 },
}
```

**Step 2: Update WindowFrame and WindowFrameBound**

```rust
pub struct WindowFrame {
    pub mode: WindowFrameMode,           // was String
    pub start: Option<WindowFrameBound>,
    pub end: Option<WindowFrameBound>,
}

pub struct WindowFrameBound {
    pub direction: WindowFrameDirection,  // was String
    // offset moved into WindowFrameDirection variants
}
```

**Step 3: Update parser + formatter, run tests, commit**

---

### Task 7: Parse `PrepareStatement.statement` into `Box<Statement>`

**Files:**
- Modify: `src/ast/mod.rs` (PrepareStatement)
- Modify: `src/parser/mod.rs` (prepare parsing)

Change `statement: String` to `statement: Box<Statement>` and parse the inner SQL.

---

### Task 8: Parse `RuleStatement` fields into structured types

Change `event: String` to enum, `condition: Option<String>` to `Option<Expr>`, `actions: Vec<String>` to `Vec<Statement>`.

---

## P3: Remaining DDL Improvements

### Task 9: Structure `CreateFunctionStatement` and `CreateProcedureStatement`

Change `parameters: Vec<String>` → `Vec<FunctionParam>`, `return_type: Option<String>` → `Option<DataType>`, `options: String` → structured option list.

**NOTE:** This is the most complex P3 task because the `options` field currently contains LANGUAGE, IMMUTABLE/STABLE/VOLATILE, STRICT, SECURITY, COST, ROWS, and the function body. Parsing this requires a new option parser.

### Task 10: Structure `CreateDomainStatement` fields

Change `data_type: String` → `DataType`, `default_value: Option<String>` → `Option<Expr>`, `check: Option<String>` → `Option<Expr>`.

### Task 11: Structure `CreateCastStatement` fields

Change `source_type: String` → `DataType`, `target_type: String` → `DataType`.

### Task 12: Structure `CreateRlsPolicyStatement.using_expr`

Change `using_expr: Option<String>` → `Option<Expr>`.

### Task 13: Structure remaining minor fields

- `CommentStatement.object_type: String` → enum
- `LockStatement.mode: String` → enum
- `TypeAttribute.data_type: String` → `DataType`
- `AlterTypeAction::AddAttribute.data_type: String` → `DataType`
- `AlterExtensionStatement.action: String` → enum

---

## Testing Strategy

After EVERY task:

1. `cargo test --lib` — must show 0 failures
2. `cargo run --example regression` — must still pass 1409/1409 regression tests
3. Spot-check: `echo "SELECT B'1010'" | cargo run -- parse -j` — verify JSON output
4. Spot-check: `echo "SELECT B'1010'" | cargo run -- format` — verify SQL output

## Execution Order

```
Task 1 (Literal expansion) → Task 2 (Deserialize) → Task 3 (integration tests)
→ Task 4 (PL/pgSQL) → Task 5-8 (P2) → Task 9-13 (P3)
```

Tasks 1-3 are sequential (each depends on the previous).
Tasks 5-8 are independent after Task 4.
Tasks 9-13 are independent after Task 8.
