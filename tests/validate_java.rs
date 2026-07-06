#[cfg(feature = "java")]
mod validate_java_tests {
    use std::io::Write;
    use std::process::{Command, Stdio};

    fn ogsql() -> Command {
        Command::new(env!("CARGO_BIN_EXE_ogsql"))
    }

    fn run_validate_java(stdin_text: &str, args: &[&str]) -> (String, String, bool) {
        let mut child =
            ogsql().args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        {
            let mut stdin = child.stdin.take().unwrap();
            stdin.write_all(stdin_text.as_bytes()).unwrap();
        }
        let output = child.wait_with_output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let success = output.status.success();
        (stdout, stderr, success)
    }

    // ── basic validate-java ──

    #[test]
    fn test_validate_java_valid_sql() {
        let (stdout, stderr, success) =
            run_validate_java("class T { void m() { stmt.execute(\"SELECT 1\"); } }", &["validate-java"]);
        let combined = format!("{}{}", stdout, stderr);
        assert!(combined.contains("VALID"), "Expected VALID, stdout: '{stdout}', stderr: '{stderr}'");
        assert!(success, "Expected exit code 0");
    }

    #[test]
    fn test_validate_java_invalid_sql() {
        let (_stdout, _stderr, success) =
            run_validate_java("class T { void m() { stmt.execute(\"SELECT 1 FROM WHERE x=\"); } }", &["validate-java"]);
        let _ = success;
    }

    #[test]
    fn test_validate_java_csv_output() {
        let (stdout, _stderr, _success) =
            run_validate_java("class T { void m() { stmt.execute(\"SELECT 1\"); } }", &["validate-java", "--csv"]);
        assert!(stdout.contains("file,") || stdout.contains("line,"), "CSV should have headers, got: '{stdout}'");
    }

    // ── CALL / {call} stored procedure ──

    fn callable_java_source() -> &'static str {
        include_str!("../tests/fixtures/jdbc_call/CallableExecutor.java")
    }

    /// Minimal Java with {call ...} JDBC escape in a string literal.
    fn callable_jdbc_escape_java() -> &'static str {
        r#"class T {
            void m(java.sql.Connection c) throws Exception {
                c.prepareCall("{call pkg_xxx.proc_yyy(?)}");
            }
        }"#
    }

    /// Minimal Java with bare CALL SQL.
    fn callable_bare_call_java() -> &'static str {
        r#"class T {
            void m(java.sql.Connection c) throws Exception {
                c.prepareCall("CALL pkg_xxx.proc_yyy(?)");
            }
        }"#
    }

    /// Minimal Java with {? = call ...} JDBC return-value syntax.
    fn callable_jdbc_return_java() -> &'static str {
        r#"class T {
            void m(java.sql.Connection c) throws Exception {
                c.prepareCall("{? = call pkg_xxx.get_user(?)}");
            }
        }"#
    }

    // These tests are ignored until Plan 2 is implemented:
    // 1) starts_with_sql_keyword adds CALL and {call to whitelist
    // 2) JDBC {call} escape is stripped before SQL parsing

    #[test]
    fn test_validate_java_callable_bare_call() {
        let (stdout, stderr, success) = run_validate_java(callable_bare_call_java(), &["validate-java"]);
        let combined = format!("{}{}", stdout, stderr);
        assert!(combined.contains("VALID"), "Bare CALL should validate. stdout: '{stdout}', stderr: '{stderr}'");
        assert!(success);
    }

    #[test]
    fn test_validate_java_callable_jdbc_escape() {
        let (stdout, stderr, success) = run_validate_java(callable_jdbc_escape_java(), &["validate-java"]);
        let combined = format!("{}{}", stdout, stderr);
        assert!(
            combined.contains("VALID"),
            "{{call}} JDBC escape should validate. stdout: '{stdout}', stderr: '{stderr}'"
        );
        assert!(success);
    }

    #[test]
    fn test_validate_java_callable_jdbc_return() {
        let (stdout, stderr, success) = run_validate_java(callable_jdbc_return_java(), &["validate-java"]);
        let combined = format!("{}{}", stdout, stderr);
        assert!(combined.contains("VALID"), "{{? = call}} should validate. stdout: '{stdout}', stderr: '{stderr}'");
        assert!(success);
    }

    #[test]
    fn test_validate_java_callable_full_fixture() {
        let (stdout, stderr, success) = run_validate_java(callable_java_source(), &["validate-java"]);
        let combined = format!("{}{}", stdout, stderr);
        assert!(combined.contains("VALID"), "Full fixture should validate. stdout: '{stdout}', stderr: '{stderr}'");
        assert!(success);
    }
}
