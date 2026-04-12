pub(crate) mod ddl;
pub(crate) mod dml;
pub(crate) mod expr;
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
    #[error("{0}")]
    TokenizerError(#[from] crate::token::tokenizer::TokenizerError),
}

pub struct Parser {
    tokens: Vec<TokenWithSpan>,
    pos: usize,
    errors: Vec<ParserError>,
    source: String,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithSpan>) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: Vec::new(),
            source: String::new(),
        }
    }

    pub fn with_source(tokens: Vec<TokenWithSpan>, source: String) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: Vec::new(),
            source,
        }
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

                    // span.start/end are byte-exact; location.column points past token (unusable for start)
                    let start_span = self.tokens[start_pos].span;
                    let end_token = if end_pos < self.tokens.len() {
                        &self.tokens[end_pos]
                    } else {
                        self.tokens.last().unwrap()
                    };
                    let end_span = end_token.span;

                    let source = &self.source;
                    let byte_start = start_span.start.min(source.len());
                    let byte_end = end_span.end.min(source.len());
                    let sql_text = if byte_start < byte_end {
                        source[byte_start..byte_end].trim().to_string()
                    } else {
                        String::new()
                    };

                    let line_offsets = Self::compute_line_offsets(source);
                    let (start_line, start_col) =
                        Self::byte_offset_to_line_col(&line_offsets, byte_start, source);
                    let (end_line, end_col) =
                        Self::byte_offset_to_line_col(&line_offsets, byte_end, source);

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
        let col = source[line_start..offset].chars().count() + 1;
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
                self.advance();
                // Convert keyword to lowercase identifier, stripping _P suffix
                let s = format!("{:?}", kw).to_lowercase();
                Ok(s.trim_end_matches("_p").to_string())
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
        while let Token::Hint(h) = self.peek().clone() {
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

    fn parse_create_resource_pool(&mut self) -> Result<crate::ast::Statement, ParserError> {
        self.advance(); // RESOURCE
        self.expect_keyword(Keyword::POOL)?;
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
        let options = self.parse_generic_options();
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::CreateMaskingPolicy(
            crate::ast::CreateMaskingPolicyStatement { name, options },
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

        match self.peek_keyword() {
            Some(Keyword::TABLE) => match self.parse_create_table(temp, unlogged) {
                Ok(stmt) => crate::ast::Statement::CreateTable(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
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
            Some(Keyword::DATABASE) => match self.parse_create_database() {
                Ok(stmt) => crate::ast::Statement::CreateDatabase(stmt),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
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
            Some(Keyword::RESOURCE) => match self.parse_create_resource_pool() {
                Ok(stmt) => stmt,
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
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
            _ => self.skip_to_semicolon(),
        }
    }

    fn dispatch_alter(&mut self) -> crate::ast::Statement {
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
            Some(Keyword::USER) => match self.parse_alter_user() {
                Ok(stmt) => {
                    self.try_consume_semicolon();
                    crate::ast::Statement::AlterUser(stmt)
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
            _ => self.skip_to_semicolon(),
        }
    }

    fn dispatch_drop(&mut self) -> crate::ast::Statement {
        match self.parse_drop() {
            Ok(stmt) => crate::ast::Statement::Drop(stmt),
            Err(e) => {
                self.add_error(e);
                self.skip_to_semicolon()
            }
        }
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
