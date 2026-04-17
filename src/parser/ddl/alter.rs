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
                if self.match_token(&Token::LParen) {
                    self.advance();
                    let mut cols = Vec::new();
                    loop {
                        cols.push(self.parse_column_def()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                        self.advance();
                    }
                    self.expect_token(&Token::RParen)?;
                    Ok(AlterTableAction::AddColumns(cols))
                } else if self.match_keyword(Keyword::STATISTICS) {
                    self.advance();
                    self.parse_statistics_columns(StatisticsOpKind::Add)
                } else if self.match_keyword(Keyword::PARTITION) {
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
                        // UNIQUE USING INDEX
                        if self.peek_keyword() == Some(Keyword::UNIQUE)
                            && self.tokens.get(self.pos + 1).map_or(false, |t| {
                                matches!(t.token, Token::Keyword(Keyword::USING))
                            })
                        {
                            self.advance();
                            self.advance();
                            self.expect_keyword(Keyword::INDEX)?;
                            let index_name = self.parse_identifier()?;
                            return Ok(AlterTableAction::AddConstraintUsingIndex {
                                name: name.clone().unwrap_or_default(),
                                index_name,
                            });
                        }
                        // PRIMARY KEY USING INDEX
                        if self.peek_keyword() == Some(Keyword::PRIMARY)
                            && self
                                .tokens
                                .get(self.pos + 1)
                                .map_or(false, |t| matches!(t.token, Token::Keyword(Keyword::KEY)))
                            && self.tokens.get(self.pos + 2).map_or(false, |t| {
                                matches!(t.token, Token::Keyword(Keyword::USING))
                            })
                        {
                            self.advance();
                            self.advance();
                            self.advance();
                            self.expect_keyword(Keyword::INDEX)?;
                            let index_name = self.parse_identifier()?;
                            return Ok(AlterTableAction::AddConstraintUsingIndex {
                                name: name.clone().unwrap_or_default(),
                                index_name,
                            });
                        }
                        let constraint = self.parse_table_constraint()?;
                        Ok(AlterTableAction::AddConstraint { name, constraint })
                    } else {
                        let col = self.parse_column_def()?;
                        Ok(AlterTableAction::AddColumn(col))
                    }
                } else if self.match_keyword(Keyword::NODE) {
                    self.advance();
                    let node_name = self.parse_identifier()?;
                    Ok(AlterTableAction::AddNode { node_name })
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
                        // UNIQUE USING INDEX
                        if self.peek_keyword() == Some(Keyword::UNIQUE)
                            && self.tokens.get(self.pos + 1).map_or(false, |t| {
                                matches!(t.token, Token::Keyword(Keyword::USING))
                            })
                        {
                            self.advance();
                            self.advance();
                            self.expect_keyword(Keyword::INDEX)?;
                            let index_name = self.parse_identifier()?;
                            return Ok(AlterTableAction::AddConstraintUsingIndex {
                                name: name.clone().unwrap_or_default(),
                                index_name,
                            });
                        }
                        // PRIMARY KEY USING INDEX
                        if self.peek_keyword() == Some(Keyword::PRIMARY)
                            && self
                                .tokens
                                .get(self.pos + 1)
                                .map_or(false, |t| matches!(t.token, Token::Keyword(Keyword::KEY)))
                            && self.tokens.get(self.pos + 2).map_or(false, |t| {
                                matches!(t.token, Token::Keyword(Keyword::USING))
                            })
                        {
                            self.advance();
                            self.advance();
                            self.advance();
                            self.expect_keyword(Keyword::INDEX)?;
                            let index_name = self.parse_identifier()?;
                            return Ok(AlterTableAction::AddConstraintUsingIndex {
                                name: name.clone().unwrap_or_default(),
                                index_name,
                            });
                        }
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
                        let (update_global_index, update_distributed_global_index) =
                            self.parse_update_index_clauses()?;
                        Ok(AlterTableAction::DropPartitionFor {
                            expr,
                            if_exists,
                            update_global_index,
                            update_distributed_global_index,
                        })
                    } else {
                        let name = self.parse_identifier()?;
                        let (update_global_index, update_distributed_global_index) =
                            self.parse_update_index_clauses()?;
                        Ok(AlterTableAction::DropPartition {
                            name,
                            if_exists,
                            update_global_index,
                            update_distributed_global_index,
                        })
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
                } else if self.match_keyword(Keyword::NODE) {
                    self.advance();
                    let node_name = self.parse_identifier()?;
                    Ok(AlterTableAction::DeleteNode { node_name })
                } else {
                    Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "COLUMN, CONSTRAINT, or NODE".to_string(),
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
                if self.match_keyword(Keyword::SET) {
                    if self.tokens.get(self.pos + 1).map_or(false, |t| {
                        matches!(t.token, Token::Keyword(Keyword::STATISTICS))
                    }) {
                        self.advance();
                        self.expect_keyword(Keyword::STATISTICS)?;
                        let percent = self.try_consume_keyword(Keyword::PERCENT);
                        let value = self.parse_integer_literal()?;
                        return Ok(AlterTableAction::AlterColumnStatistics {
                            column: name,
                            percent,
                            value,
                        });
                    }
                    if self.tokens.get(self.pos + 1).map_or(false, |t| {
                        matches!(t.token, Token::Keyword(Keyword::STORAGE))
                    }) {
                        self.advance();
                        self.expect_keyword(Keyword::STORAGE)?;
                        let storage = self.parse_identifier()?;
                        return Ok(AlterTableAction::AlterColumnStorage {
                            column: name,
                            storage,
                        });
                    }
                }
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
                } else if self.match_keyword(Keyword::COMPRESS) {
                    self.advance();
                    Ok(AlterTableAction::SetCompress)
                } else if self.match_keyword(Keyword::WITHOUT) {
                    self.advance();
                    if self.match_keyword(Keyword::CLUSTER) {
                        self.advance();
                        Ok(AlterTableAction::SetWithoutCluster)
                    } else {
                        self.expect_keyword(Keyword::OIDS)?;
                        Ok(AlterTableAction::SetWithoutOids)
                    }
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
                        expected: "SCHEMA, TABLESPACE, COMPRESS, WITHOUT OIDS, or (...)"
                            .to_string(),
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
                    let (update_global_index, update_distributed_global_index) =
                        self.parse_update_index_clauses()?;
                    Ok(AlterTableAction::TruncatePartition {
                        name,
                        cascade,
                        update_global_index,
                        update_distributed_global_index,
                    })
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
                    let (update_global_index, update_distributed_global_index) =
                        self.parse_update_index_clauses()?;
                    Ok(AlterTableAction::MergePartitions {
                        names,
                        into_name,
                        update_global_index,
                        update_distributed_global_index,
                    })
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
                    if self.match_keyword(Keyword::FOR) {
                        self.advance();
                        self.expect_token(&Token::LParen)?;
                        let expr = self.parse_expr()?;
                        self.expect_token(&Token::RParen)?;
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
                        let (update_global_index, update_distributed_global_index) =
                            self.parse_update_index_clauses()?;
                        Ok(AlterTableAction::SplitPartitionFor {
                            expr,
                            at_value,
                            into: partitions,
                            update_global_index,
                            update_distributed_global_index,
                        })
                    } else {
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
                        let (update_global_index, update_distributed_global_index) =
                            self.parse_update_index_clauses()?;
                        Ok(AlterTableAction::SplitPartition {
                            name,
                            at_value,
                            into: partitions,
                            update_global_index,
                            update_distributed_global_index,
                        })
                    }
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
                    let with_validation = if self.match_keyword(Keyword::WITH) {
                        self.advance();
                        self.expect_keyword(Keyword::VALIDATION)?;
                        Some(true)
                    } else if self.match_keyword(Keyword::WITHOUT) {
                        self.advance();
                        self.expect_keyword(Keyword::VALIDATION)?;
                        Some(false)
                    } else {
                        None
                    };
                    let verbose = self.try_consume_keyword(Keyword::VERBOSE);
                    let (update_global_index, update_distributed_global_index) =
                        self.parse_update_index_clauses()?;
                    Ok(AlterTableAction::ExchangePartition {
                        name,
                        table,
                        update_global_index,
                        update_distributed_global_index,
                        with_validation,
                        verbose,
                    })
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
                    if self.match_keyword(Keyword::FOR) {
                        self.advance();
                        self.expect_token(&Token::LParen)?;
                        let expr = self.parse_expr()?;
                        self.expect_token(&Token::RParen)?;
                        self.expect_keyword(Keyword::TABLESPACE)?;
                        let tablespace = self.parse_identifier()?;
                        Ok(AlterTableAction::MovePartitionFor { expr, tablespace })
                    } else {
                        let name = self.parse_identifier()?;
                        self.expect_keyword(Keyword::TABLESPACE)?;
                        let tablespace = self.parse_identifier()?;
                        Ok(AlterTableAction::MovePartition { name, tablespace })
                    }
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
                // Multi-column: MODIFY (col1 type1, col2 type2, ...)
                if self.match_token(&Token::LParen) {
                    self.advance();
                    let mut cols = Vec::new();
                    loop {
                        let name = self.parse_identifier()?;
                        let data_type = self.parse_data_type()?;
                        let nullability = if self.match_keyword(Keyword::NOT) {
                            self.advance();
                            self.expect_keyword(Keyword::NULL_P)?;
                            Some(false)
                        } else if self.match_keyword(Keyword::NULL_P) {
                            self.advance();
                            Some(true)
                        } else {
                            None
                        };
                        cols.push(ModifyColumnInfo {
                            name,
                            data_type,
                            nullability,
                        });
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                        self.advance();
                    }
                    self.expect_token(&Token::RParen)?;
                    Ok(AlterTableAction::ModifyColumns(cols))
                } else {
                    // Single-column: MODIFY colname ...
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
            }
            Some(Keyword::ENABLE_P) => {
                self.advance();
                if self.match_keyword(Keyword::ROW) {
                    self.advance();
                    if self.match_keyword(Keyword::MOVEMENT) {
                        self.advance();
                        Ok(AlterTableAction::EnableRowMovement)
                    } else {
                        self.expect_keyword(Keyword::LEVEL)?;
                        self.expect_keyword(Keyword::SECURITY)?;
                        Ok(AlterTableAction::EnableRowLevelSecurity)
                    }
                } else if self.match_keyword(Keyword::TRIGGER) {
                    self.advance();
                    let name =
                        if self.match_keyword(Keyword::ALL) || self.match_keyword(Keyword::USER) {
                            self.advance();
                            None
                        } else {
                            Some(self.parse_identifier()?)
                        };
                    Ok(AlterTableAction::EnableTrigger { name })
                } else if self.match_keyword(Keyword::RULE) {
                    self.advance();
                    let name = self.parse_identifier()?;
                    Ok(AlterTableAction::SetRule {
                        enable: true,
                        mode: None,
                        name,
                    })
                } else if self.match_keyword(Keyword::STATISTICS) {
                    self.advance();
                    self.parse_statistics_columns(StatisticsOpKind::Enable)
                } else {
                    let mode = if self.match_keyword(Keyword::REPLICA) {
                        self.advance();
                        Some("REPLICA".to_string())
                    } else if self.match_keyword(Keyword::ALWAYS) {
                        self.advance();
                        Some("ALWAYS".to_string())
                    } else {
                        None
                    };
                    if self.match_keyword(Keyword::RULE) {
                        self.advance();
                        let name = self.parse_identifier()?;
                        Ok(AlterTableAction::SetRule {
                            enable: true,
                            mode,
                            name,
                        })
                    } else {
                        Ok(AlterTableAction::EnableRowLevelSecurity)
                    }
                }
            }
            Some(Keyword::DISABLE_P) => {
                self.advance();
                if self.match_keyword(Keyword::ROW) {
                    self.advance();
                    if self.match_keyword(Keyword::MOVEMENT) {
                        self.advance();
                        Ok(AlterTableAction::DisableRowMovement)
                    } else {
                        self.expect_keyword(Keyword::LEVEL)?;
                        self.expect_keyword(Keyword::SECURITY)?;
                        Ok(AlterTableAction::DisableRowLevelSecurity)
                    }
                } else if self.match_keyword(Keyword::TRIGGER) {
                    self.advance();
                    let name =
                        if self.match_keyword(Keyword::ALL) || self.match_keyword(Keyword::USER) {
                            self.advance();
                            None
                        } else {
                            Some(self.parse_identifier()?)
                        };
                    Ok(AlterTableAction::DisableTrigger { name })
                } else if self.match_keyword(Keyword::RULE) {
                    self.advance();
                    let name = self.parse_identifier()?;
                    Ok(AlterTableAction::SetRule {
                        enable: false,
                        mode: None,
                        name,
                    })
                } else if self.match_keyword(Keyword::STATISTICS) {
                    self.advance();
                    self.parse_statistics_columns(StatisticsOpKind::Disable)
                } else {
                    let mode = if self.match_keyword(Keyword::REPLICA) {
                        self.advance();
                        Some("REPLICA".to_string())
                    } else if self.match_keyword(Keyword::ALWAYS) {
                        self.advance();
                        Some("ALWAYS".to_string())
                    } else {
                        None
                    };
                    if self.match_keyword(Keyword::RULE) {
                        self.advance();
                        let name = self.parse_identifier()?;
                        Ok(AlterTableAction::SetRule {
                            enable: false,
                            mode,
                            name,
                        })
                    } else {
                        Ok(AlterTableAction::DisableRowLevelSecurity)
                    }
                }
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
            Some(Keyword::VALIDATE) => {
                self.advance();
                self.expect_keyword(Keyword::CONSTRAINT)?;
                let name = self.parse_identifier()?;
                Ok(AlterTableAction::ValidateConstraint { name })
            }
            Some(Keyword::INHERIT) => {
                self.advance();
                let parent = self.parse_object_name()?;
                Ok(AlterTableAction::Inherit { parent })
            }
            Some(Keyword::NO) => {
                self.advance();
                if self.match_keyword(Keyword::INHERIT) {
                    self.advance();
                    let parent = self.parse_object_name()?;
                    Ok(AlterTableAction::NoInherit { parent })
                } else if self.match_keyword(Keyword::FORCE) {
                    self.advance();
                    self.expect_keyword(Keyword::ROW)?;
                    self.expect_keyword(Keyword::LEVEL)?;
                    self.expect_keyword(Keyword::SECURITY)?;
                    Ok(AlterTableAction::NoForceRowLevelSecurity)
                } else {
                    Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "INHERIT or FORCE after NO".to_string(),
                        got: format!("{:?}", self.peek()),
                    })
                }
            }
            Some(Keyword::CLUSTER) => {
                self.advance();
                self.expect_keyword(Keyword::ON)?;
                let index_name = self.parse_identifier()?;
                Ok(AlterTableAction::ClusterOn { index_name })
            }
            Some(Keyword::REPLICA) => {
                self.advance();
                self.expect_keyword(Keyword::IDENTITY_P)?;
                let identity = if self.match_keyword(Keyword::DEFAULT) {
                    ReplicaIdentity::Default
                } else if self.match_keyword(Keyword::NOTHING) {
                    ReplicaIdentity::Nothing
                } else if self.match_keyword(Keyword::FULL) {
                    ReplicaIdentity::Full
                } else if self.match_keyword(Keyword::USING) {
                    self.advance();
                    self.expect_keyword(Keyword::INDEX)?;
                    let name = self.parse_identifier()?;
                    ReplicaIdentity::Index { name }
                } else {
                    return Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "DEFAULT, NOTHING, FULL, or USING INDEX".to_string(),
                        got: format!("{:?}", self.peek()),
                    });
                };
                Ok(AlterTableAction::ReplicaIdentity(identity))
            }
            Some(Keyword::NOCOMPRESS) => {
                self.advance();
                Ok(AlterTableAction::SetNoCompress)
            }
            Some(Keyword::FORCE) => {
                self.advance();
                self.expect_keyword(Keyword::ROW)?;
                self.expect_keyword(Keyword::LEVEL)?;
                self.expect_keyword(Keyword::SECURITY)?;
                Ok(AlterTableAction::ForceRowLevelSecurity)
            }
            Some(Keyword::NOT) => {
                self.advance();
                if self.match_keyword(Keyword::OF) {
                    self.advance();
                    let type_name = self.parse_object_name()?;
                    Ok(AlterTableAction::NotOfType { type_name })
                } else {
                    Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "OF after NOT".to_string(),
                        got: format!("{:?}", self.peek()),
                    })
                }
            }
            Some(Keyword::OF) => {
                self.advance();
                let type_name = self.parse_object_name()?;
                Ok(AlterTableAction::OfType { type_name })
            }
            Some(Keyword::COMMENT) => {
                self.advance();
                if self.match_token(&Token::Eq) {
                    self.advance();
                }
                let comment = self.parse_string_literal()?;
                Ok(AlterTableAction::SetComment { comment })
            }
            _ if self.match_ident_str("ILM") => {
                self.advance();
                if self.match_ident_str("ENABLE") {
                    self.advance();
                    if self.match_ident_str("POLICY") {
                        self.advance();
                    }
                    Ok(AlterTableAction::IlmEnablePolicy)
                } else if self.match_ident_str("DISABLE") {
                    self.advance();
                    if self.match_ident_str("POLICY") {
                        self.advance();
                    }
                    Ok(AlterTableAction::IlmDisablePolicy)
                } else if self.match_ident_str("DELETE") {
                    self.advance();
                    if self.match_ident_str("POLICY") {
                        self.advance();
                    }
                    Ok(AlterTableAction::IlmDeletePolicy)
                } else if self.match_keyword(Keyword::ADD_P) {
                    self.advance();
                    if self.match_ident_str("POLICY") {
                        self.advance();
                    }
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
                    Ok(AlterTableAction::IlmAddPolicy(IlmPolicy {
                        after_n,
                        unit,
                        condition,
                    }))
                } else {
                    Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "ENABLE, DISABLE, DELETE, or ADD after ILM".to_string(),
                        got: format!("{:?}", self.peek()),
                    })
                }
            }
            Some(Keyword::ENCRYPTION) => {
                self.advance();
                self.expect_keyword(Keyword::KEY)?;
                self.expect_keyword(Keyword::ROTATION)?;
                Ok(AlterTableAction::EncryptionKeyRotation)
            }
            _ if self.match_ident_str("GSIWAITALL") => {
                self.advance();
                Ok(AlterTableAction::GsiWaitAll)
            }
            Some(Keyword::DELETE_P) => {
                self.advance();
                self.expect_keyword(Keyword::STATISTICS)?;
                self.parse_statistics_columns(StatisticsOpKind::Delete)
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
            let every = if self.match_keyword(Keyword::EVERY) {
                self.advance();
                self.expect_token(&Token::LParen)?;
                let e = self.parse_expr()?;
                self.expect_token(&Token::RParen)?;
                Some(e)
            } else {
                None
            };
            Ok(PartitionValues::StartEnd { start, end, every })
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

    fn parse_start_end_values(&mut self) -> Result<PartitionValues, ParserError> {
        self.advance();
        self.expect_token(&Token::LParen)?;
        let start = self.parse_expr()?;
        self.expect_token(&Token::RParen)?;
        self.expect_keyword(Keyword::END_P)?;
        self.expect_token(&Token::LParen)?;
        let end = self.parse_expr()?;
        self.expect_token(&Token::RParen)?;
        let every = if self.match_keyword(Keyword::EVERY) {
            self.advance();
            self.expect_token(&Token::LParen)?;
            let e = self.parse_expr()?;
            self.expect_token(&Token::RParen)?;
            Some(e)
        } else {
            None
        };
        Ok(PartitionValues::StartEnd { start, end, every })
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
            } else if self.match_keyword(Keyword::START) {
                Some(self.parse_start_end_values()?)
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

    fn parse_statistics_columns(
        &mut self,
        op: StatisticsOpKind,
    ) -> Result<AlterTableAction, ParserError> {
        self.expect_token(&Token::LParen)?;
        self.expect_token(&Token::LParen)?;
        let mut columns = Vec::new();
        loop {
            columns.push(self.parse_identifier()?);
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }
        self.expect_token(&Token::RParen)?;
        self.expect_token(&Token::RParen)?;
        Ok(AlterTableAction::StatisticsOp { op, columns })
    }

    fn parse_update_index_clauses(&mut self) -> Result<(bool, Option<bool>), ParserError> {
        let mut update_global_index = false;

        if self.match_keyword(Keyword::UPDATE) {
            let next_is_distributed = self
                .tokens
                .get(self.pos + 1)
                .map(|t| matches!(&t.token, Token::Ident(s) if s.eq_ignore_ascii_case("DISTRIBUTED")))
                .unwrap_or(false);
            if next_is_distributed {
            } else {
                self.advance();
                self.expect_keyword(Keyword::GLOBAL)?;
                self.expect_keyword(Keyword::INDEX)?;
                update_global_index = true;
            }
        }

        let update_distributed_global_index = if self.match_keyword(Keyword::UPDATE) {
            self.advance();
            if self.match_ident_str("DISTRIBUTED") {
                self.advance();
                self.expect_keyword(Keyword::GLOBAL)?;
                self.expect_keyword(Keyword::INDEX)?;
                Some(true)
            } else {
                return Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "DISTRIBUTED after UPDATE".to_string(),
                    got: format!("{:?}", self.peek()),
                });
            }
        } else if self.match_keyword(Keyword::NO) {
            self.advance();
            self.expect_keyword(Keyword::UPDATE)?;
            if self.match_ident_str("DISTRIBUTED") {
                self.advance();
                self.expect_keyword(Keyword::GLOBAL)?;
                self.expect_keyword(Keyword::INDEX)?;
                Some(false)
            } else {
                return Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "DISTRIBUTED after NO UPDATE".to_string(),
                    got: format!("{:?}", self.peek()),
                });
            }
        } else {
            None
        };

        Ok((update_global_index, update_distributed_global_index))
    }

    // ========== ALTER FOREIGN TABLE ==========

    pub(crate) fn parse_alter_foreign_table(
        &mut self,
    ) -> Result<AlterForeignTableStatement, ParserError> {
        let name = self.parse_object_name()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterForeignTableStatement { name, raw_rest })
    }

    // ========== ALTER FOREIGN SERVER ==========

    pub(crate) fn parse_alter_foreign_server(
        &mut self,
    ) -> Result<AlterForeignServerStatement, ParserError> {
        let name = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterForeignServerStatement { name, raw_rest })
    }

    // ========== ALTER FOREIGN DATA WRAPPER ==========

    pub(crate) fn parse_alter_fdw(&mut self) -> Result<AlterFdwStatement, ParserError> {
        let name = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterFdwStatement { name, raw_rest })
    }

    // ========== ALTER PUBLICATION ==========

    pub(crate) fn parse_alter_publication(
        &mut self,
    ) -> Result<AlterPublicationStatement, ParserError> {
        let name = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterPublicationStatement { name, raw_rest })
    }

    // ========== ALTER SUBSCRIPTION ==========

    pub(crate) fn parse_alter_subscription(
        &mut self,
    ) -> Result<AlterSubscriptionStatement, ParserError> {
        let name = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterSubscriptionStatement { name, raw_rest })
    }

    // ========== ALTER NODE ==========

    pub(crate) fn parse_alter_node(&mut self) -> Result<AlterNodeStatement, ParserError> {
        let name = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterNodeStatement { name, raw_rest })
    }

    // ========== ALTER NODE GROUP ==========

    pub(crate) fn parse_alter_node_group(
        &mut self,
    ) -> Result<AlterNodeGroupStatement, ParserError> {
        let name = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterNodeGroupStatement { name, raw_rest })
    }

    // ========== ALTER WORKLOAD GROUP ==========

    pub(crate) fn parse_alter_workload_group(
        &mut self,
    ) -> Result<AlterWorkloadGroupStatement, ParserError> {
        let name = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterWorkloadGroupStatement { name, raw_rest })
    }

    // ========== ALTER AUDIT POLICY ==========

    pub(crate) fn parse_alter_audit_policy(
        &mut self,
    ) -> Result<AlterAuditPolicyStatement, ParserError> {
        let name = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterAuditPolicyStatement { name, raw_rest })
    }

    // ========== ALTER RLS POLICY ==========

    pub(crate) fn parse_alter_rls_policy(
        &mut self,
    ) -> Result<AlterRlsPolicyStatement, ParserError> {
        let name = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterRlsPolicyStatement { name, raw_rest })
    }

    // ========== ALTER DATA SOURCE ==========

    pub(crate) fn parse_alter_data_source(
        &mut self,
    ) -> Result<AlterDataSourceStatement, ParserError> {
        let name = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterDataSourceStatement { name, raw_rest })
    }

    // ========== ALTER EVENT ==========

    pub(crate) fn parse_alter_event(&mut self) -> Result<AlterEventStatement, ParserError> {
        let name = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterEventStatement { name, raw_rest })
    }

    // ========== ALTER OPERATOR FAMILY ==========

    pub(crate) fn parse_alter_opfamily(&mut self) -> Result<AlterOpFamilyStatement, ParserError> {
        let name = self.parse_identifier()?;
        self.expect_keyword(Keyword::USING)?;
        let method = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterOpFamilyStatement {
            name,
            method,
            raw_rest,
        })
    }

    // ========== ALTER MATERIALIZED VIEW ==========

    pub(crate) fn parse_alter_materialized_view(
        &mut self,
    ) -> Result<AlterMaterializedViewStatement, ParserError> {
        let name = self.parse_object_name()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterMaterializedViewStatement { name, raw_rest })
    }

    // ========== ALTER SYNONYM ==========

    pub(crate) fn parse_alter_synonym(&mut self) -> Result<AlterSynonymStatement, ParserError> {
        let name = self.parse_identifier()?;
        let action = if self.try_consume_keyword(Keyword::COMPILE) {
            let debug = self.match_ident_str("DEBUG");
            if debug {
                self.advance();
            }
            AlterSynonymAction::Compile { debug }
        } else if self.try_consume_keyword(Keyword::OWNER) {
            self.expect_keyword(Keyword::TO)?;
            let new_owner = self.parse_identifier()?;
            AlterSynonymAction::OwnerTo { new_owner }
        } else {
            let raw_rest = self.skip_to_semicolon_and_collect();
            return Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "COMPILE or OWNER after ALTER SYNONYM name".to_string(),
                got: raw_rest,
            });
        };
        Ok(AlterSynonymStatement { name, action })
    }

    // ========== ALTER TEXT SEARCH CONFIGURATION ==========

    pub(crate) fn parse_alter_text_search_config(
        &mut self,
    ) -> Result<AlterTextSearchConfigStatement, ParserError> {
        let name = self.parse_object_name()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterTextSearchConfigStatement { name, raw_rest })
    }

    // ========== ALTER TEXT SEARCH DICTIONARY ==========

    pub(crate) fn parse_alter_text_search_dict(
        &mut self,
    ) -> Result<AlterTextSearchDictStatement, ParserError> {
        let name = self.parse_object_name()?;
        let mut options = Vec::new();
        if self.match_token(&Token::LParen) {
            self.advance();
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
        }
        Ok(AlterTextSearchDictStatement { name, options })
    }

    // ========== ALTER COORDINATOR ==========

    pub(crate) fn parse_alter_coordinator(
        &mut self,
    ) -> Result<AlterCoordinatorStatement, ParserError> {
        let name = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterCoordinatorStatement { name, raw_rest })
    }

    // ========== ALTER APP WORKLOAD GROUP MAPPING ==========

    pub(crate) fn parse_alter_app_workload_group_mapping(
        &mut self,
    ) -> Result<AlterAppWorkloadGroupMappingStatement, ParserError> {
        let name = self.parse_identifier()?;
        let raw_rest = self.skip_to_semicolon_and_collect();
        Ok(AlterAppWorkloadGroupMappingStatement { name, raw_rest })
    }

    // ========== DROP ==========
}
