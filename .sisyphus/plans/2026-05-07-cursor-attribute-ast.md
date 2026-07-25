# CursorAttribute AST Node Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace `BinaryOp(op="%")` with a dedicated `Expr::CursorAttribute` AST node for PL/pgSQL cursor attributes (`%NOTFOUND`, `%FOUND`, `%ISOPEN`, `%ROWCOUNT`, `%BULK_EXCEPTIONS`).

**Architecture:** Intercept the `%` token in the Pratt expression parser loop. When inside a PL/pgSQL scope (`!scope_stack.is_empty()`) and the right-hand token is a known cursor attribute keyword, emit `Expr::CursorAttribute` instead of `Expr::BinaryOp`. This mirrors the existing `::` → `TypeCast` interception pattern at `src/parser/expr.rs:73-82`.

**Tech Stack:** Rust, recursive descent parser, Pratt expression parser, serde JSON serialization.

---

### Task 1: Add `CursorAttributeKind` enum and `Expr::CursorAttribute` variant to AST

**Files:**
- Modify: `src/ast/mod.rs:1381` (insert before `PlVariable`)

**Step 1: Add the `CursorAttributeKind` enum**

Insert immediately before the `WhenClause` struct (line 1384), after the closing `}` of the `Expr` enum:

```rust
/// PL/pgSQL cursor attribute: `cursor_name%NOTFOUND`, `cursor_name%FOUND`, etc.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CursorAttributeKind {
    NotFound,
    Found,
    IsOpen,
    RowCount,
    BulkExceptions,
}
```

**Step 2: Add `CursorAttribute` variant to the `Expr` enum**

Insert inside the `Expr` enum, just before the `PlVariable` variant (line 1381):

```rust
    /// PL/pgSQL cursor attribute: cursor_name%NOTFOUND, cursor_name%FOUND, etc.
    CursorAttribute {
        cursor: Box<Expr>,
        attribute: CursorAttributeKind,
    },
```

**Step 3: Verify compilation**

Run: `cargo build 2>&1 | grep "^error"`
Expected: Multiple "non-exhaustive patterns" errors from `formatter.rs`, `visitor.rs`, `analyzer/mod.rs` — this is expected. The next tasks fix each.

**Step 4: Commit**

```bash
git add src/ast/mod.rs
git commit -m "feat: add CursorAttribute AST node and CursorAttributeKind enum"
```

---

### Task 2: Intercept `%` in the Pratt parser

**Files:**
- Modify: `src/parser/expr.rs:73-82` (add cursor attribute interception alongside `::` → TypeCast)

**Step 1: Write the failing test**

Append to `src/parser/tests.rs`:

```rust
#[test]
fn test_cursor_attribute_notfound() {
    let sql = "DO $$ DECLARE c CURSOR FOR SELECT 1; BEGIN EXIT WHEN c%NOTFOUND; END $$";
    let block = parse_do_block(sql);
    let exit = block.body.iter().find_map(|s| match s {
        PlStatement::Exit { condition, .. } => condition.clone(),
        _ => None,
    }).expect("should have EXIT");
    match exit {
        Expr::CursorAttribute { cursor, attribute } => {
            assert!(matches!(cursor.as_ref(), Expr::PlVariable(n) if n[0] == "c"));
            assert_eq!(attribute, CursorAttributeKind::NotFound);
        }
        other => panic!("expected CursorAttribute, got {:?}", other),
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --lib test_cursor_attribute_notfound -- --nocapture 2>&1 | tail -5`
Expected: FAIL — `Expr::CursorAttribute` variant not matched (currently produces `BinaryOp`).

**Step 3: Implement the interception**

In `src/parser/expr.rs`, right after the `::` → TypeCast block (lines 73-82), add:

```rust
            if op_str == "%" && !self.scope_stack.is_empty() {
                if let Some(attr) = self.try_parse_cursor_attribute() {
                    left = Expr::CursorAttribute {
                        cursor: Box::new(left),
                        attribute: attr,
                    };
                    continue;
                }
            }
```

Then add the helper method `try_parse_cursor_attribute` to the `impl Parser` block in `expr.rs`:

```rust
    fn try_parse_cursor_attribute(&mut self) -> Option<crate::ast::CursorAttributeKind> {
        let attr = match self.peek() {
            Token::Ident(s) => match s.to_uppercase().as_str() {
                "NOTFOUND" => crate::ast::CursorAttributeKind::NotFound,
                "FOUND" => crate::ast::CursorAttributeKind::Found,
                "ISOPEN" => crate::ast::CursorAttributeKind::IsOpen,
                "ROWCOUNT" => crate::ast::CursorAttributeKind::RowCount,
                "BULK_EXCEPTIONS" => crate::ast::CursorAttributeKind::BulkExceptions,
                _ => return None,
            },
            _ => return None,
        };
        self.advance();
        Some(attr)
    }
```

**Key design notes:**
- `!self.scope_stack.is_empty()` ensures we only intercept in PL/pgSQL context
- After consuming `%` (the `self.advance()` at line 71), we peek at the next token
- `NOTFOUND`/`FOUND`/`ISOPEN`/`ROWCOUNT`/`BULK_EXCEPTIONS` are NOT keywords — they are identifiers, so we match `Token::Ident`
- If the next token is not a known attribute, returns `None` and falls through to regular `BinaryOp("%")` for modulo

**Step 4: Run test to verify it passes**

Run: `cargo test --lib test_cursor_attribute_notfound -- --nocapture 2>&1 | tail -5`
Expected: PASS

**Step 5: Commit**

```bash
git add src/parser/expr.rs src/parser/tests.rs
git commit -m "feat: intercept % in Pratt parser to produce CursorAttribute in PL context"
```

---

### Task 3: Add comprehensive tests for all cursor attributes and modulo fallback

**Files:**
- Modify: `src/parser/tests.rs`

**Step 1: Write tests covering all attributes + modulo fallback**

```rust
#[test]
fn test_cursor_attribute_found() {
    let sql = "DO $$ DECLARE c CURSOR FOR SELECT 1; BEGIN IF c%FOUND THEN NULL; END IF; END $$";
    let block = parse_do_block(sql);
    let cond = block.body.iter().find_map(|s| match s {
        PlStatement::If { condition, .. } => Some(condition.clone()),
        _ => None,
    }).expect("should have IF");
    match cond.as_ref() {
        Expr::CursorAttribute { attribute, .. } => {
            assert_eq!(*attribute, CursorAttributeKind::Found);
        }
        other => panic!("expected CursorAttribute, got {:?}", other),
    }
}

#[test]
fn test_cursor_attribute_isopen() {
    let sql = "DO $$ DECLARE c CURSOR FOR SELECT 1; BEGIN IF NOT c%ISOPEN THEN NULL; END IF; END $$";
    let block = parse_do_block(sql);
    let cond = block.body.iter().find_map(|s| match s {
        PlStatement::If { condition, .. } => Some(condition.clone()),
        _ => None,
    }).expect("should have IF");
    // Should be UnaryOp(NOT, CursorAttribute)
    match cond.as_ref() {
        Expr::UnaryOp { op, expr } => {
            assert_eq!(op, "NOT");
            match expr.as_ref() {
                Expr::CursorAttribute { attribute, .. } => {
                    assert_eq!(*attribute, CursorAttributeKind::IsOpen);
                }
                other => panic!("expected CursorAttribute inside NOT, got {:?}", other),
            }
        }
        other => panic!("expected UnaryOp(NOT), got {:?}", other),
    }
}

#[test]
fn test_cursor_attribute_rowcount() {
    let sql = "DO $$ DECLARE c CURSOR FOR SELECT 1; v_count INT; BEGIN v_count := c%ROWCOUNT; END $$";
    let block = parse_do_block(sql);
    let expr = block.body.iter().find_map(|s| match s {
        PlStatement::Assignment { expression, .. } => Some(expression.clone()),
        _ => None,
    }).expect("should have assignment");
    match expr.as_ref() {
        Expr::CursorAttribute { attribute, .. } => {
            assert_eq!(*attribute, CursorAttributeKind::RowCount);
        }
        other => panic!("expected CursorAttribute, got {:?}", other),
    }
}

#[test]
fn test_cursor_attribute_bulk_exceptions() {
    let sql = "DO $$ DECLARE c CURSOR FOR SELECT 1; BEGIN IF c%BULK_EXCEPTIONS THEN NULL; END IF; END $$";
    let block = parse_do_block(sql);
    let cond = block.body.iter().find_map(|s| match s {
        PlStatement::If { condition, .. } => Some(condition.clone()),
        _ => None,
    }).expect("should have IF");
    match cond.as_ref() {
        Expr::CursorAttribute { attribute, .. } => {
            assert_eq!(*attribute, CursorAttributeKind::BulkExceptions);
        }
        other => panic!("expected CursorAttribute, got {:?}", other),
    }
}

#[test]
fn test_percent_still_modulo_outside_pl() {
    // Outside PL/pgSQL, % must remain the modulo operator
    let sql = "SELECT 10 % 3";
    let stmt = parse_one(sql);
    match &stmt {
        Statement::Select(sel) => {
            let target = &sel.body.target_list[0];
            match target {
                SelectTarget::Expr(expr, _) => match expr {
                    Expr::BinaryOp { op, .. } => assert_eq!(op, "%"),
                    other => panic!("expected BinaryOp, got {:?}", other),
                },
                other => panic!("expected Expr target, got {:?}", other),
            }
        }
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_percent_still_modulo_in_pl_with_number() {
    // In PL/pgSQL, x % 2 should still be modulo (right side is a number, not a cursor attribute)
    let sql = "DO $$ DECLARE v INT; BEGIN v := v % 2; END $$";
    let block = parse_do_block(sql);
    let expr = block.body.iter().find_map(|s| match s {
        PlStatement::Assignment { expression, .. } => Some(expression.clone()),
        _ => None,
    }).expect("should have assignment");
    match expr.as_ref() {
        Expr::BinaryOp { op, .. } => assert_eq!(op, "%"),
        other => panic!("expected BinaryOp(modulo), got {:?}", other),
    }
}
```

**Step 2: Run all new tests**

Run: `cargo test --lib test_cursor_attribute -- --nocapture 2>&1 | tail -20`
Expected: All 7 tests pass (notfound, found, isopen, rowcount, bulk_exceptions, modulo_outside_pl, modulo_in_pl).

**Step 3: Run full test suite**

Run: `cargo test --lib 2>&1 | tail -5`
Expected: All 1101 + new tests pass, 0 failed.

**Step 4: Commit**

```bash
git add src/parser/tests.rs
git commit -m "test: add comprehensive tests for CursorAttribute and modulo fallback"
```

---

### Task 4: Add formatter support

**Files:**
- Modify: `src/formatter.rs:1377` (insert before `PlVariable` handler)

**Step 1: Implement**

Insert before the `Expr::PlVariable` handler at line 1377:

```rust
            Expr::CursorAttribute { cursor, attribute } => {
                let attr_str = match attribute {
                    CursorAttributeKind::NotFound => "%NOTFOUND",
                    CursorAttributeKind::Found => "%FOUND",
                    CursorAttributeKind::IsOpen => "%ISOPEN",
                    CursorAttributeKind::RowCount => "%ROWCOUNT",
                    CursorAttributeKind::BulkExceptions => "%BULK_EXCEPTIONS",
                };
                format!("{}{}", self.format_expr(cursor), attr_str)
            }
```

Add the import for `CursorAttributeKind` at the top of `formatter.rs` if needed (check existing imports).

**Step 2: Verify compilation**

Run: `cargo build 2>&1 | grep "^error"`
Expected: No errors related to `CursorAttribute`.

**Step 3: Test round-trip formatting**

Add a test:

```rust
#[test]
fn test_cursor_attribute_format_roundtrip() {
    let sql = "DO $$ DECLARE c CURSOR FOR SELECT 1; BEGIN EXIT WHEN c%NOTFOUND; END $$";
    let block = parse_do_block(sql);
    let formatter = crate::formatter::SqlFormatter::new();
    let output = formatter.format_pl_block(&block);
    assert!(output.contains("c%NOTFOUND"), "formatted output should contain c%NOTFOUND, got: {}", output);
}
```

**Step 4: Commit**

```bash
git add src/formatter.rs src/parser/tests.rs
git commit -m "feat: add CursorAttribute formatting support"
```

---

### Task 5: Add visitor support

**Files:**
- Modify: `src/ast/visitor.rs:1128` (insert before `Expr::PlVariable`)

**Step 1: Implement**

Insert before `Expr::PlVariable(_)` at line 1128:

```rust
        Expr::CursorAttribute { cursor, .. } => {
            if walk_expr(visitor, cursor) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
```

**Step 2: Verify compilation**

Run: `cargo build 2>&1 | grep "^error"`
Expected: No errors.

**Step 3: Commit**

```bash
git add src/ast/visitor.rs
git commit -m "feat: add CursorAttribute visitor support"
```

---

### Task 6: Add analyzer support

**Files:**
- Modify: `src/analyzer/mod.rs`

**Step 1: Handle CursorAttribute in analyzer**

The analyzer uses `PlVariable` and `ColumnRef` in several match arms. The `CursorAttribute` variant needs handling in any exhaustive match on `Expr`. Search for all match arms on `Expr` in the analyzer and add appropriate handling:

```rust
Expr::CursorAttribute { cursor, attribute } => {
    self.collect_expr_references(cursor, symbols);
}
```

The exact placement depends on which analyzer methods match on `Expr`. Use `cargo build` to find all non-exhaustive pattern errors and fix each one.

**Step 2: Verify compilation**

Run: `cargo build 2>&1 | grep "^error"`
Expected: No errors.

**Step 3: Commit**

```bash
git add src/analyzer/mod.rs
git commit -m "feat: handle CursorAttribute in analyzer"
```

---

### Task 7: Final verification

**Step 1: Full build with no errors**

Run: `cargo build 2>&1 | grep "^error"`
Expected: No errors.

**Step 2: Full test suite**

Run: `cargo test --lib 2>&1 | tail -5`
Expected: All tests pass, 0 failed.

**Step 3: Run existing regression tests**

Run: `cargo run --example regression 2>&1 | tail -5`
Expected: All regression tests pass (1409/1409).

**Step 4: JSON round-trip test**

Verify that `CursorAttribute` serializes/deserializes correctly via the existing `serde` derives:

```rust
#[test]
fn test_cursor_attribute_json_roundtrip() {
    let sql = "DO $$ DECLARE c CURSOR FOR SELECT 1; BEGIN EXIT WHEN c%NOTFOUND; END $$";
    let stmt = parse_one(sql);
    let json = serde_json::to_string(&stmt).unwrap();
    let restored: Statement = serde_json::from_str(&json).unwrap();
    assert_eq!(stmt, restored);
}
```

**Step 5: Commit**

```bash
git add -A
git commit -m "test: final verification tests for CursorAttribute"
```
