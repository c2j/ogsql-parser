# Complete CURSOR Support Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Achieve production-grade, complete CURSOR support across both SQL-level and PL/pgSQL-level statements, matching PostgreSQL/openGauss/GaussDB syntax coverage.

**Architecture:** Fix 6 categories of gaps: (1) refactor `DeclareCursorStatement` AST for tri-state fields, (2) fix SQL-level `FetchDirection` + add `IN` keyword, (3) structure SQL-level `MoveStatement`, (4) add `WHERE CURRENT OF` support, (5) fix PL/pgSQL `FetchDirection` to carry count values, (6) add GaussDB-specific cursor extensions. All changes follow the existing patterns: hand-written recursive descent parsing, serde derive on AST types, formatter round-trip fidelity.

**Tech Stack:** Rust 2021, serde (Serialize + Deserialize), thiserror

**Current State:** 514 tests passing. SQL-level DECLARE/FETCH/CLOSE work partially. MOVE is raw string. PL/pgSQL cursor ops exist but FetchDirection lacks count values.

---

## Task 1: Refactor `DeclareCursorStatement` AST — Tri-State Fields

**Why:** Current `bool scroll`/`bool hold` cannot distinguish "not specified" from "NO SCROLL"/"WITHOUT HOLD". `INSENSITIVE` is consumed but discarded. `ASENSITIVE` is not handled at all.

**Files:**
- Modify: `src/ast/mod.rs:2088-2095` (AST struct)
- Modify: `src/parser/utility/statements.rs:1095-1151` (parser)
- Modify: `src/formatter.rs:5281-5300` (formatter)
- Modify: `src/parser/tests.rs` (tests)

### Step 1: Add new enums and refactor AST struct

In `src/ast/mod.rs`, **before** `DeclareCursorStatement` (around line 2086), add:

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CursorSensitivity {
    Sensitive,    // default (not specified)
    Insensitive,  // INSENSITIVE
    Asensitive,   // ASENSITIVE
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CursorScrollability {
    Default,   // not specified
    Scroll,    // SCROLL
    NoScroll,  // NO SCROLL
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CursorHoldability {
    Default,       // not specified
    WithHold,      // WITH HOLD
    WithoutHold,   // WITHOUT HOLD
}
```

Then replace `DeclareCursorStatement`:

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeclareCursorStatement {
    pub name: String,
    pub binary: bool,
    pub sensitivity: CursorSensitivity,
    pub scrollability: CursorScrollability,
    pub holdability: CursorHoldability,
    pub query: Box<SelectStatement>,
}
```

### Step 2: Update parser

In `src/parser/utility/statements.rs`, replace `parse_declare_cursor` body:

```rust
pub(crate) fn parse_declare_cursor(&mut self) -> Result<DeclareCursorStatement, ParserError> {
    let name = self.parse_identifier()?;

    let mut binary = false;
    let mut sensitivity = CursorSensitivity::Sensitive;
    let mut scrollability = CursorScrollability::Default;
    let mut holdability = CursorHoldability::Default;

    loop {
        match self.peek_keyword() {
            Some(Keyword::BINARY) => {
                self.advance();
                binary = true;
            }
            Some(Keyword::INSENSITIVE) => {
                self.advance();
                sensitivity = CursorSensitivity::Insensitive;
            }
            Some(Keyword::ASENSITIVE) => {
                self.advance();
                sensitivity = CursorSensitivity::Asensitive;
            }
            Some(Keyword::SCROLL) => {
                self.advance();
                scrollability = CursorScrollability::Scroll;
            }
            Some(Keyword::NO) => {
                self.advance();
                self.expect_keyword(Keyword::SCROLL)?;
                scrollability = CursorScrollability::NoScroll;
            }
            Some(Keyword::WITH) => {
                self.advance();
                if self.match_keyword(Keyword::HOLD) {
                    self.advance();
                    holdability = CursorHoldability::WithHold;
                } else {
                    // might be WITH RETURN (GaussDB) — handled in Task 6
                    break;
                }
            }
            Some(Keyword::WITHOUT) => {
                self.advance();
                if self.match_keyword(Keyword::HOLD) {
                    self.advance();
                    holdability = CursorHoldability::WithoutHold;
                } else {
                    break;
                }
            }
            Some(Keyword::CURSOR) => {
                self.advance();
            }
            Some(Keyword::FOR) => {
                break;
            }
            _ => break,
        }
    }

    self.expect_keyword(Keyword::FOR)?;
    let query = Box::new(self.parse_select_statement()?);

    Ok(DeclareCursorStatement {
        name,
        binary,
        sensitivity,
        scrollability,
        holdability,
        query,
    })
}
```

### Step 3: Update formatter

In `src/formatter.rs`, replace `format_declare_cursor`:

```rust
fn format_declare_cursor(&self, stmt: &DeclareCursorStatement) -> String {
    let mut s = format!("{} {}", self.kw("DECLARE"), stmt.name);
    if stmt.binary {
        s.push(' ');
        s.push_str(&self.kw("BINARY"));
    }
    match &stmt.sensitivity {
        CursorSensitivity::Insensitive => {
            s.push(' ');
            s.push_str(&self.kw("INSENSITIVE"));
        }
        CursorSensitivity::Asensitive => {
            s.push(' ');
            s.push_str(&self.kw("ASENSITIVE"));
        }
        CursorSensitivity::Sensitive => {}
    }
    match &stmt.scrollability {
        CursorScrollability::Scroll => {
            s.push(' ');
            s.push_str(&self.kw("SCROLL"));
        }
        CursorScrollability::NoScroll => {
            s.push(' ');
            s.push_str(&self.kw("NO SCROLL"));
        }
        CursorScrollability::Default => {}
    }
    match &stmt.holdability {
        CursorHoldability::WithHold => {
            s.push_str(&format!(" {} {}", self.kw("WITH"), self.kw("HOLD")));
        }
        CursorHoldability::WithoutHold => {
            s.push_str(&format!(" {} {}", self.kw("WITHOUT"), self.kw("HOLD")));
        }
        CursorHoldability::Default => {}
    }
    s.push_str(&format!(
        " {} {}",
        self.kw("CURSOR FOR"),
        self.format_select(&stmt.query)
    ));
    s
}
```

Add necessary `use` imports for `CursorSensitivity`, `CursorScrollability`, `CursorHoldability` in formatter.

### Step 4: Add keyword ASENSITIVE

In `src/token/keyword.rs`, check if `ASENSITIVE` keyword exists. If not, add it to:
- The `Keyword` enum
- The `FromStr` lookup table
- The `Display` impl
- The reserved keyword group if needed

Search for `INSENSITIVE` in `keyword.rs` and follow the same pattern for `ASENSITIVE`.

### Step 5: Update existing tests

In `src/parser/tests.rs`, update existing cursor tests to match new AST:

- `test_declare_cursor_with_parsed_select` (~line 3358): Update assertions for new enum fields
- `test_declare_cursor_scroll_with_select` (~line 3382): Assert `scrollability == CursorScrollability::Scroll` instead of `scroll == true`

### Step 6: Add new tests

```rust
#[test]
fn test_declare_cursor_no_scroll() {
    let sql = "DECLARE cur NO SCROLL CURSOR FOR SELECT * FROM t";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::DeclareCursor(c) => {
            assert_eq!(c.scrollability, CursorScrollability::NoScroll);
        }
        _ => panic!("expected DeclareCursor"),
    }
}

#[test]
fn test_declare_cursor_insensitive() {
    let sql = "DECLARE cur INSENSITIVE SCROLL CURSOR WITH HOLD FOR SELECT * FROM t";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::DeclareCursor(c) => {
            assert_eq!(c.sensitivity, CursorSensitivity::Insensitive);
            assert_eq!(c.scrollability, CursorScrollability::Scroll);
            assert_eq!(c.holdability, CursorHoldability::WithHold);
        }
        _ => panic!("expected DeclareCursor"),
    }
}

#[test]
fn test_declare_cursor_without_hold() {
    let sql = "DECLARE cur CURSOR WITHOUT HOLD FOR SELECT 1";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::DeclareCursor(c) => {
            assert_eq!(c.holdability, CursorHoldability::WithoutHold);
            assert_eq!(c.scrollability, CursorScrollability::Default);
        }
        _ => panic!("expected DeclareCursor"),
    }
}
```

### Step 7: Run tests

```bash
cargo test 2>&1 | tail -20
```

Expected: All tests pass (514 + new).

### Step 8: Commit

```
feat(cursor): refactor DeclareCursorStatement AST with tri-state enums
```

---

## Task 2: Fix SQL-Level `FetchDirection` + Add `IN` Keyword + Bare FORWARD/BACKWARD

**Why:** SQL-level FETCH only accepts `FROM`, not `IN`. `FORWARD`/`BACKWARD` without count cause parse failure.

**Files:**
- Modify: `src/ast/mod.rs:2104-2117` (FetchDirection enum)
- Modify: `src/parser/utility/statements.rs:1153-1223` (parser)
- Modify: `src/formatter.rs:5306-5322` (formatter)
- Modify: `src/parser/tests.rs` (tests)

### Step 1: Update FetchDirection enum

In `src/ast/mod.rs`, replace the `FetchDirection` enum:

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FetchDirection {
    Next,
    Prior,
    First,
    Last,
    Absolute(i64),
    Relative(i64),
    Forward,           // bare FORWARD (= NEXT)
    ForwardCount(i64), // FORWARD n
    ForwardAll,
    Backward,           // bare BACKWARD (= PRIOR)
    BackwardCount(i64), // BACKWARD n
    BackwardAll,
    Count(i64),
    All,
}
```

### Step 2: Update FETCH parser

In `src/parser/utility/statements.rs`, replace `parse_fetch_cursor`:

```rust
pub(crate) fn parse_fetch_cursor(&mut self) -> Result<FetchStatement, ParserError> {
    let direction = match self.peek_keyword() {
        Some(Keyword::NEXT) => {
            self.advance();
            FetchDirection::Next
        }
        Some(Keyword::PRIOR) => {
            self.advance();
            FetchDirection::Prior
        }
        Some(Keyword::FIRST_P) => {
            self.advance();
            FetchDirection::First
        }
        Some(Keyword::LAST_P) => {
            self.advance();
            FetchDirection::Last
        }
        Some(Keyword::ABSOLUTE_P) => {
            self.advance();
            let n = self.parse_integer_literal()?;
            FetchDirection::Absolute(n)
        }
        Some(Keyword::RELATIVE_P) => {
            self.advance();
            let n = self.parse_integer_literal()?;
            FetchDirection::Relative(n)
        }
        Some(Keyword::FORWARD) => {
            self.advance();
            if self.match_keyword(Keyword::ALL) {
                self.advance();
                FetchDirection::ForwardAll
            } else if let Token::Integer(n) = self.peek().clone() {
                self.advance();
                FetchDirection::ForwardCount(n)
            } else {
                FetchDirection::Forward // bare FORWARD
            }
        }
        Some(Keyword::BACKWARD) => {
            self.advance();
            if self.match_keyword(Keyword::ALL) {
                self.advance();
                FetchDirection::BackwardAll
            } else if let Token::Integer(n) = self.peek().clone() {
                self.advance();
                FetchDirection::BackwardCount(n)
            } else {
                FetchDirection::Backward // bare BACKWARD
            }
        }
        Some(Keyword::ALL) => {
            self.advance();
            FetchDirection::All
        }
        _ => {
            if let Token::Integer(n) = self.peek().clone() {
                self.advance();
                FetchDirection::Count(n)
            } else {
                FetchDirection::Next
            }
        }
    };

    // Accept both FROM and IN
    if self.match_keyword(Keyword::FROM) || self.match_keyword(Keyword::IN) {
        self.advance();
    }

    let cursor_name = self.parse_identifier()?;

    Ok(FetchStatement {
        direction,
        cursor_name,
    })
}
```

**Note:** Check if `Keyword::IN` exists (it should — it's a common SQL keyword). If not, use `self.match_ident_str("in")`.

### Step 3: Update formatter

Replace `format_fetch` in `src/formatter.rs`:

```rust
fn format_fetch(&self, stmt: &FetchStatement) -> String {
    let dir = match &stmt.direction {
        FetchDirection::Next => self.kw("NEXT").to_string(),
        FetchDirection::Prior => self.kw("PRIOR").to_string(),
        FetchDirection::First => self.kw("FIRST").to_string(),
        FetchDirection::Last => self.kw("LAST").to_string(),
        FetchDirection::Absolute(n) => format!("{} {}", self.kw("ABSOLUTE"), n),
        FetchDirection::Relative(n) => format!("{} {}", self.kw("RELATIVE"), n),
        FetchDirection::Forward => self.kw("FORWARD").to_string(),
        FetchDirection::ForwardCount(n) => format!("{} {}", self.kw("FORWARD"), n),
        FetchDirection::ForwardAll => format!("{} {}", self.kw("FORWARD"), self.kw("ALL")),
        FetchDirection::Backward => self.kw("BACKWARD").to_string(),
        FetchDirection::BackwardCount(n) => format!("{} {}", self.kw("BACKWARD"), n),
        FetchDirection::BackwardAll => format!("{} {}", self.kw("BACKWARD"), self.kw("ALL")),
        FetchDirection::Count(n) => n.to_string(),
        FetchDirection::All => self.kw("ALL").to_string(),
    };
    format!("{} {} {}", dir, self.kw("FROM"), stmt.cursor_name)
}
```

### Step 4: Update PL/pgSQL formatter that uses SQL-level FetchDirection

Search `src/formatter.rs` for any other references to `FetchDirection::Forward(` or `FetchDirection::Backward(` and update to new variant names `ForwardCount`/`BackwardCount`.

### Step 5: Add tests

```rust
#[test]
fn test_fetch_in_keyword() {
    let sql = "FETCH NEXT IN cur1";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::Fetch(f) => {
            assert_eq!(f.cursor_name, "cur1");
            assert_eq!(f.direction, FetchDirection::Next);
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_fetch_bare_forward() {
    let sql = "FETCH FORWARD FROM cur1";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::Fetch(f) => {
            assert_eq!(f.direction, FetchDirection::Forward);
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_fetch_bare_backward() {
    let sql = "FETCH BACKWARD IN cur1";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::Fetch(f) => {
            assert_eq!(f.direction, FetchDirection::Backward);
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_fetch_forward_count() {
    let sql = "FETCH FORWARD 5 FROM cur1";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::Fetch(f) => {
            assert_eq!(f.direction, FetchDirection::ForwardCount(5));
        }
        _ => panic!("expected Fetch"),
    }
}
```

### Step 6: Run tests

```bash
cargo test 2>&1 | tail -20
```

### Step 7: Commit

```
feat(cursor): support IN keyword and bare FORWARD/BACKWARD in FETCH
```

---

## Task 3: Structure SQL-Level `MoveStatement`

**Why:** Current `MoveStatement { raw_rest: String }` is unparsed. MOVE shares identical syntax with FETCH (same direction options).

**Files:**
- Modify: `src/ast/mod.rs:2849-2851` (AST struct)
- Modify: `src/parser/mod.rs:1633-1636` (parser dispatch)
- Modify: `src/formatter.rs:191-193` (formatter)

### Step 1: Replace MoveStatement AST

In `src/ast/mod.rs`, replace:

```rust
// OLD:
pub struct MoveStatement {
    pub raw_rest: String,
}

// NEW:
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MoveStatement {
    pub direction: FetchDirection,
    pub cursor_name: String,
}
```

### Step 2: Update parser dispatch

In `src/parser/mod.rs`, replace the MOVE dispatch (around line 1633):

```rust
// OLD:
Token::Ident(ref s) if s.eq_ignore_ascii_case("move") => {
    self.advance();
    let raw = self.skip_to_semicolon_and_collect();
    crate::ast::Statement::Move(crate::ast::MoveStatement { raw_rest: raw })
}

// NEW:
Token::Ident(ref s) if s.eq_ignore_ascii_case("move") => {
    self.advance();
    match self.parse_move_cursor() {
        Ok(stmt) => {
            self.try_consume_semicolon();
            crate::ast::Statement::Move(stmt)
        }
        Err(e) => {
            self.add_error(e);
            self.skip_to_semicolon()
        }
    }
}
```

### Step 3: Add `parse_move_cursor` parser method

In `src/parser/utility/statements.rs`, add after `parse_fetch_cursor`:

```rust
pub(crate) fn parse_move_cursor(&mut self) -> Result<MoveStatement, ParserError> {
    let direction = match self.peek_keyword() {
        Some(Keyword::NEXT) => {
            self.advance();
            FetchDirection::Next
        }
        Some(Keyword::PRIOR) => {
            self.advance();
            FetchDirection::Prior
        }
        Some(Keyword::FIRST_P) => {
            self.advance();
            FetchDirection::First
        }
        Some(Keyword::LAST_P) => {
            self.advance();
            FetchDirection::Last
        }
        Some(Keyword::ABSOLUTE_P) => {
            self.advance();
            let n = self.parse_integer_literal()?;
            FetchDirection::Absolute(n)
        }
        Some(Keyword::RELATIVE_P) => {
            self.advance();
            let n = self.parse_integer_literal()?;
            FetchDirection::Relative(n)
        }
        Some(Keyword::FORWARD) => {
            self.advance();
            if self.match_keyword(Keyword::ALL) {
                self.advance();
                FetchDirection::ForwardAll
            } else if let Token::Integer(n) = self.peek().clone() {
                self.advance();
                FetchDirection::ForwardCount(n)
            } else {
                FetchDirection::Forward
            }
        }
        Some(Keyword::BACKWARD) => {
            self.advance();
            if self.match_keyword(Keyword::ALL) {
                self.advance();
                FetchDirection::BackwardAll
            } else if let Token::Integer(n) = self.peek().clone() {
                self.advance();
                FetchDirection::BackwardCount(n)
            } else {
                FetchDirection::Backward
            }
        }
        Some(Keyword::ALL) => {
            self.advance();
            FetchDirection::All
        }
        _ => {
            if let Token::Integer(n) = self.peek().clone() {
                self.advance();
                FetchDirection::Count(n)
            } else {
                FetchDirection::Next
            }
        }
    };

    // Accept both FROM and IN
    if self.match_keyword(Keyword::FROM) || self.match_keyword(Keyword::IN) {
        self.advance();
    }

    let cursor_name = self.parse_identifier()?;

    Ok(MoveStatement {
        direction,
        cursor_name,
    })
}
```

**IMPORTANT:** Consider extracting the direction-parsing logic into a shared `parse_fetch_direction()` helper method to avoid duplication between `parse_fetch_cursor` and `parse_move_cursor`.

### Step 4: Update formatter

Replace in `src/formatter.rs`:

```rust
// OLD:
Statement::Move(s) => {
    format!("{} {}", self.kw("MOVE"), s.raw_rest)
}

// NEW:
Statement::Move(s) => self.format_move(s),
```

Add new method:

```rust
fn format_move(&self, stmt: &MoveStatement) -> String {
    let dir = match &stmt.direction {
        FetchDirection::Next => self.kw("NEXT").to_string(),
        FetchDirection::Prior => self.kw("PRIOR").to_string(),
        FetchDirection::First => self.kw("FIRST").to_string(),
        FetchDirection::Last => self.kw("LAST").to_string(),
        FetchDirection::Absolute(n) => format!("{} {}", self.kw("ABSOLUTE"), n),
        FetchDirection::Relative(n) => format!("{} {}", self.kw("RELATIVE"), n),
        FetchDirection::Forward => self.kw("FORWARD").to_string(),
        FetchDirection::ForwardCount(n) => format!("{} {}", self.kw("FORWARD"), n),
        FetchDirection::ForwardAll => format!("{} {}", self.kw("FORWARD"), self.kw("ALL")),
        FetchDirection::Backward => self.kw("BACKWARD").to_string(),
        FetchDirection::BackwardCount(n) => format!("{} {}", self.kw("BACKWARD"), n),
        FetchDirection::BackwardAll => format!("{} {}", self.kw("BACKWARD"), self.kw("ALL")),
        FetchDirection::Count(n) => n.to_string(),
        FetchDirection::All => self.kw("ALL").to_string(),
    };
    format!("{} {} {} {}", self.kw("MOVE"), dir, self.kw("FROM"), stmt.cursor_name)
}
```

### Step 5: Add tests

```rust
#[test]
fn test_move_next() {
    let sql = "MOVE NEXT FROM cur1";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::Move(m) => {
            assert_eq!(m.cursor_name, "cur1");
            assert_eq!(m.direction, FetchDirection::Next);
        }
        _ => panic!("expected Move"),
    }
}

#[test]
fn test_move_forward_5() {
    let sql = "MOVE FORWARD 5 IN cur1";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::Move(m) => {
            assert_eq!(m.direction, FetchDirection::ForwardCount(5));
            assert_eq!(m.cursor_name, "cur1");
        }
        _ => panic!("expected Move"),
    }
}

#[test]
fn test_move_all() {
    let sql = "MOVE ALL FROM cur1";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::Move(m) => {
            assert_eq!(m.direction, FetchDirection::All);
        }
        _ => panic!("expected Move"),
    }
}

#[test]
fn test_move_absolute() {
    let sql = "MOVE ABSOLUTE -3 FROM cur1";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::Move(m) => {
            assert_eq!(m.direction, FetchDirection::Absolute(-3));
        }
        _ => panic!("expected Move"),
    }
}
```

### Step 6: Run tests

```bash
cargo test 2>&1 | tail -20
```

### Step 7: Commit

```
feat(cursor): structure SQL-level MOVE statement with full direction support
```

---

## Task 4: Add `CLOSE ALL` Support

**Why:** PostgreSQL supports `CLOSE ALL` to close all open cursors.

**Files:**
- Modify: `src/ast/mod.rs:2119-2122` (AST)
- Modify: `src/parser/utility/statements.rs:1225-1228` (parser)
- Modify: `src/formatter.rs:5302-5304` (formatter)

### Step 1: Refactor ClosePortalStatement

```rust
// OLD:
pub struct ClosePortalStatement {
    pub name: String,
}

// NEW:
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CloseTarget {
    Name(String),
    All,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClosePortalStatement {
    pub target: CloseTarget,
}
```

### Step 2: Update parser

```rust
pub(crate) fn parse_close_portal(&mut self) -> Result<ClosePortalStatement, ParserError> {
    if self.match_keyword(Keyword::ALL) {
        self.advance();
        Ok(ClosePortalStatement {
            target: CloseTarget::All,
        })
    } else {
        let name = self.parse_identifier()?;
        Ok(ClosePortalStatement {
            target: CloseTarget::Name(name),
        })
    }
}
```

### Step 3: Update formatter

```rust
fn format_close_portal(&self, stmt: &ClosePortalStatement) -> String {
    match &stmt.target {
        CloseTarget::Name(name) => format!("{} {}", self.kw("CLOSE"), name),
        CloseTarget::All => format!("{} {}", self.kw("CLOSE"), self.kw("ALL")),
    }
}
```

### Step 4: Update all references to `ClosePortalStatement.name`

Search for `.name` on `ClosePortalStatement` across the codebase and update to match the new `.target` field. Key locations:
- `src/parser/tests.rs` — any test asserting on close portal name
- `src/formatter.rs` — already covered above

### Step 5: Add tests

```rust
#[test]
fn test_close_all() {
    let sql = "CLOSE ALL";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::ClosePortal(c) => {
            assert_eq!(c.target, CloseTarget::All);
        }
        _ => panic!("expected ClosePortal"),
    }
}

#[test]
fn test_close_named() {
    let sql = "CLOSE cur1";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::ClosePortal(c) => {
            assert_eq!(c.target, CloseTarget::Name("cur1".to_string()));
        }
        _ => panic!("expected ClosePortal"),
    }
}
```

### Step 6: Run tests + Commit

```bash
cargo test 2>&1 | tail -20
```

```
feat(cursor): support CLOSE ALL statement
```

---

## Task 5: Add `WHERE CURRENT OF cursor` Support

**Why:** `UPDATE/DELETE ... WHERE CURRENT OF cursor_name` is essential for positioned updates/deletes in cursor-based processing. This is standard SQL and fully supported by PostgreSQL/openGauss/GaussDB.

**Files:**
- Modify: `src/ast/mod.rs:1023-1123` (Expr enum — add CurrentOf variant)
- Modify: `src/parser/dml.rs:102,266,306` (UPDATE/DELETE WHERE parsing)
- Modify: `src/parser/expr.rs` (if WHERE uses expression parser)
- Modify: `src/formatter.rs` (format CurrentOf)
- Modify: `src/parser/tests.rs` (tests)

### Step 1: Add `CurrentOf` variant to `Expr` enum

In `src/ast/mod.rs`, add to the `Expr` enum:

```rust
/// WHERE CURRENT OF cursor_name — for positioned UPDATE/DELETE
CurrentOf { cursor_name: String },
```

### Step 2: Update WHERE clause parsing in UPDATE/DELETE

In `src/parser/dml.rs`, the WHERE clause is parsed as `self.parse_expr()`. The approach is:

**Option A (Recommended):** Add `CURRENT OF` handling in the expression parser (`src/parser/expr.rs`), since `WHERE` is parsed as a generic expression. When the parser encounters `CURRENT` keyword followed by `OF`, it should produce `Expr::CurrentOf`.

**Option B:** Special-case in `parse_update`/`parse_delete` before calling `parse_expr()`.

Go with Option A. In `src/parser/expr.rs`, in the primary expression parser (where atoms/literals are parsed), add:

```rust
// After checking for other primary expressions:
if self.match_keyword(Keyword::CURRENT_OF) || 
   (self.match_keyword(Keyword::CURRENT) && /* peek next is OF */) {
    self.advance(); // CURRENT_OF or CURRENT
    if !self.match_keyword(Keyword::CURRENT_OF) {
        self.expect_keyword(Keyword::OF)?; // OF
    }
    let cursor_name = self.parse_identifier()?;
    return Ok(Expr::CurrentOf { cursor_name });
}
```

**IMPORTANT:** Check how `CURRENT_OF` keyword is handled in the keyword list. In PostgreSQL grammar, `CURRENT OF` is two separate keywords. Check `src/token/keyword.rs` for `CURRENT_OF` vs separate `CURRENT` + `OF`. Adjust the parsing accordingly.

### Step 3: Update formatter

In `src/formatter.rs`, in the expression formatter, add:

```rust
Expr::CurrentOf { cursor_name } => {
    format!("{} {} {}", self.kw("CURRENT"), self.kw("OF"), cursor_name)
}
```

### Step 4: Update serde round-trip

Verify that `CurrentOf` variant serializes/deserializes correctly with serde. The derive macros handle this automatically, but verify with a test.

### Step 5: Add tests

```rust
#[test]
fn test_update_where_current_of() {
    let sql = "UPDATE accounts SET balance = balance + 100 WHERE CURRENT OF cur_account";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::Update(u) => {
            match &u.where_clause {
                Some(Expr::CurrentOf { cursor_name }) => {
                    assert_eq!(cursor_name, "cur_account");
                }
                other => panic!("expected CurrentOf, got {:?}", other),
            }
        }
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_delete_where_current_of() {
    let sql = "DELETE FROM accounts WHERE CURRENT OF cur_account";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::Delete(d) => {
            match &d.where_clause {
                Some(Expr::CurrentOf { cursor_name }) => {
                    assert_eq!(cursor_name, "cur_account");
                }
                other => panic!("expected CurrentOf, got {:?}", other),
            }
        }
        _ => panic!("expected Delete"),
    }
}
```

### Step 6: Run tests + Commit

```bash
cargo test 2>&1 | tail -20
```

```
feat(cursor): support WHERE CURRENT OF for positioned UPDATE/DELETE
```

---

## Task 6: Fix PL/pgSQL `FetchDirection` to Carry Count Values

**Why:** The PL/pgSQL `FetchDirection` enum (`src/ast/plpgsql.rs:407-417`) has unit variants without count values. `FETCH FORWARD 5 FROM cur INTO var` loses the count `5`. Same for ABSOLUTE, RELATIVE, BACKWARD.

**Files:**
- Modify: `src/ast/plpgsql.rs:405-433` (enum + Display impl)
- Modify: `src/parser/plpgsql.rs:1353-1439` (parser: parse_pl_fetch, parse_pl_move, parse_fetch_direction_from_token)
- Modify: `src/formatter.rs` (PL/pgSQL fetch/move formatter)
- Modify: `src/parser/tests.rs` (tests)

### Step 1: Replace PL/pgSQL FetchDirection enum

In `src/ast/plpgsql.rs`, replace:

```rust
// OLD:
pub enum FetchDirection {
    Next,
    Prior,
    First,
    Last,
    Forward,
    Backward,
    Absolute,
    Relative,
    All,
}

// NEW:
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FetchDirection {
    Next,
    Prior,
    First,
    Last,
    Absolute(i64),
    Relative(i64),
    Forward(Option<i64>),   // None = bare FORWARD
    Backward(Option<i64>),  // None = bare BACKWARD
    ForwardAll,
    BackwardAll,
    All,
}
```

### Step 2: Update Display impl

```rust
impl fmt::Display for FetchDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchDirection::Next => write!(f, "NEXT"),
            FetchDirection::Prior => write!(f, "PRIOR"),
            FetchDirection::First => write!(f, "FIRST"),
            FetchDirection::Last => write!(f, "LAST"),
            FetchDirection::Absolute(n) => write!(f, "ABSOLUTE {}", n),
            FetchDirection::Relative(n) => write!(f, "RELATIVE {}", n),
            FetchDirection::Forward(None) => write!(f, "FORWARD"),
            FetchDirection::Forward(Some(n)) => write!(f, "FORWARD {}", n),
            FetchDirection::Backward(None) => write!(f, "BACKWARD"),
            FetchDirection::Backward(Some(n)) => write!(f, "BACKWARD {}", n),
            FetchDirection::ForwardAll => write!(f, "FORWARD ALL"),
            FetchDirection::BackwardAll => write!(f, "BACKWARD ALL"),
            FetchDirection::All => write!(f, "ALL"),
        }
    }
}
```

### Step 3: Rewrite `parse_fetch_direction_from_token` in parser

In `src/parser/plpgsql.rs`, the direction parsing is split across `parse_pl_fetch` and `parse_fetch_direction_from_token`. The current `parse_fetch_direction_from_token` only looks at the current token string and returns a unit variant.

Replace the direction parsing in `parse_pl_fetch` and `parse_pl_move` with a new shared method:

```rust
/// Parse fetch/move direction from the current position.
/// Returns (direction, should_advance_current_token).
/// IMPORTANT: This method consumes tokens as needed for count values,
/// but the caller must advance past the initial direction keyword.
fn parse_pl_fetch_direction(&mut self) -> Result<FetchDirection, ParserError> {
    let dir_str = self.token_to_string();
    match dir_str.to_uppercase().as_str() {
        "NEXT" => Ok(FetchDirection::Next),
        "PRIOR" => Ok(FetchDirection::Prior),
        "FIRST" => Ok(FetchDirection::First),
        "LAST" => Ok(FetchDirection::Last),
        "ABSOLUTE" => {
            // ABSOLUTE requires a count
            let n = self.parse_integer_literal_after_advance()?;
            Ok(FetchDirection::Absolute(n))
        }
        "RELATIVE" => {
            let n = self.parse_integer_literal_after_advance()?;
            Ok(FetchDirection::Relative(n))
        }
        "FORWARD" => {
            // FORWARD [n | ALL] — bare FORWARD if next is not integer or ALL
            // Note: we advance past FORWARD, then check next token
            let n = self.try_parse_integer_literal();
            if let Some(n) = n {
                Ok(FetchDirection::Forward(Some(n)))
            } else if self.match_ident_str("all") {
                Ok(FetchDirection::ForwardAll)
            } else {
                Ok(FetchDirection::Forward(None))
            }
        }
        "BACKWARD" => {
            let n = self.try_parse_integer_literal();
            if let Some(n) = n {
                Ok(FetchDirection::Backward(Some(n)))
            } else if self.match_ident_str("all") {
                Ok(FetchDirection::BackwardAll)
            } else {
                Ok(FetchDirection::Backward(None))
            }
        }
        "ALL" => Ok(FetchDirection::All),
        _ => unreachable!("invalid fetch direction: {}", dir_str),
    }
}
```

**IMPORTANT:** The exact implementation depends on how `parse_integer_literal` works in the PL/pgSQL parser context. Check `self.parse_integer_literal()` and adapt accordingly. The key point is that after consuming the direction keyword, the parser needs to check if the next token is an integer and consume it.

Then update `parse_pl_fetch`:

```rust
fn parse_pl_fetch(&mut self) -> Result<PlStatement, ParserError> {
    self.advance(); // past FETCH

    let direction = if self.match_ident_str("next")
        || self.match_ident_str("prior")
        || self.match_ident_str("first")
        || self.match_ident_str("last")
        || self.match_ident_str("forward")
        || self.match_ident_str("backward")
        || self.match_ident_str("absolute")
        || self.match_ident_str("relative")
        || self.match_ident_str("all")
    {
        let dir = self.parse_pl_fetch_direction()?;
        self.advance(); // past the direction keyword
        if self.match_ident_str("from") || self.match_ident_str("in") {
            self.advance();
        }
        Some(dir)
    } else {
        None
    };

    // ... rest unchanged
}
```

**Note:** This is the trickiest task because the direction parsing needs to consume the count integer tokens as well. Pay close attention to how `self.advance()` interacts with the count-parsing. Test carefully.

Update `parse_pl_move` similarly.

### Step 4: Update formatter

Update PL/pgSQL formatter for `PlStatement::Fetch` and `PlStatement::Move` in `src/formatter.rs` (around lines 4342-4362) to handle the new enum variants with counts.

### Step 5: Update existing tests

In `src/parser/tests.rs`, update:
- `test_plpgsql_fetch_with_direction` (~line 3450)
- `test_plpgsql_move_with_direction` (~line 3467)

Update expected `FetchDirection` variants to match new parameterized forms.

### Step 6: Add new tests

```rust
#[test]
fn test_plpgsql_fetch_forward_count() {
    let sql = "DO $$ BEGIN FETCH FORWARD 5 FROM cur INTO var; END $$";
    // Assert direction == FetchDirection::Forward(Some(5))
}

#[test]
fn test_plpgsql_fetch_absolute() {
    let sql = "DO $$ BEGIN FETCH ABSOLUTE 10 cur INTO var; END $$";
    // Assert direction == FetchDirection::Absolute(10)
}

#[test]
fn test_plpgsql_move_relative() {
    let sql = "DO $$ BEGIN MOVE RELATIVE -3 FROM cur; END $$";
    // Assert direction == FetchDirection::Relative(-3)
}
```

### Step 7: Run tests + Commit

```bash
cargo test 2>&1 | tail -20
```

```
fix(cursor): PL/pgSQL FetchDirection now carries count values
```

---

## Task 7: Add GaussDB-Specific Cursor Extensions

**Why:** GaussDB/openGauss supports `WITH RETURN`/`WITHOUT RETURN`/`TO CALLER`/`TO CLIENT` on DECLARE CURSOR for returning result sets from stored procedures.

**Files:**
- Modify: `src/ast/mod.rs` (add new fields to DeclareCursorStatement)
- Modify: `src/parser/utility/statements.rs` (parser)
- Modify: `src/formatter.rs` (formatter)
- Modify: `src/parser/tests.rs` (tests)

### Step 1: Add new enums and fields

In `src/ast/mod.rs`, add after the holdability enum:

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CursorReturnability {
    Default,         // not specified
    WithReturn,      // WITH RETURN
    WithoutReturn,   // WITHOUT RETURN
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CursorReturnTo {
    Default,     // not specified
    ToCaller,    // TO CALLER
    ToClient,    // TO CLIENT
}
```

Add fields to `DeclareCursorStatement`:

```rust
pub returnability: CursorReturnability,
pub return_to: CursorReturnTo,
```

### Step 2: Update parser

In `parse_declare_cursor`, add handling in the option loop:

```rust
// Inside the loop, after the WITHOUT HOLD case:
Some(Keyword::RETURN) => {
    // WITH RETURN or WITHOUT RETURN (partially handled above)
    // This case handles standalone RETURN if needed
    break;
}
_ => break,
```

For `WITH RETURN` and `WITHOUT RETURN`:

```rust
Some(Keyword::WITH) => {
    self.advance();
    if self.match_keyword(Keyword::HOLD) {
        self.advance();
        holdability = CursorHoldability::WithHold;
    } else if self.match_keyword(Keyword::RETURN) {
        self.advance();
        returnability = CursorReturnability::WithReturn;
        if self.match_keyword(Keyword::TO) {
            self.advance();
            return_to = self.parse_cursor_return_to()?;
        }
    } else {
        break;
    }
}
Some(Keyword::WITHOUT) => {
    self.advance();
    if self.match_keyword(Keyword::HOLD) {
        self.advance();
        holdability = CursorHoldability::WithoutHold;
    } else if self.match_keyword(Keyword::RETURN) {
        self.advance();
        returnability = CursorReturnability::WithoutReturn;
        if self.match_keyword(Keyword::TO) {
            self.advance();
            return_to = self.parse_cursor_return_to()?;
        }
    } else {
        break;
    }
}
```

Add helper:

```rust
fn parse_cursor_return_to(&mut self) -> Result<CursorReturnTo, ParserError> {
    if self.match_keyword(Keyword::CALLER) || self.match_ident_str("caller") {
        self.advance();
        Ok(CursorReturnTo::ToCaller)
    } else if self.match_keyword(Keyword::CLIENT) || self.match_ident_str("client") {
        self.advance();
        Ok(CursorReturnTo::ToClient)
    } else {
        Ok(CursorReturnTo::Default)
    }
}
```

### Step 3: Update formatter

```rust
match &stmt.returnability {
    CursorReturnability::WithReturn => {
        s.push_str(&format!(" {} {}", self.kw("WITH"), self.kw("RETURN")));
    }
    CursorReturnability::WithoutReturn => {
        s.push_str(&format!(" {} {}", self.kw("WITHOUT"), self.kw("RETURN")));
    }
    CursorReturnability::Default => {}
}
match &stmt.return_to {
    CursorReturnTo::ToCaller => {
        s.push_str(&format!(" {} {}", self.kw("TO"), self.kw("CALLER")));
    }
    CursorReturnTo::ToClient => {
        s.push_str(&format!(" {} {}", self.kw("TO"), self.kw("CLIENT")));
    }
    CursorReturnTo::Default => {}
}
```

### Step 4: Add tests

```rust
#[test]
fn test_declare_cursor_with_return_to_caller() {
    let sql = "DECLARE cur CURSOR WITH RETURN TO CALLER FOR SELECT * FROM t";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::DeclareCursor(c) => {
            assert_eq!(c.returnability, CursorReturnability::WithReturn);
            assert_eq!(c.return_to, CursorReturnTo::ToCaller);
        }
        _ => panic!("expected DeclareCursor"),
    }
}

#[test]
fn test_declare_cursor_without_return_to_client() {
    let sql = "DECLARE cur SCROLL CURSOR WITHOUT RETURN TO CLIENT FOR SELECT 1";
    let stmts = parse_sql(sql);
    match &stmts[0] {
        Statement::DeclareCursor(c) => {
            assert_eq!(c.scrollability, CursorScrollability::Scroll);
            assert_eq!(c.returnability, CursorReturnability::WithoutReturn);
            assert_eq!(c.return_to, CursorReturnTo::ToClient);
        }
        _ => panic!("expected DeclareCursor"),
    }
}
```

### Step 5: Run tests + Commit

```bash
cargo test 2>&1 | tail -20
```

```
feat(cursor): add GaussDB WITH/WITHOUT RETURN and TO CALLER/CLIENT extensions
```

---

## Task 8: Fix PL/pgSQL `OPEN FOR EXECUTE` and Add `SCROLL` Option

**Why:** `OPEN cursor FOR EXECUTE query_string [USING expr, ...]` is conflated with `OPEN cursor FOR USING expr, ...`. The `SCROLL`/`NO SCROLL` option on OPEN is missing.

**Files:**
- Modify: `src/ast/plpgsql.rs:391-403` (PlOpenKind enum)
- Modify: `src/parser/plpgsql.rs:1290-1351` (parser)
- Modify: `src/formatter.rs:4321-4340` (formatter)
- Modify: `src/parser/tests.rs` (tests)

### Step 1: Add `ForExecute` variant to `PlOpenKind`

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlOpenKind {
    /// OPEN cursor [([args])]
    Simple { arguments: Vec<crate::ast::Expr> },
    /// OPEN cursor [NO] SCROLL FOR query
    ForQuery {
        scroll: Option<bool>,  // None = default, Some(true) = SCROLL, Some(false) = NO SCROLL
        query: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        parsed_query: Option<Box<crate::ast::Statement>>,
    },
    /// OPEN cursor FOR EXECUTE query_string [USING expr, ...]
    ForExecute {
        query: crate::ast::Expr,
        using_args: Vec<crate::ast::Expr>,
    },
    /// OPEN cursor FOR USING expr, ... (deprecated alias)
    ForUsing { expressions: Vec<crate::ast::Expr> },
}
```

### Step 2: Update parser

In `parse_pl_open`, update the `FOR` branch:

```rust
} else if self.match_ident_str("for") {
    self.advance();
    if self.match_ident_str("execute") {
        self.advance();
        let query = self.parse_expr()?;
        let mut using_args = Vec::new();
        if self.match_ident_str("using") {
            self.advance();
            loop {
                using_args.push(self.parse_expr()?);
                if self.match_token(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        PlOpenKind::ForExecute { query, using_args }
    } else if self.match_ident_str("using") {
        // ... existing ForUsing logic
    } else {
        // Check for SCROLL/NO SCROLL before query
        let scroll = if self.match_ident_str("scroll") {
            self.advance();
            Some(true)
        } else if self.match_ident_str("no") {
            self.advance();
            self.expect_ident_str("scroll")?;
            Some(false)
        } else {
            None
        };
        // ... existing ForQuery logic, add scroll field
    }
}
```

### Step 3: Update formatter

Update `PlOpenKind` formatting in `src/formatter.rs` to handle `ForExecute` and `scroll`.

### Step 4: Add tests

```rust
#[test]
fn test_plpgsql_open_for_execute() {
    let sql = "DO $$ BEGIN OPEN cur FOR EXECUTE 'SELECT * FROM ' || tbl; END $$";
    // Assert PlOpenKind::ForExecute with correct query
}

#[test]
fn test_plpgsql_open_for_execute_using() {
    let sql = "DO $$ BEGIN OPEN cur FOR EXECUTE q USING 1, 'x'; END $$";
    // Assert ForExecute with using_args
}

#[test]
fn test_plpgsql_open_scroll_for() {
    let sql = "DO $$ BEGIN OPEN cur SCROLL FOR SELECT * FROM t; END $$";
    // Assert ForQuery with scroll = Some(true)
}
```

### Step 5: Run tests + Commit

```bash
cargo test 2>&1 | tail -20
```

```
feat(cursor): distinguish OPEN FOR EXECUTE from FOR USING, add SCROLL option
```

---

## Task 9: Final Integration Testing + JSON Round-Trip Verification

**Why:** All cursor changes must survive the SQL → AST → JSON → AST → SQL round-trip.

**Files:**
- Modify: `src/parser/tests.rs` (integration tests)

### Step 1: Add round-trip tests

```rust
#[test]
fn test_cursor_roundtrip_declare() {
    let cases = vec![
        "DECLARE cur CURSOR FOR SELECT * FROM t",
        "DECLARE cur BINARY SCROLL CURSOR WITH HOLD FOR SELECT id FROM users",
        "DECLARE cur NO SCROLL INSENSITIVE CURSOR WITHOUT HOLD FOR SELECT 1",
        "DECLARE cur CURSOR WITH RETURN TO CALLER FOR SELECT * FROM t",
    ];
    for sql in cases {
        let tokens = Tokenizer::new(sql).tokenize().unwrap();
        let stmts = Parser::new(tokens).parse().unwrap();
        let json = serde_json::to_string(&stmts).unwrap();
        let restored: Vec<Statement> = serde_json::from_str(&json).unwrap();
        let formatter = SqlFormatter::new();
        let output: Vec<String> = restored.iter()
            .map(|s| formatter.format_statement(s))
            .collect();
        let result = output.join(";\n");
        // Parse again and compare ASTs
        let tokens2 = Tokenizer::new(&result).tokenize().unwrap();
        let stmts2 = Parser::new(tokens2).parse().unwrap();
        assert_eq!(stmts, stmts2, "Round-trip failed for: {}", sql);
    }
}

#[test]
fn test_cursor_roundtrip_fetch_move() {
    let cases = vec![
        "FETCH NEXT FROM cur1",
        "FETCH FORWARD 5 IN cur1",
        "FETCH ALL FROM cur1",
        "MOVE PRIOR FROM cur1",
        "MOVE BACKWARD 3 FROM cur1",
    ];
    // Same round-trip pattern as above
}

#[test]
fn test_cursor_roundtrip_close() {
    let cases = vec![
        "CLOSE cur1",
        "CLOSE ALL",
    ];
    // Same round-trip pattern
}

#[test]
fn test_cursor_roundtrip_update_current_of() {
    let sql = "UPDATE t SET x = 1 WHERE CURRENT OF cur";
    // Round-trip test
}
```

### Step 2: Run full test suite + regression

```bash
cargo test 2>&1 | tail -20
cargo run --example regression 2>&1 | tail -20
```

### Step 3: Commit

```
test(cursor): add comprehensive round-trip and integration tests
```

---

## Dependency Graph

```
Task 1 (DeclareCursorStatement refactor)
  ↓
Task 2 (FetchDirection + IN keyword) ← independent from Task 1
  ↓
Task 3 (MOVE structure) ← depends on Task 2 (reuses FetchDirection)
  ↓
Task 4 (CLOSE ALL) ← independent
  ↓
Task 5 (WHERE CURRENT OF) ← independent
  ↓
Task 6 (PL/pgSQL FetchDirection) ← independent
  ↓
Task 7 (GaussDB extensions) ← depends on Task 1
  ↓
Task 8 (OPEN FOR EXECUTE) ← independent
  ↓
Task 9 (Integration tests) ← depends on ALL
```

**Parallelizable:** Tasks 4, 5, 6, 8 can run in parallel after Tasks 1-3.
**Sequential:** Task 1 → Task 2 → Task 3 → (parallel 4,5,6,8) → Task 7 → Task 9

---

## Estimated Scope

| Task | Files Changed | New AST Types | Tests Added |
|------|--------------|---------------|-------------|
| 1. DeclareCursorStatement refactor | 4 | 3 enums | 4 |
| 2. FetchDirection + IN keyword | 4 | 0 (modify existing) | 4 |
| 3. Structure MOVE | 3 | 0 (modify existing) | 4 |
| 4. CLOSE ALL | 3 | 1 enum | 2 |
| 5. WHERE CURRENT OF | 4 | 1 Expr variant | 2 |
| 6. PL/pgSQL FetchDirection fix | 4 | 0 (modify existing) | 3 |
| 7. GaussDB extensions | 4 | 2 enums | 2 |
| 8. OPEN FOR EXECUTE | 4 | 1 enum variant | 3 |
| 9. Integration tests | 1 | 0 | 4 |
| **Total** | ~31 file edits | 7 new types | ~28 new tests |
