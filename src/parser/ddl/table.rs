use crate::ast::*;
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;

impl Parser {
    pub(crate) fn parse_create_table(
        &mut self,
        temporary: bool,
        unlogged: bool,
    ) -> Result<CreateTableStatement, ParserError> {
        self.expect_keyword(Keyword::TABLE)?;

        let if_not_exists = self.parse_if_not_exists();
        let name = self.parse_object_name()?;
        self.expect_token(&Token::LParen)?;

        let mut columns = Vec::new();
        let mut constraints = Vec::new();
        let mut like_clauses = Vec::new();

        loop {
            if self.match_keyword(Keyword::CONSTRAINT)
                || self.match_keyword(Keyword::PRIMARY)
                || self.match_keyword(Keyword::UNIQUE)
                || self.match_keyword(Keyword::CHECK)
                || self.match_keyword(Keyword::FOREIGN)
            {
                constraints.push(self.parse_table_constraint()?);
            } else if self.match_keyword(Keyword::LIKE) {
                self.advance();
                like_clauses.push(self.parse_like_clause()?);
            } else {
                columns.push(self.parse_column_def()?);
            }

            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }

        self.expect_token(&Token::RParen)?;

        let mut inherits = Vec::new();
        let mut partition_by = None;
        let mut subpartition_by = None;
        let mut subpartitions_count = None;
        let mut distribute_by = None;
        let mut to_group = None;
        let mut tablespace = None;
        let mut on_commit = None;
        let mut options = Vec::new();
        let mut table_options = Vec::new();
        let mut compress = None;
        let mut ilm = None;
        let mut row_movement = None;

        loop {
            if self.match_keyword(Keyword::INHERITS) {
                self.advance();
                self.expect_token(&Token::LParen)?;
                inherits.push(self.parse_object_name()?);
                while self.match_token(&Token::Comma) {
                    self.advance();
                    inherits.push(self.parse_object_name()?);
                }
                self.expect_token(&Token::RParen)?;
            } else if self.match_keyword(Keyword::COMPRESS) {
                self.advance();
                compress = Some(true);
            } else if self.match_keyword(Keyword::NOCOMPRESS) {
                self.advance();
                compress = Some(false);
            } else if self.match_ident_str("ILM") {
                self.advance();
                if self.match_keyword(Keyword::ADD_P) {
                    self.advance();
                    if self.match_ident_str("POLICY") {
                        self.advance();
                    }
                    // Skip "ROW STORE COMPRESS ADVANCED ROW"
                    while !self.match_keyword(Keyword::AFTER) && !self.peek().eq(&Token::Eof) {
                        self.advance();
                    }
                    self.expect_keyword(Keyword::AFTER)?;
                    let after_n: u64 = match self.peek().clone() {
                        Token::Integer(n) => {
                            self.advance();
                            n as u64
                        }
                        _ => 0,
                    };
                    let unit = self.parse_identifier()?;
                    self.expect_keyword(Keyword::OF)?;
                    self.advance(); // NO
                    self.advance(); // MODIFICATION
                    let condition = if self.match_keyword(Keyword::ON) {
                        self.advance();
                        self.expect_token(&Token::LParen)?;
                        let expr = self.parse_expr()?;
                        self.expect_token(&Token::RParen)?;
                        Some(expr)
                    } else {
                        None
                    };
                    ilm = Some(IlmPolicy {
                        after_n,
                        unit,
                        condition,
                    });
                } else {
                    // Skip unknown ILM clause
                    while !self.match_keyword(Keyword::PARTITION)
                        && !self.match_keyword(Keyword::WITH)
                        && !self.match_token(&Token::Semicolon)
                        && !self.match_token(&Token::RParen)
                        && !self.peek().eq(&Token::Eof)
                    {
                        self.advance();
                    }
                }
            } else if self.match_keyword(Keyword::PARTITION) {
                self.advance();
                self.expect_keyword(Keyword::BY)?;
                let (strategy, is_columns) = match self.peek() {
                    Token::Ident(s) if s.to_uppercase() == "HASH" => {
                        self.advance();
                        ("hash", false)
                    }
                    _ => match self.peek_keyword() {
                        Some(Keyword::RANGE) => {
                            self.advance();
                            let is_columns = self.match_keyword(Keyword::COLUMNS);
                            if is_columns {
                                self.advance();
                            }
                            ("range", is_columns)
                        }
                        Some(Keyword::LIST) => {
                            self.advance();
                            let is_columns = self.match_keyword(Keyword::COLUMNS);
                            if is_columns {
                                self.advance();
                            }
                            ("list", is_columns)
                        }
                        _ => {
                            return Err(ParserError::UnexpectedToken {
                                location: self.current_location(),
                                expected: "RANGE, LIST, or HASH".to_string(),
                                got: format!("{:?}", self.peek()),
                            });
                        }
                    },
                };
                self.expect_token(&Token::LParen)?;
                let mut columns = vec![self.parse_object_name()?];
                while self.match_token(&Token::Comma) {
                    self.advance();
                    columns.push(self.parse_object_name()?);
                }
                self.expect_token(&Token::RParen)?;

                let (interval, partitions, partitions_count) = match strategy {
                    "range" => {
                        let interval = if self.match_keyword(Keyword::INTERVAL) {
                            self.advance();
                            self.expect_token(&Token::LParen)?;
                            let expr = self.parse_expr()?;
                            self.expect_token(&Token::RParen)?;
                            Some(expr)
                        } else {
                            None
                        };
                        let count = if self.match_keyword(Keyword::PARTITIONS) {
                            self.advance();
                            match self.peek().clone() {
                                Token::Integer(n) => {
                                    self.advance();
                                    Some(n as u32)
                                }
                                _ => None,
                            }
                        } else {
                            None
                        };
                        let parts = self.parse_partition_defs()?;
                        (interval, parts, count)
                    }
                    "list" => {
                        let count = if self.match_keyword(Keyword::PARTITIONS) {
                            self.advance();
                            match self.peek().clone() {
                                Token::Integer(n) => {
                                    self.advance();
                                    Some(n as u32)
                                }
                                _ => None,
                            }
                        } else {
                            None
                        };
                        let parts = self.parse_partition_defs()?;
                        (None, parts, count)
                    }
                    _ => {
                        let count = if self.match_keyword(Keyword::PARTITIONS) {
                            self.advance();
                            match self.peek().clone() {
                                Token::Integer(n) => {
                                    self.advance();
                                    Some(n as u32)
                                }
                                _ => None,
                            }
                        } else {
                            None
                        };
                        let parts = self.parse_partition_defs()?;
                        (None, parts, count)
                    }
                };

                partition_by = Some(match strategy {
                    "range" => PartitionClause::Range {
                        columns,
                        interval,
                        is_columns,
                        partitions_count,
                        partitions,
                    },
                    "list" => PartitionClause::List {
                        columns,
                        is_columns,
                        partitions,
                    },
                    _ => PartitionClause::Hash {
                        columns,
                        partitions_count,
                        partitions,
                    },
                });
            } else if self.match_keyword(Keyword::SUBPARTITION) {
                self.advance();
                self.expect_keyword(Keyword::BY)?;
                let (strategy, is_columns) = match self.peek() {
                    Token::Ident(s) if s.to_uppercase() == "HASH" => {
                        self.advance();
                        ("hash", false)
                    }
                    _ => match self.peek_keyword() {
                        Some(Keyword::RANGE) => {
                            self.advance();
                            let is_columns = self.match_keyword(Keyword::COLUMNS);
                            if is_columns {
                                self.advance();
                            }
                            ("range", is_columns)
                        }
                        Some(Keyword::LIST) => {
                            self.advance();
                            let is_columns = self.match_keyword(Keyword::COLUMNS);
                            if is_columns {
                                self.advance();
                            }
                            ("list", is_columns)
                        }
                        _ => {
                            return Err(ParserError::UnexpectedToken {
                                location: self.current_location(),
                                expected: "RANGE, LIST, or HASH".to_string(),
                                got: format!("{:?}", self.peek()),
                            });
                        }
                    },
                };
                self.expect_token(&Token::LParen)?;
                let mut columns = vec![self.parse_object_name()?];
                while self.match_token(&Token::Comma) {
                    self.advance();
                    columns.push(self.parse_object_name()?);
                }
                self.expect_token(&Token::RParen)?;

                let (sp_parts, sp_count) =
                    if strategy == "hash" && self.match_keyword(Keyword::SUBPARTITIONS) {
                        self.advance();
                        let count = match self.peek().clone() {
                            Token::Integer(n) => {
                                self.advance();
                                Some(n as u32)
                            }
                            _ => None,
                        };
                        let parts = self.parse_subpartition_defs()?;
                        (parts, count)
                    } else {
                        let parts = self.parse_subpartition_defs()?;
                        (parts, None)
                    };

                subpartition_by = Some(match strategy {
                    "range" => PartitionClause::Range {
                        columns,
                        interval: None,
                        is_columns,
                        partitions_count: None,
                        partitions: sp_parts,
                    },
                    "list" => PartitionClause::List {
                        columns,
                        is_columns,
                        partitions: sp_parts,
                    },
                    _ => PartitionClause::Hash {
                        columns,
                        partitions_count: sp_count,
                        partitions: sp_parts,
                    },
                });
                subpartitions_count = sp_count;

                // If partition defs haven't been parsed yet (empty in partition_by),
                // try parsing them now - they may follow the SUBPARTITION BY clause
                if let Some(ref pb) = partition_by {
                    let empty = match pb {
                        PartitionClause::Range { partitions, .. } => partitions.is_empty(),
                        PartitionClause::List { partitions, .. } => partitions.is_empty(),
                        PartitionClause::Hash { partitions, .. } => partitions.is_empty(),
                    };
                    if empty {
                        let parts = self.parse_partition_defs()?;
                        if !parts.is_empty() {
                            partition_by = Some(match pb.clone() {
                                PartitionClause::Range {
                                    columns,
                                    interval,
                                    is_columns,
                                    partitions_count,
                                    ..
                                } => PartitionClause::Range {
                                    columns,
                                    interval,
                                    is_columns,
                                    partitions_count,
                                    partitions: parts,
                                },
                                PartitionClause::List {
                                    columns, is_columns, ..
                                } => PartitionClause::List {
                                    columns,
                                    is_columns,
                                    partitions: parts,
                                },
                                PartitionClause::Hash {
                                    columns,
                                    partitions_count,
                                    ..
                                } => PartitionClause::Hash {
                                    columns,
                                    partitions_count,
                                    partitions: parts,
                                },
                            });
                        }
                    }
                }
            } else if self.match_keyword(Keyword::TABLESPACE) {
                self.advance();
                tablespace = Some(self.parse_identifier()?);
            } else if self.match_keyword(Keyword::ON) {
                self.advance();
                self.expect_keyword(Keyword::COMMIT)?;
                on_commit = Some(if self.match_keyword(Keyword::PRESERVE) {
                    self.advance();
                    self.expect_keyword(Keyword::ROWS)?;
                    OnCommitAction::PreserveRows
                } else if self.match_keyword(Keyword::DELETE_P) {
                    self.advance();
                    self.expect_keyword(Keyword::ROWS)?;
                    OnCommitAction::DeleteRows
                } else {
                    self.expect_keyword(Keyword::DROP)?;
                    OnCommitAction::Drop
                });
            } else if self.match_keyword(Keyword::WITH) {
                self.advance();
                self.expect_token(&Token::LParen)?;
                loop {
                    let key = self.parse_identifier()?;
                    self.expect_token(&Token::Eq)?;
                    let val = match self.peek().clone() {
                        Token::StringLiteral(s) => {
                            self.advance();
                            s
                        }
                        Token::Ident(s) => {
                            self.advance();
                            s
                        }
                        Token::Integer(n) => {
                            self.advance();
                            n.to_string()
                        }
                        Token::Keyword(kw) => {
                            self.advance();
                            kw.as_str().to_string()
                        }
                        _ => {
                            return Err(ParserError::UnexpectedToken {
                                location: self.current_location(),
                                expected: "option value".to_string(),
                                got: format!("{:?}", self.peek()),
                            });
                        }
                    };
                    options.push((key, val));
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                    self.advance();
                }
                self.expect_token(&Token::RParen)?;
            } else if self.match_keyword(Keyword::DISTRIBUTE) {
                self.advance();
                self.expect_keyword(Keyword::BY)?;
                distribute_by = Some(if self.match_ident_str("HASH") {
                    self.advance();
                    self.expect_token(&Token::LParen)?;
                    let mut cols = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        cols.push(self.parse_identifier()?);
                    }
                    self.expect_token(&Token::RParen)?;
                    DistributeClause::Hash { columns: cols }
                } else if self.match_ident_str("REPLICATION") {
                    self.advance();
                    DistributeClause::Replication
                } else if self.match_ident_str("ROUNDROBIN") {
                    self.advance();
                    self.expect_token(&Token::LParen)?;
                    let mut cols = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        cols.push(self.parse_identifier()?);
                    }
                    self.expect_token(&Token::RParen)?;
                    DistributeClause::RoundRobin { columns: cols }
                } else if self.match_ident_str("MODULO") {
                    self.advance();
                    self.expect_token(&Token::LParen)?;
                    let mut cols = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        cols.push(self.parse_identifier()?);
                    }
                    self.expect_token(&Token::RParen)?;
                    DistributeClause::Modulo { columns: cols }
                } else {
                    return Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "HASH, REPLICATION, ROUNDROBIN, or MODULO".to_string(),
                        got: format!("{:?}", self.peek()),
                    });
                });
            } else if self.match_keyword(Keyword::ENABLE_P) {
                self.advance();
                self.expect_keyword(Keyword::ROW)?;
                self.expect_keyword(Keyword::MOVEMENT)?;
                row_movement = Some(true);
            } else if self.match_keyword(Keyword::DISABLE_P) {
                self.advance();
                self.expect_keyword(Keyword::ROW)?;
                self.expect_keyword(Keyword::MOVEMENT)?;
                row_movement = Some(false);
            } else if self.match_keyword(Keyword::TO) {
                self.advance();
                self.expect_keyword(Keyword::GROUP_P)?;
                to_group = Some(self.parse_identifier()?);
            } else if self.match_keyword(Keyword::TABLE) {
                self.advance();
                self.expect_keyword(Keyword::OPTION)?;
                self.expect_token(&Token::LParen)?;
                loop {
                    let key = self.parse_identifier()?;
                    if self.match_token(&Token::Eq) {
                        self.advance();
                    }
                    let val = match self.peek().clone() {
                        Token::StringLiteral(s) => {
                            self.advance();
                            s
                        }
                        Token::Ident(s) => {
                            self.advance();
                            s
                        }
                        Token::Integer(n) => {
                            self.advance();
                            n.to_string()
                        }
                        Token::Keyword(kw) => {
                            self.advance();
                            kw.as_str().to_string()
                        }
                        _ => {
                            return Err(ParserError::UnexpectedToken {
                                location: self.current_location(),
                                expected: "option value".to_string(),
                                got: format!("{:?}", self.peek()),
                            });
                        }
                    };
                    table_options.push((key, val));
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                    self.advance();
                }
                self.expect_token(&Token::RParen)?;
            } else if self.match_keyword(Keyword::COMMENT) {
                self.advance();
                if self.match_token(&Token::Eq) {
                    self.advance();
                }
                match self.peek().clone() {
                    Token::StringLiteral(s) => {
                        self.advance();
                        table_options.push(("COMMENT".to_string(), s));
                    }
                    _ => {
                        return Err(ParserError::UnexpectedToken {
                            location: self.current_location(),
                            expected: "string literal".to_string(),
                            got: format!("{:?}", self.peek()),
                        });
                    }
                }
            } else if self.match_keyword(Keyword::DEFAULT) && {
                // Lookahead: DEFAULT CHARACTER SET or DEFAULT COLLATE
                let pos = self.pos;
                self.advance();
                let is_charset =
                    self.match_keyword(Keyword::CHARACTER) || self.match_keyword(Keyword::CHARSET);
                let is_collate = self.match_keyword(Keyword::COLLATE);
                self.pos = pos;
                is_charset || is_collate
            } {
                self.advance(); // consumed DEFAULT
                if self.match_keyword(Keyword::CHARACTER) {
                    self.advance();
                    if self.match_keyword(Keyword::SET) {
                        self.advance();
                    }
                    if self.match_token(&Token::Eq) {
                        self.advance();
                    }
                    let val = self.parse_identifier()?;
                    table_options.push(("CHARACTER SET".to_string(), val));
                } else if self.match_keyword(Keyword::CHARSET) {
                    self.advance();
                    if self.match_token(&Token::Eq) {
                        self.advance();
                    }
                    let val = self.parse_identifier()?;
                    table_options.push(("CHARSET".to_string(), val));
                } else if self.match_keyword(Keyword::COLLATE) {
                    self.advance();
                    if self.match_token(&Token::Eq) {
                        self.advance();
                    }
                    let val = self.parse_identifier()?;
                    table_options.push(("COLLATE".to_string(), val));
                }
            } else if self.match_keyword(Keyword::CHARACTER) {
                self.advance();
                if self.match_keyword(Keyword::SET) {
                    self.advance();
                }
                if self.match_token(&Token::Eq) {
                    self.advance();
                }
                let val = self.parse_identifier()?;
                table_options.push(("CHARACTER SET".to_string(), val));
            } else if self.match_keyword(Keyword::CHARSET) {
                self.advance();
                if self.match_token(&Token::Eq) {
                    self.advance();
                }
                let val = self.parse_identifier()?;
                table_options.push(("CHARSET".to_string(), val));
            } else if self.match_keyword(Keyword::COLLATE) {
                self.advance();
                if self.match_token(&Token::Eq) {
                    self.advance();
                }
                let val = self.parse_identifier()?;
                table_options.push(("COLLATE".to_string(), val));
            } else if self.match_keyword(Keyword::ENCRYPTION) {
                self.advance();
                if self.match_keyword(Keyword::COLUMN) {
                    self.advance();
                }
                let spec = self.parse_identifier()?;
                table_options.push(("ENCRYPTION COLUMN".to_string(), spec));
            } else {
                break;
            }
        }

        Ok(CreateTableStatement {
            temporary,
            unlogged,
            if_not_exists,
            name,
            columns,
            constraints,
            like_clauses,
            inherits,
            partition_by,
            subpartition_by,
            subpartitions_count,
            distribute_by,
            to_group,
            tablespace,
            on_commit,
            options,
            table_options,
            compress,
            ilm,
            row_movement,
        })
    }

    pub(crate) fn parse_create_table_as(
        &mut self,
        temporary: bool,
        unlogged: bool,
    ) -> Result<crate::ast::Statement, ParserError> {
        self.expect_keyword(Keyword::TABLE)?;
        let if_not_exists = self.parse_if_not_exists();
        let name = self.parse_object_name()?;

        let column_names = if self.match_token(&Token::LParen) {
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

        let (query, as_table) = if self.match_keyword(Keyword::TABLE) {
            self.advance();
            let table_name = self.parse_object_name()?;
            let synthetic_query = SelectStatement {
                hints: vec![],
                with: None,
                distinct: false,
                distinct_on: vec![],
                targets: vec![SelectTarget::Star(None)],
                into_targets: None,
                into_table: None,
                from: vec![TableRef::Table {
                    name: table_name.clone(),
                    alias: None,
                    partition: None,
                    timecapsule: None,
                }],
                where_clause: None,
                connect_by: None,
                group_by: vec![],
                having: None,
                order_by: vec![],
                order_siblings: false,
                limit: None,
                offset: None,
                fetch: None,
                lock_clause: None,
                window_clause: vec![],
                set_operation: None,
                raw_body: None,
            };
            (Box::new(synthetic_query), Some(table_name))
        } else {
            (Box::new(self.parse_select_statement()?), None)
        };

        let with_data = if self.match_keyword(Keyword::WITH) {
            self.advance();
            if self.match_keyword(Keyword::NO) {
                self.advance();
                self.expect_keyword(Keyword::DATA_P)?;
                false
            } else {
                self.expect_keyword(Keyword::DATA_P)?;
                true
            }
        } else {
            true
        };
        Ok(crate::ast::Statement::CreateTableAs(
            crate::ast::CreateTableAsStatement {
                temporary,
                unlogged,
                if_not_exists,
                name,
                column_names,
                query,
                as_table,
                with_data,
            },
        ))
    }

    pub(crate) fn parse_if_not_exists(&mut self) -> bool {
        if self.match_keyword(Keyword::IF_P) {
            self.advance();
            if self.match_keyword(Keyword::NOT) {
                self.advance();
                if self.match_keyword(Keyword::EXISTS) {
                    self.advance();
                    return true;
                }
            }
        }
        false
    }

    fn parse_like_clause(&mut self) -> Result<LikeClause, ParserError> {
        let source_table = self.parse_object_name()?;
        let mut options = Vec::new();
        loop {
            let is_including = if self.match_keyword(Keyword::INCLUDING) {
                self.advance();
                true
            } else if self.match_keyword(Keyword::EXCLUDING) {
                self.advance();
                false
            } else {
                break;
            };
            let option_name = if self.match_keyword(Keyword::DEFAULTS) {
                self.advance();
                "DEFAULTS".to_string()
            } else if self.match_keyword(Keyword::CONSTRAINTS) {
                self.advance();
                "CONSTRAINTS".to_string()
            } else if self.match_keyword(Keyword::INDEXES) {
                self.advance();
                "INDEXES".to_string()
            } else if self.match_keyword(Keyword::STORAGE) {
                self.advance();
                "STORAGE".to_string()
            } else if self.match_keyword(Keyword::COMMENTS) {
                self.advance();
                "COMMENTS".to_string()
            } else if self.match_keyword(Keyword::PARTITION) {
                self.advance();
                "PARTITION".to_string()
            } else if self.match_keyword(Keyword::RELOPTIONS) {
                self.advance();
                "RELOPTIONS".to_string()
            } else if self.match_keyword(Keyword::DISTRIBUTION) {
                self.advance();
                "DISTRIBUTION".to_string()
            } else if self.match_keyword(Keyword::ALL) {
                self.advance();
                "ALL".to_string()
            } else {
                break;
            };
            options.push((is_including, option_name));
        }
        Ok(LikeClause {
            source_table,
            options,
        })
    }

    pub(crate) fn parse_column_def(&mut self) -> Result<ColumnDef, ParserError> {
        let name = self.parse_identifier()?;
        let data_type = self.parse_data_type()?;

        let compress_mode = if self.match_ident_str("DELTA") {
            self.advance();
            Some("DELTA".to_string())
        } else if self.match_ident_str("PREFIX") {
            self.advance();
            Some("PREFIX".to_string())
        } else if self.match_ident_str("DICTIONARY") {
            self.advance();
            Some("DICTIONARY".to_string())
        } else if self.match_ident_str("NUMSTR") {
            self.advance();
            Some("NUMSTR".to_string())
        } else {
            None
        };

        let mut charset = None;
        let mut collate = None;

        if self.match_keyword(Keyword::CHARACTER) || self.match_keyword(Keyword::CHARSET) {
            let is_charset_keyword = self.match_keyword(Keyword::CHARSET);
            self.advance();
            if !is_charset_keyword && self.match_keyword(Keyword::SET) {
                self.advance();
            }
            if self.match_token(&Token::Eq) {
                self.advance();
            }
            charset = Some(self.parse_identifier()?);
        }

        if self.match_keyword(Keyword::COLLATE) {
            self.advance();
            if self.match_token(&Token::Eq) {
                self.advance();
            }
            collate = Some(self.parse_identifier()?);
        }

        let mut constraints = Vec::new();
        while let Some(constraint) = self.try_parse_column_constraint()? {
            constraints.push(constraint);
        }

        self.consume_opt_using_index_attrs();

        if charset.is_none()
            && (self.match_keyword(Keyword::CHARACTER) || self.match_keyword(Keyword::CHARSET))
        {
            let is_charset_keyword = self.match_keyword(Keyword::CHARSET);
            self.advance();
            if !is_charset_keyword && self.match_keyword(Keyword::SET) {
                self.advance();
            }
            if self.match_token(&Token::Eq) {
                self.advance();
            }
            charset = Some(self.parse_identifier()?);
        }

        if collate.is_none() && self.match_keyword(Keyword::COLLATE) {
            self.advance();
            if self.match_token(&Token::Eq) {
                self.advance();
            }
            collate = Some(self.parse_identifier()?);
        }

        let mut on_update = None;
        if self.match_keyword(Keyword::ON) {
            let pos = self.pos;
            self.advance();
            if self.match_keyword(Keyword::UPDATE) {
                self.advance();
                let expr = if self.match_keyword(Keyword::CURRENT_TIMESTAMP) {
                    self.advance();
                    "CURRENT_TIMESTAMP".to_string()
                } else if self.match_ident_str("LOCALTIMESTAMP") {
                    self.advance();
                    "LOCALTIMESTAMP".to_string()
                } else if self.match_ident_str("NOW") {
                    self.advance();
                    self.expect_token(&Token::LParen)?;
                    self.advance();
                    self.expect_token(&Token::RParen)?;
                    "NOW()".to_string()
                } else {
                    self.parse_identifier()?
                };
                on_update = Some(expr);
            } else {
                self.pos = pos;
            }
        }

        let mut comment = None;
        if self.try_consume_keyword(Keyword::COMMENT) {
            comment = Some(self.parse_string_literal()?);
        }

        let mut generated = None;
        if self.match_keyword(Keyword::GENERATED) {
            self.advance();
            if self.try_consume_keyword(Keyword::ALWAYS) {
                self.expect_keyword(Keyword::AS)?;
                if self.match_token(&Token::LParen) {
                    self.advance();
                    let expr = self.parse_expr()?;
                    self.expect_token(&Token::RParen)?;
                    let stored = self.try_consume_keyword(Keyword::STORED);
                    generated = Some(crate::ast::GeneratedColumn { expr, stored });
                } else if self.try_consume_keyword(Keyword::IDENTITY_P) {
                    if self.match_token(&Token::LParen) {
                        self.advance();
                        while !self.match_token(&Token::RParen) && !self.peek().eq(&Token::Eof) {
                            self.advance();
                        }
                        if self.match_token(&Token::RParen) {
                            self.advance();
                        }
                    }
                }
            } else if self.try_consume_keyword(Keyword::BY) {
                self.expect_keyword(Keyword::DEFAULT)?;
                self.try_consume_keyword(Keyword::ON);
                self.try_consume_keyword(Keyword::NULL_P);
                self.expect_keyword(Keyword::AS)?;
                self.expect_keyword(Keyword::IDENTITY_P)?;
                if self.match_token(&Token::LParen) {
                    self.advance();
                    while !self.match_token(&Token::RParen) && !self.peek().eq(&Token::Eof) {
                        self.advance();
                    }
                    if self.match_token(&Token::RParen) {
                        self.advance();
                    }
                }
            }
        }

        let encrypted_with = if self.match_keyword(Keyword::ENCRYPTED) {
            self.advance();
            self.expect_keyword(Keyword::WITH)?;
            self.expect_token(&Token::LParen)?;
            let mut column_encryption_key = None;
            let mut encryption_type = None;
            loop {
                let key = self.consume_any_identifier()?;
                self.expect_token(&Token::Eq)?;
                let value = self.consume_any_identifier()?;
                match key.to_uppercase().as_str() {
                    "COLUMN_ENCRYPTION_KEY" => column_encryption_key = Some(value),
                    "ENCRYPTION_TYPE" => encryption_type = Some(value),
                    _ => {}
                }
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect_token(&Token::RParen)?;
            Some(crate::ast::EncryptedWith {
                column_encryption_key: column_encryption_key.unwrap_or_default(),
                encryption_type: encryption_type.unwrap_or_default(),
            })
        } else {
            None
        };

        Ok(ColumnDef {
            name,
            data_type,
            constraints,
            compress_mode,
            charset,
            collate,
            on_update,
            comment,
            generated,
            encrypted_with,
        })
    }

    pub(crate) fn parse_data_type(&mut self) -> Result<DataType, ParserError> {
        let result = match self.peek_keyword() {
            Some(Keyword::BOOLEAN_P) => {
                self.advance();
                DataType::Boolean
            }
            Some(Keyword::TINYINT) => {
                self.advance();
                let precision = self.parse_opt_int_precision()?;
                DataType::TinyInt(precision)
            }
            Some(Keyword::SMALLINT) => {
                self.advance();
                let precision = self.parse_opt_int_precision()?;
                DataType::SmallInt(precision)
            }
            Some(Keyword::INTEGER) | Some(Keyword::INT_P) => {
                self.advance();
                let precision = self.parse_opt_int_precision()?;
                DataType::Integer(precision)
            }
            Some(Keyword::BIGINT) => {
                self.advance();
                let precision = self.parse_opt_int_precision()?;
                DataType::BigInt(precision)
            }
            Some(Keyword::REAL) => {
                self.advance();
                DataType::Real
            }
            Some(Keyword::FLOAT_P) => {
                self.advance();
                let precision = if self.match_token(&Token::LParen) {
                    self.advance();
                    let n = self.parse_int_literal()?;
                    self.expect_token(&Token::RParen)?;
                    Some(n)
                } else {
                    None
                };
                DataType::Float(precision)
            }
            Some(Keyword::DOUBLE_P) => {
                self.advance();
                if self.match_keyword(Keyword::PRECISION) {
                    self.advance();
                }
                DataType::Double
            }
            Some(Keyword::NUMERIC) | Some(Keyword::DECIMAL_P) => {
                self.advance();
                let (precision, scale) = if self.match_token(&Token::LParen) {
                    self.advance();
                    let prec = self.parse_int_literal()?;
                    let scale = if self.match_token(&Token::Comma) {
                        self.advance();
                        Some(self.parse_int_literal()?)
                    } else {
                        None
                    };
                    self.expect_token(&Token::RParen)?;
                    (Some(prec), scale)
                } else {
                    (None, None)
                };
                DataType::Numeric(precision, scale)
            }
            Some(Keyword::CHAR_P) | Some(Keyword::CHARACTER) => {
                self.advance();
                if self.match_keyword(Keyword::VARYING) {
                    self.advance();
                    let len = if self.match_token(&Token::LParen) {
                        self.advance();
                        let n = self.parse_int_literal()?;
                        self.expect_token(&Token::RParen)?;
                        Some(n)
                    } else {
                        None
                    };
                    DataType::Varchar(len)
                } else {
                    let len = if self.match_token(&Token::LParen) {
                        self.advance();
                        let n = self.parse_int_literal()?;
                        self.expect_token(&Token::RParen)?;
                        Some(n)
                    } else {
                        None
                    };
                    DataType::Char(len)
                }
            }
            Some(Keyword::VARCHAR) => {
                self.advance();
                let len = if self.match_token(&Token::LParen) {
                    self.advance();
                    let n = self.parse_int_literal()?;
                    self.expect_token(&Token::RParen)?;
                    Some(n)
                } else {
                    None
                };
                DataType::Varchar(len)
            }
            Some(Keyword::TEXT_P) => {
                self.advance();
                DataType::Text
            }
            Some(Keyword::BYTE_P) => {
                self.advance();
                DataType::Bytea
            }
            Some(Keyword::TIMESTAMP) => {
                self.advance();
                let precision = if self.match_token(&Token::LParen) {
                    self.advance();
                    let n = self.parse_int_literal()?;
                    self.expect_token(&Token::RParen)?;
                    Some(n)
                } else {
                    None
                };
                let tz = self.parse_timezone_info()?;
                DataType::Timestamp(precision, tz)
            }
            Some(Keyword::DATE_P) => {
                self.advance();
                DataType::Date
            }
            Some(Keyword::TIME) => {
                self.advance();
                let precision = if self.match_token(&Token::LParen) {
                    self.advance();
                    let n = self.parse_int_literal()?;
                    self.expect_token(&Token::RParen)?;
                    Some(n)
                } else {
                    None
                };
                let tz = self.parse_timezone_info()?;
                DataType::Time(precision, tz)
            }
            Some(Keyword::INTERVAL) => {
                self.advance();
                let it = self.parse_opt_interval_type()?;
                DataType::Interval(it)
            }
            Some(Keyword::BIT) => {
                self.advance();
                if self.match_keyword(Keyword::VARYING) {
                    self.advance();
                    let len = if self.match_token(&Token::LParen) {
                        self.advance();
                        let n = self.parse_int_literal()?;
                        self.expect_token(&Token::RParen)?;
                        Some(n)
                    } else {
                        None
                    };
                    DataType::Varbit(len)
                } else {
                    let len = if self.match_token(&Token::LParen) {
                        self.advance();
                        let n = self.parse_int_literal()?;
                        self.expect_token(&Token::RParen)?;
                        Some(n)
                    } else {
                        None
                    };
                    DataType::Bit(len)
                }
            }
            _ => {
                if let Token::Ident(s) = self.peek().clone() {
                    match s.to_uppercase().as_str() {
                        "SERIAL" => {
                            self.advance();
                            DataType::Serial
                        }
                        "SMALLSERIAL" => {
                            self.advance();
                            DataType::SmallSerial
                        }
                        "BIGSERIAL" => {
                            self.advance();
                            DataType::BigSerial
                        }
                        "BINARY_FLOAT" => {
                            self.advance();
                            DataType::BinaryFloat
                        }
                        "BINARY_DOUBLE" => {
                            self.advance();
                            DataType::BinaryDouble
                        }
                        "BOOL" => {
                            self.advance();
                            DataType::Boolean
                        }
                        _ => {
                            let name = self.parse_object_name()?;
                            let args = if self.match_token(&Token::LParen) {
                                self.advance();
                                let mut args = Vec::new();
                                loop {
                                    args.push(self.parse_expr()?);
                                    if self.match_token(&Token::Comma) {
                                        self.advance();
                                    } else {
                                        break;
                                    }
                                }
                                self.expect_token(&Token::RParen)?;
                                args
                            } else {
                                Vec::new()
                            };
                            DataType::Custom(name, args)
                        }
                    }
                } else {
                    let name = self.parse_object_name()?;
                    let args = if self.match_token(&Token::LParen) {
                        self.advance();
                        let mut args = Vec::new();
                        loop {
                            args.push(self.parse_expr()?);
                            if self.match_token(&Token::Comma) {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        self.expect_token(&Token::RParen)?;
                        args
                    } else {
                        Vec::new()
                    };
                    DataType::Custom(name, args)
                }
            }
        };
        // Check for array suffix [] (possibly multiple, for multi-dimensional arrays)
        let mut result = result;
        while self.match_token(&Token::LBracket) {
            self.advance();
            self.expect_token(&Token::RBracket)?;
            result = DataType::Array(Box::new(result));
        }
        Ok(result)
    }

    fn parse_int_literal(&mut self) -> Result<u32, ParserError> {
        match self.peek() {
            Token::Integer(n) => {
                let n = *n as u32;
                self.advance();
                Ok(n)
            }
            _ => Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "integer literal".to_string(),
                got: format!("{:?}", self.peek()),
            }),
        }
    }

    fn parse_opt_int_precision(&mut self) -> Result<Option<u32>, ParserError> {
        if self.match_token(&Token::LParen) {
            self.advance();
            let n = self.parse_int_literal()?;
            if self.match_token(&Token::Comma) {
                self.advance();
                let _scale = self.parse_int_literal()?;
            }
            self.expect_token(&Token::RParen)?;
            Ok(Some(n))
        } else {
            Ok(None)
        }
    }

    fn parse_opt_interval_type(&mut self) -> Result<Option<IntervalType>, ParserError> {
        let unit = match self.peek_keyword() {
            Some(Keyword::YEAR_P) => "YEAR",
            Some(Keyword::MONTH_P) => "MONTH",
            Some(Keyword::DAY_P) => "DAY",
            Some(Keyword::HOUR_P) => "HOUR",
            Some(Keyword::MINUTE_P) => "MINUTE",
            Some(Keyword::SECOND_P) => "SECOND",
            _ => return Ok(None),
        };
        self.advance();
        let from_precision = self.parse_opt_int_precision().ok().flatten();
        if self.match_keyword(Keyword::TO) {
            self.advance();
            let to_unit = match self.peek_keyword() {
                Some(Keyword::YEAR_P) => "YEAR",
                Some(Keyword::MONTH_P) => "MONTH",
                Some(Keyword::DAY_P) => "DAY",
                Some(Keyword::HOUR_P) => "HOUR",
                Some(Keyword::MINUTE_P) => "MINUTE",
                Some(Keyword::SECOND_P) => "SECOND",
                _ => {
                    return Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "interval unit after TO".to_string(),
                        got: format!("{:?}", self.peek()),
                    })
                }
            };
            self.advance();
            let to_precision = self.parse_opt_int_precision().ok().flatten();
            Ok(Some(IntervalType {
                from: unit.to_string(),
                from_precision,
                to: Some(to_unit.to_string()),
                to_precision,
            }))
        } else {
            Ok(Some(IntervalType {
                from: unit.to_string(),
                from_precision,
                to: None,
                to_precision: None,
            }))
        }
    }

    fn parse_timezone_info(&mut self) -> Result<Option<TimeZoneInfo>, ParserError> {
        if self.match_keyword(Keyword::WITH) {
            self.advance();
            if self.match_keyword(Keyword::TIME) {
                self.advance();
                self.expect_keyword(Keyword::ZONE)?;
                Ok(Some(TimeZoneInfo::WithTimeZone))
            } else {
                Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "TIME".to_string(),
                    got: format!("{:?}", self.peek()),
                })
            }
        } else if self.match_keyword(Keyword::WITHOUT) {
            self.advance();
            if self.match_keyword(Keyword::TIME) {
                self.advance();
                self.expect_keyword(Keyword::ZONE)?;
                Ok(Some(TimeZoneInfo::WithoutTimeZone))
            } else {
                Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "TIME".to_string(),
                    got: format!("{:?}", self.peek()),
                })
            }
        } else {
            Ok(None)
        }
    }

    fn consume_opt_enable_disable(&mut self) {
        if self.match_keyword(Keyword::ENABLE_P) || self.match_keyword(Keyword::DISABLE_P) {
            self.advance();
        }
    }

    fn consume_opt_using_index_tablespace(&mut self) {
        if self.match_keyword(Keyword::USING) {
            self.advance();
            if self.match_keyword(Keyword::INDEX) {
                self.advance();
            }
            if self.match_keyword(Keyword::TABLESPACE) {
                self.advance();
                let _ = self.parse_identifier();
            }
        }
    }

    fn consume_opt_using_index_attrs(&mut self) {
        if !self.match_keyword(Keyword::USING) {
            return;
        }
        self.advance();
        if self.match_keyword(Keyword::INDEX) {
            self.advance();
        }
        while let Some(kw) = self.peek_keyword() {
            match kw {
                Keyword::PCTFREE
                | Keyword::INITRANS
                | Keyword::MAXTRANS
                | Keyword::STORAGE
                | Keyword::TABLESPACE => {
                    self.advance();
                    let _ = self.parse_expr();
                }
                _ => break,
            }
        }
    }

    fn try_parse_column_constraint(&mut self) -> Result<Option<ColumnConstraint>, ParserError> {
        let result = match self.peek_keyword() {
            Some(Keyword::NOT) => {
                self.advance();
                self.expect_keyword(Keyword::NULL_P)?;
                Some(ColumnConstraint::NotNull)
            }
            Some(Keyword::NULL_P) => {
                self.advance();
                Some(ColumnConstraint::Null)
            }
            Some(Keyword::DEFAULT) => {
                self.advance();
                let expr = self.parse_expr()?;
                Some(ColumnConstraint::Default(expr))
            }
            Some(Keyword::UNIQUE) => {
                self.advance();
                self.consume_opt_using_index_tablespace();
                self.try_consume_keyword(Keyword::DEFERRABLE);
                if self.match_keyword(Keyword::WITH) {
                    let _ = self.parse_generic_options();
                }
                Some(ColumnConstraint::Unique)
            }
            Some(Keyword::PRIMARY) => {
                self.advance();
                self.expect_keyword(Keyword::KEY)?;
                self.consume_opt_using_index_tablespace();
                self.try_consume_keyword(Keyword::DEFERRABLE);
                if self.match_keyword(Keyword::WITH) {
                    let _ = self.parse_generic_options();
                }
                Some(ColumnConstraint::PrimaryKey)
            }
            Some(Keyword::CHECK) => {
                self.advance();
                self.expect_token(&Token::LParen)?;
                let expr = self.parse_expr()?;
                self.expect_token(&Token::RParen)?;
                Some(ColumnConstraint::Check(expr))
            }
            Some(Keyword::REFERENCES) => {
                self.advance();
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
                    Vec::new()
                };
                Some(ColumnConstraint::References(table, columns))
            }
            _ => None,
        };
        if result.is_some() {
            self.consume_opt_enable_disable();
        }
        Ok(result)
    }

    pub(crate) fn parse_table_constraint(&mut self) -> Result<TableConstraint, ParserError> {
        if self.match_keyword(Keyword::CONSTRAINT) {
            self.advance();
            if !self.match_keyword(Keyword::PRIMARY)
                && !self.match_keyword(Keyword::UNIQUE)
                && !self.match_keyword(Keyword::CHECK)
                && !self.match_keyword(Keyword::FOREIGN)
            {
                let _name = self.parse_identifier()?;
            }
        }

        match self.peek_keyword() {
            Some(Keyword::PRIMARY) => {
                self.advance();
                self.expect_keyword(Keyword::KEY)?;
                if self.try_consume_keyword(Keyword::USING) {
                    let _ = self.parse_identifier();
                }
                let columns = self.parse_column_list()?;
                Ok(TableConstraint::PrimaryKey(columns))
            }
            Some(Keyword::UNIQUE) => {
                self.advance();
                let columns = self.parse_column_list()?;
                self.try_consume_keyword(Keyword::DEFERRABLE);
                if self.match_keyword(Keyword::WITH) {
                    let _ = self.parse_generic_options();
                }
                Ok(TableConstraint::Unique(columns))
            }
            Some(Keyword::CHECK) => {
                self.advance();
                self.expect_token(&Token::LParen)?;
                let expr = self.parse_expr()?;
                self.expect_token(&Token::RParen)?;
                Ok(TableConstraint::Check(expr))
            }
            Some(Keyword::FOREIGN) => {
                self.advance();
                self.expect_keyword(Keyword::KEY)?;
                let columns = self.parse_column_list()?;
                self.expect_keyword(Keyword::REFERENCES)?;
                let ref_table = self.parse_object_name()?;
                let ref_columns = self.parse_column_list()?;
                Ok(TableConstraint::ForeignKey {
                    columns,
                    ref_table,
                    ref_columns,
                })
            }
            _ => Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "table constraint".to_string(),
                got: format!("{:?}", self.peek()),
            }),
        }
    }

    fn parse_column_list(&mut self) -> Result<Vec<String>, ParserError> {
        self.expect_token(&Token::LParen)?;
        let mut columns = vec![self.parse_identifier()?];
        if self.match_keyword(Keyword::ASC) || self.match_keyword(Keyword::DESC) {
            self.advance();
        }
        while self.match_token(&Token::Comma) {
            self.advance();
            columns.push(self.parse_identifier()?);
            if self.match_keyword(Keyword::ASC) || self.match_keyword(Keyword::DESC) {
                self.advance();
            }
        }
        self.expect_token(&Token::RParen)?;
        Ok(columns)
    }

    // ========== ALTER TABLE ==========
}
