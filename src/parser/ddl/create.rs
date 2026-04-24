use crate::ast::*;
use crate::parser::ddl::format_data_type;
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;

impl Parser {
    pub(crate) fn parse_create_index(
        &mut self,
        unique: bool,
    ) -> Result<CreateIndexStatement, ParserError> {
        self.expect_keyword(Keyword::INDEX)?;

        let concurrent = self.try_consume_keyword(Keyword::CONCURRENTLY);
        let if_not_exists = self.parse_if_not_exists();

        let name = if !matches!(self.peek(), Token::Keyword(Keyword::ON)) {
            Some(self.parse_object_name()?)
        } else {
            None
        };

        self.expect_keyword(Keyword::ON)?;
        let table = self.parse_object_name()?;

        let using_method = if self.match_keyword(Keyword::USING) {
            self.advance();
            Some(self.parse_identifier()?)
        } else {
            None
        };

        self.expect_token(&Token::LParen)?;
        let mut columns = vec![self.parse_index_column()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            columns.push(self.parse_index_column()?);
        }
        self.expect_token(&Token::RParen)?;

        if self.match_keyword(Keyword::LOCAL) || self.match_keyword(Keyword::GLOBAL) {
            self.advance();
            if self.match_token(&Token::LParen) {
                self.advance();
                let mut depth = 1usize;
                while depth > 0 && !matches!(self.peek(), Token::Eof) {
                    match self.peek() {
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
            }
        }

        if self.match_keyword(Keyword::WITH) {
            self.advance();
            if self.match_token(&Token::LParen) {
                self.advance();
                while !self.match_token(&Token::RParen) && !matches!(self.peek(), Token::Eof) {
                    self.advance();
                }
                if self.match_token(&Token::RParen) {
                    self.advance();
                }
            }
        }

        let mut tablespace = None;
        let mut where_clause = None;

        loop {
            if self.match_keyword(Keyword::TABLESPACE) {
                self.advance();
                tablespace = Some(self.parse_identifier()?);
            } else if self.match_keyword(Keyword::WHERE) {
                self.advance();
                where_clause = Some(self.parse_expr()?);
            } else if matches!(
                self.peek_keyword(),
                Some(Keyword::PCTFREE) | Some(Keyword::INITRANS) | Some(Keyword::MAXTRANS)
            ) {
                self.advance();
                let _ = self.parse_expr();
            } else if self.match_keyword(Keyword::STORAGE) {
                self.advance();
                let _ = self.collect_until_balanced_paren();
            } else if self.match_ident_str("PCTUSED") {
                self.advance();
                let _ = self.parse_expr();
            } else {
                break;
            }
        }

        Ok(CreateIndexStatement {
            unique,
            if_not_exists,
            concurrent,
            name,
            table,
            using_method,
            columns,
            tablespace,
            where_clause,
        })
    }

    fn parse_index_column(&mut self) -> Result<IndexColumn, ParserError> {
        let (name, expr) = if self.matches_simple_column_name() {
            (Some(self.parse_identifier()?), None)
        } else {
            (None, Some(self.parse_expr()?))
        };

        let collation = if self.match_keyword(Keyword::COLLATE) {
            self.advance();
            let names = self.parse_object_name()?;
            Some(names.join("."))
        } else {
            None
        };

        let opclass = if !self.match_keyword(Keyword::ASC)
            && !self.match_keyword(Keyword::DESC)
            && !self.match_keyword(Keyword::NULLS_P)
            && !matches!(self.peek(), Token::Comma | Token::RParen)
        {
            let names = self.parse_object_name()?;
            Some(names.join("."))
        } else {
            None
        };

        let asc = if self.match_keyword(Keyword::ASC) {
            self.advance();
            Some(true)
        } else if self.match_keyword(Keyword::DESC) {
            self.advance();
            Some(false)
        } else {
            None
        };

        let nulls = if self.match_keyword(Keyword::NULLS_P) {
            self.advance();
            if self.match_keyword(Keyword::FIRST_P) {
                self.advance();
                Some(IndexNulls::First)
            } else {
                self.expect_keyword(Keyword::LAST_P)?;
                Some(IndexNulls::Last)
            }
        } else {
            None
        };

        Ok(IndexColumn {
            name,
            expr,
            collation,
            opclass,
            asc,
            nulls,
        })
    }

    fn matches_simple_column_name(&self) -> bool {
        match self.peek() {
            Token::Ident(_) | Token::QuotedIdent(_) => {
                if self.tokens.len() > self.pos + 1 {
                    let next = &self.tokens[self.pos + 1].token;
                    !matches!(next, Token::LParen | Token::Dot)
                } else {
                    true
                }
            }
            _ => false,
        }
    }

    // ========== CREATE GLOBAL INDEX ==========

    pub(crate) fn parse_create_global_index(
        &mut self,
    ) -> Result<CreateGlobalIndexStatement, ParserError> {
        let unique = self.try_consume_keyword(Keyword::UNIQUE);
        self.expect_keyword(Keyword::INDEX)?;

        let concurrent = self.try_consume_keyword(Keyword::CONCURRENTLY);
        let if_not_exists = self.parse_if_not_exists();

        let name = if !matches!(self.peek(), Token::Keyword(Keyword::ON)) {
            Some(self.parse_object_name()?)
        } else {
            None
        };

        self.expect_keyword(Keyword::ON)?;
        let table = self.parse_object_name()?;

        let using_method = if self.match_keyword(Keyword::USING) {
            self.advance();
            Some(self.parse_identifier()?)
        } else {
            None
        };

        self.expect_token(&Token::LParen)?;
        let mut columns = vec![self.parse_global_index_column()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            columns.push(self.parse_global_index_column()?);
        }
        self.expect_token(&Token::RParen)?;

        let mut containing = Vec::new();
        if self.match_ident_str("CONTAINING") {
            self.advance();
            self.expect_token(&Token::LParen)?;
            containing.push(self.parse_identifier()?);
            while self.match_token(&Token::Comma) {
                self.advance();
                containing.push(self.parse_identifier()?);
            }
            self.expect_token(&Token::RParen)?;
        }

        let mut distribute_by = None;
        if self.match_keyword(Keyword::DISTRIBUTE) {
            self.advance();
            self.expect_keyword(Keyword::BY)?;
            if !self.try_consume_ident_str("HASH") {
                return Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "HASH".into(),
                    got: format!("{:?}", self.peek()),
                });
            }
            self.expect_token(&Token::LParen)?;
            let mut cols = vec![self.parse_identifier()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                cols.push(self.parse_identifier()?);
            }
            self.expect_token(&Token::RParen)?;
            distribute_by = Some(DistributeClause::Hash { columns: cols });
        }

        let mut with_options = Vec::new();
        if self.match_keyword(Keyword::WITH) {
            self.advance();
            self.expect_token(&Token::LParen)?;
            loop {
                let key = self.parse_identifier()?;
                self.expect_token(&Token::Eq)?;
                let value = match self.peek().clone() {
                    Token::Integer(n) => {
                        self.advance();
                        n.to_string()
                    }
                    Token::StringLiteral(s) => {
                        self.advance();
                        format!("'{}'", s)
                    }
                    _ => self.parse_identifier()?,
                };
                with_options.push((key, value));
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect_token(&Token::RParen)?;
        }

        let mut tablespace = None;
        let mut visible = None;
        let mut where_clause = None;

        loop {
            if self.match_keyword(Keyword::TABLESPACE) {
                self.advance();
                tablespace = Some(self.parse_identifier()?);
            } else if self.match_keyword(Keyword::VISIBLE) {
                self.advance();
                visible = Some(true);
            } else if self.match_keyword(Keyword::INVISIBLE) {
                self.advance();
                visible = Some(false);
            } else if self.match_keyword(Keyword::WHERE) {
                self.advance();
                where_clause = Some(self.parse_expr()?);
            } else {
                break;
            }
        }

        Ok(CreateGlobalIndexStatement {
            unique,
            concurrent,
            if_not_exists,
            name,
            table,
            using_method,
            columns,
            containing,
            distribute_by,
            with_options,
            tablespace,
            visible,
            where_clause,
        })
    }

    fn parse_global_index_column(&mut self) -> Result<GlobalIndexColumn, ParserError> {
        let col_name = self.parse_identifier()?;

        if matches!(self.peek(), Token::LParen) {
            let next_pos = self.pos + 1;
            if let Some(Token::Integer(_)) = self.tokens.get(next_pos).map(|t| &t.token) {
                self.advance();
                match self.peek().clone() {
                    Token::Integer(n) => {
                        self.advance();
                        self.expect_token(&Token::RParen)?;
                        return Ok(GlobalIndexColumn {
                            name: col_name,
                            length: Some(n as u32),
                            collation: None,
                            opclass: None,
                            ordering: None,
                            nulls: None,
                            expression: None,
                        });
                    }
                    _ => {}
                }
            }
        }

        if matches!(self.peek(), Token::LParen) {
            self.advance();
            let mut args = Vec::new();
            if !matches!(self.peek(), Token::RParen) {
                args.push(self.parse_expr()?);
                while self.match_token(&Token::Comma) {
                    self.advance();
                    args.push(self.parse_expr()?);
                }
            }
            self.expect_token(&Token::RParen)?;
            return Ok(GlobalIndexColumn {
                name: String::new(),
                length: None,
                collation: None,
                opclass: None,
                ordering: None,
                nulls: None,
                expression: Some(Expr::FunctionCall {
                    name: vec![col_name],
                    args,
                    distinct: false,
                    over: None,
                    filter: None,
                    within_group: Vec::new(),
                    separator: None,
                    default: None,
                    conversion_format: None,
                    builtin: None,
                }),
            });
        }

        let collation = if self.match_keyword(Keyword::COLLATE) {
            self.advance();
            Some(self.parse_identifier()?)
        } else {
            None
        };

        let opclass = if !self.match_keyword(Keyword::ASC)
            && !self.match_keyword(Keyword::DESC)
            && !self.match_keyword(Keyword::NULLS_P)
            && !matches!(self.peek(), Token::Comma | Token::RParen)
        {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        let ordering = if self.match_keyword(Keyword::ASC) {
            self.advance();
            Some(IndexOrdering::Asc)
        } else if self.match_keyword(Keyword::DESC) {
            self.advance();
            Some(IndexOrdering::Desc)
        } else {
            None
        };

        let nulls = if self.match_keyword(Keyword::NULLS_P) {
            self.advance();
            if self.match_keyword(Keyword::FIRST_P) {
                self.advance();
                Some(IndexNulls::First)
            } else {
                self.expect_keyword(Keyword::LAST_P)?;
                Some(IndexNulls::Last)
            }
        } else {
            None
        };

        Ok(GlobalIndexColumn {
            name: col_name,
            length: None,
            collation,
            opclass,
            ordering,
            nulls,
            expression: None,
        })
    }

    // ========== CREATE SEQUENCE ==========

    pub(crate) fn parse_create_sequence(&mut self) -> Result<CreateSequenceStatement, ParserError> {
        self.expect_keyword(Keyword::SEQUENCE)?;

        let if_not_exists = self.parse_if_not_exists();
        let name = self.parse_object_name()?;

        let mut start = None;
        let mut increment = None;
        let mut min_value = None;
        let mut max_value = None;
        let mut cache = None;
        let mut cycle = false;
        let mut owned_by = None;

        loop {
            match self.peek_keyword() {
                Some(Keyword::START) => {
                    self.advance();
                    self.try_consume_keyword(Keyword::WITH);
                    start = Some(self.parse_expr()?);
                }
                Some(Keyword::INCREMENT) => {
                    self.advance();
                    self.try_consume_keyword(Keyword::BY);
                    increment = Some(self.parse_expr()?);
                }
                Some(Keyword::MINVALUE) => {
                    self.advance();
                    min_value = Some(self.parse_expr()?);
                }
                Some(Keyword::MAXVALUE) => {
                    self.advance();
                    max_value = Some(self.parse_expr()?);
                }
                Some(Keyword::CACHE) => {
                    self.advance();
                    cache = Some(self.parse_expr()?);
                }
                Some(Keyword::CYCLE) => {
                    self.advance();
                    cycle = true;
                }
                Some(Keyword::NO) => {
                    self.advance();
                    if self.match_keyword(Keyword::CYCLE) {
                        self.advance();
                        cycle = false;
                    } else if self.match_keyword(Keyword::MINVALUE) {
                        self.advance();
                        min_value = None;
                    } else if self.match_keyword(Keyword::MAXVALUE) {
                        self.advance();
                        max_value = None;
                    }
                }
                Some(Keyword::OWNED) => {
                    self.advance();
                    self.expect_keyword(Keyword::BY)?;
                    owned_by = Some(self.parse_object_name()?);
                }
                _ if self.match_ident_str("NOORDER") || self.match_ident_str("ORDER") => {
                    self.advance();
                }
                _ => break,
            }
        }

        Ok(CreateSequenceStatement {
            if_not_exists,
            name,
            start,
            increment,
            min_value,
            max_value,
            cache,
            cycle,
            owned_by,
        })
    }

    // ========== TRUNCATE ==========

    pub(crate) fn parse_truncate(&mut self) -> Result<TruncateStatement, ParserError> {
        if self.match_keyword(Keyword::TABLE) {
            self.advance();
        }

        let mut tables = vec![self.parse_object_name()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            tables.push(self.parse_object_name()?);
        }

        let cascade = self.try_consume_keyword(Keyword::CASCADE);
        let restart_identity = if self.try_consume_keyword(Keyword::RESTART) {
            self.try_consume_keyword(Keyword::IDENTITY_P);
            true
        } else {
            false
        };
        let continue_identity =
            if !restart_identity && self.try_consume_keyword(Keyword::CONTINUE_P) {
                self.try_consume_keyword(Keyword::IDENTITY_P);
                true
            } else {
                false
            };

        Ok(TruncateStatement {
            tables,
            cascade,
            restart_identity,
            continue_identity,
        })
    }

    // ========== CREATE VIEW ==========

    pub(crate) fn parse_create_view(&mut self) -> Result<CreateViewStatement, ParserError> {
        self.expect_keyword(Keyword::VIEW)?;

        let name = self.parse_object_name()?;

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
        let query = Box::new(self.parse_select_statement()?);

        let check_option = if self.match_keyword(Keyword::WITH) {
            self.advance();
            if self.match_keyword(Keyword::LOCAL) {
                self.advance();
                self.expect_keyword(Keyword::CHECK)?;
                self.expect_keyword(Keyword::OPTION)?;
                Some(CheckOption::Local)
            } else if self.match_keyword(Keyword::CASCADED) {
                self.advance();
                self.expect_keyword(Keyword::CHECK)?;
                self.expect_keyword(Keyword::OPTION)?;
                Some(CheckOption::Cascaded)
            } else {
                self.expect_keyword(Keyword::CHECK)?;
                self.expect_keyword(Keyword::OPTION)?;
                Some(CheckOption::Cascaded)
            }
        } else {
            None
        };

        let security = if self.match_keyword(Keyword::SECURITY) {
            self.advance();
            if self.match_keyword(Keyword::BARRIER) {
                self.advance();
                Some(ViewSecurity::Barrier)
            } else if self.match_keyword(Keyword::INVOKER) {
                self.advance();
                Some(ViewSecurity::Invoker)
            } else {
                None
            }
        } else {
            None
        };

        Ok(CreateViewStatement {
            replace: false,
            temporary: false,
            recursive: false,
            name,
            columns,
            query,
            check_option,
            security,
        })
    }

    // ========== CREATE SCHEMA ==========

    pub(crate) fn parse_create_schema(&mut self) -> Result<CreateSchemaStatement, ParserError> {
        self.expect_keyword(Keyword::SCHEMA)?;

        let if_not_exists = self.parse_if_not_exists();

        let (name, authorization) = if self.match_keyword(Keyword::AUTHORIZATION) {
            self.advance();
            let auth = Some(self.parse_identifier()?);
            (None, auth)
        } else {
            let schema_name = Some(self.parse_identifier()?);
            let auth = if self.match_keyword(Keyword::AUTHORIZATION) {
                self.advance();
                Some(self.parse_identifier()?)
            } else {
                None
            };
            (schema_name, auth)
        };

        let character_set = if self.match_keyword(Keyword::CHARACTER) {
            self.advance();
            self.expect_keyword(Keyword::SET)?;
            Some(self.parse_identifier()?)
        } else {
            None
        };
        let collate = if self.match_keyword(Keyword::COLLATE) {
            self.advance();
            Some(self.parse_identifier()?)
        } else {
            None
        };

        let mut elements = Vec::new();
        loop {
            if self.match_keyword(Keyword::CREATE) {
                self.advance();
                let element = match self.peek_keyword() {
                    Some(Keyword::TABLE) => {
                        SchemaElement::Table(self.parse_create_table(false, false)?)
                    }
                    Some(Keyword::INDEX) => SchemaElement::Index(self.parse_create_index(false)?),
                    Some(Keyword::VIEW) => SchemaElement::View(self.parse_create_view()?),
                    Some(Keyword::SEQUENCE) => {
                        SchemaElement::Sequence(self.parse_create_sequence()?)
                    }
                    _ => break,
                };
                elements.push(element);
            } else {
                break;
            }
        }

        Ok(CreateSchemaStatement {
            if_not_exists,
            name,
            authorization,
            character_set,
            collate,
            elements,
        })
    }

    // ========== CREATE DATABASE ==========

    pub(crate) fn parse_create_database(&mut self) -> Result<CreateDatabaseStatement, ParserError> {
        self.expect_keyword(Keyword::DATABASE)?;

        let name = self.parse_identifier()?;

        let mut owner = None;
        let mut template = None;
        let mut encoding = None;
        let mut locale = None;
        let mut lc_collate = None;
        let mut lc_ctype = None;
        let mut tablespace = None;
        let mut allow_connections = None;
        let mut connection_limit = None;
        let mut is_template = None;

        if self.match_keyword(Keyword::WITH) {
            self.advance();
        }

        loop {
            let opt_name = match self.peek() {
                Token::Ident(s) => s.to_lowercase(),
                Token::Keyword(kw) => kw.as_str().to_string(),
                _ => break,
            };
            let saved_pos = self.pos;
            self.advance();

            if self.match_token(&Token::Eq) {
                self.advance();
            }

            match opt_name.as_str() {
                "owner" => owner = Some(self.parse_identifier()?),
                "template" => template = Some(self.parse_string_or_ident()?),
                "encoding" => encoding = Some(self.parse_string_or_ident()?),
                "locale" => locale = Some(self.parse_string_or_ident()?),
                "lc_collate" => lc_collate = Some(self.parse_string_or_ident()?),
                "lc_ctype" => lc_ctype = Some(self.parse_string_or_ident()?),
                "tablespace" => tablespace = Some(self.parse_identifier()?),
                "allow_connections" => allow_connections = Some(self.parse_bool_literal()?),
                "connection_limit" => {
                    if let Token::Integer(n) = self.peek().clone() {
                        connection_limit = Some(n as i32);
                        self.advance();
                    } else if self.match_keyword(Keyword::MINVALUE) {
                        self.advance();
                        connection_limit = Some(-1);
                    } else {
                        return Err(ParserError::UnexpectedToken {
                            location: self.current_location(),
                            expected: "integer".to_string(),
                            got: format!("{:?}", self.peek()),
                        });
                    }
                }
                "is_template" => is_template = Some(self.parse_bool_literal()?),
                "dbcompatibility" | "compatibility" => {
                    let _ = self.parse_string_or_ident();
                }
                "dbtimezone" => {
                    let _ = self.parse_string_or_ident();
                }
                _ => {
                    self.pos = saved_pos;
                    break;
                }
            }

            if self.match_token(&Token::Comma) {
                self.advance();
            }
        }

        Ok(CreateDatabaseStatement {
            name,
            owner,
            template,
            encoding,
            locale,
            lc_collate,
            lc_ctype,
            tablespace,
            allow_connections,
            connection_limit,
            is_template,
        })
    }

    fn parse_string_or_ident(&mut self) -> Result<String, ParserError> {
        match self.peek().clone() {
            Token::StringLiteral(s) => {
                self.advance();
                Ok(s)
            }
            Token::Ident(s) => {
                self.advance();
                Ok(s)
            }
            Token::Keyword(kw) => {
                self.advance();
                Ok(kw.as_str().to_string())
            }
            _ => Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "string or identifier".to_string(),
                got: format!("{:?}", self.peek()),
            }),
        }
    }

    fn parse_bool_literal(&mut self) -> Result<bool, ParserError> {
        match self.peek() {
            Token::Keyword(Keyword::TRUE_P) => {
                self.advance();
                Ok(true)
            }
            Token::Keyword(Keyword::FALSE_P) => {
                self.advance();
                Ok(false)
            }
            _ => Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "TRUE or FALSE".to_string(),
                got: format!("{:?}", self.peek()),
            }),
        }
    }

    pub(crate) fn parse_create_database_link(
        &mut self,
        public_link: bool,
    ) -> Result<CreateDatabaseLinkStatement, ParserError> {
        self.expect_keyword(Keyword::DATABASE)?;
        if !self.match_ident_str("LINK") {
            return Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "LINK".to_string(),
                got: format!("{:?}", self.peek()),
            });
        }
        self.advance();
        let name = self.parse_identifier()?;
        let mut user = None;
        let mut password = None;
        let mut using_clause = None;
        if self.match_keyword(Keyword::CONNECT) {
            self.advance();
            self.expect_keyword(Keyword::TO)?;
            user = Some(self.parse_string_or_ident()?);
        }
        if self.match_keyword(Keyword::IDENTIFIED) {
            self.advance();
            self.expect_keyword(Keyword::BY)?;
            password = Some(self.parse_string_literal()?);
        }
        if self.match_keyword(Keyword::USING) {
            self.advance();
            if self.match_token(&Token::LParen) {
                using_clause = Some(self.skip_to_paren_end());
            } else {
                using_clause = Some(self.parse_string_literal()?);
            }
        }
        Ok(CreateDatabaseLinkStatement {
            name,
            public_link,
            user,
            password,
            using_clause,
        })
    }

    // ========== CREATE TABLESPACE ==========

    pub(crate) fn parse_create_tablespace(
        &mut self,
    ) -> Result<CreateTablespaceStatement, ParserError> {
        self.expect_keyword(Keyword::TABLESPACE)?;

        let name = self.parse_identifier()?;

        let owner = if self.match_keyword(Keyword::OWNER) {
            self.advance();
            Some(self.parse_identifier()?)
        } else {
            None
        };

        let relative = self.try_consume_keyword(Keyword::RELATIVE_P);

        self.expect_keyword(Keyword::LOCATION)?;
        let location = match self.peek().clone() {
            Token::StringLiteral(s) => {
                self.advance();
                s
            }
            Token::DollarString { body: s, .. } => {
                self.advance();
                s
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "string literal for location".to_string(),
                    got: format!("{:?}", self.peek()),
                });
            }
        };

        let maxsize = if self.match_keyword(Keyword::MAXSIZE) {
            self.advance();
            Some(self.parse_string_literal().unwrap_or_default())
        } else {
            None
        };

        Ok(CreateTablespaceStatement {
            name,
            owner,
            relative,
            location,
            maxsize,
        })
    }

    pub(crate) fn parse_create_type(&mut self) -> Result<CreateTypeStatement, ParserError> {
        self.expect_keyword(Keyword::TYPE_P)?;
        let name = self.parse_object_name()?;

        let type_kind =
            if self.try_consume_keyword(Keyword::AS) || self.try_consume_keyword(Keyword::IS) {
                if self.match_keyword(Keyword::ENUM_P) {
                    self.advance();
                    self.expect_token(&Token::LParen)?;
                    let mut labels = Vec::new();
                    loop {
                        match self.peek().clone() {
                            Token::StringLiteral(s) => {
                                labels.push(s);
                                self.advance();
                            }
                            _ => {
                                return Err(ParserError::UnexpectedToken {
                                    location: self.current_location(),
                                    expected: "string literal".to_string(),
                                    got: format!("{:?}", self.peek()),
                                });
                            }
                        }
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                        self.advance();
                    }
                    self.expect_token(&Token::RParen)?;
                    TypeKind::Enum { labels }
                } else if self.match_token(&Token::LParen) {
                    self.advance();
                    let mut attributes = Vec::new();
                    loop {
                        let attr_name = self.parse_identifier()?;
                        let data_type = self.parse_data_type()?;
                        attributes.push(TypeAttribute {
                            name: attr_name,
                            data_type,
                        });
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                        self.advance();
                    }
                    self.expect_token(&Token::RParen)?;
                    TypeKind::Composite { attributes }
                } else if self.match_keyword(Keyword::TABLE) {
                    self.advance();
                    self.expect_keyword(Keyword::OF)?;
                    let dt = self.parse_data_type()?;
                    TypeKind::Table {
                        element_type: format_data_type(&dt),
                    }
                } else if self.match_keyword(Keyword::RANGE) {
                    self.advance();
                    let options = self.parse_generic_options_no_with();
                    TypeKind::Range { options }
                } else {
                    return Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "ENUM, TABLE OF, RANGE, or (".to_string(),
                        got: format!("{:?}", self.peek()),
                    });
                }
            } else if self.match_token(&Token::LParen) {
                let options = self.parse_generic_options();
                TypeKind::Base { options }
            } else {
                TypeKind::Shell
            };

        Ok(CreateTypeStatement { name, type_kind })
    }

    pub(crate) fn parse_create_role_options(
        &mut self,
    ) -> Result<(String, Vec<RoleOption>), ParserError> {
        let name = self.parse_identifier()?;

        let mut options = Vec::new();

        if self.try_consume_keyword(Keyword::WITH) {
            loop {
                if self.try_consume_ident_str("SUPERUSER") {
                    options.push(RoleOption::Superuser(true));
                } else if self.try_consume_ident_str("NOSUPERUSER") {
                    options.push(RoleOption::Superuser(false));
                } else if self.try_consume_ident_str("SYSADMIN") {
                    options.push(RoleOption::Superuser(true));
                } else if self.try_consume_ident_str("NOSYSADMIN") {
                    options.push(RoleOption::Superuser(false));
                } else if self.try_consume_ident_str("CREATEDB") {
                    options.push(RoleOption::CreateDb(true));
                } else if self.try_consume_ident_str("NOCREATEDB") {
                    options.push(RoleOption::CreateDb(false));
                } else if self.try_consume_ident_str("CREATEROLE") {
                    options.push(RoleOption::CreateRole(true));
                } else if self.try_consume_ident_str("NOCREATEROLE") {
                    options.push(RoleOption::CreateRole(false));
                } else if self.try_consume_keyword(Keyword::INHERIT) {
                    options.push(RoleOption::Inherit(true));
                } else if self.try_consume_ident_str("NOINHERIT") {
                    options.push(RoleOption::Inherit(false));
                } else if self.try_consume_ident_str("LOGIN") {
                    options.push(RoleOption::Login(true));
                } else if self.try_consume_ident_str("NOLOGIN") {
                    options.push(RoleOption::Login(false));
                } else if self.try_consume_ident_str("REPLICATION") {
                    options.push(RoleOption::Replication(true));
                } else if self.try_consume_ident_str("NOREPLICATION") {
                    options.push(RoleOption::Replication(false));
                } else if self.try_consume_ident_str("BYPASSRLS") {
                    options.push(RoleOption::BypassRls(true));
                } else if self.try_consume_ident_str("NOBYPASSRLS") {
                    options.push(RoleOption::BypassRls(false));
                } else if self.match_keyword(Keyword::CONNECTION) {
                    self.advance();
                    self.expect_keyword(Keyword::LIMIT)?;
                    let limit = self.parse_integer_literal()?;
                    options.push(RoleOption::ConnectionLimit(limit));
                } else if self.match_keyword(Keyword::ENCRYPTED) {
                    self.advance();
                    if self.match_keyword(Keyword::PASSWORD) || self.match_ident_str("PASSWORD") {
                        self.advance();
                        let pwd = self.parse_string_or_quoted_ident()?;
                        options.push(RoleOption::EncryptedPassword(pwd));
                    }
                } else if self.match_keyword(Keyword::UNENCRYPTED) {
                    self.advance();
                    if self.match_keyword(Keyword::PASSWORD) || self.match_ident_str("PASSWORD") {
                        self.advance();
                        let pwd = self.parse_string_or_quoted_ident()?;
                        options.push(RoleOption::UnencryptedPassword(pwd));
                    }
                } else if self.match_keyword(Keyword::PASSWORD) || self.match_ident_str("PASSWORD")
                {
                    self.advance();
                    let pwd = self.parse_string_or_quoted_ident()?;
                    options.push(RoleOption::UnencryptedPassword(pwd));
                } else if self.match_keyword(Keyword::IDENTIFIED) {
                    self.advance();
                    self.expect_keyword(Keyword::BY)?;
                    let pwd = self.parse_string_or_quoted_ident()?;
                    options.push(RoleOption::UnencryptedPassword(pwd));
                } else if self.match_keyword(Keyword::VALID) {
                    self.advance();
                    if self.match_keyword(Keyword::UNTIL) {
                        self.advance();
                        let until = self.parse_string_literal()?;
                        options.push(RoleOption::ValidUntil(until));
                    }
                } else if self.match_keyword(Keyword::IN_P) {
                    self.advance();
                    if self.match_keyword(Keyword::ROLE) || self.match_ident_str("ROLE") {
                        self.advance();
                        let mut roles = vec![self.parse_identifier()?];
                        while self.match_token(&Token::Comma) {
                            self.advance();
                            roles.push(self.parse_identifier()?);
                        }
                        options.push(RoleOption::InRole(roles));
                    }
                } else if self.match_ident_str("ROLE") {
                    self.advance();
                    let mut roles = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        roles.push(self.parse_identifier()?);
                    }
                    options.push(RoleOption::Role(roles));
                } else if self.match_keyword(Keyword::ADMIN) || self.match_ident_str("ADMIN") {
                    self.advance();
                    let mut roles = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        roles.push(self.parse_identifier()?);
                    }
                    options.push(RoleOption::Admin(roles));
                } else if self.match_keyword(Keyword::USER) || self.match_ident_str("USER") {
                    self.advance();
                    let mut users = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        users.push(self.parse_identifier()?);
                    }
                    options.push(RoleOption::User(users));
                } else if self.match_keyword(Keyword::SYSID) || self.match_ident_str("SYSID") {
                    self.advance();
                    let id = self.parse_integer_literal()?;
                    options.push(RoleOption::Sysid(id));
                } else if self.match_keyword(Keyword::DEFAULT) {
                    self.advance();
                    if self.match_keyword(Keyword::TABLESPACE) {
                        self.advance();
                        let ts = self.parse_identifier()?;
                        options.push(RoleOption::DefaultTablespace(ts));
                    } else {
                        break;
                    }
                } else if self.match_keyword(Keyword::TABLESPACE) {
                    self.advance();
                    let ts = self.parse_identifier()?;
                    options.push(RoleOption::Tablespace(ts));
                } else if self.match_ident_str("PROFILE") {
                    self.advance();
                    let p = self.parse_identifier()?;
                    options.push(RoleOption::Profile(p));
                } else if self.match_keyword(Keyword::ACCOUNT) || self.match_ident_str("ACCOUNT") {
                    self.advance();
                    let lock = if self.try_consume_keyword(Keyword::LOCK_P)
                        || self.try_consume_ident_str("LOCK")
                    {
                        true
                    } else {
                        self.try_consume_keyword(Keyword::UNLOCK)
                            || self.try_consume_ident_str("UNLOCK");
                        false
                    };
                    options.push(RoleOption::AccountLock(lock));
                } else if self.try_consume_ident_str("AUDITADMIN") {
                    options.push(RoleOption::AuditAdmin(true));
                } else if self.try_consume_ident_str("NOAUDITADMIN") {
                    options.push(RoleOption::AuditAdmin(false));
                } else if self.try_consume_ident_str("MONADMIN") {
                    options.push(RoleOption::MonAdmin(true));
                } else if self.try_consume_ident_str("NOMONADMIN") {
                    options.push(RoleOption::MonAdmin(false));
                } else if self.try_consume_ident_str("OPRADMIN") {
                    options.push(RoleOption::OprAdmin(true));
                } else if self.try_consume_ident_str("NOOPRADMIN") {
                    options.push(RoleOption::OprAdmin(false));
                } else if self.try_consume_ident_str("POLADMIN") {
                    options.push(RoleOption::PolAdmin(true));
                } else if self.try_consume_ident_str("NOPOLADMIN") {
                    options.push(RoleOption::PolAdmin(false));
                } else if self.try_consume_ident_str("PERSISTENCE") {
                    options.push(RoleOption::Persistence(true));
                } else if self.try_consume_ident_str("INDEPENDENT") {
                    options.push(RoleOption::Independent(true));
                } else if self.try_consume_ident_str("USEFT") {
                    options.push(RoleOption::Useft(true));
                } else if self.try_consume_ident_str("VCADMIN") {
                    options.push(RoleOption::VcAdmin(true));
                } else if self.try_consume_ident_str("PERMIT") {
                    options.push(RoleOption::Permit(true));
                } else if self.try_consume_ident_str("NOPERMIT") {
                    options.push(RoleOption::Permit(false));
                } else {
                    break;
                }
            }
        }

        // Handle PASSWORD / IDENTIFIED BY without WITH keyword
        if self.match_keyword(Keyword::PASSWORD) || self.match_ident_str("PASSWORD") {
            self.advance();
            let pwd = self.parse_string_or_quoted_ident()?;
            options.push(RoleOption::UnencryptedPassword(pwd));
        } else if self.match_keyword(Keyword::IDENTIFIED) {
            self.advance();
            self.expect_keyword(Keyword::BY)?;
            let pwd = self.parse_string_or_quoted_ident()?;
            options.push(RoleOption::UnencryptedPassword(pwd));
        }

        // Handle standalone boolean attributes without WITH (CREATEDB, etc.)
        loop {
            if self.try_consume_ident_str("CREATEDB") {
                options.push(RoleOption::CreateDb(true));
            } else if self.try_consume_ident_str("NOCREATEDB") {
                options.push(RoleOption::CreateDb(false));
            } else if self.try_consume_ident_str("CREATEROLE") {
                options.push(RoleOption::CreateRole(true));
            } else if self.try_consume_ident_str("NOCREATEROLE") {
                options.push(RoleOption::CreateRole(false));
            } else {
                break;
            }
        }

        if self.match_keyword(Keyword::PASSWORD) || self.match_ident_str("PASSWORD") {
            self.advance();
            let pwd = self.parse_string_or_quoted_ident()?;
            options.push(RoleOption::UnencryptedPassword(pwd));
        } else if self.match_keyword(Keyword::IDENTIFIED) {
            self.advance();
            self.expect_keyword(Keyword::BY)?;
            let pwd = self.parse_string_or_quoted_ident()?;
            options.push(RoleOption::UnencryptedPassword(pwd));
        }

        // Handle trailing options (DEFAULT TABLESPACE, TABLESPACE, PROFILE, ACCOUNT)
        if self.match_keyword(Keyword::DEFAULT) {
            self.advance();
            self.expect_keyword(Keyword::TABLESPACE)?;
            let ts = self.parse_identifier()?;
            options.push(RoleOption::DefaultTablespace(ts));
        }
        if self.match_keyword(Keyword::TABLESPACE) {
            self.advance();
            let ts = self.parse_identifier()?;
            options.push(RoleOption::Tablespace(ts));
        }
        if self.match_ident_str("PROFILE") {
            self.advance();
            let p = self.parse_identifier()?;
            options.push(RoleOption::Profile(p));
        }
        if self.match_keyword(Keyword::ACCOUNT) || self.match_ident_str("ACCOUNT") {
            self.advance();
            let lock = if self.try_consume_keyword(Keyword::LOCK_P)
                || self.try_consume_ident_str("LOCK")
            {
                true
            } else {
                self.try_consume_keyword(Keyword::UNLOCK) || self.try_consume_ident_str("UNLOCK");
                false
            };
            options.push(RoleOption::AccountLock(lock));
        }

        Ok((name, options))
    }

    pub(crate) fn parse_alter_index(&mut self) -> Result<AlterIndexStatement, ParserError> {
        self.expect_keyword(Keyword::INDEX)?;
        let if_exists = self.parse_if_exists();
        let name = self.parse_object_name()?;

        let action = if self.match_keyword(Keyword::RENAME) {
            self.advance();
            if self.match_keyword(Keyword::PARTITION) {
                self.advance();
                let old_name = self.parse_identifier()?;
                self.expect_keyword(Keyword::TO)?;
                let new_name = self.parse_identifier()?;
                AlterIndexAction::RenamePartition { old_name, new_name }
            } else {
                self.expect_keyword(Keyword::TO)?;
                let new_name = self.parse_identifier()?;
                AlterIndexAction::RenameTo(new_name)
            }
        } else if self.match_keyword(Keyword::SET) {
            self.advance();
            if self.match_keyword(Keyword::TABLESPACE) {
                self.advance();
                let ts = self.parse_identifier()?;
                AlterIndexAction::SetTablespace(ts)
            } else {
                let options = self.parse_generic_options_no_with();
                AlterIndexAction::Set(options)
            }
        } else if self.match_keyword(Keyword::RESET) {
            self.advance();
            self.expect_token(&Token::LParen)?;
            let mut names = vec![self.parse_identifier()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                names.push(self.parse_identifier()?);
            }
            self.expect_token(&Token::RParen)?;
            AlterIndexAction::Reset(names)
        } else if self.match_keyword(Keyword::UNUSABLE) {
            self.advance();
            AlterIndexAction::Unusable
        } else if self.match_ident_str("REBUILD") {
            self.advance();
            if self.match_keyword(Keyword::PARTITION) {
                self.advance();
                let partition_name = self.parse_identifier()?;
                AlterIndexAction::RebuildPartition { partition_name }
            } else {
                AlterIndexAction::Rebuild
            }
        } else if self.match_keyword(Keyword::MOVE) {
            self.advance();
            self.expect_keyword(Keyword::PARTITION)?;
            let partition_name = self.parse_identifier()?;
            let tablespace = if self.match_keyword(Keyword::TABLESPACE) {
                self.advance();
                Some(self.parse_identifier()?)
            } else {
                None
            };
            AlterIndexAction::MovePartition {
                partition_name,
                tablespace,
            }
        } else if matches!(
            self.peek_keyword(),
            Some(Keyword::PCTFREE) | Some(Keyword::INITRANS) | Some(Keyword::MAXTRANS)
        ) || self.match_ident_str("PCTUSED")
            || self.match_keyword(Keyword::STORAGE)
        {
            while matches!(
                self.peek_keyword(),
                Some(Keyword::PCTFREE) | Some(Keyword::INITRANS) | Some(Keyword::MAXTRANS)
            ) || self.match_ident_str("PCTUSED")
            {
                self.advance();
                let _ = self.parse_expr();
            }
            if self.match_keyword(Keyword::STORAGE) {
                self.advance();
                let _ = self.collect_until_balanced_paren();
            }
            AlterIndexAction::NoOp
        } else if self.match_ident_str("NOPARALLEL") || self.match_ident_str("PARALLEL") {
            self.advance();
            if self.match_keyword(Keyword::LOGGING) || self.match_keyword(Keyword::NOLOGGING) {
                self.advance();
            }
            AlterIndexAction::NoOp
        } else {
            AlterIndexAction::NoOp
        };

        Ok(AlterIndexStatement {
            if_exists,
            name,
            action,
        })
    }

    pub(crate) fn parse_alter_type(&mut self) -> Result<AlterCompositeTypeStatement, ParserError> {
        self.expect_keyword(Keyword::TYPE_P)?;
        let name = self.parse_object_name()?;

        let action = if self.match_ident_str("ADD") {
            self.advance();
            if self.match_ident_str("ATTRIBUTE") {
                self.advance();
                let attr_name = self.parse_identifier()?;
                let dt = self.parse_data_type()?;
                let type_str = format_data_type(&dt);
                let cascade = self.try_consume_keyword(Keyword::CASCADE);
                AlterTypeAction::AddAttribute {
                    name: attr_name,
                    data_type: type_str,
                    cascade,
                }
            } else if self.match_ident_str("VALUE") {
                self.advance();
                let if_not_exists = self.match_keyword(Keyword::IF_P) && {
                    self.advance();
                    self.expect_keyword(Keyword::NOT).unwrap_or(());
                    self.expect_keyword(Keyword::EXISTS).unwrap_or(());
                    true
                };
                let value = self.parse_string_literal()?;
                let mut before = None;
                let mut after = None;
                if self.match_keyword(Keyword::BEFORE) || self.match_ident_str("BEFORE") {
                    self.advance();
                    before = Some(self.parse_string_literal()?);
                } else if self.match_keyword(Keyword::AFTER) || self.match_ident_str("AFTER") {
                    self.advance();
                    after = Some(self.parse_string_literal()?);
                }
                AlterTypeAction::AddEnumValue {
                    if_not_exists,
                    value,
                    before,
                    after,
                }
            } else {
                return Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "ATTRIBUTE or VALUE".to_string(),
                    got: format!("{:?}", self.peek()),
                });
            }
        } else if self.match_keyword(Keyword::DROP) || self.match_ident_str("DROP") {
            self.advance();
            if self.match_keyword(Keyword::ATTRIBUTE) || self.match_ident_str("ATTRIBUTE") {
                self.advance();
                let if_exists = self.parse_if_exists();
                let attr_name = self.parse_identifier()?;
                let cascade = self.try_consume_keyword(Keyword::CASCADE);
                AlterTypeAction::DropAttribute {
                    name: attr_name,
                    if_exists,
                    cascade,
                }
            } else {
                return Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "ATTRIBUTE".to_string(),
                    got: format!("{:?}", self.peek()),
                });
            }
        } else if self.match_keyword(Keyword::RENAME) || self.match_ident_str("RENAME") {
            self.advance();
            if self.match_keyword(Keyword::ATTRIBUTE) || self.match_ident_str("ATTRIBUTE") {
                self.advance();
                let old_name = self.parse_identifier()?;
                self.expect_keyword(Keyword::TO)?;
                let new_name = self.parse_identifier()?;
                let cascade = self.try_consume_keyword(Keyword::CASCADE);
                AlterTypeAction::RenameAttribute {
                    old_name,
                    new_name,
                    cascade,
                }
            } else if self.match_ident_str("VALUE") {
                self.advance();
                let old_value = self.parse_string_literal()?;
                self.expect_keyword(Keyword::TO)?;
                let new_value = self.parse_string_literal()?;
                AlterTypeAction::RenameEnumValue {
                    old_value,
                    new_value,
                }
            } else {
                self.expect_keyword(Keyword::TO)?;
                let new_name = self.parse_identifier()?;
                AlterTypeAction::RenameTo(new_name)
            }
        } else if self.match_keyword(Keyword::SET) {
            self.advance();
            if self.match_keyword(Keyword::SCHEMA) {
                self.advance();
                let schema = self.parse_identifier()?;
                AlterTypeAction::SetSchema(schema)
            } else {
                return Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "SCHEMA".to_string(),
                    got: format!("{:?}", self.peek()),
                });
            }
        } else if self.match_keyword(Keyword::OWNER) || self.match_ident_str("OWNER") {
            self.advance();
            self.expect_keyword(Keyword::TO)?;
            let owner = self.parse_identifier()?;
            AlterTypeAction::OwnerTo(owner)
        } else {
            return Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "ADD, DROP, RENAME, SET, or OWNER".to_string(),
                got: format!("{:?}", self.peek()),
            });
        };

        Ok(AlterCompositeTypeStatement { name, action })
    }

    pub(crate) fn parse_alter_view(&mut self) -> Result<AlterViewStatement, ParserError> {
        self.expect_keyword(Keyword::VIEW)?;
        let if_exists = self.parse_if_exists();
        let _if_exists = if_exists;
        let name = self.parse_object_name()?;

        let action = if self.match_keyword(Keyword::RENAME) {
            self.advance();
            self.expect_keyword(Keyword::TO)?;
            let new_name = self.parse_identifier()?;
            AlterViewAction::RenameTo(new_name)
        } else if self.match_keyword(Keyword::SET) {
            self.advance();
            if self.match_keyword(Keyword::SCHEMA) {
                self.advance();
                let schema = self.parse_identifier()?;
                AlterViewAction::SetSchema(schema)
            } else if self.match_token(&Token::LParen) {
                let options = self.parse_generic_options_no_with();
                AlterViewAction::Set(options)
            } else {
                let options = self.parse_generic_options();
                AlterViewAction::Set(options)
            }
        } else if self.match_keyword(Keyword::RESET) {
            self.advance();
            self.expect_token(&Token::LParen)?;
            let mut names = vec![self.parse_identifier()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                names.push(self.parse_identifier()?);
            }
            self.expect_token(&Token::RParen)?;
            AlterViewAction::Reset(names)
        } else if self.match_keyword(Keyword::OWNER) || self.match_ident_str("OWNER") {
            self.advance();
            self.expect_keyword(Keyword::TO)?;
            let owner = self.parse_identifier()?;
            AlterViewAction::OwnerTo(owner)
        } else if self.match_keyword(Keyword::ALTER) {
            self.advance();
            self.expect_keyword(Keyword::COLUMN)?;
            let col = self.parse_identifier()?;
            if self.match_keyword(Keyword::SET) {
                self.advance();
                self.expect_keyword(Keyword::DEFAULT)?;
                let val = self.collect_rest_until_semicolon();
                AlterViewAction::AlterColumnDefault {
                    column: col,
                    set_default: Some(val),
                }
            } else {
                self.try_consume_keyword(Keyword::DROP);
                self.expect_keyword(Keyword::DEFAULT)?;
                AlterViewAction::AlterColumnDefault {
                    column: col,
                    set_default: None,
                }
            }
        } else {
            return Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "RENAME, SET, RESET, OWNER, or ALTER COLUMN".to_string(),
                got: format!("{:?}", self.peek()),
            });
        };

        Ok(AlterViewStatement { name, action })
    }

    fn collect_rest_until_semicolon(&mut self) -> String {
        let mut collected = String::new();
        loop {
            match self.peek() {
                Token::Eof | Token::Semicolon => break,
                _ => {
                    if !collected.is_empty() {
                        collected.push(' ');
                    }
                    collected.push_str(&self.token_to_string());
                    self.advance();
                }
            }
        }
        collected.trim().to_string()
    }

    pub(crate) fn parse_alter_trigger(&mut self) -> Result<AlterTriggerStatement, ParserError> {
        self.expect_keyword(Keyword::TRIGGER)?;
        let name = self.parse_identifier()?;

        if self.match_keyword(Keyword::ENABLE_P) {
            self.advance();
            return Ok(AlterTriggerStatement {
                name,
                table: None,
                new_name: None,
                enable: Some(true),
            });
        }

        if self.match_keyword(Keyword::DISABLE_P) {
            self.advance();
            return Ok(AlterTriggerStatement {
                name,
                table: None,
                new_name: None,
                enable: Some(false),
            });
        }

        self.expect_keyword(Keyword::ON)?;
        let table = self.parse_object_name()?;
        self.expect_keyword(Keyword::RENAME)?;
        self.expect_keyword(Keyword::TO)?;
        let new_name = self.parse_identifier()?;
        Ok(AlterTriggerStatement {
            name,
            table: Some(table),
            new_name: Some(new_name),
            enable: None,
        })
    }

    pub(crate) fn parse_alter_extension(&mut self) -> Result<AlterExtensionStatement, ParserError> {
        self.expect_keyword(Keyword::EXTENSION)?;
        let name = self.parse_identifier()?;
        let action = self.collect_rest_until_semicolon();
        Ok(AlterExtensionStatement { name, action })
    }

    // ========== 12 new CREATE statement parsers ==========

    pub(crate) fn parse_create_conversion(
        &mut self,
    ) -> Result<CreateConversionStatement, ParserError> {
        self.expect_keyword(Keyword::CONVERSION_P)?;
        let name = self.parse_identifier()?;
        self.expect_keyword(Keyword::FOR)?;
        let source_encoding = match self.peek().clone() {
            Token::StringLiteral(s) => {
                self.advance();
                s
            }
            _ => self.parse_identifier()?,
        };
        self.expect_keyword(Keyword::TO)?;
        let dest_encoding = match self.peek().clone() {
            Token::StringLiteral(s) => {
                self.advance();
                s
            }
            _ => self.parse_identifier()?,
        };
        self.expect_keyword(Keyword::FROM)?;
        let function_name = self.parse_identifier()?;
        self.try_consume_semicolon();
        Ok(CreateConversionStatement {
            name,
            source_encoding,
            dest_encoding,
            function_name,
        })
    }

    pub(crate) fn parse_create_synonym(
        &mut self,
        replace: bool,
    ) -> Result<CreateSynonymStatement, ParserError> {
        self.expect_keyword(Keyword::SYNONYM)?;
        let name = self.parse_object_name()?;
        self.expect_keyword(Keyword::FOR)?;
        let target = self.parse_object_name()?;
        let public = self.try_consume_ident_str("PUBLIC");
        self.try_consume_semicolon();
        Ok(CreateSynonymStatement {
            replace,
            name,
            target,
            public,
        })
    }

    pub(crate) fn parse_create_model(&mut self) -> Result<CreateModelStatement, ParserError> {
        self.expect_keyword(Keyword::MODEL)?;
        let name = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(CreateModelStatement { name, raw_rest })
    }

    pub(crate) fn parse_create_am(&mut self) -> Result<CreateAmStatement, ParserError> {
        self.expect_keyword(Keyword::ACCESS)?;
        self.expect_keyword(Keyword::METHOD)?;
        let name = self.parse_identifier()?;
        self.expect_keyword(Keyword::TYPE_P)?;
        let method = self.parse_identifier()?;
        self.expect_keyword(Keyword::HANDLER)?;
        let handler = self.parse_identifier()?;
        self.try_consume_semicolon();
        Ok(CreateAmStatement {
            name,
            method,
            handler,
        })
    }

    pub(crate) fn parse_create_directory(
        &mut self,
    ) -> Result<CreateDirectoryStatement, ParserError> {
        self.expect_keyword(Keyword::DIRECTORY)?;
        let name = self.parse_identifier()?;
        if self.try_consume_keyword(Keyword::AS) {
            let path = match self.peek().clone() {
                Token::StringLiteral(s) => {
                    self.advance();
                    s
                }
                _ => self.parse_identifier()?,
            };
            self.try_consume_semicolon();
            Ok(CreateDirectoryStatement { name, path })
        } else {
            let path = String::new();
            self.try_consume_semicolon();
            Ok(CreateDirectoryStatement { name, path })
        }
    }

    pub(crate) fn parse_create_data_source(
        &mut self,
    ) -> Result<CreateDataSourceStatement, ParserError> {
        self.expect_keyword(Keyword::SOURCE_P)?;
        let name = self.parse_identifier()?;

        let mut ds_type = None;
        if self.try_consume_keyword(Keyword::TYPE_P) {
            ds_type = Some(self.parse_string_or_ident()?);
        }

        let mut version = None;
        if self.try_consume_keyword(Keyword::VERSION_P) {
            if self.try_consume_keyword(Keyword::NULL_P) {
                version = Some("NULL".to_string());
            } else {
                version = Some(self.parse_string_or_ident()?);
            }
        }

        let options = self.parse_options_clause();
        self.try_consume_semicolon();
        Ok(CreateDataSourceStatement {
            name,
            ds_type,
            version,
            options,
        })
    }

    pub(crate) fn parse_create_event(&mut self) -> Result<CreateEventStatement, ParserError> {
        self.expect_keyword(Keyword::EVENT)?;
        let name = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(CreateEventStatement { name, raw_rest })
    }

    pub(crate) fn parse_create_opclass(&mut self) -> Result<CreateOpClassStatement, ParserError> {
        self.expect_keyword(Keyword::CLASS)?;
        let name = self.parse_identifier()?;
        self.expect_keyword(Keyword::USING)?;
        let method = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(CreateOpClassStatement {
            name,
            method,
            raw_rest,
        })
    }

    pub(crate) fn parse_create_opfamily(&mut self) -> Result<CreateOpFamilyStatement, ParserError> {
        self.expect_keyword(Keyword::FAMILY)?;
        let name = self.parse_identifier()?;
        self.expect_keyword(Keyword::USING)?;
        let method = self.parse_identifier()?;
        self.try_consume_semicolon();
        Ok(CreateOpFamilyStatement { name, method })
    }

    pub(crate) fn parse_create_contquery(
        &mut self,
    ) -> Result<CreateContQueryStatement, ParserError> {
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(CreateContQueryStatement { raw_rest })
    }

    pub(crate) fn parse_create_stream(&mut self) -> Result<CreateStreamStatement, ParserError> {
        self.expect_keyword(Keyword::STREAM)?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(CreateStreamStatement { raw_rest })
    }

    pub(crate) fn parse_create_key(&mut self) -> Result<CreateKeyStatement, ParserError> {
        self.expect_keyword(Keyword::KEY)?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(CreateKeyStatement { raw_rest })
    }
}
