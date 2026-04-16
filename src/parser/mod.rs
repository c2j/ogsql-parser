pub(crate) mod ddl;
pub(crate) mod dml;
pub(crate) mod expr;
pub(crate) mod function_validator;
pub(crate) mod hint_validator;
pub(crate) mod plpgsql;
pub(crate) mod select;
pub(crate) mod utility;

use crate::token::keyword::Keyword;
use crate::token::{SourceLocation, Token, TokenWithSpan};

#[derive(Debug, Clone, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum ParserError {
    #[error("unexpected token at line {}, column {}: expected {}, got {}", .location.line, .location.column, expected, got)]
    UnexpectedToken {
        location: SourceLocation,
        expected: String,
        got: String,
    },
    #[error("unexpected end of input at line {}, column {}: expected {}", .location.line, .location.column, expected)]
    UnexpectedEof {
        expected: String,
        location: SourceLocation,
    },
    #[error("{}", .message)]
    Warning {
        message: String,
        location: SourceLocation,
    },
    #[error("reserved keyword \"{}\" cannot be used as identifier at line {}, column {}", .keyword, .location.line, .location.column)]
    ReservedKeywordAsIdentifier {
        keyword: String,
        location: SourceLocation,
    },
    #[error("{0}")]
    TokenizerError(#[from] crate::token::tokenizer::TokenizerError),
}

pub struct Parser {
    tokens: Vec<TokenWithSpan>,
    pos: usize,
    errors: Vec<ParserError>,
    source: String,
    depth: u32,
}

const MAX_PARSE_DEPTH: u32 = 256;

impl Parser {
    pub fn new(tokens: Vec<TokenWithSpan>) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: Vec::new(),
            source: String::new(),
            depth: 0,
        }
    }

    pub fn with_source(tokens: Vec<TokenWithSpan>, source: String) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: Vec::new(),
            source,
            depth: 0,
        }
    }

    fn enter_scope(&mut self) -> Result<(), ParserError> {
        self.depth += 1;
        if self.depth > MAX_PARSE_DEPTH {
            self.depth -= 1;
            Err(ParserError::Warning {
                message: format!("nesting depth exceeded {} — skipping", MAX_PARSE_DEPTH),
                location: self.current_location(),
            })
        } else {
            Ok(())
        }
    }

    fn leave_scope(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }

    pub fn parse_sql(input: &str) -> (Vec<crate::ast::StatementInfo>, Vec<ParserError>) {
        match crate::token::tokenizer::Tokenizer::new(input).tokenize() {
            Ok(tokens) => {
                let mut parser = Parser::with_source(tokens, input.to_string());
                let infos = parser.parse_with_text();
                (infos, parser.errors().to_vec())
            }
            Err(e) => (vec![], vec![ParserError::TokenizerError(e)]),
        }
    }

    pub fn parse_one(
        input: &str,
    ) -> Result<(crate::ast::StatementInfo, Vec<ParserError>), ParserError> {
        let tokens = crate::token::tokenizer::Tokenizer::new(input).tokenize()?;
        let mut parser = Parser::with_source(tokens, input.to_string());
        let infos = parser.parse_with_text();
        match infos.len() {
            0 => Err(ParserError::UnexpectedEof {
                expected: "statement".to_string(),
                location: parser.current_location(),
            }),
            1 => Ok((infos.into_iter().next().unwrap(), parser.errors().to_vec())),
            n => Err(ParserError::UnexpectedToken {
                location: parser.current_location(),
                expected: "single statement".to_string(),
                got: format!("{} statements", n),
            }),
        }
    }

    pub fn errors(&self) -> &[ParserError] {
        &self.errors
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn add_error(&mut self, error: ParserError) {
        self.errors.push(error);
    }

    fn current_location(&self) -> SourceLocation {
        self.tokens
            .get(self.pos)
            .map(|t| t.location)
            .unwrap_or_default()
    }

    pub fn parse(&mut self) -> Vec<crate::ast::Statement> {
        let mut stmts = Vec::new();
        loop {
            match self.peek() {
                Token::Eof => break,
                Token::Semicolon => {
                    self.advance();
                    continue;
                }
                _ => match self.parse_statement() {
                    Ok(stmt) => stmts.push(stmt),
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon();
                        stmts.push(crate::ast::Statement::Empty);
                    }
                },
            }
        }
        stmts
    }

    pub fn parse_with_text(&mut self) -> Vec<crate::ast::StatementInfo> {
        let mut infos = Vec::new();
        let line_offsets = Self::compute_line_offsets(&self.source);
        loop {
            match self.peek() {
                Token::Eof => break,
                Token::Semicolon | Token::Slash => {
                    self.advance();
                    continue;
                }
                _ => {
                    let start_pos = self.pos;
                    let end_pos = self.find_statement_end_pos();
                    let result = self.parse_statement();
                    let stmt = match result {
                        Ok(s) => {
                            self.try_consume_semicolon();
                            if self.pos <= end_pos {
                                self.pos = end_pos + 1;
                            }
                            s
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.pos = end_pos + 1;
                            crate::ast::Statement::Empty
                        }
                    };

                    let start_span = self.tokens[start_pos].span;
                    let end_token = if end_pos < self.tokens.len() {
                        &self.tokens[end_pos]
                    } else {
                        self.tokens.last().unwrap()
                    };
                    let end_span = end_token.span;

                    let byte_start = start_span.start.min(self.source.len());
                    let byte_end = end_span.end.min(self.source.len());
                    let sql_text = if byte_start < byte_end {
                        self.source[byte_start..byte_end].trim().to_string()
                    } else {
                        String::new()
                    };

                    let (start_line, start_col) =
                        Self::byte_offset_to_line_col(&line_offsets, byte_start, &self.source);
                    let (end_line, end_col) =
                        Self::byte_offset_to_line_col(&line_offsets, byte_end, &self.source);

                    infos.push(crate::ast::StatementInfo {
                        sql_text,
                        start_line,
                        start_col,
                        end_line,
                        end_col,
                        statement: stmt,
                    });
                }
            }
        }
        infos
    }

    fn find_statement_end_pos(&self) -> usize {
        let is_package = self.detect_package_context();
        let mut depth = 0i32;
        let mut begin_depth = 0i32;
        let mut subprog_depth = 0i32;
        let mut seen_outer_end = false;

        for i in self.pos..self.tokens.len() {
            match &self.tokens[i].token {
                Token::Eof => return if i > 0 { i - 1 } else { 0 },
                Token::LParen => depth += 1,
                Token::RParen => depth = (depth - 1).max(0),
                Token::DollarString { .. } => {}
                Token::Keyword(Keyword::BEGIN_P) => begin_depth += 1,
                Token::Keyword(Keyword::END_P) => {
                    let next_is_compound = (i + 1) < self.tokens.len()
                        && matches!(
                            self.tokens[i + 1].token,
                            Token::Keyword(Keyword::LOOP)
                                | Token::Keyword(Keyword::IF_P)
                                | Token::Keyword(Keyword::CASE)
                        );
                    if next_is_compound {
                        // compound END (END IF, END LOOP, END CASE) — no depth change
                    } else if begin_depth > 0 {
                        begin_depth -= 1;
                        if begin_depth == 0 && is_package && subprog_depth > 0 {
                            subprog_depth -= 1;
                        }
                    } else if is_package && subprog_depth > 0 {
                        subprog_depth -= 1;
                    } else if is_package {
                        // Package-level END reached
                        seen_outer_end = true;
                    }
                }
                Token::Keyword(Keyword::PROCEDURE) | Token::Keyword(Keyword::FUNCTION)
                    if is_package && depth == 0 && begin_depth == 0 && subprog_depth == 0 =>
                {
                    if self.looks_like_subprogram_def_at(i) {
                        subprog_depth += 1;
                    }
                }
                Token::Semicolon if depth <= 0 && begin_depth <= 0 => {
                    if is_package {
                        if seen_outer_end {
                            if let Some(slash_pos) = self.find_slash_after(i) {
                                return slash_pos;
                            }
                            return i;
                        }
                        // Semicolon inside package spec/body — not a terminator
                    } else {
                        if let Some(slash_pos) = self.find_slash_after(i) {
                            return slash_pos;
                        }
                        return i;
                    }
                }
                Token::Slash if depth <= 0 && begin_depth <= 0 => {
                    if is_package && !seen_outer_end {
                        // Slash inside package — not a terminator
                    } else {
                        return i;
                    }
                }
                _ => {}
            }
        }
        self.tokens.len().saturating_sub(1)
    }

    /// Check if tokens starting at `self.pos` form `CREATE [OR REPLACE] PACKAGE [BODY]`.
    fn detect_package_context(&self) -> bool {
        let mut i = self.pos;
        if i >= self.tokens.len() {
            return false;
        }
        if !matches!(self.tokens[i].token, Token::Keyword(Keyword::CREATE)) {
            return false;
        }
        i += 1;
        if i < self.tokens.len() && matches!(self.tokens[i].token, Token::Keyword(Keyword::OR)) {
            i += 1;
            if i < self.tokens.len()
                && matches!(self.tokens[i].token, Token::Keyword(Keyword::REPLACE))
            {
                i += 1;
            }
        }
        i < self.tokens.len() && matches!(self.tokens[i].token, Token::Keyword(Keyword::PACKAGE))
    }

    /// From position `start` (pointing at PROCEDURE or FUNCTION keyword), peek ahead
    /// to determine if this is a subprogram definition (has IS/AS body) vs a declaration (ends with ;).
    fn looks_like_subprogram_def_at(&self, start: usize) -> bool {
        let mut j = start + 1;
        let mut paren_d = 0i32;
        while j < self.tokens.len() {
            match &self.tokens[j].token {
                Token::LParen => paren_d += 1,
                Token::RParen => paren_d = (paren_d - 1).max(0),
                Token::Keyword(Keyword::IS) | Token::Keyword(Keyword::AS) if paren_d == 0 => {
                    return true;
                }
                Token::Semicolon if paren_d == 0 => return false,
                Token::Eof => return false,
                _ => {}
            }
            j += 1;
        }
        false
    }

    fn find_slash_after(&self, semicolon_pos: usize) -> Option<usize> {
        for j in (semicolon_pos + 1)..self.tokens.len() {
            match &self.tokens[j].token {
                Token::Slash => return Some(j),
                Token::Keyword(Keyword::END_P) | Token::Semicolon => return None,
                _ if !matches!(
                    self.tokens[j].token,
                    Token::Keyword(Keyword::END_P) | Token::Ident(_)
                ) =>
                {
                    return None
                }
                _ => {}
            }
        }
        None
    }

    fn lookahead_is_compound_end(&self) -> bool {
        if self.pos + 1 >= self.tokens.len() {
            return false;
        }
        matches!(
            self.tokens[self.pos + 1].token,
            Token::Keyword(Keyword::LOOP)
                | Token::Keyword(Keyword::IF_P)
                | Token::Keyword(Keyword::CASE)
        )
    }

    fn byte_offset_to_line_col(
        line_offsets: &[usize],
        byte_offset: usize,
        source: &str,
    ) -> (usize, usize) {
        let offset = byte_offset.min(source.len());
        let line = line_offsets.partition_point(|&lo| lo <= offset).max(1);
        let line_start = line_offsets[line - 1];
        let col = offset - line_start + 1;
        (line, col)
    }

    fn compute_line_offsets(source: &str) -> Vec<usize> {
        let mut offsets = vec![0usize];
        for (i, c) in source.char_indices() {
            if c == '\n' {
                offsets.push(i + 1);
            }
        }
        offsets
    }

    pub fn into_iter(self) -> StatementIter {
        StatementIter {
            parser: self,
            done: false,
        }
    }

    pub fn parse_next(&mut self) -> Option<Result<crate::ast::Statement, ParserError>> {
        loop {
            match self.peek() {
                Token::Eof => return None,
                Token::Semicolon => {
                    self.advance();
                    continue;
                }
                _ => {
                    let result = self.parse_statement();
                    return Some(result);
                }
            }
        }
    }

    // ── Token navigation helpers ──

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.pos)
            .map(|t| &t.token)
            .unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn match_keyword(&self, kw: Keyword) -> bool {
        matches!(self.peek(), Token::Keyword(k) if *k == kw)
    }

    fn expect_keyword(&mut self, kw: Keyword) -> Result<(), ParserError> {
        if self.match_keyword(kw) {
            self.advance();
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: format!("{:?}", kw),
                got: format!("{:?}", self.peek()),
            })
        }
    }

    pub(crate) fn peek_keyword(&self) -> Option<Keyword> {
        if let Token::Keyword(kw) = self.peek() {
            Some(*kw)
        } else {
            None
        }
    }

    fn match_token(&self, expected: &Token) -> bool {
        self.peek() == expected
    }

    fn expect_token(&mut self, expected: &Token) -> Result<(), ParserError> {
        if self.match_token(expected) {
            self.advance();
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: format!("{:?}", expected),
                got: format!("{:?}", self.peek()),
            })
        }
    }

    fn try_consume_keyword(&mut self, kw: Keyword) -> bool {
        if self.match_keyword(kw) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek_ident_str(&self) -> Option<&str> {
        match self.peek() {
            Token::Ident(s) => Some(s.as_str()),
            _ => None,
        }
    }

    fn match_ident_str(&self, target: &str) -> bool {
        match self.peek() {
            Token::Ident(s) => s.eq_ignore_ascii_case(target),
            Token::Keyword(kw) => {
                let s = format!("{:?}", kw).to_lowercase();
                let trimmed = s.trim_end_matches("_p");
                trimmed.eq_ignore_ascii_case(target)
            }
            _ => false,
        }
    }

    fn try_consume_ident_str(&mut self, target: &str) -> bool {
        if self.match_ident_str(target) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Consume an identifier (Ident, QuotedIdent, or Keyword-as-identifier).
    /// Reserved keywords used as identifiers emit error; non-reserved are silently accepted.
    fn parse_identifier(&mut self) -> Result<String, ParserError> {
        match self.peek().clone() {
            Token::Ident(s) => {
                self.advance();
                Ok(s)
            }
            Token::QuotedIdent(s) => {
                self.advance();
                Ok(s)
            }
            Token::Keyword(kw) => {
                let location = self.current_location();
                self.advance();
                let s = format!("{:?}", kw).to_lowercase();
                let name = s.trim_end_matches("_p").to_string();

                if kw.category() == crate::token::keyword::KeywordCategory::Reserved {
                    self.add_error(ParserError::ReservedKeywordAsIdentifier {
                        keyword: name.clone(),
                        location,
                    });
                }
                Ok(name)
            }
            _ => Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "identifier".to_string(),
                got: format!("{:?}", self.peek()),
            }),
        }
    }

    /// Parse a qualified name: `name` or `schema.name` or `catalog.schema.name`.
    fn parse_object_name(&mut self) -> Result<crate::ast::ObjectName, ParserError> {
        let mut name = vec![self.parse_identifier()?];
        while self.match_token(&Token::Dot) {
            self.advance();
            name.push(self.parse_identifier()?);
        }
        Ok(name)
    }

    /// Try to consume an optional alias: [AS] identifier
    fn parse_optional_alias(&mut self) -> Result<Option<String>, ParserError> {
        if self.match_keyword(Keyword::AS) {
            self.advance();
            Ok(Some(self.parse_identifier()?))
        } else {
            match self.peek() {
                Token::Ident(_) | Token::QuotedIdent(_) => Ok(Some(self.parse_identifier()?)),
                _ => Ok(None),
            }
        }
    }

    /// Try to consume an optional column alias: [AS] identifier
    /// Unlike parse_optional_alias, also accepts non-reserved keywords as implicit aliases.
    /// Uses 1-token lookahead to avoid consuming keywords that start subsequent clauses
    /// (e.g., LOOP in PL/pgSQL, CONNECT in hierarchical queries, ON CONFLICT in INSERT).
    fn parse_optional_column_alias(&mut self) -> Result<Option<String>, ParserError> {
        if self.match_keyword(Keyword::AS) {
            self.advance();
            Ok(Some(self.parse_identifier()?))
        } else {
            match self.peek() {
                Token::Ident(_) | Token::QuotedIdent(_) => Ok(Some(self.parse_identifier()?)),
                Token::Keyword(kw) => {
                    if kw.category() != crate::token::keyword::KeywordCategory::Reserved
                        && self.looks_like_alias()
                    {
                        Ok(Some(self.parse_identifier()?))
                    } else {
                        Ok(None)
                    }
                }
                _ => Ok(None),
            }
        }
    }

    /// Check if the token *after* the current one confirms the current token is an alias.
    /// After a valid alias we expect: Comma, FROM, RParen, WHERE, GROUP, ORDER, HAVING,
    /// LIMIT, OFFSET, UNION, INTERSECT, EXCEPT, MINUS, FOR, EOF, Semicolon, or certain
    /// keywords that continue the query — NOT keywords like LOOP, CONNECT, ON, etc.
    fn looks_like_alias(&self) -> bool {
        if self.pos + 1 >= self.tokens.len() {
            return true;
        }
        match &self.tokens[self.pos + 1].token {
            Token::Comma | Token::RParen | Token::Semicolon | Token::Eof => true,
            Token::Keyword(kw) => matches!(
                kw,
                Keyword::FROM
                    | Keyword::WHERE
                    | Keyword::GROUP_P
                    | Keyword::ORDER
                    | Keyword::HAVING
                    | Keyword::LIMIT
                    | Keyword::OFFSET
                    | Keyword::UNION
                    | Keyword::INTERSECT
                    | Keyword::EXCEPT
                    | Keyword::MINUS_P
                    | Keyword::FOR
                    | Keyword::INTO
                    | Keyword::END_P
                    | Keyword::THEN
                    | Keyword::ELSE
                    | Keyword::WHEN
                    | Keyword::AND
                    | Keyword::OR
                    | Keyword::AS
            ),
            _ => false,
        }
    }

    /// Check if the current token starts an expression.
    fn is_expr_start(&self) -> bool {
        matches!(
            self.peek(),
            Token::Integer(_)
                | Token::Float(_)
                | Token::StringLiteral(_)
                | Token::EscapeString(_)
                | Token::BitString(_)
                | Token::HexString(_)
                | Token::NationalString(_)
                | Token::DollarString { .. }
                | Token::Ident(_)
                | Token::QuotedIdent(_)
                | Token::Param(_)
                | Token::LParen
                | Token::LBracket
                | Token::Minus
                | Token::Plus
                | Token::Star
                | Token::SetIdent(_)
                | Token::Keyword(Keyword::TRUE_P)
                | Token::Keyword(Keyword::FALSE_P)
                | Token::Keyword(Keyword::NULL_P)
                | Token::Keyword(Keyword::DEFAULT)
                | Token::Keyword(Keyword::CASE)
                | Token::Keyword(Keyword::EXISTS)
                | Token::Keyword(Keyword::NOT)
                | Token::Keyword(Keyword::ARRAY)
                | Token::Keyword(Keyword::CAST)
                | Token::Keyword(Keyword::PRIOR)
        )
    }

    // ── Statement dispatch ──

    pub(crate) fn consume_hints(&mut self) -> Vec<String> {
        let mut hints = Vec::new();
        let loc = self.current_location();
        while let Token::Hint(h) = self.peek().clone() {
            for w in hint_validator::validate_hints(&h, loc.clone()) {
                self.add_error(w);
            }
            hints.push(h);
            self.advance();
        }
        hints
    }

    fn parse_statement(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let stmt = match self.peek().clone() {
            Token::Keyword(Keyword::SELECT) | Token::Keyword(Keyword::WITH) => {
                let pre_hints = self.consume_hints();
                match self.parse_select_statement() {
                    Ok(mut stmt) => {
                        let mut hints = pre_hints;
                        hints.append(&mut stmt.hints);
                        stmt.hints = hints;
                        self.try_consume_semicolon();
                        crate::ast::Statement::Select(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::INSERT) => {
                let pre_hints = self.consume_hints();
                self.advance();
                if self.match_keyword(Keyword::ALL) {
                    self.advance();
                    match self.parse_insert_all() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::InsertAll(stmt)
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else if self.match_keyword(Keyword::FIRST_P) {
                    self.advance();
                    match self.parse_insert_first() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::InsertFirst(stmt)
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    match self.parse_insert() {
                        Ok(mut stmt) => {
                            let mut hints = pre_hints;
                            hints.append(&mut stmt.hints);
                            stmt.hints = hints;
                            self.try_consume_semicolon();
                            crate::ast::Statement::Insert(stmt)
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Token::Keyword(Keyword::UPDATE) => {
                let pre_hints = self.consume_hints();
                self.advance();
                match self.parse_update() {
                    Ok(mut stmt) => {
                        let mut hints = pre_hints;
                        hints.append(&mut stmt.hints);
                        stmt.hints = hints;
                        self.try_consume_semicolon();
                        crate::ast::Statement::Update(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::DELETE_P) => {
                let pre_hints = self.consume_hints();
                self.advance();
                match self.parse_delete() {
                    Ok(mut stmt) => {
                        let mut hints = pre_hints;
                        hints.append(&mut stmt.hints);
                        stmt.hints = hints;
                        self.try_consume_semicolon();
                        crate::ast::Statement::Delete(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::MERGE) => {
                let pre_hints = self.consume_hints();
                self.advance();
                match self.parse_merge() {
                    Ok(mut stmt) => {
                        let mut hints = pre_hints;
                        hints.append(&mut stmt.hints);
                        stmt.hints = hints;
                        self.try_consume_semicolon();
                        crate::ast::Statement::Merge(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::TRUNCATE) => {
                self.advance();
                match self.parse_truncate() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Truncate(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::PROCEDURE) => {
                self.advance();
                match self.parse_create_procedure() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::CreateProcedure(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::FUNCTION) => {
                self.advance();
                match self.parse_create_function() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::CreateFunction(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            // ── All other statements: skip to semicolon (not yet implemented) ──
            Token::Keyword(Keyword::CREATE) => {
                self.advance();
                self.dispatch_create()
            }
            Token::Keyword(Keyword::ALTER) => {
                self.advance();
                self.dispatch_alter()
            }
            Token::Keyword(Keyword::DROP) => {
                self.advance();
                self.dispatch_drop()
            }
            Token::Keyword(Keyword::SET) => {
                self.advance();
                match self.parse_set() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::VariableSet(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::SHOW) => {
                self.advance();
                match self.parse_show() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::VariableShow(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::RESET) => {
                self.advance();
                match self.parse_reset() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::VariableReset(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::BEGIN_P) => {
                if self.is_transaction_begin() {
                    self.advance();
                    match self.parse_transaction_begin() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::Transaction(stmt)
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    self.advance();
                    match self.parse_anonymous_block() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::AnonyBlock(stmt)
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Token::Keyword(Keyword::START) => {
                self.advance();
                match self.parse_transaction_begin() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Transaction(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::COMMIT) | Token::Keyword(Keyword::END_P) => {
                self.advance();
                match self.parse_transaction_commit() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Transaction(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::ROLLBACK) => {
                self.advance();
                match self.parse_transaction_rollback() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Transaction(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::SAVEPOINT) => {
                self.advance();
                match self.parse_savepoint() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Transaction(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::DISCARD) => {
                self.advance();
                match self.parse_discard() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Discard(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::CHECKPOINT) => {
                self.advance();
                self.try_consume_semicolon();
                crate::ast::Statement::Checkpoint
            }
            Token::Keyword(Keyword::COPY) => {
                self.advance();
                match self.parse_copy() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Copy(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::EXPLAIN) => {
                self.advance();
                match self.parse_explain() {
                    Ok(stmt) => crate::ast::Statement::Explain(stmt),
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::CALL) => {
                self.advance();
                match self.parse_call() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Call(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::GRANT) => {
                self.advance();
                if self.is_grant_role() {
                    match self.parse_grant_role() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::GrantRole(stmt)
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    match self.parse_grant() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::Grant(stmt)
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Token::Keyword(Keyword::REVOKE) => {
                self.advance();
                if self.is_revoke_role() {
                    match self.parse_revoke_role() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::RevokeRole(stmt)
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    match self.parse_revoke() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::Revoke(stmt)
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Token::Keyword(Keyword::VACUUM) => {
                self.advance();
                match self.parse_vacuum() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Vacuum(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::DO) => {
                self.advance();
                match self.parse_do() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Do(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::PREPARE) => {
                self.advance();
                match self.parse_prepare() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Prepare(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::EXECUTE) => {
                self.advance();
                match self.parse_execute() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Execute(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::DEALLOCATE) => {
                self.advance();
                match self.parse_deallocate() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Deallocate(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::COMMENT) => {
                self.advance();
                match self.parse_comment() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Comment(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::LOCK_P) => {
                self.advance();
                match self.parse_lock() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Lock(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::REFRESH) => {
                self.advance();
                let incremental = self.try_consume_keyword(Keyword::INCREMENTAL);
                if incremental {
                    match self.parse_refresh_materialized_view() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::RefreshMaterializedView(stmt)
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    match self.parse_refresh_materialized_view() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::RefreshMaterializedView(stmt)
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Token::Keyword(Keyword::FETCH) => {
                self.advance();
                match self.parse_fetch_cursor() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Fetch(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::DECLARE) => {
                self.advance();
                // Check if this is a bare PL/pgSQL declaration (e.g., "declare type ...")
                // rather than a DECLARE CURSOR statement
                if self.match_keyword(Keyword::TYPE_P) || self.match_ident_str("type") {
                    self.skip_to_semicolon()
                } else if self.looks_like_variable_decl() {
                    self.skip_to_semicolon()
                } else {
                    match self.parse_declare_cursor() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::DeclareCursor(stmt)
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Token::Keyword(Keyword::CLOSE) => {
                self.advance();
                match self.parse_close_portal() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::ClosePortal(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::CLUSTER) => {
                self.advance();
                match self.parse_cluster() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Cluster(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::REINDEX) => {
                self.advance();
                match self.parse_reindex() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Reindex(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::LISTEN) => {
                self.advance();
                match self.parse_listen() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Listen(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::NOTIFY) => {
                self.advance();
                match self.parse_notify() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Notify(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::UNLISTEN) => {
                self.advance();
                match self.parse_unlisten() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Unlisten(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::RULE) => {
                self.advance();
                match self.parse_rule() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Rule(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::ANALYZE) => {
                self.advance();
                match self.parse_analyze() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Analyze(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::SHUTDOWN) => {
                self.advance();
                match self.parse_shutdown() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Shutdown(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::BARRIER) => {
                self.advance();
                match self.parse_barrier() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Barrier(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::PURGE) => {
                self.advance();
                match self.parse_purge() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Purge(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::SNAPSHOT) => {
                self.advance();
                match self.parse_snapshot() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Snapshot(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::TIMECAPSULE) => {
                self.advance();
                match self.parse_timecapsule() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::TimeCapsule(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::SHRINK) => {
                self.advance();
                match self.parse_shrink() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Shrink(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::VERIFY) => {
                self.advance();
                match self.parse_verify() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Verify(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::COMPILE) => {
                self.advance();
                match self.parse_compile() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Compile(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::CLEAN) => {
                self.advance();
                match self.parse_clean_conn() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::CleanConn(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::SECURITY) => {
                self.advance();
                match self.parse_sec_label() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::SecLabel(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(_) => {
                self.advance();
                self.skip_to_semicolon()
            }
            _ => {
                self.advance();
                self.skip_to_semicolon()
            }
        };
        Ok(stmt)
    }

    fn try_consume_semicolon(&mut self) {
        if self.match_token(&Token::Semicolon) {
            self.advance();
        }
    }

    pub(crate) fn parse_generic_options(&mut self) -> Vec<(String, String)> {
        let mut options = Vec::new();
        if !self.try_consume_keyword(Keyword::WITH) {
            return options;
        }
        if !self.match_token(&Token::LParen) {
            return options;
        }
        self.advance();
        loop {
            let key = self.parse_identifier().unwrap_or_default();
            if self.match_token(&Token::Eq) {
                self.advance();
            }
            let value = match self.peek().clone() {
                Token::StringLiteral(s) => {
                    self.advance();
                    s
                }
                Token::Ident(s) => {
                    self.advance();
                    s
                }
                _ => String::new(),
            };
            options.push((key, value));
            if self.match_token(&Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        let _ = self.expect_token(&Token::RParen);
        options
    }

    pub(crate) fn parse_generic_options_no_with(&mut self) -> Vec<(String, String)> {
        let mut options = Vec::new();
        if !self.match_token(&Token::LParen) {
            return options;
        }
        self.advance();
        loop {
            let key = self.parse_identifier().unwrap_or_default();
            if self.match_token(&Token::Eq) {
                self.advance();
            }
            let value = match self.peek().clone() {
                Token::StringLiteral(s) => {
                    self.advance();
                    s
                }
                Token::Ident(s) => {
                    self.advance();
                    s
                }
                Token::Keyword(kw) => {
                    self.advance();
                    format!("{:?}", kw).trim_end_matches("_P").to_lowercase()
                }
                _ => String::new(),
            };
            options.push((key, value));
            if self.match_token(&Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        let _ = self.expect_token(&Token::RParen);
        options
    }

    fn parse_create_foreign(&mut self) -> Result<crate::ast::Statement, ParserError> {
        if self.match_keyword(Keyword::FOREIGN) {
            self.advance();
        }
        match self.peek_keyword() {
            Some(Keyword::DATA_P) => {
                self.advance(); // DATA
                self.expect_keyword(Keyword::WRAPPER)?;
                let name = self.parse_identifier()?;
                let mut handler = None;
                let mut validator = None;
                if self.try_consume_keyword(Keyword::HANDLER) {
                    handler = Some(self.parse_identifier()?);
                }
                if self.try_consume_keyword(Keyword::NO) {
                    self.advance(); // skip HANDLER or VALIDATOR after NO
                    if self.match_keyword(Keyword::HANDLER)
                        || self.match_keyword(Keyword::VALIDATOR)
                    {
                        self.advance();
                    }
                }
                if self.match_keyword(Keyword::VALIDATOR) {
                    self.advance();
                    validator = Some(self.parse_identifier()?);
                }
                let options = self.parse_generic_options();
                self.try_consume_semicolon();
                Ok(crate::ast::Statement::CreateFdw(
                    crate::ast::CreateFdwStatement {
                        name,
                        handler,
                        validator,
                        options,
                    },
                ))
            }
            Some(Keyword::SERVER) => {
                self.advance(); // SERVER
                let name = self.parse_identifier()?;
                let mut server_type = None;
                let mut version = None;
                if self.try_consume_keyword(Keyword::TYPE_P) {
                    self.advance();
                    server_type = Some(self.parse_identifier()?);
                }
                if self.try_consume_keyword(Keyword::VERSION_P) {
                    self.advance();
                    version = Some(self.parse_identifier()?);
                }
                self.expect_keyword(Keyword::FOREIGN)?;
                self.expect_keyword(Keyword::DATA_P)?;
                self.expect_keyword(Keyword::WRAPPER)?;
                let fdw_name = self.parse_identifier()?;
                let options = self.parse_generic_options();
                self.try_consume_semicolon();
                Ok(crate::ast::Statement::CreateForeignServer(
                    crate::ast::CreateForeignServerStatement {
                        name,
                        server_type,
                        version,
                        fdw_name,
                        options,
                    },
                ))
            }
            Some(Keyword::TABLE) => {
                self.advance(); // TABLE
                let name = self.parse_object_name()?;
                let mut columns = Vec::new();
                if self.match_token(&Token::LParen) {
                    self.advance();
                    loop {
                        if self.match_token(&Token::RParen) {
                            self.advance();
                            break;
                        }
                        let col_name = self.parse_identifier()?;
                        let data_type =
                            self.parse_data_type().unwrap_or(crate::ast::DataType::Text);
                        let mut constraints = Vec::new();
                        if self.try_consume_keyword(Keyword::NOT) {
                            self.advance(); // NOT NULL
                            if self.match_keyword(Keyword::NULL_P) {
                                self.advance();
                            }
                        }
                        columns.push(crate::ast::ColumnDef {
                            name: col_name,
                            data_type,
                            constraints,
                            compress_mode: None,
                        });
                        if !self.match_token(&Token::Comma) {
                            if self.match_token(&Token::RParen) {
                                self.advance();
                                break;
                            }
                            break;
                        }
                        self.advance();
                    }
                }
                self.expect_keyword(Keyword::SERVER)?;
                let server_name = self.parse_identifier()?;
                let options = self.parse_generic_options();
                self.try_consume_semicolon();
                Ok(crate::ast::Statement::CreateForeignTable(
                    crate::ast::CreateForeignTableStatement {
                        name,
                        columns,
                        server_name,
                        options,
                    },
                ))
            }
            _ => Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "TABLE, SERVER, or DATA WRAPPER".to_string(),
                got: format!("{:?}", self.peek()),
            }),
        }
    }

    fn parse_create_publication(&mut self) -> Result<crate::ast::Statement, ParserError> {
        self.advance(); // PUBLICATION
        let name = self.parse_identifier()?;
        let mut tables = Vec::new();
        let mut all_tables = false;
        if self.try_consume_keyword(Keyword::FOR) {
            if self.try_consume_keyword(Keyword::ALL) {
                self.expect_keyword(Keyword::TABLES)?;
                all_tables = true;
            } else {
                self.expect_keyword(Keyword::TABLE)?;
                loop {
                    tables.push(self.parse_object_name()?);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                    self.advance();
                }
            }
        }
        let options = self.parse_generic_options();
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::CreatePublication(
            crate::ast::CreatePublicationStatement {
                name,
                tables,
                all_tables,
                options,
            },
        ))
    }

    fn parse_create_subscription(&mut self) -> Result<crate::ast::Statement, ParserError> {
        self.advance(); // SUBSCRIPTION
        let name = self.parse_identifier()?;
        self.expect_keyword(Keyword::CONNECTION)?;
        let connection = match self.peek().clone() {
            Token::StringLiteral(s) => {
                self.advance();
                s
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "connection string".to_string(),
                    got: format!("{:?}", self.peek()),
                })
            }
        };
        self.expect_keyword(Keyword::PUBLICATION)?;
        let mut publications = Vec::new();
        loop {
            publications.push(self.parse_identifier()?);
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }
        let options = self.parse_generic_options();
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::CreateSubscription(
            crate::ast::CreateSubscriptionStatement {
                name,
                connection,
                publications,
                options,
            },
        ))
    }

    fn parse_create_node_group_inner(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let name = self.parse_identifier()?;
        let mut nodes = Vec::new();
        if self.match_token(&Token::LParen) {
            self.advance();
            loop {
                if self.match_token(&Token::RParen) {
                    self.advance();
                    break;
                }
                nodes.push(self.parse_identifier()?);
                if !self.match_token(&Token::Comma) {
                    if self.match_token(&Token::RParen) {
                        self.advance();
                        break;
                    }
                    break;
                }
                self.advance();
            }
        }
        let options = self.parse_generic_options();
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::CreateNodeGroup(
            crate::ast::CreateNodeGroupStatement {
                name,
                nodes,
                options,
            },
        ))
    }

    fn parse_create_resource_pool_after_resource(
        &mut self,
    ) -> Result<crate::ast::Statement, ParserError> {
        self.expect_keyword(Keyword::POOL)?;
        self.parse_create_resource_pool_inner()
    }

    fn parse_create_resource_pool_inner(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let name = self.parse_identifier()?;
        let options = self.parse_generic_options();
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::CreateResourcePool(
            crate::ast::CreateResourcePoolStatement { name, options },
        ))
    }

    fn parse_create_workload_group(&mut self) -> Result<crate::ast::Statement, ParserError> {
        self.advance(); // WORKLOAD
        self.expect_keyword(Keyword::GROUP_P)?;
        let name = self.parse_identifier()?;
        let mut pool_name = None;
        if self.try_consume_keyword(Keyword::USING) {
            self.expect_keyword(Keyword::RESOURCE)?;
            self.expect_keyword(Keyword::POOL)?;
            pool_name = Some(self.parse_identifier()?);
        }
        let options = self.parse_generic_options();
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::CreateWorkloadGroup(
            crate::ast::CreateWorkloadGroupStatement {
                name,
                pool_name,
                options,
            },
        ))
    }

    fn parse_create_audit_policy(&mut self) -> Result<crate::ast::Statement, ParserError> {
        self.advance(); // AUDIT
        self.expect_keyword(Keyword::POLICY)?;
        let name = self.parse_identifier()?;
        let policy_type = self.parse_identifier()?;
        let options = self.parse_generic_options();
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::CreateAuditPolicy(
            crate::ast::CreateAuditPolicyStatement {
                name,
                policy_type,
                options,
            },
        ))
    }

    fn parse_create_masking_policy(&mut self) -> Result<crate::ast::Statement, ParserError> {
        self.advance(); // MASKING
        self.expect_keyword(Keyword::POLICY)?;
        let name = self.parse_identifier()?;
        let mut masking_function = None;
        let mut labels = Vec::new();
        if !self.match_keyword(Keyword::WITH)
            && !matches!(self.peek(), Token::LParen | Token::Semicolon | Token::Eof)
        {
            masking_function = Some(self.parse_identifier()?);
        }
        if self.match_keyword(Keyword::ON) {
            self.advance();
            self.expect_keyword(Keyword::LABEL)?;
            self.expect_token(&Token::LParen)?;
            loop {
                labels.push(self.parse_identifier()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect_token(&Token::RParen)?;
        }
        let options = self.parse_generic_options();
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::CreateMaskingPolicy(
            crate::ast::CreateMaskingPolicyStatement {
                name,
                masking_function,
                labels,
                options,
            },
        ))
    }

    fn parse_create_rls_policy(&mut self) -> Result<crate::ast::Statement, ParserError> {
        self.advance(); // POLICY
        let name = self.parse_identifier()?;
        self.expect_keyword(Keyword::ON)?;
        let table = self.parse_object_name()?;
        let mut permissive = true;
        let kw_str = match self.peek() {
            Token::Ident(s) | Token::QuotedIdent(s) => s.to_lowercase(),
            _ => String::new(),
        };
        if kw_str == "restrictive" {
            permissive = false;
            self.advance();
        } else if kw_str == "permissive" {
            self.advance();
        }
        let mut using_expr = None;
        if self.try_consume_keyword(Keyword::USING) {
            self.expect_token(&Token::LParen)?;
            using_expr = Some(self.parse_expr()?);
            self.expect_token(&Token::RParen)?;
        }
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::CreateRlsPolicy(
            crate::ast::CreateRlsPolicyStatement {
                name,
                table,
                permissive,
                using_expr,
            },
        ))
    }

    fn parse_create_resource_label(&mut self) -> Result<crate::ast::Statement, ParserError> {
        self.advance(); // LABEL
        let name = self.parse_identifier()?;
        let add = if self.match_keyword(Keyword::ADD_P) {
            self.advance();
            true
        } else if self.match_keyword(Keyword::REMOVE) {
            self.advance();
            false
        } else {
            return Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "ADD or REMOVE".to_string(),
                got: format!("{:?}", self.peek()),
            });
        };
        let label_type = self.parse_identifier()?;
        self.expect_token(&Token::LParen)?;
        let mut targets = Vec::new();
        loop {
            targets.push(self.parse_object_name()?);
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }
        self.expect_token(&Token::RParen)?;
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::CreatePolicyLabel(
            crate::ast::CreatePolicyLabelStatement {
                name,
                add,
                label_type,
                targets,
            },
        ))
    }

    fn parse_alter_masking_policy(&mut self) -> Result<crate::ast::Statement, ParserError> {
        self.expect_keyword(Keyword::POLICY)?;
        let name = self.parse_identifier()?;
        let action = if self.match_keyword(Keyword::COMMENTS) {
            self.advance();
            let comment = self.parse_string_literal()?;
            crate::ast::AlterMaskingPolicyAction::Comments(comment)
        } else if self.match_keyword(Keyword::ADD_P) {
            self.advance();
            let function = self.parse_identifier()?;
            self.expect_keyword(Keyword::ON)?;
            self.expect_keyword(Keyword::LABEL)?;
            self.expect_token(&Token::LParen)?;
            let mut labels = Vec::new();
            loop {
                labels.push(self.parse_identifier()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect_token(&Token::RParen)?;
            crate::ast::AlterMaskingPolicyAction::Add { function, labels }
        } else if self.match_keyword(Keyword::REMOVE) {
            self.advance();
            let function = self.parse_identifier()?;
            self.expect_keyword(Keyword::ON)?;
            self.expect_keyword(Keyword::LABEL)?;
            self.expect_token(&Token::LParen)?;
            let mut labels = Vec::new();
            loop {
                labels.push(self.parse_identifier()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect_token(&Token::RParen)?;
            crate::ast::AlterMaskingPolicyAction::Remove { function, labels }
        } else if self.match_keyword(Keyword::MODIFY_P) {
            self.advance();
            let function = self.parse_identifier()?;
            self.expect_keyword(Keyword::ON)?;
            self.expect_keyword(Keyword::LABEL)?;
            self.expect_token(&Token::LParen)?;
            let mut labels = Vec::new();
            loop {
                labels.push(self.parse_identifier()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect_token(&Token::RParen)?;
            crate::ast::AlterMaskingPolicyAction::Modify { function, labels }
        } else if self.match_keyword(Keyword::DROP) {
            self.advance();
            self.expect_keyword(Keyword::FILTER)?;
            crate::ast::AlterMaskingPolicyAction::DropFilter
        } else if self.match_keyword(Keyword::DISABLE_P) {
            self.advance();
            crate::ast::AlterMaskingPolicyAction::Disable
        } else {
            return Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "COMMENTS, ADD, REMOVE, MODIFY, DROP FILTER or DISABLE".to_string(),
                got: format!("{:?}", self.peek()),
            });
        };
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::AlterMaskingPolicy(
            crate::ast::AlterMaskingPolicyStatement { name, action },
        ))
    }

    fn parse_alter_resource_label(&mut self) -> Result<crate::ast::Statement, ParserError> {
        self.expect_keyword(Keyword::LABEL)?;
        let name = self.parse_identifier()?;
        let add = if self.match_keyword(Keyword::ADD_P) {
            self.advance();
            true
        } else if self.match_keyword(Keyword::REMOVE) {
            self.advance();
            false
        } else {
            return Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "ADD or REMOVE".to_string(),
                got: format!("{:?}", self.peek()),
            });
        };
        let label_type = self.parse_identifier()?;
        self.expect_token(&Token::LParen)?;
        let mut targets = Vec::new();
        loop {
            targets.push(self.parse_object_name()?);
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }
        self.expect_token(&Token::RParen)?;
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::AlterPolicyLabel(
            crate::ast::AlterPolicyLabelStatement {
                name,
                add,
                label_type,
                targets,
            },
        ))
    }

    fn parse_alter_resource_pool(
        &mut self,
    ) -> Result<crate::ast::AlterResourcePoolStatement, ParserError> {
        let name = self.parse_identifier()?;
        let mut options = Vec::new();
        if self.match_keyword(Keyword::WITH) {
            self.advance();
            self.expect_token(&Token::LParen)?;
            loop {
                let key = self.parse_identifier()?;
                self.expect_token(&Token::Eq)?;
                let value = self.parse_identifier()?;
                options.push((key, value));
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect_token(&Token::RParen)?;
        }
        Ok(crate::ast::AlterResourcePoolStatement { name, options })
    }

    fn skip_to_paren_end(&mut self) -> String {
        let mut depth = 0;
        let mut result = String::new();
        loop {
            match self.peek() {
                Token::LParen => {
                    depth += 1;
                    result.push('(');
                    self.advance();
                }
                Token::RParen => {
                    if depth == 0 {
                        break;
                    }
                    depth -= 1;
                    result.push(')');
                    self.advance();
                }
                Token::Eof => break,
                Token::Semicolon if depth == 0 => break,
                t => {
                    result.push_str(&format!("{:?}", t).to_lowercase());
                    self.advance();
                }
            }
        }
        result
    }

    fn dispatch_create(&mut self) -> crate::ast::Statement {
        let replace = if self.match_keyword(Keyword::OR) {
            self.advance();
            if self.match_keyword(Keyword::REPLACE) {
                self.advance();
                true
            } else {
                return self.skip_to_semicolon();
            }
        } else {
            false
        };

        let recursive = self.try_consume_keyword(Keyword::RECURSIVE);

        let temp =
            self.try_consume_keyword(Keyword::TEMPORARY) || self.try_consume_keyword(Keyword::TEMP);
        let unlogged = self.try_consume_keyword(Keyword::UNLOGGED);

        let is_public = self.try_consume_ident_str("PUBLIC");

        match self.peek_keyword() {
            Some(Keyword::TABLE) => {
                // Check for CREATE TABLE AS (CTAS) via lookahead
                let is_ctas = {
                    let mut look = self.pos + 1;
                    let mut depth = 0usize;
                    let mut found_as = false;
                    while look < self.tokens.len() {
                        match &self.tokens[look].token {
                            Token::Keyword(Keyword::AS) if depth == 0 => {
                                found_as = true;
                                break;
                            }
                            Token::Keyword(Keyword::PARTITION)
                            | Token::Keyword(Keyword::DISTRIBUTE)
                            | Token::Keyword(Keyword::INHERITS)
                            | Token::Keyword(Keyword::TABLESPACE)
                            | Token::Keyword(Keyword::WITH)
                            | Token::Keyword(Keyword::ON)
                            | Token::Keyword(Keyword::TO)
                            | Token::Semicolon
                            | Token::Eof => break,
                            Token::LParen => depth += 1,
                            Token::RParen => depth = depth.saturating_sub(1),
                            _ => {}
                        }
                        look += 1;
                        if look > self.pos + 50 {
                            break;
                        }
                    }
                    found_as
                };
                if is_ctas {
                    match self.parse_create_table_as(temp, unlogged) {
                        Ok(stmt) => stmt,
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    match self.parse_create_table(temp, unlogged) {
                        Ok(stmt) => crate::ast::Statement::CreateTable(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Some(Keyword::GLOBAL) => {
                self.advance();
                match self.parse_create_global_index() {
                    Ok(stmt) => crate::ast::Statement::CreateGlobalIndex(stmt),
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Some(Keyword::INDEX) => match self.parse_create_index() {
                Ok(stmt) => crate::ast::Statement::CreateIndex(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::SEQUENCE) => match self.parse_create_sequence() {
                Ok(stmt) => crate::ast::Statement::CreateSequence(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::VIEW) => match self.parse_create_view() {
                Ok(mut stmt) => {
                    stmt.replace = replace;
                    stmt.temporary = temp;
                    stmt.recursive = recursive;
                    crate::ast::Statement::CreateView(stmt)
                }
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::SCHEMA) => match self.parse_create_schema() {
                Ok(stmt) => crate::ast::Statement::CreateSchema(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::DATABASE) => {
                let is_link = {
                    let ahead = if self.pos + 1 < self.tokens.len() {
                        &self.tokens[self.pos + 1].token
                    } else {
                        &Token::Eof
                    };
                    matches!(ahead, Token::Ident(s) if s.eq_ignore_ascii_case("LINK"))
                };
                if is_link {
                    match self.parse_create_database_link(is_public) {
                        Ok(stmt) => crate::ast::Statement::CreateDatabaseLink(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    match self.parse_create_database() {
                        Ok(stmt) => crate::ast::Statement::CreateDatabase(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Some(Keyword::TABLESPACE) => match self.parse_create_tablespace() {
                Ok(stmt) => crate::ast::Statement::CreateTablespace(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::FUNCTION) => {
                self.advance();
                match self.parse_create_function() {
                    Ok(mut stmt) => {
                        stmt.replace = replace;
                        self.try_consume_semicolon();
                        crate::ast::Statement::CreateFunction(stmt)
                    }
                    Err(_) => self.skip_to_semicolon(),
                }
            }
            Some(Keyword::PROCEDURE) => {
                self.advance();
                match self.parse_create_procedure() {
                    Ok(mut stmt) => {
                        stmt.replace = replace;
                        self.try_consume_semicolon();
                        crate::ast::Statement::CreateProcedure(stmt)
                    }
                    Err(_) => self.skip_to_semicolon(),
                }
            }
            Some(Keyword::TRIGGER) => {
                self.advance();
                match self.parse_create_trigger() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::CreateTrigger(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Some(Keyword::INCREMENTAL) => {
                self.advance();
                match self.expect_keyword(Keyword::MATERIALIZED) {
                    Ok(()) => match self.parse_create_materialized_view() {
                        Ok(stmt) => crate::ast::Statement::CreateMaterializedView(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    },
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Some(Keyword::MATERIALIZED) => {
                self.advance();
                match self.parse_create_materialized_view() {
                    Ok(stmt) => crate::ast::Statement::CreateMaterializedView(stmt),
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Some(Keyword::EXTENSION) => match self.parse_create_extension() {
                Ok(stmt) => crate::ast::Statement::CreateExtension(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::ROLE) => {
                self.advance();
                match self.parse_create_role_options() {
                    Ok((name, options)) => {
                        crate::ast::Statement::CreateRole(crate::ast::CreateRoleStatement {
                            name,
                            options,
                        })
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Some(Keyword::USER) => {
                self.advance();
                if self.match_keyword(Keyword::MAPPING) {
                    self.advance();
                    match self.parse_create_user_mapping() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::CreateUserMapping(stmt)
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    match self.parse_create_role_options() {
                        Ok((name, options)) => {
                            crate::ast::Statement::CreateUser(crate::ast::CreateUserStatement {
                                name,
                                options,
                            })
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Some(Keyword::GROUP_P) => {
                self.advance();
                match self.parse_create_role_options() {
                    Ok((name, options)) => {
                        crate::ast::Statement::CreateGroup(crate::ast::CreateGroupStatement {
                            name,
                            options,
                        })
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Some(Keyword::PACKAGE) => match self.parse_create_package(replace) {
                Ok(stmt) => stmt,
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::TYPE_P) => match self.parse_create_type() {
                Ok(stmt) => crate::ast::Statement::CreateType(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::CAST) => match self.parse_create_cast() {
                Ok(stmt) => crate::ast::Statement::CreateCast(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::DOMAIN_P) => match self.parse_create_domain() {
                Ok(stmt) => crate::ast::Statement::CreateDomain(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::FOREIGN) => match self.parse_create_foreign() {
                Ok(stmt) => stmt,
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::SERVER) => match self.parse_create_foreign() {
                Ok(stmt) => stmt,
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::PUBLICATION) => match self.parse_create_publication() {
                Ok(stmt) => stmt,
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::SUBSCRIPTION) => match self.parse_create_subscription() {
                Ok(stmt) => stmt,
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::NODE) => {
                self.advance();
                if self.match_keyword(Keyword::GROUP_P) {
                    self.advance();
                    match self.parse_create_node_group_inner() {
                        Ok(stmt) => stmt,
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    let name = match self.parse_identifier() {
                        Ok(n) => n,
                        Err(e) => {
                            self.add_error(e);
                            return self.skip_to_semicolon();
                        }
                    };
                    let options = self.parse_generic_options();
                    self.try_consume_semicolon();
                    crate::ast::Statement::CreateNode(crate::ast::CreateNodeStatement {
                        name,
                        options,
                    })
                }
            }
            Some(Keyword::WORKLOAD) => match self.parse_create_workload_group() {
                Ok(stmt) => stmt,
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::RESOURCE) => {
                self.advance(); // RESOURCE
                if self.match_keyword(Keyword::POOL) {
                    match self.parse_create_resource_pool_after_resource() {
                        Ok(stmt) => stmt,
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else if self.match_keyword(Keyword::LABEL) {
                    match self.parse_create_resource_label() {
                        Ok(stmt) => stmt,
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    self.add_error(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "POOL or LABEL".to_string(),
                        got: format!("{:?}", self.peek()),
                    });
                    self.skip_to_semicolon()
                }
            }
            Some(Keyword::AUDIT) => match self.parse_create_audit_policy() {
                Ok(stmt) => stmt,
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::MASKING) => match self.parse_create_masking_policy() {
                Ok(stmt) => stmt,
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::POLICY) => match self.parse_create_rls_policy() {
                Ok(stmt) => stmt,
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::AGGREGATE) => {
                self.advance();
                match self.parse_create_aggregate() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::CreateAggregate(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Some(Keyword::OPERATOR) => {
                self.advance();
                if self.match_keyword(Keyword::CLASS) {
                    match self.parse_create_opclass() {
                        Ok(stmt) => crate::ast::Statement::CreateOpClass(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else if self.match_keyword(Keyword::FAMILY) {
                    match self.parse_create_opfamily() {
                        Ok(stmt) => crate::ast::Statement::CreateOpFamily(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    match self.parse_create_operator() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::CreateOperator(stmt)
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Some(Keyword::CONVERSION_P) => match self.parse_create_conversion() {
                Ok(stmt) => crate::ast::Statement::CreateConversion(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::SYNONYM) => match self.parse_create_synonym(replace) {
                Ok(stmt) => crate::ast::Statement::CreateSynonym(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::MODEL) => match self.parse_create_model() {
                Ok(stmt) => crate::ast::Statement::CreateModel(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::ACCESS) => match self.parse_create_am() {
                Ok(stmt) => crate::ast::Statement::CreateAm(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::DIRECTORY) => match self.parse_create_directory() {
                Ok(stmt) => crate::ast::Statement::CreateDirectory(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::DATA_P) => {
                self.advance();
                if self.match_keyword(Keyword::SOURCE_P) {
                    match self.parse_create_data_source() {
                        Ok(stmt) => crate::ast::Statement::CreateDataSource(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    self.add_error(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "SOURCE".to_string(),
                        got: format!("{:?}", self.peek()),
                    });
                    self.skip_to_semicolon()
                }
            }
            Some(Keyword::EVENT) => match self.parse_create_event() {
                Ok(stmt) => crate::ast::Statement::CreateEvent(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::STREAM) => match self.parse_create_stream() {
                Ok(stmt) => crate::ast::Statement::CreateStream(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::KEY) => match self.parse_create_key() {
                Ok(stmt) => crate::ast::Statement::CreateKey(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            _ => {
                if self.match_ident_str("CONTINUOUS") {
                    self.advance();
                    if self.match_keyword(Keyword::QUERY) || self.match_ident_str("QUERY") {
                        self.advance();
                    }
                    match self.parse_create_contquery() {
                        Ok(stmt) => crate::ast::Statement::CreateContQuery(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    self.skip_to_semicolon()
                }
            }
        }
    }

    fn dispatch_alter(&mut self) -> crate::ast::Statement {
        if self.match_keyword(Keyword::DEFAULT) {
            self.advance();
            match self.parse_alter_default_privileges() {
                Ok(stmt) => {
                    self.try_consume_semicolon();
                    crate::ast::Statement::AlterDefaultPrivileges(stmt)
                }
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            }
        } else {
            match self.peek_keyword() {
                Some(Keyword::INDEX) => match self.parse_alter_index() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterIndex(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::TABLE) => match self.parse_alter_table() {
                    Ok(stmt) => crate::ast::Statement::AlterTable(stmt),
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::TABLESPACE) => {
                    self.advance();
                    match self.parse_alter_tablespace() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::AlterTablespace(stmt)
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::DATABASE) => match self.parse_alter_database() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterDatabase(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::SCHEMA) => match self.parse_alter_schema() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterSchema(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::SEQUENCE) => match self.parse_alter_sequence() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterSequence(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::FUNCTION) => match self.parse_alter_function() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterFunction(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::PROCEDURE) => match self.parse_alter_function() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterProcedure(crate::ast::AlterProcedureStatement {
                            name: stmt.name.clone(),
                            action: stmt.action,
                        })
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::ROLE) => match self.parse_alter_role() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterRole(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::USER) => {
                    self.advance();
                    if self.match_keyword(Keyword::MAPPING) {
                        match self.parse_alter_user_mapping() {
                            Ok(stmt) => {
                                self.try_consume_semicolon();
                                crate::ast::Statement::AlterUserMapping(stmt)
                            }
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else {
                        match self.parse_alter_user_inner() {
                            Ok(stmt) => {
                                self.try_consume_semicolon();
                                crate::ast::Statement::AlterUser(stmt)
                            }
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    }
                }
                Some(Keyword::GROUP_P) => match self.parse_alter_group() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterGroup(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::SYSTEM_P) => match self.parse_alter_global_config() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterGlobalConfig(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::TYPE_P) => match self.parse_alter_type() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterCompositeType(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::VIEW) => match self.parse_alter_view() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterView(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::TRIGGER) => match self.parse_alter_trigger() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterTrigger(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::EXTENSION) => match self.parse_alter_extension() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterExtension(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::MASKING) => {
                    self.advance();
                    match self.parse_alter_masking_policy() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            stmt
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::RESOURCE) => {
                    self.advance();
                    if self.match_keyword(Keyword::POOL) {
                        self.advance();
                        match self.parse_alter_resource_pool() {
                            Ok(stmt) => {
                                self.try_consume_semicolon();
                                crate::ast::Statement::AlterResourcePool(stmt)
                            }
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else {
                        match self.parse_alter_resource_label() {
                            Ok(stmt) => {
                                self.try_consume_semicolon();
                                stmt
                            }
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    }
                }
                Some(Keyword::FOREIGN) => {
                    self.advance();
                    if self.match_keyword(Keyword::TABLE) {
                        self.advance();
                        match self.parse_alter_foreign_table() {
                            Ok(stmt) => crate::ast::Statement::AlterForeignTable(stmt),
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else if self.match_keyword(Keyword::SERVER) {
                        self.advance();
                        match self.parse_alter_foreign_server() {
                            Ok(stmt) => crate::ast::Statement::AlterForeignServer(stmt),
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else if self.match_keyword(Keyword::DATA_P) {
                        self.advance();
                        if !self.match_keyword(Keyword::WRAPPER) {
                            self.add_error(ParserError::UnexpectedToken {
                                location: self.current_location(),
                                expected: "WRAPPER after DATA".to_string(),
                                got: format!("{:?}", self.peek()),
                            });
                            return self.skip_to_semicolon();
                        }
                        self.advance();
                        match self.parse_alter_fdw() {
                            Ok(stmt) => crate::ast::Statement::AlterFdw(stmt),
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else {
                        self.add_error(ParserError::UnexpectedToken {
                            location: self.current_location(),
                            expected: "TABLE, SERVER, or DATA WRAPPER after FOREIGN".to_string(),
                            got: format!("{:?}", self.peek()),
                        });
                        self.skip_to_semicolon()
                    }
                }
                Some(Keyword::SERVER) => {
                    self.advance();
                    match self.parse_alter_foreign_server() {
                        Ok(stmt) => crate::ast::Statement::AlterForeignServer(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::PUBLICATION) => {
                    self.advance();
                    match self.parse_alter_publication() {
                        Ok(stmt) => crate::ast::Statement::AlterPublication(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::SUBSCRIPTION) => {
                    self.advance();
                    match self.parse_alter_subscription() {
                        Ok(stmt) => crate::ast::Statement::AlterSubscription(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::NODE) => {
                    self.advance();
                    if self.match_keyword(Keyword::GROUP_P) {
                        self.advance();
                        match self.parse_alter_node_group() {
                            Ok(stmt) => crate::ast::Statement::AlterNodeGroup(stmt),
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else {
                        match self.parse_alter_node() {
                            Ok(stmt) => crate::ast::Statement::AlterNode(stmt),
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    }
                }
                Some(Keyword::WORKLOAD) => {
                    self.advance();
                    if !self.match_keyword(Keyword::GROUP_P) {
                        self.add_error(ParserError::UnexpectedToken {
                            location: self.current_location(),
                            expected: "GROUP after WORKLOAD".to_string(),
                            got: format!("{:?}", self.peek()),
                        });
                        return self.skip_to_semicolon();
                    }
                    self.advance();
                    match self.parse_alter_workload_group() {
                        Ok(stmt) => crate::ast::Statement::AlterWorkloadGroup(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::AUDIT) => {
                    self.advance();
                    if !self.match_keyword(Keyword::POLICY) {
                        self.add_error(ParserError::UnexpectedToken {
                            location: self.current_location(),
                            expected: "POLICY after AUDIT".to_string(),
                            got: format!("{:?}", self.peek()),
                        });
                        return self.skip_to_semicolon();
                    }
                    self.advance();
                    match self.parse_alter_audit_policy() {
                        Ok(stmt) => crate::ast::Statement::AlterAuditPolicy(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::POLICY) => {
                    self.advance();
                    match self.parse_alter_rls_policy() {
                        Ok(stmt) => crate::ast::Statement::AlterRlsPolicy(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::DATA_P) => {
                    self.advance();
                    if self.match_ident_str("source") {
                        self.advance();
                        match self.parse_alter_data_source() {
                            Ok(stmt) => crate::ast::Statement::AlterDataSource(stmt),
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else {
                        self.add_error(ParserError::UnexpectedToken {
                            location: self.current_location(),
                            expected: "SOURCE after DATA".to_string(),
                            got: format!("{:?}", self.peek()),
                        });
                        self.skip_to_semicolon()
                    }
                }
                Some(Keyword::EVENT) => {
                    self.advance();
                    match self.parse_alter_event() {
                        Ok(stmt) => crate::ast::Statement::AlterEvent(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::OPERATOR) => {
                    self.advance();
                    if self.match_keyword(Keyword::FAMILY) {
                        self.advance();
                        match self.parse_alter_opfamily() {
                            Ok(stmt) => crate::ast::Statement::AlterOpFamily(stmt),
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else if self.match_keyword(Keyword::CLASS) {
                        self.advance();
                        match self.parse_alter_opfamily() {
                            Ok(stmt) => crate::ast::Statement::AlterOpFamily(stmt),
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else {
                        self.add_error(ParserError::UnexpectedToken {
                            location: self.current_location(),
                            expected: "FAMILY or CLASS after OPERATOR".to_string(),
                            got: format!("{:?}", self.peek()),
                        });
                        self.skip_to_semicolon()
                    }
                }
                Some(Keyword::MATERIALIZED) => {
                    self.advance();
                    if !self.match_keyword(Keyword::VIEW) {
                        self.add_error(ParserError::UnexpectedToken {
                            location: self.current_location(),
                            expected: "VIEW after MATERIALIZED".to_string(),
                            got: format!("{:?}", self.peek()),
                        });
                        return self.skip_to_semicolon();
                    }
                    self.advance();
                    match self.parse_alter_materialized_view() {
                        Ok(stmt) => crate::ast::Statement::AlterMaterializedView(stmt),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                _ => {
                    if self.match_ident_str("rls") {
                        self.advance();
                        if self.match_keyword(Keyword::POLICY) {
                            self.advance();
                            match self.parse_alter_rls_policy() {
                                Ok(stmt) => crate::ast::Statement::AlterRlsPolicy(stmt),
                                Err(e) => {
                                    self.add_error(e);
                                    self.skip_to_semicolon()
                                }
                            }
                        } else {
                            self.skip_to_semicolon()
                        }
                    } else {
                        self.skip_to_semicolon()
                    }
                }
            }
        }
    }

    fn dispatch_drop(&mut self) -> crate::ast::Statement {
        if self.match_keyword(Keyword::USER) {
            let saved_pos = self.pos;
            self.advance();
            if self.match_keyword(Keyword::MAPPING) {
                self.advance();
                match self.parse_drop_user_mapping() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        return crate::ast::Statement::DropUserMapping(stmt);
                    }
                    Err(e) => {
                        self.add_error(e);
                        return self.skip_to_semicolon();
                    }
                }
            }
            self.pos = saved_pos;
        }
        match self.parse_drop() {
            Ok(stmt) => crate::ast::Statement::Drop(stmt),
            Err(e) => {
                self.add_error(e);
                self.skip_to_semicolon()
            }
        }
    }

    fn parse_alter_tablespace(
        &mut self,
    ) -> Result<crate::ast::AlterTablespaceStatement, ParserError> {
        let name = self.parse_identifier()?;
        let action = if self.match_keyword(Keyword::RENAME) {
            self.advance();
            self.expect_keyword(Keyword::TO)?;
            let new_name = self.parse_identifier()?;
            crate::ast::AlterTablespaceAction::RenameTo { new_name }
        } else if self.match_keyword(Keyword::OWNER) {
            self.advance();
            self.expect_keyword(Keyword::TO)?;
            let new_owner = self.parse_identifier()?;
            crate::ast::AlterTablespaceAction::OwnerTo { new_owner }
        } else if self.match_keyword(Keyword::SET) {
            self.advance();
            self.expect_token(&Token::LParen)?;
            let mut options = Vec::new();
            loop {
                let key = self.parse_identifier()?;
                self.expect_token(&Token::Eq)?;
                let value = self.parse_identifier()?;
                options.push((key, value));
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect_token(&Token::RParen)?;
            crate::ast::AlterTablespaceAction::SetOptions { options }
        } else if self.match_keyword(Keyword::RESET) {
            self.advance();
            self.expect_token(&Token::LParen)?;
            let mut options = Vec::new();
            loop {
                options.push(self.parse_identifier()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect_token(&Token::RParen)?;
            crate::ast::AlterTablespaceAction::ResetOptions { options }
        } else {
            return Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "RENAME TO, OWNER TO, SET, or RESET".to_string(),
                got: format!("{:?}", self.peek()),
            });
        };
        Ok(crate::ast::AlterTablespaceStatement { name, action })
    }

    fn skip_to_semicolon_as(&mut self, stmt: crate::ast::Statement) -> crate::ast::Statement {
        let mut depth = 0i32;
        let mut begin_depth = 0i32;
        loop {
            match self.peek() {
                Token::Eof => break,
                Token::LParen => {
                    depth += 1;
                    self.advance();
                }
                Token::RParen => {
                    depth = (depth - 1).max(0);
                    self.advance();
                }
                Token::Keyword(Keyword::BEGIN_P) => {
                    begin_depth += 1;
                    self.advance();
                }
                Token::Keyword(Keyword::END_P) => {
                    if begin_depth > 0 {
                        let next_is_compound = self.lookahead_is_compound_end();
                        if !next_is_compound {
                            begin_depth -= 1;
                        }
                    }
                    self.advance();
                }
                Token::Semicolon if depth == 0 && begin_depth == 0 => {
                    self.advance();
                    break;
                }
                Token::Slash if depth == 0 && begin_depth == 0 => {
                    self.advance();
                    break;
                }
                _ => {
                    self.advance();
                }
            }
        }
        stmt
    }

    fn skip_to_semicolon(&mut self) -> crate::ast::Statement {
        let mut depth = 0i32;
        let mut begin_depth = 0i32;
        loop {
            match self.peek() {
                Token::Eof => break,
                Token::LParen => {
                    depth += 1;
                    self.advance();
                }
                Token::RParen => {
                    depth = (depth - 1).max(0);
                    self.advance();
                }
                Token::Keyword(Keyword::BEGIN_P) => {
                    begin_depth += 1;
                    self.advance();
                }
                Token::Keyword(Keyword::END_P) => {
                    if begin_depth > 0 {
                        let next_is_compound = self.lookahead_is_compound_end();
                        if !next_is_compound {
                            begin_depth -= 1;
                        }
                    }
                    self.advance();
                }
                Token::Semicolon if depth <= 0 && begin_depth <= 0 => {
                    self.advance();
                    break;
                }
                Token::Slash if depth <= 0 && begin_depth <= 0 => {
                    self.advance();
                    break;
                }
                _ => {
                    self.advance();
                }
            }
        }
        crate::ast::Statement::Empty
    }

    fn looks_like_variable_decl(&self) -> bool {
        let mut i = self.pos;
        if i >= self.tokens.len() {
            return false;
        }
        match &self.tokens[i].token {
            Token::Ident(_) | Token::QuotedIdent(_) | Token::Keyword(_) => {}
            _ => return false,
        }
        i += 1;
        if i >= self.tokens.len() {
            return false;
        }
        match &self.tokens[i].token {
            Token::Ident(_) | Token::QuotedIdent(_) | Token::Keyword(_) => {}
            _ => return false,
        }
        if let Token::Keyword(kw) = &self.tokens[i].token {
            match kw {
                Keyword::CURSOR
                | Keyword::BINARY
                | Keyword::SCROLL
                | Keyword::NO
                | Keyword::WITH
                | Keyword::WITHOUT
                | Keyword::INSENSITIVE
                | Keyword::FOR => {
                    return false;
                }
                _ => {}
            }
        }
        if let Token::Ident(s) = &self.tokens[i].token {
            let upper = s.to_uppercase();
            if upper == "CURSOR" || upper == "BINARY" || upper == "SCROLL" {
                return false;
            }
        }
        true
    }
}

pub struct StatementIter {
    parser: Parser,
    done: bool,
}

impl Iterator for StatementIter {
    type Item = Result<crate::ast::Statement, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        match self.parser.parse_next() {
            Some(result) => Some(result),
            None => {
                self.done = true;
                None
            }
        }
    }
}

#[cfg(test)]
mod tests;
