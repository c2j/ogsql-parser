use crate::token::{Keyword, Token, TokenWithSpan};

pub struct TokenFormatter<'a> {
    source: &'a str,
    tokens: Vec<TokenWithSpan>,
    pos: usize,
    indent_stack: Vec<IndentKind>,
    needs_line: bool,
    output: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IndentKind {
    Begin,
    If,
    Loop,
    Case,
    Select,
}

impl<'a> TokenFormatter<'a> {
    pub fn new(source: &'a str, tokens: Vec<TokenWithSpan>) -> Self {
        Self {
            source,
            tokens,
            pos: 0,
            indent_stack: Vec::new(),
            needs_line: false,
            output: String::new(),
        }
    }

    pub fn format(mut self) -> String {
        while self.pos < self.tokens.len() {
            let tws = &self.tokens[self.pos];
            match &tws.token {
                Token::Eof => break,
                Token::Comment(ref s) => {
                    let comment = s.clone();
                    if self.needs_line {
                        self.flush_pending_line();
                    } else if !self.output.is_empty() {
                        self.output.push('\n');
                        self.emit_indent();
                    }
                    self.output.push_str(&comment);
                    self.needs_line = true;
                    self.pos += 1;
                }
                _ => {
                    self.handle_token();
                }
            }
        }
        self.output
    }

    fn flush_pending_line(&mut self) {
        self.output.push('\n');
        self.emit_indent();
        self.needs_line = false;
    }

    fn handle_token(&mut self) {
        let token = &self.tokens[self.pos].token;
        let next_token = self.peek_token(1);

        match token {
            Token::Keyword(Keyword::BEGIN_P) => {
                self.emit_line_start();
                self.emit_current_token();
                self.indent_stack.push(IndentKind::Begin);
                self.needs_line = true;
                self.pos += 1;
            }

            Token::Keyword(Keyword::THEN) => {
                self.emit_space();
                self.emit_current_token();
                self.indent_stack.push(IndentKind::If);
                self.needs_line = true;
                self.pos += 1;
            }

            Token::Keyword(Keyword::LOOP) => {
                let prev_token = self.peek_token_back(1);
                if !matches!(prev_token, Some(Token::Keyword(Keyword::END_P))) {
                    self.emit_line_start();
                    self.emit_current_token();
                    self.indent_stack.push(IndentKind::Loop);
                    self.needs_line = true;
                    self.pos += 1;
                }
            }

            Token::Keyword(Keyword::END_P) => {
                match next_token {
                    Some(Token::Keyword(Keyword::IF_P)) => {
                        self.pop_indent_to(IndentKind::If);
                        self.emit_line_start();
                        self.emit_current_token();
                        self.pos += 1;
                        self.emit_space();
                        self.emit_current_token();
                        self.pos += 1;
                    }
                    Some(Token::Keyword(Keyword::LOOP)) => {
                        self.pop_indent_to(IndentKind::Loop);
                        self.emit_line_start();
                        self.emit_current_token();
                        self.pos += 1;
                        self.emit_space();
                        self.emit_current_token();
                        self.pos += 1;
                    }
                    Some(Token::Keyword(Keyword::CASE)) => {
                        self.pop_indent_to(IndentKind::Case);
                        self.emit_line_start();
                        self.emit_current_token();
                        self.pos += 1;
                        self.emit_space();
                        self.emit_current_token();
                        self.pos += 1;
                    }
                    _ => {
                        self.pop_indent_to(IndentKind::Begin);
                        self.emit_line_start();
                        self.emit_current_token();
                        self.pos += 1;
                    }
                }
            }

            Token::Ident(name) if name.to_uppercase() == "EXCEPTION" => {
                self.pop_indent_to(IndentKind::Begin);
                self.emit_line_start();
                self.emit_current_token();
                self.indent_stack.push(IndentKind::Begin);
                self.needs_line = true;
                self.pos += 1;
            }

            Token::Keyword(Keyword::WHEN) => {
                self.emit_line_start();
                self.emit_current_token();
                self.pos += 1;
            }

            Token::Keyword(Keyword::ELSE) => {
                self.emit_line_start();
                self.emit_current_token();
                self.needs_line = true;
                self.pos += 1;
            }
            Token::Ident(name) if name.to_uppercase() == "ELSIF" => {
                self.pop_indent_to(IndentKind::If);
                self.emit_line_start();
                self.emit_current_token();
                self.needs_line = true;
                self.pos += 1;
            }

            Token::Semicolon => {
                self.emit_current_token();
                self.needs_line = true;
                self.pos += 1;
            }

            Token::Keyword(Keyword::SELECT) => {
                self.emit_line_start();
                self.emit_current_token();
                self.indent_stack.push(IndentKind::Select);
                self.pos += 1;
            }
            Token::Keyword(Keyword::FROM)
            | Token::Keyword(Keyword::WHERE)
            | Token::Keyword(Keyword::GROUP_P)
            | Token::Keyword(Keyword::HAVING)
            | Token::Keyword(Keyword::ORDER)
            | Token::Keyword(Keyword::LIMIT)
            | Token::Keyword(Keyword::OFFSET)
            | Token::Keyword(Keyword::UNION)
            | Token::Keyword(Keyword::INTERSECT)
            | Token::Keyword(Keyword::EXCEPT) => {
                if self.in_select_context() {
                    self.pop_indent_to(IndentKind::Select);
                }
                self.emit_line_start();
                self.emit_current_token();
                self.pos += 1;
            }

            Token::Keyword(Keyword::PROCEDURE) | Token::Keyword(Keyword::FUNCTION) => {
                self.emit_line_start();
                self.emit_current_token();
                self.pos += 1;
            }

            Token::Keyword(Keyword::IS) | Token::Keyword(Keyword::AS) => {
                self.emit_space();
                self.emit_current_token();
                self.pos += 1;
                if self.is_procedure_or_function_context() {
                    self.needs_line = true;
                }
            }

            Token::Keyword(Keyword::IF_P) => {
                self.emit_line_start();
                self.emit_current_token();
                self.pos += 1;
            }

            _ => {
                self.emit_default_token();
            }
        }
    }

    fn emit_line_start(&mut self) {
        if self.needs_line {
            self.flush_pending_line();
        } else if !self.output.is_empty() {
            self.output.push('\n');
            self.emit_indent();
        }
    }

    fn emit_current_token(&mut self) {
        let tws = &self.tokens[self.pos];
        let text = &self.source[tws.span.start..tws.span.end];
        self.output.push_str(text);
    }

    fn emit_space(&mut self) {
        if !self.output.ends_with(' ') && !self.output.ends_with('\n') {
            self.output.push(' ');
        }
    }

    fn emit_indent(&mut self) {
        let spaces = self.indent_stack.len() * 2;
        for _ in 0..spaces {
            self.output.push(' ');
        }
    }

    fn peek_token(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.pos + offset).map(|t| &t.token)
    }

    fn peek_token_back(&self, offset: usize) -> Option<&Token> {
        if self.pos >= offset {
            self.tokens.get(self.pos - offset).map(|t| &t.token)
        } else {
            None
        }
    }

    fn pop_indent_to(&mut self, kind: IndentKind) {
        while let Some(top) = self.indent_stack.last() {
            if *top == kind {
                self.indent_stack.pop();
                break;
            } else {
                self.indent_stack.pop();
            }
        }
    }

    fn in_select_context(&self) -> bool {
        self.indent_stack.iter().any(|k| *k == IndentKind::Select)
    }

    fn is_procedure_or_function_context(&self) -> bool {
        let mut i = self.pos;
        while i > 0 {
            i -= 1;
            if let Some(tws) = self.tokens.get(i) {
                match &tws.token {
                    Token::Keyword(Keyword::PROCEDURE) | Token::Keyword(Keyword::FUNCTION) => {
                        return true;
                    }
                    Token::Keyword(Keyword::BEGIN_P) | Token::Keyword(Keyword::DECLARE) => {
                        return false;
                    }
                    _ => {}
                }
            }
        }
        false
    }

    fn emit_default_token(&mut self) {
        let tws = &self.tokens[self.pos];
        let is_space_rejecting = matches!(
            &tws.token,
            Token::Comma
                | Token::Semicolon
                | Token::RParen
                | Token::RBracket
                | Token::Dot
        );
        let prev_rejects_space = self.pos > 0 && {
            let prev = &self.tokens[self.pos - 1].token;
            matches!(
                prev,
                Token::LParen | Token::LBracket | Token::Comma | Token::Dot
            )
        };
        let text = self.source[tws.span.start..tws.span.end].to_string();
        let pos = self.pos;
        let _ = tws;

        if self.needs_line {
            self.flush_pending_line();
        }

        if !is_space_rejecting && !prev_rejects_space && !self.output.ends_with(' ') && !self.output.ends_with('\n') {
            self.output.push(' ');
        }

        self.output.push_str(&text);
        self.pos = pos + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn format_sql(input: &str) -> String {
        let tokens = crate::Tokenizer::new(input)
            .preserve_comments(true)
            .tokenize()
            .unwrap();
        TokenFormatter::new(input, tokens).format()
    }

    #[test]
    fn test_simple_select_preserves_content() {
        let input = "SELECT id, name FROM users WHERE id = 1";
        let output = format_sql(input);
        assert_eq!(output.replace(char::is_whitespace, ""), input.replace(char::is_whitespace, ""));
    }

    #[test]
    fn test_preserves_quoted_identifiers() {
        let input = r#"SELECT "BIGFUND"."PKG_BM_2" FROM dual"#;
        let output = format_sql(input);
        assert!(output.contains(r#""BIGFUND""#), "Quoted identifier should stay quoted");
        assert!(output.contains(r#""PKG_BM_2""#), "Quoted identifier should stay quoted");
    }

    #[test]
    fn test_preserves_unquoted_identifiers() {
        let input = "SELECT BIGFUND.PKG_BM_2 FROM dual";
        let output = format_sql(input);
        assert!(output.contains("BIGFUND.PKG_BM_2"), "Unquoted should stay unquoted");
        assert!(!output.contains(r#""BIGFUND""#), "Should NOT add quotes to unquoted identifiers");
    }

    #[test]
    fn test_preserves_single_line_comment() {
        let input = "SELECT -- this is a comment\na FROM t";
        let output = format_sql(input);
        assert!(output.contains("-- this is a comment"), "Single-line comment should be preserved");
    }

    #[test]
    fn test_preserves_block_comment() {
        let input = "SELECT /* block comment */ a FROM t";
        let output = format_sql(input);
        assert!(output.contains("/* block comment */"), "Block comment should be preserved");
    }

    #[test]
    fn test_begin_end_indentation() {
        let input = "BEGIN p_out := 0; END";
        let output = format_sql(input);
        assert!(output.contains("BEGIN\n  p_out := 0;\nEND"), "BEGIN body should be indented, got: {:?}", output);
    }

    #[test]
    fn test_nested_begin_end() {
        let input = "BEGIN BEGIN x := 1; END; END";
        let output = format_sql(input);
        assert!(output.contains("BEGIN\n    x := 1;\n  END"), "Nested block should be doubly indented");
    }

    #[test]
    fn test_exception_block() {
        let input = "BEGIN x := 1; EXCEPTION WHEN OTHERS THEN x := 0; END";
        let output = format_sql(input);
        assert!(output.contains("EXCEPTION\n  WHEN OTHERS THEN\n    x := 0;"));
    }

    #[test]
    fn test_if_then_end_if() {
        let input = "IF x > 0 THEN y := 1; END IF";
        let output = format_sql(input);
        assert!(output.contains("IF x > 0 THEN\n  y := 1;\nEND IF"));
    }

    #[test]
    fn test_loop_end_loop() {
        let input = "LOOP x := x + 1; END LOOP";
        let output = format_sql(input);
        assert!(output.contains("LOOP\n  x := x + 1;\nEND LOOP"));
    }

    #[test]
    fn test_preserves_end_label() {
        let input = "END pkg_batchpay_management_2";
        let output = format_sql(input);
        assert!(output.contains("pkg_batchpay_management_2"), "End label should be preserved");
    }

    #[test]
    fn test_string_literals_preserved() {
        let input = "SELECT 'hello world' FROM dual";
        let output = format_sql(input);
        assert!(output.contains("'hello world'"), "String literal should be preserved exactly");
    }

    #[test]
    fn test_keyword_casing_preserved() {
        let input = "select id from users";
        let output = format_sql(input);
        assert!(output.contains("select"), "Lowercase keyword should stay lowercase");
        assert!(!output.contains("SELECT"), "Should NOT uppercase keywords");
    }
}

