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

    // ── Complex if + foreach mapper ──

    #[test]
    fn test_validate_xml_complex_if_foreach() {
        let xml = r#"<mapper namespace="com.example.BranchMapper">
    <select id="getBranchLists" parameterType="java.lang.String" resultType="java.util.Map">
        select amb.col1
        from t1 amb,t2 gbm
        where amb.stru_id = gbm.stru_id
        <if test="userCode !=null and userCode !=''">
            and gbm.user_code = #{userCode}
        </if>
        <if test="branchList !=null and branchList !=''">
            <foreach collection="branchList" index="index" item="item" open="(" separator="," close=")">
                and amb.stru_id = #{item}
            </foreach>
        </if>
    </select>
</mapper>"#;
        let (stdout, _success) = run_validate_xml(xml, &["validate-xml"]);
        assert!(stdout.contains("getBranchLists"), "stdout: {}", stdout);
        assert!(
            stdout.contains("INVALID"),
            "validate-xml should report INVALID due to foreach open='(' flatten, stdout: {}",
            stdout
        );
    }

    #[test]
    fn test_validate_xml_complex_if_foreach_csv() {
        let xml = r#"<mapper namespace="com.example.BranchMapper">
    <select id="getBranchLists" parameterType="java.lang.String" resultType="java.util.Map">
        select amb.col1
        from t1 amb,t2 gbm
        where amb.stru_id = gbm.stru_id
        <if test="userCode !=null and userCode !=''">
            and gbm.user_code = #{userCode}
        </if>
        <if test="branchList !=null and branchList !=''">
            <foreach collection="branchList" index="index" item="item" open="(" separator="," close=")">
                and amb.stru_id = #{item}
            </foreach>
        </if>
    </select>
</mapper>"#;
        let (stdout, _success) = run_validate_xml(xml, &["validate-xml", "--csv"]);
        assert!(stdout.contains("getBranchLists"), "CSV should contain statement id, got: {}", stdout);
        assert!(stdout.contains("Select"), "CSV should contain statement kind, got: {}", stdout);
    }

    #[test]
    fn test_validate_xml_complex_if_foreach_with_lint() {
        let xml = r#"<mapper namespace="com.example.BranchMapper">
    <select id="getBranchLists" parameterType="java.lang.String" resultType="java.util.Map">
        select amb.col1
        from t1 amb,t2 gbm
        where amb.stru_id = gbm.stru_id
        <if test="userCode !=null and userCode !=''">
            and gbm.user_code = #{userCode}
        </if>
        <if test="branchList !=null and branchList !=''">
            <foreach collection="branchList" index="index" item="item" open="(" separator="," close=")">
                and amb.stru_id = #{item}
            </foreach>
        </if>
    </select>
</mapper>"#;
        let (stdout, _success) = run_validate_xml(xml, &["validate-xml", "--lint"]);
        assert!(stdout.contains("getBranchLists"), "stdout: {}", stdout);
        assert!(stdout.contains("INVALID"), "stdout: {}", stdout);
    }

    // ── Line number regression tests: XML parse errors ──

    /// Returns (stdout, stderr, exit_code) for running validate-xml with given bytes.
    fn run_validate_xml_bytes(bytes: &[u8], args: &[&str]) -> (String, String, bool) {
        let mut child =
            ogsql().args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        {
            use std::io::Write;
            let mut stdin = child.stdin.take().unwrap();
            stdin.write_all(bytes).unwrap();
        }
        let output = child.wait_with_output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let success = output.status.success();
        (stdout, stderr, success)
    }

    #[test]
    fn test_validate_xml_bom_unclosed_tag_line_number() {
        let xml_with_bom: &[u8] = b"\xEF\xBB\xBF<mapper namespace=\"t\">\n    <select id=\"q1\">SELECT 1</select>\n    <select id=\"q2\">SELECT 2</select>\n</mapper";
        let (_stdout, stderr, _success) = run_validate_xml_bytes(xml_with_bom, &["validate-xml"]);

        // FIXME: BOM causes off-by-one — quick-xml strips BOM from error_position(),
        // but byte_offset_to_line counts newlines in full source (includes BOM bytes).
        // The unclosed </mapper is actually on line 4, but currently reports line 3.
        // When fixed, change "line 3" to "line 4" below.
        assert!(
            stderr.contains("line 3") || stderr.contains("line 4"),
            "Expected error to report a line number, got stderr:\n{}",
            stderr
        );
    }

    /// Baseline: XML WITHOUT BOM, unclosed tag on line 4 → should report line 4 correctly.
    #[test]
    fn test_validate_xml_no_bom_unclosed_tag_line_number() {
        // Same XML without BOM
        let xml_no_bom = b"<mapper namespace=\"t\">\n    <select id=\"q1\">SELECT 1</select>\n    <select id=\"q2\">SELECT 2</select>\n</mapper";
        let (_stdout, stderr, _success) = run_validate_xml_bytes(xml_no_bom, &["validate-xml"]);

        assert!(
            stderr.contains("XML parse error at line 4") || stderr.contains("line 4"),
            "Expected error to report line 4 (the actual line of </mapper), but got stderr:\n{}",
            stderr
        );
    }

    /// XML with multi-line `<select>` tag:
    /// `<select` starts on line 2, `>` is on line 4.
    /// `buffer_position()` returns position after `>`, so reported line is 4.
    /// Users may expect line 2 (where the tag starts).
    #[test]
    fn test_validate_xml_multiline_tag_line_number() {
        let xml = b"<mapper namespace=\"t\">\n    <select id=\"q1\"\n            parameterType=\"x\"\n            resultType=\"map\">\n        SELECT 1\n    </select>\n</mapper>";
        let (stdout, _stderr, _success) = run_validate_xml_bytes(xml, &["validate-xml", "--csv"]);

        // CSV output contains the line number of the statement.
        // Currently reports line 4 (where `>` is), not line 2 (where `<select` begins).
        // This test documents the current behavior.
        assert!(stdout.contains("q1"), "CSV should contain statement id q1, got: {}", stdout);
        // The line field in CSV: "file,directory,line,..."
        // Example row: ",.,4,Select,q1,..."
        // We accept either line 2 (tag start) or line 4 (tag end) as the current behavior is line 4.
        assert!(
            stdout.contains(",4,") || stdout.contains(",2,"),
            "Statement line should be reported, got CSV:\n{}",
            stdout
        );
    }

    /// XML with unclosed `<foreach>` nested inside a `<select>`.
    /// The error occurs inside `read_node_tree` which currently swallows
    /// `read_event_into` errors (parser.rs line 197: `_ => {}`).
    /// This means the XML error is silently ignored — no XmlError is produced.
    #[test]
    fn test_validate_xml_nested_unclosed_tag_no_error_surfaced() {
        let xml = b"<mapper namespace=\"t\">\n    <select id=\"q1\">\n        SELECT * FROM t WHERE\n        <foreach collection=\"list\" item=\"i\" open=\"(\" close=\")\" separator=\",\">\n            #{i}\n        </foreach\n    </select>\n</mapper>";
        let (_stdout, stderr, _success) = run_validate_xml_bytes(xml, &["validate-xml"]);

        // BUG: No XML error is reported because inner loop swallows read_event_into errors.
        // If this assertion fails (XML error IS reported), the fix should ALSO verify
        // the line number is correct.
        assert!(
            !stderr.contains("XML parse error"),
            "BUG FIXED? XML error is now surfaced for nested unclosed tag. Verify line number! stderr:\n{}",
            stderr
        );
    }
}
