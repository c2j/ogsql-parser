use super::keyword::lookup_keyword;
use super::{SourceLocation, Span, Token, TokenWithSpan};

#[derive(Debug, Clone, thiserror::Error, serde::Serialize, serde::Deserialize)]
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
    line: usize,
    line_start: usize,
    pending_hint: Option<String>,
    preserve_comments: bool,
    pending_comment: Option<String>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars().peekable(),
            pos: 0,
            line: 1,
            line_start: 0,
            pending_hint: None,
            preserve_comments: false,
            pending_comment: None,
        }
    }

    pub fn preserve_comments(mut self, yes: bool) -> Self {
        self.preserve_comments = yes;
        self
    }

    fn current_location(&self) -> SourceLocation {
        SourceLocation {
            line: self.line,
            column: self.pos - self.line_start + 1,
            offset: self.pos,
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<TokenWithSpan>, TokenizerError> {
        let mut tokens = Vec::with_capacity(self.input.len() / 8);
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
                        location: self.current_location(),
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
            if c == '\n' {
                self.line += 1;
                self.line_start = self.pos + c.len_utf8();
            }
            self.pos += c.len_utf8();
        }
        c
    }

    fn advance_while_pos<F: Fn(char) -> bool>(&mut self, predicate: F) -> usize {
        let start = self.pos;
        while let Some(&c) = self.chars.peek() {
            if predicate(c) {
                self.chars.next();
                if c == '\n' {
                    self.line += 1;
                    self.line_start = self.pos + c.len_utf8();
                }
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
        start
    }

    fn slice_from(&self, start: usize) -> String {
        self.input[start..self.pos].to_string()
    }

    fn skip_while<F: Fn(char) -> bool>(&mut self, predicate: F) {
        while let Some(&c) = self.chars.peek() {
            if predicate(c) {
                self.chars.next();
                if c == '\n' {
                    self.line += 1;
                    self.line_start = self.pos + c.len_utf8();
                }
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
    }

    fn skip_whitespace_and_comments(&mut self) -> Result<(), TokenizerError> {
        loop {
            match self.chars.peek().copied() {
                None => return Ok(()),
                Some(c) if c.is_whitespace() => {
                    self.advance();
                }
                Some('-') => {
                    if self.peek_byte_at(1) == Some(b'-') {
                        let start = self.pos;
                        self.advance();
                        self.advance();
                        if self.preserve_comments {
                            self.skip_while(|c| c != '\n');
                            let content = self.input[start..self.pos].to_string();
                            self.pending_comment = Some(content);
                            return Ok(());
                        } else {
                            self.skip_while(|c| c != '\n');
                        }
                    } else {
                        return Ok(());
                    }
                }
                Some('/') => {
                    if self.peek_byte_at(1) == Some(b'*') {
                        let start = self.pos;
                        self.advance();
                        self.advance();
                        if self.peek_byte_at(0) == Some(b'+') {
                            self.advance();
                            let hint = self.collect_hint_content();
                            self.pending_hint = Some(hint);
                        } else if self.preserve_comments {
                            let content = self.collect_block_comment_content(start);
                            self.pending_comment = Some(content);
                            return Ok(());
                        } else {
                            let saved_pos = self.pos;
                            self.skip_block_comment(saved_pos)?;
                        }
                    } else {
                        return Ok(());
                    }
                }
                _ => return Ok(()),
            }
        }
    }

    fn peek_byte_at(&self, offset: usize) -> Option<u8> {
        self.input.as_bytes().get(self.pos + offset).copied()
    }

    fn peek_byte_past_whitespace(&self, start_offset: usize) -> Option<u8> {
        let mut i = self.pos + start_offset;
        let bytes = self.input.as_bytes();
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        bytes.get(i).copied()
    }

    fn skip_whitespace_in_token(&mut self) {
        while self.peek().map_or(false, |c| c.is_ascii_whitespace()) {
            self.advance();
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

    fn collect_block_comment_content(&mut self, start: usize) -> String {
        let mut depth = 1;
        while depth > 0 {
            match self.advance() {
                None => break,
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
        self.input[start..self.pos].to_string()
    }

    fn collect_hint_content(&mut self) -> String {
        let mut content = String::new();
        let mut depth = 1;
        while depth > 0 {
            match self.advance() {
                None => break,
                Some('/') => {
                    if self.peek() == Some('*') {
                        self.advance();
                        depth += 1;
                        content.push_str("/*");
                    } else {
                        content.push('/');
                    }
                }
                Some('*') => {
                    if self.peek() == Some('/') {
                        self.advance();
                        depth -= 1;
                        if depth > 0 {
                            content.push_str("*/");
                        }
                    } else {
                        content.push('*');
                    }
                }
                Some(c) => content.push(c),
            }
        }
        content.trim().to_string()
    }

    fn next_token(&mut self) -> Result<Option<TokenWithSpan>, TokenizerError> {
        if let Some(hint) = self.pending_hint.take() {
            let start = self.pos;
            return Ok(Some(TokenWithSpan {
                token: Token::Hint(hint),
                span: Span {
                    start,
                    end: self.pos,
                },
                location: self.current_location(),
            }));
        }

        if let Some(comment) = self.pending_comment.take() {
            let start = self.pos;
            return Ok(Some(TokenWithSpan {
                token: Token::Comment(comment),
                span: Span {
                    start,
                    end: self.pos,
                },
                location: self.current_location(),
            }));
        }

        self.skip_whitespace_and_comments()?;

        // After skipping whitespace/comments, a hint may have been collected.
        // Return it before scanning the next real token to preserve correct order.
        if let Some(hint) = self.pending_hint.take() {
            let start = self.pos;
            return Ok(Some(TokenWithSpan {
                token: Token::Hint(hint),
                span: Span {
                    start,
                    end: self.pos,
                },
                location: self.current_location(),
            }));
        }

        if let Some(comment) = self.pending_comment.take() {
            let start = self.pos;
            return Ok(Some(TokenWithSpan {
                token: Token::Comment(comment),
                span: Span {
                    start,
                    end: self.pos,
                },
                location: self.current_location(),
            }));
        }

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
                self.scan_dollar_or_param()?
            }

            // Numbers
            '0'..='9' => self.scan_number(),

            // National character string N'...'
            'n' | 'N' => {
                if self.peek_byte_at(1) == Some(b'\'') {
                    self.advance();
                    self.advance();
                    let s = self.scan_string()?;
                    Token::NationalString(s)
                } else if self
                    .peek_byte_at(1)
                    .map_or(false, |b| b.is_ascii_whitespace())
                    && self.peek_byte_past_whitespace(2) == Some(b'\'')
                {
                    self.advance();
                    self.skip_whitespace_in_token();
                    self.advance();
                    let s = self.scan_string()?;
                    Token::NationalString(s)
                } else {
                    self.scan_ident_or_keyword()
                }
            }

            // Escape String E'...'
            'e' | 'E' => {
                if self.peek_byte_at(1) == Some(b'\'') {
                    self.advance();
                    self.advance();
                    let s = self.scan_escape_string()?;
                    Token::EscapeString(s)
                } else if self
                    .peek_byte_at(1)
                    .map_or(false, |b| b.is_ascii_whitespace())
                    && self.peek_byte_past_whitespace(2) == Some(b'\'')
                {
                    self.advance();
                    self.skip_whitespace_in_token();
                    self.advance();
                    let s = self.scan_escape_string()?;
                    Token::EscapeString(s)
                } else {
                    self.scan_ident_or_keyword()
                }
            }

            // Bit string B'...' or b'...'
            'b' | 'B' => {
                if self.peek_byte_at(1) == Some(b'\'') {
                    self.advance();
                    self.advance();
                    let s = self.scan_string()?;
                    Token::BitString(s)
                } else if self
                    .peek_byte_at(1)
                    .map_or(false, |b| b.is_ascii_whitespace())
                    && self.peek_byte_past_whitespace(2) == Some(b'\'')
                {
                    self.advance();
                    self.skip_whitespace_in_token();
                    self.advance();
                    let s = self.scan_string()?;
                    Token::BitString(s)
                } else {
                    self.scan_ident_or_keyword()
                }
            }

            // Hex string X'...' or x'...'
            'x' | 'X' => {
                if self.peek_byte_at(1) == Some(b'\'') {
                    self.advance();
                    self.advance();
                    let s = self.scan_string()?;
                    Token::HexString(s)
                } else if self
                    .peek_byte_at(1)
                    .map_or(false, |b| b.is_ascii_whitespace())
                    && self.peek_byte_past_whitespace(2) == Some(b'\'')
                {
                    self.advance();
                    self.skip_whitespace_in_token();
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
                    if self.chars.peek().map_or(false, |c| {
                        c.is_alphanumeric() || *c == '_' || *c == '.' || *c == '$'
                    }) {
                        let start = self.advance_while_pos(|c| {
                            c.is_alphanumeric() || c == '_' || c == '.' || c == '$'
                        });
                        Token::SetIdent(self.slice_from(start))
                    } else {
                        Token::Op("@@".to_string())
                    }
                } else if self.peek() == Some('>') {
                    self.advance();
                    Token::Op("@>".to_string())
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
                    let frac_start = self.advance_while_pos(|c| c.is_ascii_digit());
                    let mut full = format!(".{}", &self.input[frac_start..self.pos]);
                    if self.peek() == Some('e') || self.peek() == Some('E') {
                        full.push(self.advance().unwrap());
                        if self.peek() == Some('+') || self.peek() == Some('-') {
                            full.push(self.advance().unwrap());
                        }
                        let exp_start = self.advance_while_pos(|c| c.is_ascii_digit());
                        full.push_str(&self.input[exp_start..self.pos]);
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
                if self.peek() == Some('|') {
                    let chars: Vec<char> = self.chars.clone().collect();
                    if chars.len() >= 2 && chars[0] == '|' && chars[1] == '-' {
                        self.advance(); // consume '|'
                        self.advance(); // consume '-'
                        Token::Op("-|-".to_string())
                    } else {
                        Token::Minus
                    }
                } else if self.peek() == Some('>') {
                    self.advance();
                    if self.peek() == Some('>') {
                        self.advance();
                        Token::OpJsonArrow
                    } else {
                        Token::OpArrow
                    }
                } else {
                    Token::Minus
                }
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
                        if self.peek() == Some('>') {
                            self.advance();
                            Token::Op("<=>".to_string())
                        } else {
                            Token::OpLe
                        }
                    }
                    Some('-') => {
                        // Lookahead: current peek is '-', check if char after is '>'
                        let chars: Vec<char> = self.chars.clone().collect();
                        if chars.len() >= 2 && chars[0] == '-' && chars[1] == '>' {
                            self.advance(); // consume '-'
                            self.advance(); // consume '>'
                            Token::Op("<->".to_string())
                        } else {
                            Token::Lt
                        }
                    }
                    Some('>') => {
                        self.advance();
                        Token::OpNe
                    }
                    Some('<') => {
                        self.advance();
                        if self.peek() == Some('|') {
                            self.advance();
                            Token::Op("<<|".to_string())
                        } else if self.peek() == Some('=') {
                            self.advance();
                            Token::Op("<<=".to_string())
                        } else {
                            Token::OpShiftL
                        }
                    }
                    Some('@') => {
                        self.advance();
                        Token::Op("<@".to_string())
                    }
                    Some('^') => {
                        self.advance();
                        Token::Op("<^".to_string())
                    }
                    _ => Token::Lt,
                }
            }

            '>' => {
                self.advance();
                match self.peek() {
                    Some('=') => {
                        self.advance();
                        Token::OpGe
                    }
                    Some('>') => {
                        self.advance();
                        if self.peek() == Some('=') {
                            self.advance();
                            Token::Op(">>=".to_string())
                        } else {
                            Token::OpShiftR
                        }
                    }
                    Some('^') => {
                        self.advance();
                        Token::Op(">^".to_string())
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
                    Token::OpNe2
                } else if self.peek() == Some('!') {
                    self.advance();
                    Token::OpDblBang
                } else if self.peek() == Some('~') {
                    self.advance();
                    if self.peek() == Some('*') {
                        self.advance();
                        Token::Op("!~*".to_string())
                    } else {
                        Token::Op("!~".to_string())
                    }
                } else {
                    Token::Op("!".to_string())
                }
            }

            '~' | '&' | '|' | '`' | '#' | '?' => {
                self.advance();
                let start = self.pos - c.len_utf8();
                while let Some(&nc) = self.chars.peek() {
                    if is_op_char(nc) {
                        self.chars.next();
                        self.pos += nc.len_utf8();
                    } else {
                        break;
                    }
                }
                let op_str = &self.input[start..self.pos];
                if op_str == "||" {
                    Token::OpConcat
                } else {
                    Token::Op(op_str.to_string())
                }
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
            location: self.current_location(),
        }))
    }

    fn scan_string(&mut self) -> Result<String, TokenizerError> {
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
                Some('\\') => {
                    if self.peek() == Some('\'') {
                        self.advance();
                        result.push('\'');
                    } else {
                        result.push('\\');
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
                        let hex_start = self.advance_while_pos(|c| c.is_ascii_hexdigit());
                        let hex = &self.input[hex_start..self.pos];
                        if !hex.is_empty() {
                            if let Ok(byte) = u8::from_str_radix(hex, 16) {
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
        let start_line = self.line;
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
                Some('\n') => {
                    // Deliberately deviate from PostgreSQL: require same-line closure
                    // to prevent cascade misparse. Remove this branch to allow multi-line.
                    return Err(TokenizerError::UnterminatedQuotedIdentifier(start));
                }
                Some(c) => result.push(c),
            }
        }
    }

    fn scan_dollar_or_param(&mut self) -> Result<Token, TokenizerError> {
        if let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                let start = self.advance_while_pos(|c| c.is_ascii_digit());
                let num = &self.input[start..self.pos];
                if let Ok(n) = num.parse::<i32>() {
                    return Ok(Token::Param(n));
                }
                return Ok(Token::Ident(format!("${}", num)));
            }
        }

        // Per openGauss scan.l: tag chars must be valid identifier chars (no spaces, #, etc.)
        let tag_start = self
            .advance_while_pos(|c| c.is_ascii_alphanumeric() || c == '_' || (c as u32) >= 0x200);
        if self.peek() == Some('$') {
            self.advance();
            let tag = self.input[tag_start..self.pos - 1].to_string();
            let delimiter = format!("${}$", tag);
            let content = self.scan_dollar_string_content(&delimiter)?;
            let tag_opt = if tag.is_empty() { None } else { Some(tag) };
            Ok(Token::DollarString {
                tag: tag_opt,
                body: content,
            })
        } else {
            let tag = &self.input[tag_start..self.pos];
            if tag.is_empty() {
                Ok(Token::Op("$".to_string()))
            } else {
                Ok(Token::Ident(format!("${}", tag)))
            }
        }
    }

    fn scan_dollar_string_content(&mut self, delimiter: &str) -> Result<String, TokenizerError> {
        let start = self.pos;
        let delim_chars: Vec<char> = delimiter.chars().collect();
        let delim_len = delim_chars.len();
        let mut result = String::with_capacity(256);
        let mut window: std::collections::VecDeque<char> =
            std::collections::VecDeque::with_capacity(delim_len);

        loop {
            match self.advance() {
                None => return Err(TokenizerError::UnterminatedDollarString(start)),
                Some(c) => {
                    window.push_back(c);
                    if window.len() > delim_len {
                        result.push(window.pop_front().unwrap());
                    }
                    if window.len() == delim_len && window.iter().eq(delim_chars.iter()) {
                        return Ok(result);
                    }
                }
            }
        }
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
                let hex_start = self.advance_while_pos(|c| c.is_ascii_hexdigit());
                let hex = &self.input[hex_start..self.pos];
                num.push_str(hex);
                return Token::Integer(i64::from_str_radix(hex, 16).unwrap_or(0));
            }
            // Continue with decimal
        }

        // Integer part
        let int_start = self.advance_while_pos(|c| c.is_ascii_digit());
        num.push_str(&self.input[int_start..self.pos]);

        // Check for ".." (1..10 should be 1 DOT_DOT 10)
        if self.peek() == Some('.') {
            if self.peek_byte_at(1) == Some(b'.') {
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
            let frac_start = self.advance_while_pos(|c| c.is_ascii_digit());
            num.push_str(&self.input[frac_start..self.pos]);
        }

        // Exponent part
        if self.peek() == Some('e') || self.peek() == Some('E') {
            num.push(self.advance().unwrap());
            if self.peek() == Some('+') || self.peek() == Some('-') {
                num.push(self.advance().unwrap());
            }
            let exp_start = self.advance_while_pos(|c| c.is_ascii_digit());
            num.push_str(&self.input[exp_start..self.pos]);
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
        let start = self.pos;
        self.advance(); // consume first char
        while let Some(&c) = self.chars.peek() {
            if is_ident_cont(c) {
                self.chars.next();
                if c == '\n' {
                    self.line += 1;
                    self.line_start = self.pos + c.len_utf8();
                }
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }

        let ident = &self.input[start..self.pos];
        if let Some(kw) = lookup_keyword(ident) {
            Token::Keyword(kw)
        } else {
            Token::Ident(ident.to_string())
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
        assert!(matches!(tokens[0], Token::OpGe));
        assert!(matches!(tokens[1], Token::OpNe));
        assert!(matches!(tokens[2], Token::OpLe));
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
        assert!(
            matches!(&tokens[0], Token::DollarString { tag: None, body } if body == "hello world")
        );
    }

    #[test]
    fn test_tagged_dollar_string() {
        let tokens = tokens_as_vec("$tag$hello world$tag$");
        assert!(
            matches!(&tokens[0], Token::DollarString { tag: Some(t), body } if t == "tag" && body == "hello world")
        );
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

    #[test]
    fn test_comment_not_preserved_by_default() {
        let tokens = tokens_as_vec("SELECT -- comment\nFROM dual");
        assert!(!tokens.iter().any(|t| matches!(t, Token::Comment(_))));
    }

    #[test]
    fn test_single_line_comment_preserved() {
        let tokens = Tokenizer::new("SELECT -- this is a comment\nFROM dual")
            .preserve_comments(true)
            .tokenize()
            .unwrap();
        let tokens: Vec<Token> = tokens.into_iter().map(|t| t.token).collect();
        let comment = tokens.iter().find(|t| matches!(t, Token::Comment(_)));
        assert!(comment.is_some(), "Should have a Comment token");
        if let Some(Token::Comment(content)) = comment {
            assert!(content.contains("--"), "Comment should include -- prefix");
            assert!(
                content.contains("this is a comment"),
                "Comment should contain the text"
            );
        }
    }

    #[test]
    fn test_block_comment_preserved() {
        let tokens = Tokenizer::new("SELECT /* block\ncomment */ FROM dual")
            .preserve_comments(true)
            .tokenize()
            .unwrap();
        let tokens: Vec<Token> = tokens.into_iter().map(|t| t.token).collect();
        let comment = tokens.iter().find(|t| matches!(t, Token::Comment(_)));
        assert!(comment.is_some(), "Should have a Comment token");
        if let Some(Token::Comment(content)) = comment {
            assert!(content.starts_with("/*"), "Block comment should start with /*");
            assert!(content.ends_with("*/"), "Block comment should end with */");
            assert!(
                content.contains("block") && content.contains("comment"),
                "Should contain content"
            );
        }
    }

    #[test]
    fn test_comment_token_ordering() {
        let tokens = Tokenizer::new("SELECT /* c1 */ a, /* c2 */ b FROM t")
            .preserve_comments(true)
            .tokenize()
            .unwrap();
        let select_pos = tokens
            .iter()
            .position(|t| matches!(&t.token, Token::Keyword(Keyword::SELECT)))
            .unwrap();
        let first_comment_pos = tokens
            .iter()
            .position(|t| matches!(&t.token, Token::Comment(_)))
            .unwrap();
        assert!(
            select_pos < first_comment_pos,
            "SELECT should come before first comment"
        );
    }

    #[test]
    fn test_hint_not_affected_by_comment_preservation() {
        let tokens = Tokenizer::new("SELECT /*+ INDEX(t idx) */ a FROM t")
            .preserve_comments(true)
            .tokenize()
            .unwrap();
        let tokens: Vec<Token> = tokens.into_iter().map(|t| t.token).collect();
        assert!(
            tokens.iter().any(|t| matches!(t, Token::Hint(_))),
            "Hints should still be Hint tokens"
        );
        assert!(
            !tokens.iter().any(|t| matches!(t, Token::Comment(c) if c.contains("INDEX"))),
            "Hints should NOT be Comment tokens"
        );
    }
}
