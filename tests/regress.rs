//! Lint rule regression guard tests.
//!
//! Scans `tests/regress/` for `.sql` fixture files.  Each file header
//! declares expected rule behaviour:
//!
//! ```sql
//! -- description: standalone SELECT * should warn
//! -- warn: R001
//! SELECT * FROM t1;
//! ```
//!
//! - `-- description:`  (required) human-readable test name
//! - `-- warn: R001`     (repeatable) rule MUST fire
//! - `-- nowarn: R001`   (repeatable) rule must NOT fire
//! - `-- split: semicolon` (optional) split by `;`, parse each block separately

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use ogsql_parser::linter::{Confidence, LintConfig, SqlLinter};
use ogsql_parser::Parser;

// ── metadata parsing ──────────────────────────────────────

struct Fixture {
    name: String,
    description: String,
    sql: String,
    warn: Vec<String>,
    nowarn: Vec<String>,
    split: Option<String>,
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
        let split = meta.get("split").cloned();

        results.push(Fixture {
            name,
            description,
            sql,
            warn: split_ids(&warn_lines),
            nowarn: split_ids(&nowarn_lines),
            split,
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

// ── assertion ─────────────────────────────────────────────

fn lint_sql(sql: &str) -> Vec<String> {
    let (infos, errors) = Parser::parse_sql(sql);
    assert!(errors.is_empty(), "解析失败: {errors:?}");
    assert!(!infos.is_empty(), "未生成任何 statement");

    let linter = SqlLinter::with_default_rules(LintConfig::default());
    let warnings = linter.lint(&infos, None, Confidence::Full);

    warnings.iter().map(|w| w.rule_id.clone()).collect()
}

// ── test entry ────────────────────────────────────────────

#[test]
fn all_regress_fixtures() {
    let fixtures = discover_fixtures();
    assert!(!fixtures.is_empty(), "未发现 regress fixture 文件 (tests/regress/*.sql)");

    for f in &fixtures {
        let label = format!("{} ({})", f.name, f.description);

        let rule_ids = if let Some(ref sep) = f.split {
            let delimiter = match sep.as_str() {
                "semicolon" => ";",
                "blank-line" => "\n\n",
                other => panic!("[{label}] 未知 Split 值: '{other}'"),
            };
            let blocks: Vec<&str> = f.sql.split(delimiter).filter(|b| !b.trim().is_empty()).collect();
            assert!(!blocks.is_empty(), "[{label}] 无可分块的 SQL");
            let mut all_ids = Vec::new();
            for (_i, block) in blocks.iter().enumerate() {
                let sql = if delimiter == "\n\n" { block.to_string() } else { format!("{};", block.trim()) };
                let ids = lint_sql(&sql);
                all_ids.extend(ids);
            }
            all_ids
        } else {
            lint_sql(&f.sql)
        };

        for id in &f.warn {
            assert!(rule_ids.contains(id), "[{label}] 期望规则 {id} 触发，实际未触发\n  实际触发的规则: {rule_ids:?}");
        }

        for id in &f.nowarn {
            assert!(
                !rule_ids.contains(id),
                "[{label}] 期望规则 {id} 不触发，实际触发了\n  实际触发的规则: {rule_ids:?}"
            );
        }

        eprintln!("  ✓ {label}");
    }

    eprintln!("  [regress] {} fixture(s) passed", fixtures.len());
}
