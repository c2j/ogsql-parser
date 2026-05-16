use std::io::Read as _;

use clap::{Parser as ClapParser, Subcommand};
use ogsql_parser::*;
use ogsql_parser::token_formatter::{FormatConfig, KeywordCase, CommaStyle};
use serde::Serialize;

const OGSQL_LOGO: &str = r#"
  ██████╗  ██████╗ ███████╗ ██████╗ ██╗      ██████╗  █████╗ ██████╗ ███████╗███████╗██████╗
 ██╔═══██╗██╔════╝ ██╔════╝██╔═══██╗██║      ██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔════╝██╔══██╗
 ██║   ██║██║  ███╗███████╗██║   ██║██║      ██████╔╝███████║██████╔╝███████╗███████╗██████╔╝
 ██║   ██║██║   ██║╚════██║██║   ██║██║      ██╔═══╝ ██╔══██║██╔══██╗╚════██║██╔═══╝ ██╔══██╗
 ╚██████╔╝╚██████╔╝███████║╚██████╔╝███████╗ ██║     ██║  ██║██║  ██║███████║███████╗██║  ██║
  ╚═════╝  ╚═════╝ ╚══════╝ ╚═════╝ ╚══════╝ ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝"#;

#[derive(ClapParser)]
#[command(
    name = "ogsql",
    version,
    about = "openGauss/GaussDB SQL Parser",
    help_template = "\
{before-help}{name} {version}
{about-with-newline}\
{usage-heading} {usage}

{all-args}{after-help}\
",
    before_help = OGSQL_LOGO,
)]
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

    #[arg(long, global = true)]
    comments: bool,

    #[arg(long, global = true)]
    /// Enable MyBatis #{param, jdbcType=...} and ${expr} placeholder support.
    /// Preserves MyBatis placeholders during formatting and tokenization.
    /// 启用 MyBatis #{param} 和 ${expr} 占位符支持，格式化时保留占位符
    mybatis: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Format SQL statements with configurable indentation, keyword casing,
    /// comma style, and line width. Supports SELECT, INSERT, DELETE, UPDATE,
    /// MERGE, WITH (CTE), CREATE TABLE, and PL/pgSQL.
    /// 格式化 SQL 语句，支持缩进、关键字大小写、逗号风格、行宽等配置
    Format {
        /// Indentation width in spaces
        #[arg(short = 'i', long, default_value_t = 2)]
        indent: usize,
        /// Keyword casing: preserve, upper, lower
        #[arg(short = 'k', long, default_value = "preserve")]
        keyword_case: String,
        /// Comma style: trailing, leading
        #[arg(long, default_value = "trailing")]
        comma: String,
        /// Maximum line width (0 = unlimited)
        #[arg(short = 'w', long, default_value_t = 120)]
        line_width: usize,
        /// Shorthand for --keyword-case upper
        #[arg(short = 'u', long)]
        uppercase: bool,
        /// Don't put each SELECT column on its own line
        #[arg(long)]
        no_select_newline: bool,
        /// Don't put AND/OR on new lines
        #[arg(long)]
        no_logical_newline: bool,
        /// Don't put semicolons on their own line
        #[arg(long)]
        no_semicolon_newline: bool,
    },
    /// Parse SQL into AST and print the abstract syntax tree / 解析 SQL 为 AST
    Parse {
        /// Recursively scan directory for SQL files (can specify multiple times)
        /// 递归扫描目录中的 SQL 文件（可多次指定）
        #[arg(short = 'd', long = "dir")]
        dir: Vec<String>,
        /// File extensions to scan, comma-separated (default: sql) / 扫描的文件扩展名，逗号分隔
        #[arg(short = 'e', long = "ext", value_delimiter = ',', default_value = "sql")]
        ext: Vec<String>,
        /// Output in CSV format (flat, one row per statement; packages expanded) / 以 CSV 格式输出
        #[arg(long = "csv")]
        csv: bool,
        /// Target directory for JSON output files (required with --dir -j; preserves directory structure)
        /// JSON 文件输出目录（配合 --dir -j 使用，保留目录层次结构）
        #[arg(short = 'o', long = "output-dir")]
        output_dir: Option<String>,
        /// Print statistics after directory processing
        #[arg(long)]
        stats: bool,
    },
    /// Convert JSON (from `parse -j`) back to SQL / 将 JSON（parse -j 的输出）还原为 SQL
    #[command(name = "json2sql")]
    JsonToSql,
    /// Tokenize SQL into a list of tokens / 将 SQL 分词为 token 列表
    Tokenize,
    /// Validate SQL syntax and report errors / 校验 SQL 语法
    Validate {
        /// Recursively scan directory for SQL files (can specify multiple times)
        /// 递归扫描目录中的 SQL 文件（可多次指定）
        #[arg(short = 'd', long = "dir")]
        dir: Vec<String>,
        /// File extensions to scan, comma-separated (default: sql) / 扫描的文件扩展名，逗号分隔
        #[arg(short = 'e', long = "ext", value_delimiter = ',', default_value = "sql")]
        ext: Vec<String>,
        /// Print statistics after directory processing
        #[arg(long)]
        stats: bool,
    },
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
        #[cfg(feature = "java")]
        /// Java source root directory for parameter type inference / Java 源码根目录，用于参数类型推断
        #[arg(long = "java-src")]
        java_src: Option<String>,
        /// Print statistics after directory processing
        #[arg(long)]
        stats: bool,
    },
    #[cfg(feature = "java")]
    /// Extract and parse SQL from Java source files / 从 Java 源文件中提取并解析 SQL
    #[command(name = "parse-java")]
    ParseJava {
        #[arg(long = "extra-sql-methods", value_delimiter = ',')]
        extra_sql_methods: Vec<String>,
        #[arg(long = "extra-sql-var-patterns", value_delimiter = ',')]
        extra_sql_var_patterns: Vec<String>,
        /// Recursively scan directory for Java files / 递归扫描目录中的 Java 文件
        #[arg(short = 'd', long = "dir")]
        dir: Option<String>,
        /// Output in CSV format / 以 CSV 格式输出
        #[arg(long = "csv")]
        csv: bool,
        /// Print statistics after directory processing
        #[arg(long)]
        stats: bool,
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

fn parse_input(sql: &str, preserve_comments: bool, mybatis_params: bool) -> ogsql_parser::ParseOutput {
    let options = ogsql_parser::ParseOptions { preserve_comments, mybatis_params };
    ogsql_parser::Parser::parse_sql_with_options(sql, options)
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
        Token::Comment(s) => ("Comment".into(), s.clone()),
        Token::MyBatisParam(s) => ("MyBatisParam".into(), format!("#{{{}}}", s)),
        Token::MyBatisRawExpr(s) => ("MyBatisRawExpr".into(), format!("${{{}}}", s)),
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

fn cmd_format(
    cli: &Cli,
    indent: usize,
    keyword_case: String,
    comma: String,
    line_width: usize,
    uppercase: bool,
    no_select_newline: bool,
    no_logical_newline: bool,
    no_semicolon_newline: bool,
) {
    let sql = read_input(cli.file.as_deref());
    let mut tokenizer = Tokenizer::new(&sql).preserve_comments(true);
    if cli.mybatis {
        tokenizer = tokenizer.mybatis_params(true);
    }
    let tokens = match tokenizer.tokenize() {
        Ok(t) => t,
        Err(e) => die!("Tokenization error: {}", e),
    };
    let keyword_case = match keyword_case.as_str() {
        "upper" => KeywordCase::Upper,
        "lower" => KeywordCase::Lower,
        _ => KeywordCase::Preserve,
    };
    let comma_style = match comma.as_str() {
        "leading" => CommaStyle::Leading,
        _ => CommaStyle::Trailing,
    };
    let config = FormatConfig {
        indent_width: indent,
        keyword_case,
        comma_style,
        line_width,
        uppercase_keywords: uppercase,
        select_newline: !no_select_newline,
        logical_operator_newline: !no_logical_newline,
        semicolon_newline: !no_semicolon_newline,
    };
    let formatted = token_formatter::TokenFormatter::with_config(&sql, tokens, config).format();

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

fn cmd_parse(cli: &Cli, csv: bool) {
    let sql = read_input(cli.file.as_deref());
    let output = parse_input(&sql, cli.comments, cli.mybatis);

    if csv {
        let file_name = cli.file.as_deref().unwrap_or("<stdin>");
        output_csv_parse_header();
        output_csv_parse_rows(&output.statements, file_name, ".", &output.errors, cli.mybatis);
    } else if cli.json {
        let stmt_values: Vec<serde_json::Value> = output
            .statements
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
                        serde_json::to_string_pretty(&tx_report).unwrap().into(),
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

        let all_stmts: Vec<_> = output.statements.iter().map(|si| si.statement.clone()).collect();
        let fingerprints = ogsql_parser::compute_query_fingerprints(&all_stmts);

        let mut out = serde_json::json!({
            "statements": stmt_values,
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
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        for stmt in &output.statements {
            println!("{:#?}", stmt);
        }
        if !output.errors.is_empty() {
            let warnings: Vec<_> = output.errors.iter().filter(|e| is_warning(e)).collect();
            let real_errors: Vec<_> = output.errors.iter().filter(|e| !is_warning(e)).collect();
            if !real_errors.is_empty() {
                eprintln!("\n{} error(s):", real_errors.len());
                for e in &real_errors {
                    eprintln!("  {}", e);
                }
                if cli.verbose {
                    write_error_log(&sql, cli.file.as_deref(), &output.statements, &real_errors);
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

fn cmd_parse_dir(
    cli: &Cli,
    dir_paths: &[String],
    exts: &[String],
    csv: bool,
    output_dir: Option<&str>,
    stats: bool,
) {
    use std::path::Path;

    for dir_path in dir_paths {
        if !Path::new(dir_path).is_dir() {
            die!("Error: '{}' is not a directory", dir_path);
        }
    }

    if cli.json && output_dir.is_none() {
        die!("Error: --output-dir (-o) is required when using --dir with -j");
    }

    let normalized_exts: Vec<String> = exts
        .iter()
        .map(|e| e.trim_start_matches('.').to_ascii_lowercase())
        .collect();

    let mut files: Vec<(String, String, std::path::PathBuf)> = Vec::new();
    for dir_path in dir_paths {
        let root = Path::new(dir_path);
        for entry in walkdir::WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let file_ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();
            if !normalized_exts.iter().any(|e| *e == file_ext) {
                continue;
            }

            let rel_dir = path
                .parent()
                .and_then(|p| p.strip_prefix(root).ok())
                .map(|p| {
                    let s = p.to_str().unwrap_or(".");
                    if s.is_empty() { "." } else { s }
                })
                .unwrap_or(".")
                .to_string();

            let file_name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();

        files.push((file_name, rel_dir, path.to_path_buf()));
        }
    }

    files.sort_by(|a, b| a.2.cmp(&b.2));
    files.dedup_by(|a, b| a.2 == b.2);

    if files.is_empty() {
        eprintln!("No files found with extension(s): {}", exts.join(", "));
        return;
    }

    let mut files_with_errors: HashSet<String> = HashSet::new();
    let mut files_with_warnings: HashSet<String> = HashSet::new();
    let mut stmt_counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut error_kinds: BTreeMap<&'static str, (usize, HashSet<String>)> = BTreeMap::new();
    let mut warning_kinds: BTreeMap<&'static str, (usize, HashSet<String>)> = BTreeMap::new();

    if csv {
        output_csv_parse_header();
        for (file_name, rel_dir, abs_path) in &files {
            let sql = read_file_path(abs_path);
            let output = parse_input(&sql, cli.comments, cli.mybatis);
            output_csv_parse_rows(&output.statements, file_name, rel_dir, &output.errors, cli.mybatis);
            for si in &output.statements {
                *stmt_counts.entry(stmt_category(&si.statement)).or_insert(0) += 1;
            }
            for err in &output.errors {
                let kind = parser_error_kind(err);
                let file_set = if is_warning(err) { &mut warning_kinds } else { &mut error_kinds };
                let entry = file_set.entry(kind).or_insert((0, HashSet::new()));
                entry.0 += 1;
                entry.1.insert(file_name.clone());
                if is_warning(err) { files_with_warnings.insert(file_name.clone()); }
                else { files_with_errors.insert(file_name.clone()); }
            }
        }
    } else if cli.json {
        let out_root = Path::new(output_dir.unwrap());
        std::fs::create_dir_all(out_root)
            .unwrap_or_else(|e| die!("Error creating output directory '{}': {}", output_dir.unwrap(), e));

        let mut total_files = 0usize;
        let mut total_stmts = 0usize;
        let mut all_errors: Vec<(String, String, String, Vec<ogsql_parser::ParserError>)> = Vec::new();
        for (file_name, rel_dir, abs_path) in &files {
            let sql = read_file_path(abs_path);
            let output = parse_input(&sql, cli.comments, cli.mybatis);

            let target_dir = out_root.join(rel_dir);
            std::fs::create_dir_all(&target_dir)
                .unwrap_or_else(|e| die!("Error creating directory '{}': {}", target_dir.display(), e));

            let json_name = Path::new(file_name).with_extension("json");
            let target_path = target_dir.join(&json_name);
            let json_str = serde_json::to_string_pretty(&output).unwrap();
            std::fs::write(&target_path, json_str)
                .unwrap_or_else(|e| die!("Error writing '{}': {}", target_path.display(), e));

            total_stmts += output.statements.len();
            total_files += 1;

            for si in &output.statements {
                *stmt_counts.entry(stmt_category(&si.statement)).or_insert(0) += 1;
            }
            for err in &output.errors {
                let kind = parser_error_kind(err);
                let file_set = if is_warning(err) { &mut warning_kinds } else { &mut error_kinds };
                let entry = file_set.entry(kind).or_insert((0, HashSet::new()));
                entry.0 += 1;
                entry.1.insert(file_name.clone());
                if is_warning(err) { files_with_warnings.insert(file_name.clone()); }
                else { files_with_errors.insert(file_name.clone()); }
            }

            let real_errors: Vec<_> = output.errors.iter().filter(|e| !is_warning(e)).cloned().collect();
            if !real_errors.is_empty() {
                eprintln!(
                    "[{}/{}] {} error(s)",
                    rel_dir, file_name, real_errors.len()
                );
                all_errors.push((file_name.clone(), rel_dir.clone(), sql, real_errors));
            }
        }
        if !all_errors.is_empty() {
            write_dir_error_log(out_root, &all_errors);
        }
        eprintln!(
            "Wrote {} file(s), {} statement(s) to {}",
            total_files, total_stmts, out_root.display()
        );
        if stats {
            print_parse_stats(total_files, &files_with_errors, &files_with_warnings,
                total_stmts, &stmt_counts, &error_kinds, &warning_kinds, "parse -j");
        }
    } else {
        let mut total_files_txt = 0usize;
        let mut total_stmts_txt = 0usize;
        for (file_name, _rel_dir, abs_path) in &files {
            let sql = read_file_path(abs_path);
            let output = parse_input(&sql, cli.comments, cli.mybatis);

            total_files_txt += 1;
            total_stmts_txt += output.statements.len();

            for si in &output.statements {
                *stmt_counts.entry(stmt_category(&si.statement)).or_insert(0) += 1;
            }
            for err in &output.errors {
                let kind = parser_error_kind(err);
                let file_set = if is_warning(err) { &mut warning_kinds } else { &mut error_kinds };
                let entry = file_set.entry(kind).or_insert((0, HashSet::new()));
                entry.0 += 1;
                entry.1.insert(file_name.clone());
                if is_warning(err) { files_with_warnings.insert(file_name.clone()); }
                else { files_with_errors.insert(file_name.clone()); }
            }

            println!("═══ {} ({} statement(s)) ═══", file_name, output.statements.len());
            for stmt in &output.statements {
                println!("{:#?}", stmt);
            }
            if !output.errors.is_empty() {
                let warnings: Vec<_> = output.errors.iter().filter(|e| is_warning(e)).collect();
                let real_errors: Vec<_> = output.errors.iter().filter(|e| !is_warning(e)).collect();
                if !real_errors.is_empty() {
                    eprintln!("[{}] {} error(s):", file_name, real_errors.len());
                    for e in &real_errors {
                        eprintln!("  {}", e);
                    }
                }
                if !warnings.is_empty() {
                    eprintln!("[{}] {} warning(s):", file_name, warnings.len());
                    for w in &warnings {
                        eprintln!("  {}", w);
                    }
                }
            }
            println!();
        }
        println!("Total: {} statement(s) from {} file(s)", total_stmts_txt, files.len());
        if stats {
            print_parse_stats(total_files_txt, &files_with_errors, &files_with_warnings,
                total_stmts_txt, &stmt_counts, &error_kinds, &warning_kinds, "parse");
        }
    }
}

fn read_file_path(path: &std::path::Path) -> String {
    let bytes = std::fs::read(path)
        .unwrap_or_else(|e| die!("Error reading {}: {}", path.display(), e));
    token::decode_sql_file(&bytes)
        .unwrap_or_else(|e| die!("Error decoding {}: {}", path.display(), e))
        .0
}

fn output_csv_parse_header() {
    println!("file,directory,line,type,name,parent,parameters,return_type,sql,error,warning");
}

struct ParseCsvRow {
    line: usize,
    stmt_type: String,
    name: String,
    parent: String,
    parameters: String,
    return_type: String,
    sql: String,
}

fn format_params(params: &[ogsql_parser::ast::RoutineParam]) -> String {
    params
        .iter()
        .map(|p| match &p.mode {
            Some(mode) => format!("{} {} {}", p.name, mode, p.data_type),
            None => format!("{} {}", p.name, p.data_type),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

#[derive(Clone)]
enum ConcatPart {
    Literal(String),
    Variable(String),
    Unresolved(String),
}

/// Evaluate a PL/pgSQL string-concatenation expression (`||`) into concrete parts.
/// String literals are extracted as their raw content; variables/other expressions
/// are left as named placeholders. This turns `'SELECT * FROM ' || v_table || ' WHERE 1=1'`
/// into `["SELECT * FROM ", "${v_table}", " WHERE 1=1"]`.
fn eval_concat_expr(expr: &ogsql_parser::ast::Expr) -> Vec<ConcatPart> {
    use ogsql_parser::ast::{Expr, Literal};

    match expr {
        Expr::BinaryOp { op, left, right, .. } if op == "||" => {
            let mut parts = eval_concat_expr(left);
            parts.extend(eval_concat_expr(right));
            parts
        }
        Expr::Literal(Literal::String(s)) => vec![ConcatPart::Literal(s.clone())],
        Expr::Literal(Literal::DollarString { body, .. }) => vec![ConcatPart::Literal(body.clone())],
        Expr::Literal(Literal::EscapeString(s)) => vec![ConcatPart::Literal(s.clone())],
        Expr::PlVariable(names) if names.len() == 1 => {
            vec![ConcatPart::Variable(names[0].clone())]
        }
        Expr::ColumnRef(names) if names.len() == 1 => {
            let name = &names[0];
            if name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                vec![ConcatPart::Variable(name.clone())]
            } else {
                vec![ConcatPart::Literal(name.clone())]
            }
        }
        Expr::Parenthesized(inner) => eval_concat_expr(inner),
        _ => vec![ConcatPart::Unresolved(format_pl_expr(expr))],
    }
}

/// Join evaluated concat parts into a single SQL string, replacing variables via `replace_fn`.
fn join_concat_parts(
    parts: &[ConcatPart],
    replace_fn: &dyn Fn(&str, &std::collections::HashMap<String, Option<String>>) -> String,
    vars: &std::collections::HashMap<String, Option<String>>,
    assigns: &std::collections::HashMap<String, Vec<ConcatPart>>,
) -> String {
    join_concat_parts_inner(parts, replace_fn, vars, assigns, &mut std::collections::HashSet::new())
}

fn join_concat_parts_inner(
    parts: &[ConcatPart],
    replace_fn: &dyn Fn(&str, &std::collections::HashMap<String, Option<String>>) -> String,
    vars: &std::collections::HashMap<String, Option<String>>,
    assigns: &std::collections::HashMap<String, Vec<ConcatPart>>,
    visited: &mut std::collections::HashSet<String>,
) -> String {
    let vars_empty = vars.is_empty();
    let mut sql = String::new();
    for part in parts {
        match part {
            ConcatPart::Literal(s) => sql.push_str(s),
            ConcatPart::Variable(name) => {
                let key = name.to_ascii_lowercase();
                if !visited.contains(&key) {
                    if let Some(traced_parts) = assigns.get(&key) {
                        let has_inner_vars = traced_parts.iter().any(|p| !matches!(p, ConcatPart::Literal(_)));
                        if has_inner_vars {
                            visited.insert(key.clone());
                            sql.push_str(&join_concat_parts_inner(traced_parts, replace_fn, vars, assigns, visited));
                            visited.remove(&key);
                            continue;
                        } else {
                            let flat: String = traced_parts.iter().map(|p| match p {
                                ConcatPart::Literal(s) => s.as_str(), _ => ""
                            }).collect();
                            sql.push_str(&flat);
                            continue;
                        }
                    }
                }
                if vars_empty {
                    sql.push_str(name);
                } else {
                    let single_var: std::collections::HashMap<String, Option<String>> = {
                        let mut m = std::collections::HashMap::new();
                        m.insert(key.clone(), vars.get(&key).cloned().flatten());
                        m
                    };
                    let replaced = replace_fn(name, &single_var);
                    sql.push_str(&replaced);
                }
            }
            ConcatPart::Unresolved(s) => {
                if vars_empty {
                    sql.push_str(s);
                } else {
                    let replaced = replace_fn(s, vars);
                    sql.push_str(&replaced);
                }
            }
        }
    }
    sql
}

fn try_parse_sql_assignment(sql: &str) -> Option<(&str, &str)> {
    let s = sql.trim();
    let eq_pos = s.find(":=").or_else(|| {
        let bytes = s.as_bytes();
        for (i, &b) in bytes.iter().enumerate() {
            if b == b'=' && i > 0 {
                let prev = bytes[i - 1];
                if prev != b'!' && prev != b'<' && prev != b'>' && prev != b'=' && prev != b':' {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
                        continue;
                    }
                    return Some(i);
                }
            }
        }
        None
    })?;
    let (lhs, rhs) = if s.get(eq_pos..eq_pos+2) == Some(":=") {
        (&s[..eq_pos], &s[eq_pos+2..])
    } else {
        (&s[..eq_pos], &s[eq_pos+1..])
    };
    let var = lhs.trim();
    if var.is_empty() || !var.chars().next().map(|c| c.is_ascii_alphabetic() || c == '_').unwrap_or(false) {
        return None;
    }
    Some((var, rhs.trim()))
}

fn extract_execute_sql_content(exec: &ogsql_parser::ast::plpgsql::PlExecuteStmt) -> String {
    use ogsql_parser::ast::{Expr, Literal};

    if let Some(ref parsed) = exec.parsed_query {
        let formatter = ogsql_parser::SqlFormatter::new();
        return formatter.format_statement(parsed);
    }

    match &exec.string_expr {
        Expr::Literal(Literal::String(s)) => return s.clone(),
        Expr::Literal(Literal::DollarString { body, .. }) => return body.clone(),
        Expr::Literal(Literal::EscapeString(s)) => return s.clone(),
        _ => {}
    }

    let parts = eval_concat_expr(&exec.string_expr);
    let all_literal = parts.iter().all(|p| matches!(p, ConcatPart::Literal(_)));
    if all_literal {
        return parts.iter().map(|p| match p { ConcatPart::Literal(s) => s.as_str(), _ => "" }).collect();
    }
    format_pl_expr(&exec.string_expr)
}

/// Build the expanded SQL for an EXECUTE row in CSV output.
/// Handles string concatenation by flattening `'...' || var || '...'` into
/// a single SQL string with variables replaced as placeholders.
fn build_execute_csv_sql(
    exec: &ogsql_parser::ast::plpgsql::PlExecuteStmt,
    vars: &std::collections::HashMap<String, Option<String>>,
    raw: bool,
) -> String {
    use ogsql_parser::ast::{Expr, Literal};

    if let Some(ref parsed) = exec.parsed_query {
        let formatter = ogsql_parser::SqlFormatter::new();
        let formatted = formatter.format_statement(parsed);
        return if raw {
            replace_pl_vars_in_sql_raw(&formatted, vars)
        } else {
            replace_pl_vars_in_sql(&formatted, vars)
        };
    }

    match &exec.string_expr {
        Expr::Literal(Literal::String(s)) => {
            let sql = s.trim();
            return if raw {
                replace_pl_vars_in_sql_raw(sql, vars)
            } else {
                replace_pl_vars_in_sql(sql, vars)
            };
        }
        Expr::Literal(Literal::DollarString { body, .. }) => {
            let sql = body.trim();
            return if raw {
                replace_pl_vars_in_sql_raw(sql, vars)
            } else {
                replace_pl_vars_in_sql(sql, vars)
            };
        }
        Expr::Literal(Literal::EscapeString(s)) => {
            let sql = s.trim();
            return if raw {
                replace_pl_vars_in_sql_raw(sql, vars)
            } else {
                replace_pl_vars_in_sql(sql, vars)
            };
        }
        _ => {}
    }

    let empty_assigns: std::collections::HashMap<String, Vec<ConcatPart>> = std::collections::HashMap::new();
    let parts = eval_concat_expr(&exec.string_expr);
    let has_vars = parts.iter().any(|p| !matches!(p, ConcatPart::Literal(_)));
    if has_vars {
        let replace_fn: &dyn Fn(&str, &std::collections::HashMap<String, Option<String>>) -> String =
            if raw { &replace_pl_vars_in_sql_raw } else { &replace_pl_vars_in_sql };
        return join_concat_parts(&parts, replace_fn, vars, &empty_assigns);
    }

    let plain = extract_execute_sql_content(exec);
    if raw {
        replace_pl_vars_in_sql_raw(&plain, vars)
    } else {
        replace_pl_vars_in_sql(&plain, vars)
    }
}

fn build_execute_csv_sql_with_trace(
    exec: &ogsql_parser::ast::plpgsql::PlExecuteStmt,
    vars: &std::collections::HashMap<String, Option<String>>,
    assigns: &std::collections::HashMap<String, Vec<ConcatPart>>,
) -> String {
    use ogsql_parser::ast::{Expr, Literal};

    if let Some(ref parsed) = exec.parsed_query {
        let formatter = ogsql_parser::SqlFormatter::new();
        let formatted = formatter.format_statement(parsed);
        return replace_pl_vars_in_sql_raw(&formatted, vars);
    }

    match &exec.string_expr {
        Expr::Literal(Literal::String(s)) => {
            return replace_pl_vars_in_sql_raw(s.trim(), vars);
        }
        Expr::Literal(Literal::DollarString { body, .. }) => {
            return replace_pl_vars_in_sql_raw(body.trim(), vars);
        }
        Expr::Literal(Literal::EscapeString(s)) => {
            return replace_pl_vars_in_sql_raw(s.trim(), vars);
        }
        Expr::ColumnRef(names) | Expr::PlVariable(names) if names.len() == 1 => {
            let var_name = &names[0];
            if let Some(traced_parts) = assigns.get(&var_name.to_ascii_lowercase()) {
                let has_vars = traced_parts.iter().any(|p| !matches!(p, ConcatPart::Literal(_)));
                if has_vars {
                    return join_concat_parts(traced_parts, &replace_pl_vars_in_sql_raw, vars, assigns);
                } else {
                    let flat: String = traced_parts.iter().map(|p| match p {
                        ConcatPart::Literal(s) => s.as_str(), _ => ""
                    }).collect();
                    return replace_pl_vars_in_sql_raw(flat.trim(), vars);
                }
            }
        }
        _ => {}
    }

    let parts = eval_concat_expr(&exec.string_expr);
    let has_vars = parts.iter().any(|p| !matches!(p, ConcatPart::Literal(_)));
    if has_vars {
        return join_concat_parts(&parts, &replace_pl_vars_in_sql_raw, vars, assigns);
    }

    let plain = extract_execute_sql_content(exec);
    replace_pl_vars_in_sql_raw(&plain, vars)
}

fn format_pl_expr(expr: &ogsql_parser::ast::Expr) -> String {
    use ogsql_parser::ast::{Expr, Literal};

    match expr {
        Expr::Literal(Literal::String(s)) => format!("'{}'", s),
        Expr::Literal(Literal::DollarString { tag: None, body }) => format!("$${}$$", body),
        Expr::Literal(Literal::DollarString { tag: Some(t), body }) => format!("${}${}", t, body),
        Expr::Literal(Literal::EscapeString(s)) => format!("E'{}'", s),
        Expr::Literal(Literal::Integer(n)) => n.to_string(),
        Expr::Literal(Literal::Float(s)) => s.clone(),
        Expr::Literal(Literal::Boolean(b)) => b.to_string(),
        Expr::Literal(Literal::Null) => "NULL".into(),
        Expr::Literal(Literal::BitString(s)) => format!("B'{}'", s),
        Expr::Literal(Literal::HexString(s)) => format!("X'{}'", s),
        Expr::Literal(Literal::NationalString(s)) => format!("N'{}'", s),
        Expr::ColumnRef(names) | Expr::PlVariable(names) => names.join("."),
        Expr::BinaryOp { op, left, right, .. } => {
            format!("{} {} {}", format_pl_expr(left), op, format_pl_expr(right))
        }
        Expr::Parenthesized(inner) => format!("({})", format_pl_expr(inner)),
        Expr::FunctionCall { name, args, .. } => {
            let formatted_args: Vec<String> = args.iter().map(|a| format_pl_expr(a)).collect();
            format!("{}({})", name.join("."), formatted_args.join(", "))
        }
        _ => format!("{:?}", expr),
    }
}

fn extract_block_sql(block: &ogsql_parser::ast::plpgsql::PlBlock) -> String {
    use ogsql_parser::ast::plpgsql::PlStatement;
    let mut parts: Vec<String> = Vec::new();
    for stmt in &block.body {
        match stmt {
            PlStatement::SqlStatement { sql_text, .. } => {
                if !sql_text.is_empty() {
                    parts.push(sql_text.clone());
                }
            }
            PlStatement::Sql(text) => {
                if !text.is_empty() {
                    parts.push(text.clone());
                }
            }
            PlStatement::Execute(spanned) => {
                let sql = extract_execute_sql_content(&spanned.node);
                if !sql.is_empty() {
                    parts.push(sql);
                }
            }
            PlStatement::Perform { query, .. } => {
                parts.push(format!("PERFORM {}", query));
            }
            _ => {}
        }
    }
    parts.join("\\n")
}

fn format_using_args_pl(args: &[ogsql_parser::ast::plpgsql::PlUsingArg]) -> String {
    if args.is_empty() {
        return String::new();
    }
    let parts: Vec<String> = args.iter().map(|a| {
        let mode_prefix = match a.mode {
            ogsql_parser::ast::plpgsql::PlUsingMode::In => "",
            ogsql_parser::ast::plpgsql::PlUsingMode::Out => "OUT ",
            ogsql_parser::ast::plpgsql::PlUsingMode::InOut => "INOUT ",
        };
        format!("{}{}", mode_prefix, format_pl_expr(&a.argument))
    }).collect();
    format!("USING {}", parts.join(", "))
}

fn format_using_args_exprs(args: &[ogsql_parser::ast::Expr]) -> String {
    if args.is_empty() {
        return String::new();
    }
    let parts: Vec<String> = args.iter().map(|a| format_pl_expr(a)).collect();
    parts.join(", ")
}

fn build_dynamic_sql_from_expr(
    expr: &ogsql_parser::ast::Expr,
    vars: &std::collections::HashMap<String, Option<String>>,
    assigns: &std::collections::HashMap<String, Vec<ConcatPart>>,
) -> String {
    use ogsql_parser::ast::{Expr, Literal};

    match expr {
        Expr::Literal(Literal::String(s)) => replace_pl_vars_in_sql(s.trim(), vars),
        Expr::Literal(Literal::DollarString { body, .. }) => replace_pl_vars_in_sql(body.trim(), vars),
        Expr::Literal(Literal::EscapeString(s)) => replace_pl_vars_in_sql(s.trim(), vars),
        Expr::ColumnRef(names) | Expr::PlVariable(names) if names.len() == 1 => {
            let var_name = &names[0];
            if let Some(traced_parts) = assigns.get(&var_name.to_ascii_lowercase()) {
                let has_vars = traced_parts.iter().any(|p| !matches!(p, ConcatPart::Literal(_)));
                if has_vars {
                    return join_concat_parts(traced_parts, &replace_pl_vars_in_sql_raw, vars, assigns);
                } else {
                    let flat: String = traced_parts.iter().map(|p| match p {
                        ConcatPart::Literal(s) => s.as_str(), _ => ""
                    }).collect();
                    return replace_pl_vars_in_sql_raw(flat.trim(), vars);
                }
            }
            format_pl_expr(expr)
        }
        Expr::BinaryOp { op, .. } if op == "||" => {
            let parts = eval_concat_expr(expr);
            join_concat_parts(&parts, &replace_pl_vars_in_sql_raw, vars, assigns)
        }
        _ => format_pl_expr(expr),
    }
}

fn resolve_for_query_text(
    query: &str,
    vars: &std::collections::HashMap<String, Option<String>>,
    assigns: &std::collections::HashMap<String, Vec<ConcatPart>>,
) -> (String, String) {
    let q = query.trim();
    if q.is_empty() {
        return ("Embedded/Select".into(), String::new());
    }

    let is_execute_prefix = q.to_ascii_lowercase().starts_with("execute ");
    let looks_like_static_sql = ["select ", "insert ", "update ", "delete ", "merge ", "with "]
        .iter()
        .any(|kw| q.to_ascii_lowercase().starts_with(kw));

    if is_execute_prefix {
        let after_execute = q[8..].trim_start();
        let after_execute = if after_execute.to_ascii_lowercase().starts_with("immediate ") {
            after_execute[9..].trim_start()
        } else {
            after_execute
        };
        let inner = strip_trailing_using(after_execute);
        let sql = resolve_dynamic_query_text(inner.trim(), vars, assigns);
        let using_part = extract_trailing_using_text(after_execute);
        let full_sql = if using_part.is_empty() {
            sql
        } else {
            format!("{}\nUSING {}", sql, replace_pl_vars_in_sql(&using_part, vars))
        };
        return ("Embedded/Execute".into(), full_sql);
    }

    if looks_like_static_sql {
        let (sql_part, using_part) = split_query_and_using(q);
        let sql = replace_pl_vars_in_sql(sql_part.trim(), vars);
        let full_sql = if using_part.is_empty() {
            sql
        } else {
            format!("{}\nUSING {}", sql, replace_pl_vars_in_sql(&using_part, vars))
        };
        return ("Embedded/Select".into(), full_sql);
    }

    let inner = strip_trailing_using(q);
    let using_part = extract_trailing_using_text(q);
    let sql = resolve_dynamic_query_text(inner.trim(), vars, assigns);
    let full_sql = if using_part.is_empty() {
        sql
    } else {
        format!("{}\nUSING {}", sql, replace_pl_vars_in_sql(&using_part, vars))
    };
    ("Embedded/Execute".into(), full_sql)
}

fn split_query_and_using(text: &str) -> (String, String) {
    if let Some(pos) = find_using_keyword_pos(text) {
        (text[..pos].to_string(), text[pos + 5..].trim().to_string())
    } else {
        (text.to_string(), String::new())
    }
}

fn strip_trailing_using(text: &str) -> String {
    if let Some(pos) = find_using_keyword_pos(text) {
        text[..pos].to_string()
    } else {
        text.to_string()
    }
}

fn extract_trailing_using_text(text: &str) -> String {
    if let Some(pos) = find_using_keyword_pos(text) {
        text[pos + 5..].trim().to_string()
    } else {
        String::new()
    }
}

fn find_using_keyword_pos(text: &str) -> Option<usize> {
    let lower = text.to_ascii_lowercase();
    let mut in_string_single = false;
    let mut in_string_double = false;
    for (i, c) in lower.char_indices() {
        if c == '\'' && !in_string_double {
            in_string_single = !in_string_single;
            continue;
        }
        if c == '"' && !in_string_single {
            in_string_double = !in_string_double;
            continue;
        }
        if !in_string_single && !in_string_double && lower[i..].starts_with("using ") {
            return Some(i);
        }
    }
    None
}

fn resolve_dynamic_query_text(
    text: &str,
    vars: &std::collections::HashMap<String, Option<String>>,
    assigns: &std::collections::HashMap<String, Vec<ConcatPart>>,
) -> String {
    let t = text.trim();
    if t.is_empty() {
        return String::new();
    }
    if t.contains("||") {
        let (stmts, _) = ogsql_parser::parser::Parser::parse_sql(&format!("SELECT {}", t));
        if let Some(si) = stmts.first() {
            if let ogsql_parser::Statement::Select(ref sel) = si.statement {
                if let Some(ref first_item) = sel.node.targets.first() {
                    if let ogsql_parser::ast::SelectTarget::Expr(expr, _) = first_item {
                        let parts = eval_concat_expr(expr);
                        return join_concat_parts(&parts, &replace_pl_vars_in_sql_raw, vars, assigns);
                    }
                }
            }
        }
    }
    if let Some(rest) = t.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')) {
        return replace_pl_vars_in_sql(rest.trim(), vars);
    }
    if let Some(rest) = t.strip_prefix("E'").and_then(|s| s.strip_suffix('\'')) {
        return replace_pl_vars_in_sql(rest.trim(), vars);
    }
    if t.starts_with("$$") {
        if let Some(rest) = t.strip_prefix("$$").and_then(|s| s.strip_suffix("$$")) {
            return replace_pl_vars_in_sql(rest.trim(), vars);
        }
    }
    if let Some(traced_parts) = assigns.get(&t.to_ascii_lowercase()) {
        let has_vars = traced_parts.iter().any(|p| !matches!(p, ConcatPart::Literal(_)));
        if has_vars {
            return join_concat_parts(traced_parts, &replace_pl_vars_in_sql_raw, vars, assigns);
        } else {
            let flat: String = traced_parts.iter().map(|p| match p {
                ConcatPart::Literal(s) => s.as_str(), _ => ""
            }).collect();
            return replace_pl_vars_in_sql_raw(flat.trim(), vars);
        }
    }
    replace_pl_vars_in_sql(t, vars)
}

/// Recursively collect SQL rows from a PL/pgSQL block into individual CSV rows.
fn collect_block_sql_rows(
    block: &ogsql_parser::ast::plpgsql::PlBlock,
    parent_name: &str,
    fallback_line: usize,
    vars: &std::collections::HashMap<String, Option<String>>,
) -> Vec<ParseCsvRow> {
    use ogsql_parser::ast::plpgsql::PlDeclaration;
    let mut rows = Vec::new();
    let mut assigns: std::collections::HashMap<String, Vec<ConcatPart>> = std::collections::HashMap::new();
    for decl in &block.declarations {
        if let PlDeclaration::Variable(ref v) = decl {
            if let Some(ref expr) = v.default {
                let parts = eval_concat_expr(expr);
                if !parts.is_empty() {
                    assigns.insert(v.name.to_ascii_lowercase(), parts);
                }
            }
        }
    }
    for stmt in &block.body {
        collect_pl_stmt_rows(stmt, parent_name, fallback_line, vars, &mut assigns, &mut rows);
    }
    if let Some(ref exc) = block.exception_block {
        for handler in &exc.handlers {
            for stmt in &handler.statements {
                collect_pl_stmt_rows(stmt, parent_name, fallback_line, vars, &mut assigns, &mut rows);
            }
        }
    }
    rows
}

/// Extract the start line from a Spanned PlStatement variant, if available.
fn spanned_line(span: &Option<ogsql_parser::ast::SourceSpan>) -> usize {
    span.as_ref().map_or(0, |s| s.start.line)
}

/// Derive a descriptive type string and target name from an embedded SQL Statement.
fn sql_statement_type_and_name(stmt: &ogsql_parser::Statement) -> (String, String) {
    use ogsql_parser::Statement;
    match stmt {
        Statement::Select(s) => (
            "SqlStatement/Select".into(),
            s.from.first().and_then(|f| {
                if let ogsql_parser::ast::TableRef::Table { name, .. } = f {
                    Some(name.join("."))
                } else {
                    None
                }
            }).unwrap_or_default(),
        ),
        Statement::Insert(s) => ("SqlStatement/Insert".into(), s.table.join(".")),
        Statement::Update(s) => (
            "SqlStatement/Update".into(),
            s.tables.first().and_then(|f| {
                if let ogsql_parser::ast::TableRef::Table { name, .. } = f {
                    Some(name.join("."))
                } else {
                    None
                }
            }).unwrap_or_default(),
        ),
        Statement::Delete(s) => (
            "SqlStatement/Delete".into(),
            s.tables.first().and_then(|f| {
                if let ogsql_parser::ast::TableRef::Table { name, .. } = f {
                    Some(name.join("."))
                } else {
                    None
                }
            }).unwrap_or_default(),
        ),
        Statement::Merge(s) => (
            "SqlStatement/Merge".into(),
            match &s.target {
                ogsql_parser::ast::TableRef::Table { name, .. } => name.join("."),
                _ => String::new(),
            },
        ),
        Statement::CreateTable(s) => ("SqlStatement/CreateTable".into(), s.name.join(".")),
        _ => {
            let type_name = serde_json::to_value(stmt)
                .ok()
                .and_then(|v| {
                    if let serde_json::Value::Object(map) = v {
                        map.keys().next().cloned()
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| "Unknown".to_string());
            (format!("SqlStatement/{}", type_name), String::new())
        }
    }
}

/// Recursively walk a single PlStatement, collecting SQL-bearing nodes as CSV rows.
fn collect_pl_stmt_rows(
    pl_stmt: &ogsql_parser::ast::plpgsql::PlStatement,
    parent_name: &str,
    fallback_line: usize,
    vars: &std::collections::HashMap<String, Option<String>>,
    assigns: &mut std::collections::HashMap<String, Vec<ConcatPart>>,
    rows: &mut Vec<ParseCsvRow>,
) {
    use ogsql_parser::ast::plpgsql::PlStatement;

    match pl_stmt {
        PlStatement::Assignment { target, expression } => {
            let target_name = match target {
                ogsql_parser::ast::Expr::PlVariable(n) | ogsql_parser::ast::Expr::ColumnRef(n) => {
                    n.last().cloned().unwrap_or_default()
                }
                _ => String::new(),
            };
            if !target_name.is_empty() {
                let key = target_name.to_ascii_lowercase();
                let mut parts = eval_concat_expr(expression);
                if let Some(prev) = assigns.get(&key).cloned() {
                    let mut resolved = Vec::with_capacity(parts.len());
                    for part in &parts {
                        match part {
                            ConcatPart::Variable(name) if name.to_ascii_lowercase() == key => {
                                resolved.extend(prev.iter().cloned());
                            }
                            other => resolved.push(other.clone()),
                        }
                    }
                    parts = resolved;
                }
                assigns.insert(key, parts);
            }
        }

        PlStatement::SqlStatement { span, sql_text, statement } => {
            let (stmt_type, name) = sql_statement_type_and_name(statement);
            let sql = replace_pl_vars_in_sql(sql_text.trim(), vars);
            let line = span.as_ref().map(|s| s.start.line).unwrap_or(fallback_line).max(1);
            rows.push(ParseCsvRow {
                line,
                stmt_type,
                name,
                parent: parent_name.to_string(),
                parameters: String::new(),
                return_type: String::new(),
                sql,
            });
        }
        PlStatement::Sql(text) => {
            if !text.is_empty() {
                if let Some((var, rhs)) = try_parse_sql_assignment(text) {
                    let rhs_sql = rhs.trim();
                    if rhs_sql.starts_with('"') && rhs_sql.ends_with('"') {
                        let inner = &rhs_sql[1..rhs_sql.len()-1];
                        assigns.insert(var.to_ascii_lowercase(), vec![ConcatPart::Literal(inner.to_string())]);
                    } else if rhs_sql.starts_with('\'') && rhs_sql.ends_with('\'') {
                        let inner = &rhs_sql[1..rhs_sql.len()-1];
                        assigns.insert(var.to_ascii_lowercase(), vec![ConcatPart::Literal(inner.to_string())]);
                    }
                }
                let sql = replace_pl_vars_in_sql(text.trim(), vars);
                rows.push(ParseCsvRow {
                    line: fallback_line,
                    stmt_type: "Sql".into(),
                    name: String::new(),
                    parent: parent_name.to_string(),
                    parameters: String::new(),
                    return_type: String::new(),
                    sql,
                });
            }
        }
        PlStatement::Execute(spanned) => {
            let line = spanned_line(&spanned.span).max(fallback_line);
            let sql = build_execute_csv_sql_with_trace(&spanned.node, vars, assigns);
            rows.push(ParseCsvRow {
                line,
                stmt_type: "Execute".into(),
                name: String::new(),
                parent: parent_name.to_string(),
                parameters: String::new(),
                return_type: String::new(),
                sql,
            });
        }
        PlStatement::Perform { span, query, .. } => {
            let sql = replace_pl_vars_in_sql(&format!("PERFORM {}", query), vars);
            let line = span.as_ref().map(|s| s.start.line).unwrap_or(fallback_line).max(1);
            rows.push(ParseCsvRow {
                line,
                stmt_type: "Perform".into(),
                name: String::new(),
                parent: parent_name.to_string(),
                parameters: String::new(),
                return_type: String::new(),
                sql,
            });
        }
        PlStatement::Block(spanned) => {
            let line = spanned_line(&spanned.span).max(fallback_line);
            for s in &spanned.node.body {
                collect_pl_stmt_rows(s, parent_name, line, vars, assigns, rows);
            }
            if let Some(ref exc) = spanned.node.exception_block {
                for handler in &exc.handlers {
                    for s in &handler.statements {
                        collect_pl_stmt_rows(s, parent_name, line, vars, assigns, rows);
                    }
                }
            }
        }
        PlStatement::If(spanned) => {
            let line = spanned_line(&spanned.span).max(fallback_line);
            let if_stmt = &spanned.node;
            for s in &if_stmt.then_stmts {
                collect_pl_stmt_rows(s, parent_name, line, vars, assigns, rows);
            }
            for elsif in &if_stmt.elsifs {
                for s in &elsif.stmts {
                    collect_pl_stmt_rows(s, parent_name, line, vars, assigns, rows);
                }
            }
            for s in &if_stmt.else_stmts {
                collect_pl_stmt_rows(s, parent_name, line, vars, assigns, rows);
            }
        }
        PlStatement::Case(spanned) => {
            let line = spanned_line(&spanned.span).max(fallback_line);
            let case_stmt = &spanned.node;
            for when in &case_stmt.whens {
                for s in &when.stmts {
                    collect_pl_stmt_rows(s, parent_name, line, vars, assigns, rows);
                }
            }
            for s in &case_stmt.else_stmts {
                collect_pl_stmt_rows(s, parent_name, line, vars, assigns, rows);
            }
        }
        PlStatement::Loop(spanned) => {
            let line = spanned_line(&spanned.span).max(fallback_line);
            for s in &spanned.node.body {
                collect_pl_stmt_rows(s, parent_name, line, vars, assigns, rows);
            }
        }
        PlStatement::While(spanned) => {
            let line = spanned_line(&spanned.span).max(fallback_line);
            for s in &spanned.node.body {
                collect_pl_stmt_rows(s, parent_name, line, vars, assigns, rows);
            }
        }
        PlStatement::For(spanned) => {
            let line = spanned_line(&spanned.span).max(fallback_line);
            let for_stmt = &spanned.node;
            match &for_stmt.kind {
                ogsql_parser::ast::plpgsql::PlForKind::Query { query, parsed_query, using_args: _ } => {
                    rows.push(ParseCsvRow {
                        line,
                        stmt_type: "ForQuery".into(),
                        name: for_stmt.variable.clone(),
                        parent: parent_name.to_string(),
                        parameters: String::new(),
                        return_type: String::new(),
                        sql: String::new(),
                    });
                    if let Some(ref stmt) = parsed_query {
                        let formatter = ogsql_parser::SqlFormatter::new();
                        let formatted = formatter.format_statement(stmt);
                        let sql = replace_pl_vars_in_sql(&formatted, vars);
                        rows.push(ParseCsvRow {
                            line,
                            stmt_type: "Embedded/Select".into(),
                            name: String::new(),
                            parent: parent_name.to_string(),
                            parameters: String::new(),
                            return_type: String::new(),
                            sql,
                        });
                    } else {
                        let (embedded_type, sql) = resolve_for_query_text(query, vars, assigns);
                        rows.push(ParseCsvRow {
                            line,
                            stmt_type: embedded_type,
                            name: String::new(),
                            parent: parent_name.to_string(),
                            parameters: String::new(),
                            return_type: String::new(),
                            sql,
                        });
                    }
                }
                ogsql_parser::ast::plpgsql::PlForKind::Cursor { cursor_name, arguments } => {
                    let args_str: Vec<String> = arguments.iter().map(|a| format_pl_expr(a)).collect();
                    rows.push(ParseCsvRow {
                        line,
                        stmt_type: "ForCursor".into(),
                        name: for_stmt.variable.clone(),
                        parent: parent_name.to_string(),
                        parameters: args_str.join(", "),
                        return_type: String::new(),
                        sql: String::new(),
                    });
                }
                _ => {}
            }
            for s in &for_stmt.body {
                collect_pl_stmt_rows(s, parent_name, line, vars, assigns, rows);
            }
        }
        PlStatement::ForEach(spanned) => {
            let line = spanned_line(&spanned.span).max(fallback_line);
            for s in &spanned.node.body {
                collect_pl_stmt_rows(s, parent_name, line, vars, assigns, rows);
            }
        }
        PlStatement::ForAll(spanned) => {
            let line = spanned_line(&spanned.span).max(fallback_line);
            let body = &spanned.node.body;
            if !body.is_empty() {
                rows.push(ParseCsvRow {
                    line,
                    stmt_type: "ForAll".into(),
                    name: String::new(),
                    parent: parent_name.to_string(),
                    parameters: String::new(),
                    return_type: String::new(),
                    sql: body.trim().to_string(),
                });
            }
        }
        PlStatement::Open(spanned) => {
            let line = spanned_line(&spanned.span).max(fallback_line);
            let open_stmt = &spanned.node;
            let cursor_name = format_pl_expr(&open_stmt.cursor);
            match &open_stmt.kind {
                ogsql_parser::ast::plpgsql::PlOpenKind::ForQuery { scroll: _, query, parsed_query } => {
                    rows.push(ParseCsvRow {
                        line,
                        stmt_type: "Open/ForQuery".into(),
                        name: cursor_name,
                        parent: parent_name.to_string(),
                        parameters: String::new(),
                        return_type: String::new(),
                        sql: String::new(),
                    });
                    if let Some(ref stmt) = parsed_query {
                        let formatter = ogsql_parser::SqlFormatter::new();
                        let formatted = formatter.format_statement(stmt);
                        let sql = replace_pl_vars_in_sql(&formatted, vars);
                        rows.push(ParseCsvRow {
                            line,
                            stmt_type: "Embedded/Select".into(),
                            name: String::new(),
                            parent: parent_name.to_string(),
                            parameters: String::new(),
                            return_type: String::new(),
                            sql,
                        });
                    } else {
                        let (embedded_type, sql) = resolve_for_query_text(query, vars, assigns);
                        rows.push(ParseCsvRow {
                            line,
                            stmt_type: embedded_type,
                            name: String::new(),
                            parent: parent_name.to_string(),
                            parameters: String::new(),
                            return_type: String::new(),
                            sql,
                        });
                    }
                }
                ogsql_parser::ast::plpgsql::PlOpenKind::ForExecute { query, using_args } => {
                    rows.push(ParseCsvRow {
                        line,
                        stmt_type: "Open/ForExecute".into(),
                        name: cursor_name,
                        parent: parent_name.to_string(),
                        parameters: String::new(),
                        return_type: String::new(),
                        sql: String::new(),
                    });
                    let dynamic_sql = build_dynamic_sql_from_expr(query, vars, assigns);
                    let using_suffix = format_using_args_exprs(using_args);
                    let full_sql = if using_suffix.is_empty() {
                        dynamic_sql
                    } else {
                        format!("{}\nUSING {}", dynamic_sql, using_suffix)
                    };
                    rows.push(ParseCsvRow {
                        line,
                        stmt_type: "Embedded/Execute".into(),
                        name: String::new(),
                        parent: parent_name.to_string(),
                        parameters: String::new(),
                        return_type: String::new(),
                        sql: full_sql,
                    });
                }
                ogsql_parser::ast::plpgsql::PlOpenKind::ForUsing { expressions } => {
                    rows.push(ParseCsvRow {
                        line,
                        stmt_type: "Open/ForUsing".into(),
                        name: cursor_name,
                        parent: parent_name.to_string(),
                        parameters: String::new(),
                        return_type: String::new(),
                        sql: String::new(),
                    });
                    let exprs: Vec<String> = expressions.iter().map(|e| format_pl_expr(e)).collect();
                    rows.push(ParseCsvRow {
                        line,
                        stmt_type: "Embedded/Execute".into(),
                        name: String::new(),
                        parent: parent_name.to_string(),
                        parameters: String::new(),
                        return_type: String::new(),
                        sql: exprs.join(", "),
                    });
                }
                ogsql_parser::ast::plpgsql::PlOpenKind::Simple { arguments } => {
                    let args_str: Vec<String> = arguments.iter().map(|a| format_pl_expr(a)).collect();
                    rows.push(ParseCsvRow {
                        line,
                        stmt_type: "Open/Simple".into(),
                        name: cursor_name,
                        parent: parent_name.to_string(),
                        parameters: args_str.join(", "),
                        return_type: String::new(),
                        sql: String::new(),
                    });
                }
            }
        }
        _ => {}
    }
}

fn flatten_statement(si: &ogsql_parser::StatementInfo, mybatis: bool) -> Vec<ParseCsvRow> {
    use ogsql_parser::Statement;
    let mut rows = Vec::new();

    match &si.statement {
        Statement::CreatePackageBody(s) => {
            rows.push(ParseCsvRow {
                line: si.start_line,
                stmt_type: "CreatePackageBody".into(),
                name: s.name.join("."),
                parent: String::new(),
                parameters: String::new(),
                return_type: String::new(),
                sql: String::new(),
            });
            for item in &s.items {
                match item {
                    ogsql_parser::ast::PackageItem::Procedure(p) => {
                        rows.push(ParseCsvRow {
                            line: p.start_line.max(si.start_line),
                            stmt_type: "Procedure".into(),
                            name: p.name.join("."),
                            parent: s.name.join("."),
                            parameters: format_params(&p.parameters),
                            return_type: String::new(),
                            sql: String::new(),
                        });
                        if let Some(ref block) = p.block {
                            let vars = if mybatis { collect_block_vars(block, &p.parameters) } else { std::collections::HashMap::new() };
                            rows.extend(collect_block_sql_rows(
                                block,
                                &p.name.join("."),
                                p.start_line.max(si.start_line),
                                &vars,
                            ));
                        }
                    }
                    ogsql_parser::ast::PackageItem::Function(f) => {
                        rows.push(ParseCsvRow {
                            line: f.start_line.max(si.start_line),
                            stmt_type: "Function".into(),
                            name: f.name.join("."),
                            parent: s.name.join("."),
                            parameters: format_params(&f.parameters),
                            return_type: f.return_type.clone().unwrap_or_default(),
                            sql: String::new(),
                        });
                        if let Some(ref block) = f.block {
                            let vars = if mybatis { collect_block_vars(block, &f.parameters) } else { std::collections::HashMap::new() };
                            rows.extend(collect_block_sql_rows(
                                block,
                                &f.name.join("."),
                                f.start_line.max(si.start_line),
                                &vars,
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
        Statement::CreatePackage(s) => {
            rows.push(ParseCsvRow {
                line: si.start_line,
                stmt_type: "CreatePackage".into(),
                name: s.name.join("."),
                parent: String::new(),
                parameters: String::new(),
                return_type: String::new(),
                sql: String::new(),
            });
            for item in &s.items {
                match item {
                    ogsql_parser::ast::PackageItem::Procedure(p) => {
                        rows.push(ParseCsvRow {
                            line: p.start_line.max(si.start_line),
                            stmt_type: "Procedure".into(),
                            name: p.name.join("."),
                            parent: s.name.join("."),
                            parameters: format_params(&p.parameters),
                            return_type: String::new(),
                            sql: String::new(),
                        });
                    }
                    ogsql_parser::ast::PackageItem::Function(f) => {
                        rows.push(ParseCsvRow {
                            line: f.start_line.max(si.start_line),
                            stmt_type: "Function".into(),
                            name: f.name.join("."),
                            parent: s.name.join("."),
                            parameters: format_params(&f.parameters),
                            return_type: f.return_type.clone().unwrap_or_default(),
                            sql: String::new(),
                        });
                    }
                    _ => {}
                }
            }
        }
        Statement::CreateProcedure(s) => {
            let proc_name = s.name.join(".");
            rows.push(ParseCsvRow {
                line: si.start_line,
                stmt_type: "CreateProcedure".into(),
                name: proc_name.clone(),
                parent: String::new(),
                parameters: format_params(&s.parameters),
                return_type: String::new(),
                sql: String::new(),
            });
            if let Some(ref block) = s.block {
                let vars = if mybatis { collect_block_vars(block, &s.parameters) } else { std::collections::HashMap::new() };
                rows.extend(collect_block_sql_rows(block, &proc_name, si.start_line, &vars));
            }
        }
        Statement::CreateFunction(s) => {
            let func_name = s.name.join(".");
            rows.push(ParseCsvRow {
                line: si.start_line,
                stmt_type: "CreateFunction".into(),
                name: func_name.clone(),
                parent: String::new(),
                parameters: format_params(&s.parameters),
                return_type: s.return_type.clone().unwrap_or_default(),
                sql: String::new(),
            });
            if let Some(ref block) = s.block {
                let vars = if mybatis { collect_block_vars(block, &s.parameters) } else { std::collections::HashMap::new() };
                rows.extend(collect_block_sql_rows(block, &func_name, si.start_line, &vars));
            }
        }
        Statement::Do(s) => {
            rows.push(ParseCsvRow {
                line: si.start_line,
                stmt_type: "Do".into(),
                name: String::new(),
                parent: String::new(),
                parameters: String::new(),
                return_type: String::new(),
                sql: String::new(),
            });
            if let Some(ref block) = s.block {
                let vars = if mybatis { collect_block_vars(block, &[]) } else { std::collections::HashMap::new() };
                rows.extend(collect_block_sql_rows(block, "", si.start_line, &vars));
            }
        }
        Statement::AnonyBlock(s) => {
            rows.push(ParseCsvRow {
                line: si.start_line,
                stmt_type: "AnonyBlock".into(),
                name: String::new(),
                parent: String::new(),
                parameters: String::new(),
                return_type: String::new(),
                sql: String::new(),
            });
            let vars = if mybatis { collect_block_vars(&s.block, &[]) } else { std::collections::HashMap::new() };
            rows.extend(collect_block_sql_rows(&s.block, "", si.start_line, &vars));
        }
        Statement::Select(s) => {
            rows.push(ParseCsvRow {
                line: si.start_line,
                stmt_type: "Select".into(),
                name: s.from.first().and_then(|f| {
                    if let ogsql_parser::ast::TableRef::Table { name, .. } = f {
                        Some(name.join("."))
                    } else {
                        None
                    }
                }).unwrap_or_default(),
                parent: String::new(),
                parameters: String::new(),
                return_type: String::new(),
                sql: si.sql_text.clone(),
            });
        }
        Statement::Insert(s) => {
            rows.push(ParseCsvRow {
                line: si.start_line,
                stmt_type: "Insert".into(),
                name: s.table.join("."),
                parent: String::new(),
                parameters: String::new(),
                return_type: String::new(),
                sql: si.sql_text.clone(),
            });
        }
        Statement::Update(s) => {
            rows.push(ParseCsvRow {
                line: si.start_line,
                stmt_type: "Update".into(),
                name: s.tables.first().and_then(|f| {
                    if let ogsql_parser::ast::TableRef::Table { name, .. } = f {
                        Some(name.join("."))
                    } else {
                        None
                    }
                }).unwrap_or_default(),
                parent: String::new(),
                parameters: String::new(),
                return_type: String::new(),
                sql: si.sql_text.clone(),
            });
        }
        Statement::Delete(s) => {
            rows.push(ParseCsvRow {
                line: si.start_line,
                stmt_type: "Delete".into(),
                name: s.tables.first().and_then(|f| {
                    if let ogsql_parser::ast::TableRef::Table { name, .. } = f {
                        Some(name.join("."))
                    } else {
                        None
                    }
                }).unwrap_or_default(),
                parent: String::new(),
                parameters: String::new(),
                return_type: String::new(),
                sql: si.sql_text.clone(),
            });
        }
        Statement::Merge(s) => {
            rows.push(ParseCsvRow {
                line: si.start_line,
                stmt_type: "Merge".into(),
                name: match &s.target {
                    ogsql_parser::ast::TableRef::Table { name, .. } => name.join("."),
                    _ => String::new(),
                },
                parent: String::new(),
                parameters: String::new(),
                return_type: String::new(),
                sql: si.sql_text.clone(),
            });
        }
        Statement::CreateTable(s) => {
            rows.push(ParseCsvRow {
                line: si.start_line,
                stmt_type: "CreateTable".into(),
                name: s.name.join("."),
                parent: String::new(),
                parameters: String::new(),
                return_type: String::new(),
                sql: si.sql_text.clone(),
            });
        }
        _ => {
            let type_name = serde_json::to_value(&si.statement)
                .ok()
                .and_then(|v| {
                    if let serde_json::Value::Object(map) = v {
                        map.keys().next().cloned()
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| "Unknown".to_string());
            rows.push(ParseCsvRow {
                line: si.start_line,
                stmt_type: type_name,
                name: String::new(),
                parent: String::new(),
                parameters: String::new(),
                return_type: String::new(),
                sql: si.sql_text.clone(),
            });
        }
    }
    rows
}

fn output_csv_parse_rows(
    statements: &[ogsql_parser::StatementInfo],
    file_name: &str,
    rel_dir: &str,
    errors: &[ogsql_parser::ParserError],
    mybatis: bool,
) {
    for si in statements {
        let stmt_start = si.start_line;
        let stmt_end = si.end_line;

        let stmt_errors: Vec<&ogsql_parser::ParserError> = errors
            .iter()
            .filter(|e| {
                let eline = error_line(e);
                eline == 0 || (eline >= stmt_start && eline <= stmt_end)
            })
            .collect();

        let rows = flatten_statement(si, mybatis);
        for row in rows {
            let (row_err, row_warn) = filter_errors_for_row(
                &stmt_errors,
                &si.statement,
                row.line,
            );

            let sql = row.sql.trim().replace('\n', "\\n").replace('\r', "");
            println!(
                "{},{},{},{},{},{},{},{},{},{},{}",
                csv_escape(file_name),
                csv_escape(rel_dir),
                row.line,
                csv_escape(&row.stmt_type),
                csv_escape(&row.name),
                csv_escape(&row.parent),
                csv_escape(&row.parameters),
                csv_escape(&row.return_type),
                csv_escape(&sql),
                csv_escape(&row_err),
                csv_escape(&row_warn),
            );
        }
    }
}

fn filter_errors_for_row(
    errors: &[&ogsql_parser::ParserError],
    stmt: &ogsql_parser::Statement,
    row_line: usize,
) -> (String, String) {
    use ogsql_parser::ast::PackageItem;

    let sub_range: Option<(usize, usize)> = match stmt {
        ogsql_parser::Statement::CreatePackageBody(pkg) => {
            pkg.items.iter().find_map(|item| match item {
                PackageItem::Procedure(p)
                    if p.start_line > 0 && row_line >= p.start_line && row_line <= p.end_line =>
                {
                    Some((p.start_line, p.end_line))
                }
                PackageItem::Function(f)
                    if f.start_line > 0 && row_line >= f.start_line && row_line <= f.end_line =>
                {
                    Some((f.start_line, f.end_line))
                }
                _ => None,
            })
        }
        ogsql_parser::Statement::CreatePackage(pkg) => {
            pkg.items.iter().find_map(|item| match item {
                PackageItem::Procedure(p)
                    if p.start_line > 0 && row_line >= p.start_line && row_line <= p.end_line =>
                {
                    Some((p.start_line, p.end_line))
                }
                PackageItem::Function(f)
                    if f.start_line > 0 && row_line >= f.start_line && row_line <= f.end_line =>
                {
                    Some((f.start_line, f.end_line))
                }
                _ => None,
            })
        }
        _ => None,
    };

    let (range_start, range_end) = sub_range.unwrap_or((row_line, row_line));

    let real_errors: Vec<String> = errors
        .iter()
        .filter(|e| !is_warning(e))
        .filter(|e| {
            let eline = error_line(e);
            eline >= range_start && eline <= range_end
        })
        .map(|e| e.to_string())
        .collect();

    let warnings: Vec<String> = errors
        .iter()
        .filter(|e| is_warning(e))
        .filter(|e| {
            let eline = error_line(e);
            eline >= range_start && eline <= range_end
        })
        .map(|e| e.to_string())
        .collect();

    (real_errors.join("; "), warnings.join("; "))
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
    let mut tokenizer = Tokenizer::new(&sql);
    if cli.comments {
        tokenizer = tokenizer.preserve_comments(true);
    }
    if cli.mybatis {
        tokenizer = tokenizer.mybatis_params(true);
    }
    let tokens = match tokenizer.tokenize() {
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

fn validate_sql(
    sql: &str,
    mybatis: bool,
) -> (Vec<ogsql_parser::StatementInfo>, Vec<ogsql_parser::ParserError>, Vec<ogsql_parser::PackageConsistencyError>) {
    let output = parse_input(sql, false, mybatis);
    let stmts = output.statements;
    let mut errors = output.errors;

    let pkg_errors = ogsql_parser::validate_package_consistency(&stmts);
    if !pkg_errors.is_empty() {
        for pe in &pkg_errors {
            let msg = match &pe.detail {
                Some(d) => format!("package {}: {} — {}", pe.package_name, pe.subprogram_name, d),
                None => format!("package {}: {} — {:?}", pe.package_name, pe.subprogram_name, pe.kind),
            };
            errors.push(ogsql_parser::ParserError::Warning {
                message: msg,
                location: ogsql_parser::SourceLocation::default(),
            });
        }
    }
    (stmts, errors, pkg_errors)
}

fn cmd_validate(cli: &Cli) {
    let sql = read_input(cli.file.as_deref());
    let (stmts, errors, pkg_errors) = validate_sql(&sql, cli.mybatis);

    if cli.json {
        let warnings: Vec<_> = errors.iter().filter(|e| is_warning(e)).collect();
        let real_errors: Vec<_> = errors.iter().filter(|e| !is_warning(e)).collect();
        let mut out = serde_json::json!({
            "valid": real_errors.is_empty(),
            "error_count": real_errors.len(),
            "warning_count": warnings.len(),
            "errors": errors,
        });
        if !pkg_errors.is_empty() {
            out.as_object_mut().unwrap().insert(
                "package_consistency_errors".to_string(),
                serde_json::json!(pkg_errors),
            );
        }
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

fn cmd_validate_dir(cli: &Cli, dir_paths: &[String], exts: &[String], stats: bool) {
    use std::path::Path;

    for dir_path in dir_paths {
        if !Path::new(dir_path).is_dir() {
            die!("Error: '{}' is not a directory", dir_path);
        }
    }

    let normalized_exts: Vec<String> = exts
        .iter()
        .map(|e| e.trim_start_matches('.').to_ascii_lowercase())
        .collect();

    let mut files: Vec<(String, String, std::path::PathBuf)> = Vec::new();
    for dir_path in dir_paths {
        let root = Path::new(dir_path);
        for entry in walkdir::WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let file_ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();
            if !normalized_exts.iter().any(|e| *e == file_ext) {
                continue;
            }

            let rel_dir = path
                .parent()
                .and_then(|p| p.strip_prefix(root).ok())
                .map(|p| {
                    let s = p.to_str().unwrap_or(".");
                    if s.is_empty() { "." } else { s }
                })
                .unwrap_or(".")
                .to_string();

            let file_name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();

            files.push((file_name, rel_dir, path.to_path_buf()));
        }
    }

    files.sort_by(|a, b| a.2.cmp(&b.2));
    files.dedup_by(|a, b| a.2 == b.2);

    if files.is_empty() {
        eprintln!("No files found with extension(s): {}", exts.join(", "));
        return;
    }

    let mut total_files = 0usize;
    let mut total_errors = 0usize;
    let mut total_warnings = 0usize;
    let mut any_invalid = false;
    let mut all_results: Vec<(String, String, String, Vec<ogsql_parser::ParserError>)> = Vec::new();
    let mut files_with_errors: HashSet<String> = HashSet::new();
    let mut files_with_warnings: HashSet<String> = HashSet::new();
    let mut stmt_counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut error_kinds: BTreeMap<&'static str, (usize, HashSet<String>)> = BTreeMap::new();
    let mut warning_kinds: BTreeMap<&'static str, (usize, HashSet<String>)> = BTreeMap::new();

    for (file_name, rel_dir, abs_path) in &files {
        let sql = read_file_path(abs_path);
        let (stmts, errors, _pkg_errors) = validate_sql(&sql, cli.mybatis);

        let real_errors: Vec<_> = errors.iter().filter(|e| !is_warning(e)).collect();
        let warnings: Vec<_> = errors.iter().filter(|e| is_warning(e)).collect();

        if !real_errors.is_empty() {
            any_invalid = true;
            files_with_errors.insert(file_name.clone());
        }
        if !warnings.is_empty() {
            files_with_warnings.insert(file_name.clone());
        }

        total_errors += real_errors.len();
        total_warnings += warnings.len();
        total_files += 1;

        // Stats accumulation
        for si in &stmts {
            *stmt_counts.entry(stmt_category(&si.statement)).or_insert(0) += 1;
        }
        for err in &errors {
            let kind = parser_error_kind(err);
            let file_set = if is_warning(err) { &mut warning_kinds } else { &mut error_kinds };
            let entry = file_set.entry(kind).or_insert((0, HashSet::new()));
            entry.0 += 1;
            entry.1.insert(file_name.clone());
        }

        if cli.json {
            if !real_errors.is_empty() {
                all_results.push((
                    file_name.clone(),
                    rel_dir.clone(),
                    sql,
                    errors.iter().filter(|e| !is_warning(e)).cloned().collect(),
                ));
            }
        } else {
            if real_errors.is_empty() && warnings.is_empty() {
                println!("[{}/{}] VALID", rel_dir, file_name);
            } else if real_errors.is_empty() {
                println!("[{}/{}] VALID ({} warning(s))", rel_dir, file_name, warnings.len());
                for w in &warnings {
                    eprintln!("  warning: {}", w);
                }
            } else {
                println!(
                    "[{}/{}] INVALID ({} error(s), {} warning(s))",
                    rel_dir, file_name, real_errors.len(), warnings.len()
                );
                for e in &real_errors {
                    eprintln!("  error: {}", e);
                }
                for w in &warnings {
                    eprintln!("  warning: {}", w);
                }
            }
        }
        let _ = &stmts;
    }

    if cli.json {
        let mut results = Vec::new();
        for (file_name, rel_dir, abs_path) in &files {
            let sql = read_file_path(abs_path);
            let (_stmts, errors, pkg_errors) = validate_sql(&sql, cli.mybatis);
            let real_errors: Vec<_> = errors.iter().filter(|e| !is_warning(e)).collect();
            let warnings: Vec<_> = errors.iter().filter(|e| is_warning(e)).collect();

            let mut file_result = serde_json::json!({
                "file": file_name,
                "directory": rel_dir,
                "valid": real_errors.is_empty(),
                "error_count": real_errors.len(),
                "warning_count": warnings.len(),
                "errors": errors,
            });
            if !pkg_errors.is_empty() {
                file_result.as_object_mut().unwrap().insert(
                    "package_consistency_errors".to_string(),
                    serde_json::json!(pkg_errors),
                );
            }
            results.push(file_result);
        }
        let mut out = serde_json::json!({
            "valid": !any_invalid,
            "total_files": total_files,
            "total_errors": total_errors,
            "total_warnings": total_warnings,
            "files": results,
        });
        if !all_results.is_empty() {
            out.as_object_mut().unwrap().insert(
                "error_log".to_string(),
                serde_json::json!(all_results.len()),
            );
        }
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
        if stats {
            let total_stmts: usize = stmt_counts.values().sum();
            print_parse_stats(total_files, &files_with_errors, &files_with_warnings,
                total_stmts, &stmt_counts, &error_kinds, &warning_kinds, "validate -j");
        }
    } else {
        println!();
        if stats {
            let total_stmts: usize = stmt_counts.values().sum();
            print_parse_stats(total_files, &files_with_errors, &files_with_warnings,
                total_stmts, &stmt_counts, &error_kinds, &warning_kinds, "validate");
        }
        if any_invalid {
            println!(
                "Result: INVALID — {} error(s), {} warning(s) from {} file(s)",
                total_errors, total_warnings, total_files
            );
            std::process::exit(1);
        } else if total_warnings > 0 {
            println!(
                "Result: VALID — {} warning(s) from {} file(s)",
                total_warnings, total_files
            );
        } else {
            println!("Result: VALID — {} file(s)", total_files);
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

fn write_dir_error_log(
    out_root: &std::path::Path,
    all_errors: &[(String, String, String, Vec<ogsql_parser::ParserError>)],
) {
    use std::io::Write;
    let log_path = out_root.join("error.log");
    let mut file = match std::fs::File::create(&log_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("  warning: cannot create {}: {}", log_path.display(), e);
            return;
        }
    };
    let context_radius: usize = 5;
    for (file_name, rel_dir, source, errors) in all_errors {
        let _ = writeln!(file, "File: {}/{}", rel_dir, file_name);
        for err in errors {
            let _ = writeln!(file, "Error: {}", err);
        }
        let source_lines: Vec<&str> = source.lines().collect();
        let err_lines: Vec<usize> = errors.iter().map(|e| error_line(e)).filter(|&l| l > 0).collect();
        if err_lines.is_empty() {
            let _ = writeln!(file, "{}", "-".repeat(60));
            continue;
        }
        let min_line = *err_lines.iter().min().unwrap_or(&1);
        let max_line = *err_lines.iter().max().unwrap_or(&1);
        let start = min_line.saturating_sub(context_radius + 1);
        let end = (max_line + context_radius).min(source_lines.len());
        for (i, line) in source_lines[start..end].iter().enumerate() {
            let abs_line = start + i + 1;
            if err_lines.contains(&abs_line) {
                let _ = writeln!(file, "  {:>4} |> {}", abs_line, line);
            } else {
                let _ = writeln!(file, "  {:>4} |  {}", abs_line, line);
            }
        }
        let _ = writeln!(file, "{}", "-".repeat(60));
    }
    eprintln!(
        "  {} error(s) from {} file(s) written to {}",
        all_errors.iter().map(|(_, _, _, e)| e.len()).sum::<usize>(),
        all_errors.len(),
        log_path.display()
    );
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
        #[serde(default)]
        pub preserve_comments: bool,
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
        let output = super::parse_input(&input.sql, input.preserve_comments, false);
        let all_stmts: Vec<_> = output.statements.iter().map(|si| si.statement.clone()).collect();
        let fingerprints = ogsql_parser::compute_query_fingerprints(&all_stmts);
        let mut out = serde_json::json!({"statements": output.statements, "errors": output.errors});
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
        let tokens = match ogsql_parser::Tokenizer::new(&input.sql).preserve_comments(true).tokenize() {
            Ok(t) => t,
            Err(e) => {
                return Json(serde_json::json!({
                    "formatted": "",
                    "error": format!("{}", e),
                }));
            }
        };

        let mut config = ogsql_parser::FormatConfig::default();
        if let Some(indent) = input.indent {
            config.indent_width = indent;
        }
        if let Some(ref kw) = input.keyword_case {
            config.keyword_case = match kw.as_str() {
                "upper" => ogsql_parser::token_formatter::KeywordCase::Upper,
                "lower" => ogsql_parser::token_formatter::KeywordCase::Lower,
                _ => ogsql_parser::token_formatter::KeywordCase::Preserve,
            };
        }
        if let Some(ref comma) = input.comma_style {
            config.comma_style = match comma.as_str() {
                "leading" => ogsql_parser::token_formatter::CommaStyle::Leading,
                _ => ogsql_parser::token_formatter::CommaStyle::Trailing,
            };
        }
        if let Some(line_width) = input.line_width {
            config.line_width = line_width;
        }
        if input.uppercase == Some(true) {
            config.uppercase_keywords = true;
        }

        let formatted = ogsql_parser::token_formatter::TokenFormatter::with_config(&input.sql, tokens, config).format();
        Json(serde_json::json!({
            "formatted": formatted,
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
        let output = super::parse_input(&input.sql, false, false);
        let pkg_errors = ogsql_parser::validate_package_consistency(&output.statements);
        let has_pkg_issues = !pkg_errors.is_empty();
        let mut errors = output.errors;
        if has_pkg_issues {
            for pe in &pkg_errors {
                let msg = match &pe.detail {
                    Some(d) => format!("package {}: {} — {}", pe.package_name, pe.subprogram_name, d),
                    None => format!("package {}: {} — {:?}", pe.package_name, pe.subprogram_name, pe.kind),
                };
                errors.push(ogsql_parser::ParserError::Warning {
                    message: msg,
                    location: ogsql_parser::SourceLocation::default(),
                });
            }
        }
        let has_real_errors = errors.iter().any(|e| !super::is_warning(e));
        let mut result = serde_json::json!({
            "valid": !has_real_errors,
            "error_count": errors.iter().filter(|e| !super::is_warning(e)).count(),
            "warning_count": errors.iter().filter(|e| super::is_warning(e)).count(),
            "errors": errors,
        });
        if has_pkg_issues {
            result.as_object_mut().unwrap().insert(
                "package_consistency_errors".to_string(),
                serde_json::json!(pkg_errors),
            );
        }
        Json(result)
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
fn cmd_parse_xml(cli: &Cli, dir: Option<&str>, csv: bool, java_src: Option<&str>, stats: bool) {
    if dir.is_some() && cli.file.is_some() {
        die!("Error: --dir and -f are mutually exclusive");
    }

    #[cfg(feature = "java")]
    let java_roots: Vec<std::path::PathBuf> = match java_src {
        Some(path) => {
            let p = std::path::Path::new(path);
            if !p.is_dir() {
                die!("Error: '{}' is not a directory", path);
            }
            vec![p.to_path_buf()]
        }
        None => Vec::new(),
    };
    #[cfg(not(feature = "java"))]
    let java_roots: Vec<std::path::PathBuf> = Vec::new();

    if let Some(dir_path) = dir {
        cmd_parse_xml_dir(cli, dir_path, csv, &java_roots, stats);
    } else {
        cmd_parse_xml_single(cli, csv, &java_roots);
    }
}

#[cfg(feature = "ibatis")]
fn cmd_parse_xml_single(cli: &Cli, csv: bool, java_roots: &[std::path::PathBuf]) {
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

    #[cfg(feature = "java")]
    let result = if java_roots.is_empty() {
        ogsql_parser::ibatis::parse_mapper_bytes_with_path(&input, cli.file.as_deref())
    } else {
        ogsql_parser::ibatis::parse_mapper_bytes_with_java_src(
            &input, cli.file.as_deref(), java_roots.to_vec()
        )
    };
    #[cfg(not(feature = "java"))]
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
fn ibatis_error_kind(err: &ogsql_parser::ibatis::error::IbatisError) -> &'static str {
    use ogsql_parser::ibatis::error::IbatisError;
    match err {
        IbatisError::XmlError { .. } => "XmlError",
        IbatisError::UnknownFragment { .. } => "UnknownFragment",
        IbatisError::CircularInclude { .. } => "CircularInclude",
        IbatisError::MissingAttribute { .. } => "MissingAttribute",
        IbatisError::EmptyMapper => "EmptyMapper",
        IbatisError::SqlParseError(_) => "SqlParseError",
    }
}

#[cfg(feature = "ibatis")]
fn cmd_parse_xml_dir(cli: &Cli, dir_path: &str, csv: bool, java_roots: &[std::path::PathBuf], stats: bool) {
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

        #[cfg(feature = "java")]
        let result = if java_roots.is_empty() {
            ogsql_parser::ibatis::parse_mapper_bytes_with_path(
                &bytes,
                Some(&path.to_string_lossy()),
            )
        } else {
            ogsql_parser::ibatis::parse_mapper_bytes_with_java_src(
                &bytes,
                Some(&path.to_string_lossy()),
                java_roots.to_vec(),
            )
        };
        #[cfg(not(feature = "java"))]
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

    // Stats accumulators
    let mut files_with_xml_errors: HashSet<String> = HashSet::new();
    let mut files_with_sql_errors: HashSet<String> = HashSet::new();
    let mut files_with_sql_warnings: HashSet<String> = HashSet::new();
    let mut mapper_stmt_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut total_sql_stmts = 0usize;
    let mut xml_error_kinds: BTreeMap<&'static str, (usize, HashSet<String>)> = BTreeMap::new();
    let mut sql_error_kinds: BTreeMap<&'static str, (usize, HashSet<String>)> = BTreeMap::new();
    let mut sql_warning_kinds: BTreeMap<&'static str, (usize, HashSet<String>)> = BTreeMap::new();

    for (file_name, _rel_dir, result) in &all_results {
        if !result.errors.is_empty() {
            files_with_xml_errors.insert(file_name.clone());
        }
        for err in &result.errors {
            let kind = ibatis_error_kind(err);
            let entry = xml_error_kinds.entry(kind).or_insert((0, HashSet::new()));
            entry.0 += 1;
            entry.1.insert(file_name.clone());
        }
        for stmt in &result.statements {
            *mapper_stmt_counts.entry(format!("{:?}", stmt.kind)).or_insert(0) += 1;
            if let Some((infos, parse_errors)) = &stmt.parse_result {
                total_sql_stmts += infos.len();
                for perr in parse_errors {
                    let kind = parser_error_kind(perr);
                    let file_set = if is_warning(perr) { &mut sql_warning_kinds } else { &mut sql_error_kinds };
                    let entry = file_set.entry(kind).or_insert((0, HashSet::new()));
                    entry.0 += 1;
                    entry.1.insert(file_name.clone());
                    if is_warning(perr) { files_with_sql_warnings.insert(file_name.clone()); }
                    else { files_with_sql_errors.insert(file_name.clone()); }
                }
            }
        }
    }

    let total_mapper: usize = all_results.iter().map(|(_, _, r)| r.statements.len()).sum();

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
                if !stmt.parameters.is_empty() {
                    let typed: Vec<String> = stmt.parameters.iter().map(|p| {
                        match &p.jdbc_type {
                            Some(jt) => format!("{}:{:?}", p.name, jt),
                            None => p.name.clone(),
                        }
                    }).collect();
                    println!("  [params: {}]", typed.join(", "));
                }
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
        }
        println!(
            "Total: {} statement(s) from {} file(s)",
            total_mapper,
            all_results.len()
        );
    }

    if stats {
        let total_xml_errors: usize = xml_error_kinds.values().map(|(c, _)| c).sum();
        let total_sql_err_count: usize = sql_error_kinds.values().map(|(c, _)| c).sum();
        let total_sql_warn_count: usize = sql_warning_kinds.values().map(|(c, _)| c).sum();

        stats_bar("");
        stats_bar("Summary / parse-xml");
        stats_bar("");

        eprintln!("  {:<28} {}", "XML files processed:", all_results.len());
        eprintln!("  {:<28} {}", "Total mapper statements:", total_mapper);
        let max_kind = mapper_stmt_counts.keys().map(|k| k.len()).max().unwrap_or(10);
        for (kind, count) in &mapper_stmt_counts {
            let pct = if total_mapper > 0 { *count as f64 / total_mapper as f64 * 100.0 } else { 0.0 };
            eprintln!("    {:width$} {:>6} ({:>5.1}%)", kind, count, pct, width = max_kind + 4);
        }
        if total_sql_stmts > 0 {
            eprintln!("  {:<28} {}", "SQL statements (parsed):", total_sql_stmts);
        }
        eprintln!();

        if total_xml_errors > 0 {
            stats_bar("XML error breakdown");
            eprintln!("  Total: {} (in {} file(s))", total_xml_errors, files_with_xml_errors.len());
            for (kind, (cnt, files)) in &xml_error_kinds {
                eprintln!("    {:<20} {:>4} ({} file(s))", kind, cnt, files.len());
            }
            eprintln!();
        }
        if total_sql_err_count > 0 {
            stats_bar("SQL parse error breakdown");
            eprintln!("  Total: {} (in {} file(s))", total_sql_err_count, files_with_sql_errors.len());
            for (kind, (cnt, files)) in &sql_error_kinds {
                eprintln!("    {:<20} {:>4} ({} file(s))", kind, cnt, files.len());
            }
            eprintln!();
        }
        if total_sql_warn_count > 0 {
            stats_bar("SQL warning breakdown");
            eprintln!("  Total: {} (in {} file(s))", total_sql_warn_count, files_with_sql_warnings.len());
            for (kind, (cnt, files)) in &sql_warning_kinds {
                eprintln!("    {:<20} {:>4} ({} file(s))", kind, cnt, files.len());
            }
            eprintln!();
        }
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
        if !stmt.parameters.is_empty() {
            let typed: Vec<String> = stmt.parameters.iter().map(|p| {
                match &p.jdbc_type {
                    Some(jt) => format!("{}:{:?}", p.name, jt),
                    None => p.name.clone(),
                }
            }).collect();
            println!("  [params: {}]", typed.join(", "));
        }
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
fn cmd_parse_java(cli: &Cli, extra_sql_methods: &[String], extra_sql_var_patterns: &[String], dir: Option<&str>, csv: bool, stats: bool) {
    match dir {
        Some(dir_path) => cmd_parse_java_dir(cli, extra_sql_methods, extra_sql_var_patterns, dir_path, csv, stats),
        None => cmd_parse_java_single(cli, extra_sql_methods, extra_sql_var_patterns, csv),
    }
}

#[cfg(feature = "java")]
fn cmd_parse_java_single(cli: &Cli, extra_sql_methods: &[String], extra_sql_var_patterns: &[String], csv: bool) {
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
        extra_sql_var_patterns: extra_sql_var_patterns.to_vec(),
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
fn java_error_kind(err: &ogsql_parser::java::error::JavaError) -> &'static str {
    use ogsql_parser::java::error::JavaError;
    match err {
        JavaError::ParseError { .. } => "ParseError",
        JavaError::SqlParseError { .. } => "SqlParseError",
        JavaError::IoError(_) => "IoError",
        JavaError::EncodingError(_) => "EncodingError",
    }
}

#[cfg(feature = "java")]
fn cmd_parse_java_dir(cli: &Cli, extra_sql_methods: &[String], extra_sql_var_patterns: &[String], dir_path: &str, csv: bool, stats: bool) {
    use std::path::Path;

    let root = Path::new(dir_path);
    if !root.is_dir() {
        die!("Error: '{}' is not a directory", dir_path);
    }

    let config = ogsql_parser::java::JavaExtractConfig {
        extra_sql_methods: extra_sql_methods.to_vec(),
        extra_sql_var_patterns: extra_sql_var_patterns.to_vec(),
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

    // Stats accumulators
    let mut files_with_java_errors: HashSet<String> = HashSet::new();
    let mut files_with_sql_errors: HashSet<String> = HashSet::new();
    let mut files_with_sql_warnings: HashSet<String> = HashSet::new();
    let mut total_sql_stmts = 0usize;
    let mut java_error_kinds: BTreeMap<&'static str, (usize, HashSet<String>)> = BTreeMap::new();
    let mut sql_error_kinds: BTreeMap<&'static str, (usize, HashSet<String>)> = BTreeMap::new();
    let mut sql_warning_kinds: BTreeMap<&'static str, (usize, HashSet<String>)> = BTreeMap::new();
    let mut extraction_method_counts: BTreeMap<String, usize> = BTreeMap::new();

    for (file_name, _rel_dir, result) in &all_results {
        if !result.errors.is_empty() {
            files_with_java_errors.insert(file_name.clone());
        }
        for err in &result.errors {
            let kind = java_error_kind(err);
            let entry = java_error_kinds.entry(kind).or_insert((0, HashSet::new()));
            entry.0 += 1;
            entry.1.insert(file_name.clone());
        }
        for ext in &result.extractions {
            *extraction_method_counts.entry(format!("{:?}", ext.origin.method)).or_insert(0) += 1;
            if let Some(parse_result) = &ext.parse_result {
                total_sql_stmts += parse_result.statements.len();
                for perr in &parse_result.errors {
                    let kind = parser_error_kind(perr);
                    let file_set = if is_warning(perr) { &mut sql_warning_kinds } else { &mut sql_error_kinds };
                    let entry = file_set.entry(kind).or_insert((0, HashSet::new()));
                    entry.0 += 1;
                    entry.1.insert(file_name.clone());
                    if is_warning(perr) { files_with_sql_warnings.insert(file_name.clone()); }
                    else { files_with_sql_errors.insert(file_name.clone()); }
                }
            }
        }
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

    if stats {
        let total_extractions: usize = all_results.iter().map(|(_, _, r)| r.extractions.len()).sum();
        let total_java_errors: usize = java_error_kinds.values().map(|(c, _)| c).sum();
        let total_sql_err_count: usize = sql_error_kinds.values().map(|(c, _)| c).sum();
        let total_sql_warn_count: usize = sql_warning_kinds.values().map(|(c, _)| c).sum();

        stats_bar("");
        stats_bar("Summary / parse-java");
        stats_bar("");

        eprintln!("  {:<28} {}", "Java files processed:", all_results.len());
        eprintln!("  {:<28} {}", "Total extractions:", total_extractions);
        if !extraction_method_counts.is_empty() {
            let max_kind = extraction_method_counts.keys().map(|k| k.len()).max().unwrap_or(10);
            for (kind, count) in &extraction_method_counts {
                let pct = if total_extractions > 0 { *count as f64 / total_extractions as f64 * 100.0 } else { 0.0 };
                eprintln!("    {:width$} {:>6} ({:>5.1}%)", kind, count, pct, width = max_kind + 4);
            }
        }
        if total_sql_stmts > 0 {
            eprintln!("  {:<28} {}", "SQL statements (parsed):", total_sql_stmts);
        }
        eprintln!();

        if total_java_errors > 0 {
            stats_bar("Java error breakdown");
            eprintln!("  Total: {} (in {} file(s))", total_java_errors, files_with_java_errors.len());
            for (kind, (cnt, files)) in &java_error_kinds {
                eprintln!("    {:<20} {:>4} ({} file(s))", kind, cnt, files.len());
            }
            eprintln!();
        }
        if total_sql_err_count > 0 {
            stats_bar("SQL parse error breakdown");
            eprintln!("  Total: {} (in {} file(s))", total_sql_err_count, files_with_sql_errors.len());
            for (kind, (cnt, files)) in &sql_error_kinds {
                eprintln!("    {:<20} {:>4} ({} file(s))", kind, cnt, files.len());
            }
            eprintln!();
        }
        if total_sql_warn_count > 0 {
            stats_bar("SQL warning breakdown");
            eprintln!("  Total: {} (in {} file(s))", total_sql_warn_count, files_with_sql_warnings.len());
            for (kind, (cnt, files)) in &sql_warning_kinds {
                eprintln!("    {:<20} {:>4} ({} file(s))", kind, cnt, files.len());
            }
            eprintln!();
        }
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
    let prefixes = ["__XML_PARAM_", "__XML_RAW_", "__JAVA_VAR_", "__SQL_PARAM_", "__SQL_RAW_"];
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
    println!("file,directory,line,method,sql,variables,parameter_types,error,warning");
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
        let parameter_types: String = stmt
            .parameters
            .iter()
            .map(|p| match p.jdbc_type {
                Some(ref jt) => format!("{}:{:?}", p.name, jt),
                None => p.name.clone(),
            })
            .collect::<Vec<_>>()
            .join(";");
        println!(
            "{},{},{},{},{},{},{},{},{}",
            csv_escape(file_name),
            csv_escape(rel_dir),
            stmt.line,
            csv_escape(&stmt.id),
            csv_escape(&sql),
            csv_escape(&variables),
            csv_escape(&parameter_types),
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

// ═══════════════════════════════════════════════════════════════
//  Stats helpers for --dir commands
// ═══════════════════════════════════════════════════════════════

use std::collections::{BTreeMap, HashSet};

fn stmt_category(stmt: &Statement) -> &'static str {
    use Statement::*;
    match stmt {
        Select(_) => "SELECT",
        Insert(_) | InsertAll(_) | InsertFirst(_) => "INSERT",
        Update(_) => "UPDATE",
        Delete(_) => "DELETE",
        Merge(_) => "MERGE",
        CreateTable(_) | CreateTableAs(_) => "CREATE TABLE",
        AlterTable(_) | AlterTablespace(_) => "ALTER TABLE",
        Drop(_) => "DROP",
        Truncate(_) => "TRUNCATE",
        CreateIndex(_) | CreateGlobalIndex(_) => "CREATE INDEX",
        AlterIndex(_) => "ALTER INDEX",
        CreateSchema(_) => "CREATE SCHEMA",
        AlterSchema(_) => "ALTER SCHEMA",
        CreateDatabase(_) | CreateDatabaseLink(_) => "CREATE DATABASE",
        AlterDatabase(_) => "ALTER DATABASE",
        CreateTablespace(_) => "CREATE TABLESPACE",
        CreateFunction(_) => "CREATE FUNCTION",
        AlterFunction(_) => "ALTER FUNCTION",
        CreateProcedure(_) => "CREATE PROCEDURE",
        AlterProcedure(_) => "ALTER PROCEDURE",
        CreatePackage(_) | CreatePackageBody(_) => "CREATE PACKAGE",
        AlterPackage(_) => "ALTER PACKAGE",
        CreateView(_) | CreateMaterializedView(_) => "CREATE VIEW",
        AlterView(_) | AlterMaterializedView(_) => "ALTER VIEW",
        CreateSequence(_) => "CREATE SEQUENCE",
        AlterSequence(_) => "ALTER SEQUENCE",
        CreateTrigger(_) => "CREATE TRIGGER",
        AlterTrigger(_) => "ALTER TRIGGER",
        CreateDomain(_) | AlterDomain(_) => "DOMAIN",
        Do(_) | AnonyBlock(_) => "PL/BLOCK",
        Call(_) => "CALL",
        Transaction(_) => "TRANSACTION",
        Copy(_) => "COPY",
        Explain(_) => "EXPLAIN",
        Prepare(_) | Execute(_) | Deallocate(_) => "PREPARE/EXECUTE",
        VariableSet(_) | VariableShow(_) | VariableReset(_) => "SET/SHOW",
        Grant(_) | Revoke(_) => "GRANT/REVOKE",
        GrantRole(_) | RevokeRole(_) => "GRANT/REVOKE ROLE",
        Comment(_) => "COMMENT",
        Lock(_) => "LOCK",
        Analyze(_) => "ANALYZE",
        Vacuum(_) => "VACUUM",
        Values(_) => "VALUES",
        ExecuteDirect(_) => "EXECUTE DIRECT",
        CreateSynonym(_) | AlterSynonym(_) => "SYNONYM",
        CreateExtension(_) | AlterExtension(_) => "EXTENSION",
        CreateRole(_) | AlterRole(_) => "ROLE",
        CreateUser(_) | AlterUser(_) => "USER",
        CreateGroup(_) | AlterGroup(_) => "GROUP",
        CreateType(_) => "CREATE TYPE",
        CreateLanguage(_) => "CREATE LANGUAGE",
        CreateForeignTable(_) => "FOREIGN TABLE",
        CreateForeignServer(_) | CreateFdw(_) => "FOREIGN SERVER",
        CreatePublication(_) | AlterPublication(_) => "PUBLICATION",
        CreateSubscription(_) | AlterSubscription(_) => "SUBSCRIPTION",
        CreateNode(_) | AlterNode(_) | CreateNodeGroup(_) | AlterNodeGroup(_) => "NODE",
        CreateResourcePool(_) | AlterResourcePool(_) => "RESOURCE POOL",
        CreateWorkloadGroup(_) | AlterWorkloadGroup(_) => "WORKLOAD GROUP",
        CreateAuditPolicy(_) | AlterAuditPolicy(_) => "AUDIT POLICY",
        CreateMaskingPolicy(_) | AlterMaskingPolicy(_) => "MASKING POLICY",
        CreateRlsPolicy(_) | AlterRlsPolicy(_) => "RLS POLICY",
        CreateDataSource(_) | AlterDataSource(_) => "DATA SOURCE",
        CreateEvent(_) | AlterEvent(_) => "EVENT",
        CreateDirectory(_) => "DIRECTORY",
        AlterDirectory(_) => "ALTER DIRECTORY",
        CreateAggregate(_) => "CREATE AGGREGATE",
        CreateOperator(_) => "CREATE OPERATOR",
        RefreshMaterializedView(_) => "REFRESH MVIEW",
        Reindex(_) => "REINDEX",
        Cluster(_) => "CLUSTER",
        Listen(_) | Notify(_) | Unlisten(_) => "LISTEN/NOTIFY",
        DeclareCursor(_) | ClosePortal(_) | Fetch(_) | Move(_) => "CURSOR",
        Purge(_) => "PURGE",
        TimeCapsule(_) => "TIME CAPSULE",
        Snapshot(_) => "SNAPSHOT",
        Shrink(_) => "SHRINK",
        Compile(_) => "COMPILE",
        SecLabel(_) => "SECURITY LABEL",
        Rule(_) | DropRule(_) => "RULE",
        CreateCast(_) | CreateConversion(_) => "CAST/CONVERSION",
        CreateOpClass(_) | CreateOpFamily(_) | AlterOpFamily(_) => "OPERATOR CLASS",
        CreateTextSearchConfig(_) | CreateTextSearchDict(_)
            | AlterTextSearchConfig(_) | AlterTextSearchDict(_)
            | AlterTextSearchConfigFull(_) | AlterTextSearchDictFull(_) => "TEXT SEARCH",
        CreateUserMapping(_) | AlterUserMapping(_) | DropUserMapping(_) => "USER MAPPING",
        AlterDefaultPrivileges(_) => "ALTER DEFAULT PRIVILEGES",
        AlterCompositeType(_) => "ALTER TYPE",
        AlterCoordinator(_) => "ALTER COORDINATOR",
        AlterAppWorkloadGroupMapping(_) => "ALTER APP WORKLOAD GROUP",
        AlterDatabaseLink(_) => "ALTER DATABASE LINK",
        AlterLargeObject(_) => "ALTER LARGE OBJECT",
        AlterSession(_) => "ALTER SESSION",
        AlterSystemKillSession(_) => "ALTER SYSTEM KILL",
        AlterGlobalConfig(_) => "ALTER GLOBAL CONFIG",
        DropWeakPasswordDictionary | CreateWeakPasswordDictionary
            | CreateWeakPasswordDictionaryWithValues(_) => "WEAK PASSWORD DICT",
        CreatePolicyLabel(_) | AlterPolicyLabel(_) | DropPolicyLabel(_) => "POLICY LABEL",
        CreateContQuery(_) => "CONTINUOUS QUERY",
        CreateStream(_) => "STREAM",
        CreateKey(_) => "KEY",
        SetSessionAuthorization(_) => "SET SESSION AUTHORIZATION",
        CreateAppWorkloadGroupMapping(_) | DropAppWorkloadGroupMapping(_) => "APP WORKLOAD GROUP",
        ExpdpDatabase(_) | ExpdpTable(_) | ImpdpDatabase(_) | ImpdpTable(_) => "EXPDP/IMPDP",
        ReassignOwned(_) => "REASSIGN OWNED",
        LockBuckets(_) | MarkBuckets(_) => "LOCK BUCKETS",
        Shutdown(_) | Barrier(_) | Abort | Checkpoint | Empty => "UTILITY",
        Replace(_) => "REPLACE",
        GetDiag(_) => "GET DIAGNOSTICS",
        ShowEvent(_) => "SHOW EVENT",
        RemovePackage(_) => "REMOVE PACKAGE",
        PredictBy(_) => "PREDICT BY",
        CleanConn(_) => "CLEAN CONNECTION",
        Verify(_) => "VERIFY",
        _ => "OTHER",
    }
}

fn parser_error_kind(err: &ParserError) -> &'static str {
    match err {
        ParserError::UnexpectedToken { .. } => "UnexpectedToken",
        ParserError::UnexpectedEof { .. } => "UnexpectedEof",
        ParserError::Warning { .. } => "Warning",
        ParserError::ReservedKeywordAsIdentifier { .. } => "ReservedKeyword",
        ParserError::TokenizerError(_) => "TokenizerError",
    }
}

fn stats_bar(label: &str) {
    if label.is_empty() {
        eprintln!("  {}", "─".repeat(55));
    } else {
        let side = (55usize.saturating_sub(label.len() + 2)) / 2;
        eprintln!("  {}{}{}", "─".repeat(side), format!(" {} ", label), "─".repeat(side));
    }
}

fn print_parse_stats(
    files_processed: usize,
    files_with_errors: &HashSet<String>,
    files_with_warnings: &HashSet<String>,
    total_stmts: usize,
    stmt_counts: &BTreeMap<&'static str, usize>,
    error_counts: &BTreeMap<&'static str, (usize, HashSet<String>)>,
    warning_counts: &BTreeMap<&'static str, (usize, HashSet<String>)>,
    extra_title: &str,
) {
    let total_errors: usize = error_counts.values().map(|(c, _)| c).sum();
    let total_warnings: usize = warning_counts.values().map(|(c, _)| c).sum();

    stats_bar("");
    let title = if extra_title.is_empty() {
        "Summary".to_string()
    } else {
        format!("Summary / {}", extra_title)
    };
    stats_bar(&title);
    stats_bar("");

    eprintln!("  {:<24} {}", "Files processed:", files_processed);
    if !files_with_errors.is_empty() {
        eprintln!("  {:<24} {}", "Files with errors:", files_with_errors.len());
    }
    if !files_with_warnings.is_empty() {
        eprintln!("  {:<24} {}", "Files with warnings:", files_with_warnings.len());
    }
    eprintln!("  {:<24} {}", "Total statements:", total_stmts);
    eprintln!();

    if !stmt_counts.is_empty() {
        stats_bar("Statement breakdown");
        let max_name = stmt_counts.keys().map(|k| k.len()).max().unwrap_or(10);
        for (cat, count) in stmt_counts {
            let pct = if total_stmts > 0 { *count as f64 / total_stmts as f64 * 100.0 } else { 0.0 };
            eprintln!("  {:width$} {:>6} ({:>5.1}%)", cat, count, pct, width = max_name);
        }
        eprintln!();
    }

    if total_errors > 0 || total_warnings > 0 {
        stats_bar("Error / Warning breakdown");
        eprint!("  Total: ");
        if total_errors > 0 {
            eprint!("{} error(s) (in {} file(s))", total_errors, files_with_errors.len());
        }
        if total_errors > 0 && total_warnings > 0 {
            eprint!(", ");
        }
        if total_warnings > 0 {
            eprint!("{} warning(s) (in {} file(s))", total_warnings, files_with_warnings.len());
        }
        eprintln!();

        if !error_counts.is_empty() {
            eprintln!();
            eprintln!("  Errors:");
            for (kind, (count, files)) in error_counts {
                eprintln!("    {:<20} {:>4} ({} file(s))", kind, count, files.len());
            }
        }
        if !warning_counts.is_empty() {
            eprintln!();
            eprintln!("  Warnings:");
            for (kind, (count, files)) in warning_counts {
                eprintln!("    {:<20} {:>4} ({} file(s))", kind, count, files.len());
            }
        }
        eprintln!();
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Format {
            indent,
            ref keyword_case,
            ref comma,
            line_width,
            uppercase,
            no_select_newline,
            no_logical_newline,
            no_semicolon_newline,
        } => cmd_format(
            &cli,
            indent,
            keyword_case.clone(),
            comma.clone(),
            line_width,
            uppercase,
            no_select_newline,
            no_logical_newline,
            no_semicolon_newline,
        ),
        Commands::Parse { ref dir, ref ext, csv, ref output_dir, stats } => {
            if !dir.is_empty() && cli.file.is_some() {
                die!("Error: --dir and -f are mutually exclusive");
            }
            if !dir.is_empty() {
                cmd_parse_dir(&cli, dir, ext, csv, output_dir.as_deref(), stats);
            } else {
                cmd_parse(&cli, csv);
            }
        }
        Commands::JsonToSql => cmd_json2sql(&cli),
        Commands::Tokenize => cmd_tokenize(&cli),
        Commands::Validate { ref dir, ref ext, stats } => {
            if !dir.is_empty() && cli.file.is_some() {
                die!("Error: --dir and -f are mutually exclusive");
            }
            if !dir.is_empty() {
                cmd_validate_dir(&cli, dir, ext, stats);
            } else {
                cmd_validate(&cli);
            }
        }
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
        #[cfg(not(feature = "java"))]
        Commands::ParseXml { ref dir, csv, stats } => cmd_parse_xml(&cli, dir.as_deref(), csv, None, stats),
        #[cfg(feature = "ibatis")]
        #[cfg(feature = "java")]
        Commands::ParseXml { ref dir, csv, ref java_src, stats } => {
            cmd_parse_xml(&cli, dir.as_deref(), csv, java_src.as_deref(), stats)
        }
        #[cfg(feature = "java")]
        Commands::ParseJava {
            ref extra_sql_methods,
            ref extra_sql_var_patterns,
            ref dir,
            csv,
            stats,
        } => cmd_parse_java(&cli, extra_sql_methods, extra_sql_var_patterns, dir.as_deref(), csv, stats),
    }
}

fn pl_data_type_to_string(dt: &ogsql_parser::ast::plpgsql::PlDataType) -> Option<String> {
    match dt {
        ogsql_parser::ast::plpgsql::PlDataType::TypeName(s) => Some(s.clone()),
        ogsql_parser::ast::plpgsql::PlDataType::PercentType { column, .. } => Some(column.clone()),
        ogsql_parser::ast::plpgsql::PlDataType::PercentRowType(s) => Some(s.clone()),
        ogsql_parser::ast::plpgsql::PlDataType::Record => Some("RECORD".into()),
        ogsql_parser::ast::plpgsql::PlDataType::Cursor => None,
        ogsql_parser::ast::plpgsql::PlDataType::RefCursor => None,
    }
}

fn collect_block_vars(
    block: &ogsql_parser::ast::plpgsql::PlBlock,
    params: &[ogsql_parser::ast::RoutineParam],
) -> std::collections::HashMap<String, Option<String>> {
    use ogsql_parser::ast::plpgsql::PlDeclaration;
    let mut vars = std::collections::HashMap::new();
    for p in params {
        vars.insert(p.name.to_ascii_lowercase(), Some(p.data_type.clone()));
    }
    for decl in &block.declarations {
        match decl {
            PlDeclaration::Variable(v) => {
                let t = pl_data_type_to_string(&v.data_type);
                vars.insert(v.name.to_ascii_lowercase(), t);
            }
            PlDeclaration::Record(r) => {
                vars.insert(r.name.to_ascii_lowercase(), Some("RECORD".into()));
            }
            PlDeclaration::Cursor(c) => {
                vars.insert(c.name.to_ascii_lowercase(), None);
            }
            PlDeclaration::Type(t) => {
                let name = match t {
                    ogsql_parser::ast::plpgsql::PlTypeDecl::Record { name, .. } => name,
                    ogsql_parser::ast::plpgsql::PlTypeDecl::TableOf { name, .. } => name,
                    ogsql_parser::ast::plpgsql::PlTypeDecl::VarrayOf { name, .. } => name,
                    ogsql_parser::ast::plpgsql::PlTypeDecl::RefCursor { name } => name,
                };
                vars.insert(name.to_ascii_lowercase(), None);
            }
            PlDeclaration::NestedProcedure(p) => {
                vars.insert(p.name.join(".").to_ascii_lowercase(), None);
            }
            PlDeclaration::NestedFunction(f) => {
                vars.insert(f.name.join(".").to_ascii_lowercase(), None);
            }
            PlDeclaration::Pragma { name, .. } => {
                vars.insert(name.to_ascii_lowercase(), None);
            }
        }
    }
    vars
}

fn sanitize_type_for_placeholder(t: &str) -> String {
    let mut s: String = t
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    // Collapse consecutive underscores
    while s.contains("__") {
        s = s.replace("__", "_");
    }
    // Strip trailing underscores to avoid creating "__" when combined with
    // the "_" separator in format!("{}{}_{}__", prefix, sanitized_type, ident)
    while s.ends_with('_') {
        s.pop();
    }
    s
}

fn replace_pl_vars_in_sql_with_prefix(
    sql: &str,
    vars: &std::collections::HashMap<String, Option<String>>,
    prefix: &str,
) -> String {
    let mut result = String::with_capacity(sql.len());
    let bytes = sql.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let c = bytes[i];

        if c == b'\'' {
            result.push(c as char);
            i += 1;
            while i < len {
                if bytes[i] == b'\'' {
                    result.push(bytes[i] as char);
                    i += 1;
                    if i < len && bytes[i] == b'\'' {
                        result.push(bytes[i] as char);
                        i += 1;
                        continue;
                    }
                    break;
                }
                result.push(bytes[i] as char);
                i += 1;
            }
            continue;
        }

        if c == b'"' {
            result.push(c as char);
            i += 1;
            while i < len {
                if bytes[i] == b'"' {
                    result.push(bytes[i] as char);
                    i += 1;
                    if i < len && bytes[i] == b'"' {
                        result.push(bytes[i] as char);
                        i += 1;
                        continue;
                    }
                    break;
                }
                result.push(bytes[i] as char);
                i += 1;
            }
            continue;
        }

        let is_ident_start = c == b'_' || c.is_ascii_alphabetic();
        if is_ident_start {
            let start = i;
            i += 1;
            while i < len && (bytes[i] == b'_' || bytes[i].is_ascii_alphanumeric()) {
                i += 1;
            }
            let ident = &sql[start..i];
            if let Some(maybe_type) = vars.get(&ident.to_ascii_lowercase()) {
                match maybe_type {
                    Some(t) => result.push_str(&format!("{}{}_{}__", prefix, sanitize_type_for_placeholder(t), ident)),
                    None => result.push_str(&format!("{}{}__", prefix, ident)),
                }
            } else {
                result.push_str(ident);
            }
            continue;
        }

        result.push(c as char);
        i += 1;
    }

    result
}

/// Replace PL/pgSQL variable references with `__SQL_PARAM_` (bind-variable semantics).
fn replace_pl_vars_in_sql(sql: &str, vars: &std::collections::HashMap<String, Option<String>>) -> String {
    replace_pl_vars_in_sql_with_prefix(sql, vars, "__SQL_PARAM_")
}

/// Replace PL/pgSQL variable references with `__SQL_RAW_` (string-interpolation semantics).
fn replace_pl_vars_in_sql_raw(sql: &str, vars: &std::collections::HashMap<String, Option<String>>) -> String {
    replace_pl_vars_in_sql_with_prefix(sql, vars, "__SQL_RAW_")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_vars(vars: &[(&str, Option<&str>)]) -> std::collections::HashMap<String, Option<String>> {
        vars.iter()
            .map(|(k, v)| (k.to_ascii_lowercase(), v.map(|s| s.to_string())))
            .collect()
    }

    #[test]
    fn test_single_var_in_where() {
        let vars = make_vars(&[("p_account_id", Some("INTEGER"))]);
        assert_eq!(
            replace_pl_vars_in_sql(
                "UPDATE accounts SET frozen_flag = 'Y' WHERE account_id = p_account_id",
                &vars,
            ),
            "UPDATE accounts SET frozen_flag = 'Y' WHERE account_id = __SQL_PARAM_INTEGER_p_account_id__",
        );
    }

    #[test]
    fn test_multiple_vars() {
        let vars = make_vars(&[
            ("p_name", Some("VARCHAR")),
            ("p_age", Some("INTEGER")),
        ]);
        assert_eq!(
            replace_pl_vars_in_sql(
                "SELECT * FROM users WHERE name = p_name AND age = p_age",
                &vars,
            ),
            "SELECT * FROM users WHERE name = __SQL_PARAM_VARCHAR_p_name__ AND age = __SQL_PARAM_INTEGER_p_age__",
        );
    }

    #[test]
    fn test_var_without_type() {
        let vars = make_vars(&[("v_result", None)]);
        assert_eq!(
            replace_pl_vars_in_sql("SELECT * FROM t WHERE id = v_result", &vars),
            "SELECT * FROM t WHERE id = __SQL_PARAM_v_result__",
        );
    }

    #[test]
    fn test_mixed_typed_and_untyped() {
        let vars = make_vars(&[
            ("p_id", Some("INT")),
            ("v_count", None),
        ]);
        assert_eq!(
            replace_pl_vars_in_sql(
                "SELECT * FROM t WHERE id = p_id AND cnt > v_count",
                &vars,
            ),
            "SELECT * FROM t WHERE id = __SQL_PARAM_INT_p_id__ AND cnt > __SQL_PARAM_v_count__",
        );
    }

    #[test]
    fn test_case_insensitive_var_match() {
        let vars = make_vars(&[("P_ACCOUNT_ID", Some("INTEGER"))]);
        assert_eq!(
            replace_pl_vars_in_sql(
                "WHERE account_id = p_account_id",
                &vars,
            ),
            "WHERE account_id = __SQL_PARAM_INTEGER_p_account_id__",
        );
    }

    #[test]
    fn test_uppercase_var_in_sql() {
        let vars = make_vars(&[("p_id", Some("INT"))]);
        assert_eq!(
            replace_pl_vars_in_sql("WHERE id = P_ID", &vars),
            "WHERE id = __SQL_PARAM_INT_P_ID__",
        );
    }

    #[test]
    fn test_var_in_single_quote_string_not_replaced() {
        let vars = make_vars(&[("p_name", Some("VARCHAR"))]);
        assert_eq!(
            replace_pl_vars_in_sql(
                "INSERT INTO logs (msg) VALUES ('p_name was here')",
                &vars,
            ),
            "INSERT INTO logs (msg) VALUES ('p_name was here')",
        );
    }

    #[test]
    fn test_var_adjacent_to_string_literal() {
        let vars = make_vars(&[("p_status", Some("VARCHAR"))]);
        assert_eq!(
            replace_pl_vars_in_sql(
                "SELECT * FROM t WHERE status = p_status AND msg = 'active'",
                &vars,
            ),
            "SELECT * FROM t WHERE status = __SQL_PARAM_VARCHAR_p_status__ AND msg = 'active'",
        );
    }

    #[test]
    fn test_escaped_quote_in_string() {
        let vars = make_vars(&[("p_val", Some("TEXT"))]);
        assert_eq!(
            replace_pl_vars_in_sql(
                "SELECT * FROM t WHERE name = 'it''s p_val' AND val = p_val",
                &vars,
            ),
            "SELECT * FROM t WHERE name = 'it''s p_val' AND val = __SQL_PARAM_TEXT_p_val__",
        );
    }

    #[test]
    fn test_var_in_double_quote_not_replaced() {
        let vars = make_vars(&[("p_id", Some("INT"))]);
        assert_eq!(
            replace_pl_vars_in_sql(
                "SELECT \"p_id\" FROM t WHERE id = p_id",
                &vars,
            ),
            "SELECT \"p_id\" FROM t WHERE id = __SQL_PARAM_INT_p_id__",
        );
    }

    #[test]
    fn test_var_as_substring_of_column_not_replaced() {
        let vars = make_vars(&[("p_id", Some("INT"))]);
        assert_eq!(
            replace_pl_vars_in_sql(
                "SELECT * FROM t WHERE p_id_extra = 1 AND id = p_id",
                &vars,
            ),
            "SELECT * FROM t WHERE p_id_extra = 1 AND id = __SQL_PARAM_INT_p_id__",
        );
    }

    #[test]
    fn test_var_prefix_of_another_not_replaced() {
        let vars = make_vars(&[("p", Some("INT"))]);
        assert_eq!(
            replace_pl_vars_in_sql("SELECT * FROM t WHERE id = p_name", &vars),
            "SELECT * FROM t WHERE id = p_name",
        );
    }

    #[test]
    fn test_column_name_same_pattern_preserved() {
        let vars = make_vars(&[("p_id", Some("INT"))]);
        assert_eq!(
            replace_pl_vars_in_sql(
                "SELECT account_id FROM accounts WHERE account_id = p_id",
                &vars,
            ),
            "SELECT account_id FROM accounts WHERE account_id = __SQL_PARAM_INT_p_id__",
        );
    }

    #[test]
    fn test_no_vars_no_change() {
        let vars = make_vars(&[]);
        assert_eq!(
            replace_pl_vars_in_sql("SELECT * FROM t WHERE id = 1", &vars),
            "SELECT * FROM t WHERE id = 1",
        );
    }

    #[test]
    fn test_empty_sql() {
        let vars = make_vars(&[("p_id", Some("INT"))]);
        assert_eq!(replace_pl_vars_in_sql("", &vars), "");
    }

    #[test]
    fn test_unrelated_vars_not_touched() {
        let vars = make_vars(&[("v_x", Some("INT"))]);
        assert_eq!(
            replace_pl_vars_in_sql("SELECT * FROM t WHERE id = v_y", &vars),
            "SELECT * FROM t WHERE id = v_y",
        );
    }

    #[test]
    fn test_underscore_var_name() {
        let vars = make_vars(&[("v_total_count", Some("BIGINT"))]);
        assert_eq!(
            replace_pl_vars_in_sql(
                "SELECT * FROM t WHERE cnt = v_total_count",
                &vars,
            ),
            "SELECT * FROM t WHERE cnt = __SQL_PARAM_BIGINT_v_total_count__",
        );
    }

    #[test]
    fn test_var_in_set_clause() {
        let vars = make_vars(&[
            ("p_flag", Some("CHAR")),
            ("p_id", Some("INTEGER")),
        ]);
        assert_eq!(
            replace_pl_vars_in_sql(
                "UPDATE t SET flag = p_flag WHERE id = p_id",
                &vars,
            ),
            "UPDATE t SET flag = __SQL_PARAM_CHAR_p_flag__ WHERE id = __SQL_PARAM_INTEGER_p_id__",
        );
    }

    #[test]
    fn test_var_as_function_arg() {
        let vars = make_vars(&[("p_limit", Some("INTEGER"))]);
        assert_eq!(
            replace_pl_vars_in_sql(
                "SELECT * FROM t LIMIT p_limit",
                &vars,
            ),
            "SELECT * FROM t LIMIT __SQL_PARAM_INTEGER_p_limit__",
        );
    }

    #[test]
    fn test_same_var_multiple_times() {
        let vars = make_vars(&[("p_id", Some("INT"))]);
        assert_eq!(
            replace_pl_vars_in_sql(
                "SELECT * FROM t1 WHERE id = p_id UNION SELECT * FROM t2 WHERE ref_id = p_id",
                &vars,
            ),
            "SELECT * FROM t1 WHERE id = __SQL_PARAM_INT_p_id__ UNION SELECT * FROM t2 WHERE ref_id = __SQL_PARAM_INT_p_id__",
        );
    }

    #[test]
    fn test_extract_variables_sql_param() {
        let sql = "WHERE id = __SQL_PARAM_INT_p_id__ AND name = __SQL_PARAM_VARCHAR_p_name__";
        let vars = extract_variables(sql);
        assert!(vars.contains("__SQL_PARAM_INT_p_id__"), "got: {}", vars);
        assert!(vars.contains("__SQL_PARAM_VARCHAR_p_name__"), "got: {}", vars);
    }

    #[test]
    fn test_extract_variables_mixed_prefixes() {
        let sql = "WHERE id = __SQL_PARAM_INT_p_id__ AND x = __XML_PARAM_id__ AND y = __JAVA_VAR_String_name__";
        let vars = extract_variables(sql);
        assert!(vars.contains("__SQL_PARAM_INT_p_id__"), "got: {}", vars);
        assert!(vars.contains("__XML_PARAM_id__"), "got: {}", vars);
        assert!(vars.contains("__JAVA_VAR_String_name__"), "got: {}", vars);
    }

    #[test]
    fn test_replace_pl_vars_raw() {
        let vars = make_vars(&[("v_sql", Some("VARCHAR2"))]);
        assert_eq!(
            replace_pl_vars_in_sql_raw("SELECT * FROM t WHERE id = v_sql", &vars),
            "SELECT * FROM t WHERE id = __SQL_RAW_VARCHAR2_v_sql__",
        );
    }

    #[test]
    fn test_replace_pl_vars_raw_no_type() {
        let vars = make_vars(&[("v_sql", None)]);
        assert_eq!(
            replace_pl_vars_in_sql_raw("v_sql", &vars),
            "__SQL_RAW_v_sql__",
        );
    }

    #[test]
    fn test_replace_pl_vars_raw_concat() {
        let vars = make_vars(&[("v_table", Some("VARCHAR2"))]);
        assert_eq!(
            replace_pl_vars_in_sql_raw("'SELECT * FROM ' || v_table", &vars),
            "'SELECT * FROM ' || __SQL_RAW_VARCHAR2_v_table__",
        );
    }

    #[test]
    fn test_extract_variables_sql_raw() {
        let sql = "EXECUTE IMMEDIATE __SQL_RAW_VARCHAR2_v_sql__";
        let vars = extract_variables(sql);
        assert!(vars.contains("__SQL_RAW_VARCHAR2_v_sql__"), "got: {}", vars);
    }
}
