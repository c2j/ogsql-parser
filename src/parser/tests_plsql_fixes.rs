use crate::ast::plpgsql::*;
use crate::ast::*;
use crate::formatter::SqlFormatter;
use crate::parser::{Parser, ParserError};
use crate::token::keyword::lookup_keyword;
use crate::token::tokenizer::Tokenizer;
use crate::token::Token;

fn parse(sql: &str) -> Vec<Statement> {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    Parser::new(tokens).parse()
}

fn parse_one(sql: &str) -> Statement {
    let stmts = parse(sql);
    stmts
        .into_iter()
        .next()
        .expect("expected at least one statement")
}

fn parse_do_block(sql: &str) -> PlBlock {
    let stmt = parse_one(sql);
    match stmt {
        Statement::Do(d) => d.node
            .block
            .expect("DO statement should have parsed a PL/pgSQL block"),
        _ => panic!("expected DO statement"),
    }
}

fn parse_err(sql: &str) -> Statement {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    stmts.into_iter().next().unwrap()
}

#[test]
fn test_cursor_with_params_in_function_body() {
    let sql = "CREATE OR REPLACE FUNCTION fnc_test(p_i_id VARCHAR2) RETURN VARCHAR2 IS
        CURSOR c_info(v_step IN VARCHAR2) IS
            SELECT t.dept FROM my_table t WHERE t.id = p_i_id;
    BEGIN
        RETURN '';
    END;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateFunction(f) => {
            let block = f.block.as_ref().expect("function should have a body");
            assert_eq!(block.declarations.len(), 1);
            match &block.declarations[0] {
                PlDeclaration::Cursor(c) => {
                    assert_eq!(c.name, "c_info");
                    assert_eq!(c.arguments.len(), 1);
                    assert_eq!(c.arguments[0].name, "v_step");
                    assert!(matches!(c.arguments[0].mode, PlArgMode::In));
                }
                other => panic!("expected Cursor, got {:?}", other),
            }
        }
        other => panic!("expected CreateFunction, got {:?}", other),
    }
}

#[test]
fn test_cursor_with_params_in_procedure_body() {
    let sql = "CREATE OR REPLACE PROCEDURE prc_test(p_i_id VARCHAR2) IS
        CURSOR c_dept_info(v_step_code IN VARCHAR2) IS
            SELECT t.dept FROM my_table t WHERE t.step_code = v_step_code;
    BEGIN
        NULL;
    END;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            let block = p.block.as_ref().expect("procedure should have a body");
            assert_eq!(block.declarations.len(), 1);
            match &block.declarations[0] {
                PlDeclaration::Cursor(c) => {
                    assert_eq!(c.name, "c_dept_info");
                    assert_eq!(c.arguments.len(), 1);
                    assert_eq!(c.arguments[0].name, "v_step_code");
                    assert!(matches!(c.arguments[0].mode, PlArgMode::In));
                }
                other => panic!("expected Cursor, got {:?}", other),
            }
        }
        other => panic!("expected CreateProcedure, got {:?}", other),
    }
}

#[test]
fn test_package_body_procedure_call_as_statement() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS
               PROCEDURE proc1 IS
               BEGIN
                 pack_log.log('proc', 'desc', '1');
               END proc1;
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            let proc = p.items.iter().find_map(|i| match i {
                PackageItem::Procedure(pr) => Some(pr),
                _ => None,
            }).expect("should have a procedure");
            let block = proc.block.as_ref().expect("procedure should have a block");
            assert_eq!(block.body.len(), 1);
            assert!(
                matches!(&block.body[0], PlStatement::ProcedureCall(_)),
                "expected ProcedureCall, got {:?}",
                block.body[0]
            );
        }
        other => panic!("expected CreatePackageBody, got {:?}", other),
    }
}

#[test]
fn test_procedure_body_qualified_procedure_call() {
    let sql = "CREATE OR REPLACE PROCEDURE prc_test IS
               BEGIN
                 dbms_output.put_line('hello');
               END;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            let block = p.block.as_ref().expect("procedure should have a body");
            assert_eq!(block.body.len(), 1);
            match &block.body[0] {
                PlStatement::ProcedureCall(call) => {
                    assert_eq!(call.name, vec!["dbms_output", "put_line"]);
                }
                other => panic!("expected ProcedureCall, got {:?}", other),
            }
        }
        other => panic!("expected CreateProcedure, got {:?}", other),
    }
}

#[test]
fn test_package_body_with_nested_begin_end_error_recovery() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS
               PROCEDURE proc1 IS
               BEGIN
                 IF 1 = 1 THEN
                   BEGIN
                     NULL;
                   END;
                 END IF;
               END proc1;
               FUNCTION func1 RETURN NUMBER IS
               BEGIN
                 RETURN 1;
               END func1;
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            assert!(
                p.items.iter().any(|i| matches!(i, PackageItem::Procedure(_))),
                "should have a procedure"
            );
            assert!(
                p.items.iter().any(|i| matches!(i, PackageItem::Function(_))),
                "should have a function"
            );
        }
        other => panic!("expected CreatePackageBody, got {:?}", other),
    }
}

#[test]
fn test_package_body_error_recovery_preserves_remaining_items() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS
               PROCEDURE proc1 IS
               BEGIN
                 IF 1 = 1 THEN
                   BEGIN
                     NULL;
                   END;
                 END IF;
               END proc1;
               PROCEDURE proc2 IS
               BEGIN
                 NULL;
               END proc2;
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            let procs: Vec<_> = p.items.iter().filter_map(|i| match i {
                PackageItem::Procedure(pr) => Some(pr),
                _ => None,
            }).collect();
            assert!(
                procs.len() >= 1,
                "should have at least one parsed procedure, got {} items total",
                p.items.len()
            );
        }
        other => panic!("expected CreatePackageBody, got {:?}", other),
    }
}

#[test]
fn test_subscripted_into_target() {
    let sql = r#"CREATE OR REPLACE PROCEDURE test_proc IS
  v_value VARCHAR2_ARRAY;
BEGIN
  SELECT v_value(1) || to_char(COUNT(1)) || ','
    INTO v_value(1)
    FROM tranlog t;
END;"#;
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1, "should parse one statement");
}

#[test]
fn test_subscripted_into_target_simple() {
    let sql = r#"CREATE OR REPLACE PROCEDURE test_proc IS
  v_value VARCHAR2_ARRAY;
BEGIN
  SELECT 1 INTO v_value(1) FROM dual;
END;"#;
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1, "should parse one statement");
}

#[test]
fn test_subscripted_into_multiple_targets() {
    let sql = r#"CREATE OR REPLACE PROCEDURE test_proc IS
  v_arr VARCHAR2_ARRAY;
  v_cnt NUMBER;
BEGIN
  SELECT v_cnt + 1, v_arr(2) INTO v_cnt, v_arr(1) FROM dual;
END;"#;
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1, "should parse one statement");
}

#[test]
fn test_unreserved_keyword_as_variable_name() {
    // RESULT is an unreserved keyword in openGauss/GaussDB and can be used as a variable name
    let sql = r#"CREATE OR REPLACE PROCEDURE test_proc IS
  result INTEGER;
BEGIN
  result := 1;
END;"#;
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            let block = p.block.as_ref().expect("procedure should have a body");
            assert_eq!(block.declarations.len(), 1);
            match &block.declarations[0] {
                PlDeclaration::Variable(v) => {
                    assert_eq!(v.name, "result");
                }
                other => panic!("expected Var, got {:?}", other),
            }
        }
        other => panic!("expected CreateProcedure, got {:?}", other),
    }
}

#[test]
fn test_percent_rowtype_in_inner_block() {
    // %ROWTYPE was previously not handled in parse_pl_data_type, causing
    // ROWTYPE_P keyword to remain unconsumed and cascade parse failures
    let sql = r#"CREATE OR REPLACE PROCEDURE test_proc IS
BEGIN
  DECLARE
    CURSOR c IS SELECT id, name FROM t;
    r c%ROWTYPE;
    w NUMBER;
  BEGIN
    SELECT COUNT(1) INTO w FROM t;
  END;
END;"#;
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            let block = p.block.as_ref().expect("procedure should have a body");
            assert_eq!(block.body.len(), 1);
            match &block.body[0] {
                PlStatement::Block(inner) => {
                    assert_eq!(inner.declarations.len(), 3);
                    match &inner.declarations[1] {
                        PlDeclaration::Variable(v) => {
                            assert_eq!(v.name, "r");
                            assert!(
                                matches!(v.data_type, PlDataType::PercentRowType(ref t) if t == "c"),
                                "expected PercentRowType(\"c\"), got {:?}",
                                v.data_type
                            );
                        }
                        other => panic!("expected Variable, got {:?}", other),
                    }
                }
                other => panic!("expected Block, got {:?}", other),
            }
        }
        other => panic!("expected CreateProcedure, got {:?}", other),
    }
}

#[test]
fn test_fetch_bulk_collect_into() {
    let sql = r#"CREATE OR REPLACE PROCEDURE test_proc IS
  v_cur SYS_REFCURSOR;
  v_list VARCHAR2_ARRAY;
BEGIN
  OPEN v_cur FOR 'SELECT id FROM t';
  FETCH v_cur BULK COLLECT INTO v_list;
  CLOSE v_cur;
END;"#;
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            let block = p.block.as_ref().expect("procedure should have a body");
            match &block.body[1] {
                PlStatement::Fetch(f) => {
                    assert!(f.bulk_collect, "expected bulk_collect = true");
                }
                other => panic!("expected Fetch, got {:?}", other),
            }
        }
        other => panic!("expected CreateProcedure, got {:?}", other),
    }
}

#[test]
fn test_fetch_bulk_collect_into_multiple_targets() {
    let sql = r#"CREATE OR REPLACE PROCEDURE test_proc IS
  v_cur SYS_REFCURSOR;
  v_ids VARCHAR2_ARRAY;
  v_names VARCHAR2_ARRAY;
BEGIN
  OPEN v_cur FOR 'SELECT id, name FROM t';
  FETCH v_cur BULK COLLECT INTO v_ids, v_names;
  CLOSE v_cur;
END;"#;
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1, "should parse one statement");
}

#[test]
fn test_execute_immediate_after_bulk_collect() {
    let sql = r#"CREATE OR REPLACE PROCEDURE test_proc IS
  v_sql VARCHAR2(4000);
BEGIN
  v_sql := 'UPDATE t SET x = 1';
  EXECUTE IMMEDIATE v_sql;
END;"#;
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            let block = p.block.as_ref().expect("procedure should have a body");
            match &block.body[1] {
                PlStatement::Execute(e) => {
                    assert!(e.immediate, "expected immediate = true");
                }
                other => panic!("expected Execute, got {:?}", other),
            }
        }
        other => panic!("expected CreateProcedure, got {:?}", other),
    }
}

#[test]
fn test_case_alias_in_package_spec_cursor() {
    let sql = r#"CREATE OR REPLACE PACKAGE test_pkg IS
  CURSOR c_test IS
    SELECT xwdm,
           CASE WHEN v = 1 THEN 'A' ELSE 'B' END check_type,
           '' account_date
    FROM t1;
END test_pkg;"#;
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1, "should parse one statement");
}

#[test]
fn test_fetch_into_multiple_variables() {
    let block = parse_do_block("DO $$ BEGIN FETCH cur INTO x, y, z; END $$");
    match &block.body[0] {
        PlStatement::Fetch(f) => {
            assert_eq!(f.into.len(), 3, "expected 3 INTO targets");
            assert!(matches!(
                &f.into[0], Expr::ColumnRef(name) if name == &["x".to_string()]
            ));
            assert!(matches!(
                &f.into[1], Expr::ColumnRef(name) if name == &["y".to_string()]
            ));
            assert!(matches!(
                &f.into[2], Expr::ColumnRef(name) if name == &["z".to_string()]
            ));
        }
        _ => panic!("expected Fetch"),
    }
}