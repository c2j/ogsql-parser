use super::*;

    // ── Parameter deserialization tests ────────────────────────────────────

    #[test]
    fn test_parse_params_deserialization() {
        let json = r#"{"sql": "SELECT 1", "preserve_comments": true}"#;
        let params: ParseParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.sql, "SELECT 1");
        assert!(params.preserve_comments);
    }

    #[test]
    fn test_parse_params_default_preserve_comments() {
        let json = r#"{"sql": "SELECT 1"}"#;
        let params: ParseParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.sql, "SELECT 1");
        assert!(!params.preserve_comments);
    }

    #[test]
    fn test_tokenize_params_deserialization() {
        let json = r#"{"sql": "SELECT * FROM t"}"#;
        let params: TokenizeParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.sql, "SELECT * FROM t");
    }

    #[test]
    fn test_format_params_defaults() {
        let json = r#"{"sql": "select 1", "keyword_case": "", "comma_style": "", "uppercase": false}"#;
        let params: FormatParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.sql, "select 1");
        assert_eq!(params.indent, 2);
        assert_eq!(params.line_width, 120);
    }

    #[test]
    fn test_format_params_custom() {
        let json = r#"{"sql": "select 1", "indent": 4, "keyword_case": "upper", "comma_style": "leading", "line_width": 80, "uppercase": true}"#;
        let params: FormatParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.indent, 4);
        assert_eq!(params.line_width, 80);
        assert_eq!(params.keyword_case, "upper");
        assert_eq!(params.comma_style, "leading");
    }

    #[test]
    fn test_validate_params_deserialization() {
        let json = r#"{"sql": "SELECT * FROM"}"#;
        let params: ValidateParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.sql, "SELECT * FROM");
    }

    #[test]
    fn test_json2sql_params_deserialization() {
        let json = r#"{"json": "[]"}"#;
        let params: Json2SqlParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.json, "[]");
    }

    #[test]
    fn test_parse_xml_params_deserialization() {
        let json = r#"{"xml": "<mapper></mapper>"}"#;
        let params: ParseXmlParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.xml, "<mapper></mapper>");
    }

    #[test]
    fn test_parse_java_params_deserialization() {
        let json = r#"{"source": "class Foo {}", "extra_sql_methods": [], "extra_sql_var_patterns": []}"#;
        let params: ParseJavaParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.source, "class Foo {}");
        assert!(params.extra_sql_methods.is_empty());
        assert!(params.extra_sql_var_patterns.is_empty());
    }

    // ── Tool functionality tests ───────────────────────────────────────────

    #[test]
    fn test_parse_tool_valid_sql() {
        let server = OgsqlServer;
        let json = r#"{"sql": "SELECT 1", "preserve_comments": false}"#;
        let params: ParseParams = serde_json::from_str(json).unwrap();
        let result = server.parse(Parameters(params));
        assert!(result.contains("\"statements\""));
        assert!(result.contains("\"errors\""));
    }

    #[test]
    fn test_parse_tool_invalid_sql() {
        let server = OgsqlServer;
        let json = r#"{"sql": "BROKEN SYNTAX !!! @@@", "preserve_comments": false}"#;
        let params: ParseParams = serde_json::from_str(json).unwrap();
        let result = server.parse(Parameters(params));
        assert!(result.contains("\"statements\""));
        // Even invalid SQL should return a result (with errors array)
    }

    #[test]
    fn test_tokenize_tool() {
        let server = OgsqlServer;
        let json = r#"{"sql": "SELECT id FROM users"}"#;
        let params: TokenizeParams = serde_json::from_str(json).unwrap();
        let result = server.tokenize(Parameters(params));
        assert!(result.contains("\"tokens\""));
        assert!(result.contains("\"type\""));
    }

    #[test]
    fn test_format_tool() {
        let server = OgsqlServer;
        let json = r#"{"sql": "select id,name from users where id=1", "indent": 2, "keyword_case": "upper", "comma_style": "trailing", "line_width": 120, "uppercase": false}"#;
        let params: FormatParams = serde_json::from_str(json).unwrap();
        let result = server.format(Parameters(params));
        assert!(result.contains("\"formatted\""));
    }

    #[test]
    fn test_validate_tool_valid() {
        let server = OgsqlServer;
        let json = r#"{"sql": "SELECT 1"}"#;
        let params: ValidateParams = serde_json::from_str(json).unwrap();
        let result = server.validate(Parameters(params));
        assert!(result.contains("\"valid\""));
    }

    #[test]
    fn test_validate_tool_invalid() {
        let server = OgsqlServer;
        let json = r#"{"sql": "BROKEN !!! @@@ SYNTAX"}"#;
        let params: ValidateParams = serde_json::from_str(json).unwrap();
        let result = server.validate(Parameters(params));
        assert!(result.contains("\"valid\""));
        assert!(result.contains("\"errors\""));
    }

    #[test]
    fn test_json2sql_tool_bad_json() {
        let server = OgsqlServer;
        let json = r#"{"json": "not valid json at all {{{"}"#;
        let params: Json2SqlParams = serde_json::from_str(json).unwrap();
        let result = server.json2sql(Parameters(params));
        assert!(result.contains("\"error\""));
    }

    // ── Helper function tests ──────────────────────────────────────────────

    #[test]
    fn test_is_warning() {
        let warning = crate::ParserError::Warning {
            message: "test".to_string(),
            location: crate::SourceLocation::default(),
        };
        assert!(is_warning(&warning));

        let error = crate::ParserError::UnexpectedEof {
            expected: "stmt".to_string(),
            location: crate::SourceLocation::default(),
        };
        assert!(!is_warning(&error));
    }

    #[test]
    fn test_token_display_keyword() {
        let tok = crate::TokenWithSpan {
            token: crate::Token::Keyword(crate::Keyword::SELECT),
            span: crate::Span { start: 0, end: 6 },
            location: crate::SourceLocation::default(),
        };
        let (ty, val) = token_display(&tok);
        assert_eq!(ty, "Keyword");
        assert!(val.contains("SELECT"));
    }

    #[test]
    fn test_token_display_ident() {
        let tok = crate::TokenWithSpan {
            token: crate::Token::Ident("my_col".to_string()),
            span: crate::Span { start: 0, end: 6 },
            location: crate::SourceLocation::default(),
        };
        let (ty, val) = token_display(&tok);
        assert_eq!(ty, "Ident");
        assert_eq!(val, "my_col");
    }

    #[test]
    fn test_token_display_integer() {
        let tok = crate::TokenWithSpan {
            token: crate::Token::Integer(42),
            span: crate::Span { start: 0, end: 2 },
            location: crate::SourceLocation::default(),
        };
        let (ty, val) = token_display(&tok);
        assert_eq!(ty, "Integer");
        assert_eq!(val, "42");
    }
