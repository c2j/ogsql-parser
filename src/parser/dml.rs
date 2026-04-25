use crate::ast::{
    DeleteStatement, DmlPartitionClause, InsertAllCondition, InsertAllStatement, InsertAllTarget,
    InsertFirstStatement, InsertSource, InsertStatement, MergeAction, MergeStatement,
    MergeWhenClause, OnConflictAction, OnConflictTarget, SelectTarget, TablePartitionRef, TableRef,
    UpdateAssignment, UpdateStatement,
};
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;

impl Parser {
    fn apply_dml_partition_to_table_ref(table: TableRef, dml: DmlPartitionClause) -> TableRef {
        match table {
            TableRef::Table {
                name,
                alias,
                partition: _,
                timecapsule,
            } => {
                let part = match dml {
                    DmlPartitionClause::Partition(names) => TablePartitionRef {
                        for_values: None,
                        values: names,
                    },
                    DmlPartitionClause::Subpartition(names) => TablePartitionRef {
                        for_values: None,
                        values: names,
                    },
                    DmlPartitionClause::PartitionFor(exprs) => TablePartitionRef {
                        for_values: Some(exprs),
                        values: vec![],
                    },
                    DmlPartitionClause::SubpartitionFor(exprs) => TablePartitionRef {
                        for_values: Some(exprs),
                        values: vec![],
                    },
                };
                TableRef::Table {
                    name,
                    alias,
                    partition: Some(part),
                    timecapsule,
                }
            }
            other => other,
        }
    }
    pub(crate) fn parse_insert(&mut self) -> Result<InsertStatement, ParserError> {
        let post_hints = self.consume_hints();
        self.try_consume_keyword(Keyword::INTO);
        let table = self.parse_object_name()?;
        let alias = self.parse_optional_alias()?;
        let partition = self.parse_dml_partition()?;
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
        } else if self.match_keyword(Keyword::SET) {
            self.advance();
            let mut assignments = Vec::new();
            loop {
                let col = self.parse_identifier()?;
                self.expect_token(&Token::Eq)?;
                let value = self.parse_expr()?;
                assignments.push(crate::ast::UpdateAssignment {
                    column: vec![col],
                    value,
                });
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            InsertSource::Set(assignments)
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
        } else if self.match_token(&Token::LParen) {
            if let Some(Token::Keyword(kw)) = self.tokens.get(self.pos + 1).map(|tws| &tws.token) {
                if *kw == Keyword::SELECT || *kw == Keyword::WITH {
                    self.advance();
                    let select = self.parse_select_statement()?;
                    self.expect_token(&Token::RParen)?;
                    InsertSource::Select(Box::new(select))
                } else {
                    return Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "VALUES, SELECT, DEFAULT VALUES".to_string(),
                        got: format!("{:?}", self.peek()),
                    });
                }
            } else {
                return Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "VALUES, SELECT, DEFAULT VALUES".to_string(),
                    got: format!("{:?}", self.peek()),
                });
            }
        } else {
            return Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "VALUES, SELECT, DEFAULT VALUES".to_string(),
                got: format!("{:?}", self.peek()),
            });
        };
        let on_conflict = if self.match_keyword(Keyword::ON) {
            self.advance();
            if self.match_keyword(Keyword::DUPLICATE) {
                self.advance();
                self.expect_keyword(Keyword::KEY)?;
                self.expect_keyword(Keyword::UPDATE)?;
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
                let where_clause = if self.match_keyword(Keyword::WHERE) {
                    self.advance();
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                Some(OnConflictAction::Update {
                    target: None,
                    assignments,
                    where_clause,
                })
            } else if self.match_keyword(Keyword::CONFLICT) {
                self.advance();
                let target = if self.match_keyword(Keyword::ON) {
                    self.advance();
                    self.expect_keyword(Keyword::CONSTRAINT)?;
                    let name = self.parse_identifier()?;
                    Some(OnConflictTarget::OnConstraint(name))
                } else if self.match_token(&Token::LParen) {
                    self.advance();
                    let mut cols = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        cols.push(self.parse_identifier()?);
                    }
                    self.expect_token(&Token::RParen)?;
                    Some(OnConflictTarget::Columns(cols))
                } else {
                    None
                };
                self.expect_keyword(Keyword::DO)?;
                if self.match_keyword(Keyword::NOTHING) {
                    self.advance();
                    Some(OnConflictAction::Nothing { target })
                } else if self.match_keyword(Keyword::UPDATE) {
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
                    let where_clause = if self.match_keyword(Keyword::WHERE) {
                        self.advance();
                        Some(self.parse_expr()?)
                    } else {
                        None
                    };
                    Some(OnConflictAction::Update {
                        target,
                        assignments,
                        where_clause,
                    })
                } else {
                    return Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "NOTHING or UPDATE".to_string(),
                        got: format!("{:?}", self.peek()),
                    });
                }
            } else {
                self.pos -= 1;
                None
            }
        } else {
            None
        };
        let (returning, into_targets, bulk_collect) = if self.match_keyword(Keyword::RETURNING) {
            self.advance();
            let returning = self.parse_target_list()?;
            let (into_targets, bulk_collect) = if self.pl_into_mode {
                if self.match_ident_str("bulk") {
                    self.advance();
                    self.expect_ident_str("collect")?;
                    self.expect_keyword(Keyword::INTO)?;
                    (Some(self.parse_pl_into_target_list()?), true)
                } else if self.match_keyword(Keyword::INTO) {
                    self.advance();
                    (Some(self.parse_pl_into_target_list()?), false)
                } else {
                    (None, false)
                }
            } else {
                (None, false)
            };
            (returning, into_targets, bulk_collect)
        } else {
            (vec![], None, false)
        };
        Ok(InsertStatement {
            hints: post_hints,
            with: None,
            table,
            alias,
            partition,
            columns,
            source,
            on_conflict,
            returning,
            into_targets,
            bulk_collect,
        })
    }

    pub(crate) fn parse_dml_partition(
        &mut self,
    ) -> Result<Option<DmlPartitionClause>, ParserError> {
        if self.match_keyword(Keyword::PARTITION) {
            self.advance();
            if self.match_keyword(Keyword::FOR) {
                self.advance();
                self.expect_token(&Token::LParen)?;
                let mut exprs = vec![self.parse_expr()?];
                while self.match_token(&Token::Comma) {
                    self.advance();
                    exprs.push(self.parse_expr()?);
                }
                self.expect_token(&Token::RParen)?;
                Ok(Some(DmlPartitionClause::PartitionFor(exprs)))
            } else {
                self.expect_token(&Token::LParen)?;
                let mut names = vec![self.parse_identifier()?];
                while self.match_token(&Token::Comma) {
                    self.advance();
                    names.push(self.parse_identifier()?);
                }
                self.expect_token(&Token::RParen)?;
                Ok(Some(DmlPartitionClause::Partition(names)))
            }
        } else if self.match_keyword(Keyword::SUBPARTITION) {
            self.advance();
            if self.match_keyword(Keyword::FOR) {
                self.advance();
                self.expect_token(&Token::LParen)?;
                let mut exprs = vec![self.parse_expr()?];
                while self.match_token(&Token::Comma) {
                    self.advance();
                    exprs.push(self.parse_expr()?);
                }
                self.expect_token(&Token::RParen)?;
                Ok(Some(DmlPartitionClause::SubpartitionFor(exprs)))
            } else {
                self.expect_token(&Token::LParen)?;
                let mut names = vec![self.parse_identifier()?];
                while self.match_token(&Token::Comma) {
                    self.advance();
                    names.push(self.parse_identifier()?);
                }
                self.expect_token(&Token::RParen)?;
                Ok(Some(DmlPartitionClause::Subpartition(names)))
            }
        } else {
            Ok(None)
        }
    }

    pub(crate) fn parse_update(&mut self) -> Result<UpdateStatement, ParserError> {
        let post_hints = self.consume_hints();
        let mut tables = vec![self.parse_table_ref()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            tables.push(self.parse_table_ref()?);
        }
        let partition = self.parse_dml_partition()?;
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
        let (returning, into_targets, bulk_collect) = if self.match_keyword(Keyword::RETURNING) {
            self.advance();
            let returning = self.parse_target_list()?;
            let (into_targets, bulk_collect) = if self.pl_into_mode {
                if self.match_ident_str("bulk") {
                    self.advance();
                    self.expect_ident_str("collect")?;
                    self.expect_keyword(Keyword::INTO)?;
                    (Some(self.parse_pl_into_target_list()?), true)
                } else if self.match_keyword(Keyword::INTO) {
                    self.advance();
                    (Some(self.parse_pl_into_target_list()?), false)
                } else {
                    (None, false)
                }
            } else {
                (None, false)
            };
            (returning, into_targets, bulk_collect)
        } else {
            (vec![], None, false)
        };
        Ok(UpdateStatement {
            hints: post_hints,
            with: None,
            tables,
            partition,
            assignments,
            from,
            where_clause,
            returning,
            into_targets,
            bulk_collect,
        })
    }

    pub(crate) fn parse_delete(&mut self) -> Result<DeleteStatement, ParserError> {
        let post_hints = self.consume_hints();
        let has_from = self.try_consume_keyword(Keyword::FROM);
        let mut tables = vec![self.parse_table_ref()?];
        if let Some(dml_part) = self.parse_dml_partition()? {
            if let Some(last) = tables.last_mut() {
                *last = Self::apply_dml_partition_to_table_ref(last.clone(), dml_part);
            }
        }
        while self.match_token(&Token::Comma) {
            self.advance();
            tables.push(self.parse_table_ref()?);
            if let Some(dml_part) = self.parse_dml_partition()? {
                if let Some(last) = tables.last_mut() {
                    *last = Self::apply_dml_partition_to_table_ref(last.clone(), dml_part);
                }
            }
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
        let (returning, into_targets, bulk_collect) = if self.match_keyword(Keyword::RETURNING) {
            self.advance();
            let returning = self.parse_target_list()?;
            let (into_targets, bulk_collect) = if self.pl_into_mode {
                if self.match_ident_str("bulk") {
                    self.advance();
                    self.expect_ident_str("collect")?;
                    self.expect_keyword(Keyword::INTO)?;
                    (Some(self.parse_pl_into_target_list()?), true)
                } else if self.match_keyword(Keyword::INTO) {
                    self.advance();
                    (Some(self.parse_pl_into_target_list()?), false)
                } else {
                    (None, false)
                }
            } else {
                (None, false)
            };
            (returning, into_targets, bulk_collect)
        } else {
            (vec![], None, false)
        };
        Ok(DeleteStatement {
            hints: post_hints,
            with: None,
            tables,
            using,
            where_clause,
            returning,
            into_targets,
            bulk_collect,
        })
    }

    pub(crate) fn parse_merge(&mut self) -> Result<MergeStatement, ParserError> {
        let post_hints = self.consume_hints();
        self.try_consume_keyword(Keyword::INTO);
        let target_name = self.parse_object_name()?;
        let partition = self.parse_dml_partition()?;
        let target_alias = self.parse_optional_alias()?;
        let target = TableRef::Table {
            name: target_name,
            alias: target_alias,
            partition: None,
            timecapsule: None,
        };
        self.expect_keyword(Keyword::USING)?;
        let mut source = self.parse_table_ref()?;
        let source_partition = self.parse_dml_partition()?;
        if let TableRef::Table { ref mut alias, .. } = source {
            if alias.is_none() {
                *alias = self.parse_optional_alias()?;
            }
        }
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
                    let mut cols = vec![self.parse_object_name()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        cols.push(self.parse_object_name()?);
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
            let where_clause = if self.match_keyword(Keyword::WHERE) {
                self.advance();
                Some(self.parse_expr()?)
            } else {
                None
            };
            when_clauses.push(MergeWhenClause {
                matched,
                action,
                where_clause,
            });
        }
        Ok(MergeStatement {
            hints: post_hints,
            target,
            partition,
            source,
            source_partition,
            on_condition,
            when_clauses,
        })
    }

    fn parse_insert_all_target(&mut self) -> Result<InsertAllTarget, ParserError> {
        self.expect_keyword(Keyword::INTO)?;
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
        self.expect_keyword(Keyword::VALUES)?;
        let mut all_rows = Vec::new();
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
            all_rows.push(row);
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }
        Ok(InsertAllTarget {
            table,
            columns,
            values: all_rows,
        })
    }

    pub(crate) fn parse_insert_all(&mut self) -> Result<InsertAllStatement, ParserError> {
        let mut targets: Vec<InsertAllTarget> = Vec::new();
        let mut conditions: Vec<InsertAllCondition> = Vec::new();
        let mut else_targets: Vec<InsertAllTarget> = Vec::new();

        loop {
            if self.match_keyword(Keyword::WHEN) {
                self.advance();
                let condition = self.parse_expr()?;
                self.expect_keyword(Keyword::THEN)?;
                let mut cond_targets = vec![self.parse_insert_all_target()?];
                while self.match_keyword(Keyword::INTO) {
                    cond_targets.push(self.parse_insert_all_target()?);
                }
                conditions.push(InsertAllCondition {
                    condition,
                    targets: cond_targets,
                });
            } else if self.match_keyword(Keyword::INTO) {
                targets.push(self.parse_insert_all_target()?);
            } else if self.match_keyword(Keyword::ELSE) {
                self.advance();
                else_targets.push(self.parse_insert_all_target()?);
                while self.match_keyword(Keyword::INTO) {
                    else_targets.push(self.parse_insert_all_target()?);
                }
                break;
            } else {
                break;
            }
        }

        let source = Box::new(self.parse_select_statement()?);

        Ok(InsertAllStatement {
            targets,
            conditions,
            else_targets,
            source,
        })
    }

    pub(crate) fn parse_insert_first(&mut self) -> Result<InsertFirstStatement, ParserError> {
        let mut when_clauses: Vec<InsertAllCondition> = Vec::new();
        let mut else_targets: Vec<InsertAllTarget> = Vec::new();

        loop {
            if self.match_keyword(Keyword::WHEN) {
                self.advance();
                let condition = self.parse_expr()?;
                self.expect_keyword(Keyword::THEN)?;
                let mut cond_targets = vec![self.parse_insert_all_target()?];
                while self.match_keyword(Keyword::INTO) {
                    cond_targets.push(self.parse_insert_all_target()?);
                }
                when_clauses.push(InsertAllCondition {
                    condition,
                    targets: cond_targets,
                });
            } else if self.match_keyword(Keyword::ELSE) {
                self.advance();
                else_targets.push(self.parse_insert_all_target()?);
                while self.match_keyword(Keyword::INTO) {
                    else_targets.push(self.parse_insert_all_target()?);
                }
                break;
            } else {
                break;
            }
        }

        let source = Box::new(self.parse_select_statement()?);

        Ok(InsertFirstStatement {
            when_clauses,
            else_targets,
            source,
        })
    }
}
