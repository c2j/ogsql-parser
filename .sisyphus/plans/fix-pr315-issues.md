# Plan: Fix PR#315 Confirmed Issues

## Objective

Fix 7 categories of confirmed real bugs discovered by SQLsmith differential test harness (PR#315), covering expression operators, set operations, tokenizer misclassification, MERGE parser gap, AT TIME ZONE feature gap, function arity, and reserved word alias handling.

## Scope & Non-Scope

**IN SCOPE (7 categories, confirmed real bugs):**
1. P0-A: Compound PG operators split by tokenizer → expression parse failure
2. P1-D: `parse_set_operations` overwrites previous set-op instead of appending to chain tail
3. P1-B: `?` unconditionally mapped to `JdbcParam`, breaking JSONB/geometric operators
4. P1-C: MERGE USING source TABLESAMPLE not consumed
5. P1-A: AT TIME ZONE expression operator not implemented
6. P2: 4 function arity mismatches (concat, width_bucket, regexp_instr, regexp_substr)
7. P2: Reserved keywords rejected as AS-aliases (openGauss allows)

**OUT OF SCOPE:**
- P0-B: Subquery JOIN edge cases (not yet root-caused; `OUTER_P` gap is low-priority)
- P1-C #0019: MERGE WHEN MATCHED AND (verified NOT supported by openGauss)
- P2: `date_trunc` arity (already correct)

## Verification Baseline

Before any changes, run:
```bash
cargo fmt --all -- --check   # must pass
cargo clippy --all-features -- -D warnings   # must pass
cargo test --all-features    # must pass (1772+ tests)
```

After all changes, same 3 commands must pass. Additionally, SQLsmith guard must show regressions empty.

---

## Step 1 — P2: Function arity fixes (lowest risk, quick wins)

### 1a. `concat`: min_args 2 → 1

**File**: `src/parser/function_registry.rs:495`
**Change**: `2, None` → `1, None`
**Reason**: openGauss accepts `concat('a')` (1 arg). Rejects `concat()` (0 args). variadic from 1+.

**Regression test** (`src/parser/tests.rs`):
```rust
#[test]
fn test_func_concat_one_arg_ok() {
    let sql = "SELECT concat('a')";
    let (_stmts, errors) = parse_with_errors(sql);
    let has_concat_warn = errors.iter().any(|e| e.to_string().contains("concat"));
    assert!(!has_concat_warn, "concat with 1 arg should not warn");
}
```

### 1b. `width_bucket`: min_args 3 → 4

**File**: `src/parser/function_registry.rs:1427`
**Change**: `3, Some(4)` → `4, Some(4)`
**Reason**: openGauss only supports 4-arg `width_bucket`. 3-arg call is treated as column reference (no such function).

### 1c. `regexp_instr`: max_args 5 → 6

**File**: `src/parser/function_registry.rs:1277`
**Change**: `2, Some(5)` → `2, Some(6)`
**Reason**: openGauss docs + live DB confirm 6th arg `flags` is valid: `regexp_instr(string, pattern, pos, occ, ret_opt, flags)`.

**Regression test** (`src/parser/tests.rs`):
```rust
#[test]
fn test_func_regexp_instr_6_args_ok() {
    let sql = "SELECT regexp_instr('foobarbaz','b(..)', 1, 1, 0, 'i') FROM t";
    let (_stmts, errors) = parse_with_errors(sql);
    let has_warn = errors.iter().any(|e| e.to_string().contains("regexp_instr"));
    assert!(!has_warn, "regexp_instr with 6 args should not warn");
}
```

### 1d. `regexp_substr`: max_args 4 → 5

**File**: `src/parser/function_registry.rs:1283`
**Change**: `2, Some(4)` → `2, Some(5)`
**Reason**: openGauss accepts 5th arg `flags`. This is already documented as KNOWN BUG in `tests.rs:6955-6967`.

**Regression test**: Convert existing `test_func_regexp_substr_5_args_should_be_ok_integration` from soft-fail to hard assertion (remove the `KNOWN BUG` eprintln and change to assert no warnings).

---

## Step 2 — P2: Reserved keywords as AS-aliases

### 2a. Allow Reserved keywords after `AS` keyword

**File**: `src/parser/mod.rs` — `parse_optional_alias` (around L966)

**Current behavior**: `parse_optional_alias` calls `parse_ident()` which rejects all `Reserved` keywords via `ReservedKeywordAsIdentifier` error.

**Fix**: After consuming `AS` keyword (L968), bypass the reserved-keyword check by using a new method `parse_ident_as_alias()` that skips the `Reserved` check. In the implicit-alias branch (no `AS`), keep existing behavior.

Or more surgically: in `parse_ident()`, add a parameter `in_alias_position: bool` defaulting to `false`. When `true`, suppress the `ReservedKeywordAsIdentifier` error emission.

**Specific change in `parse_ident`** (L920-946):
```rust
// Before:
if kw.category() == KeywordCategory::Reserved {
    self.add_error(ParserError::ReservedKeywordAsIdentifier { ... });
}

// After:
if kw.category() == KeywordCategory::Reserved && !self.in_as_alias_position {
    self.add_error(ParserError::ReservedKeywordAsIdentifier { ... });
}
```

Alternative simpler approach: in `parse_optional_alias`, after `match_keyword(AS)` and `advance()`, if next token is a reserved keyword, consume it as an ident WITHOUT calling `parse_ident()` (which triggers the error). Instead, directly construct the Ident.

**Regression tests** (`src/parser/tests.rs`):
```rust
#[test]
fn test_reserved_keyword_as_alias_allowed() {
    // openGauss allows reserved keywords after AS
    assert_valid("SELECT 1 AS current_user");
    assert_valid("SELECT 1 AS session_user");
    assert_valid("SELECT 1 AS cast");
    assert_valid("SELECT 1 AS user");
}

#[test]
fn test_reserved_keyword_as_implicit_alias_still_rejected() {
    // Without AS keyword, reserved words should still be rejected as identifiers
    // (this is a column-name test, not alias-specific)
    let (_stmts, errors) = parse_with_errors("SELECT current_user x FROM t");
    // May or may not error depending on context — just verify it doesn't crash
}
```

---

## Step 3 — P1-C: MERGE TABLESAMPLE

### 3a. Consume TABLESAMPLE on USING source

**File**: `src/parser/dml.rs:581-588`

**Current flow**:
```rust
let mut source = self.parse_table_ref()?;       // L581
let source_partition = self.parse_dml_partition()?; // L582 — only PARTITION
// MISSING: TABLESAMPLE consumption
self.expect_keyword(Keyword::ON)?;               // L588 — fails if TABLESAMPLE unconsumed
```

**Fix**: After L582, add `try_consume_table_modifiers` call (same function used in `parse_from_clause` in `select.rs`).

```rust
let mut source = self.parse_table_ref()?;
let source_partition = self.parse_dml_partition()?;
self.try_consume_table_modifiers(&mut source);   // NEW: consume TABLESAMPLE etc.
self.expect_keyword(Keyword::ON)?;
```

Wait — `try_consume_table_modifiers` takes `&mut TableRef`. Need to verify it handles TABLESAMPLE correctly. Check `src/parser/select.rs` for its definition and ensure it's `pub(crate)`.

If `try_consume_table_modifiers` is only in `select.rs` and not `pub(crate)`, either:
- Make it `pub(crate)` and import it, or
- Inline the TABLESAMPLE consumption in `parse_merge`.

**Regression test** (`src/parser/tests.rs`):
```rust
#[test]
fn test_merge_using_tablesample() {
    let sql = "MERGE INTO t USING s TABLESAMPLE SYSTEM(5.6) ON t.id = s.id WHEN MATCHED THEN UPDATE SET a = 1";
    assert_valid(sql); // use existing assert_valid helper (tests.rs:2974)
}
```

---

## Step 4 — P1-D: `parse_set_operations` chain overwrite

### 4a. Append set_operation to chain tail, not head

**File**: `src/parser/select.rs:45-79`

**Bug**: L76 `stmt.set_operation = Some(set_op)` always assigns to original stmt head. For chained `A UNION B UNION C`, iteration 2 overwrites B.

**Fix**: Walk to the rightmost SelectStatement (tail) and set set_operation there.

```rust
pub(crate) fn parse_set_operations(&mut self, mut stmt: SelectStatement) -> Result<SelectStatement, ParserError> {
    // Find the tail — the rightmost SelectStatement with no set_operation
    let tail = {
        let mut cur = &mut stmt;
        while let Some(ref mut op) = cur.set_operation {
            cur = op.right_mut();
        }
        cur
    };
    
    loop {
        let (op, all) = match self.peek_keyword() {
            Some(Keyword::UNION) => { self.advance(); ("union", self.match_keyword(Keyword::ALL)) }
            Some(Keyword::INTERSECT) => { self.advance(); ("intersect", self.match_keyword(Keyword::ALL)) }
            Some(Keyword::EXCEPT) | Some(Keyword::MINUS_P) => { self.advance(); ("except", self.match_keyword(Keyword::ALL)) }
            _ => break,
        };
        let right = self.parse_simple_select()?;
        let set_op = match op {
            "union" => SetOperation::Union { all, right: Box::new(right) },
            "intersect" => SetOperation::Intersect { all, right: Box::new(right) },
            _ => SetOperation::Except { all, right: Box::new(right) },
        };
        // Set on tail, then advance tail to the newly set right
        tail.set_operation = Some(set_op);
        // Advance tail pointer to the rightmost node
        let mut cur = &mut stmt;
        while let Some(ref mut op) = cur.set_operation {
            let right = op.right_mut();
            if right.set_operation.is_none() {
                tail = right;  // Hmm, borrow checker issue with re-assigning tail
                break;
            }
            cur = right;
        }
    }
    Ok(stmt)
}
```

This has borrow-checker challenges. A cleaner approach: collect all set operations first, then build the chain from the rightmost outward.

**Alternative clean fix**: build the chain by advancing `stmt`:
```rust
pub(crate) fn parse_set_operations(&mut self, mut root: SelectStatement) -> Result<SelectStatement, ParserError> {
    // stmt always points to the rightmost leaf (the one whose set_operation is None)
    let mut stmt_ptr: *mut SelectStatement = &mut root;
    // Walk to rightmost
    unsafe {
        while let Some(ref mut op) = (*stmt_ptr).set_operation {
            stmt_ptr = op.right_mut() as *mut SelectStatement;
        }
    }
    
    loop {
        let (op, all) = match self.peek_keyword() { ... };
        let right = self.parse_simple_select()?;
        let set_op = SetOperation::Union { all, right: Box::new(right) };
        
        unsafe {
            (*stmt_ptr).set_operation = Some(set_op);
            // Advance stmt_ptr to the new right
            if let Some(ref mut op) = (*stmt_ptr).set_operation {
                stmt_ptr = op.right_mut() as *mut SelectStatement;
            }
        }
    }
    Ok(root)
}
```

Raw pointers avoid the borrow checker issue. The `stmt_ptr` always points to the current rightmost leaf. Each new set_op is set there, then the pointer advances.

**Regression tests** (`src/parser/tests.rs`):
```rust
#[test]
fn test_triple_union_flat() {
    let sql = "SELECT a FROM t1 UNION ALL SELECT b FROM t2 UNION ALL SELECT c FROM t3";
    let (stmts, errors) = parse_with_errors(sql);
    assert!(!stmts.is_empty());
    assert!(errors.is_empty());
    // Verify 3 branches in AST
    if let Statement::Select(s) = &stmts[0].statement {
        let mut count = 1;
        let mut cur = &s.node;
        while let Some(op) = &cur.set_operation {
            count += 1;
            cur = op.right();
        }
        assert_eq!(count, 3, "triple union should have 3 branches");
    }
}

#[test]
fn test_nested_paren_union_all_branches_preserved() {
    let sql = "(((SELECT 1 UNION ALL SELECT 2) UNION ALL SELECT 3) UNION ALL SELECT 4)";
    let (stmts, errors) = parse_with_errors(sql);
    assert!(!stmts.is_empty());
    let mut count = 1;
    let mut cur = /* extract SelectStatement */;
    while let Some(op) = &cur.set_operation {
        count += 1;
        cur = op.right();
    }
    assert_eq!(count, 4, "nested paren union should preserve all 4 branches");
}
```

---

## Step 5 — P1-A: AT TIME ZONE expression operator

### 5a. Add AST node

**File**: `src/ast/mod.rs` — add variant to `Expr` enum (around L1384, before `SpecialFunction`):
```rust
/// AT TIME ZONE expression: `expr AT TIME ZONE zone`
AtTimeZone {
    expr: Box<Expr>,
    zone: Box<Expr>,
},
```

### 5b. Add parser handling

**File**: `src/parser/expr.rs` — add to `try_postfix_op()` (starts L272).

After existing postfix handlers, add:
```rust
// AT TIME ZONE / AT LOCAL
if self.match_keyword(Keyword::AT) {
    let at_pos = self.pos;
    self.advance();
    if self.match_keyword(Keyword::TIME) {
        self.advance();
        self.expect_keyword(Keyword::ZONE)?;
        let zone = self.parse_expr()?;
        return Ok(Expr::AtTimeZone {
            expr: Box::new(expr),
            zone: Box::new(zone),
        });
    } else if self.match_keyword(Keyword::LOCAL) {
        self.advance();
        // AT LOCAL is equivalent to AT TIME ZONE 'localtime'
        return Ok(Expr::AtTimeZone {
            expr: Box::new(expr),
            zone: Box::new(Expr::Literal(Literal::String("localtime".to_string()))),
        });
    } else {
        self.pos = at_pos; // rewind — not AT TIME ZONE/LOCAL
    }
}
```

Need to verify `Keyword::LOCAL` and `Keyword::ZONE` exist in keyword.rs. `ZONE` already exists (used for type modifiers). `LOCAL` should exist.

### 5c. Add formatter

**File**: `src/formatter/mod.rs` — in `format_expr`, add:
```rust
Expr::AtTimeZone { expr, zone } => {
    self.format_expr(expr)?;
    write!(f, " AT TIME ZONE ")?;
    self.format_expr(zone)
}
```

### 5d. Add visitor

**File**: `src/ast/visitor.rs` — add to `walk_expr`:
```rust
Expr::AtTimeZone { expr, zone } => {
    self.visit_expr(expr)?;
    self.visit_expr(zone)?;
}
```

### 5e. Regression tests (`src/parser/tests.rs`)
```rust
#[test]
fn test_at_time_zone() {
    assert_valid("SELECT x AT TIME ZONE 'UTC' FROM t");
    assert_valid("SELECT now() AT TIME ZONE 'Asia/Shanghai'");
    assert_valid("SELECT TIMESTAMP '2024-03-20 01:30:00' AT TIME ZONE 'Europe/Moscow'");
}

#[test]
fn test_at_local() {
    assert_valid("SELECT x AT LOCAL FROM t");
}
```

**Round-trip test**: SQL → parse → format → re-parse → assert AST equivalence.

---

## Step 6 — P1-B: `?` → JdbcParam tokenizer fix

### 6a. Add context-aware `?` handling

**File**: `src/token/tokenizer.rs:741`

**Current**:
```rust
'?' => {
    self.advance();
    Token::JdbcParam
}
```

**Fix**: Look ahead for JSONB/geometric operator suffixes (`|`, `&`, `-`):
```rust
'?' => {
    self.advance();
    // Check if this is a JSONB or geometric operator: ?|, ?&, ?-, ?-|, ?||
    if let Some(next) = self.peek_char() {
        if matches!(next, '|' | '&' | '-') {
            // Scan the multi-char operator
            let start = self.pos - 1;
            self.advance(); // consume the suffix char (|, &, or -)
            // Check for longer variants: ?-|, ?||
            if next == '|' || next == '-' {
                if let Some(c) = self.peek_char() {
                    if (next == '|' && c == '|') || (next == '-' && c == '|') {
                        self.advance();
                    }
                }
            }
            let op = &self.source[start..self.pos];
            return Token::Op(op.to_string());
        }
        // Bare ? is ambiguous — JDBC placeholder by default
    }
    Token::JdbcParam
}
```

Wait — there's a complication. Current users of the parser may rely on `?` being tokenized as JdbcParam for prepared-statement SQL. Changing bare `?` to `Token::Op("?")` would break them.

**Better approach**: Keep bare `?` as `JdbcParam` (backward compat). Only change when followed by `|`, `&`, `-` to produce a `Token::Op(...)`. This handles JSONB `?|`/`?&` and geometric `?-`/`?-|`/`?||` without breaking JDBC usage.

Bare `?` as JSONB key-exists operator remains unsolved — but this is rarely used in practice compared to `?|`/`?&`.

### 6b. Verify dead code at expr.rs:211 becomes live

The geometric unary operator list at `expr.rs:211` (`"?|"`, `"?-"`, `"?-|"`, `"?||"`) will now actually match since the tokenizer will produce `Token::Op("?|")` etc.

### 6c. Regression tests (`src/token/tokenizer.rs` and `src/parser/tests.rs`)
```rust
// In tokenizer tests:
#[test]
fn test_tokenize_jsonb_operator_qpipe() {
    let tokens = tokenize("'{}'::jsonb ?| array['a']");
    assert_has_token(&tokens, Token::Op("?|".into()));
    assert_no_token(&tokens, Token::JdbcParam);
}

#[test]
fn test_tokenize_jsonb_operator_qamp() {
    let tokens = tokenize("'{}'::jsonb ?& array['a']");
    assert_has_token(&tokens, Token::Op("?&".into()));
}

// JDBC ? still works:
#[test]
fn test_tokenize_jdbc_param_still_works() {
    let tokens = tokenize("SELECT * FROM t WHERE id = ?");
    assert_has_token(&tokens, Token::JdbcParam);
}
```

---

## Step 7 — P0-A: Compound operator tokenization

### 7a. Recognize compound PG operators as single tokens

**File**: `src/token/tokenizer.rs`

**Problem**: Operators like `<%`, `*<>`, `!~~*`, `~>~`, `%>>` get split into fragments. The `is_op_char` function already includes all these characters but the tokenizer's operator-scanning logic doesn't greedily consume them all.

**Fix**: In the operator-scanning section of the tokenizer (where `#`, `~`, `!`, `@`, `^`, `|`, `&`, `*`, `+`, `-`, `/`, `%`, `<`, `>` are scanned), ensure that once an operator character sequence starts, ALL consecutive `is_op_char` characters are consumed into a single `Token::Op(...)`.

Specifically: `<` currently triggers consumption until `>` or end. But `<%` should consume both `<` and `%` as one token `Token::Op("<%")`. The fix is to ensure the operator scanner greedily consumes ALL op_chars, not stopping at specific boundaries.

Need to check the current operator-scanning loop logic. The current behavior likely uses a character-specific branch (e.g., `<` → look for `=` or `>`; `*` → look for `*` for `**`; `%` → standalone `Percent` token). The fix is to replace these hardcoded patterns with a greedy `is_op_char` loop.

### 7b. Strategy: Fix the `<` / `>` / `!` scanning

The specific problem operators and their current tokenization:
| Input | Current | Should be |
|---|---|---|
| `<%` | `Lt` + `Percent` | `Op("<%")` |
| `*<>` | `Star` + `OpNe` | `Op("*<>")` |
| `!~~*` | `Op("!")` + `Op("~~*")` or worse | `Op("!~~*")` |
| `%>>` | `Percent` + `OpShiftR` | `Op("%>>")` |
| `~>~` | `Op("~")` + `Op(">~")` | `Op("~>~")` |

The root cause: after the first operator char is consumed, the scanner checks for specific known sequences (like `<=`, `>=`, `<>`, `<<`, `>>`) but doesn't fall back to greedy `is_op_char` consumption.

**Fix**: After the hardcoded sequence checks, continue consuming any `is_op_char` chars:
```rust
// Example for '<' branch:
'<' => {
    self.advance();
    let op = match self.peek_char() {
        Some('=') => { self.advance(); "<=".to_string() }
        Some('>') => { self.advance(); "<>".to_string() }
        _ => {
            // Greedy: consume all following op_chars
            let start = self.pos - 1;
            while let Some(c) = self.peek_char() {
                if is_op_char(c) && c != ';' {
                    self.advance();
                } else {
                    break;
                }
            }
            self.source[start..self.pos].to_string()
        }
    };
    Token::Op(op)
}
```

Apply the same pattern to `>`, `!`, `%`, `*`, `~` branches.

### 7c. Regression tests
```rust
#[test]
fn test_compound_op_lt_percent() {
    assert_valid("SELECT 1 <% 2");
}

#[test]
fn test_compound_op_star_ne() {
    assert_valid("SELECT 1 *<> 2");
}
```

---

## Step 8 — Final Verification

### 8a. CI checks
```bash
cargo fmt --all
cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings
cargo test --all-features
```

### 8b. SQLsmith guard
```bash
cd tests/sqlsmith
make guard
```
- `regressions.csv` must be empty
- `improvements.csv` should list all fixed cases

### 8c. Update regress metadata
For each fixed case, update `meta.json`: `expected_outcome: "OK"`, `fixed_in_commit: "<commit-hash>"`.

---

## Execution Order

| Step | Category | Risk | Dependencies |
|---|---|---|---|
| 1 | P2 Function arity | Low | None |
| 2 | P2 Reserved alias | Low | None |
| 3 | P1-C MERGE TABLESAMPLE | Low | None |
| 4 | P1-D Set-op chain | Medium | None |
| 5 | P1-A AT TIME ZONE | Medium | None |
| 6 | P1-B `?` tokenizer | High (backward compat) | None |
| 7 | P0-A Compound ops | High (tokenizer-wide) | None |
| 8 | Verification | — | All above |

Steps 1-3 can be done in parallel (different files). Steps 4-7 are sequential (single-file changes with complex interactions).
