# PL Variable Resolution Phase 2 — Remaining Blind Spots

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Upgrade remaining PL/pgSQL variable references from raw `String` to `Expr` so they can resolve as `PlVariable`, and structure the RAISE parser to extract params/options.

**Architecture:** Four independent changes, each touching 1 AST field + 1 parser function + formatter + tests. RAISE is the most complex (requires splitting a monolithic `skip_to_semicolon_or_keyword` into structured parsing). All changes maintain JSON backward compatibility via `#[serde(default)]`.

**Tech Stack:** Rust, existing recursive descent parser, existing `Expr::PlVariable(ObjectName)` variant.

**Test baseline:** 887 tests passing on `main` branch.

---

## Task 1: Assignment target `String → Expr`

**Why:** `v_count := v_count + 1` — the `target` is always a declared PL variable. Currently `String`, should be `Expr::PlVariable`.

**Files:**
- Modify: `src/ast/plpgsql.rs` line 127 — `target: String` → `target: crate::ast::Expr`
- Modify: `src/parser/plpgsql.rs` line 953 — `parse_identifier()` → construct `Expr::ColumnRef` or `Expr::PlVariable`
- Modify: `src/formatter.rs` line 4707 — `target` → `self.format_expr(target)`
- Modify: `src/ast/visitor.rs` line 114 — also walk `target` expr
- Modify: `src/parser/tests.rs` — update existing assertions

### Step 1: Write failing tests

Add to `src/parser/tests.rs` after existing assignment tests:

```rust
#[test]
fn test_plpgsql_assignment_target_is_plvariable() {
    let sql = "DO $$ DECLARE v_count INTEGER; BEGIN v_count := v_count + 1; END $$";
    let block = parse_do_block(sql);
    match &block.body[0] {
        PlStatement::Assignment { target, expression } => {
            // target should be PlVariable since v_count is declared
            assert!(matches!(target, Expr::PlVariable(n) if n == &["v_count"]), "expected PlVariable, got {:?}", target);
            // RHS should also resolve v_count as PlVariable
        }
        _ => panic!("expected Assignment"),
    }
}

#[test]
fn test_plpgsql_assignment_target_undeclared_is_columnref() {
    // Without DECLARE, v_count is not in scope → ColumnRef
    let block = parse_do_block("DO $$ BEGIN v_count := 1; END $$");
    match &block.body[0] {
        PlStatement::Assignment { target, .. } => {
            assert!(matches!(target, Expr::ColumnRef(n) if n == &["v_count"]), "expected ColumnRef, got {:?}", target);
        }
        _ => panic!("expected Assignment"),
    }
}
```

### Step 2: Run tests to verify RED

```bash
cargo test test_plpgsql_assignment_target 2>&1 | grep "error\["
```
Expected: `E0026` — variant `Assignment` does not have field layout matching `Expr::PlVariable` check (or `target` type mismatch).

### Step 3: Change AST

In `src/ast/plpgsql.rs` line 126-129, change:

```rust
// BEFORE:
Assignment {
    target: String,
    expression: crate::ast::Expr,
},

// AFTER:
Assignment {
    target: crate::ast::Expr,
    expression: crate::ast::Expr,
},
```

### Step 4: Update parser

In `src/parser/plpgsql.rs`, function `parse_pl_sql_or_assignment` (around line 950-964), change:

```rust
// BEFORE:
fn parse_pl_sql_or_assignment(&mut self) -> Result<PlStatement, ParserError> {
    let save = self.pos;
    if matches!(self.peek(), Token::Ident(_) | Token::QuotedIdent(_)) {
        let name = self.parse_identifier().unwrap_or_default();
        if self.match_token(&Token::ColonEquals) {
            self.advance();
            let expression = self.parse_expr()?;
            self.try_consume_semicolon();
            return Ok(PlStatement::Assignment {
                target: name,
                expression,
            });
        }
        self.pos = save;
    }
    // ...

// AFTER:
fn parse_pl_sql_or_assignment(&mut self) -> Result<PlStatement, ParserError> {
    let save = self.pos;
    if matches!(self.peek(), Token::Ident(_) | Token::QuotedIdent(_)) {
        let name = self.parse_identifier().unwrap_or_default();
        if self.match_token(&Token::ColonEquals) {
            self.advance();
            let expression = self.parse_expr()?;
            self.try_consume_semicolon();
            let target = if !self.scope_stack.is_empty()
                && self.is_var_declared(&name.to_lowercase())
            {
                Expr::PlVariable(ObjectName::from(vec![name]))
            } else {
                Expr::ColumnRef(ObjectName::from(vec![name]))
            };
            return Ok(PlStatement::Assignment { target, expression });
        }
        self.pos = save;
    }
    // ...
```

Note: We keep `parse_identifier()` (not `parse_expr()`) because the left-hand side of `:=` is always a simple identifier. We manually resolve it using the same scope check logic from `parse_primary_expr` in `expr.rs`.

### Step 5: Update formatter

In `src/formatter.rs` line 4706-4708, change:

```rust
// BEFORE:
PlStatement::Assignment { target, expression } => {
    format!("{} := {};", target, self.format_expr(expression))
}

// AFTER:
PlStatement::Assignment { target, expression } => {
    format!("{} := {};", self.format_expr(target), self.format_expr(expression))
}
```

### Step 6: Update visitor

In `src/ast/visitor.rs` line 114-116, change:

```rust
// BEFORE:
crate::ast::plpgsql::PlStatement::Assignment { expression, .. } => {
    walk_expr(visitor, expression)
}

// AFTER:
crate::ast::plpgsql::PlStatement::Assignment { target, expression } => {
    if walk_expr(visitor, target) == VisitorResult::Stop {
        return VisitorResult::Stop;
    }
    walk_expr(visitor, expression)
}
```

### Step 7: Update existing test assertions

Search for `PlStatement::Assignment` in `src/parser/tests.rs` and update `target` field access from string comparison to `Expr` matching. The existing tests likely just use `matches!` so may already pass.

```bash
grep -n 'Assignment' src/parser/tests.rs
```

### Step 8: Run all tests

```bash
cargo test 2>&1 | tail -5
```
Expected: 889 passed (887 + 2 new).

### Step 9: Commit

```bash
git add -A && git commit -m "feat: upgrade assignment target from String to Expr for PL variable resolution"
```

---

## Task 2: GET DIAGNOSTICS target `String → Expr`

**Why:** `GET DIAGNOSTICS v_rc = ROW_COUNT` — `v_rc` is always a declared PL variable.

**Files:**
- Modify: `src/ast/plpgsql.rs` line 532 — `target: String` → `target: crate::ast::Expr`
- Modify: `src/parser/plpgsql.rs` line 1822 — construct `Expr::PlVariable` or `Expr::ColumnRef`
- Modify: `src/formatter.rs` line 4891 — `i.target` → `self.format_expr(&i.target)`
- Modify: `src/parser/tests.rs` — update assertions

### Step 1: Write failing tests

```rust
#[test]
fn test_plpgsql_get_diag_target_is_plvariable() {
    let sql = "DO $$ DECLARE v_rc INTEGER; BEGIN GET DIAGNOSTICS v_rc = ROW_COUNT; END $$";
    let block = parse_do_block(sql);
    match &block.body[0] {
        PlStatement::GetDiagnostics(g) => {
            assert_eq!(g.items.len(), 1);
            assert!(matches!(&g.items[0].target, Expr::PlVariable(n) if n == &["v_rc"]));
        }
        _ => panic!("expected GetDiagnostics"),
    }
}
```

### Step 2: Run tests to verify RED

```bash
cargo test test_plpgsql_get_diag_target 2>&1 | grep "error\["
```

### Step 3: Change AST

In `src/ast/plpgsql.rs` line 530-534:

```rust
// BEFORE:
pub struct PlGetDiagItem {
    pub target: String,
    pub item: GetDiagItemKind,
}

// AFTER:
pub struct PlGetDiagItem {
    pub target: crate::ast::Expr,
    pub item: GetDiagItemKind,
}
```

### Step 4: Update parser

In `src/parser/plpgsql.rs`, function `parse_pl_get_diagnostics`, around line 1822:

```rust
// BEFORE:
let target = self.parse_identifier()?;

// AFTER:
let target_name = self.parse_identifier()?;
let target = if !self.scope_stack.is_empty()
    && self.is_var_declared(&target_name.to_lowercase())
{
    Expr::PlVariable(ObjectName::from(vec![target_name]))
} else {
    Expr::ColumnRef(ObjectName::from(vec![target_name]))
};
```

### Step 5: Update formatter

In `src/formatter.rs` line 4888-4894:

```rust
// BEFORE:
.map(|i| format!("{} = {}", i.target, i.item))

// AFTER:
.map(|i| format!("{} = {}", self.format_expr(&i.target), i.item))
```

### Step 6: Update existing test assertions

Find `GetDiagnostics` test assertions in `src/parser/tests.rs`:

```bash
grep -n 'GetDiagnostics\|\.target.*==' src/parser/tests.rs
```

Update `assert_eq!(g.items[0].target, "x")` to `assert!(matches!(&g.items[0].target, Expr::ColumnRef(n) if n == &["x"]))` (or `Expr::PlVariable` if declared).

### Step 7: Run all tests

```bash
cargo test 2>&1 | tail -5
```
Expected: 890 passed.

### Step 8: Commit

```bash
git add -A && git commit -m "feat: upgrade GET DIAGNOSTICS target from String to Expr for PL variable resolution"
```

---

## Task 3: FOR cursor loop cursor_name `String → Expr`

**Why:** `FOR rec IN cur_name LOOP` — `cur_name` references a declared REFCURSOR or CURSOR variable.

**Files:**
- Modify: `src/ast/plpgsql.rs` line 325 — `cursor_name: String` → `cursor_name: crate::ast::Expr`
- Modify: `src/parser/plpgsql.rs` line 1183 — `parse_identifier()` → `parse_expr()`
- Modify: `src/formatter.rs` line 5095 — `cursor_name` → `self.format_expr(cursor_name)`
- Modify: `src/ast/visitor.rs` line 212 — walk `cursor_name`
- Modify: `src/parser/tests.rs` — update assertions

### Step 1: Write failing tests

```rust
#[test]
fn test_plpgsql_for_cursor_name_is_plvariable() {
    let sql = "DO $$ DECLARE cur REFCURSOR; BEGIN FOR rec IN cur LOOP NULL; END LOOP; END $$";
    let block = parse_do_block(sql);
    match &block.body[0] {
        PlStatement::For(f) => {
            match &f.kind {
                PlForKind::Cursor { cursor_name, .. } => {
                    assert!(matches!(cursor_name, Expr::PlVariable(n) if n == &["cur"]),
                        "expected PlVariable, got {:?}", cursor_name);
                }
                other => panic!("expected Cursor kind, got {:?}", other),
            }
        }
        _ => panic!("expected For"),
    }
}
```

### Step 2: Run tests to verify RED

### Step 3: Change AST

In `src/ast/plpgsql.rs` line 323-327:

```rust
// BEFORE:
Cursor {
    cursor_name: String,
    arguments: Vec<crate::ast::Expr>,
},

// AFTER:
Cursor {
    cursor_name: crate::ast::Expr,
    arguments: Vec<crate::ast::Expr>,
},
```

### Step 4: Update parser

In `src/parser/plpgsql.rs`, function `parse_pl_for_kind`, around line 1183-1204:

```rust
// BEFORE (line 1183-1185):
if let Ok(name) = self.parse_identifier() {
    if self.match_ident_str("loop") || matches!(self.peek(), Token::LParen) {
        let mut arguments = Vec::new();

// AFTER:
if let Ok(name_expr) = self.parse_expr() {
    let is_cursor_ref = matches!(&name_expr, Expr::ColumnRef(_) | Expr::PlVariable(_));
    if is_cursor_ref && (self.match_ident_str("loop") || matches!(self.peek(), Token::LParen)) {
        let mut arguments = Vec::new();
```

And in the return at line 1200-1203:

```rust
// BEFORE:
return Ok(PlForKind::Cursor {
    cursor_name: name,
    arguments,
});

// AFTER:
return Ok(PlForKind::Cursor {
    cursor_name: name_expr,
    arguments,
});
```

⚠️ **IMPORTANT:** `parse_expr()` is more aggressive than `parse_identifier()` — it may consume tokens that aren't a simple name. The `is_cursor_ref` check ensures we only enter the cursor path for `ColumnRef`/`PlVariable` results. If `parse_expr()` succeeds but the result isn't a simple name, we need to backtrack:

```rust
let saved_pos = self.pos;
if let Ok(name_expr) = self.parse_expr() {
    let is_simple_name = matches!(&name_expr, Expr::ColumnRef(_) | Expr::PlVariable(_));
    if is_simple_name && (self.match_ident_str("loop") || matches!(self.peek(), Token::LParen)) {
        let mut arguments = Vec::new();
        if self.match_token(&Token::LParen) {
            self.advance();
            if !self.match_token(&Token::RParen) {
                loop {
                    arguments.push(self.parse_expr()?);
                    if self.match_token(&Token::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            self.expect_token(&Token::RParen)?;
        }
        return Ok(PlForKind::Cursor {
            cursor_name: name_expr,
            arguments,
        });
    }
    // Not a cursor reference, backtrack to try range parse
    self.pos = saved_pos;
}
```

### Step 5: Update formatter

In `src/formatter.rs` line 5091-5099:

```rust
// BEFORE:
PlForKind::Cursor { cursor_name, arguments } => {
    s.push_str(cursor_name);
    // ...

// AFTER:
PlForKind::Cursor { cursor_name, arguments } => {
    s.push_str(&self.format_expr(cursor_name));
    // ...
```

### Step 6: Update visitor

In `src/ast/visitor.rs` line 212-218:

```rust
// BEFORE:
crate::ast::plpgsql::PlForKind::Cursor { arguments, .. } => {
    for arg in arguments {
        // ...

// AFTER:
crate::ast::plpgsql::PlForKind::Cursor { cursor_name, arguments } => {
    if walk_expr(visitor, cursor_name) == VisitorResult::Stop {
        return VisitorResult::Stop;
    }
    for arg in arguments {
        // ...
```

### Step 7: Update existing test assertions

```bash
grep -n 'cursor_name' src/parser/tests.rs
```

Update `assert_eq!(f.cursor_name, "cur1")` to `assert!(matches!(&f.cursor_name, Expr::ColumnRef(n) if n == &["cur1"]))`.

### Step 8: Run all tests

```bash
cargo test 2>&1 | tail -5
```
Expected: 891 passed.

### Step 9: Commit

```bash
git add -A && git commit -m "feat: upgrade FOR cursor name from String to Expr for PL variable resolution"
```

---

## Task 4: RAISE structured parsing

**Why:** `RAISE NOTICE 'Count: %', v_count` — the entire message+params+options is currently swallowed as one raw string. The AST has `params: Vec<Expr>` and `options: Vec<RaiseOption>` fields ready but never populated. This is the highest-value fix because RAISE is the most common debugging/logging statement in PL/pgSQL.

**Complexity:** This is the most complex task. The RAISE syntax has multiple forms:

1. `RAISE;` — re-raise (no args)
2. `RAISE level;` — level only
3. `RAISE level 'format', param1, param2 USING option = expr;` — full form
4. `RAISE condition_name;` — named condition
5. `RAISE SQLSTATE 'code', 'message';` — SQLSTATE form

**Files:**
- Modify: `src/parser/plpgsql.rs` line 1399-1457 — `parse_pl_raise()`
- Modify: `src/parser/tests.rs` — update existing RAISE test assertions

The AST (`PlRaiseStmt`) already has the right fields. The formatter already handles them. Only the parser needs to be rewritten.

### Step 1: Write failing tests

```rust
#[test]
fn test_plpgsql_raise_with_variable_param() {
    let sql = "DO $$ DECLARE v_name TEXT; BEGIN RAISE NOTICE 'Hello %', v_name; END $$";
    let block = parse_do_block(sql);
    match &block.body[0] {
        PlStatement::Raise(r) => {
            assert!(matches!(r.level, Some(RaiseLevel::Notice)));
            assert!(r.message.is_some());
            assert_eq!(r.params.len(), 1);
            assert!(matches!(&r.params[0], Expr::PlVariable(n) if n == &["v_name"]),
                "expected PlVariable for v_name, got {:?}", r.params[0]);
        }
        _ => panic!("expected Raise"),
    }
}

#[test]
fn test_plpgsql_raise_with_multiple_params() {
    let sql = "DO $$ DECLARE v_a INT; v_b INT; BEGIN RAISE NOTICE '% and %', v_a, v_b; END $$";
    let block = parse_do_block(sql);
    match &block.body[0] {
        PlStatement::Raise(r) => {
            assert_eq!(r.params.len(), 2);
            assert!(matches!(&r.params[0], Expr::PlVariable(n) if n == &["v_a"]));
            assert!(matches!(&r.params[1], Expr::PlVariable(n) if n == &["v_b"]));
        }
        _ => panic!("expected Raise"),
    }
}

#[test]
fn test_plpgsql_raise_with_using() {
    let block = parse_do_block("DO $$ BEGIN RAISE EXCEPTION USING ERRCODE = '12345'; END $$");
    match &block.body[0] {
        PlStatement::Raise(r) => {
            assert!(matches!(r.level, Some(RaiseLevel::Exception)));
            assert_eq!(r.options.len(), 1);
            assert_eq!(r.options[0].name.to_uppercase(), "ERRCODE");
        }
        _ => panic!("expected Raise"),
    }
}

#[test]
fn test_plpgsql_raise_condition_only() {
    let block = parse_do_block("DO $$ BEGIN RAISE division_by_zero; END $$");
    match &block.body[0] {
        PlStatement::Raise(r) => {
            assert!(r.level.is_none());
            assert!(r.condname.is_some());
            assert_eq!(r.condname.as_deref(), Some("division_by_zero"));
            assert!(r.message.is_none());
        }
        _ => panic!("expected Raise"),
    }
}
```

### Step 2: Run tests to verify RED

```bash
cargo test test_plpgsql_raise 2>&1 | tail -15
```
Expected: New tests fail because `params` is empty and `options` is empty (current parser never populates them).

### Step 3: Rewrite `parse_pl_raise()`

Replace the entire function body in `src/parser/plpgsql.rs` (lines 1399-1457). The new logic:

```rust
fn parse_pl_raise(&mut self) -> Result<PlStatement, ParserError> {
    self.advance(); // consume "raise"

    // Form 1: RAISE; (re-raise)
    if matches!(self.peek(), Token::Semicolon) {
        self.advance();
        return Ok(PlStatement::Raise(PlRaiseStmt {
            level: None,
            message: None,
            params: Vec::new(),
            options: Vec::new(),
            condname: None,
            sqlstate: None,
        }));
    }

    let level = if self.match_ident_str("debug") {
        Some(RaiseLevel::Debug)
    } else if self.match_ident_str("log") {
        Some(RaiseLevel::Log)
    } else if self.match_ident_str("info") {
        Some(RaiseLevel::Info)
    } else if self.match_ident_str("notice") {
        Some(RaiseLevel::Notice)
    } else if self.match_ident_str("warning") {
        Some(RaiseLevel::Warning)
    } else if self.match_ident_str("exception") {
        Some(RaiseLevel::Exception)
    } else {
        None
    };

    if level.is_some() {
        self.advance();
    }

    // Form 2: RAISE level; (level-only raise)
    if matches!(self.peek(), Token::Semicolon) {
        self.advance();
        return Ok(PlStatement::Raise(PlRaiseStmt {
            level,
            message: None,
            params: Vec::new(),
            options: Vec::new(),
            condname: None,
            sqlstate: None,
        }));
    }

    // Check for SQLSTATE form: RAISE [level] SQLSTATE 'code'
    if level.is_none() && self.match_ident_str("sqlstate") {
        self.advance();
        let code = if let Token::StringLiteral(s) = self.peek().clone() {
            self.advance();
            Some(s)
        } else {
            None
        };
        let message = if self.match_token(&Token::Comma) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.try_consume_semicolon();
        return Ok(PlStatement::Raise(PlRaiseStmt {
            level: None,
            message: message.map(|e| self.expr_to_raw_string(&e)),
            params: Vec::new(),
            options: Vec::new(),
            condname: None,
            sqlstate: code,
        }));
    }

    // Check for condition name form: RAISE condition_name
    // A condition name is a bare identifier NOT followed by a string literal or comma
    if level.is_none() {
        let save_pos = self.pos;
        if let Ok(name) = self.parse_identifier() {
            let is_followed_by_semicolon = matches!(self.peek(), Token::Semicolon);
            if is_followed_by_semicolon {
                self.try_consume_semicolon();
                return Ok(PlStatement::Raise(PlRaiseStmt {
                    level: None,
                    message: None,
                    params: Vec::new(),
                    options: Vec::new(),
                    condname: Some(name),
                    sqlstate: None,
                }));
            }
        }
        self.pos = save_pos;
    }

    // Form 3: RAISE [level] 'format_string', param1, param2 [USING option = expr, ...]
    let message = self.parse_expr()?;
    let message_str = self.expr_to_raw_string(&message);

    let mut params = Vec::new();
    while self.match_token(&Token::Comma) {
        self.advance();
        // Check if this is USING keyword (not a param)
        if self.match_ident_str("using") {
            self.advance();
            break;
        }
        params.push(self.parse_expr()?);
    }

    let mut options = Vec::new();
    if self.match_ident_str("using") {
        self.advance();
        loop {
            let opt_name = self.parse_identifier()?;
            self.expect_token(&Token::Eq)?;
            let opt_value = self.parse_expr()?;
            options.push(RaiseOption {
                name: opt_name,
                value: opt_value,
            });
            if self.match_token(&Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }
    }

    self.try_consume_semicolon();

    Ok(PlStatement::Raise(PlStatement::Raise(PlRaiseStmt {
        level,
        message: Some(message_str),
        params,
        options,
        condname: None,
        sqlstate: None,
    })))
}
```

⚠️ **Note on `expr_to_raw_string`:** This is a helper that converts an `Expr` back to a string for the `message` field. If it doesn't exist, use `self.tokens_to_raw_string(save_pos, self.pos)` instead (save `self.pos` before parsing the message expr).

**Important consideration:** The existing tests assert that `r.message` contains the raw text (e.g., `Some("'hello'")`). After the change, `message` will be derived from `parse_expr()`. For string literals, `expr_to_raw_string` should produce `'hello'`. We may need to store the raw text via `tokens_to_raw_string` to preserve exact formatting.

**Alternative simpler approach:** Keep `message: Option<String>` populated via `tokens_to_raw_string`, but parse params and options:

```rust
// Simpler approach: parse message, params, options separately
let msg_start = self.pos;
let message_expr = self.parse_expr()?;
let message = self.tokens_to_raw_string(msg_start, self.pos);

let mut params = Vec::new();
while self.match_token(&Token::Comma) {
    self.advance();
    if self.match_ident_str("using") {
        self.advance();
        break;
    }
    params.push(self.parse_expr()?);
}
// ... rest of USING handling
```

### Step 4: Update existing RAISE test assertions

The existing tests (lines 462-537) need updates:

- `test_plpgsql_raise_notice`: `r.message` changes from `Some("'hello'")` to `Some("' hello'")` or similar — adjust based on actual parse result
- `test_plpgsql_raise_format_params`: `r.params` should now be non-empty; `r.message` should just be the format string portion
- `test_plpgsql_raise_using_errcode`: `r.options` should now be populated; `r.message` should be None
- `test_plpgsql_raise_condition_name`: `r.condname` should now be `Some("division_by_zero")`; `r.message` should be None

**⚠️ The exact string values may differ after the refactor. Read the actual test output and adjust accordingly.**

### Step 5: Run all tests

```bash
cargo test 2>&1 | tail -5
```
Expected: 895 passed (891 + 4 new RAISE tests).

### Step 6: Commit

```bash
git add -A && git commit -m "feat: structure RAISE parser to extract params and options with PL variable resolution"
```

---

## Task 5: FORALL bounds `String → structured range`

**Why:** `FORALL i IN 1..v_count` — `v_count` is a PL variable swallowed as raw text. However, this is Oracle-specific syntax and less common than the other fixes.

**Priority:** LOW. Consider deferring unless explicitly needed.

**Files:**
- Modify: `src/ast/plpgsql.rs` line 342-346 — `PlForAllStmt { bounds: String, body: String }`
- Modify: `src/parser/plpgsql.rs` line 1865-1957 — rewrite `parse_pl_forall()`
- Modify: `src/formatter.rs` line 4935-4949

**Design:** Replace `bounds: String` with structured `low: Expr, high: Expr`, and parse them via `parse_expr()`.

---

## Task 6: Loop variable declarations (consistency, optional)

**Why:** `PlForStmt.variable`, `PlForEachStmt.variable`, `PlForAllStmt.variable` are `String` — they ARE declarations, not references, so `String` is semantically correct. Upgrading to `Expr` is purely for uniformity.

**Priority:** LOWEST. Defer unless there's a downstream need.

---

## Dependency Graph

```
Task 1 (Assignment)  ─┐
Task 2 (GET DIAG)    ─┤── independent, can be parallelized
Task 3 (FOR cursor)  ─┤
Task 4 (RAISE)       ─┘
Task 5 (FORALL)      ─── deferred
Task 6 (Loop vars)   ─── deferred
```

Tasks 1-4 are independent of each other and can be implemented in any order or in parallel. Each is a self-contained change with its own test, AST, parser, formatter, and visitor updates.

## Risk Assessment

| Task | Risk | Mitigation |
|------|------|-----------|
| Task 1 (Assignment) | LOW — simple field type change, `parse_identifier` kept | Manual PlVariable wrapping identical to existing pattern |
| Task 2 (GET DIAG) | LOW — same pattern as Task 1 | Same approach |
| Task 3 (FOR cursor) | MEDIUM — replacing `parse_identifier` with `parse_expr` may consume too many tokens | Backtrack on failure; only enter cursor path for `ColumnRef`/`PlVariable` results |
| Task 4 (RAISE) | HIGH — complete parser rewrite; 5 existing tests must still pass | Incremental approach: keep raw text for message via `tokens_to_raw_string`, but parse params/options with `parse_expr()` |
| Task 5 (FORALL) | MEDIUM — Oracle-specific, less tested | Defer |
