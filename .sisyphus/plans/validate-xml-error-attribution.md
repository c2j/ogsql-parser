# Fix: validate-xml Error Attribution (N× amplification / silent drop)

## Goal

Fix `validate-xml --csv` (and text mode) where errors in a single XML mapper statement are either amplified N× (N = number of SQL statements) or silently dropped, due to a broken line-number-based per-statement error filter.

## Root Cause

`error_line()` has `_ => 0` for `UnsupportedSyntax` and `TokenizerError` → all non-parse errors get `error_line == 0` → match EVERY statement → N× amplification.

Per-statement filter `error_line(e) == stmt.line` compares two different concepts: `stmt.line` = XML tag line, error remapped line = XML content line. For multi-line content, these differ → error silently dropped.

Validation errors (`validate_from_stmts`) are never remapped — their `location` comes from `StatementInfo.start_line` (always `1` for flat SQL), not from the XML source line.

**Bug introduced in commit `7317361`** (original `validate-xml` implementation). Subsequent commits `5f678dd`, `0f4fa48`, `da6a1d5` preserved the broken filter.

**Previous "fix"** (PR #247, `2c767f8`) was for linter double-iteration in text mode — not for CSV error attribution.

## Plan

### Step 1 — Replace per-statement line-number filter with `parse_result` direct access

**Files**: `src/bin/ogsql.rs`
**Locations**: 5382 (CSV single), 5475 (text single), 5675 (CSV dir)

Current filter:
```rust
let stmt_real_errors = real_errors.iter()
    .filter(|e| error_line(e) == 0 || error_line(e) == stmt.line)
    .collect();
```

Replace with per-statement `parse_result`:
```rust
let (stmt_errors, stmt_warnings) = match &stmt.parse_result {
    Some((_, parse_errors)) => {
        let errs: Vec<_> = parse_errors.iter().filter(|e| !is_warning(e)).collect();
        let warns: Vec<_> = parse_errors.iter().filter(|e| is_warning(e)).collect();
        (errs, warns)
    }
    None => (vec![], vec![]),
};
```

### Step 2 — Collect global errors separately

`all_errors` contains errors that don't belong to any single statement (XML parse errors, MERGE/PACKAGE semantic validation errors). These should appear once as a summary/global row, not attributed per-statement.

After collecting per-statement errors via `parse_result`, collect remaining errors from `all_errors` that are NOT per-statement parse errors, and display them once in a summary section.

**CSV**: emit a `line=0` summary row
**Text**: print after per-statement section, before total summary

### Step 3 — Add `error_line()` arm for `UnsupportedSyntax`

**File**: `src/bin/ogsql.rs:4611`

```rust
ParserError::UnsupportedSyntax { location, .. } => location.line,
```

Needed for text mode summary display (even though CSV/text per-statement filtering no longer uses `error_line`).

Also verify `TokenizerError` handling — should it have a dedicated arm?

### Step 4 — Verify and update tests

**File**: `tests/validate_xml.rs`

Two regression tests already added (19 tests pass). Update assertions to expect correct behavior:
- `test_validate_xml_csv_multi_statement_error_attribution`: q2 should be INVALID with error_count > 0 (currently shows VALID — error dropped)
- `test_validate_xml_csv_merge_validation_error_not_amplified`: error appears exactly once (currently shows 3× amplified)

Run:
```
cargo test --features ibatis --test validate_xml
cargo clippy --all-features -- -D warnings
```

### Step 5 — Validation error line remapping (lower priority)

Currently `validate_from_stmts` → `validate_merge_semantics` creates `SourceLocation` from `si.start_line` (= 1 for flat SQL). Consider passing `body_start_line` context through so validation errors report correct XML source lines.

## Affected Code

| Location | File | Description |
|---|---|---|
| Line 4611-4619 | `src/bin/ogsql.rs` | `error_line()` missing `UnsupportedSyntax` |
| Line 5322-5344 | `src/bin/ogsql.rs` | Error collection (flat `all_errors`) |
| Line 5381-5384 | `src/bin/ogsql.rs` | CSV per-statement filter (SINGLE) |
| Line 5474-5481 | `src/bin/ogsql.rs` | Text per-statement filter (SINGLE) |
| Line 5494-5501 | `src/bin/ogsql.rs` | Text summary (re-prints all errors) |
| Line 5674-5677 | `src/bin/ogsql.rs` | CSV per-statement filter (DIR) |
| ~Line 3798-3835 | `src/bin/ogsql.rs` | `validate_from_stmts` |
| Line 2149-2156 | `src/analyzer/mod.rs` | `validate_merge_semantics` |
| Line 270-278 | `src/ibatis/mod.rs` | `remap_error_line` |
| `tests/validate_xml.rs` | tests | 2 new regression tests |

## Questions

1. How to distinguish "global" errors (MERGE/PACKAGE/XML) from per-statement parse errors after collection?
   - **Option A**: Track which statement each error came from during collection (e.g., `all_errors: Vec<(Option<usize>, ParserError)>`)
   - **Option B**: Parse `all_errors` after collection — can't reliably distinguish
   - **Option C**: Don't flatten — keep per-statement errors separate from global errors at collection time

2. Validation error line remapping: pass `body_start_line` through `validate_from_stmts` → `validate_merge_semantics`, or do a separate remapping step in the CLI output layer?
