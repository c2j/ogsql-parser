use crate::ast::plpgsql::{PlDataType, PlDeclaration, PlStatement};
use crate::ast::{Expr, Statement, StatementInfo};
use crate::linter::{
    loc_from_spanned, make_warning, stmt_location, walk_expr, Confidence, LintConfig, LintRuleEntry, SqlLinter,
    SqlWarning, StatementKind, WarningLevel,
};

pub fn register(linter: &mut SqlLinter) {
    let rules: Vec<LintRuleEntry> = vec![
        LintRuleEntry {
            id: "S001",
            name: "delete-full-table-use-truncate",
            level: WarningLevel::Suggestion,
            stmt_kind: StatementKind::Delete,
            check_fn: check_s001,
        },
        LintRuleEntry {
            id: "S002",
            name: "limit-offset-use-cursor",
            level: WarningLevel::Suggestion,
            stmt_kind: StatementKind::Select,
            check_fn: check_s002,
        },
        LintRuleEntry {
            id: "S005",
            name: "prefer-percent-type",
            level: WarningLevel::Suggestion,
            stmt_kind: StatementKind::PlBlock,
            check_fn: check_s005,
        },
        LintRuleEntry {
            id: "S006",
            name: "limit-without-order-by",
            level: WarningLevel::Suggestion,
            stmt_kind: StatementKind::Select,
            check_fn: check_s006,
        },
        LintRuleEntry {
            id: "S007",
            name: "explicit-type-for-literals",
            level: WarningLevel::Suggestion,
            stmt_kind: StatementKind::Dml,
            check_fn: check_s007,
        },
        LintRuleEntry {
            id: "S008",
            name: "complex-sql-consider-split",
            level: WarningLevel::Suggestion,
            stmt_kind: StatementKind::All,
            check_fn: check_s008,
        },
    ];
    for rule in rules {
        linter.register(rule);
    }
}

// ── S001: DELETE full table → use TRUNCATE ──

fn check_s001(
    stmts: &[StatementInfo],
    _schema: Option<&crate::analyzer::schema::SchemaMap>,
    _config: &LintConfig,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    for info in stmts {
        if let Statement::Delete(s) = &info.statement {
            if s.where_clause.is_none() {
                let loc = loc_from_spanned(s, stmt_location(info));
                warnings.push(make_warning(
                    WarningLevel::Suggestion,
                    "S001",
                    "delete-full-table-use-truncate",
                    "DELETE \u{5168}\u{8868}\u{53ef}\u{7528} TRUNCATE \u{66ff}\u{4ee3}\u{ff0c}\u{91ca}\u{653e}\u{7a7a}\u{95f4}\u{66f4}\u{5feb}".into(),
                    Some("\u{4f7f}\u{7528} TRUNCATE TABLE \u{66ff}\u{4ee3} DELETE \u{65e0} WHERE"),
                    loc,
                    None,
                    confidence,
                ));
            }
        }
    }
}

// ── S002: LIMIT + OFFSET → use cursor ──

fn check_s002(
    stmts: &[StatementInfo],
    _schema: Option<&crate::analyzer::schema::SchemaMap>,
    _config: &LintConfig,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    for info in stmts {
        if let Statement::Select(s) = &info.statement {
            if s.offset.is_some() {
                let loc = loc_from_spanned(s, stmt_location(info));
                warnings.push(make_warning(
                    WarningLevel::Suggestion,
                    "S002",
                    "limit-offset-use-cursor",
                    "OFFSET \u{5206}\u{9875}\u{5728}\u{5927}\u{504f}\u{79fb}\u{65f6}\u{6027}\u{80fd}\u{8f83}\u{5dee}"
                        .into(),
                    Some(
                        "\u{8003}\u{8651}\u{4f7f}\u{7528}\u{6e38}\u{6807}\u{7ffb}\u{9875}\u{66ff}\u{4ee3} LIMIT/OFFSET",
                    ),
                    loc,
                    None,
                    confidence,
                ));
            }
        }
    }
}

// ── S005: Prefer %TYPE anchored types ──

fn check_s005(
    stmts: &[StatementInfo],
    _schema: Option<&crate::analyzer::schema::SchemaMap>,
    _config: &LintConfig,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    for info in stmts {
        let loc = stmt_location(info);
        let mut found = false;
        walk_pl_for_type_name(&info.statement, &mut found);
        if found {
            warnings.push(make_warning(
                WarningLevel::Suggestion,
                "S005",
                "prefer-percent-type",
                "PL/pgSQL \u{53d8}\u{91cf}\u{4f7f}\u{7528}\u{666e}\u{901a}\u{7c7b}\u{578b}\u{800c}\u{975e} %TYPE/%ROWTYPE \u{951a}\u{5b9a}".into(),
                Some("\u{4f7f}\u{7528} table.column%TYPE \u{6216} table%ROWTYPE \u{951a}\u{5b9a}\u{7c7b}\u{578b}\u{4ee5}\u{63d0}\u{9ad8}\u{53ef}\u{7ef4}\u{62a4}\u{6027}"),
                loc,
                None,
                confidence,
            ));
        }
    }
}

fn walk_pl_for_type_name(stmt: &Statement, found: &mut bool) {
    let block = match stmt {
        Statement::AnonyBlock(b) => &b.block,
        Statement::Do(d) => {
            if let Some(ref block) = d.block {
                block
            } else {
                return;
            }
        }
        _ => return,
    };
    check_decls_for_type_name(&block.declarations, found);
    if !*found {
        check_pl_stmts_for_type_name(&block.body, found);
    }
}

fn check_decls_for_type_name(decls: &[PlDeclaration], found: &mut bool) {
    for d in decls {
        if let PlDeclaration::Variable(v) = d {
            if let PlDataType::TypeName(name) = &v.data_type {
                let lower = name.to_lowercase();
                let is_simple = [
                    "integer",
                    "int",
                    "bigint",
                    "smallint",
                    "serial",
                    "bigserial",
                    "text",
                    "varchar",
                    "char",
                    "character",
                    "boolean",
                    "bool",
                    "numeric",
                    "decimal",
                    "real",
                    "double precision",
                    "float",
                    "date",
                    "timestamp",
                    "timestamptz",
                    "time",
                    "timetz",
                    "interval",
                    "bytea",
                    "uuid",
                    "json",
                    "jsonb",
                ]
                .contains(&lower.as_str());
                if !is_simple {
                    *found = true;
                    return;
                }
            }
        }
    }
}

fn check_pl_stmts_for_type_name(pl_stmts: &[PlStatement], found: &mut bool) {
    if *found {
        return;
    }
    for s in pl_stmts {
        if let PlStatement::Block(b) = s {
            check_decls_for_type_name(&b.declarations, found);
            if !*found {
                check_pl_stmts_for_type_name(&b.body, found);
            }
        }
        if *found {
            return;
        }
    }
}

// ── S006: LIMIT without ORDER BY ──

fn check_s006(
    stmts: &[StatementInfo],
    _schema: Option<&crate::analyzer::schema::SchemaMap>,
    _config: &LintConfig,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    for info in stmts {
        if let Statement::Select(s) = &info.statement {
            let has_limit = s.limit.is_some() || s.fetch.is_some() || matches!(&s.fetch, Some(f) if f.count.is_some());
            if has_limit && s.order_by.is_empty() {
                let loc = loc_from_spanned(s, stmt_location(info));
                warnings.push(make_warning(
                    WarningLevel::Suggestion,
                    "S006",
                    "limit-without-order-by",
                    "LIMIT \u{65e0} ORDER BY\u{ff0c}\u{7ed3}\u{679c}\u{987a}\u{5e8f}\u{4e0d}\u{786e}\u{5b9a}".into(),
                    Some("\u{6dfb}\u{52a0} ORDER BY \u{4fdd}\u{8bc1}\u{7ed3}\u{679c}\u{786e}\u{5b9a}\u{6027}"),
                    loc,
                    None,
                    confidence,
                ));
            }
        }
    }
}

// ── S007: Explicit type for literals in WHERE ──

fn check_s007(
    stmts: &[StatementInfo],
    schema: Option<&crate::analyzer::schema::SchemaMap>,
    _config: &LintConfig,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    for info in stmts {
        let loc = stmt_location(info);
        let (where_clause, tables) = match &info.statement {
            Statement::Select(s) => (s.where_clause.as_ref(), &s.from),
            Statement::Update(s) => (s.where_clause.as_ref(), &s.tables),
            Statement::Delete(s) => (s.where_clause.as_ref(), &s.tables),
            _ => continue,
        };
        let Some(where_clause) = where_clause else { continue };

        // When schema is available, build a column→type lookup so we can
        // distinguish same-family comparisons (varchar_col = 'str') from
        // cross-family ones (int_col = 'str') and avoid false positives.
        let col_types = schema.map(|s| build_column_type_map(s, tables));

        let mut found = false;
        walk_expr(where_clause, &mut |e| {
            if found {
                return false;
            }
            if let Expr::BinaryOp { left, right, op, .. } = e {
                let is_compare = matches!(op.as_str(), "=" | "<>" | "!=" | ">" | "<" | ">=" | "<=");
                if !is_compare {
                    return true;
                }
                let l_untyped = matches!(left.as_ref(), Expr::Literal(crate::ast::Literal::String(_)));
                let r_untyped = matches!(right.as_ref(), Expr::Literal(crate::ast::Literal::String(_)));
                if !l_untyped && !r_untyped {
                    return true;
                }

                // If we have schema info, check whether the comparison is
                // same-type-family (string literal vs string column) — skip those.
                if let Some(ref ct) = col_types {
                    let col_expr = if l_untyped { right.as_ref() } else { left.as_ref() };
                    if let Some(col_type) = resolve_column_type(col_expr, ct) {
                        if is_string_type(&col_type) {
                            // String literal vs string-type column — safe, no warning.
                            return true;
                        }
                    }
                    // Could not resolve column type, or cross-type-family → warn.
                }
                // No schema → still warn (backward compatible).
                found = true;
                return false;
            }
            true
        });
        if found {
            warnings.push(make_warning(
                WarningLevel::Suggestion,
                "S007",
                "explicit-type-for-literals",
                "WHERE \u{4e2d}\u{5b57}\u{7b26}\u{4e32}\u{5e38}\u{91cf}\u{672a}\u{663e}\u{5f0f}\u{6307}\u{5b9a}\u{7c7b}\u{578b}\u{ff0c}\u{53ef}\u{80fd}\u{5bfc}\u{81f4}\u{9690}\u{5f0f}\u{8f6c}\u{6362}".into(),
                Some("\u{4f7f}\u{7528}\u{663e}\u{5f0f}\u{7c7b}\u{578b}\u{8f6c}\u{6362}\u{ff1a} 'val'::type \u{6216} CAST('val' AS type)"),
                loc,
                None,
                confidence,
            ));
        }
    }
}

/// Build a lowercased `(table_alias_or_name, column_name) → data_type` map
/// from the schema for all tables referenced in the FROM clause.
fn build_column_type_map(
    schema: &crate::analyzer::schema::SchemaMap,
    tables: &[crate::ast::TableRef],
) -> std::collections::HashMap<(String, String), String> {
    let mut map = std::collections::HashMap::new();
    for tref in tables {
        collect_table_types(schema, tref, &mut map);
    }
    map
}

/// Recursively collect column types from a table reference (handles joins).
fn collect_table_types(
    schema: &crate::analyzer::schema::SchemaMap,
    tref: &crate::ast::TableRef,
    map: &mut std::collections::HashMap<(String, String), String>,
) {
    use crate::ast::TableRef;
    match tref {
        TableRef::Table { name, alias, .. } => {
            // ObjectName is Vec<String>, e.g. ["schema", "table"] or ["table"].
            // Try progressively shorter prefixes to match the schema key.
            let table_key = find_schema_table(schema, name);
            if let Some(columns) = table_key.and_then(|k| schema.get(k)) {
                let lookup_name = alias.as_deref().unwrap_or_else(|| name.last().unwrap_or(&name[0]));
                let lookup_lower = lookup_name.to_lowercase();
                for (col, dtype) in columns {
                    map.insert((lookup_lower.clone(), col.to_lowercase()), dtype.clone());
                }
            }
        }
        TableRef::Join { left, right, .. } => {
            collect_table_types(schema, left, map);
            collect_table_types(schema, right, map);
        }
        TableRef::Subquery { .. } | TableRef::FunctionCall { .. } | TableRef::Values { .. } => {}
        TableRef::Pivot { source, .. } | TableRef::Unpivot { source, .. } => {
            collect_table_types(schema, source, map);
        }
    }
}

/// Find the schema key that matches the given table name, trying
/// `schema.table`, then `table` alone.
fn find_schema_table<'a>(schema: &'a crate::analyzer::schema::SchemaMap, name: &[String]) -> Option<&'a String> {
    if name.is_empty() {
        return None;
    }
    // Try "schema.table" as key.
    if name.len() >= 2 {
        let full = format!("{}.{}", name[0].to_lowercase(), name[1].to_lowercase());
        if let Some(key) = schema.keys().find(|k| *k == &full) {
            return Some(key);
        }
    }
    // Try just "table" as key.
    let single = name.last().unwrap().to_lowercase();
    schema.keys().find(|k| *k == &single)
}

/// Try to resolve the SQL data type for a column expression using the
/// pre-built type map. Handles both `col` and `table.col` forms.
fn resolve_column_type(expr: &Expr, col_types: &std::collections::HashMap<(String, String), String>) -> Option<String> {
    match expr {
        Expr::ColumnRef(name) => {
            if name.len() == 1 {
                // Unqualified: try every table alias to find a match.
                for ((_, col), dtype) in col_types {
                    if col == &name[0].to_lowercase() {
                        return Some(dtype.clone());
                    }
                }
                None
            } else if name.len() >= 2 {
                // Qualified: table.col or schema.table.col.
                let table = name[name.len() - 2].to_lowercase();
                let col = name[name.len() - 1].to_lowercase();
                col_types.get(&(table, col)).cloned()
            } else {
                None
            }
        }
        Expr::FieldAccess { object, field } => {
            // Handle alias.col via FieldAccess.
            if let Expr::ColumnRef(obj_name) = object.as_ref() {
                if obj_name.len() == 1 {
                    let table = obj_name[0].to_lowercase();
                    let col = field.to_lowercase();
                    return col_types.get(&(table, col)).cloned();
                }
            }
            None
        }
        _ => None,
    }
}

/// Check whether a resolved SQL data type belongs to the string family
/// (varchar, char, text, bpchar, name, clob, nchar, nvarchar, etc.).
/// Comparison with a string literal against these types is safe — no
/// implicit cross-family conversion occurs.
fn is_string_type(data_type: &str) -> bool {
    let lower = data_type.to_lowercase();
    // Strip any length/precision suffix: "varchar(100)" → "varchar".
    let base = lower.split('(').next().unwrap_or(&lower).trim();
    matches!(
        base,
        "varchar"
            | "varchar2"
            | "character varying"
            | "char"
            | "character"
            | "text"
            | "bpchar"
            | "name"
            | "clob"
            | "nchar"
            | "nvarchar"
            | "nvarchar2"
            | "string"
    )
}

// ── S008: Complex SQL — consider splitting ──

fn check_s008(
    stmts: &[StatementInfo],
    _schema: Option<&crate::analyzer::schema::SchemaMap>,
    config: &LintConfig,
    confidence: Confidence,
    warnings: &mut Vec<SqlWarning>,
) {
    for info in stmts {
        if info.sql_text.len() > config.sql_length_limit {
            let loc = stmt_location(info);
            warnings.push(make_warning(
                WarningLevel::Suggestion,
                "S008",
                "complex-sql-consider-split",
                format!(
                    "SQL \u{6587}\u{672c}\u{957f}\u{5ea6} {} \u{5b57}\u{7b26}\u{ff0c}\u{8d85}\u{8fc7}\u{9608}\u{503c} {}\u{ff0c}\u{5efa}\u{8bae}\u{62c6}\u{5206}\u{7b80}\u{5316}",
                    info.sql_text.len(),
                    config.sql_length_limit
                ),
                Some("\u{5c06}\u{590d}\u{6742} SQL \u{62c6}\u{5206}\u{4e3a}\u{591a}\u{4e2a}\u{7b80}\u{5355}\u{67e5}\u{8be2}\u{6216}\u{4f7f}\u{7528} CTE"),
                loc,
                None,
                confidence,
            ));
        }
    }
}
