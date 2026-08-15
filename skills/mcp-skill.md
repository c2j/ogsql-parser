# MCP (Model Context Protocol) Server Skill

## Description

oggsql-parser provides a complete MCP (Model Context Protocol) server that exposes 7 SQL parsing tools for AI assistants (Claude Desktop, Cursor, etc.). Built on `rmcp` 1.5 with stdio transport, it gives AI models direct access to SQL tokenization, parsing, formatting, validation, and iBatis/MyBatis XML/Java SQL extraction capabilities.

## When to Use

- Integrating oggsql-parser with AI editors (Claude Desktop, Cursor, Continue.dev, etc.)
- Adding SQL parsing capabilities to AI agent workflows
- Exposing the SQL linter (53 rules) through MCP for AI-assisted code review
- Round-tripping SQL ↔ AST JSON with AI assistance
- Validating iBatis/MyBatis XML mappers or extracting SQL from Java via MCP

## Architecture

### Crate Structure

```
src/mcp/
├── mod.rs      # OgsqlServer struct + all 7 tool implementations (499 lines)
├── tests.rs    # Parameter deserialization + tool functionality tests (196 lines)
src/bin/
├── ogsql-mcp.rs # Standalone MCP binary entry point (13 lines)
└── ogsql.rs     # CLI: `ogsl mcp` subcommand dispatches here (15 lines)
```

### Dependency Chain

| Layer | Crate | Purpose |
|---|---|---|
| Transport | `rmcp` 1.5 + `schemars` 1 | MCP protocol (server, macros, transport-io, schemars features) |
| Async Runtime | `tokio` 1 | Single-threaded async for stdio I/O |
| Core Parser | `ogsql-parser` (crate internals) | Tokenizer, Parser, Formatter, Linter, Analyzer |
| Extensions | `quick-xml` 0.41, `tree-sitter-java` 0.23 | XML parsing (ibatis), Java extraction (java) |

### Feature Flags

```toml
# Minimal MCP (parse, tokenize, format, validate, json2sql only)
cargo build --release --features mcp

# The `mcp` feature implies:
mcp = ["dep:rmcp", "dep:schemars", "dep:tokio", "ibatis", "java"]
```

`mcp` aggregates `ibatis` + `java` by default — so `parse_xml` and `parse_java` tools are always available when MCP is built.

### Entry Points

Two ways to start the MCP server:

```bash
# 1. Dedicated binary (smaller binary size)
cargo build --release --features mcp
./target/release/ogsql-mcp

# 2. CLI subcommand (same OgsqlServer underneath)
cargo build --release --features mcp
./target/release/ogsql mcp
```

Both use `OgsqlServer.serve(rmcp::transport::stdio())` — stdio transport, no HTTP. Tokio runtime is single-threaded (no `rt-multi-thread`).

## Available Tools

### 1. `parse` — Parse SQL to AST JSON

Parse SQL text into structured AST with error reports, query fingerprints, optional lint warnings, and routine analysis for stored procedures/functions/packages.

**Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `sql` | string | Yes | — | SQL text to parse |
| `preserve_comments` | boolean | No | `false` | Include comments in output |
| `lint` | boolean | No | `false` | Enable SQL anti-pattern linting (53 rules, 4 severity levels) |

**Response:**

```json
{
  "statements": [
    {
      "statement": { ... },
      "range": { "start": 0, "end": 20 },
      "routine_analysis": { ... }
    }
  ],
  "errors": [],
  "query_fingerprints": ["...", "..."],
  "comments": [],
  "lint_warnings": [],
  "lint_summary": { ... }
}
```

Response fields:
- `statements[]`: Each StatementInfo with AST + source range. For `CREATE PROCEDURE`/`CREATE FUNCTION`/`CREATE PACKAGE BODY`, includes `routine_analysis` with return cursor analysis.
- `errors[]`: Parse errors with locations and hints
- `query_fingerprints[]`: Deterministic fingerprints for query identification (appears when statements present)
- `comments[]`: Extracted comments (when `preserve_comments: true`)
- `lint_warnings[]` / `lint_summary`: Lint results (when `lint: true`)

**Example prompt for Claude:**
```
Parse this SQL and tell me what statements are in it:
SELECT name, COUNT(*) FROM users GROUP BY name HAVING COUNT(*) > 5
```

### 2. `tokenize` — Tokenize SQL

Break SQL text into typed tokens with `(line, column)` positions.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `sql` | string | Yes | SQL text to tokenize |

**Response:**

```json
{
  "tokens": [
    { "type": "Keyword", "value": "SELECT", "line": 1, "column": 0 },
    { "type": "Ident", "value": "id", "line": 1, "column": 7 },
    { "type": "Keyword", "value": "FROM", "line": 1, "column": 10 },
    { "type": "Ident", "value": "users", "line": 1, "column": 15 },
    { "type": "Other", "value": ";", "line": 1, "column": 20 }
  ]
}
```

Token types: `Keyword`, `Ident`, `Integer`, `Float`, `String`, `Op`, `Comment`, `Other`.

### 3. `format` — Format SQL

Format SQL with configurable indentation, keyword casing, comma style, and line width.

**Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `sql` | string | Yes | — | SQL text to format |
| `indent` | number | No | `2` | Spaces per indentation level |
| `keyword_case` | string | No | `""` | `"upper"`, `"lower"`, or `""` (preserve) |
| `comma_style` | string | No | `""` | `"trailing"` or `"leading"` |
| `line_width` | number | No | `120` | Max line width before wrapping (0 = unlimited) |
| `uppercase` | boolean | No | `false` | Legacy compat: converts keywords to uppercase (overrides `keyword_case` when `true`) |

**Response:**

```json
{
  "formatted": "SELECT id,\n       name\nFROM users\nWHERE status = 'active'",
  "error_count": 0,
  "errors": []
}
```

**Example prompt for Claude:**
```
Format this SQL with uppercase keywords, 4-space indent, leading commas:
select id,name,email from users where created_at > '2024-01-01' order by name desc
```

### 4. `validate` — Validate SQL

Validate SQL syntax and report errors, warnings, package consistency errors, MERGE semantic errors, and optional lint warnings. This is the most comprehensive tool — it runs semantic validation on top of parse.

**Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `sql` | string | Yes | — | SQL text to validate |
| `lint` | boolean | No | `false` | Enable SQL anti-pattern linting |

**Response:**

```json
{
  "valid": true,
  "error_count": 0,
  "warning_count": 1,
  "errors": [],
  "package_consistency_errors": [],
  "merge_semantic_errors": [],
  "lint_warnings": [],
  "lint_summary": { ... }
}
```

Validation layers (all run automatically):
1. **Parse validation** — syntax errors detected by the parser
2. **Package consistency** — PACKAGE vs PACKAGE BODY mismatch detection
3. **MERGE semantics** — non-deterministic/invalid MERGE pattern detection
4. **Lint rules** (optional, when `lint: true`) — 53 anti-pattern rules at Prohibition/Performance/Caution/Suggestion levels

`valid` is `true` only when there are zero non-warning errors. Warning-level issues (e.g., linter Caution/Suggestion, package consistency warnings) do not affect validity.

**Example prompt for Claude:**
```
Validate this SQL and tell me if there are any issues:
SELECT * FROM users WHERE LEFT(name, 3) = 'abc'
```

### 5. `json2sql` — AST JSON to SQL

Convert AST JSON (from the `parse` tool output) back to formatted SQL text. Supports lossless semantic round-trip.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `json` | string | Yes | JSON string containing statements (parse tool output) |

**Response:**

```json
{
  "statements": ["SELECT id, name FROM users WHERE status = 'active'"],
  "count": 1
}
```

Accepts both raw `Statement[]` JSON and `{"statements": [...]}` wrapper format (auto-detected).

**Example prompt for Claude:**
```
I have this AST JSON, convert it back to SQL: {"statements":[{"Select":{...}}]}
```

### 6. `parse_xml` — Parse iBatis/MyBatis XML Mapper

Parse MyBatis XML mapper content and extract SQL statements. Optionally enables Java parameter type inference from Java source.

**Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `xml` | string | Yes | — | XML content of an iBatis/MyBatis mapper file |
| `java_src` | string | No | `null` | Directory path containing Java source files for parameter type inference |
| `java_sources` | object | No | `null` | Inline Java source map: `{"relative_path": "source_code", ...}` for parameter type inference |

`java_src` / `java_sources` require the `java` feature (included by default in `mcp`).

**Response:**

```json
{
  "statements": [
    {
      "id": "findById",
      "type": "select",
      "flat_sql": "SELECT * FROM users WHERE id = #{id} AND name = #{name}",
      "parameters": [
        { "name": "id", "jdbc_type": "BIGINT" },
        { "name": "name", "jdbc_type": "VARCHAR" }
      ],
      "dynamic_sql": { ... }
    }
  ]
}
```

Temporary Java source directories (from `java_sources` inline map) are cleaned up automatically after parsing.

**Example prompt for Claude:**
```
Parse this MyBatis mapper: <mapper namespace="test"><select id="findById">SELECT * FROM t WHERE id = #{id}</select></mapper>
```

### 7. `parse_java` — Extract SQL from Java Source

Extract embedded SQL from Java source files by analyzing string literals, annotations, and method calls.

**Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `source` | string | Yes | — | Java source file content |
| `extra_sql_methods` | string[] | No | `[]` | Extra method names to treat as SQL-bearing (e.g. `["executeQuery", "nativeQuery"]`) |
| `extra_sql_var_patterns` | string[] | No | `[]` | Extra variable name patterns for SQL detection (e.g. `["QUERY", "STMT"]`) |

**Response:**

```json
{
  "extractions": [
    {
      "sql": "SELECT * FROM users WHERE id = ?",
      "location": { "line": 42, "column": 16 },
      "source": "string_literal"
    }
  ]
}
```

**Example prompt for Claude:**
```
Extract SQL from this Java code:
String sql = "SELECT * FROM users WHERE name LIKE '%test%'";
PreparedStatement ps = conn.prepareStatement(sql);
```

## Integration Configuration

### Claude Desktop

Add to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "ogsql": {
      "command": "/path/to/target/release/ogsql-mcp"
    }
  }
}
```

Or use the CLI subcommand:

```json
{
  "mcpServers": {
    "ogsql": {
      "command": "/path/to/target/release/ogsql",
      "args": ["mcp"]
    }
  }
}
```

### Cursor

Create `.cursor/mcp.json` in your project root:

```json
{
  "mcpServers": {
    "ogsql": {
      "command": "/path/to/target/release/ogsql-mcp"
    }
  }
}
```

### Testing the MCP Server

Use the MCP Inspector for manual testing:

```bash
npx @modelcontextprotocol/inspector /path/to/ogsql-mcp
```

## Building

```bash
# Build both binaries (CLI + MCP) with all features
cargo build --release --features full

# Build only the MCP binary (smaller, includes ibatis + java + linter)
cargo build --release --features mcp

# Windows 7 compatible build
cargo +nightly build --release --features mcp \
  --target x86_64-win7-windows-msvc \
  -Zbuild-std
```

The `mcp` feature flag implies: `rmcp` + `schemars` + `tokio` + `ibatis` + `java` + `lint-config`.

## Implementation Patterns

### Tool Definition

All tools use the `rmcp` proc-macro pattern:

```rust
#[tool_router(server_handler)]
impl OgsqlServer {
    #[tool(description = "Parse SQL into structured AST JSON...")]
    fn parse(&self, Parameters(ParseParams { ... }): Parameters<ParseParams>) -> String {
        // ...implementation...
        serde_json::to_string_pretty(&result).unwrap_or_default()
    }
}
```

Key conventions:
- Every tool returns `String` (JSON-serialized response)
- Parameters derive `Deserialize + JsonSchema` for automatic MCP schema generation
- Error states are returned as JSON `{"error": "..."}` — never panics
- All core logic delegates to crate-internal library functions (no duplicate implementation)

### Parameter Defaults

`FormatParams` uses explicit default functions:

```rust
fn default_indent() -> usize { 2 }
fn default_line_width() -> usize { 120 }
```

Others use `#[serde(default)]` for `Option`/`Vec`/`bool` (defaulting to `None`/`[]`/`false`).

### Conditional Compilation for `java` Feature

`parse_xml` has dual parameter signatures — one with `java_src`/`java_sources` fields (when `java` feature is on) and one without:

```rust
fn parse_xml(
    &self,
    #[cfg(feature = "java")] Parameters(ParseXmlParams { xml, java_src, java_sources }): ...,
    #[cfg(not(feature = "java"))] Parameters(ParseXmlParams { xml }): ...,
) -> String { ... }
```

All temporary directories from `java_sources` are cleaned up in `finally`-style code after the call completes.

### Lint Integration

Both `parse` and `validate` support optional linting via the `lint: true` parameter. When enabled:

```rust
let config = LintConfig::default();
let linter = SqlLinter::with_default_rules(config);
let lint_warnings = linter.lint(&output.statements, None, Confidence::Full);
```

Lint output is appended as `lint_warnings` and `lint_summary` fields in the response JSON.

### Routine Analysis

The `parse` tool includes `compute_routine_analysis()` which runs return cursor analysis on:
- `Statement::CreateProcedure` — cursor return analysis on PL/pgSQL blocks
- `Statement::CreateFunction` — cursor return analysis with return type info
- `Statement::CreatePackageBody` — per-subprogram cursor analysis

Result is injected as `routine_analysis` in each statement's output object. Empty analysis results are omitted.

### Token Display Helpers

`token_display()` maps internal `Token` variants to human-readable `(type, value)` pairs:

| Token Variant | Display Type | Display Value |
|---|---|---|
| `Keyword(k)` | `"Keyword"` | Debug format (e.g. `"SELECT"`) |
| `Ident(s)` | `"Ident"` | String value |
| `Integer(n)` | `"Integer"` | Number string |
| `StringLiteral(s)` | `"String"` | String value |
| `Float(s)` | `"Float"` | String value |
| `Op(s)` / `OpLe` / `OpNe` / etc. | `"Op"` | Operator string |
| `Comment(s)` | `"Comment"` | String value |
| Other variants | `"Other"` | Debug format |

## Testing

MCP tests are in `src/mcp/tests.rs` (196 lines):

```bash
# Run all tests (includes MCP tests when mcp feature enabled)
cargo test --all-features

# Run only MCP tests
cargo test --features mcp mcp::
```

Test coverage:
- **Parameter deserialization** — all 7 param types tested for default values and custom inputs
- **Tool functionality** — `parse`, `tokenize`, `format`, `validate`, `json2sql` tested with valid and invalid inputs
- **Helper functions** — `is_warning()`, `token_display()` for keyword, ident, integer variants

## Relationship to HTTP API (`serve` feature)

The MCP server and HTTP API server serve different use cases:

| Aspect | MCP (`mcp`) | HTTP API (`serve`) |
|---|---|---|
| Protocol | MCP over stdio | RESTful HTTP via axum |
| Transport | Process stdio (pipes) | HTTP over TCP (host:port) |
| Audience | AI assistants (Claude, Cursor) | Web apps, scripts, CI/CD |
| Endpoints | 7 MCP tools | 5+ REST endpoints |
| Authentication | None (local process) | Optional (CORS, tower-http middleware) |
| Lint | Inline in parse/validate | Dedicated `/api/lint` endpoint |
| Swagger UI | No | Optional (`utoipa-swagger-ui`) |
| Multi-statement | Parser-driven | Post-body JSON array |
| File upload | Via `java_sources` map | Multipart upload |

## Error Handling

The MCP server never panics. All error paths return JSON:

```json
{ "error": "description message" }
```

Error sources:
- **JSON serialization failure** → `{"error": "..."}` fallback
- **Tokenizer error** → `{"error": "message"}` in format tool
- **Invalid input JSON** → `{"error": "Invalid JSON: ..."}` in json2sql tool
- **Deserialization failure** → `{"error": "Failed to deserialize statements: ..."}` in json2sql tool
- **Server init failure** → stderr log + process exit with error

## Version Compatibility

| Component | Version |
|---|---|
| `rmcp` (MCP framework) | 1.5 |
| `schemars` (JSON schema) | 1 |
| `tokio` (async runtime) | 1 |
| Minimum Rust | 1.71 |

The MCP binary requires exactly the `mcp` feature flag. Attempting to run `ogsl-mcp` without it will fail at build time (`required-features = ["mcp"]`).
