use ogsql_parser::{Parser, Tokenizer};

#[test]
fn incomplete_select_no_panic() {
    let result = Tokenizer::new("SELECT").tokenize();
    assert!(result.is_ok());
    // Parsing incomplete SELECT should not panic
    let tokens = result.unwrap();
    let _stmts = Parser::new(tokens).parse();
    // May contain Empty statements but should not panic
}

#[test]
fn mismatched_parens_no_panic() {
    let sql = "SELECT * FROM (SELECT id FROM users";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let _stmts = Parser::new(tokens).parse();
}

#[test]
fn garbage_input_returns_errors() {
    let sql = "@@@ !!! ### garbage ^^^";
    let output = Parser::parse_sql(sql);
    // Should produce errors but not panic
    assert!(!output.0.is_empty() || !output.1.is_empty());
}

#[test]
fn empty_input() {
    let tokens = Tokenizer::new("").tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert!(stmts.is_empty());
}

#[test]
fn only_semicolons() {
    let sql = ";;;";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert!(stmts.is_empty());
}

#[test]
fn only_whitespace() {
    let sql = "   \n  \t  \n  ";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert!(stmts.is_empty());
}

#[test]
fn unterminated_string_returns_error() {
    let result = Tokenizer::new("SELECT 'unterminated").tokenize();
    assert!(result.is_err());
}

#[test]
fn unterminated_block_comment_returns_error() {
    let result = Tokenizer::new("SELECT /* unclosed comment").tokenize();
    assert!(result.is_err());
}

#[test]
fn reserved_keyword_as_identifier() {
    let sql = "CREATE TABLE select (id INTEGER)";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    // May produce errors or warnings but should not panic
    assert!(!stmts.is_empty());
}

#[test]
fn trailing_junk_after_statement() {
    let sql = "SELECT 1 garbage";
    let output = Parser::parse_sql(sql);
    // Should parse the SELECT but report errors for garbage
    assert!(!output.0.is_empty());
}

#[test]
fn parse_one_returns_error_for_multiple_statements() {
    let result = Parser::parse_one("SELECT 1; SELECT 2");
    assert!(result.is_err());
}
