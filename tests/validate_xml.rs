#[cfg(feature = "ibatis")]
mod validate_xml_tests {
    use std::io::Write;
    use std::process::{Command, Stdio};

    fn ogsql() -> Command {
        Command::new(env!("CARGO_BIN_EXE_ogsql"))
    }

    fn run_validate_xml(stdin_text: &str, args: &[&str]) -> (String, bool) {
        let mut child =
            ogsql().args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        {
            let mut stdin = child.stdin.take().unwrap();
            stdin.write_all(stdin_text.as_bytes()).unwrap();
        }
        let output = child.wait_with_output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let success = output.status.success();
        (stdout, success)
    }

    // ── basic validate-xml ──

    #[test]
    fn test_validate_xml_valid_simple() {
        let (stdout, success) =
            run_validate_xml(r#"<mapper namespace="t"><select id="q">SELECT 1</select></mapper>"#, &["validate-xml"]);
        assert!(stdout.contains("VALID"), "Expected VALID, got: {}", stdout);
        assert!(success);
    }

    #[test]
    fn test_validate_xml_invalid_sql() {
        let (_stdout, success) = run_validate_xml(
            r#"<mapper namespace="t"><select id="q">INVALID SYNTAX !!!</select></mapper>"#,
            &["validate-xml"],
        );
        assert!(!success, "Expected non-zero exit code for truly invalid SQL");
    }

    #[test]
    fn test_validate_xml_csv_output() {
        let (stdout, _success) = run_validate_xml(
            r#"<mapper namespace="t"><select id="q">SELECT 1</select></mapper>"#,
            &["validate-xml", "--csv"],
        );
        assert!(stdout.contains("VALID"), "CSV should contain VALID");
    }

    #[test]
    fn test_validate_xml_with_lint() {
        let (_stdout, success) = run_validate_xml(
            r#"<mapper namespace="t"><select id="q">SELECT * FROM t</select></mapper>"#,
            &["validate-xml", "--lint"],
        );
        assert!(success, "Lint should not cause failure for valid SQL");
    }

    // ── CALLABLE stored procedure ──

    /// The full callable_mapper.xml fixture with 5 CALLABLE statements.
    fn callable_mapper_xml() -> &'static str {
        include_str!("../tests/fixtures/jdbc_call/callable_mapper.xml")
    }

    /// Fixture trimmed to only statements that use bare `call` (no JDBC {call} escape).
    /// These should pass NOW — the flat SQL starts with the CALL keyword.
    fn callable_bare_call_xml() -> &'static str {
        r#"<mapper namespace="t">
            <select id="callProc" statementType="CALLABLE" resultType="java.util.Map">
                call pkg_xxx.proc_yyy(#{p_id,mode=IN,jdbcType=INTEGER}, #{p_name,mode=IN,jdbcType=VARCHAR})
            </select>
        </mapper>"#
    }

    /// Fixture with {call} JDBC escape syntax.
    /// These require Plan 1 (translate_jdbc_call in ibatis flatten pipeline).
    fn callable_jdbc_escape_xml() -> &'static str {
        r#"<mapper namespace="t">
            <select id="callProcJdbc" statementType="CALLABLE">
                {call pkg_xxx.proc_yyy(#{p_id,mode=IN,jdbcType=INTEGER}, #{p_name,mode=IN,jdbcType=VARCHAR})}
            </select>
        </mapper>"#
    }

    /// Fixture with {? = call ...} JDBC return-value syntax.
    /// Requires Plan 1.
    fn callable_jdbc_return_xml() -> &'static str {
        r#"<mapper namespace="t">
            <select id="callFuncWithReturn" statementType="CALLABLE">
                {? = call pkg_xxx.get_user(#{p_id,mode=IN,jdbcType=INTEGER})}
            </select>
        </mapper>"#
    }

    #[test]
    fn test_validate_xml_callable_bare_call() {
        let (stdout, success) = run_validate_xml(callable_bare_call_xml(), &["validate-xml"]);
        assert!(stdout.contains("VALID"), "Bare CALL should validate. Got: {}", stdout);
        assert!(success);
    }

    #[test]
    fn test_validate_xml_callable_csv() {
        let (stdout, _success) = run_validate_xml(callable_bare_call_xml(), &["validate-xml", "--csv"]);
        assert!(stdout.contains("VALID"), "CSV should report VALID for bare CALL. Got: {}", stdout);
    }

    #[test]
    fn test_validate_xml_callable_with_lint() {
        let (_stdout, success) = run_validate_xml(callable_bare_call_xml(), &["validate-xml", "--lint"]);
        assert!(success, "Lint should not fail on CALLABLE");
    }

    // ── Full callable_mapper.xml (includes {call} JDBC escape) ──

    #[test]
    fn test_validate_xml_callable_jdbc_escape() {
        let (stdout, success) = run_validate_xml(callable_jdbc_escape_xml(), &["validate-xml"]);
        assert!(stdout.contains("VALID"), "{{call}} JDBC escape should validate. Got: {}", stdout);
        assert!(success);
    }

    #[test]
    fn test_validate_xml_callable_jdbc_return() {
        let (stdout, success) = run_validate_xml(callable_jdbc_return_xml(), &["validate-xml"]);
        assert!(stdout.contains("VALID"), "{{? = call}} should validate. Got: {}", stdout);
        assert!(success);
    }

    #[test]
    fn test_validate_xml_callable_full_fixture() {
        let (stdout, success) = run_validate_xml(callable_mapper_xml(), &["validate-xml"]);
        assert!(stdout.contains("VALID"), "Full callable fixture should validate. Got: {}", stdout);
        assert!(success);
    }
}
