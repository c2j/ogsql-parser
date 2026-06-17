use ogsql_parser::{Parser, SqlFormatter, Tokenizer};

fn format_sql(sql: &str) -> String {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    let formatter = SqlFormatter::new();
    stmts.iter().map(|s| formatter.format_statement(s)).collect::<Vec<_>>().join(";\n")
}

#[test]
fn unquoted_uppercase_not_quoted() {
    let out = format_sql("SELECT * FROM MyTable");
    assert!(!out.contains("\"MyTable\""), "Unquoted identifier should not gain quotes. Got: {out}");
}

#[test]
fn quoted_identifier_preserved() {
    let out = format_sql("SELECT * FROM \"MyTable\"");
    assert!(out.contains("\"MyTable\""), "Quoted identifier should keep quotes. Got: {out}");
}

#[test]
fn quoted_lowercase_preserved() {
    let out = format_sql("SELECT * FROM \"mytable\"");
    assert!(out.contains("\"mytable\""), "Quoted lowercase should keep quotes. Got: {out}");
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

// --- Issue #228: alias quote_style preservation ---

#[test]
fn unquoted_uppercase_table_alias_not_quoted() {
    let out = format_sql("SELECT * FROM PAR A, VAB v WHERE a.col = v.col");
    assert!(!out.contains("\"A\""), "Unquoted alias should not gain quotes. Got: {out}");
    assert!(!out.contains("\"v\""), "Unquoted lowercase alias should not gain quotes. Got: {out}");
}

#[test]
fn quoted_table_alias_preserved() {
    let out = format_sql("SELECT * FROM \"PAR\" \"A\"");
    assert!(out.contains("\"PAR\""), "Quoted table name should keep quotes. Got: {out}");
    assert!(out.contains("\"A\""), "Quoted alias should keep quotes. Got: {out}");
}

#[test]
fn subquery_alias_quote_style_preserved() {
    let out = format_sql("SELECT * FROM (SELECT 1) T");
    assert!(!out.contains("\"T\""), "Unquoted subquery alias should not gain quotes. Got: {out}");

    let out = format_sql("SELECT * FROM (SELECT 1) \"T\"");
    assert!(out.contains("\"T\""), "Quoted subquery alias should keep quotes. Got: {out}");
}

#[test]
fn column_alias_quote_style_preserved() {
    let out = format_sql("SELECT col AS MyCol FROM t");
    assert!(!out.contains("\"MyCol\""), "Unquoted column alias should not gain quotes. Got: {out}");

    let out = format_sql("SELECT col AS \"MyCol\" FROM t");
    assert!(out.contains("\"MyCol\""), "Quoted column alias should keep quotes. Got: {out}");
}

#[test]
fn insert_alias_quote_style_preserved() {
    let out = format_sql("INSERT INTO t T VALUES (1)");
    assert!(!out.contains("\"T\""), "Unquoted INSERT alias should not gain quotes. Got: {out}");
}

// --- Issue #229: Oracle (+) outer-join marker round-trip preservation ---

#[test]
fn oracle_plus_single_column_preserved() {
    let out = format_sql("SELECT * FROM t1, t2 WHERE t1.id = t2.id(+)");
    assert!(out.contains("id(+)"), "(+) must be preserved on round-trip. Got: {out}");
}

#[test]
fn oracle_plus_qualified_column_preserved() {
    let out = format_sql("SELECT * FROM t1, t2 WHERE t.code = exchange.coin_code(+)");
    assert!(out.contains("coin_code(+)"), "Qualified column (+) must be preserved. Got: {out}");
}

#[test]
fn oracle_plus_keyword_column_preserved() {
    let out = format_sql("SELECT * FROM t1, t2 WHERE LANGUAGE(+) = '02'");
    assert!(out.to_lowercase().contains("language(+)"), "Keyword column (+) must be preserved. Got: {out}");
}

#[test]
fn oracle_plus_not_doubled_or_misplaced() {
    let out = format_sql("SELECT * FROM t1, t2 WHERE t1.id = t2.id(+)");
    assert_eq!(out.matches("(+)").count(), 1, "Exactly one (+) expected. Got: {out}");
    assert!(!out.contains("t1.id(+)"), "(+) must stay on t2 side. Got: {out}");
}
