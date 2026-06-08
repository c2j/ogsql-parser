use crate::analyzer::schema::SchemaMap;
use crate::ast::Expr;

/// SQL data type families for comparing literal-column compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeFamily {
    String,
    Integer,
    Numeric,
    DateTime,
    Boolean,
    BitString,
    Unknown,
}

pub fn build_column_type_map(
    schema: &SchemaMap,
    tables: &[crate::ast::TableRef],
) -> std::collections::HashMap<(String, String), String> {
    let mut map = std::collections::HashMap::new();
    for tref in tables {
        collect_table_types(schema, tref, &mut map);
    }
    map
}

fn collect_table_types(
    schema: &SchemaMap,
    tref: &crate::ast::TableRef,
    map: &mut std::collections::HashMap<(String, String), String>,
) {
    use crate::ast::TableRef;
    match tref {
        TableRef::Table { name, alias, .. } => {
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

fn find_schema_table<'a>(schema: &'a SchemaMap, name: &[String]) -> Option<&'a String> {
    if name.is_empty() {
        return None;
    }
    if name.len() >= 2 {
        let full = format!("{}.{}", name[0].to_lowercase(), name[1].to_lowercase());
        if let Some(key) = schema.keys().find(|k| *k == &full) {
            return Some(key);
        }
    }
    let single = name.last().unwrap().to_lowercase();
    schema.keys().find(|k| *k == &single)
}

pub fn resolve_column_type(
    expr: &Expr,
    col_types: &std::collections::HashMap<(String, String), String>,
) -> Option<String> {
    match expr {
        Expr::ColumnRef(name) => {
            if name.len() == 1 {
                for ((_, col), dtype) in col_types {
                    if col == &name[0].to_lowercase() {
                        return Some(dtype.clone());
                    }
                }
                None
            } else if name.len() >= 2 {
                let table = name[name.len() - 2].to_lowercase();
                let col = name[name.len() - 1].to_lowercase();
                col_types.get(&(table, col)).cloned()
            } else {
                None
            }
        }
        Expr::FieldAccess { object, field } => {
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

/// Classify a SQL data type into a type family.
pub fn classify_type_family(sql_type: &str) -> TypeFamily {
    let lower = sql_type.to_lowercase();
    let base = lower.split('(').next().unwrap_or(&lower).trim();
    // Strip array suffix: "integer[]" → "integer".
    let base = base.strip_suffix("[]").unwrap_or(base).trim();

    if is_string_type_base(base) {
        return TypeFamily::String;
    }
    if matches!(
        base,
        "integer" | "int" | "bigint" | "smallint" | "int2" | "int4" | "int8" | "serial" | "bigserial" | "smallserial"
    ) {
        return TypeFamily::Integer;
    }
    if matches!(base, "numeric" | "decimal" | "real" | "float" | "float4" | "float8" | "double precision" | "number") {
        return TypeFamily::Numeric;
    }
    if matches!(
        base,
        "date"
            | "timestamp"
            | "timestamptz"
            | "timestamp with time zone"
            | "timestamp without time zone"
            | "time"
            | "timetz"
            | "time with time zone"
            | "time without time zone"
            | "interval"
    ) {
        return TypeFamily::DateTime;
    }
    if matches!(base, "boolean" | "bool") {
        return TypeFamily::Boolean;
    }
    if matches!(base, "bit" | "bit varying" | "varbit") {
        return TypeFamily::BitString;
    }
    TypeFamily::Unknown
}

/// Infer the type family of a literal value.
pub fn literal_type_family(lit: &crate::ast::Literal) -> TypeFamily {
    match lit {
        crate::ast::Literal::String(_) | crate::ast::Literal::EscapeString(_) => TypeFamily::String,
        crate::ast::Literal::NationalString(_) => TypeFamily::String,
        crate::ast::Literal::DollarString { .. } => TypeFamily::String,
        crate::ast::Literal::Integer(_) => TypeFamily::Integer,
        crate::ast::Literal::Float(_) => TypeFamily::Numeric,
        crate::ast::Literal::BitString(_) => TypeFamily::BitString,
        crate::ast::Literal::HexString(_) => TypeFamily::BitString,
        crate::ast::Literal::Boolean(_) => TypeFamily::Boolean,
        crate::ast::Literal::Null => TypeFamily::Unknown,
    }
}

/// Check whether a resolved SQL data type belongs to the string family.
/// Comparison with a string literal against these types is safe — no
/// implicit cross-family conversion occurs.
fn is_string_type_base(base: &str) -> bool {
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

/// Convenience: check if a full SQL type string belongs to the string family.
pub fn is_string_type(data_type: &str) -> bool {
    matches!(classify_type_family(data_type), TypeFamily::String)
}
