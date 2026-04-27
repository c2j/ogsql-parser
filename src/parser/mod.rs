pub(crate) mod ddl;
pub(crate) mod dml;
pub(crate) mod expr;
pub mod function_registry;
pub(crate) mod hint_validator;
pub(crate) mod plpgsql;
pub(crate) mod select;
pub(crate) mod utility;

use std::collections::HashSet;

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
    pl_into_mode: bool,
    scope_stack: Vec<HashSet<String>>,
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
            pl_into_mode: false,
            scope_stack: Vec::new(),
        }
    }

    pub fn with_source(tokens: Vec<TokenWithSpan>, source: String) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: Vec::new(),
            source,
            depth: 0,
            pl_into_mode: false,
            scope_stack: Vec::new(),
        }
    }

    pub fn set_pl_into_mode(&mut self, enabled: bool) {
        self.pl_into_mode = enabled;
    }

    pub fn push_scope(&mut self) {
        self.scope_stack.push(HashSet::new());
    }

    pub fn pop_scope(&mut self) -> Option<HashSet<String>> {
        self.scope_stack.pop()
    }

    pub fn declare_var(&mut self, name: &str) {
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.insert(name.to_lowercase());
        }
    }

    pub fn is_var_declared(&self, name: &str) -> bool {
        let lower = name.to_lowercase();
        self.scope_stack.iter().rev().any(|scope| scope.contains(&lower))
    }

    /// Parse an identifier in a PL context where it might be a variable.
    /// If the base identifier is found in the current scope stack, emit PlVariable.
    /// Otherwise, emit ColumnRef (may be resolved later by the analyzer).
    pub(crate) fn parse_pl_variable_or_column(&mut self) -> Result<crate::ast::Expr, ParserError> {
        let name = self.parse_object_name()?;
        let base = if !name.is_empty() && self.is_var_declared(&name[0]) {
            crate::ast::Expr::PlVariable(name)
        } else {
            crate::ast::Expr::ColumnRef(name)
        };

        let mut expr = base;
        while self.match_token(&Token::LParen) {
            self.advance();
            let index = self.parse_expr()?;
            self.expect_token(&Token::RParen)?;
            expr = crate::ast::Expr::Subscript {
                object: Box::new(expr),
                index: Box::new(index),
            };
        }
        Ok(expr)
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

    fn prev_location(&self) -> SourceLocation {
        if self.pos == 0 {
            return self.current_location();
        }
        self.tokens
            .get(self.pos.saturating_sub(1))
            .map(|t| t.location)
            .unwrap_or_default()
    }

    pub fn parse(&mut self) -> Vec<crate::ast::Statement> {
        let mut stmts = Vec::new();
        loop {
            match self.peek() {
                Token::Eof => break,
                Token::Semicolon | Token::Slash => {
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
                    if matches!(self.peek(), Token::Eq) && self.is_separator_line() {
                        self.skip_separator_line();
                        continue;
                    }
                    let start_pos = self.pos;
                    let end_pos = self.find_statement_end_pos();
                    let saved_error_count = self.errors.len();
                    let result = self.parse_statement();
                    let stmt = match result {
                        Ok(s) => {
                            self.try_consume_semicolon();
                            // Detect successful parse that didn't consume all tokens —
                            // this means there are unrecognized tokens in the statement.
                            if self.pos < end_pos && !matches!(s, crate::ast::Statement::Empty) {
                                let unconsumed = &self.tokens[self.pos];
                                self.add_error(ParserError::UnexpectedToken {
                                    location: unconsumed.location,
                                    expected: "end of statement".to_string(),
                                    got: format!("{:?}", unconsumed.token),
                                });
                            }
                            // If internal error recovery happened (returns Empty),
                            // rollback spurious errors from consuming past the statement
                            // boundary and keep only the most relevant one (last = from catch block).
                            if matches!(s, crate::ast::Statement::Empty)
                                && self.errors.len() > saved_error_count
                            {
                                let real_error = self.errors.pop().unwrap();
                                self.errors.truncate(saved_error_count);
                                self.errors.push(real_error);
                            }
                            s
                        }
                        Err(e) => {
                            self.add_error(e);
                            crate::ast::Statement::Empty
                        }
                    };
                    // Always reset to statement boundary to prevent cascading into next statement
                    if matches!(stmt, crate::ast::Statement::Empty) && end_pos - start_pos > 500 {
                        // When parsing fails and the estimated boundary is far away,
                        // the statement splitter was confused (e.g. unmatched paren).
                        // Fall back to the first semicolon after start_pos to avoid
                        // swallowing hundreds of subsequent statements.
                        if let Some(fallback) = self.find_next_semicolon_from(start_pos) {
                            self.pos = fallback + 1;
                        } else {
                            self.pos = end_pos + 1;
                        }
                    } else {
                        self.pos = end_pos + 1;
                    }

                    let start_span = self.tokens[start_pos].span;
                    let end_token = if end_pos < self.tokens.len() {
                        &self.tokens[end_pos]
                    } else {
                        self.tokens.last().unwrap()
                    };
                    let end_span = end_token.span;

                    let source_len = self.source.len();
                    let byte_start = start_span.start.min(source_len);
                    let byte_end = end_span.end.min(source_len);

                    // Only populate sql_text for top-level DML statements.
                    // For DDL/package bodies the full source text is not useful
                    // (inner PlStatement::SqlStatement already captures per-DML text).
                    let sql_text = if matches!(
                        stmt,
                        crate::ast::Statement::Select(_)
                            | crate::ast::Statement::Insert(_)
                            | crate::ast::Statement::Update(_)
                            | crate::ast::Statement::Delete(_)
                            | crate::ast::Statement::Merge(_)
                    ) {
                        if byte_start < byte_end {
                            self.source[byte_start..byte_end].trim().to_string()
                        } else {
                            String::new()
                        }
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
        let mut case_depth = 0i32;
        let mut seen_outer_end = false;
        let mut in_routine_decl = false;
        let in_declare_section = self.tokens.get(self.pos).map_or(false, |t| {
            if !matches!(t.token, Token::Keyword(Keyword::DECLARE)) {
                return false;
            }
            if self.is_declare_cursor_at(self.pos) {
                return false;
            }
            self.has_begin_after_declare(self.pos)
        });

        for i in self.pos..self.tokens.len() {
            match &self.tokens[i].token {
                Token::Eof => return if i > 0 { i - 1 } else { 0 },
                Token::LParen => depth += 1,
                Token::RParen => depth = (depth - 1).max(0),
                Token::DollarString { .. } => {}
                Token::Keyword(Keyword::CASE) => {
                    case_depth += 1;
                }
                Token::Keyword(Keyword::BEGIN_P) => {
                    begin_depth += 1;
                    if in_routine_decl && begin_depth == 1 {
                        in_routine_decl = false;
                    }
                }
                Token::Keyword(Keyword::END_P) => {
                    let next_is_compound = (i + 1) < self.tokens.len()
                        && matches!(
                            self.tokens[i + 1].token,
                            Token::Keyword(Keyword::LOOP)
                                | Token::Keyword(Keyword::IF_P)
                                | Token::Keyword(Keyword::CASE)
                        );
                    if next_is_compound {
                        if matches!(self.tokens[i + 1].token, Token::Keyword(Keyword::CASE))
                            && case_depth > 0
                        {
                            case_depth -= 1;
                        }
                    } else if case_depth > 0 {
                        case_depth -= 1;
                    } else if begin_depth > 0 {
                        begin_depth -= 1;
                        if begin_depth == 0 && is_package && subprog_depth > 0 {
                            subprog_depth -= 1;
                        }
                    } else if is_package && subprog_depth > 0 {
                        subprog_depth -= 1;
                    } else if is_package {
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
                Token::Keyword(Keyword::IS) | Token::Keyword(Keyword::AS)
                    if !is_package && depth == 0 && begin_depth == 0 && !in_routine_decl =>
                {
                    if self.is_routine_body_marker(i) {
                        in_routine_decl = true;
                    }
                }
                Token::Semicolon if depth <= 0 && begin_depth <= 0 => {
                    if in_declare_section || in_routine_decl {
                        continue;
                    }
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
                Token::Keyword(Keyword::CREATE)
                    if !is_package && depth <= 0 && begin_depth <= 0 =>
                {
                    if self.detect_package_context_at(i) {
                        return if i > self.pos { i - 1 } else { i };
                    }
                }
                _ => {}
            }
        }
        self.tokens.len().saturating_sub(1)
    }

    fn is_separator_line(&self) -> bool {
        let mut count = 0;
        for i in self.pos..self.tokens.len() {
            match &self.tokens[i].token {
                Token::Eq => count += 1,
                _ => break,
            }
            if count >= 10 {
                return true;
            }
        }
        false
    }

    fn skip_separator_line(&mut self) {
        while self.pos < self.tokens.len() && matches!(self.tokens[self.pos].token, Token::Eq) {
            self.advance();
        }
    }

    fn find_next_semicolon_from(&self, start: usize) -> Option<usize> {
        for i in start..self.tokens.len() {
            if matches!(self.tokens[i].token, Token::Semicolon) {
                return Some(i);
            }
        }
        None
    }

    fn has_begin_after_declare(&self, declare_pos: usize) -> bool {
        let mut depth = 0i32;
        for i in (declare_pos + 1)..self.tokens.len().min(declare_pos + 500) {
            match &self.tokens[i].token {
                Token::LParen => depth += 1,
                Token::RParen => depth = (depth - 1).max(0),
                Token::Keyword(Keyword::BEGIN_P) if depth == 0 => return true,
                Token::Keyword(Keyword::DO) if depth == 0 => return true,
                _ => {}
            }
        }
        false
    }

    /// Check if tokens starting at `self.pos` form `CREATE [OR REPLACE] PACKAGE [BODY]`.
    /// DECLARE name [BINARY] [ASENSITIVE|INSENSITIVE] [[NO] SCROLL] CURSOR ...
    /// Scan forward up to 7 tokens for CURSOR; semicolon/BEGIN before CURSOR → anonymous block.
    fn is_declare_cursor_at(&self, pos: usize) -> bool {
        for i in 1..=7 {
            if let Some(t) = self.tokens.get(pos + i) {
                if matches!(t.token, Token::Keyword(Keyword::CURSOR)) {
                    return i > 1;
                }
                if matches!(t.token, Token::Semicolon | Token::Keyword(Keyword::BEGIN_P)) {
                    return false;
                }
            } else {
                return false;
            }
        }
        false
    }

    fn is_with_dml_at(&self, pos: usize) -> bool {
        let mut i = pos + 1;
        if i < self.tokens.len()
            && matches!(self.tokens[i].token, Token::Keyword(Keyword::RECURSIVE))
        {
            i += 1;
        }
        while i < self.tokens.len() {
            match &self.tokens[i].token {
                Token::Keyword(Keyword::INSERT)
                | Token::Keyword(Keyword::UPDATE)
                | Token::Keyword(Keyword::DELETE_P) => {
                    return true;
                }
                Token::Semicolon => return false,
                Token::Keyword(Keyword::SELECT) => return false,
                Token::LParen => {
                    i += 1;
                    let mut depth = 1i32;
                    while i < self.tokens.len() {
                        match &self.tokens[i].token {
                            Token::LParen => depth += 1,
                            Token::RParen => {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            Token::Semicolon => return false,
                            _ => {}
                        }
                        i += 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }
        false
    }

    fn detect_package_context(&self) -> bool {
        self.detect_package_context_at(self.pos)
    }

    /// Check if tokens at `pos` form `CREATE [OR REPLACE] PACKAGE [BODY]`.
    fn detect_package_context_at(&self, pos: usize) -> bool {
        let mut i = pos;
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

    /// Check if `IS`/`AS` at position `i` is the body marker for a top-level
    /// CREATE [OR REPLACE] FUNCTION/PROCEDURE (not inside a package, not CASE..IS, etc).
    fn is_routine_body_marker(&self, i: usize) -> bool {
        let mut j = i;
        // Walk backward looking for CREATE ... FUNCTION/PROCEDURE before this IS/AS.
        // The pattern: CREATE [OR REPLACE] FUNCTION/PROCEDURE name [(params)] [RETURN type] IS/AS
        let mut paren_depth = 0i32;
        while j > 0 {
            j -= 1;
            match &self.tokens[j].token {
                Token::Keyword(Keyword::FUNCTION) | Token::Keyword(Keyword::PROCEDURE)
                    if paren_depth == 0 =>
                {
                    // Check if CREATE [OR REPLACE] precedes this FUNCTION/PROCEDURE
                    let mut k = j;
                    if k > 0 && matches!(self.tokens[k - 1].token, Token::Keyword(Keyword::REPLACE)) {
                        k -= 1;
                    }
                    if k > 0 && matches!(self.tokens[k - 1].token, Token::Keyword(Keyword::OR)) {
                        k -= 1;
                    }
                    if k > 0 && matches!(self.tokens[k - 1].token, Token::Keyword(Keyword::CREATE)) {
                        return true;
                    }
                    return false;
                }
                Token::LParen => paren_depth += 1,
                Token::RParen => paren_depth -= 1,
                Token::Semicolon | Token::Keyword(Keyword::BEGIN_P) | Token::Keyword(Keyword::END_P)
                    if paren_depth == 0 =>
                {
                    return false;
                }
                _ => {}
            }
        }
        false
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
                Token::Semicolon | Token::Slash => {
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
                expected: kw.as_str().to_string(),
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

    pub(crate) fn peek_keyword_at(&self, offset: usize) -> Option<Keyword> {
        if let Token::Keyword(kw) = self
            .tokens
            .get(self.pos + offset)
            .map(|t| &t.token)
            .unwrap_or(&Token::Eof)
        {
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
            Token::Keyword(kw) => kw.as_str().eq_ignore_ascii_case(target),
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

    fn expect_ident_str(&mut self, target: &str) -> Result<(), ParserError> {
        if self.try_consume_ident_str(target) {
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: target.to_string(),
                got: format!("{:?}", self.peek()),
            })
        }
    }

    fn consume_any_identifier(&mut self) -> Result<String, ParserError> {
        match self.peek().clone() {
            Token::Ident(s) | Token::QuotedIdent(s) => {
                self.advance();
                Ok(s)
            }
            Token::Keyword(kw) => {
                self.advance();
                Ok(kw.as_str().to_string())
            }
            _ => Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "identifier".to_string(),
                got: format!("{:?}", self.peek()),
            }),
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
                let name = kw.as_str().to_string();

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
                Token::Keyword(kw) => {
                    if kw.category() != crate::token::keyword::KeywordCategory::Reserved
                        && !self.is_clause_keyword(kw)
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
                Token::Ident(_) | Token::QuotedIdent(_) => {
                    if self.looks_like_alias() || self.is_bulk_collect_ahead() {
                        Ok(Some(self.parse_identifier()?))
                    } else {
                        Ok(None)
                    }
                }
                Token::Keyword(kw) => {
                    if kw.category() != crate::token::keyword::KeywordCategory::Reserved
                        && (self.looks_like_alias() || self.is_bulk_collect_ahead())
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

    fn is_bulk_collect_ahead(&self) -> bool {
        if self.pos + 1 >= self.tokens.len() {
            return false;
        }
        let bulk_match = match &self.tokens[self.pos + 1].token {
            Token::Ident(s) => s.eq_ignore_ascii_case("BULK"),
            Token::Keyword(kw) => kw.as_str().eq_ignore_ascii_case("BULK"),
            _ => false,
        };
        if !bulk_match || self.pos + 2 >= self.tokens.len() {
            return false;
        }
        match &self.tokens[self.pos + 2].token {
            Token::Ident(s) => s.eq_ignore_ascii_case("COLLECT"),
            Token::Keyword(kw) => kw.as_str().eq_ignore_ascii_case("COLLECT"),
            _ => false,
        }
    }

    /// Check if the keyword is known to start a SQL clause and should never be
    /// consumed as an implicit table alias (without AS).
    fn is_clause_keyword(&self, kw: &Keyword) -> bool {
        matches!(
            kw,
            Keyword::PARTITION
                | Keyword::SUBPARTITION
                | Keyword::CONNECT
                | Keyword::START
                | Keyword::SET
                | Keyword::VALUES
                | Keyword::UPDATE
                | Keyword::DELETE_P
                | Keyword::INSERT
                | Keyword::SELECT
                | Keyword::WITH
                | Keyword::FROM
                | Keyword::WHERE
                | Keyword::RETURNING
                | Keyword::HAVING
                | Keyword::GROUP_P
                | Keyword::ORDER
                | Keyword::LIMIT
                | Keyword::OFFSET
                | Keyword::UNION
                | Keyword::INTERSECT
                | Keyword::EXCEPT
                | Keyword::MINUS_P
                | Keyword::FOR
                | Keyword::USING
                | Keyword::ON
                | Keyword::WHEN
                | Keyword::THEN
                | Keyword::ELSE
                | Keyword::END_P
                | Keyword::CASE
                | Keyword::LOOP
                | Keyword::WHILE_P
                | Keyword::IF_P
                | Keyword::MERGE
                | Keyword::MATCHED
                | Keyword::CROSS
                | Keyword::INNER_P
                | Keyword::LEFT
                | Keyword::RIGHT
                | Keyword::FULL
                | Keyword::NATURAL
                | Keyword::JOIN
        )
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
                    // JOIN keywords — a table alias can be followed by any join clause
                    | Keyword::LEFT
                    | Keyword::RIGHT
                    | Keyword::FULL
                    | Keyword::INNER_P
                    | Keyword::JOIN
                    | Keyword::CROSS
                    | Keyword::NATURAL
            ),
            _ => false,
        }
    }

    fn is_pl_boundary(&self) -> bool {
        matches!(self.peek(), Token::Eof | Token::Semicolon)
            || matches!(
                self.peek_keyword(),
                Some(Keyword::END_P)
                    | Some(Keyword::ELSE)
                    | Some(Keyword::WHEN)
                    | Some(Keyword::THEN)
                    | Some(Keyword::LOOP)
            )
            || self.match_ident_str("exception")
            || self.match_ident_str("elsif")
    }

    /// Check if the current token starts an expression.
    pub(crate) fn is_expr_start(&self) -> bool {
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
        let start = self.current_location();
        let stmt = match self.peek().clone() {
            Token::Keyword(Keyword::SELECT) => {
                let pre_hints = self.consume_hints();
                match self.parse_select_statement() {
                    Ok(mut stmt) => {
                        let mut hints = pre_hints;
                        hints.append(&mut stmt.hints);
                        stmt.hints = hints;
                        self.try_consume_semicolon();
                        crate::ast::Statement::Select(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::WITH) => {
                if self.is_with_dml_at(self.pos) {
                    let with = match self.parse_with_clause() {
                        Ok(Some(w)) => w,
                        Ok(None) => unreachable!("is_with_dml_at confirmed WITH"),
                        Err(e) => {
                            self.add_error(e);
                            return Ok(self.skip_to_semicolon());
                        }
                    };
                    match self.peek_keyword() {
                        Some(Keyword::INSERT) => {
                            self.advance();
                            match self.parse_insert() {
                                Ok(mut stmt) => {
                                    stmt.with = Some(with);
                                    self.try_consume_semicolon();
                                    crate::ast::Statement::Insert(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                                }
                                Err(e) => {
                                    self.add_error(e);
                                    self.skip_to_semicolon()
                                }
                            }
                        }
                        Some(Keyword::UPDATE) => {
                            self.advance();
                            match self.parse_update() {
                                Ok(mut stmt) => {
                                    stmt.with = Some(with);
                                    self.try_consume_semicolon();
                                    crate::ast::Statement::Update(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                                }
                                Err(e) => {
                                    self.add_error(e);
                                    self.skip_to_semicolon()
                                }
                            }
                        }
                        Some(Keyword::DELETE_P) => {
                            self.advance();
                            match self.parse_delete() {
                                Ok(mut stmt) => {
                                    stmt.with = Some(with);
                                    self.try_consume_semicolon();
                                    crate::ast::Statement::Delete(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                                }
                                Err(e) => {
                                    self.add_error(e);
                                    self.skip_to_semicolon()
                                }
                            }
                        }
                        _ => {
                            self.add_error(ParserError::UnexpectedToken {
                                location: self.current_location(),
                                expected: "INSERT, UPDATE, or DELETE after WITH clause".to_string(),
                                got: format!("{:?}", self.peek()),
                            });
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    let pre_hints = self.consume_hints();
                    match self.parse_select_statement() {
                        Ok(mut stmt) => {
                            let mut hints = pre_hints;
                            hints.append(&mut stmt.hints);
                            stmt.hints = hints;
                            self.try_consume_semicolon();
                            crate::ast::Statement::Select(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
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
                            crate::ast::Statement::InsertAll(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                            crate::ast::Statement::InsertFirst(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                            crate::ast::Statement::Insert(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Update(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Delete(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Merge(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Truncate(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::CreateProcedure(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::CreateFunction(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::VariableSet(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::VariableShow(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::VariableReset(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                            crate::ast::Statement::Transaction(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                            crate::ast::Statement::AnonyBlock(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Transaction(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::COMMIT) | Token::Keyword(Keyword::END_P) => {
                self.advance();
                if self.match_keyword(Keyword::PREPARED) {
                    self.advance();
                    let transaction_id = self.parse_string_literal()?;
                    self.try_consume_semicolon();
                    crate::ast::Statement::Transaction(crate::ast::Spanned::new(crate::ast::TransactionStatement {
                        kind: crate::ast::TransactionKind::CommitPrepared,
                        modes: vec![],
                        savepoint_name: None,
                        transaction_id: Some(transaction_id),
                    }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                } else {
                    match self.parse_transaction_commit() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::Transaction(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Token::Keyword(Keyword::ROLLBACK) => {
                self.advance();
                if self.match_keyword(Keyword::PREPARED) {
                    self.advance();
                    let transaction_id = self.parse_string_literal()?;
                    self.try_consume_semicolon();
                    crate::ast::Statement::Transaction(crate::ast::Spanned::new(crate::ast::TransactionStatement {
                        kind: crate::ast::TransactionKind::RollbackPrepared,
                        modes: vec![],
                        savepoint_name: None,
                        transaction_id: Some(transaction_id),
                    }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                } else {
                    match self.parse_transaction_rollback() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::Transaction(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Token::Keyword(Keyword::SAVEPOINT) => {
                self.advance();
                match self.parse_savepoint() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Transaction(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Discard(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Copy(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                    Ok(stmt) => crate::ast::Statement::Explain(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                        crate::ast::Statement::Call(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                            crate::ast::Statement::GrantRole(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                            crate::ast::Statement::Grant(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                            crate::ast::Statement::RevokeRole(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                            crate::ast::Statement::Revoke(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Vacuum(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Do(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::PREPARE) => {
                self.advance();
                if self.match_keyword(Keyword::TRANSACTION) {
                    self.advance();
                    let transaction_id = self.parse_string_literal()?;
                    self.try_consume_semicolon();
                    crate::ast::Statement::Transaction(crate::ast::Spanned::new(crate::ast::TransactionStatement {
                        kind: crate::ast::TransactionKind::PrepareTransaction,
                        modes: vec![],
                        savepoint_name: None,
                        transaction_id: Some(transaction_id),
                    }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                } else {
                    match self.parse_prepare() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::Prepare(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Token::Keyword(Keyword::EXECUTE) => {
                self.advance();
                if self.match_keyword(Keyword::DIRECT) || self.match_ident_str("direct") {
                    self.advance();
                    self.expect_keyword(Keyword::ON)?;
                    self.expect_token(&Token::LParen)?;
                    let node_name = self.parse_identifier()?;
                    self.expect_token(&Token::RParen)?;
                    let query = self.parse_string_literal()?;
                    self.try_consume_semicolon();
                    crate::ast::Statement::ExecuteDirect(crate::ast::Spanned::new(crate::ast::ExecuteDirectStatement {
                        node_name,
                        query,
                    }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                } else {
                    match self.parse_execute() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::Execute(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Token::Keyword(Keyword::DEALLOCATE) => {
                self.advance();
                match self.parse_deallocate() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Deallocate(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Comment(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Lock(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                            crate::ast::Statement::RefreshMaterializedView(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                            crate::ast::Statement::RefreshMaterializedView(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Fetch(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::DECLARE) => {
                if !self.is_declare_cursor_at(self.pos) && self.has_begin_after_declare(self.pos) {
                    self.advance();
                    let declarations = self.parse_pl_declarations_until_begin()?;
                    let block = self.parse_pl_block_body(None, declarations)?;
                    self.try_consume_semicolon();
                    crate::ast::Statement::AnonyBlock(crate::ast::Spanned::new(crate::ast::AnonyBlockStatement { block }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                } else if self.is_declare_cursor_at(self.pos) {
                    self.advance();
                    match self.parse_declare_cursor() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::DeclareCursor(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    self.advance();
                    crate::ast::Statement::Empty
                }
            }
            Token::Keyword(Keyword::CLOSE) => {
                self.advance();
                match self.parse_close_portal() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::ClosePortal(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Cluster(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::CURSOR) => {
                self.advance();
                match self.parse_declare_cursor() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::DeclareCursor(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Reindex(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Listen(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Notify(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Unlisten(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Rule(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Analyze(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Shutdown(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Barrier(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Purge(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Snapshot(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::TimeCapsule(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Shrink(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Verify(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::Compile(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::CleanConn(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::SecLabel(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::ABORT_P) => {
                self.advance();
                self.try_consume_keyword(Keyword::WORK);
                self.try_consume_keyword(Keyword::TRANSACTION);
                self.try_consume_semicolon();
                crate::ast::Statement::Abort
            }
            Token::Keyword(Keyword::VALUES) => {
                self.advance();
                match self.parse_values_statement() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Values(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::REPLACE) => {
                self.advance();
                match self.parse_insert() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Replace(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::PREDICT) => {
                self.advance();
                self.expect_keyword(Keyword::BY).unwrap_or(());
                let model_result = self.parse_identifier();
                match model_result {
                    Ok(model) => {
                        let mut features = Vec::new();
                        if self.match_keyword(Keyword::FEATURES) {
                            self.advance();
                            if self.match_token(&Token::LParen) {
                                self.advance();
                                if let Ok(f) = self.parse_identifier() {
                                    features.push(f);
                                }
                                while self.match_token(&Token::Comma) {
                                    self.advance();
                                    if let Ok(f) = self.parse_identifier() {
                                        features.push(f);
                                    } else {
                                        break;
                                    }
                                }
                                if self.match_token(&Token::RParen) {
                                    self.advance();
                                }
                            }
                        }
                        let using_clause = if self.match_keyword(Keyword::USING) {
                            self.advance();
                            Some(self.skip_to_semicolon_and_collect())
                        } else {
                            None
                        };
                        self.try_consume_semicolon();
                        crate::ast::Statement::PredictBy(crate::ast::Spanned::new(crate::ast::PredictByStatement {
                            model,
                            features,
                            using_clause,
                        }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::REASSIGN) => {
                self.advance();
                self.expect_keyword(Keyword::OWNED).unwrap_or(());
                self.expect_keyword(Keyword::BY).unwrap_or(());
                let old_result = self.parse_identifier();
                match old_result {
                    Ok(old_role) => {
                        self.expect_keyword(Keyword::TO).unwrap_or(());
                        match self.parse_identifier() {
                            Ok(new_role) => {
                                self.try_consume_semicolon();
                                crate::ast::Statement::ReassignOwned(crate::ast::Spanned::new(
                                    crate::ast::ReassignOwnedStatement { old_role, new_role }, Some(crate::ast::SourceSpan { start, end: self.prev_location() }),
                                ))
                            }
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::MOVE) => {
                self.advance();
                match self.parse_move_cursor() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Move(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
            Token::Ident(ref s) if s.eq_ignore_ascii_case("lock") => {
                self.advance();
                if self.match_ident_str("buckets") {
                    self.advance();
                    let raw = self.skip_to_semicolon_and_collect();
                    crate::ast::Statement::LockBuckets(crate::ast::Spanned::new(crate::ast::LockBucketsStatement {
                        raw_rest: raw,
                    }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                } else {
                    self.skip_to_semicolon()
                }
            }
            Token::Ident(ref s) if s.eq_ignore_ascii_case("mark") => {
                self.advance();
                if self.match_ident_str("buckets") {
                    self.advance();
                    let raw = self.skip_to_semicolon_and_collect();
                    crate::ast::Statement::MarkBuckets(crate::ast::Spanned::new(crate::ast::MarkBucketsStatement {
                        raw_rest: raw,
                    }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                } else {
                    self.skip_to_semicolon()
                }
            }
            Token::Ident(ref s) if s.eq_ignore_ascii_case("expdp") => {
                self.advance();
                let raw = self.skip_to_semicolon_and_collect();
                if raw.starts_with("database") || raw.starts_with("DATABASE") {
                    crate::ast::Statement::ExpdpDatabase(crate::ast::Spanned::new(crate::ast::ExpdpDatabaseStatement {
                        raw_rest: raw,
                    }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                } else {
                    crate::ast::Statement::ExpdpTable(crate::ast::Spanned::new(crate::ast::ExpdpTableStatement {
                        raw_rest: raw,
                    }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                }
            }
            Token::Ident(ref s) if s.eq_ignore_ascii_case("impdp") => {
                self.advance();
                let raw = self.skip_to_semicolon_and_collect();
                if raw.starts_with("database")
                    || raw.starts_with("DATABASE")
                    || raw.starts_with("recover")
                    || raw.starts_with("RECOVER")
                {
                    crate::ast::Statement::ImpdpDatabase(crate::ast::Spanned::new(crate::ast::ImpdpDatabaseStatement {
                        raw_rest: raw,
                    }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                } else {
                    crate::ast::Statement::ImpdpTable(crate::ast::Spanned::new(crate::ast::ImpdpTableStatement {
                        raw_rest: raw,
                    }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                }
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
            let key = self.consume_any_identifier().unwrap_or_default();
            if self.match_token(&Token::Eq) {
                self.advance();
            }
            let value = match self.peek().clone() {
                Token::StringLiteral(s) | Token::QuotedIdent(s) => {
                    self.advance();
                    s
                }
                Token::Ident(s) => {
                    self.advance();
                    s
                }
                Token::Keyword(kw) => {
                    self.advance();
                    kw.as_str().to_string()
                }
                Token::Integer(n) => {
                    self.advance();
                    n.to_string()
                }
                Token::Float(f) => {
                    self.advance();
                    f.to_string()
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
            let key = self.consume_any_identifier().unwrap_or_default();
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
                    let mut val = s;
                    while self.match_token(&Token::Dot) {
                        self.advance();
                        val.push('.');
                        val.push_str(&self.consume_any_identifier().unwrap_or_default());
                    }
                    val
                }
                Token::Keyword(kw) => {
                    self.advance();
                    let mut val = kw.as_str().to_string();
                    while self.match_token(&Token::Dot) {
                        self.advance();
                        val.push('.');
                        val.push_str(&self.consume_any_identifier().unwrap_or_default());
                    }
                    val
                }
                Token::Integer(n) => {
                    self.advance();
                    n.to_string()
                }
                Token::Float(f) => {
                    self.advance();
                    f.to_string()
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

    /// Parse `OPTIONS (...)` or `WITH (...)` clause. Handles `key value`, `key = value`, and `SET/DROP/ADD key value`.
    pub(crate) fn parse_options_clause(&mut self) -> Vec<(String, String)> {
        if !self.try_consume_keyword(Keyword::OPTIONS) {
            return self.parse_generic_options();
        }
        if !self.match_token(&Token::LParen) {
            return Vec::new();
        }
        self.advance();
        let mut options = Vec::new();
        loop {
            if self.match_token(&Token::RParen) {
                self.advance();
                break;
            }
            let action = if self.match_keyword(Keyword::SET)
                || self.match_keyword(Keyword::ADD_P)
                || self.match_keyword(Keyword::DROP)
            {
                let act = format!("{:?}", self.peek_keyword().unwrap()).to_lowercase();
                self.advance();
                act
            } else {
                String::new()
            };
            let key = self.consume_any_identifier().unwrap_or_default();
            let full_key = if action.is_empty() {
                key
            } else {
                format!("{} {}", action, key)
            };
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
                    kw.as_str().to_string()
                }
                Token::Integer(n) => {
                    self.advance();
                    n.to_string()
                }
                Token::Float(f) => {
                    self.advance();
                    f.to_string()
                }
                _ => String::new(),
            };
            options.push((full_key, value));
            if self.match_token(&Token::Comma) {
                self.advance();
            } else if self.match_token(&Token::RParen) {
                self.advance();
                break;
            } else {
                break;
            }
        }
        options
    }

    fn parse_create_foreign(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let start = self.current_location();
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
                Ok(crate::ast::Statement::CreateFdw(crate::ast::Spanned::new(
                    crate::ast::CreateFdwStatement {
                        name,
                        handler,
                        validator,
                        options,
                    },
                    Some(crate::ast::SourceSpan { start, end: self.prev_location() }))))
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
                Ok(crate::ast::Statement::CreateForeignServer(crate::ast::Spanned::new(
                    crate::ast::CreateForeignServerStatement {
                        name,
                        server_type,
                        version,
                        fdw_name,
                        options,
                    },
                    Some(crate::ast::SourceSpan { start, end: self.prev_location() }))))
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
                            charset: None,
                            collate: None,
                            on_update: None,
                            comment: None,
                            generated: None,
                            encrypted_with: None,
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
                let options = self.parse_options_clause();
                self.try_consume_semicolon();
                Ok(crate::ast::Statement::CreateForeignTable(crate::ast::Spanned::new(
                    crate::ast::CreateForeignTableStatement {
                        name,
                        columns,
                        server_name,
                        options,
                    },
                    Some(crate::ast::SourceSpan { start, end: self.prev_location() }))))
            }
            _ => Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "TABLE, SERVER, or DATA WRAPPER".to_string(),
                got: format!("{:?}", self.peek()),
            }),
        }
    }

    fn parse_create_publication(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let start = self.current_location();
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
        Ok(crate::ast::Statement::CreatePublication(crate::ast::Spanned::new(
            crate::ast::CreatePublicationStatement {
                name,
                tables,
                all_tables,
                options,
            },
            Some(crate::ast::SourceSpan { start, end: self.prev_location() }))))
    }

    fn parse_create_subscription(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let start = self.current_location();
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
        Ok(crate::ast::Statement::CreateSubscription(crate::ast::Spanned::new(
            crate::ast::CreateSubscriptionStatement {
                name,
                connection,
                publications,
                options,
            },
            Some(crate::ast::SourceSpan { start, end: self.prev_location() }))))
    }

    fn parse_create_node_group_inner(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let start = self.current_location();
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
        Ok(crate::ast::Statement::CreateNodeGroup(crate::ast::Spanned::new(
            crate::ast::CreateNodeGroupStatement {
                name,
                nodes,
                options,
            },
            Some(crate::ast::SourceSpan { start, end: self.prev_location() }))))
    }

    fn parse_create_resource_pool_after_resource(
        &mut self,
    ) -> Result<crate::ast::Statement, ParserError> {
        let start = self.current_location();
        self.expect_keyword(Keyword::POOL)?;
        let name = self.parse_identifier()?;
        let options = self.parse_generic_options();
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::CreateResourcePool(crate::ast::Spanned::new(
            crate::ast::CreateResourcePoolStatement { name, options },
            Some(crate::ast::SourceSpan { start, end: self.prev_location() }))))
    }

    fn parse_create_resource_pool_inner(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let start = self.current_location();
        let name = self.parse_identifier()?;
        let options = self.parse_generic_options();
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::CreateResourcePool(crate::ast::Spanned::new(
            crate::ast::CreateResourcePoolStatement { name, options },
            Some(crate::ast::SourceSpan { start, end: self.prev_location() }))))
    }

    fn parse_create_workload_group(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let start = self.current_location();
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
        Ok(crate::ast::Statement::CreateWorkloadGroup(crate::ast::Spanned::new(
            crate::ast::CreateWorkloadGroupStatement {
                name,
                pool_name,
                options,
            },
            Some(crate::ast::SourceSpan { start, end: self.prev_location() }))))
    }

    fn parse_create_audit_policy(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let start = self.current_location();
        self.advance(); // AUDIT
        self.expect_keyword(Keyword::POLICY)?;
        let name = self.parse_identifier()?;
        let policy_type = self.parse_identifier()?;
        let mut privileges = Vec::new();
        let mut labels = Vec::new();

        loop {
            // Privilege names can be reserved keywords (CREATE, SELECT, etc.)
            // Consume without emitting reserved-keyword warning
            let privilege = match self.peek().clone() {
                Token::Ident(s) | Token::QuotedIdent(s) => {
                    self.advance();
                    s
                }
                Token::Keyword(kw) => {
                    self.advance();
                    kw.as_str().to_string()
                }
                _ => break,
            };
            privileges.push(privilege);

            if self.try_consume_keyword(Keyword::ON) {
                self.expect_keyword(Keyword::LABEL)?;
                self.expect_token(&Token::LParen)?;
                let label = self.parse_identifier()?;
                labels.push(label);
                self.expect_token(&Token::RParen)?;
            }

            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }

        if self.try_consume_keyword(Keyword::FILTER) {
            self.expect_keyword(Keyword::ON)?;
            while !self.match_keyword(Keyword::WITH)
                && !self.match_token(&Token::Semicolon)
                && !self.match_token(&Token::Eof)
            {
                if self.match_token(&Token::LParen) {
                    self.skip_to_paren_end();
                } else {
                    self.advance();
                }
                if self.match_token(&Token::Comma) {
                    self.advance();
                }
            }
        }

        let options = self.parse_generic_options();
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::CreateAuditPolicy(crate::ast::Spanned::new(
            crate::ast::CreateAuditPolicyStatement {
                name,
                policy_type,
                privileges,
                labels,
                options,
            },
            Some(crate::ast::SourceSpan { start, end: self.prev_location() }))))
    }

    fn parse_masking_filter_clauses(
        &mut self,
    ) -> Result<Vec<crate::ast::FilterClause>, ParserError> {
        let mut clauses = Vec::new();
        loop {
            // Parse kind: ROLES, APP, IP
            let kind = self.parse_identifier()?;
            self.expect_token(&Token::LParen)?;
            let mut values = Vec::new();
            loop {
                // Accept identifiers or string literals as values
                let val = match self.peek().clone() {
                    Token::Ident(s) => {
                        self.advance();
                        s
                    }
                    Token::QuotedIdent(s) => {
                        self.advance();
                        s
                    }
                    Token::Keyword(kw) => {
                        self.advance();
                        kw.as_str().to_string()
                    }
                    Token::StringLiteral(s) => {
                        self.advance();
                        s
                    }
                    _ => self.parse_identifier()?,
                };
                values.push(val);
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect_token(&Token::RParen)?;
            clauses.push(crate::ast::FilterClause {
                kind: kind.to_uppercase(),
                values,
            });
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }
        Ok(clauses)
    }

    fn parse_create_masking_policy(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let start = self.current_location();
        self.advance(); // MASKING
        self.expect_keyword(Keyword::POLICY)?;
        let name = self.parse_identifier()?;
        let mut masking_function = None;
        let mut function_args = Vec::new();
        let mut labels = Vec::new();
        let mut filter_clauses = Vec::new();
        if !self.match_keyword(Keyword::WITH)
            && !self.match_keyword(Keyword::ON)
            && !self.match_keyword(Keyword::FILTER)
            && !matches!(self.peek(), Token::LParen | Token::Semicolon | Token::Eof)
        {
            masking_function = Some(self.parse_identifier()?);
            // Parse optional function arguments: (arg1, arg2, ...)
            if self.match_token(&Token::LParen) {
                self.advance();
                if !self.match_token(&Token::RParen) {
                    loop {
                        function_args.push(self.parse_expr()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                        self.advance();
                    }
                }
                self.expect_token(&Token::RParen)?;
            }
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
        // Parse FILTER ON ROLES(...), APP(...), IP(...)
        if self.try_consume_keyword(Keyword::FILTER) {
            self.expect_keyword(Keyword::ON)?;
            filter_clauses = self.parse_masking_filter_clauses()?;
        }
        let options = self.parse_generic_options();
        self.try_consume_semicolon();
        Ok(crate::ast::Statement::CreateMaskingPolicy(crate::ast::Spanned::new(
            crate::ast::CreateMaskingPolicyStatement {
                name,
                masking_function,
                function_args,
                labels,
                filter_clauses,
                options,
            },
            Some(crate::ast::SourceSpan { start, end: self.prev_location() }))))
    }

    fn parse_create_rls_policy(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let start = self.current_location();
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
        Ok(crate::ast::Statement::CreateRlsPolicy(crate::ast::Spanned::new(
            crate::ast::CreateRlsPolicyStatement {
                name,
                table,
                permissive,
                using_expr,
            },
            Some(crate::ast::SourceSpan { start, end: self.prev_location() }))))
    }

    fn parse_create_resource_label(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let start = self.current_location();
        self.advance(); // LABEL
        let if_not_exists = self.parse_if_not_exists();
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
        let label_type = match self.peek_keyword() {
            Some(Keyword::TABLE)
            | Some(Keyword::COLUMN)
            | Some(Keyword::SCHEMA)
            | Some(Keyword::VIEW)
            | Some(Keyword::FUNCTION) => {
                let kw = self.peek_keyword().unwrap();
                self.advance();
                kw.as_str().to_string()
            }
            _ => self.consume_any_identifier()?,
        };
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
        Ok(crate::ast::Statement::CreatePolicyLabel(crate::ast::Spanned::new(
            crate::ast::CreatePolicyLabelStatement {
                if_not_exists,
                name,
                add,
                label_type,
                targets,
            },
            Some(crate::ast::SourceSpan { start, end: self.prev_location() }))))
    }

    fn parse_alter_masking_policy(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let start = self.current_location();
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
            // Two forms:
            //   MODIFY function ON LABEL (label, ...)
            //   MODIFY ( FILTER ON ROLES(...), APP(...), IP(...) )
            if self.match_token(&Token::LParen) {
                // MODIFY ( FILTER ON ... )
                self.advance();
                self.expect_keyword(Keyword::FILTER)?;
                self.expect_keyword(Keyword::ON)?;
                let filter_clauses = self.parse_masking_filter_clauses()?;
                self.expect_token(&Token::RParen)?;
                crate::ast::AlterMaskingPolicyAction::ModifyFilter { filter_clauses }
            } else {
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
            }
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
        Ok(crate::ast::Statement::AlterMaskingPolicy(crate::ast::Spanned::new(
            crate::ast::AlterMaskingPolicyStatement { name, action },
            Some(crate::ast::SourceSpan { start, end: self.prev_location() }))))
    }

    fn parse_alter_resource_label(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let start = self.current_location();
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
        let label_type = self.consume_any_identifier()?;
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
        Ok(crate::ast::Statement::AlterPolicyLabel(crate::ast::Spanned::new(
            crate::ast::AlterPolicyLabelStatement {
                name,
                add,
                label_type,
                targets,
            },
            Some(crate::ast::SourceSpan { start, end: self.prev_location() }))))
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
        let start = self.current_location();
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
        let unique = self.try_consume_keyword(Keyword::UNIQUE);

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
                        Ok(stmt) => crate::ast::Statement::CreateTable(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Some(Keyword::GLOBAL) => {
                self.advance();
                // Check for GLOBAL TEMPORARY TABLE
                if self.try_consume_keyword(Keyword::TEMPORARY)
                    || self.try_consume_keyword(Keyword::TEMP)
                {
                    match self.parse_create_table(true, unlogged) {
                        Ok(stmt) => crate::ast::Statement::CreateTable(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    match self.parse_create_global_index() {
                        Ok(stmt) => crate::ast::Statement::CreateGlobalIndex(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Some(Keyword::INDEX) => match self.parse_create_index(unique) {
                Ok(stmt) => crate::ast::Statement::CreateIndex(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::SEQUENCE) => match self.parse_create_sequence() {
                Ok(stmt) => crate::ast::Statement::CreateSequence(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                    crate::ast::Statement::CreateView(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                }
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::SCHEMA) => match self.parse_create_schema() {
                Ok(stmt) => crate::ast::Statement::CreateSchema(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                        Ok(stmt) => crate::ast::Statement::CreateDatabaseLink(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    match self.parse_create_database() {
                        Ok(stmt) => crate::ast::Statement::CreateDatabase(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Some(Keyword::TABLESPACE) => match self.parse_create_tablespace() {
                Ok(stmt) => crate::ast::Statement::CreateTablespace(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                        crate::ast::Statement::CreateFunction(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Some(Keyword::PROCEDURE) => {
                self.advance();
                match self.parse_create_procedure() {
                    Ok(mut stmt) => {
                        stmt.replace = replace;
                        self.try_consume_semicolon();
                        crate::ast::Statement::CreateProcedure(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Some(Keyword::TRIGGER) => {
                self.advance();
                match self.parse_create_trigger() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::CreateTrigger(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        Ok(stmt) => crate::ast::Statement::CreateMaterializedView(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                    Ok(stmt) => crate::ast::Statement::CreateMaterializedView(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Some(Keyword::EXTENSION) => match self.parse_create_extension() {
                Ok(stmt) => crate::ast::Statement::CreateExtension(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::ROLE) => {
                self.advance();
                match self.parse_create_role_options() {
                    Ok((name, options)) => {
                        crate::ast::Statement::CreateRole(crate::ast::Spanned::new(crate::ast::CreateRoleStatement {
                            name, 
                            options,
                        }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                            crate::ast::Statement::CreateUserMapping(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    match self.parse_create_role_options() {
                        Ok((name, options)) => {
                            crate::ast::Statement::CreateUser(crate::ast::Spanned::new(crate::ast::CreateUserStatement {
                                name, 
                                options,
                            }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::CreateGroup(crate::ast::Spanned::new(crate::ast::CreateGroupStatement {
                            name, 
                            options,
                        }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                Ok(stmt) => crate::ast::Statement::CreateType(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::CAST) => match self.parse_create_cast() {
                Ok(stmt) => crate::ast::Statement::CreateCast(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::DOMAIN_P) => match self.parse_create_domain() {
                Ok(stmt) => crate::ast::Statement::CreateDomain(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                    crate::ast::Statement::CreateNode(crate::ast::Spanned::new(crate::ast::CreateNodeStatement {
                        name, 
                        options,
                    }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::CreateAggregate(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        Ok(stmt) => crate::ast::Statement::CreateOpClass(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else if self.match_keyword(Keyword::FAMILY) {
                    match self.parse_create_opfamily() {
                        Ok(stmt) => crate::ast::Statement::CreateOpFamily(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else {
                    match self.parse_create_operator() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::CreateOperator(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
            }
            Some(Keyword::CONVERSION_P) => match self.parse_create_conversion() {
                Ok(stmt) => crate::ast::Statement::CreateConversion(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::SYNONYM) => match self.parse_create_synonym(replace) {
                Ok(stmt) => crate::ast::Statement::CreateSynonym(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::MODEL) => match self.parse_create_model() {
                Ok(stmt) => crate::ast::Statement::CreateModel(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::ACCESS) => match self.parse_create_am() {
                Ok(stmt) => crate::ast::Statement::CreateAm(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::DIRECTORY) => match self.parse_create_directory() {
                Ok(stmt) => crate::ast::Statement::CreateDirectory(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::DATA_P) => {
                self.advance();
                if self.match_keyword(Keyword::SOURCE_P) {
                    match self.parse_create_data_source() {
                        Ok(stmt) => crate::ast::Statement::CreateDataSource(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                Ok(stmt) => crate::ast::Statement::CreateEvent(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::STREAM) => match self.parse_create_stream() {
                Ok(stmt) => crate::ast::Statement::CreateStream(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::KEY) => match self.parse_create_key() {
                Ok(stmt) => crate::ast::Statement::CreateKey(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                Err(e) => {
                    self.add_error(e);
                    self.skip_to_semicolon()
                }
            },
            Some(Keyword::LANGUAGE) => {
                self.advance();
                match self.parse_create_language() {
                    Ok(stmt) => crate::ast::Statement::CreateLanguage(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Some(Keyword::PROCEDURAL) => {
                self.advance();
                self.expect_keyword(Keyword::LANGUAGE).unwrap_or(());
                match self.parse_create_language() {
                    Ok(stmt) => crate::ast::Statement::CreateLanguage(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            _ => {
                if self.match_ident_str("WEAK") {
                    self.advance();
                    if self.match_keyword(Keyword::PASSWORD) {
                        self.advance();
                    }
                    if self.match_keyword(Keyword::DICTIONARY) {
                        self.advance();
                    }
                    let mut values = Vec::new();
                    self.try_consume_keyword(Keyword::WITH);
                    self.try_consume_keyword(Keyword::VALUES);
                    loop {
                        if !self.match_token(&Token::LParen) {
                            break;
                        }
                        self.advance();
                        match self.parse_string_literal() {
                            Ok(v) => values.push(v),
                            Err(e) => {
                                self.add_error(e);
                                return self.skip_to_semicolon();
                            }
                        }
                        if self.match_token(&Token::RParen) {
                            self.advance();
                        }
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                        self.advance();
                    }
                    crate::ast::Statement::CreateWeakPasswordDictionaryWithValues(crate::ast::Spanned::new(crate::ast::CreateWeakPasswordDictStatement { values },  Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                } else if self.match_ident_str("CONTINUOUS") {
                    self.advance();
                    if self.match_keyword(Keyword::QUERY) || self.match_ident_str("QUERY") {
                        self.advance();
                    }
                    match self.parse_create_contquery() {
                        Ok(stmt) => crate::ast::Statement::CreateContQuery(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                } else if self.match_keyword(Keyword::TEXT_P) {
                    self.advance();
                    self.expect_keyword(Keyword::SEARCH).unwrap_or(());
                    if self.match_ident_str("configuration") {
                        self.advance();
                        let name = match self.parse_object_name() {
                            Ok(n) => n,
                            Err(e) => {
                                self.add_error(e);
                                return self.skip_to_semicolon();
                            }
                        };
                        let raw = self.skip_to_semicolon_and_collect();
                        crate::ast::Statement::CreateTextSearchConfig(crate::ast::Spanned::new(crate::ast::CreateTextSearchConfigStatement {
                                name, 
                                parser_name: None,
                                raw_rest: raw,
                            }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    } else if self.match_ident_str("dictionary") {
                        self.advance();
                        let name = match self.parse_object_name() {
                            Ok(n) => n,
                            Err(e) => {
                                self.add_error(e);
                                return self.skip_to_semicolon();
                            }
                        };
                        let options = self.parse_generic_options_no_with();
                        crate::ast::Statement::CreateTextSearchDict(crate::ast::Spanned::new(crate::ast::CreateTextSearchDictStatement { name,  options }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    } else {
                        self.skip_to_semicolon()
                    }
                } else if self.match_keyword(Keyword::APP) {
                    self.advance();
                    if self.match_keyword(Keyword::WORKLOAD) {
                        self.advance();
                    }
                    if self.match_keyword(Keyword::GROUP_P) {
                        self.advance();
                    }
                    if self.match_ident_str("mapping") {
                        self.advance();
                    }
                    let name = match self.parse_identifier() {
                        Ok(n) => n,
                        Err(e) => {
                            self.add_error(e);
                            return self.skip_to_semicolon();
                        }
                    };
                    let raw = self.skip_to_semicolon_and_collect();
                    crate::ast::Statement::CreateAppWorkloadGroupMapping(crate::ast::Spanned::new(crate::ast::CreateAppWorkloadGroupMappingStatement {
                            name, 
                            raw_rest: raw,
                        }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                } else {
                    self.skip_to_semicolon()
                }
            }
        }
    }

    fn dispatch_alter(&mut self) -> crate::ast::Statement {
        let start = self.current_location();
        if self.match_keyword(Keyword::DEFAULT) {
            self.advance();
            match self.parse_alter_default_privileges() {
                Ok(stmt) => {
                    self.try_consume_semicolon();
                    crate::ast::Statement::AlterDefaultPrivileges(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::AlterIndex(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::TABLE) => match self.parse_alter_table() {
                    Ok(stmt) => crate::ast::Statement::AlterTable(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                            crate::ast::Statement::AlterTablespace(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::DATABASE) => {
                    let saved_pos = self.pos;
                    self.advance();
                    if self.match_ident_str("link") {
                        self.advance();
                        match self.parse_alter_database_link() {
                            Ok(stmt) => {
                                self.try_consume_semicolon();
                                crate::ast::Statement::AlterDatabaseLink(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                            }
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else {
                        self.pos = saved_pos;
                        match self.parse_alter_database() {
                            Ok(stmt) => {
                                self.try_consume_semicolon();
                                crate::ast::Statement::AlterDatabase(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                            }
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    }
                }
                Some(Keyword::SCHEMA) => match self.parse_alter_schema() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterSchema(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::SEQUENCE) => match self.parse_alter_sequence() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterSequence(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::FUNCTION) => match self.parse_alter_function() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterFunction(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::PROCEDURE) => {
                    self.advance();
                    match self.parse_alter_function_skip_keyword() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::AlterProcedure(crate::ast::Spanned::new(crate::ast::AlterProcedureStatement {
                                    name: stmt.name.clone(), 
                                    action: stmt.action,
                                }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::ROLE) => match self.parse_alter_role() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterRole(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                                crate::ast::Statement::AlterUserMapping(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                                crate::ast::Statement::AlterUser(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        crate::ast::Statement::AlterGroup(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::SYSTEM_P) => {
                    let saved_pos = self.pos;
                    self.advance();
                    if self.match_keyword(Keyword::KILL) {
                        self.advance();
                        if self.match_keyword(Keyword::SESSION) {
                            self.advance();
                            match self.parse_alter_system_kill_session() {
                                Ok(stmt) => {
                                    self.try_consume_semicolon();
                                    crate::ast::Statement::AlterSystemKillSession(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                                }
                                Err(e) => {
                                    self.add_error(e);
                                    self.skip_to_semicolon()
                                }
                            }
                        } else {
                            self.pos = saved_pos;
                            match self.parse_alter_global_config() {
                                Ok(stmt) => {
                                    self.try_consume_semicolon();
                                    crate::ast::Statement::AlterGlobalConfig(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                                }
                                Err(e) => {
                                    self.add_error(e);
                                    self.skip_to_semicolon()
                                }
                            }
                        }
                    } else {
                        self.pos = saved_pos;
                        match self.parse_alter_global_config() {
                            Ok(stmt) => {
                                self.try_consume_semicolon();
                                crate::ast::Statement::AlterGlobalConfig(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                            }
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    }
                }
                Some(Keyword::TYPE_P) => match self.parse_alter_type() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterCompositeType(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::VIEW) => match self.parse_alter_view() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterView(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::DOMAIN_P) => {
                    self.advance();
                    match self.parse_alter_domain() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::AlterDomain(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::DIRECTORY) => {
                    self.advance();
                    match self.parse_alter_directory() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::AlterDirectory(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::LANGUAGE) => match self.parse_alter_language() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterLanguage(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::LARGE_P) => {
                    self.advance();
                    match self.parse_alter_large_object() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::AlterLargeObject(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::PACKAGE) => {
                    self.advance();
                    match self.parse_alter_package() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::AlterPackage(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::SESSION) => {
                    self.advance();
                    match self.parse_alter_session() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::AlterSession(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::PROCEDURAL) => match self.parse_alter_language() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterLanguage(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::TRIGGER) => match self.parse_alter_trigger() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterTrigger(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::EXTENSION) => match self.parse_alter_extension() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterExtension(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                                crate::ast::Statement::AlterResourcePool(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                            Ok(stmt) => crate::ast::Statement::AlterForeignTable(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else if self.match_keyword(Keyword::SERVER) {
                        self.advance();
                        match self.parse_alter_foreign_server() {
                            Ok(stmt) => crate::ast::Statement::AlterForeignServer(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                            Ok(stmt) => crate::ast::Statement::AlterFdw(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                        Ok(stmt) => crate::ast::Statement::AlterForeignServer(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::PUBLICATION) => {
                    self.advance();
                    match self.parse_alter_publication() {
                        Ok(stmt) => crate::ast::Statement::AlterPublication(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::SUBSCRIPTION) => {
                    self.advance();
                    match self.parse_alter_subscription() {
                        Ok(stmt) => crate::ast::Statement::AlterSubscription(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                            Ok(stmt) => crate::ast::Statement::AlterNodeGroup(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else {
                        match self.parse_alter_node() {
                            Ok(stmt) => crate::ast::Statement::AlterNode(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                        Ok(stmt) => crate::ast::Statement::AlterWorkloadGroup(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                        Ok(stmt) => crate::ast::Statement::AlterAuditPolicy(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::POLICY) => {
                    self.advance();
                    match self.parse_alter_rls_policy() {
                        Ok(stmt) => crate::ast::Statement::AlterRlsPolicy(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                            Ok(stmt) => crate::ast::Statement::AlterDataSource(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                        Ok(stmt) => crate::ast::Statement::AlterEvent(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
                            Ok(stmt) => crate::ast::Statement::AlterOpFamily(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else if self.match_keyword(Keyword::CLASS) {
                        self.advance();
                        match self.parse_alter_opfamily() {
                            Ok(stmt) => crate::ast::Statement::AlterOpFamily(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else {
                        let op_name = match self.peek().clone() {
                            Token::Op(s) => {
                                self.advance();
                                s
                            }
                            Token::Ident(s) => {
                                self.advance();
                                s
                            }
                            tok @ (Token::OpLe
                            | Token::OpNe
                            | Token::OpGe
                            | Token::OpShiftL
                            | Token::OpShiftR
                            | Token::OpArrow
                            | Token::OpJsonArrow
                            | Token::OpNe2
                            | Token::OpDblBang
                            | Token::OpConcat) => {
                                self.advance();
                                tok.as_op_str().unwrap().to_string()
                            }
                            other => {
                                self.add_error(ParserError::UnexpectedToken {
                                    location: self.current_location(),
                                    expected: "operator name".to_string(),
                                    got: format!("{:?}", other),
                                });
                                return self.skip_to_semicolon();
                            }
                        };
                        let mut left_type = String::new();
                        let mut right_type = None;
                        if self.match_token(&Token::LParen) {
                            self.advance();
                            left_type = self.consume_any_identifier().unwrap_or_default();
                            if self.match_token(&Token::Comma) {
                                self.advance();
                                right_type =
                                    Some(self.consume_any_identifier().unwrap_or_default());
                            }
                            self.expect_token(&Token::RParen).unwrap_or(());
                        }
                        let raw_rest = self.skip_to_semicolon_and_collect();
                        crate::ast::Statement::AlterOperator(crate::ast::Spanned::new(crate::ast::AlterOperatorStatement {
                            name: op_name, 
                            left_type,
                            right_type,
                            raw_rest,
                        }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
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
                        Ok(stmt) => crate::ast::Statement::AlterMaterializedView(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::SYNONYM) => match self.parse_alter_synonym() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::AlterSynonym(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                },
                Some(Keyword::TEXT_P) => {
                    self.advance();
                    if !self.match_keyword(Keyword::SEARCH) {
                        self.add_error(ParserError::UnexpectedToken {
                            location: self.current_location(),
                            expected: "SEARCH after TEXT".to_string(),
                            got: format!("{:?}", self.peek()),
                        });
                        return self.skip_to_semicolon();
                    }
                    self.advance();
                    if self.match_keyword(Keyword::CONFIGURATION) {
                        self.advance();
                        match self.parse_alter_text_search_config() {
                            Ok(stmt) => {
                                self.try_consume_semicolon();
                                crate::ast::Statement::AlterTextSearchConfig(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                            }
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else if self.match_keyword(Keyword::DICTIONARY) {
                        self.advance();
                        match self.parse_alter_text_search_dict() {
                            Ok(stmt) => {
                                self.try_consume_semicolon();
                                crate::ast::Statement::AlterTextSearchDict(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                            }
                            Err(e) => {
                                self.add_error(e);
                                self.skip_to_semicolon()
                            }
                        }
                    } else {
                        self.add_error(ParserError::UnexpectedToken {
                            location: self.current_location(),
                            expected: "CONFIGURATION or DICTIONARY after TEXT SEARCH".to_string(),
                            got: format!("{:?}", self.peek()),
                        });
                        self.skip_to_semicolon()
                    }
                }
                Some(Keyword::COORDINATOR) => {
                    self.advance();
                    match self.parse_alter_coordinator() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::AlterCoordinator(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
                        Err(e) => {
                            self.add_error(e);
                            self.skip_to_semicolon()
                        }
                    }
                }
                Some(Keyword::APP) => {
                    self.advance();
                    if !self.match_keyword(Keyword::WORKLOAD) {
                        self.add_error(ParserError::UnexpectedToken {
                            location: self.current_location(),
                            expected: "WORKLOAD after APP".to_string(),
                            got: format!("{:?}", self.peek()),
                        });
                        return self.skip_to_semicolon();
                    }
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
                    if !self.match_keyword(Keyword::MAPPING) {
                        self.add_error(ParserError::UnexpectedToken {
                            location: self.current_location(),
                            expected: "MAPPING after GROUP".to_string(),
                            got: format!("{:?}", self.peek()),
                        });
                        return self.skip_to_semicolon();
                    }
                    self.advance();
                    match self.parse_alter_app_workload_group_mapping() {
                        Ok(stmt) => {
                            self.try_consume_semicolon();
                            crate::ast::Statement::AlterAppWorkloadGroupMapping(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })))
                        }
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
                                Ok(stmt) => crate::ast::Statement::AlterRlsPolicy(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
        let start = self.current_location();
        if self.match_keyword(Keyword::USER) {
            let saved_pos = self.pos;
            self.advance();
            if self.match_keyword(Keyword::MAPPING) {
                self.pos = saved_pos;
                match self.parse_drop_user_mapping() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        return crate::ast::Statement::DropUserMapping(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() })));
                    }
                    Err(e) => {
                        self.add_error(e);
                        return self.skip_to_semicolon();
                    }
                }
            }
            self.pos = saved_pos;
        }
        if self.match_keyword(Keyword::GLOBAL) {
            self.advance();
            self.try_consume_keyword(Keyword::CONFIGURATION);
            let name = match self.parse_identifier() {
                Ok(n) => n,
                Err(e) => {
                    self.add_error(e);
                    return self.skip_to_semicolon();
                }
            };
            self.try_consume_semicolon();
            return crate::ast::Statement::Drop(crate::ast::Spanned::new(crate::ast::DropStatement {
                object_type: crate::ast::ObjectType::Global, 
                if_exists: false,
                names: vec![vec![name]],
                cascade: false,
                purge: false,
            }, Some(crate::ast::SourceSpan { start, end: self.prev_location() })));
        }
        match self.parse_drop() {
            Ok(stmt) => crate::ast::Statement::Drop(crate::ast::Spanned::new(stmt, Some(crate::ast::SourceSpan { start, end: self.prev_location() }))),
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
#[cfg(test)]
mod tests_plsql_fixes;
