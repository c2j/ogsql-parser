//! Shared helpers for regression test files.
//!
//! Each `tests/integration/{module}.rs` imports this module and uses
//! the `fixture_tests!` macro to auto-discover and test all fixtures
//! in `tests/fixtures/{module}/`.
//!
//! ## Fixture file format
//!
//! ```sql
//! -- Issue: #263                       # optional, "N/A" if none
//! -- Description: human-readable text  # required
//! -- Expect: parse | snapshot | snapshot-only
//! -- Command: parse --extract-sql      # required when Expect=snapshot
//!
//! <SQL content>
//! ```
//!
//! ## Usage
//!
//! ```rust
//! // tests/integration/extract_sql.rs
//! mod common;
//! fixture_tests!("extract_sql");
//! ```

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

// ============================================================
// Data types
// ============================================================

pub enum ExpectMode {
    Parse,
    Snapshot { command: String },
    SnapshotOnly { command: String },
}

pub struct Fixture {
    pub name: String,
    pub module: String,
    pub issue: Option<String>,
    pub description: String,
    pub expect: ExpectMode,
    pub split: Option<String>,
    pub sql_path: PathBuf,
    pub expected_path: PathBuf,
    pub sql_content: String,
}

// ============================================================
// Discovery
// ============================================================

pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures")
}

pub fn discover_fixtures(module: &str) -> Vec<Fixture> {
    let dir = fixtures_dir().join(module);
    let mut results = Vec::new();

    let entries = match fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("Warning: fixture directory not found: {}", dir.display());
            return results;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("sql") {
            continue;
        }
        let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown").to_string();
        let expected_path = path.with_extension("expected");

        let raw = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Warning: cannot read fixture '{}': {e}", path.display());
                continue;
            }
        };

        let (meta, content) = parse_fixture_metadata(&raw);
        let issue = meta.get("Issue").filter(|v| *v != "N/A").cloned();
        let description = meta.get("Description").cloned().unwrap_or_default();
        let expect = parse_expect_mode(meta.get("Expect").map(|s| s.as_str()).unwrap_or("parse"), meta.get("Command"));
        let split = meta.get("Split").cloned();

        results.push(Fixture {
            name,
            module: module.to_string(),
            issue,
            description,
            expect,
            split,
            sql_path: path,
            expected_path,
            sql_content: content,
        });
    }

    results.sort_by(|a, b| a.name.cmp(&b.name));
    results
}

fn parse_expect_mode(expect: &str, command: Option<&String>) -> ExpectMode {
    match expect {
        "snapshot" => ExpectMode::Snapshot { command: command.cloned().unwrap_or_else(|| "parse".to_string()) },
        "snapshot-only" => {
            ExpectMode::SnapshotOnly { command: command.cloned().unwrap_or_else(|| "parse".to_string()) }
        }
        _ => ExpectMode::Parse,
    }
}

/// Parse metadata comments from the beginning of fixture content.
fn parse_fixture_metadata(raw: &str) -> (HashMap<String, String>, String) {
    let comment_prefix = "-- ";
    let mut meta = HashMap::new();
    let mut content_start = 0usize;

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            content_start += line.len() + 1;
            continue;
        }
        if let Some(stripped) = trimmed.strip_prefix(comment_prefix) {
            if let Some((key, value)) = stripped.split_once(':') {
                meta.insert(key.trim().to_string(), value.trim().to_string());
            }
            content_start += line.len() + 1;
        } else {
            break;
        }
    }

    let content = if content_start < raw.len() { raw[content_start..].trim_end().to_string() } else { String::new() };
    (meta, content)
}

// ============================================================
// Assertions
// ============================================================

pub fn assert_parse_fixture(f: &Fixture, label: &str) {
    let separator = match f.split.as_deref() {
        Some("semicolon") => ";",
        Some("blank-line") => "\n\n",
        Some("none") | None => "",
        Some(other) => {
            eprintln!("Warning [{label}]: unknown Split value '{other}', parsing as single unit");
            ""
        }
    };

    if separator.is_empty() {
        // Parse entire content as one SQL string
        let (infos, errors) = ogsql_parser::Parser::parse_sql(&f.sql_content);
        let empty_count = infos.iter().filter(|s| matches!(s.statement, ogsql_parser::Statement::Empty)).count();
        assert!(
            errors.is_empty() && empty_count == 0,
            "回归守护 [{label}]: 解析失败\n  errors: {errors:?}\n  empty statements: {empty_count}"
        );
    } else {
        // Split content by the specified separator and parse each block independently
        let blocks: Vec<&str> = f.sql_content.split(separator).filter(|b| !b.trim().is_empty()).collect();
        assert!(
            !blocks.is_empty(),
            "回归守护 [{label}]: 内容可分块为空 (Split={})",
            f.split.as_deref().unwrap_or("none")
        );
        for (i, block) in blocks.iter().enumerate() {
            let sql = if separator == "\n\n" { block.to_string() } else { format!("{};", block.trim()) };
            let (infos, errors) = ogsql_parser::Parser::parse_sql(&sql);
            let empty_count = infos.iter().filter(|s| matches!(s.statement, ogsql_parser::Statement::Empty)).count();
            assert!(
                errors.is_empty() && empty_count == 0,
                "回归守护 [{label}/block{i}]: 解析失败\n  errors: {errors:?}\n  empty statements: {empty_count}\n  SQL: {sql}"
            );
        }
    }
}

pub fn assert_snapshot_fixture(f: &Fixture, command: &str, label: &str) {
    let fixture_str = f.sql_path.to_str().unwrap();
    let args: Vec<&str> =
        command.split_whitespace().chain(std::iter::once("-f")).chain(std::iter::once(fixture_str)).collect();

    let output = Command::new(env!("CARGO_BIN_EXE_ogsql"))
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap_or_else(|e| panic!("[{label}] 无法启动 ogsql: {e}"));

    let actual = String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n");
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(output.status.success(), "[{label}] ogsql 命令失败\n  command: {command}\n  stderr: {stderr}");

    let expected = fs::read_to_string(&f.expected_path)
        .unwrap_or_else(|_e| {
            panic!(
                "[{label}] 快照文件缺失: {}\n  提示: ogsql {command} -f {} > {}",
                f.expected_path.display(),
                f.sql_path.display(),
                f.expected_path.display()
            )
        })
        .replace("\r\n", "\n");

    assert_eq!(
        actual, expected,
        "[{label}] 快照不匹配: 实际输出与 .expected 不一致\n  fixture: {}\n  expected: {}\n  如果变化合理，请更新 .expected 文件。",
        f.sql_path.display(),
        f.expected_path.display()
    );
}

// ============================================================
// fixture_tests! macro
// ============================================================

#[macro_export]
macro_rules! fixture_tests {
    ($module:expr) => {
        #[test]
        fn all_fixtures() {
            let fixtures = $crate::common::discover_fixtures($module);
            assert!(
                !fixtures.is_empty(),
                "未发现 fixture 文件 (tests/fixtures/{}/**/*.sql)\n  请检查 fixture 目录是否存在。",
                $module
            );
            for f in &fixtures {
                let label = format!("{}/{}", f.module, f.name);
                match &f.expect {
                    $crate::common::ExpectMode::Parse => {
                        $crate::common::assert_parse_fixture(f, &label);
                    }
                    $crate::common::ExpectMode::Snapshot { command } => {
                        $crate::common::assert_parse_fixture(f, &label);
                        $crate::common::assert_snapshot_fixture(f, command, &label);
                    }
                    $crate::common::ExpectMode::SnapshotOnly { command } => {
                        $crate::common::assert_snapshot_fixture(f, command, &label);
                    }
                }
            }
            eprintln!("  [fixture_tests] {}: {} fixture(s) passed", $module, fixtures.len());
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fixture_metadata() {
        let raw = "-- Issue: #263\n-- Description: test desc\n-- Expect: snapshot\n-- Command: parse --extract-sql\n\nSELECT 1;\n";
        let (meta, content) = parse_fixture_metadata(raw);
        assert_eq!(meta.get("Issue").unwrap(), "#263");
        assert_eq!(meta.get("Description").unwrap(), "test desc");
        assert_eq!(meta.get("Expect").unwrap(), "snapshot");
        assert_eq!(meta.get("Command").unwrap(), "parse --extract-sql");
        assert_eq!(content.trim(), "SELECT 1;");
    }

    #[test]
    fn test_parse_expect_parse() {
        let mode = parse_expect_mode("parse", None);
        assert!(matches!(mode, ExpectMode::Parse));
    }

    #[test]
    fn test_parse_expect_snapshot() {
        let mode = parse_expect_mode("snapshot", Some(&"parse --extract-sql".to_string()));
        match mode {
            ExpectMode::Snapshot { command } => assert_eq!(command, "parse --extract-sql"),
            _ => panic!("expected Snapshot"),
        }
    }

    #[test]
    fn test_discover_fixtures_empty() {
        let fixtures = discover_fixtures("nonexistent_module_xyz");
        assert!(fixtures.is_empty());
    }
}
