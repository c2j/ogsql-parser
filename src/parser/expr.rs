use crate::ast::{
    DataType, Expr, Literal, ObjectName, OrderByItem, SelectStatement, WhenClause, WindowFrame,
    WindowFrameBound, WindowFrameDirection, WindowFrameMode, WindowSpec, XmlAttribute,
    XmlAttributes, XmlContent, XmlOption,
};
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;

impl Parser {
    fn validate_func(
        &mut self,
        name: &ObjectName,
        arg_count: usize,
        distinct: bool,
        has_over: bool,
    ) {
        let lower = name.last().map(|s| s.to_lowercase()).unwrap_or_default();
        let last = lower.split('.').last().unwrap_or_default();
        let warnings = crate::parser::function_validator::validate_function_call(
            &last,
            arg_count,
            distinct,
            has_over,
            self.current_location(),
        );
        for w in warnings {
            self.add_error(w);
        }
    }

    pub(crate) fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        self.enter_scope()?;
        let result = self.parse_expr_with_precedence(0);
        self.leave_scope();
        result
    }

    fn parse_expr_with_precedence(&mut self, min_prec: u8) -> Result<Expr, ParserError> {
        let mut left = self.parse_unary_expr()?;

        loop {
            // Try postfix operators first — they bind tighter than any infix operator.
            // This must be inside the loop so that after consuming e.g. "IS NULL",
            // we continue and can still pick up subsequent infix operators like "OR".
            if self.try_postfix_op(&mut left)? {
                continue;
            }

            let (op_prec, op_str, is_right_assoc) = match self.get_infix_operator() {
                Some(info) => info,
                None => break,
            };

            if op_prec < min_prec {
                break;
            }

            self.advance();

            if op_str == "::" {
                let type_name = self.parse_data_type()?;
                left = Expr::TypeCast {
                    expr: Box::new(left),
                    type_name,
                    default: None,
                    format: None,
                };
                continue;
            }

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

        Ok(left)
    }

    fn parse_unary_expr(&mut self) -> Result<Expr, ParserError> {
        if self.match_keyword(Keyword::PRIOR) {
            self.advance();
            let expr = self.parse_expr_with_precedence(15)?;
            return Ok(Expr::Prior(Box::new(expr)));
        }
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
        if let Token::Op(op) = self.peek() {
            if matches!(
                op.as_str(),
                "|/" | "||/" | "!!" | "?|" | "?-" | "?-|" | "?||"
            ) {
                let op_str = op.clone();
                self.advance();
                let expr = self.parse_expr_with_precedence(60)?;
                return Ok(Expr::UnaryOp {
                    op: op_str,
                    expr: Box::new(expr),
                });
            }
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

    fn try_postfix_op(&mut self, left: &mut Expr) -> Result<bool, ParserError> {
        match self.peek() {
            Token::Keyword(Keyword::IS) => {
                if let Some(next) = self.tokens.get(self.pos + 1) {
                    match &next.token {
                        Token::Keyword(Keyword::NULL_P) => {
                            self.advance();
                            self.advance();
                            *left = Expr::IsNull {
                                expr: Box::new(std::mem::replace(left, Expr::Default)),
                                negated: false,
                            };
                            return Ok(true);
                        }
                        Token::Keyword(Keyword::NOT) => {
                            if let Some(next2) = self.tokens.get(self.pos + 2) {
                                if matches!(&next2.token, Token::Keyword(Keyword::NULL_P)) {
                                    self.advance();
                                    self.advance();
                                    self.advance();
                                    *left = Expr::IsNull {
                                        expr: Box::new(std::mem::replace(left, Expr::Default)),
                                        negated: true,
                                    };
                                    return Ok(true);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(false)
            }
            Token::Keyword(Keyword::ISNULL) => {
                self.advance();
                *left = Expr::IsNull {
                    expr: Box::new(std::mem::replace(left, Expr::Default)),
                    negated: false,
                };
                Ok(true)
            }
            Token::Keyword(Keyword::NOTNULL) => {
                self.advance();
                *left = Expr::IsNull {
                    expr: Box::new(std::mem::replace(left, Expr::Default)),
                    negated: true,
                };
                Ok(true)
            }
            Token::Keyword(Keyword::BETWEEN) => {
                self.advance();
                let low = self.parse_expr_with_precedence(40)?;
                self.expect_keyword(Keyword::AND)?;
                let high = self.parse_expr_with_precedence(40)?;
                *left = Expr::Between {
                    expr: Box::new(std::mem::replace(left, Expr::Default)),
                    low: Box::new(low),
                    high: Box::new(high),
                    negated: false,
                };
                Ok(true)
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
                            *left = Expr::Between {
                                expr: Box::new(std::mem::replace(left, Expr::Default)),
                                low: Box::new(low),
                                high: Box::new(high),
                                negated: true,
                            };
                            return Ok(true);
                        }
                        Token::Keyword(Keyword::IN_P) => {
                            self.advance();
                            self.advance();
                            *left =
                                self.parse_in_expr(std::mem::replace(left, Expr::Default), true)?;
                            return Ok(true);
                        }
                        Token::Keyword(Keyword::LIKE) => {
                            self.advance();
                            self.advance();
                            let pattern = self.parse_expr()?;
                            *left = Expr::BinaryOp {
                                left: Box::new(std::mem::replace(left, Expr::Default)),
                                op: "NOT LIKE".to_string(),
                                right: Box::new(pattern),
                            };
                            return Ok(true);
                        }
                        _ => {}
                    }
                }
                Ok(false)
            }
            Token::Keyword(Keyword::IN_P) => {
                self.advance();
                *left = self.parse_in_expr(std::mem::replace(left, Expr::Default), false)?;
                Ok(true)
            }
            Token::Keyword(Keyword::LIKE) => {
                self.advance();
                let pattern = self.parse_expr()?;
                *left = Expr::BinaryOp {
                    left: Box::new(std::mem::replace(left, Expr::Default)),
                    op: "LIKE".to_string(),
                    right: Box::new(pattern),
                };
                Ok(true)
            }
            Token::Keyword(Keyword::ILIKE) => {
                self.advance();
                let pattern = self.parse_expr()?;
                *left = Expr::BinaryOp {
                    left: Box::new(std::mem::replace(left, Expr::Default)),
                    op: "ILIKE".to_string(),
                    right: Box::new(pattern),
                };
                Ok(true)
            }
            Token::LBracket => {
                self.advance();
                let index = self.parse_expr()?;
                self.expect_token(&Token::RBracket)?;
                *left = Expr::Subscript {
                    object: Box::new(std::mem::replace(left, Expr::Default)),
                    index: Box::new(index),
                };
                Ok(true)
            }
            Token::LParen => {
                if let Some(next) = self.tokens.get(self.pos + 1) {
                    if matches!(&next.token, Token::Plus) {
                        if let Some(next2) = self.tokens.get(self.pos + 2) {
                            if matches!(&next2.token, Token::RParen) {
                                let loc = self.current_location();
                                self.advance();
                                self.advance();
                                self.advance();
                                self.add_error(ParserError::Warning {
                                    message: "Oracle-style outer join operator '(+)' is deprecated, use standard JOIN syntax instead".to_string(),
                                    location: loc,
                                });
                                return Ok(true);
                            }
                        }
                    }
                }
                Ok(false)
            }
            _ => Ok(false),
        }
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
                if self.match_token(&Token::Dot) {
                    if let Some(next) = self.tokens.get(self.pos + 1) {
                        if let Token::Integer(frac) = &next.token {
                            let frac = frac.clone();
                            self.advance();
                            self.advance();
                            let float_str = format!("{}.{}", n, frac);
                            return Ok(Expr::Literal(Literal::Float(float_str)));
                        }
                    }
                }
                if let Token::Ident(s) = self.peek() {
                    let lower = s.to_lowercase();
                    if lower.starts_with('e')
                        && lower.len() > 1
                        && lower[1..].chars().all(|c| c.is_ascii_digit())
                    {
                        let exp_str = s.clone();
                        self.advance();
                        let float_str = format!("{}{}", n, exp_str);
                        return Ok(Expr::Literal(Literal::Float(float_str)));
                    }
                }
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
                Ok(Expr::Literal(Literal::EscapeString(s)))
            }
            Token::BitString(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::BitString(s)))
            }
            Token::HexString(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::HexString(s)))
            }
            Token::NationalString(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::NationalString(s)))
            }
            Token::DollarString { tag, body } => {
                self.advance();
                Ok(Expr::Literal(Literal::DollarString { tag, body }))
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
                if !self.match_token(&Token::RParen) && !self.match_token(&Token::Comma) {
                    let val = self.parse_expr()?;
                    if self.match_keyword(Keyword::ON) || self.match_ident_str("on") {
                        self.advance();
                        if self.match_ident_str("CONVERSION") {
                            self.advance();
                        }
                        if self.match_ident_str("ERROR") {
                            self.advance();
                        }
                    }
                    Ok(val)
                } else {
                    Ok(Expr::Default)
                }
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
                if self.match_token(&Token::Comma) {
                    let mut elems = vec![expr];
                    loop {
                        self.advance();
                        elems.push(self.parse_expr()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                    self.expect_token(&Token::RParen)?;
                    return Ok(Expr::RowConstructor(elems));
                }
                self.expect_token(&Token::RParen)?;
                Ok(Expr::Parenthesized(Box::new(expr)))
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
                } else if self.match_token(&Token::LBracket) {
                    self.advance();
                    if self.match_keyword(Keyword::SELECT) || self.match_keyword(Keyword::WITH) {
                        let subquery = self.parse_select_statement()?;
                        self.expect_token(&Token::RBracket)?;
                        return Ok(Expr::Subquery(Box::new(subquery)));
                    }
                    let mut elems = vec![self.parse_expr()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        elems.push(self.parse_expr()?);
                    }
                    self.expect_token(&Token::RBracket)?;
                    return Ok(Expr::Array(elems));
                }
                Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "'(' or '[' after ARRAY".to_string(),
                    got: format!("{:?}", self.peek()),
                })
            }
            Token::Ident(_) | Token::QuotedIdent(_) => {
                let name = self.parse_column_ref_or_qualified_star()?;
                Ok(name)
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
                    let type_name = self.parse_data_type()?;
                    let default = if self.match_keyword(Keyword::DEFAULT)
                        || self.match_ident_str("default")
                    {
                        self.advance();
                        let val = self.parse_expr()?;
                        if self.match_keyword(Keyword::ON) || self.match_ident_str("on") {
                            self.advance();
                            if self.match_ident_str("CONVERSION") {
                                self.advance();
                            }
                            if self.match_ident_str("ERROR") {
                                self.advance();
                            }
                        }
                        Some(Box::new(val))
                    } else {
                        None
                    };
                    let format = if default.is_some() && self.match_token(&Token::Comma) {
                        self.advance();
                        Some(Box::new(self.parse_expr()?))
                    } else {
                        None
                    };
                    self.expect_token(&Token::RParen)?;
                    return Ok(Expr::TypeCast {
                        expr: Box::new(expr),
                        type_name,
                        default,
                        format,
                    });
                }
                if kw == Keyword::XMLELEMENT {
                    self.advance();
                    return self.parse_xml_element();
                }
                if kw == Keyword::XMLCONCAT {
                    self.advance();
                    return self.parse_xml_concat();
                }
                if kw == Keyword::XMLFOREST {
                    self.advance();
                    return self.parse_xml_forest();
                }
                if kw == Keyword::XMLPARSE {
                    self.advance();
                    return self.parse_xml_parse();
                }
                if kw == Keyword::XMLPI {
                    self.advance();
                    return self.parse_xml_pi();
                }
                if kw == Keyword::XMLROOT {
                    self.advance();
                    return self.parse_xml_root();
                }
                if kw == Keyword::XMLSERIALIZE {
                    self.advance();
                    return self.parse_xml_serialize();
                }
                if kw == Keyword::INTERVAL {
                    self.advance();
                    if let Token::StringLiteral(s) = self.peek().clone() {
                        self.advance();
                        // Check for optional unit keyword: INTERVAL '2' MONTH, INTERVAL '1' YEAR, etc.
                        let unit = self.peek_keyword();
                        if let Some(unit_kw) = unit {
                            if matches!(
                                unit_kw,
                                Keyword::DAY_P
                                    | Keyword::YEAR_P
                                    | Keyword::MONTH_P
                                    | Keyword::HOUR_P
                                    | Keyword::MINUTE_P
                                    | Keyword::SECOND_P
                                    | Keyword::DAY_HOUR_P
                                    | Keyword::DAY_MINUTE_P
                                    | Keyword::DAY_SECOND_P
                                    | Keyword::HOUR_MINUTE_P
                                    | Keyword::HOUR_SECOND_P
                                    | Keyword::MINUTE_SECOND_P
                                    | Keyword::YEAR_MONTH_P
                            ) {
                                let unit_name = unit_kw.as_str().to_string();
                                self.advance();
                                return Ok(Expr::SpecialFunction {
                                    name: "interval".to_string(),
                                    args: vec![
                                        Expr::Literal(Literal::String(s)),
                                        Expr::ColumnRef(vec![unit_name]),
                                    ],
                                });
                            }
                        }
                        return Ok(Expr::TypeCast {
                            expr: Box::new(Expr::Literal(Literal::String(s))),
                            type_name: DataType::Custom(vec!["interval".to_string()], Vec::new()),
                            default: None,
                            format: None,
                        });
                    }
                    let expr = self.parse_expr()?;
                    let unit = self.peek_keyword();
                    if let Some(unit_kw) = unit {
                        if matches!(
                            unit_kw,
                            Keyword::DAY_P
                                | Keyword::YEAR_P
                                | Keyword::MONTH_P
                                | Keyword::HOUR_P
                                | Keyword::MINUTE_P
                                | Keyword::SECOND_P
                                | Keyword::DAY_HOUR_P
                                | Keyword::DAY_MINUTE_P
                                | Keyword::DAY_SECOND_P
                                | Keyword::HOUR_MINUTE_P
                                | Keyword::HOUR_SECOND_P
                                | Keyword::MINUTE_SECOND_P
                                | Keyword::YEAR_MONTH_P
                        ) {
                            let unit_name = unit_kw.as_str().to_string();
                            self.advance();
                            return Ok(Expr::SpecialFunction {
                                name: "interval".to_string(),
                                args: vec![expr, Expr::ColumnRef(vec![unit_name])],
                            });
                        }
                    }
                    return Ok(expr);
                }
                // Built-in expression keywords that are RESERVED but valid as expressions
                if matches!(
                    kw,
                    Keyword::SYSDATE
                        | Keyword::ROWNUM
                        | Keyword::CURRENT_DATE
                        | Keyword::CURRENT_CATALOG
                        | Keyword::CURRENT_USER
                        | Keyword::SESSION_USER
                ) {
                    self.advance();
                    return Ok(Expr::ColumnRef(vec![kw.as_str().to_string()]));
                }
                if matches!(
                    kw,
                    Keyword::CURRENT_TIME
                        | Keyword::CURRENT_TIMESTAMP
                        | Keyword::LOCALTIME
                        | Keyword::LOCALTIMESTAMP
                ) {
                    let name = kw.as_str().to_string();
                    self.advance();
                    if self.match_token(&Token::LParen) {
                        self.advance();
                        let precision = self.parse_expr()?;
                        self.expect_token(&Token::RParen)?;
                        return Ok(Expr::SpecialFunction {
                            name,
                            args: vec![precision],
                        });
                    }
                    return Ok(Expr::ColumnRef(vec![name]));
                }
                let name = self.parse_object_name()?;
                // PostgreSQL typecast syntax: typename 'literal'
                if let Token::StringLiteral(s) = self.peek().clone() {
                    self.advance();
                    return Ok(Expr::TypeCast {
                        expr: Box::new(Expr::Literal(Literal::String(s))),
                        type_name: DataType::Custom(name, Vec::new()),
                        default: None,
                        format: None,
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
            Token::LBracket => {
                self.advance();
                let mut elems = vec![self.parse_expr()?];
                while self.match_token(&Token::Comma) {
                    self.advance();
                    elems.push(self.parse_expr()?);
                }
                self.expect_token(&Token::RBracket)?;
                Ok(Expr::Array(elems))
            }
            _ => Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "expression".to_string(),
                got: format!("{:?}", self.peek()),
            }),
        }
    }

    fn parse_column_ref_or_qualified_star(&mut self) -> Result<Expr, ParserError> {
        let first = self.parse_identifier()?;
        if self.match_token(&Token::Dot) {
            self.advance();
            if self.match_token(&Token::Star) {
                self.advance();
                return Ok(Expr::QualifiedStar(first));
            }
            let mut name = vec![first];
            name.push(self.parse_identifier()?);
            while self.match_token(&Token::Dot) {
                self.advance();
                name.push(self.parse_identifier()?);
            }
            let obj_name = name;
            if let Token::StringLiteral(s) = self.peek().clone() {
                self.advance();
                return Ok(Expr::TypeCast {
                    expr: Box::new(Expr::Literal(Literal::String(s))),
                    type_name: DataType::Custom(obj_name, Vec::new()),
                    default: None,
                    format: None,
                });
            }
            if self.match_token(&Token::LParen) {
                // Check for Oracle-style outer join (+): LParen at pos, Plus at pos+1, RParen at pos+2
                if self.tokens.len() > self.pos + 2 {
                    let next = &self.tokens[self.pos + 1].token;
                    let next2 = &self.tokens[self.pos + 2].token;
                    if matches!(next, Token::Plus) && matches!(next2, Token::RParen) {
                        self.advance();
                        self.advance();
                        self.advance();
                        self.add_error(ParserError::Warning {
                            message: "Oracle-style outer join operator '(+)' is deprecated, use standard JOIN syntax instead".to_string(),
                            location: self.current_location(),
                        });
                        return Ok(Expr::ColumnRef(obj_name));
                    }
                }
                return self.parse_function_call(obj_name);
            }
            return Ok(Expr::ColumnRef(obj_name));
        }
        let name = vec![first];
        if let Token::StringLiteral(s) = self.peek().clone() {
            self.advance();
            return Ok(Expr::TypeCast {
                expr: Box::new(Expr::Literal(Literal::String(s))),
                type_name: DataType::Custom(name, Vec::new()),
                default: None,
                format: None,
            });
        }
        if self.match_token(&Token::LParen) {
            // Check for Oracle-style outer join (+): LParen at pos, Plus at pos+1, RParen at pos+2
            if self.tokens.len() > self.pos + 2 {
                let next = &self.tokens[self.pos + 1].token;
                let next2 = &self.tokens[self.pos + 2].token;
                if matches!(next, Token::Plus) && matches!(next2, Token::RParen) {
                    self.advance();
                    self.advance();
                    self.advance();
                    self.add_error(ParserError::Warning {
                        message: "Oracle-style outer join operator '(+)' is deprecated, use standard JOIN syntax instead".to_string(),
                        location: self.current_location(),
                    });
                    return Ok(Expr::ColumnRef(name));
                }
            }
            return self.parse_function_call(name);
        }
        Ok(Expr::ColumnRef(name))
    }

    fn validate_function_args(
        &mut self,
        name: &ObjectName,
        args: &[Expr],
        distinct: bool,
        over: &Option<Box<Expr>>,
    ) {
        if let Some(last) = name.last() {
            let loc = self.current_location();
            let warnings = crate::parser::function_validator::validate_function_call(
                last,
                args.len(),
                distinct,
                over.is_some(),
                loc,
            );
            for w in warnings {
                self.add_error(w);
            }
        }
    }

    fn parse_function_call(&mut self, name: ObjectName) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;

        let lower_name = name.last().map(|s| s.to_lowercase()).unwrap_or_default();
        if lower_name == "overlay" {
            return self.parse_overlay_function(name);
        }
        if lower_name == "position" {
            return self.parse_position_function(name);
        }
        if lower_name == "substring" || lower_name == "substr" {
            return self.parse_substring_function(name);
        }
        if lower_name == "extract" {
            return self.parse_extract_function(name);
        }
        if lower_name == "trim" {
            return self.parse_trim_function(name);
        }

        if self.match_token(&Token::RParen) {
            self.advance();
            let filter = self.try_parse_filter()?;
            let within_group = self.try_parse_within_group()?;
            let over = self.try_parse_over_clause()?;
            self.validate_func(&name, 0, false, over.is_some());
            return Ok(Expr::FunctionCall {
                name,
                args: vec![],
                distinct: false,
                over,
                filter,
                within_group,
            });
        }
        let distinct = self.try_consume_keyword(Keyword::DISTINCT);
        if self.match_token(&Token::Star) {
            self.advance();
            self.expect_token(&Token::RParen)?;
            let filter = self.try_parse_filter()?;
            let within_group = self.try_parse_within_group()?;
            let over = self.try_parse_over_clause()?;
            self.validate_func(&name, 1, distinct, over.is_some());
            return Ok(Expr::FunctionCall {
                name,
                args: vec![Expr::ColumnRef(vec!["*".to_string()])],
                distinct,
                over,
                filter,
                within_group,
            });
        }
        let mut args = vec![self.parse_expr()?];
        if self.match_keyword(Keyword::DEFAULT) || self.match_ident_str("default") {
            self.advance();
            args.push(self.parse_expr()?);
            if self.match_keyword(Keyword::ON) || self.match_ident_str("on") {
                self.advance();
                if self.match_ident_str("CONVERSION") {
                    self.advance();
                }
                if self.match_ident_str("ERROR") {
                    self.advance();
                }
            }
            if self.match_token(&Token::Comma) {
                self.advance();
                args.push(self.parse_expr()?);
            }
        }
        while self.match_token(&Token::Comma) {
            self.advance();
            args.push(self.parse_expr()?);
        }
        self.expect_token(&Token::RParen)?;
        let filter = self.try_parse_filter()?;
        let within_group = self.try_parse_within_group()?;
        let over = self.try_parse_over_clause()?;
        self.validate_func(&name, args.len(), distinct, over.is_some());
        Ok(Expr::FunctionCall {
            name,
            args,
            distinct,
            over,
            filter,
            within_group,
        })
    }

    fn parse_overlay_function(&mut self, name: ObjectName) -> Result<Expr, ParserError> {
        let arg1 = self.parse_expr()?;
        self.expect_keyword(Keyword::PLACING)?;
        let arg2 = self.parse_expr()?;
        self.expect_keyword(Keyword::FROM)?;
        let arg3 = self.parse_expr()?;
        let arg4 = if self.try_consume_keyword(Keyword::FOR) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect_token(&Token::RParen)?;
        let mut args = vec![arg1, arg2, arg3];
        if let Some(a) = arg4 {
            args.push(a);
        }
        Ok(Expr::SpecialFunction {
            name: name.join("."),
            args,
        })
    }

    fn parse_position_function(&mut self, name: ObjectName) -> Result<Expr, ParserError> {
        let arg1 = self.parse_primary_expr()?;
        if self.match_keyword(Keyword::IN_P) {
            self.advance();
        } else if self.match_ident_str("IN") {
            self.advance();
        }
        let arg2 = self.parse_primary_expr()?;
        self.expect_token(&Token::RParen)?;
        Ok(Expr::SpecialFunction {
            name: name.join("."),
            args: vec![arg1, arg2],
        })
    }

    fn parse_substring_function(&mut self, name: ObjectName) -> Result<Expr, ParserError> {
        let arg1 = self.parse_expr()?;
        let mut args = vec![arg1];
        if self.try_consume_keyword(Keyword::FROM) {
            args.push(self.parse_expr()?);
            if self.try_consume_keyword(Keyword::FOR) {
                args.push(self.parse_expr()?);
            }
        } else if self.try_consume_keyword(Keyword::FOR) {
            args.push(self.parse_expr()?);
        } else if self.match_token(&Token::Comma) {
            self.advance();
            args.push(self.parse_expr()?);
            if self.match_token(&Token::Comma) {
                self.advance();
                args.push(self.parse_expr()?);
            }
        }
        self.expect_token(&Token::RParen)?;
        Ok(Expr::SpecialFunction {
            name: name.join("."),
            args,
        })
    }

    fn parse_extract_function(&mut self, name: ObjectName) -> Result<Expr, ParserError> {
        let field = self.parse_identifier()?;
        self.expect_keyword(Keyword::FROM)?;
        let expr = self.parse_expr()?;
        self.expect_token(&Token::RParen)?;
        Ok(Expr::SpecialFunction {
            name: name.join("."),
            args: vec![Expr::ColumnRef(vec![field]), expr],
        })
    }

    fn parse_trim_function(&mut self, name: ObjectName) -> Result<Expr, ParserError> {
        let direction = if self.match_keyword(Keyword::LEADING)
            || self.match_keyword(Keyword::TRAILING)
            || self.match_keyword(Keyword::BOTH)
        {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        if let Some(dir) = direction {
            if self.match_keyword(Keyword::FROM) {
                // TRIM(direction FROM expr) — no explicit chars
                self.advance();
                let source = self.parse_expr()?;
                self.expect_token(&Token::RParen)?;
                Ok(Expr::SpecialFunction {
                    name: name.join("."),
                    args: vec![Expr::ColumnRef(vec![dir]), source],
                })
            } else {
                // TRIM(direction chars FROM expr)
                let chars = self.parse_expr()?;
                self.expect_keyword(Keyword::FROM)?;
                let source = self.parse_expr()?;
                self.expect_token(&Token::RParen)?;
                Ok(Expr::SpecialFunction {
                    name: name.join("."),
                    args: vec![Expr::ColumnRef(vec![dir]), chars, source],
                })
            }
        } else {
            // No direction keyword — could be TRIM(chars FROM expr) or TRIM(expr)
            let first = self.parse_expr()?;
            if self.match_keyword(Keyword::FROM) {
                // TRIM(chars FROM expr)
                self.advance();
                let source = self.parse_expr()?;
                self.expect_token(&Token::RParen)?;
                Ok(Expr::SpecialFunction {
                    name: name.join("."),
                    args: vec![first, source],
                })
            } else {
                // Regular function call: TRIM(expr [, ...])
                let mut args = vec![first];
                while self.match_token(&Token::Comma) {
                    self.advance();
                    args.push(self.parse_expr()?);
                }
                self.expect_token(&Token::RParen)?;
                let filter = self.try_parse_filter()?;
                let within_group = self.try_parse_within_group()?;
                let over = self.try_parse_over_clause()?;
                self.validate_func(&name, args.len(), false, over.is_some());
                Ok(Expr::FunctionCall {
                    name,
                    args,
                    distinct: false,
                    over,
                    filter,
                    within_group,
                })
            }
        }
    }

    fn try_parse_filter(&mut self) -> Result<Option<Box<Expr>>, ParserError> {
        if self.match_ident_str("FILTER") {
            self.advance();
            self.expect_token(&Token::LParen)?;
            self.expect_keyword(Keyword::WHERE)?;
            let expr = self.parse_expr()?;
            self.expect_token(&Token::RParen)?;
            Ok(Some(Box::new(expr)))
        } else {
            Ok(None)
        }
    }

    fn try_parse_within_group(&mut self) -> Result<Vec<OrderByItem>, ParserError> {
        if self.match_keyword(Keyword::WITHIN) {
            self.advance();
            self.expect_keyword(Keyword::GROUP_P)?;
            self.expect_token(&Token::LParen)?;
            self.expect_keyword(Keyword::ORDER)?;
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
                items.push(OrderByItem {
                    expr,
                    asc,
                    nulls_first: None,
                });
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect_token(&Token::RParen)?;
            Ok(items)
        } else {
            Ok(Vec::new())
        }
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
        let frame = if self.match_keyword(Keyword::ROWS)
            || self.match_keyword(Keyword::RANGE)
            || self.match_keyword(Keyword::GROUPS)
        {
            let mode = if self.match_keyword(Keyword::ROWS) {
                self.advance();
                WindowFrameMode::Rows
            } else if self.match_keyword(Keyword::RANGE) {
                self.advance();
                WindowFrameMode::Range
            } else {
                self.advance();
                WindowFrameMode::Groups
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
                    direction: WindowFrameDirection::UnboundedPreceding,
                })
            } else {
                self.expect_keyword(Keyword::FOLLOWING)?;
                Ok(WindowFrameBound {
                    direction: WindowFrameDirection::UnboundedFollowing,
                })
            }
        } else if self.match_keyword(Keyword::CURRENT_P) {
            self.advance();
            self.expect_keyword(Keyword::ROW)?;
            Ok(WindowFrameBound {
                direction: WindowFrameDirection::CurrentRow,
            })
        } else {
            // numeric offset PRECEDING | FOLLOWING
            let offset = match self.peek().clone() {
                Token::Integer(n) => {
                    self.advance();
                    n
                }
                _ => 0,
            };
            if self.match_keyword(Keyword::PRECEDING) {
                self.advance();
                Ok(WindowFrameBound {
                    direction: WindowFrameDirection::Preceding(offset),
                })
            } else {
                self.expect_keyword(Keyword::FOLLOWING)?;
                Ok(WindowFrameBound {
                    direction: WindowFrameDirection::Following(offset),
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

    fn parse_xml_element(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;

        let entity_escaping = if self.try_consume_keyword(Keyword::ENTITYESCAPING) {
            Some(true)
        } else if self.try_consume_keyword(Keyword::NOENTITYESCAPING) {
            Some(false)
        } else {
            None
        };

        let (evalname, name) = if self.try_consume_keyword(Keyword::EVALNAME) {
            let expr = self.parse_expr()?;
            (Some(Box::new(expr)), None)
        } else {
            let _ = self.try_consume_keyword(Keyword::NAME_P);
            (None, Some(self.parse_identifier()?))
        };

        let mut attributes: Option<XmlAttributes> = None;
        let mut content: Vec<XmlContent> = Vec::new();

        while self.match_token(&Token::Comma) {
            self.advance();

            if self.match_keyword(Keyword::XMLATTRIBUTES) && attributes.is_none() {
                self.advance();
                attributes = Some(self.parse_xml_attributes_inner()?);
                continue;
            }

            let expr = self.parse_expr()?;
            let alias = self.parse_optional_alias()?;
            content.push(XmlContent { expr, alias });
        }

        self.expect_token(&Token::RParen)?;

        Ok(Expr::XmlElement {
            entity_escaping,
            evalname,
            name,
            attributes,
            content,
        })
    }

    fn parse_xml_attributes_inner(&mut self) -> Result<XmlAttributes, ParserError> {
        self.expect_token(&Token::LParen)?;

        let entity_escaping = if self.try_consume_keyword(Keyword::ENTITYESCAPING) {
            Some(true)
        } else if self.try_consume_keyword(Keyword::NOENTITYESCAPING) {
            Some(false)
        } else {
            None
        };

        let mut items = Vec::new();
        loop {
            let expr = self.parse_expr()?;
            let name = self.parse_optional_alias()?;
            items.push(XmlAttribute { value: expr, name });
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }

        self.expect_token(&Token::RParen)?;
        Ok(XmlAttributes {
            entity_escaping,
            items,
        })
    }

    fn parse_xml_concat(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        let mut args = vec![self.parse_expr()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            args.push(self.parse_expr()?);
        }
        self.expect_token(&Token::RParen)?;
        Ok(Expr::XmlConcat(args))
    }

    fn parse_xml_forest(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        let mut items = Vec::new();
        loop {
            let expr = self.parse_expr()?;
            let alias = self.parse_optional_alias()?;
            items.push(XmlContent { expr, alias });
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }
        self.expect_token(&Token::RParen)?;
        Ok(Expr::XmlForest(items))
    }

    fn parse_xml_parse(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        let option = if self.match_keyword(Keyword::DOCUMENT_P) {
            self.advance();
            XmlOption::Document
        } else {
            self.expect_keyword(Keyword::CONTENT_P)?;
            XmlOption::Content
        };
        let expr = self.parse_expr()?;
        let wellformed = self.try_consume_keyword(Keyword::WELLFORMED);
        self.expect_token(&Token::RParen)?;
        Ok(Expr::XmlParse {
            option,
            expr: Box::new(expr),
            wellformed,
        })
    }

    fn parse_xml_pi(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        let _ = self.try_consume_keyword(Keyword::NAME_P);
        let name = self.parse_identifier()?;
        let content = if self.match_token(&Token::Comma) {
            self.advance();
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        self.expect_token(&Token::RParen)?;
        Ok(Expr::XmlPi {
            name: Some(name),
            content,
        })
    }

    fn parse_xml_root(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        let expr = self.parse_expr()?;
        self.expect_token(&Token::Comma)?;
        self.expect_keyword(Keyword::VERSION_P)?;
        let version = if self.match_keyword(Keyword::NO) {
            self.advance();
            if self.try_consume_keyword(Keyword::VALUE_P) {
                None
            } else {
                Some(Box::new(Expr::ColumnRef(vec!["no".to_string()])))
            }
        } else {
            Some(Box::new(self.parse_expr()?))
        };
        let standalone = if self.match_token(&Token::Comma) {
            self.advance();
            self.expect_keyword(Keyword::STANDALONE_P)?;
            if self.try_consume_keyword(Keyword::YES_P) {
                Some(Some(true))
            } else if self.match_keyword(Keyword::NO) {
                self.advance();
                if self.try_consume_keyword(Keyword::VALUE_P) {
                    Some(None)
                } else {
                    Some(Some(false))
                }
            } else {
                None
            }
        } else {
            None
        };
        self.expect_token(&Token::RParen)?;
        Ok(Expr::XmlRoot {
            expr: Box::new(expr),
            version,
            standalone,
        })
    }

    fn parse_xml_serialize(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        let option = if self.match_keyword(Keyword::DOCUMENT_P) {
            self.advance();
            XmlOption::Document
        } else {
            self.expect_keyword(Keyword::CONTENT_P)?;
            XmlOption::Content
        };
        let expr = self.parse_expr()?;
        self.expect_keyword(Keyword::AS)?;
        let type_name = self.parse_data_type()?;
        self.expect_token(&Token::RParen)?;
        Ok(Expr::XmlSerialize {
            option,
            expr: Box::new(expr),
            type_name,
        })
    }
}
