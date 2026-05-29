use ogsql_parser::{Parser, SqlFormatter, Tokenizer};

#[test]
fn do_block_basic() {
    let sql = "DO $$ BEGIN RAISE NOTICE 'hello'; END $$";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn anonymous_block_with_declare() {
    let sql = "DECLARE x INTEGER := 1; BEGIN x := x + 1; END";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert!(!stmts.is_empty());
}

#[test]
fn if_else_block() {
    let sql = "DO $$ DECLARE x INTEGER := 10; BEGIN IF x > 5 THEN RAISE NOTICE 'big'; ELSE RAISE NOTICE 'small'; END IF; END $$";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn while_loop() {
    let sql = "DO $$ DECLARE i INTEGER := 0; BEGIN WHILE i < 10 LOOP i := i + 1; END LOOP; END $$";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn for_loop() {
    let sql =
        "DO $$ DECLARE r RECORD; BEGIN FOR r IN SELECT id FROM users LOOP RAISE NOTICE '%', r.id; END LOOP; END $$";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn cursor_operations() {
    let sql = "DO $$ DECLARE cur CURSOR FOR SELECT id FROM users; r RECORD; BEGIN OPEN cur; FETCH cur INTO r; CLOSE cur; END $$";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn exception_handler() {
    let sql = "DO $$ BEGIN RAISE NOTICE 'try'; EXCEPTION WHEN OTHERS THEN RAISE NOTICE 'catch'; END $$";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn execute_immediate() {
    let sql = "DO $$ BEGIN EXECUTE IMMEDIATE 'SELECT 1'; END $$";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn create_function_with_plpgsql_body() {
    let sql = "CREATE FUNCTION add_one(x INTEGER) RETURNS INTEGER AS $$ BEGIN RETURN x + 1; END $$ LANGUAGE plpgsql";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn create_procedure() {
    let sql = "CREATE PROCEDURE log_message(msg VARCHAR) AS $$ BEGIN RAISE NOTICE '%', msg; END $$ LANGUAGE plpgsql";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 1);
}

#[test]
fn plpgsql_roundtrip() {
    let sql = "DO $$ DECLARE x INTEGER := 42; BEGIN IF x > 0 THEN RAISE NOTICE 'positive'; END IF; END $$";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 1);

    let json = serde_json::to_string(&stmts).unwrap();
    let restored: Vec<ogsql_parser::Statement> = serde_json::from_str(&json).unwrap();
    let formatter = SqlFormatter::new();
    let formatted: Vec<String> = restored.iter().map(|s| formatter.format_statement(s)).collect();
    assert_eq!(formatted.len(), 1);
}
