use ogsql_parser::{Parser, SqlFormatter, Tokenizer};

fn roundtrip(sql: &str) -> (String, Vec<String>) {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();

    let json = serde_json::to_string(&stmts).unwrap();
    let restored: Vec<ogsql_parser::Statement> = serde_json::from_str(&json).unwrap();

    let formatter = SqlFormatter::new();
    let formatted: Vec<String> = restored.iter().map(|s| formatter.format_statement(s)).collect();

    (json, formatted)
}

#[test]
fn roundtrip_select_simple() {
    let sql = "SELECT id, name FROM users WHERE status = 'active'";
    let (json, formatted) = roundtrip(sql);
    assert!(json.contains("ColumnRef"));
    assert_eq!(formatted.len(), 1);
    assert!(formatted[0].to_uppercase().contains("SELECT"));
}

#[test]
fn roundtrip_select_with_joins() {
    let sql = "SELECT u.id, o.total FROM users u JOIN orders o ON u.id = o.user_id WHERE u.status = 'active'";
    let (_json, formatted) = roundtrip(sql);
    assert_eq!(formatted.len(), 1);
    assert!(formatted[0].to_uppercase().contains("JOIN"));
    assert!(formatted[0].to_uppercase().contains("ON"));
}

#[test]
fn roundtrip_select_with_cte() {
    let sql = "WITH active AS (SELECT id FROM users WHERE status = 'active') SELECT * FROM active";
    let (_json, formatted) = roundtrip(sql);
    assert_eq!(formatted.len(), 1);
    assert!(formatted[0].to_uppercase().contains("WITH"));
    assert!(formatted[0].to_uppercase().contains("ACTIVE"));
}

#[test]
fn roundtrip_select_with_subquery() {
    let sql = "SELECT id FROM users WHERE id IN (SELECT user_id FROM orders WHERE total > 100)";
    let (_json, formatted) = roundtrip(sql);
    assert_eq!(formatted.len(), 1);
    assert!(formatted[0].to_uppercase().contains("IN"));
}

#[test]
fn roundtrip_insert_values() {
    let sql = "INSERT INTO users (id, name, email) VALUES (1, 'Alice', 'alice@test.com')";
    let (_json, formatted) = roundtrip(sql);
    assert_eq!(formatted.len(), 1);
    assert!(formatted[0].to_uppercase().contains("INSERT"));
    assert!(formatted[0].contains("Alice"));
}

#[test]
fn roundtrip_update() {
    let sql = "UPDATE users SET name = 'Bob', status = 'inactive' WHERE id = 1";
    let (_json, formatted) = roundtrip(sql);
    assert_eq!(formatted.len(), 1);
    assert!(formatted[0].to_uppercase().contains("UPDATE"));
    assert!(formatted[0].to_uppercase().contains("SET"));
}

#[test]
fn roundtrip_delete() {
    let sql = "DELETE FROM users WHERE status = 'inactive'";
    let (_json, formatted) = roundtrip(sql);
    assert_eq!(formatted.len(), 1);
    assert!(formatted[0].to_uppercase().contains("DELETE"));
}

#[test]
fn roundtrip_create_table() {
    let sql = "CREATE TABLE products (id INTEGER PRIMARY KEY, name VARCHAR(200) NOT NULL, price NUMERIC(10, 2))";
    let (_json, formatted) = roundtrip(sql);
    assert_eq!(formatted.len(), 1);
    assert!(formatted[0].to_uppercase().contains("CREATE TABLE"));
    assert!(formatted[0].contains("PRIMARY KEY"));
}

#[test]
fn roundtrip_create_index() {
    let sql = "CREATE INDEX idx_users_status ON users (status)";
    let (_json, formatted) = roundtrip(sql);
    assert_eq!(formatted.len(), 1);
    assert!(formatted[0].to_uppercase().contains("CREATE INDEX"));
}

#[test]
fn roundtrip_drop_table() {
    let sql = "DROP TABLE IF EXISTS temp_data";
    let (_json, formatted) = roundtrip(sql);
    assert_eq!(formatted.len(), 1);
    assert!(formatted[0].to_uppercase().contains("DROP TABLE"));
}

#[test]
fn roundtrip_truncate() {
    let sql = "TRUNCATE TABLE logs";
    let (_json, formatted) = roundtrip(sql);
    assert_eq!(formatted.len(), 1);
    assert!(formatted[0].to_uppercase().contains("TRUNCATE"));
}

#[test]
fn roundtrip_window_function() {
    let sql = "SELECT id, ROW_NUMBER() OVER (PARTITION BY dept ORDER BY salary DESC) AS rank FROM employees";
    let (_json, formatted) = roundtrip(sql);
    assert_eq!(formatted.len(), 1);
    assert!(formatted[0].to_uppercase().contains("ROW_NUMBER"));
    assert!(formatted[0].to_uppercase().contains("PARTITION BY"));
}

#[test]
fn roundtrip_special_literals() {
    let sql = "SELECT E'\\ttext', B'1010', X'FF', N'unicode'";
    let (json, _formatted) = roundtrip(sql);
    assert!(json.contains("EscapeString"));
    assert!(json.contains("BitString"));
    assert!(json.contains("HexString"));
    assert!(json.contains("NationalString"));
}

#[test]
fn roundtrip_merge() {
    let sql = "MERGE INTO target t USING source s ON t.id = s.id WHEN MATCHED THEN UPDATE SET t.name = s.name WHEN NOT MATCHED THEN INSERT (id, name) VALUES (s.id, s.name)";
    let (_json, formatted) = roundtrip(sql);
    assert_eq!(formatted.len(), 1);
    assert!(formatted[0].to_uppercase().contains("MERGE"));
    assert!(formatted[0].to_uppercase().contains("WHEN MATCHED"));
}

#[test]
fn roundtrip_order_by_with_nulls() {
    let sql = "SELECT id, name FROM users ORDER BY name ASC NULLS LAST, id DESC NULLS FIRST";
    let (_json, formatted) = roundtrip(sql);
    assert_eq!(formatted.len(), 1);
    assert!(formatted[0].to_uppercase().contains("ORDER BY"));
    assert!(formatted[0].to_uppercase().contains("NULLS"));
}

#[test]
fn roundtrip_serialization_roundtrip() {
    // Full JSON roundtrip: SQL → AST → JSON → AST → verify structural equality
    let sql = "SELECT id, name FROM users WHERE id = 1 AND status = 'active'";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let original = Parser::new(tokens).parse();

    let json = serde_json::to_string(&original).unwrap();
    let restored: Vec<ogsql_parser::Statement> = serde_json::from_str(&json).unwrap();

    assert_eq!(original.len(), restored.len());
    // Re-serialize both and compare
    let json2 = serde_json::to_string(&restored).unwrap();
    assert_eq!(json, json2);
}
