//! MCP (Model Context Protocol) server support.
//!
//! Exposes ogsql-parser capabilities as MCP tools via the `rmcp` crate.
//! Supports stdio transport for integration with Claude Desktop, Cursor, etc.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::schemars::JsonSchema;
use rmcp::tool;
use rmcp::tool_router;
use serde::Deserialize;

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

// ── Server struct ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct OgsqlServer;

// ── Tool implementations ─────────────────────────────────────────────────────

#[tool_router(server_handler)]
impl OgsqlServer {
    #[tool(description = "Parse SQL into structured AST JSON with error reports and query fingerprints")]
    fn parse(
        &self,
        Parameters(ParseParams { sql, preserve_comments }): Parameters<ParseParams>,
    ) -> String {
        let options = crate::ParseOptions { preserve_comments, mybatis_params: false };
        let output = crate::Parser::parse_sql_with_options(&sql, options);

        let all_stmts: Vec<_> = output
            .statements
            .iter()
            .map(|si| si.statement.clone())
            .collect();
        let fingerprints = crate::compute_query_fingerprints(&all_stmts);

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
        match crate::Tokenizer::new(&sql).tokenize() {
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
        let output = crate::Parser::parse_sql_with_options(
            &sql,
            crate::ParseOptions {
                preserve_comments: false,
                mybatis_params: false,
            },
        );
        let formatter = crate::SqlFormatter::new();
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
        let output = crate::Parser::parse_sql_with_options(
            &sql,
            crate::ParseOptions {
                preserve_comments: false,
                mybatis_params: false,
            },
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

        let statements: Vec<crate::Statement> = if let Some(arr) =
            json_value.get("statements")
        {
            match serde_json::from_value(arr.clone()) {
                Ok(s) => s,
                Err(e) => {
                    return format!(
                        "{{\"error\": \"Failed to deserialize statements: {}\"}}",
                        e
                    )
                }
            }
        } else {
            match serde_json::from_value(json_value) {
                Ok(s) => s,
                Err(e) => return format!("{{\"error\": \"Failed to deserialize: {}\"}}", e),
            }
        };

        let formatter = crate::SqlFormatter::new();
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
        let result =
            crate::ibatis::parse_mapper_bytes_with_path(xml.as_bytes(), None);
        serde_json::to_string_pretty(&result)
            .unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
    }

    #[tool(description = "Extract embedded SQL from Java source files (string literals, annotations, method calls)")]
    fn parse_java(
        &self,
        Parameters(ParseJavaParams { source, extra_sql_methods }): Parameters<ParseJavaParams>,
    ) -> String {
        let config = crate::java::JavaExtractConfig {
            extra_sql_methods,
        };
        let result =
            crate::java::extract_sql_from_java(&source, "<mcp-input>", &config);
        serde_json::to_string_pretty(&result)
            .unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn token_display(t: &crate::TokenWithSpan) -> (String, String) {
    use crate::Token;
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

fn is_warning(e: &crate::ParserError) -> bool {
    matches!(
        e,
        crate::ParserError::Warning { .. }
            | crate::ParserError::ReservedKeywordAsIdentifier { .. }
    )
}
