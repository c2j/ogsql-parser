use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::ast::plpgsql::{PlBlock, PlDataType, PlStatement};

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

pub fn load_schema(path: &str) -> Result<SchemaMap, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read schema file '{}': {}", path, e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse schema JSON: {}", e))
}

pub fn resolve_schema(block: &PlBlock, schema: &SchemaMap) -> SchemaResolutionReport {
    let mut report = SchemaResolutionReport {
        resolved_types: Vec::new(),
        resolved_rowtypes: Vec::new(),
        unresolved: Vec::new(),
    };
    resolve_schema_recursive(block, schema, &mut report);
    report
}

fn resolve_schema_recursive(block: &PlBlock, schema: &SchemaMap, report: &mut SchemaResolutionReport) {
    for decl in &block.declarations {
        let data_type = match decl {
            crate::ast::plpgsql::PlDeclaration::Variable(v) => &v.data_type,
            crate::ast::plpgsql::PlDeclaration::Cursor(c) => {
                match c.return_type {
                    Some(ref dt) => dt,
                    None => continue,
                }
            }
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
                    let cols: Vec<ColumnDef> = columns.iter()
                        .map(|(name, dt)| ColumnDef { name: name.clone(), data_type: dt.clone() })
                        .collect();
                    report.resolved_rowtypes.push(ResolvedRowType {
                        table: table.clone(),
                        columns: cols,
                    });
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::plpgsql::{PlBlock, PlDeclaration, PlDataType, PlVarDecl};

    fn make_block(decls: Vec<PlDeclaration>) -> PlBlock {
        PlBlock {
            label: None,
            declarations: decls,
            body: vec![],
            exception_block: None,
            end_label: None,
        }
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
        let block = make_block(vec![
            var_decl("v_procname", PlDataType::PercentType {
                table: "DB_LOG".to_string(),
                column: "PROC_NAME".to_string(),
            }),
        ]);
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
        let block = make_block(vec![
            var_decl("v_unknown", PlDataType::PercentType {
                table: "MISSING_TABLE".to_string(),
                column: "COL".to_string(),
            }),
        ]);
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

        let block = make_block(vec![
            var_decl("v_user", PlDataType::PercentRowType("users".to_string())),
        ]);
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
            var_decl("v_procname", PlDataType::PercentType {
                table: "DB_LOG".to_string(),
                column: "PROC_NAME".to_string(),
            }),
            var_decl("v_unknown", PlDataType::PercentType {
                table: "MISSING_TABLE".to_string(),
                column: "COL".to_string(),
            }),
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
}
