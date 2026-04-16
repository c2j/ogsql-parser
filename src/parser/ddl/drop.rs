use crate::ast::*;
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;

impl Parser {
    pub(crate) fn parse_drop(&mut self) -> Result<DropStatement, ParserError> {
        let obj_type = match self.peek_keyword() {
            Some(Keyword::TABLE) => {
                self.advance();
                ObjectType::Table
            }
            Some(Keyword::INDEX) => {
                self.advance();
                ObjectType::Index
            }
            Some(Keyword::SEQUENCE) => {
                self.advance();
                ObjectType::Sequence
            }
            Some(Keyword::VIEW) => {
                self.advance();
                ObjectType::View
            }
            Some(Keyword::SCHEMA) => {
                self.advance();
                ObjectType::Schema
            }
            Some(Keyword::DATABASE) => {
                self.advance();
                ObjectType::Database
            }
            Some(Keyword::TABLESPACE) => {
                self.advance();
                ObjectType::Tablespace
            }
            Some(Keyword::MATERIALIZED) => {
                self.advance();
                self.expect_keyword(Keyword::VIEW)?;
                ObjectType::MaterializedView
            }
            Some(Keyword::FUNCTION) => {
                self.advance();
                ObjectType::Function
            }
            Some(Keyword::PROCEDURE) => {
                self.advance();
                ObjectType::Procedure
            }
            Some(Keyword::TRIGGER) => {
                self.advance();
                ObjectType::Trigger
            }
            Some(Keyword::EXTENSION) => {
                self.advance();
                ObjectType::Extension
            }
            Some(Keyword::FOREIGN) => {
                self.advance();
                if self.match_keyword(Keyword::TABLE) {
                    self.advance();
                    ObjectType::ForeignTable
                } else if self.match_keyword(Keyword::DATA_P) {
                    self.advance();
                    self.expect_keyword(Keyword::WRAPPER)?;
                    ObjectType::Fdw
                } else {
                    self.expect_keyword(Keyword::SERVER)?;
                    ObjectType::ForeignServer
                }
            }
            Some(Keyword::USER) => {
                self.advance();
                ObjectType::User
            }
            Some(Keyword::ROLE) => {
                self.advance();
                ObjectType::Role
            }
            Some(Keyword::GROUP_P) => {
                self.advance();
                ObjectType::Group
            }
            Some(Keyword::RESOURCE) => {
                self.advance();
                if self.match_keyword(Keyword::POOL) {
                    self.advance();
                    ObjectType::ResourcePool
                } else {
                    self.expect_keyword(Keyword::LABEL)?;
                    ObjectType::ResourceLabel
                }
            }
            Some(Keyword::WORKLOAD) => {
                self.advance();
                self.expect_keyword(Keyword::GROUP_P)?;
                ObjectType::WorkloadGroup
            }
            Some(Keyword::AUDIT) => {
                self.advance();
                self.expect_keyword(Keyword::POLICY)?;
                ObjectType::AuditPolicy
            }
            Some(Keyword::MASKING) => {
                self.advance();
                self.expect_keyword(Keyword::POLICY)?;
                ObjectType::MaskingPolicy
            }
            Some(Keyword::ROW) => {
                self.advance();
                self.expect_keyword(Keyword::LEVEL)?;
                self.expect_keyword(Keyword::SECURITY)?;
                self.expect_keyword(Keyword::POLICY)?;
                ObjectType::RlsPolicy
            }
            Some(Keyword::DATA_P) => {
                self.advance();
                self.expect_keyword(Keyword::SOURCE_P)?;
                ObjectType::DataSource
            }
            Some(Keyword::DIRECTORY) => {
                self.advance();
                ObjectType::Directory
            }
            Some(Keyword::EVENT) => {
                self.advance();
                ObjectType::Event
            }
            Some(Keyword::PUBLICATION) => {
                self.advance();
                ObjectType::Publication
            }
            Some(Keyword::SUBSCRIPTION) => {
                self.advance();
                ObjectType::Subscription
            }
            Some(Keyword::SYNONYM) => {
                self.advance();
                ObjectType::Synonym
            }
            Some(Keyword::MODEL) => {
                self.advance();
                ObjectType::Model
            }
            Some(Keyword::SECURITY) => {
                self.advance();
                self.expect_keyword(Keyword::LABEL)?;
                ObjectType::SecurityLabel
            }
            Some(Keyword::NODE) => {
                self.advance();
                if self.match_keyword(Keyword::GROUP_P) {
                    self.advance();
                    ObjectType::NodeGroup
                } else {
                    ObjectType::Node
                }
            }
            Some(Keyword::TEXT_P) => {
                self.advance();
                self.expect_keyword(Keyword::SEARCH)?;
                if self.match_keyword(Keyword::CONFIGURATION) {
                    self.advance();
                    ObjectType::TextSearchConfig
                } else {
                    self.expect_keyword(Keyword::DICTIONARY)?;
                    ObjectType::TextSearchDict
                }
            }
            Some(Keyword::OPERATOR) => {
                self.advance();
                if self.match_keyword(Keyword::CLASS) {
                    self.advance();
                    ObjectType::OpClass
                } else if self.match_keyword(Keyword::FAMILY) {
                    self.advance();
                    ObjectType::OpFamily
                } else {
                    ObjectType::Operator
                }
            }
            Some(Keyword::AGGREGATE) => {
                self.advance();
                ObjectType::Aggregate
            }
            Some(Keyword::CAST) => {
                self.advance();
                ObjectType::Cast
            }
            Some(Keyword::CONVERSION_P) => {
                self.advance();
                ObjectType::Conversion
            }
            Some(Keyword::DOMAIN_P) => {
                self.advance();
                ObjectType::Domain
            }
            Some(Keyword::RULE) => {
                self.advance();
                ObjectType::Rule
            }
            Some(Keyword::LANGUAGE) => {
                self.advance();
                ObjectType::Language
            }
            Some(Keyword::POLICY) => {
                self.advance();
                ObjectType::Policy
            }
            Some(Keyword::OWNED) => {
                self.advance();
                self.expect_keyword(Keyword::BY)?;
                let _name = self.parse_object_name()?;
                return Ok(DropStatement {
                    object_type: ObjectType::User,
                    if_exists: false,
                    names: vec![_name],
                    cascade: false,
                    purge: false,
                });
            }
            Some(Keyword::WEAK) => {
                self.advance();
                self.expect_keyword(Keyword::PASSWORD)?;
                self.expect_keyword(Keyword::DICTIONARY)?;
                ObjectType::WeakPasswordDictionary
            }
            Some(Keyword::SERVER) => {
                self.advance();
                ObjectType::ForeignServer
            }
            Some(Keyword::TYPE_P) => {
                self.advance();
                ObjectType::Type
            }
            _ if self.match_ident_str("client") => {
                self.advance();
                if !self.try_consume_ident_str("master") {
                    return Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "MASTER after CLIENT".to_string(),
                        got: format!("{:?}", self.peek()),
                    });
                }
                if !self.try_consume_ident_str("key") {
                    return Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "KEY after CLIENT MASTER".to_string(),
                        got: format!("{:?}", self.peek()),
                    });
                }
                ObjectType::User
            }
            _ if self.match_ident_str("column") => {
                self.advance();
                if !self.try_consume_ident_str("encryption") {
                    return Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "ENCRYPTION after COLUMN".to_string(),
                        got: format!("{:?}", self.peek()),
                    });
                }
                if !self.try_consume_ident_str("key") {
                    return Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "KEY after COLUMN ENCRYPTION".to_string(),
                        got: format!("{:?}", self.peek()),
                    });
                }
                ObjectType::User
            }
            _ if self.match_ident_str("app") => {
                self.advance();
                ObjectType::App
            }
            _ if self.match_ident_str("global") => {
                self.advance();
                ObjectType::Global
            }
            _ if self.match_ident_str("procedure") => {
                self.advance();
                ObjectType::Procedure
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "DROP object type".to_string(),
                    got: format!("{:?}", self.peek()),
                });
            }
        };
        let if_exists = self.parse_if_exists();
        self.parse_drop_statement_with_type(obj_type, if_exists)
    }

    fn parse_drop_statement_with_type(
        &mut self,
        object_type: ObjectType,
        if_exists: bool,
    ) -> Result<DropStatement, ParserError> {
        let mut names = vec![self.parse_object_name()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            names.push(self.parse_object_name()?);
        }
        match object_type {
            ObjectType::Aggregate | ObjectType::Operator | ObjectType::Cast => {
                if self.match_token(&Token::LParen) {
                    self.advance();
                    let mut depth = 1;
                    while depth > 0 && self.pos < self.tokens.len() {
                        match self.peek() {
                            Token::LParen => depth += 1,
                            Token::RParen => depth -= 1,
                            _ => {}
                        }
                        self.advance();
                    }
                }
            }
            ObjectType::OperatorClass | ObjectType::OperatorFamily => {
                if self.match_keyword(Keyword::USING) {
                    self.advance();
                    let _ = self.parse_identifier();
                }
            }
            ObjectType::Rule => {
                if self.match_keyword(Keyword::ON) {
                    self.advance();
                    let _ = self.parse_object_name();
                }
            }
            _ => {}
        }
        let cascade = self.try_consume_keyword(Keyword::CASCADE);
        let purge = self.try_consume_keyword(Keyword::PURGE);
        Ok(DropStatement {
            object_type,
            if_exists,
            names,
            cascade,
            purge,
        })
    }

    // ========== CREATE INDEX ==========
}
