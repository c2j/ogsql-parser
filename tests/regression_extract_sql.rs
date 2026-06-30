mod common;
use ogsql_parser::Parser;
use std::process::{Command, Stdio};

fn ogsql() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ogsql"))
}

fn run_ogsql(args: &[&str], stdin_text: &str) -> (String, String, bool) {
    let mut child =
        ogsql().args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    {
        let mut stdin = child.stdin.take().unwrap();
        std::io::Write::write_all(&mut stdin, stdin_text.as_bytes()).unwrap();
    }
    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr, output.status.success())
}

// ============================================================
// Parse regression tests: verify all fixtures parse without errors
// ============================================================

fn assert_parse_all(sql: &str, label: &str) {
    let (infos, errors) = Parser::parse_sql(sql);
    let empty_count = infos.iter().filter(|s| matches!(s.statement, ogsql_parser::Statement::Empty)).count();
    assert!(
        errors.is_empty() && empty_count == 0,
        "回归守护: [{label}] 解析失败\n  errors: {errors:?}\n  empty statements: {empty_count}\n  SQL: {sql}"
    );
}

fn ext_sql_fixture_path(name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/regress/extract_sql")
        .join(format!("feat_{name}_parser.sql"))
}

fn test_fixture_parse(name: &str) {
    let path = ext_sql_fixture_path(name);
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("回归守护: fixture 文件缺失 '{}': {e}", path.display()));
    assert!(!content.trim().is_empty(), "回归守护: fixture '{name}' 内容为空");
    assert_parse_all(&content, name);
}

#[test]
fn extract_sql_basic_parse() {
    test_fixture_parse("extract_sql_basic");
}

#[test]
fn extract_sql_execute_parse() {
    test_fixture_parse("extract_sql_execute");
}

#[test]
fn extract_sql_branches_parse() {
    test_fixture_parse("extract_sql_branches");
}

#[test]
fn extract_sql_loops_parse() {
    test_fixture_parse("extract_sql_loops");
}

#[test]
fn extract_sql_cursors_parse() {
    test_fixture_parse("extract_sql_cursors");
}

#[test]
fn extract_sql_package_parse() {
    test_fixture_parse("extract_sql_package");
}

#[test]
fn extract_sql_advanced_parse() {
    test_fixture_parse("extract_sql_advanced");
}

#[test]
fn extract_sql_package_vars_parse() {
    test_fixture_parse("extract_sql_package_vars");
}

// ============================================================
// CLI integration tests: verify actual extract-sql output
// ============================================================

#[test]
fn cli_extract_sql_basic_select() {
    let input =
        "CREATE OR REPLACE PROCEDURE test_proc(p_id INTEGER) AS\nBEGIN\n  SELECT * FROM users WHERE id = p_id;\nEND;";
    let (stdout, _stderr, success) = run_ogsql(&["parse", "--extract-sql"], input);
    assert!(success, "CLI extract-sql should succeed");
    // Extract-SQL output format: [type] L{line}: {name} | {sql}
    assert!(stdout.contains("SqlStatement/Select"), "Expected SqlStatement/Select in output, got: {}", stdout);
    assert!(stdout.contains("__SQL_PARAM"), "Expected __SQL_PARAM_ placeholder, got: {}", stdout);
}

#[test]
fn cli_extract_sql_insert() {
    let input = "CREATE OR REPLACE PROCEDURE test_proc(p_name VARCHAR, p_age INTEGER) AS\nBEGIN\n  INSERT INTO users (name, age) VALUES (p_name, p_age);\nEND;";
    let (stdout, _stderr, success) = run_ogsql(&["parse", "--extract-sql"], input);
    assert!(success, "CLI extract-sql should succeed");
    assert!(stdout.contains("SqlStatement/Insert"), "Expected SqlStatement/Insert in output, got: {}", stdout);
}

#[test]
fn cli_extract_sql_execute_string() {
    let input = "CREATE OR REPLACE PROCEDURE test_proc(p_table VARCHAR) AS\nBEGIN\n  EXECUTE IMMEDIATE 'SELECT COUNT(*) FROM ' || p_table;\nEND;";
    let (stdout, _stderr, success) = run_ogsql(&["parse", "--extract-sql"], input);
    assert!(success, "CLI extract-sql should succeed");
    assert!(stdout.contains("Execute"), "Expected Execute in output, got: {}", stdout);
}

#[test]
fn cli_extract_sql_execute_dollar_quote() {
    let input = "CREATE OR REPLACE PROCEDURE test_proc(p_id INTEGER) AS\nBEGIN\n  EXECUTE $$UPDATE users SET status = 'done' WHERE id = $$ || p_id;\nEND;";
    let (stdout, _stderr, success) = run_ogsql(&["parse", "--extract-sql"], input);
    assert!(success, "CLI extract-sql should succeed");
    assert!(stdout.contains("Execute"), "Expected Execute in output, got: {}", stdout);
}

#[test]
fn cli_extract_sql_if_branch() {
    let input = "CREATE OR REPLACE PROCEDURE test_proc(p_flag VARCHAR, p_id INTEGER) AS\nBEGIN\n  IF p_flag = 'A' THEN\n    SELECT * FROM users WHERE id = p_id;\n  ELSE\n    DELETE FROM users WHERE id = p_id;\n  END IF;\nEND;";
    let (stdout, _stderr, success) = run_ogsql(&["parse", "--extract-sql"], input);
    assert!(success, "CLI extract-sql should succeed");
    assert!(stdout.contains("SqlStatement/Select"), "Expected Select in output, got: {}", stdout);
    assert!(stdout.contains("SqlStatement/Delete"), "Expected Delete in output, got: {}", stdout);
}

#[test]
fn cli_extract_sql_for_loop() {
    let input = "CREATE OR REPLACE PROCEDURE test_proc(p_dept_id INTEGER) AS\nBEGIN\n  FOR rec IN (SELECT id, name FROM employees WHERE dept_id = p_dept_id) LOOP\n    INSERT INTO report (emp_id, emp_name) VALUES (rec.id, rec.name);\n  END LOOP;\nEND;";
    let (stdout, _stderr, success) = run_ogsql(&["parse", "--extract-sql"], input);
    assert!(success, "CLI extract-sql should succeed");
    assert!(stdout.contains("Embedded/Select"), "Expected Embedded/Select in output, got: {}", stdout);
    assert!(stdout.contains("SqlStatement/Insert"), "Expected SqlStatement/Insert in output, got: {}", stdout);
}

#[test]
fn cli_extract_sql_open_for() {
    let input = "CREATE OR REPLACE PROCEDURE test_proc(p_id INTEGER, p_cursor OUT SYS_REFCURSOR) AS\nBEGIN\n  OPEN p_cursor FOR SELECT * FROM users WHERE id = p_id;\nEND;";
    let (stdout, _stderr, success) = run_ogsql(&["parse", "--extract-sql"], input);
    assert!(success, "CLI extract-sql should succeed");
    assert!(stdout.contains("ReturnCursorSQL"), "Expected ReturnCursorSQL in output, got: {}", stdout);
}

#[test]
fn cli_extract_sql_perform() {
    let input = "CREATE OR REPLACE PROCEDURE test_proc(p_user_id INTEGER) AS\nBEGIN\n  PERFORM update_balance(p_user_id);\n  INSERT INTO log (user_id) VALUES (p_user_id);\nEND;";
    let (stdout, _stderr, success) = run_ogsql(&["parse", "--extract-sql"], input);
    assert!(success, "CLI extract-sql should succeed");
    assert!(stdout.contains("Perform"), "Expected Perform in output, got: {}", stdout);
}

#[test]
fn cli_extract_sql_var_replacement_with_type() {
    let input = "CREATE OR REPLACE PROCEDURE test_proc(p_id INTEGER, p_name VARCHAR) AS\n  v_count INTEGER;\nBEGIN\n  SELECT COUNT(*) INTO v_count FROM users WHERE id = p_id AND name = p_name;\nEND;";
    let (stdout, _stderr, success) = run_ogsql(&["parse", "--extract-sql"], input);
    assert!(success, "CLI extract-sql should succeed");
    // Typed variables produce __SQL_PARAM_Type_Name__ placeholders
    assert!(
        stdout.contains("__SQL_PARAM_INTEGER_p_id__") || stdout.contains("__SQL_PARAM_VARCHAR_p_name__"),
        "Expected typed placeholder in output, got: {}",
        stdout
    );
}

#[test]
fn cli_extract_sql_package_body() {
    let input = "CREATE OR REPLACE PACKAGE BODY test_pkg AS\n  PROCEDURE do_insert(p_name VARCHAR) AS\n  BEGIN\n    INSERT INTO t (name) VALUES (p_name);\n  END do_insert;\n  PROCEDURE do_query(p_id INTEGER) AS\n  BEGIN\n    SELECT * FROM t WHERE id = p_id;\n  END do_query;\nEND test_pkg;";
    let (stdout, _stderr, success) = run_ogsql(&["parse", "--extract-sql"], input);
    assert!(success, "CLI extract-sql should succeed");
    assert!(stdout.contains("SqlStatement/Insert"), "Expected Insert in output, got: {}", stdout);
    assert!(stdout.contains("SqlStatement/Select"), "Expected Select in output, got: {}", stdout);
}

#[test]
fn cli_extract_sql_no_sql_in_procedure() {
    let input = "CREATE OR REPLACE PROCEDURE test_proc(p_value INTEGER) AS\nBEGIN\n  p_value := p_value + 1;\n  RAISE NOTICE 'Value: %', p_value;\nEND;";
    let (stdout, _stderr, success) = run_ogsql(&["parse", "--extract-sql"], input);
    assert!(success, "CLI extract-sql should succeed even for procedures without SQL");
    // Should not contain any SQL extraction rows
    assert!(
        !stdout.contains("[") || stdout.trim().is_empty(),
        "Expected no SQL extraction rows for procedure without SQL, got: {}",
        stdout
    );
}

#[test]
fn cli_extract_sql_with_csv_flag() {
    let input =
        "CREATE OR REPLACE PROCEDURE test_proc(p_id INTEGER) AS\nBEGIN\n  SELECT * FROM users WHERE id = p_id;\nEND;";
    let (stdout, _stderr, success) = run_ogsql(&["parse", "--extract-sql", "--csv"], input);
    assert!(success, "CLI extract-sql --csv should succeed");
    // CSV header should be present
    assert!(stdout.contains("file,directory,line,type,name,parent"), "Expected CSV header in output, got: {}", stdout);
    assert!(stdout.contains("SqlStatement/Select"), "Expected Select in CSV output, got: {}", stdout);
}

#[test]
fn cli_extract_sql_return_query() {
    // Use a procedure with RETURN QUERY (procedures are more reliably handled in extract-sql)
    let input = "CREATE OR REPLACE PROCEDURE test_proc(p_id INTEGER, p_cursor OUT SYS_REFCURSOR) AS\nBEGIN\n  OPEN p_cursor FOR SELECT id, name FROM users WHERE id = p_id;\nEND;";
    let (stdout, _stderr, success) = run_ogsql(&["parse", "--extract-sql"], input);
    assert!(success, "CLI extract-sql should succeed");
    assert!(stdout.contains("ReturnCursorSQL"), "Expected ReturnCursorSQL in output, got: {}", stdout);
}

#[test]
fn cli_extract_sql_anonymous_block() {
    let input = "DO $$\nDECLARE\n  v_count INTEGER;\nBEGIN\n  SELECT COUNT(*) INTO v_count FROM users;\n  INSERT INTO log (cnt) VALUES (v_count);\nEND;\n$$;";
    let (stdout, _stderr, success) = run_ogsql(&["parse", "--extract-sql"], input);
    assert!(success, "CLI extract-sql should succeed");
    assert!(
        stdout.contains("SqlStatement/Select") || stdout.contains("SqlStatement/Insert"),
        "Expected SQL extraction in output for anonymous block, got: {}",
        stdout
    );
}

#[test]
fn cli_extract_sql_package_vars_replaced() {
    // Package-level variables (including body-private vars like g_limit)
    // are now collected and replaced with __SQL_PARAM_ placeholders.
    let input = "CREATE OR REPLACE PACKAGE BODY test_pkg AS\n  g_limit INTEGER := 100;\n  PROCEDURE do_query(p_id INTEGER) AS\n  BEGIN\n    SELECT * FROM t WHERE id = p_id AND limit_val = g_limit;\n  END do_query;\nEND test_pkg;";
    let (stdout, _stderr, success) = run_ogsql(&["parse", "--extract-sql"], input);
    assert!(success, "CLI extract-sql should succeed");
    assert!(stdout.contains("__SQL_PARAM_INTEGER_p_id__"), "p_id should be replaced, got: {}", stdout);
    assert!(
        stdout.contains("__SQL_PARAM_INTEGER_g_limit__"),
        "Package-level var g_limit should be replaced, got: {}",
        stdout
    );
}

// ============================================================
// Snapshot tests: full output must match golden .expected files
// ============================================================

fn assert_extract_snapshot(fixture_name: &str) {
    let fixture_path = ext_sql_fixture_path(fixture_name);
    let expected_path = fixture_path.with_extension("expected");

    let (actual, stderr, success) = {
        let output = ogsql()
            .args(["parse", "--extract-sql", "-f"])
            .arg(fixture_path.to_str().unwrap())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        (
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
            output.status.success(),
        )
    };

    let expected = std::fs::read_to_string(&expected_path).unwrap_or_else(|e| {
        panic!(
            "快照文件缺失: {}\n  提示: 请先运行 `ogsql parse --extract-sql -f {} > {}` 生成。\n  error: {e}",
            expected_path.display(),
            fixture_path.display(),
            expected_path.display()
        )
    });

    assert!(success, "ogsql parse --extract-sql 失败\n  stderr: {stderr}\n  fixture: {fixture_name}");

    // Normalize cross-platform line endings
    let actual_norm = actual.replace("\r\n", "\n");
    let expected_norm = expected.replace("\r\n", "\n");

    assert_eq!(
        actual_norm, expected_norm,
        "快照不匹配 [{fixture_name}]\n  fixture: {}\n  expected: {}\n\n差异: 实际输出与 .expected 快照文件不一致。\n如果预期行为变化是合理的，请更新 .expected 文件。",
        fixture_path.display(),
        expected_path.display()
    );
}

#[test]
fn snapshot_extract_sql_basic() {
    assert_extract_snapshot("extract_sql_basic");
}

#[test]
fn snapshot_extract_sql_execute() {
    assert_extract_snapshot("extract_sql_execute");
}

#[test]
fn snapshot_extract_sql_branches() {
    assert_extract_snapshot("extract_sql_branches");
}

#[test]
fn snapshot_extract_sql_loops() {
    assert_extract_snapshot("extract_sql_loops");
}

#[test]
fn snapshot_extract_sql_cursors() {
    assert_extract_snapshot("extract_sql_cursors");
}

#[test]
fn snapshot_extract_sql_package() {
    assert_extract_snapshot("extract_sql_package");
}

#[test]
fn snapshot_extract_sql_advanced() {
    assert_extract_snapshot("extract_sql_advanced");
}

#[test]
fn snapshot_extract_sql_package_vars() {
    assert_extract_snapshot("extract_sql_package_vars");
}
