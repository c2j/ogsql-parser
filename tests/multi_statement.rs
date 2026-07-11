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
fn anonymous_block_then_select() {
    // Regression test: anonymous block `BEGIN ... END;` followed by another statement.
    // The statement splitter doesn't track PL block nesting, so `find_statement_end_pos`
    // returns the first inner semicolon as the boundary. The parser must not reset
    // `self.pos` back to that boundary; it should keep the actual end position.
    let sql = "BEGIN SELECT 1; END;\nSELECT 2;";
    let (infos, errors) = Parser::parse_sql(sql);
    assert!(errors.is_empty(), "expected no errors, got: {errors:?}");
    assert_eq!(infos.len(), 2, "expected 2 statements (AnonyBlock + Select)");

    assert!(matches!(infos[0].statement, ogsql_parser::Statement::AnonyBlock(_)));
    assert!(matches!(infos[1].statement, ogsql_parser::Statement::Select(_)));

    // Verify end span points to END; not the first inner semicolon.
    // The AnonyBlock should end on line 1 where END; is (not at the first `;` inside).
    assert_eq!(infos[0].end_line, 1, "AnonyBlock end_line should be line 1 where END; is");
    // The Select on line 2 should have its own correct span.
    assert_eq!(infos[1].end_line, 2, "Select end_line should be line 2");
}

#[test]
fn statement_with_slash_terminator() {
    let sql = "CREATE OR REPLACE FUNCTION foo() RETURNS INTEGER AS $$ BEGIN RETURN 1; END $$ LANGUAGE plpgsql /\n";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 1);
}
