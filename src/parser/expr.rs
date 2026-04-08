use crate::ast::{
    Expr, Literal, ObjectName, OrderByItem, SelectStatement, WhenClause, WindowFrame,
    WindowFrameBound, WindowSpec,
};
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
                    "<=" | ">=" | "<>" | "!=" | "<?>" => 20,
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
                    location: self.current_location(),
                    expected: "'(' after ARRAY".to_string(),
                    got: format!("{:?}", self.peek()),
                })
            }
            Token::Ident(_) | Token::QuotedIdent(_) => {
                let name = self.parse_object_name()?;
                // PostgreSQL typecast syntax: typename 'literal'
                if let Token::StringLiteral(s) = self.peek().clone() {
                    self.advance();
                    return Ok(Expr::TypeCast {
                        expr: Box::new(Expr::Literal(Literal::String(s))),
                        type_name: name.join("."),
                    });
                }
                if self.match_token(&Token::LParen) {
                    return self.parse_function_call(name);
                }
                Ok(Expr::ColumnRef(name))
            }
            Token::Keyword(kw) => {
                // CAST(expr AS type) — must handle before generic keyword arm
                if kw == Keyword::CAST {
                    self.advance();
                    self.expect_token(&Token::LParen)?;
                    let expr = self.parse_expr()?;
                    if !self.match_keyword(Keyword::AS) {
                        return Err(ParserError::UnexpectedToken {
                            location: self.current_location(),
                            expected: "AS in CAST expression".to_string(),
                            got: format!("{:?}", self.peek()),
                        });
                    }
                    self.advance();
                    let type_name = self.parse_object_name()?;
                    self.expect_token(&Token::RParen)?;
                    return Ok(Expr::TypeCast {
                        expr: Box::new(expr),
                        type_name: type_name.join("."),
                    });
                }
                let name = self.parse_object_name()?;
                // PostgreSQL typecast syntax: typename 'literal'
                if let Token::StringLiteral(s) = self.peek().clone() {
                    self.advance();
                    return Ok(Expr::TypeCast {
                        expr: Box::new(Expr::Literal(Literal::String(s))),
                        type_name: name.join("."),
                    });
                }
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
                location: self.current_location(),
                expected: "expression".to_string(),
                got: format!("{:?}", self.peek()),
            }),
        }
    }

    fn parse_function_call(&mut self, name: ObjectName) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        if self.match_token(&Token::RParen) {
            self.advance();
            let over = self.try_parse_over_clause()?;
            return Ok(Expr::FunctionCall {
                name,
                args: vec![],
                distinct: false,
                over,
            });
        }
        let distinct = self.try_consume_keyword(Keyword::DISTINCT);
        if self.match_token(&Token::Star) {
            self.advance();
            self.expect_token(&Token::RParen)?;
            let over = self.try_parse_over_clause()?;
            return Ok(Expr::FunctionCall {
                name,
                args: vec![Expr::ColumnRef(vec!["*".to_string()])],
                distinct,
                over,
            });
        }
        let mut args = vec![self.parse_expr()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            args.push(self.parse_expr()?);
        }
        self.expect_token(&Token::RParen)?;
        let over = self.try_parse_over_clause()?;
        Ok(Expr::FunctionCall {
            name,
            args,
            distinct,
            over,
        })
    }

    /// Try to parse OVER clause after a function call.
    /// Returns None if the next token is not OVER.
    fn try_parse_over_clause(&mut self) -> Result<Option<WindowSpec>, ParserError> {
        if !self.match_keyword(Keyword::OVER) {
            return Ok(None);
        }
        self.advance();

        if self.match_token(&Token::LParen) {
            // OVER (window_specification)
            self.advance();
            let spec = self.parse_window_specification()?;
            self.expect_token(&Token::RParen)?;
            Ok(Some(spec))
        } else {
            // OVER window_name (identifier)
            let name = self.parse_identifier()?;
            Ok(Some(WindowSpec {
                partition_by: vec![],
                order_by: vec![],
                frame: None,
                window_name: Some(name),
            }))
        }
    }

    /// Parse the body of a window specification (inside parens).
    /// Grammar: [existing_window_name] [PARTITION BY expr_list] [ORDER BY sort_clause] [frame_clause]
    fn parse_window_specification(&mut self) -> Result<WindowSpec, ParserError> {
        // Try to parse existing window name (an identifier that is NOT PARTITION, ORDER, ROWS, RANGE)
        let window_name = self.try_parse_window_name();

        // PARTITION BY expr_list
        let partition_by = if self.match_keyword(Keyword::PARTITION) {
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

        // ORDER BY sort_clause
        let order_by = if self.match_keyword(Keyword::ORDER) {
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
            items
        } else {
            vec![]
        };

        // Frame clause: ROWS|RANGE frame_extent
        let frame = if self.match_keyword(Keyword::ROWS) || self.match_keyword(Keyword::RANGE) {
            let mode = if self.match_keyword(Keyword::ROWS) {
                self.advance();
                "ROWS".to_string()
            } else {
                self.advance();
                "RANGE".to_string()
            };
            let (start, end) = self.parse_frame_extent()?;
            Some(WindowFrame { mode, start, end })
        } else {
            None
        };

        Ok(WindowSpec {
            partition_by,
            order_by,
            frame,
            window_name,
        })
    }

    /// Try to parse an existing window name (identifier).
    /// Returns None if the next token looks like PARTITION, ORDER, ROWS, RANGE, or closing paren.
    fn try_parse_window_name(&mut self) -> Option<String> {
        match self.peek_keyword() {
            Some(Keyword::PARTITION) | Some(Keyword::ORDER) => None,
            _ => {
                match self.peek() {
                    Token::Ident(_) | Token::QuotedIdent(_) => {
                        // This is a window name if it's followed by something other than
                        // just more identifiers (i.e., PARTITION/ORDER follows, or RParen)
                        // Simple heuristic: take it as a window name
                        let name = match self.peek().clone() {
                            Token::Ident(s) => s,
                            Token::QuotedIdent(s) => s,
                            _ => unreachable!(),
                        };
                        self.advance();
                        Some(name)
                    }
                    _ => None,
                }
            }
        }
    }

    /// Parse frame_extent: BETWEEN frame_bound AND frame_bound | frame_bound
    fn parse_frame_extent(
        &mut self,
    ) -> Result<(Option<WindowFrameBound>, Option<WindowFrameBound>), ParserError> {
        if self.match_keyword(Keyword::BETWEEN) {
            self.advance();
            let start = self.parse_frame_bound()?;
            self.expect_keyword(Keyword::AND)?;
            let end = self.parse_frame_bound()?;
            Ok((Some(start), Some(end)))
        } else {
            let start = self.parse_frame_bound()?;
            Ok((Some(start), None))
        }
    }

    /// Parse a single frame bound:
    ///   UNBOUNDED PRECEDING | UNBOUNDED FOLLOWING | CURRENT ROW
    ///   n PRECEDING | n FOLLOWING
    fn parse_frame_bound(&mut self) -> Result<WindowFrameBound, ParserError> {
        if self.match_keyword(Keyword::UNBOUNDED) {
            self.advance();
            if self.match_keyword(Keyword::PRECEDING) {
                self.advance();
                Ok(WindowFrameBound {
                    direction: "UNBOUNDED PRECEDING".to_string(),
                    offset: None,
                })
            } else {
                self.expect_keyword(Keyword::FOLLOWING)?;
                Ok(WindowFrameBound {
                    direction: "UNBOUNDED FOLLOWING".to_string(),
                    offset: None,
                })
            }
        } else if self.match_keyword(Keyword::CURRENT_P) {
            self.advance();
            self.expect_keyword(Keyword::ROW)?;
            Ok(WindowFrameBound {
                direction: "CURRENT ROW".to_string(),
                offset: None,
            })
        } else {
            // numeric offset PRECEDING | FOLLOWING
            let offset = match self.peek().clone() {
                Token::Integer(n) => {
                    self.advance();
                    Some(n)
                }
                _ => None,
            };
            if self.match_keyword(Keyword::PRECEDING) {
                self.advance();
                Ok(WindowFrameBound {
                    direction: "PRECEDING".to_string(),
                    offset,
                })
            } else {
                self.expect_keyword(Keyword::FOLLOWING)?;
                Ok(WindowFrameBound {
                    direction: "FOLLOWING".to_string(),
                    offset,
                })
            }
        }
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
