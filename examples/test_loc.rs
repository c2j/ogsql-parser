fn main() {
    // Test a simpler file with known-good statements
    let sqls = vec![
        ("SELECT 1;", "SELECT 1;"),
        ("SELECT * FROM get_tab_ptf(2);", "SELECT * FROM get_tab_ptf(2);"),
        ("  INSERT INTO t VALUES (1, 2);  ", "INSERT INTO t VALUES (1, 2);"),
        ("-- comment\nSELECT 1;\nSELECT 2;", "SELECT 1;"),
        ("CREATE TABLE t (id int);\nDROP TABLE t;", "CREATE TABLE t (id int);"),
    ];
    for (sql, expected_first) in &sqls {
        let (infos, errors) = ogsql_parser::parser::Parser::parse_sql(sql);
        let first_text = infos.first().map(|i| i.sql_text.clone()).unwrap_or_default();
        let ok = first_text == *expected_first;
        println!("{} expected={:?} got={:?}", if ok { "✓" } else { "✗" }, expected_first, first_text);
        if !ok {
            println!("  SQL: {:?}", sql);
        }
        if !errors.is_empty() {
            println!("  Errors: {:?}", errors);
        }
    }
    
    // Test multiline
    let sql = "CREATE TABLE stocktable\n(\n    ticker VARCHAR2(20)\n);";
    let (infos, _) = ogsql_parser::parser::Parser::parse_sql(sql);
    for info in &infos {
        println!("multiline: {:?}", info.sql_text);
        println!("  start={}:{} end={}:{}", info.start_line, info.start_col, info.end_line, info.end_col);
    }
}
