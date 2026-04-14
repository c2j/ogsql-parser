use crate::ast::*;
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;

impl Parser {
    pub(crate) fn parse_alter_table(&mut self) -> Result<AlterTableStatement, ParserError> {
        self.expect_keyword(Keyword::TABLE)?;

        let if_exists = self.parse_if_exists();
        let name = self.parse_object_name()?;

        let mut actions = Vec::new();
        actions.push(self.parse_alter_table_action()?);

        while self.match_token(&Token::Comma) {
            self.advance();
            actions.push(self.parse_alter_table_action()?);
        }

        Ok(AlterTableStatement {
            if_exists,
            name,
            actions,
        })
    }

    pub(crate) fn parse_if_exists(&mut self) -> bool {
        if self.match_keyword(Keyword::IF_P) {
            self.advance();
            if self.match_keyword(Keyword::EXISTS) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn parse_alter_table_action(&mut self) -> Result<AlterTableAction, ParserError> {
        match self.peek_keyword() {
            Some(Keyword::ADD_P) => {
                self.advance();
                if self.match_keyword(Keyword::PARTITION) {
                    self.advance();
                    let name = self.parse_identifier()?;
                    let values = self.parse_partition_values()?;
                    let tablespace = if self.match_keyword(Keyword::TABLESPACE) {
                        self.advance();
                        Some(self.parse_identifier()?)
                    } else {
                        None
                    };
                    Ok(AlterTableAction::AddPartition {
                        name,
                        values,
                        tablespace,
                    })
                } else if self.match_keyword(Keyword::SUBPARTITION) {
                    self.advance();
                    let name = self.parse_identifier()?;
                    let values = if self.match_keyword(Keyword::VALUES) {
                        Some(self.parse_partition_values()?)
                    } else {
                        None
                    };
                    Ok(AlterTableAction::AddSubPartition {
                        partition_name: String::new(),
                        name,
                        values,
                    })
                } else if self.match_keyword(Keyword::COLUMN) {
                    self.advance();
                    if self.match_keyword(Keyword::IF_P) {
                        self.advance();
                        if self.match_keyword(Keyword::NOT) {
                            self.advance();
                            if self.match_keyword(Keyword::EXISTS) {
                                self.advance();
                            }
                        }
                    }
                    if self.match_keyword(Keyword::CONSTRAINT)
                        || self.match_keyword(Keyword::PRIMARY)
                        || self.match_keyword(Keyword::UNIQUE)
                        || self.match_keyword(Keyword::CHECK)
                        || self.match_keyword(Keyword::FOREIGN)
                    {
                        let name = if self.match_keyword(Keyword::CONSTRAINT) {
                            self.advance();
                            Some(self.parse_identifier()?)
                        } else {
                            None
                        };
                        let constraint = self.parse_table_constraint()?;
                        Ok(AlterTableAction::AddConstraint { name, constraint })
                    } else {
                        let col = self.parse_column_def()?;
                        Ok(AlterTableAction::AddColumn(col))
                    }
                } else {
                    if self.match_keyword(Keyword::IF_P) {
                        self.advance();
                        if self.match_keyword(Keyword::NOT) {
                            self.advance();
                            if self.match_keyword(Keyword::EXISTS) {
                                self.advance();
                            }
                        }
                    }
                    if self.match_keyword(Keyword::CONSTRAINT)
                        || self.match_keyword(Keyword::PRIMARY)
                        || self.match_keyword(Keyword::UNIQUE)
                        || self.match_keyword(Keyword::CHECK)
                        || self.match_keyword(Keyword::FOREIGN)
                    {
                        let name = if self.match_keyword(Keyword::CONSTRAINT) {
                            self.advance();
                            Some(self.parse_identifier()?)
                        } else {
                            None
                        };
                        let constraint = self.parse_table_constraint()?;
                        Ok(AlterTableAction::AddConstraint { name, constraint })
                    } else {
                        let col = self.parse_column_def()?;
                        Ok(AlterTableAction::AddColumn(col))
                    }
                }
            }
            Some(Keyword::DROP) => {
                self.advance();
                if self.match_keyword(Keyword::PARTITION) {
                    self.advance();
                    let if_exists = self.parse_if_exists();
                    if self.match_keyword(Keyword::FOR) {
                        self.advance();
                        self.expect_token(&Token::LParen)?;
                        let expr = self.parse_expr()?;
                        self.expect_token(&Token::RParen)?;
                        Ok(AlterTableAction::DropPartitionFor { expr, if_exists })
                    } else {
                        let name = self.parse_identifier()?;
                        Ok(AlterTableAction::DropPartition { name, if_exists })
                    }
                } else if self.match_keyword(Keyword::SUBPARTITION) {
                    self.advance();
                    let if_exists = self.parse_if_exists();
                    let name = self.parse_identifier()?;
                    Ok(AlterTableAction::DropSubPartition { name, if_exists })
                } else if self.match_keyword(Keyword::COLUMN) {
                    self.advance();
                    let if_exists = self.parse_if_exists();
                    let name = self.parse_identifier()?;
                    let cascade = self.try_consume_keyword(Keyword::CASCADE);
                    Ok(AlterTableAction::DropColumn {
                        name,
                        if_exists,
                        cascade,
                    })
                } else if self.match_keyword(Keyword::CONSTRAINT) {
                    self.advance();
                    let if_exists = self.parse_if_exists();
                    let name = self.parse_identifier()?;
                    let cascade = self.try_consume_keyword(Keyword::CASCADE);
                    Ok(AlterTableAction::DropConstraint {
                        name,
                        if_exists,
                        cascade,
                    })
                } else {
                    Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "COLUMN or CONSTRAINT".to_string(),
                        got: format!("{:?}", self.peek()),
                    })
                }
            }
            Some(Keyword::ALTER) => {
                self.advance();
                if self.match_keyword(Keyword::COLUMN) {
                    self.advance();
                }
                let name = self.parse_identifier()?;
                let action = self.parse_alter_column_action()?;
                Ok(AlterTableAction::AlterColumn { name, action })
            }
            Some(Keyword::RENAME) => {
                self.advance();
                if self.match_keyword(Keyword::SUBPARTITION) {
                    self.advance();
                    let old_name = self.parse_identifier()?;
                    self.expect_keyword(Keyword::TO)?;
                    let new_name = self.parse_identifier()?;
                    Ok(AlterTableAction::RenameSubPartition { old_name, new_name })
                } else if self.match_keyword(Keyword::PARTITION) {
                    self.advance();
                    if self.match_keyword(Keyword::FOR) {
                        self.advance();
                        self.expect_token(&Token::LParen)?;
                        let expr = self.parse_expr()?;
                        self.expect_token(&Token::RParen)?;
                        self.expect_keyword(Keyword::TO)?;
                        let new_name = self.parse_identifier()?;
                        Ok(AlterTableAction::RenamePartitionFor { expr, new_name })
                    } else {
                        let old_name = self.parse_identifier()?;
                        self.expect_keyword(Keyword::TO)?;
                        let new_name = self.parse_identifier()?;
                        Ok(AlterTableAction::RenamePartition { old_name, new_name })
                    }
                } else if self.match_keyword(Keyword::COLUMN) {
                    self.advance();
                    let old = self.parse_identifier()?;
                    self.expect_keyword(Keyword::TO)?;
                    let new = self.parse_identifier()?;
                    Ok(AlterTableAction::RenameColumn { old, new })
                } else if self.match_keyword(Keyword::CONSTRAINT) {
                    self.advance();
                    let _old = self.parse_identifier()?;
                    self.expect_keyword(Keyword::TO)?;
                    let _new = self.parse_identifier()?;
                    Ok(AlterTableAction::DropConstraint {
                        name: String::new(),
                        if_exists: false,
                        cascade: false,
                    })
                } else {
                    self.expect_keyword(Keyword::TO)?;
                    let new_name = self.parse_identifier()?;
                    Ok(AlterTableAction::RenameTo { new_name })
                }
            }
            Some(Keyword::OWNER) => {
                self.advance();
                self.expect_keyword(Keyword::TO)?;
                let owner = self.parse_identifier()?;
                Ok(AlterTableAction::OwnerTo { owner })
            }
            Some(Keyword::SET) => {
                self.advance();
                if self.match_keyword(Keyword::SCHEMA) {
                    self.advance();
                    let schema = self.parse_identifier()?;
                    Ok(AlterTableAction::SetSchema { schema })
                } else if self.match_keyword(Keyword::TABLESPACE) {
                    self.advance();
                    let tablespace = self.parse_identifier()?;
                    Ok(AlterTableAction::SetTablespace { tablespace })
                } else if self.match_keyword(Keyword::WITHOUT) {
                    self.advance();
                    self.expect_keyword(Keyword::OIDS)?;
                    Ok(AlterTableAction::SetWithoutOids)
                } else if self.match_token(&Token::LParen) {
                    self.advance();
                    let mut options = Vec::new();
                    loop {
                        let key = self.parse_identifier()?;
                        self.expect_token(&Token::Eq)?;
                        let value = self.parse_identifier()?;
                        options.push((key, value));
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                        self.advance();
                    }
                    self.expect_token(&Token::RParen)?;
                    Ok(AlterTableAction::SetOptions { options })
                } else {
                    Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "SCHEMA, TABLESPACE, WITHOUT OIDS, or (...)".to_string(),
                        got: format!("{:?}", self.peek()),
                    })
                }
            }
            Some(Keyword::TRUNCATE) => {
                self.advance();
                if self.match_keyword(Keyword::SUBPARTITION) {
                    self.advance();
                    let name = self.parse_identifier()?;
                    let cascade = self.try_consume_keyword(Keyword::CASCADE);
                    Ok(AlterTableAction::TruncateSubPartition { name, cascade })
                } else {
                    self.expect_keyword(Keyword::PARTITION)?;
                    let name = self.parse_identifier()?;
                    let cascade = self.try_consume_keyword(Keyword::CASCADE);
                    Ok(AlterTableAction::TruncatePartition { name, cascade })
                }
            }
            Some(Keyword::MERGE) => {
                self.advance();
                if self.match_keyword(Keyword::SUBPARTITIONS) {
                    self.advance();
                    let mut names = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        names.push(self.parse_identifier()?);
                    }
                    self.expect_keyword(Keyword::INTO)?;
                    self.expect_keyword(Keyword::SUBPARTITION)?;
                    let into_name = self.parse_identifier()?;
                    Ok(AlterTableAction::MergeSubPartitions { names, into_name })
                } else {
                    self.expect_keyword(Keyword::PARTITIONS)?;
                    let mut names = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        names.push(self.parse_identifier()?);
                    }
                    self.expect_keyword(Keyword::INTO)?;
                    self.expect_keyword(Keyword::PARTITION)?;
                    let into_name = self.parse_identifier()?;
                    Ok(AlterTableAction::MergePartitions { names, into_name })
                }
            }
            Some(Keyword::SPLIT) => {
                self.advance();
                if self.match_keyword(Keyword::SUBPARTITION) {
                    self.advance();
                    let name = self.parse_identifier()?;
                    let at_value = if self.match_keyword(Keyword::AT) {
                        self.advance();
                        Some(self.parse_expr()?)
                    } else {
                        None
                    };
                    self.expect_keyword(Keyword::INTO)?;
                    self.expect_token(&Token::LParen)?;
                    let mut partitions = Vec::new();
                    loop {
                        self.expect_keyword(Keyword::SUBPARTITION)?;
                        let pname = self.parse_identifier()?;
                        let values = if self.match_keyword(Keyword::VALUES) {
                            Some(self.parse_partition_values()?)
                        } else {
                            None
                        };
                        let tablespace = if self.match_keyword(Keyword::TABLESPACE) {
                            self.advance();
                            Some(self.parse_identifier()?)
                        } else {
                            None
                        };
                        partitions.push(PartitionDef {
                            name: pname,
                            values,
                            tablespace,
                            subpartitions: Vec::new(),
                        });
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                        self.advance();
                    }
                    self.expect_token(&Token::RParen)?;
                    Ok(AlterTableAction::SplitSubPartition {
                        name,
                        at_value,
                        into: partitions,
                    })
                } else {
                    self.expect_keyword(Keyword::PARTITION)?;
                    let name = self.parse_identifier()?;
                    let at_value = if self.match_keyword(Keyword::AT) {
                        self.advance();
                        Some(self.parse_expr()?)
                    } else {
                        None
                    };
                    self.expect_keyword(Keyword::INTO)?;
                    self.expect_token(&Token::LParen)?;
                    let mut partitions = Vec::new();
                    loop {
                        self.expect_keyword(Keyword::PARTITION)?;
                        let pname = self.parse_identifier()?;
                        let values = if self.match_keyword(Keyword::VALUES) {
                            Some(self.parse_partition_values()?)
                        } else {
                            None
                        };
                        partitions.push(PartitionDef {
                            name: pname,
                            values,
                            tablespace: None,
                            subpartitions: Vec::new(),
                        });
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                        self.advance();
                    }
                    self.expect_token(&Token::RParen)?;
                    Ok(AlterTableAction::SplitPartition {
                        name,
                        at_value,
                        into: partitions,
                    })
                }
            }
            Some(Keyword::EXCHANGE) => {
                self.advance();
                if self.match_keyword(Keyword::SUBPARTITION) {
                    self.advance();
                    let name = self.parse_identifier()?;
                    self.expect_keyword(Keyword::WITH)?;
                    self.expect_keyword(Keyword::TABLE)?;
                    let table = self.parse_object_name()?;
                    Ok(AlterTableAction::ExchangeSubPartition { name, table })
                } else {
                    self.expect_keyword(Keyword::PARTITION)?;
                    // Handle both PARTITION name and PARTITION (name) forms
                    let name = if self.match_token(&Token::LParen) {
                        self.advance();
                        let n = self.parse_identifier()?;
                        self.expect_token(&Token::RParen)?;
                        n
                    } else {
                        self.parse_identifier()?
                    };
                    self.expect_keyword(Keyword::WITH)?;
                    self.expect_keyword(Keyword::TABLE)?;
                    let table = self.parse_object_name()?;
                    Ok(AlterTableAction::ExchangePartition { name, table })
                }
            }
            Some(Keyword::RESET) => {
                self.advance();
                if self.match_token(&Token::LParen) {
                    self.advance();
                    let mut options = Vec::new();
                    loop {
                        options.push(self.parse_identifier()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                        self.advance();
                    }
                    self.expect_token(&Token::RParen)?;
                    Ok(AlterTableAction::ResetOptions { options })
                } else {
                    Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "(option, ...)".to_string(),
                        got: format!("{:?}", self.peek()),
                    })
                }
            }
            Some(Keyword::MOVE) => {
                self.advance();
                if self.match_keyword(Keyword::SUBPARTITION) {
                    self.advance();
                    let name = self.parse_identifier()?;
                    self.expect_keyword(Keyword::TABLESPACE)?;
                    let tablespace = self.parse_identifier()?;
                    Ok(AlterTableAction::MoveSubPartition { name, tablespace })
                } else if self.match_keyword(Keyword::PARTITION) {
                    self.advance();
                    let name = self.parse_identifier()?;
                    self.expect_keyword(Keyword::TABLESPACE)?;
                    let tablespace = self.parse_identifier()?;
                    Ok(AlterTableAction::MovePartition { name, tablespace })
                } else {
                    Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "PARTITION or SUBPARTITION".to_string(),
                        got: format!("{:?}", self.peek()),
                    })
                }
            }
            Some(Keyword::MODIFY_P) => {
                self.advance();
                let name = self.parse_identifier()?;
                let action = if self.match_keyword(Keyword::NOT) {
                    self.advance();
                    self.expect_keyword(Keyword::NULL_P)?;
                    AlterColumnAction::SetNotNull
                } else if self.match_keyword(Keyword::NULL_P) {
                    self.advance();
                    AlterColumnAction::DropNotNull
                } else {
                    let data_type = self.parse_data_type()?;
                    AlterColumnAction::SetDataType(data_type)
                };
                Ok(AlterTableAction::AlterColumn { name, action })
            }
            Some(Keyword::ENABLE_P) => {
                self.advance();
                if self.match_keyword(Keyword::ROW) {
                    self.advance();
                    self.expect_keyword(Keyword::LEVEL)?;
                    self.expect_keyword(Keyword::SECURITY)?;
                }
                Ok(AlterTableAction::EnableRowLevelSecurity)
            }
            Some(Keyword::DISABLE_P) => {
                self.advance();
                if self.match_keyword(Keyword::ROW) {
                    self.advance();
                    self.expect_keyword(Keyword::LEVEL)?;
                    self.expect_keyword(Keyword::SECURITY)?;
                }
                Ok(AlterTableAction::DisableRowLevelSecurity)
            }
            Some(Keyword::CHARSET) | Some(Keyword::CHARACTER) => {
                self.advance();
                if self.match_keyword(Keyword::SET) {
                    self.advance();
                }
                let charset = self.parse_identifier()?;
                let collation = if self.match_keyword(Keyword::COLLATE)
                    || self.match_ident_str("collate")
                    || self.match_ident_str("COLLATION")
                {
                    self.advance();
                    Some(self.parse_identifier()?)
                } else {
                    None
                };
                Ok(AlterTableAction::SetCharset { charset, collation })
            }
            _ => Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "ALTER TABLE action".to_string(),
                got: format!("{:?}", self.peek()),
            }),
        }
    }

    fn parse_partition_values(&mut self) -> Result<PartitionValues, ParserError> {
        self.expect_keyword(Keyword::VALUES)?;
        if self.match_keyword(Keyword::LESS) {
            self.advance();
            self.expect_keyword(Keyword::THAN)?;
            self.expect_token(&Token::LParen)?;
            let mut vals = Vec::new();
            if !self.match_token(&Token::RParen) {
                vals.push(self.parse_expr()?);
                while self.match_token(&Token::Comma) {
                    self.advance();
                    vals.push(self.parse_expr()?);
                }
                self.expect_token(&Token::RParen)?;
            }
            Ok(PartitionValues::LessThan(vals))
        } else if self.match_keyword(Keyword::IN_P) {
            self.advance();
            self.expect_token(&Token::LParen)?;
            let mut vals = Vec::new();
            if !self.match_token(&Token::RParen) {
                vals.push(self.parse_expr()?);
                while self.match_token(&Token::Comma) {
                    self.advance();
                    vals.push(self.parse_expr()?);
                }
                self.expect_token(&Token::RParen)?;
            }
            Ok(PartitionValues::InValues(vals))
        } else if self.match_keyword(Keyword::START) {
            self.advance();
            self.expect_token(&Token::LParen)?;
            let start = self.parse_expr()?;
            self.expect_token(&Token::RParen)?;
            self.expect_keyword(Keyword::END_P)?;
            self.expect_token(&Token::LParen)?;
            let end = self.parse_expr()?;
            self.expect_token(&Token::RParen)?;
            Ok(PartitionValues::StartEnd { start, end })
        } else if self.match_token(&Token::LParen) {
            self.advance();
            let mut vals = Vec::new();
            if !self.match_token(&Token::RParen) {
                vals.push(self.parse_expr()?);
                while self.match_token(&Token::Comma) {
                    self.advance();
                    vals.push(self.parse_expr()?);
                }
                self.expect_token(&Token::RParen)?;
            }
            Ok(PartitionValues::InValues(vals))
        } else {
            Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "LESS THAN, IN, or START".to_string(),
                got: format!("{:?}", self.peek()),
            })
        }
    }

    pub(crate) fn parse_partition_defs(&mut self) -> Result<Vec<PartitionDef>, ParserError> {
        if !self.match_token(&Token::LParen) {
            return Ok(Vec::new());
        }
        self.advance();
        let mut defs = Vec::new();
        loop {
            self.expect_keyword(Keyword::PARTITION)?;
            let name = self.parse_identifier()?;
            let values = if self.match_keyword(Keyword::VALUES) {
                Some(self.parse_partition_values()?)
            } else {
                None
            };
            let tablespace = if self.match_keyword(Keyword::TABLESPACE) {
                self.advance();
                Some(self.parse_identifier()?)
            } else {
                None
            };
            let subpartitions = self.parse_subpartition_defs()?;
            if self.match_ident_str("ILM") {
                self.advance();
                while !self.match_token(&Token::Comma)
                    && !self.match_token(&Token::RParen)
                    && !self.peek().eq(&Token::Eof)
                {
                    self.advance();
                }
            }
            defs.push(PartitionDef {
                name,
                values,
                tablespace,
                subpartitions,
            });
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }
        self.expect_token(&Token::RParen)?;
        Ok(defs)
    }

    pub(crate) fn parse_subpartition_defs(&mut self) -> Result<Vec<PartitionDef>, ParserError> {
        if !self.match_token(&Token::LParen) {
            return Ok(Vec::new());
        }
        // Peek inside: if starts with PARTITION (not SUBPARTITION), these are partition defs, not subpartition defs
        if self.pos + 1 < self.tokens.len()
            && matches!(
                self.tokens[self.pos + 1].token,
                Token::Keyword(Keyword::PARTITION)
            )
            && !matches!(
                self.tokens[self.pos + 1].token,
                Token::Keyword(Keyword::SUBPARTITION)
            )
        {
            return Ok(Vec::new());
        }
        self.advance();
        let mut defs = Vec::new();
        loop {
            self.expect_keyword(Keyword::SUBPARTITION)?;
            let name = self.parse_identifier()?;
            let values = if self.match_keyword(Keyword::VALUES) {
                Some(self.parse_partition_values()?)
            } else {
                None
            };
            let tablespace = if self.match_keyword(Keyword::TABLESPACE) {
                self.advance();
                Some(self.parse_identifier()?)
            } else {
                None
            };
            defs.push(PartitionDef {
                name,
                values,
                tablespace,
                subpartitions: Vec::new(),
            });
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }
        self.expect_token(&Token::RParen)?;
        Ok(defs)
    }

    fn parse_alter_column_action(&mut self) -> Result<AlterColumnAction, ParserError> {
        match self.peek_keyword() {
            Some(Keyword::TYPE_P) | Some(Keyword::SET) => {
                if self.match_keyword(Keyword::TYPE_P) {
                    self.advance();
                    let data_type = self.parse_data_type()?;
                    Ok(AlterColumnAction::SetDataType(data_type))
                } else {
                    self.advance();
                    if self.match_keyword(Keyword::DATA_P) {
                        self.advance();
                        self.expect_keyword(Keyword::TYPE_P)?;
                        let data_type = self.parse_data_type()?;
                        Ok(AlterColumnAction::SetDataType(data_type))
                    } else if self.match_keyword(Keyword::DEFAULT) {
                        self.advance();
                        let expr = self.parse_expr()?;
                        Ok(AlterColumnAction::SetDefault(expr))
                    } else if self.match_keyword(Keyword::NOT) {
                        self.advance();
                        self.expect_keyword(Keyword::NULL_P)?;
                        Ok(AlterColumnAction::SetNotNull)
                    } else {
                        Err(ParserError::UnexpectedToken {
                            location: self.current_location(),
                            expected: "ALTER COLUMN SET option".to_string(),
                            got: format!("{:?}", self.peek()),
                        })
                    }
                }
            }
            Some(Keyword::DROP) => {
                self.advance();
                if self.match_keyword(Keyword::DEFAULT) {
                    self.advance();
                    Ok(AlterColumnAction::DropDefault)
                } else if self.match_keyword(Keyword::NOT) {
                    self.advance();
                    self.expect_keyword(Keyword::NULL_P)?;
                    Ok(AlterColumnAction::DropNotNull)
                } else {
                    Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "DROP DEFAULT or DROP NOT NULL".to_string(),
                        got: format!("{:?}", self.peek()),
                    })
                }
            }
            _ => Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "ALTER COLUMN action".to_string(),
                got: format!("{:?}", self.peek()),
            }),
        }
    }

    // ========== DROP ==========
}
