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
}
