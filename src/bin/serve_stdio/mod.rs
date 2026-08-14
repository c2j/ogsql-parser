//! `serve-stdio` — line-delimited JSON (NDJSON) protocol over stdin/stdout.
//!
//! A long-lived, lightweight request/response channel for embedded clients
//! (Java/Python/Node) that spawn `ogsql serve-stdio` as a child process.
//! Protocol spec: docs/stdio-protocol.md
//!
//! Framing: one JSON object per line on stdin and stdout (UTF-8, LF). serde_json
//! escapes newlines inside string values, so lines are unambiguous. Requests are
//! processed strictly in order and responses are written in the same order.

use serde::Deserialize;
use serde_json::{Map, Value};
use std::io::{BufRead, BufWriter, Write};

/// Bumped whenever the wire protocol changes incompatibly.
const PROTOCOL_VERSION: u32 = 1;

/// Upper bound on a single request line, to bound memory use.
const MAX_LINE_BYTES: usize = 16 * 1024 * 1024;

/// Maximum allowed parenthesized expression nesting. The recursive-descent parser
/// overflows the stack well below 50 nested parens (observed ~50 in debug builds),
/// and a stack overflow aborts the process and cannot be caught — so the protocol
/// server rejects overly deep nesting up front instead. 32 leaves a safe margin
/// across platforms/stack sizes while covering all realistic SQL.
const MAX_PAREN_NESTING: usize = 32;

const OPS: &[&str] = &["hello", "ping", "shutdown", "parse", "format", "tokenize", "validate", "json2sql"];

#[derive(Deserialize)]
struct Envelope {
    id: i64,
    op: String,
    #[serde(flatten)]
    params: Map<String, Value>,
}

fn ok(id: i64, result: Value) -> Value {
    serde_json::json!({ "id": id, "ok": true, "result": result })
}

fn err(id: i64, code: &str, message: impl Into<String>) -> Value {
    serde_json::json!({ "id": id, "ok": false, "error": { "code": code, "message": message.into() } })
}

// ─── param helpers ───────────────────────────────────────────

fn str_param<'a>(p: &'a Map<String, Value>, key: &str) -> Option<&'a str> {
    match p.get(key) {
        Some(Value::String(s)) => Some(s),
        _ => None,
    }
}

fn bool_param(p: &Map<String, Value>, key: &str) -> Option<bool> {
    match p.get(key) {
        Some(Value::Bool(b)) => Some(*b),
        _ => None,
    }
}

fn usize_param(p: &Map<String, Value>, key: &str) -> Option<usize> {
    match p.get(key) {
        Some(Value::Number(n)) => n.as_u64().map(|u| u as usize),
        _ => None,
    }
}

fn require_str<'a>(p: &'a Map<String, Value>, key: &str, id: i64) -> Result<&'a str, Value> {
    str_param(p, key).ok_or_else(|| err(id, "BAD_PARAM", format!("missing required param: {key}")))
}

/// Maximum '(' … ')' nesting depth over the token stream.
fn paren_nesting_depth(sql: &str) -> Result<usize, String> {
    let tokens = ogsql_parser::Tokenizer::new(sql).tokenize().map_err(|e| e.to_string())?;
    let mut depth = 0usize;
    let mut max_depth = 0usize;
    for t in &tokens {
        match &t.token {
            ogsql_parser::Token::LParen => {
                depth += 1;
                max_depth = max_depth.max(depth);
            }
            ogsql_parser::Token::RParen => {
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
    }
    Ok(max_depth)
}

/// Reject input that could stack-overflow the parser before it reaches it.
/// Tokenization failures are left to the parse itself to report.
fn guard_nesting(sql: &str, id: i64) -> Result<(), Value> {
    match paren_nesting_depth(sql) {
        Ok(d) if d > MAX_PAREN_NESTING => {
            Err(err(id, "TOO_DEEP", format!("expression nesting depth {d} exceeds limit {MAX_PAREN_NESTING}")))
        }
        _ => Ok(()),
    }
}

// ─── ops ─────────────────────────────────────────────────────

fn handle_parse(id: i64, p: &Map<String, Value>) -> Value {
    let sql = match require_str(p, "sql", id) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let preserve_comments = bool_param(p, "preserve_comments").unwrap_or(false);
    let mybatis = bool_param(p, "mybatis").unwrap_or(false);
    if let Err(e) = guard_nesting(sql, id) {
        return e;
    }

    let output = crate::parse_input(sql, preserve_comments, mybatis);

    let output = match p.get("procedure") {
        Some(Value::String(proc)) => match crate::filter_output_by_procedure(output, proc) {
            Ok(o) => o,
            Err(m) => return err(id, "NOT_FOUND", m),
        },
        _ => output,
    };

    let statements: Vec<Value> =
        output.statements.iter().map(|si| serde_json::to_value(si).unwrap_or(Value::Null)).collect();
    let errors: Vec<Value> = output.errors.iter().map(|e| serde_json::to_value(e).unwrap_or(Value::Null)).collect();
    let comments = if output.comments.is_empty() {
        Value::Null
    } else {
        serde_json::to_value(&output.comments).unwrap_or(Value::Null)
    };

    let all_stmts: Vec<_> = output.statements.iter().map(|si| si.statement.clone()).collect();
    let fingerprints = ogsql_parser::compute_query_fingerprints(&all_stmts);
    let fingerprints =
        if fingerprints.is_empty() { Value::Null } else { serde_json::to_value(&fingerprints).unwrap_or(Value::Null) };

    ok(
        id,
        serde_json::json!({
            "statements": statements,
            "errors": errors,
            "query_fingerprints": fingerprints,
            "comments": comments,
        }),
    )
}

fn handle_format(id: i64, p: &Map<String, Value>) -> Value {
    let sql = match require_str(p, "sql", id) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mybatis = bool_param(p, "mybatis").unwrap_or(false);
    let indent = usize_param(p, "indent").unwrap_or(2);
    let keyword_case = str_param(p, "keyword_case").unwrap_or("preserve");
    let comma_style = str_param(p, "comma_style").unwrap_or("trailing");
    let line_width = usize_param(p, "line_width").unwrap_or(120);
    let uppercase = bool_param(p, "uppercase").unwrap_or(false);
    let no_select_newline = bool_param(p, "no_select_newline").unwrap_or(false);
    let no_logical_newline = bool_param(p, "no_logical_newline").unwrap_or(false);
    let no_semicolon_newline = bool_param(p, "no_semicolon_newline").unwrap_or(false);
    if let Err(e) = guard_nesting(sql, id) {
        return e;
    }

    match crate::format_sql_to_string(
        sql,
        mybatis,
        indent,
        keyword_case,
        comma_style,
        line_width,
        uppercase,
        no_select_newline,
        no_logical_newline,
        no_semicolon_newline,
    ) {
        Ok(formatted) => ok(id, serde_json::json!({ "sql": formatted })),
        Err(m) => err(id, "INVALID_SQL", m),
    }
}

fn handle_tokenize(id: i64, p: &Map<String, Value>) -> Value {
    let sql = match require_str(p, "sql", id) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let preserve_comments = bool_param(p, "preserve_comments").unwrap_or(false);
    let mybatis = bool_param(p, "mybatis").unwrap_or(false);

    let mut tokenizer = ogsql_parser::Tokenizer::new(sql);
    if preserve_comments {
        tokenizer = tokenizer.preserve_comments(true);
    }
    if mybatis {
        tokenizer = tokenizer.mybatis_params(true);
    }
    let tokens = match tokenizer.tokenize() {
        Ok(t) => t,
        Err(e) => return err(id, "TOKENIZE_ERROR", e.to_string()),
    };
    let info: Vec<Value> = tokens
        .iter()
        .map(|t| {
            let (token_type, value) = crate::token_display(t);
            serde_json::json!({
                "type": token_type,
                "value": value,
                "line": t.location.line,
                "column": t.location.column,
            })
        })
        .collect();
    ok(id, serde_json::json!({ "tokens": info }))
}

fn handle_validate(id: i64, p: &Map<String, Value>) -> Value {
    let sql = match require_str(p, "sql", id) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mybatis = bool_param(p, "mybatis").unwrap_or(false);
    let strict = bool_param(p, "strict").unwrap_or(false);
    if let Err(e) = guard_nesting(sql, id) {
        return e;
    }

    let (stmts, errors, pkg_errors, var_errors) = crate::validate_sql(sql, mybatis, &[], strict);

    let statements: Vec<Value> = stmts.iter().map(|si| serde_json::to_value(si).unwrap_or(Value::Null)).collect();
    let errors_json: Vec<Value> = errors.iter().map(|e| serde_json::to_value(e).unwrap_or(Value::Null)).collect();
    let pkg_json: Vec<Value> = pkg_errors.iter().map(|e| serde_json::to_value(e).unwrap_or(Value::Null)).collect();
    let var_json: Vec<Value> = var_errors.iter().map(|e| serde_json::to_value(e).unwrap_or(Value::Null)).collect();

    // Warnings and reserved-keyword-as-identifier are not hard failures.
    let valid = !errors.iter().any(|e| !ogsql_parser::is_warning(e)) && pkg_errors.is_empty() && var_errors.is_empty();

    ok(
        id,
        serde_json::json!({
            "valid": valid,
            "statements": statements,
            "errors": errors_json,
            "package_errors": pkg_json,
            "undefined_variable_errors": var_json,
        }),
    )
}

fn handle_json2sql(id: i64, p: &Map<String, Value>) -> Value {
    let json = match require_str(p, "json", id) {
        Ok(s) => s,
        Err(e) => return e,
    };
    match crate::json2sql_to_strings(json) {
        Ok(parts) => ok(id, serde_json::json!({ "sql": parts.join(";\n") })),
        Err(m) => err(id, "BAD_JSON", m),
    }
}

fn dispatch(env: &Envelope) -> (Value, bool) {
    let id = env.id;
    match env.op.as_str() {
        "hello" => (
            ok(
                id,
                serde_json::json!({
                    "version": env!("CARGO_PKG_VERSION"),
                    "protocol": PROTOCOL_VERSION,
                    "ops": OPS,
                }),
            ),
            false,
        ),
        "ping" => (ok(id, serde_json::json!({ "pong": true })), false),
        "shutdown" => (ok(id, serde_json::json!({ "bye": true })), true),
        "parse" => (handle_parse(id, &env.params), false),
        "format" => (handle_format(id, &env.params), false),
        "tokenize" => (handle_tokenize(id, &env.params), false),
        "validate" => (handle_validate(id, &env.params), false),
        "json2sql" => (handle_json2sql(id, &env.params), false),
        other => (err(id, "UNKNOWN_OP", format!("unknown op: {other}")), false),
    }
}

/// Main loop: read one JSON request per line, reply with one JSON response per line.
pub(crate) fn run() {
    eprintln!("ogsql: serve-stdio started (protocol v{PROTOCOL_VERSION})");
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let reader = std::io::BufReader::new(stdin.lock());
    let mut out = BufWriter::new(stdout.lock());

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("serve-stdio: stdin read error: {e}");
                break;
            }
        };
        if line.trim().is_empty() {
            continue;
        }
        if line.len() > MAX_LINE_BYTES {
            let _ =
                writeln!(out, "{}", err(-1, "LINE_TOO_LONG", format!("request line exceeds {MAX_LINE_BYTES} bytes")));
            let _ = out.flush();
            continue;
        }

        let env: Envelope = match serde_json::from_str(&line) {
            Ok(e) => e,
            Err(e) => {
                let _ = writeln!(out, "{}", err(-1, "PROTOCOL_ERROR", format!("invalid JSON: {e}")));
                let _ = out.flush();
                continue;
            }
        };

        // A panic inside a single request must not kill the server loop.
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| dispatch(&env)));
        let (response, shutdown) = match outcome {
            Ok(v) => v,
            Err(_) => (err(env.id, "INTERNAL_ERROR", "internal parser panic"), false),
        };
        let _ = writeln!(out, "{}", response);
        let _ = out.flush();
        if shutdown {
            break;
        }
    }
    let _ = out.flush();
}
