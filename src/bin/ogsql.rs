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
    ParseXml,
    #[cfg(feature = "java")]
    /// Extract and parse SQL from Java source files / 从 Java 源文件中提取并解析 SQL
    #[command(name = "parse-java")]
    ParseJava,
}

macro_rules! die {
    ($($t:tt)*) => {{ eprintln!($($t)*); std::process::exit(1); }};
}

fn read_input(file: Option<&str>) -> String {
    match file {
        Some(path) => {
            let bytes = std::fs::read(path)
                .unwrap_or_else(|e| die!("Error reading {}: {}", path, e));
            token::decode_sql_file(&bytes)
                .unwrap_or_else(|e| die!("Error decoding {}: {}", path, e))
                .0
        }
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)
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
    let (stmts, errors) = parse_input(&sql);

    let formatter = SqlFormatter::new();
    let formatted: Vec<String> = stmts.iter().map(|si| formatter.format_statement(&si.statement)).collect();

    if cli.json {
        let out = serde_json::json!({
            "statements": formatted,
            "error_count": errors.len(),
            "errors": errors,
        });
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        if !errors.is_empty() {
            for e in &errors {
                eprintln!("error: {}", e);
            }
        }
        if !formatted.is_empty() {
            println!("{}", formatted.join(";\n"));
            println!(";");
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
                        obj.as_object_mut()
                            .unwrap()
                            .insert("dynamic_sql_analysis".to_string(), serde_json::json!(report));
                    }
                }
                obj
            })
            .collect();

        let out = serde_json::json!({
            "statements": stmt_values,
            "errors": errors,
        });
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        for stmt in &stmts {
            println!("{:#?}", stmt);
        }
        if !errors.is_empty() {
            eprintln!("\n{} error(s):", errors.len());
            for e in &errors {
                eprintln!("  {}", e);
            }
        }
    }
}

fn extract_pl_block(stmt: &ogsql_parser::Statement) -> Option<&ogsql_parser::ast::plpgsql::PlBlock> {
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
        items.iter().filter_map(|v| {
            if v.get("sql_text").is_some() {
                serde_json::from_value::<StatementInfo>(v.clone()).ok().map(|si| si.statement)
            } else {
                serde_json::from_value::<Statement>(v.clone()).ok()
            }
        }).collect()
    } else {
        die!("\"statements\" must be an array");
    };

    if statements.is_empty() {
        die!("No valid statements found in JSON");
    }

    let formatter = SqlFormatter::new();
    let formatted: Vec<String> = statements.iter().map(|s| formatter.format_statement(s)).collect();

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

fn cmd_validate(cli: &Cli) {
    let sql = read_input(cli.file.as_deref());
    let (_, errors) = parse_input(&sql);

    if cli.json {
        let out = serde_json::json!({
            "valid": errors.is_empty(),
            "error_count": errors.len(),
            "errors": errors,
        });
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        if errors.is_empty() {
            println!("VALID");
        } else {
            println!("INVALID ({} error(s)):", errors.len());
            for e in &errors {
                eprintln!("  {}", e);
            }
            std::process::exit(1);
        }
    }
}

#[cfg(feature = "serve")]
mod api {
    use axum::Json;
    use axum::routing::{get, post};
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
        Json(serde_json::json!({"statements": stmts, "errors": errors}))
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
        let formatted: Vec<String> = stmts.iter().map(|si| formatter.format_statement(&si.statement)).collect();
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

        let statements: Vec<ogsql_parser::Statement> = if let Some(arr) = json_value.get("statements") {
            match serde_json::from_value(arr.clone()) {
                Ok(s) => s,
                Err(e) => return Json(serde_json::json!({"error": format!("Failed to deserialize statements: {}", e)})),
            }
        } else {
            match serde_json::from_value(json_value) {
                Ok(s) => s,
                Err(e) => return Json(serde_json::json!({"error": format!("Failed to deserialize: {}", e)})),
            }
        };

        let formatter = ogsql_parser::SqlFormatter::new();
        let formatted: Vec<String> = statements.iter().map(|s| formatter.format_statement(s)).collect();

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
    use crossterm::terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    };
    use crossterm::execute;
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
                    format!("{:<16} L{:>3}:C{:>3}  {}", tt, t.location.line, t.location.column, val)
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
                stmts.iter().map(|s| fmt.format_statement(s)).collect::<Vec<_>>().join(";\n")
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
            Span::styled(" SQL Input ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
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

        let tabs_block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan));
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
            Event::Key(KeyEvent { code, modifiers, .. }) => match code {
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
fn cmd_parse_xml(cli: &Cli) {
    let input = match cli.file.as_deref() {
        Some(path) => {
            std::fs::read(path).unwrap_or_else(|e| die!("Error reading {}: {}", path, e))
        }
        None => {
            let mut buf = Vec::new();
            std::io::stdin().read_to_end(&mut buf)
                .unwrap_or_else(|e| die!("Error reading stdin: {}", e));
            buf
        }
    };

    let result = ogsql_parser::ibatis::parse_mapper_bytes(&input);

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
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
                if !errors.is_empty() {
                    eprintln!("  {} parse error(s):", errors.len());
                    for e in errors {
                        eprintln!("    {}", e);
                    }
                } else {
                    println!("  ✓ Parsed successfully ({} statement(s))", infos.len());
                }
            }
            println!();
        }

        println!("Total: {} statement(s) in namespace '{}'", result.statements.len(), result.namespace);
    }
}

#[cfg(feature = "java")]
fn cmd_parse_java(cli: &Cli) {
    let (source, file_path) = match cli.file.as_deref() {
        Some(path) => {
            let bytes = std::fs::read(path).unwrap_or_else(|e| die!("Error reading {}: {}", path, e));
            let text = String::from_utf8(bytes)
                .unwrap_or_else(|e| die!("Error decoding {} as UTF-8: {}", path, e));
            (text, path.to_string())
        }
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)
                .unwrap_or_else(|e| die!("Error reading stdin: {}", e));
            (buf, "<stdin>".to_string())
        }
    };

    let result = ogsql_parser::java::extract_sql_from_java(&source, &file_path);

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
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
                Some(cls) => format!("{}::{}", cls, ext.origin.method_name.as_deref().unwrap_or("")),
                None => file_path.clone(),
            };
            println!("── {:?} [{:?}] @ {} L{} ──", ext.origin.method, ext.sql_kind, location, ext.origin.line);
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
                if !parse_result.errors.is_empty() {
                    eprintln!("  {} parse error(s):", parse_result.errors.len());
                    for e in &parse_result.errors {
                        eprintln!("    {}", e);
                    }
                } else {
                    println!("  ✓ Parsed successfully ({} statement(s))", parse_result.statements.len());
                }
            }
            println!();
        }

        println!("Total: {} extraction(s) from {}", result.extractions.len(), file_path);
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
        Commands::ParseXml => cmd_parse_xml(&cli),
        #[cfg(feature = "java")]
        Commands::ParseJava => cmd_parse_java(&cli),
    }
}
