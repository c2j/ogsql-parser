pub(crate) mod ddl;
pub(crate) mod dml;
pub(crate) mod expr;
pub(crate) mod select;
pub(crate) mod utility;

use crate::token::keyword::Keyword;
use crate::token::{SourceLocation, Token, TokenWithSpan};

#[derive(Debug, thiserror::Error)]
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
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithSpan>) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse_sql(input: &str) -> Result<Vec<crate::ast::Statement>, ParserError> {
        let tokens = crate::token::tokenizer::Tokenizer::new(input).tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    pub fn parse_one(input: &str) -> Result<crate::ast::Statement, ParserError> {
        let tokens = crate::token::tokenizer::Tokenizer::new(input).tokenize()?;
        let mut parser = Parser::new(tokens);
        let mut stmts = parser.parse()?;
        match stmts.len() {
            0 => Err(ParserError::UnexpectedEof {
                expected: "statement".to_string(),
                location: parser.current_location(),
            }),
            1 => Ok(stmts.remove(0)),
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

    pub fn parse(&mut self) -> Result<Vec<crate::ast::Statement>, ParserError> {
        let mut stmts = Vec::new();
        loop {
            match self.peek() {
                Token::Eof => break,
                Token::Semicolon => {
                    self.advance();
                    continue;
                }
                _ => {
                    let stmt = self.parse_statement()?;
                    stmts.push(stmt);
                }
            }
        }
        Ok(stmts)
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
                | Token::DollarString(_)
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

    fn parse_statement(&mut self) -> Result<crate::ast::Statement, ParserError> {
        let stmt = match self.peek().clone() {
            Token::Keyword(Keyword::SELECT) | Token::Keyword(Keyword::WITH) => {
                match self.parse_select_statement() {
                    Ok(stmt) => {
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
                self.advance();
                match self.parse_insert() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Insert(stmt)
                    }
                    Err(e) => {
                        self.add_error(e);
                        self.skip_to_semicolon()
                    }
                }
            }
            Token::Keyword(Keyword::UPDATE) => {
                self.advance();
                match self.parse_update() {
                    Ok(stmt) => {
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
                self.advance();
                match self.parse_delete() {
                    Ok(stmt) => {
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
                self.advance();
                match self.parse_merge() {
                    Ok(stmt) => {
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
            Token::Keyword(Keyword::BEGIN_P) | Token::Keyword(Keyword::START) => {
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
                self.skip_to_semicolon_as(crate::ast::Statement::Grant(
                    crate::ast::GrantStatement { _stub: () },
                ))
            }
            Token::Keyword(Keyword::REVOKE) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Revoke(
                    crate::ast::RevokeStatement { _stub: () },
                ))
            }
            Token::Keyword(Keyword::VACUUM) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Vacuum(
                    crate::ast::VacuumStatement { _stub: () },
                ))
            }
            Token::Keyword(Keyword::DO) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Do(crate::ast::DoStatement {
                    _stub: (),
                }))
            }
            Token::Keyword(Keyword::PREPARE) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Prepare(
                    crate::ast::PrepareStatement { _stub: () },
                ))
            }
            Token::Keyword(Keyword::EXECUTE) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Execute(
                    crate::ast::ExecuteStatement { _stub: () },
                ))
            }
            Token::Keyword(Keyword::DEALLOCATE) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Deallocate(
                    crate::ast::DeallocateStatement { _stub: () },
                ))
            }
            Token::Keyword(Keyword::COMMENT) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Comment(
                    crate::ast::CommentStatement { _stub: () },
                ))
            }
            Token::Keyword(Keyword::LOCK_P) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Lock(crate::ast::LockStatement {
                    _stub: (),
                }))
            }
            Token::Keyword(Keyword::DECLARE) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::DeclareCursor(
                    crate::ast::DeclareCursorStatement { _stub: () },
                ))
            }
            Token::Keyword(Keyword::CLOSE) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::ClosePortal(
                    crate::ast::ClosePortalStatement { _stub: () },
                ))
            }
            Token::Keyword(Keyword::FETCH) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Fetch(
                    crate::ast::FetchStatement { _stub: () },
                ))
            }
            Token::Keyword(Keyword::CLUSTER) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Cluster(
                    crate::ast::ClusterStatement { _stub: () },
                ))
            }
            Token::Keyword(Keyword::REINDEX) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Reindex(
                    crate::ast::ReindexStatement { _stub: () },
                ))
            }
            Token::Keyword(Keyword::LISTEN) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Listen(
                    crate::ast::ListenStatement { _stub: () },
                ))
            }
            Token::Keyword(Keyword::NOTIFY) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Notify(
                    crate::ast::NotifyStatement { _stub: () },
                ))
            }
            Token::Keyword(Keyword::UNLISTEN) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Unlisten(
                    crate::ast::UnlistenStatement { _stub: () },
                ))
            }
            Token::Keyword(Keyword::RULE) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Rule(crate::ast::RuleStatement {
                    _stub: (),
                }))
            }
            Token::Keyword(Keyword::ANALYZE) => {
                self.advance();
                self.skip_to_semicolon_as(crate::ast::Statement::Analyze(
                    crate::ast::AnalyzeStatement { _stub: () },
                ))
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

        let temporary =
            if self.match_keyword(Keyword::TEMPORARY) || self.match_keyword(Keyword::TEMP) {
                self.advance();
                true
            } else {
                false
            };

        let recursive = self.try_consume_keyword(Keyword::RECURSIVE);

        match self.peek_keyword() {
            Some(Keyword::TABLE) => match self.parse_create_table() {
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
                    stmt.temporary = temporary;
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
                self.skip_to_semicolon_as(crate::ast::Statement::CreateFunction(
                    crate::ast::CreateFunctionStatement { _stub: () },
                ))
            }
            Some(Keyword::PROCEDURE) => {
                self.skip_to_semicolon_as(crate::ast::Statement::CreateProcedure(
                    crate::ast::CreateProcedureStatement { _stub: () },
                ))
            }
            Some(Keyword::TRIGGER) => {
                self.skip_to_semicolon_as(crate::ast::Statement::CreateTrigger(
                    crate::ast::CreateTriggerStatement { _stub: () },
                ))
            }
            Some(Keyword::MATERIALIZED) => {
                self.skip_to_semicolon_as(crate::ast::Statement::CreateMaterializedView(
                    crate::ast::CreateMaterializedViewStatement { _stub: () },
                ))
            }
            Some(Keyword::EXTENSION) => {
                self.skip_to_semicolon_as(crate::ast::Statement::CreateExtension(
                    crate::ast::CreateExtensionStatement { _stub: () },
                ))
            }
            Some(Keyword::ROLE) => self.skip_to_semicolon_as(crate::ast::Statement::CreateRole(
                crate::ast::CreateRoleStatement { _stub: () },
            )),
            Some(Keyword::USER) => self.skip_to_semicolon_as(crate::ast::Statement::CreateUser(
                crate::ast::CreateUserStatement { _stub: () },
            )),
            Some(Keyword::GROUP_P) => self.skip_to_semicolon_as(
                crate::ast::Statement::CreateGroup(crate::ast::CreateGroupStatement { _stub: () }),
            ),
            Some(Keyword::PACKAGE) => {
                self.skip_to_semicolon_as(crate::ast::Statement::CreatePackage(
                    crate::ast::CreatePackageStatement { _stub: () },
                ))
            }
            Some(Keyword::TYPE_P) => self.skip_to_semicolon_as(crate::ast::Statement::Empty),
            Some(Keyword::CAST) => self.skip_to_semicolon_as(crate::ast::Statement::Empty),
            Some(Keyword::DOMAIN_P) => self.skip_to_semicolon_as(crate::ast::Statement::Empty),
            _ => self.skip_to_semicolon(),
        }
    }

    fn dispatch_alter(&mut self) -> crate::ast::Statement {
        match self.peek_keyword() {
            Some(Keyword::TABLE) => match self.parse_alter_table() {
                Ok(stmt) => crate::ast::Statement::AlterTable(stmt),
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
        loop {
            match self.peek() {
                Token::Eof => break,
                Token::Semicolon if depth == 0 => {
                    self.advance();
                    break;
                }
                Token::LParen => {
                    depth += 1;
                    self.advance();
                }
                Token::RParen => {
                    depth -= 1;
                    self.advance();
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
        loop {
            match self.peek() {
                Token::Eof => break,
                Token::Semicolon if depth == 0 => {
                    self.advance();
                    break;
                }
                Token::LParen => {
                    depth += 1;
                    self.advance();
                }
                Token::RParen => {
                    depth -= 1;
                    self.advance();
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
