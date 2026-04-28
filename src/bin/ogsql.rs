use std::io::Read as _;

use clap::{Parser as ClapParser, Subcommand};
use ogsql_parser::*;
use serde::Serialize;

#[derive(ClapParser)]
#[command(name = "ogsql", version, about = "openGauss/GaussDB SQL Parser")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short = 'f', long, global = true)]
    file: Option<String>,

    #[arg(short = 'j', long, global = true)]
    json: bool,

    #[arg(short = 'v', long, global = true)]
    verbose: bool,

    #[arg(long = "schema-json", global = true)]
    schema_json: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Format SQL statements with standardized keyword casing / 格式化 SQL 语句
    Format,
    /// Parse SQL into AST and print the abstract syntax tree / 解析 SQL 为 AST
    Parse,
    /// Convert JSON (from `parse -j`) back to SQL / 将 JSON（parse -j 的输出）还原为 SQL
    #[command(name = "json2sql")]
    JsonToSql,
    /// Tokenize SQL into a list of tokens / 将 SQL 分词为 token 列表
    Tokenize,
    /// Validate SQL syntax and report errors / 校验 SQL 语法
    Validate,
    #[cfg(feature = "serve")]
    /// Start an HTTP API server for parsing SQL / 启动 HTTP API 服务器
    Serve {
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
    },
    #[cfg(feature = "tui")]
    /// Launch an interactive terminal UI playground / 启动交互式终端演练场
    Playground,
    #[cfg(feature = "ibatis")]
    /// Parse iBatis/MyBatis XML mapper file / 解析 iBatis XML mapper 文件
    #[command(name = "parse-xml")]
    ParseXml {
        /// Recursively scan directory for XML files / 递归扫描目录中的 XML 文件
        #[arg(short = 'd', long = "dir")]
        dir: Option<String>,
        /// Output in CSV format / 以 CSV 格式输出
        #[arg(long = "csv")]
        csv: bool,
    },
    #[cfg(feature = "java")]
    /// Extract and parse SQL from Java source files / 从 Java 源文件中提取并解析 SQL
    #[command(name = "parse-java")]
    ParseJava {
        #[arg(long = "extra-sql-methods", value_delimiter = ',')]
        extra_sql_methods: Vec<String>,
        /// Recursively scan directory for Java files / 递归扫描目录中的 Java 文件
        #[arg(short = 'd', long = "dir")]
        dir: Option<String>,
        /// Output in CSV format / 以 CSV 格式输出
        #[arg(long = "csv")]
        csv: bool,
    },
}

macro_rules! die {
    ($($t:tt)*) => {{ eprintln!($($t)*); std::process::exit(1); }};
}

fn annotate_builtin_functions(_value: &mut serde_json::Value) {}

fn read_input(file: Option<&str>) -> String {
    match file {
        Some(path) => {
            let bytes =
                std::fs::read(path).unwrap_or_else(|e| die!("Error reading {}: {}", path, e));
            token::decode_sql_file(&bytes)
                .unwrap_or_else(|e| die!("Error decoding {}: {}", path, e))
                .0
        }
        None => {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .unwrap_or_else(|e| die!("Error reading stdin: {}", e));
            buf
        }
    }
}

fn parse_input(sql: &str) -> (Vec<StatementInfo>, Vec<ParserError>) {
    let tokens = match Tokenizer::new(sql).tokenize() {
        Ok(t) => t,
        Err(e) => return (vec![], vec![ParserError::TokenizerError(e)]),
    };
    let mut parser = Parser::with_source(tokens, sql.to_string());
    let infos = parser.parse_with_text();
    (infos, parser.errors().to_vec())
}

fn token_display(t: &TokenWithSpan) -> (String, String) {
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
        other => ("Other".into(), format!("{:?}", other)),
    }
}

#[derive(Serialize)]
struct TokenInfo {
    #[serde(rename = "type")]
    token_type: String,
    value: String,
    line: usize,
    column: usize,
}

fn cmd_format(cli: &Cli) {
    let sql = read_input(cli.file.as_deref());
    let tokens = match Tokenizer::new(&sql).preserve_comments(true).tokenize() {
        Ok(t) => t,
        Err(e) => die!("Tokenization error: {}", e),
    };
    let formatted = token_formatter::TokenFormatter::new(&sql, tokens).format();

    if cli.json {
        let out = serde_json::json!({
            "formatted": formatted,
        });
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        println!("{}", formatted);
        if !formatted.ends_with('\n') {
            println!();
        }
    }
}

fn cmd_parse(cli: &Cli) {
    let sql = read_input(cli.file.as_deref());
    let (stmts, errors) = parse_input(&sql);

    if cli.json {
        let stmt_values: Vec<serde_json::Value> = stmts
            .iter()
            .map(|si| {
                let mut obj = serde_json::to_value(si).unwrap();
                if let Some(block) = extract_pl_block(&si.statement) {
                    let report = ogsql_parser::analyze_pl_block(block);
                    if !report.execute_findings.is_empty() {
                        obj.as_object_mut().unwrap().insert(
                            "dynamic_sql_analysis".to_string(),
                            serde_json::json!(report),
                        );
                    }
                    let tx_report = ogsql_parser::analyze_transactions(block);
                    obj.as_object_mut().unwrap().insert(
                        "transaction_analysis".to_string(),
                        serde_json::json!(tx_report),
                    );
                    if let Some(ref schema_path) = cli.schema_json {
                        match ogsql_parser::load_schema(schema_path) {
                            Ok(schema) => {
                                let schema_report = ogsql_parser::resolve_schema(block, &schema);
                                obj.as_object_mut().unwrap().insert(
                                    "schema_resolution".to_string(),
                                    serde_json::json!(schema_report),
                                );
                            }
                            Err(e) => eprintln!("Warning: {}", e),
                        }
                    }
                }
                annotate_builtin_functions(&mut obj);
                obj
            })
            .collect();

        let all_stmts: Vec<_> = stmts.iter().map(|si| si.statement.clone()).collect();
        let fingerprints = ogsql_parser::compute_query_fingerprints(&all_stmts);

        let mut out = serde_json::json!({
            "statements": stmt_values,
            "errors": errors,
        });
        if !fingerprints.is_empty() {
            out.as_object_mut().unwrap().insert(
                "query_fingerprints".to_string(),
                serde_json::json!(fingerprints),
            );
        }
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        for stmt in &stmts {
            println!("{:#?}", stmt);
        }
        if !errors.is_empty() {
            let warnings: Vec<_> = errors.iter().filter(|e| is_warning(e)).collect();
            let real_errors: Vec<_> = errors.iter().filter(|e| !is_warning(e)).collect();
            if !real_errors.is_empty() {
                eprintln!("\n{} error(s):", real_errors.len());
                for e in &real_errors {
                    eprintln!("  {}", e);
                }
                if cli.verbose {
                    write_error_log(&sql, cli.file.as_deref(), &stmts, &real_errors);
                }
            }
            if !warnings.is_empty() {
                eprintln!("\n{} warning(s):", warnings.len());
                for w in &warnings {
                    eprintln!("  {}", w);
                }
            }
        }
    }
}

fn extract_pl_block(
    stmt: &ogsql_parser::Statement,
) -> Option<&ogsql_parser::ast::plpgsql::PlBlock> {
    use ogsql_parser::Statement;
    match stmt {
        Statement::Do(d) => d.block.as_ref(),
        Statement::AnonyBlock(ab) => Some(&ab.block),
        Statement::CreateFunction(cf) => cf.block.as_ref(),
        Statement::CreateProcedure(cp) => cp.block.as_ref(),
        _ => None,
    }
}

fn cmd_tokenize(cli: &Cli) {
    let sql = read_input(cli.file.as_deref());
    let tokens = match Tokenizer::new(&sql).tokenize() {
        Ok(t) => t,
        Err(e) => die!("Tokenizer error: {}", e),
    };

    if cli.json {
        let info: Vec<TokenInfo> = tokens
            .iter()
            .map(|t| {
                let (token_type, value) = token_display(t);
                TokenInfo {
                    token_type,
                    value,
                    line: t.location.line,
                    column: t.location.column,
                }
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&info).unwrap());
    } else {
        for t in &tokens {
            println!("{:?}", t.token);
        }
    }
}

fn cmd_json2sql(cli: &Cli) {
    let input = read_input(cli.file.as_deref());

    let json_value: serde_json::Value = match serde_json::from_str(&input) {
        Ok(v) => v,
        Err(e) => die!("Invalid JSON: {}", e),
    };

    let arr = match json_value.get("statements") {
        Some(v) => v,
        None => die!("Expected JSON object with \"statements\" array"),
    };

    let statements: Vec<Statement> = if let serde_json::Value::Array(items) = arr {
        items
            .iter()
            .filter_map(|v| {
                if v.get("sql_text").is_some() {
                    serde_json::from_value::<StatementInfo>(v.clone())
                        .ok()
                        .map(|si| si.statement)
                } else {
                    serde_json::from_value::<Statement>(v.clone()).ok()
                }
            })
            .collect()
    } else {
        die!("\"statements\" must be an array");
    };

    if statements.is_empty() {
        die!("No valid statements found in JSON");
    }

    let formatter = SqlFormatter::new();
    let formatted: Vec<String> = statements
        .iter()
        .map(|s| formatter.format_statement(s))
        .collect();

    if cli.json {
        let out = serde_json::json!({
            "statements": formatted,
            "count": formatted.len(),
        });
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        println!("{}", formatted.join(";\n"));
        println!(";");
    }
}

fn is_warning(e: &ogsql_parser::ParserError) -> bool {
    matches!(
        e,
        ogsql_parser::ParserError::Warning { .. }
            | ogsql_parser::ParserError::ReservedKeywordAsIdentifier { .. }
    )
}

fn cmd_validate(cli: &Cli) {
    let sql = read_input(cli.file.as_deref());
    let (stmts, errors) = parse_input(&sql);

    if cli.json {
        let warnings: Vec<_> = errors.iter().filter(|e| is_warning(e)).collect();
        let real_errors: Vec<_> = errors.iter().filter(|e| !is_warning(e)).collect();
        let out = serde_json::json!({
            "valid": real_errors.is_empty(),
            "error_count": real_errors.len(),
            "warning_count": warnings.len(),
            "errors": errors,
        });
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        let real_errors: Vec<_> = errors.iter().filter(|e| !is_warning(e)).collect();
        let warnings: Vec<_> = errors.iter().filter(|e| is_warning(e)).collect();
        if real_errors.is_empty() && warnings.is_empty() {
            println!("VALID");
        } else if real_errors.is_empty() {
            println!("VALID ({} warning(s)):", warnings.len());
            for w in &warnings {
                eprintln!("  warning: {}", w);
            }
        } else {
            println!(
                "INVALID ({} error(s), {} warning(s)):",
                real_errors.len(),
                warnings.len()
            );
            for e in &real_errors {
                eprintln!("  error: {}", e);
            }
            for w in &warnings {
                eprintln!("  warning: {}", w);
            }
            if cli.verbose {
                write_error_log(&sql, cli.file.as_deref(), &stmts, &real_errors);
            }
            std::process::exit(1);
        }
    }
}

fn write_error_log(
    source: &str,
    file_path: Option<&str>,
    stmts: &[StatementInfo],
    errors: &[&ParserError],
) {
    use std::io::Write;
    let mut file = match std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("error.log")
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("  warning: cannot create error.log: {}", e);
            return;
        }
    };

    let source_lines: Vec<&str> = source.lines().collect();

    let mut groups: Vec<(usize, Option<String>, usize, usize, Vec<usize>)> = Vec::new();

    for (err_idx, err) in errors.iter().enumerate() {
        let (line, _col) = match err {
            ParserError::UnexpectedToken { location, .. } => (location.line, location.column),
            ParserError::UnexpectedEof { location, .. } => (location.line, location.column),
            ParserError::TokenizerError(_) => (0, 0),
            ParserError::ReservedKeywordAsIdentifier { location, .. } => {
                (location.line, location.column)
            }
            _ => (0, 0),
        };
        if line == 0 {
            groups.push((usize::MAX, None, 0, 0, vec![err_idx])); // sentinel: no line info
        } else if let Some(si_idx) = stmts
            .iter()
            .position(|si| line >= si.start_line && line <= si.end_line)
        {
            let si = &stmts[si_idx];
            let (sub_name, sub_start, sub_end) = find_error_sub_item(&si.statement, line);
            if let Some(pos) = groups.iter().position(|(idx, sn, ss, se, _)| {
                *idx == si_idx && *sn == sub_name && *ss == sub_start && *se == sub_end
            }) {
                groups[pos].4.push(err_idx);
            } else {
                groups.push((si_idx, sub_name, sub_start, sub_end, vec![err_idx]));
            }
        } else {
            groups.push((usize::MAX, None, 0, 0, vec![err_idx])); // sentinel: unmatched line
        }
    }

    let context_radius: usize = 5;

    for (si_idx, sub_name, sub_start, sub_end, err_indices) in &groups {
        if let Some(fp) = file_path {
            let _ = writeln!(file, "File: {}", fp);
        }
        for &err_idx in err_indices {
            let _ = writeln!(file, "Error: {}", errors[err_idx]);
        }

        if *si_idx != usize::MAX {
            let si = &stmts[*si_idx];

            let all_lines: Vec<&str> = if !si.sql_text.is_empty() {
                si.sql_text.lines().collect()
            } else {
                // sql_text 为空 (Package Body / DDL) 时回退到完整源码
                let start = si.start_line.saturating_sub(1);
                let end = si.end_line.min(source_lines.len());
                source_lines[start..end].to_vec()
            };
            let stmt_start = si.start_line;

            let (label_start, label_end, omitted_after) =
                if *sub_start > 0 && *sub_end > 0 {
                    (*sub_start, *sub_end, sub_end - sub_start + 1)
                } else {
                    (si.start_line, si.end_line, all_lines.len())
                };

            let error_lines: Vec<usize> = err_indices
                .iter()
                .map(|&ei| error_line(errors[ei]))
                .filter(|&l| l > 0)
                .collect();

            let label_relative_start = label_start.saturating_sub(stmt_start);
            let label_len = label_end.saturating_sub(label_start) + 1;

            let mut ctx_start = label_relative_start.saturating_sub(context_radius);
            let mut ctx_end = (label_relative_start + label_len).min(all_lines.len());

            for &eline in &error_lines {
                let rel = eline.saturating_sub(stmt_start);
                ctx_start = ctx_start.min(rel.saturating_sub(context_radius));
                ctx_end = ctx_end.max((rel + 1).min(all_lines.len()));
            }

            let ctx_lines = &all_lines[ctx_start..ctx_end];
            let omitted_before = label_relative_start.saturating_sub(ctx_start);
            let omitted_after_actual = all_lines.len().saturating_sub(ctx_end);

            if let Some(name) = sub_name {
                let _ = writeln!(
                    file,
                    "In {} (line {}-{} of {}-line statement):",
                    name, sub_start, sub_end, omitted_after
                );
            } else {
                let _ = writeln!(file, "Statement (line {}-{}):", si.start_line, si.end_line);
            }

            for (i, l) in ctx_lines.iter().enumerate() {
                let abs_line = ctx_start + i + stmt_start;
                if error_lines.contains(&abs_line) {
                    let _ = writeln!(file, "  {:>4} |> {}", abs_line, l);
                } else {
                    let _ = writeln!(file, "  {:>4} |  {}", abs_line, l);
                }
            }
            if omitted_before > context_radius || omitted_after_actual > context_radius {
                let _ = writeln!(
                    file,
                    "  ... ({} lines omitted) ...",
                    omitted_before.saturating_sub(context_radius)
                        + omitted_after_actual.saturating_sub(context_radius)
                );
            }
        }
        let _ = writeln!(file, "{}", "-".repeat(60));
    }
    eprintln!("  error details written to error.log");
}

fn error_line(err: &ParserError) -> usize {
    match err {
        ParserError::UnexpectedToken { location, .. } => location.line,
        ParserError::UnexpectedEof { location, .. } => location.line,
        ParserError::ReservedKeywordAsIdentifier { location, .. } => location.line,
        ParserError::Warning { location, .. } => location.line,
        _ => 0,
    }
}

fn find_error_sub_item(stmt: &Statement, error_line: usize) -> (Option<String>, usize, usize) {
    use ogsql_parser::ast::PackageItem;
    match stmt {
        Statement::CreatePackageBody(pkg) => {
            for item in &pkg.items {
                match item {
                    PackageItem::Procedure(p)
                        if p.start_line > 0
                            && error_line >= p.start_line
                            && error_line <= p.end_line =>
                    {
                        let name = p.name.join(".");
                        return (Some(format!("PROCEDURE {}", name)), p.start_line, p.end_line);
                    }
                    PackageItem::Function(f)
                        if f.start_line > 0
                            && error_line >= f.start_line
                            && error_line <= f.end_line =>
                    {
                        let name = f.name.join(".");
                        return (Some(format!("FUNCTION {}", name)), f.start_line, f.end_line);
                    }
                    _ => {}
                }
            }
            (None, 0, 0)
        }
        Statement::CreatePackage(pkg) => {
            for item in &pkg.items {
                match item {
                    PackageItem::Procedure(p)
                        if p.start_line > 0
                            && error_line >= p.start_line
                            && error_line <= p.end_line =>
                    {
                        let name = p.name.join(".");
                        return (Some(format!("PROCEDURE {}", name)), p.start_line, p.end_line);
                    }
                    PackageItem::Function(f)
                        if f.start_line > 0
                            && error_line >= f.start_line
                            && error_line <= f.end_line =>
                    {
                        let name = f.name.join(".");
                        return (Some(format!("FUNCTION {}", name)), f.start_line, f.end_line);
                    }
                    _ => {}
                }
            }
            (None, 0, 0)
        }
        _ => (None, 0, 0),
    }
}

#[cfg(feature = "serve")]
mod api {
    use axum::routing::{get, post};
    use axum::Json;
    use axum::Router;
    use serde::Deserialize;
    use utoipa::{OpenApi, ToSchema};

    #[derive(OpenApi)]
    #[openapi(
        paths(health, handle_parse, handle_format, handle_tokenize, handle_validate, handle_json2sql),
        components(schemas(SqlInput, JsonInput)),
        tags(
            (name = "ogsql", description = "openGauss/GaussDB SQL Parser API")
        )
    )]
    pub struct ApiDoc;

    #[derive(Deserialize, ToSchema)]
    pub struct SqlInput {
        pub sql: String,
    }

    #[derive(Deserialize, ToSchema)]
    pub struct JsonInput {
        pub json: String,
    }

    /// Health check endpoint
    #[utoipa::path(
        get,
        path = "/api/health",
        tag = "ogsql",
        responses((status = 200, description = "Service is healthy"))
    )]
    pub async fn health() -> Json<serde_json::Value> {
        Json(serde_json::json!({"status": "ok"}))
    }

    /// Parse SQL into AST
    #[utoipa::path(
        post,
        path = "/api/parse",
        tag = "ogsql",
        request_body = SqlInput,
        responses((status = 200, description = "Parsed AST result"))
    )]
    pub async fn handle_parse(Json(input): Json<SqlInput>) -> Json<serde_json::Value> {
        let (stmts, errors) = super::parse_input(&input.sql);
        let all_stmts: Vec<_> = stmts.iter().map(|si| si.statement.clone()).collect();
        let fingerprints = ogsql_parser::compute_query_fingerprints(&all_stmts);
        let mut out = serde_json::json!({"statements": stmts, "errors": errors});
        if !fingerprints.is_empty() {
            out.as_object_mut().unwrap().insert(
                "query_fingerprints".to_string(),
                serde_json::json!(fingerprints),
            );
        }
        Json(out)
    }

    /// Format SQL statements
    #[utoipa::path(
        post,
        path = "/api/format",
        tag = "ogsql",
        request_body = SqlInput,
        responses((status = 200, description = "Formatted SQL result"))
    )]
    pub async fn handle_format(Json(input): Json<SqlInput>) -> Json<serde_json::Value> {
        let (stmts, errors) = super::parse_input(&input.sql);
        let formatter = ogsql_parser::SqlFormatter::new();
        let formatted: Vec<String> = stmts
            .iter()
            .map(|si| formatter.format_statement(&si.statement))
            .collect();
        Json(serde_json::json!({
            "formatted": formatted.join(";\n"),
            "error_count": errors.len(),
            "errors": errors,
        }))
    }

    /// Tokenize SQL into tokens
    #[utoipa::path(
        post,
        path = "/api/tokenize",
        tag = "ogsql",
        request_body = SqlInput,
        responses((status = 200, description = "Token list"))
    )]
    pub async fn handle_tokenize(Json(input): Json<SqlInput>) -> Json<serde_json::Value> {
        let tokens = match ogsql_parser::Tokenizer::new(&input.sql).tokenize() {
            Ok(t) => t,
            Err(e) => return Json(serde_json::json!({"error": e.to_string()})),
        };
        let list: Vec<serde_json::Value> = tokens
            .iter()
            .map(|t| {
                let (token_type, value) = super::token_display(t);
                serde_json::json!({
                    "type": token_type,
                    "value": value,
                    "line": t.location.line,
                    "column": t.location.column,
                })
            })
            .collect();
        Json(serde_json::json!({"tokens": list}))
    }

    /// Validate SQL syntax
    #[utoipa::path(
        post,
        path = "/api/validate",
        tag = "ogsql",
        request_body = SqlInput,
        responses((status = 200, description = "Validation result"))
    )]
    pub async fn handle_validate(Json(input): Json<SqlInput>) -> Json<serde_json::Value> {
        let (_, errors) = super::parse_input(&input.sql);
        Json(serde_json::json!({
            "valid": errors.is_empty(),
            "error_count": errors.len(),
            "errors": errors,
        }))
    }

    /// Convert JSON (from /api/parse) back to SQL
    #[utoipa::path(
        post,
        path = "/api/json2sql",
        tag = "ogsql",
        request_body = JsonInput,
        responses((status = 200, description = "Reconstructed SQL"))
    )]
    pub async fn handle_json2sql(Json(input): Json<JsonInput>) -> Json<serde_json::Value> {
        let json_value: serde_json::Value = match serde_json::from_str(&input.json) {
            Ok(v) => v,
            Err(e) => return Json(serde_json::json!({"error": format!("Invalid JSON: {}", e)})),
        };

        let statements: Vec<ogsql_parser::Statement> = if let Some(arr) =
            json_value.get("statements")
        {
            match serde_json::from_value(arr.clone()) {
                Ok(s) => s,
                Err(e) => {
                    return Json(
                        serde_json::json!({"error": format!("Failed to deserialize statements: {}", e)}),
                    )
                }
            }
        } else {
            match serde_json::from_value(json_value) {
                Ok(s) => s,
                Err(e) => {
                    return Json(
                        serde_json::json!({"error": format!("Failed to deserialize: {}", e)}),
                    )
                }
            }
        };

        let formatter = ogsql_parser::SqlFormatter::new();
        let formatted: Vec<String> = statements
            .iter()
            .map(|s| formatter.format_statement(s))
            .collect();

        Json(serde_json::json!({
            "statements": formatted,
            "count": formatted.len(),
        }))
    }

    async fn openapi_spec() -> Json<utoipa::openapi::OpenApi> {
        Json(ApiDoc::openapi())
    }

    pub fn router() -> Router {
        Router::new()
            .route("/api/health", get(health))
            .route("/api/parse", post(handle_parse))
            .route("/api/json2sql", post(handle_json2sql))
            .route("/api/format", post(handle_format))
            .route("/api/tokenize", post(handle_tokenize))
            .route("/api/validate", post(handle_validate))
            .route("/api-docs/openapi.json", get(openapi_spec))
    }
}

#[cfg(feature = "tui")]
fn cmd_playground() {
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
    use crossterm::execute;
    use crossterm::terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    };
    use ratatui::backend::CrosstermBackend;
    use ratatui::layout::{Constraint, Direction, Layout};
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::text::{Line, Span};
    use ratatui::widgets::{Block, Borders, Paragraph, Tabs, Wrap};
    use ratatui::Frame;
    use std::io;

    struct App {
        input: String,
        cursor: usize,
        tab_index: usize,
        input_scroll: u16,
        output_scroll: u16,
    }

    impl App {
        fn new() -> Self {
            Self {
                input: String::from(
                    "SELECT id, name\nFROM users\nWHERE status = 'active'\nORDER BY id LIMIT 10;",
                ),
                cursor: 42,
                tab_index: 0,
                input_scroll: 0,
                output_scroll: 0,
            }
        }
    }

    fn compute_output(app: &App) -> String {
        let sql = app.input.trim();
        if sql.is_empty() {
            return String::new();
        }

        let tokens = match Tokenizer::new(sql).tokenize() {
            Ok(t) => t,
            Err(e) => return format!("Tokenizer error: {}", e),
        };

        match app.tab_index {
            1 => tokens
                .iter()
                .map(|t| {
                    let (tt, val) = token_display(t);
                    format!(
                        "{:<16} L{:>3}:C{:>3}  {}",
                        tt, t.location.line, t.location.column, val
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
            2 => {
                let mut parser = Parser::new(tokens);
                let mut stmts = Vec::new();
                while let Some(r) = parser.parse_next() {
                    if let Ok(s) = r {
                        stmts.push(s);
                    }
                }
                let fmt = SqlFormatter::new();
                stmts
                    .iter()
                    .map(|s| fmt.format_statement(s))
                    .collect::<Vec<_>>()
                    .join(";\n")
            }
            _ => {
                let mut parser = Parser::new(tokens);
                let mut stmts = Vec::new();
                while let Some(r) = parser.parse_next() {
                    if let Ok(s) = r {
                        stmts.push(s);
                    }
                }
                let errors = parser.errors();
                let mut out = format!("{:#?}", stmts);
                if !errors.is_empty() {
                    out.push_str("\n\nErrors:\n");
                    for e in errors {
                        out.push_str(&format!("  {}\n", e));
                    }
                }
                out
            }
        }
    }

    fn draw(f: &mut Frame, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(f.area());

        let input_title = Line::from(vec![
            Span::styled(
                " SQL Input ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" (Esc=quit, Tab=switch view, Shift+Up/Down=scroll output)"),
        ]);
        let input_block = Block::default()
            .title(input_title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
        let input = Paragraph::new(app.input.as_str())
            .block(input_block)
            .scroll((app.input_scroll, 0));
        f.render_widget(input, chunks[0]);

        let tabs_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        let tabs = Tabs::new(vec!["AST", "Tokens", "Formatted"])
            .block(tabs_block.clone())
            .select(app.tab_index)
            .style(Style::default().fg(Color::DarkGray))
            .highlight_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            );
        f.render_widget(&tabs, chunks[1]);

        let output_area = tabs_block.inner(chunks[1]);
        let output_text = compute_output(app);
        let output = Paragraph::new(output_text.as_str())
            .wrap(Wrap { trim: false })
            .scroll((app.output_scroll, 0));
        f.render_widget(output, output_area);

        let line_before_cursor = app.input[..app.cursor].matches('\n').count() as u16;
        let last_col = app.input[..app.cursor]
            .split('\n')
            .last()
            .map(|l| l.len())
            .unwrap_or(0) as u16;
        f.set_cursor_position((
            chunks[0].x + last_col + 1,
            chunks[0].y + line_before_cursor + 1 - app.input_scroll,
        ));
    }

    enable_raw_mode().expect("Failed to enable raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).expect("Failed to enter alt screen");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend).expect("Failed to create terminal");

    let mut app = App::new();

    loop {
        terminal.draw(|f| draw(f, &app)).expect("Failed to draw");

        match event::read().expect("Failed to read event") {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match code {
                KeyCode::Esc => break,
                KeyCode::Tab => app.tab_index = (app.tab_index + 1) % 3,
                KeyCode::Backspace => {
                    if app.cursor > 0 {
                        app.input.remove(app.cursor - 1);
                        app.cursor -= 1;
                    }
                }
                KeyCode::Delete => {
                    if app.cursor < app.input.len() {
                        app.input.remove(app.cursor);
                    }
                }
                KeyCode::Left => {
                    if app.cursor > 0 {
                        app.cursor -= 1;
                    }
                }
                KeyCode::Right => {
                    if app.cursor < app.input.len() {
                        app.cursor += 1;
                    }
                }
                KeyCode::Up => {
                    if modifiers.contains(KeyModifiers::SHIFT) {
                        app.output_scroll = app.output_scroll.saturating_sub(1);
                    }
                }
                KeyCode::Down => {
                    if modifiers.contains(KeyModifiers::SHIFT) {
                        app.output_scroll = app.output_scroll.saturating_add(1);
                    }
                }
                KeyCode::Enter => {
                    app.input.insert(app.cursor, '\n');
                    app.cursor += 1;
                }
                KeyCode::Char(c) => {
                    app.input.insert(app.cursor, c);
                    app.cursor += 1;
                }
                _ => {}
            },
            Event::Resize(_, _) => {}
            _ => {}
        }
    }

    disable_raw_mode().expect("Failed to disable raw mode");
    execute!(terminal.backend_mut(), LeaveAlternateScreen).expect("Failed to leave alt screen");
    terminal.show_cursor().unwrap();
}

#[cfg(feature = "ibatis")]
fn cmd_parse_xml(cli: &Cli, dir: Option<&str>, csv: bool) {
    if dir.is_some() && cli.file.is_some() {
        die!("Error: --dir and -f are mutually exclusive");
    }

    if let Some(dir_path) = dir {
        cmd_parse_xml_dir(cli, dir_path, csv);
    } else {
        cmd_parse_xml_single(cli, csv);
    }
}

#[cfg(feature = "ibatis")]
fn cmd_parse_xml_single(cli: &Cli, csv: bool) {
    let input = match cli.file.as_deref() {
        Some(path) => std::fs::read(path).unwrap_or_else(|e| die!("Error reading {}: {}", path, e)),
        None => {
            let mut buf = Vec::new();
            std::io::stdin()
                .read_to_end(&mut buf)
                .unwrap_or_else(|e| die!("Error reading stdin: {}", e));
            buf
        }
    };

    let result = ogsql_parser::ibatis::parse_mapper_bytes_with_path(&input, cli.file.as_deref());

    if csv {
        output_csv_xml_header();
        output_csv_xml_rows(
            &result.statements,
            cli.file.as_deref().unwrap_or("<stdin>"),
            ".",
        );
    } else if cli.json {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        print_xml_text(&result);
    }
}

#[cfg(feature = "ibatis")]
fn cmd_parse_xml_dir(cli: &Cli, dir_path: &str, csv: bool) {
    use std::path::Path;

    let root = Path::new(dir_path);
    if !root.is_dir() {
        die!("Error: '{}' is not a directory", dir_path);
    }

    let mut all_results: Vec<(String, String, ogsql_parser::ibatis::ParsedMapper)> = Vec::new();

    for entry in walkdir::WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !ext.eq_ignore_ascii_case("xml") {
            continue;
        }

        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Warning: skipping {}: {}", path.display(), e);
                continue;
            }
        };

        let rel_dir = path
            .parent()
            .and_then(|p| p.strip_prefix(root).ok())
            .map(|p| p.to_str().unwrap_or("."))
            .unwrap_or(".");

        let result = ogsql_parser::ibatis::parse_mapper_bytes_with_path(
            &bytes,
            Some(&path.to_string_lossy()),
        );

        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        all_results.push((file_name, rel_dir.to_string(), result));
    }

    if csv {
        output_csv_xml_header();
        for (file_name, rel_dir, result) in &all_results {
            output_csv_xml_rows(&result.statements, file_name, rel_dir);
        }
    } else if cli.json {
        let combined: Vec<serde_json::Value> = all_results
            .iter()
            .map(|(f, d, r)| {
                serde_json::json!({
                    "file": f,
                    "directory": d,
                    "namespace": r.namespace,
                    "statements": r.statements,
                    "errors": r.errors,
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&combined).unwrap());
    } else {
        let mut total = 0usize;
        for (file_name, _rel_dir, result) in &all_results {
            if !result.errors.is_empty() {
                eprintln!("[{}] {} error(s):", file_name, result.errors.len());
                for e in &result.errors {
                    eprintln!("  {}", e);
                }
            }

            for stmt in &result.statements {
                println!(
                    "── {} ({:?}) [{} L{}] ──",
                    stmt.id, stmt.kind, file_name, stmt.line
                );
                println!("{}", stmt.flat_sql.trim());
                if stmt.has_dynamic_elements {
                    println!("  [contains dynamic SQL elements]");
                }
                if let Some((infos, errors)) = &stmt.parse_result {
                    let warnings: Vec<_> = errors.iter().filter(|e| is_warning(e)).collect();
                    let real_errors: Vec<_> = errors.iter().filter(|e| !is_warning(e)).collect();
                    if !real_errors.is_empty() {
                        eprintln!("  {} parse error(s):", real_errors.len());
                        for e in &real_errors {
                            eprintln!("    {}", e);
                        }
                    }
                    if !warnings.is_empty() {
                        eprintln!("  {} warning(s):", warnings.len());
                        for w in &warnings {
                            eprintln!("    {}", w);
                        }
                    }
                    if real_errors.is_empty() {
                        println!(
                            "  ✓ Parsed successfully ({} statement(s)){}",
                            infos.len(),
                            if warnings.is_empty() {
                                ""
                            } else {
                                " (with warnings)"
                            }
                        );
                    }
                }
                println!();
            }
            total += result.statements.len();
        }
        println!(
            "Total: {} statement(s) from {} file(s)",
            total,
            all_results.len()
        );
    }
}

#[cfg(feature = "ibatis")]
fn print_xml_text(result: &ogsql_parser::ibatis::ParsedMapper) {
    if !result.errors.is_empty() {
        eprintln!("{} error(s):", result.errors.len());
        for e in &result.errors {
            eprintln!("  {}", e);
        }
    }

    for stmt in &result.statements {
        println!("── {} ({:?}) ──", stmt.id, stmt.kind);
        println!("{}", stmt.flat_sql.trim());
        if stmt.has_dynamic_elements {
            println!("  [contains dynamic SQL elements]");
        }
        if let Some((infos, errors)) = &stmt.parse_result {
            let warnings: Vec<_> = errors.iter().filter(|e| is_warning(e)).collect();
            let real_errors: Vec<_> = errors.iter().filter(|e| !is_warning(e)).collect();
            if !real_errors.is_empty() {
                eprintln!("  {} parse error(s):", real_errors.len());
                for e in &real_errors {
                    eprintln!("    {}", e);
                }
            }
            if !warnings.is_empty() {
                eprintln!("  {} warning(s):", warnings.len());
                for w in &warnings {
                    eprintln!("    {}", w);
                }
            }
            if real_errors.is_empty() {
                println!(
                    "  ✓ Parsed successfully ({} statement(s)){}",
                    infos.len(),
                    if warnings.is_empty() {
                        ""
                    } else {
                        " (with warnings)"
                    }
                );
            }
        }
        println!();
    }

    println!(
        "Total: {} statement(s) in namespace '{}'",
        result.statements.len(),
        result.namespace
    );
}

#[cfg(feature = "java")]
fn cmd_parse_java(cli: &Cli, extra_sql_methods: &[String], dir: Option<&str>, csv: bool) {
    if dir.is_some() && cli.file.is_some() {
        die!("Error: --dir and -f are mutually exclusive");
    }

    if let Some(dir_path) = dir {
        cmd_parse_java_dir(cli, extra_sql_methods, dir_path, csv);
    } else {
        cmd_parse_java_single(cli, extra_sql_methods, csv);
    }
}

#[cfg(feature = "java")]
fn cmd_parse_java_single(cli: &Cli, extra_sql_methods: &[String], csv: bool) {
    let (source, file_path) = match cli.file.as_deref() {
        Some(path) => {
            let bytes =
                std::fs::read(path).unwrap_or_else(|e| die!("Error reading {}: {}", path, e));
            let (text, _encoding) = ogsql_parser::token::decode_sql_file(&bytes)
                .unwrap_or_else(|e| die!("Error decoding {}: {}", path, e));
            (text, path.to_string())
        }
        None => {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .unwrap_or_else(|e| die!("Error reading stdin: {}", e));
            (buf, "<stdin>".to_string())
        }
    };

    let config = ogsql_parser::java::JavaExtractConfig {
        extra_sql_methods: extra_sql_methods.to_vec(),
    };
    let result = ogsql_parser::java::extract_sql_from_java(&source, &file_path, &config);

    if csv {
        output_csv_java_header();
        output_csv_java_rows(&result.extractions, &file_path, ".");
    } else if cli.json {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        print_java_text(&result, &file_path);
    }
}

#[cfg(feature = "java")]
fn cmd_parse_java_dir(cli: &Cli, extra_sql_methods: &[String], dir_path: &str, csv: bool) {
    use std::path::Path;

    let root = Path::new(dir_path);
    if !root.is_dir() {
        die!("Error: '{}' is not a directory", dir_path);
    }

    let config = ogsql_parser::java::JavaExtractConfig {
        extra_sql_methods: extra_sql_methods.to_vec(),
    };

    let mut all_results: Vec<(String, String, ogsql_parser::java::JavaExtractResult)> = Vec::new();

    for entry in walkdir::WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !ext.eq_ignore_ascii_case("java") {
            continue;
        }

        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Warning: skipping {}: {}", path.display(), e);
                continue;
            }
        };

        let (source, _encoding) = match ogsql_parser::token::decode_sql_file(&bytes) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Warning: skipping {}: {}", path.display(), e);
                continue;
            }
        };

        let file_path_str = path.to_string_lossy().to_string();
        let result = ogsql_parser::java::extract_sql_from_java(&source, &file_path_str, &config);

        let rel_dir = path
            .parent()
            .and_then(|p| p.strip_prefix(root).ok())
            .map(|p| p.to_str().unwrap_or("."))
            .unwrap_or(".");

        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        all_results.push((file_name, rel_dir.to_string(), result));
    }

    if csv {
        output_csv_java_header();
        for (file_name, rel_dir, result) in &all_results {
            output_csv_java_rows(&result.extractions, file_name, rel_dir);
        }
    } else if cli.json {
        let combined: Vec<serde_json::Value> = all_results
            .iter()
            .map(|(f, d, r)| {
                serde_json::json!({
                    "file": f,
                    "directory": d,
                    "extractions": r.extractions,
                    "errors": r.errors,
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&combined).unwrap());
    } else {
        let mut total = 0usize;
        for (file_name, _rel_dir, result) in &all_results {
            if !result.errors.is_empty() {
                eprintln!("[{}] {} error(s):", file_name, result.errors.len());
                for e in &result.errors {
                    eprintln!("  {}", e);
                }
            }

            for ext in &result.extractions {
                let location = match &ext.origin.class_name {
                    Some(cls) => format!(
                        "{}::{}",
                        cls,
                        ext.origin.method_name.as_deref().unwrap_or("")
                    ),
                    None => file_name.clone(),
                };
                println!(
                    "── {:?} [{:?}] @ {} L{} [{}] ──",
                    ext.origin.method, ext.sql_kind, location, ext.origin.line, file_name
                );
                println!("{}", ext.sql.trim());
                if ext.is_concatenated {
                    println!("  [concatenated]");
                }
                if ext.is_text_block {
                    println!("  [text block]");
                }
                if ext.parameter_style != ogsql_parser::java::ParameterStyle::None {
                    println!("  [params: {:?}]", ext.parameter_style);
                }
                if let Some(parse_result) = &ext.parse_result {
                    let warnings: Vec<_> = parse_result
                        .errors
                        .iter()
                        .filter(|e| is_warning(e))
                        .collect();
                    let real_errors: Vec<_> = parse_result
                        .errors
                        .iter()
                        .filter(|e| !is_warning(e))
                        .collect();
                    if !real_errors.is_empty() {
                        eprintln!("  {} parse error(s):", real_errors.len());
                        for e in &real_errors {
                            eprintln!("    {}", e);
                        }
                    }
                    if !warnings.is_empty() {
                        eprintln!("  {} warning(s):", warnings.len());
                        for w in &warnings {
                            eprintln!("    {}", w);
                        }
                    }
                    if real_errors.is_empty() {
                        println!(
                            "  ✓ Parsed successfully ({} statement(s)){}",
                            parse_result.statements.len(),
                            if warnings.is_empty() {
                                ""
                            } else {
                                " (with warnings)"
                            }
                        );
                    }
                }
                println!();
            }
            total += result.extractions.len();
        }
        println!(
            "Total: {} extraction(s) from {} file(s)",
            total,
            all_results.len()
        );
    }
}

#[cfg(feature = "java")]
fn print_java_text(result: &ogsql_parser::java::JavaExtractResult, file_path: &str) {
    if !result.errors.is_empty() {
        eprintln!("{} error(s):", result.errors.len());
        for e in &result.errors {
            eprintln!("  {}", e);
        }
    }

    if result.extractions.is_empty() {
        println!("No SQL statements found in {}", file_path);
    }

    for ext in &result.extractions {
        let location = match &ext.origin.class_name {
            Some(cls) => format!(
                "{}::{}",
                cls,
                ext.origin.method_name.as_deref().unwrap_or("")
            ),
            None => file_path.to_string(),
        };
        println!(
            "── {:?} [{:?}] @ {} L{} ──",
            ext.origin.method, ext.sql_kind, location, ext.origin.line
        );
        println!("{}", ext.sql.trim());
        if ext.is_concatenated {
            println!("  [concatenated]");
        }
        if ext.is_text_block {
            println!("  [text block]");
        }
        if ext.parameter_style != ogsql_parser::java::ParameterStyle::None {
            println!("  [params: {:?}]", ext.parameter_style);
        }
        if let Some(parse_result) = &ext.parse_result {
            let warnings: Vec<_> = parse_result
                .errors
                .iter()
                .filter(|e| is_warning(e))
                .collect();
            let real_errors: Vec<_> = parse_result
                .errors
                .iter()
                .filter(|e| !is_warning(e))
                .collect();
            if !real_errors.is_empty() {
                eprintln!("  {} parse error(s):", real_errors.len());
                for e in &real_errors {
                    eprintln!("    {}", e);
                }
            }
            if !warnings.is_empty() {
                eprintln!("  {} warning(s):", warnings.len());
                for w in &warnings {
                    eprintln!("    {}", w);
                }
            }
            if real_errors.is_empty() {
                println!(
                    "  ✓ Parsed successfully ({} statement(s)){}",
                    parse_result.statements.len(),
                    if warnings.is_empty() {
                        ""
                    } else {
                        " (with warnings)"
                    }
                );
            }
        }
        println!();
    }

    println!(
        "Total: {} extraction(s) from {}",
        result.extractions.len(),
        file_path
    );
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn extract_variables(sql: &str) -> String {
    let prefixes = ["__XML_PARAM_", "__XML_RAW_", "__JAVA_VAR_"];
    let mut vars = Vec::new();
    let mut i = 0;
    let bytes = sql.as_bytes();
    let len = bytes.len();

    while i < len {
        if bytes[i] != b'_' {
            i += 1;
            continue;
        }
        let mut found = false;
        for prefix in &prefixes {
            let prefix_bytes = prefix.as_bytes();
            if i + prefix_bytes.len() + 2 <= len
                && &bytes[i..i + prefix_bytes.len()] == prefix_bytes
            {
                let content_start = i + prefix_bytes.len();
                let mut end = content_start;
                while end + 1 < len && !(bytes[end] == b'_' && bytes[end + 1] == b'_') {
                    end += 1;
                }
                if end + 1 < len {
                    vars.push(sql[i..end + 2].to_string());
                    i = end + 2;
                    found = true;
                    break;
                }
            }
        }
        if !found {
            i += 1;
        }
    }

    vars.join(";")
}

#[cfg(feature = "ibatis")]
fn output_csv_xml_header() {
    println!("file,directory,line,method,sql,variables,error,warning");
}

#[cfg(feature = "ibatis")]
fn output_csv_xml_rows(
    statements: &[ogsql_parser::ibatis::ParsedStatement],
    file_name: &str,
    rel_dir: &str,
) {
    for stmt in statements {
        let (errors, warnings) = match &stmt.parse_result {
            Some((_, parse_errors)) => {
                let errs: Vec<String> = parse_errors
                    .iter()
                    .filter(|e| !is_warning(e))
                    .map(|e| e.to_string())
                    .collect();
                let warns: Vec<String> = parse_errors
                    .iter()
                    .filter(|e| is_warning(e))
                    .map(|e| e.to_string())
                    .collect();
                (errs.join("; "), warns.join("; "))
            }
            None => (String::new(), String::new()),
        };

        let sql = stmt.flat_sql.trim().replace('\n', "\\n").replace('\r', "");
        let variables = extract_variables(&stmt.flat_sql);
        println!(
            "{},{},{},{},{},{},{},{}",
            csv_escape(file_name),
            csv_escape(rel_dir),
            stmt.line,
            csv_escape(&stmt.id),
            csv_escape(&sql),
            csv_escape(&variables),
            csv_escape(&errors),
            csv_escape(&warnings),
        );
    }
}

#[cfg(feature = "java")]
fn output_csv_java_header() {
    println!("file,directory,line,method,sql,variables,error,warning");
}

#[cfg(feature = "java")]
fn output_csv_java_rows(
    extractions: &[ogsql_parser::java::ExtractedSql],
    file_name: &str,
    rel_dir: &str,
) {
    for ext in extractions {
        let method = match (&ext.origin.class_name, &ext.origin.method_name) {
            (Some(cls), Some(m)) => format!("{}::{}", cls, m),
            (None, Some(m)) => m.clone(),
            (Some(cls), None) => cls.clone(),
            (None, None) => ext.origin.variable_name.clone().unwrap_or_default(),
        };

        let (errors, warnings) = match &ext.parse_result {
            Some(parse_result) => {
                let errs: Vec<String> = parse_result
                    .errors
                    .iter()
                    .filter(|e| !is_warning(e))
                    .map(|e| e.to_string())
                    .collect();
                let warns: Vec<String> = parse_result
                    .errors
                    .iter()
                    .filter(|e| is_warning(e))
                    .map(|e| e.to_string())
                    .collect();
                (errs.join("; "), warns.join("; "))
            }
            None => (String::new(), String::new()),
        };

        let sql = ext.sql.trim().replace('\n', "\\n").replace('\r', "");
        let variables = extract_variables(&ext.sql);
        println!(
            "{},{},{},{},{},{},{},{}",
            csv_escape(file_name),
            csv_escape(rel_dir),
            ext.origin.line,
            csv_escape(&method),
            csv_escape(&sql),
            csv_escape(&variables),
            csv_escape(&errors),
            csv_escape(&warnings),
        );
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Format => cmd_format(&cli),
        Commands::Parse => cmd_parse(&cli),
        Commands::JsonToSql => cmd_json2sql(&cli),
        Commands::Tokenize => cmd_tokenize(&cli),
        Commands::Validate => cmd_validate(&cli),
        #[cfg(feature = "serve")]
        Commands::Serve { port, host } => {
            let addr = format!("{}:{}", host, port);
            eprintln!("ogsql server listening on http://{}", addr);
            eprintln!("  OpenAPI spec: http://{}/api-docs/openapi.json", addr);

            let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
            rt.block_on(async {
                let listener = tokio::net::TcpListener::bind(&addr)
                    .await
                    .unwrap_or_else(|e| die!("Failed to bind {}: {}", addr, e));
                axum::serve(listener, api::router())
                    .await
                    .unwrap_or_else(|e| die!("Server error: {}", e));
            });
        }
        #[cfg(feature = "tui")]
        Commands::Playground => cmd_playground(),
        #[cfg(feature = "ibatis")]
        Commands::ParseXml { ref dir, csv } => cmd_parse_xml(&cli, dir.as_deref(), csv),
        #[cfg(feature = "java")]
        Commands::ParseJava {
            ref extra_sql_methods,
            ref dir,
            csv,
        } => cmd_parse_java(&cli, extra_sql_methods, dir.as_deref(), csv),
    }
}
