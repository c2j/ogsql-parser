//! Lint rule & parser warning regression guard tests.
//!
//! Scans `tests/regress/` for `.sql` fixture files.  Each file header
//! declares expected behaviour:
//!
//! ```sql
//! -- description: standalone SELECT * should warn
//! -- warn: R001
//! SELECT * FROM t1;
//! ```
//!
//! - `-- description:`     (required) human-readable test name
//! - `-- warn: R001`        (repeatable) linter rule MUST fire
//! - `-- nowarn: R001`      (repeatable) linter rule must NOT fire
//! - `-- parse-warn: <text>`(repeatable) parser warning MUST contain <text>
//! - `-- parse-nowarn: <text>`(repeatable) parser warning must NOT contain <text>
//! - `-- split: semicolon`  (optional) split by `;`, parse each block separately
//!
//! Fixtures with `-- parse-warn` or `-- parse-nowarn` relax the
//! `errors.is_empty()` check — only fatal parse errors cause failure.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use ogsql_parser::analyzer::schema::SchemaMap;
use ogsql_parser::linter::{Confidence, LintConfig, SqlLinter};
use ogsql_parser::{Parser, ParserError};

// ── metadata parsing ──────────────────────────────────────

struct Fixture {
    name: String,
    description: String,
    sql: String,
    warn: Vec<String>,
    nowarn: Vec<String>,
    parse_warn: Vec<String>,
    parse_nowarn: Vec<String>,
    split: Option<String>,
    schema_entries: Vec<SchemaEntry>,
}

struct SchemaEntry {
    table: String,
    column: String,
    data_type: String,
}

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("regress")
}

fn discover_fixtures() -> Vec<Fixture> {
    let dir = fixtures_dir();
    let mut results = Vec::new();
    collect_fixtures(&dir, "", &mut results);
    results.sort_by(|a, b| a.name.cmp(&b.name));
    results
}

fn collect_fixtures(dir: &PathBuf, prefix: &str, results: &mut Vec<Fixture>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => panic!("无法读取 regress 目录 '{}': {e}", dir.display()),
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let sub = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            let next = if prefix.is_empty() { sub.to_string() } else { format!("{prefix}/{sub}") };
            collect_fixtures(&path, &next, results);
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) != Some("sql") {
            continue;
        }
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown");
        let name = if prefix.is_empty() { stem.to_string() } else { format!("{prefix}/{stem}") };
        let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("无法读取 fixture '{}': {e}", path.display()));

        let (meta, sql) = parse_metadata(&raw);

        let description =
            meta.get("description").cloned().unwrap_or_else(|| panic!("[{name}] 缺少 '-- description:' 元数据"));
        let warn_lines = meta.get("warn").cloned().unwrap_or_default();
        let nowarn_lines = meta.get("nowarn").cloned().unwrap_or_default();
        let parse_warn_lines = meta.get("parse-warn").cloned().unwrap_or_default();
        let parse_nowarn_lines = meta.get("parse-nowarn").cloned().unwrap_or_default();
        let split = meta.get("split").cloned();
        let schema_entries = parse_schema_entries(meta.get("schema"));

        results.push(Fixture {
            name,
            description,
            sql,
            warn: split_ids(&warn_lines),
            nowarn: split_ids(&nowarn_lines),
            parse_warn: split_texts(&parse_warn_lines),
            parse_nowarn: split_texts(&parse_nowarn_lines),
            split,
            schema_entries,
        });
    }
}

/// Parse `-- key: value` metadata from comment lines at the top of a file.
fn parse_metadata(raw: &str) -> (BTreeMap<String, String>, String) {
    let mut meta: BTreeMap<String, String> = BTreeMap::new();
    let mut content_start = 0usize;

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            content_start += line.len() + 1;
            continue;
        }
        if let Some(stripped) = trimmed.strip_prefix("-- ") {
            if let Some((key, value)) = stripped.split_once(':') {
                let k = key.trim().to_lowercase();
                let v = value.trim().to_string();
                meta.entry(k).and_modify(|e| e.push_str(&format!(",{v}"))).or_insert(v);
            }
            content_start += line.len() + 1;
        } else {
            break;
        }
    }

    let sql = if content_start < raw.len() { raw[content_start..].trim().to_string() } else { String::new() };
    (meta, sql)
}

fn split_ids(s: &str) -> Vec<String> {
    s.split(',').map(|id| id.trim().to_uppercase()).filter(|id| !id.is_empty()).collect()
}

fn split_texts(s: &str) -> Vec<String> {
    s.split(',').map(|t| t.trim().to_string()).filter(|t| !t.is_empty()).collect()
}

fn parse_schema_entries(raw: Option<&String>) -> Vec<SchemaEntry> {
    let Some(raw) = raw else { return vec![] };
    raw.split(',')
        .filter_map(|entry| {
            let entry = entry.trim();
            if entry.is_empty() {
                return None;
            }
            let (table_col, data_type) = entry.split_once('=')?;
            let (table, column) = table_col.trim().split_once('.')?;
            Some(SchemaEntry {
                table: table.trim().to_string(),
                column: column.trim().to_string(),
                data_type: data_type.trim().to_string(),
            })
        })
        .collect()
}

fn build_schema(entries: &[SchemaEntry]) -> SchemaMap {
    let mut schema: SchemaMap = std::collections::HashMap::new();
    for e in entries {
        schema.entry(e.table.clone()).or_default().insert(e.column.clone(), e.data_type.clone());
    }
    schema
}

// ── assertion ─────────────────────────────────────────────

struct LintResult {
    rule_ids: Vec<String>,
    parse_warnings: Vec<String>,
    parse_errors: Vec<ParserError>,
}

fn lint_sql(sql: &str, schema: Option<&SchemaMap>, check_parse_warnings: bool) -> LintResult {
    let (infos, errors) = Parser::parse_sql(sql);
    assert!(!infos.is_empty(), "未生成任何 statement");

    let mut parse_warnings = Vec::new();
    let mut parse_errors = Vec::new();
    for e in &errors {
        if matches!(e, ParserError::Warning { .. }) || matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }) {
            parse_warnings.push(e.to_string());
        } else {
            parse_errors.push(e.clone());
        }
    }

    if !check_parse_warnings {
        assert!(parse_errors.is_empty(), "解析失败: {parse_errors:?}");
    } else {
        assert!(parse_errors.is_empty(), "解析失败(非 Warning): {parse_errors:?}");
    }

    let linter = SqlLinter::with_default_rules(LintConfig::default());
    let linter_warnings = linter.lint(&infos, schema, Confidence::Full);
    let rule_ids = linter_warnings.iter().map(|w| w.rule_id.clone()).collect();

    LintResult { rule_ids, parse_warnings, parse_errors }
}

// ── test entry ────────────────────────────────────────────

#[test]
fn all_regress_fixtures() {
    let fixtures = discover_fixtures();
    assert!(!fixtures.is_empty(), "未发现 regress fixture 文件 (tests/regress/*.sql)");

    for f in &fixtures {
        let label = format!("{} ({})", f.name, f.description);
        let schema = if f.schema_entries.is_empty() { None } else { Some(build_schema(&f.schema_entries)) };
        let check_parse_warnings = !f.parse_warn.is_empty() || !f.parse_nowarn.is_empty();

        let lint_result = if let Some(ref sep) = f.split {
            let delimiter = match sep.as_str() {
                "semicolon" => ";",
                "blank-line" => "\n\n",
                other => panic!("[{label}] 未知 Split 值: '{other}'"),
            };
            let blocks: Vec<&str> = f.sql.split(delimiter).filter(|b| !b.trim().is_empty()).collect();
            assert!(!blocks.is_empty(), "[{label}] 无可分块的 SQL");
            let mut combined =
                LintResult { rule_ids: Vec::new(), parse_warnings: Vec::new(), parse_errors: Vec::new() };
            for block in &blocks {
                let sql = if delimiter == "\n\n" { block.to_string() } else { format!("{};", block.trim()) };
                let r = lint_sql(&sql, schema.as_ref(), check_parse_warnings);
                combined.rule_ids.extend(r.rule_ids);
                combined.parse_warnings.extend(r.parse_warnings);
                combined.parse_errors.extend(r.parse_errors);
            }
            combined
        } else {
            lint_sql(&f.sql, schema.as_ref(), check_parse_warnings)
        };

        for id in &f.warn {
            assert!(
                lint_result.rule_ids.contains(id),
                "[{label}] 期望规则 {id} 触发，实际未触发\n  实际触发的规则: {:?}",
                lint_result.rule_ids
            );
        }

        for id in &f.nowarn {
            assert!(
                !lint_result.rule_ids.contains(id),
                "[{label}] 期望规则 {id} 不触发，实际触发了\n  实际触发的规则: {:?}",
                lint_result.rule_ids
            );
        }

        for text in &f.parse_warn {
            assert!(
                lint_result.parse_warnings.iter().any(|w| w.contains(text.as_str())),
                "[{label}] 期望 parser warning 包含 '{text}'，实际未匹配\n  实际 parser warning: {:?}",
                lint_result.parse_warnings
            );
        }

        for text in &f.parse_nowarn {
            assert!(
                !lint_result.parse_warnings.iter().any(|w| w.contains(text.as_str())),
                "[{label}] 期望 parser warning 不包含 '{text}'，实际匹配了\n  实际 parser warning: {:?}",
                lint_result.parse_warnings
            );
        }

        eprintln!("  ✓ {label}");
    }

    eprintln!("  [regress] {} fixture(s) passed", fixtures.len());
}
