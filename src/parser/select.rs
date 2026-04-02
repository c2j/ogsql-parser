use crate::ast::{
    Cte, JoinType, ObjectName, OrderByItem, SelectStatement, SelectTarget, SetOperation, TableRef,
    WithClause,
};
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;

impl Parser {
    pub(crate) fn parse_select_statement(&mut self) -> Result<SelectStatement, ParserError> {
        let with = self.parse_with_clause()?;
        let mut stmt = self.parse_simple_select()?;
        stmt.with = with;

        loop {
            let (op, all) = match self.peek_keyword() {
                Some(Keyword::UNION) => {
                    self.advance();
                    let all = self.try_consume_keyword(Keyword::ALL);
                    ("union", all)
                }
                Some(Keyword::INTERSECT) => {
                    self.advance();
                    let all = self.try_consume_keyword(Keyword::ALL);
                    ("intersect", all)
                }
                Some(Keyword::EXCEPT) => {
                    self.advance();
                    let all = self.try_consume_keyword(Keyword::ALL);
                    ("except", all)
                }
                _ => break,
            };
            let right = self.parse_simple_select()?;
            let set_op = match op {
                "union" => SetOperation::Union {
                    all,
                    right: Box::new(right),
                },
                "intersect" => SetOperation::Intersect {
                    all,
                    right: Box::new(right),
                },
                _ => SetOperation::Except {
                    all,
                    right: Box::new(right),
                },
            };
            stmt.set_operation = Some(set_op);
        }

        self.parse_order_limit_offset(&mut stmt)?;
        Ok(stmt)
    }

    fn parse_with_clause(&mut self) -> Result<Option<WithClause>, ParserError> {
        if !self.match_keyword(Keyword::WITH) {
            return Ok(None);
        }
        self.advance();
        let recursive = self.try_consume_keyword(Keyword::RECURSIVE);
        let mut ctes = Vec::new();
        loop {
            let name = self.parse_identifier()?;
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
            self.expect_keyword(Keyword::AS)?;
            self.expect_token(&Token::LParen)?;
            let query = self.parse_select_statement()?;
            self.expect_token(&Token::RParen)?;
            ctes.push(Cte {
                name,
                columns,
                query: Box::new(query),
            });
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }
        Ok(Some(WithClause { recursive, ctes }))
    }

    fn parse_simple_select(&mut self) -> Result<SelectStatement, ParserError> {
        self.expect_keyword(Keyword::SELECT)?;
        let distinct = if self.match_keyword(Keyword::DISTINCT) {
            self.advance();
            if self.match_keyword(Keyword::ON) {
                self.advance();
                self.expect_token(&Token::LParen)?;
                while !self.match_token(&Token::RParen) {
                    self.advance();
                }
                self.expect_token(&Token::RParen)?;
            }
            true
        } else {
            if self.match_keyword(Keyword::ALL) {
                self.advance();
            }
            false
        };
        let targets = self.parse_target_list()?;
        let from = self.parse_from_clause()?;
        let where_clause = if self.match_keyword(Keyword::WHERE) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        let group_by = if self.match_keyword(Keyword::GROUP_P) {
            self.advance();
            self.expect_keyword(Keyword::BY)?;
            let mut items = vec![self.parse_expr()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                items.push(self.parse_expr()?);
            }
            items
        } else {
            vec![]
        };
        let having = if self.match_keyword(Keyword::HAVING) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        Ok(SelectStatement {
            with: None,
            distinct,
            targets,
            from,
            where_clause,
            group_by,
            having,
            order_by: vec![],
            limit: None,
            offset: None,
            set_operation: None,
        })
    }

    pub(crate) fn parse_target_list(&mut self) -> Result<Vec<SelectTarget>, ParserError> {
        let mut targets = vec![self.parse_target_el()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            targets.push(self.parse_target_el()?);
        }
        Ok(targets)
    }

    fn parse_target_el(&mut self) -> Result<SelectTarget, ParserError> {
        if self.match_token(&Token::Star) {
            self.advance();
            return Ok(SelectTarget::Star(None));
        }
        let expr = self.parse_expr()?;
        let alias = if self.match_keyword(Keyword::AS) {
            self.advance();
            Some(self.parse_identifier()?)
        } else {
            self.parse_optional_alias()?
        };
        Ok(SelectTarget::Expr(expr, alias))
    }

    pub(crate) fn parse_from_clause(&mut self) -> Result<Vec<TableRef>, ParserError> {
        if !self.match_keyword(Keyword::FROM) {
            return Ok(vec![]);
        }
        self.advance();
        let mut tables = vec![self.parse_table_ref()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            tables.push(self.parse_table_ref()?);
        }
        Ok(tables)
    }

    pub(crate) fn parse_table_ref(&mut self) -> Result<TableRef, ParserError> {
        let mut left = self.parse_primary_table_ref()?;
        loop {
            let join_type = match self.peek_keyword() {
                Some(Keyword::JOIN) => {
                    self.advance();
                    JoinType::Inner
                }
                Some(Keyword::INNER_P) => {
                    self.advance();
                    self.expect_keyword(Keyword::JOIN)?;
                    JoinType::Inner
                }
                Some(Keyword::LEFT) => {
                    self.advance();
                    self.try_consume_keyword(Keyword::OUTER_P);
                    self.expect_keyword(Keyword::JOIN)?;
                    JoinType::Left
                }
                Some(Keyword::RIGHT) => {
                    self.advance();
                    self.try_consume_keyword(Keyword::OUTER_P);
                    self.expect_keyword(Keyword::JOIN)?;
                    JoinType::Right
                }
                Some(Keyword::FULL) => {
                    self.advance();
                    self.try_consume_keyword(Keyword::OUTER_P);
                    self.expect_keyword(Keyword::JOIN)?;
                    JoinType::Full
                }
                Some(Keyword::CROSS) => {
                    self.advance();
                    self.expect_keyword(Keyword::JOIN)?;
                    JoinType::Cross
                }
                _ => break,
            };
            let right = self.parse_primary_table_ref()?;
            let condition = if join_type != JoinType::Cross {
                if self.match_keyword(Keyword::ON) {
                    self.advance();
                    Some(self.parse_expr()?)
                } else if self.match_keyword(Keyword::USING) {
                    self.advance();
                    self.expect_token(&Token::LParen)?;
                    while !self.match_token(&Token::RParen) {
                        self.advance();
                    }
                    self.expect_token(&Token::RParen)?;
                    None
                } else {
                    None
                }
            } else {
                None
            };
            left = TableRef::Join {
                left: Box::new(left),
                right: Box::new(right),
                join_type,
                condition,
            };
        }
        Ok(left)
    }

    fn parse_primary_table_ref(&mut self) -> Result<TableRef, ParserError> {
        if self.match_token(&Token::LParen) {
            self.advance();
            if self.match_keyword(Keyword::SELECT) || self.match_keyword(Keyword::WITH) {
                let query = self.parse_select_statement()?;
                self.expect_token(&Token::RParen)?;
                let alias = self.parse_optional_alias()?;
                return Ok(TableRef::Subquery {
                    query: Box::new(query),
                    alias,
                });
            }
            let table_ref = self.parse_table_ref()?;
            self.expect_token(&Token::RParen)?;
            return Ok(table_ref);
        }
        if self.match_keyword(Keyword::LATERAL_P) {
            self.advance();
            self.expect_token(&Token::LParen)?;
            let query = self.parse_select_statement()?;
            self.expect_token(&Token::RParen)?;
            let alias = self.parse_optional_alias()?;
            return Ok(TableRef::Subquery {
                query: Box::new(query),
                alias,
            });
        }
        let name = self.parse_object_name()?;
        let alias = self.parse_optional_alias()?;
        Ok(TableRef::Table { name, alias })
    }

    fn parse_order_limit_offset(&mut self, stmt: &mut SelectStatement) -> Result<(), ParserError> {
        if self.match_keyword(Keyword::ORDER) {
            self.advance();
            self.expect_keyword(Keyword::BY)?;
            let mut items = Vec::new();
            loop {
                let expr = self.parse_expr()?;
                let asc = match self.peek_keyword() {
                    Some(Keyword::ASC) => {
                        self.advance();
                        Some(true)
                    }
                    Some(Keyword::DESC) => {
                        self.advance();
                        Some(false)
                    }
                    _ => None,
                };
                let nulls_first = if self.match_keyword(Keyword::NULLS_P) {
                    self.advance();
                    if self.match_keyword(Keyword::FIRST_P) {
                        self.advance();
                        Some(true)
                    } else {
                        self.expect_keyword(Keyword::LAST_P)?;
                        Some(false)
                    }
                } else {
                    None
                };
                items.push(OrderByItem {
                    expr,
                    asc,
                    nulls_first,
                });
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            stmt.order_by = items;
        }
        if self.match_keyword(Keyword::LIMIT) {
            self.advance();
            if self.match_keyword(Keyword::ALL) {
                self.advance();
                stmt.limit = None;
            } else {
                stmt.limit = Some(self.parse_expr()?);
            }
        }
        if self.match_keyword(Keyword::OFFSET) {
            self.advance();
            stmt.offset = Some(self.parse_expr()?);
        }
        Ok(())
    }
}
