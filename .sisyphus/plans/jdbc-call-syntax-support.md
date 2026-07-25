# Plan: JDBC `{call}` Stored Procedure Syntax Support

## Summary

Support JDBC `{call pkg.proc(args)}` / `{? = call pkg.func(args)}` escape syntax
in `validate-xml` (iBatis XML mapper) and `validate-java` (Java source extraction)
validation pipelines.

---

## Motivation

Users write stored-procedure calls in JDBC escape syntax `{call pkg_xxx.proc_yyy(?)}`
inside MyBatis mapper XML and Java source files. Currently:

- **validate-xml**: iBatis layer captures `{call ...}` as `SqlNode::Text` and emits
  it verbatim in `flat_sql`. The core SQL parser then fails because `{` is not a
  valid SQL token.
- **validate-java**: `starts_with_sql_keyword()` only matches
  `SELECT|INSERT|UPDATE|DELETE|MERGE|WITH`. `CALL` and `{call` are not in the
  whitelist, so CALL statements are never extracted.

The SQL parser already supports bare `CALL pkg.proc(args)` statements
(`Statement::Call(CallFuncStatement)`). The missing piece is translating the JDBC
escape wrapper before it reaches the SQL parser.

---

## Approach

### Plan 1: iBatis Flatten Pipeline Translation

**File**: `src/ibatis/mod.rs`

In `parse_mapper_bytes_internal` (line ~256), before `Parser::parse_sql(&flat_sql)`:

1. Check if `stmt.statement_type == Some("CALLABLE")`
2. If so, run `translate_jdbc_call(&flat_sql)` to strip the JDBC escape wrapper
3. Feed the translated SQL to `Parser::parse_sql`
4. Keep original `flat_sql` in the output struct unchanged

New helper function `translate_jdbc_call`:
- `{call pkg.proc(args)}` → `CALL pkg.proc(args)`
- `{? = call pkg.func(args)}` → `CALL pkg.func(args)`
- Case-insensitive matching
- Handles whitespace between tokens
- Falls through unchanged if not a JDBC call pattern

### Plan 2: Java Extractor Whitelist + Translation

**Files**: `src/java/extract.rs`, `src/java/` (as needed)

1. Extend `starts_with_sql_keyword()` to match:
   - `CALL` (bare call)
   - `{CALL` (JDBC escape with leading brace)
   - `{?` (JDBC return-value syntax)

2. In the extraction pipeline, before SQL strings are parsed:
   - Apply the same `translate_jdbc_call` logic to strip JDBC escape wrappers
   - (Reuse the translate function from Plan 1, or add a shared utility)

---

## What does NOT change

- **SQL tokenizer** — stays pure; `{` remains unrecognized as a SQL token
- **SQL parser dispatch** — no new `Statement` variants; everything routes through
  existing `Statement::Call(CallFuncStatement)`
- **flat_sql output** — original `{call ...}` text preserved verbatim for
  user-facing output
- **JDBC `?` placeholder** — already handled by `Token::JdbcParam` (requires
  `mybatis_params: true`, out of scope for this plan)

---

## Test Coverage

### Already Created (TDD)

| Layer | Location | Count | Status |
|-------|----------|:-----:|--------|
| SQL regression | `tests/regress/jdbc_call/*.sql` | 10 | All pass now |
| XML fixture | `tests/fixtures/jdbc_call/callable_mapper.xml` | 5 statements | Pending |
| Java fixture | `tests/fixtures/jdbc_call/CallableExecutor.java` | 4 methods | Pending |
| validate-xml | `tests/validate_xml.rs` | +3 tests | `#[ignore]` |
| validate-java | `tests/validate_java.rs` | +4 tests | `#[ignore]` |

### After Implementation

- Remove `#[ignore]` from all 7 tests
- Verify they pass
- Run full `cargo test --all-features` to confirm no regressions

---

## Verification Checklist

- [ ] `translate_jdbc_call("{call pkg.proc(?)}")` → `"CALL pkg.proc(?)"`
- [ ] `translate_jdbc_call("{? = call pkg.func(?)}")` → `"CALL pkg.func(?)"`
- [ ] `translate_jdbc_call("call pkg.proc(?)")` → `"call pkg.proc(?)"` (no-op for bare call)
- [ ] `translate_jdbc_call("SELECT 1")` → `"SELECT 1"` (no-op for non-CALLABLE)
- [ ] iBatis `statementType="CALLABLE"` triggers translation
- [ ] Non-CALLABLE statements unaffected
- [ ] `starts_with_sql_keyword` matches `"CALL proc(?)"`, `"{call proc(?)}"`, `"{?=call proc(?)}"`
- [ ] Existing tests all pass
- [ ] New `#[ignore]` tests all pass after un-ignoring
- [ ] `cargo fmt --all -- --check` clean
- [ ] `cargo clippy --all-features -- -D warnings` clean

---

## Files to Modify

| File | Change |
|------|--------|
| `src/ibatis/mod.rs` | Add `translate_jdbc_call` fn; call before `parse_sql` for CALLABLE |
| `src/java/extract.rs` | Extend `starts_with_sql_keyword`; add JDBC escape stripping |
| `tests/validate_xml.rs` | Remove `#[ignore]` from 3 CALLABLE tests |
| `tests/validate_java.rs` | Remove `#[ignore]` from 4 CALLABLE tests |

---

## Risk Assessment

- **Low risk**: Changes are additive and gated behind `statementType="CALLABLE"`
  (Plan 1) or keyword match (Plan 2)
- **No breaking changes**: Non-CALLABLE flows are untouched
- **Translation edge cases**: Malformed `{call...` text without closing `}` should
  fall through gracefully (return original text)
