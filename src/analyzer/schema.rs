use crate::ast::plpgsql::{PlBlock, PlDataType, PlStatement};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub type SchemaMap = HashMap<String, HashMap<String, String>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaResolutionReport {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub resolved_types: Vec<ResolvedType>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub resolved_rowtypes: Vec<ResolvedRowType>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub unresolved: Vec<UnresolvedRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedType {
    pub table: String,
    pub column: String,
    pub resolved_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedRowType {
    pub table: String,
    pub columns: Vec<ColumnDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnresolvedRef {
    pub table: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    pub ref_kind: String,
}

/// Loads a schema definition from a JSON file.
///
/// # Errors
///
/// Returns `Err(String)` if the file cannot be read or the JSON is invalid.
pub fn load_schema(path: &str) -> Result<SchemaMap, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("Failed to read schema file '{}': {}", path, e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse schema JSON: {}", e))
}

pub fn resolve_schema(block: &PlBlock, schema: &SchemaMap) -> SchemaResolutionReport {
    let mut report =
        SchemaResolutionReport { resolved_types: Vec::new(), resolved_rowtypes: Vec::new(), unresolved: Vec::new() };
    resolve_schema_recursive(block, schema, &mut report);
    report
}

fn resolve_schema_recursive(block: &PlBlock, schema: &SchemaMap, report: &mut SchemaResolutionReport) {
    for decl in &block.declarations {
        let data_type = match decl {
            crate::ast::plpgsql::PlDeclaration::Variable(v) => &v.data_type,
            crate::ast::plpgsql::PlDeclaration::Cursor(c) => match c.return_type {
                Some(ref dt) => dt,
                None => continue,
            },
            _ => continue,
        };

        match data_type {
            PlDataType::PercentType { table, column } => {
                let table_lower = table.to_lowercase();
                let column_lower = column.to_lowercase();
                if let Some(columns) = schema.get(&table_lower) {
                    if let Some(sql_type) = columns.get(&column_lower) {
                        report.resolved_types.push(ResolvedType {
                            table: table.clone(),
                            column: column.clone(),
                            resolved_type: sql_type.clone(),
                        });
                    } else {
                        report.unresolved.push(UnresolvedRef {
                            table: table.clone(),
                            column: Some(column.clone()),
                            ref_kind: "PercentType".to_string(),
                        });
                    }
                } else {
                    report.unresolved.push(UnresolvedRef {
                        table: table.clone(),
                        column: Some(column.clone()),
                        ref_kind: "PercentType".to_string(),
                    });
                }
            }
            PlDataType::PercentRowType(table) => {
                let table_lower = table.to_lowercase();
                if let Some(columns) = schema.get(&table_lower) {
                    let cols: Vec<ColumnDef> = columns
                        .iter()
                        .map(|(name, dt)| ColumnDef { name: name.clone(), data_type: dt.clone() })
                        .collect();
                    report.resolved_rowtypes.push(ResolvedRowType { table: table.clone(), columns: cols });
                } else {
                    report.unresolved.push(UnresolvedRef {
                        table: table.clone(),
                        column: None,
                        ref_kind: "PercentRowType".to_string(),
                    });
                }
            }
            _ => {}
        }
    }

    for stmt in &block.body {
        if let Some(nested) = extract_block_from_statement(stmt) {
            resolve_schema_recursive(nested, schema, report);
        }
    }

    if let Some(ref eb) = block.exception_block {
        for handler in &eb.handlers {
            for stmt in &handler.statements {
                if let Some(nested) = extract_block_from_statement(stmt) {
                    resolve_schema_recursive(nested, schema, report);
                }
            }
        }
    }
}

fn extract_block_from_statement(stmt: &PlStatement) -> Option<&PlBlock> {
    match stmt {
        PlStatement::Block(spanned) => Some(&spanned.node),
        _ => None,
    }
}

// ── Index Metadata Types ──

/// Maps table_name → index_name → list of column names or function expressions in the index.
pub type IndexMapV2 = HashMap<String, HashMap<String, Vec<String>>>;

/// Full schema with both column type info and index metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FullSchema {
    #[serde(default)]
    pub columns: SchemaMap,
    #[serde(default)]
    pub indexes: IndexMapV2,
}

/// Load a FullSchema from a JSON file. Supports both the new format
/// (with `columns` and `indexes` fields) and the old format (just a
/// column map, which gets loaded with empty indexes).
///
/// # Errors
///
/// Returns `Err(String)` if the file cannot be read or the JSON is invalid.
pub fn load_full_schema(path: &str) -> Result<FullSchema, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("Failed to read schema file '{}': {}", path, e))?;
    // Try to parse as FullSchema first (new format)
    if let Ok(fs) = serde_json::from_str::<FullSchema>(&content) {
        return Ok(fs);
    }
    // Fall back to old format (just a SchemaMap)
    let columns: SchemaMap =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse schema JSON: {}", e))?;
    Ok(FullSchema { columns, indexes: HashMap::new() })
}

/// Build a lookup table: table(lowercase) → set of column names(lowercase)
/// that have a plain index. Function expression indexes like `lower(email)`
/// are skipped — only plain column names are included.
pub fn build_column_index_set(indexes: &IndexMapV2) -> HashMap<String, HashSet<String>> {
    let mut result: HashMap<String, HashSet<String>> = HashMap::new();
    for (table, indices) in indexes {
        let table_lower = table.to_lowercase();
        let entry = result.entry(table_lower).or_default();
        for cols in indices.values() {
            for col in cols {
                // Skip function expressions (anything containing '(' or whitespace)
                if col.contains('(') || col.contains(char::is_whitespace) {
                    continue;
                }
                entry.insert(col.to_lowercase());
            }
        }
    }
    result
}

/// Check if there's a function index matching `function_name(col_name...)`
/// for the given table. Does case-insensitive matching on the prefix
/// `function_name(col_name`.
pub fn matches_function_index(indexes: &IndexMapV2, table: &str, function_name: &str, col_name: &str) -> bool {
    let table_lower = table.to_lowercase();
    let prefix = format!("{}(", function_name.to_lowercase());
    let needle = format!("{}{}", prefix, col_name.to_lowercase());

    // Case-insensitive table lookup: iterate all entries since DDL may
    // preserve original casing while callers pass lowercase or mixed.
    for (tbl, indices) in indexes {
        if tbl.to_lowercase() != table_lower {
            continue;
        }
        for cols in indices.values() {
            for col_expr in cols {
                let col_lower = col_expr.to_lowercase();
                if col_lower.starts_with(&needle) {
                    return true;
                }
            }
        }
    }
    false
}

/// Collect DDL schema from parsed statements.
/// Extracts CREATE TABLE columns and CREATE INDEX definitions into a FullSchema.
pub fn collect_ddl_schema(stmts: &[crate::ast::StatementInfo]) -> FullSchema {
    use crate::ast::Statement;
    let mut columns: SchemaMap = HashMap::new();
    let mut indexes: IndexMapV2 = HashMap::new();

    for si in stmts {
        match &si.statement {
            Statement::CreateTable(t) => {
                let table_name = t.name.last().map(|s| s.to_lowercase()).unwrap_or_default();
                let col_map: HashMap<String, String> =
                    t.columns.iter().map(|c| (c.name.to_lowercase(), data_type_display(&c.data_type))).collect();
                columns.insert(table_name, col_map);
            }
            Statement::CreateIndex(idx) => {
                let table_name = idx.table.last().map(|s| s.to_lowercase()).unwrap_or_default();
                let index_name = idx
                    .name
                    .as_ref()
                    .and_then(|n| n.last())
                    .map(|s| s.to_lowercase())
                    .unwrap_or_else(|| format!("idx_{table_name}"));
                let col_entries: Vec<String> = idx
                    .columns
                    .iter()
                    .map(|ic| {
                        if let Some(ref name) = ic.name {
                            name.clone()
                        } else if let Some(ref expr) = ic.expr {
                            index_expr_to_string(expr)
                        } else {
                            String::new()
                        }
                    })
                    .filter(|s| !s.is_empty())
                    .collect();
                indexes.entry(table_name).or_default().insert(index_name, col_entries);
            }
            Statement::CreateGlobalIndex(cgi) => {
                let table_name = cgi.table.last().map(|s| s.to_lowercase()).unwrap_or_default();
                let index_name = cgi
                    .name
                    .as_ref()
                    .and_then(|n| n.last())
                    .map(|s| s.to_lowercase())
                    .unwrap_or_else(|| format!("gidx_{table_name}"));
                let col_entries: Vec<String> =
                    cgi.columns
                        .iter()
                        .map(|c| {
                            if let Some(ref expr) = c.expression {
                                index_expr_to_string(expr)
                            } else {
                                c.name.clone()
                            }
                        })
                        .filter(|s| !s.is_empty())
                        .collect();
                indexes.entry(table_name).or_default().insert(index_name, col_entries);
            }
            _ => {}
        }
    }

    FullSchema { columns, indexes }
}

fn data_type_display(dt: &crate::ast::DataType) -> String {
    use crate::ast::DataType;
    let fmt_p = |name: &str, p: &Option<u32>| match p {
        Some(n) => format!("{name}({n})"),
        None => name.to_string(),
    };
    match dt {
        DataType::Boolean => "boolean".into(),
        DataType::TinyInt(p) => fmt_p("tinyint", p),
        DataType::SmallInt(p) => fmt_p("smallint", p),
        DataType::Integer(p) => fmt_p("integer", p),
        DataType::BigInt(p) => fmt_p("bigint", p),
        DataType::Real => "real".into(),
        DataType::Float(p) => fmt_p("float", p),
        DataType::Double => "double precision".into(),
        DataType::Numeric(p, s) => match (p, s) {
            (Some(p), Some(s)) => format!("numeric({p},{s})"),
            (Some(p), None) => format!("numeric({p})"),
            _ => "numeric".into(),
        },
        DataType::Char(n) => fmt_p("char", n),
        DataType::Varchar(n) => fmt_p("varchar", n),
        DataType::Text => "text".into(),
        DataType::Bytea => "bytea".into(),
        DataType::Timestamp(p, _tz) => fmt_p("timestamp", p),
        DataType::Timestamptz(p) => fmt_p("timestamptz", p),
        DataType::Date => "date".into(),
        DataType::Time(p, _tz) => fmt_p("time", p),
        DataType::Interval(_) => "interval".into(),
        DataType::Json => "json".into(),
        DataType::Jsonb => "jsonb".into(),
        DataType::Uuid => "uuid".into(),
        DataType::Bit(n) => fmt_p("bit", n),
        DataType::Varbit(n) => fmt_p("varbit", n),
        DataType::Serial => "serial".into(),
        DataType::SmallSerial => "smallserial".into(),
        DataType::BigSerial => "bigserial".into(),
        DataType::BinaryFloat => "binary_float".into(),
        DataType::BinaryDouble => "binary_double".into(),
        DataType::Array(inner) => format!("{}[]", data_type_display(inner)),
        DataType::Custom(name, _args) => name.iter().map(|s| s.to_lowercase()).collect::<Vec<_>>().join("."),
    }
}

fn index_expr_to_string(expr: &crate::ast::Expr) -> String {
    use crate::ast::Expr;
    match expr {
        Expr::FunctionCall { name, args, .. } => {
            let fn_name = name.last().map(|s| s.to_string()).unwrap_or_default();
            let args_str: Vec<String> = args.iter().map(index_expr_to_string).collect();
            format!("{fn_name}({})", args_str.join(", "))
        }
        Expr::ColumnRef(names) => names.join("."),
        Expr::TypeCast { expr, type_name, .. } => {
            format!("{}::{}", index_expr_to_string(expr), data_type_display(type_name))
        }
        Expr::BinaryOp { left, right, op } => {
            format!("{} {} {}", index_expr_to_string(left), op, index_expr_to_string(right))
        }
        Expr::UnaryOp { op, expr } => format!("{op} {}", index_expr_to_string(expr)),
        Expr::Literal(crate::ast::Literal::Integer(n)) => n.to_string(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::plpgsql::{PlBlock, PlDataType, PlDeclaration, PlVarDecl};

    fn make_block(decls: Vec<PlDeclaration>) -> PlBlock {
        PlBlock { label: None, declarations: decls, body: vec![], exception_block: None, end_label: None }
    }

    fn var_decl(name: &str, data_type: PlDataType) -> PlDeclaration {
        PlDeclaration::Variable(PlVarDecl {
            name: name.to_string(),
            data_type,
            default: None,
            constant: false,
            not_null: false,
            collate: None,
        })
    }

    fn schema_with(table: &str, column: &str, sql_type: &str) -> SchemaMap {
        let mut inner = HashMap::new();
        inner.insert(column.to_lowercase(), sql_type.to_string());
        let mut outer = HashMap::new();
        outer.insert(table.to_lowercase(), inner);
        outer
    }

    #[test]
    fn test_resolve_percent_type_found() {
        let schema = schema_with("DB_LOG", "proc_name", "varchar2(200)");
        let block = make_block(vec![var_decl(
            "v_procname",
            PlDataType::PercentType { table: "DB_LOG".to_string(), column: "PROC_NAME".to_string() },
        )]);
        let report = resolve_schema(&block, &schema);
        assert_eq!(report.resolved_types.len(), 1);
        assert_eq!(report.resolved_types[0].table, "DB_LOG");
        assert_eq!(report.resolved_types[0].column, "PROC_NAME");
        assert_eq!(report.resolved_types[0].resolved_type, "varchar2(200)");
        assert!(report.unresolved.is_empty());
    }

    #[test]
    fn test_resolve_percent_type_not_found() {
        let schema = schema_with("DB_LOG", "proc_name", "varchar2(200)");
        let block = make_block(vec![var_decl(
            "v_unknown",
            PlDataType::PercentType { table: "MISSING_TABLE".to_string(), column: "COL".to_string() },
        )]);
        let report = resolve_schema(&block, &schema);
        assert!(report.resolved_types.is_empty());
        assert_eq!(report.unresolved.len(), 1);
        assert_eq!(report.unresolved[0].table, "MISSING_TABLE");
        assert_eq!(report.unresolved[0].column, Some("COL".to_string()));
        assert_eq!(report.unresolved[0].ref_kind, "PercentType");
    }

    #[test]
    fn test_resolve_percent_rowtype_found() {
        let mut inner = HashMap::new();
        inner.insert("id".to_string(), "integer".to_string());
        inner.insert("name".to_string(), "varchar(100)".to_string());
        let mut schema = HashMap::new();
        schema.insert("users".to_string(), inner);

        let block = make_block(vec![var_decl("v_user", PlDataType::PercentRowType("users".to_string()))]);
        let report = resolve_schema(&block, &schema);
        assert_eq!(report.resolved_rowtypes.len(), 1);
        assert_eq!(report.resolved_rowtypes[0].table, "users");
        assert_eq!(report.resolved_rowtypes[0].columns.len(), 2);
        assert!(report.unresolved.is_empty());
    }

    #[test]
    fn test_resolve_mixed() {
        let schema = schema_with("DB_LOG", "proc_name", "varchar2(200)");
        let block = make_block(vec![
            var_decl(
                "v_procname",
                PlDataType::PercentType { table: "DB_LOG".to_string(), column: "PROC_NAME".to_string() },
            ),
            var_decl(
                "v_unknown",
                PlDataType::PercentType { table: "MISSING_TABLE".to_string(), column: "COL".to_string() },
            ),
        ]);
        let report = resolve_schema(&block, &schema);
        assert_eq!(report.resolved_types.len(), 1);
        assert_eq!(report.unresolved.len(), 1);
    }

    #[test]
    fn test_load_schema_file_not_found() {
        let result = load_schema("/nonexistent/path/schema.json");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Failed to read schema file"));
    }

    #[test]
    fn test_load_schema_invalid_json() {
        use std::io::Write;
        let dir = std::env::temp_dir();
        let path = dir.join("ogsql_test_invalid_schema.json");
        {
            let mut file = std::fs::File::create(&path).unwrap();
            write!(file, "not json at all").unwrap();
        }
        let result = load_schema(path.to_str().unwrap());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Failed to parse schema JSON"));
        let _ = std::fs::remove_file(&path);
    }

    // ── Index Metadata Tests ──

    #[test]
    fn test_load_full_schema_new_format() {
        use std::io::Write;
        let dir = std::env::temp_dir();
        let path = dir.join("ogsql_test_full_schema_new.json");
        {
            let mut file = std::fs::File::create(&path).unwrap();
            write!(
                file,
                r#"{{
                "columns": {{ "users": {{ "id": "integer", "name": "varchar(100)" }} }},
                "indexes": {{ "users": {{ "pk_users": ["id"], "idx_name": ["name"], "idx_func": ["lower(email)"] }} }}
            }}"#
            )
            .unwrap();
        }
        let result = load_full_schema(path.to_str().unwrap());
        assert!(result.is_ok(), "Failed to load FullSchema: {:?}", result.err());
        let fs = result.unwrap();
        assert_eq!(fs.columns.len(), 1);
        assert!(fs.columns.contains_key("users"));
        assert_eq!(fs.columns["users"].get("id").unwrap(), "integer");
        assert_eq!(fs.indexes.len(), 1);
        assert!(fs.indexes.contains_key("users"));
        let user_indexes = &fs.indexes["users"];
        assert!(user_indexes.contains_key("pk_users"));
        assert_eq!(user_indexes["pk_users"], vec!["id"]);
        assert!(user_indexes.contains_key("idx_name"));
        assert_eq!(user_indexes["idx_name"], vec!["name"]);
        assert!(user_indexes.contains_key("idx_func"));
        assert_eq!(user_indexes["idx_func"], vec!["lower(email)"]);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_load_full_schema_backward_compat() {
        use std::io::Write;
        let dir = std::env::temp_dir();
        let path = dir.join("ogsql_test_full_schema_old.json");
        {
            let mut file = std::fs::File::create(&path).unwrap();
            write!(file, r#"{{ "users": {{ "id": "integer", "name": "varchar(100)" }} }}"#).unwrap();
        }
        let result = load_full_schema(path.to_str().unwrap());
        assert!(result.is_ok(), "Failed to load old format: {:?}", result.err());
        let fs = result.unwrap();
        assert_eq!(fs.columns.len(), 1);
        assert!(fs.columns.contains_key("users"));
        assert_eq!(fs.columns["users"].get("id").unwrap(), "integer");
        // Indexes should be empty (backward compat)
        assert!(fs.indexes.is_empty(), "Old format should produce empty indexes");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_load_full_schema_file_not_found() {
        let result = load_full_schema("/nonexistent/path/full_schema.json");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Failed to read schema file"));
    }

    #[test]
    fn test_load_full_schema_invalid_json() {
        use std::io::Write;
        let dir = std::env::temp_dir();
        let path = dir.join("ogsql_test_full_schema_invalid.json");
        {
            let mut file = std::fs::File::create(&path).unwrap();
            write!(file, "not json at all").unwrap();
        }
        let result = load_full_schema(path.to_str().unwrap());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Failed to parse schema JSON"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_build_column_index_set_basic() {
        let mut indexes: IndexMapV2 = HashMap::new();
        let mut user_indexes = HashMap::new();
        user_indexes.insert("pk_users".to_string(), vec!["id".to_string()]);
        user_indexes.insert("idx_name".to_string(), vec!["name".to_string()]);
        indexes.insert("users".to_string(), user_indexes);

        let result = build_column_index_set(&indexes);
        assert_eq!(result.len(), 1);
        assert!(result.contains_key("users"));
        let user_cols = &result["users"];
        assert_eq!(user_cols.len(), 2);
        assert!(user_cols.contains("id"));
        assert!(user_cols.contains("name"));
    }

    #[test]
    fn test_build_column_index_set_skips_function_indexes() {
        let mut indexes: IndexMapV2 = HashMap::new();
        let mut user_indexes = HashMap::new();
        user_indexes.insert("idx_email".to_string(), vec!["lower(email)".to_string()]);
        user_indexes.insert("idx_name".to_string(), vec!["name".to_string()]);
        indexes.insert("users".to_string(), user_indexes);

        let result = build_column_index_set(&indexes);
        assert_eq!(result.len(), 1);
        let user_cols = &result["users"];
        // "lower(email)" should be skipped, only "name" should remain
        assert_eq!(user_cols.len(), 1);
        assert!(user_cols.contains("name"));
        assert!(!user_cols.contains("email"));
    }

    #[test]
    fn test_build_column_index_set_multiple_tables() {
        let mut indexes: IndexMapV2 = HashMap::new();
        let mut user_indexes = HashMap::new();
        user_indexes.insert("pk_users".to_string(), vec!["id".to_string()]);
        indexes.insert("users".to_string(), user_indexes);
        let mut log_indexes = HashMap::new();
        log_indexes.insert("idx_proc".to_string(), vec!["proc_name".to_string()]);
        indexes.insert("db_log".to_string(), log_indexes);

        let result = build_column_index_set(&indexes);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("users"));
        assert!(result.contains_key("db_log"));
        assert!(result["users"].contains("id"));
        assert!(result["db_log"].contains("proc_name"));
    }

    #[test]
    fn test_build_column_index_set_empty() {
        let indexes: IndexMapV2 = HashMap::new();
        let result = build_column_index_set(&indexes);
        assert!(result.is_empty());
    }

    #[test]
    fn test_build_column_index_set_case_insensitive_keys() {
        let mut indexes: IndexMapV2 = HashMap::new();
        let mut user_indexes = HashMap::new();
        user_indexes.insert("idx_id".to_string(), vec!["ID".to_string()]);
        indexes.insert("USERS".to_string(), user_indexes);

        let result = build_column_index_set(&indexes);
        assert!(result.contains_key("users"));
        assert!(result["users"].contains("id"));
    }

    #[test]
    fn test_matches_function_index_found() {
        let mut indexes: IndexMapV2 = HashMap::new();
        let mut user_indexes = HashMap::new();
        user_indexes.insert("idx_func".to_string(), vec!["lower(email)".to_string()]);
        indexes.insert("users".to_string(), user_indexes);

        assert!(matches_function_index(&indexes, "users", "lower", "email"));
    }

    #[test]
    fn test_matches_function_index_not_found() {
        let mut indexes: IndexMapV2 = HashMap::new();
        let mut user_indexes = HashMap::new();
        user_indexes.insert("idx_func".to_string(), vec!["lower(email)".to_string()]);
        indexes.insert("users".to_string(), user_indexes);

        assert!(!matches_function_index(&indexes, "users", "upper", "email"));
        assert!(!matches_function_index(&indexes, "users", "lower", "name"));
    }

    #[test]
    fn test_matches_function_index_case_insensitive() {
        let mut indexes: IndexMapV2 = HashMap::new();
        let mut user_indexes = HashMap::new();
        user_indexes.insert("idx_func".to_string(), vec!["lower(email)".to_string()]);
        indexes.insert("USERS".to_string(), user_indexes);

        assert!(matches_function_index(&indexes, "USERS", "LOWER", "EMAIL"));
        assert!(matches_function_index(&indexes, "users", "LOWER", "email"));
        assert!(matches_function_index(&indexes, "USERS", "lower", "EMAIL"));
    }

    #[test]
    fn test_matches_function_index_table_not_found() {
        let indexes: IndexMapV2 = HashMap::new();
        assert!(!matches_function_index(&indexes, "nonexistent", "lower", "email"));
    }

    #[test]
    fn test_matches_function_index_no_indexes_for_table() {
        let mut indexes: IndexMapV2 = HashMap::new();
        let mut other_indexes = HashMap::new();
        other_indexes.insert("idx_func".to_string(), vec!["lower(email)".to_string()]);
        indexes.insert("other".to_string(), other_indexes);
        assert!(!matches_function_index(&indexes, "users", "lower", "email"));
    }

    #[test]
    fn test_full_schema_serde_roundtrip() {
        let fs = FullSchema {
            columns: {
                let mut cols = HashMap::new();
                let mut inner = HashMap::new();
                inner.insert("id".to_string(), "integer".to_string());
                cols.insert("t".to_string(), inner);
                cols
            },
            indexes: {
                let mut idxs = HashMap::new();
                let mut inner = HashMap::new();
                inner.insert("pk_t".to_string(), vec!["id".to_string()]);
                idxs.insert("t".to_string(), inner);
                idxs
            },
        };
        let json = serde_json::to_string(&fs).unwrap();
        let deserialized: FullSchema = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.columns.len(), 1);
        assert_eq!(deserialized.indexes.len(), 1);
        assert_eq!(deserialized.indexes["t"]["pk_t"], vec!["id"]);
    }

    #[test]
    fn test_full_schema_serde_default_fallback() {
        // Old format JSON (without "columns"/"indexes" wrapper) should be
        // rejected by FullSchema's deny_unknown_fields, but load_full_schema
        // handles it via the SchemaMap fallback path.
        let json = r#"{"t": {"id": "integer"}}"#;
        let result: Result<FullSchema, _> = serde_json::from_str(json);
        assert!(result.is_err(), "Old format should be rejected by deny_unknown_fields");
    }
}
