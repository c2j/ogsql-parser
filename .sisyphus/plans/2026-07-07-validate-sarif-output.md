# Plan: SARIF Output for /api/validate, /api/validate-xml, /api/validate-java

## Summary

Add an optional `format: "sarif"` request-body field to **all three validate endpoints**. When set, the response is `application/sarif+json` (SARIF 2.1.0) instead of the current custom JSON. The existing JSON format is completely preserved — SARIF is strictly opt-in.

## Motivation

- GitHub Code Scanning native integration (upload-sarif action → inline PR annotations)
- VS Code / IntelliJ SARIF viewer support
- Standardized severity levels for 53 linter rules + parser errors
- Self-documenting output (SARIF embeds rule metadata in `tool.driver.rules[]`)

## Non-Goals (out of scope)

- Streaming/large-file optimization
- SARIF automation details extension

---

## Architecture

```
                           ┌───────────────────┐
                           │  build_sarif_log() │ ← ONE builder, three callers
                           │  (src/bin/serve/   │
                           │   sarif.rs)        │
                           └────────┬──────────┘
                                    │
        ┌───────────────────────────┼───────────────────────────┐
        ▼                           ▼                           ▼
┌───────────────┐     ┌─────────────────────┐     ┌───────────────────┐
│ do_validate() │     │ do_validate_xml()   │     │ do_validate_java()│
│ (handlers.rs) │     │ (handlers.rs)       │     │ (handlers.rs)     │
├───────────────┤     ├─────────────────────┤     ├───────────────────┤
│ input: SQL    │     │ input: XML          │     │ input: Java src   │
│ → ParseInput  │     │ → iBatis parser     │     │ → Java extractor  │
│ → validate    │     │ → validate          │     │ → validate        │
│ → Response    │     │ → Response          │     │ → Response        │
└───────────────┘     └─────────────────────┘     └───────────────────┘
```

Three endpoints, same `ValidateResponse` shape, **one shared SARIF builder**.

---

## Implementation Plan (7 steps)

### Step 0 — Read plan documents for full context

**Files:** `docs/plans/2026-06-10-serve-api-gap-fill.md`, `docs/plans/2026-06-15-serve-enhancement.md`

Both prior serve enhancement plans exist in `docs/plans/`. Read them to ensure the SARIF change aligns with existing serve architecture patterns and doesn't conflict with ongoing work.

**Verification:** No architectural conflicts identified.

---

### Step 1 — Upstream `is_warning` to `lib.rs`

**Files:** `src/lib.rs` (add), `src/bin/ogsql.rs` (delegate)

Move the 4-line `is_warning` helper from the binary crate into the library crate so the SARIF module (which lives in `src/bin/serve/`) can call it without reaching into the binary crate.

```rust
// lib.rs — add
pub fn is_warning(e: &ParserError) -> bool {
    matches!(e, ParserError::Warning { .. } | ParserError::ReservedKeywordAsIdentifier { .. })
}
```

Update `src/bin/ogsql.rs` to call `ogsql_parser::is_warning(e)` instead of local `is_warning(e)`.

**Verification:** `cargo build --all-features` compiles cleanly.

---

### Step 2 — Add `description` field to `LintRuleEntry`

**Files:** `src/linter/mod.rs` (struct), `src/linter/rules_prohibition.rs`, `src/linter/rules_performance.rs`, `src/linter/rules_caution.rs`, `src/linter/rules_suggestion.rs` (registrations)

Add `pub description: &'static str` to `LintRuleEntry`. Populate with one short English sentence per rule (53 total).

Data sources for descriptions:
- `// RXXX: ...` inline comments in each `rules_*.rs` file
- `docs/ecosystem-rule-mapping.md` for cross-linter descriptions

```rust
// mod.rs
pub struct LintRuleEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,   // NEW
    pub level: WarningLevel,
    pub stmt_kind: StatementKind,
    pub check_fn: ...,
}
```

All existing call sites that construct `LintRuleEntry { .. }` must add `description:`.

**Verification:** `cargo build` + grep assertion `LintRuleEntry { id: "..", description: $STRING }` confirms all 53 entries have non-empty description.

---

### Step 3 — Create SARIF type definitions + builder

**New file:** `src/bin/serve/sarif.rs`

Define SARIF 2.1.0 subset as `#[derive(Serialize)]` structs (~15 types):

```
SarifLog { version, runs }
Run { tool, artifacts?, results }
Tool { driver }
Driver { name, version, information_uri, rules }
Artifact { location, description }
ArtifactLocation { uri }
ReportingDescriptor { id, name, short_description, full_description, default_configuration }
Result { rule_id, rule_index, level, message, locations? }
Location { physical_location?, logical_locations? }
PhysicalLocation { artifact_location, region }
Region { start_line, start_column?, byte_offset? }
Message { text }
ReportingConfiguration { level }
```

**Builder function** (shared across all three endpoints):

```rust
pub fn build_sarif_log(
    response: &ValidateResponse,      // Shared output type
    source_text: &str,                // Original input (SQL / XML / Java)
    rules: &[LintRuleEntry],          // 53 rule catalog
    tool_name: &str,                  // "ogsql-parser"
    tool_version: &str,              // env!("CARGO_PKG_VERSION")
) -> SarifLog
```

**Severity mappings:**

| Source | SARIF level |
|---|---|
| `ParserError` where `is_warning() == false` | `error` |
| `ParserError::Warning { level }` | `Prohibition → error`, `Performance → warning`, `Caution → note` |
| `ParserError::ReservedKeywordAsIdentifier` | `warning` |
| `SqlWarning.level` | `Prohibition → error`, `Performance → warning`, `Caution → note`, `Suggestion → none` |

**Artifact URI scheme:**

| Endpoint | `artifactLocation.uri` | Sent to builder as |
|---|---|---|
| `/api/validate` | `"api://validate/sql"` | `source_text = input.sql` |
| `/api/validate-xml` | `"api://validate-xml/mapper.xml"` | `source_text = input.xml` |
| `/api/validate-java` | `"api://validate-java/MyClass.java"` | `source_text = input.source` |

**Edge case: `TokenizerError` only carries byte `position`, not line/column.** Build a helper `byte_offset_to_line_col(source: &str, offset: usize) -> (usize, usize)` using `source[..offset].chars().filter(|c| *c == '\n').count() + 1` for line, and compute column from the last newline position.

**Per-statement validation (validate-xml / validate-java):**
`Response.statements: Vec<StatementValidation>` maps to SARIF as:

```
result.locations[0].physicalLocation.artifactLocation.uri  → "api://validate-xml/mapper.xml"
result.locations[0].physicalLocation.region.startLine      → StatementValidation.line
result.properties["method"]                                → StatementValidation.method  
result.properties["sqlType"]                               → StatementValidation.sql_type
result.properties["extractedSql"]                          → StatementValidation.sql
```

**Verification:** Unit test with a minimal `ValidateResponse` containing 1 lint warning + 1 parse error serializes to valid SARIF JSON that passes `python -c "import json; json.loads(...)"` structural check.

---

### Step 4 — Add `format` field to all three Input structs

**File:** `src/bin/serve/schema.rs`

Three structs all get the same field:

```rust
pub struct ValidateInput {
    // ... existing fields ...
    #[serde(default)]
    pub format: Option<String>,    // NEW: "sarif" → SARIF 2.1.0
}

pub struct ValidateXmlInput {
    // ... existing fields ...
    #[serde(default)]
    pub format: Option<String>,    // NEW
}

pub struct ValidateJavaInput {
    // ... existing fields ...
    #[serde(default)]
    pub format: Option<String>,    // NEW
}
```

All three already have `#[non_exhaustive]` — non-breaking for all callers.

**Verification:** Deserialize test: `{"sql": "...", "format": "sarif"}` → `ValidateInput.format == Some("sarif")`.

---

### Step 5 — Branch in all three validate handlers

**File:** `src/bin/serve/handlers.rs`

Change all three `do_*` functions to return `Result<axum::response::Response, ApiError>` instead of `Result<Json<ValidateResponse>, ApiError>`.

**`do_validate` (line 277):**

```rust
fn do_validate(input: ValidateInput) -> Result<Response, ApiError> {
    let response = build_validate_response(&input)?;
    maybe_sarif_response(response, &input.sql, input.format.as_deref())
}
```

**`do_validate_xml` (line 498):**

```rust
fn do_validate_xml(input: ValidateXmlInput) -> Result<Response, ApiError> {
    let response = build_xml_validate_response(&input)?;
    maybe_sarif_response(response, &input.xml, input.format.as_deref())
}
```

**`do_validate_java` (line 649):**

```rust
fn do_validate_java(input: ValidateJavaInput) -> Result<Response, ApiError> {
    let response = build_java_validate_response(&input)?;
    maybe_sarif_response(response, &input.source, input.format.as_deref())
}
```

**Shared dispatch helper:**

```rust
fn maybe_sarif_response(
    response: ValidateResponse,
    source_text: &str,
    format: Option<&str>,
) -> Result<Response, ApiError> {
    match format {
        Some("sarif") => {
            let rules = ogsql_parser::linter::SqlLinter::all_rules_metadata();
            let sarif = sarif::build_sarif_log(
                &response, source_text, &rules,
                "ogsql-parser", env!("CARGO_PKG_VERSION"),
            );
            Ok(Response::builder()
                .header(CONTENT_TYPE, "application/sarif+json")
                .body(Body::from(serde_json::to_string(&sarif)?))
                .unwrap())
        }
        _ => Ok(Json(response).into_response()),
    }
}
```

The three `handle_*` wrapper functions (which do multipart-vs-JSON dispatch) continue to call the `do_*` functions as before — only the return type widens. The `Result<Response, ApiError>` is compatible with axum's `IntoResponse` for both `Json<T>` and `Response`.

**Verification:** Three separate curl tests confirm each endpoint with `format: "sarif"` returns `Content-Type: application/sarif+json`.

---

### Step 6 — Register all SARIF endpoint variants in OpenAPI documentation

**Files:** `src/bin/serve/openapi.rs`, `src/bin/serve/handlers.rs` (utoipa annotations)

1. Register SARIF schema types (`SarifLog`, `SarifRun`, `SarifResult`, etc.) in `ApiDoc::components(schemas(...))`.

2. Update `#[utoipa::path]` on all three validate handlers:

```rust
// handle_validate
#[utoipa::path(
    post,
    path = "/api/validate",
    tag = "ogsql",
    request_body = ValidateInput,
    responses(
        (status = 200, description = "Validation result (JSON)", body = ValidateResponse),
        (status = 200, description = "Validation result (SARIF 2.1.0)", body = SarifLog, content_type = "application/sarif+json"),
        (status = 400, description = "Invalid request", body = ApiErrorBody),
    )
)]

// handle_validate_xml (same pattern, path = "/api/validate-xml")
// handle_validate_java (same pattern, path = "/api/validate-java")
```

**Verification:** `/api-docs/openapi.json` lists three SARIF response variants; Swagger UI shows them.

---

### Step 7 — Expose rule catalog from linter module

**Files:** `src/linter/mod.rs` (new function)

The SARIF builder needs access to all 53 `LintRuleEntry`s. Currently `SqlLinter::with_default_rules(config)` takes a config and returns a linter. Add a new direct function:

```rust
// src/linter/mod.rs
/// Return all rule metadata without constructing a full linter.
/// Useful for building SARIF rule catalog.
pub fn all_rules_metadata() -> Vec<LintRuleEntry>
```

This calls the four `register()` functions with a dummy collector, or simply returns a static slice.

**Verification:** `cargo test` passes; `all_rules_metadata().len() == 53`.

---

## Verification Checklist (before merge)

- [ ] `cargo fmt --all && cargo fmt --all -- --check` passes
- [ ] `cargo clippy --all-features -- -D warnings` passes
- [ ] `cargo test --all-features` passes (all 1772+ tests)
- [ ] `/api/validate` without `format` returns current JSON (backward compat)
- [ ] `/api/validate` with `format: "sarif"` returns `Content-Type: application/sarif+json`
- [ ] `/api/validate-xml` with `format: "sarif"` returns valid SARIF with `properties.method` per statement
- [ ] `/api/validate-java` with `format: "sarif"` returns valid SARIF with `properties.extractedSql` per statement
- [ ] SARIF output parses with `python -c "import json; json.loads(sys.stdin.read())"`
- [ ] `/api-docs/openapi.json` includes SarifLog schema

## Rollback Strategy

SARIF output is purely additive behind a `format: "sarif"` request field. If issues arise:
1. Remove `format: Option<String>` from the three Input structs
2. Remove `sarif.rs` and `maybe_sarif_response` helper
3. Revert `do_*` return types to `Result<Json<ValidateResponse>, ApiError>`
4. Leave Step 1 (is_warning) and Step 2 (LintRuleEntry.description) — they're independent quality improvements
5. Leave Step 7 (all_rules_metadata) — useful for other consumers
