# Serve API 能力补齐 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 将 serve HTTP API 补全至与 CLI 命令和 MCP Server 一致的能力水平，同时按端点拆分 Input 结构体。

**Architecture:** 所有分析函数已是库公开 API，serve handler 只需串管线调用。按端点拆分为 `ParseInput` / `FormatInput` / `ValidateInput` / `TokenizeInput`，新增 feature-gated 的 `ParseXmlInput` / `ParseJavaInput`。OpenAPI spec 通过多 `ApiDoc` struct + merge 方式支持条件编译。

**Tech Stack:** Rust, axum 0.7, utoipa 5, serde, ogsql_parser 库内分析函数

---

### 前置验证：当前 benchmark

记录修改前状态，后续验证对比用：

```bash
cargo check --features serve 2>&1 | tail -20
cargo clippy --features serve -- -D warnings 2>&1 | tail -20
```

---

### Task 1: 拆分 Input 结构体 + 新增字段

**Files:**
- Modify: `src/bin/ogsql.rs:4719-4749` (替换 SqlInput 为 4 个 per-endpoint struct)

**操作：** 删除 `SqlInput`，新增以下 struct：

```rust
/// POST /api/parse request body
#[derive(Deserialize, ToSchema)]
pub struct ParseInput {
    pub sql: String,
    #[serde(default)]
    pub preserve_comments: bool,
    /// Enable MyBatis #{param} and ${expr} placeholder support
    #[serde(default)]
    pub mybatis: bool,
    /// Only parse the specified stored procedure/function
    #[serde(default)]
    pub procedure: Option<String>,
    /// Extract SQL statements from stored procedures (one row per SQL; variables → __SQL_PARAM_Type_Name__)
    #[serde(default)]
    pub extract_sql: bool,
    /// Enable SQL anti-pattern linting
    #[serde(default)]
    pub lint: Option<bool>,
    /// Path to schema JSON file for schema-aware lint and analysis
    #[serde(default)]
    pub schema_json: Option<String>,
}

/// POST /api/format request body
#[derive(Deserialize, ToSchema)]
pub struct FormatInput {
    pub sql: String,
    #[serde(default)]
    pub indent: Option<usize>,
    #[serde(default)]
    pub keyword_case: Option<String>,
    #[serde(default)]
    pub comma_style: Option<String>,
    #[serde(default)]
    pub line_width: Option<usize>,
    #[serde(default)]
    pub uppercase: Option<bool>,
    /// Enable MyBatis #{param} and ${expr} placeholder support (preserves during formatting)
    #[serde(default)]
    pub mybatis: bool,
    /// Don't put each SELECT column on its own line
    #[serde(default)]
    pub no_select_newline: Option<bool>,
    /// Don't put AND/OR on new lines
    #[serde(default)]
    pub no_logical_newline: Option<bool>,
    /// Don't put semicolons on their own line
    #[serde(default)]
    pub no_semicolon_newline: Option<bool>,
}

/// POST /api/validate request body
#[derive(Deserialize, ToSchema)]
pub struct ValidateInput {
    pub sql: String,
    /// Enable MyBatis #{param} and ${expr} placeholder support
    #[serde(default)]
    pub mybatis: bool,
    /// Enable strict mode: detect undefined function calls in PL blocks
    #[serde(default)]
    pub strict: Option<bool>,
    /// Enable SQL anti-pattern linting
    #[serde(default)]
    pub lint: Option<bool>,
    /// Path to schema JSON file for schema-aware lint rules
    #[serde(default)]
    pub schema_json: Option<String>,
}

/// POST /api/tokenize request body
#[derive(Deserialize, ToSchema)]
pub struct TokenizeInput {
    pub sql: String,
    #[serde(default)]
    pub preserve_comments: bool,
    /// Enable MyBatis #{param} and ${expr} placeholder support
    #[serde(default)]
    pub mybatis: bool,
}
```

`JsonInput` 保持不变。

**Step 1: Replace structs in ogsql.rs**
- 精确定位 `pub struct SqlInput {` (line 4720) 到 `}` (line 4744)
- 替换为上面 4 个 struct 定义
- 注意保留 `JsonInput` 不动

**Step 2: Update handler signatures**
- `handle_parse`: `Json(SqlInput)` → `Json(ParseInput)`
- `handle_format`: `Json(SqlInput)` → `Json(FormatInput)`
- `handle_tokenize`: `Json(SqlInput)` → `Json(TokenizeInput)`
- `handle_validate`: `Json(SqlInput)` → `Json(ValidateInput)`

**Step 3: Update utoipa #[openapi] schemas**

```rust
#[derive(OpenApi)]
#[openapi(
    paths(health, handle_parse, handle_format, handle_tokenize, handle_validate, handle_json2sql),
    components(schemas(ParseInput, FormatInput, ValidateInput, TokenizeInput, JsonInput)),
    tags((name = "ogsql", description = "openGauss/GaussDB SQL Parser API"))
)]
pub struct ApiDoc;
```

**Step 4: Update utoipa #[utoipa::path] request_body**
- `handle_parse`: `request_body = SqlInput` → `request_body = ParseInput`
- `handle_format`: `request_body = SqlInput` → `request_body = FormatInput`
- `handle_tokenize`: `request_body = SqlInput` → `request_body = TokenizeInput`
- `handle_validate`: `request_body = SqlInput` → `request_body = ValidateInput`

**Step 5: Verify**

```bash
cargo check --features serve
```

预期：由于 handler 中引用的 `input.xxx` 字段可能在新 struct 中不存在（如 `handle_format` 引用的 `input.sql`、`input.indent` 等都存在于 `FormatInput`），如果字段名一致应该编译通过。不一致的地方会在后续 Task 中修复。

---

### Task 2: Validate handler 补全 — MERGE 语义 + strict 模式

**Files:**
- Modify: `src/bin/ogsql.rs:4882-4929` (`handle_validate` 函数体)

**当前状态：** 有 pkg 一致性 + PL 变量校验。缺失：MERGE 语义校验、strict 参数未使用。

**Step 1: 补全 MERGE 语义校验**

在 `handle_validate` 中，`var_errors` 计算之后（line 4899 之前），插入：

```rust
// MERGE semantic validation (same logic as CLI validate_sql and MCP validate)
let merge_errors = ogsql_parser::validate_merge_semantics(&output.statements);
if !merge_errors.is_empty() {
    for me in &merge_errors {
        errors.push(ogsql_parser::ParserError::UnsupportedSyntax {
            location: me.location,
            syntax: "MERGE".to_string(),
            hint: super::merge_error_detail(me),
        });
    }
}
```

**Step 2: 接入 strict 参数**

将 line 4899：
```rust
let var_errors = super::validate_pl_variables_from_stmts(&output.statements, &[], false);
```
改为：
```rust
let strict = input.strict.unwrap_or(false);
let var_errors = super::validate_pl_variables_from_stmts(&output.statements, &[], strict);
```

**Step 3: 接入 mybatis 参数**

将 line 4883：
```rust
let output = super::parse_input(&input.sql, false, false);
```
改为：
```rust
let output = super::parse_input(&input.sql, false, input.mybatis);
```

**Step 4: Verify**

```bash
cargo check --features serve
```

---

### Task 3: Format handler 补全 — mybatis + 布局开关

**Files:**
- Modify: `src/bin/ogsql.rs:4805-4844` (`handle_format` 函数体)

**Step 1: 接入 mybatis**

Line 4806，tokenizer 调用改为：
```rust
let mut tokenizer = ogsql_parser::Tokenizer::new(&input.sql).preserve_comments(true);
if input.mybatis {
    tokenizer = tokenizer.mybatis_params(true);
}
let tokens = match tokenizer.tokenize() {
```

**Step 2: 接入布局开关**

在 `config` 构建（line 4840 之后，`format()` 调用之前）插入：
```rust
if input.no_select_newline == Some(true) {
    config.select_newline = false;
}
if input.no_logical_newline == Some(true) {
    config.logical_operator_newline = false;
}
if input.no_semicolon_newline == Some(true) {
    config.semicolon_newline = false;
}
```

**Step 3: Verify**

```bash
cargo check --features serve
```

---

### Task 4: Tokenize handler 补全 — preserve_comments + mybatis

**Files:**
- Modify: `src/bin/ogsql.rs:4854-4872` (`handle_tokenize` 函数体)

**Step 1: 接入 preserve_comments 和 mybatis**

替换 line 4855：
```rust
let tokens = match ogsql_parser::Tokenizer::new(&input.sql).tokenize() {
```
改为：
```rust
let mut tokenizer = ogsql_parser::Tokenizer::new(&input.sql);
if input.preserve_comments {
    tokenizer = tokenizer.preserve_comments(true);
}
if input.mybatis {
    tokenizer = tokenizer.mybatis_params(true);
}
let tokens = match tokenizer.tokenize() {
```

**Step 2: Verify**

```bash
cargo check --features serve
```

---

### Task 5: Parse handler 补全 — 语义分析 + extract_sql

**Files:**
- Modify: `src/bin/ogsql.rs:4770-4795` (`handle_parse` 函数体)

**Step 1: 接入 mybatis**

Line 4771：
```rust
let output = super::parse_input(&input.sql, input.preserve_comments, false);
```
改为：
```rust
let output = super::parse_input(&input.sql, input.preserve_comments, input.mybatis);
```

**Step 2: 接入 lint**

在 `Json(out)` 之前（line 4794），插入：
```rust
if input.lint == Some(true) {
    let config = super::build_lint_config_for_api();
    let lint_warnings =
        super::run_lint(&output.statements, ogsql_parser::linter::Confidence::Full, &config, None);
    if !lint_warnings.is_empty() {
        out.as_object_mut().unwrap().insert("lint_warnings".to_string(), serde_json::json!(lint_warnings));
        out.as_object_mut()
            .unwrap()
            .insert("lint_summary".to_string(), super::format_warnings_summary(&lint_warnings));
    }
}
```

**Step 3: 接入 schema_resolution**

在 stmt 遍历（line 4785 之后）插入 schema 解析逻辑。对每个 statement 的 PL block 调用 `analyze_pl_block` + `analyze_transactions` + `resolve_schema`：

```rust
let stmt_values: Vec<serde_json::Value> = output
    .statements
    .iter()
    .map(|si| {
        let mut obj = serde_json::to_value(si).unwrap();
        if let Some(block) = extract_pl_block(&si.statement) {
            // dynamic_sql_analysis
            let report = ogsql_parser::analyze_pl_block(block);
            if !report.execute_findings.is_empty() {
                obj.as_object_mut()
                    .unwrap()
                    .insert("dynamic_sql_analysis".to_string(), serde_json::json!(report));
            }
            // transaction_analysis
            let tx_report = ogsql_parser::analyze_transactions(block);
            obj.as_object_mut().unwrap().insert(
                "transaction_analysis".to_string(),
                serde_json::to_string_pretty(&tx_report).unwrap().into(),
            );
            // schema_resolution (only if schema_json provided)
            if let Some(ref schema_path) = input.schema_json {
                if let Ok(schema) = ogsql_parser::load_schema(schema_path) {
                    let schema_report = ogsql_parser::resolve_schema(block, &schema);
                    obj.as_object_mut()
                        .unwrap()
                        .insert("schema_resolution".to_string(), serde_json::json!(schema_report));
                }
            }
        }
        // routine_analysis
        if super::has_routine_return_cursors(&si.statement) {
            if let Some(analysis) = super::compute_routine_analysis(&si.statement) {
                obj.as_object_mut().unwrap().insert("routine_analysis".to_string(), analysis);
            }
        }
        obj
    })
    .collect();
```

注意：需要在 `output.statements` 迭代之前保存 `schema_json` 的引用（因为 `schema_json` 会被 move 进闭包）。

**Step 4: extract_sql 支持**

在 stmt 遍历完成后，如果 `input.extract_sql` 为 true，调用已有的 extract_sql 逻辑：

```rust
if input.extract_sql {
    let schema = input.schema_json.as_deref().and_then(|p| ogsql_parser::load_full_schema(p).ok());
    let rows: Vec<serde_json::Value> = output
        .statements
        .iter()
        .flat_map(|si| {
            // 对每个 statement，如果它有 PL block，提取 SQL rows
            if let Some(block) = extract_pl_block(&si.statement) {
                let vars = std::collections::HashMap::new();
                let out_cursors = std::collections::HashSet::new();
                super::collect_block_sql_rows(
                    block,
                    "",  // parent_name from statement context
                    si.start_line,
                    &vars,
                    &out_cursors,
                    false,
                )
                .into_iter()
                .map(|row| {
                    serde_json::json!({
                        "line": row.line,
                        "type": row.stmt_type,
                        "name": row.name,
                        "parent": row.parent,
                        "sql": row.sql,
                        "branch_path": row.branch_path,
                        "branch_condition": row.branch_condition,
                    })
                })
                .collect::<Vec<_>>()
            } else {
                vec![]
            }
        })
        .collect();
    out.as_object_mut().unwrap().insert("extracted_sql".to_string(), serde_json::json!(rows));
}
```

**Step 5: Verify**

```bash
cargo check --features serve
```

---

### Task 6: 新增 POST /api/parse-xml 端点

**Files:**
- Modify: `src/bin/ogsql.rs` — api module 内新增

**前提：** `#[cfg(feature = "ibatis")]` feature-gate

**Step 1: 定义 ParseXmlInput**

```rust
#[cfg(feature = "ibatis")]
#[derive(Deserialize, ToSchema)]
pub struct ParseXmlInput {
    /// XML content of an iBatis/MyBatis mapper file
    pub xml: String,
    #[cfg(feature = "java")]
    /// Directory path containing Java source files for parameter type inference
    #[serde(default)]
    pub java_src: Option<String>,
    /// Output structured dynamic SQL AST (preserves SqlNode tree instead of flattening)
    #[serde(default)]
    pub structured: Option<bool>,
}
```

**Step 2: 实现 handle_parse_xml**

```rust
#[cfg(feature = "ibatis")]
#[utoipa::path(
    post,
    path = "/api/parse-xml",
    tag = "ogsql",
    request_body = ParseXmlInput,
    responses((status = 200, description = "Parsed iBatis XML result"))
)]
pub async fn handle_parse_xml(Json(input): Json<ParseXmlInput>) -> Json<serde_json::Value> {
    #[cfg(feature = "java")]
    let java_roots: Vec<std::path::PathBuf> = input.java_src
        .as_deref()
        .map(|p| vec![std::path::PathBuf::from(p)])
        .unwrap_or_default();
    #[cfg(not(feature = "java"))]
    let java_roots: Vec<std::path::PathBuf> = vec![];

    let result = if input.structured.unwrap_or(false) {
        let parsed = ogsql_parser::ibatis::parse_mapper_bytes_structured(input.xml.as_bytes());
        serde_json::to_value(&parsed).unwrap_or(serde_json::json!({"error": "serialization failed"}))
    } else {
        #[cfg(feature = "java")]
        let r = ogsql_parser::ibatis::parse_mapper_bytes_with_java_src(input.xml.as_bytes(), None, java_roots);
        #[cfg(not(feature = "java"))]
        let r = ogsql_parser::ibatis::parse_mapper_bytes(input.xml.as_bytes());
        serde_json::to_value(&r).unwrap_or(serde_json::json!({"error": "serialization failed"}))
    };

    Json(result)
}
```

**Step 3: 注册路由**

router 函数改为用可变绑定的条件路由：

```rust
pub fn router() -> Router {
    let mut router = Router::new()
        .route("/api/health", get(health))
        .route("/api/parse", post(handle_parse))
        .route("/api/json2sql", post(handle_json2sql))
        .route("/api/format", post(handle_format))
        .route("/api/tokenize", post(handle_tokenize))
        .route("/api/validate", post(handle_validate))
        .route("/api-docs/openapi.json", get(openapi_spec));
    #[cfg(feature = "ibatis")]
    {
        router = router.route("/api/parse-xml", post(handle_parse_xml));
    }
    #[cfg(feature = "java")]
    {
        router = router.route("/api/parse-java", post(handle_parse_java));
    }
    router
}
```

**Step 4: 注册 OpenAPI schema**

在 `ApiDoc` struct 之后新增 feature-gated doc struct，并修改 `openapi_spec` handler：

```rust
#[cfg(feature = "ibatis")]
#[derive(OpenApi)]
#[openapi(
    paths(handle_parse_xml),
    components(schemas(ParseXmlInput)),
    tags((name = "ogsql", description = "iBatis/MyBatis XML parsing"))
)]
pub struct ApiDocIbatis;

#[cfg(feature = "java")]
#[derive(OpenApi)]
#[openapi(
    paths(handle_parse_java),
    components(schemas(ParseJavaInput)),
    tags((name = "ogsql", description = "Java SQL extraction"))
)]
pub struct ApiDocJava;

fn build_openapi() -> utoipa::openapi::OpenApi {
    let mut spec = ApiDoc::openapi();
    #[cfg(feature = "ibatis")]
    {
        spec.merge(ApiDocIbatis::openapi());
    }
    #[cfg(feature = "java")]
    {
        spec.merge(ApiDocJava::openapi());
    }
    spec
}

async fn openapi_spec() -> Json<utoipa::openapi::OpenApi> {
    Json(build_openapi())
}
```

**Step 5: Verify**

```bash
cargo check --features serve,ibatis
cargo check --features serve,ibatis,java
```

---

### Task 7: 新增 POST /api/parse-java 端点

**Files:**
- Modify: `src/bin/ogsql.rs` — api module 内新增

**前提：** `#[cfg(feature = "java")]` feature-gate

**Step 1: 定义 ParseJavaInput**

```rust
#[cfg(feature = "java")]
#[derive(Deserialize, ToSchema)]
pub struct ParseJavaInput {
    /// Java source file content
    pub source: String,
    /// Extra method names to treat as SQL-bearing (e.g. ["executeQuery"])
    #[serde(default)]
    pub extra_sql_methods: Option<Vec<String>>,
    /// Extra variable name patterns for SQL detection (e.g. ["QUERY"])
    #[serde(default)]
    pub extra_sql_var_patterns: Option<Vec<String>>,
}
```

**Step 2: 实现 handle_parse_java**

```rust
#[cfg(feature = "java")]
#[utoipa::path(
    post,
    path = "/api/parse-java",
    tag = "ogsql",
    request_body = ParseJavaInput,
    responses((status = 200, description = "Extracted SQL from Java source"))
)]
pub async fn handle_parse_java(Json(input): Json<ParseJavaInput>) -> Json<serde_json::Value> {
    let config = ogsql_parser::java::JavaExtractConfig {
        extra_sql_methods: input.extra_sql_methods.unwrap_or_default(),
        extra_sql_var_patterns: input.extra_sql_var_patterns.unwrap_or_default(),
    };
    let result = ogsql_parser::java::extract_sql_from_java(&input.source, "<api-input>", &config);
    Json(serde_json::to_value(&result).unwrap_or(serde_json::json!({"error": "serialization failed"})))
}
```

**Step 3: 路由和 OpenAPI — 见 Task 6 Step 3 & 4，已一并处理**

**Step 4: Verify**

```bash
cargo check --features serve,java
```

---

### Task 8: 全量验证

**Step 1: full features 编译检查**

```bash
cargo check --features full
```

预期：0 errors。

**Step 2: Clippy**

```bash
cargo clippy --features full -- -D warnings
```

预期：0 warnings。

**Step 3: 测试**

```bash
cargo test --features full
```

预期：全部现有测试通过（full features 下新增代码不应影响已有测试）。

**Step 4: 手动验证端点（可选）**

```bash
# Terminal 1: start server
cargo run --features full -- serve --port 8765

# Terminal 2: test endpoints
# health
curl http://127.0.0.1:8765/api/health

# parse with mybatis
curl -s -X POST http://127.0.0.1:8765/api/parse \
  -H 'Content-Type: application/json' \
  -d '{"sql": "SELECT * FROM t WHERE id = #{userId}", "mybatis": true}' | jq '.statements | length'

# format with mybatis
curl -s -X POST http://127.0.0.1:8765/api/format \
  -H 'Content-Type: application/json' \
  -d '{"sql": "select id,name from t where id=1", "keyword_case": "upper", "indent": 4}' | jq '.formatted'

# tokenize with comments
curl -s -X POST http://127.0.0.1:8765/api/tokenize \
  -H 'Content-Type: application/json' \
  -d '{"sql": "-- comment\nSELECT 1", "preserve_comments": true}' | jq '.tokens | length'

# validate with strict
curl -s -X POST http://127.0.0.1:8765/api/validate \
  -H 'Content-Type: application/json' \
  -d '{"sql": "SELECT 1", "strict": true}' | jq '.valid'

# parse with extract_sql (需要含 PL block 的 SQL)
curl -s -X POST http://127.0.0.1:8765/api/parse \
  -H 'Content-Type: application/json' \
  -d '{"sql": "CREATE FUNCTION f() RETURNS VOID AS $$ BEGIN EXECUTE '\''SELECT 1'\''; END $$ LANGUAGE plpgsql", "extract_sql": true}' | jq '.extracted_sql'

# parse-xml (needs ibatis feature)
curl -s -X POST http://127.0.0.1:8765/api/parse-xml \
  -H 'Content-Type: application/json' \
  -d '{"xml": "<mapper namespace=\"t\"><select id=\"q\">SELECT 1</select></mapper>"}' | jq '.statements | length'

# parse-java (needs java feature)
curl -s -X POST http://127.0.0.1:8765/api/parse-java \
  -H 'Content-Type: application/json' \
  -d '{"source": "class T { void m() { stmt.execute(\"SELECT 1\"); } }"}' | jq '.extractions | length'
```

**Step 5: Commit**

```bash
git add src/bin/ogsql.rs
git commit -m "feat(serve): fill API capability gaps — semantic analysis, validation, formatting, new endpoints

- Split SqlInput into ParseInput / FormatInput / ValidateInput / TokenizeInput
- Parse: add lint, routine_analysis, dynamic_sql_analysis, transaction_analysis,
  schema_resolution, extract_sql, mybatis support
- Validate: add MERGE semantic validation, --strict mode wiring, mybatis support
- Format: add mybatis support, layout flags (no_select_newline, no_logical_newline,
  no_semicolon_newline)
- Tokenize: add preserve_comments, mybatis support
- New endpoints: /api/parse-xml (feature=ibatis), /api/parse-java (feature=java)
- OpenAPI: multi-ApiDoc struct merge for conditional feature-gated endpoint docs"
```

---

### 依赖关系

```
Task 1 (拆分 struct)
 ├── Task 2 (Validate 补全)
 ├── Task 3 (Format 补全)
 ├── Task 4 (Tokenize 补全)
 ├── Task 5 (Parse 补全)
 ├── Task 6 (parse-xml 端点)
 └── Task 7 (parse-java 端点)
      └── Task 8 (全量验证)
```

Tasks 2-7 都依赖 Task 1（struct 拆分），但 Task 2-7 之间相互独立，可并行执行。

---

### 风险点

1. **`extract_pl_block` 函数** — 需要确认该函数在 `ogsql.rs` 的父作用域中已定义。如果不存在，需要从 CLI parse handler 中提取到独立函数或直接从 `ogsql_parser` 库引用。

2. **`collect_block_sql_rows` 的 `extract-sql` 参数** — Task 5 中的 extract_sql 逻辑较复杂，依赖 `collect_block_sql_rows`。如果父作用域没有该函数的合适包装，需要调整实现方式（如先做简单的 PL 块内 SQL 文本提取，而非完整的 CSV row 格式）。

3. **utoipa OpenApi::merge** — 需确认 utoipa 5.x 的 `OpenApi` 类型确实有 `merge` 方法。如果不存在，可以用 `utoipa::openapi::OpenApiBuilder` 手动构建，或降级为单个 `ApiDoc` struct 用 `#[cfg_attr]` 条件字段。

4. **feature 组合** — `ibatis` + `java` 组合下，`parse-xml` 端点需要 `java_src` 字段。需确保所有 feature 组合（`serve` only, `serve+ibatis`, `serve+java`, `serve+ibatis+java`）都能编译通过。
