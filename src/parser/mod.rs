pub mod ddl;
pub mod dml;
pub mod expr;
pub mod select;

use crate::token::keyword::Keyword;
use crate::token::{Token, TokenWithSpan};

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("unexpected token at position {position}: expected {expected}, got {got}")]
    UnexpectedToken {
        position: usize,
        expected: String,
        got: String,
    },
    #[error("unexpected end of input: expected {0}")]
    UnexpectedEof(String),
    #[error("{0}")]
    TokenizerError(#[from] crate::token::tokenizer::TokenizerError),
}

pub struct Parser {
    tokens: Vec<TokenWithSpan>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithSpan>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse_sql(input: &str) -> Result<Vec<crate::ast::Statement>, ParserError> {
        let tokens = crate::token::tokenizer::Tokenizer::new(input).tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse()
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
                position: self.pos,
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
                position: self.pos,
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
                position: self.pos,
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
                    Err(_) => self.skip_to_semicolon(),
                }
            }
            Token::Keyword(Keyword::INSERT) => {
                self.advance();
                match self.parse_insert() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Insert(stmt)
                    }
                    Err(_) => self.skip_to_semicolon(),
                }
            }
            Token::Keyword(Keyword::UPDATE) => {
                self.advance();
                match self.parse_update() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Update(stmt)
                    }
                    Err(_) => self.skip_to_semicolon(),
                }
            }
            Token::Keyword(Keyword::DELETE_P) => {
                self.advance();
                match self.parse_delete() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Delete(stmt)
                    }
                    Err(_) => self.skip_to_semicolon(),
                }
            }
            Token::Keyword(Keyword::MERGE) => {
                self.advance();
                match self.parse_merge() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Merge(stmt)
                    }
                    Err(_) => self.skip_to_semicolon(),
                }
            }
            Token::Keyword(Keyword::TRUNCATE) => {
                self.advance();
                match self.parse_truncate() {
                    Ok(stmt) => {
                        self.try_consume_semicolon();
                        crate::ast::Statement::Truncate(stmt)
                    }
                    Err(_) => self.skip_to_semicolon(),
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
                Err(_) => self.skip_to_semicolon(),
            },
            Some(Keyword::INDEX) => match self.parse_create_index() {
                Ok(stmt) => crate::ast::Statement::CreateIndex(stmt),
                Err(_) => self.skip_to_semicolon(),
            },
            Some(Keyword::SEQUENCE) => match self.parse_create_sequence() {
                Ok(stmt) => crate::ast::Statement::CreateSequence(stmt),
                Err(_) => self.skip_to_semicolon(),
            },
            Some(Keyword::VIEW) => match self.parse_create_view() {
                Ok(mut stmt) => {
                    stmt.replace = replace;
                    stmt.temporary = temporary;
                    stmt.recursive = recursive;
                    crate::ast::Statement::CreateView(stmt)
                }
                Err(_) => self.skip_to_semicolon(),
            },
            Some(Keyword::SCHEMA) => match self.parse_create_schema() {
                Ok(stmt) => crate::ast::Statement::CreateSchema(stmt),
                Err(_) => self.skip_to_semicolon(),
            },
            Some(Keyword::DATABASE) => match self.parse_create_database() {
                Ok(stmt) => crate::ast::Statement::CreateDatabase(stmt),
                Err(_) => self.skip_to_semicolon(),
            },
            Some(Keyword::TABLESPACE) => match self.parse_create_tablespace() {
                Ok(stmt) => crate::ast::Statement::CreateTablespace(stmt),
                Err(_) => self.skip_to_semicolon(),
            },
            _ => self.skip_to_semicolon(),
        }
    }

    fn dispatch_alter(&mut self) -> crate::ast::Statement {
        match self.peek_keyword() {
            Some(Keyword::TABLE) => match self.parse_alter_table() {
                Ok(stmt) => crate::ast::Statement::AlterTable(stmt),
                Err(_) => self.skip_to_semicolon(),
            },
            _ => self.skip_to_semicolon(),
        }
    }

    fn dispatch_drop(&mut self) -> crate::ast::Statement {
        match self.parse_drop() {
            Ok(stmt) => crate::ast::Statement::Drop(stmt),
            Err(_) => self.skip_to_semicolon(),
        }
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

#[cfg(test)]
mod tests;
