use crate::ast::*;
use crate::parser::Parser;
use crate::token::tokenizer::Tokenizer;

fn parse(sql: &str) -> Vec<Statement> {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    Parser::new(tokens).parse().unwrap()
}

fn parse_one(sql: &str) -> Statement {
    let stmts = parse(sql);
    stmts
        .into_iter()
        .next()
        .expect("expected at least one statement")
}

fn parse_err(sql: &str) -> Statement {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse().unwrap();
    stmts.into_iter().next().unwrap()
}

// ========== TRUNCATE ==========

#[test]
fn test_truncate_single_table() {
    let stmt = parse_one("TRUNCATE TABLE users");
    match stmt {
        Statement::Truncate(t) => {
            assert_eq!(t.tables.len(), 1);
            assert_eq!(t.tables[0], vec!["users"]);
            assert!(!t.cascade);
            assert!(!t.restart_identity);
        }
        _ => panic!("expected TRUNCATE, got {:?}", stmt),
    }
}

#[test]
fn test_truncate_without_table_keyword() {
    let stmt = parse_one("TRUNCATE users");
    match stmt {
        Statement::Truncate(t) => {
            assert_eq!(t.tables[0], vec!["users"]);
        }
        _ => panic!("expected TRUNCATE"),
    }
}

#[test]
fn test_truncate_multiple_tables() {
    let stmt = parse_one("TRUNCATE TABLE users, orders, items");
    match stmt {
        Statement::Truncate(t) => {
            assert_eq!(t.tables.len(), 3);
        }
        _ => panic!("expected TRUNCATE"),
    }
}

#[test]
fn test_truncate_cascade() {
    let stmt = parse_one("TRUNCATE TABLE users CASCADE");
    match stmt {
        Statement::Truncate(t) => {
            assert!(t.cascade);
        }
        _ => panic!("expected TRUNCATE"),
    }
}

#[test]
fn test_truncate_qualified_name() {
    let stmt = parse_one("TRUNCATE TABLE public.users");
    match stmt {
        Statement::Truncate(t) => {
            assert_eq!(t.tables[0], vec!["public", "users"]);
        }
        _ => panic!("expected TRUNCATE"),
    }
}

// ========== CREATE VIEW ==========

#[test]
fn test_create_simple_view() {
    let stmt = parse_one("CREATE VIEW active_users AS SELECT * FROM users WHERE status = 'active'");
    match stmt {
        Statement::CreateView(v) => {
            assert!(!v.replace);
            assert!(!v.temporary);
            assert!(!v.recursive);
            assert_eq!(v.name, vec!["active_users"]);
            assert!(v.columns.is_empty());
            assert!(v.check_option.is_none());
        }
        _ => panic!("expected CREATE VIEW"),
    }
}

#[test]
fn test_create_or_replace_view() {
    let stmt = parse_one("CREATE OR REPLACE VIEW v AS SELECT 1");
    match stmt {
        Statement::CreateView(v) => {
            assert!(v.replace);
        }
        _ => panic!("expected CREATE VIEW"),
    }
}

#[test]
fn test_create_temporary_view() {
    let stmt = parse_one("CREATE TEMPORARY VIEW tv AS SELECT * FROM t");
    match stmt {
        Statement::CreateView(v) => {
            assert!(v.temporary);
        }
        _ => panic!("expected CREATE VIEW"),
    }
}

#[test]
fn test_create_temp_view() {
    let stmt = parse_one("CREATE TEMP VIEW tv AS SELECT 1");
    match stmt {
        Statement::CreateView(v) => {
            assert!(v.temporary);
        }
        _ => panic!("expected CREATE VIEW"),
    }
}

#[test]
fn test_create_recursive_view() {
    let stmt = parse_one("CREATE RECURSIVE VIEW rv AS SELECT 1");
    match stmt {
        Statement::CreateView(v) => {
            assert!(v.recursive);
        }
        _ => panic!("expected CREATE VIEW"),
    }
}

#[test]
fn test_create_view_with_columns() {
    let stmt = parse_one("CREATE VIEW v(a, b, c) AS SELECT x, y, z FROM t");
    match stmt {
        Statement::CreateView(v) => {
            assert_eq!(v.columns, vec!["a", "b", "c"]);
        }
        _ => panic!("expected CREATE VIEW"),
    }
}

#[test]
fn test_create_view_with_check_option() {
    let stmt = parse_one("CREATE VIEW v AS SELECT * FROM t WITH CHECK OPTION");
    match stmt {
        Statement::CreateView(v) => {
            assert_eq!(v.check_option, Some(CheckOption::Cascaded));
        }
        _ => panic!("expected CREATE VIEW"),
    }
}

#[test]
fn test_create_view_with_local_check_option() {
    let stmt = parse_one("CREATE VIEW v AS SELECT * FROM t WITH LOCAL CHECK OPTION");
    match stmt {
        Statement::CreateView(v) => {
            assert_eq!(v.check_option, Some(CheckOption::Local));
        }
        _ => panic!("expected CREATE VIEW"),
    }
}

#[test]
fn test_create_view_with_cascaded_check_option() {
    let stmt = parse_one("CREATE VIEW v AS SELECT * FROM t WITH CASCADED CHECK OPTION");
    match stmt {
        Statement::CreateView(v) => {
            assert_eq!(v.check_option, Some(CheckOption::Cascaded));
        }
        _ => panic!("expected CREATE VIEW"),
    }
}

#[test]
fn test_create_qualified_view() {
    let stmt = parse_one("CREATE VIEW schema.my_view AS SELECT 1");
    match stmt {
        Statement::CreateView(v) => {
            assert_eq!(v.name, vec!["schema", "my_view"]);
        }
        _ => panic!("expected CREATE VIEW"),
    }
}

// ========== CREATE SCHEMA ==========

#[test]
fn test_create_simple_schema() {
    let stmt = parse_one("CREATE SCHEMA myschema");
    match stmt {
        Statement::CreateSchema(s) => {
            assert_eq!(s.name, Some("myschema".to_string()));
            assert!(s.authorization.is_none());
            assert!(s.elements.is_empty());
            assert!(!s.if_not_exists);
        }
        _ => panic!("expected CREATE SCHEMA"),
    }
}

#[test]
fn test_create_schema_if_not_exists() {
    let stmt = parse_one("CREATE SCHEMA IF NOT EXISTS myschema");
    match stmt {
        Statement::CreateSchema(s) => {
            assert!(s.if_not_exists);
        }
        _ => panic!("expected CREATE SCHEMA"),
    }
}

#[test]
fn test_create_schema_authorization_only() {
    let stmt = parse_one("CREATE SCHEMA AUTHORIZATION admin");
    match stmt {
        Statement::CreateSchema(s) => {
            assert!(s.name.is_none());
            assert_eq!(s.authorization, Some("admin".to_string()));
        }
        _ => panic!("expected CREATE SCHEMA"),
    }
}

#[test]
fn test_create_schema_with_authorization() {
    let stmt = parse_one("CREATE SCHEMA myschema AUTHORIZATION admin");
    match stmt {
        Statement::CreateSchema(s) => {
            assert_eq!(s.name, Some("myschema".to_string()));
            assert_eq!(s.authorization, Some("admin".to_string()));
        }
        _ => panic!("expected CREATE SCHEMA"),
    }
}

// ========== CREATE DATABASE ==========

#[test]
fn test_create_simple_database() {
    let stmt = parse_one("CREATE DATABASE mydb");
    match stmt {
        Statement::CreateDatabase(d) => {
            assert_eq!(d.name, "mydb");
            assert!(d.owner.is_none());
            assert!(d.template.is_none());
            assert!(d.encoding.is_none());
        }
        _ => panic!("expected CREATE DATABASE"),
    }
}

#[test]
fn test_create_database_with_owner() {
    let stmt = parse_one("CREATE DATABASE mydb OWNER = admin");
    match stmt {
        Statement::CreateDatabase(d) => {
            assert_eq!(d.owner, Some("admin".to_string()));
        }
        _ => panic!("expected CREATE DATABASE"),
    }
}

#[test]
fn test_create_database_with_template() {
    let stmt = parse_one("CREATE DATABASE mydb TEMPLATE = template0");
    match stmt {
        Statement::CreateDatabase(d) => {
            assert_eq!(d.template, Some("template0".to_string()));
        }
        _ => panic!("expected CREATE DATABASE"),
    }
}

#[test]
fn test_create_database_with_encoding() {
    let stmt = parse_one("CREATE DATABASE mydb ENCODING = 'UTF8'");
    match stmt {
        Statement::CreateDatabase(d) => {
            assert_eq!(d.encoding, Some("UTF8".to_string()));
        }
        _ => panic!("expected CREATE DATABASE"),
    }
}

#[test]
fn test_create_database_with_tablespace() {
    let stmt = parse_one("CREATE DATABASE mydb TABLESPACE = pg_default");
    match stmt {
        Statement::CreateDatabase(d) => {
            assert_eq!(d.tablespace, Some("pg_default".to_string()));
        }
        _ => panic!("expected CREATE DATABASE"),
    }
}

#[test]
fn test_create_database_with_multiple_options() {
    let stmt = parse_one(
        "CREATE DATABASE mydb WITH OWNER = admin ENCODING = 'UTF8' TABLESPACE = pg_default",
    );
    match stmt {
        Statement::CreateDatabase(d) => {
            assert_eq!(d.owner, Some("admin".to_string()));
            assert_eq!(d.encoding, Some("UTF8".to_string()));
            assert_eq!(d.tablespace, Some("pg_default".to_string()));
        }
        _ => panic!("expected CREATE DATABASE"),
    }
}

// ========== CREATE TABLESPACE ==========

#[test]
fn test_create_tablespace() {
    let stmt = parse_one("CREATE TABLESPACE myspace LOCATION '/data/myspace'");
    match stmt {
        Statement::CreateTablespace(t) => {
            assert_eq!(t.name, "myspace");
            assert_eq!(t.location, "/data/myspace");
            assert!(t.owner.is_none());
        }
        _ => panic!("expected CREATE TABLESPACE"),
    }
}

#[test]
fn test_create_tablespace_with_owner() {
    let stmt = parse_one("CREATE TABLESPACE myspace OWNER admin LOCATION '/data/myspace'");
    match stmt {
        Statement::CreateTablespace(t) => {
            assert_eq!(t.name, "myspace");
            assert_eq!(t.owner, Some("admin".to_string()));
            assert_eq!(t.location, "/data/myspace");
        }
        _ => panic!("expected CREATE TABLESPACE"),
    }
}

#[test]
fn test_copy_from_file() {
    let stmt = parse_one("COPY my_table FROM '/data/file.csv'");
    match stmt {
        Statement::Copy(s) => {
            assert!(s.is_from);
            assert_eq!(s.filename.as_deref(), Some("/data/file.csv"));
            assert!(s.relation.is_some());
            assert!(s.query.is_none());
        }
        _ => panic!("expected Copy statement"),
    }
}

#[test]
fn test_copy_to_stdout() {
    let stmt = parse_one("COPY my_table TO STDOUT");
    match stmt {
        Statement::Copy(s) => {
            assert!(!s.is_from);
            assert!(s.filename.is_none());
        }
        _ => panic!("expected Copy statement"),
    }
}

#[test]
fn test_copy_from_stdin_with_options() {
    let stmt = parse_one("COPY my_table FROM STDIN WITH (FORMAT csv, DELIMITER ',', HEADER)");
    match stmt {
        Statement::Copy(s) => {
            assert!(s.is_from);
            assert_eq!(s.options.len(), 3);
        }
        _ => panic!("expected Copy statement"),
    }
}

#[test]
fn test_copy_with_columns() {
    let stmt = parse_one("COPY my_table(col1, col2) FROM STDIN");
    match stmt {
        Statement::Copy(s) => {
            assert!(s.is_from);
            assert_eq!(s.columns.len(), 2);
        }
        _ => panic!("expected Copy statement"),
    }
}

#[test]
fn test_copy_query_to_file() {
    let stmt = parse_one("COPY (SELECT * FROM foo) TO '/tmp/out.csv'");
    match stmt {
        Statement::Copy(s) => {
            assert!(!s.is_from);
            assert!(s.query.is_some());
            assert!(s.relation.is_none());
        }
        _ => panic!("expected Copy statement"),
    }
}

#[test]
fn test_explain_simple() {
    let stmt = parse_one("EXPLAIN SELECT * FROM foo");
    match stmt {
        Statement::Explain(s) => {
            assert!(!s.analyze);
            assert!(!s.verbose);
            assert!(!s.performance);
        }
        _ => panic!("expected Explain statement"),
    }
}

#[test]
fn test_explain_analyze() {
    let stmt = parse_one("EXPLAIN ANALYZE SELECT 1");
    match stmt {
        Statement::Explain(s) => {
            assert!(s.analyze);
        }
        _ => panic!("expected Explain statement"),
    }
}

#[test]
fn test_explain_with_options() {
    let stmt = parse_one("EXPLAIN (COSTS OFF, TIMING ON, FORMAT JSON) SELECT 1");
    match stmt {
        Statement::Explain(s) => {
            assert_eq!(s.options.len(), 3);
        }
        _ => panic!("expected Explain statement"),
    }
}

#[test]
fn test_call_no_args() {
    let stmt = parse_one("CALL my_func()");
    match stmt {
        Statement::Call(s) => {
            assert!(s.args.is_empty());
        }
        _ => panic!("expected Call statement"),
    }
}

#[test]
fn test_call_with_args() {
    let stmt = parse_one("CALL my_func(1, 'hello')");
    match stmt {
        Statement::Call(s) => {
            assert_eq!(s.args.len(), 2);
        }
        _ => panic!("expected Call statement"),
    }
}

#[test]
fn test_call_qualified_name() {
    let stmt = parse_one("CALL schema.my_func(1)");
    match stmt {
        Statement::Call(s) => {
            assert_eq!(s.func_name.len(), 2);
        }
        _ => panic!("expected Call statement"),
    }
}

// ========== WINDOW FUNCTIONS ==========

#[test]
fn test_window_function_over_partition_by() {
    let stmt =
        parse_one("SELECT ROW_NUMBER() OVER (PARTITION BY dept ORDER BY salary DESC) FROM emp");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            if let SelectTarget::Expr(Expr::FunctionCall { name, over, .. }, _) = &s.targets[0] {
                assert_eq!(name, &vec!["ROW_NUMBER"]);
                let ws = over.as_ref().expect("expected window spec");
                assert_eq!(ws.partition_by.len(), 1);
                assert_eq!(ws.order_by.len(), 1);
                assert!(ws.frame.is_none());
            } else {
                panic!("expected FunctionCall with OVER");
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_window_function_over_window_name() {
    let stmt = parse_one("SELECT SUM(salary) OVER my_window FROM emp");
    match stmt {
        Statement::Select(s) => {
            if let SelectTarget::Expr(Expr::FunctionCall { name, over, .. }, _) = &s.targets[0] {
                assert_eq!(name, &vec!["SUM"]);
                let ws = over.as_ref().expect("expected window spec");
                assert_eq!(ws.window_name.as_deref(), Some("my_window"));
                assert!(ws.partition_by.is_empty());
            } else {
                panic!("expected FunctionCall with OVER");
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_window_function_rows_between() {
    let stmt = parse_one(
        "SELECT AVG(salary) OVER (ORDER BY id ROWS BETWEEN 1 PRECEDING AND 1 FOLLOWING) FROM emp",
    );
    match stmt {
        Statement::Select(s) => {
            if let SelectTarget::Expr(Expr::FunctionCall { over, .. }, _) = &s.targets[0] {
                let ws = over.as_ref().expect("expected window spec");
                let frame = ws.frame.as_ref().expect("expected frame");
                assert_eq!(frame.mode, "ROWS");
                assert!(frame.start.is_some());
                assert!(frame.end.is_some());
            } else {
                panic!("expected FunctionCall with OVER");
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_window_function_range_unbounded() {
    let stmt = parse_one(
        "SELECT SUM(x) OVER (ORDER BY id RANGE BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) FROM t",
    );
    match stmt {
        Statement::Select(s) => {
            if let SelectTarget::Expr(Expr::FunctionCall { over, .. }, _) = &s.targets[0] {
                let ws = over.as_ref().expect("expected window spec");
                let frame = ws.frame.as_ref().expect("expected frame");
                assert_eq!(frame.mode, "RANGE");
            } else {
                panic!("expected FunctionCall with OVER");
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_window_function_no_args_over() {
    let stmt = parse_one("SELECT COUNT(*) OVER () FROM t");
    match stmt {
        Statement::Select(s) => {
            if let SelectTarget::Expr(Expr::FunctionCall { name, over, .. }, _) = &s.targets[0] {
                assert_eq!(name, &vec!["COUNT"]);
                let ws = over.as_ref().expect("expected window spec");
                assert!(ws.partition_by.is_empty());
                assert!(ws.order_by.is_empty());
                assert!(ws.frame.is_none());
            } else {
                panic!("expected FunctionCall with OVER");
            }
        }
        _ => panic!("expected Select"),
    }
}

// ========== CTE MATERIALIZED ==========

#[test]
fn test_cte_not_materialized() {
    let stmt = parse_one("WITH cte AS NOT MATERIALIZED (SELECT * FROM cte) SELECT * FROM cte");
    match stmt {
        Statement::Select(s) => {
            let with = s.with.as_ref().expect("expected WITH clause");
            assert!(!with.recursive);
            assert_eq!(with.ctes.len(), 1);
            assert_eq!(with.ctes[0].materialized, Some(false));
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_cte_materialized() {
    let stmt = parse_one("WITH cte AS MATERIALIZED (SELECT 1) SELECT * FROM cte");
    match stmt {
        Statement::Select(s) => {
            let with = s.with.as_ref().expect("expected WITH clause");
            assert_eq!(with.ctes[0].materialized, Some(true));
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_cte_default_materialized() {
    let stmt = parse_one("WITH cte AS (SELECT 1) SELECT * FROM cte");
    match stmt {
        Statement::Select(s) => {
            let with = s.with.as_ref().expect("expected WITH clause");
            assert_eq!(with.ctes[0].materialized, None);
        }
        _ => panic!("expected Select"),
    }
}

// ========== BIT TYPES ==========

#[test]
fn test_create_table_bit_type() {
    let stmt = parse_one("CREATE TABLE t (col BIT(16))");
    match stmt {
        Statement::CreateTable(t) => {
            assert_eq!(t.columns.len(), 1);
            assert_eq!(t.columns[0].data_type, DataType::Bit(Some(16)));
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_bit_varying() {
    let stmt = parse_one("CREATE TABLE t (col BIT VARYING(20))");
    match stmt {
        Statement::CreateTable(t) => {
            assert_eq!(t.columns.len(), 1);
            assert_eq!(t.columns[0].data_type, DataType::Varbit(Some(20)));
        }
        _ => panic!("expected CreateTable"),
    }
}

// ========== ERROR COLLECTION ==========

#[test]
fn test_parser_collects_errors() {
    let sql = "INSERT INTO (; SELECT 1;";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse().unwrap();
    assert!(parser.has_errors());
    assert!(!parser.errors().is_empty());
}

#[test]
fn test_parser_no_errors_on_valid_sql() {
    let sql = "SELECT 1; INSERT INTO t VALUES (1);";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse().unwrap();
    assert!(!parser.has_errors());
    assert_eq!(stmts.len(), 2);
}

#[test]
fn test_parser_error_recovery() {
    let sql = "INSERT INTO (; SELECT 1;";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse().unwrap();
    assert!(parser.has_errors());
    assert_eq!(parser.errors().len(), 1);
}

#[test]
fn test_parse_one_success() {
    let stmt = Parser::parse_one("SELECT 1").unwrap();
    assert!(matches!(stmt, Statement::Select(_)));
}

#[test]
fn test_error_reports_line_column() {
    let result = Parser::parse_one("SELECT 1; SELECT 2");
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("line"), "Error should contain line: {}", msg);
    assert!(
        msg.contains("column"),
        "Error should contain column: {}",
        msg
    );
}

#[test]
fn test_error_multiline_location() {
    let result = Parser::parse_one("SELECT\n*\nFROM t;\nINSERT INTO t VALUES (1);");
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("line"), "Error should contain line: {}", msg);
}

#[test]
fn test_error_eof_reports_location() {
    let result = Parser::parse_one("");
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("line"),
        "EOF error should contain line: {}",
        msg
    );
}

#[test]
fn test_parse_sql_convenience() {
    let stmts = Parser::parse_sql("SELECT 1; SELECT 2").unwrap();
    assert_eq!(stmts.len(), 2);
}

// ── Visitor tests ──

use crate::ast::visitor::{walk_statement, Visitor, VisitorResult};

struct CountingVisitor {
    statement_count: usize,
    expr_count: usize,
    select_count: usize,
}

impl Visitor for CountingVisitor {
    fn visit_statement(&mut self, _stmt: &Statement) -> VisitorResult {
        self.statement_count += 1;
        VisitorResult::Continue
    }

    fn visit_expr(&mut self, _expr: &Expr) -> VisitorResult {
        self.expr_count += 1;
        VisitorResult::Continue
    }

    fn visit_select(&mut self, _select: &SelectStatement) -> VisitorResult {
        self.select_count += 1;
        VisitorResult::Continue
    }
}

#[test]
fn test_visitor_counts_statements() {
    let stmts = parse("SELECT 1; SELECT 2;");
    let mut visitor = CountingVisitor {
        statement_count: 0,
        expr_count: 0,
        select_count: 0,
    };

    for stmt in &stmts {
        walk_statement(&mut visitor, stmt);
    }

    assert_eq!(visitor.statement_count, 2);
    assert_eq!(visitor.select_count, 2);
}

#[test]
fn test_visitor_counts_exprs() {
    let stmts = parse("SELECT 1 + 2, 3");
    let mut visitor = CountingVisitor {
        statement_count: 0,
        expr_count: 0,
        select_count: 0,
    };

    walk_statement(&mut visitor, &stmts[0]);

    assert_eq!(visitor.statement_count, 1);
    assert_eq!(visitor.select_count, 1);
    // 1 (literal) + 2 (literal) + binary op = 3 exprs, plus 3 (literal) = 4
    assert_eq!(visitor.expr_count, 4);
}

struct StoppingVisitor {
    target_count: usize,
    stop_at: usize,
}

impl Visitor for StoppingVisitor {
    fn visit_expr(&mut self, _expr: &Expr) -> VisitorResult {
        self.target_count += 1;
        if self.target_count >= self.stop_at {
            VisitorResult::Stop
        } else {
            VisitorResult::Continue
        }
    }
}

#[test]
fn test_visitor_can_stop_early() {
    let stmts = parse("SELECT 1 + 2 + 3");
    let mut visitor = StoppingVisitor {
        target_count: 0,
        stop_at: 2,
    };

    walk_statement(&mut visitor, &stmts[0]);

    // Should stop at expr count 2, not visit all 4 exprs
    assert_eq!(visitor.target_count, 2);
}

struct SkippingVisitor {
    expr_count: usize,
}

impl Visitor for SkippingVisitor {
    fn visit_expr(&mut self, _expr: &Expr) -> VisitorResult {
        self.expr_count += 1;
        VisitorResult::SkipChildren
    }
}

#[test]
fn test_visitor_skip_children() {
    let stmts = parse("SELECT 1 + 2");
    let mut visitor = SkippingVisitor { expr_count: 0 };

    walk_statement(&mut visitor, &stmts[0]);

    // Should count the binary op but not descend into operands
    assert_eq!(visitor.expr_count, 1);
}

// ── Streaming API tests ──

use crate::parser::StatementIter;

#[test]
fn test_statement_iter_parses_multiple() {
    let tokens = Tokenizer::new("SELECT 1; SELECT 2; SELECT 3")
        .tokenize()
        .unwrap();
    let mut iter = Parser::new(tokens).into_iter();

    let count = iter.by_ref().filter_map(|r| r.ok()).count();
    assert_eq!(count, 3);
}

#[test]
fn test_statement_iter_handles_semicolons() {
    let tokens = Tokenizer::new("SELECT 1;; SELECT 2;;;").tokenize().unwrap();
    let mut iter = Parser::new(tokens).into_iter();

    let count = iter.by_ref().filter_map(|r| r.ok()).count();
    assert_eq!(count, 2);
}

#[test]
fn test_parse_next_returns_none_at_eof() {
    let tokens = Tokenizer::new("").tokenize().unwrap();
    let mut parser = Parser::new(tokens);

    assert!(parser.parse_next().is_none());
    assert!(parser.parse_next().is_none());
}

// ── Formatter round-trip tests ──

use crate::formatter::SqlFormatter;

fn round_trip(sql: &str) -> bool {
    let stmts = parse(sql);
    let formatter = SqlFormatter::new();
    for stmt in &stmts {
        let formatted = formatter.format_statement(stmt);
        if formatted.is_empty() {
            continue;
        }
        if formatted.starts_with("/* stub:") {
            continue;
        }
        if let Ok(reparsed) = Parser::parse_sql(&formatted) {
            if reparsed.is_empty() {
                return false;
            }
        }
    }
    true
}

#[test]
fn test_round_trip_select() {
    assert!(round_trip(
        "SELECT id, name FROM users WHERE status = 'active'"
    ));
}

#[test]
fn test_round_trip_select_with_join() {
    assert!(round_trip("SELECT * FROM t1 JOIN t2 ON t1.id = t2.id"));
}

#[test]
fn test_round_trip_insert() {
    assert!(round_trip("INSERT INTO t (a, b) VALUES (1, 2)"));
}

#[test]
fn test_round_trip_update() {
    assert!(round_trip("UPDATE t SET a = 1 WHERE b = 2"));
}

#[test]
fn test_round_trip_delete() {
    assert!(round_trip("DELETE FROM t WHERE id = 1"));
}

#[test]
fn test_round_trip_create_table() {
    assert!(round_trip(
        "CREATE TABLE t (id INTEGER PRIMARY KEY, name TEXT)"
    ));
}

#[test]
fn test_round_trip_create_index() {
    assert!(round_trip("CREATE INDEX idx ON t (col)"));
}

#[test]
fn test_round_trip_create_view() {
    assert!(round_trip("CREATE VIEW v AS SELECT * FROM t"));
}

#[test]
fn test_round_trip_transaction() {
    assert!(round_trip("BEGIN"));
    assert!(round_trip("COMMIT"));
    assert!(round_trip("ROLLBACK"));
}

#[test]
fn test_round_trips_multiple_statements() {
    assert!(round_trip(
        "CREATE TABLE t (id INTEGER); INSERT INTO t VALUES (1); SELECT * FROM t"
    ));
}

// ── openGauss extension tests (Waves 4-7) ──

#[test]
fn test_parse_create_fdw() {
    let stmts = parse("CREATE FOREIGN DATA WRAPPER foo OPTIONS (testing '1', another '2')");
    assert!(matches!(&stmts[0], Statement::CreateFdw(_)));
}

#[test]
fn test_parse_create_server() {
    let stmts = parse("CREATE SERVER s1 FOREIGN DATA WRAPPER foo OPTIONS (host 'a', dbname 'b')");
    assert!(matches!(&stmts[0], Statement::CreateForeignServer(_)));
}

#[test]
fn test_parse_create_foreign_table() {
    let stmts =
        parse("CREATE FOREIGN TABLE ft1 (c1 integer, c2 text) SERVER s0 OPTIONS (delimiter ',')");
    assert!(matches!(&stmts[0], Statement::CreateForeignTable(_)));
}

#[test]
fn test_parse_create_publication() {
    let stmts = parse("CREATE PUBLICATION pub1 FOR TABLE t1, t2");
    assert!(matches!(&stmts[0], Statement::CreatePublication(_)));
}

#[test]
fn test_parse_create_subscription() {
    let stmts = parse("CREATE SUBSCRIPTION sub1 CONNECTION 'host=abc' PUBLICATION pub1");
    assert!(matches!(&stmts[0], Statement::CreateSubscription(_)));
}

#[test]
fn test_parse_create_node() {
    let stmts = parse("CREATE NODE dn1 WITH (TYPE = 'datanode', HOST = 'localhost')");
    assert!(matches!(&stmts[0], Statement::CreateNode(_)));
}

#[test]
fn test_parse_create_node_group() {
    let stmts = parse("CREATE NODE GROUP grp1 WITH (dn1, dn2)");
    assert!(matches!(&stmts[0], Statement::CreateNodeGroup(_)));
}

#[test]
fn test_parse_create_resource_pool() {
    let stmts = parse("CREATE RESOURCE POOL rp1 WITH (MEM_PERCENT = 20)");
    assert!(matches!(&stmts[0], Statement::CreateResourcePool(_)));
}

#[test]
fn test_parse_create_workload_group() {
    let stmts = parse("CREATE WORKLOAD GROUP wg1 USING RESOURCE POOL rp1");
    assert!(matches!(&stmts[0], Statement::CreateWorkloadGroup(_)));
}

#[test]
fn test_parse_create_audit_policy() {
    let stmts = parse("CREATE AUDIT POLICY ap1 PRIVILEGES SELECT ON TABLE t1");
    assert!(matches!(&stmts[0], Statement::CreateAuditPolicy(_)));
}

#[test]
fn test_parse_create_masking_policy() {
    let stmts = parse("CREATE MASKING POLICY mp1 WITH (masking_function ON (col1))");
    assert!(matches!(&stmts[0], Statement::CreateMaskingPolicy(_)));
}

#[test]
fn test_parse_create_rls_policy() {
    let stmts = parse("CREATE POLICY rls1 ON t1 USING (id > 0)");
    assert!(matches!(&stmts[0], Statement::CreateRlsPolicy(_)));
}

#[test]
fn test_format_create_fdw() {
    let stmts = parse("CREATE FOREIGN DATA WRAPPER foo");
    let formatted = SqlFormatter::new().format_statement(&stmts[0]);
    assert!(formatted.contains("FOREIGN DATA WRAPPER"));
    assert!(formatted.contains("foo"));
}

#[test]
fn test_format_create_publication() {
    let stmts = parse("CREATE PUBLICATION pub1 FOR ALL TABLES");
    let formatted = SqlFormatter::new().format_statement(&stmts[0]);
    assert!(formatted.contains("PUBLICATION"));
    assert!(formatted.contains("FOR ALL TABLES"));
}

#[test]
fn test_format_create_node() {
    let stmts = parse("CREATE NODE dn1 WITH (TYPE = 'datanode')");
    let formatted = SqlFormatter::new().format_statement(&stmts[0]);
    assert!(formatted.contains("CREATE NODE"));
}

// ========== Wave 1: SELECT FOR UPDATE/SHARE + FETCH FIRST ==========

#[test]
fn test_select_for_update() {
    let stmt = parse_one("SELECT * FROM t FOR UPDATE");
    match stmt {
        Statement::Select(s) => {
            assert!(matches!(s.lock_clause, Some(LockClause::Update { .. })));
            let lc = s.lock_clause.as_ref().unwrap();
            match lc {
                LockClause::Update {
                    tables,
                    nowait,
                    skip_locked,
                } => {
                    assert!(tables.is_empty());
                    assert!(!nowait);
                    assert!(!skip_locked);
                }
                _ => panic!("expected Update lock"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_for_share_of_tables_nowait() {
    let stmt = parse_one("SELECT * FROM t FOR SHARE OF t1, t2 NOWAIT");
    match stmt {
        Statement::Select(s) => match s.lock_clause.as_ref().unwrap() {
            LockClause::Share { tables, nowait, .. } => {
                assert_eq!(tables.len(), 2);
                assert_eq!(tables[0], vec!["t1"]);
                assert_eq!(tables[1], vec!["t2"]);
                assert!(*nowait);
            }
            _ => panic!("expected Share lock"),
        },
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_for_key_share_skip_locked() {
    let stmt = parse_one("SELECT * FROM t FOR KEY SHARE SKIP LOCKED");
    match stmt {
        Statement::Select(s) => match s.lock_clause.as_ref().unwrap() {
            LockClause::KeyShare { skip_locked, .. } => {
                assert!(*skip_locked);
            }
            _ => panic!("expected KeyShare lock"),
        },
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_for_no_key_update() {
    let stmt = parse_one("SELECT * FROM t FOR NO KEY UPDATE");
    match stmt {
        Statement::Select(s) => {
            assert!(matches!(
                s.lock_clause,
                Some(LockClause::NoKeyUpdate { .. })
            ));
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_fetch_first_n_rows() {
    let stmt = parse_one("SELECT * FROM t FETCH FIRST 10 ROWS ONLY");
    match stmt {
        Statement::Select(s) => {
            let fetch = s.fetch.as_ref().expect("expected fetch clause");
            assert!(!fetch.with_ties);
            assert!(fetch.count.is_some());
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_fetch_first_row_with_ties() {
    let stmt = parse_one("SELECT * FROM t ORDER BY id FETCH FIRST ROW WITH TIES");
    match stmt {
        Statement::Select(s) => {
            let fetch = s.fetch.as_ref().expect("expected fetch clause");
            assert!(fetch.with_ties);
            assert!(fetch.count.is_none());
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_limit_and_for_update() {
    let stmt = parse_one("SELECT * FROM t LIMIT 10 OFFSET 5 FOR UPDATE OF t1 NOWAIT");
    match stmt {
        Statement::Select(s) => {
            assert!(s.limit.is_some());
            assert!(s.offset.is_some());
            match s.lock_clause.as_ref().unwrap() {
                LockClause::Update { tables, nowait, .. } => {
                    assert_eq!(tables.len(), 1);
                    assert!(*nowait);
                }
                _ => panic!("expected Update lock"),
            }
        }
        _ => panic!("expected Select"),
    }
}

// ========== Wave 2: INSERT ON DUPLICATE KEY UPDATE ==========

#[test]
fn test_insert_on_duplicate_key_update() {
    let stmt = parse_one("INSERT INTO t (a, b) VALUES (1, 2) ON DUPLICATE KEY UPDATE b = 3");
    match stmt {
        Statement::Insert(s) => {
            assert!(s.on_conflict.is_some());
            match s.on_conflict.as_ref().unwrap() {
                OnConflictAction::Update {
                    assignments,
                    where_clause,
                    ..
                } => {
                    assert_eq!(assignments.len(), 1);
                    assert!(where_clause.is_none());
                }
                _ => panic!("expected Update action"),
            }
        }
        _ => panic!("expected Insert"),
    }
}

#[test]
fn test_insert_on_duplicate_key_update_with_where() {
    let stmt = parse_one(
        "INSERT INTO t (a, b) VALUES (1, 2) ON DUPLICATE KEY UPDATE a = a + 1 WHERE t.c > 0",
    );
    match stmt {
        Statement::Insert(s) => match s.on_conflict.as_ref().unwrap() {
            OnConflictAction::Update { where_clause, .. } => {
                assert!(where_clause.is_some());
            }
            _ => panic!("expected Update action"),
        },
        _ => panic!("expected Insert"),
    }
}

#[test]
fn test_insert_no_conflict() {
    let stmt = parse_one("INSERT INTO t (a) VALUES (1)");
    match stmt {
        Statement::Insert(s) => {
            assert!(s.on_conflict.is_none());
        }
        _ => panic!("expected Insert"),
    }
}

// ========== Wave 3: CREATE TABLE enhancements ==========

#[test]
fn test_create_temp_table() {
    let stmt = parse_one("CREATE TEMP TABLE t (id INT)");
    match stmt {
        Statement::CreateTable(s) => {
            assert!(s.temporary);
            assert!(!s.unlogged);
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_unlogged_table() {
    let stmt = parse_one("CREATE UNLOGGED TABLE t (id INT)");
    match stmt {
        Statement::CreateTable(s) => {
            assert!(!s.temporary);
            assert!(s.unlogged);
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_with_options() {
    let stmt =
        parse_one("CREATE TABLE t (id INT) WITH (fillfactor = 70, autovacuum_enabled = false)");
    match stmt {
        Statement::CreateTable(s) => {
            assert_eq!(s.options.len(), 2);
            assert_eq!(s.options[0], ("fillfactor".to_string(), "70".to_string()));
            assert_eq!(
                s.options[1],
                ("autovacuum_enabled".to_string(), "false".to_string())
            );
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_inherits() {
    let stmt = parse_one("CREATE TABLE t (id INT) INHERITS (parent1, parent2)");
    match stmt {
        Statement::CreateTable(s) => {
            assert_eq!(s.inherits.len(), 2);
            assert_eq!(s.inherits[0], vec!["parent1"]);
            assert_eq!(s.inherits[1], vec!["parent2"]);
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_partition_by_range() {
    let stmt = parse_one("CREATE TABLE t (id INT, dt DATE) PARTITION BY RANGE (dt)");
    match stmt {
        Statement::CreateTable(s) => {
            assert!(s.partition_by.is_some());
            match s.partition_by.as_ref().unwrap() {
                PartitionClause::Range { column } => {
                    assert_eq!(column, &vec!["dt"]);
                }
                _ => panic!("expected Range partition"),
            }
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_partition_by_list() {
    let stmt = parse_one("CREATE TABLE t (id INT, region TEXT) PARTITION BY LIST (region)");
    match stmt {
        Statement::CreateTable(s) => match s.partition_by.as_ref().unwrap() {
            PartitionClause::List { column } => {
                assert_eq!(column, &vec!["region"]);
            }
            _ => panic!("expected List partition"),
        },
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_tablespace() {
    let stmt = parse_one("CREATE TABLE t (id INT) TABLESPACE ts1");
    match stmt {
        Statement::CreateTable(s) => {
            assert_eq!(s.tablespace.as_deref(), Some("ts1"));
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_temp_table_on_commit() {
    let stmt = parse_one("CREATE TEMP TABLE t (id INT) ON COMMIT DELETE ROWS");
    match stmt {
        Statement::CreateTable(s) => {
            assert!(s.temporary);
            assert!(matches!(s.on_commit, Some(OnCommitAction::DeleteRows)));
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_combined() {
    let stmt = parse_one(
        "CREATE UNLOGGED TABLE IF NOT EXISTS t (id INT PRIMARY KEY) \
         INHERITS (base) WITH (fillfactor = 90) TABLESPACE ts1",
    );
    match stmt {
        Statement::CreateTable(s) => {
            assert!(s.unlogged);
            assert!(s.if_not_exists);
            assert!(!s.inherits.is_empty());
            assert!(!s.options.is_empty());
            assert!(s.tablespace.is_some());
        }
        _ => panic!("expected CreateTable"),
    }
}

// ========== CREATE FUNCTION ==========

#[test]
fn test_create_function_simple() {
    let stmt =
        parse_one("CREATE FUNCTION my_func() RETURNS integer AS $$ SELECT 1 $$ LANGUAGE sql");
    match stmt {
        Statement::CreateFunction(f) => {
            assert!(!f.replace);
            assert_eq!(f.name, vec!["my_func"]);
            assert!(f.parameters.is_empty());
            assert_eq!(f.return_type, Some("integer".to_string()));
        }
        _ => panic!("expected CreateFunction statement"),
    }
}

#[test]
fn test_create_function_with_params() {
    let stmt = parse_one("CREATE FUNCTION add(a integer, b integer) RETURNS integer AS $$ SELECT a + b $$ LANGUAGE sql");
    match stmt {
        Statement::CreateFunction(f) => {
            assert_eq!(f.name, vec!["add"]);
            assert_eq!(f.parameters.len(), 2);
            assert_eq!(f.return_type, Some("integer".to_string()));
        }
        _ => panic!("expected CreateFunction statement"),
    }
}

// ========== CREATE PROCEDURE ==========

#[test]
fn test_create_procedure_simple() {
    let stmt = parse_one("CREATE PROCEDURE my_proc() AS $$ BEGIN NULL; END $$ LANGUAGE plpgsql");
    match stmt {
        Statement::CreateProcedure(p) => {
            assert!(!p.replace);
            assert_eq!(p.name, vec!["my_proc"]);
            assert!(p.parameters.is_empty());
        }
        _ => panic!("expected CreateProcedure statement"),
    }
}

#[test]
fn test_create_procedure_with_params() {
    let stmt = parse_one("CREATE PROCEDURE insert_data(x integer, y integer) AS $$ BEGIN INSERT INTO test VALUES (x, y); END $$ LANGUAGE plpgsql");
    match stmt {
        Statement::CreateProcedure(p) => {
            assert_eq!(p.name, vec!["insert_data"]);
            assert_eq!(p.parameters.len(), 2);
        }
        _ => panic!("expected CreateProcedure statement"),
    }
}

// ========== Wave 4: ALTER TABLE enhancements ==========

#[test]
fn test_alter_table_add_constraint() {
    let stmt = parse_one("ALTER TABLE t ADD CONSTRAINT pk PRIMARY KEY (id)");
    match stmt {
        Statement::AlterTable(s) => match &s.actions[0] {
            AlterTableAction::AddConstraint { name, constraint } => {
                assert_eq!(name.as_deref(), Some("pk"));
                assert!(matches!(constraint, TableConstraint::PrimaryKey(_)));
            }
            _ => panic!("expected AddConstraint"),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_add_unique() {
    let stmt = parse_one("ALTER TABLE t ADD UNIQUE (email)");
    match stmt {
        Statement::AlterTable(s) => match &s.actions[0] {
            AlterTableAction::AddConstraint { constraint, .. } => {
                assert!(matches!(constraint, TableConstraint::Unique(_)));
            }
            _ => panic!("expected AddConstraint"),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_drop_constraint() {
    let stmt = parse_one("ALTER TABLE t DROP CONSTRAINT ck CASCADE");
    match stmt {
        Statement::AlterTable(s) => match &s.actions[0] {
            AlterTableAction::DropConstraint { name, cascade, .. } => {
                assert_eq!(name, "ck");
                assert!(*cascade);
            }
            _ => panic!("expected DropConstraint"),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_rename_column() {
    let stmt = parse_one("ALTER TABLE t RENAME COLUMN old_name TO new_name");
    match stmt {
        Statement::AlterTable(s) => match &s.actions[0] {
            AlterTableAction::RenameColumn { old, new } => {
                assert_eq!(old, "old_name");
                assert_eq!(new, "new_name");
            }
            _ => panic!("expected RenameColumn"),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_rename_to() {
    let stmt = parse_one("ALTER TABLE t RENAME TO new_table");
    match stmt {
        Statement::AlterTable(s) => match &s.actions[0] {
            AlterTableAction::RenameTo { new_name } => {
                assert_eq!(new_name, "new_table");
            }
            _ => panic!("expected RenameTo"),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_owner_to() {
    let stmt = parse_one("ALTER TABLE t OWNER TO admin");
    match stmt {
        Statement::AlterTable(s) => match &s.actions[0] {
            AlterTableAction::OwnerTo { owner } => {
                assert_eq!(owner, "admin");
            }
            _ => panic!("expected OwnerTo"),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_set_schema() {
    let stmt = parse_one("ALTER TABLE t SET SCHEMA new_schema");
    match stmt {
        Statement::AlterTable(s) => match &s.actions[0] {
            AlterTableAction::SetSchema { schema } => {
                assert_eq!(schema, "new_schema");
            }
            _ => panic!("expected SetSchema"),
        },
        _ => panic!("expected AlterTable"),
    }
}

// ========== Wave 5: DROP more types + CREATE INDEX ==========

#[test]
fn test_drop_tablespace() {
    let stmt = parse_one("DROP TABLESPACE ts1");
    match stmt {
        Statement::Drop(s) => {
            assert!(matches!(&s.object_type, ObjectType::Tablespace));
        }
        _ => panic!("expected Drop"),
    }
}

#[test]
fn test_drop_function() {
    let stmt = parse_one("DROP FUNCTION IF EXISTS my_func CASCADE");
    match stmt {
        Statement::Drop(s) => {
            assert!(matches!(&s.object_type, ObjectType::Function));
            assert!(s.if_exists);
            assert!(s.cascade);
        }
        _ => panic!("expected Drop"),
    }
}

#[test]
fn test_drop_trigger() {
    let stmt = parse_one("DROP TRIGGER trg1");
    match stmt {
        Statement::Drop(s) => {
            assert!(matches!(&s.object_type, ObjectType::Trigger));
        }
        _ => panic!("expected Drop"),
    }
}

#[test]
fn test_drop_materialized_view() {
    let stmt = parse_one("DROP MATERIALIZED VIEW mv1");
    match stmt {
        Statement::Drop(s) => {
            assert!(matches!(&s.object_type, ObjectType::MaterializedView));
        }
        _ => panic!("expected Drop"),
    }
}

#[test]
fn test_create_index_concurrently() {
    let stmt = parse_one("CREATE INDEX CONCURRENTLY idx ON t (col)");
    match stmt {
        Statement::CreateIndex(s) => {
            assert!(s.concurrent);
        }
        _ => panic!("expected CreateIndex"),
    }
}

#[test]
fn test_create_index_tablespace() {
    let stmt = parse_one("CREATE INDEX idx ON t (col) TABLESPACE ts1");
    match stmt {
        Statement::CreateIndex(s) => {
            assert_eq!(s.tablespace.as_deref(), Some("ts1"));
        }
        _ => panic!("expected CreateIndex"),
    }
}

// ========== Wave 6: GRANT / REVOKE ==========

#[test]
fn test_grant_select_insert() {
    let stmt = parse_one("GRANT SELECT, INSERT ON table1 TO user1, user2");
    match stmt {
        Statement::Grant(s) => {
            assert_eq!(s.privileges.len(), 2);
            assert!(matches!(&s.privileges[0], Privilege::Select));
            assert!(matches!(&s.privileges[1], Privilege::Insert));
            assert_eq!(s.grantees, vec!["user1", "user2"]);
        }
        _ => panic!("expected Grant"),
    }
}

#[test]
fn test_grant_all_on_schema() {
    let stmt = parse_one("GRANT ALL PRIVILEGES ON SCHEMA public TO admin WITH GRANT OPTION");
    match stmt {
        Statement::Grant(s) => {
            assert!(matches!(&s.privileges[0], Privilege::All));
            assert!(matches!(&s.target, GrantTarget::Schema(_)));
            assert!(s.with_grant_option);
        }
        _ => panic!("expected Grant"),
    }
}

#[test]
fn test_grant_execute_on_function() {
    let stmt = parse_one("GRANT EXECUTE ON FUNCTION func1(INT) TO public");
    match stmt {
        Statement::Grant(s) => {
            assert!(matches!(&s.privileges[0], Privilege::Execute));
            assert!(matches!(&s.target, GrantTarget::Function(_)));
        }
        _ => panic!("expected Grant"),
    }
}

#[test]
fn test_grant_all_tables_in_schema() {
    let stmt = parse_one("GRANT SELECT ON ALL TABLES IN SCHEMA public TO reader");
    match stmt {
        Statement::Grant(s) => {
            assert!(matches!(&s.target, GrantTarget::AllTablesInSchema(_)));
        }
        _ => panic!("expected Grant"),
    }
}

#[test]
fn test_revoke_insert() {
    let stmt = parse_one("REVOKE INSERT ON table1 FROM user1");
    match stmt {
        Statement::Revoke(s) => {
            assert!(matches!(&s.privileges[0], Privilege::Insert));
            assert_eq!(s.grantees, vec!["user1"]);
            assert!(!s.cascade);
        }
        _ => panic!("expected Revoke"),
    }
}

#[test]
fn test_revoke_cascade() {
    let stmt = parse_one("REVOKE ALL ON SCHEMA public FROM admin CASCADE");
    match stmt {
        Statement::Revoke(s) => {
            assert!(matches!(&s.privileges[0], Privilege::All));
            assert!(s.cascade);
        }
        _ => panic!("expected Revoke"),
    }
}

// ========== Wave 8: CREATE TRIGGER + MATERIALIZED VIEW ==========

#[test]
fn test_create_trigger_insert_update() {
    let stmt = parse_one(
        "CREATE TRIGGER trg AFTER INSERT OR UPDATE ON t FOR EACH ROW EXECUTE PROCEDURE func1()",
    );
    match stmt {
        Statement::CreateTrigger(s) => {
            assert_eq!(s.name, "trg");
            assert_eq!(s.events.len(), 2);
            assert!(matches!(&s.events[0], TriggerEvent::Insert));
            assert!(matches!(&s.events[1], TriggerEvent::Update));
            assert!(matches!(s.for_each, TriggerForEach::Row));
        }
        _ => panic!("expected CreateTrigger"),
    }
}

#[test]
fn test_create_trigger_delete_when() {
    let stmt = parse_one("CREATE TRIGGER trg BEFORE DELETE ON t FOR EACH ROW WHEN (OLD.status = 'active') EXECUTE PROCEDURE func2()");
    match stmt {
        Statement::CreateTrigger(s) => {
            assert_eq!(s.name, "trg");
            assert!(s.when.is_some());
        }
        _ => panic!("expected CreateTrigger"),
    }
}

#[test]
fn test_create_materialized_view() {
    let stmt = parse_one("CREATE MATERIALIZED VIEW mv AS SELECT * FROM t WITH DATA");
    match stmt {
        Statement::CreateMaterializedView(s) => {
            assert_eq!(s.name, vec!["mv"]);
            assert!(s.with_data);
        }
        _ => panic!("expected CreateMaterializedView"),
    }
}

#[test]
fn test_refresh_materialized_view() {
    let stmt = parse_one("REFRESH MATERIALIZED VIEW mv");
    match stmt {
        Statement::RefreshMaterializedView(s) => {
            assert!(!s.concurrent);
            assert_eq!(s.name, vec!["mv"]);
        }
        _ => panic!("expected RefreshMaterializedView"),
    }
}

#[test]
fn test_refresh_materialized_view_concurrently() {
    let stmt = parse_one("REFRESH MATERIALIZED VIEW CONCURRENTLY mv");
    match stmt {
        Statement::RefreshMaterializedView(s) => {
            assert!(s.concurrent);
        }
        _ => panic!("expected RefreshMaterializedView"),
    }
}

// ========== Wave 9: VACUUM / ANALYZE / COMMENT ON / LOCK TABLE ==========

#[test]
fn test_vacuum_simple() {
    let stmt = parse_one("VACUUM");
    match stmt {
        Statement::Vacuum(s) => {
            assert!(!s.full);
            assert!(!s.analyze);
            assert!(s.tables.is_empty());
        }
        _ => panic!("expected Vacuum"),
    }
}

#[test]
fn test_vacuum_full_analyze() {
    let stmt = parse_one("VACUUM FULL VERBOSE ANALYZE t1, t2");
    match stmt {
        Statement::Vacuum(s) => {
            assert!(s.full);
            assert!(s.verbose);
            assert!(s.analyze);
            assert_eq!(s.tables.len(), 2);
        }
        _ => panic!("expected Vacuum"),
    }
}

#[test]
fn test_analyze_simple() {
    let stmt = parse_one("ANALYZE");
    match stmt {
        Statement::Analyze(s) => {
            assert!(!s.verbose);
        }
        _ => panic!("expected Analyze"),
    }
}

#[test]
fn test_analyze_verbose() {
    let stmt = parse_one("ANALYZE VERBOSE t1");
    match stmt {
        Statement::Analyze(s) => {
            assert!(s.verbose);
            assert_eq!(s.tables.len(), 1);
        }
        _ => panic!("expected Analyze"),
    }
}

#[test]
fn test_comment_on_table() {
    let stmt = parse_one("COMMENT ON TABLE t IS 'my table'");
    match stmt {
        Statement::Comment(s) => {
            assert_eq!(s.object_type, "TABLE");
            assert_eq!(s.comment, "my table");
        }
        _ => panic!("expected Comment"),
    }
}

#[test]
fn test_comment_on_column() {
    let stmt = parse_one("COMMENT ON COLUMN t.id IS 'primary key'");
    match stmt {
        Statement::Comment(s) => {
            assert_eq!(s.object_type, "COLUMN");
            assert_eq!(s.comment, "primary key");
        }
        _ => panic!("expected Comment"),
    }
}

#[test]
fn test_lock_table() {
    let stmt = parse_one("LOCK TABLE t1, t2 IN ACCESS EXCLUSIVE MODE NOWAIT");
    match stmt {
        Statement::Lock(s) => {
            assert_eq!(s.tables.len(), 2);
            assert!(s.nowait);
        }
        _ => panic!("expected Lock"),
    }
}

// ========== Wave 10: PREPARE / EXECUTE / DEALLOCATE / DO ==========

#[test]
fn test_prepare() {
    let stmt = parse_one("PREPARE stmt (INT, TEXT) AS SELECT * FROM t WHERE id = $1");
    match stmt {
        Statement::Prepare(s) => {
            assert_eq!(s.name, "stmt");
            assert_eq!(s.data_types.len(), 2);
            assert!(s.statement.to_lowercase().contains("select"));
        }
        _ => panic!("expected Prepare"),
    }
}

#[test]
fn test_execute() {
    let stmt = parse_one("EXECUTE stmt(1, 'test')");
    match stmt {
        Statement::Execute(s) => {
            assert_eq!(s.name, "stmt");
            assert_eq!(s.params.len(), 2);
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_deallocate() {
    let stmt = parse_one("DEALLOCATE stmt");
    match stmt {
        Statement::Deallocate(s) => {
            assert_eq!(s.name.as_deref(), Some("stmt"));
            assert!(!s.all);
        }
        _ => panic!("expected Deallocate"),
    }
}

#[test]
fn test_deallocate_all() {
    let stmt = parse_one("DEALLOCATE ALL");
    match stmt {
        Statement::Deallocate(s) => {
            assert!(s.all);
        }
        _ => panic!("expected Deallocate"),
    }
}

#[test]
fn test_do() {
    let stmt = parse_one("DO $$ BEGIN RAISE NOTICE 'hello'; END $$");
    match stmt {
        Statement::Do(s) => {
            assert!(s.code.contains("BEGIN"));
        }
        _ => panic!("expected Do"),
    }
}

#[test]
fn test_do_with_language() {
    let stmt = parse_one("DO LANGUAGE plpgsql $$ BEGIN RAISE NOTICE 'hello'; END $$");
    match stmt {
        Statement::Do(s) => {
            assert_eq!(s.language.as_deref(), Some("plpgsql"));
        }
        _ => panic!("expected Do"),
    }
}

// ========== Wave 11: ALTER DATABASE/SCHEMA/SEQUENCE/FUNCTION/ROLE/USER/SYSTEM ==========

#[test]
fn test_alter_database_set() {
    let stmt = parse_one("ALTER DATABASE mydb SET search_path TO public");
    match stmt {
        Statement::AlterDatabase(s) => {
            assert_eq!(s.name, "mydb");
            assert!(matches!(&s.action, AlterDatabaseAction::Set { .. }));
        }
        _ => panic!("expected AlterDatabase"),
    }
}

#[test]
fn test_alter_database_rename() {
    let stmt = parse_one("ALTER DATABASE mydb RENAME TO newdb");
    match stmt {
        Statement::AlterDatabase(s) => {
            assert!(
                matches!(&s.action, AlterDatabaseAction::RenameTo { ref new_name } if new_name == "newdb")
            );
        }
        _ => panic!("expected AlterDatabase"),
    }
}

#[test]
fn test_alter_schema_rename() {
    let stmt = parse_one("ALTER SCHEMA myschema RENAME TO newschema");
    match stmt {
        Statement::AlterSchema(s) => {
            assert_eq!(s.name, "myschema");
            assert!(
                matches!(&s.action, AlterSchemaAction::RenameTo { ref new_name } if new_name == "newschema")
            );
        }
        _ => panic!("expected AlterSchema"),
    }
}

#[test]
fn test_alter_sequence() {
    let stmt = parse_one("ALTER SEQUENCE seq INCREMENT BY 2 RESTART WITH 100");
    match stmt {
        Statement::AlterSequence(s) => {
            assert_eq!(s.name, vec!["seq"]);
            assert_eq!(s.options.len(), 3);
            assert!(matches!(&s.options[0], SequenceOption::IncrementBy(2)));
            assert!(matches!(&s.options[1], SequenceOption::Restart(true)));
            assert!(matches!(&s.options[2], SequenceOption::StartWith(100)));
        }
        _ => panic!("expected AlterSequence"),
    }
}

#[test]
fn test_alter_function_rename() {
    let stmt = parse_one("ALTER FUNCTION add(INT, INT) RENAME TO sum_it");
    match stmt {
        Statement::AlterFunction(s) => {
            assert!(
                matches!(&s.action, AlterFunctionAction::RenameTo { ref new_name } if new_name == "sum_it")
            );
        }
        _ => panic!("expected AlterFunction"),
    }
}

#[test]
fn test_alter_role() {
    let stmt = parse_one("ALTER ROLE admin WITH PASSWORD 'secret'");
    match stmt {
        Statement::AlterRole(s) => {
            assert_eq!(s.name, "admin");
            assert!(s.options.iter().any(|(k, _)| k == "PASSWORD"));
        }
        _ => panic!("expected AlterRole"),
    }
}

#[test]
fn test_alter_system_set() {
    let stmt = parse_one("ALTER SYSTEM SET max_connections = 200");
    match stmt {
        Statement::AlterGlobalConfig(s) => {
            assert!(matches!(&s.action, AlterGlobalConfigAction::Set { .. }));
        }
        _ => panic!("expected AlterGlobalConfig"),
    }
}

// ========== Wave 12: CURSOR / LISTEN / NOTIFY / RULE / CLUSTER / REINDEX ==========

#[test]
fn test_declare_cursor() {
    let stmt = parse_one("DECLARE cur CURSOR FOR SELECT * FROM t");
    match stmt {
        Statement::DeclareCursor(s) => {
            assert_eq!(s.name, "cur");
        }
        _ => panic!("expected DeclareCursor"),
    }
}

#[test]
fn test_declare_cursor_scroll_hold() {
    let stmt = parse_one("DECLARE cur BINARY SCROLL CURSOR WITH HOLD FOR SELECT * FROM t");
    match stmt {
        Statement::DeclareCursor(s) => {
            assert!(s.binary);
            assert!(s.scroll);
            assert!(s.hold);
        }
        _ => panic!("expected DeclareCursor"),
    }
}

#[test]
fn test_fetch_next() {
    let stmt = parse_one("FETCH NEXT FROM cur");
    match stmt {
        Statement::Fetch(s) => {
            assert!(matches!(s.direction, FetchDirection::Next));
            assert_eq!(s.cursor_name, "cur");
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_fetch_all() {
    let stmt = parse_one("FETCH ALL FROM cur");
    match stmt {
        Statement::Fetch(s) => {
            assert!(matches!(s.direction, FetchDirection::All));
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_close_portal() {
    let stmt = parse_one("CLOSE cur");
    match stmt {
        Statement::ClosePortal(s) => {
            assert_eq!(s.name, "cur");
        }
        _ => panic!("expected ClosePortal"),
    }
}

#[test]
fn test_listen() {
    let stmt = parse_one("LISTEN mychannel");
    match stmt {
        Statement::Listen(s) => {
            assert_eq!(s.channel, "mychannel");
        }
        _ => panic!("expected Listen"),
    }
}

#[test]
fn test_notify() {
    let stmt = parse_one("NOTIFY mychannel, 'payload'");
    match stmt {
        Statement::Notify(s) => {
            assert_eq!(s.channel, "mychannel");
            assert_eq!(s.payload.as_deref(), Some("payload"));
        }
        _ => panic!("expected Notify"),
    }
}

#[test]
fn test_unlisten() {
    let stmt = parse_one("UNLISTEN mychannel");
    match stmt {
        Statement::Unlisten(s) => {
            assert_eq!(s.channel.as_deref(), Some("mychannel"));
        }
        _ => panic!("expected Unlisten"),
    }
}

#[test]
fn test_cluster() {
    let stmt = parse_one("CLUSTER VERBOSE t1");
    match stmt {
        Statement::Cluster(s) => {
            assert!(s.verbose);
            assert!(s.table.is_some());
        }
        _ => panic!("expected Cluster"),
    }
}

#[test]
fn test_reindex_table() {
    let stmt = parse_one("REINDEX TABLE t1");
    match stmt {
        Statement::Reindex(s) => {
            assert!(matches!(&s.target, ReindexTarget::Table(_)));
        }
        _ => panic!("expected Reindex"),
    }
}

#[test]
fn test_reindex_index_concurrently() {
    let stmt = parse_one("REINDEX INDEX CONCURRENTLY idx1");
    match stmt {
        Statement::Reindex(s) => {
            assert!(s.concurrent);
            assert!(matches!(&s.target, ReindexTarget::Index(_)));
        }
        _ => panic!("expected Reindex"),
    }
}
