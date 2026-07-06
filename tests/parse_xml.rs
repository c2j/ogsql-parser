#[cfg(feature = "ibatis")]
mod parse_xml_tests {
    use std::io::Write;
    use std::process::{Command, Stdio};

    fn ogsql() -> Command {
        Command::new(env!("CARGO_BIN_EXE_ogsql"))
    }

    fn run_parse_xml(stdin_text: &str, args: &[&str]) -> (String, bool) {
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

    #[test]
    fn test_parse_xml_simple() {
        let (stdout, success) =
            run_parse_xml(r#"<mapper namespace="t"><select id="q">SELECT 1</select></mapper>"#, &["parse-xml"]);
        assert!(success);
        assert!(stdout.contains("q"));
        assert!(stdout.contains("Select"));
        assert!(stdout.contains("SELECT 1"));
        assert!(stdout.contains("Total:"));
    }

    #[test]
    fn test_parse_xml_complex_if_foreach() {
        // Regression: complex XML mapper with <if> + <foreach>, parameterType, resultType
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
        let (stdout, success) = run_parse_xml(xml, &["parse-xml"]);
        assert!(success, "parse-xml should succeed, stdout: {}", stdout);

        // Check statement metadata
        assert!(stdout.contains("getBranchLists"), "stdout: {}", stdout);
        assert!(stdout.contains("Select"), "stdout: {}", stdout);
        assert!(stdout.contains("com.example.BranchMapper"), "stdout: {}", stdout);

        // Check flattened SQL content
        assert!(stdout.contains("select amb.col1"), "stdout: {}", stdout);
        assert!(stdout.contains("from t1 amb,t2 gbm"), "stdout: {}", stdout);
        assert!(stdout.contains("amb.stru_id = gbm.stru_id"), "stdout: {}", stdout);

        // Check parameters in output (branchList is a foreach collection attribute,
        // not a #{param} placeholder, so it won't appear in the params list)
        assert!(stdout.contains("userCode"), "stdout: {}", stdout);
        assert!(stdout.contains("item"), "stdout: {}", stdout);

        // Check dynamic SQL element detection
        assert!(stdout.contains("dynamic SQL elements"), "stdout: {}", stdout);

        // Check summary line
        assert!(stdout.contains("Total:"), "stdout: {}", stdout);
    }

    #[test]
    fn test_parse_xml_complex_if_foreach_csv() {
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
        let (stdout, success) = run_parse_xml(xml, &["parse-xml", "--csv"]);
        assert!(success, "parse-xml --csv should succeed, stdout: {}", stdout);
        // CSV header
        assert!(stdout.contains("file,directory,line,method,sql"), "CSV header, got: {}", stdout);
        // CSV data row
        assert!(stdout.contains("getBranchLists"), "CSV should contain statement id, got: {}", stdout);
        assert!(stdout.contains("userCode__"), "CSV should contain variable, got: {}", stdout);
        assert!(stdout.contains("item__"), "CSV should contain variable, got: {}", stdout);
    }

    #[test]
    fn test_parse_xml_complex_if_foreach_structured() {
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
        let (stdout, success) = run_parse_xml(xml, &["parse-xml", "--structured"]);
        assert!(success, "parse-xml --structured should succeed, stdout: {}", stdout);

        // Structured output is JSON — verify key fields
        assert!(stdout.contains("\"namespace\""), "stdout: {}", stdout);
        assert!(stdout.contains("com.example.BranchMapper"), "stdout: {}", stdout);
        assert!(stdout.contains("getBranchLists"), "stdout: {}", stdout);
        assert!(stdout.contains("userCode !=null and userCode !='"), "stdout: {}", stdout);
        assert!(stdout.contains("branchList !=null and branchList !='"), "stdout: {}", stdout);
        assert!(stdout.contains("\"collection\": \"branchList\""), "stdout: {}", stdout);
        assert!(stdout.contains("\"item\": \"item\""), "stdout: {}", stdout);
        assert!(stdout.contains("\"index\": \"index\""), "stdout: {}", stdout);
        assert!(stdout.contains("\"open\": \"(\""), "stdout: {}", stdout);
        assert!(stdout.contains("\"separator\": \",\""), "stdout: {}", stdout);
        assert!(stdout.contains("\"close\": \")\""), "stdout: {}", stdout);
        assert!(stdout.contains("ForEach"), "stdout: {}", stdout);
    }

    #[test]
    fn test_parse_xml_complex_if_foreach_with_lint() {
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
        let (_stdout, success) = run_parse_xml(xml, &["parse-xml", "--lint"]);
        assert!(success, "parse-xml --lint should succeed for valid XML with if+foreach");
    }
}
