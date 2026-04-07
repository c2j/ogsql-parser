use crate::ast::plpgsql::*;
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
            assert_eq!(v.default.as_deref(), Some("42"));
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
            assert_eq!(expression, "1");
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
            assert_eq!(if_stmt.condition, "true");
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
            assert_eq!(if_stmt.elsifs[0].condition, "false");
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
            assert_eq!(case_stmt.expression.as_deref(), Some("x")); // plain CASE
            assert_eq!(case_stmt.whens.len(), 1);
            assert_eq!(case_stmt.whens[0].condition, "1");
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
            assert_eq!(w.condition, "true");
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
            assert_eq!(w.condition, "true");
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
                    assert_eq!(low, "1");
                    assert_eq!(high, "10");
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
                PlForKind::Query { query } => assert_eq!(query, "select 1"),
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
        } => assert_eq!(c, "true"),
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
        } => assert_eq!(c, "false"),
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
        } => assert_eq!(e, "42"),
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
            assert!(e.string_expr.contains("SELECT 1"));
        }
        _ => panic!("expected Execute"),
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
            assert_eq!(f.into, "x");
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
                    assert_eq!(low, "1");
                    assert_eq!(high, "5");
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
