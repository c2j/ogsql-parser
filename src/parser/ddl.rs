use crate::ast::{
    AlterColumnAction, AlterCompositeTypeStatement, AlterExtensionStatement, AlterIndexAction,
    AlterIndexStatement, AlterTableAction, AlterTableStatement, AlterTriggerStatement,
    AlterTypeAction, AlterViewAction, AlterViewStatement, CheckOption, ColumnConstraint, ColumnDef,
    CreateDatabaseStatement, CreateGroupStatement, CreateIndexStatement, CreateRoleStatement,
    CreateSchemaStatement, CreateSequenceStatement, CreateTableStatement,
    CreateTablespaceStatement, CreateTypeStatement, CreateUserStatement, CreateViewStatement,
    DataType, DropStatement, IndexColumn, ObjectType, OnCommitAction, PartitionClause, RoleOption,
    SchemaElement, TableConstraint, TimeZoneInfo, TruncateStatement, TypeAttribute, TypeKind,
};
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;

pub(crate) fn format_data_type(dt: &DataType) -> String {
    match dt {
        DataType::Boolean => "boolean".to_string(),
        DataType::SmallInt => "smallint".to_string(),
        DataType::Integer => "integer".to_string(),
        DataType::BigInt => "bigint".to_string(),
        DataType::Real => "real".to_string(),
        DataType::Double => "double precision".to_string(),
        DataType::Text => "text".to_string(),
        DataType::Char(Some(n)) => format!("char({})", n),
        DataType::Char(None) => "char".to_string(),
        DataType::Varchar(Some(n)) => format!("varchar({})", n),
        DataType::Varchar(None) => "varchar".to_string(),
        DataType::Numeric(Some(p), Some(s)) => format!("numeric({},{})", p, s),
        DataType::Numeric(Some(p), None) => format!("numeric({})", p),
        DataType::Numeric(None, _) => "numeric".to_string(),
        DataType::Date => "date".to_string(),
        DataType::Timestamp(Some(p), _) => format!("timestamp({})", p),
        DataType::Timestamp(None, _) => "timestamp".to_string(),
        DataType::Uuid => "uuid".to_string(),
        DataType::Json => "json".to_string(),
        DataType::Jsonb => "jsonb".to_string(),
        DataType::Bytea => "bytea".to_string(),
        DataType::Custom(obj) => obj.join("."),
        other => format!("{:?}", other).to_lowercase(),
    }
}

impl Parser {
    // ========== CREATE TABLE ==========

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
                partition_by = Some(match strategy {
                    "range" => PartitionClause::Range { column },
                    "list" => PartitionClause::List { column },
                    _ => PartitionClause::Hash { column },
                });
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
            tablespace,
            on_commit,
            options,
        })
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

    fn parse_column_def(&mut self) -> Result<ColumnDef, ParserError> {
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
            Some(Keyword::REAL) | Some(Keyword::FLOAT_P) => {
                self.advance();
                Ok(DataType::Real)
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
                let name = self.parse_object_name()?;
                Ok(DataType::Custom(name))
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

    fn parse_table_constraint(&mut self) -> Result<TableConstraint, ParserError> {
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

    fn parse_if_exists(&mut self) -> bool {
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
                if self.match_keyword(Keyword::COLUMN) {
                    self.advance();
                }
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
            Some(Keyword::DROP) => {
                self.advance();
                if self.match_keyword(Keyword::COLUMN) {
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
                if self.match_keyword(Keyword::COLUMN) {
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
                } else {
                    Err(ParserError::UnexpectedToken {
                        location: self.current_location(),
                        expected: "SCHEMA".to_string(),
                        got: format!("{:?}", self.peek()),
                    })
                }
            }
            _ => Err(ParserError::UnexpectedToken {
                location: self.current_location(),
                expected: "ALTER TABLE action".to_string(),
                got: format!("{:?}", self.peek()),
            }),
        }
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

    pub(crate) fn parse_drop(&mut self) -> Result<DropStatement, ParserError> {
        let obj_type = match self.peek_keyword() {
            Some(Keyword::TABLE) => {
                self.advance();
                ObjectType::Table
            }
            Some(Keyword::INDEX) => {
                self.advance();
                ObjectType::Index
            }
            Some(Keyword::SEQUENCE) => {
                self.advance();
                ObjectType::Sequence
            }
            Some(Keyword::VIEW) => {
                self.advance();
                ObjectType::View
            }
            Some(Keyword::SCHEMA) => {
                self.advance();
                ObjectType::Schema
            }
            Some(Keyword::DATABASE) => {
                self.advance();
                ObjectType::Database
            }
            Some(Keyword::TABLESPACE) => {
                self.advance();
                ObjectType::Tablespace
            }
            Some(Keyword::MATERIALIZED) => {
                self.advance();
                self.expect_keyword(Keyword::VIEW)?;
                ObjectType::MaterializedView
            }
            Some(Keyword::FUNCTION) => {
                self.advance();
                ObjectType::Function
            }
            Some(Keyword::PROCEDURE) => {
                self.advance();
                ObjectType::Procedure
            }
            Some(Keyword::TRIGGER) => {
                self.advance();
                ObjectType::Trigger
            }
            Some(Keyword::EXTENSION) => {
                self.advance();
                ObjectType::Extension
            }
            Some(Keyword::FOREIGN) => {
                self.advance();
                if self.match_keyword(Keyword::TABLE) {
                    self.advance();
                    ObjectType::ForeignTable
                } else if self.match_keyword(Keyword::DATA_P) {
                    self.advance();
                    self.expect_keyword(Keyword::WRAPPER)?;
                    ObjectType::Fdw
                } else {
                    self.expect_keyword(Keyword::SERVER)?;
                    ObjectType::ForeignServer
                }
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "DROP object type".to_string(),
                    got: format!("{:?}", self.peek()),
                });
            }
        };
        let if_exists = self.parse_if_exists();
        self.parse_drop_statement_with_type(obj_type, if_exists)
    }

    fn parse_drop_statement_with_type(
        &mut self,
        object_type: ObjectType,
        if_exists: bool,
    ) -> Result<DropStatement, ParserError> {
        let mut names = vec![self.parse_object_name()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            names.push(self.parse_object_name()?);
        }
        let cascade = self.try_consume_keyword(Keyword::CASCADE);
        let purge = self.try_consume_keyword(Keyword::PURGE);
        Ok(DropStatement {
            object_type,
            if_exists,
            names,
            cascade,
            purge,
        })
    }

    // ========== CREATE INDEX ==========

    pub(crate) fn parse_create_index(&mut self) -> Result<CreateIndexStatement, ParserError> {
        self.expect_keyword(Keyword::INDEX)?;

        let concurrent = self.try_consume_keyword(Keyword::CONCURRENTLY);
        let unique = self.try_consume_keyword(Keyword::UNIQUE);
        let if_not_exists = self.parse_if_not_exists();

        let name = if !matches!(self.peek(), Token::Keyword(Keyword::ON)) {
            Some(self.parse_identifier()?)
        } else {
            None
        };

        self.expect_keyword(Keyword::ON)?;
        let table = self.parse_object_name()?;

        self.expect_token(&Token::LParen)?;
        let mut columns = vec![self.parse_index_column()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            columns.push(self.parse_index_column()?);
        }
        self.expect_token(&Token::RParen)?;

        let mut tablespace = None;
        let mut where_clause = None;

        loop {
            if self.match_keyword(Keyword::TABLESPACE) {
                self.advance();
                tablespace = Some(self.parse_identifier()?);
            } else if self.match_keyword(Keyword::WHERE) {
                self.advance();
                where_clause = Some(self.parse_expr()?);
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
            columns,
            tablespace,
            where_clause,
        })
    }

    fn parse_index_column(&mut self) -> Result<IndexColumn, ParserError> {
        let name = self.parse_identifier()?;

        let asc = if self.match_keyword(Keyword::ASC) {
            self.advance();
            Some(true)
        } else if self.match_keyword(Keyword::DESC) {
            self.advance();
            Some(false)
        } else {
            None
        };

        Ok(IndexColumn { name, asc })
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
        let restart_identity = self.try_consume_keyword(Keyword::RESTART);

        Ok(TruncateStatement {
            tables,
            cascade,
            restart_identity,
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

        Ok(CreateViewStatement {
            replace: false,
            temporary: false,
            recursive: false,
            name,
            columns,
            query,
            check_option,
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

        let mut elements = Vec::new();
        loop {
            if self.match_keyword(Keyword::CREATE) {
                self.advance();
                let element = match self.peek_keyword() {
                    Some(Keyword::TABLE) => {
                        SchemaElement::Table(self.parse_create_table(false, false)?)
                    }
                    Some(Keyword::INDEX) => SchemaElement::Index(self.parse_create_index()?),
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
                Token::Keyword(kw) => format!("{:?}", kw)
                    .to_lowercase()
                    .trim_end_matches("_p")
                    .to_string(),
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
                Ok(format!("{:?}", kw)
                    .to_lowercase()
                    .trim_end_matches("_p")
                    .to_string())
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

        self.expect_keyword(Keyword::LOCATION)?;
        let location = match self.peek().clone() {
            Token::StringLiteral(s) => {
                self.advance();
                s
            }
            Token::DollarString(s) => {
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

        Ok(CreateTablespaceStatement {
            name,
            owner,
            location,
        })
    }

    pub(crate) fn parse_create_type(&mut self) -> Result<CreateTypeStatement, ParserError> {
        self.expect_keyword(Keyword::TYPE_P)?;
        let name = self.parse_object_name()?;

        let type_kind = if self.try_consume_keyword(Keyword::AS) {
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
                    let dt = self.parse_data_type()?;
                    let type_str = format_data_type(&dt);
                    attributes.push(TypeAttribute {
                        name: attr_name,
                        data_type: type_str,
                    });
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                    self.advance();
                }
                self.expect_token(&Token::RParen)?;
                TypeKind::Composite { attributes }
            } else {
                return Err(ParserError::UnexpectedToken {
                    location: self.current_location(),
                    expected: "ENUM or (".to_string(),
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
                        let pwd = self.parse_string_literal()?;
                        options.push(RoleOption::EncryptedPassword(pwd));
                    }
                } else if self.match_keyword(Keyword::UNENCRYPTED) {
                    self.advance();
                    if self.match_keyword(Keyword::PASSWORD) || self.match_ident_str("PASSWORD") {
                        self.advance();
                        let pwd = self.parse_string_literal()?;
                        options.push(RoleOption::UnencryptedPassword(pwd));
                    }
                } else if self.match_ident_str("PASSWORD") {
                    self.advance();
                    let pwd = self.parse_string_literal()?;
                    options.push(RoleOption::UnencryptedPassword(pwd));
                } else if self.match_keyword(Keyword::VALID) {
                    self.advance();
                    self.expect_keyword(Keyword::UNTIL)?;
                    let until = self.parse_string_literal()?;
                    options.push(RoleOption::ValidUntil(until));
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
                } else {
                    break;
                }
            }
        }

        Ok((name, options))
    }

    pub(crate) fn parse_alter_index(&mut self) -> Result<AlterIndexStatement, ParserError> {
        self.expect_keyword(Keyword::INDEX)?;
        let if_exists = self.parse_if_exists();
        let name = self.parse_object_name()?;

        let action = if self.match_keyword(Keyword::RENAME) {
            self.advance();
            self.expect_keyword(Keyword::TO)?;
            let new_name = self.parse_identifier()?;
            AlterIndexAction::RenameTo(new_name)
        } else if self.match_keyword(Keyword::SET) {
            self.advance();
            if self.match_keyword(Keyword::TABLESPACE) {
                self.advance();
                let ts = self.parse_identifier()?;
                AlterIndexAction::SetTablespace(ts)
            } else {
                let options = self.parse_generic_options();
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
        self.expect_keyword(Keyword::ON)?;
        let table = self.parse_object_name()?;
        self.expect_keyword(Keyword::RENAME)?;
        self.expect_keyword(Keyword::TO)?;
        let new_name = self.parse_identifier()?;
        Ok(AlterTriggerStatement {
            name,
            table,
            new_name,
        })
    }

    pub(crate) fn parse_alter_extension(&mut self) -> Result<AlterExtensionStatement, ParserError> {
        self.expect_keyword(Keyword::EXTENSION)?;
        let name = self.parse_identifier()?;
        let action = self.collect_rest_until_semicolon();
        Ok(AlterExtensionStatement { name, action })
    }
}
