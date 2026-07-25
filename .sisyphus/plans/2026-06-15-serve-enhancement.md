# Serve HTTP API Enhancement Plan

> **Goal:** Enhance serve command with proper error handling, body limits, tracing, lint config,
> missing endpoints, typed responses, and Swagger UI — all compliant with M-ARCH-03 (≤600 lines/file).

---

## Current State

The `api` module lives inline in `src/bin/ogsql.rs` (lines 4830–5411, ~580 lines). Adding 7
enhancements will exceed M-ARCH-03's 600-line limit. **Must extract to a dedicated directory.**

### Existing Routes (8)

| Method | Path | Handler | Feature |
|--------|------|---------|---------|
| GET | `/api/health` | `health` | serve |
| POST | `/api/parse` | `handle_parse` | serve |
| POST | `/api/json2sql` | `handle_json2sql` | serve |
| POST | `/api/format` | `handle_format` | serve |
| POST | `/api/tokenize` | `handle_tokenize` | serve |
| POST | `/api/validate` | `handle_validate` | serve |
| GET | `/api-docs/openapi.json` | `openapi_spec` | serve |
| POST | `/api/parse-xml` | `handle_parse_xml` | ibatis |
| POST | `/api/parse-java` | `handle_parse_java` | java |

### Problems Being Fixed

| # | Problem | Enhancement |
|---|---------|-------------|
| 2 | All errors return HTTP 200 | Typed `ApiError` → proper 4xx/5xx |
| 3 | No body size limit | `DefaultBodyLimit::max(10MB)` |
| 4 | No tracing/logging | `tracing` + `TraceLayer` + request_id |
| 5 | Lint config hardcoded | `LintConfigInput` accepted in requests |
| 7 | Missing validate-xml/java | New endpoints mirroring CLI |
| 8 | Responses are `serde_json::Value` | Typed response structs with `ToSchema` |
| 9 | No Swagger UI | `utoipa-swagger-ui` at `/api-docs/swagger-ui/` |

---

## Architecture: Module Extraction

Extract the inline `mod api` into a dedicated directory tree:

```
src/bin/
├── ogsql.rs              # CLI entry — replace `mod api` with `mod serve;`
└── serve/
    ├── mod.rs             # Router, middleware stack, server bootstrap
    ├── error.rs           # ApiError enum + IntoResponse impl
    ├── schema.rs          # All request/response structs (ToSchema)
    ├── handlers.rs        # All handler functions
    └── openapi.rs         # ApiDoc structs + build_openapi()
```

**Module references:** From `serve/handlers.rs`, parent-scope helpers in `ogsql.rs` are reached
via `crate::` (e.g., `crate::parse_input`, `crate::filter_output_by_procedure`). This preserves
existing `super::` semantics — `super::` refers to the `serve` module, `crate::` to the binary root.

### Estimated File Sizes

| File | Est. Lines | Purpose |
|------|-----------|---------|
| `serve/mod.rs` | ~120 | Router, middleware, `start_server()` |
| `serve/error.rs` | ~100 | `ApiError`, `IntoResponse`, `ApiResponse` |
| `serve/schema.rs` | ~350 | Request/response types |
| `serve/handlers.rs` | ~500 | 10 handler functions |
| `serve/openapi.rs` | ~80 | OpenAPI doc configuration |

All under 600 lines. ✓

---

## Dependency Changes (Cargo.toml)

```toml
# Modified — add "trace" and "request-id" features
tower-http = { version = "0.6", features = ["cors", "trace", "request-id"], optional = true }

# New (all feature-gated under serve)
tracing = { version = "0.1", optional = true }
tracing-subscriber = { version = "0.3", features = ["json", "env-filter"], optional = true }
utoipa-swagger-ui = { version = "9", features = ["axum"], optional = true }

# serve feature updated
serve = ["cli", "dep:axum", "dep:tokio", "dep:tower-http", "dep:utoipa",
         "dep:tracing", "dep:tracing-subscriber", "dep:utoipa-swagger-ui"]
```

**Middleware layer order** (ServiceBuilder: first added = outermost = executes first on request):

```
Request → SetRequestIdLayer → PropagateRequestIdLayer → TraceLayer → CorsLayer → DefaultBodyLimit → Handler
```

---

## Task Breakdown

### Task 1: Cargo.toml dependency update

**File:** `Cargo.toml`

- Add `tracing`, `tracing-subscriber` (with json + env-filter features)
- Add `utoipa-swagger-ui` (with axum feature)
- Update `tower-http` features: `["cors"]` → `["cors", "trace"]`
- Update `serve` feature list to include new deps

**Verify:**
```bash
cargo check --features serve 2>&1 | tail -5
```

---

### Task 2: Extract api module → src/bin/serve/ directory

**Files:**
- Create: `src/bin/serve/mod.rs`
- Create: `src/bin/serve/error.rs`
- Create: `src/bin/serve/schema.rs`
- Create: `src/bin/serve/handlers.rs`
- Create: `src/bin/serve/openapi.rs`
- Modify: `src/bin/ogsql.rs` — remove inline `mod api { ... }`, add `mod serve;`

**Step 1:** Create `serve/schema.rs` — move all `*Input` structs from ogsql.rs api module.

**Step 2:** Create `serve/handlers.rs` — move all handler functions.
- Replace `super::` references with `crate::` (e.g., `super::parse_input` → `crate::parse_input`)
- Exception: `super::` for intra-module references stays (e.g., `super::schema::ParseInput`)

**Step 3:** Create `serve/openapi.rs` — move `ApiDoc`, `ApiDocIbatis`, `ApiDocJava`, `build_openapi`.

**Step 4:** Create `serve/mod.rs` — module declarations + `pub fn router()` re-export.

**Step 5:** Create `serve/error.rs` — (placeholder, filled in Task 3).

**Step 6:** In `ogsql.rs`:
- Delete the entire `#[cfg(feature = "serve")] mod api { ... }` block (lines 4829–5411)
- Replace with `#[cfg(feature = "serve")] mod serve;`
- Update the call site: `api::router()` → `serve::router()`

**Verify:**
```bash
cargo check --features full 2>&1 | tail -10
cargo clippy --features serve -- -D warnings 2>&1 | tail -10
```

---

### Task 3: Item 2 — Typed ApiError with HTTP status codes

**File:** `src/bin/serve/error.rs`

Define a typed error that maps to proper HTTP status codes:

```rust
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

/// API error response body
#[derive(Debug, Serialize, ToSchema)]
#[non_exhaustive]
pub struct ApiErrorBody {
    /// Error type identifier
    pub error: String,
    /// Human-readable error message
    pub message: String,
}

/// Typed API error with proper HTTP status code mapping
#[derive(Debug)]
#[non_exhaustive]
pub enum ApiError {
    /// 400 — malformed request body (invalid JSON, missing required field)
    BadRequest(String),
    /// 422 — SQL tokenization or parsing failed
    UnprocessableEntity(String),
    /// 404 — referenced resource not found (e.g., schema_json file)
    NotFound(String),
    /// 500 — internal server error (serialization failure, etc.)
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg),
            ApiError::UnprocessableEntity(msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "unprocessable_entity", msg)
            }
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg),
            ApiError::Internal(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", msg)
            }
        };
        let body = ApiErrorBody { error: error_type.to_string(), message };
        (status, Json(body)).into_response()
    }
}
```

**Handler changes:** Replace `return Json(json!({"error": ...}))` patterns with
`return Err(ApiError::BadRequest(...))`.

Handlers change return type from `Json<serde_json::Value>` to
`Result<Json<TypedResponse>, ApiError>`.

---

### Task 4: Item 8 — Typed response structs

**File:** `src/bin/serve/schema.rs`

Define typed responses for each endpoint. Strategy: **hybrid** — simple endpoints get fully typed
responses; complex endpoints (parse/validate) get typed envelopes with `serde_json::Value` for
AST data (AST types are too complex to add `ToSchema` to).

```rust
/// GET /api/health response
#[derive(Debug, Serialize, ToSchema)]
#[non_exhaustive]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// POST /api/format response
#[derive(Debug, Serialize, ToSchema)]
#[non_exhaustive]
pub struct FormatResponse {
    pub formatted: String,
}

/// POST /api/tokenize response
#[derive(Debug, Serialize, ToSchema)]
#[non_exhaustive]
pub struct TokenizeResponse {
    pub tokens: Vec<TokenInfo>,
}

/// POST /api/json2sql response
#[derive(Debug, Serialize, ToSchema)]
#[non_exhaustive]
pub struct Json2SqlResponse {
    pub statements: Vec<String>,
    pub count: usize,
}

/// POST /api/parse response — typed envelope
/// AST body stays as serde_json::Value (too complex for ToSchema)
#[derive(Debug, Serialize, ToSchema)]
#[non_exhaustive]
pub struct ParseResponse {
    pub statements: Vec<serde_json::Value>,
    pub errors: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_fingerprints: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lint_warnings: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lint_summary: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted_sql: Option<Vec<serde_json::Value>>,
}

/// POST /api/validate response
#[derive(Debug, Serialize, ToSchema)]
#[non_exhaustive]
pub struct ValidateResponse {
    pub valid: bool,
    pub error_count: usize,
    pub warning_count: usize,
    pub errors: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undefined_variables: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_consistency_errors: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lint_warnings: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lint_summary: Option<serde_json::Value>,
}
```

**TokenInfo** (already exists in ogsql.rs, move to schema.rs):
```rust
#[derive(Debug, Serialize, ToSchema)]
pub struct TokenInfo {
    #[serde(rename = "type")]
    pub token_type: String,
    pub value: String,
    pub line: usize,
    pub column: usize,
}
```

---

### Task 5: Item 5 — LintConfigInput for customizable lint

**File:** `src/bin/serve/schema.rs`

```rust
/// Lint configuration accepted by parse/validate endpoints
#[derive(Debug, Default, Deserialize, ToSchema)]
#[non_exhaustive]
pub struct LintConfigInput {
    /// Minimum warning level: prohibition, performance, caution, suggestion
    #[serde(default)]
    pub min_level: Option<String>,
    /// Minimum confidence: full, partial
    #[serde(default)]
    pub min_confidence: Option<String>,
    /// Suppress specific rule IDs
    #[serde(default)]
    pub suppress: Vec<String>,
    /// P003 IN list size threshold (default: 500)
    #[serde(default)]
    pub in_list_threshold: Option<usize>,
    /// P014 subquery nesting depth limit (default: 3)
    #[serde(default)]
    pub subquery_depth_limit: Option<usize>,
    /// P007 non-equi join count limit (default: 2)
    #[serde(default)]
    pub non_equi_join_limit: Option<usize>,
}

impl LintConfigInput {
    /// Convert to LintConfig, starting from defaults
    pub fn to_lint_config(&self) -> ogsql_parser::linter::LintConfig {
        let mut config = ogsql_parser::linter::LintConfig::default();
        // ... map fields, same logic as build_lint_config() in ogsql.rs
        config
    }
}
```

**Update ParseInput / ValidateInput** to accept optional lint config:

```rust
pub struct ParseInput {
    pub sql: String,
    // ... existing fields ...
    /// Lint configuration (overrides defaults)
    #[serde(default)]
    pub lint_config: Option<LintConfigInput>,
}
```

**Update handlers** to use `lint_config` instead of `LintConfig::default()`:

```rust
let config = input.lint_config
    .as_ref()
    .map(|c| c.to_lint_config())
    .unwrap_or_default();
```

---

### Task 6: Item 7 — validate-xml and validate-java endpoints

**File:** `src/bin/serve/schema.rs` + `src/bin/serve/handlers.rs`

#### validate-xml (feature = "ibatis")

```rust
/// POST /api/validate-xml request body
#[cfg(feature = "ibatis")]
#[derive(Deserialize, ToSchema)]
#[non_exhaustive]
pub struct ValidateXmlInput {
    pub xml: String,
    #[cfg(feature = "java")]
    #[serde(default)]
    pub java_src: Option<String>,
    #[serde(default)]
    pub strict: Option<bool>,
    #[serde(default)]
    pub lint: Option<bool>,
    #[serde(default)]
    pub lint_config: Option<LintConfigInput>,
}

/// POST /api/validate-xml response
#[cfg(feature = "ibatis")]
#[derive(Serialize, ToSchema)]
#[non_exhaustive]
pub struct ValidateXmlResponse {
    pub valid: bool,
    pub error_count: usize,
    pub warning_count: usize,
    pub errors: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lint_warnings: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lint_summary: Option<serde_json::Value>,
}
```

Handler mirrors CLI `cmd_validate_xml`: parse XML → semantic checks → lint.

#### validate-java (feature = "java")

```rust
/// POST /api/validate-java request body
#[cfg(feature = "java")]
#[derive(Deserialize, ToSchema)]
#[non_exhaustive]
pub struct ValidateJavaInput {
    pub source: String,
    #[serde(default)]
    pub extra_sql_methods: Option<Vec<String>>,
    #[serde(default)]
    pub extra_sql_var_patterns: Option<Vec<String>>,
    #[serde(default)]
    pub strict: Option<bool>,
    #[serde(default)]
    pub lint: Option<bool>,
    #[serde(default)]
    pub lint_config: Option<LintConfigInput>,
}
```

Handler mirrors CLI `cmd_validate_java`: extract SQL → parse → semantic checks → lint.

---

### Task 7: Item 4 — Tracing + TraceLayer + request_id

**File:** `src/bin/serve/mod.rs` + `src/bin/ogsql.rs` (serve command init)

#### 7a. Tracing initialization (in serve command dispatch)

```rust
Commands::Serve { port, host } => {
    // Initialize tracing with JSON format (M-LOG-02)
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ogsql=info,tower_http=info".into())
        )
        .json()
        .with_target(true)
        .init();

    // ... rest of server startup
}
```

#### 7b. Middleware stack (in serve/mod.rs router function)

```rust
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use axum::extract::DefaultBodyLimit;

pub fn router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let mut router = Router::new()
        .route("/api/health", get(handlers::health))
        .route("/api/parse", post(handlers::handle_parse))
        // ... all routes ...
        .route("/api-docs/openapi.json", get(openapi::openapi_spec))
        .layer((
            SetRequestIdLayer::x_request_id(MakeRequestUuid),
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let request_id = request.headers()
                        .get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("-");
                    tracing::info_span!("http_request",
                        request_id = %request_id,
                        method = %request.method(),
                        uri = %request.uri(),
                    )
                }),
            PropagateRequestIdLayer::x_request_id(),
        ))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))  // 10 MB
        .layer(cors);

    // Feature-gated routes
    #[cfg(feature = "ibatis")]
    {
        router = router
            .route("/api/parse-xml", post(handlers::handle_parse_xml))
            .route("/api/validate-xml", post(handlers::handle_validate_xml));
    }
    #[cfg(feature = "java")]
    {
        router = router
            .route("/api/parse-java", post(handlers::handle_parse_java))
            .route("/api/validate-java", post(handlers::handle_validate_java));
    }

    router
}
```

---

### Task 8: Item 3 — Body limit

Already included in Task 7's middleware stack:
```rust
.layer(DefaultBodyLimit::max(10 * 1024 * 1024))  // 10 MB
```

---

### Task 9: Item 9 — Swagger UI

**File:** `src/bin/serve/mod.rs`

```rust
use utoipa_swagger_ui::SwaggerUi;

pub fn router() -> Router {
    let openapi = openapi::build_openapi();

    let router = Router::new()
        // ... API routes ...
        .merge(SwaggerUi::new("/api-docs/swagger-ui/*")
            .url("/api-docs/openapi.json", openapi))
        // ... middleware layers ...
    ;
    router
}
```

**Note:** The existing `/api-docs/openapi.json` GET handler is replaced by SwaggerUi's built-in
`.url()` method which serves the spec at that path.

---

### Task 10: Update OpenAPI docs

**File:** `src/bin/serve/openapi.rs`

Update all `#[derive(OpenApi)]` structs to include:
1. New endpoints (validate-xml, validate-java)
2. Response schemas (all new `*Response` types)
3. Error schema (`ApiErrorBody`)

```rust
#[derive(OpenApi)]
#[openapi(
    paths(
        health, handle_parse, handle_format, handle_tokenize,
        handle_validate, handle_json2sql,
    ),
    components(
        schemas(
            ParseInput, FormatInput, ValidateInput, TokenizeInput, JsonInput,
            LintConfigInput,
            HealthResponse, ParseResponse, FormatResponse, TokenizeResponse,
            ValidateResponse, Json2SqlResponse,
            ApiErrorBody,
        )
    ),
    tags((name = "ogsql", description = "openGauss/GaussDB SQL Parser API"))
)]
pub struct ApiDoc;
```

---

### Task 11: Full verification

```bash
# Format
cargo fmt --all -- --check

# Clippy
cargo clippy --features serve -- -D warnings
cargo clippy --features full -- -D warnings

# Tests
cargo test --features serve
cargo test --features full

# Manual smoke test
cargo run --features full -- serve --port 8765 &
# health
curl -s http://127.0.0.1:8765/api/health | jq .
# Swagger UI
curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:8765/api-docs/swagger-ui/
# parse with lint_config
curl -s -X POST http://127.0.0.1:8765/api/parse \
  -H 'Content-Type: application/json' \
  -d '{"sql":"SELECT * FROM t","lint":true,"lint_config":{"min_level":"suggestion"}}' | jq .
# error case (bad JSON)
curl -s -o /dev/null -w "%{http_code}" -X POST http://127.0.0.1:8765/api/parse \
  -H 'Content-Type: application/json' -d 'not json'
# Expected: 400
```

---

### Task 12: Update docs/user-guide.md

- Add validate-xml, validate-java endpoints to the API table
- Add Swagger UI URL
- Document `lint_config` parameter
- Document error response format (HTTP status codes)

---

## Dependency Graph

```
Task 1 (Cargo.toml)
  └── Task 2 (module extraction)
       ├── Task 3 (ApiError)
       │    └── Task 4 (typed responses) — needs error type for Result returns
       ├── Task 5 (lint config)
       ├── Task 6 (validate-xml/java)
       ├── Task 7 (tracing) + Task 8 (body limit) + Task 9 (Swagger UI)
       └── Task 10 (OpenAPI update)
            └── Task 11 (verification)
                 └── Task 12 (docs)
```

Tasks 3, 5, 6 can proceed in parallel after Task 2 (module extraction).
Tasks 7, 8, 9 are independent and can also proceed in parallel.

---

## Risk Mitigations

| Risk | Mitigation |
|------|-----------|
| `utoipa-swagger-ui` version mismatch with utoipa 5 | Verified via librarian research; fallback: embed static HTML |
| Module extraction breaks `super::` references | Systematic `super::` → `crate::` replacement; compile after each file |
| Typed responses break existing API consumers | Response structs use `#[non_exhaustive]` + same JSON field names |
| `tower-http` trace feature conflicts with cors | Both are independent layers; order: cors → trace → body limit |
| Handler refactoring introduces bugs | Incremental extraction: move first, then enhance |
