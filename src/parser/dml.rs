use crate::ast::{
    DeleteStatement, InsertSource, InsertStatement, MergeAction, MergeStatement, MergeWhenClause,
    SelectTarget, TableRef, UpdateAssignment, UpdateStatement,
};
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;

impl Parser {
    pub(crate) fn parse_insert(&mut self) -> Result<InsertStatement, ParserError> {
        self.try_consume_keyword(Keyword::INTO);
        let table = self.parse_object_name()?;
        let columns = if self.match_token(&Token::LParen) {
            self.advance();
            let mut cols = vec![self.parse_identifier()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                cols.push(self.parse_identifier()?);
            }
            self.expect_token(&Token::RParen)?;
            cols
        } else {
            vec![]
        };
        let source = if self.match_keyword(Keyword::DEFAULT) {
            self.advance();
            self.expect_keyword(Keyword::VALUES)?;
            InsertSource::DefaultValues
        } else if self.match_keyword(Keyword::VALUES) {
            self.advance();
            let mut rows = Vec::new();
            loop {
                self.expect_token(&Token::LParen)?;
                let mut row = Vec::new();
                if !self.match_token(&Token::RParen) {
                    row.push(self.parse_expr()?);
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        row.push(self.parse_expr()?);
                    }
                }
                self.expect_token(&Token::RParen)?;
                rows.push(row);
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            InsertSource::Values(rows)
        } else if self.match_keyword(Keyword::SELECT) || self.match_keyword(Keyword::WITH) {
            InsertSource::Select(Box::new(self.parse_select_statement()?))
        } else {
            return Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "VALUES, SELECT, DEFAULT VALUES".to_string(),
                got: format!("{:?}", self.peek()),
            });
        };
        let returning = if self.match_keyword(Keyword::RETURNING) {
            self.advance();
            self.parse_target_list()?
        } else {
            vec![]
        };
        Ok(InsertStatement {
            table,
            columns,
            source,
            returning,
        })
    }

    pub(crate) fn parse_update(&mut self) -> Result<UpdateStatement, ParserError> {
        let mut tables = vec![self.parse_table_ref()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            tables.push(self.parse_table_ref()?);
        }
        self.expect_keyword(Keyword::SET)?;
        let mut assignments = Vec::new();
        loop {
            let column = self.parse_object_name()?;
            self.expect_token(&Token::Eq)?;
            let value = self.parse_expr()?;
            assignments.push(UpdateAssignment { column, value });
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }
        let from = if self.match_keyword(Keyword::FROM) {
            self.advance();
            let mut froms = vec![self.parse_table_ref()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                froms.push(self.parse_table_ref()?);
            }
            froms
        } else {
            vec![]
        };
        let where_clause = if self.match_keyword(Keyword::WHERE) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        let returning = if self.match_keyword(Keyword::RETURNING) {
            self.advance();
            self.parse_target_list()?
        } else {
            vec![]
        };
        Ok(UpdateStatement {
            tables,
            assignments,
            from,
            where_clause,
            returning,
        })
    }

    pub(crate) fn parse_delete(&mut self) -> Result<DeleteStatement, ParserError> {
        let has_from = self.try_consume_keyword(Keyword::FROM);
        let mut tables = vec![self.parse_table_ref()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            tables.push(self.parse_table_ref()?);
        }
        let using = if !has_from && self.match_keyword(Keyword::FROM) {
            self.advance();
            self.parse_from_clause()?
        } else if self.match_keyword(Keyword::USING) {
            self.advance();
            self.parse_from_clause()?
        } else {
            vec![]
        };
        let where_clause = if self.match_keyword(Keyword::WHERE) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        let returning = if self.match_keyword(Keyword::RETURNING) {
            self.advance();
            self.parse_target_list()?
        } else {
            vec![]
        };
        Ok(DeleteStatement {
            tables,
            using,
            where_clause,
            returning,
        })
    }

    pub(crate) fn parse_merge(&mut self) -> Result<MergeStatement, ParserError> {
        self.try_consume_keyword(Keyword::INTO);
        let target = self.parse_table_ref()?;
        self.expect_keyword(Keyword::USING)?;
        let source = self.parse_table_ref()?;
        self.expect_keyword(Keyword::ON)?;
        let on_condition = self.parse_expr()?;
        let mut when_clauses = Vec::new();
        while self.match_keyword(Keyword::WHEN) {
            self.advance();
            let matched = if self.match_keyword(Keyword::NOT) {
                self.advance();
                self.expect_keyword(Keyword::MATCHED)?;
                false
            } else {
                self.expect_keyword(Keyword::MATCHED)?;
                true
            };
            self.expect_keyword(Keyword::THEN)?;
            let action = if self.match_keyword(Keyword::UPDATE) {
                self.advance();
                self.expect_keyword(Keyword::SET)?;
                let mut assignments = Vec::new();
                loop {
                    let column = self.parse_object_name()?;
                    self.expect_token(&Token::Eq)?;
                    let value = self.parse_expr()?;
                    assignments.push(UpdateAssignment { column, value });
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                    self.advance();
                }
                MergeAction::Update(assignments)
            } else if self.match_keyword(Keyword::DELETE_P) {
                self.advance();
                MergeAction::Delete
            } else if self.match_keyword(Keyword::INSERT) {
                self.advance();
                let columns = if self.match_token(&Token::LParen) {
                    self.advance();
                    let mut cols = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        cols.push(self.parse_identifier()?);
                    }
                    self.expect_token(&Token::RParen)?;
                    cols
                } else {
                    vec![]
                };
                self.expect_keyword(Keyword::VALUES)?;
                self.expect_token(&Token::LParen)?;
                let mut values = vec![self.parse_expr()?];
                while self.match_token(&Token::Comma) {
                    self.advance();
                    values.push(self.parse_expr()?);
                }
                self.expect_token(&Token::RParen)?;
                MergeAction::Insert { columns, values }
            } else {
                return Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "UPDATE, DELETE, or INSERT".to_string(),
                    got: format!("{:?}", self.peek()),
                });
            };
            when_clauses.push(MergeWhenClause { matched, action });
        }
        Ok(MergeStatement {
            target,
            source,
            on_condition,
            when_clauses,
        })
    }
}
