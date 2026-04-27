# Fix PL/SQL Parser Errors Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix 588 parse errors in error-latest.log by adding support for missing Oracle PL/SQL syntax patterns in the ogsql-parser.

**Architecture:** The parser is a hand-written recursive descent parser in Rust. Errors come from missing syntax support in PL/pgSQL parsing (cursor declarations, TYPE declarations, procedure body parsing, expression handling). Each fix targets a specific gap in the grammar.

**Tech Stack:** Rust, recursive descent parser, Pratt expression parser

---

## Error Categories (588 total errors across 247 files)

| # | Error Pattern | Count | Root Cause | Fix Location |
|---|--------------|-------|-----------|-------------|
| 1 | `expected end of statement, got Keyword(IF_P)` | 102 | Package body items parser skips non-PROCEDURE/FUNCTION tokens, losing IF etc. in inner procedures | `plpgsql.rs:parse_package_body_items` |
| 2 | `expected end of DML statement, got LParen` | 97 | Function calls as statements (e.g. `pack_log.log(...)`) not recognized | `plpgsql.rs:parse_pl_sql_or_assignment` |
| 3 | `expected for, got LParen` | 74 | Cursor declaration with parameters `(v_step_code IN VARCHAR2)` fails | `plpgsql.rs:parse_pl_cursor_decl` / procedure body parsing |
| 4 | `expected identifier, got Semicolon` | 68 | `TYPE ... IS REF CURSOR;` not supported | `plpgsql.rs:parse_pl_type_decl_body` |
| 5 | `expected RParen, got Dot` | 33 | Qualified names in expressions like `n.a - n.b` | Expression parsing context |
| 6 | `expected into, got Ident("BULK")` | 23 | `SELECT ... BULK COLLECT INTO ...` not fully supported in PL context | `select.rs` / `plpgsql.rs` |
| 7 | `expected end of statement, got Keyword(END_P)` | 14 | Mismatched BEGIN/END nesting in package bodies | `plpgsql.rs:parse_procedure_body` |
| 8 | `expected end of statement, got Keyword(BEGIN_P)` | 12 | Nested anonymous blocks in procedures | `plpgsql.rs:parse_pl_statement` |
| 9 | `expected RParen, got Ident("，")` | 10 | Fullwidth Chinese comma `，` (U+FF0C) not handled as comma | `tokenizer.rs` |
| 10 | `expected end of statement, got Keyword(PROCEDURE)` | 9 | Nested procedure declarations inside package body procedures | `plpgsql.rs:parse_procedure_body` |
| 11 | `expected end of statement, got Ident("EXCEPTION")` | 9 | EXCEPTION handling block in wrong context | `plpgsql.rs:parse_pl_block_body_inner` |
| 12 | `expected RParen, got Keyword(MINUS_P)` | 8 | Negative numbers in function call args like `to_date(...) - 1` | Expression parsing |
| 13 | `expected end of statement, got Keyword(COMMIT)` | 6 | COMMIT used as standalone statement in PL block | `plpgsql.rs:parse_pl_statement` |
| 14 | `expected end of statement, got Keyword(LOOP)` | 5 | LOOP keyword not recognized in certain PL contexts | `plpgsql.rs:parse_pl_statement` |
| 15 | `expected and, got OpConcat` | 4 | `||` after BETWEEN range in expressions | `expr.rs` |
| 16 | `expected identifier, got ColonEquals` | 3 | Qualified assignment target `var.field :=` | `plpgsql.rs:parse_pl_sql_or_assignment` |
| 17 | `expected Eq, got Dot` | 3 | Named parameter in function call `name => expr` with dots | `expr.rs` |
| 18 | Unterminated strings/comments/identifiers | 7 | Tokenizer edge cases with encoding | `tokenizer.rs` |
| 19 | Various other | ~122 | Cascading effects from above root causes | Multiple |

---

## Task 1: Support `TYPE ... IS REF CURSOR` declaration

**Priority:** HIGH — Fixes 68 errors (largest single grammar gap)

**Files:**
- Modify: `src/parser/plpgsql.rs:413-478` (`parse_pl_type_decl_body`)
- Modify: `src/ast/plpgsql.rs` (add `RefCursor` variant to `PlTypeDecl`)

**Step 1: Write failing test**

Add to `src/parser/tests.rs`:
```rust
#[test]
fn test_type_ref_cursor_decl() {
    let sql = "CREATE OR REPLACE PROCEDURE test_proc IS
        TYPE t_refcur IS REF CURSOR;
        v_cur t_refcur;
    BEGIN
        NULL;
    END;";
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_type_ref_cursor_decl`
Expected: FAIL with "expected RECORD, TABLE, or VARRAY after IS/AS"

**Step 3: Implement REF CURSOR support**

In `src/ast/plpgsql.rs`, add `RefCursor` variant to `PlTypeDecl`:
```rust
pub enum PlTypeDecl {
    Record { name: String, fields: Vec<PlTypeField> },
    TableOf { name: String, elem_type: PlDataType, index_by: Option<PlDataType> },
    VarrayOf { name: String, size: Box<Expr>, elem_type: PlDataType },
    RefCursor { name: String },  // NEW
}
```

In `src/parser/plpgsql.rs:parse_pl_type_decl_body`, after the `varray` branch, add before the else:
```rust
} else if self.match_ident_str("ref") {
    self.advance();
    self.expect_ident_str("cursor")?;
    self.try_consume_semicolon();
    Ok(PlDeclaration::Type(PlTypeDecl::RefCursor { name }))
} else {
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_type_ref_cursor_decl`
Expected: PASS

**Step 5: Commit**

```bash
git add -A && git commit -m "feat: support TYPE ... IS REF CURSOR declaration"
```

---

## Task 2: Support qualified assignment targets (`var.field := expr`)

**Priority:** HIGH — Fixes 3 direct errors + cascading

**Files:**
- Modify: `src/parser/plpgsql.rs:947-989` (`parse_pl_sql_or_assignment`)

**Step 1: Write failing test**

```rust
#[test]
fn test_qualified_assignment() {
    let sql = "CREATE OR REPLACE PROCEDURE test IS
    BEGIN
        p_o_msg := 'error' || SQLERRM;
    END;";
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_qualified_assignment`

**Step 3: Fix the assignment target parsing**

The issue is in `parse_pl_sql_or_assignment` at line 949-955. When it sees `name.field`, it reads the dotted name parts. But then at line 956, it checks for `:=`. The problem is likely that the identifier parsing stops at certain tokens.

Looking at line 949:
```rust
if matches!(self.peek(), Token::Ident(_) | Token::QuotedIdent(_)) {
    let first = self.parse_identifier().unwrap_or_default();
    let mut name_parts = vec![first];
    while self.match_token(&Token::Dot) {
        self.advance();
        name_parts.push(self.parse_identifier().unwrap_or_default());
    }
```

The issue: `parse_identifier()` may fail when the next token after `.` is a keyword like `STEP_NO` that's recognized as a keyword. Need to use `parse_identifier_or_keyword()` instead for subsequent parts.

Actually, the real issue for `expected identifier, got ColonEquals` is that the name `V_STEP_NO` is a keyword. Check: no, it's an Ident. The actual error pattern is:
```
Error: unexpected token at line 5, column 41: expected identifier, got ColonEquals
```
This means after consuming name parts and hitting `:=`, the code goes to `parse_pl_sql_or_assignment`, reads the identifier, then sees `:=`... but wait, looking at line 956:
```rust
if self.match_token(&Token::ColonEquals) {
```
This should work. Let me re-read... The error says `expected identifier, got ColonEquals` at column 41. This might happen in a context where `:=` appears after a qualified name but the parser is in `parse_pl_declaration` not `parse_pl_sql_or_assignment`.

The root issue: in `parse_procedure_body`, declarations are parsed before BEGIN. If a line like `v_step_no VARCHAR2(100) := '1';` appears, `parse_pl_data_type` might fail to parse `VARCHAR2` correctly, causing the parser to not reach the `:=` handling.

Fix: ensure `parse_pl_data_type` can handle qualified type names and that `try_parse_oracle_var_decl` properly handles `:=` defaults.

**Step 4: Run test to verify**

Run: `cargo test test_qualified_assignment`

**Step 5: Commit**

```bash
git add -A && git commit -m "fix: support qualified assignment targets in PL/SQL blocks"
```

---

## Task 3: Support `SELECT ... BULK COLLECT INTO` in PL context

**Priority:** HIGH — Fixes 23 errors

**Files:**
- Modify: `src/parser/select.rs` (BULK COLLECT INTO handling)
- Modify: `src/parser/plpgsql.rs` (PL INTO mode awareness)

**Step 1: Write failing test**

```rust
#[test]
fn test_bulk_collect_into_in_procedure() {
    let sql = "CREATE OR REPLACE PROCEDURE test IS
        v_data SYS_REFCURSOR;
    BEGIN
        SELECT id BULK COLLECT INTO v_data FROM my_table;
    END;";
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_bulk_collect_into_in_procedure`

**Step 3: Implement BULK COLLECT INTO support**

Check how `pl_into_mode` is used in `select.rs`. The `BULK COLLECT INTO` syntax needs to be recognized as a variant of `INTO` in PL mode. Find where `INTO` is parsed in SELECT and add handling for `BULK COLLECT INTO`.

**Step 4: Run test to verify**

Run: `cargo test test_bulk_collect_into_in_procedure`

**Step 5: Commit**

```bash
git add -A && git commit -m "feat: support SELECT ... BULK COLLECT INTO in PL/SQL context"
```

---

## Task 4: Support function/procedure calls as PL statements

**Priority:** HIGH — Fixes 97 errors (2nd largest category)

**Files:**
- Modify: `src/parser/plpgsql.rs:947-989` (`parse_pl_sql_or_assignment`)
- Modify: `src/parser/plpgsql.rs:898-946` (`try_parse_pl_procedure_call`)

**Step 1: Write failing test**

```rust
#[test]
fn test_procedure_call_as_statement() {
    let sql = "CREATE OR REPLACE PROCEDURE test IS
    BEGIN
        pack_log.log('proc_name', 'description', '1');
        COMMIT;
    END;";
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_procedure_call_as_statement`

**Step 3: Fix procedure call parsing**

The issue: `try_parse_pl_procedure_call` is called from `parse_pl_sql_or_assignment`. It reads `pack_log.log(...)` but the outer `parse_pl_sql_or_assignment` first tries to parse an identifier, reads `pack_log`, then sees `.`, reads `log`, then sees `(` but doesn't recognize `:=`. It then restores position and falls through to `try_parse_pl_procedure_call`.

In `try_parse_pl_procedure_call`, it reads `pack_log` as object name, then sees `.` and tries to continue... The problem might be that `parse_object_name()` reads `pack_log.log` as a multi-part name, then expects `(`. But after reading the dotted name in the first branch of `parse_pl_sql_or_assignment`, the position was restored to `save`, so `try_parse_pl_procedure_call` should start fresh.

Actually, the root cause might be that `pack_log` is treated as a known variable (it's declared in scope via package spec) and the parser tries to parse it differently. Or the issue is that `expected end of DML statement, got LParen` means the parser is in DML parsing mode and sees `pack_log.log(` as the end of a DML statement followed by `(`.

The real fix: in `parse_pl_statement`, when none of the known PL statement keywords match, and DML parsing fails, the fallback `parse_pl_sql_or_assignment` should properly handle qualified function calls. The `try_parse_pl_procedure_call` at line 977 should be able to handle `pack_log.log(...)`.

Need to trace the exact failure path and fix it.

**Step 4: Run test to verify**

Run: `cargo test test_procedure_call_as_statement`

**Step 5: Commit**

```bash
git add -A && git commit -m "fix: support qualified function/procedure calls as PL statements"
```

---

## Task 5: Support cursor declarations with parameters in procedure bodies

**Priority:** HIGH — Fixes 74 errors

**Files:**
- Modify: `src/parser/plpgsql.rs:2265-2302` (`parse_procedure_body` declaration parsing)

**Step 1: Write failing test**

```rust
#[test]
fn test_cursor_with_params_in_procedure() {
    let sql = "CREATE OR REPLACE FUNCTION fnc_test(p_i_contract_id VARCHAR2) RETURN VARCHAR2 IS
        CURSOR c_dept_info(v_step_code IN VARCHAR2) IS
            SELECT t.dept FROM dat_contract_flow t
            WHERE t.contract_id = p_i_contract_id;
    BEGIN
        RETURN '';
    END;";
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_cursor_with_params_in_procedure`

**Step 3: Fix cursor declaration parsing**

The issue: In `parse_procedure_body` (line 2292-2296):
```rust
} else if self.match_ident_str("cursor") {
    self.advance();
    let cursor_name = self.parse_identifier()?;
    let decl = self.parse_pl_cursor_decl(cursor_name)?;
```
This handles `CURSOR name(params) IS SELECT...`. But `parse_pl_cursor_decl` at line 322 handles `match_token(&Token::LParen)` to parse parameters. The error `expected for, got LParen` means the `(` is not being matched.

The root cause might be that `CURSOR` is a keyword (`Keyword::CURSOR`) not an Ident, so `match_ident_str("cursor")` at line 2292 might not match. Need to check if CURSOR is a reserved keyword.

If CURSOR is a keyword token, `match_ident_str` won't match it. The fix is to also check for `Token::Keyword(Keyword::CURSOR)`:
```rust
} else if self.match_ident_str("cursor") || self.match_keyword(Keyword::CURSOR) {
    self.advance();
    ...
```

And in the `try_parse_oracle_var_decl` function (line 2420):
```rust
if self.match_ident_str("cursor") {
```
Same fix needed there.

**Step 4: Run test to verify**

Run: `cargo test test_cursor_with_params_in_procedure`

**Step 5: Commit**

```bash
git add -A && git commit -m "fix: support cursor declarations with parameters in procedure bodies"
```

---

## Task 6: Handle fullwidth Chinese comma `，` in tokenizer

**Priority:** MEDIUM — Fixes 10 errors directly

**Files:**
- Modify: `src/token/tokenizer.rs`

**Step 1: Write failing test**

```rust
#[test]
fn test_fullwidth_comma_in_values() {
    let sql = "INSERT INTO t (a, b) VALUES ('x'， 0);";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    // Should tokenize without error
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_fullwidth_comma_in_values`

**Step 3: Add fullwidth comma support**

In `tokenizer.rs`, where single-character tokens are matched, add:
```rust
'，' => { /* U+FF0C fullwidth comma */ return Some(Token::Comma); }
```

**Step 4: Run test**

Run: `cargo test test_fullwidth_comma_in_values`

**Step 5: Commit**

```bash
git add -A && git commit -m "fix: handle fullwidth Chinese comma in tokenizer"
```

---

## Task 7: Support COMMIT as standalone PL statement

**Priority:** MEDIUM — Fixes 6 errors

**Files:**
- Modify: `src/parser/plpgsql.rs:550-670` (`parse_pl_statement`)

**Step 1: Write failing test**

```rust
#[test]
fn test_commit_in_procedure() {
    let sql = "CREATE OR REPLACE PROCEDURE test IS
    BEGIN
        DELETE FROM t WHERE id = 1;
        COMMIT;
        INSERT INTO t (id) VALUES (1);
    END;";
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1);
}
```

**Step 2: Verify test status**

COMMIT is already handled at line 588-592. But the error `expected end of statement, got Keyword(COMMIT)` means COMMIT appears where the parser doesn't expect a PL statement. This is likely inside `parse_package_body_items` where COMMIT appears inside a nested procedure but the package body item parser at the outer level encounters it.

Root cause: `parse_package_body_items` (line 676) uses a depth-tracking approach for BEGIN/END, but doesn't recognize COMMIT as a valid inner token. When a nested procedure's inner COMMIT is encountered, the package body parser might not be at the right depth level.

**Step 3: Fix package body parsing depth tracking**

The issue is in `parse_package_body_items` — it tracks BEGIN/END depth but doesn't properly enter nested procedures. When PROCEDURE is seen, it delegates to `parse_package_sub_procedure`, but if that fails, the error recovery skips tokens without updating depth properly.

Fix: Improve error recovery in `parse_package_body_items` to properly handle BEGIN/END depth during skip.

**Step 4: Commit**

```bash
git add -A && git commit -m "fix: handle COMMIT and other PL statements in package body context"
```

---

## Task 8: Support `EXCEPTION` keyword as block terminator in more contexts

**Priority:** MEDIUM — Fixes 9 errors

**Files:**
- Modify: `src/parser/plpgsql.rs:53-90` (`parse_pl_block_body_inner`)

**Step 1: Understand the issue**

Error: `expected end of statement, got Ident("EXCEPTION")`. This means `exception` is not recognized as a PL statement keyword inside certain parsing contexts. The `parse_pl_statement` at line 550 doesn't have an `exception` case — it's only handled in `parse_pl_block_body_inner` at line 69.

The issue: when `parse_pl_statements_until` is used (e.g. inside IF/LOOP bodies), it calls `parse_pl_statement` which doesn't know about EXCEPTION. If an EXCEPTION block appears inside an IF body, the parser fails.

Actually, EXCEPTION should only appear at the block level (not inside IF/LOOP). The error occurs because the block body parser doesn't properly handle EXCEPTION before trying `parse_pl_statement`. Need to check `parse_pl_statements_until` — it checks for terminators but not for "exception".

**Step 2: Fix**

Add `exception` handling to `parse_pl_statements_until` — when it sees `exception`, break out and let the caller handle it. Or better: in `parse_pl_block_body_inner`, ensure EXCEPTION is checked before `parse_pl_statement`.

**Step 3: Commit**

```bash
git add -A && git commit -m "fix: handle EXCEPTION block in nested PL contexts"
```

---

## Task 9: Support negative number expressions in function arguments

**Priority:** MEDIUM — Fixes 8 errors

**Files:**
- Modify: `src/parser/expr.rs`

**Step 1: Write failing test**

```rust
#[test]
fn test_negative_in_function_arg() {
    let sql = "SELECT to_char(to_date('20240101', 'YYYYMMDD') - 1, 'YYYYMMDD') FROM sys_dummy";
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1);
}
```

**Step 2: Verify and fix**

The error `expected RParen, got Keyword(MINUS_P)` suggests the parser sees the closing `)` of `to_date()` but then encounters `- 1` and fails. This might be because `-` is tokenized as `Keyword(MINUS_P)` instead of `Token::Minus`, and the expression parser doesn't handle `Keyword(MINUS_P)` as a binary minus operator.

Fix: In the infix operator handling (`infix_binding_power`), add `Keyword(MINUS_P)` as equivalent to `Token::Minus`:
```rust
Token::Keyword(Keyword::MINUS_P) => Some((40, "-".to_string(), false)),
```

**Step 3: Commit**

```bash
git add -A && git commit -m "fix: handle MINUS_P keyword as binary minus operator in expressions"
```

---

## Task 10: Support nested PROCEDURE/FUNCTION declarations inside procedures

**Priority:** MEDIUM — Fixes 9 errors

**Files:**
- Modify: `src/parser/plpgsql.rs:2265-2302` (`parse_procedure_body`)

**Step 1: Understand the issue**

Error: `expected end of statement, got Keyword(PROCEDURE)`. This happens when a nested `PROCEDURE` or `FUNCTION` declaration appears inside a procedure body (after BEGIN). Oracle allows this in certain contexts.

Actually, looking at the procedure body parser, PROCEDURE/FUNCTION declarations are handled before BEGIN (line 2274-2287). The error might occur when the parser is in `parse_pl_statement` and encounters `PROCEDURE` — this keyword is not in the statement dispatch list at line 550.

Fix: Add PROCEDURE/FUNCTION handling to `parse_pl_statement`:
```rust
} else if self.match_ident_str("procedure") {
    // Handle as nested procedure declaration or forward declaration
} else if self.match_ident_str("function") {
    // Handle as nested function declaration or forward declaration
```

**Step 2: Commit**

```bash
git add -A && git commit -m "feat: support nested procedure/function declarations in PL blocks"
```

---

## Task 11: Fix expression parser for BETWEEN...AND with concat operator

**Priority:** LOW — Fixes 4 errors (likely cascading from other issues)

**Files:**
- Modify: `src/parser/expr.rs:327-338`

**Step 1: Analyze the issue**

The error `expected and, got OpConcat` means the parser parsed `BETWEEN <low>` and then expected `AND` but got `||`. This happens because `parse_expr_with_precedence(40)` for the `low` expression consumes too much — it might be consuming past the `AND` keyword.

Actually, the real issue is the context: these errors appear in deeply nested package body code where error recovery has already desynchronized the parser. The BETWEEN...AND handling itself is correct, but the surrounding context is wrong due to other failures.

**Step 2: Fix**

The fix is likely indirect — fixing the higher-priority issues (package body parsing, procedure call handling) should eliminate most of these errors. If any remain after other fixes, investigate the specific SQL pattern.

For now: ensure `parse_expr_with_precedence(40)` properly stops at `AND` keyword. Check that `AND` is not consumed by the expression parser at precedence 40.

The binding power of AND is `(10, 10)` and min_prec is 40, so AND should NOT be consumed. This confirms these errors are cascading from other root causes.

**Step 3: Commit (if direct fix needed)**

```bash
git add -A && git commit -m "fix: BETWEEN...AND expression handling in PL context"
```

---

## Task 12: Fix package body item parsing to properly handle nested blocks

**Priority:** CRITICAL — This is the ROOT CAUSE of the 102 IF_P errors

**Files:**
- Modify: `src/parser/utility/functions.rs:676-738` (`parse_package_body_items`)

**Step 1: Understand the root cause**

The `parse_package_body_items` function at line 676 uses a simple BEGIN/END depth counter. It only looks for PROCEDURE/FUNCTION keywords to delegate parsing. For all other tokens, it just advances (line 732).

When a nested procedure body contains IF/LOOP/etc., the package body parser doesn't know about them. If `parse_package_sub_procedure` fails (returns Err), the error recovery at line 706-712 calls `skip_to_end_subprogram()` which might not properly skip the nested body.

The real issue: when `parse_package_sub_procedure` encounters an internal parse error (e.g., unknown TYPE, failed cursor), it returns Err. The error recovery then tries to skip to the end of the subprogram. But the skip logic might land in the wrong place, leaving the package body parser in a bad state.

**Step 2: Fix the approach**

Option A: Make `parse_package_body_items` more robust by only using it as a top-level dispatcher, and ensuring that when PROCEDURE/FUNCTION is found, the entire sub-program body is consumed (either parsed or skipped) before continuing.

Option B: Improve error recovery in `skip_to_end_subprogram` to properly handle nested BEGIN/END depth.

Recommended: Option B — improve `skip_to_end_subprogram` to track BEGIN/END depth and stop at the right END.

**Step 3: Write test**

```rust
#[test]
fn test_package_body_with_complex_procedure() {
    let sql = "CREATE OR REPLACE PACKAGE BODY test_pkg IS
        PROCEDURE inner_proc(p1 VARCHAR2) IS
            v_count NUMBER;
        BEGIN
            IF v_count > 0 THEN
                DELETE FROM t WHERE id = p1;
                COMMIT;
            END IF;
        END;
    END test_pkg;";
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1);
}
```

**Step 4: Implement fix**

Improve `skip_to_end_subprogram` (line 920) to properly track depth:
```rust
fn skip_to_end_subprogram(&mut self) -> String {
    let mut depth = 0i32;
    let start = self.pos;
    loop {
        match self.peek() {
            Token::Eof => break,
            Token::Keyword(Keyword::BEGIN_P) => {
                depth += 1;
                self.advance();
            }
            Token::Keyword(Keyword::END_P) => {
                if depth > 0 {
                    depth -= 1;
                    self.advance();
                    // consume optional label after END
                    while matches!(self.peek(), Token::Ident(_) | Token::Keyword(_)) {
                        let next = self.peek();
                        if matches!(next, Token::Semicolon) { break; }
                        // Stop if we see a statement boundary
                        self.advance();
                    }
                    self.try_consume_semicolon();
                    if depth == 0 {
                        break;
                    }
                } else {
                    self.advance();
                    break;
                }
            }
            Token::Keyword(Keyword::PROCEDURE) | Token::Keyword(Keyword::FUNCTION) => {
                // Nested sub-program — skip its IS/AS body too
                self.advance();
                // Skip to its IS/AS keyword
                while !self.match_keyword(Keyword::IS) && !self.match_keyword(Keyword::AS) 
                      && !matches!(self.peek(), Token::Semicolon | Token::Eof) {
                    self.advance();
                }
                if self.match_keyword(Keyword::IS) || self.match_keyword(Keyword::AS) {
                    self.advance();
                    depth += 1; // Expect a BEGIN inside
                }
            }
            _ => {
                self.advance();
            }
        }
    }
    self.tokens_to_raw_string(start, self.pos)
}
```

**Step 5: Run test and commit**

```bash
cargo test test_package_body_with_complex_procedure
git add -A && git commit -m "fix: improve error recovery in package body parsing"
```

---

## Task 13: Support PERFORM as PL statement

**Priority:** MEDIUM — Fixes errors like `expected end of statement, got Ident("PERFORM")`

**Files:**
- Modify: `src/parser/plpgsql.rs:550` (`parse_pl_statement`)

**Step 1: Understand the issue**

PERFORM is already handled at line 576 (`self.match_ident_str("perform")`). But the error `expected end of statement, got Ident("PERFORM")` suggests PERFORM might be tokenized as a keyword (`Keyword::PERFORM`) rather than an Ident, so `match_ident_str` doesn't match.

Check if PERFORM is a keyword. If so, add keyword matching:
```rust
} else if self.match_ident_str("perform") || self.match_keyword(Keyword::PERFORM) {
```

**Step 2: Same pattern for other keywords**

Check EXIT, RAISE, ELSIF — these may also be keywords in the openGauss keyword list. Add keyword alternatives where needed.

**Step 3: Commit**

```bash
git add -A && git commit -m "fix: handle PERFORM and other keywords in PL statement dispatch"
```

---

## Task 14: Support `ELSIF` as keyword (not just ident)

**Priority:** MEDIUM

**Files:**
- Modify: `src/parser/plpgsql.rs:parse_pl_if`

**Step 1: Check the issue**

At line 1000: `while self.match_ident_str("elsif")`. If ELSIF is a keyword, this won't match. Need to also check `self.match_keyword(Keyword::ELSIF)` if that keyword exists.

**Step 2: Commit**

```bash
git add -A && git commit -m "fix: handle ELSIF as keyword in IF statement parsing"
```

---

## Task 15: Fix unterminated tokenizer errors

**Priority:** LOW — 7 errors, likely encoding issues in source SQL files

**Files:**
- Modify: `src/token/tokenizer.rs`

**Step 1: Investigate**

The 7 unterminated errors (strings, block comments, quoted identifiers, dollar-quoted strings) are likely caused by:
1. Source files using non-standard encoding that causes mismatched delimiters
2. Source files containing actual syntax errors (broken SQL)
3. The tokenizer not handling multi-byte characters that cross chunk boundaries

**Step 2: Fix approach**

These may be actual errors in the source SQL files (not parser bugs). Before fixing, verify by manually inspecting the problematic SQL files. If the source is genuinely malformed SQL, no parser fix is needed.

If tokenizer fixes are needed, they would be in the string/comment literal parsing loops in `tokenizer.rs`.

**Step 3: Commit (if fix needed)**

```bash
git add -A && git commit -m "fix: improve tokenizer robustness for edge cases"
```

---

## Task 16: Run full regression test and measure improvement

**Priority:** VALIDATION

**Step 1: Build the project**

```bash
cargo build --release
```

**Step 2: Run existing unit tests**

```bash
cargo test
```

Expected: All existing tests pass

**Step 3: Re-run the parser against the error files**

Run the same command that generated `error-latest.log` against the same files. Compare error count: target < 100 errors (from 588).

**Step 4: Analyze remaining errors**

If >50 errors remain, categorize the new error patterns and create follow-up tasks.

**Step 5: Final commit**

```bash
git add -A && git commit -m "test: verify parser error reduction"
```

---

## Execution Order

Tasks should be executed in this order (dependencies):

1. **Task 1** (REF CURSOR) — standalone, high impact
2. **Task 5** (cursor params) — standalone, high impact
3. **Task 4** (procedure calls as statements) — high impact
4. **Task 12** (package body error recovery) — ROOT CAUSE for many cascading errors
5. **Task 2** (qualified assignment) — may overlap with Task 4
6. **Task 3** (BULK COLLECT INTO) — standalone
7. **Task 6** (fullwidth comma) — standalone, easy
8. **Task 7** (COMMIT) — may be fixed by Task 12
9. **Task 9** (MINUS_P) — standalone, easy
10. **Task 8** (EXCEPTION) — may be fixed by Task 12
11. **Task 10** (nested PROCEDURE) — may be fixed by Task 12
12. **Task 13** (PERFORM keyword) — easy
13. **Task 14** (ELSIF keyword) — easy
14. **Task 11** (BETWEEN...AND) — likely cascading, check after others
15. **Task 15** (tokenizer) — low priority
16. **Task 16** (regression) — final validation
