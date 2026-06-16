use ogsql_parser::{Parser, SqlFormatter, Tokenizer};

fn format_sql(sql: &str) -> String {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    let formatter = SqlFormatter::new();
    stmts
        .iter()
        .map(|s| formatter.format_statement(s))
        .collect::<Vec<_>>()
        .join(";\n")
}

#[test]
fn unquoted_uppercase_not_quoted() {
    let out = format_sql("SELECT * FROM MyTable");
    assert!(
        !out.contains("\"MyTable\""),
        "Unquoted identifier should not gain quotes. Got: {out}"
    );
}

#[test]
fn quoted_identifier_preserved() {
    let out = format_sql("SELECT * FROM \"MyTable\"");
    assert!(
        out.contains("\"MyTable\""),
        "Quoted identifier should keep quotes. Got: {out}"
    );
}

#[test]
fn quoted_lowercase_preserved() {
    let out = format_sql("SELECT * FROM \"mytable\"");
    assert!(
        out.contains("\"mytable\""),
        "Quoted lowercase should keep quotes. Got: {out}"
    );
}

#[test]
fn mixed_quoted_unqualified() {
    let out = format_sql("SELECT id FROM userDetails WHERE id = 1");
    assert!(!out.contains("\""), "No quotes expected. Got: {out}");
}

#[test]
fn qualified_name_with_quotes() {
    let out = format_sql("SELECT * FROM public.\"MyTable\"");
    assert!(out.contains("\"MyTable\""), "Got: {out}");
}

#[test]
fn create_table_quoted_name() {
    let out = format_sql("CREATE TABLE \"MyTable\" (id INT)");
    assert!(out.contains("\"MyTable\""), "Got: {out}");
}
