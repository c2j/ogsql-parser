# ogsql-parser Codebase Cleanup Plan

## Context

Comprehensive codebase health review completed (2026-07-25). Four parallel exploration agents analyzed: project structure, OSS conventions, code organization, CI/tooling. 24 issues identified across 4 severity tiers.

## Scope

Cleanup and reorganization only. No feature changes, no behavioral modifications. Every change must pass existing CI (fmt, clippy, test, audit).

## Change Log

### Phase 1: Critical — Must Fix First (4 items)

#### 1.1 Add LICENSE files
- **Files to create**: `LICENSE-MIT`, `LICENSE-APACHE` (or single `LICENSE` if changing SPDX)
- **Why**: `Cargo.toml` `include` field references these files; `cargo publish` fails without them
- **Risk**: Zero — pure additive
- **Verification**: `cargo package --list` confirms LICENSE files included

#### 1.2 Fix Cargo.toml repository URL
- **File to edit**: `Cargo.toml` line 11
- **Change**: `repository = "https://github.com/User/ogsql-parser"` → real repository URL
- **Risk**: Zero
- **Verification**: merge placeholder `User` replaced; `cargo metadata` shows correct URL

#### 1.3 Delete misplaced docs/CONTRIBUTING.md
- **File to delete**: `docs/CONTRIBUTING.md` (365 lines, belongs to "Metamorphosis" project)
- **Why**: Describes a completely different project (workspace with crates/core, rules, cli, qed, etc.). Root `CONTRIBUTING.md` is the correct one.
- **Risk**: Zero — content is for a different project
- **Verification**: root `CONTRIBUTING.md` still exists

#### 1.4 Untrack committed .DS_Store files
- **Action**: `git rm --cached .DS_Store docs/.DS_Store src/.DS_Store docs/reports/.DS_Store`
- **Why**: macOS metadata already in `.gitignore` but tracked; persistent annoyance
- **Risk**: Zero — already gitignored
- **Verification**: `git ls-files '*.DS_Store'` returns empty

### Phase 2: High Priority (6 items)

#### 2.1 Relocate GaussDB-2.23.07.210/ out of repo root
- **Current**: `GaussDB-2.23.07.210/` at repo root (57MB, 7,376 files, committed directly)
- **Proposed**: Move to `docs/references/gaussdb-2.23.07.210/`
- **Why**: Product documentation doesn't belong at repo root; also contains path with spaces + Chinese chars that breaks shell tooling
- **Risk**: Path change breaks any scripts referencing the old path
- **Mitigation**: Grep for `GaussDB-2.23.07.210` references first; update as needed
- **Verification**: old path doesn't exist; `cargo test --all-features` passes

#### 2.2 Move binary artifact to docs/reports/.gitignore exclusion
- **File**: `docs/reports/ogsql_bench_artifact.tar.gz` (1.5MB tracked binary)
- **Action**: Add to `.gitignore`, `git rm --cached` it, document that benchmarks are published to GitHub Releases
- **Risk**: None — the file isn't needed for builds
- **Verification**: `git ls-files '*.tar.gz'` returns empty

#### 2.3 Reorganize lib/ submodules
- **Current**: 9 git submodules under `lib/` (8 Java test fixtures + 1 reference grammar)
- **Proposed**: Keep `lib/openGauss-server` (reference grammar); document that Java test submodules are optional test fixtures
- **Action**: No file moves (submodules are git-managed). Add note to CONTRIBUTING.md explaining `git submodule update --init lib/openGauss-server` for core, vs `--recursive` for all.
- **Risk**: Zero — no file changes

#### 2.4 Create CHANGELOG.md
- **Action**: Create minimal `CHANGELOG.md` with current version and a note that future releases will use `git-cliff` or `cargo-release` automation
- **Risk**: Zero — additive
- **Verification**: file exists

#### 2.5 Add CI MSRV verification job
- **File**: `.github/workflows/ci.yml`
- **Action**: Add job that checks `cargo hack check --rust-version --all-features` (or `cargo check` with `dtolnay/rust-toolchain@1.70`)
- **Risk**: Low — purely additive CI step; won't block existing pipelines
- **Verification**: CI passes on the PR adding this job

#### 2.6 Clean up Cargo.toml dev-dependencies section
- **Action**: Remove empty `[dev-dependencies]` section (line 46-47) if truly unused; or populate with any test-only deps currently pulled through main deps
- **Risk**: Zero if empty; low if adding deps
- **Verification**: `cargo test --all-features` passes

### Phase 3: Medium Priority (8 items)

#### 3.1 Fix lib.rs allow attributes
- **File**: `src/lib.rs` lines 49-65
- **Issue**: Global `#![allow(dead_code, unused_assignments, unused_macros, unreachable_patterns, unexpected_cfgs, ...)]` suppresses real compiler warnings
- **Action**: Remove `dead_code`, `unused_assignments`, `unused_macros`, `unreachable_patterns` from crate-level allow; fix surfaced issues; add `[PERMANENT]` / `[TODO: cleanup]` annotations per CONTRIBUTING.md Appendix A5
- **Risk**: Medium — may surface dozens of warnings across 81 files
- **Mitigation**: Phase the removals; fix one category at a time with `cargo check` validation
- **Verification**: `cargo clippy --all-features -- -D warnings` passes with fewer allows

#### 3.2 Delete dead test file
- **File**: `src/parser/tests_plsql_fixes_debug.rs` (1 line: `// Temporary debug test`)
- **Why**: Never referenced by `parser/mod.rs`; dead code
- **Risk**: Zero
- **Verification**: `cargo test --all-features` passes

#### 3.3 Extract shared test helpers
- **Files**: `src/parser/tests.rs`, `src/parser/tests_plsql_fixes.rs`, `src/linter/tests.rs`
- **Issue**: `parse()` and `parse_one()` duplicated verbatim across 3 test files
- **Action**: Create `src/parser/test_helpers.rs` (behind `#[cfg(test)]`); import shared helpers in dependent test files. The linter version differs slightly (returns `Vec<StatementInfo>`) — keep separate or parameterize.
- **Risk**: Low — test-only code, no production impact
- **Verification**: `cargo test --all-features` passes

#### 3.4 Split bin/ogsql.rs monolith
- **File**: `src/bin/ogsql.rs` (8,238 lines, 80+ functions)
- **Action**: Create `src/bin/cli/` directory; extract subcommand modules:
  - `parse.rs` (cmd_parse, parse output logic)
  - `format.rs` (cmd_format)
  - `validate.rs` (cmd_validate with lint/strict)
  - `extract_sql.rs` (dynamic SQL extraction)
  - `csv.rs` (CSV output utilities)
  - `common.rs` (shared CLI utilities)
- **Risk**: Medium — refactoring the main binary; must preserve exact behavior
- **Verification**: All CLI commands produce identical output before/after; existing tests pass

#### 3.5 Split analyzer/mod.rs
- **File**: `src/analyzer/mod.rs` (2,425 lines)
- **Action**: Extract sub-modules: `dynamic_sql.rs`, `fingerprints.rs`, `merge.rs`, `transactions.rs`, `package.rs`
- **Risk**: Medium — module boundary changes affect import paths
- **Verification**: `cargo check --all-features` + `cargo test --all-features` pass

#### 3.6 Add cargo-deny configuration
- **Files to create**: `deny.toml` with license allowlist, advisory DB, ban list
- **CI change**: Add `embarkstudios/cargo-deny-action@v2` job to ci.yml
- **Risk**: Low — advisory only initially (use `deny = "warn"`)
- **Verification**: `cargo deny check` passes

#### 3.7 Consolidate plan directories
- **Action**: Move `docs/plans/` contents to `.sisyphus/plans/`; delete `docs/plans/`
- **Why**: Two directories serving the same purpose; `.sisyphus/` is the more canonical location (used by active tooling)
- **Risk**: Low — all files are markdown, no code references them
- **Verification**: `docs/plans/` no longer exists; `.sisyphus/plans/` has all files

#### 3.8 Delete stale visitor spec v1
- **File**: `docs/ogsql-parser-visitor-enhancement-spec.md`
- **Why**: Superseded by `docs/ogsql-parser-visitor-enhancement-spec-v2.md` (same commit, `034d133`)
- **Risk**: Zero
- **Verification**: v2 still exists as single source of truth

### Phase 4: Low Priority (6 items)

#### 4.1 Fix filename typo
- **Rename**: `docs/BEST-PRATICE.md` → `docs/BEST-PRACTICE.md`
- **Risk**: Zero — search for references first

#### 4.2 Merge testcases/ into tests/fixtures/
- **Move**: `testcases/Dynamic_mapper.xml`, `testcases/GaussDBDynamicSQLAdvanced.java`, `testcases/JavaVarRow1.java` → `tests/fixtures/`
- **Risk**: Low — test files, adjust any path references

#### 4.3 Add CODE_OF_CONDUCT.md, SECURITY.md, .github/dependabot.yml
- **Risk**: Zero — additive community files

#### 4.4 Add .gitattributes for linguist overrides
- **Purpose**: Prevent 57MB of SQL/JSON vendor data from skewing GitHub language stats
- **Risk**: Zero

#### 4.5 Migrate mod.rs to new-style naming
- **Priority**: Start with `parser/utility/mod.rs` (4 lines — trivial)
- **Risk**: Low per-file but broad scope (12 files)

#### 4.6 Unify test placement convention
- **Action**: Document preferred pattern in CONTRIBUTING.md; apply to new code; refactor existing incrementally

---

## Verification Gates (MUST PASS after each phase)

```bash
cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings
cargo test --all-features
```

## Out of Scope

- Workspace split (separate investigation needed)
- `unwrap()` cleanup campaign (separate effort)
- Crate API design changes
- Test coverage improvements
- Performance optimization
