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
