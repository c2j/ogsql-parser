use crate::ast::{Expr, Literal, ObjectName, SelectStatement, WhenClause};
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;

impl Parser {
    pub(crate) fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        self.parse_expr_with_precedence(0)
    }

    fn parse_expr_with_precedence(&mut self, min_prec: u8) -> Result<Expr, ParserError> {
        let mut left = self.parse_unary_expr()?;

        loop {
            let (op_prec, op_str, is_right_assoc) = match self.get_infix_operator() {
                Some(info) => info,
                None => break,
            };

            if op_prec < min_prec {
                break;
            }

            self.advance();

            let right = self.parse_expr_with_precedence(if is_right_assoc {
                op_prec
            } else {
                op_prec + 1
            })?;

            left = Expr::BinaryOp {
                left: Box::new(left),
                op: op_str,
                right: Box::new(right),
            };
        }

        left = self.parse_postfix_ops(left)?;
        Ok(left)
    }

    fn parse_unary_expr(&mut self) -> Result<Expr, ParserError> {
        if self.match_keyword(Keyword::NOT) {
            self.advance();
            let expr = self.parse_expr_with_precedence(12)?;
            return Ok(Expr::UnaryOp {
                op: "NOT".to_string(),
                expr: Box::new(expr),
            });
        }
        if self.match_token(&Token::Minus) {
            self.advance();
            let expr = self.parse_expr_with_precedence(60)?;
            return Ok(Expr::UnaryOp {
                op: "-".to_string(),
                expr: Box::new(expr),
            });
        }
        if self.match_token(&Token::Plus) {
            self.advance();
            return self.parse_expr_with_precedence(60);
        }
        if self.match_token(&Token::At) {
            self.advance();
            let expr = self.parse_expr_with_precedence(60)?;
            return Ok(Expr::UnaryOp {
                op: "@".to_string(),
                expr: Box::new(expr),
            });
        }
        self.parse_primary_expr()
    }

    fn get_infix_operator(&self) -> Option<(u8, String, bool)> {
        match self.peek() {
            Token::Keyword(Keyword::OR) => Some((5, "OR".to_string(), false)),
            Token::Keyword(Keyword::AND) => Some((10, "AND".to_string(), false)),
            Token::Eq => Some((20, "=".to_string(), false)),
            Token::Lt => Some((20, "<".to_string(), false)),
            Token::Gt => Some((20, ">".to_string(), false)),
            Token::Op(op) => {
                let prec = match op.as_str() {
                    "<=" | ">=" | "<>" | "!=" => 20,
                    "||" => 30,
                    _ => 30,
                };
                Some((prec, op.clone(), false))
            }
            Token::Plus => Some((40, "+".to_string(), false)),
            Token::Minus => Some((40, "-".to_string(), false)),
            Token::Star => Some((50, "*".to_string(), false)),
            Token::Slash => Some((50, "/".to_string(), false)),
            Token::Percent => Some((50, "%".to_string(), false)),
            Token::Caret => Some((55, "^".to_string(), false)),
            Token::Typecast => Some((90, "::".to_string(), false)),
            _ => None,
        }
    }

    fn parse_postfix_ops(&mut self, mut left: Expr) -> Result<Expr, ParserError> {
        loop {
            match self.peek() {
                Token::Keyword(Keyword::IS) => {
                    self.advance();
                    if self.match_keyword(Keyword::NOT) {
                        self.advance();
                        if self.match_keyword(Keyword::NULL_P) {
                            self.advance();
                            left = Expr::IsNull {
                                expr: Box::new(left),
                                negated: true,
                            };
                        } else {
                            break;
                        }
                    } else if self.match_keyword(Keyword::NULL_P) {
                        self.advance();
                        left = Expr::IsNull {
                            expr: Box::new(left),
                            negated: false,
                        };
                    } else {
                        break;
                    }
                }
                Token::Keyword(Keyword::ISNULL) => {
                    self.advance();
                    left = Expr::IsNull {
                        expr: Box::new(left),
                        negated: false,
                    };
                }
                Token::Keyword(Keyword::NOTNULL) => {
                    self.advance();
                    left = Expr::IsNull {
                        expr: Box::new(left),
                        negated: true,
                    };
                }
                Token::Keyword(Keyword::BETWEEN) => {
                    self.advance();
                    let low = self.parse_expr_with_precedence(40)?;
                    self.expect_keyword(Keyword::AND)?;
                    let high = self.parse_expr_with_precedence(40)?;
                    left = Expr::Between {
                        expr: Box::new(left),
                        low: Box::new(low),
                        high: Box::new(high),
                        negated: false,
                    };
                }
                Token::Keyword(Keyword::NOT) => {
                    if let Some(tws) = self.tokens.get(self.pos + 1) {
                        match &tws.token {
                            Token::Keyword(Keyword::BETWEEN) => {
                                self.advance();
                                self.advance();
                                let low = self.parse_expr_with_precedence(40)?;
                                self.expect_keyword(Keyword::AND)?;
                                let high = self.parse_expr_with_precedence(40)?;
                                left = Expr::Between {
                                    expr: Box::new(left),
                                    low: Box::new(low),
                                    high: Box::new(high),
                                    negated: true,
                                };
                                continue;
                            }
                            Token::Keyword(Keyword::IN_P) => {
                                self.advance();
                                self.advance();
                                left = self.parse_in_expr(left, true)?;
                                continue;
                            }
                            Token::Keyword(Keyword::LIKE) => {
                                self.advance();
                                self.advance();
                                let pattern = self.parse_expr()?;
                                left = Expr::BinaryOp {
                                    left: Box::new(left),
                                    op: "NOT LIKE".to_string(),
                                    right: Box::new(pattern),
                                };
                                continue;
                            }
                            _ => break,
                        }
                    }
                    break;
                }
                Token::Keyword(Keyword::IN_P) => {
                    self.advance();
                    left = self.parse_in_expr(left, false)?;
                }
                Token::Keyword(Keyword::LIKE) => {
                    self.advance();
                    let pattern = self.parse_expr()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: "LIKE".to_string(),
                        right: Box::new(pattern),
                    };
                }
                Token::Keyword(Keyword::ILIKE) => {
                    self.advance();
                    let pattern = self.parse_expr()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: "ILIKE".to_string(),
                        right: Box::new(pattern),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_in_expr(&mut self, left: Expr, negated: bool) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        if self.match_keyword(Keyword::SELECT) || self.match_keyword(Keyword::WITH) {
            let subquery = self.parse_select_statement()?;
            self.expect_token(&Token::RParen)?;
            return Ok(Expr::InSubquery {
                expr: Box::new(left),
                subquery: Box::new(subquery),
                negated,
            });
        }
        let mut list = vec![self.parse_expr()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            list.push(self.parse_expr()?);
        }
        self.expect_token(&Token::RParen)?;
        Ok(Expr::InList {
            expr: Box::new(left),
            list,
            negated,
        })
    }

    pub(crate) fn parse_primary_expr(&mut self) -> Result<Expr, ParserError> {
        match self.peek().clone() {
            Token::Integer(n) => {
                self.advance();
                Ok(Expr::Literal(Literal::Integer(n)))
            }
            Token::Float(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::Float(s)))
            }
            Token::StringLiteral(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            Token::EscapeString(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            Token::BitString(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            Token::HexString(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            Token::NationalString(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            Token::DollarString(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            Token::Keyword(Keyword::TRUE_P) => {
                self.advance();
                Ok(Expr::Literal(Literal::Boolean(true)))
            }
            Token::Keyword(Keyword::FALSE_P) => {
                self.advance();
                Ok(Expr::Literal(Literal::Boolean(false)))
            }
            Token::Keyword(Keyword::NULL_P) => {
                self.advance();
                Ok(Expr::Literal(Literal::Null))
            }
            Token::Keyword(Keyword::DEFAULT) => {
                self.advance();
                Ok(Expr::Default)
            }
            Token::Param(n) => {
                self.advance();
                Ok(Expr::Parameter(n))
            }
            Token::Keyword(Keyword::EXISTS) => {
                self.advance();
                self.expect_token(&Token::LParen)?;
                let subquery = self.parse_select_statement()?;
                self.expect_token(&Token::RParen)?;
                Ok(Expr::Exists(Box::new(subquery)))
            }
            Token::Keyword(Keyword::CASE) => self.parse_case_expr(),
            Token::LParen => {
                self.advance();
                if self.match_keyword(Keyword::SELECT) || self.match_keyword(Keyword::WITH) {
                    let subquery = self.parse_select_statement()?;
                    self.expect_token(&Token::RParen)?;
                    return Ok(Expr::Subquery(Box::new(subquery)));
                }
                let expr = self.parse_expr()?;
                self.expect_token(&Token::RParen)?;
                Ok(expr)
            }
            Token::Keyword(Keyword::ARRAY) => {
                self.advance();
                if self.match_token(&Token::LParen) {
                    self.advance();
                    if self.match_keyword(Keyword::SELECT) || self.match_keyword(Keyword::WITH) {
                        let subquery = self.parse_select_statement()?;
                        self.expect_token(&Token::RParen)?;
                        return Ok(Expr::Subquery(Box::new(subquery)));
                    }
                    let mut elems = vec![self.parse_expr()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        elems.push(self.parse_expr()?);
                    }
                    self.expect_token(&Token::RParen)?;
                    return Ok(Expr::Array(elems));
                }
                Err(ParserError::UnexpectedToken {
                    position: self.pos,
                    expected: "'(' after ARRAY".to_string(),
                    got: format!("{:?}", self.peek()),
                })
            }
            Token::Ident(_) | Token::QuotedIdent(_) => {
                let name = self.parse_object_name()?;
                if self.match_token(&Token::LParen) {
                    return self.parse_function_call(name);
                }
                Ok(Expr::ColumnRef(name))
            }
            Token::Keyword(_) => {
                let name = self.parse_object_name()?;
                if self.match_token(&Token::LParen) {
                    return self.parse_function_call(name);
                }
                Ok(Expr::ColumnRef(name))
            }
            Token::SetIdent(s) => {
                self.advance();
                Ok(Expr::ColumnRef(vec![s]))
            }
            Token::Star => {
                self.advance();
                Ok(Expr::ColumnRef(vec!["*".to_string()]))
            }
            _ => Err(ParserError::UnexpectedToken {
                position: self.pos,
                expected: "expression".to_string(),
                got: format!("{:?}", self.peek()),
            }),
        }
    }

    fn parse_function_call(&mut self, name: ObjectName) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        if self.match_token(&Token::RParen) {
            self.advance();
            return Ok(Expr::FunctionCall {
                name,
                args: vec![],
                distinct: false,
            });
        }
        let distinct = self.try_consume_keyword(Keyword::DISTINCT);
        if self.match_token(&Token::Star) {
            self.advance();
            self.expect_token(&Token::RParen)?;
            return Ok(Expr::FunctionCall {
                name,
                args: vec![Expr::ColumnRef(vec!["*".to_string()])],
                distinct,
            });
        }
        let mut args = vec![self.parse_expr()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            args.push(self.parse_expr()?);
        }
        self.expect_token(&Token::RParen)?;
        Ok(Expr::FunctionCall {
            name,
            args,
            distinct,
        })
    }

    fn parse_case_expr(&mut self) -> Result<Expr, ParserError> {
        self.expect_keyword(Keyword::CASE)?;
        let operand = if !self.match_keyword(Keyword::WHEN) {
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        let mut whens = Vec::new();
        while self.match_keyword(Keyword::WHEN) {
            self.advance();
            let condition = self.parse_expr()?;
            self.expect_keyword(Keyword::THEN)?;
            let result = self.parse_expr()?;
            whens.push(WhenClause { condition, result });
        }
        let else_expr = if self.match_keyword(Keyword::ELSE) {
            self.advance();
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        self.expect_keyword(Keyword::END_P)?;
        Ok(Expr::Case {
            operand,
            whens,
            else_expr,
        })
    }
}
