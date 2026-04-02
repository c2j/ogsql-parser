use crate::ast::*;
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;

impl Parser {
    pub(crate) fn parse_set(&mut self) -> Result<VariableSetStatement, ParserError> {
        let local = self.try_consume_keyword(Keyword::LOCAL);
        let session = if self.match_keyword(Keyword::SESSION) {
            self.advance();
            true
        } else {
            false
        };

        let name = self.parse_set_variable_name()?;

        if self.match_token(&Token::Eq) || self.match_keyword(Keyword::TO) {
            self.advance();
        }

        let value = if self.match_keyword(Keyword::DEFAULT) {
            self.advance();
            vec![]
        } else {
            let mut values = vec![self.parse_expr()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                values.push(self.parse_expr()?);
            }
            values
        };

        Ok(VariableSetStatement {
            local,
            session,
            name,
            value,
        })
    }

    pub(crate) fn parse_show(&mut self) -> Result<VariableShowStatement, ParserError> {
        let name = if self.match_keyword(Keyword::ALL) {
            self.advance();
            "ALL".to_string()
        } else {
            self.parse_set_variable_name()?
        };
        Ok(VariableShowStatement { name })
    }

    pub(crate) fn parse_reset(&mut self) -> Result<VariableResetStatement, ParserError> {
        if self.match_keyword(Keyword::ALL) {
            self.advance();
            Ok(VariableResetStatement {
                name: "ALL".to_string(),
            })
        } else {
            let name = self.parse_set_variable_name()?;
            Ok(VariableResetStatement { name })
        }
    }

    pub(crate) fn parse_discard(&mut self) -> Result<DiscardStatement, ParserError> {
        let target = match self.peek_keyword() {
            Some(Keyword::ALL) => {
                self.advance();
                DiscardTarget::All
            }
            Some(Keyword::PLANS) => {
                self.advance();
                DiscardTarget::Plans
            }
            Some(Keyword::SEQUENCE) | Some(Keyword::SEQUENCES) => {
                self.advance();
                DiscardTarget::Sequences
            }
            Some(Keyword::TEMP) | Some(Keyword::TEMPORARY) => {
                self.advance();
                DiscardTarget::Temp
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    position: self.pos,
                    expected: "ALL, PLANS, SEQUENCES, or TEMP".to_string(),
                    got: format!("{:?}", self.peek()),
                });
            }
        };
        Ok(DiscardStatement { target })
    }

    pub(crate) fn parse_transaction_begin(&mut self) -> Result<TransactionStatement, ParserError> {
        if self.match_keyword(Keyword::TRANSACTION) || self.match_keyword(Keyword::TRANSACTION) {
            self.advance();
        }
        let mut modes = Vec::new();
        loop {
            if let Some(mode) = self.try_parse_transaction_mode()? {
                modes.push(mode);
            } else {
                break;
            }
        }
        Ok(TransactionStatement {
            kind: TransactionKind::Begin,
            modes,
            savepoint_name: None,
        })
    }

    pub(crate) fn parse_transaction_commit(&mut self) -> Result<TransactionStatement, ParserError> {
        if self.match_keyword(Keyword::TRANSACTION) || self.match_keyword(Keyword::TRANSACTION) {
            self.advance();
        }
        if self.match_keyword(Keyword::CHAIN) {
            self.advance();
        }
        Ok(TransactionStatement {
            kind: TransactionKind::Commit,
            modes: vec![],
            savepoint_name: None,
        })
    }

    pub(crate) fn parse_transaction_rollback(
        &mut self,
    ) -> Result<TransactionStatement, ParserError> {
        if self.match_keyword(Keyword::TRANSACTION) || self.match_keyword(Keyword::TRANSACTION) {
            self.advance();
        }
        if self.match_keyword(Keyword::AND) {
            self.advance();
            if self.match_keyword(Keyword::CHAIN) {
                self.advance();
            }
        }
        let savepoint_name = if self.match_keyword(Keyword::TO) {
            self.advance();
            self.try_consume_keyword(Keyword::SAVEPOINT);
            Some(self.parse_identifier()?)
        } else {
            None
        };
        Ok(TransactionStatement {
            kind: TransactionKind::Rollback,
            modes: vec![],
            savepoint_name,
        })
    }

    pub(crate) fn parse_savepoint(&mut self) -> Result<TransactionStatement, ParserError> {
        let name = self.parse_identifier()?;
        Ok(TransactionStatement {
            kind: TransactionKind::Savepoint,
            modes: vec![],
            savepoint_name: Some(name),
        })
    }

    pub(crate) fn parse_release_savepoint(&mut self) -> Result<TransactionStatement, ParserError> {
        self.try_consume_keyword(Keyword::SAVEPOINT);
        let name = self.parse_identifier()?;
        Ok(TransactionStatement {
            kind: TransactionKind::ReleaseSavepoint,
            modes: vec![],
            savepoint_name: Some(name),
        })
    }

    fn try_parse_transaction_mode(&mut self) -> Result<Option<TransactionMode>, ParserError> {
        if self.match_keyword(Keyword::ISOLATION) {
            self.advance();
            self.expect_keyword(Keyword::LEVEL)?;
            let level = match self.peek_keyword() {
                Some(Keyword::SERIALIZABLE) => {
                    self.advance();
                    IsolationLevel::Serializable
                }
                Some(Keyword::REPEATABLE) => {
                    self.advance();
                    self.expect_keyword(Keyword::READ)?;
                    IsolationLevel::RepeatableRead
                }
                Some(Keyword::READ) => {
                    self.advance();
                    if self.match_keyword(Keyword::COMMITTED) {
                        self.advance();
                        IsolationLevel::ReadCommitted
                    } else if self.match_keyword(Keyword::UNCOMMITTED) {
                        self.advance();
                        IsolationLevel::ReadUncommitted
                    } else {
                        return Err(ParserError::UnexpectedToken {
                            position: self.pos,
                            expected: "COMMITTED or UNCOMMITTED".to_string(),
                            got: format!("{:?}", self.peek()),
                        });
                    }
                }
                _ => {
                    return Err(ParserError::UnexpectedToken {
                        position: self.pos,
                        expected: "isolation level".to_string(),
                        got: format!("{:?}", self.peek()),
                    });
                }
            };
            Ok(Some(TransactionMode::IsolationLevel(level)))
        } else if self.match_keyword(Keyword::READ) {
            self.advance();
            if self.match_keyword(Keyword::ONLY) {
                self.advance();
                Ok(Some(TransactionMode::ReadOnly))
            } else if self.match_keyword(Keyword::WRITE) {
                self.advance();
                Ok(Some(TransactionMode::ReadWrite))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn parse_set_variable_name(&mut self) -> Result<String, ParserError> {
        let mut parts = vec![self.parse_identifier()?];
        while self.match_token(&Token::Dot) {
            self.advance();
            parts.push(self.parse_identifier()?);
        }
        Ok(parts.join("."))
    }
}
