# MCP Server Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Expose ogsql-parser's parsing capabilities (parse, tokenize, format, validate, json2sql, parse-xml, parse-java) as MCP tools via the `rmcp` crate, using stdio transport.

**Architecture:** Add a new `mcp` feature gate to Cargo.toml. Create `src/mcp/mod.rs` with an `OgsqlServer` struct using `rmcp`'s `#[tool_router(server_handler)]` macro to define all tools. Create `src/bin/ogsql-mcp.rs` as the binary entry point that starts the server with stdio transport. The MCP module depends on the parser core only — no axum/HTTP dependency.

**Tech Stack:** Rust, `rmcp` v1.5 (official MCP Rust SDK), `schemars` for JSON Schema generation, `tokio` for async runtime.

**Worktree:** `/Users/c2j/Projects/Desktop_Projects/DB/ogsql-parser/.worktrees/feat/mcp-server`
**Branch:** `feat/mcp-server`

---

## Reference Files (READ FIRST before starting)

- `Cargo.toml` — Current feature gates and dependencies pattern
- `src/lib.rs` — Module exports pattern
- `src/bin/ogsql.rs` — Existing CLI binary, shows how parser is called
- `src/ibatis/mod.rs` — `parse_mapper_bytes_with_path()` API
- `src/java/mod.rs` — `extract_sql_from_java()` API

## Key rmcp API Patterns

```rust
// Server struct — must derive Clone
#[derive(Debug, Clone)]
struct OgsqlServer;

// Tool router + auto ServerHandler impl
#[tool_router(server_handler)]
impl OgsqlServer {
    // Each tool: #[tool(description = "...")] + Parameters<T> -> return type
    #[tool(description = "Parse SQL into AST JSON")]
    fn parse(
        &self,
        Parameters(params): Parameters<ParseParams>,
    ) -> String {
        // ... return JSON string
    }
}

// Binary entry: stdio transport
// let service = OgsqlServer.serve(stdio()).await?;
// service.waiting().await?;
```

**Return types:** Tools return `String` (plain text) or `Json<T>` (structured). For our case, returning JSON strings is simplest since our parser already produces serde_json output.

---

### Task 1: Add rmcp dependencies + mcp feature to Cargo.toml

**Files:**
- Modify: `Cargo.toml`

**Step 1: Add new optional dependencies**

Add after the existing optional deps block (after `walkdir` line):

```toml
rmcp = { version = "1.5", features = ["server", "macros", "transport-io"], optional = true }
schemars = { version = "0.8", optional = true }
```

**Step 2: Add `mcp` feature**

Add to the `[features]` section after the `tui` feature:

```toml
mcp = ["dep:rmcp", "dep:schemars", "dep:tokio"]
```

Also add `mcp` to the `full` feature:

```toml
full = ["cli", "ibatis", "java", "mcp", "serve", "tui"]
```

**Step 3: Add new binary target**

Add after the existing `[[bin]]` block:

```toml
[[bin]]
name = "ogsql-mcp"
path = "src/bin/ogsql-mcp.rs"
required-features = ["mcp"]
```

**Step 4: Verify Cargo.toml parses correctly**

Run: `cargo check --features mcp 2>&1 | head -20`
Expected: May show missing module errors (that's OK — we haven't created the files yet). Should NOT show dependency resolution errors.

**Step 5: Commit**

```bash
git add Cargo.toml
git commit -m "feat(mcp): add rmcp dependency and mcp feature gate"
```

---

### Task 2: Create src/mcp/mod.rs with all tool definitions

**Files:**
- Create: `src/mcp/mod.rs`

This is the core file. It defines the `OgsqlServer` struct and all 7 MCP tools.

**Step 1: Create the file with the complete implementation**

```rust
//! MCP (Model Context Protocol) server support.
//!
//! Exposes ogsql-parser capabilities as MCP tools via the `rmcp` crate.

use rmcp::handler::server::tool::ToolRouter;
use rmcp::model::{ServerInfo, ServerTool};
use rmcp::{tool, tool_router, ServiceExt};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct OgsqlServer;

// ── Parameter types ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ParseParams {
    /// SQL text to parse
    pub sql: String,
    /// Whether to preserve comments in output
    #[serde(default)]
    pub preserve_comments: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TokenizeParams {
    /// SQL text to tokenize
    pub sql: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FormatParams {
    /// SQL text to format
    pub sql: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ValidateParams {
    /// SQL text to validate
    pub sql: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Json2SqlParams {
    /// JSON string (output from parse tool) containing statements
    pub json: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ParseXmlParams {
    /// XML content of an iBatis/MyBatis mapper file
    pub xml: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ParseJavaParams {
    /// Java source file content
    pub source: String,
    /// Extra method names to treat as SQL-bearing (e.g. ["executeQuery", "nativeQuery"])
    #[serde(default)]
    pub extra_sql_methods: Vec<String>,
}

// ── Tool implementations ─────────────────────────────────────────────────────

#[tool_router(server_handler)]
impl OgsqlServer {
    #[tool(description = "Parse SQL into structured AST JSON with error reports and query fingerprints")]
    fn parse(
        &self,
        Parameters(ParseParams { sql, preserve_comments }): Parameters<ParseParams>,
    ) -> String {
        let options = ogsql_parser::ParseOptions { preserve_comments };
        let output = ogsql_parser::Parser::parse_sql_with_options(&sql, options);

        let all_stmts: Vec<_> = output.statements.iter().map(|si| si.statement.clone()).collect();
        let fingerprints = ogsql_parser::compute_query_fingerprints(&all_stmts);

        let mut out = serde_json::json!({
            "statements": output.statements,
            "errors": output.errors,
        });
        if !fingerprints.is_empty() {
            out.as_object_mut().unwrap().insert(
                "query_fingerprints".to_string(),
                serde_json::json!(fingerprints),
            );
        }
        if !output.comments.is_empty() {
            out.as_object_mut().unwrap().insert(
                "comments".to_string(),
                serde_json::json!(output.comments),
            );
        }
        serde_json::to_string_pretty(&out).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
    }

    #[tool(description = "Tokenize SQL into a list of typed tokens with line/column positions")]
    fn tokenize(
        &self,
        Parameters(TokenizeParams { sql }): Parameters<TokenizeParams>,
    ) -> String {
        match ogsql_parser::Tokenizer::new(&sql).tokenize() {
            Ok(tokens) => {
                let list: Vec<serde_json::Value> = tokens
                    .iter()
                    .map(|t| {
                        let (token_type, value) = token_display(t);
                        serde_json::json!({
                            "type": token_type,
                            "value": value,
                            "line": t.location.line,
                            "column": t.location.column,
                        })
                    })
                    .collect();
                serde_json::to_string_pretty(&serde_json::json!({"tokens": list}))
                    .unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
            }
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    #[tool(description = "Format SQL with standardized keyword casing and indentation")]
    fn format(
        &self,
        Parameters(FormatParams { sql }): Parameters<FormatParams>,
    ) -> String {
        let output = ogsql_parser::Parser::parse_sql_with_options(
            &sql,
            ogsql_parser::ParseOptions { preserve_comments: false },
        );
        let formatter = ogsql_parser::SqlFormatter::new();
        let formatted: Vec<String> = output
            .statements
            .iter()
            .map(|si| formatter.format_statement(&si.statement))
            .collect();
        serde_json::to_string_pretty(&serde_json::json!({
            "formatted": formatted.join(";\n"),
            "statement_count": formatted.len(),
            "error_count": output.errors.len(),
            "errors": output.errors,
        }))
        .unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
    }

    #[tool(description = "Validate SQL syntax and report errors and warnings")]
    fn validate(
        &self,
        Parameters(ValidateParams { sql }): Parameters<ValidateParams>,
    ) -> String {
        let output = ogsql_parser::Parser::parse_sql_with_options(
            &sql,
            ogsql_parser::ParseOptions { preserve_comments: false },
        );
        let errors = output.errors;
        let has_real_errors = errors.iter().any(|e| !is_warning(e));
        serde_json::to_string_pretty(&serde_json::json!({
            "valid": !has_real_errors,
            "error_count": errors.iter().filter(|e| !is_warning(e)).count(),
            "warning_count": errors.iter().filter(|e| is_warning(e)).count(),
            "errors": errors,
        }))
        .unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
    }

    #[tool(description = "Convert JSON AST (from parse tool output) back to SQL text")]
    fn json2sql(
        &self,
        Parameters(Json2SqlParams { json }): Parameters<Json2SqlParams>,
    ) -> String {
        let json_value: serde_json::Value = match serde_json::from_str(&json) {
            Ok(v) => v,
            Err(e) => return format!("{{\"error\": \"Invalid JSON: {}\"}}", e),
        };

        let statements: Vec<ogsql_parser::Statement> = if let Some(arr) = json_value.get("statements") {
            match serde_json::from_value(arr.clone()) {
                Ok(s) => s,
                Err(e) => return format!("{{\"error\": \"Failed to deserialize statements: {}\"}}", e),
            }
        } else {
            match serde_json::from_value(json_value) {
                Ok(s) => s,
                Err(e) => return format!("{{\"error\": \"Failed to deserialize: {}\"}}", e),
            }
        };

        let formatter = ogsql_parser::SqlFormatter::new();
        let formatted: Vec<String> = statements
            .iter()
            .map(|s| formatter.format_statement(s))
            .collect();

        serde_json::to_string_pretty(&serde_json::json!({
            "statements": formatted,
            "count": formatted.len(),
        }))
        .unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
    }

    #[tool(description = "Parse iBatis/MyBatis XML mapper content and extract SQL statements")]
    fn parse_xml(
        &self,
        Parameters(ParseXmlParams { xml }): Parameters<ParseXmlParams>,
    ) -> String {
        let result = ogsql_parser::ibatis::parse_mapper_bytes_with_path(xml.as_bytes(), None);
        serde_json::to_string_pretty(&result)
            .unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
    }

    #[tool(description = "Extract embedded SQL from Java source files (string literals, annotations, method calls)")]
    fn parse_java(
        &self,
        Parameters(ParseJavaParams { source, extra_sql_methods }): Parameters<ParseJavaParams>,
    ) -> String {
        let config = ogsql_parser::java::JavaExtractConfig { extra_sql_methods };
        let result = ogsql_parser::java::extract_sql_from_java(&source, "<mcp-input>", &config);
        serde_json::to_string_pretty(&result)
            .unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn token_display(t: &ogsql_parser::TokenWithSpan) -> (String, String) {
    use ogsql_parser::Token;
    match &t.token {
        Token::Keyword(k) => ("Keyword".into(), format!("{:?}", k)),
        Token::Ident(s) => ("Ident".into(), s.clone()),
        Token::Integer(n) => ("Integer".into(), n.to_string()),
        Token::StringLiteral(s) => ("String".into(), s.clone()),
        Token::Float(s) => ("Float".into(), s.clone()),
        Token::Op(s) => ("Op".into(), s.clone()),
        Token::OpLe => ("Op".into(), "<=".into()),
        Token::OpNe => ("Op".into(), "<>".into()),
        Token::OpGe => ("Op".into(), ">=".into()),
        Token::OpShiftL => ("Op".into(), "<<".into()),
        Token::OpShiftR => ("Op".into(), ">>".into()),
        Token::OpArrow => ("Op".into(), "->".into()),
        Token::OpJsonArrow => ("Op".into(), "->>".into()),
        Token::OpNe2 => ("Op".into(), "!=".into()),
        Token::OpDblBang => ("Op".into(), "!!".into()),
        Token::OpConcat => ("Op".into(), "||".into()),
        Token::Comment(s) => ("Comment".into(), s.clone()),
        other => ("Other".into(), format!("{:?}", other)),
    }
}

fn is_warning(e: &ogsql_parser::ParserError) -> bool {
    matches!(
        e,
        ogsql_parser::ParserError::Warning { .. }
            | ogsql_parser::ParserError::ReservedKeywordAsIdentifier { .. }
    )
}
```

**IMPORTANT:** The `parse_xml` and `parse_java` tools MUST be defined unconditionally in this file. The feature gates for `ibatis` and `java` are handled at the `Cargo.toml` level — the `mcp` feature should require `ibatis` and `java` as well. Update the feature definition:

```toml
mcp = ["dep:rmcp", "dep:schemars", "dep:tokio", "ibatis", "java"]
```

This way, when `mcp` is enabled, `ibatis` and `java` are automatically pulled in.

**Step 2: Verify it compiles (expect errors for missing module registration — we fix that in Task 3)**

Run: `cargo check --features mcp 2>&1 | head -30`

**Step 3: Commit**

```bash
git add src/mcp/mod.rs
git commit -m "feat(mcp): add OgsqlServer with all 7 tool definitions"
```

---

### Task 3: Update lib.rs to expose mcp module

**Files:**
- Modify: `src/lib.rs`

**Step 1: Add mcp module export**

Add at the end of `src/lib.rs` (after the `#[cfg(feature = "java")]` block):

```rust
#[cfg(feature = "mcp")]
pub mod mcp;
```

**Step 2: Verify compilation**

Run: `cargo check --features mcp 2>&1 | tail -10`
Expected: Should compile successfully (or only show warnings).

**Step 3: Commit**

```bash
git add src/lib.rs
git commit -m "feat(mcp): expose mcp module from lib.rs"
```

---

### Task 4: Create binary entry src/bin/ogsql-mcp.rs

**Files:**
- Create: `src/bin/ogsql-mcp.rs`

**Step 1: Create the file**

```rust
//! MCP server binary for ogsql-parser.
//!
//! Starts an MCP server over stdio transport, exposing all parser tools.
//!
//! Usage with Claude Desktop (add to claude_desktop_config.json):
//! ```json
//! {
//!   "mcpServers": {
//!     "ogsql": {
//!       "command": "ogsql-mcp",
//!       "args": []
//!     }
//!   }
//! }
//! ```

use ogsql_parser::mcp::OgsqlServer;
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // IMPORTANT: Use stderr for logging in stdio mode — stdout is reserved for MCP protocol
    eprintln!("ogsql-mcp: starting MCP server on stdio");

    let service = OgsqlServer
        .serve(rmcp::transport::stdio())
        .await
        .map_err(|e| {
            eprintln!("ogsql-mcp: server initialization failed: {:?}", e);
            e
        })?;

    service.waiting().await?;
    Ok(())
}
```

**Step 2: Verify the binary compiles**

Run: `cargo check --features mcp --bin ogsql-mcp 2>&1 | tail -10`
Expected: Compiles successfully.

**Step 3: Commit**

```bash
git add src/bin/ogsql-mcp.rs
git commit -m "feat(mcp): add ogsql-mcp binary with stdio transport"
```

---

### Task 5: Build verification and smoke test

**Step 1: Full build with mcp feature**

Run: `cargo build --features mcp 2>&1 | tail -10`
Expected: Build succeeds.

**Step 2: Build with full features**

Run: `cargo build --features full 2>&1 | tail -10`
Expected: Build succeeds.

**Step 3: Existing tests still pass**

Run: `cargo test 2>&1 | tail -10`
Expected: All existing tests pass (same as baseline).

**Step 4: Manual smoke test — verify server starts**

Run the binary in background briefly to check it starts without panicking:

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0.1"}}}' | timeout 3 ./target/debug/ogsql-mcp 2>/dev/null || true
```

Expected: JSON-RPC response with server info (not a panic/crash).

**Step 5: Commit any fixes**

If any fixes were needed, commit them.

---

### Task 6: Update README with MCP documentation

**Files:**
- Modify: `README.md`

**Step 1: Add MCP section after the HTTP API section**

Add a new section:

```markdown
### MCP Server / MCP 服务器

When built with `--features mcp`, an MCP server binary is available:

```bash
# Build MCP server
cargo build --release --features mcp

# Run (stdio transport)
ogsql-mcp
```

#### MCP Tools / MCP 工具

| Tool | Description |
|------|-------------|
| `parse` | Parse SQL → AST JSON (with fingerprints, comments, errors) |
| `tokenize` | SQL → Token list with types, values, positions |
| `format` | Format SQL with standardized casing |
| `validate` | Validate SQL syntax, report errors/warnings |
| `json2sql` | Convert AST JSON back to SQL |
| `parse_xml` | Parse iBatis/MyBatis XML mapper → extracted SQL |
| `parse_java` | Extract SQL from Java source files |

#### Claude Desktop Configuration

Add to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "ogsql": {
      "command": "/path/to/ogsql-mcp"
    }
  }
}
```
```

**Step 2: Update CLI Commands section**

Add `mcp` to the feature build examples and note the binary.

**Step 3: Commit**

```bash
git add README.md
git commit -m "docs: add MCP server documentation to README"
```

---

## Summary of File Changes

| File | Action | Description |
|------|--------|-------------|
| `Cargo.toml` | Modify | Add rmcp, schemars deps; add mcp feature; add ogsql-mcp binary |
| `src/mcp/mod.rs` | Create | OgsqlServer struct with 7 tools |
| `src/lib.rs` | Modify | Add `#[cfg(feature = "mcp")] pub mod mcp;` |
| `src/bin/ogsql-mcp.rs` | Create | Binary entry with stdio transport |
| `README.md` | Modify | MCP documentation section |
