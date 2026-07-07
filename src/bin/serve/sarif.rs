//! SARIF 2.1.0 output builder for the validate API.
//!
//! Produces a standard Static Analysis Results Interchange Format log
//! from a [`ValidateResponse`] and the original input text.
//!
//! SARIF spec: <https://docs.oasis-open.org/sarif/sarif/v2.1.0/cs01/sarif-v2.1.0-cs01.html>

use serde::Serialize;

use super::schema::ValidateResponse;
use ogsql_parser::ast::SourceSpan;
use ogsql_parser::linter::{LintRuleEntry, SqlWarning, WarningLevel};
use ogsql_parser::{ParserError, SourceLocation};

// ─── SARIF 2.1.0 type definitions (subset) ────────────────────

/// Top-level SARIF log file.
#[derive(Debug, Serialize)]
pub struct SarifLog {
    #[serde(rename = "$schema")]
    schema: &'static str,
    version: &'static str,
    runs: Vec<Run>,
}

/// A single analysis run.
#[derive(Debug, Serialize)]
struct Run {
    tool: Tool,
    results: Vec<Result>,
}

/// Tool information.
#[derive(Debug, Serialize)]
struct Tool {
    driver: Driver,
}

/// The driver (analysis tool) metadata and rule catalog.
#[derive(Debug, Serialize)]
struct Driver {
    name: &'static str,
    version: &'static str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    rules: Vec<ReportingDescriptor>,
}

/// A rule descriptor for the `tool.driver.rules[]` catalog.
#[derive(Debug, Serialize)]
struct ReportingDescriptor {
    id: String,
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_description: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full_description: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_configuration: Option<ReportingConfiguration>,
}

/// Default severity configuration for a rule.
#[derive(Debug, Serialize)]
struct ReportingConfiguration {
    level: &'static str,
}

/// A single result (error, warning, or note).
#[derive(Debug, Serialize)]
struct Result {
    rule_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_index: Option<usize>,
    level: &'static str,
    message: Message,
    #[serde(skip_serializing_if = "Option::is_none")]
    locations: Option<Vec<Location>>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    properties: Option<serde_json::Value>,
}

/// A message text.
#[derive(Debug, Serialize)]
struct Message {
    text: String,
}

/// A location within a result.
#[derive(Debug, Serialize)]
struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    physical_location: Option<PhysicalLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logical_locations: Option<Vec<LogicalLocation>>,
}

/// A physical location (file + region).
#[derive(Debug, Serialize)]
struct PhysicalLocation {
    artifact_location: ArtifactLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<Region>,
}

/// An artifact (file) reference.
#[derive(Debug, Serialize)]
struct ArtifactLocation {
    uri: String,
}

/// A region (position span) within a file.
#[derive(Debug, Serialize)]
struct Region {
    #[serde(skip_serializing_if = "Option::is_none")]
    start_line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_column: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    byte_offset: Option<usize>,
}

/// A logical location (e.g. procedure name, mapper ID).
#[derive(Debug, Serialize)]
struct LogicalLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fully_qualified_name: Option<String>,
}

// ─── Constants ────────────────────────────────────────────────

const SARIF_SCHEMA: &str =
    "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/refs/heads/main/sarif-2.1/schema/sarif-2-1.json";
const SARIF_VERSION: &str = "2.1.0";

// ─── Builder ──────────────────────────────────────────────────

/// Build a SARIF log from a validate response.
pub fn build_sarif_log(
    response: &ValidateResponse,
    source_text: &str,
    rules: &[LintRuleEntry],
    tool_name: &'static str,
    tool_version: &'static str,
) -> SarifLog {
    let driver_rules = build_rule_catalog(rules);
    let mut results: Vec<Result> = Vec::new();

    // 1. Parser errors (SyntaxError, UnexpectedToken, etc.)
    for err in &response.errors {
        if let Ok(parsed) = serde_json::from_value::<ParserError>(err.clone()) {
            results.push(parser_error_to_result(&parsed, source_text, &driver_rules));
        }
    }

    // 2. Undefined variables (strict mode)
    if let Some(ref undefined_vars) = response.undefined_variables {
        for var in undefined_vars {
            results.push(undefined_var_to_result(var));
        }
    }

    // 3. Lint warnings
    if let Some(ref lint_warnings) = response.lint_warnings {
        for warning in lint_warnings {
            if let Ok(parsed) = serde_json::from_value::<SqlWarning>(warning.clone()) {
                results.push(sql_warning_to_result(&parsed, &driver_rules));
            }
        }
    }

    // 4. Per-statement validation breakdown is skipped here because it duplicates
    //    the errors/warnings already in response.errors (the handlers aggregate all
    //    per-statement errors into the top-level errors vec).  Including both would
    //    produce double-counted results in SARIF output.

    SarifLog {
        schema: SARIF_SCHEMA,
        version: SARIF_VERSION,
        runs: vec![Run {
            tool: Tool { driver: Driver { name: tool_name, version: tool_version, rules: driver_rules } },
            results,
        }],
    }
}

// ─── Rule catalog ─────────────────────────────────────────────

fn build_rule_catalog(rules: &[LintRuleEntry]) -> Vec<ReportingDescriptor> {
    rules
        .iter()
        .map(|rule| ReportingDescriptor {
            id: rule.id.to_string(),
            name: Some(rule.name.to_string()),
            short_description: Some(Message { text: rule.description.to_string() }),
            full_description: None,
            default_configuration: Some(ReportingConfiguration { level: warning_level_to_sarif(rule.level) }),
        })
        .collect()
}

// ─── Level mapping ────────────────────────────────────────────

fn warning_level_to_sarif(level: WarningLevel) -> &'static str {
    match level {
        WarningLevel::Prohibition => "error",
        WarningLevel::Performance => "warning",
        WarningLevel::Caution => "note",
        WarningLevel::Suggestion => "none",
    }
}

fn parser_severity(e: &ParserError) -> &'static str {
    if ogsql_parser::is_warning(e) {
        match e {
            ParserError::Warning { level, .. } => warning_level_to_sarif(*level),
            ParserError::ReservedKeywordAsIdentifier { .. } => "warning",
            _ => "error",
        }
    } else {
        "error"
    }
}

// ─── Converters ───────────────────────────────────────────────

fn parser_error_to_result(err: &ParserError, source_text: &str, rules: &[ReportingDescriptor]) -> Result {
    let (rule_id, loc, message, extra) = parser_error_fields(err, source_text);
    let rule_index = rules.iter().position(|r| r.id == rule_id);

    Result {
        level: parser_severity(err),
        rule_id,
        rule_index,
        message: Message { text: message },
        locations: loc,
        properties: extra,
    }
}

fn parser_error_fields(
    err: &ParserError,
    source_text: &str,
) -> (String, Option<Vec<Location>>, String, Option<serde_json::Value>) {
    match err {
        ParserError::UnexpectedToken { location, expected, got } => {
            let rule_id = "OGSQL/UNEXPECTED_TOKEN".to_string();
            let loc = source_location_to_locations(location, "api://input", source_text);
            let msg = format!("Unexpected token: expected {expected}, got {got}");
            (rule_id, loc, msg, None)
        }
        ParserError::UnexpectedEof { location, expected } => {
            let rule_id = "OGSQL/UNEXPECTED_EOF".to_string();
            let loc = source_location_to_locations(location, "api://input", source_text);
            let msg = format!("Unexpected end of input: expected {expected}");
            (rule_id, loc, msg, None)
        }
        ParserError::Warning { message, location, level } => {
            let rule_id = "OGSQL/PARSER_WARNING".to_string();
            let loc = source_location_to_locations(location, "api://input", source_text);
            let extra = serde_json::json!({ "warningLevel": level });
            (rule_id, loc, message.clone(), Some(extra))
        }
        ParserError::ReservedKeywordAsIdentifier { keyword, location } => {
            let rule_id = "OGSQL/RESERVED_KEYWORD".to_string();
            let loc = source_location_to_locations(location, "api://input", source_text);
            let msg = format!("'{keyword}' is a reserved keyword used as an identifier");
            (rule_id, loc, msg, None)
        }
        ParserError::UnsupportedSyntax { location, syntax, hint } => {
            let rule_id = "OGSQL/UNSUPPORTED_SYNTAX".to_string();
            let loc = source_location_to_locations(location, "api://input", source_text);
            let msg = if hint.is_empty() {
                format!("Unsupported syntax: {syntax}")
            } else {
                format!("Unsupported syntax: {syntax} — {hint}")
            };
            (rule_id, loc, msg, None)
        }
        ParserError::TokenizerError(te) => {
            let rule_id = "OGSQL/TOKENIZER_ERROR".to_string();
            let (loc, pos) = te_location_offset(te);
            let loc = loc.map(|(line, col)| {
                vec![Location {
                    physical_location: Some(PhysicalLocation {
                        artifact_location: ArtifactLocation { uri: "api://input".to_string() },
                        region: Some(Region {
                            start_line: Some(line),
                            start_column: Some(col),
                            byte_offset: Some(pos),
                        }),
                    }),
                    logical_locations: None,
                }]
            });
            let msg = format!("{te}");
            (rule_id, loc, msg, None)
        }
    }
}

/// Extract (line, col) and byte offset from TokenizerError.
fn te_location_offset(te: &ogsql_parser::TokenizerError) -> (Option<(usize, usize)>, usize) {
    let loc = match te {
        ogsql_parser::TokenizerError::UnterminatedString(loc)
        | ogsql_parser::TokenizerError::UnterminatedComment(loc)
        | ogsql_parser::TokenizerError::UnterminatedDollarString(loc)
        | ogsql_parser::TokenizerError::UnterminatedQuotedIdentifier(loc)
        | ogsql_parser::TokenizerError::InvalidCharacter { location: loc, .. } => loc,
        ogsql_parser::TokenizerError::UnexpectedEof { location: loc, .. } => loc,
    };
    ((loc.line > 0 || loc.column > 0 || loc.offset > 0).then_some((loc.line, loc.column)), loc.offset)
}

fn undefined_var_to_result(var: &serde_json::Value) -> Result {
    let name = var.get("variable_name").and_then(|v| v.as_str()).unwrap_or("<unknown>");
    let msg = format!("Undefined variable or function reference: {name}");

    // UndefinedVariableError.location is Option<SourceSpan> — extract .start as SourceLocation
    let loc = var
        .get("location")
        .and_then(|v| serde_json::from_value::<SourceSpan>(v.clone()).ok())
        .and_then(|span| source_location_to_locations(&span.start, "api://input", ""));

    Result {
        level: "error",
        rule_id: "OGSQL/UNDEFINED_VARIABLE".to_string(),
        rule_index: None,
        message: Message { text: msg },
        locations: loc,
        properties: None,
    }
}

fn sql_warning_to_result(warning: &SqlWarning, rules: &[ReportingDescriptor]) -> Result {
    let rule_index = rules.iter().position(|r| r.id == warning.rule_id);
    let loc = source_location_to_locations(&warning.location, "api://input", "");

    let mut properties = serde_json::json!({
        "ruleName": warning.rule_name,
        "confidence": warning.confidence,
    });

    // Attach suggestion if present
    if let Some(ref suggestion) = warning.suggestion {
        properties
            .as_object_mut()
            .unwrap()
            .insert("suggestion".to_string(), serde_json::Value::String(suggestion.clone()));
    }
    if let Some(ref gaussdb_ref) = warning.gaussdb_ref {
        properties
            .as_object_mut()
            .unwrap()
            .insert("gaussdbRef".to_string(), serde_json::Value::String(gaussdb_ref.clone()));
    }

    Result {
        level: warning_level_to_sarif(warning.level),
        rule_id: warning.rule_id.clone(),
        rule_index,
        message: Message { text: warning.message.clone() },
        locations: loc,
        properties: Some(properties),
    }
}

// ─── Position helpers ─────────────────────────────────────────

fn source_location_to_locations(loc: &SourceLocation, uri: &str, _source_text: &str) -> Option<Vec<Location>> {
    // SourceLocation has line=1, column=1, offset=0 as default.
    // If all zeros (no real position), skip the location.
    if loc.line == 0 && loc.column == 0 && loc.offset == 0 {
        return None;
    }
    Some(vec![Location {
        physical_location: Some(PhysicalLocation {
            artifact_location: ArtifactLocation { uri: uri.to_string() },
            region: Some(Region {
                start_line: if loc.line > 0 { Some(loc.line) } else { None },
                start_column: if loc.column > 0 { Some(loc.column) } else { None },
                byte_offset: Some(loc.offset),
            }),
        }),
        logical_locations: None,
    }])
}

fn byte_offset_to_line_col(source: &str, offset: usize) -> Option<(usize, usize)> {
    if offset >= source.len() {
        return None;
    }
    let line = source[..offset].chars().filter(|c| *c == '\n').count() + 1;
    let last_newline = source[..offset].rfind('\n');
    let col = match last_newline {
        Some(pos) => source[pos + 1..offset].chars().count() + 1,
        None => source[..offset].chars().count() + 1,
    };
    Some((line, col))
}
