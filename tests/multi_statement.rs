use ogsql_parser::{Parser, Tokenizer};

#[test]
fn two_selects_semicolon() {
    let sql = "SELECT 1; SELECT 2";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 2);
}

#[test]
fn three_statements_mixed_types() {
    let sql = "CREATE TABLE t (id INTEGER); INSERT INTO t VALUES (1); SELECT * FROM t";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 3);
}

#[test]
fn statements_with_trailing_semicolons() {
    let sql = "SELECT 1;;; SELECT 2;;";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 2);
}

#[test]
fn ddl_and_dml_sequence() {
    let sql = "DROP TABLE IF EXISTS t; CREATE TABLE t (id INTEGER, name VARCHAR(100)); INSERT INTO t VALUES (1, 'test'); UPDATE t SET name = 'updated' WHERE id = 1; DELETE FROM t WHERE id = 1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 5);
}

#[test]
fn parse_sql_helper() {
    let (infos, errors) = Parser::parse_sql("SELECT 1; SELECT 2; SELECT 3");
    assert_eq!(infos.len(), 3);
    assert!(errors.is_empty());
}

#[test]
fn statement_with_slash_terminator() {
    let sql = "CREATE OR REPLACE FUNCTION foo() RETURNS INTEGER AS $$ BEGIN RETURN 1; END $$ LANGUAGE plpgsql /\n";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 1);
}
