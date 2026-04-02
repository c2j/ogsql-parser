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
