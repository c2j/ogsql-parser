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
    stmts.into_iter().next().expect("expected at least one statement")
}

fn parse_do_block(sql: &str) -> PlBlock {
    let stmt = parse_one(sql);
    match stmt {
        Statement::Do(d) => d.node.block.expect("DO statement should have parsed a PL/pgSQL block"),
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
            let proc = p
                .items
                .iter()
                .find_map(|i| match i {
                    PackageItem::Procedure(pr) => Some(pr),
                    _ => None,
                })
                .expect("should have a procedure");
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
            assert!(p.items.iter().any(|i| matches!(i, PackageItem::Procedure(_))), "should have a procedure");
            assert!(p.items.iter().any(|i| matches!(i, PackageItem::Function(_))), "should have a function");
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
            let procs: Vec<_> = p
                .items
                .iter()
                .filter_map(|i| match i {
                    PackageItem::Procedure(pr) => Some(pr),
                    _ => None,
                })
                .collect();
            assert!(procs.len() >= 1, "should have at least one parsed procedure, got {} items total", p.items.len());
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

#[test]
fn test_for_in_select_loop_in_procedure() {
    let sql = r#"CREATE OR REPLACE PROCEDURE test_for_query()
AS $$
DECLARE
    v_rec RECORD;
    v_count INTEGER := 0;
BEGIN
    FOR v_rec IN SELECT id, name, amount FROM t_orders WHERE status = 'PENDING' ORDER BY id LOOP
        v_count := v_count + 1;
        UPDATE t_orders SET processed = true WHERE id = v_rec.id;
        INSERT INTO t_audit(order_id, action) VALUES(v_rec.id, 'PROCESSED');
    END LOOP;
    INSERT INTO t_log(id, msg) VALUES(1, 'done');
END;
$$ LANGUAGE plpgsql"#;

    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1, "should parse one statement, got {}", stmts.len());

    match &stmts[0] {
        Statement::CreateProcedure(proc) => {
            let block = proc.block.as_ref().expect("procedure should have a body (block should not be null)");
            assert!(block.body.len() >= 1, "block body should have statements");

            // The first statement should be a FOR loop
            match &block.body[0] {
                PlStatement::For(for_stmt) => {
                    assert_eq!(for_stmt.node.variable, "v_rec");
                    match &for_stmt.node.kind {
                        PlForKind::Query { query, parsed_query, .. } => {
                            assert!(
                                query.to_uppercase().contains("SELECT"),
                                "query should contain SELECT, got: {:?}",
                                query
                            );
                            assert!(parsed_query.is_some(), "parsed_query should be Some");
                        }
                        other => panic!("expected PlForKind::Query, got {:?}", other),
                    }
                    assert_eq!(for_stmt.node.body.len(), 3, "FOR loop body should have 3 statements");
                }
                other => panic!("expected FOR statement, got {:?}", other),
            }

            // Second statement should be the INSERT after the loop
            assert!(block.body.len() >= 2, "block should have at least 2 statements (FOR + INSERT)");
        }
        other => panic!("expected CreateProcedure, got {:?}", other),
    }
}

#[test]
fn test_pragma_autonomous_transaction_in_block() {
    let sql = r#"CREATE OR REPLACE PROCEDURE test_auto()
AS $$
DECLARE
    PRAGMA AUTONOMOUS_TRANSACTION;
    v_count INTEGER;
BEGIN
    INSERT INTO t_log(id, msg) VALUES(1, 'auto');
END;
$$ LANGUAGE plpgsql"#;
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Statement::CreateProcedure(proc) => {
            let block = proc.block.as_ref().expect("block should not be null");
            assert_eq!(block.declarations.len(), 2, "expected 2 declarations");
            match &block.declarations[0] {
                PlDeclaration::Pragma { name, arguments } => {
                    assert_eq!(name, "AUTONOMOUS_TRANSACTION");
                    assert!(arguments.is_empty());
                }
                other => panic!("expected Pragma declaration, got {:?}", other),
            }
        }
        other => panic!("expected CreateProcedure, got {:?}", other),
    }
}

#[test]
fn test_set_transaction_isolation_level() {
    let block = parse_do_block("DO $$ BEGIN SET TRANSACTION ISOLATION LEVEL READ COMMITTED; END $$");
    match &block.body[0] {
        PlStatement::SetTransaction { isolation_level, read_only, deferrable } => {
            assert!(matches!(isolation_level, Some(PlIsolationLevel::ReadCommitted)));
            assert!(read_only.is_none());
            assert!(deferrable.is_none());
        }
        other => panic!("expected SetTransaction, got {:?}", other),
    }
}

#[test]
fn test_set_transaction_read_only() {
    let block = parse_do_block("DO $$ BEGIN SET TRANSACTION READ ONLY; END $$");
    match &block.body[0] {
        PlStatement::SetTransaction { isolation_level, read_only, deferrable } => {
            assert!(isolation_level.is_none());
            assert_eq!(*read_only, Some(true));
            assert!(deferrable.is_none());
        }
        other => panic!("expected SetTransaction, got {:?}", other),
    }
}

#[test]
fn test_set_transaction_serializable_deferrable() {
    let block =
        parse_do_block("DO $$ BEGIN SET TRANSACTION ISOLATION LEVEL SERIALIZABLE READ WRITE DEFERRABLE; END $$");
    match &block.body[0] {
        PlStatement::SetTransaction { isolation_level, read_only, deferrable } => {
            assert!(matches!(isolation_level, Some(PlIsolationLevel::Serializable)));
            assert_eq!(*read_only, Some(false));
            assert_eq!(*deferrable, Some(true));
        }
        other => panic!("expected SetTransaction, got {:?}", other),
    }
}

#[test]
fn test_pl_call_with_qualified_name() {
    let sql = "CREATE OR REPLACE PROCEDURE prc_test IS
               BEGIN
                 CALL pkg_inventory.check_stock(p_product_id, p_qty);
               END;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            let block = p.block.as_ref().expect("procedure should have a body");
            assert_eq!(block.body.len(), 1);
            match &block.body[0] {
                PlStatement::ProcedureCall(call) => {
                    assert_eq!(call.name, vec!["pkg_inventory", "check_stock"]);
                    assert_eq!(call.arguments.len(), 2);
                }
                other => panic!("expected ProcedureCall, got {:?}", other),
            }
        }
        other => panic!("expected CreateProcedure, got {:?}", other),
    }
}

#[test]
fn test_pl_call_with_simple_name_no_args() {
    let sql = "CREATE OR REPLACE PROCEDURE prc_test IS
               BEGIN
                 CALL my_procedure();
               END;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            let block = p.block.as_ref().expect("procedure should have a body");
            assert_eq!(block.body.len(), 1);
            match &block.body[0] {
                PlStatement::ProcedureCall(call) => {
                    assert_eq!(call.name, vec!["my_procedure"]);
                    assert_eq!(call.arguments.len(), 0);
                }
                other => panic!("expected ProcedureCall, got {:?}", other),
            }
        }
        other => panic!("expected CreateProcedure, got {:?}", other),
    }
}

#[test]
fn test_pl_call_with_literal_arguments() {
    let sql = "CREATE OR REPLACE PROCEDURE prc_test IS
               BEGIN
                 CALL pkg_order.create_order(p_user_id, 0, 1);
               END;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            let block = p.block.as_ref().expect("procedure should have a body");
            assert_eq!(block.body.len(), 1);
            match &block.body[0] {
                PlStatement::ProcedureCall(call) => {
                    assert_eq!(call.name, vec!["pkg_order", "create_order"]);
                    assert_eq!(call.arguments.len(), 3);
                }
                other => panic!("expected ProcedureCall, got {:?}", other),
            }
        }
        other => panic!("expected CreateProcedure, got {:?}", other),
    }
}

#[test]
fn test_pl_call_inside_package_body() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS
               PROCEDURE proc1 IS
               BEGIN
                 CALL pkg_order.create_order(1, 2, 3);
               END proc1;
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            let proc = p
                .items
                .iter()
                .find_map(|i| match i {
                    PackageItem::Procedure(pr) => Some(pr),
                    _ => None,
                })
                .expect("should have a procedure");
            let block = proc.block.as_ref().expect("procedure should have a block");
            assert_eq!(block.body.len(), 1);
            match &block.body[0] {
                PlStatement::ProcedureCall(call) => {
                    assert_eq!(call.name, vec!["pkg_order", "create_order"]);
                    assert_eq!(call.arguments.len(), 3);
                }
                other => panic!("expected ProcedureCall, got {:?}", other),
            }
        }
        other => panic!("expected CreatePackageBody, got {:?}", other),
    }
}

#[test]
fn test_with_insert_in_procedure_body() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS
               PROCEDURE proc1 IS
               BEGIN
                 WITH cte AS (SELECT id FROM t1)
                 INSERT INTO t2 SELECT * FROM cte;
               END proc1;
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            let proc = p
                .items
                .iter()
                .find_map(|i| match i {
                    PackageItem::Procedure(pr) => Some(pr),
                    _ => None,
                })
                .expect("should have a procedure");
            let block = proc.block.as_ref().expect("procedure should have a block");
            assert_eq!(block.body.len(), 1);
            match &block.body[0] {
                PlStatement::SqlStatement { statement, .. } => match statement.as_ref() {
                    crate::ast::Statement::Insert(ins) => {
                        assert!(ins.node.with.is_some(), "INSERT should have WITH clause");
                        assert_eq!(ins.node.table, vec!["t2"]);
                    }
                    other => panic!("expected Insert, got {:?}", other),
                },
                other => panic!("expected SqlStatement, got {:?}", other),
            }
        }
        other => panic!("expected CreatePackageBody, got {:?}", other),
    }
}

#[test]
fn test_with_update_in_procedure_body() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS
               PROCEDURE proc1 IS
               BEGIN
                 WITH cte AS (SELECT id, name FROM t1)
                 UPDATE t2 SET name = cte.name FROM cte WHERE t2.id = cte.id;
               END proc1;
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            let proc = p
                .items
                .iter()
                .find_map(|i| match i {
                    PackageItem::Procedure(pr) => Some(pr),
                    _ => None,
                })
                .expect("should have a procedure");
            let block = proc.block.as_ref().expect("procedure should have a block");
            assert_eq!(block.body.len(), 1);
            match &block.body[0] {
                PlStatement::SqlStatement { statement, .. } => match statement.as_ref() {
                    crate::ast::Statement::Update(upd) => {
                        assert!(upd.node.with.is_some(), "UPDATE should have WITH clause");
                    }
                    other => panic!("expected Update, got {:?}", other),
                },
                other => panic!("expected SqlStatement, got {:?}", other),
            }
        }
        other => panic!("expected CreatePackageBody, got {:?}", other),
    }
}

#[test]
fn test_with_delete_in_procedure_body() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS
               PROCEDURE proc1 IS
               BEGIN
                 WITH cte AS (SELECT id FROM t1 WHERE status = 'inactive')
                 DELETE FROM t2 USING cte WHERE t2.id = cte.id;
               END proc1;
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            let proc = p
                .items
                .iter()
                .find_map(|i| match i {
                    PackageItem::Procedure(pr) => Some(pr),
                    _ => None,
                })
                .expect("should have a procedure");
            let block = proc.block.as_ref().expect("procedure should have a block");
            assert_eq!(block.body.len(), 1);
            match &block.body[0] {
                PlStatement::SqlStatement { statement, .. } => match statement.as_ref() {
                    crate::ast::Statement::Delete(del) => {
                        assert!(del.node.with.is_some(), "DELETE should have WITH clause");
                    }
                    other => panic!("expected Delete, got {:?}", other),
                },
                other => panic!("expected SqlStatement, got {:?}", other),
            }
        }
        other => panic!("expected CreatePackageBody, got {:?}", other),
    }
}

#[test]
fn test_with_select_cte_still_works_in_procedure_body() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS
               PROCEDURE proc1 IS
                 v_result INTEGER;
               BEGIN
                 WITH cte AS (SELECT id FROM t1) SELECT COUNT(*) INTO v_result FROM cte;
               END proc1;
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            let proc = p
                .items
                .iter()
                .find_map(|i| match i {
                    PackageItem::Procedure(pr) => Some(pr),
                    _ => None,
                })
                .expect("should have a procedure");
            let block = proc.block.as_ref().expect("procedure should have a block");
            assert_eq!(block.body.len(), 1);
            match &block.body[0] {
                PlStatement::SqlStatement { statement, .. } => match statement.as_ref() {
                    crate::ast::Statement::Select(sel) => {
                        assert!(sel.node.with.is_some(), "SELECT should have WITH clause");
                    }
                    other => panic!("expected Select, got {:?}", other),
                },
                other => panic!("expected SqlStatement, got {:?}", other),
            }
        }
        other => panic!("expected CreatePackageBody, got {:?}", other),
    }
}

// ========== IS TRUE / IS FALSE expression support (issue #122) ==========

#[test]
fn test_is_true_in_select() {
    let sql = "SELECT * FROM t WHERE active IS TRUE";
    let stmts = parse(sql);
    assert!(stmts.len() == 1);
    match &stmts[0] {
        Statement::Select(sel) => {
            let where_clause = sel.node.where_clause.as_ref().expect("should have WHERE");
            match where_clause {
                Expr::IsBoolean { expr, value, negated } => {
                    assert!(matches!(expr.as_ref(), Expr::ColumnRef(_)));
                    assert!(*value);
                    assert!(!negated);
                }
                other => panic!("expected IsBoolean, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_is_false_in_select() {
    let sql = "SELECT * FROM t WHERE active IS FALSE";
    let stmts = parse(sql);
    assert!(stmts.len() == 1);
    match &stmts[0] {
        Statement::Select(sel) => {
            let where_clause = sel.node.where_clause.as_ref().expect("should have WHERE");
            match where_clause {
                Expr::IsBoolean { expr, value, negated } => {
                    assert!(matches!(expr.as_ref(), Expr::ColumnRef(_)));
                    assert!(!value);
                    assert!(!negated);
                }
                other => panic!("expected IsBoolean, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_is_not_true_in_select() {
    let sql = "SELECT * FROM t WHERE active IS NOT TRUE";
    let stmts = parse(sql);
    assert!(stmts.len() == 1);
    match &stmts[0] {
        Statement::Select(sel) => {
            let where_clause = sel.node.where_clause.as_ref().expect("should have WHERE");
            match where_clause {
                Expr::IsBoolean { expr, value, negated } => {
                    assert!(matches!(expr.as_ref(), Expr::ColumnRef(_)));
                    assert!(*value);
                    assert!(*negated);
                }
                other => panic!("expected IsBoolean, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_is_not_false_in_select() {
    let sql = "SELECT * FROM t WHERE active IS NOT FALSE";
    let stmts = parse(sql);
    assert!(stmts.len() == 1);
    match &stmts[0] {
        Statement::Select(sel) => {
            let where_clause = sel.node.where_clause.as_ref().expect("should have WHERE");
            match where_clause {
                Expr::IsBoolean { expr, value, negated } => {
                    assert!(matches!(expr.as_ref(), Expr::ColumnRef(_)));
                    assert!(!value);
                    assert!(*negated);
                }
                other => panic!("expected IsBoolean, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_is_true_in_plpgsql_if() {
    let sql = "CREATE OR REPLACE FUNCTION test_fn(p_flag IN BOOLEAN) RETURN NUMBER IS
    BEGIN
        IF p_flag IS TRUE THEN RETURN 1; ELSE RETURN 0; END IF;
    END;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateFunction(f) => {
            let block = f.block.as_ref().expect("function should have a body");
            assert_eq!(block.body.len(), 1);
            match &block.body[0] {
                PlStatement::If(if_stmt) => {
                    match &if_stmt.node.condition {
                        Expr::IsBoolean { expr, value, negated } => {
                            assert!(matches!(expr.as_ref(), Expr::PlVariable(_)));
                            assert!(*value);
                            assert!(!negated);
                        }
                        other => panic!("expected IsBoolean condition, got {:?}", other),
                    }
                    assert_eq!(if_stmt.node.then_stmts.len(), 1);
                    assert_eq!(if_stmt.node.else_stmts.len(), 1);
                }
                other => panic!("expected If statement, got {:?}", other),
            }
        }
        other => panic!("expected CreateFunction, got {:?}", other),
    }
}

#[test]
fn test_is_false_in_plpgsql_if() {
    let sql = "CREATE OR REPLACE FUNCTION test_fn(p_flag IN BOOLEAN) RETURN NUMBER IS
    BEGIN
        IF p_flag IS FALSE THEN RETURN 0; END IF;
    END;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateFunction(f) => {
            let block = f.block.as_ref().expect("function should have a body");
            match &block.body[0] {
                PlStatement::If(if_stmt) => match &if_stmt.node.condition {
                    Expr::IsBoolean { value, negated, .. } => {
                        assert!(!value);
                        assert!(!negated);
                    }
                    other => panic!("expected IsBoolean, got {:?}", other),
                },
                other => panic!("expected If, got {:?}", other),
            }
        }
        other => panic!("expected CreateFunction, got {:?}", other),
    }
}

#[test]
fn test_is_not_true_in_plpgsql_if() {
    let sql = "CREATE OR REPLACE FUNCTION test_fn(p_flag IN BOOLEAN) RETURN NUMBER IS
    BEGIN
        IF p_flag IS NOT TRUE THEN RETURN 0; END IF;
    END;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateFunction(f) => {
            let block = f.block.as_ref().expect("function should have a body");
            match &block.body[0] {
                PlStatement::If(if_stmt) => match &if_stmt.node.condition {
                    Expr::IsBoolean { value, negated, .. } => {
                        assert!(*value);
                        assert!(*negated);
                    }
                    other => panic!("expected IsBoolean, got {:?}", other),
                },
                other => panic!("expected If, got {:?}", other),
            }
        }
        other => panic!("expected CreateFunction, got {:?}", other),
    }
}

#[test]
fn test_is_true_formatter_roundtrip() {
    let sql = "SELECT * FROM t WHERE active IS TRUE";
    let stmts = parse(sql);
    let formatter = SqlFormatter::new();
    let output = formatter.format_statement(&stmts[0]);
    assert!(output.contains("IS TRUE"), "formatted output should contain 'IS TRUE': {}", output);
}

#[test]
fn test_is_not_false_formatter_roundtrip() {
    let sql = "SELECT * FROM t WHERE active IS NOT FALSE";
    let stmts = parse(sql);
    let formatter = SqlFormatter::new();
    let output = formatter.format_statement(&stmts[0]);
    assert!(output.contains("IS NOT FALSE"), "formatted output should contain 'IS NOT FALSE': {}", output);
}

#[test]
fn test_package_body_overloaded_functions_with_is_true() {
    let sql = "CREATE OR REPLACE PACKAGE BODY complex_clearing_pkg AS
        FUNCTION calc_fee(p_amount IN NUMERIC) RETURN NUMERIC IS
        BEGIN RETURN p_amount; END;
        FUNCTION calc_fee(p_amount IN NUMERIC, p_vip_level IN INT DEFAULT 1) RETURN NUMERIC IS
            v_rate NUMERIC;
        BEGIN
            v_rate := 0.0015;
            RETURN ROUND(p_amount * v_rate, 2);
        END;
        FUNCTION calc_fee(p_amount IN NUMERIC, p_discount_rate IN NUMERIC, p_apply_ceil IN BOOLEAN) RETURN NUMERIC IS
            v_raw NUMERIC;
        BEGIN
            v_raw := p_amount * p_discount_rate;
            IF p_apply_ceil IS TRUE THEN RETURN CEIL(v_raw); ELSE RETURN v_raw; END IF;
        END;
    END;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            let funcs: Vec<_> = p
                .items
                .iter()
                .filter_map(|item| match item {
                    PackageItem::Function(f) => Some(f.clone()),
                    _ => None,
                })
                .collect();
            assert_eq!(funcs.len(), 3, "should have 3 overloaded calc_fee functions, got {}", funcs.len());
            assert_eq!(funcs[0].name, vec!["calc_fee"]);
            assert_eq!(funcs[0].parameters.len(), 1);
            assert!(funcs[0].block.is_some());
            assert_eq!(funcs[1].name, vec!["calc_fee"]);
            assert_eq!(funcs[1].parameters.len(), 2);
            assert!(funcs[1].block.is_some());
            assert_eq!(funcs[2].name, vec!["calc_fee"]);
            assert_eq!(funcs[2].parameters.len(), 3);
            assert!(funcs[2].block.is_some());

            let block3 = funcs[2].block.as_ref().expect("3rd overload should have body");
            assert_eq!(block3.body.len(), 2, "3rd overload body should have 2 statements");
            match &block3.body[1] {
                PlStatement::If(if_stmt) => match &if_stmt.node.condition {
                    Expr::IsBoolean { value, negated, .. } => {
                        assert!(*value, "condition should be IS TRUE");
                        assert!(!negated);
                    }
                    other => panic!("expected IsBoolean in 3rd overload IF condition, got {:?}", other),
                },
                other => panic!("expected If in 3rd overload body, got {:?}", other),
            }
        }
        other => panic!("expected CreatePackageBody, got {:?}", other),
    }
}

#[test]
fn test_is_true_json_roundtrip() {
    let sql = "SELECT * FROM t WHERE x IS TRUE AND y IS NOT FALSE";
    let stmts = parse(sql);
    let json = serde_json::to_string(&stmts).unwrap();
    let restored: Vec<Statement> = serde_json::from_str(&json).unwrap();
    let formatter = SqlFormatter::new();
    let output = formatter.format_statement(&restored[0]);
    assert!(output.contains("IS TRUE"));
    assert!(output.contains("IS NOT FALSE"));
}
