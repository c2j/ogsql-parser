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

        loop {
            if self.match_keyword(Keyword::CONSTRAINT)
                || self.match_keyword(Keyword::PRIMARY)
                || self.match_keyword(Keyword::UNIQUE)
                || self.match_keyword(Keyword::CHECK)
                || self.match_keyword(Keyword::FOREIGN)
            {
                constraints.push(self.parse_table_constraint()?);
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
            } else if self.match_ident_str("ILM") {
                self.advance();
                while !self.match_keyword(Keyword::PARTITION)
                    && !self.match_keyword(Keyword::WITH)
                    && !self.match_token(&Token::Semicolon)
                    && !self.match_token(&Token::RParen)
                    && !self.peek().eq(&Token::Eof)
                {
                    self.advance();
                }
            } else if self.match_keyword(Keyword::PARTITION) {
                self.advance();
                self.expect_keyword(Keyword::BY)?;
                let strategy = match self.peek() {
                    Token::Ident(s) if s.to_uppercase() == "HASH" => {
                        self.advance();
                        "hash"
                    }
                    _ => match self.peek_keyword() {
                        Some(Keyword::RANGE) => {
                            self.advance();
                            "range"
                        }
                        Some(Keyword::LIST) => {
                            self.advance();
                            "list"
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
                let column = self.parse_object_name()?;
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
                        let parts = self.parse_partition_defs()?;
                        (interval, parts, None)
                    }
                    "list" => {
                        let parts = self.parse_partition_defs()?;
                        (None, parts, None)
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
                        column,
                        interval,
                        partitions,
                    },
                    "list" => PartitionClause::List { column, partitions },
                    _ => PartitionClause::Hash {
                        column,
                        partitions_count,
                        partitions,
                    },
                });
            } else if self.match_keyword(Keyword::SUBPARTITION) {
                self.advance();
                self.expect_keyword(Keyword::BY)?;
                let strategy = match self.peek() {
                    Token::Ident(s) if s.to_uppercase() == "HASH" => {
                        self.advance();
                        "hash"
                    }
                    _ => match self.peek_keyword() {
                        Some(Keyword::RANGE) => {
                            self.advance();
                            "range"
                        }
                        Some(Keyword::LIST) => {
                            self.advance();
                            "list"
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
                let column = self.parse_object_name()?;
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
                        column,
                        interval: None,
                        partitions: sp_parts,
                    },
                    "list" => PartitionClause::List {
                        column,
                        partitions: sp_parts,
                    },
                    _ => PartitionClause::Hash {
                        column,
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
                                    column, interval, ..
                                } => PartitionClause::Range {
                                    column,
                                    interval,
                                    partitions: parts,
                                },
                                PartitionClause::List { column, .. } => PartitionClause::List {
                                    column,
                                    partitions: parts,
                                },
                                PartitionClause::Hash {
                                    column,
                                    partitions_count,
                                    ..
                                } => PartitionClause::Hash {
                                    column,
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
                            format!("{:?}", kw)
                                .to_lowercase()
                                .trim_end_matches("_p")
                                .to_string()
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
            } else if self.match_keyword(Keyword::TO) {
                self.advance();
                self.expect_keyword(Keyword::GROUP_P)?;
                to_group = Some(self.parse_identifier()?);
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
            inherits,
            partition_by,
            subpartition_by,
            subpartitions_count,
            distribute_by,
            to_group,
            tablespace,
            on_commit,
            options,
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
                from: vec![TableRef::Table {
                    name: table_name.clone(),
                    alias: None,
                }],
                where_clause: None,
                connect_by: None,
                group_by: vec![],
                having: None,
                order_by: vec![],
                limit: None,
                offset: None,
                fetch: None,
                lock_clause: None,
                set_operation: None,
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
                column_names: Vec::new(),
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

    pub(crate) fn parse_column_def(&mut self) -> Result<ColumnDef, ParserError> {
        let name = self.parse_identifier()?;
        let data_type = self.parse_data_type()?;

        let mut constraints = Vec::new();
        while let Some(constraint) = self.try_parse_column_constraint()? {
            constraints.push(constraint);
        }

        Ok(ColumnDef {
            name,
            data_type,
            constraints,
        })
    }

    pub(crate) fn parse_data_type(&mut self) -> Result<DataType, ParserError> {
        match self.peek_keyword() {
            Some(Keyword::BOOLEAN_P) => {
                self.advance();
                Ok(DataType::Boolean)
            }
            Some(Keyword::TINYINT) => {
                self.advance();
                Ok(DataType::TinyInt)
            }
            Some(Keyword::SMALLINT) => {
                self.advance();
                Ok(DataType::SmallInt)
            }
            Some(Keyword::INTEGER) | Some(Keyword::INT_P) => {
                self.advance();
                Ok(DataType::Integer)
            }
            Some(Keyword::BIGINT) => {
                self.advance();
                Ok(DataType::BigInt)
            }
            Some(Keyword::REAL) => {
                self.advance();
                Ok(DataType::Real)
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
                Ok(DataType::Float(precision))
            }
            Some(Keyword::DOUBLE_P) => {
                self.advance();
                if self.match_keyword(Keyword::PRECISION) {
                    self.advance();
                }
                Ok(DataType::Double)
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
                Ok(DataType::Numeric(precision, scale))
            }
            Some(Keyword::CHAR_P) => {
                self.advance();
                let len = if self.match_token(&Token::LParen) {
                    self.advance();
                    let n = self.parse_int_literal()?;
                    self.expect_token(&Token::RParen)?;
                    Some(n)
                } else {
                    None
                };
                Ok(DataType::Char(len))
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
                Ok(DataType::Varchar(len))
            }
            Some(Keyword::TEXT_P) => {
                self.advance();
                Ok(DataType::Text)
            }
            Some(Keyword::BYTE_P) => {
                self.advance();
                Ok(DataType::Bytea)
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
                Ok(DataType::Timestamp(precision, tz))
            }
            Some(Keyword::DATE_P) => {
                self.advance();
                Ok(DataType::Date)
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
                Ok(DataType::Time(precision, tz))
            }
            Some(Keyword::INTERVAL) => {
                self.advance();
                Ok(DataType::Interval)
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
                    Ok(DataType::Varbit(len))
                } else {
                    let len = if self.match_token(&Token::LParen) {
                        self.advance();
                        let n = self.parse_int_literal()?;
                        self.expect_token(&Token::RParen)?;
                        Some(n)
                    } else {
                        None
                    };
                    Ok(DataType::Bit(len))
                }
            }
            _ => {
                if let Token::Ident(s) = self.peek().clone() {
                    match s.to_uppercase().as_str() {
                        "SERIAL" => {
                            self.advance();
                            return Ok(DataType::Serial);
                        }
                        "SMALLSERIAL" => {
                            self.advance();
                            return Ok(DataType::SmallSerial);
                        }
                        "BIGSERIAL" => {
                            self.advance();
                            return Ok(DataType::BigSerial);
                        }
                        "BINARY_FLOAT" => {
                            self.advance();
                            return Ok(DataType::BinaryFloat);
                        }
                        "BINARY_DOUBLE" => {
                            self.advance();
                            return Ok(DataType::BinaryDouble);
                        }
                        "BOOL" => {
                            self.advance();
                            return Ok(DataType::Boolean);
                        }
                        _ => {}
                    }
                }
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
                Ok(DataType::Custom(name, args))
            }
        }
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

    fn try_parse_column_constraint(&mut self) -> Result<Option<ColumnConstraint>, ParserError> {
        match self.peek_keyword() {
            Some(Keyword::NOT) => {
                self.advance();
                self.expect_keyword(Keyword::NULL_P)?;
                Ok(Some(ColumnConstraint::NotNull))
            }
            Some(Keyword::NULL_P) => {
                self.advance();
                Ok(Some(ColumnConstraint::Null))
            }
            Some(Keyword::DEFAULT) => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(Some(ColumnConstraint::Default(expr)))
            }
            Some(Keyword::UNIQUE) => {
                self.advance();
                Ok(Some(ColumnConstraint::Unique))
            }
            Some(Keyword::PRIMARY) => {
                self.advance();
                self.expect_keyword(Keyword::KEY)?;
                Ok(Some(ColumnConstraint::PrimaryKey))
            }
            Some(Keyword::CHECK) => {
                self.advance();
                self.expect_token(&Token::LParen)?;
                let expr = self.parse_expr()?;
                self.expect_token(&Token::RParen)?;
                Ok(Some(ColumnConstraint::Check(expr)))
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
                Ok(Some(ColumnConstraint::References(table, columns)))
            }
            _ => Ok(None),
        }
    }

    pub(crate) fn parse_table_constraint(&mut self) -> Result<TableConstraint, ParserError> {
        if self.match_keyword(Keyword::CONSTRAINT) {
            self.advance();
            let _name = self.parse_identifier()?;
        }

        match self.peek_keyword() {
            Some(Keyword::PRIMARY) => {
                self.advance();
                self.expect_keyword(Keyword::KEY)?;
                let columns = self.parse_column_list()?;
                Ok(TableConstraint::PrimaryKey(columns))
            }
            Some(Keyword::UNIQUE) => {
                self.advance();
                let columns = self.parse_column_list()?;
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
        while self.match_token(&Token::Comma) {
            self.advance();
            columns.push(self.parse_identifier()?);
        }
        self.expect_token(&Token::RParen)?;
        Ok(columns)
    }

    // ========== ALTER TABLE ==========
}
