use crate::ast::plpgsql::*;
use crate::ast::*;
use crate::parser::Parser;
use crate::token::tokenizer::Tokenizer;

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

fn parse_err(sql: &str) -> Statement {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    stmts.into_iter().next().unwrap()
}

/// Helper: parse a DO statement and return its PlBlock (panics if no block parsed)
fn parse_do_block(sql: &str) -> PlBlock {
    let stmt = parse_one(sql);
    match stmt {
        Statement::Do(d) => d
            .block
            .expect("DO statement should have parsed a PL/pgSQL block"),
        _ => panic!("expected DO statement"),
    }
}

// ========== PL/pgSQL Tests ==========

// --- Basic DO Block ---

#[test]
fn test_plpgsql_simple_do_block() {
    let block = parse_do_block("DO $$ BEGIN NULL; END $$");
    assert_eq!(block.body.len(), 1);
    assert!(matches!(&block.body[0], PlStatement::Null));
}

#[test]
fn test_plpgsql_do_with_language() {
    let stmt = parse_one("DO LANGUAGE plpgsql $$ BEGIN NULL; END $$");
    match stmt {
        Statement::Do(d) => {
            assert_eq!(d.language.as_deref(), Some("plpgsql"));
            assert!(d.block.is_some());
        }
        _ => panic!("expected Do"),
    }
}

#[test]
fn test_plpgsql_do_multiple_statements() {
    let block = parse_do_block("DO $$ BEGIN NULL; NULL; NULL; END $$");
    assert_eq!(block.body.len(), 3);
    for stmt in &block.body {
        assert!(matches!(stmt, PlStatement::Null));
    }
}

// --- Declarations ---

#[test]
fn test_plpgsql_variable_declarations() {
    let block = parse_do_block("DO $$ DECLARE x INTEGER; BEGIN NULL; END $$");
    assert_eq!(block.declarations.len(), 1);
    match &block.declarations[0] {
        PlDeclaration::Variable(v) => {
            assert_eq!(v.name, "x");
            assert!(matches!(&v.data_type, PlDataType::TypeName(t) if t == "integer"));
            assert!(!v.constant);
            assert!(!v.not_null);
        }
        _ => panic!("expected Variable declaration"),
    }
}

#[test]
fn test_plpgsql_variable_with_default() {
    let block = parse_do_block("DO $$ DECLARE x INTEGER := 42; BEGIN NULL; END $$");
    assert_eq!(block.declarations.len(), 1);
    match &block.declarations[0] {
        PlDeclaration::Variable(v) => {
            assert_eq!(v.name, "x");
            match &v.default {
                Some(Expr::Literal(Literal::Integer(42))) => {}
                other => panic!("expected Integer(42), got: {:?}", other),
            }
        }
        _ => panic!("expected Variable declaration"),
    }
}

#[test]
fn test_plpgsql_multiple_declarations() {
    let block = parse_do_block("DO $$ DECLARE x INTEGER; y TEXT := 'hello'; BEGIN NULL; END $$");
    assert_eq!(block.declarations.len(), 2);
    assert!(matches!(&block.declarations[0], PlDeclaration::Variable(_)));
    assert!(matches!(&block.declarations[1], PlDeclaration::Variable(_)));
}

// --- Assignment ---

#[test]
fn test_plpgsql_assignment() {
    let block = parse_do_block("DO $$ BEGIN x := 1; END $$");
    assert_eq!(block.body.len(), 1);
    match &block.body[0] {
        PlStatement::Assignment { target, expression } => {
            assert_eq!(target, "x");
            assert!(matches!(expression, Expr::Literal(Literal::Integer(1))));
        }
        _ => panic!("expected Assignment"),
    }
}

#[test]
fn test_plpgsql_assignment_complex() {
    let block = parse_do_block("DO $$ BEGIN sname := 'IF.' || sysname; END $$");
    assert_eq!(block.body.len(), 1);
    assert!(matches!(&block.body[0], PlStatement::Assignment { .. }));
}

// --- IF/ELSIF/ELSE ---

#[test]
fn test_plpgsql_simple_if() {
    let block = parse_do_block("DO $$ BEGIN IF TRUE THEN NULL; END IF; END $$");
    assert_eq!(block.body.len(), 1);
    match &block.body[0] {
        PlStatement::If(if_stmt) => {
            assert!(matches!(
                &if_stmt.condition,
                Expr::Literal(Literal::Boolean(true))
            ));
            assert_eq!(if_stmt.then_stmts.len(), 1);
            assert!(if_stmt.elsifs.is_empty());
            assert!(if_stmt.else_stmts.is_empty());
        }
        _ => panic!("expected If"),
    }
}

#[test]
fn test_plpgsql_if_elsif_else() {
    let block = parse_do_block(
        "DO $$ BEGIN IF TRUE THEN NULL; ELSIF FALSE THEN NULL; ELSE NULL; END IF; END $$",
    );
    assert_eq!(block.body.len(), 1);
    match &block.body[0] {
        PlStatement::If(if_stmt) => {
            assert_eq!(if_stmt.elsifs.len(), 1);
            assert_eq!(if_stmt.else_stmts.len(), 1);
            assert!(matches!(
                &if_stmt.elsifs[0].condition,
                Expr::Literal(Literal::Boolean(false))
            ));
        }
        _ => panic!("expected If"),
    }
}

#[test]
fn test_plpgsql_nested_if() {
    let block =
        parse_do_block("DO $$ BEGIN IF TRUE THEN IF FALSE THEN NULL; END IF; END IF; END $$");
    assert_eq!(block.body.len(), 1);
    match &block.body[0] {
        PlStatement::If(if_stmt) => {
            assert_eq!(if_stmt.then_stmts.len(), 1);
            assert!(matches!(&if_stmt.then_stmts[0], PlStatement::If(_)));
        }
        _ => panic!("expected If"),
    }
}

// --- CASE ---

#[test]
fn test_plpgsql_searched_case() {
    let block = parse_do_block("DO $$ BEGIN CASE WHEN TRUE THEN NULL; END CASE; END $$");
    assert_eq!(block.body.len(), 1);
    match &block.body[0] {
        PlStatement::Case(case_stmt) => {
            assert!(case_stmt.expression.is_none()); // searched CASE
            assert_eq!(case_stmt.whens.len(), 1);
        }
        _ => panic!("expected Case"),
    }
}

#[test]
fn test_plpgsql_plain_case() {
    let block = parse_do_block("DO $$ BEGIN CASE x WHEN 1 THEN NULL; END CASE; END $$");
    assert_eq!(block.body.len(), 1);
    match &block.body[0] {
        PlStatement::Case(case_stmt) => {
            assert!(case_stmt.expression.is_some());
            assert_eq!(case_stmt.whens.len(), 1);
            assert!(matches!(
                &case_stmt.whens[0].condition,
                Expr::Literal(Literal::Integer(1))
            ));
        }
        _ => panic!("expected Case"),
    }
}

// --- LOOP ---

#[test]
fn test_plpgsql_loop_with_exit() {
    let block = parse_do_block("DO $$ BEGIN LOOP EXIT; END LOOP; END $$");
    assert_eq!(block.body.len(), 1);
    match &block.body[0] {
        PlStatement::Loop(loop_stmt) => {
            assert_eq!(loop_stmt.body.len(), 1);
            assert!(matches!(
                &loop_stmt.body[0],
                PlStatement::Exit {
                    label: None,
                    condition: None
                }
            ));
        }
        _ => panic!("expected Loop"),
    }
}

#[test]
fn test_plpgsql_labeled_loop() {
    let block = parse_do_block("DO $$ BEGIN <<myloop>> LOOP EXIT myloop; END LOOP myloop; END $$");
    match &block.body[0] {
        PlStatement::Loop(loop_stmt) => {
            assert_eq!(loop_stmt.label.as_deref(), Some("myloop"));
            assert_eq!(loop_stmt.body.len(), 1);
            assert!(matches!(&loop_stmt.body[0], PlStatement::Exit { .. }));
        }
        _ => panic!("expected Loop"),
    }
}

// --- WHILE ---

#[test]
fn test_plpgsql_while_loop() {
    let block = parse_do_block("DO $$ BEGIN WHILE TRUE LOOP EXIT; END LOOP; END $$");
    assert_eq!(block.body.len(), 1);
    match &block.body[0] {
        PlStatement::While(w) => {
            assert!(matches!(
                &w.condition,
                Expr::Literal(Literal::Boolean(true))
            ));
            assert_eq!(w.body.len(), 1);
        }
        _ => panic!("expected While"),
    }
}

#[test]
fn test_plpgsql_while_labeled() {
    let block = parse_do_block("DO $$ BEGIN <<wl>> WHILE TRUE LOOP EXIT; END LOOP wl; END $$");
    match &block.body[0] {
        PlStatement::While(w) => {
            assert_eq!(w.label.as_deref(), Some("wl"));
            assert!(matches!(
                &w.condition,
                Expr::Literal(Literal::Boolean(true))
            ));
            assert_eq!(w.body.len(), 1);
        }
        _ => panic!("expected While"),
    }
}

// --- FOR ---

#[test]
fn test_plpgsql_for_range() {
    let block = parse_do_block("DO $$ BEGIN FOR i IN 1..10 LOOP EXIT; END LOOP; END $$");
    assert_eq!(block.body.len(), 1);
    match &block.body[0] {
        PlStatement::For(f) => {
            assert_eq!(f.variable, "i");
            match &f.kind {
                PlForKind::Range {
                    low,
                    high,
                    step: None,
                    reverse: false,
                } => {
                    assert!(matches!(low, Expr::Literal(Literal::Integer(1))));
                    assert!(matches!(high, Expr::Literal(Literal::Integer(10))));
                }
                _ => panic!("expected Range kind"),
            }
        }
        _ => panic!("expected For"),
    }
}

#[test]
fn test_plpgsql_for_range_reverse() {
    let block = parse_do_block("DO $$ BEGIN FOR i IN REVERSE 1..10 LOOP EXIT; END LOOP; END $$");
    assert_eq!(block.body.len(), 1);
    match &block.body[0] {
        PlStatement::For(f) => match &f.kind {
            PlForKind::Range { reverse: true, .. } => {}
            _ => panic!("expected reverse Range"),
        },
        _ => panic!("expected For"),
    }
}

#[test]
fn test_plpgsql_for_query() {
    let block = parse_do_block("DO $$ BEGIN FOR rec IN SELECT 1 LOOP EXIT; END LOOP; END $$");
    assert_eq!(block.body.len(), 1);
    match &block.body[0] {
        PlStatement::For(f) => {
            assert_eq!(f.variable, "rec");
            match &f.kind {
                PlForKind::Query { query, .. } => assert_eq!(query, "select 1"),
                _ => panic!("expected Query kind"),
            }
        }
        _ => panic!("expected For"),
    }
}

// --- EXIT/CONTINUE ---

#[test]
fn test_plpgsql_exit() {
    let block = parse_do_block("DO $$ BEGIN EXIT; END $$");
    assert!(matches!(
        &block.body[0],
        PlStatement::Exit {
            label: None,
            condition: None
        }
    ));
}

#[test]
fn test_plpgsql_exit_when() {
    let block = parse_do_block("DO $$ BEGIN EXIT WHEN TRUE; END $$");
    match &block.body[0] {
        PlStatement::Exit {
            label: None,
            condition: Some(c),
        } => assert!(matches!(c, Expr::Literal(Literal::Boolean(true)))),
        _ => panic!("expected Exit with condition"),
    }
}

#[test]
fn test_plpgsql_continue_when() {
    let block = parse_do_block("DO $$ BEGIN CONTINUE WHEN FALSE; END $$");
    match &block.body[0] {
        PlStatement::Continue {
            label: None,
            condition: Some(c),
        } => assert!(matches!(c, Expr::Literal(Literal::Boolean(false)))),
        _ => panic!("expected Continue with condition"),
    }
}

// --- RETURN ---

#[test]
fn test_plpgsql_return() {
    let block = parse_do_block("DO $$ BEGIN RETURN; END $$");
    assert!(matches!(
        &block.body[0],
        PlStatement::Return { expression: None }
    ));
}

#[test]
fn test_plpgsql_return_expr() {
    let block = parse_do_block("DO $$ BEGIN RETURN 42; END $$");
    match &block.body[0] {
        PlStatement::Return {
            expression: Some(e),
        } => assert!(matches!(e, Expr::Literal(Literal::Integer(42)))),
        _ => panic!("expected Return with expression"),
    }
}

// --- RAISE ---

#[test]
fn test_plpgsql_raise_notice() {
    let block = parse_do_block("DO $$ BEGIN RAISE NOTICE 'hello'; END $$");
    match &block.body[0] {
        PlStatement::Raise(r) => {
            assert!(matches!(r.level, Some(RaiseLevel::Notice)));
            assert_eq!(r.message.as_deref(), Some("'hello'"));
        }
        _ => panic!("expected Raise"),
    }
}

#[test]
fn test_plpgsql_raise_exception() {
    let block = parse_do_block("DO $$ BEGIN RAISE EXCEPTION 'error %', 'msg'; END $$");
    match &block.body[0] {
        PlStatement::Raise(r) => {
            assert!(matches!(r.level, Some(RaiseLevel::Exception)));
            assert!(r.message.is_some());
        }
        _ => panic!("expected Raise"),
    }
}

#[test]
fn test_plpgsql_reraise() {
    let block = parse_do_block("DO $$ BEGIN EXCEPTION WHEN OTHERS THEN RAISE; END; END $$");
    assert!(block.body.is_empty());
    let exc = block.exception_block.expect("expected exception block");
    assert_eq!(exc.handlers.len(), 1);
    match &exc.handlers[0].statements[0] {
        PlStatement::Raise(r) => {
            assert!(r.level.is_none());
            assert!(r.message.is_none());
        }
        _ => panic!("expected re-RAISE"),
    }
}

// --- EXECUTE ---

#[test]
fn test_plpgsql_execute() {
    let block = parse_do_block("DO $$ BEGIN EXECUTE 'SELECT 1'; END $$");
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(
                matches!(&e.string_expr, Expr::Literal(Literal::String(s)) if s.contains("SELECT 1"))
            );
            assert!(!e.immediate);
            assert!(e.into_targets.is_empty());
            assert!(e.using_args.is_empty());
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_immediate_simple() {
    let block = parse_do_block(
        "DO $$ BEGIN EXECUTE IMMEDIATE 'INSERT INTO t VALUES(:1, :2)' USING a, b; END $$",
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert!(e.into_targets.is_empty());
            assert_eq!(e.using_args.len(), 2);
            assert!(matches!(e.using_args[0].mode, PlUsingMode::In));
            assert!(matches!(e.using_args[1].mode, PlUsingMode::In));
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_immediate_into() {
    let block = parse_do_block(
        "DO $$ BEGIN EXECUTE IMMEDIATE 'SELECT count(*) FROM t' INTO v_count; END $$",
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert_eq!(e.into_targets.len(), 1);
            assert!(e.using_args.is_empty());
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_immediate_into_using() {
    let block = parse_do_block(
        "DO $$ BEGIN EXECUTE IMMEDIATE 'SELECT name FROM t WHERE id=:1' INTO v_name USING IN v_id; END $$"
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert_eq!(e.into_targets.len(), 1);
            assert_eq!(e.using_args.len(), 1);
            assert!(matches!(e.using_args[0].mode, PlUsingMode::In));
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_immediate_using_in_out() {
    let block =
        parse_do_block("DO $$ BEGIN EXECUTE IMMEDIATE stmt USING OUT v1, IN v2, IN OUT v3; END $$");
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert!(e.into_targets.is_empty());
            assert_eq!(e.using_args.len(), 3);
            assert!(matches!(e.using_args[0].mode, PlUsingMode::Out));
            assert!(matches!(e.using_args[1].mode, PlUsingMode::In));
            assert!(matches!(e.using_args[2].mode, PlUsingMode::InOut));
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_immediate_multi_into() {
    let block = parse_do_block(
        "DO $$ BEGIN EXECUTE IMMEDIATE 'SELECT name, salary FROM t WHERE id=:1' INTO v_name, v_salary USING v_id; END $$"
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert_eq!(e.into_targets.len(), 2);
            assert_eq!(e.using_args.len(), 1);
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_concat_expr() {
    let block = parse_do_block(
        "DO $$ BEGIN EXECUTE IMMEDIATE 'ALTER TABLE ' || tab_name || ' ADD COLUMN c INT'; END $$",
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert!(matches!(e.string_expr, Expr::BinaryOp { .. }));
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_for_in_execute() {
    let block = parse_do_block(
        "DO $$ BEGIN FOR rec IN EXECUTE 'SELECT * FROM ' || tab_name LOOP NULL; END LOOP; END $$",
    );
    match &block.body[0] {
        PlStatement::For(f) => match &f.kind {
            PlForKind::Query {
                query, using_args, ..
            } => {
                assert!(query.to_lowercase().contains("execute"));
                assert!(using_args.is_empty());
            }
            _ => panic!("expected Query kind"),
        },
        _ => panic!("expected For"),
    }
}

#[test]
fn test_plpgsql_for_in_execute_using() {
    let block = parse_do_block(
        "DO $$ BEGIN FOR rec IN EXECUTE 'SELECT * FROM t WHERE id=:1' USING v_id LOOP NULL; END LOOP; END $$"
    );
    match &block.body[0] {
        PlStatement::For(f) => match &f.kind {
            PlForKind::Query {
                query, using_args, ..
            } => {
                assert!(query.to_lowercase().contains("using"));
                assert!(using_args.is_empty());
            }
            _ => panic!("expected Query kind"),
        },
        _ => panic!("expected For"),
    }
}

// --- PERFORM ---

#[test]
fn test_plpgsql_perform() {
    let block = parse_do_block("DO $$ BEGIN PERFORM 'SELECT 1'; END $$");
    assert!(matches!(&block.body[0], PlStatement::Perform { .. }));
}

// --- Cursor Operations ---

#[test]
fn test_plpgsql_open_cursor() {
    let block = parse_do_block("DO $$ BEGIN OPEN cur; END $$");
    match &block.body[0] {
        PlStatement::Open(o) => {
            assert_eq!(o.cursor, "cur");
            assert!(matches!(&o.kind, PlOpenKind::Simple { arguments }));
        }
        _ => panic!("expected Open"),
    }
}

#[test]
fn test_plpgsql_fetch_cursor() {
    let block = parse_do_block("DO $$ BEGIN FETCH cur INTO x; END $$");
    match &block.body[0] {
        PlStatement::Fetch(f) => {
            assert_eq!(f.cursor, "cur");
            assert!(matches!(&f.into, Expr::ColumnRef(name) if name == &["x".to_string()]));
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_plpgsql_close_cursor() {
    let block = parse_do_block("DO $$ BEGIN CLOSE cur; END $$");
    match &block.body[0] {
        PlStatement::Close { cursor } => assert_eq!(cursor, "cur"),
        _ => panic!("expected Close"),
    }
}

// --- GET DIAGNOSTICS ---

#[test]
fn test_plpgsql_get_diagnostics() {
    let block = parse_do_block("DO $$ BEGIN GET DIAGNOSTICS x = ROW_COUNT; END $$");
    match &block.body[0] {
        PlStatement::GetDiagnostics(g) => {
            assert!(!g.stacked);
            assert_eq!(g.items.len(), 1);
            assert_eq!(g.items[0].target, "x");
            assert!(matches!(
                g.items[0].item,
                plpgsql::GetDiagItemKind::RowCount
            ));
        }
        _ => panic!("expected GetDiagnostics"),
    }
}

#[test]
fn test_plpgsql_get_stacked_diagnostics() {
    let block = parse_do_block("DO $$ BEGIN GET STACKED DIAGNOSTICS x = RETURNED_SQLSTATE; END $$");
    match &block.body[0] {
        PlStatement::GetDiagnostics(g) => {
            assert!(g.stacked);
            assert_eq!(g.items.len(), 1);
            assert!(matches!(
                g.items[0].item,
                plpgsql::GetDiagItemKind::ReturnedSqlstate
            ));
        }
        _ => panic!("expected GetDiagnostics"),
    }
}

// --- Transaction in Block ---

#[test]
fn test_plpgsql_commit() {
    let block = parse_do_block("DO $$ BEGIN COMMIT; END $$");
    assert!(matches!(&block.body[0], PlStatement::Commit));
}

#[test]
fn test_plpgsql_rollback() {
    let block = parse_do_block("DO $$ BEGIN ROLLBACK; END $$");
    match &block.body[0] {
        PlStatement::Rollback { to_savepoint: None } => {}
        _ => panic!("expected Rollback"),
    }
}

#[test]
fn test_plpgsql_rollback_to_savepoint() {
    let block = parse_do_block("DO $$ BEGIN ROLLBACK TO sp; END $$");
    match &block.body[0] {
        PlStatement::Rollback {
            to_savepoint: Some(sp),
        } => assert_eq!(sp, "sp"),
        _ => panic!("expected Rollback TO"),
    }
}

#[test]
fn test_plpgsql_savepoint() {
    let block = parse_do_block("DO $$ BEGIN SAVEPOINT sp; END $$");
    match &block.body[0] {
        PlStatement::Savepoint { name } => assert_eq!(name, "sp"),
        _ => panic!("expected Savepoint"),
    }
}

// --- GOTO ---

#[test]
fn test_plpgsql_goto() {
    let block = parse_do_block("DO $$ BEGIN GOTO lbl; END $$");
    match &block.body[0] {
        PlStatement::Goto { label } => assert_eq!(label, "lbl"),
        _ => panic!("expected Goto"),
    }
}

// --- Nested Blocks ---

#[test]
fn test_plpgsql_nested_block() {
    let block = parse_do_block("DO $$ BEGIN BEGIN NULL; END; END $$");
    match &block.body[0] {
        PlStatement::Block(inner) => {
            assert_eq!(inner.body.len(), 1);
            assert!(matches!(&inner.body[0], PlStatement::Null));
        }
        _ => panic!("expected nested Block"),
    }
}

// --- Exception Handling ---

#[test]
fn test_plpgsql_exception_handler() {
    let block = parse_do_block("DO $$ BEGIN EXCEPTION WHEN OTHERS THEN NULL; END; END $$");
    assert!(block.body.is_empty());
    let exc = block
        .exception_block
        .as_ref()
        .expect("expected exception block");
    assert_eq!(exc.handlers.len(), 1);
    assert_eq!(exc.handlers[0].conditions, vec!["OTHERS".to_string()]);
    assert_eq!(exc.handlers[0].statements.len(), 1);
}

#[test]
fn test_plpgsql_multiple_exception_handlers() {
    let block = parse_do_block(
        "DO $$ BEGIN EXCEPTION WHEN no_data_found THEN NULL; WHEN OTHERS THEN NULL; END; END $$",
    );
    assert!(block.body.is_empty());
    let exc = block.exception_block.as_ref().unwrap();
    assert_eq!(exc.handlers.len(), 2);
    assert_eq!(exc.handlers[0].conditions[0], "no_data_found");
    assert_eq!(exc.handlers[1].conditions[0], "OTHERS");
}

// --- Real-world Examples ---

#[test]
fn test_plpgsql_realworld_if_with_assignment() {
    // Inspired by openGauss trigger function patterns
    let block = parse_do_block("DO $$ BEGIN IF TRUE THEN x := 1; END IF; END $$");
    match &block.body[0] {
        PlStatement::If(if_stmt) => {
            assert_eq!(if_stmt.then_stmts.len(), 1);
            assert!(matches!(
                &if_stmt.then_stmts[0],
                PlStatement::Assignment { .. }
            ));
        }
        _ => panic!("expected If"),
    }
}

#[test]
fn test_plpgsql_realworld_for_loop_with_body() {
    // Inspired by: FOR i IN oldnslots + 1 .. newnslots LOOP ... END LOOP
    let block = parse_do_block("DO $$ BEGIN FOR i IN 1..5 LOOP x := x + 1; END LOOP; END $$");
    match &block.body[0] {
        PlStatement::For(f) => {
            assert_eq!(f.variable, "i");
            match &f.kind {
                PlForKind::Range { low, high, .. } => {
                    assert!(matches!(low, Expr::Literal(Literal::Integer(_))));
                    assert!(matches!(high, Expr::Literal(Literal::Integer(_))));
                }
                _ => panic!("expected Range"),
            }
            assert_eq!(f.body.len(), 1);
            assert!(matches!(&f.body[0], PlStatement::Assignment { .. }));
        }
        _ => panic!("expected For"),
    }
}

// --- Combined: Multiple statement types in one block ---

#[test]
fn test_plpgsql_combined_statements() {
    let block = parse_do_block("DO $$ BEGIN NULL; x := 1; RETURN; END $$");
    assert_eq!(block.body.len(), 3);
    assert!(matches!(&block.body[0], PlStatement::Null));
    assert!(matches!(&block.body[1], PlStatement::Assignment { .. }));
    assert!(matches!(
        &block.body[2],
        PlStatement::Return { expression: None }
    ));
}

// --- Anonymous Block Dispatch ---

#[test]
fn test_anonymous_block_via_do() {
    let stmt = parse_one("DO $$ BEGIN NULL; END $$");
    assert!(matches!(stmt, Statement::Do(_)));
}

#[test]
fn test_anonymous_block_via_begin_dollar() {
    let stmt = parse_one("BEGIN $$ BEGIN NULL; END $$");
    assert!(matches!(stmt, Statement::AnonyBlock(_)));
}

#[test]
fn test_begin_transaction_still_works() {
    let stmt = parse_one("BEGIN");
    assert!(matches!(stmt, Statement::Transaction(_)));
}

#[test]
fn test_begin_transaction_with_semicolon() {
    let stmt = parse_one("BEGIN;");
    assert!(matches!(stmt, Statement::Transaction(_)));
}

#[test]
fn test_begin_transaction_work() {
    let stmt = parse_one("BEGIN WORK");
    assert!(matches!(stmt, Statement::Transaction(_)));
}

#[test]
fn test_begin_transaction_isolation_level() {
    let stmt = parse_one("BEGIN ISOLATION LEVEL READ COMMITTED");
    assert!(matches!(stmt, Statement::Transaction(_)));
}

#[test]
fn test_begin_transaction_read_only() {
    let stmt = parse_one("BEGIN READ ONLY");
    assert!(matches!(stmt, Statement::Transaction(_)));
}

#[test]
fn test_begin_anon_block_with_select() {
    let stmt = parse_one("BEGIN SELECT 1; END");
    match stmt {
        Statement::AnonyBlock(b) => {
            assert_eq!(b.block.body.len(), 1);
        }
        _ => panic!("expected AnonyBlock, got {:?}", stmt),
    }
}

#[test]
fn test_begin_anon_block_with_update() {
    let stmt = parse_one("BEGIN UPDATE t SET x = 1; END");
    match stmt {
        Statement::AnonyBlock(b) => {
            assert_eq!(b.block.body.len(), 1);
        }
        _ => panic!("expected AnonyBlock, got {:?}", stmt),
    }
}

#[test]
fn test_begin_anon_block_with_if() {
    let stmt = parse_one("BEGIN IF true THEN NULL; END IF; END");
    match stmt {
        Statement::AnonyBlock(b) => {
            assert_eq!(b.block.body.len(), 1);
            match &b.block.body[0] {
                PlStatement::If(_) => {}
                other => panic!("expected If, got {:?}", other),
            }
        }
        _ => panic!("expected AnonyBlock, got {:?}", stmt),
    }
}

#[test]
fn test_begin_anon_block_with_insert_and_exception() {
    let sql = "BEGIN INSERT INTO t VALUES (1); EXCEPTION WHEN OTHERS THEN NULL; END";
    let stmt = parse_one(sql);
    match stmt {
        Statement::AnonyBlock(b) => {
            assert_eq!(b.block.body.len(), 1);
            assert!(b.block.exception_block.is_some());
        }
        _ => panic!("expected AnonyBlock, got {:?}", stmt),
    }
}

#[test]
fn test_begin_anon_block_with_multiple_statements() {
    let sql = "BEGIN SELECT 1; SELECT 2; COMMIT; END";
    let stmt = parse_one(sql);
    match stmt {
        Statement::AnonyBlock(b) => {
            assert_eq!(b.block.body.len(), 3);
            assert!(matches!(b.block.body[2], PlStatement::Commit));
        }
        _ => panic!("expected AnonyBlock, got {:?}", stmt),
    }
}

// ========== CREATE TYPE Tests ==========

#[test]
fn test_create_shell_type() {
    let stmt = parse_one("CREATE TYPE complex");
    match stmt {
        Statement::CreateType(t) => {
            assert_eq!(t.name, vec!["complex"]);
            assert!(matches!(t.type_kind, TypeKind::Shell));
        }
        _ => panic!("expected CreateType, got {:?}", stmt),
    }
}

#[test]
fn test_create_composite_type() {
    let stmt = parse_one("CREATE TYPE compfoo AS (f1 int, f2 text)");
    match stmt {
        Statement::CreateType(t) => {
            assert_eq!(t.name, vec!["compfoo"]);
            match &t.type_kind {
                TypeKind::Composite { attributes } => {
                    assert_eq!(attributes.len(), 2);
                    assert_eq!(attributes[0].name, "f1");
                    assert_eq!(attributes[1].name, "f2");
                }
                other => panic!("expected Composite, got {:?}", other),
            }
        }
        _ => panic!("expected CreateType, got {:?}", stmt),
    }
}

#[test]
fn test_create_enum_type() {
    let stmt = parse_one("CREATE TYPE bug_status AS ENUM ('new', 'open', 'closed')");
    match stmt {
        Statement::CreateType(t) => {
            assert_eq!(t.name, vec!["bug_status"]);
            match &t.type_kind {
                TypeKind::Enum { labels } => {
                    assert_eq!(labels.len(), 3);
                    assert_eq!(labels[0], "new");
                    assert_eq!(labels[1], "open");
                    assert_eq!(labels[2], "closed");
                }
                other => panic!("expected Enum, got {:?}", other),
            }
        }
        _ => panic!("expected CreateType, got {:?}", stmt),
    }
}

#[test]
fn test_create_base_type() {
    let stmt = parse_one("CREATE TYPE box (INPUT = box_in, OUTPUT = box_out)");
    match stmt {
        Statement::CreateType(t) => {
            assert_eq!(t.name, vec!["box"]);
            assert!(matches!(t.type_kind, TypeKind::Base { .. }));
        }
        _ => panic!("expected CreateType, got {:?}", stmt),
    }
}

#[test]
fn test_create_role_basic() {
    let stmt = parse_one("CREATE ROLE admin");
    match stmt {
        Statement::CreateRole(r) => {
            assert_eq!(r.name, "admin");
            assert!(r.options.is_empty());
        }
        _ => panic!("expected CreateRole, got {:?}", stmt),
    }
}

#[test]
fn test_create_role_with_options() {
    let stmt = parse_one("CREATE ROLE admin WITH SUPERUSER CREATEDB LOGIN PASSWORD 'secret'");
    match stmt {
        Statement::CreateRole(r) => {
            assert_eq!(r.name, "admin");
            assert!(r
                .options
                .iter()
                .any(|o| matches!(o, RoleOption::Superuser(true))));
            assert!(r
                .options
                .iter()
                .any(|o| matches!(o, RoleOption::CreateDb(true))));
            assert!(r
                .options
                .iter()
                .any(|o| matches!(o, RoleOption::Login(true))));
        }
        _ => panic!("expected CreateRole, got {:?}", stmt),
    }
}

#[test]
fn test_create_user_with_password() {
    let stmt = parse_one("CREATE USER davide WITH PASSWORD 'jw8s0F4'");
    match stmt {
        Statement::CreateUser(u) => {
            assert_eq!(u.name, "davide");
            assert!(u
                .options
                .iter()
                .any(|o| matches!(o, RoleOption::UnencryptedPassword(_))));
        }
        _ => panic!("expected CreateUser, got {:?}", stmt),
    }
}

#[test]
fn test_create_group_basic() {
    let stmt = parse_one("CREATE GROUP staff");
    match stmt {
        Statement::CreateGroup(g) => {
            assert_eq!(g.name, "staff");
            assert!(g.options.is_empty());
        }
        _ => panic!("expected CreateGroup, got {:?}", stmt),
    }
}

#[test]
fn test_grant_role() {
    let stmt = parse_one("GRANT admin TO davide");
    match stmt {
        Statement::GrantRole(g) => {
            assert_eq!(g.roles, vec!["admin"]);
            assert_eq!(g.grantees, vec!["davide"]);
            assert!(!g.with_admin_option);
        }
        _ => panic!("expected GrantRole, got {:?}", stmt),
    }
}

#[test]
fn test_grant_role_with_admin() {
    let stmt = parse_one("GRANT admin TO davide WITH ADMIN OPTION");
    match stmt {
        Statement::GrantRole(g) => {
            assert_eq!(g.roles, vec!["admin"]);
            assert!(g.with_admin_option);
        }
        _ => panic!("expected GrantRole, got {:?}", stmt),
    }
}

#[test]
fn test_revoke_role() {
    let stmt = parse_one("REVOKE admin FROM davide");
    match stmt {
        Statement::RevokeRole(r) => {
            assert_eq!(r.roles, vec!["admin"]);
            assert_eq!(r.grantees, vec!["davide"]);
            assert!(!r.cascade);
        }
        _ => panic!("expected RevokeRole, got {:?}", stmt),
    }
}

#[test]
fn test_revoke_role_cascade() {
    let stmt = parse_one("REVOKE admin FROM davide CASCADE");
    match stmt {
        Statement::RevokeRole(r) => {
            assert!(r.cascade);
        }
        _ => panic!("expected RevokeRole, got {:?}", stmt),
    }
}

#[test]
fn test_grant_privilege_still_works() {
    let stmt = parse_one("GRANT SELECT ON users TO admin");
    match stmt {
        Statement::Grant(g) => {
            assert!(g.privileges.iter().any(|p| matches!(p, Privilege::Select)));
        }
        _ => panic!("expected Grant, got {:?}", stmt),
    }
}

#[test]
fn test_alter_index_rename() {
    let stmt = parse_one("ALTER INDEX distributors RENAME TO suppliers");
    match stmt {
        Statement::AlterIndex(a) => {
            assert_eq!(a.name, vec!["distributors"]);
            match &a.action {
                AlterIndexAction::RenameTo(new_name) => assert_eq!(new_name, "suppliers"),
                other => panic!("expected RenameTo, got {:?}", other),
            }
        }
        _ => panic!("expected AlterIndex, got {:?}", stmt),
    }
}

#[test]
fn test_alter_index_set() {
    let stmt = parse_one("ALTER INDEX idx SET (fillfactor = 75)");
    match stmt {
        Statement::AlterIndex(a) => {
            assert!(matches!(a.action, AlterIndexAction::Set(_)));
        }
        _ => panic!("expected AlterIndex, got {:?}", stmt),
    }
}

#[test]
fn test_alter_index_set_tablespace() {
    let stmt = parse_one("ALTER INDEX idx SET TABLESPACE fast_tablespace");
    match stmt {
        Statement::AlterIndex(a) => {
            assert!(matches!(a.action, AlterIndexAction::SetTablespace(_)));
        }
        _ => panic!("expected AlterIndex, got {:?}", stmt),
    }
}

// ========== ALTER TYPE tests ==========

#[test]
fn test_alter_type_add_attribute() {
    let stmt = parse_one("ALTER TYPE compfoo ADD ATTRIBUTE f3 text");
    match stmt {
        Statement::AlterCompositeType(a) => {
            assert_eq!(a.name, vec!["compfoo"]);
            match &a.action {
                AlterTypeAction::AddAttribute {
                    name,
                    data_type,
                    cascade,
                } => {
                    assert_eq!(name, "f3");
                    assert_eq!(data_type, "text");
                    assert!(!cascade);
                }
                other => panic!("expected AddAttribute, got {:?}", other),
            }
        }
        _ => panic!("expected AlterCompositeType, got {:?}", stmt),
    }
}

#[test]
fn test_alter_type_add_attribute_cascade() {
    let stmt = parse_one("ALTER TYPE compfoo ADD ATTRIBUTE f3 text CASCADE");
    match stmt {
        Statement::AlterCompositeType(a) => match &a.action {
            AlterTypeAction::AddAttribute { cascade, .. } => assert!(cascade),
            other => panic!("expected AddAttribute, got {:?}", other),
        },
        _ => panic!("expected AlterCompositeType, got {:?}", stmt),
    }
}

#[test]
fn test_alter_type_drop_attribute() {
    let stmt = parse_one("ALTER TYPE compfoo DROP ATTRIBUTE f2");
    match stmt {
        Statement::AlterCompositeType(a) => {
            assert_eq!(a.name, vec!["compfoo"]);
            match &a.action {
                AlterTypeAction::DropAttribute {
                    name,
                    if_exists,
                    cascade,
                } => {
                    assert_eq!(name, "f2");
                    assert!(!if_exists);
                    assert!(!cascade);
                }
                other => panic!("expected DropAttribute, got {:?}", other),
            }
        }
        _ => panic!("expected AlterCompositeType, got {:?}", stmt),
    }
}

#[test]
fn test_alter_type_drop_attribute_if_exists() {
    let stmt = parse_one("ALTER TYPE compfoo DROP ATTRIBUTE IF EXISTS f2 CASCADE");
    match stmt {
        Statement::AlterCompositeType(a) => match &a.action {
            AlterTypeAction::DropAttribute {
                name,
                if_exists,
                cascade,
            } => {
                assert_eq!(name, "f2");
                assert!(if_exists);
                assert!(cascade);
            }
            other => panic!("expected DropAttribute, got {:?}", other),
        },
        _ => panic!("expected AlterCompositeType, got {:?}", stmt),
    }
}

#[test]
fn test_alter_type_rename_attribute() {
    let stmt = parse_one("ALTER TYPE compfoo RENAME ATTRIBUTE f1 TO f1_new");
    match stmt {
        Statement::AlterCompositeType(a) => match &a.action {
            AlterTypeAction::RenameAttribute {
                old_name,
                new_name,
                cascade,
            } => {
                assert_eq!(old_name, "f1");
                assert_eq!(new_name, "f1_new");
                assert!(!cascade);
            }
            other => panic!("expected RenameAttribute, got {:?}", other),
        },
        _ => panic!("expected AlterCompositeType, got {:?}", stmt),
    }
}

#[test]
fn test_alter_type_rename_to() {
    let stmt = parse_one("ALTER TYPE compfoo RENAME TO new_compfoo");
    match stmt {
        Statement::AlterCompositeType(a) => match &a.action {
            AlterTypeAction::RenameTo(new_name) => assert_eq!(new_name, "new_compfoo"),
            other => panic!("expected RenameTo, got {:?}", other),
        },
        _ => panic!("expected AlterCompositeType, got {:?}", stmt),
    }
}

#[test]
fn test_alter_type_add_enum_value() {
    let stmt = parse_one("ALTER TYPE bug_status ADD VALUE 'in_progress' BEFORE 'closed'");
    match stmt {
        Statement::AlterCompositeType(a) => {
            assert_eq!(a.name, vec!["bug_status"]);
            match &a.action {
                AlterTypeAction::AddEnumValue {
                    value,
                    before,
                    after,
                } => {
                    assert_eq!(value, "in_progress");
                    assert_eq!(before, &Some("closed".to_string()));
                    assert!(after.is_none());
                }
                other => panic!("expected AddEnumValue, got {:?}", other),
            }
        }
        _ => panic!("expected AlterCompositeType, got {:?}", stmt),
    }
}

#[test]
fn test_alter_type_add_enum_value_after() {
    let stmt = parse_one("ALTER TYPE bug_status ADD VALUE 'in_progress' AFTER 'open'");
    match stmt {
        Statement::AlterCompositeType(a) => match &a.action {
            AlterTypeAction::AddEnumValue {
                value,
                before,
                after,
            } => {
                assert_eq!(value, "in_progress");
                assert!(before.is_none());
                assert_eq!(after, &Some("open".to_string()));
            }
            other => panic!("expected AddEnumValue, got {:?}", other),
        },
        _ => panic!("expected AlterCompositeType, got {:?}", stmt),
    }
}

#[test]
fn test_alter_type_rename_enum_value() {
    let stmt = parse_one("ALTER TYPE bug_status RENAME VALUE 'open' TO 'new_open'");
    match stmt {
        Statement::AlterCompositeType(a) => match &a.action {
            AlterTypeAction::RenameEnumValue {
                old_value,
                new_value,
            } => {
                assert_eq!(old_value, "open");
                assert_eq!(new_value, "new_open");
            }
            other => panic!("expected RenameEnumValue, got {:?}", other),
        },
        _ => panic!("expected AlterCompositeType, got {:?}", stmt),
    }
}

#[test]
fn test_alter_type_set_schema() {
    let stmt = parse_one("ALTER TYPE compfoo SET SCHEMA myschema");
    match stmt {
        Statement::AlterCompositeType(a) => match &a.action {
            AlterTypeAction::SetSchema(schema) => assert_eq!(schema, "myschema"),
            other => panic!("expected SetSchema, got {:?}", other),
        },
        _ => panic!("expected AlterCompositeType, got {:?}", stmt),
    }
}

#[test]
fn test_alter_type_owner_to() {
    let stmt = parse_one("ALTER TYPE compfoo OWNER TO postgres");
    match stmt {
        Statement::AlterCompositeType(a) => match &a.action {
            AlterTypeAction::OwnerTo(owner) => assert_eq!(owner, "postgres"),
            other => panic!("expected OwnerTo, got {:?}", other),
        },
        _ => panic!("expected AlterCompositeType, got {:?}", stmt),
    }
}

// ========== CREATE PACKAGE tests ==========

#[test]
fn test_create_package_basic() {
    let stmt = parse_one("CREATE PACKAGE my_pkg AS END my_pkg;");
    match stmt {
        Statement::CreatePackage(p) => {
            assert!(!p.replace);
            assert_eq!(p.name, vec!["my_pkg"]);
            assert!(p.authid.is_none());
            assert!(p.body.is_empty() || p.body.to_lowercase().contains("end"));
        }
        _ => panic!("expected CreatePackage, got {:?}", stmt),
    }
}

#[test]
fn test_create_or_replace_package() {
    let stmt = parse_one("CREATE OR REPLACE PACKAGE exp_pkg AS user_exp EXCEPTION; END exp_pkg;");
    match stmt {
        Statement::CreatePackage(p) => {
            assert!(p.replace);
            assert_eq!(p.name, vec!["exp_pkg"]);
        }
        _ => panic!("expected CreatePackage, got {:?}", stmt),
    }
}

#[test]
fn test_create_package_with_schema() {
    let stmt = parse_one("CREATE OR REPLACE PACKAGE dams_ci.pack_log AS PROCEDURE excption_1(in_desc IN varchar); END pack_log;");
    match stmt {
        Statement::CreatePackage(p) => {
            assert_eq!(p.name, vec!["dams_ci", "pack_log"]);
            assert!(p.body.contains("excption_1"));
        }
        _ => panic!("expected CreatePackage, got {:?}", stmt),
    }
}

#[test]
fn test_create_package_authid_current_user() {
    let stmt = parse_one("CREATE PACKAGE my_pkg AUTHID CURRENT_USER IS END my_pkg;");
    match stmt {
        Statement::CreatePackage(p) => {
            assert_eq!(p.authid, Some(PackageAuthid::CurrentUser));
        }
        _ => panic!("expected CreatePackage, got {:?}", stmt),
    }
}

#[test]
fn test_create_package_authid_definer() {
    let stmt = parse_one("CREATE PACKAGE my_pkg AUTHID DEFINER AS END my_pkg;");
    match stmt {
        Statement::CreatePackage(p) => {
            assert_eq!(p.authid, Some(PackageAuthid::Definer));
        }
        _ => panic!("expected CreatePackage, got {:?}", stmt),
    }
}

#[test]
fn test_create_package_body_basic() {
    let stmt = parse_one("CREATE OR REPLACE PACKAGE BODY exp_pkg AS END exp_pkg;");
    match stmt {
        Statement::CreatePackageBody(p) => {
            assert!(p.replace);
            assert_eq!(p.name, vec!["exp_pkg"]);
        }
        _ => panic!("expected CreatePackageBody, got {:?}", stmt),
    }
}

#[test]
fn test_create_package_body_with_function() {
    let stmt = parse_one("CREATE OR REPLACE PACKAGE BODY trigger_test AS function tri_insert_func() return trigger as begin insert into test_trigger_des_tbl values(new.id1, new.id2, new.id3); return new; end; end trigger_test;");
    match stmt {
        Statement::CreatePackageBody(p) => {
            assert!(p.body.contains("tri_insert_func"));
            assert!(p.body.contains("insert into"));
        }
        _ => panic!("expected CreatePackageBody, got {:?}", stmt),
    }
}

#[test]
fn test_create_package_spec_multi_procs() {
    let sql = "CREATE OR REPLACE PACKAGE my_pkg IS\n\
               PROCEDURE proc1(i_date IN VARCHAR2, o_flag OUT VARCHAR2);\n\
               PROCEDURE proc2(i_date IN VARCHAR2);\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackage(p) => {
            assert_eq!(p.name, vec!["my_pkg"]);
            assert!(p.body.contains("proc1"));
            assert!(p.body.contains("proc2"));
        }
        _ => panic!("expected CreatePackage, got {:?}", stmt),
    }
}

#[test]
fn test_create_package_body_multi_procedures() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS\n\
               PROCEDURE proc1(i_date IN VARCHAR2) IS\n\
                 v_x NUMBER;\n\
               BEGIN\n\
                 DELETE FROM t1 WHERE id = 1;\n\
               END proc1;\n\
               PROCEDURE proc2 IS\n\
               BEGIN\n\
                 INSERT INTO t2 VALUES(1);\n\
               END proc2;\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            assert_eq!(p.name, vec!["my_pkg"]);
            assert!(p.body.contains("proc1"));
            assert!(p.body.contains("proc2"));
            assert!(p.body.contains("delete from"));
            assert!(p.body.contains("insert into"));
        }
        _ => panic!("expected CreatePackageBody, got {:?}", stmt),
    }
}

#[test]
fn test_create_package_body_with_function_and_procedure() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS\n\
               FUNCTION get_name RETURN VARCHAR2 IS\n\
               BEGIN\n\
                 RETURN 'test';\n\
               END get_name;\n\
               PROCEDURE do_thing IS\n\
               BEGIN\n\
                 NULL;\n\
               END do_thing;\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            assert_eq!(p.name, vec!["my_pkg"]);
            assert!(p.body.contains("get_name"));
            assert!(p.body.contains("do_thing"));
        }
        _ => panic!("expected CreatePackageBody, got {:?}", stmt),
    }
}

// ========== P2: Structured Package Body Tests ==========

#[test]
fn test_package_body_structured_procedure() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS\n\
               PROCEDURE proc1(i_date IN VARCHAR2) IS\n\
                 v_x NUMBER;\n\
               BEGIN\n\
                 DELETE FROM t1 WHERE id = 1;\n\
                 INSERT INTO t2 VALUES(1);\n\
               END proc1;\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            assert_eq!(p.name, vec!["my_pkg"]);
            assert!(!p.items.is_empty(), "should have structured items");
            let proc = p
                .items
                .iter()
                .find_map(|item| match item {
                    PackageItem::Procedure(pr) => Some(pr),
                    _ => None,
                })
                .expect("should have a procedure");
            assert_eq!(proc.name, vec!["proc1"]);
            assert!(proc.block.is_some(), "procedure should have a body");
            let block = proc.block.as_ref().unwrap();
            assert!(
                !block.body.is_empty(),
                "procedure body should have statements"
            );
            assert!(
                !block.declarations.is_empty(),
                "procedure should have variable declarations"
            );
        }
        _ => panic!("expected CreatePackageBody, got {:?}", stmt),
    }
}

#[test]
fn test_package_body_structured_function() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS\n\
               FUNCTION get_name RETURN VARCHAR2 IS\n\
               BEGIN\n\
                 RETURN 'test';\n\
               END get_name;\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            assert!(!p.items.is_empty(), "should have structured items");
            let func = p
                .items
                .iter()
                .find_map(|item| match item {
                    PackageItem::Function(f) => Some(f),
                    _ => None,
                })
                .expect("should have a function");
            assert_eq!(func.name, vec!["get_name"]);
            assert_eq!(func.return_type.as_deref(), Some("varchar2"));
            assert!(func.block.is_some(), "function should have a body");
            let block = func.block.as_ref().unwrap();
            assert!(
                !block.body.is_empty(),
                "function body should have statements"
            );
        }
        _ => panic!("expected CreatePackageBody, got {:?}", stmt),
    }
}

#[test]
fn test_package_body_structured_multi() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS\n\
               PROCEDURE proc1(i_date IN VARCHAR2) IS\n\
               BEGIN\n\
                 DELETE FROM t1 WHERE id = 1;\n\
               END proc1;\n\
               PROCEDURE proc2 IS\n\
               BEGIN\n\
                 INSERT INTO t2 VALUES(1);\n\
               END proc2;\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            let procs: Vec<_> = p
                .items
                .iter()
                .filter_map(|item| match item {
                    PackageItem::Procedure(pr) => Some(pr),
                    _ => None,
                })
                .collect();
            assert_eq!(procs.len(), 2, "should have 2 procedures");
            assert_eq!(procs[0].name, vec!["proc1"]);
            assert_eq!(procs[1].name, vec!["proc2"]);
        }
        _ => panic!("expected CreatePackageBody, got {:?}", stmt),
    }
}

#[test]
fn test_package_body_structured_mixed() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS\n\
               FUNCTION get_name RETURN VARCHAR2 IS\n\
               BEGIN\n\
                 RETURN 'test';\n\
               END get_name;\n\
               PROCEDURE do_thing IS\n\
               BEGIN\n\
                 NULL;\n\
               END do_thing;\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            assert!(!p.items.is_empty(), "should have structured items");
            let has_func = p
                .items
                .iter()
                .any(|item| matches!(item, PackageItem::Function(_)));
            let has_proc = p
                .items
                .iter()
                .any(|item| matches!(item, PackageItem::Procedure(_)));
            assert!(has_func, "should have a function");
            assert!(has_proc, "should have a procedure");
        }
        _ => panic!("expected CreatePackageBody, got {:?}", stmt),
    }
}

// ========== Bare PROCEDURE / FUNCTION tests ==========

#[test]
fn test_bare_procedure_definition() {
    let sql = "PROCEDURE my_proc(i_date IN VARCHAR2) IS\n\
               v_x NUMBER;\n\
               BEGIN\n\
                 DELETE FROM t1 WHERE id = 1;\n\
               END my_proc;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            assert_eq!(p.name, vec!["my_proc"]);
        }
        _ => panic!("expected CreateProcedure, got {:?}", stmt),
    }
}

#[test]
fn test_bare_function_definition() {
    let sql = "FUNCTION get_name RETURN VARCHAR2 IS\n\
               BEGIN\n\
                 RETURN 'test';\n\
               END get_name;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateFunction(f) => {
            assert_eq!(f.name, vec!["get_name"]);
        }
        _ => panic!("expected CreateFunction, got {:?}", stmt),
    }
}

#[test]
fn test_create_procedure_with_structured_body() {
    let sql = "CREATE PROCEDURE my_proc(p_id IN INTEGER)\n\
               AS BEGIN\n\
                 DELETE FROM t1 WHERE id = 1;\n\
                 INSERT INTO t1 VALUES(2);\n\
               END;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            assert_eq!(p.name, vec!["my_proc"]);
            assert_eq!(p.parameters.len(), 1);
            let block = p.block.as_ref().expect("expected block to be parsed");
            assert!(
                block.body.len() >= 2,
                "expected at least 2 statements in body, got {}",
                block.body.len()
            );
        }
        _ => panic!("expected CreateProcedure, got {:?}", stmt),
    }
}

#[test]
fn test_create_procedure_with_declare_and_exception() {
    let sql = "CREATE PROCEDURE complex_proc\n\
               IS\n\
                 v_count INTEGER;\n\
               BEGIN\n\
                 SELECT count(*) INTO v_count FROM t1;\n\
                 IF v_count > 0 THEN\n\
                   DELETE FROM t1;\n\
                 END IF;\n\
               END;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            assert_eq!(p.name, vec!["complex_proc"]);
            let block = p.block.as_ref().expect("expected block to be parsed");
            assert!(!block.declarations.is_empty(), "expected declarations");
            assert!(block.body.len() >= 2, "expected at least 2 body statements");
        }
        _ => panic!("expected CreateProcedure, got {:?}", stmt),
    }
}

#[test]
fn test_create_function_with_structured_body() {
    let sql = "CREATE FUNCTION get_name(id INTEGER) RETURN VARCHAR2\n\
               IS\n\
               BEGIN\n\
                 RETURN 'test';\n\
               END;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateFunction(f) => {
            assert_eq!(f.name, vec!["get_name"]);
            let block = f.block.as_ref().expect("expected block to be parsed");
            assert!(!block.body.is_empty(), "expected body statements");
        }
        _ => panic!("expected CreateFunction, got {:?}", stmt),
    }
}

#[test]
fn test_create_procedure_without_body_falls_back() {
    let sql = "CREATE PROCEDURE java_proc LANGUAGE JAVA NAME 'com.example.proc()'";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            assert_eq!(p.name, vec!["java_proc"]);
            assert!(
                p.block.is_none(),
                "expected no block for LANGUAGE JAVA style"
            );
            assert!(
                !p.options.is_empty(),
                "expected options string for fallback case"
            );
        }
        _ => panic!("expected CreateProcedure, got {:?}", stmt),
    }
}

#[test]
fn test_create_function_dollar_quoted_body() {
    let sql =
        "CREATE FUNCTION foo() RETURNS integer AS $$ BEGIN RETURN 1; END; $$ LANGUAGE plpgsql";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateFunction(f) => {
            assert_eq!(f.name, vec!["foo"]);
            let block = f
                .block
                .as_ref()
                .expect("expected block to be parsed from dollar-quoted body");
            assert!(!block.body.is_empty(), "expected body statements");
        }
        _ => panic!("expected CreateFunction, got {:?}", stmt),
    }
}

#[test]
fn test_create_function_dollar_quoted_multi_statement() {
    let sql = "CREATE FUNCTION bar() RETURNS void AS $$ DECLARE x INTEGER; BEGIN x := 1; RETURN; END; $$ LANGUAGE plpgsql";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateFunction(f) => {
            assert_eq!(f.name, vec!["bar"]);
            let block = f.block.as_ref().expect("expected block");
            assert!(!block.declarations.is_empty(), "expected declarations");
            assert!(!block.body.is_empty(), "expected body statements");
        }
        _ => panic!("expected CreateFunction, got {:?}", stmt),
    }
}

#[test]
fn test_create_function_dollar_quoted_not_consume_next() {
    let sql = "CREATE FUNCTION f1() RETURNS void AS $$ BEGIN RETURN; END; $$ LANGUAGE plpgsql;\n\
               SELECT 1;\n\
               CREATE FUNCTION f2() RETURNS void AS $$ BEGIN RETURN; END; $$ LANGUAGE plpgsql;";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    assert_eq!(stmts.len(), 3, "expected 3 statements, got {}", stmts.len());
    assert!(matches!(&stmts[0], Statement::CreateFunction(_)));
    assert!(matches!(&stmts[1], Statement::Select(_)));
    assert!(matches!(&stmts[2], Statement::CreateFunction(_)));
}

#[test]
fn test_create_procedure_dollar_quoted_body() {
    let sql = "CREATE PROCEDURE my_proc() AS $$ BEGIN RETURN; END; $$ LANGUAGE plpgsql";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            assert_eq!(p.name, vec!["my_proc"]);
            let block = p.block.as_ref().expect("expected block");
            assert!(!block.body.is_empty());
        }
        _ => panic!("expected CreateProcedure, got {:?}", stmt),
    }
}

// ========== CREATE EXTENSION / DOMAIN / CAST tests ==========

#[test]
fn test_create_extension_basic() {
    let stmt = parse_one("CREATE EXTENSION hstore");
    match stmt {
        Statement::CreateExtension(e) => {
            assert!(!e.if_not_exists);
            assert_eq!(e.name, "hstore");
            assert!(e.schema.is_none());
            assert!(e.version.is_none());
            assert!(!e.cascade);
        }
        _ => panic!("expected CreateExtension, got {:?}", stmt),
    }
}

#[test]
fn test_create_extension_if_not_exists() {
    let stmt = parse_one("CREATE EXTENSION IF NOT EXISTS gms_debug");
    match stmt {
        Statement::CreateExtension(e) => {
            assert!(e.if_not_exists);
            assert_eq!(e.name, "gms_debug");
        }
        _ => panic!("expected CreateExtension, got {:?}", stmt),
    }
}

#[test]
fn test_create_extension_with_options() {
    let stmt =
        parse_one("CREATE EXTENSION IF NOT EXISTS hstore WITH SCHEMA public VERSION '1.0' CASCADE");
    match stmt {
        Statement::CreateExtension(e) => {
            assert!(e.if_not_exists);
            assert_eq!(e.name, "hstore");
            assert_eq!(e.schema, Some("public".to_string()));
            assert_eq!(e.version, Some("1.0".to_string()));
            assert!(e.cascade);
        }
        _ => panic!("expected CreateExtension, got {:?}", stmt),
    }
}

#[test]
fn test_create_domain_basic() {
    let stmt = parse_one("CREATE DOMAIN domaindroptest int4");
    match stmt {
        Statement::CreateDomain(d) => {
            assert_eq!(d.name, vec!["domaindroptest"]);
            assert!(matches!(d.data_type, DataType::Custom(_)));
            assert!(d.default_value.is_none());
            assert!(!d.not_null);
            assert!(d.check.is_none());
        }
        _ => panic!("expected CreateDomain, got {:?}", stmt),
    }
}

#[test]
fn test_create_domain_not_null() {
    let stmt = parse_one("CREATE DOMAIN dnotnull varchar(15) NOT NULL");
    match stmt {
        Statement::CreateDomain(d) => {
            assert_eq!(d.name, vec!["dnotnull"]);
            assert!(d.not_null);
        }
        _ => panic!("expected CreateDomain, got {:?}", stmt),
    }
}

#[test]
fn test_create_domain_with_check() {
    let stmt =
        parse_one("CREATE DOMAIN dcheck varchar(15) NOT NULL CHECK (VALUE = 'a' OR VALUE = 'c')");
    match stmt {
        Statement::CreateDomain(d) => {
            assert!(d.not_null);
            assert!(d.check.is_some());
        }
        _ => panic!("expected CreateDomain, got {:?}", stmt),
    }
}

#[test]
fn test_create_domain_with_default() {
    let stmt = parse_one("CREATE DOMAIN ddef1 int4 DEFAULT 3");
    match stmt {
        Statement::CreateDomain(d) => {
            assert!(matches!(d.data_type, DataType::Custom(_)));
            assert!(d.default_value.is_some());
        }
        _ => panic!("expected CreateDomain, got {:?}", stmt),
    }
}

#[test]
fn test_create_cast_without_function() {
    let stmt = parse_one("CREATE CAST (text AS casttesttype) WITHOUT FUNCTION");
    match stmt {
        Statement::CreateCast(c) => {
            assert!(matches!(c.source_type, DataType::Text));
            assert!(matches!(c.target_type, DataType::Custom(_)));
            assert!(matches!(c.method, CastMethod::WithoutFunction));
            assert!(c.context.is_none());
        }
        _ => panic!("expected CreateCast, got {:?}", stmt),
    }
}

#[test]
fn test_create_cast_without_function_implicit() {
    let stmt = parse_one("CREATE CAST (text AS casttesttype) WITHOUT FUNCTION AS IMPLICIT");
    match stmt {
        Statement::CreateCast(c) => {
            assert!(matches!(c.method, CastMethod::WithoutFunction));
            assert_eq!(c.context, Some(CastContext::Implicit));
        }
        _ => panic!("expected CreateCast, got {:?}", stmt),
    }
}

#[test]
fn test_create_cast_with_inout() {
    let stmt = parse_one("CREATE CAST (int4 AS casttesttype) WITH INOUT");
    match stmt {
        Statement::CreateCast(c) => {
            assert!(matches!(c.method, CastMethod::WithInout));
        }
        _ => panic!("expected CreateCast, got {:?}", stmt),
    }
}

#[test]
fn test_create_cast_with_function() {
    let stmt = parse_one(
        "CREATE CAST (int4 AS casttesttype) WITH FUNCTION int4_casttesttype(int4) AS IMPLICIT",
    );
    match stmt {
        Statement::CreateCast(c) => {
            match &c.method {
                CastMethod::WithFunction(func) => {
                    assert!(func.contains("int4_casttesttype"));
                }
                other => panic!("expected WithFunction, got {:?}", other),
            }
            assert_eq!(c.context, Some(CastContext::Implicit));
        }
        _ => panic!("expected CreateCast, got {:?}", stmt),
    }
}

// ========== ALTER VIEW / TRIGGER / EXTENSION tests ==========

#[test]
fn test_alter_view_rename() {
    let stmt = parse_one("ALTER VIEW my_view RENAME TO new_view");
    match stmt {
        Statement::AlterView(a) => {
            assert_eq!(a.name, vec!["my_view"]);
            match &a.action {
                AlterViewAction::RenameTo(name) => assert_eq!(name, "new_view"),
                other => panic!("expected RenameTo, got {:?}", other),
            }
        }
        _ => panic!("expected AlterView, got {:?}", stmt),
    }
}

#[test]
fn test_alter_view_set() {
    let stmt = parse_one("ALTER VIEW my_property_normal SET (security_barrier=true)");
    match stmt {
        Statement::AlterView(a) => match &a.action {
            AlterViewAction::Set(opts) => {
                assert!(!opts.is_empty());
            }
            other => panic!("expected Set, got {:?}", other),
        },
        _ => panic!("expected AlterView, got {:?}", stmt),
    }
}

#[test]
fn test_alter_view_reset() {
    let stmt = parse_one("ALTER VIEW rw_view2 RESET (check_option)");
    match stmt {
        Statement::AlterView(a) => match &a.action {
            AlterViewAction::Reset(names) => {
                assert!(names.contains(&"check_option".to_string()));
            }
            other => panic!("expected Reset, got {:?}", other),
        },
        _ => panic!("expected AlterView, got {:?}", stmt),
    }
}

#[test]
fn test_alter_view_set_schema() {
    let stmt = parse_one("ALTER VIEW test SET SCHEMA target_schema");
    match stmt {
        Statement::AlterView(a) => match &a.action {
            AlterViewAction::SetSchema(schema) => assert_eq!(schema, "target_schema"),
            other => panic!("expected SetSchema, got {:?}", other),
        },
        _ => panic!("expected AlterView, got {:?}", stmt),
    }
}

#[test]
fn test_alter_view_alter_column_default() {
    let stmt = parse_one("ALTER VIEW rw_view1 ALTER COLUMN bb SET DEFAULT 'View default'");
    match stmt {
        Statement::AlterView(a) => match &a.action {
            AlterViewAction::AlterColumnDefault {
                column,
                set_default,
            } => {
                assert_eq!(column, "bb");
                assert!(set_default.is_some());
            }
            other => panic!("expected AlterColumnDefault, got {:?}", other),
        },
        _ => panic!("expected AlterView, got {:?}", stmt),
    }
}

#[test]
fn test_alter_trigger_rename() {
    let stmt =
        parse_one("ALTER TRIGGER repcount_update_row ON my_table RENAME TO repcount_update_row2");
    match stmt {
        Statement::AlterTrigger(a) => {
            assert_eq!(a.name, "repcount_update_row");
            assert_eq!(a.table, vec!["my_table"]);
            assert_eq!(a.new_name, "repcount_update_row2");
        }
        _ => panic!("expected AlterTrigger, got {:?}", stmt),
    }
}

#[test]
fn test_alter_extension_update() {
    let stmt = parse_one("ALTER EXTENSION hstore UPDATE TO '1.1'");
    match stmt {
        Statement::AlterExtension(a) => {
            assert_eq!(a.name, "hstore");
            assert!(a.action.contains("update") || a.action.contains("UPDATE"));
        }
        _ => panic!("expected AlterExtension, got {:?}", stmt),
    }
}

// ========== Cursor/Query parsed_query tests ==========

#[test]
fn test_cursor_decl_with_parsed_select() {
    let sql = "DO $$ DECLARE cur1 CURSOR FOR SELECT id, name FROM users WHERE active = 1; BEGIN OPEN cur1; END $$";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Do(d) => {
            let block = d.block.as_ref().expect("DO block should be parsed");
            assert_eq!(block.declarations.len(), 1);
            match &block.declarations[0] {
                PlDeclaration::Cursor(c) => {
                    assert_eq!(c.name, "cur1");
                    assert!(c.parsed_query.is_some(), "cursor query should be parsed");
                    let parsed = c.parsed_query.as_ref().unwrap();
                    match parsed.as_ref() {
                        crate::ast::Statement::Select(sel) => {
                            assert_eq!(sel.targets.len(), 2);
                        }
                        other => panic!("expected Select, got {:?}", other),
                    }
                }
                other => panic!("expected Cursor decl, got {:?}", other),
            }
        }
        other => panic!("expected Do, got {:?}", other),
    }
}

#[test]
fn test_open_for_with_parsed_select() {
    let sql = r#"
        BEGIN
            OPEN cur1 FOR SELECT id, name FROM users;
        END
    "#;
    let stmt = parse_one(sql);
    match stmt {
        Statement::AnonyBlock(ab) => {
            assert_eq!(ab.block.body.len(), 1);
            match &ab.block.body[0] {
                PlStatement::Open(open_stmt) => match &open_stmt.kind {
                    PlOpenKind::ForQuery {
                        query,
                        parsed_query,
                    } => {
                        assert!(!query.is_empty());
                        assert!(parsed_query.is_some(), "OPEN FOR query should be parsed");
                        let parsed = parsed_query.as_ref().unwrap();
                        match parsed.as_ref() {
                            crate::ast::Statement::Select(sel) => {
                                assert_eq!(sel.targets.len(), 2);
                            }
                            other => panic!("expected Select, got {:?}", other),
                        }
                    }
                    other => panic!("expected ForQuery, got {:?}", other),
                },
                other => panic!("expected Open, got {:?}", other),
            }
        }
        other => panic!("expected AnonyBlock, got {:?}", other),
    }
}

#[test]
fn test_for_in_query_with_parsed_select() {
    let sql = "BEGIN FOR rec IN SELECT id FROM users LOOP NULL; END LOOP; END";
    let stmt = parse_one(sql);
    match stmt {
        Statement::AnonyBlock(ab) => {
            assert_eq!(ab.block.body.len(), 1);
            match &ab.block.body[0] {
                PlStatement::For(for_stmt) => match &for_stmt.kind {
                    PlForKind::Query {
                        query,
                        parsed_query,
                        ..
                    } => {
                        assert!(!query.is_empty());
                        assert!(parsed_query.is_some(), "FOR IN query should be parsed");
                        let parsed = parsed_query.as_ref().unwrap();
                        match parsed.as_ref() {
                            crate::ast::Statement::Select(sel) => {
                                assert_eq!(sel.targets.len(), 1);
                            }
                            other => panic!("expected Select, got {:?}", other),
                        }
                    }
                    other => panic!("expected Query kind, got {:?}", other),
                },
                other => panic!("expected For, got {:?}", other),
            }
        }
        other => panic!("expected AnonyBlock, got {:?}", other),
    }
}

#[test]
fn test_nested_procedure_declaration() {
    let sql = "CREATE OR REPLACE PROCEDURE outer_proc(p1 IN NUMBER) AS \
               v_count NUMBER := 0; \
               PROCEDURE inner_proc(p2 IN NUMBER) AS \
                 v_inner NUMBER; \
               BEGIN \
                 v_inner := p2 + 1; \
               END inner_proc; \
               BEGIN \
                 v_count := p1; \
                 inner_proc(v_count); \
               END";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(proc) => {
            assert_eq!(proc.name, vec!["outer_proc"]);
            let block = proc.block.as_ref().expect("outer block should be parsed");
            let nested = block
                .declarations
                .iter()
                .filter_map(|d| match d {
                    PlDeclaration::NestedProcedure(p) => Some(p),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(nested.len(), 1, "should have 1 nested procedure");
            assert_eq!(nested[0].name, vec!["inner_proc"]);
            let inner_block = nested[0]
                .block
                .as_ref()
                .expect("inner block should be parsed");
            assert_eq!(inner_block.declarations.len(), 1);
            assert!(inner_block.body.len() > 0, "inner block should have body");
        }
        other => panic!("expected CreateProcedure, got {:?}", other),
    }
}

// ── P3/P4/P5 tests ──

#[test]
fn test_create_foreign_table_with_types() {
    let sql = "CREATE FOREIGN TABLE ft (id INT, name VARCHAR(100)) SERVER my_server";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateForeignTable(t) => {
            assert_eq!(t.columns.len(), 2);
            assert!(matches!(t.columns[0].data_type, DataType::Integer));
            assert!(matches!(
                t.columns[1].data_type,
                DataType::Varchar(Some(100))
            ));
        }
        _ => panic!("expected CreateForeignTable, got {:?}", stmt),
    }
}

#[test]
fn test_create_materialized_view_parsed_query() {
    let sql =
        "CREATE MATERIALIZED VIEW mv AS SELECT id, name FROM users WHERE active = true WITH DATA";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateMaterializedView(mv) => {
            assert!(mv.with_data);
            assert!(!mv.query.targets.is_empty());
            assert!(!mv.query.from.is_empty());
        }
        _ => panic!("expected CreateMaterializedView, got {:?}", stmt),
    }
}

#[test]
fn test_create_trigger_with_when_expr() {
    let sql = "CREATE TRIGGER trg AFTER UPDATE ON users FOR EACH ROW WHEN (OLD.status IS DISTINCT FROM NEW.status) EXECUTE PROCEDURE log_change()";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateTrigger(t) => {
            assert_eq!(t.name, "trg");
            assert!(t.when.is_some());
            assert!(t.func_args.is_empty());
        }
        _ => panic!("expected CreateTrigger, got {:?}", stmt),
    }
}

#[test]
fn test_create_trigger_with_func_args() {
    let sql = "CREATE TRIGGER trg BEFORE INSERT ON t FOR EACH ROW EXECUTE PROCEDURE fn(1, 'hello')";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateTrigger(t) => {
            assert_eq!(t.func_args.len(), 2);
        }
        _ => panic!("expected CreateTrigger, got {:?}", stmt),
    }
}

#[test]
fn test_format_create_extension() {
    use crate::formatter::SqlFormatter;
    let sql = "CREATE EXTENSION IF NOT EXISTS hstore SCHEMA public";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    assert!(formatted.contains("CREATE EXTENSION"));
    assert!(formatted.contains("IF NOT EXISTS"));
    assert!(formatted.contains("hstore"));
    assert!(!formatted.contains("stub"));
}

#[test]
fn test_format_create_function() {
    use crate::formatter::SqlFormatter;
    let sql = "FUNCTION get_name RETURN VARCHAR2 IS\n\
               BEGIN\n\
                 RETURN 'test';\n\
               END get_name";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateFunction(_) => {
            let formatted = SqlFormatter::new().format_statement(&stmt);
            assert!(formatted.contains("CREATE FUNCTION"));
            assert!(!formatted.contains("stub"));
        }
        other => panic!("expected CreateFunction, got {:?}", other),
    }
}

#[test]
fn test_format_grant_role() {
    use crate::formatter::SqlFormatter;
    let sql = "GRANT admin TO user1 WITH ADMIN OPTION";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    assert!(formatted.contains("GRANT"));
    assert!(formatted.contains("admin"));
    assert!(formatted.contains("user1"));
    assert!(!formatted.contains("stub"));
}

#[test]
fn test_format_alter_trigger() {
    use crate::formatter::SqlFormatter;
    let sql = "ALTER TRIGGER trg ON users RENAME TO trg2";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    assert!(formatted.contains("ALTER TRIGGER"));
    assert!(formatted.contains("trg"));
    assert!(formatted.contains("trg2"));
    assert!(!formatted.contains("stub"));
}

#[test]
fn test_format_create_cast() {
    use crate::formatter::SqlFormatter;
    let sql = "CREATE CAST (text AS integer) WITHOUT FUNCTION AS IMPLICIT";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    assert!(formatted.contains("CREATE CAST"));
    assert!(!formatted.contains("stub"));
}

#[test]
fn test_format_create_domain() {
    use crate::formatter::SqlFormatter;
    let sql = "CREATE DOMAIN pos_int AS INTEGER NOT NULL CHECK (VALUE > 0)";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    assert!(formatted.contains("CREATE DOMAIN"));
    assert!(!formatted.contains("stub"));
}

#[test]
fn test_format_create_package() {
    use crate::formatter::SqlFormatter;
    let sql = "CREATE OR REPLACE PACKAGE my_pkg IS PROCEDURE proc1(i INT); END my_pkg";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    assert!(formatted.contains("CREATE"));
    assert!(formatted.contains("PACKAGE"));
    assert!(!formatted.contains("stub"));
}

#[test]
fn test_roundtrip_select() {
    use crate::formatter::SqlFormatter;
    let sql = "SELECT id, name FROM users WHERE active = true ORDER BY id LIMIT 10";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(stmt, stmt2);
}

#[test]
fn test_roundtrip_insert() {
    use crate::formatter::SqlFormatter;
    let sql = "INSERT INTO users (id, name) VALUES (1, 'Alice')";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(stmt, stmt2);
}

#[test]
fn test_select_union() {
    let sql = "SELECT id FROM users UNION ALL SELECT id FROM admins";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert!(s.set_operation.is_some());
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_select_with_cte() {
    let sql = "WITH RECURSIVE cte AS (SELECT 1 AS n UNION ALL SELECT n + 1 FROM cte WHERE n < 10) SELECT * FROM cte";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert!(s.with.is_some());
            let w = s.with.as_ref().unwrap();
            assert!(w.recursive);
            assert_eq!(w.ctes.len(), 1);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_format_alter_group() {
    use crate::formatter::SqlFormatter;
    // ALTER GROUP is not yet dispatched in dispatch_alter(), returns Empty
    let sql = "ALTER GROUP admins ADD USER john";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let _ = formatted;
}

#[test]
fn test_format_revoke_role() {
    use crate::formatter::SqlFormatter;
    let sql = "REVOKE admin FROM user1 CASCADE";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    assert!(formatted.contains("REVOKE"));
    assert!(formatted.contains("CASCADE"));
    assert!(!formatted.contains("stub"));
}

#[test]
fn test_materialized_view_with_tablespace() {
    let sql = "CREATE MATERIALIZED VIEW mv AS SELECT id FROM users TABLESPACE ts1 WITH DATA";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateMaterializedView(mv) => {
            assert_eq!(mv.tablespace, Some("ts1".to_string()));
            assert!(mv.with_data);
        }
        _ => panic!("expected CreateMaterializedView, got {:?}", stmt),
    }
}

// ========== Literal Type Preservation Tests ==========

#[test]
fn test_bit_string_literal() {
    let stmt = parse_one("SELECT B'10101'");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, None) => {
                    assert!(matches!(expr, Expr::Literal(Literal::BitString(s)) if s == "10101"));
                }
                _ => panic!("expected expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_hex_string_literal() {
    let stmt = parse_one("SELECT X'FF00'");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, None) => {
                    assert!(matches!(expr, Expr::Literal(Literal::HexString(s)) if s == "FF00"));
                }
                _ => panic!("expected expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_escape_string_literal() {
    let stmt = parse_one("SELECT E'tab\\there'");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, None) => {
                    assert!(
                        matches!(expr, Expr::Literal(Literal::EscapeString(_))),
                        "expected EscapeString, got: {:?}",
                        expr
                    );
                }
                _ => panic!("expected expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_national_string_literal() {
    let stmt = parse_one("SELECT N'hello'");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, None) => {
                    assert!(
                        matches!(expr, Expr::Literal(Literal::NationalString(s)) if s == "hello")
                    );
                }
                _ => panic!("expected expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_dollar_string_literal() {
    let stmt = parse_one("SELECT $$hello world$$");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, None) => {
                    assert!(
                        matches!(expr, Expr::Literal(Literal::DollarString { tag: None, body }) if body == "hello world")
                    );
                }
                _ => panic!("expected expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_tagged_dollar_string_literal() {
    let stmt = parse_one("SELECT $tag$hello$tag$");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, None) => {
                    assert!(
                        matches!(expr, Expr::Literal(Literal::DollarString { tag: Some(t), body }) if t == "tag" && body == "hello")
                    );
                }
                _ => panic!("expected expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_plain_string_literal_unchanged() {
    let stmt = parse_one("SELECT 'hello'");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, None) => {
                    assert!(matches!(expr, Expr::Literal(Literal::String(s)) if s == "hello"));
                }
                _ => panic!("expected expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_literal_format_roundtrip() {
    use crate::formatter::SqlFormatter;
    let formatter = SqlFormatter::new();

    // B'...'
    let stmt = parse_one("SELECT B'10101'");
    let sql = formatter.format_statement(&stmt);
    assert!(sql.contains("B'10101'"), "expected B'10101' in: {}", sql);

    // X'...'
    let stmt = parse_one("SELECT X'FF00'");
    let sql = formatter.format_statement(&stmt);
    assert!(sql.contains("X'FF00'"), "expected X'FF00' in: {}", sql);

    // E'...'
    let stmt = parse_one("SELECT E'\\\\n'");
    let sql = formatter.format_statement(&stmt);
    assert!(sql.contains("E'"), "expected E' prefix in: {}", sql);

    // N'...'
    let stmt = parse_one("SELECT N'hello'");
    let sql = formatter.format_statement(&stmt);
    assert!(sql.contains("N'hello'"), "expected N'hello' in: {}", sql);

    // $$...$$
    let stmt = parse_one("SELECT $$body$$");
    let sql = formatter.format_statement(&stmt);
    assert!(sql.contains("$$body$$"), "expected $$body$$ in: {}", sql);

    // $tag$...$tag$
    let stmt = parse_one("SELECT $tag$hello$tag$");
    let sql = formatter.format_statement(&stmt);
    assert!(
        sql.contains("$tag$hello$tag$"),
        "expected $tag$hello$tag$ in: {}",
        sql
    );
}

// ========== JSON Deserialize Round-Trip Tests ==========

fn json_roundtrip(stmt: &Statement) -> Statement {
    let json = serde_json::to_string(stmt).unwrap();
    serde_json::from_str(&json).unwrap()
}

fn sql_roundtrip(sql: &str) -> String {
    use crate::formatter::SqlFormatter;
    let stmt = parse_one(sql);
    let de = json_roundtrip(&stmt);
    SqlFormatter::new().format_statement(&de)
}

#[test]
fn test_json_roundtrip_select() {
    let stmt =
        parse_one("SELECT id, name FROM users WHERE status = 'active' ORDER BY id DESC LIMIT 10");
    assert_eq!(stmt, json_roundtrip(&stmt));
}

#[test]
fn test_json_roundtrip_insert() {
    let stmt =
        parse_one("INSERT INTO users (id, name) VALUES (1, 'Alice'), (2, 'Bob') RETURNING id");
    assert_eq!(stmt, json_roundtrip(&stmt));
}

#[test]
fn test_json_roundtrip_update() {
    let stmt = parse_one("UPDATE users SET name = 'Bob' WHERE id = 1 RETURNING *");
    assert_eq!(stmt, json_roundtrip(&stmt));
}

#[test]
fn test_json_roundtrip_delete() {
    let stmt = parse_one("DELETE FROM users WHERE id = 1");
    assert_eq!(stmt, json_roundtrip(&stmt));
}

#[test]
fn test_json_roundtrip_create_table() {
    let stmt = parse_one(
        "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name VARCHAR(100) NOT NULL)",
    );
    assert_eq!(stmt, json_roundtrip(&stmt));
}

#[test]
fn test_json_roundtrip_special_literals() {
    let stmt = parse_one("SELECT B'1010', X'FF', N'hello'");
    assert_eq!(stmt, json_roundtrip(&stmt));
}

#[test]
fn test_json_roundtrip_complex_expressions() {
    let stmt = parse_one("SELECT CASE WHEN x > 0 THEN 1 WHEN x < 0 THEN -1 ELSE 0 END FROM t WHERE a BETWEEN 1 AND 10 AND b IN (1, 2, 3)");
    assert_eq!(stmt, json_roundtrip(&stmt));
}

#[test]
fn test_sql_roundtrip_select_basic() {
    assert_eq!(
        sql_roundtrip("SELECT id FROM users"),
        "SELECT id FROM users"
    );
}

#[test]
fn test_sql_roundtrip_special_literals() {
    assert!(sql_roundtrip("SELECT B'10101'").contains("B'10101'"));
    assert!(sql_roundtrip("SELECT X'FF'").contains("X'FF'"));
    assert!(sql_roundtrip("SELECT N'hello'").contains("N'hello'"));
}

#[test]
fn test_sql_roundtrip_insert_values() {
    let result = sql_roundtrip("INSERT INTO t (a, b) VALUES (1, 'x')");
    assert!(result.contains("INSERT INTO"));
    assert!(result.contains("VALUES"));
    assert!(result.contains("'x'"));
}

#[test]
fn test_sql_roundtrip_join() {
    let result =
        sql_roundtrip("SELECT a.id FROM users AS a INNER JOIN orders AS o ON a.id = o.user_id");
    assert!(result.contains("INNER JOIN"));
    assert!(result.contains("ON"));
}

// ========== Window Frame Enum Tests ==========

#[test]
fn test_json_roundtrip_window_frame_rows() {
    let stmt = parse_one("SELECT ROW_NUMBER() OVER (ORDER BY id ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING) FROM t");
    assert_eq!(stmt, json_roundtrip(&stmt));
}

#[test]
fn test_json_roundtrip_window_frame_range() {
    let stmt = parse_one(
        "SELECT AVG(x) OVER (ORDER BY id RANGE BETWEEN 1 PRECEDING AND 1 FOLLOWING) FROM t",
    );
    assert_eq!(stmt, json_roundtrip(&stmt));
}

#[test]
fn test_json_roundtrip_window_frame_current_row() {
    let stmt = parse_one("SELECT SUM(x) OVER (PARTITION BY a ORDER BY b ROWS BETWEEN CURRENT ROW AND 1 FOLLOWING) FROM t");
    assert_eq!(stmt, json_roundtrip(&stmt));
}

#[test]
fn test_json_roundtrip_create_domain() {
    let stmt = parse_one("CREATE DOMAIN pos_int AS INTEGER NOT NULL CHECK (VALUE > 0)");
    assert_eq!(stmt, json_roundtrip(&stmt));
}

#[test]
fn test_json_roundtrip_create_domain_with_default() {
    let stmt = parse_one("CREATE DOMAIN ddef1 int4 DEFAULT 3 NOT NULL");
    assert_eq!(stmt, json_roundtrip(&stmt));
}

#[test]
fn test_json_roundtrip_create_cast() {
    let stmt = parse_one("CREATE CAST (text AS casttesttype) WITHOUT FUNCTION AS IMPLICIT");
    assert_eq!(stmt, json_roundtrip(&stmt));
}

#[test]
fn test_json_roundtrip_create_rls_policy() {
    let stmt = parse_one("CREATE POLICY p1 ON t1 USING (true)");
    assert_eq!(stmt, json_roundtrip(&stmt));
}

// ========== P3 Semantic Skip Tests ==========

#[test]
fn test_declare_cursor_with_parsed_select() {
    let sql = "DECLARE cur1 CURSOR FOR SELECT id, name FROM users WHERE active = true";
    let stmt = parse_one(sql);
    match stmt {
        Statement::DeclareCursor(c) => {
            assert_eq!(c.name, "cur1");
            assert!(!c.scroll);
            assert!(!c.binary);
            // query is now Box<SelectStatement>, not String
            assert!(
                !c.query.targets.is_empty(),
                "cursor query should have targets"
            );
            assert!(!c.query.from.is_empty(), "cursor query should have FROM");
            assert!(
                c.query.where_clause.is_some(),
                "cursor query should have WHERE"
            );
        }
        _ => panic!("expected DeclareCursor, got {:?}", stmt),
    }
}

#[test]
fn test_declare_cursor_scroll_with_select() {
    let sql = "DECLARE cur2 SCROLL CURSOR FOR SELECT * FROM t";
    let stmt = parse_one(sql);
    match stmt {
        Statement::DeclareCursor(c) => {
            assert_eq!(c.name, "cur2");
            assert!(c.scroll);
            assert!(!c.query.targets.is_empty());
        }
        _ => panic!("expected DeclareCursor, got {:?}", stmt),
    }
}

#[test]
fn test_execute_with_expr_params() {
    let sql = "EXECUTE prep_stmt(1, 'hello', 3.14)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Execute(e) => {
            assert_eq!(e.name, "prep_stmt");
            assert_eq!(e.params.len(), 3);
            // params are now Expr, not String
            assert!(matches!(&e.params[0], Expr::Literal(Literal::Integer(1))));
        }
        _ => panic!("expected Execute, got {:?}", stmt),
    }
}

#[test]
fn test_execute_no_params() {
    let sql = "EXECUTE prep_stmt";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Execute(e) => {
            assert_eq!(e.name, "prep_stmt");
            assert!(e.params.is_empty());
        }
        _ => panic!("expected Execute, got {:?}", stmt),
    }
}

#[test]
fn test_rule_with_parsed_condition() {
    let sql = "RULE r1 AS ON SELECT TO users DO INSTEAD NOTHING";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Rule(r) => {
            assert_eq!(r.name, "r1");
            assert!(r.condition.is_none());
            assert!(r.instead);
        }
        _ => panic!("expected Rule, got {:?}", stmt),
    }
}

#[test]
fn test_rule_with_where_condition() {
    let sql = "RULE r2 AS ON UPDATE TO users WHERE old.status = 'active' DO INSTEAD NOTHING";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Rule(r) => {
            assert_eq!(r.name, "r2");
            assert!(r.condition.is_some(), "rule should have a condition");
        }
        _ => panic!("expected Rule, got {:?}", stmt),
    }
}

#[test]
fn test_plpgsql_fetch_with_direction() {
    let block = parse_do_block("DO $$ BEGIN FETCH NEXT FROM cur INTO x; END $$");
    match &block.body[0] {
        PlStatement::Fetch(f) => {
            assert_eq!(f.cursor, "cur");
            assert!(matches!(f.direction, Some(plpgsql::FetchDirection::Next)));
            assert!(matches!(&f.into, Expr::ColumnRef(name) if name == &["x".to_string()]));
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_plpgsql_move_with_direction() {
    let block = parse_do_block("DO $$ BEGIN MOVE NEXT cur; END $$");
    match &block.body[0] {
        PlStatement::Move { cursor, direction } => {
            assert_eq!(cursor, "cur");
            assert!(matches!(direction, Some(plpgsql::FetchDirection::Next)));
        }
        _ => panic!("expected Move"),
    }
}

#[test]
fn test_plpgsql_get_diagnostics_message_text() {
    let block = parse_do_block("DO $$ BEGIN GET DIAGNOSTICS msg = MESSAGE_TEXT; END $$");
    match &block.body[0] {
        PlStatement::GetDiagnostics(g) => {
            assert!(!g.stacked);
            assert_eq!(g.items.len(), 1);
            assert_eq!(g.items[0].target, "msg");
            assert!(matches!(
                g.items[0].item,
                plpgsql::GetDiagItemKind::MessageText
            ));
        }
        _ => panic!("expected GetDiagnostics"),
    }
}

#[test]
fn test_cast_with_numeric_data_type() {
    let sql = "SELECT CAST(123.45 AS NUMERIC(10,2))";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            if let SelectTarget::Expr(expr, _) = &s.targets[0] {
                match expr {
                    Expr::TypeCast { type_name, .. } => {
                        assert!(matches!(type_name, DataType::Numeric(Some(10), Some(2))));
                    }
                    _ => panic!("expected TypeCast expression"),
                }
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_cast_with_integer_data_type() {
    let sql = "SELECT CAST(123 AS INTEGER)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            if let SelectTarget::Expr(expr, _) = &s.targets[0] {
                match expr {
                    Expr::TypeCast { type_name, .. } => {
                        assert!(matches!(type_name, DataType::Integer));
                    }
                    _ => panic!("expected TypeCast expression"),
                }
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_implicit_typecast_custom_data_type() {
    let sql = "SELECT date '2023-01-01'";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            if let SelectTarget::Expr(expr, _) = &s.targets[0] {
                match expr {
                    Expr::TypeCast { type_name, .. } => {
                        assert!(matches!(type_name, DataType::Custom(_)));
                    }
                    _ => panic!("expected TypeCast expression"),
                }
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_json_roundtrip_typecast() {
    let sql = "SELECT CAST(123 AS INTEGER)";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    let json = serde_json::to_string(&stmts).unwrap();
    let deserialized: Vec<Statement> = serde_json::from_str(&json).unwrap();
    assert_eq!(stmts, deserialized);
}

#[test]
fn test_prepare_with_parsed_select() {
    let sql = "PREPARE q1 AS SELECT * FROM users";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Prepare(p) => {
            assert_eq!(p.name, "q1");
            assert!(p.parsed_statement.is_some());
            let inner = *p.parsed_statement.unwrap();
            assert!(matches!(inner, Statement::Select(_)));
        }
        _ => panic!("expected Prepare"),
    }
}

#[test]
fn test_prepare_with_parsed_insert() {
    let sql = "PREPARE ins(int, text) AS INSERT INTO t VALUES($1, $2)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Prepare(p) => {
            assert_eq!(p.name, "ins");
            assert_eq!(p.data_types, vec!["int", "text"]);
            assert!(p.parsed_statement.is_some());
            let inner = *p.parsed_statement.unwrap();
            assert!(matches!(inner, Statement::Insert(_)));
        }
        _ => panic!("expected Prepare"),
    }
}

#[test]
fn test_rule_statement_has_parsed_actions_none() {
    let sql = "RULE notify_me AS ON UPDATE TO users DO INSTEAD NOTHING";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Rule(r) => {
            assert_eq!(r.name, "notify_me");
            assert!(r.instead);
            assert!(r.parsed_actions.is_none());
        }
        _ => panic!("expected Rule"),
    }
}
