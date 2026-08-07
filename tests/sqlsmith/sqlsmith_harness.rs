//! SQLsmith test harness for ogsql-parser.
//!
//! 三种模式 / Three modes:
//! - `mine <corpus>`: 扫描语料，挖掘新失败入 `regress/`
//! - `guard`: 守护 `regress/` 中案例，对比 expected vs actual，CI 友好
//! - `run <corpus>`: 一次性扫描出 CSV 报告，不动 `regress/`
//!
//! 详见 `tests/sqlsmith/README.md` 与 `regress/README.md`。

use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use ogsql_parser::{Parser, ParserError, SqlFormatter, Tokenizer};

// ============================================================
// 公共类型 / Common types
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum FailureKind {
    TokenizeErr,
    ParseErr,
    RoundtripReparseErr,
    RoundtripAstDiff,
}

impl FailureKind {
    fn as_str(&self) -> &'static str {
        match self {
            FailureKind::TokenizeErr => "TOKENIZE_ERR",
            FailureKind::ParseErr => "PARSE_ERR",
            FailureKind::RoundtripReparseErr => "ROUNDTRIP_REPARSE_ERR",
            FailureKind::RoundtripAstDiff => "ROUNDTRIP_AST_DIFF",
        }
    }
}

impl std::fmt::Display for FailureKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
enum Outcome {
    Ok,
    Fail { kind: FailureKind, error_message: String, error_class: String },
}

impl Outcome {
    fn is_ok(&self) -> bool {
        matches!(self, Outcome::Ok)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CaseMeta {
    id: String,
    slug: String,
    added_at: String,
    added_in_commit: Option<String>,
    kind: String,
    signature: String,
    error_message: String,
    error_class: String,
    expected_outcome: String,
    fixed_in_commit: Option<String>,
    known_acceptable: bool,
    seed: Option<String>,
    notes: Option<String>,
}

// ============================================================
// 核心判定 / Core oracle
// ============================================================

/// 跑一层 oracle（tokenize + parse）+ 可选二层（round-trip）
fn try_parse(sql: &str, do_roundtrip: bool) -> Outcome {
    // Layer 1: tokenize
    let tokens = match Tokenizer::new(sql).tokenize() {
        Ok(t) => t,
        Err(e) => {
            return Outcome::Fail {
                kind: FailureKind::TokenizeErr,
                error_message: e.to_string(),
                error_class: classify_error(&e.to_string(), "tokenize"),
            }
        }
    };

    // Layer 1: parse
    let mut parser = Parser::with_source(tokens, sql.to_string());
    let stmts = parser.parse();
    let has_empty = stmts.iter().any(|s| matches!(s, ogsql_parser::Statement::Empty));
    // 仅硬错误算失败：Warning / ReservedKeywordAsIdentifier 是软提示（is_warning），
    // 例如函数 arity 警告、保留字 AS-alias——不应计为 parse 失败。
    let hard_errors: Vec<&ParserError> = parser.errors().iter().filter(|e| !ogsql_parser::is_warning(e)).collect();
    if !hard_errors.is_empty() || has_empty {
        let err_msg =
            hard_errors.first().map(|e| e.to_string()).unwrap_or_else(|| "Statement::Empty (recovery)".to_string());
        return Outcome::Fail {
            kind: FailureKind::ParseErr,
            error_class: classify_error(&err_msg, "parse"),
            error_message: err_msg,
        };
    }

    if !do_roundtrip {
        return Outcome::Ok;
    }

    // Layer 2: round-trip via SqlFormatter
    let formatter = SqlFormatter::new();
    let formatted: Vec<String> = stmts.iter().map(|s| formatter.format_statement(s)).collect();
    let sql_prime = formatted.join(";\n");

    // Layer 2a: reparse formatted SQL
    let reparsed_tokens = match Tokenizer::new(&sql_prime).tokenize() {
        Ok(t) => t,
        Err(e) => {
            return Outcome::Fail {
                kind: FailureKind::RoundtripReparseErr,
                error_message: format!("reparse tokenize failed: {e}"),
                error_class: "roundtrip-reparse-tokenize".to_string(),
            }
        }
    };
    let mut parser2 = Parser::with_source(reparsed_tokens, sql_prime.clone());
    let stmts2 = parser2.parse();
    let has_empty2 = stmts2.iter().any(|s| matches!(s, ogsql_parser::Statement::Empty));
    let hard_errors2: Vec<&ParserError> = parser2.errors().iter().filter(|e| !ogsql_parser::is_warning(e)).collect();
    if !hard_errors2.is_empty() || has_empty2 {
        let err_msg =
            hard_errors2.first().map(|e| e.to_string()).unwrap_or_else(|| "reparse Statement::Empty".to_string());
        return Outcome::Fail {
            kind: FailureKind::RoundtripReparseErr,
            error_message: format!("reparse failed: {err_msg}"),
            error_class: "roundtrip-reparse".to_string(),
        };
    }

    // Layer 2b: AST diff (via idempotency: format(parse(format(parse(sql)))) == format(parse(sql)))
    let formatted2: Vec<String> = stmts2.iter().map(|s| formatter.format_statement(s)).collect();
    let sql_prime2 = formatted2.join(";\n");
    if sql_prime != sql_prime2 {
        return Outcome::Fail {
            kind: FailureKind::RoundtripAstDiff,
            error_message: format!(
                "formatter not idempotent after reparse (len {} -> {})",
                sql_prime.len(),
                sql_prime2.len()
            ),
            error_class: "roundtrip-ast-diff".to_string(),
        };
    }

    Outcome::Ok
}

/// 全词匹配（大小写不敏感），避免 "expected" 误命中 "cte" 之类的子串陷阱。
fn contains_word(s: &str, word: &str) -> bool {
    s.split(|c: char| !c.is_alphanumeric()).any(|w| w.eq_ignore_ascii_case(word))
}

fn classify_error(msg: &str, layer: &str) -> String {
    if contains_word(msg, "array") || contains_word(msg, "subscript") {
        return "array".to_string();
    }
    if contains_word(msg, "cast") || msg.contains("::") {
        return "cast".to_string();
    }
    if contains_word(msg, "over") || contains_word(msg, "window") {
        return "window".to_string();
    }
    if contains_word(msg, "recursive") || contains_word(msg, "cte") {
        return "cte".to_string();
    }
    if contains_word(msg, "join") {
        return "join".to_string();
    }
    if contains_word(msg, "union") || contains_word(msg, "intersect") || contains_word(msg, "except") {
        return "setop".to_string();
    }
    if contains_word(msg, "case") {
        return "case".to_string();
    }
    if contains_word(msg, "function") {
        return "function".to_string();
    }
    if contains_word(msg, "merge") {
        return "merge".to_string();
    }
    if contains_word(msg, "create") {
        return "ddl-create".to_string();
    }
    if contains_word(msg, "insert") {
        return "insert".to_string();
    }
    if contains_word(msg, "update") {
        return "update".to_string();
    }
    if contains_word(msg, "delete") {
        return "delete".to_string();
    }
    if msg.contains("pg_catalog") || msg.contains("pg_") {
        return "pg_catalog".to_string();
    }
    if contains_word(msg, "concurrently") {
        return "concurrently".to_string();
    }
    if contains_word(msg, "unexpected") {
        return "unexpected-token".to_string();
    }
    format!("{layer}-other")
}

// ============================================================
// 失败签名 / Failure signature (stable hash for dedup)
// ============================================================

/// FNV-1a 64-bit, deterministic and stable across Rust versions.
fn fnv1a_64(s: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn compute_signature(kind: FailureKind, error_class: &str, error_message: &str) -> String {
    // 取错误信息前 80 字符，去掉行/列位置前缀以稳定签名
    let stripped = strip_location_prefix(error_message);
    let prefix: String = stripped.chars().take(80).collect();
    let raw = format!("{}::{}::{}", kind.as_str(), error_class, prefix);
    let h = fnv1a_64(&raw);
    format!("fnv1a:{h:012x}")
}

fn strip_location_prefix(msg: &str) -> String {
    // 去掉 "at line X, column Y" 形式的位置前缀
    let mut s = msg.to_string();
    while let Some(idx) = s.find(" at line ") {
        // 找到下一个 ':' 或结尾，截断
        if let Some(colon) = s[idx..].find(':') {
            s = s[idx + colon..].trim_start_matches([' ', ':']).to_string();
        } else {
            s.truncate(idx);
            break;
        }
    }
    s.trim().to_string()
}

// ============================================================
// 语料读取 / Corpus reading
// ============================================================

/// 把 SQL 文件按语句切分（按 `;` 终止符，处理多行）。
fn split_statements(content: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut buf = String::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("--") {
            continue;
        }
        if !buf.is_empty() {
            buf.push(' ');
        }
        buf.push_str(line);
        // 简单启发：行末以 `;` 结束视为一条完整语句
        if line.ends_with(';') {
            let stmt = buf.trim_end_matches(';').trim().to_string();
            if !stmt.is_empty() {
                statements.push(stmt);
            }
            buf.clear();
        }
    }
    if !buf.is_empty() {
        let stmt = buf.trim().to_string();
        if !stmt.is_empty() {
            statements.push(stmt);
        }
    }
    statements
}

fn read_corpus(path: &Path) -> Result<Vec<(String, String)>, String> {
    // 返回 (seed_hint, sql) 列表；seed_hint 从文件名推断
    let mut out = Vec::new();

    let files: Vec<PathBuf> = if path.is_file() {
        vec![path.to_path_buf()]
    } else {
        let mut fs_files: Vec<PathBuf> = fs::read_dir(path)
            .map_err(|e| format!("read_dir({}): {e}", path.display()))?
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("sql"))
            .collect();
        fs_files.sort();
        fs_files
    };

    for f in files {
        let seed_hint = extract_seed_from_name(f.file_name().and_then(|s| s.to_str()).unwrap_or(""));
        let content = fs::read_to_string(&f).map_err(|e| format!("read {}: {e}", f.display()))?;
        for stmt in split_statements(&content) {
            out.push((seed_hint.clone(), stmt));
        }
    }
    Ok(out)
}

fn extract_seed_from_name(name: &str) -> String {
    // corpus-s42-50000.sql -> "42"
    if let Some(rest) = name.strip_prefix("corpus-s") {
        if let Some(end) = rest.find('-') {
            return rest[..end].to_string();
        }
        return rest.trim_end_matches(".sql").to_string();
    }
    String::new()
}

// ============================================================
// known-acceptable-failures 加载
// ============================================================

struct KnownRules {
    /// (kind, pattern) pairs. kind = "PREFIX" 或 "REGEX"（REGEX 当前做简单 glob，仅支持 `*`）
    rules: Vec<(String, String)>,
}

impl KnownRules {
    fn load(path: &Path) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| format!("read known {}: {e}", path.display()))?;
        let mut rules = Vec::new();
        for (i, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((lhs, rhs)) = line.split_once(":|:") {
                let lhs = lhs.trim();
                let rhs = rhs.trim();
                match lhs {
                    "PREFIX" => rules.push(("PREFIX".to_string(), rhs.to_string())),
                    "REGEX" => rules.push(("REGEX".to_string(), rhs.to_string())),
                    _ => return Err(format!("line {}: unknown rule kind '{lhs}' (expected PREFIX or REGEX)", i + 1)),
                }
            } else {
                return Err(format!("line {}: missing ':|:' separator", i + 1));
            }
        }
        Ok(Self { rules })
    }

    fn matches(&self, kind: FailureKind, error_class: &str, error_message: &str) -> bool {
        let sig = format!("error_class={error_class}");
        for (rule_kind, pattern) in &self.rules {
            if rule_kind == "PREFIX" {
                if sig.starts_with(pattern) || error_message.contains(pattern) {
                    return true;
                }
            } else if rule_kind == "REGEX" && glob_match(pattern, error_message) {
                return true;
            }
        }
        let _ = kind;
        false
    }
}

/// 极简 glob 匹配：仅支持 `*` 作为通配符。不引入 regex crate 依赖。
fn glob_match(pattern: &str, text: &str) -> bool {
    if !pattern.contains('*') {
        return text.contains(pattern);
    }
    let parts: Vec<&str> = pattern.split('*').filter(|p| !p.is_empty()).collect();
    if parts.is_empty() {
        return true;
    }
    let mut pos = 0;
    for part in &parts {
        match text[pos..].find(part) {
            Some(idx) => pos += idx + part.len(),
            None => return false,
        }
    }
    true
}

// ============================================================
// regress/ 案例读写
// ============================================================

fn case_dirs(regress_dir: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> =
        fs::read_dir(regress_dir).map(|rd| rd.filter_map(Result::ok).map(|e| e.path()).collect()).unwrap_or_default();
    out.retain(|p| p.is_dir());
    out.sort();
    out
}

fn next_case_id(regress_dir: &Path) -> String {
    let mut max_n = 0u32;
    for case_dir in case_dirs(regress_dir) {
        if let Some(name) = case_dir.file_name().and_then(|n| n.to_str()) {
            if let Some(num_str) = name.split('-').next() {
                if let Ok(n) = num_str.parse::<u32>() {
                    max_n = max_n.max(n);
                }
            }
        }
    }
    format!("{:04}", max_n + 1)
}

fn slugify(s: &str) -> String {
    let mut out: String =
        s.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' { c.to_ascii_lowercase() } else { '-' }).collect();
    while out.contains("--") {
        out = out.replace("--", "-");
    }
    let trimmed = out.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "unnamed".to_string()
    } else if trimmed.len() > 50 {
        trimmed[..50].trim_end_matches('-').to_string()
    } else {
        trimmed
    }
}

fn case_exists_with_signature(regress_dir: &Path, signature: &str) -> bool {
    for case_dir in case_dirs(regress_dir) {
        let meta_path = case_dir.join("meta.json");
        if let Ok(content) = fs::read_to_string(&meta_path) {
            if let Ok(meta) = serde_json::from_str::<CaseMeta>(&content) {
                if meta.signature == signature {
                    return true;
                }
            }
        }
    }
    false
}

/// Parameters for [`create_case`], kept as a struct to keep argument count under clippy's threshold.
struct CaseParams<'a> {
    regress_dir: &'a Path,
    case_id: &'a str,
    slug: &'a str,
    sql: &'a str,
    seed_hint: &'a str,
    outcome: &'a Outcome,
    signature: &'a str,
    known_acceptable: bool,
    commit_hint: Option<String>,
}

fn create_case(p: &CaseParams<'_>) -> Result<PathBuf, String> {
    let dir_name = format!("{}-{}", p.case_id, p.slug);
    let case_dir = p.regress_dir.join(&dir_name);
    fs::create_dir_all(&case_dir).map_err(|e| format!("mkdir {}: {e}", case_dir.display()))?;

    let (kind, error_message, error_class) = match p.outcome {
        Outcome::Fail { kind, error_message, error_class } => (*kind, error_message.clone(), error_class.clone()),
        Outcome::Ok => return Err("cannot create case for Ok outcome".to_string()),
    };

    let case_sql_path = case_dir.join("case.sql");
    fs::write(&case_sql_path, format!("{};\n", p.sql))
        .map_err(|e| format!("write {}: {e}", case_sql_path.display()))?;

    let meta = CaseMeta {
        id: p.case_id.to_string(),
        slug: p.slug.to_string(),
        added_at: today_iso(),
        added_in_commit: p.commit_hint.clone(),
        kind: kind.as_str().to_string(),
        signature: p.signature.to_string(),
        error_message: error_message.clone(),
        error_class: error_class.clone(),
        expected_outcome: "FAIL".to_string(),
        fixed_in_commit: None,
        known_acceptable: p.known_acceptable,
        seed: if p.seed_hint.is_empty() { None } else { Some(p.seed_hint.to_string()) },
        notes: None,
    };
    let meta_path = case_dir.join("meta.json");
    let meta_json = serde_json::to_string_pretty(&meta).map_err(|e| format!("serialize meta: {e}"))?;
    fs::write(&meta_path, format!("{meta_json}\n")).map_err(|e| format!("write {}: {e}", meta_path.display()))?;

    Ok(case_dir)
}

fn today_iso() -> String {
    // 简化：用系统时间。harness 不依赖 chrono。
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    // 仅返回 "<epoch>" 作为占位；生产环境可改为读系统时区
    // 不引入 chrono 依赖，保持零额外依赖
    format!("epoch:{secs}")
}

// ============================================================
// INDEX.md 维护
// ============================================================

fn write_index_md(regress_dir: &Path) -> Result<(), String> {
    let mut cases: Vec<(String, CaseMeta)> = Vec::new();
    for case_dir in case_dirs(regress_dir) {
        let meta_path = case_dir.join("meta.json");
        let content = match fs::read_to_string(&meta_path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        if let Ok(meta) = serde_json::from_str::<CaseMeta>(&content) {
            cases.push((case_dir.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string(), meta));
        }
    }
    cases.sort_by(|a, b| a.0.cmp(&b.0));

    let mut out = String::new();
    out.push_str("# `regress/` 案例总览 / Case index\n\n");
    out.push_str("> 自动生成，请勿手动编辑。/ Auto-generated, do not edit manually.\n\n");
    out.push_str("| ID | dir | kind | class | added | status |\n");
    out.push_str("|---|---|---|---|---|---|\n");
    for (dir, m) in &cases {
        out.push_str(&format!(
            "| {} | `{}` | {} | {} | {} | {} |\n",
            m.id, dir, m.kind, m.error_class, m.added_at, m.expected_outcome
        ));
    }
    let idx_path = regress_dir.join("INDEX.md");
    fs::write(&idx_path, out).map_err(|e| format!("write {}: {e}", idx_path.display()))?;
    Ok(())
}

// ============================================================
// CSV / metrics 写出
// ============================================================

fn write_csv(path: &Path, header: &str, rows: &[Vec<String>]) -> Result<(), String> {
    let mut s = String::from(header);
    s.push('\n');
    for row in rows {
        let escaped: Vec<String> = row
            .iter()
            .map(|cell| {
                if cell.contains(',') || cell.contains('"') || cell.contains('\n') {
                    format!("\"{}\"", cell.replace('"', "\"\""))
                } else {
                    cell.clone()
                }
            })
            .collect();
        s.push_str(&escaped.join(","));
        s.push('\n');
    }
    fs::write(path, s).map_err(|e| format!("write {}: {e}", path.display()))?;
    Ok(())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Metrics {
    total_cases: usize,
    expected_fail: usize,
    expected_ok: usize,
    still_failing: usize,
    fixed: usize,
    regressions: usize,
    generated_at: String,
}

// ============================================================
// 子命令实现 / Subcommand implementations
// ============================================================

mod args {
    pub fn parse_flag<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
        let needle = format!("--{key}");
        for (i, a) in args.iter().enumerate() {
            if a == &needle {
                return args.get(i + 1).map(|s| s.as_str());
            }
            let prefix = format!("--{key}=");
            if let Some(v) = a.strip_prefix(&prefix) {
                return Some(v);
            }
        }
        None
    }
}

fn cmd_mine(args: &[String]) -> Result<usize, String> {
    let corpus_arg = args.first().ok_or_else(|| "missing corpus path argument".to_string()).map(|s| s.as_str())?;
    let corpus_path = PathBuf::from(corpus_arg);
    let out_dir = PathBuf::from(args::parse_flag(args, "out").unwrap_or("regress"));
    let known_path = args::parse_flag(args, "known").map(PathBuf::from);
    let report_dir = PathBuf::from(args::parse_flag(args, "report").unwrap_or("reports"));
    let max_statements: Option<usize> = args::parse_flag(args, "max-statements")
        .map(|s| s.parse().map_err(|e| format!("--max-statements: {e}")))
        .transpose()?;
    let commit_hint = std::env::var("GIT_COMMIT_HINT").ok().or_else(|| {
        // 试着拿 short git hash（不要求成功）
        std::process::Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    });

    fs::create_dir_all(&out_dir).map_err(|e| format!("mkdir {}: {e}", out_dir.display()))?;
    fs::create_dir_all(&report_dir).map_err(|e| format!("mkdir {}: {e}", report_dir.display()))?;

    let known = if let Some(p) = &known_path {
        match KnownRules::load(p) {
            Ok(k) => k,
            Err(e) => {
                eprintln!("warn: failed to load known-acceptable rules: {e}");
                eprintln!("warn: continuing without known rules");
                KnownRules { rules: vec![] }
            }
        }
    } else {
        KnownRules { rules: vec![] }
    };

    let corpus = read_corpus(&corpus_path)?;
    let total = corpus.len();
    let take = max_statements.unwrap_or(total).min(total);
    eprintln!("mine: scanning {take}/{total} statements from {}", corpus_path.display());

    let mut ok_count = 0usize;
    let mut fail_count = 0usize;
    let mut known_count = 0usize;
    let mut new_cases: Vec<(PathBuf, String, Outcome)> = Vec::new();
    let mut new_failures_csv_rows: Vec<Vec<String>> = Vec::new();

    for (i, (seed_hint, sql)) in corpus.iter().enumerate().take(take) {
        if i % 1000 == 0 && i > 0 {
            eprintln!("  progress {i}/{take} (ok={ok_count}, fail={fail_count}, new={})", new_cases.len());
        }
        let outcome = try_parse(sql, true);
        if outcome.is_ok() {
            ok_count += 1;
            continue;
        }
        fail_count += 1;
        let (kind, error_message, error_class) = match &outcome {
            Outcome::Fail { kind, error_message, error_class } => (*kind, error_message.clone(), error_class.clone()),
            Outcome::Ok => unreachable!(),
        };
        let signature = compute_signature(kind, &error_class, &error_message);
        let known_acceptable = known.matches(kind, &error_class, &error_message);
        if known_acceptable {
            known_count += 1;
        }
        if case_exists_with_signature(&out_dir, &signature) {
            continue;
        }
        let case_id = next_case_id(&out_dir);
        let slug = slugify(&error_class);
        let case_dir = create_case(&CaseParams {
            regress_dir: &out_dir,
            case_id: &case_id,
            slug: &slug,
            sql,
            seed_hint,
            outcome: &outcome,
            signature: &signature,
            known_acceptable,
            commit_hint: commit_hint.clone(),
        })?;
        new_cases.push((case_dir.clone(), sql.clone(), outcome.clone()));
        new_failures_csv_rows.push(vec![
            case_id,
            seed_hint.clone(),
            kind.as_str().to_string(),
            error_class,
            signature,
            truncate_for_csv(&error_message),
            truncate_for_csv(sql),
        ]);
    }

    eprintln!("mine: done — ok={ok_count}, fail={fail_count}, known={known_count}, new_cases={}", new_cases.len());

    // 写报告
    let new_fail_csv = report_dir.join("new-failures.csv");
    write_csv(&new_fail_csv, "case_id,seed,kind,error_class,signature,error_message,sql", &new_failures_csv_rows)?;
    eprintln!("mine: wrote {}", new_fail_csv.display());

    // 重写 INDEX.md
    write_index_md(&out_dir)?;
    eprintln!("mine: refreshed {}/INDEX.md", out_dir.display());

    Ok(new_cases.len())
}

fn truncate_for_csv(s: &str) -> String {
    let s: String = s.chars().map(|c| if c == '\n' || c == '\r' { ' ' } else { c }).collect();
    if s.len() > 500 {
        let mut t = s[..500].to_string();
        t.push_str("...(truncated)");
        t
    } else {
        s
    }
}

fn cmd_guard(args: &[String]) -> Result<usize, String> {
    let cases_dir = PathBuf::from(args::parse_flag(args, "cases").unwrap_or("regress"));
    let baseline_path = args::parse_flag(args, "baseline").map(PathBuf::from);
    let report_dir = PathBuf::from(args::parse_flag(args, "report").unwrap_or("reports"));
    fs::create_dir_all(&report_dir).map_err(|e| format!("mkdir {}: {e}", report_dir.display()))?;

    let dirs = case_dirs(&cases_dir);
    if dirs.is_empty() {
        eprintln!("guard: no cases under {}", cases_dir.display());
        // 仍写出空 metrics 以便 CI 通过
        let m = Metrics {
            total_cases: 0,
            expected_fail: 0,
            expected_ok: 0,
            still_failing: 0,
            fixed: 0,
            regressions: 0,
            generated_at: today_iso(),
        };
        write_metrics(&report_dir, &m, baseline_path.as_deref())?;
        return Ok(0);
    }

    let mut total = 0usize;
    let mut expected_fail = 0usize;
    let mut expected_ok = 0usize;
    let mut still_failing = 0usize;
    let mut fixed = 0usize;
    let mut regressions = 0usize;
    let mut regression_rows: Vec<Vec<String>> = Vec::new();
    let mut improvement_rows: Vec<Vec<String>> = Vec::new();

    for case_dir in &dirs {
        let case_name = case_dir.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        let meta_path = case_dir.join("meta.json");
        let case_sql_path = case_dir.join("case.sql");

        let meta_content = match fs::read_to_string(&meta_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("warn: skip {}: {e}", meta_path.display());
                continue;
            }
        };
        let meta: CaseMeta = match serde_json::from_str(&meta_content) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("warn: skip {} parse meta: {e}", meta_path.display());
                continue;
            }
        };
        let sql = match fs::read_to_string(&case_sql_path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("warn: skip {}: {e}", case_sql_path.display());
                continue;
            }
        };
        // trim whitespace first, then trailing ';', then whitespace again
        let sql = sql.trim().trim_end_matches(';').trim().to_string();
        // Round-trip 用一次就够，无论 kind；为 fixed 案例验证完整 round-trip
        let outcome = try_parse(&sql, true);
        let actual_ok = outcome.is_ok();
        let expected_ok_flag = meta.expected_outcome == "OK";

        total += 1;
        if expected_ok_flag {
            expected_ok += 1;
        } else {
            expected_fail += 1;
        }

        match (expected_ok_flag, actual_ok) {
            (false, false) => {
                still_failing += 1;
            }
            (false, true) => {
                fixed += 1;
                improvement_rows.push(vec![
                    meta.id.clone(),
                    case_name.clone(),
                    meta.kind.clone(),
                    meta.error_class.clone(),
                    meta.signature.clone(),
                    "FAIL -> OK".to_string(),
                ]);
            }
            (true, true) => {
                // 守护通过
            }
            (true, false) => {
                regressions += 1;
                let (kind, error_message) = match &outcome {
                    Outcome::Fail { kind, error_message, .. } => (kind.as_str().to_string(), error_message.clone()),
                    Outcome::Ok => unreachable!(),
                };
                regression_rows.push(vec![
                    meta.id.clone(),
                    case_name.clone(),
                    meta.kind.clone(),
                    meta.error_class.clone(),
                    meta.signature.clone(),
                    format!("OK -> {kind}"),
                    truncate_for_csv(&error_message),
                ]);
            }
        }
    }

    let m = Metrics {
        total_cases: total,
        expected_fail,
        expected_ok,
        still_failing,
        fixed,
        regressions,
        generated_at: today_iso(),
    };

    // 写出指标和报告
    write_metrics(&report_dir, &m, baseline_path.as_deref())?;
    write_csv(
        &report_dir.join("regressions.csv"),
        "case_id,dir,kind,error_class,signature,transition,error_message",
        &regression_rows,
    )?;
    write_csv(
        &report_dir.join("improvements.csv"),
        "case_id,dir,kind,error_class,signature,transition",
        &improvement_rows,
    )?;

    eprintln!(
        "guard: total={total}, expected_fail={expected_fail}, expected_ok={expected_ok}, still_failing={still_failing}, fixed={fixed}, regressions={regressions}"
    );

    Ok(regressions)
}

fn write_metrics(report_dir: &Path, m: &Metrics, baseline: Option<&Path>) -> Result<(), String> {
    let path = report_dir.join("metrics.json");
    let body = serde_json::to_string_pretty(m).map_err(|e| format!("serialize metrics: {e}"))?;
    fs::write(&path, format!("{body}\n")).map_err(|e| format!("write {}: {e}", path.display()))?;

    // 简单 baseline diff
    let mut summary = String::new();
    summary.push_str("# SQLsmith guard summary\n\n");
    summary.push_str(&format!("- total_cases: {}\n", m.total_cases));
    summary.push_str(&format!("- expected_fail: {}\n", m.expected_fail));
    summary.push_str(&format!("- expected_ok: {}\n", m.expected_ok));
    summary.push_str(&format!("- still_failing: {}\n", m.still_failing));
    summary.push_str(&format!("- improvements: {}\n", m.fixed));
    summary.push_str(&format!("- **regressions: {}**\n", m.regressions));

    if let Some(bp) = baseline {
        if let Ok(prev) = fs::read_to_string(bp) {
            if let Ok(prev_m) = serde_json::from_str::<Metrics>(&prev) {
                summary.push_str(&format!("\n## vs baseline ({})\n\n", bp.display()));
                summary.push_str(&format!("- total_cases: {} -> {}\n", prev_m.total_cases, m.total_cases));
                summary.push_str(&format!("- regressions: {} -> {}\n", prev_m.regressions, m.regressions));
                summary.push_str(&format!("- still_failing: {} -> {}\n", prev_m.still_failing, m.still_failing));
            } else {
                summary.push_str(&format!("\n(baseline {} exists but unreadable)\n", bp.display()));
            }
        } else {
            summary.push_str("\n(baseline not present, treating as fresh)\n");
        }
    }
    let summary_path = report_dir.join("summary.md");
    fs::write(&summary_path, summary).map_err(|e| format!("write {}: {e}", summary_path.display()))?;
    Ok(())
}

fn cmd_run(args: &[String]) -> Result<(), String> {
    let corpus_arg = args.first().ok_or_else(|| "missing corpus path argument".to_string()).map(|s| s.as_str())?;
    let corpus_path = PathBuf::from(corpus_arg);
    let report_dir = PathBuf::from(args::parse_flag(args, "report").unwrap_or("reports"));
    let max_statements: Option<usize> = args::parse_flag(args, "max-statements")
        .map(|s| s.parse().map_err(|e| format!("--max-statements: {e}")))
        .transpose()?;
    fs::create_dir_all(&report_dir).map_err(|e| format!("mkdir {}: {e}", report_dir.display()))?;

    let corpus = read_corpus(&corpus_path)?;
    let total = corpus.len();
    let take = max_statements.unwrap_or(total).min(total);
    eprintln!("run: scanning {take}/{total} statements");

    let mut ok_count = 0usize;
    let mut fail_count = 0usize;
    let mut counts_by_kind: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut counts_by_class: BTreeMap<String, usize> = BTreeMap::new();
    let mut rows: Vec<Vec<String>> = Vec::new();

    for (i, (seed_hint, sql)) in corpus.iter().enumerate().take(take) {
        if i % 1000 == 0 && i > 0 {
            eprintln!("  progress {i}/{take}");
        }
        let outcome = try_parse(sql, true);
        match &outcome {
            Outcome::Ok => ok_count += 1,
            Outcome::Fail { kind, error_class, error_message } => {
                fail_count += 1;
                *counts_by_kind.entry(kind.as_str()).or_insert(0) += 1;
                *counts_by_class.entry(error_class.clone()).or_insert(0) += 1;
                rows.push(vec![
                    seed_hint.clone(),
                    i.to_string(),
                    kind.as_str().to_string(),
                    error_class.clone(),
                    truncate_for_csv(error_message),
                    truncate_for_csv(sql),
                ]);
            }
        }
    }

    let failures_csv = report_dir.join("run-failures.csv");
    write_csv(&failures_csv, "seed,stmt_idx,kind,error_class,error_message,sql", &rows)?;

    let mut summary = String::new();
    summary.push_str("# SQLsmith run summary\n\n");
    summary.push_str(&format!("- total: {take}\n- ok: {ok_count}\n- fail: {fail_count}\n"));
    summary.push_str("\n## by kind\n\n");
    for (k, v) in &counts_by_kind {
        summary.push_str(&format!("- {k}: {v}\n"));
    }
    summary.push_str("\n## by class (top 20)\n\n");
    let mut by_class: Vec<_> = counts_by_class.into_iter().collect();
    by_class.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
    for (cls, n) in by_class.into_iter().take(20) {
        summary.push_str(&format!("- {cls}: {n}\n"));
    }
    let summary_path = report_dir.join("run-summary.md");
    fs::write(&summary_path, summary).map_err(|e| format!("write {}: {e}", summary_path.display()))?;

    eprintln!("run: ok={ok_count} fail={fail_count}");
    eprintln!("run: wrote {} and {}", failures_csv.display(), summary_path.display());
    Ok(())
}

// ============================================================
// CLI dispatch
// ============================================================

fn usage() -> &'static str {
    r#"sqlsmith-harness — ogsql-parser SQLsmith test harness

USAGE:
    sqlsmith-harness mine <corpus-path> [--out regress/] [--known FILE]
                       [--report reports/] [--max-statements N]
    sqlsmith-harness guard [--cases regress/] [--baseline FILE]
                           [--report reports/]
    sqlsmith-harness run <corpus-path> [--report reports/] [--max-statements N]

Exit codes:
    mine : always 0 (cases created even if 0)
    guard: 0 if no regressions, 1 if any regression
    run  : always 0
"#
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("{}", usage());
        return ExitCode::from(2);
    }
    let sub = args[0].as_str();
    let rest = &args[1..];

    let result = match sub {
        "mine" => cmd_mine(rest).map(|_| 0usize),
        "guard" => cmd_guard(rest),
        "run" => cmd_run(rest).map(|_| 0),
        "-h" | "--help" | "help" => {
            println!("{}", usage());
            return ExitCode::from(0);
        }
        other => Err(format!("unknown subcommand: {other}\n\n{}", usage())),
    };

    match result {
        Ok(regression_count) => {
            if sub == "guard" && regression_count > 0 {
                eprintln!("\n❌ {regression_count} regression(s) detected");
                return ExitCode::from(1);
            }
            // 强制 stdout/stderr 刷新，防止 CI 截断
            let _ = std::io::stdout().flush();
            let _ = std::io::stderr().flush();
            ExitCode::from(0)
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}
