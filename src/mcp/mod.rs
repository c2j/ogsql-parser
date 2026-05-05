//! MCP (Model Context Protocol) server support.
//!
//! Exposes ogsql-parser capabilities as MCP tools via the `rmcp` crate.
//! Supports stdio transport for integration with Claude Desktop, Cursor, etc.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::schemars::JsonSchema;
use rmcp::tool;
use rmcp::tool_router;
use serde::Deserialize;

use crate::token_formatter::{CommaStyle, FormatConfig, KeywordCase, TokenFormatter};

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

fn default_indent() -> usize { 2 }
fn default_line_width() -> usize { 120 }

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FormatParams {
    /// SQL text to format
    pub sql: String,
    /// Number of spaces per indentation level
    #[serde(default = "default_indent")]
    pub indent: usize,
    /// Keyword casing: "preserve", "upper", or "lower"
    #[serde(default)]
    pub keyword_case: String,
    /// Comma positioning: "trailing" or "leading"
    #[serde(default)]
    pub comma_style: String,
    /// Maximum line width before wrapping
    #[serde(default = "default_line_width")]
    pub line_width: usize,
    /// Convert keywords to uppercase (legacy compat, overrides keyword_case when true)
    #[serde(default)]
    pub uppercase: bool,
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
    #[cfg(feature = "java")]
    /// Directory path containing Java source files for parameter type inference
    #[serde(default)]
    pub java_src: Option<String>,
    #[cfg(feature = "java")]
    /// Inline Java source map: {relative_path: source_code} for parameter type inference
    #[serde(default)]
    pub java_sources: Option<std::collections::HashMap<String, String>>,
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
        Parameters(FormatParams {
            sql,
            indent,
            keyword_case,
            comma_style,
            line_width,
            uppercase,
        }): Parameters<FormatParams>,
    ) -> String {
        let tokens = match crate::Tokenizer::new(&sql).preserve_comments(true).tokenize() {
            Ok(t) => t,
            Err(e) => return format!("{{\"error\": \"{}\"}}", e),
        };

        let keyword_case = match keyword_case.to_lowercase().as_str() {
            "upper" => KeywordCase::Upper,
            "lower" => KeywordCase::Lower,
            _ => KeywordCase::Preserve,
        };

        let comma_style = match comma_style.to_lowercase().as_str() {
            "leading" => CommaStyle::Leading,
            _ => CommaStyle::Trailing,
        };

        let config = FormatConfig {
            indent_width: indent,
            keyword_case,
            comma_style,
            line_width,
            uppercase_keywords: uppercase,
            ..Default::default()
        };

        let formatter = TokenFormatter::with_config(&sql, tokens, config);
        let formatted = formatter.format();

        serde_json::to_string_pretty(&serde_json::json!({
            "formatted": formatted,
            "error_count": 0usize,
            "errors": Vec::<crate::ParserError>::new(),
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

    #[tool(description = "Parse iBatis/MyBatis XML mapper content and extract SQL statements. Optional java_src (directory path) or java_sources (inline {path: content} map) enables parameter type inference from Java source.")]
    fn parse_xml(
        &self,
        #[cfg(feature = "java")]
        Parameters(ParseXmlParams { xml, java_src, java_sources }): Parameters<ParseXmlParams>,
        #[cfg(not(feature = "java"))]
        Parameters(ParseXmlParams { xml }): Parameters<ParseXmlParams>,
    ) -> String {
        #[cfg(feature = "java")]
        let (java_roots, tmp_dir) = {
            if let Some(ref sources) = java_sources {
                let tmp = std::env::temp_dir().join(format!("ogsql_mcp_{}", std::process::id()));
                for (path, content) in sources {
                    let full_path = tmp.join(path);
                    if let Some(parent) = full_path.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    let _ = std::fs::write(&full_path, content);
                }
                (vec![tmp.clone()], Some(tmp))
            } else if let Some(ref src) = java_src {
                (vec![std::path::PathBuf::from(src)], None)
            } else {
                (vec![], None)
            }
        };
        #[cfg(not(feature = "java"))]
        let java_roots: Vec<std::path::PathBuf> = vec![];

        #[cfg(feature = "java")]
        let result = crate::ibatis::parse_mapper_bytes_with_java_src(
            xml.as_bytes(), None, java_roots
        );
        #[cfg(not(feature = "java"))]
        let result = crate::ibatis::parse_mapper_bytes(xml.as_bytes());

        #[cfg(feature = "java")]
        if let Some(tmp) = tmp_dir {
            let _ = std::fs::remove_dir_all(&tmp);
        }

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
