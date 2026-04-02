use super::keyword::lookup_keyword;
use super::{Span, Token, TokenWithSpan};

#[derive(Debug, thiserror::Error)]
pub enum TokenizerError {
    #[error("unterminated string literal at position {0}")]
    UnterminatedString(usize),
    #[error("unterminated block comment at position {0}")]
    UnterminatedComment(usize),
    #[error("unterminated dollar-quoted string at position {0}")]
    UnterminatedDollarString(usize),
    #[error("unterminated quoted identifier at position {0}")]
    UnterminatedQuotedIdentifier(usize),
    #[error("invalid character {0:?} at position {1}")]
    InvalidCharacter(char, usize),
}

pub struct Tokenizer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars().peekable(),
            pos: 0,
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<TokenWithSpan>, TokenizerError> {
        let mut tokens = Vec::new();
        loop {
            match self.next_token()? {
                Some(t) => tokens.push(t),
                None => {
                    tokens.push(TokenWithSpan {
                        token: Token::Eof,
                        span: Span {
                            start: self.pos,
                            end: self.pos,
                        },
                    });
                    return Ok(tokens);
                }
            }
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next();
        if let Some(c) = c {
            self.pos += c.len_utf8();
        }
        c
    }

    fn advance_while<F: Fn(char) -> bool>(&mut self, predicate: F) -> String {
        let mut s = String::new();
        while let Some(&c) = self.chars.peek() {
            if predicate(c) {
                self.chars.next();
                self.pos += c.len_utf8();
                s.push(c);
            } else {
                break;
            }
        }
        s
    }

    fn skip_whitespace_and_comments(&mut self) -> Result<(), TokenizerError> {
        loop {
            match self.peek() {
                None => return Ok(()),
                Some(c) if c.is_whitespace() => {
                    self.advance();
                }
                Some('-') => {
                    self.advance();
                    if self.peek() == Some('-') {
                        self.advance();
                        self.advance_while(|c| c != '\n');
                    } else {
                        self.pos -= 1;
                        return Ok(());
                    }
                }
                Some('/') => {
                    let saved_pos = self.pos;
                    self.advance();
                    if self.peek() == Some('*') {
                        self.advance();
                        self.skip_block_comment(saved_pos)?;
                    } else {
                        self.pos = saved_pos;
                        return Ok(());
                    }
                }
                _ => return Ok(()),
            }
        }
    }

    fn skip_block_comment(&mut self, start: usize) -> Result<(), TokenizerError> {
        let mut depth = 1;
        while depth > 0 {
            match self.advance() {
                None => return Err(TokenizerError::UnterminatedComment(start)),
                Some('/') => {
                    if self.peek() == Some('*') {
                        self.advance();
                        depth += 1;
                    }
                }
                Some('*') => {
                    if self.peek() == Some('/') {
                        self.advance();
                        depth -= 1;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn next_token(&mut self) -> Result<Option<TokenWithSpan>, TokenizerError> {
        self.skip_whitespace_and_comments()?;

        let start = self.pos;
        let c = match self.peek() {
            None => return Ok(None),
            Some(c) => c,
        };

        let token = match c {
            // String literal (single quote)
            '\'' => {
                self.advance();
                let s = self.scan_string()?;
                Token::StringLiteral(s)
            }

            // Quoted identifier (double quote)
            '"' => {
                self.advance();
                let s = self.scan_quoted_identifier()?;
                Token::QuotedIdent(s)
            }

            // Dollar-quoted string or parameter ($1, $2) or $tag$ string
            '$' => {
                self.advance();
                self.scan_dollar_or_param()
            }

            // Numbers
            '0'..='9' => self.scan_number(),

            // National character string N'...'
            'n' | 'N' => {
                // Check if followed by single quote
                let mut chars_clone = self.chars.clone();
                chars_clone.next(); // skip current
                if chars_clone.next() == Some('\'') {
                    self.advance(); // consume 'N'
                    self.advance(); // consume '\''
                    let s = self.scan_string()?;
                    Token::NationalString(s)
                } else {
                    self.scan_ident_or_keyword()
                }
            }

            // Escape string E'...'
            'e' | 'E' => {
                let mut chars_clone = self.chars.clone();
                chars_clone.next();
                if chars_clone.next() == Some('\'') {
                    self.advance();
                    self.advance();
                    let s = self.scan_escape_string()?;
                    Token::EscapeString(s)
                } else {
                    self.scan_ident_or_keyword()
                }
            }

            // Bit string B'...' or b'...'
            'b' | 'B' => {
                let mut chars_clone = self.chars.clone();
                chars_clone.next();
                if chars_clone.next() == Some('\'') {
                    self.advance();
                    self.advance();
                    let s = self.scan_string()?;
                    Token::BitString(s)
                } else {
                    self.scan_ident_or_keyword()
                }
            }

            // Hex string X'...' or x'...'
            'x' | 'X' => {
                let mut chars_clone = self.chars.clone();
                chars_clone.next();
                if chars_clone.next() == Some('\'') {
                    self.advance();
                    self.advance();
                    let s = self.scan_string()?;
                    Token::HexString(s)
                } else {
                    self.scan_ident_or_keyword()
                }
            }

            // @@ session/global variable
            '@' => {
                self.advance();
                if self.peek() == Some('@') {
                    self.advance();
                    let ident = self
                        .advance_while(|c| c.is_alphanumeric() || c == '_' || c == '.' || c == '$');
                    Token::SetIdent(ident)
                } else {
                    Token::At
                }
            }

            // Operators and punctuation
            '(' => {
                self.advance();
                Token::LParen
            }
            ')' => {
                self.advance();
                Token::RParen
            }
            '[' => {
                self.advance();
                Token::LBracket
            }
            ']' => {
                self.advance();
                Token::RBracket
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            ';' => {
                self.advance();
                Token::Semicolon
            }
            '%' => {
                self.advance();
                Token::Percent
            }
            '^' => {
                self.advance();
                Token::Caret
            }

            ':' => {
                self.advance();
                if self.peek() == Some(':') {
                    self.advance();
                    Token::Typecast
                } else if self.peek() == Some('=') {
                    self.advance();
                    Token::ColonEquals
                } else {
                    Token::Colon
                }
            }

            '.' => {
                self.advance();
                // Check for ".." (range operator)
                if self.peek() == Some('.') {
                    self.advance();
                    Token::DotDot
                } else if self.peek().map_or(false, |c| c.is_ascii_digit()) {
                    // Number starting with dot: .123
                    let frac = self.advance_while(|c| c.is_ascii_digit());
                    // Check for exponent
                    let mut full = format!(".{}", frac);
                    if self.peek() == Some('e') || self.peek() == Some('E') {
                        full.push(self.advance().unwrap());
                        if self.peek() == Some('+') || self.peek() == Some('-') {
                            full.push(self.advance().unwrap());
                        }
                        full.push_str(&self.advance_while(|c| c.is_ascii_digit()));
                    }
                    // Check for f/d suffix
                    if self.peek() == Some('f')
                        || self.peek() == Some('F')
                        || self.peek() == Some('d')
                        || self.peek() == Some('D')
                    {
                        self.advance();
                    }
                    Token::Float(full)
                } else {
                    Token::Dot
                }
            }

            '+' => {
                self.advance();
                Token::Plus
            }

            '-' => {
                self.advance();
                Token::Minus
            }

            '*' => {
                self.advance();
                Token::Star
            }
            '/' => {
                self.advance();
                Token::Slash
            }

            '<' => {
                self.advance();
                match self.peek() {
                    Some('=') => {
                        self.advance();
                        Token::Op("<=".to_string())
                    }
                    Some('>') => {
                        self.advance();
                        Token::Op("<>".to_string())
                    }
                    Some('<') => {
                        self.advance();
                        Token::Op("<<".to_string())
                    }
                    _ => Token::Lt,
                }
            }

            '>' => {
                self.advance();
                match self.peek() {
                    Some('=') => {
                        self.advance();
                        Token::Op(">=".to_string())
                    }
                    Some('>') => {
                        self.advance();
                        Token::Op(">>".to_string())
                    }
                    _ => Token::Gt,
                }
            }

            '=' => {
                self.advance();
                if self.peek() == Some('>') {
                    self.advance();
                    Token::ParamEquals
                } else {
                    Token::Eq
                }
            }

            '!' => {
                self.advance();
                if self.peek() == Some('=') {
                    self.advance();
                    Token::Op("!=".to_string())
                } else if self.peek() == Some('!') {
                    self.advance();
                    Token::Op("!!".to_string())
                } else {
                    Token::Op("!".to_string())
                }
            }

            '~' | '&' | '|' | '`' | '#' | '?' => {
                self.advance();
                let mut op = String::new();
                op.push(c);
                // Consume more operator chars
                while let Some(&nc) = self.chars.peek() {
                    if is_op_char(nc) {
                        self.chars.next();
                        self.pos += nc.len_utf8();
                        op.push(nc);
                    } else {
                        break;
                    }
                }
                Token::Op(op)
            }

            '\\' => {
                self.advance();
                Token::Op("\\".to_string())
            }

            // Identifier or keyword
            _ if is_ident_start(c) => self.scan_ident_or_keyword(),

            _ if c.is_whitespace() => {
                self.advance();
                return self.next_token();
            }

            _ => {
                self.advance();
                Token::Op(format!("{}", c))
            }
        };

        Ok(Some(TokenWithSpan {
            token,
            span: Span {
                start,
                end: self.pos,
            },
        }))
    }

    fn scan_string(&mut self) -> Result<String, TokenizerError> {
        let mut result = String::new();
        let start = self.pos;
        loop {
            match self.advance() {
                None => return Err(TokenizerError::UnterminatedString(start)),
                Some('\'') => {
                    // Check for doubled quote ''
                    if self.peek() == Some('\'') {
                        self.advance();
                        result.push('\'');
                    } else {
                        return Ok(result);
                    }
                }
                Some(c) => result.push(c),
            }
        }
    }

    fn scan_escape_string(&mut self) -> Result<String, TokenizerError> {
        let mut result = String::new();
        let start = self.pos;
        loop {
            match self.advance() {
                None => return Err(TokenizerError::UnterminatedString(start)),
                Some('\'') => {
                    if self.peek() == Some('\'') {
                        self.advance();
                        result.push('\'');
                    } else {
                        return Ok(result);
                    }
                }
                Some('\\') => match self.advance() {
                    None => return Err(TokenizerError::UnterminatedString(start)),
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some('\'') => result.push('\''),
                    Some('0') => result.push('\0'),
                    Some('b') => result.push('\x08'),
                    Some('f') => result.push('\x0c'),
                    Some('x') => {
                        let hex: String = self.advance_while(|c| c.is_ascii_hexdigit());
                        if !hex.is_empty() {
                            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                                result.push(byte as char);
                            }
                        }
                    }
                    Some(c) => result.push(c),
                },
                Some(c) => result.push(c),
            }
        }
    }

    fn scan_quoted_identifier(&mut self) -> Result<String, TokenizerError> {
        let mut result = String::new();
        let start = self.pos;
        loop {
            match self.advance() {
                None => return Err(TokenizerError::UnterminatedQuotedIdentifier(start)),
                Some('"') => {
                    if self.peek() == Some('"') {
                        self.advance();
                        result.push('"');
                    } else {
                        return Ok(result);
                    }
                }
                Some(c) => result.push(c),
            }
        }
    }

    fn scan_dollar_or_param(&mut self) -> Token {
        // Check if it's a parameter: $1, $2, etc.
        if let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                let num: String = self.advance_while(|c| c.is_ascii_digit());
                if let Ok(n) = num.parse::<i32>() {
                    return Token::Param(n);
                }
                // If parse fails, treat as identifier starting with $
                return Token::Ident(format!("${}", num));
            }
        }

        // Check for dollar-quoted string: $$text$$ or $tag$text$tag$
        // We need to find the end of the opening delimiter (next $)
        let tag = self.advance_while(|c| c != '$' && c != '\0');
        if self.peek() == Some('$') {
            self.advance(); // consume closing $ of delimiter
            let delimiter = format!("${}$", tag);
            let content = self.scan_dollar_string_content(&delimiter);
            Token::DollarString(content)
        } else {
            // Just a $ followed by an identifier-like thing
            if tag.is_empty() {
                Token::Op("$".to_string())
            } else {
                Token::Ident(format!("${}", tag))
            }
        }
    }

    fn scan_dollar_string_content(&mut self, delimiter: &str) -> String {
        let mut result = String::new();
        let delim_bytes = delimiter.as_bytes();
        let delim_len = delim_bytes.len();

        // Collect characters and check for delimiter match
        let mut window: Vec<char> = Vec::new();
        loop {
            match self.advance() {
                None => break,
                Some(c) => {
                    window.push(c);
                    if window.len() > delim_len {
                        let oldest = window.remove(0);
                        result.push(oldest);
                    }
                    if window.len() == delim_len {
                        let window_str: String = window.iter().collect();
                        if window_str == delimiter {
                            return result;
                        }
                    }
                }
            }
        }
        // Unterminated, but return what we have
        result.extend(window);
        result
    }

    fn scan_number(&mut self) -> Token {
        let mut num = String::new();

        // Check for hex: 0x...
        if self.peek() == Some('0') {
            self.advance();
            num.push('0');
            if self.peek() == Some('x') || self.peek() == Some('X') {
                self.advance();
                num.push('x');
                let hex = self.advance_while(|c| c.is_ascii_hexdigit());
                num.push_str(&hex);
                return Token::Integer(i64::from_str_radix(&hex, 16).unwrap_or(0));
            }
            // Continue with decimal
        }

        // Integer part
        num.push_str(&self.advance_while(|c| c.is_ascii_digit()));

        // Check for ".." (1..10 should be 1 DOT_DOT 10)
        if self.peek() == Some('.') {
            let mut chars_clone = self.chars.clone();
            chars_clone.next();
            if chars_clone.next() == Some('.') {
                // It's a ".." range, don't consume the dot
                if let Ok(n) = num.parse::<i64>() {
                    return Token::Integer(n);
                }
                return Token::Float(num);
            }
        }

        // Decimal part
        if self.peek() == Some('.') {
            self.advance();
            num.push('.');
            num.push_str(&self.advance_while(|c| c.is_ascii_digit()));
        }

        // Exponent part
        if self.peek() == Some('e') || self.peek() == Some('E') {
            num.push(self.advance().unwrap());
            if self.peek() == Some('+') || self.peek() == Some('-') {
                num.push(self.advance().unwrap());
            }
            num.push_str(&self.advance_while(|c| c.is_ascii_digit()));
            // f/d suffix after scientific notation
            if self.peek() == Some('f')
                || self.peek() == Some('F')
                || self.peek() == Some('d')
                || self.peek() == Some('D')
            {
                self.advance();
            }
            return Token::Float(num);
        }

        // f/d suffix (e.g., 3.14f, 123d)
        if self.peek() == Some('f')
            || self.peek() == Some('F')
            || self.peek() == Some('d')
            || self.peek() == Some('D')
        {
            self.advance();
            return Token::Float(num);
        }

        // Pure integer
        if !num.contains('.') && !num.contains('e') && !num.contains('E') {
            if let Ok(n) = num.parse::<i64>() {
                return Token::Integer(n);
            }
        }

        Token::Float(num)
    }

    fn scan_ident_or_keyword(&mut self) -> Token {
        let mut ident = String::new();
        ident.push(self.advance().unwrap());
        ident.push_str(&self.advance_while(is_ident_cont));

        // Check if it's a keyword
        if let Some(kw) = lookup_keyword(&ident) {
            Token::Keyword(kw)
        } else {
            Token::Ident(ident)
        }
    }
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || (c as u32) >= 0x80
}

fn is_ident_cont(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '$' || c == '#' || (c as u32) >= 0x80
}

fn is_op_char(c: char) -> bool {
    matches!(
        c,
        '~' | '!'
            | '@'
            | '#'
            | '^'
            | '&'
            | '|'
            | '`'
            | '?'
            | '+'
            | '-'
            | '*'
            | '/'
            | '%'
            | '<'
            | '>'
            | '='
    )
}

#[cfg(test)]
mod tests {
    use super::super::Keyword;
    use super::*;

    fn tokenize(input: &str) -> Result<Vec<TokenWithSpan>, TokenizerError> {
        Tokenizer::new(input).tokenize()
    }

    fn tokens_as_vec(input: &str) -> Vec<Token> {
        tokenize(input)
            .unwrap()
            .into_iter()
            .map(|t| t.token)
            .collect()
    }

    #[test]
    fn test_basic_keywords() {
        let tokens = tokens_as_vec("SELECT * FROM table1");
        assert!(matches!(tokens[0], Token::Keyword(Keyword::SELECT)));
        assert!(matches!(tokens[1], Token::Star));
        assert!(matches!(tokens[2], Token::Keyword(Keyword::FROM)));
    }

    #[test]
    fn test_string_literal() {
        let tokens = tokens_as_vec("'hello world'");
        assert!(matches!(&tokens[0], Token::StringLiteral(s) if s == "hello world"));
    }

    #[test]
    fn test_string_with_doubled_quotes() {
        let tokens = tokens_as_vec("'it''s a test'");
        assert!(matches!(&tokens[0], Token::StringLiteral(s) if s == "it's a test"));
    }

    #[test]
    fn test_integer() {
        let tokens = tokens_as_vec("42");
        assert!(matches!(&tokens[0], Token::Integer(n) if *n == 42));
    }

    #[test]
    fn test_float() {
        let tokens = tokens_as_vec("3.14");
        assert!(matches!(&tokens[0], Token::Float(s) if s == "3.14"));
    }

    #[test]
    fn test_dot_dot_range() {
        let tokens = tokens_as_vec("1..10");
        assert!(matches!(&tokens[0], Token::Integer(_)));
        assert!(matches!(&tokens[1], Token::DotDot));
        assert!(matches!(&tokens[2], Token::Integer(_)));
    }

    #[test]
    fn test_operators() {
        let tokens = tokens_as_vec(">= <> <=");
        assert!(matches!(&tokens[0], Token::Op(s) if s == ">="));
        assert!(matches!(&tokens[1], Token::Op(s) if s == "<>"));
        assert!(matches!(&tokens[2], Token::Op(s) if s == "<="));
    }

    #[test]
    fn test_line_comment() {
        let tokens = tokens_as_vec("SELECT -- this is a comment\nFROM");
        assert!(matches!(tokens[0], Token::Keyword(Keyword::SELECT)));
        assert!(matches!(tokens[1], Token::Keyword(Keyword::FROM)));
    }

    #[test]
    fn test_block_comment() {
        let tokens = tokens_as_vec("SELECT /* comment */ FROM");
        assert!(matches!(tokens[0], Token::Keyword(Keyword::SELECT)));
        assert!(matches!(tokens[1], Token::Keyword(Keyword::FROM)));
    }

    #[test]
    fn test_nested_block_comment() {
        let tokens = tokens_as_vec("SELECT /* outer /* inner */ still comment */ FROM");
        assert!(matches!(tokens[0], Token::Keyword(Keyword::SELECT)));
        assert!(matches!(tokens[1], Token::Keyword(Keyword::FROM)));
    }

    #[test]
    fn test_parameter() {
        let tokens = tokens_as_vec("$1 $2");
        assert!(matches!(&tokens[0], Token::Param(n) if *n == 1));
        assert!(matches!(&tokens[1], Token::Param(n) if *n == 2));
    }

    #[test]
    fn test_quoted_identifier() {
        let tokens = tokens_as_vec("\"my table\"");
        assert!(matches!(&tokens[0], Token::QuotedIdent(s) if s == "my table"));
    }

    #[test]
    fn test_escape_string() {
        let tokens = tokens_as_vec("E'hello\\nworld'");
        assert!(matches!(&tokens[0], Token::EscapeString(s) if s == "hello\nworld"));
    }

    #[test]
    fn test_bit_string() {
        let tokens = tokens_as_vec("B'1010'");
        assert!(matches!(&tokens[0], Token::BitString(s) if s == "1010"));
    }

    #[test]
    fn test_hex_string() {
        let tokens = tokens_as_vec("X'FF'");
        assert!(matches!(&tokens[0], Token::HexString(s) if s == "FF"));
    }

    #[test]
    fn test_national_string() {
        let tokens = tokens_as_vec("N'hello'");
        assert!(matches!(&tokens[0], Token::NationalString(s) if s == "hello"));
    }

    #[test]
    fn test_dollar_quoted_string() {
        let tokens = tokens_as_vec("$$hello world$$");
        assert!(matches!(&tokens[0], Token::DollarString(s) if s == "hello world"));
    }

    #[test]
    fn test_tagged_dollar_string() {
        let tokens = tokens_as_vec("$tag$hello world$tag$");
        assert!(matches!(&tokens[0], Token::DollarString(s) if s == "hello world"));
    }

    #[test]
    fn test_typecast() {
        let tokens = tokens_as_vec("::");
        assert!(matches!(tokens[0], Token::Typecast));
    }

    #[test]
    fn test_plus_join_components() {
        let tokens = tokens_as_vec("(+)");
        assert!(matches!(tokens[0], Token::LParen));
        assert!(matches!(tokens[1], Token::Plus));
        assert!(matches!(tokens[2], Token::RParen));
    }

    #[test]
    fn test_colon_equals() {
        let tokens = tokens_as_vec(":=");
        assert!(matches!(tokens[0], Token::ColonEquals));
    }

    #[test]
    fn test_param_equals() {
        let tokens = tokens_as_vec("=>");
        assert!(matches!(tokens[0], Token::ParamEquals));
    }

    #[test]
    fn test_at_at_variable() {
        let tokens = tokens_as_vec("@@session.tx_isolation");
        assert!(matches!(&tokens[0], Token::SetIdent(s) if s == "session.tx_isolation"));
    }

    #[test]
    fn test_scientific_notation() {
        let tokens = tokens_as_vec("1.5e-3");
        assert!(matches!(&tokens[0], Token::Float(s) if s == "1.5e-3"));
    }

    #[test]
    fn test_hex_integer() {
        let tokens = tokens_as_vec("0xFF");
        assert!(matches!(&tokens[0], Token::Integer(n) if *n == 255));
    }
}
