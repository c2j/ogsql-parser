use crate::ast::{Expr, SelectTarget, Statement, StatementInfo};
use crate::linter::type_helpers::{
    build_column_type_map, classify_type_family, literal_type_family, resolve_column_type,
};
use crate::linter::{
    loc_from_spanned, make_warning, stmt_location, walk_expr, Confidence, LintConfig, LintRuleEntry, SqlLinter,
    SqlWarning, StatementKind, WarningLevel,
};

pub fn register(linter: &mut SqlLinter) {
    let rules: Vec<LintRuleEntry> = vec![
        LintRuleEntry {
            id: "R001",
            name: "select-star",
            level: WarningLevel::Prohibition,
            stmt_kind: StatementKind::Select,
            check_fn: check_r001,
        },
        LintRuleEntry {
            id: "R002",
            name: "large-column-sort",
            level: WarningLevel::Prohibition,
            stmt_kind: StatementKind::Select,
            check_fn: check_r002,
        },
        LintRuleEntry {
            id: "R003",
            name: "lock-table",
            level: WarningLevel::Prohibition,
            stmt_kind: StatementKind::All,
            check_fn: check_r003,
        },
        LintRuleEntry {
            id: "R004",
            name: "drop-cascade",
            level: WarningLevel::Prohibition,
            stmt_kind: StatementKind::All,
            check_fn: check_r004,
        },
        LintRuleEntry {
            id: "R005",
            name: "implicit-type-conversion",
            level: WarningLevel::Prohibition,
            stmt_kind: StatementKind::Select,
            check_fn: check_r005,
        },
        LintRuleEntry {
            id: "R006",
            name: "function-on-where-column",
            level: WarningLevel::Prohibition,
            stmt_kind: StatementKind::Dml,
            check_fn: check_r006,
        },
        LintRuleEntry {
            id: "R007",
            name: "like-leading-wildcard",
            level: WarningLevel::Prohibition,
            stmt_kind: StatementKind::Dml,
            check_fn: check_r007,
        },
        LintRuleEntry {
            id: "R008",
            name: "same-table-column-compare",
            level: WarningLevel::Caution,
            stmt_kind: StatementKind::Dml,
            check_fn: check_r008,
        },
        LintRuleEntry {
            id: "R009",
            name: "scalar-subquery-in-select",
            level: WarningLevel::Performance,
            stmt_kind: StatementKind::Select,
            check_fn: check_r009,
        },
    ];
    for rule in rules {
        linter.register(rule);
    }
}

// R001: SELECT * (unqualified)
fn check_r001(
    curr_stmt: &StatementInfo,
    _stmts: &[StatementInfo],
    _schema: Option<&crate::analyzer::schema::SchemaMap>,
    _indexes: Option<&crate::linter::IndexInfo>,
    _config: &LintConfig,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    if let Statement::Select(s) = &curr_stmt.statement {
        let loc = loc_from_spanned(s, stmt_location(curr_stmt));
        for target in &s.targets {
            if let SelectTarget::Star(None) = target {
                warnings.push(make_warning(
                    WarningLevel::Prohibition, "R001", "select-star",
                    "SELECT * \u{8fdd}\u{53cd} GaussDB \u{7f16}\u{7801}\u{89c4}\u{8303}\u{ff1a}\u{8868}\u{7ed3}\u{6784}\u{53d8}\u{5316}\u{65f6}\u{53ef}\u{80fd}\u{5bfc}\u{81f4}\u{4e0d}\u{517c}\u{5bb9}".into(),
                    Some("\u{660e}\u{786e}\u{5217}\u{51fa}\u{6240}\u{9700}\u{5b57}\u{6bb5}\u{540d}"), loc,
                    Some("\u{5f00}\u{53d1}\u{8bbe}\u{8ba1}\u{5efa}\u{8bae} > SELECT \u{89c4}\u{8303}"), confidence,
                ));
            }
        }
    }
}

// R002: Large column sort / group by / distinct
fn check_r002(
    curr_stmt: &StatementInfo,
    _stmts: &[StatementInfo],
    _schema: Option<&crate::analyzer::schema::SchemaMap>,
    _indexes: Option<&crate::linter::IndexInfo>,
    config: &LintConfig,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    if let Statement::Select(s) = &curr_stmt.statement {
        let loc = loc_from_spanned(s, stmt_location(curr_stmt));
        let group_count = s.group_by.len();
        let order_count = s.order_by.len();
        if group_count > config.group_by_column_limit {
            warnings.push(make_warning(
                WarningLevel::Prohibition, "R002", "large-column-sort",
                format!("GROUP BY \u{5305}\u{542b} {group_count} \u{4e2a}\u{8868}\u{8fbe}\u{5f0f}\u{ff0c}\u{8d85}\u{8fc7}\u{9608}\u{503c} {} \u{ff0c}\u{53ef}\u{80fd}\u{5bfc}\u{81f4}\u{6027}\u{80fd}\u{95ee}\u{9898}", config.group_by_column_limit),
                Some("\u{7b80}\u{5316} GROUP BY\u{ff0c}\u{51cf}\u{5c11}\u{5206}\u{7ec4}\u{5217}\u{6570}\u{91cf}"), loc,
                Some("SELECT \u{89c4}\u{8303}"), confidence,
            ));
        }
        if order_count > config.group_by_column_limit {
            warnings.push(make_warning(
                WarningLevel::Prohibition, "R002", "large-column-sort",
                format!("ORDER BY \u{5305}\u{542b} {order_count} \u{4e2a}\u{8868}\u{8fbe}\u{5f0f}\u{ff0c}\u{8d85}\u{8fc7}\u{9608}\u{503c} {} \u{ff0c}\u{53ef}\u{80fd}\u{5bfc}\u{81f4}\u{6027}\u{80fd}\u{95ee}\u{9898}", config.group_by_column_limit),
                Some("\u{7b80}\u{5316} ORDER BY\u{ff0c}\u{51cf}\u{5c11}\u{6392}\u{5e8f}\u{5217}\u{6570}\u{91cf}"), loc,
                Some("SELECT \u{89c4}\u{8303}"), confidence,
            ));
        }
    }
}

// R003: LOCK TABLE
fn check_r003(
    curr_stmt: &StatementInfo,
    _stmts: &[StatementInfo],
    _schema: Option<&crate::analyzer::schema::SchemaMap>,
    _indexes: Option<&crate::linter::IndexInfo>,
    _config: &LintConfig,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    if let Statement::Lock(s) = &curr_stmt.statement {
        let loc = loc_from_spanned(s, stmt_location(curr_stmt));
        warnings.push(make_warning(
            WarningLevel::Prohibition, "R003", "lock-table",
            "LOCK TABLE \u{53ef}\u{80fd}\u{5bfc}\u{81f4}\u{6b7b}\u{9501}\u{98ce}\u{9669}".into(),
            Some("\u{907f}\u{514d}\u{5728}\u{4e8b}\u{52a1}\u{4e2d}\u{4f7f}\u{7528} LOCK TABLE\u{ff0c}\u{4f18}\u{5148}\u{4f7f}\u{7528} SELECT ... FOR UPDATE"), loc,
            Some("SELECT \u{89c4}\u{8303}"), confidence,
        ));
    }
}

// R004: DROP ... CASCADE
fn check_r004(
    curr_stmt: &StatementInfo,
    _stmts: &[StatementInfo],
    _schema: Option<&crate::analyzer::schema::SchemaMap>,
    _indexes: Option<&crate::linter::IndexInfo>,
    _config: &LintConfig,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    if let Statement::Drop(s) = &curr_stmt.statement {
        if s.cascade {
            let loc = loc_from_spanned(s, stmt_location(curr_stmt));
            let obj_type = object_type_str(&s.object_type);
            warnings.push(make_warning(
                WarningLevel::Prohibition,
                "R004",
                "drop-cascade",
                format!("DROP {obj_type} CASCADE \u{53ef}\u{80fd}\u{8bef}\u{5220}\u{4f9d}\u{8d56}\u{5bf9}\u{8c61}"),
                Some("\u{786e}\u{8ba4}\u{4f9d}\u{8d56}\u{5173}\u{7cfb}\u{540e}\u{518d}\u{4f7f}\u{7528} CASCADE"),
                loc,
                Some("SQL \u{7f16}\u{5199}"),
                confidence,
            ));
        }
    }
}

fn object_type_str(ot: &crate::ast::ObjectType) -> &'static str {
    use crate::ast::ObjectType;
    match ot {
        ObjectType::Table => "TABLE",
        ObjectType::Index => "INDEX",
        ObjectType::Sequence => "SEQUENCE",
        ObjectType::View => "VIEW",
        ObjectType::Schema => "SCHEMA",
        ObjectType::Database => "DATABASE",
        ObjectType::Tablespace => "TABLESPACE",
        ObjectType::Function => "FUNCTION",
        ObjectType::Procedure => "PROCEDURE",
        ObjectType::Trigger => "TRIGGER",
        ObjectType::Extension => "EXTENSION",
        _ => "OBJECT",
    }
}

// R005: Implicit type conversion (schema-aware)
fn check_r005(
    curr_stmt: &StatementInfo,
    _stmts: &[StatementInfo],
    schema: Option<&crate::analyzer::schema::SchemaMap>,
    _indexes: Option<&crate::linter::IndexInfo>,
    _config: &LintConfig,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    // Without schema, R005 cannot distinguish genuine cross-family implicit
    // type conversion from safe same-family comparisons (e.g. varchar_col =
    // 'str'). Skip entirely to avoid false positives on legitimate col =
    // literal patterns. Mirrors S007's "no schema → skip, no evidence"
    // approach. See issue #240.
    let Some(schema) = schema else {
        return;
    };

    let loc = stmt_location(curr_stmt);
    // Handle non-SELECT/UPDATE/DELETE statements with fallback check
    if !matches!(curr_stmt.statement, Statement::Select(_) | Statement::Update(_) | Statement::Delete(_)) {
        let Some(wc) = extract_where_clause(&curr_stmt.statement) else { return };
        check_r005_fallback(wc, loc, confidence, warnings);
        return;
    }
    let (where_clause, tables) = match &curr_stmt.statement {
        Statement::Select(s) => (s.where_clause.as_ref(), &s.from),
        Statement::Update(s) => (s.where_clause.as_ref(), &s.tables),
        Statement::Delete(s) => (s.where_clause.as_ref(), &s.tables),
        _ => unreachable!(),
    };
    let Some(where_clause) = where_clause else { return };

    let col_types = build_column_type_map(schema, tables);

    let mut found = false;
    walk_expr(where_clause, &mut |e| {
        if found {
            return false;
        }
        if let Expr::BinaryOp { left, right, .. } = e {
            let l_lit = matches!(left.as_ref(), Expr::Literal(_));
            let r_lit = matches!(right.as_ref(), Expr::Literal(_));
            let l_col = matches!(left.as_ref(), Expr::ColumnRef(_));
            let r_col = matches!(right.as_ref(), Expr::ColumnRef(_));
            if !((l_lit && r_col) || (l_col && r_lit)) {
                return true;
            }

            let (lit_expr, col_expr) =
                if l_lit { (left.as_ref(), right.as_ref()) } else { (right.as_ref(), left.as_ref()) };
            if let (Expr::Literal(lit), Some(col_type)) = (lit_expr, resolve_column_type(col_expr, &col_types)) {
                let lit_family = literal_type_family(lit);
                let col_family = classify_type_family(&col_type);
                if lit_family == col_family {
                    return true;
                }
            }
            found = true;
            return false;
        }
        true
    });
    if found {
        warnings.push(make_warning(
                WarningLevel::Prohibition, "R005", "implicit-type-conversion",
                "WHERE \u{4e2d}\u{53ef}\u{80fd}\u{5b58}\u{5728}\u{9690}\u{5f0f}\u{7c7b}\u{578b}\u{8f6c}\u{6362}\u{ff0c}\u{53ef}\u{80fd}\u{5bfc}\u{81f4}\u{7d22}\u{5f15}\u{5931}\u{6548}".into(),
                Some("\u{663e}\u{5f0f}\u{6dfb}\u{52a0}\u{7c7b}\u{578b}\u{8f6c}\u{6362}\u{ff0c}\u{907f}\u{514d}\u{9690}\u{5f0f}\u{8f6c}\u{6362}\u{5bfc}\u{81f4}\u{7d22}\u{5f15}\u{5931}\u{6548}"), loc,
                Some("WHERE \u{89c4}\u{8303}"), confidence,
            ));
    }
}

fn check_r005_fallback(
    where_clause: &Expr,
    loc: crate::token::SourceLocation,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    let mut found = false;
    walk_expr(where_clause, &mut |e| {
        if found {
            return false;
        }
        if let Expr::BinaryOp { left, right, .. } = e {
            let l_lit = matches!(**left, Expr::Literal(_));
            let r_lit = matches!(**right, Expr::Literal(_));
            let l_col = matches!(**left, Expr::ColumnRef(_));
            let r_col = matches!(**right, Expr::ColumnRef(_));
            if (l_lit && r_col) || (l_col && r_lit) {
                found = true;
                return false;
            }
        }
        true
    });
    if found {
        warnings.push(make_warning(
                    WarningLevel::Prohibition, "R005", "implicit-type-conversion",
                    "WHERE \u{4e2d}\u{53ef}\u{80fd}\u{5b58}\u{5728}\u{9690}\u{5f0f}\u{7c7b}\u{578b}\u{8f6c}\u{6362}\u{ff08}\u{9700}\u{7ed3}\u{5408}\u{5b57}\u{6bb5}\u{7c7b}\u{578b}\u{786e}\u{8ba4}\u{ff09}".into(),
                    Some("\u{663e}\u{5f0f}\u{6dfb}\u{52a0}\u{7c7b}\u{578b}\u{8f6c}\u{6362}\u{ff0c}\u{907f}\u{514d}\u{9690}\u{5f0f}\u{8f6c}\u{6362}\u{5bfc}\u{81f4}\u{7d22}\u{5f15}\u{5931}\u{6548}"), loc,
                    Some("WHERE \u{89c4}\u{8303}"), confidence,
                ));
    }
}

// R006: Function wrapping column in WHERE (index-killing pattern)
const SAFE_FUNCTIONS_ON_COLUMNS: &[&str] = &["coalesce", "nvl", "nvl2", "ifnull", "isnull", "greatest", "least"];

fn check_r006(
    curr_stmt: &StatementInfo,
    _stmts: &[StatementInfo],
    _schema: Option<&crate::analyzer::schema::SchemaMap>,
    _indexes: Option<&crate::linter::IndexInfo>,
    _config: &LintConfig,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    let loc = stmt_location(curr_stmt);
    let (where_clause, tables) = where_and_tables(&curr_stmt.statement);
    let Some(where_clause) = where_clause else { return };

    walk_expr(where_clause, &mut |e| {
        if let Expr::FunctionCall { name, args, .. } = e {
            let fn_lower = name.last().map(|s| s.to_lowercase()).unwrap_or_default();
            if SAFE_FUNCTIONS_ON_COLUMNS.contains(&fn_lower.as_str()) {
                return true;
            }
            for arg in args {
                if let Expr::ColumnRef(col_ref) = arg {
                    match _indexes {
                        Some(idx_info) => {
                            let table = resolve_table_from_column(col_ref, tables);
                            let col_lower = col_ref.last().map(|s| s.to_lowercase()).unwrap_or_default();

                            let has_index = table
                                .as_ref()
                                .and_then(|t| idx_info.column_indexes.get(t))
                                .map(|cols| cols.contains(&col_lower))
                                .unwrap_or(false);

                            let has_func_index = table
                                .as_ref()
                                .map(|t| {
                                    crate::analyzer::schema::matches_function_index(
                                        &idx_info.indexes,
                                        t,
                                        &fn_lower,
                                        &col_lower,
                                    )
                                })
                                .unwrap_or(false);

                            if has_index && !has_func_index {
                                warnings.push(make_warning(
                                    WarningLevel::Prohibition,
                                    "R006",
                                    "function-on-where-column",
                                    "WHERE 中对有索引的列使用函数，将导致索引失效".into(),
                                    Some("将函数运算移到等号另一侧或使用函数索引"),
                                    loc,
                                    Some("WHERE 规范"),
                                    confidence,
                                ));
                                return false;
                            }
                        }
                        None => {
                            warnings.push(make_warning(
                                WarningLevel::Prohibition,
                                "R006",
                                "function-on-where-column",
                                "WHERE 中对列使用函数，可能导致索引失效".into(),
                                Some("将函数运算移到等号另一侧或使用函数索引"),
                                loc,
                                Some("WHERE 规范"),
                                confidence,
                            ));
                            return false;
                        }
                    }
                }
            }
        }
        if let Expr::TypeCast { expr, .. } = e {
            if let Expr::ColumnRef(col_ref) = expr.as_ref() {
                emit_index_aware_r006(col_ref, tables, _indexes, loc, confidence, warnings);
            }
        }
        if let Expr::BinaryOp { op, left, right, .. } = e {
            if is_sargability_breaking_op(op) {
                if let Expr::ColumnRef(col_ref) = left.as_ref() {
                    emit_index_aware_r006(col_ref, tables, _indexes, loc, confidence, warnings);
                }
                if let Expr::ColumnRef(col_ref) = right.as_ref() {
                    emit_index_aware_r006(col_ref, tables, _indexes, loc, confidence, warnings);
                }
            }
        }
        true
    });
}

fn is_sargability_breaking_op(op: &str) -> bool {
    matches!(op, "+" | "-" | "*" | "/" | "||")
}

fn emit_index_aware_r006(
    col_ref: &[crate::ast::Ident],
    tables: &[crate::ast::TableRef],
    _indexes: Option<&crate::linter::IndexInfo>,
    loc: crate::token::SourceLocation,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    if let Some(idx_info) = _indexes {
        let table = resolve_table_from_column(col_ref, tables);
        let col_lower = col_ref.last().map(|s| s.to_lowercase()).unwrap_or_default();
        let has_index = table
            .as_ref()
            .and_then(|t| idx_info.column_indexes.get(t))
            .map(|cols| cols.contains(&col_lower))
            .unwrap_or(false);
        if !has_index {
            return;
        }
    }
    warnings.push(make_warning(
        WarningLevel::Prohibition,
        "R006",
        "function-on-where-column",
        "WHERE 中对有索引的列使用函数或表达式，将导致索引失效".into(),
        Some("将运算移到等号另一侧"),
        loc,
        Some("WHERE 规范"),
        confidence,
    ));
}

/// Extract the WHERE clause and FROM/tables list from a statement.
fn where_and_tables(stmt: &Statement) -> (Option<&Expr>, &[crate::ast::TableRef]) {
    match stmt {
        Statement::Select(s) => (s.where_clause.as_ref(), &s.from),
        Statement::Update(s) => (s.where_clause.as_ref(), &s.tables),
        Statement::Delete(s) => (s.where_clause.as_ref(), &s.tables),
        _ => (extract_where_clause(stmt), &[]),
    }
}

/// Resolve which table a column belongs to from the FROM clause.
/// Returns the table name (lowercase), or None if unresolvable.
fn resolve_table_from_column(col_ref: &[crate::ast::Ident], tables: &[crate::ast::TableRef]) -> Option<String> {
    if col_ref.len() >= 2 {
        return Some(col_ref[col_ref.len() - 2].to_lowercase());
    }

    if col_ref.len() == 1 {
        for tref in tables {
            if let Some(name) = table_name(tref) {
                return Some(name.to_lowercase());
            }
        }
    }

    None
}

/// Extract the effective table name from a TableRef (handles alias).
fn table_name(tref: &crate::ast::TableRef) -> Option<&str> {
    use crate::ast::TableRef;
    match tref {
        TableRef::Table { name, alias, .. } => {
            if let Some(a) = alias {
                Some(a.as_str())
            } else {
                name.last().map(|s| s.as_str())
            }
        }
        TableRef::Join { left, .. } => table_name(left),
        TableRef::Subquery { alias, .. } | TableRef::FunctionCall { alias, .. } | TableRef::Values { alias, .. } => {
            alias.as_deref()
        }
        TableRef::Pivot { source, .. } | TableRef::Unpivot { source, .. } => table_name(source),
    }
}

// R007: LIKE with leading wildcard
fn check_r007(
    curr_stmt: &StatementInfo,
    _stmts: &[StatementInfo],
    _schema: Option<&crate::analyzer::schema::SchemaMap>,
    _indexes: Option<&crate::linter::IndexInfo>,
    _config: &LintConfig,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    let loc = stmt_location(curr_stmt);
    let (where_clause, tables) = where_and_tables(&curr_stmt.statement);
    let Some(where_clause) = where_clause else { return };
    walk_expr(where_clause, &mut |e| {
        if let Expr::Like { expr, pattern, negated: false, .. } = e {
            if let Expr::Literal(crate::ast::Literal::String(s)) = pattern.as_ref() {
                if s.starts_with('%') || s.starts_with('_') {
                    let should_warn = match _indexes {
                        Some(idx_info) => {
                            if let Expr::ColumnRef(col_ref) = expr.as_ref() {
                                let table = resolve_table_from_column(col_ref, tables);
                                let col_lower = col_ref.last().map(|s| s.to_lowercase()).unwrap_or_default();
                                table
                                    .and_then(|t| idx_info.column_indexes.get(&t))
                                    .map(|cols| cols.contains(&col_lower))
                                    .unwrap_or(false)
                            } else {
                                true
                            }
                        }
                        None => true,
                    };
                    if should_warn {
                        warnings.push(make_warning(
                            WarningLevel::Prohibition, "R007", "like-leading-wildcard",
                            format!("LIKE '\u{524d}\u{5bfc}\u{901a}\u{914d}\u{7b26} {s}' \u{5c06}\u{5bfc}\u{81f4}\u{65e0}\u{6cd5}\u{4f7f}\u{7528}\u{7d22}\u{5f15}\u{ff0c}\u{89e6}\u{53d1}\u{5168}\u{8868}\u{626b}\u{63cf}"),
                            Some("\u{907f}\u{514d}\u{4ee5}\u{901a}\u{914d}\u{7b26}\u{5f00}\u{5934}\u{7684} LIKE \u{6a21}\u{5f0f}"), loc,
                            Some("WHERE \u{89c4}\u{8303}"), confidence,
                        ));
                    }
                }
            }
        }
        true
    });
}

// R008: Same-table column comparison in WHERE
fn check_r008(
    curr_stmt: &StatementInfo,
    _stmts: &[StatementInfo],
    _schema: Option<&crate::analyzer::schema::SchemaMap>,
    _indexes: Option<&crate::linter::IndexInfo>,
    _config: &LintConfig,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    let loc = stmt_location(curr_stmt);
    if let Some(where_clause) = extract_where_clause(&curr_stmt.statement) {
        walk_expr(where_clause, &mut |e| {
            if let Expr::BinaryOp { left, right, .. } = e {
                if let (Expr::ColumnRef(l), Expr::ColumnRef(r)) = (left.as_ref(), right.as_ref()) {
                    if l.len() >= 2 && r.len() >= 2 && l[0] == r[0] {
                        warnings.push(make_warning(
                            WarningLevel::Caution, "R008", "same-table-column-compare",
                            format!("\u{540c}\u{8868}\u{5217}\u{6bd4}\u{8f83}: {}.{} \u{4e0e} {}.{}\u{ff0c}\u{53ef}\u{80fd}\u{672a}\u{6b63}\u{786e}\u{4f7f}\u{7528}\u{7d22}\u{5f15}", l[0], l.last().unwrap_or(&"".into()), r[0], r.last().unwrap_or(&"".into())),
                            Some("\u{68c0}\u{67e5}\u{662f}\u{5426}\u{5e94}\u{4f7f}\u{7528}\u{4e0d}\u{540c}\u{8868}\u{7684}\u{5217}\u{8fdb}\u{884c}\u{6bd4}\u{8f83}"), loc,
                            Some("WHERE \u{89c4}\u{8303}"), confidence,
                        ));
                    }
                }
            }
            true
        });
    }
}

// R009: Scalar subquery in SELECT target list
fn check_r009(
    curr_stmt: &StatementInfo,
    _stmts: &[StatementInfo],
    _schema: Option<&crate::analyzer::schema::SchemaMap>,
    _indexes: Option<&crate::linter::IndexInfo>,
    _config: &LintConfig,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    if let Statement::Select(s) = &curr_stmt.statement {
        let loc = loc_from_spanned(s, stmt_location(curr_stmt));
        for target in &s.targets {
            if let SelectTarget::Expr(e, _) = target {
                let mut found = false;
                walk_expr(e, &mut |inner| {
                    if found {
                        return false;
                    }
                    if matches!(inner, Expr::Subquery(_)) {
                        found = true;
                        return false;
                    }
                    true
                });
                if found {
                    warnings.push(make_warning(
                        WarningLevel::Performance, "R009", "scalar-subquery-in-select",
                        "SELECT \u{5217}\u{4e2d}\u{5305}\u{542b}\u{6807}\u{91cf}\u{5b50}\u{67e5}\u{8be2}\u{ff0c}\u{6bcf}\u{884c}\u{90fd}\u{4f1a}\u{6267}\u{884c}\u{4e00}\u{6b21}\u{5b50}\u{67e5}\u{8be2}".into(),
                        Some("\u{6539}\u{7528} JOIN \u{66ff}\u{4ee3}\u{6807}\u{91cf}\u{5b50}\u{67e5}\u{8be2}"), loc,
                        Some("SQL \u{7f16}\u{5199}"), confidence,
                    ));
                }
            }
        }
    }
}

fn extract_where_clause(stmt: &Statement) -> Option<&Expr> {
    match stmt {
        Statement::Select(s) => s.where_clause.as_ref(),
        Statement::Update(s) => s.where_clause.as_ref(),
        Statement::Delete(s) => s.where_clause.as_ref(),
        _ => None,
    }
}
