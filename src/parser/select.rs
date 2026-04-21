use crate::ast::{
    ConnectByClause, Cte, FetchClause, GroupByItem, JoinType, LockClause, ObjectName, OrderByItem,
    PivotClause, PivotValue, SelectIntoTable, SelectStatement, SelectTarget, SetOperation,
    TableRef, UnpivotClause, ValuesStatement, WithClause,
};
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;

impl Parser {
    pub(crate) fn parse_select_statement(&mut self) -> Result<SelectStatement, ParserError> {
        self.enter_scope()?;
        let result = self.parse_select_statement_inner();
        self.leave_scope();
        result
    }

    fn parse_select_statement_inner(&mut self) -> Result<SelectStatement, ParserError> {
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

    pub(crate) fn parse_with_clause(&mut self) -> Result<Option<WithClause>, ParserError> {
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
            let materialized = if self.match_keyword(Keyword::NOT) {
                self.advance();
                self.expect_keyword(Keyword::MATERIALIZED)?;
                Some(false)
            } else if self.match_keyword(Keyword::MATERIALIZED) {
                self.advance();
                Some(true)
            } else {
                None
            };
            self.expect_token(&Token::LParen)?;
            let query = if self.match_keyword(Keyword::VALUES) {
                let raw_body = self.collect_until_balanced_paren();
                let mut s = SelectStatement::default();
                s.raw_body = Some(raw_body);
                s
            } else if matches!(self.peek_keyword(), Some(Keyword::UPDATE) | Some(Keyword::INSERT) | Some(Keyword::DELETE_P)) {
                let raw_body = self.collect_until_balanced_paren();
                let mut s = SelectStatement::default();
                s.raw_body = Some(raw_body);
                s
            } else {
                self.parse_select_statement()?
            };
            if !query.raw_body.is_some() {
                self.expect_token(&Token::RParen)?;
            }
            ctes.push(Cte {
                name,
                columns,
                query: Box::new(query),
                raw_body: None,
                materialized,
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
        let hints = self.consume_hints();
        let (distinct, mut distinct_on) = if self.match_keyword(Keyword::DISTINCT) {
            self.advance();
            let cols = if self.match_keyword(Keyword::ON) {
                self.advance();
                self.expect_token(&Token::LParen)?;
                let mut exprs = vec![self.parse_expr()?];
                while self.match_token(&Token::Comma) {
                    self.advance();
                    exprs.push(self.parse_expr()?);
                }
                self.expect_token(&Token::RParen)?;
                exprs
            } else {
                vec![]
            };
            (true, cols)
        } else {
            if self.match_keyword(Keyword::ALL) {
                self.advance();
            }
            (false, vec![])
        };
        let targets = self.parse_target_list()?;
        let (into_targets, into_table) = if self.match_keyword(Keyword::INTO) {
            self.advance();
            let unlogged = if self.match_keyword(Keyword::UNLOGGED) {
                self.advance();
                true
            } else {
                false
            };
            if unlogged || self.match_keyword(Keyword::TABLE) {
                if self.match_keyword(Keyword::TABLE) {
                    self.advance();
                }
                let table_name = self.parse_object_name()?;
                (
                    None,
                    Some(SelectIntoTable {
                        unlogged,
                        table_name,
                    }),
                )
            } else {
                let save_pos = self.pos;
                if let Ok(table_name) = self.parse_object_name() {
                    if self.match_keyword(Keyword::FROM) || self.match_token(&Token::Eof) {
                        (
                            None,
                            Some(SelectIntoTable {
                                unlogged: false,
                                table_name,
                            }),
                        )
                    } else {
                        self.pos = save_pos;
                        (Some(self.parse_target_list()?), None)
                    }
                } else {
                    self.pos = save_pos;
                    (Some(self.parse_target_list()?), None)
                }
            }
        } else {
            (None, None)
        };
        let from = self.parse_from_clause()?;
        let where_clause = if self.match_keyword(Keyword::WHERE) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };

        // START WITH (can appear before CONNECT BY)
        let start_with = if self.match_keyword(Keyword::START) {
            self.advance();
            self.expect_keyword(Keyword::WITH)?;
            Some(self.parse_expr()?)
        } else {
            None
        };

        // CONNECT BY [NOCYCLE] condition [START WITH ...]
        let connect_by = if self.match_keyword(Keyword::CONNECT) {
            self.advance();
            self.expect_keyword(Keyword::BY)?;
            let nocycle = self.try_consume_keyword(Keyword::NOCYCLE);
            let condition = self.parse_expr()?;
            let sw = if start_with.is_none() && self.match_keyword(Keyword::START) {
                self.advance();
                self.expect_keyword(Keyword::WITH)?;
                Some(self.parse_expr()?)
            } else {
                start_with
            };
            Some(ConnectByClause {
                nocycle,
                condition,
                start_with: sw,
            })
        } else {
            None
        };

        let group_by = if self.match_keyword(Keyword::GROUP_P) {
            self.advance();
            self.expect_keyword(Keyword::BY)?;
            let mut items = vec![self.parse_group_by_item()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                items.push(self.parse_group_by_item()?);
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
            hints,
            with: None,
            distinct,
            distinct_on,
            targets,
            into_targets,
            into_table,
            from,
            where_clause,
            connect_by,
            group_by,
            having,
            order_by: vec![],
            order_siblings: false,
            limit: None,
            offset: None,
            set_operation: None,
            fetch: None,
            lock_clause: None,
            window_clause: vec![],
            raw_body: None,
        })
    }

    fn parse_group_by_item(&mut self) -> Result<GroupByItem, ParserError> {
        if self.match_keyword(Keyword::ROLLUP) {
            self.advance();
            self.expect_token(&Token::LParen)?;
            let mut cols = vec![self.parse_expr()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                cols.push(self.parse_expr()?);
            }
            self.expect_token(&Token::RParen)?;
            return Ok(GroupByItem::Rollup(cols));
        }

        if self.match_keyword(Keyword::CUBE) {
            self.advance();
            self.expect_token(&Token::LParen)?;
            let mut cols = vec![self.parse_expr()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                cols.push(self.parse_expr()?);
            }
            self.expect_token(&Token::RParen)?;
            return Ok(GroupByItem::Cube(cols));
        }

        if self.match_keyword(Keyword::GROUPING_P) {
            self.advance();
            if self.match_keyword(Keyword::SETS) {
                self.advance();
                self.expect_token(&Token::LParen)?;
                let mut sets = Vec::new();
                loop {
                    if self.match_token(&Token::LParen) {
                        self.advance();
                        let mut group = Vec::new();
                        if !self.match_token(&Token::RParen) {
                            group.push(self.parse_expr()?);
                            while self.match_token(&Token::Comma) {
                                self.advance();
                                group.push(self.parse_expr()?);
                            }
                        }
                        self.expect_token(&Token::RParen)?;
                        sets.push(group);
                    } else {
                        let group = vec![self.parse_expr()?];
                        sets.push(group);
                    }
                    if self.match_token(&Token::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect_token(&Token::RParen)?;
                return Ok(GroupByItem::GroupingSets(sets));
            } else {
                self.pos -= 1;
            }
        }

        Ok(GroupByItem::Expr(self.parse_expr()?))
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
            self.parse_optional_column_alias()?
        };
        Ok(SelectTarget::Expr(expr, alias))
    }

    pub(crate) fn parse_from_clause(&mut self) -> Result<Vec<TableRef>, ParserError> {
        if !self.match_keyword(Keyword::FROM) {
            return Ok(vec![]);
        }
        self.advance();
        let mut tables = vec![self.parse_table_ref()?];
        self.try_consume_table_modifiers(&mut tables[0]);
        while self.match_token(&Token::Comma) {
            self.advance();
            tables.push(self.parse_table_ref()?);
            if let Some(last) = tables.last_mut() {
                self.try_consume_table_modifiers(last);
            }
        }
        Ok(tables)
    }

    fn try_consume_table_modifiers(&mut self, table_ref: &mut TableRef) {
        if self.match_keyword(Keyword::PARTITION) {
            if let Ok(Some(p)) = self.try_parse_partition_ref(Keyword::PARTITION) {
                if let TableRef::Table {
                    partition: ref mut pp,
                    ..
                } = table_ref
                {
                    *pp = Some(p);
                }
            }
        }
        if self.match_keyword(Keyword::SUBPARTITION) {
            if let Ok(Some(p)) = self.try_parse_partition_ref(Keyword::SUBPARTITION) {
                if let TableRef::Table {
                    partition: ref mut pp,
                    ..
                } = table_ref
                {
                    *pp = Some(p);
                }
            }
        }
        if self.match_keyword(Keyword::TIMECAPSULE) {
            if let Ok(tc) = self.try_parse_timecapsule() {
                if let TableRef::Table {
                    timecapsule: ref mut tc_field,
                    ..
                } = table_ref
                {
                    *tc_field = Some(tc);
                }
            }
        }
    }

    fn try_parse_timecapsule(&mut self) -> Result<crate::ast::Expr, ParserError> {
        self.expect_keyword(Keyword::TIMECAPSULE)?;
        if self.match_keyword(Keyword::TIMESTAMP) {
            self.advance();
            Ok(self.parse_expr()?)
        } else if self.match_keyword(Keyword::CSN) {
            self.advance();
            Ok(self.parse_expr()?)
        } else {
            Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "TIMESTAMP or CSN".to_string(),
                got: format!("{:?}", self.peek()),
            })
        }
    }

    fn try_parse_partition_ref(
        &mut self,
        keyword: Keyword,
    ) -> Result<Option<crate::ast::TablePartitionRef>, ParserError> {
        if !self.match_keyword(keyword) {
            return Ok(None);
        }
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
            Ok(Some(crate::ast::TablePartitionRef {
                values: vec![],
                for_values: Some(exprs),
            }))
        } else {
            self.expect_token(&Token::LParen)?;
            let mut values = vec![self.parse_identifier()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                values.push(self.parse_identifier()?);
            }
            self.expect_token(&Token::RParen)?;
            Ok(Some(crate::ast::TablePartitionRef {
                values,
                for_values: None,
            }))
        }
    }

    pub(crate) fn parse_table_ref(&mut self) -> Result<TableRef, ParserError> {
        let mut left = self.parse_primary_table_ref()?;
        loop {
            let natural = self.match_keyword(Keyword::NATURAL);
            if natural {
                self.advance();
            }
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
                _ => {
                    if natural {
                        self.pos -= 1; // put back NATURAL
                    }
                    break;
                }
            };
            let right = self.parse_primary_table_ref()?;
            let (condition, using_columns) = if !natural && join_type != JoinType::Cross {
                if self.match_keyword(Keyword::ON) {
                    self.advance();
                    (Some(self.parse_expr()?), vec![])
                } else if self.match_keyword(Keyword::USING) {
                    self.advance();
                    self.expect_token(&Token::LParen)?;
                    let mut cols = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        cols.push(self.parse_identifier()?);
                    }
                    self.expect_token(&Token::RParen)?;
                    (None, cols)
                } else {
                    (None, vec![])
                }
            } else {
                (None, vec![])
            };
            left = TableRef::Join {
                left: Box::new(left),
                right: Box::new(right),
                join_type,
                condition,
                natural,
                using_columns,
            };
        }
        if self.match_ident_str("PIVOT") {
            self.advance();
            let pivot = self.parse_pivot()?;
            left = TableRef::Pivot {
                source: Box::new(left),
                pivot,
            };
        } else if self.match_ident_str("UNPIVOT") {
            self.advance();
            let unpivot = self.parse_unpivot()?;
            left = TableRef::Unpivot {
                source: Box::new(left),
                unpivot,
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
            if self.match_keyword(Keyword::VALUES) {
                self.advance();
                let values = self.parse_values_statement()?;
                self.expect_token(&Token::RParen)?;
                let alias = self.parse_optional_alias()?;
                let column_names = if alias.is_some() && self.match_token(&Token::LParen) {
                    self.advance();
                    let mut names = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        names.push(self.parse_identifier()?);
                    }
                    self.expect_token(&Token::RParen)?;
                    names
                } else {
                    vec![]
                };
                return Ok(TableRef::Values {
                    values: Box::new(values),
                    alias,
                    column_names,
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
        if self.match_token(&Token::LParen) {
            self.advance();
            let args = if self.match_token(&Token::RParen) {
                vec![]
            } else {
                let mut args = vec![self.parse_maybe_named_arg()?];
                while self.match_token(&Token::Comma) {
                    self.advance();
                    args.push(self.parse_maybe_named_arg()?);
                }
                args
            };
            self.expect_token(&Token::RParen)?;
            let alias = self.parse_optional_column_alias()?;
            let column_defs = if alias.is_some() && self.match_token(&Token::LParen) {
                self.advance();
                let mut defs = vec![(self.parse_identifier()?, self.parse_optional_func_col_type()?)];
                while self.match_token(&Token::Comma) {
                    self.advance();
                    defs.push((self.parse_identifier()?, self.parse_optional_func_col_type()?));
                }
                self.expect_token(&Token::RParen)?;
                defs.into_iter().map(|(name, data_type)| crate::ast::ColumnDef {
                    name,
                    data_type,
                    constraints: vec![],
                    compress_mode: None,
                    charset: None,
                    collate: None,
                    on_update: None,
                    comment: None,
                    generated: None,
                    encrypted_with: None,
                }).collect()
            } else {
                vec![]
            };
            return Ok(TableRef::FunctionCall {
                name,
                args,
                alias,
                column_defs,
            });
        }
        let alias = if self.match_ident_str("PIVOT") || self.match_ident_str("UNPIVOT") {
            None
        } else {
            self.parse_optional_alias()?
        };
        Ok(TableRef::Table {
            name,
            alias,
            partition: None,
            timecapsule: None,
        })
    }

    fn parse_order_limit_offset(&mut self, stmt: &mut SelectStatement) -> Result<(), ParserError> {
        if self.match_keyword(Keyword::ORDER) {
            self.advance();
            let siblings = self.try_consume_keyword(Keyword::SIBLINGS);
            self.expect_keyword(Keyword::BY)?;
            stmt.order_siblings = siblings;
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
        if self.match_keyword(Keyword::WINDOW) {
            self.advance();
            let mut windows = Vec::new();
            loop {
                let name = self.parse_identifier()?;
                self.expect_keyword(Keyword::AS)?;
                self.expect_token(&Token::LParen)?;
                let spec = self.parse_window_specification()?;
                self.expect_token(&Token::RParen)?;
                windows.push(crate::ast::NamedWindow { name, spec });
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            stmt.window_clause = windows;
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
        if stmt.limit.is_none() && self.match_keyword(Keyword::LIMIT) {
            self.advance();
            if self.match_keyword(Keyword::ALL) {
                self.advance();
            } else {
                stmt.limit = Some(self.parse_expr()?);
            }
        }
        stmt.fetch = self.parse_fetch_clause()?;
        stmt.lock_clause = self.parse_lock_clause()?;
        Ok(())
    }

    fn parse_fetch_clause(&mut self) -> Result<Option<FetchClause>, ParserError> {
        if !self.match_keyword(Keyword::FETCH) {
            return Ok(None);
        }
        self.advance();

        let count = if self.match_keyword(Keyword::FIRST_P) || self.match_keyword(Keyword::NEXT) {
            self.advance();
            if self.match_keyword(Keyword::ROW) || self.match_keyword(Keyword::ROWS) {
                self.advance();
                None
            } else {
                Some(self.parse_expr()?)
            }
        } else {
            None
        };

        let with_ties = if self.match_keyword(Keyword::WITH) {
            self.advance();
            self.expect_keyword(Keyword::TIES)?;
            true
        } else {
            self.try_consume_keyword(Keyword::ONLY);
            false
        };

        Ok(Some(FetchClause { count, with_ties }))
    }

    fn parse_lock_clause(&mut self) -> Result<Option<LockClause>, ParserError> {
        if !self.match_keyword(Keyword::FOR) {
            return Ok(None);
        }
        self.advance();

        let (lock_type, has_no_key) = if self.match_keyword(Keyword::UPDATE) {
            self.advance();
            (0usize, false)
        } else if self.match_keyword(Keyword::SHARE) {
            self.advance();
            (1, false)
        } else if self.match_keyword(Keyword::NO) {
            self.advance();
            self.expect_keyword(Keyword::KEY)?;
            self.expect_keyword(Keyword::UPDATE)?;
            (2, true)
        } else if self.match_keyword(Keyword::KEY) {
            self.advance();
            self.expect_keyword(Keyword::SHARE)?;
            (3, true)
        } else {
            return Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "UPDATE, SHARE, NO KEY UPDATE, or KEY SHARE".to_string(),
                got: format!("{:?}", self.peek()),
            });
        };

        let tables = if self.match_keyword(Keyword::OF) {
            self.advance();
            let mut tbls = vec![self.parse_object_name()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                tbls.push(self.parse_object_name()?);
            }
            tbls
        } else {
            vec![]
        };

        let nowait = self.try_consume_keyword(Keyword::NOWAIT);
        let skip_locked = self.try_consume_keyword(Keyword::SKIP) && {
            self.expect_keyword(Keyword::LOCKED)?;
            true
        };

        let clause = match lock_type {
            0 => LockClause::Update {
                tables,
                nowait,
                skip_locked,
            },
            1 => LockClause::Share {
                tables,
                nowait,
                skip_locked,
            },
            2 => LockClause::NoKeyUpdate {
                tables,
                nowait,
                skip_locked,
            },
            _ => LockClause::KeyShare {
                tables,
                nowait,
                skip_locked,
            },
        };

        Ok(Some(clause))
    }

    fn parse_pivot_alias(&mut self) -> Result<Option<String>, ParserError> {
        if self.match_keyword(Keyword::AS) {
            self.advance();
            let alias = match self.peek().clone() {
                Token::Ident(_) | Token::QuotedIdent(_) => self.parse_identifier()?,
                Token::StringLiteral(s) => {
                    self.advance();
                    s
                }
                _ => {
                    return Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "identifier or string".to_string(),
                        got: format!("{:?}", self.peek()),
                    })
                }
            };
            Ok(Some(alias))
        } else {
            Ok(None)
        }
    }

    fn parse_pivot(&mut self) -> Result<PivotClause, ParserError> {
        self.expect_token(&Token::LParen)?;
        let aggregate = self.parse_expr()?;
        self.expect_keyword(Keyword::FOR)?;
        let for_column = self.parse_object_name()?;
        self.expect_keyword(Keyword::IN_P)?;
        self.expect_token(&Token::LParen)?;
        let mut values = Vec::new();
        loop {
            let value = self.parse_expr()?;
            let alias = self.parse_pivot_alias()?;
            values.push(PivotValue { value, alias });
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }
        self.expect_token(&Token::RParen)?;
        self.expect_token(&Token::RParen)?;
        Ok(PivotClause {
            aggregate,
            for_column,
            values,
        })
    }

    fn parse_unpivot(&mut self) -> Result<UnpivotClause, ParserError> {
        self.expect_token(&Token::LParen)?;
        let value_column = self.parse_object_name()?;
        self.expect_keyword(Keyword::FOR)?;
        let for_column = self.parse_object_name()?;
        self.expect_keyword(Keyword::IN_P)?;
        self.expect_token(&Token::LParen)?;
        let mut columns = Vec::new();
        loop {
            let value = self.parse_expr()?;
            let alias = self.parse_pivot_alias()?;
            columns.push(PivotValue { value, alias });
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }
        self.expect_token(&Token::RParen)?;
        self.expect_token(&Token::RParen)?;
        Ok(UnpivotClause {
            value_column,
            for_column,
            columns,
        })
    }

    pub(crate) fn parse_values_statement(&mut self) -> Result<ValuesStatement, ParserError> {
        let mut rows = Vec::new();
        loop {
            self.expect_token(&Token::LParen)?;
            let mut row = Vec::new();
            if !self.match_token(&Token::RParen) {
                loop {
                    row.push(self.parse_expr()?);
                    if self.match_token(&Token::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            self.expect_token(&Token::RParen)?;
            rows.push(row);
            if self.match_token(&Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        let mut order_by = Vec::new();
        if self.match_keyword(Keyword::ORDER) {
            self.advance();
            self.expect_keyword(Keyword::BY)?;
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
                order_by.push(OrderByItem {
                    expr,
                    asc,
                    nulls_first,
                });
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
        }

        let limit = if self.match_keyword(Keyword::LIMIT) {
            self.advance();
            if self.match_keyword(Keyword::ALL) {
                self.advance();
                None
            } else {
                Some(self.parse_expr()?)
            }
        } else {
            None
        };

        let offset = if self.match_keyword(Keyword::OFFSET) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };

        Ok(ValuesStatement {
            rows,
            order_by,
            limit,
            offset,
        })
    }

    fn parse_optional_func_col_type(&mut self) -> Result<crate::ast::DataType, ParserError> {
        use crate::ast::DataType;
        if self.match_token(&Token::Comma) || self.match_token(&Token::RParen) {
            return Ok(DataType::Text);
        }
        self.parse_data_type()
    }
}
