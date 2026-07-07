//! Request and response types for the HTTP API.

use serde::Deserialize;
use utoipa::ToSchema;

// ─── Request types ────────────────────────────────────────────

/// POST /api/parse request body.
#[derive(Deserialize, ToSchema)]
#[non_exhaustive]
pub struct ParseInput {
    pub sql: String,
    #[serde(default)]
    pub preserve_comments: bool,
    #[serde(default)]
    pub mybatis: bool,
    #[serde(default)]
    pub procedure: Option<String>,
    #[serde(default)]
    pub extract_sql: bool,
    #[serde(default)]
    pub lint: Option<bool>,
    #[serde(default)]
    pub schema_json: Option<String>,
    #[serde(default)]
    pub lint_config: Option<LintConfigInput>,
}

/// POST /api/format request body.
#[derive(Deserialize, ToSchema)]
#[non_exhaustive]
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
    #[serde(default)]
    pub mybatis: bool,
    #[serde(default)]
    pub no_select_newline: Option<bool>,
    #[serde(default)]
    pub no_logical_newline: Option<bool>,
    #[serde(default)]
    pub no_semicolon_newline: Option<bool>,
}

/// POST /api/validate request body.
#[derive(Deserialize, ToSchema)]
#[non_exhaustive]
pub struct ValidateInput {
    pub sql: String,
    #[serde(default)]
    pub mybatis: bool,
    #[serde(default)]
    pub strict: Option<bool>,
    #[serde(default)]
    pub lint: Option<bool>,
    #[serde(default)]
    pub schema_json: Option<String>,
    #[serde(default)]
    pub lint_config: Option<LintConfigInput>,
    /// Output format: `"sarif"` for SARIF 2.1.0, omitted for default JSON.
    #[serde(default)]
    pub format: Option<String>,
}

/// POST /api/tokenize request body.
#[derive(Deserialize, ToSchema)]
#[non_exhaustive]
pub struct TokenizeInput {
    pub sql: String,
    #[serde(default)]
    pub preserve_comments: bool,
    #[serde(default)]
    pub mybatis: bool,
}

/// POST /api/json2sql request body.
#[derive(Deserialize, ToSchema)]
#[non_exhaustive]
pub struct JsonInput {
    pub json: String,
}

#[cfg(feature = "ibatis")]
/// POST /api/parse-xml request body.
#[derive(Deserialize, ToSchema)]
#[non_exhaustive]
pub struct ParseXmlInput {
    pub xml: String,
    #[cfg(feature = "java")]
    #[serde(default)]
    pub java_src: Option<String>,
    #[serde(default)]
    pub structured: Option<bool>,
}

#[cfg(feature = "java")]
/// POST /api/parse-java request body.
#[derive(Deserialize, ToSchema)]
#[non_exhaustive]
pub struct ParseJavaInput {
    pub source: String,
    #[serde(default)]
    pub extra_sql_methods: Option<Vec<String>>,
    #[serde(default)]
    pub extra_sql_var_patterns: Option<Vec<String>>,
}

#[cfg(feature = "ibatis")]
/// POST /api/validate-xml request body.
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
    /// Output format: `"sarif"` for SARIF 2.1.0, omitted for default JSON.
    #[serde(default)]
    pub format: Option<String>,
}

#[cfg(feature = "java")]
/// POST /api/validate-java request body.
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
    /// Output format: `"sarif"` for SARIF 2.1.0, omitted for default JSON.
    #[serde(default)]
    pub format: Option<String>,
}

// ─── Response types ───────────────────────────────────────────

/// GET /api/health response.
#[derive(Debug, serde::Serialize, ToSchema)]
#[non_exhaustive]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// POST /api/format response.
#[derive(Debug, serde::Serialize, ToSchema)]
#[non_exhaustive]
pub struct FormatResponse {
    pub formatted: String,
}

/// Single token info in tokenize response.
#[derive(Debug, serde::Serialize, ToSchema)]
#[non_exhaustive]
pub struct TokenInfo {
    #[serde(rename = "type")]
    pub token_type: String,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

/// POST /api/tokenize response.
#[derive(Debug, serde::Serialize, ToSchema)]
#[non_exhaustive]
pub struct TokenizeResponse {
    pub tokens: Vec<TokenInfo>,
}

/// POST /api/json2sql response.
#[derive(Debug, serde::Serialize, ToSchema)]
#[non_exhaustive]
pub struct Json2SqlResponse {
    pub statements: Vec<String>,
    pub count: usize,
}

/// POST /api/parse response.
///
/// `statements` and `errors` contain serialized AST which is too complex
/// for `ToSchema`, so they remain as `serde_json::Value`.
#[derive(Debug, serde::Serialize, ToSchema)]
#[non_exhaustive]
pub struct ParseResponse {
    pub statements: Vec<serde_json::Value>,
    pub errors: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_fingerprints: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lint_warnings: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lint_summary: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted_sql: Option<Vec<serde_json::Value>>,
}

/// Per-statement validation result for granular error attribution.
///
/// Each entry corresponds to one mapper statement (validate-xml),
/// Java SQL extraction (validate-java), or top-level SQL statement (validate).
/// Includes the method identifier, source line, extracted SQL, and
/// its own valid/error/warning breakdown so API consumers can locate
/// the exact source of validation failures.
#[derive(Debug, serde::Serialize, ToSchema)]
#[non_exhaustive]
pub struct StatementValidation {
    /// Statement or method identifier: mapper ID, procedure name, or className::methodName.
    pub method: String,
    /// Line number in the input source (XML, Java, or SQL file).
    pub line: usize,
    /// Statement type, e.g. "Select", "Insert", "CreateProcedure".
    #[serde(rename = "type")]
    pub sql_type: String,
    /// The extracted SQL text (may be empty for SQL-level validate).
    pub sql: String,
    /// Whether this specific statement/method is valid.
    pub valid: bool,
    /// Number of real errors (non-warning) in this statement.
    pub error_count: usize,
    /// Number of warnings in this statement.
    pub warning_count: usize,
    /// Real errors attributed to this statement (ParserError JSON).
    pub errors: Vec<serde_json::Value>,
    /// Warnings attributed to this statement (ParserError JSON).
    pub warnings: Vec<serde_json::Value>,
}

/// POST /api/validate response.
#[derive(Debug, serde::Serialize, ToSchema)]
#[non_exhaustive]
pub struct ValidateResponse {
    pub valid: bool,
    pub error_count: usize,
    pub warning_count: usize,
    pub errors: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undefined_variables: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_consistency_errors: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lint_warnings: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lint_summary: Option<serde_json::Value>,
    /// Whether strict validation mode was enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_mode: Option<bool>,
    /// Per-statement validation breakdown for granular error attribution.
    /// Present for validate-xml, validate-java, and validate (SQL) when
    /// the input contains identifiable statements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<StatementValidation>>,
}

// ─── Lint configuration ──────────────────────────────────────

/// Lint configuration accepted by parse/validate endpoints.
///
/// Mirrors the CLI lint flags so API consumers can customize rule behavior.
#[derive(Debug, Default, Deserialize, ToSchema)]
#[non_exhaustive]
pub struct LintConfigInput {
    /// Minimum warning level: prohibition, performance, caution, suggestion.
    #[serde(default)]
    pub min_level: Option<String>,
    /// Minimum confidence: full, partial.
    #[serde(default)]
    pub min_confidence: Option<String>,
    /// Suppress specific rule IDs.
    #[serde(default)]
    pub suppress: Vec<String>,
    /// P003 IN list size threshold (default: 500).
    #[serde(default)]
    pub in_list_threshold: Option<usize>,
    /// P014 subquery nesting depth limit (default: 3).
    #[serde(default)]
    pub subquery_depth_limit: Option<usize>,
    /// P007 non-equi join count limit (default: 2).
    #[serde(default)]
    pub non_equi_join_limit: Option<usize>,
}

impl LintConfigInput {
    /// Convert to `LintConfig`, starting from defaults.
    pub fn to_lint_config(&self) -> ogsql_parser::linter::LintConfig {
        use ogsql_parser::linter::{Confidence, LintConfig, WarningLevel};

        let mut config = LintConfig::default();

        if let Some(ref level) = self.min_level {
            config.min_level = match level.to_lowercase().as_str() {
                "prohibition" => WarningLevel::Prohibition,
                "performance" => WarningLevel::Performance,
                "caution" => WarningLevel::Caution,
                _ => WarningLevel::Suggestion,
            };
        }
        if let Some(ref conf) = self.min_confidence {
            config.min_confidence = match conf.to_lowercase().as_str() {
                "full" => Confidence::Full,
                _ => Confidence::Partial,
            };
        }
        if !self.suppress.is_empty() {
            config.suppress = self.suppress.clone();
        }
        if let Some(t) = self.in_list_threshold {
            config.in_list_threshold = t;
        }
        if let Some(t) = self.subquery_depth_limit {
            config.subquery_depth_limit = t;
        }
        if let Some(t) = self.non_equi_join_limit {
            config.non_equi_join_limit = t;
        }
        config
    }
}

// ─── Multipart form config types ────────────────────────────

/// Config JSON expected in the `config` field of a multipart validate request.
#[derive(Debug, Default, Deserialize)]
pub struct ValidateMultipartConfig {
    #[serde(default)]
    pub strict: Option<bool>,
    #[serde(default)]
    pub lint: Option<bool>,
    #[serde(default)]
    pub lint_config: Option<LintConfigInput>,
    /// Output format: `"sarif"` for SARIF 2.1.0, omitted for default JSON.
    #[serde(default)]
    pub format: Option<String>,
}

#[cfg(feature = "ibatis")]
/// Config JSON for multipart validate-xml request.
#[derive(Debug, Default, Deserialize)]
pub struct ValidateXmlMultipartConfig {
    #[serde(default)]
    pub strict: Option<bool>,
    #[serde(default)]
    pub lint: Option<bool>,
    #[serde(default)]
    pub lint_config: Option<LintConfigInput>,
    /// Output format: `"sarif"` for SARIF 2.1.0, omitted for default JSON.
    #[serde(default)]
    pub format: Option<String>,
}

#[cfg(feature = "java")]
/// Config JSON for multipart validate-java request.
#[derive(Debug, Default, Deserialize)]
pub struct ValidateJavaMultipartConfig {
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
    /// Output format: `"sarif"` for SARIF 2.1.0, omitted for default JSON.
    #[serde(default)]
    pub format: Option<String>,
}
