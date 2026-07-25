# Public API Extraction (Issues #243 + #244) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Promote two CLI-internal orchestration functions (`validate_from_stmts` and `lint_xml_expanded`) to public library APIs so that all consumers (CLI, HTTP serve, MCP server, TUI, external library users like CodeRoughcollie) share one validation/lint pipeline.

**Architecture:** This is a mechanical-extraction refactor, not new logic. (1) For #244: move three CLI-private helpers (`validate_pl_variables_from_stmts`, `collect_defined_routine_names`, plus a new orchestrator) into a new `analyzer/validate.rs` submodule, expose `validate_statements(&[StatementInfo], ...) -> ValidationReport` preserving typed errors (no folding into `ParserError`). (2) For #243: move `lint_xml_expanded` + 3 helpers into a new `linter/structured.rs` submodule, fix an existing logic bug in the threshold condition, make `estimated_rows` configurable via `LintConfig`, and change the signature to accept `&StructuredMapper` (decoupling parse from lint). The CLI becomes a thin formatting layer over the new public functions.

**Tech Stack:** Rust 2021, existing `analyzer`/`linter`/`ibatis` modules, `cargo test` / `cargo clippy --all-features -D warnings` / `cargo fmt --all -- --check` as the CI gate.

**Why #244 before #243:** #244 is pure relocation + visibility change (zero logic change, lowest risk, immediately unblocks MCP server). #243 requires a bug fix before promotion, so it's sequenced second.

**Reference:**
- Issue #243 body (C018 foreach-in-INSERT-VALUES library API)
- Issue #244 body (validate_statements orchestration API)
- Current CLI code: `src/bin/ogsql.rs:408` (`lint_xml_expanded`), `:3810` (`collect_defined_routine_names`), `:3889` (`validate_from_stmts`), `:3959` (`validate_pl_variables_from_stmts`)
- Current library API: `src/analyzer/mod.rs:1088/1458/2114` (three public validators), `src/linter/mod.rs:303` (`SqlLinter`), `src/ibatis/types.rs:282` (`StructuredMapper`)
- Call sites to update: `src/bin/ogsql.rs:3953, 5110, 5230, 5524, 5539, 5812, 5823, 6597, 6884`; `src/bin/serve/handlers.rs:575, 605, 711`

---

## Prerequisites

**Step 0.1: Create isolated worktree**

This plan modifies `src/bin/ogsql.rs` (6500+ lines), `src/lib.rs`, and adds 2 new library modules. Do this in an isolated worktree to avoid disrupting the current workspace.

```bash
# From repo root
git worktree add ../ogsql-parser-public-api-243-244 -b public-api-extraction-243-244
cd ../ogsql-parser-public-api-243-244
```

**Step 0.2: Verify clean baseline (CI must be green before we start)**

Run all three CI gates and confirm they pass before any change:

```bash
cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings
cargo test --all-features
```

Expected: all three exit 0. Test count baseline: **1772+ tests** (note exact number from output — we must not regress).

**Step 0.3: Verify `ogsql-mcp` binary still builds (it's a separate binary that currently cannot access these functions)**

```bash
cargo build --all-features --bin ogsql-mcp
```

Expected: exit 0. This is our baseline for "MCP still builds" — after the refactor MCP should still build (and could optionally be enhanced to use the new APIs, but that's out of scope).

---

# Phase 1 — Issue #244: Public `validate_statements`

## Task 1: Create `ValidationReport` struct with failing tests

**Files:**
- Create: `src/analyzer/validate.rs`
- Modify: `src/analyzer/mod.rs` (add `mod validate;` declaration)
- Modify: `src/lib.rs` (add re-exports)

**Step 1.1: Write the failing test first (TDD)**

Create `src/analyzer/validate.rs` with **only** the test and a stub struct (compile-error-driven):

```rust
//! Public orchestration API for the `validate` CLI command.
//!
//! Runs PACKAGE consistency, MERGE semantics, and PL variable validation on
//! already-parsed statements, preserving typed errors (no folding into
//! `ParserError`). This is the library-level entry point that the `validate`,
//! `validate-xml`, and `validate-java` CLI commands build on.

use crate::ast::StatementInfo;
use crate::{
    MergeSemanticError, PackageConsistencyError, UndefinedVariableError,
};

/// Aggregate result of running all three validators on a slice of statements.
///
/// Each bucket is independent — e.g. `merge_errors` may be empty while
/// `package_errors` is non-empty. Use [`ValidationReport::is_empty`] to check
/// whether any validator produced findings.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ValidationReport {
    /// PACKAGE spec vs PACKAGE BODY mismatches.
    pub package_errors: Vec<PackageConsistencyError>,
    /// Non-deterministic / invalid MERGE patterns.
    pub merge_errors: Vec<MergeSemanticError>,
    /// Undefined variables / functions in PL/pgSQL blocks.
    pub undefined_variable_errors: Vec<UndefinedVariableError>,
}

impl ValidationReport {
    /// `true` when every bucket is empty (no findings from any validator).
    pub fn is_empty(&self) -> bool {
        self.package_errors.is_empty()
            && self.merge_errors.is_empty()
            && self.undefined_variable_errors.is_empty()
    }

    /// Total number of findings across all buckets.
    pub fn total_count(&self) -> usize {
        self.package_errors.len() + self.merge_errors.len() + self.undefined_variable_errors.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_default_is_empty() {
        let r = ValidationReport::default();
        assert!(r.is_empty());
        assert_eq!(r.total_count(), 0);
    }

    #[test]
    fn report_total_count_sums_all_buckets() {
        let r = ValidationReport {
            package_errors: vec![PackageConsistencyError {
                package_name: "p".into(),
                subprogram_name: "s".into(),
                kind: crate::PackageConsistencyErrorKind::MissingInBody,
                detail: None,
            }],
            merge_errors: vec![MergeSemanticError {
                kind: crate::MergeSemanticErrorKind::DeleteNotSupported,
                detail: None,
                location: crate::SourceLocation::default(),
            }],
            undefined_variable_errors: vec![UndefinedVariableError {
                variable_name: "x".into(),
                location: None,
                context: "ctx".into(),
                kind: crate::UndefinedRefKind::Variable,
            }],
        };
        assert!(!r.is_empty());
        assert_eq!(r.total_count(), 3);
    }
}
```

**Step 1.2: Register the submodule**

In `src/analyzer/mod.rs`, add at the top alongside the existing `mod return_cursor;` / `mod schema;` declarations:

```rust
pub mod validate;
```

**Step 1.3: Add crate-root re-exports in `src/lib.rs`**

In the existing `pub use analyzer::{ ... };` block (lines 66–72), add two new entries (keep alphabetical-ish ordering with the other validate items):

```rust
pub use analyzer::{
    analyze_pl_block, analyze_transactions, compute_query_fingerprints, validate_merge_semantics,
    validate_package_consistency, validate_pl_variables, validate_pl_variables_with_extra_vars,
    validate_pl_variables_with_extra_vars_and_funcs, validate_statements, DynamicSqlReport,
    MergeSemanticError, MergeSemanticErrorKind, PackageConsistencyError, PackageConsistencyErrorKind,
    QueryFingerprint, TransactionReport, UndefinedRefKind, UndefinedVariableError, ValidationReport,
};
```

(Added: `validate_statements` and `ValidationReport`.)

**Step 1.4: Run the struct tests — they should PASS (struct already compiles)**

```bash
cargo test --all-features analyzer::validate
```

Expected: 2 passed.

**Step 1.5: Commit**

```bash
git add src/analyzer/validate.rs src/analyzer/mod.rs src/lib.rs
git commit -m "feat(analyzer): add ValidationReport struct for issue #244

Public aggregate result type for the upcoming validate_statements
orchestration function. Buckets preserve typed errors (no folding
into ParserError). Refs: #244."
```

---

## Task 2: Move `collect_defined_routine_names` into the library

**Files:**
- Modify: `src/analyzer/validate.rs` (add function)
- Modify: `src/bin/ogsql.rs:3810-3883` (delete the CLI-private copy)

**Step 2.1: Read the current CLI implementation**

Read `src/bin/ogsql.rs` lines 3810–3888 to see the exact body (it walks `Statement::CreateFunction`, `CreateProcedure`, `CreatePackage`, `CreatePackageBody`, lowercases + dedups).

**Step 2.2: Copy the function into `src/analyzer/validate.rs` as `pub`**

Add to `src/analyzer/validate.rs` (above the `ValidationReport` impl block, below the `use` statements). Change visibility to `pub` and replace `ogsql_parser::` paths with `crate::`:

```rust
use crate::ast::{Statement, StatementInfo};
// (keep existing use lines)

/// Collect routine names (functions, procedures, package subprograms) defined
/// anywhere in `stmts`. Used to build the "known functions" list for PL
/// variable validation so that intra-statement calls are not flagged as
/// undefined.
///
/// Names are lowercased, sorted, and deduplicated.
pub fn collect_defined_routine_names(stmts: &[StatementInfo]) -> Vec<String> {
    // Paste the EXACT body from src/bin/ogsql.rs:3810-3883, replacing every
    // `ogsql_parser::` prefix with `crate::` and every `ogsql_parser::ast::`
    // with `crate::ast::`.
    // ... (body copy) ...
}
```

**Step 2.3: Delete the CLI-private copy**

Remove the entire `fn collect_defined_routine_names` (lines 3810–3883) from `src/bin/ogsql.rs`.

**Step 2.4: Update the single CLI call site**

Search for callers in `src/bin/ogsql.rs`:

```bash
grep -n "collect_defined_routine_names" src/bin/ogsql.rs
```

The only caller is inside `validate_from_stmts` (which we'll move in Task 3). For now, if `validate_from_stmts` still references the old name, update it to call the library version via the `ogsql_parser::` prefix (it already does so for other validators — e.g. `ogsql_parser::validate_package_consistency`). Confirm by grepping after the edit.

**Step 2.5: Run all tests — verify no regression**

```bash
cargo test --all-features
```

Expected: same count as baseline (1772+ passed, 0 failed). If a test fails, the body copy was not exact.

**Step 2.6: Commit**

```bash
git add src/analyzer/validate.rs src/bin/ogsql.rs
git commit -m "refactor(analyzer): move collect_defined_routine_names to library

Now public at ogsql_parser::collect_defined_routine_names. Mechanical
move — no logic change. Part of #244."
```

---

## Task 3: Move `validate_pl_variables_from_stmts` into the library

**Files:**
- Modify: `src/analyzer/validate.rs` (add function)
- Modify: `src/bin/ogsql.rs:3959-4062` (delete the CLI-private copy)

**Step 3.1: Read the current CLI implementation**

Read `src/bin/ogsql.rs` lines 3959–4063 — this is the ~100-line function that walks `StatementInfo` variants, extracts `PlBlock`s from `CreateProcedure`/`CreateFunction`/`Do`/`CreatePackageBody`, cross-references package variables across spec+body, and calls `validate_pl_variables_with_extra_vars_and_funcs` per block.

**Step 3.2: Copy into `src/analyzer/validate.rs` as `pub`**

Same mechanical relocation: change `ogsql_parser::` → `crate::`, mark the function `pub`. The function signature is:

```rust
pub fn validate_pl_variables_from_stmts(
    stmts: &[StatementInfo],
    known_funcs: &[String],
    strict: bool,
) -> Vec<UndefinedVariableError>
```

**Step 3.3: Delete the CLI-private copy** (lines 3959–4062 of `src/bin/ogsql.rs`).

**Step 3.4: Verify call sites compile**

`validate_pl_variables_from_stmts` is currently called only from `validate_from_stmts` (which is CLI-internal and will move in Task 4). After Task 4, the CLI will call the library version. For now, just confirm with `cargo check --all-features` that nothing else references the deleted copy.

```bash
cargo check --all-features
```

Expected: exit 0. (If `validate_from_stmts` still calls the old local version, point it at `ogsql_parser::validate_pl_variables_from_stmts` temporarily — it'll be deleted in Task 4 anyway.)

**Step 3.5: Run all tests**

```bash
cargo test --all-features
```

Expected: same count as baseline.

**Step 3.6: Commit**

```bash
git add src/analyzer/validate.rs src/bin/ogsql.rs
git commit -m "refactor(analyzer): move validate_pl_variables_from_stmts to library

Now public at ogsql_parser::validate_pl_variables_from_stmts. Bridges
StatementInfo slices to per-PlBlock validate_pl_variables_* calls.
Part of #244."
```

---

## Task 4: Implement `validate_statements` orchestrator

**Files:**
- Modify: `src/analyzer/validate.rs` (add orchestrator + tests)

**Step 4.1: Write failing tests first (TDD)**

Append to the `mod tests` block in `src/analyzer/validate.rs`:

```rust
    #[test]
    fn validate_statements_empty_input_yields_empty_report() {
        let report = validate_statements(&[], &[], false);
        assert!(report.is_empty());
    }

    #[test]
    fn validate_statements_detects_merge_error() {
        // Non-deterministic MERGE: DELETE not supported
        let sql = "MERGE INTO t USING s ON t.id = s.id \
                   WHEN MATCHED THEN DELETE \
                   WHEN NOT MATCHED THEN INSERT (id) VALUES (s.id)";
        let stmts = parse_stmts(sql);
        let report = validate_statements(&stmts, &[], false);
        assert!(!report.merge_errors.is_empty());
        assert!(report.package_errors.is_empty());
    }

    #[test]
    fn validate_statements_detects_package_mismatch() {
        // PACKAGE spec declares proc; BODY omits it
        let sql = "CREATE PACKAGE pkg AS PROCEDURE foo(); END pkg; \
                   CREATE PACKAGE BODY pkg AS END pkg;";
        let stmts = parse_stmts(sql);
        let report = validate_statements(&stmts, &[], false);
        assert!(!report.package_errors.is_empty());
    }

    #[test]
    fn validate_statements_detects_undefined_variable() {
        let sql = "CREATE OR REPLACE FUNCTION f() RETURNS VOID AS \$\$ \
                   BEGIN PERFORM undefined_thing(); END; \$\$ LANGUAGE plpgsql";
        let stmts = parse_stmts(sql);
        let report = validate_statements(&stmts, &[], true); // strict
        assert!(!report.undefined_variable_errors.is_empty());
    }

    /// Helper: parse a SQL string into StatementInfo (mirrors CLI parse_input).
    fn parse_stmts(sql: &str) -> Vec<StatementInfo> {
        use crate::{Parser, Tokenizer};
        let tokens = Tokenizer::new(sql).tokenize().expect("tokenize");
        let output = Parser::new(tokens).parse();
        // StatementInfo construction: see src/ast/mod.rs:958 — fields are
        // sql_text, line/col spans, and the parsed Statement.
        // If Parser already returns StatementInfo, just return output.statements;
        // otherwise wrap each Statement into StatementInfo with default span.
        output // adjust to actual Parser API
    }
```

> Note: the exact `parse_stmts` helper depends on what `Parser::parse()` returns — check the return type first and adjust. If `Parser::parse()` returns `Vec<Statement>` rather than `Vec<StatementInfo>`, look at how `src/bin/ogsql.rs` builds `StatementInfo` (search for `StatementInfo {` in the binary) and mirror that wrapping.

**Step 4.2: Run the tests — verify they fail to compile (function doesn't exist yet)**

```bash
cargo test --all-features analyzer::validate
```

Expected: compile error ("cannot find function `validate_statements`").

**Step 4.3: Implement `validate_statements`**

Add to `src/analyzer/validate.rs`:

```rust
/// Run PACKAGE consistency, MERGE semantics, and PL variable validation on
/// already-parsed statements. Returns typed errors in three independent
/// buckets — no folding into `ParserError` (that's a CLI output concern).
///
/// This is the library-level equivalent of the `validate` / `validate-xml` /
/// `validate-java` CLI commands' shared pipeline.
///
/// # Arguments
/// * `stmts` - Already-parsed SQL statements (one slice per source file).
/// * `extra_funcs` - Additional function names to treat as defined (e.g.
///   routines declared in external packages the consumer knows about).
/// * `strict` - When `true`, flag undefined function calls in PL blocks
///   (mirrors the CLI `--strict` flag).
pub fn validate_statements(
    stmts: &[StatementInfo],
    extra_funcs: &[String],
    strict: bool,
) -> ValidationReport {
    let package_errors = crate::validate_package_consistency(stmts);
    let merge_errors = crate::validate_merge_semantics(stmts);

    // Build known-functions list: caller-provided extras ∪ routines defined
    // in-statement.
    let mut all_funcs: Vec<String> = extra_funcs.to_vec();
    all_funcs.extend(collect_defined_routine_names(stmts));
    all_funcs.sort();
    all_funcs.dedup();

    let undefined_variable_errors =
        validate_pl_variables_from_stmts(stmts, &all_funcs, strict);

    ValidationReport {
        package_errors,
        merge_errors,
        undefined_variable_errors,
    }
}
```

**Step 4.4: Run the tests — verify they pass**

```bash
cargo test --all-features analyzer::validate
```

Expected: all `analyzer::validate` tests pass (2 from Task 1 + 4 new = 6).

**Step 4.5: Run full test suite**

```bash
cargo test --all-features
```

Expected: baseline + 4 new tests passed, 0 failed.

**Step 4.6: Commit**

```bash
git add src/analyzer/validate.rs
git commit -m "feat(analyzer): add validate_statements orchestrator (issue #244)

Public ogsql_parser::validate_statements(&[StatementInfo], &[String],
bool) -> ValidationReport. Runs all three validators (package
consistency, merge semantics, PL variables) and preserves typed errors.
Replaces the CLI-private validate_from_stmts pipeline."
```

---

## Task 5: Refactor CLI `validate_from_stmts` to use the new public API

**Files:**
- Modify: `src/bin/ogsql.rs:3889-3956` (rewrite as a thin formatting wrapper)
- Modify: `src/bin/serve/handlers.rs:575, 711` (HTTP handlers)

**Step 5.1: Rewrite the CLI `validate_from_stmts`**

The current return type is `(Vec<ParserError>, Vec<PackageConsistencyError>, Vec<UndefinedVariableError>)` — a 3-tuple where `MergeSemanticError` is awkwardly folded into `ParserError::UnsupportedSyntax` (loss of type info). Callers use this tuple for CLI output formatting only.

Replace the body with a call to the new library function, then format `ValidationReport` into the legacy tuple shape so existing callers (lines 3953, 5524, 5812, 6597, 6884) continue to work without changes:

```rust
fn validate_from_stmts(
    stmts: &[ogsql_parser::StatementInfo],
    extra_funcs: &[String],
    strict: bool,
) -> (
    Vec<ogsql_parser::ParserError>,
    Vec<ogsql_parser::PackageConsistencyError>,
    Vec<ogsql_parser::UndefinedVariableError>,
) {
    // Delegate to the public library orchestrator (preserves typed errors).
    let report = ogsql_parser::validate_statements(stmts, extra_funcs, strict);

    // Format typed errors into ParserError for CLI display. This conversion
    // lives in the binary because ParserError formatting is a CLI concern.
    let mut errors = Vec::new();

    for pe in &report.package_errors {
        let msg = match &pe.detail {
            Some(d) => format!("package {}: {} — {}", pe.package_name, pe.subprogram_name, d),
            None => format!("package {}: {} — {:?}", pe.package_name, pe.subprogram_name, pe.kind),
        };
        errors.push(ogsql_parser::ParserError::Warning {
            message: msg,
            location: ogsql_parser::SourceLocation::default(),
        });
    }

    for me in &report.merge_errors {
        errors.push(ogsql_parser::ParserError::UnsupportedSyntax {
            location: me.location,
            syntax: "MERGE".to_string(),
            hint: merge_error_detail(me),
        });
    }

    (errors, report.package_errors, report.undefined_variable_errors)
}
```

Note: the current CLI tuple does NOT surface `MergeSemanticError` as a typed bucket — it only surfaces them inside the `ParserError` vector. We preserve that (questionable but existing) behavior for CLI-output backwards compatibility. Library consumers get the clean typed `report.merge_errors` bucket directly.

**Step 5.2: Update `src/bin/serve/handlers.rs` HTTP handlers**

Lines 575 and 711 currently call `crate::validate_from_stmts(&all_stmts, &[], strict)`. Two options:

- **Minimal (recommended for this PR):** leave them calling `crate::validate_from_stmts` — no behavior change, HTTP API stays identical.
- **Cleanup (optional follow-up):** have handlers call `ogsql_parser::validate_statements` directly and format themselves. This is a larger change to `handlers.rs` and should be a separate PR.

For this plan: do the **minimal** option. The handlers still benefit indirectly because `crate::validate_from_stmts` now delegates to the public library function.

**Step 5.3: Verify everything compiles and tests pass**

```bash
cargo build --all-features --bins
cargo test --all-features
```

Expected: all binaries build; all tests pass at baseline + new count.

**Step 5.4: Manually verify CLI behavior is unchanged**

```bash
# MERGE error detection
echo "MERGE INTO t USING s ON t.id = s.id WHEN MATCHED THEN DELETE" | \
  cargo run --all-features --bin ogsql -- validate

# Package consistency
echo "CREATE PACKAGE pkg AS PROCEDURE foo(); END pkg; CREATE PACKAGE BODY pkg AS END pkg;" | \
  cargo run --all-features --bin ogsql -- validate
```

Expected output matches the pre-refactor behavior exactly (compare against `git stash` + run + `git stash pop` if unsure).

**Step 5.5: Commit**

```bash
git add src/bin/ogsql.rs src/bin/serve/handlers.rs
git commit -m "refactor(cli): delegate validate_from_stmts to public validate_statements

CLI now thin wrapper over ogsql_parser::validate_statements.
ParserError formatting stays in binary (CLI output concern).
Behavior unchanged. Closes #244 library-API gap."
```

---

## Task 6: Phase 1 verification gate

**Step 6.1: Run full CI suite locally**

```bash
cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings
cargo test --all-features
```

Expected: all three exit 0. Test count = baseline + 6 new tests (2 + 4).

**Step 6.2: Verify `ogsql-mcp` still builds**

```bash
cargo build --all-features --bin ogsql-mcp
```

Expected: exit 0.

**Step 6.3: Do NOT commit (verification only). If anything fails, fix before Phase 2.**

---

# Phase 2 — Issue #243: Public `lint_structured_mapper` (foreach C018)

> **Rename note:** The CLI function is called `lint_xml_expanded` because it took raw XML bytes. The library version will accept a `&StructuredMapper` (already parsed), so the name `lint_xml_expanded` becomes misleading. New name: **`lint_structured_mapper`**. Callers who have raw XML bytes should call `ibatis::parse_mapper_bytes_structured` first, then pass the result.

## Task 7: Write a failing test that reproduces the existing threshold bug

**Context — the bug:** In the current `lint_xml_expanded` (ogsql.rs:425–428):

```rust
let estimated_rows = 1000;
let total_params = params_per_row * estimated_rows;
if total_params > config.max_insert_values_rows || is_insert_values {
    warnings.push(...);
}
```

The `|| is_insert_values` clause means: any `<foreach>` nested inside INSERT VALUES triggers a warning **regardless of the threshold**, even if `params_per_row * 1000` is well below `config.max_insert_values_rows`. That's almost certainly not intended — a 1-parameter foreach on a 5-row collection should not fire C018 if the threshold is 1000.

**Files:**
- Create: `src/linter/structured.rs` (initial skeleton with test only)

**Step 7.1: Create the new module with a failing test**

```rust
//! Structured-mapper lint rules — rules that need the pre-expansion `SqlNode`
//! tree (dynamic SQL), which the flat `SqlLinter::lint(&[StatementInfo])` API
//! cannot see.
//!
//! Currently houses the foreach-in-INSERT-VALUES flavor of rule C018.

use crate::ibatis::types::{SqlNode, StructuredMapper};
use crate::linter::{Confidence, LintConfig, SqlWarning, WarningLevel};

/// Lint a structured mapper for foreach-in-INSERT-VALUES (rule C018,
/// dynamic-SQL variant). Flat SQL collapses `<foreach>` to a single iteration,
/// so this must run on the `SqlNode` tree before expansion.
pub fn lint_structured_mapper(
    mapper: &StructuredMapper,
    config: &LintConfig,
) -> Vec<SqlWarning> {
    unimplemented!("placeholder — implemented in Task 9")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_mapper(xml: &[u8]) -> StructuredMapper {
        crate::ibatis::parse_mapper_bytes_structured(xml)
    }

    #[test]
    fn c018_fires_when_estimated_params_exceed_threshold() {
        // 5 params per row × estimated_rows (1000) = 5000. We set the
        // threshold explicitly to make the test deterministic and decoupled
        // from the LintConfig default (which is 65535 — verified in Task 8).
        let xml = br#"<mapper namespace="t">
            <insert id="batch">
                INSERT INTO t (a, b, c, d, e) VALUES
                <foreach collection="rows" item="r" separator=",">
                    (#{r.a}, #{r.b}, #{r.c}, #{r.d}, #{r.e})
                </foreach>
            </insert>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let mut config = LintConfig::default();
        config.max_insert_values_rows = 1000; // 5000 > 1000 → must fire
        let warnings = lint_structured_mapper(&mapper, &config);
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].rule_id, "C018");
    }

    #[test]
    fn c018_does_NOT_fire_when_estimated_params_below_threshold() {
        // BUG REPRODUCTION: 1 param per row. With the bug present
        // (`|| is_insert_values`), this would still fire because the foreach
        // is inside INSERT VALUES. The fix removes that clause.
        let xml = br#"<mapper namespace="t">
            <insert id="batch">
                INSERT INTO t (a) VALUES
                <foreach collection="rows" item="r" separator=",">
                    (#{r.a})
                </foreach>
            </insert>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let mut config = LintConfig::default();
        config.max_insert_values_rows = usize::MAX; // 1000 << usize::MAX → must NOT fire
        let warnings = lint_structured_mapper(&mapper, &config);
        assert_eq!(
            warnings.len(),
            0,
            "threshold is usize::MAX — single-param foreach must NOT fire C018"
        );
    }

    #[test]
    fn c018_no_foreach_no_warning() {
        let xml = br#"<mapper namespace="t">
            <insert id="one">
                INSERT INTO t (a) VALUES (1)
            </insert>
        </mapper>"#;
        let mapper = parse_mapper(xml);
        let config = LintConfig::default();
        let warnings = lint_structured_mapper(&mapper, &config);
        assert!(warnings.is_empty());
    }
}
```

**Step 7.2: Register the submodule**

In `src/linter/mod.rs`, near the top (alongside `mod rules_caution;` etc.):

```rust
#[cfg(feature = "ibatis")]
pub mod structured;
```

**Step 7.3: Run the tests — verify they fail**

```bash
cargo test --all-features --features ibatis linter::structured
```

Expected: tests **panic** with `unimplemented!` (this confirms the tests are wired correctly).

**Step 7.4: Commit (test commits first — TDD red phase)**

```bash
git add src/linter/structured.rs src/linter/mod.rs
git commit -m "test(linter): add failing tests for lint_structured_mapper (issue #243)

Three tests including one that reproduces the existing `|| is_insert_values`
threshold bug in lint_xml_expanded. Red phase before green."
```

---

## Task 8: Inspect and verify the `max_insert_values_rows` default

**Step 8.1: Read the `LintConfig` definition**

Open `src/linter/mod.rs:116-165` and confirm the default value of `max_insert_values_rows`. If it's `usize::MAX` or unset, the tests above need adjustment. If it's a sensible number like 1000, the tests are correct as written.

```bash
grep -n "max_insert_values_rows" src/linter/mod.rs
```

**Step 8.2: If default is NOT a finite number, fix `Default` impl**

If `max_insert_values_rows` defaults to 0 or `usize::MAX`, the threshold semantics break. A sensible default is 1000 (matches the CLI's hardcoded `estimated_rows`). Adjust the `Default` impl for `LintConfig` accordingly, but only if needed.

**Step 8.3: Commit if changed**

```bash
git add src/linter/mod.rs
git commit -m "fix(linter): sensible default for max_insert_values_rows"
```

---

## Task 9: Implement `lint_structured_mapper` (with bug fix baked in)

**Files:**
- Modify: `src/linter/structured.rs` (replace `unimplemented!` body + add helpers)
- Modify: `src/linter/mod.rs` (add `foreach_estimated_rows` to `LintConfig`)

**Step 9.1: Add `foreach_estimated_rows` to `LintConfig`**

In `src/linter/mod.rs`, add a new field to `LintConfig` (after `max_insert_values_rows`):

```rust
pub struct LintConfig {
    // ... existing fields ...
    /// Representative iteration count for `<foreach>` collections when
    /// estimating dynamic-SQL bind parameter totals. Used by rule C018's
    /// structured-mapper variant. Defaults to 1000.
    #[cfg(feature = "ibatis")]
    pub foreach_estimated_rows: usize,
    // ... rest ...
}
```

And in the `Default` impl:

```rust
impl Default for LintConfig {
    fn default() -> Self {
        Self {
            // ... existing ...
            #[cfg(feature = "ibatis")]
            foreach_estimated_rows: 1000,
            // ... rest ...
        }
    }
}
```

**Step 9.2: Implement `lint_structured_mapper` + helpers**

Replace the `unimplemented!` body in `src/linter/structured.rs` with the corrected logic. **Critical: the buggy `|| is_insert_values` clause is dropped.**

```rust
pub fn lint_structured_mapper(
    mapper: &StructuredMapper,
    config: &LintConfig,
) -> Vec<SqlWarning> {
    if !mapper.errors.is_empty() {
        return vec![];
    }
    let mut warnings = Vec::new();

    for stmt in &mapper.statements {
        if let Some(foreach_node) = find_foreach_in_insert_values(&stmt.body) {
            let params_per_row = count_params_in_foreach_body(foreach_node);
            if params_per_row == 0 {
                continue;
            }
            let estimated_rows = config.foreach_estimated_rows;
            let total_params = params_per_row.saturating_mul(estimated_rows);

            // FIXED: removed the `|| is_insert_values` clause that fired
            // regardless of threshold. Now the threshold is the sole trigger,
            // which is what the rule name "excessive-insert-values" implies.
            if total_params > config.max_insert_values_rows {
                warnings.push(SqlWarning {
                    level: WarningLevel::Caution,
                    rule_id: "C018".to_string(),
                    rule_name: "excessive-insert-values".to_string(),
                    message: format!(
                        "INSERT VALUES 包含 foreach 动态批量插入，每行 {} 个参数。\
                         若运行时集合包含约 {} 行，总绑定参数将达 {}，超过阈值 {}。\
                         建议分批提交或使用 COPY。",
                        params_per_row,
                        estimated_rows,
                        total_params,
                        config.max_insert_values_rows
                    ),
                    suggestion: Some(
                        "拆分为更小批次插入以减少锁持有时间，或使用 COPY 替代".to_string(),
                    ),
                    location: crate::SourceLocation::default(),
                    gaussdb_ref: None,
                    confidence: Confidence::Partial,
                });
            }
        }
    }
    warnings
}

/// True if the SqlNode tree contains an INSERT ... VALUES pattern.
fn is_insert_with_values(node: &SqlNode) -> bool {
    use SqlNode::*;
    match node {
        Text { content } => {
            let lower = content.to_lowercase();
            lower.contains("insert") && lower.contains("values")
        }
        Sequence { children } | Trim { children, .. } => {
            children.iter().any(is_insert_with_values)
        }
        _ => false,
    }
}

/// Find a `ForEach` node nested inside an INSERT VALUES context, if any.
fn find_foreach_in_insert_values(node: &SqlNode) -> Option<&SqlNode> {
    // Copy exact body from src/bin/ogsql.rs:468-501.
    // Replace `ogsql_parser::ibatis::types::SqlNode` with the local `SqlNode` alias.
    unimplemented!("copy body from ogsql.rs:468-501")
}

/// Count `#{...}` / `${...}` parameters inside a foreach body.
fn count_params_in_foreach_body(node: &SqlNode) -> usize {
    // Copy exact body from src/bin/ogsql.rs:502-529.
    unimplemented!("copy body from ogsql.rs:502-529")
}
```

> Note: the helper bodies are mechanical copies from `src/bin/ogsql.rs:468-529`. The plan leaves them as `unimplemented!()` placeholders because the exact recursion shape must be copied verbatim — do this in the editor, not from the plan.

**Step 9.3: Run the tests — verify they pass (green)**

```bash
cargo test --all-features --features ibatis linter::structured
```

Expected: all 3 tests pass, including `c018_does_NOT_fire_when_estimated_params_below_threshold` (which would have failed under the old buggy logic).

**Step 9.4: Run full test suite**

```bash
cargo test --all-features
```

Expected: baseline + new tests pass.

**Step 9.5: Commit**

```bash
git add src/linter/structured.rs src/linter/mod.rs
git commit -m "feat(linter): add lint_structured_mapper for foreach C018 (issue #243)

Public ogsql_parser::linter::structured::lint_structured_mapper. Accepts
&StructuredMapper (not raw bytes), foreach_estimated_rows now configurable
via LintConfig. Drops the buggy || is_insert_values clause that fired
regardless of threshold. Green phase."
```

---

## Task 10: Refactor CLI call sites to use the new public function

**Files:**
- Modify: `src/bin/ogsql.rs` (delete `lint_xml_expanded` + 3 helpers, update 4 call sites)
- Modify: `src/bin/serve/handlers.rs:605` (update 1 call site)

**Step 10.1: Delete the CLI-private `lint_xml_expanded` and its helpers**

Remove from `src/bin/ogsql.rs`:
- `fn lint_xml_expanded` (lines 408–450)
- `fn is_insert_with_values` (lines 454–467)
- `fn find_foreach_in_insert_values` (lines 468–501)
- `fn count_params_in_foreach_body` (lines 502–529)

These are ~120 lines of code removed from the binary.

**Step 10.2: Update CLI call sites (4 occurrences)**

The current call pattern is `lint_xml_expanded(&bytes_or_input, &config)`. Replace each with:

```rust
let structured = ogsql_parser::ibatis::parse_mapper_bytes_structured(&bytes_or_input);
let expand_ws = ogsql_parser::linter::structured::lint_structured_mapper(&structured, &config);
```

Update lines: 5110, 5230, 5539, 5823 in `src/bin/ogsql.rs`.

> Rationale: the new signature takes `&StructuredMapper`, so callers must parse first. If a call site already has a `StructuredMapper` from a prior step, reuse it instead of re-parsing.

**Step 10.3: Update the HTTP handler call site**

In `src/bin/serve/handlers.rs:605`, replace:

```rust
let expand_ws = crate::lint_xml_expanded(xml_bytes, &config);
```

with:

```rust
let structured = ogsql_parser::ibatis::parse_mapper_bytes_structured(xml_bytes);
let expand_ws = ogsql_parser::linter::structured::lint_structured_mapper(&structured, &config);
```

**Step 10.4: Verify everything builds and tests pass**

```bash
cargo build --all-features --bins
cargo test --all-features
```

Expected: all binaries build; all tests pass.

**Step 10.5: Manually verify CLI behavior**

The behavior change here is **intentional**: the bug fix means some previously-fired C018 warnings (single-param foreach below threshold) will no longer fire. Verify the legitimate cases still fire:

```bash
# Should fire C018 (5 params/row × 1000 = 5000 > threshold)
cat > /tmp/test_foreach.xml <<'EOF'
<mapper namespace="t">
  <insert id="batch">
    INSERT INTO t (a,b,c,d,e) VALUES
    <foreach collection="rows" item="r" separator=",">
      (#{r.a},#{r.b},#{r.c},#{r.d},#{r.e})
    </foreach>
  </insert>
</mapper>
EOF
cargo run --all-features --bin ogsql -- validate-xml -f /tmp/test_foreach.xml
```

Expected output contains a C018 warning. (This was already the behavior — regression check.)

**Step 10.6: Commit**

```bash
git add src/bin/ogsql.rs src/bin/serve/handlers.rs
git commit -m "refactor(cli): use public lint_structured_mapper for C018

Removes ~120 lines of private helpers from the binary. Call sites now
parse_mapper_bytes_structured + lint_structured_mapper. Closes #243.

Behavior change: single-param foreach below threshold no longer fires
C018 (fixes pre-existing || is_insert_values bug)."
```

---

## Task 11: Phase 2 verification gate

**Step 11.1: Full CI gate**

```bash
cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings
cargo test --all-features
```

Expected: all three exit 0.

**Step 11.2: `cargo audit`**

```bash
cargo audit
```

Expected: no vulnerabilities (we added no new dependencies).

**Step 11.3: Build all binaries including MCP**

```bash
cargo build --all-features --bins
```

Expected: `ogsql`, `ogsql-mcp` both build cleanly.

---

# Phase 3 — Documentation & Wrap-up

## Task 12: Update `lib.rs` doc comment + `docs/crate-guide.md`

**Files:**
- Modify: `src/lib.rs` (top-level doc comment, lines 18–27 feature list)
- Modify: `docs/crate-guide.md` (add `validate_statements` and `lint_structured_mapper` to the public API section)

**Step 12.1: Add a "Validation" example to `src/lib.rs` doc comment**

After the existing "Quick start" example (lines 7–16), add a second example showing the new public API:

```rust
//! # Validation
//!
//! ```
//! use ogsql_parser::{Tokenizer, parser::Parser, validate_statements};
//!
//! let sql = "MERGE INTO t USING s ON t.id = s.id WHEN MATCHED THEN DELETE";
//! let tokens = Tokenizer::new(sql).tokenize()?;
//! let stmts = Parser::new(tokens).parse();
//! let report = validate_statements(&stmts, &[], /* strict */ false);
//! if !report.merge_errors.is_empty() {
//!     println!("MERGE semantic issues found: {}", report.merge_errors.len());
//! }
//! # Ok::<(), ogsql_parser::TokenizerError>(())
//! ```
```

**Step 12.2: Update `docs/crate-guide.md`**

Add entries for `validate_statements`, `ValidationReport`, and `linter::structured::lint_structured_mapper` in the public API reference section. Include the typed-error buckets and the `foreach_estimated_rows` config field.

**Step 12.3: Commit**

```bash
git add src/lib.rs docs/crate-guide.md
git commit -m "docs: document new public validate_statements and lint_structured_mapper APIs"
```

---

## Task 13: Open PR

**Step 13.1: Push the branch**

```bash
git push -u origin public-api-extraction-243-244
```

**Step 13.2: Create the PR**

```bash
gh pr create \
  --title "Public API extraction for issues #243 and #244" \
  --body "Closes #243, closes #244.

## Summary

Promotes two CLI-internal orchestration functions to public library APIs:

- **#244**: \`validate_statements(&[StatementInfo], &[String], bool) -> ValidationReport\` — runs all three validators (package consistency, merge semantics, PL variables) with typed errors preserved.
- **#243**: \`linter::structured::lint_structured_mapper(&StructuredMapper, &LintConfig) -> Vec<SqlWarning>\` — foreach-in-INSERT-VALUES detection (C018 dynamic variant).

## Behavior changes

- **Bug fix (intentional)**: single-param \`<foreach>\` below the C018 threshold no longer fires. Previous logic had \`|| is_insert_values\` which fired regardless of threshold.
- CLI / HTTP behavior unchanged for all legitimate warnings.

## Consumers enabled

- \`ogsql-mcp\` binary can now call \`validate_statements\` directly (previously cut off from this orchestration).
- External library users (e.g. CodeRoughcollie) get both APIs without walking parser internals.

## Verification

- [x] \`cargo fmt --all -- --check\`
- [x] \`cargo clippy --all-features -- -D warnings\`
- [x] \`cargo test --all-features\` (baseline + 9 new tests)
- [x] \`cargo audit\`
- [x] All binaries (\`ogsql\`, \`ogsql-mcp\`) build with \`--all-features\`"
```

---

## Risk Roll-up

| Risk | Likelihood | Mitigation |
|---|---|---|
| Parser API mismatch in `parse_stmts` test helper (Task 4) | Medium | Read `Parser::parse` return type first; mirror binary's `StatementInfo` construction |
| Helper copy-paste introduces typo (Tasks 2, 3, 9) | Low | Mechanical — diff the deleted body against the new location; full test suite catches regressions |
| C018 bug fix breaks downstream consumer depending on buggy behavior | Low | The buggy behavior was over-firing (false positives); fixing it strictly improves UX. Documented in PR body. |
| `LintConfig` field addition breaks serde deserialization of existing `.ogsql-lint.toml` files | Low | New field has `Default`; serde ignores missing fields by default unless `#[serde(deny_unknown_fields)]` is set. Verify in Task 9. |
| Call site count mismatch (missed a caller) | Low | Each task greps before deleting; CI catches the rest |

## Out of Scope (Explicit Non-Goals)

- Refactoring `serve/handlers.rs` to call library functions directly (kept minimal — separate PR)
- Adding new lint rules
- Splitting `analyzer/mod.rs` further (only extracting the orchestration pieces)
- MCP server feature work beyond "still builds"
- Changing `LintRuleEntry.check_fn` signature (option (a) from issue #243 — rejected; see Architecture)
