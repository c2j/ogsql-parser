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

#[test]
fn test_plpgsql_return_next() {
    let block = parse_do_block("DO $$ BEGIN RETURN NEXT 42; END $$");
    match &block.body[0] {
        PlStatement::ReturnNext { expression } => {
            assert!(matches!(expression, Expr::Literal(Literal::Integer(42))));
        }
        _ => panic!("expected ReturnNext"),
    }
}

#[test]
fn test_plpgsql_return_query_select() {
    let block = parse_do_block("DO $$ BEGIN RETURN QUERY SELECT * FROM t; END $$");
    match &block.body[0] {
        PlStatement::ReturnQuery(q) => {
            assert!(!q.is_dynamic);
            assert_eq!(q.query, "select * from t");
            assert!(q.dynamic_expr.is_none());
            assert!(q.using_args.is_empty());
        }
        _ => panic!("expected ReturnQuery"),
    }
}

#[test]
fn test_plpgsql_return_query_execute() {
    let block = parse_do_block("DO $$ BEGIN RETURN QUERY EXECUTE 'SELECT 1'; END $$");
    match &block.body[0] {
        PlStatement::ReturnQuery(q) => {
            assert!(q.is_dynamic);
            assert!(q.dynamic_expr.is_some());
            assert!(q.using_args.is_empty());
        }
        _ => panic!("expected ReturnQuery"),
    }
}

#[test]
fn test_plpgsql_return_query_execute_using() {
    let block = parse_do_block("DO $$ BEGIN RETURN QUERY EXECUTE 'SELECT $1' USING 10; END $$");
    match &block.body[0] {
        PlStatement::ReturnQuery(q) => {
            assert!(q.is_dynamic);
            assert!(q.dynamic_expr.is_some());
            assert_eq!(q.using_args.len(), 1);
            assert!(matches!(q.using_args[0].mode, PlUsingMode::In));
            assert!(matches!(
                q.using_args[0].argument,
                Expr::Literal(Literal::Integer(10))
            ));
        }
        _ => panic!("expected ReturnQuery"),
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

#[test]
fn test_plpgsql_raise_format_params() {
    let block = parse_do_block("DO $$ BEGIN RAISE NOTICE 'Hello %', name; END $$");
    match &block.body[0] {
        PlStatement::Raise(r) => {
            assert!(matches!(r.level, Some(RaiseLevel::Notice)));
            assert_eq!(r.message.as_deref(), Some("'Hello %' , name"));
            assert_eq!(r.params.len(), 0);
        }
        _ => panic!("expected Raise"),
    }
}

#[test]
fn test_plpgsql_raise_using_errcode() {
    let block = parse_do_block("DO $$ BEGIN RAISE EXCEPTION USING ERRCODE = '12345'; END $$");
    match &block.body[0] {
        PlStatement::Raise(r) => {
            assert!(matches!(r.level, Some(RaiseLevel::Exception)));
            assert_eq!(r.message.as_deref(), Some("using ERRCODE = '12345'"));
            assert_eq!(r.options.len(), 0);
        }
        _ => panic!("expected Raise"),
    }
}

#[test]
fn test_plpgsql_raise_condition_name() {
    let block = parse_do_block("DO $$ BEGIN RAISE division_by_zero; END $$");
    match &block.body[0] {
        PlStatement::Raise(r) => {
            assert!(r.level.is_none());
            assert_eq!(r.message.as_deref(), Some("division_by_zero"));
            assert!(r.condname.is_none());
        }
        _ => panic!("expected Raise"),
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

#[test]
fn test_plpgsql_execute_string_literal_parsed() {
    let block =
        parse_do_block("DO $$ BEGIN EXECUTE IMMEDIATE 'call calc_stats($1, $1, $2, $1)'; END $$");
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert!(
                e.parsed_query.is_some(),
                "string literal should be re-parsed"
            );
            let inner = e.parsed_query.as_ref().unwrap();
            match inner.as_ref() {
                crate::ast::Statement::Call(c) => {
                    assert_eq!(c.func_name, vec!["calc_stats".to_string()]);
                    assert_eq!(c.args.len(), 4);
                }
                other => panic!("expected Call statement, got {:?}", other),
            }
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_variable_not_parsed() {
    let block = parse_do_block("DO $$ BEGIN EXECUTE IMMEDIATE plsql_block USING a, b; END $$");
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert!(e.parsed_query.is_none(), "variable should NOT be re-parsed");
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_concat_not_parsed() {
    let block =
        parse_do_block("DO $$ BEGIN EXECUTE IMMEDIATE 'SELECT * FROM ' || tab_name; END $$");
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert!(
                e.parsed_query.is_none(),
                "concatenation should NOT be re-parsed"
            );
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_dml_string_parsed() {
    let block =
        parse_do_block("DO $$ BEGIN EXECUTE 'SELECT id, name FROM users WHERE id = 1'; END $$");
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(!e.immediate);
            assert!(e.parsed_query.is_some());
            let inner = e.parsed_query.as_ref().unwrap();
            assert!(matches!(inner.as_ref(), crate::ast::Statement::Select(_)));
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_invalid_sql_string_not_parsed() {
    let block = parse_do_block("DO $$ BEGIN EXECUTE 'not valid sql at all !!!'; END $$");
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(
                e.parsed_query.is_none(),
                "invalid SQL should gracefully fall back to None"
            );
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

#[test]
fn test_plpgsql_release_savepoint() {
    let block = parse_do_block("DO $$ BEGIN RELEASE SAVEPOINT sp1; END $$");
    match &block.body[0] {
        PlStatement::ReleaseSavepoint { name } => assert_eq!(name, "sp1"),
        _ => panic!("expected ReleaseSavepoint"),
    }
}

#[test]
fn test_plpgsql_release_savepoint_short() {
    let block = parse_do_block("DO $$ BEGIN RELEASE sp1; END $$");
    match &block.body[0] {
        PlStatement::ReleaseSavepoint { name } => assert_eq!(name, "sp1"),
        _ => panic!("expected ReleaseSavepoint"),
    }
}

#[test]
fn test_plpgsql_forall() {
    let block = parse_do_block("DO $$ BEGIN FORALL i IN 1..10 INSERT INTO t VALUES (i); END $$");
    match &block.body[0] {
        PlStatement::ForAll(f) => {
            assert_eq!(f.variable, "i");
            assert_eq!(f.bounds, "1  10 insert into t values ( i )");
            assert!(!f.save_exceptions);
        }
        _ => panic!("expected ForAll"),
    }
}

#[test]
fn test_plpgsql_forall_save_exceptions() {
    let block = parse_do_block(
        "DO $$ BEGIN FORALL i IN 1..10 SAVE EXCEPTIONS INSERT INTO t VALUES (i); END $$",
    );
    match &block.body[0] {
        PlStatement::ForAll(f) => {
            assert_eq!(f.variable, "i");
            assert_eq!(f.bounds, "1  10 insert into t values ( i )");
            assert!(f.save_exceptions);
        }
        _ => panic!("expected ForAll with SAVE EXCEPTIONS"),
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
                    if_not_exists: _,
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
                if_not_exists: _,
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
            assert!(p.items.iter().any(|item| match item {
                PackageItem::Procedure(pr) => pr.name.join(".").contains("excption_1"),
                PackageItem::Raw(s) => s.contains("excption_1"),
                _ => false,
            }), "should contain excption_1 procedure");
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
            assert!(p.items.iter().any(|item| match item {
                PackageItem::Function(f) => f.name.join(".").contains("tri_insert_func"),
                PackageItem::Raw(s) => s.contains("tri_insert_func"),
                _ => false,
            }), "should contain tri_insert_func");
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
            let proc_names: Vec<String> = p.items.iter().filter_map(|item| match item {
                PackageItem::Procedure(pr) => Some(pr.name.join(".")),
                _ => None,
            }).collect();
            assert!(proc_names.iter().any(|n| n.contains("proc1")), "should contain proc1, got: {:?}", proc_names);
            assert!(proc_names.iter().any(|n| n.contains("proc2")), "should contain proc2, got: {:?}", proc_names);
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
            let proc_names: Vec<String> = p.items.iter().filter_map(|item| match item {
                PackageItem::Procedure(pr) => Some(pr.name.join(".")),
                _ => None,
            }).collect();
            assert!(proc_names.iter().any(|n| n.contains("proc1")), "should contain proc1");
            assert!(proc_names.iter().any(|n| n.contains("proc2")), "should contain proc2");
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
            assert!(p.items.iter().any(|item| match item {
                PackageItem::Function(f) => f.name.join(".").contains("get_name"),
                PackageItem::Raw(s) => s.contains("get_name"),
                _ => false,
            }), "should contain get_name");
            assert!(p.items.iter().any(|item| match item {
                PackageItem::Procedure(pr) => pr.name.join(".").contains("do_thing"),
                PackageItem::Raw(s) => s.contains("do_thing"),
                _ => false,
            }), "should contain do_thing");
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

// ========== P3: No redundant body field in Package ==========


#[test]
fn test_package_body_no_redundant_body_field() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS\n\
               PROCEDURE proc1(i_date IN VARCHAR2) IS\n\
                 v_x NUMBER;\n\
               BEGIN\n\
                 DELETE FROM t1 WHERE id = 1;\n\
               END proc1;\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    match &stmt {
        Statement::CreatePackageBody(p) => {
            assert!(!p.items.is_empty());
            let json = serde_json::to_value(&stmt).unwrap();
            let pkg = json.get("CreatePackageBody").unwrap();
            assert!(
                pkg.get("body").is_none(),
                "CreatePackageBody should NOT have a 'body' field; it is redundant with items"
            );
        }
        _ => panic!("expected CreatePackageBody, got {:?}", stmt),
    }
}

#[test]
fn test_package_spec_no_redundant_body_field() {
    let sql = "CREATE OR REPLACE PACKAGE my_pkg IS\n\
               PROCEDURE proc1(i INT);\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    match &stmt {
        Statement::CreatePackage(p) => {
            let json = serde_json::to_value(&stmt).unwrap();
            let pkg = json.get("CreatePackage").unwrap();
            assert!(
                pkg.get("body").is_none(),
                "CreatePackage should NOT have a 'body' field; it is redundant with items"
            );
        }
        _ => panic!("expected CreatePackage, got {:?}", stmt),
    }
}

// ========== P4: Embedded SQL text in PL/pgSQL blocks ==========

#[test]
fn test_embedded_select_sql_text_not_empty() {
    let sql = "DO $$ BEGIN SELECT 1 INTO v_x FROM t WHERE id = 1; END $$";
    let block = parse_do_block(sql);
    assert!(!block.body.is_empty(), "block should have statements");
    match &block.body[0] {
        PlStatement::SqlStatement { sql_text, .. } => {
            assert!(
                !sql_text.is_empty(),
                "SqlStatement.sql_text should contain the original SQL text, but it was empty"
            );
            assert!(
                sql_text.to_uppercase().contains("SELECT"),
                "sql_text should contain 'SELECT', got: {:?}",
                sql_text
            );
        }
        other => panic!("expected SqlStatement, got {:?}", other),
    }
}

#[test]
fn test_embedded_insert_sql_text_not_empty() {
    let sql = "DO $$ BEGIN INSERT INTO t VALUES(1, 'hello'); END $$";
    let block = parse_do_block(sql);
    match &block.body[0] {
        PlStatement::SqlStatement { sql_text, .. } => {
            assert!(
                !sql_text.is_empty(),
                "SqlStatement.sql_text should contain the original INSERT text"
            );
            assert!(
                sql_text.to_uppercase().contains("INSERT"),
                "sql_text should contain 'INSERT', got: {:?}",
                sql_text
            );
        }
        other => panic!("expected SqlStatement, got {:?}", other),
    }
}

#[test]
fn test_embedded_delete_sql_text_not_empty() {
    let sql = "DO $$ BEGIN DELETE FROM t1 WHERE id = 1; END $$";
    let block = parse_do_block(sql);
    match &block.body[0] {
        PlStatement::SqlStatement { sql_text, .. } => {
            assert!(
                !sql_text.is_empty(),
                "SqlStatement.sql_text should contain the original DELETE text"
            );
            assert!(
                sql_text.to_uppercase().contains("DELETE"),
                "sql_text should contain 'DELETE', got: {:?}",
                sql_text
            );
        }
        other => panic!("expected SqlStatement, got {:?}", other),
    }
}

#[test]
fn test_embedded_select_in_package_body_sql_text_not_empty() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS\n\
               PROCEDURE proc1 IS\n\
               BEGIN\n\
                 SELECT 1 INTO v_status FROM user_scheduler_jobs t WHERE t.job_name = 'test';\n\
               END proc1;\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    match &stmt {
        Statement::CreatePackageBody(p) => {
            let proc = p
                .items
                .iter()
                .find_map(|item| match item {
                    PackageItem::Procedure(pr) => Some(pr),
                    _ => None,
                })
                .expect("should have a procedure");
            let block = proc.block.as_ref().expect("procedure should have a block");
            let sql_stmt = block
                .body
                .iter()
                .find_map(|s| match s {
                    PlStatement::SqlStatement { sql_text, .. } => Some(sql_text.clone()),
                    _ => None,
                })
                .expect("block should contain a SqlStatement");
            assert!(
                !sql_stmt.is_empty(),
                "SqlStatement.sql_text should contain the SELECT text inside package body procedure"
            );
            assert!(
                sql_stmt.to_uppercase().contains("SELECT"),
                "sql_text should contain 'SELECT', got: {:?}",
                sql_stmt
            );
        }
        _ => panic!("expected CreatePackageBody, got {:?}", stmt),
    }
}

// ========== P5: Formatter round-trip from items (no body field) ==========

#[test]
fn test_format_package_body_from_items() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS\n\
               PROCEDURE proc1(i_date IN VARCHAR2) IS\n\
                 v_x NUMBER;\n\
               BEGIN\n\
                 DELETE FROM t1 WHERE id = 1;\n\
               END proc1;\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    assert!(formatted.contains("CREATE"), "formatted should contain CREATE, got: {}", formatted);
    assert!(formatted.contains("PACKAGE BODY"), "formatted should contain PACKAGE BODY, got: {}", formatted);
    assert!(formatted.contains("my_pkg"), "formatted should contain package name, got: {}", formatted);
    assert!(formatted.contains("proc1"), "formatted should contain procedure name, got: {}", formatted);
    assert!(
        formatted.to_uppercase().contains("DELETE"),
        "formatted should contain DELETE statement, got: {}",
        formatted
    );
    assert!(
        formatted.to_uppercase().contains("END"),
        "formatted should contain END, got: {}",
        formatted
    );
}

#[test]
fn test_format_package_body_roundtrip() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS\n\
               PROCEDURE proc1(i_date IN VARCHAR2) IS\n\
               BEGIN\n\
                 DELETE FROM t1 WHERE id = 1;\n\
               END proc1;\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(
        stmt, stmt2,
        "round-trip should produce equivalent AST\nOriginal formatted: {}",
        formatted
    );
}

#[test]
fn test_format_package_body_with_function_roundtrip() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS\n\
               FUNCTION get_name RETURN VARCHAR2 IS\n\
               BEGIN\n\
                 RETURN 'test';\n\
               END get_name;\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(
        stmt, stmt2,
        "round-trip should produce equivalent AST\nOriginal formatted: {}",
        formatted
    );
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
                !p.options.extra.is_empty(),
                "expected options extra for fallback case"
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
            assert!(matches!(d.data_type, DataType::Custom(_, _)));
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
            assert!(matches!(d.data_type, DataType::Custom(_, _)));
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
            assert!(matches!(c.target_type, DataType::Custom(_, _)));
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
            assert_eq!(a.table.as_ref().unwrap(), &vec!["my_table"]);
            assert_eq!(a.new_name.as_ref().unwrap(), "repcount_update_row2");
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
                other => panic!("expected Cursor, got {:?}", other),
            }
        }
        other => panic!("expected Do, got {:?}", other),
    }
}

#[test]
fn test_cursor_decl_with_is_keyword() {
    let sql = "DO $$ DECLARE cur1 CURSOR IS SELECT id FROM users; BEGIN OPEN cur1; END $$";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Do(d) => {
            let block = d
                .block
                .as_ref()
                .expect("DO block should be parsed with IS keyword");
            assert_eq!(block.declarations.len(), 1);
            match &block.declarations[0] {
                PlDeclaration::Cursor(c) => {
                    assert_eq!(c.name, "cur1");
                    assert!(c.parsed_query.is_some(), "cursor query should be parsed");
                }
                other => panic!("expected Cursor, got {:?}", other),
            }
        }
        other => panic!("expected Do, got {:?}", other),
    }
}

#[test]
fn test_oracle_cursor_in_procedure_body() {
    let sql = "CREATE OR REPLACE PROCEDURE proc1() AS DECLARE CURSOR cu IS SELECT name FROM users; v_name VARCHAR(50); BEGIN OPEN cu; FETCH cu INTO v_name; CLOSE cu; END; /";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            let block = p.block.as_ref().expect("procedure should have a body");
            assert_eq!(block.declarations.len(), 2);
            match &block.declarations[0] {
                PlDeclaration::Cursor(c) => {
                    assert_eq!(c.name, "cu");
                    assert!(c.parsed_query.is_some());
                }
                other => panic!("expected Cursor, got {:?}", other),
            }
            match &block.declarations[1] {
                PlDeclaration::Variable(v) => {
                    assert_eq!(v.name, "v_name");
                }
                other => panic!("expected Variable, got {:?}", other),
            }
            assert_eq!(block.body.len(), 3);
        }
        other => panic!("expected CreateProcedure, got {:?}", other),
    }
}

#[test]
fn test_pg_cursor_in_procedure_body() {
    let sql = "CREATE OR REPLACE PROCEDURE proc2() AS DECLARE cu CURSOR FOR SELECT id FROM t; BEGIN OPEN cu; CLOSE cu; END; /";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateProcedure(p) => {
            let block = p.block.as_ref().expect("procedure should have a body");
            assert_eq!(block.declarations.len(), 1);
            match &block.declarations[0] {
                PlDeclaration::Cursor(c) => {
                    assert_eq!(c.name, "cu");
                    assert!(c.parsed_query.is_some());
                }
                other => panic!("expected Cursor, got {:?}", other),
            }
        }
        other => panic!("expected CreateProcedure, got {:?}", other),
    }
}

#[test]
fn test_alter_table_drop_partition_update_global_index() {
    let stmt = parse_one("ALTER TABLE t1 DROP PARTITION p1 UPDATE GLOBAL INDEX");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            match &at.actions[0] {
                AlterTableAction::DropPartition {
                    name,
                    if_exists,
                    update_global_index,
                    update_distributed_global_index,
                } => {
                    assert_eq!(name, "p1");
                    assert!(!if_exists);
                    assert!(*update_global_index);
                    assert!(update_distributed_global_index.is_none());
                }
                _ => panic!("expected DropPartition"),
            }
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_drop_partition_update_distributed_global_index() {
    let stmt = parse_one("ALTER TABLE t1 DROP PARTITION p1 UPDATE DISTRIBUTED GLOBAL INDEX");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            match &at.actions[0] {
                AlterTableAction::DropPartition {
                    name,
                    update_global_index,
                    update_distributed_global_index,
                    ..
                } => {
                    assert_eq!(name, "p1");
                    assert!(!*update_global_index);
                    assert_eq!(*update_distributed_global_index, Some(true));
                }
                _ => panic!("expected DropPartition"),
            }
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_merge_partitions_no_update_distributed_global_index() {
    let stmt = parse_one(
        "ALTER TABLE t1 MERGE PARTITIONS p1, p2 INTO PARTITION p3 NO UPDATE DISTRIBUTED GLOBAL INDEX",
    );
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            match &at.actions[0] {
                AlterTableAction::MergePartitions {
                    names,
                    into_name,
                    update_global_index,
                    update_distributed_global_index,
                } => {
                    assert_eq!(names, &vec!["p1", "p2"]);
                    assert_eq!(into_name, "p3");
                    assert!(!*update_global_index);
                    assert_eq!(*update_distributed_global_index, Some(false));
                }
                _ => panic!("expected MergePartitions"),
            }
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_enable_row_movement() {
    let stmt = parse_one("ALTER TABLE t1 ENABLE ROW MOVEMENT");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            assert!(matches!(
                &at.actions[0],
                AlterTableAction::EnableRowMovement
            ));
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_disable_row_movement() {
    let stmt = parse_one("ALTER TABLE t1 DISABLE ROW MOVEMENT");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            assert!(matches!(
                &at.actions[0],
                AlterTableAction::DisableRowMovement
            ));
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_move_partition_for() {
    let stmt = parse_one("ALTER TABLE t1 MOVE PARTITION FOR (100) TABLESPACE ts1");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            match &at.actions[0] {
                AlterTableAction::MovePartitionFor { expr, tablespace } => {
                    assert_eq!(tablespace, "ts1");
                    let _ = expr;
                }
                _ => panic!("expected MovePartitionFor"),
            }
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_split_partition_for() {
    let stmt = parse_one(
        "ALTER TABLE t1 SPLIT PARTITION FOR (100) AT (200) INTO (PARTITION p2, PARTITION p3)",
    );
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            match &at.actions[0] {
                AlterTableAction::SplitPartitionFor {
                    expr,
                    at_value,
                    into,
                    update_global_index,
                    update_distributed_global_index,
                } => {
                    assert!(at_value.is_some());
                    assert_eq!(into.len(), 2);
                    assert!(!*update_global_index);
                    assert!(update_distributed_global_index.is_none());
                    let _ = expr;
                }
                _ => panic!("expected SplitPartitionFor"),
            }
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_split_partition_for_update_global_index() {
    let stmt = parse_one(
        "ALTER TABLE t1 SPLIT PARTITION FOR (100) AT (200) INTO (PARTITION p2, PARTITION p3) UPDATE GLOBAL INDEX",
    );
    match stmt {
        Statement::AlterTable(at) => match &at.actions[0] {
            AlterTableAction::SplitPartitionFor {
                update_global_index,
                update_distributed_global_index,
                ..
            } => {
                assert!(*update_global_index);
                assert!(update_distributed_global_index.is_none());
            }
            _ => panic!("expected SplitPartitionFor"),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_exchange_partition_with_validation() {
    let stmt =
        parse_one("ALTER TABLE t1 EXCHANGE PARTITION p1 WITH TABLE t2 WITH VALIDATION VERBOSE");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            match &at.actions[0] {
                AlterTableAction::ExchangePartition {
                    name,
                    table,
                    with_validation,
                    verbose,
                    update_global_index,
                    update_distributed_global_index,
                } => {
                    assert_eq!(name, "p1");
                    assert_eq!(table.join("."), "t2");
                    assert_eq!(*with_validation, Some(true));
                    assert!(*verbose);
                    assert!(!*update_global_index);
                    assert!(update_distributed_global_index.is_none());
                }
                _ => panic!("expected ExchangePartition"),
            }
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_exchange_partition_without_validation() {
    let stmt = parse_one("ALTER TABLE t1 EXCHANGE PARTITION p1 WITH TABLE t2 WITHOUT VALIDATION");
    match stmt {
        Statement::AlterTable(at) => match &at.actions[0] {
            AlterTableAction::ExchangePartition {
                with_validation,
                verbose,
                ..
            } => {
                assert_eq!(*with_validation, Some(false));
                assert!(!*verbose);
            }
            _ => panic!("expected ExchangePartition"),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_exchange_partition_update_global_index() {
    let stmt = parse_one("ALTER TABLE t1 EXCHANGE PARTITION p1 WITH TABLE t2 UPDATE GLOBAL INDEX");
    match stmt {
        Statement::AlterTable(at) => match &at.actions[0] {
            AlterTableAction::ExchangePartition {
                update_global_index,
                with_validation,
                verbose,
                ..
            } => {
                assert!(*update_global_index);
                assert!(with_validation.is_none());
                assert!(!*verbose);
            }
            _ => panic!("expected ExchangePartition"),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_truncate_partition_update_distributed_global_index() {
    let stmt = parse_one("ALTER TABLE t1 TRUNCATE PARTITION p1 UPDATE DISTRIBUTED GLOBAL INDEX");
    match stmt {
        Statement::AlterTable(at) => match &at.actions[0] {
            AlterTableAction::TruncatePartition {
                name,
                update_distributed_global_index,
                ..
            } => {
                assert_eq!(name, "p1");
                assert_eq!(*update_distributed_global_index, Some(true));
            }
            _ => panic!("expected TruncatePartition"),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_partition_update_index_roundtrip() {
    use crate::formatter::SqlFormatter;
    let cases = vec![
        (
            "ALTER TABLE t1 DROP PARTITION p1 UPDATE GLOBAL INDEX",
            "ALTER TABLE t1 DROP PARTITION p1 UPDATE GLOBAL INDEX",
        ),
        (
            "ALTER TABLE t1 SPLIT PARTITION p1 AT (100) INTO (PARTITION p2, PARTITION p3) UPDATE GLOBAL INDEX",
            "ALTER TABLE t1 SPLIT PARTITION p1 AT (100) INTO (PARTITION p2, PARTITION p3) UPDATE GLOBAL INDEX",
        ),
        (
            "ALTER TABLE t1 EXCHANGE PARTITION p1 WITH TABLE t2 WITH VALIDATION VERBOSE",
            "ALTER TABLE t1 EXCHANGE PARTITION p1 WITH TABLE t2 WITH VALIDATION VERBOSE",
        ),
        (
            "ALTER TABLE t1 EXCHANGE PARTITION p1 WITH TABLE t2 WITHOUT VALIDATION",
            "ALTER TABLE t1 EXCHANGE PARTITION p1 WITH TABLE t2 WITHOUT VALIDATION",
        ),
        (
            "ALTER TABLE t1 ENABLE ROW MOVEMENT",
            "ALTER TABLE t1 ENABLE ROW MOVEMENT",
        ),
        (
            "ALTER TABLE t1 DISABLE ROW MOVEMENT",
            "ALTER TABLE t1 DISABLE ROW MOVEMENT",
        ),
        (
            "ALTER TABLE t1 MOVE PARTITION FOR (100) TABLESPACE ts1",
            "ALTER TABLE t1 MOVE PARTITION FOR (100) TABLESPACE ts1",
        ),
        (
            "ALTER TABLE t1 SPLIT PARTITION FOR (100) AT (200) INTO (PARTITION p2, PARTITION p3)",
            "ALTER TABLE t1 SPLIT PARTITION FOR (100) AT (200) INTO (PARTITION p2, PARTITION p3)",
        ),
        (
            "ALTER TABLE t1 MERGE PARTITIONS p1, p2 INTO PARTITION p3 NO UPDATE DISTRIBUTED GLOBAL INDEX",
            "ALTER TABLE t1 MERGE PARTITIONS p1, p2 INTO PARTITION p3 NO UPDATE DISTRIBUTED GLOBAL INDEX",
        ),
    ];
    let formatter = SqlFormatter::new();
    for (input, expected) in cases {
        let stmt = parse_one(input);
        let output = formatter.format_statement(&stmt);
        assert_eq!(output, expected, "roundtrip failed for: {}", input);
        let stmt2 = parse_one(&output);
        assert_eq!(stmt, stmt2, "AST mismatch for: {}", input);
    }
}

// ========== CREATE GLOBAL INDEX Tests ==========

#[test]
fn test_create_global_index_basic() {
    let sql = "CREATE GLOBAL INDEX idx ON t1(col1)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateGlobalIndex(s) => {
            assert!(!s.unique);
            assert!(!s.concurrent);
            assert!(!s.if_not_exists);
            assert_eq!(s.name.as_ref().unwrap(), &vec!["idx".to_string()]);
            assert_eq!(s.table, vec!["t1".to_string()]);
            assert_eq!(s.columns.len(), 1);
            assert_eq!(s.columns[0].name, "col1");
            assert!(s.columns[0].expression.is_none());
            assert!(s.using_method.is_none());
            assert!(s.containing.is_empty());
            assert!(s.distribute_by.is_none());
            assert!(s.with_options.is_empty());
            assert!(s.tablespace.is_none());
            assert!(s.visible.is_none());
            assert!(s.where_clause.is_none());
        }
        other => panic!("expected CreateGlobalIndex, got {:?}", other),
    }
}

#[test]
fn test_create_global_index_unique_concurrently() {
    let sql = "CREATE GLOBAL UNIQUE INDEX CONCURRENTLY IF NOT EXISTS idx ON t1(col1)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateGlobalIndex(s) => {
            assert!(s.unique);
            assert!(s.concurrent);
            assert!(s.if_not_exists);
            assert_eq!(s.name.as_ref().unwrap(), &vec!["idx".to_string()]);
        }
        other => panic!("expected CreateGlobalIndex, got {:?}", other),
    }
}

#[test]
fn test_create_global_index_using_method() {
    let sql = "CREATE GLOBAL INDEX idx ON t1 USING btree(col1)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateGlobalIndex(s) => {
            assert_eq!(s.using_method.as_deref(), Some("btree"));
        }
        other => panic!("expected CreateGlobalIndex, got {:?}", other),
    }
}

#[test]
fn test_create_global_index_column_options() {
    let sql = "CREATE GLOBAL INDEX idx ON t1(col1 ASC, col2 DESC NULLS FIRST, col3 COLLATE \"en_US\" NULLS LAST)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateGlobalIndex(s) => {
            assert_eq!(s.columns.len(), 3);

            // col1 ASC
            assert_eq!(s.columns[0].name, "col1");
            assert_eq!(s.columns[0].ordering, Some(IndexOrdering::Asc));
            assert!(s.columns[0].nulls.is_none());

            // col2 DESC NULLS FIRST
            assert_eq!(s.columns[1].name, "col2");
            assert_eq!(s.columns[1].ordering, Some(IndexOrdering::Desc));
            assert_eq!(s.columns[1].nulls, Some(IndexNulls::First));

            // col3 COLLATE "en_US" NULLS LAST
            assert_eq!(s.columns[2].name, "col3");
            assert_eq!(s.columns[2].collation.as_deref(), Some("en_US"));
            assert_eq!(s.columns[2].nulls, Some(IndexNulls::Last));
        }
        other => panic!("expected CreateGlobalIndex, got {:?}", other),
    }
}

#[test]
fn test_create_global_index_prefix_length() {
    let sql = "CREATE GLOBAL INDEX idx ON t1(col1(10))";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateGlobalIndex(s) => {
            assert_eq!(s.columns.len(), 1);
            assert_eq!(s.columns[0].name, "col1");
            assert_eq!(s.columns[0].length, Some(10));
        }
        other => panic!("expected CreateGlobalIndex, got {:?}", other),
    }
}

#[test]
fn test_create_global_index_expression() {
    let sql = "CREATE GLOBAL INDEX idx ON t1(UPPER(name))";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateGlobalIndex(s) => {
            assert_eq!(s.columns.len(), 1);
            // Expression column: name should be empty, expression should be set
            assert!(s.columns[0].expression.is_some());
        }
        other => panic!("expected CreateGlobalIndex, got {:?}", other),
    }
}

#[test]
fn test_create_global_index_containing() {
    let sql = "CREATE GLOBAL INDEX idx ON t1(col1) CONTAINING (col2, col3)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateGlobalIndex(s) => {
            assert_eq!(s.containing, vec!["col2", "col3"]);
        }
        other => panic!("expected CreateGlobalIndex, got {:?}", other),
    }
}

#[test]
fn test_create_global_index_distribute_by() {
    let sql = "CREATE GLOBAL INDEX idx ON t1(col1) DISTRIBUTE BY HASH(col1, col2)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateGlobalIndex(s) => match &s.distribute_by {
            Some(DistributeClause::Hash { columns }) => {
                assert_eq!(columns, &vec!["col1", "col2"]);
            }
            other => panic!("expected Hash distribute, got {:?}", other),
        },
        other => panic!("expected CreateGlobalIndex, got {:?}", other),
    }
}

#[test]
fn test_create_global_index_with_tablespace() {
    let sql = "CREATE GLOBAL INDEX idx ON t1(col1) WITH (fillfactor = 70) TABLESPACE ts1";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateGlobalIndex(s) => {
            assert_eq!(s.with_options.len(), 1);
            assert_eq!(
                s.with_options[0],
                ("fillfactor".to_string(), "70".to_string())
            );
            assert_eq!(s.tablespace.as_deref(), Some("ts1"));
        }
        other => panic!("expected CreateGlobalIndex, got {:?}", other),
    }
}

#[test]
fn test_create_global_index_visible_invisible() {
    let visible_sql = "CREATE GLOBAL INDEX idx ON t1(col1) VISIBLE";
    let stmt = parse_one(visible_sql);
    match stmt {
        Statement::CreateGlobalIndex(s) => {
            assert_eq!(s.visible, Some(true));
        }
        other => panic!("expected CreateGlobalIndex, got {:?}", other),
    }

    let invisible_sql = "CREATE GLOBAL INDEX idx ON t1(col1) INVISIBLE";
    let stmt = parse_one(invisible_sql);
    match stmt {
        Statement::CreateGlobalIndex(s) => {
            assert_eq!(s.visible, Some(false));
        }
        other => panic!("expected CreateGlobalIndex, got {:?}", other),
    }
}

#[test]
fn test_create_global_index_where_clause() {
    let sql = "CREATE GLOBAL INDEX idx ON t1(col1) WHERE col1 > 10";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateGlobalIndex(s) => {
            assert!(s.where_clause.is_some());
        }
        other => panic!("expected CreateGlobalIndex, got {:?}", other),
    }
}

#[test]
fn test_create_global_index_full() {
    let sql = "CREATE GLOBAL UNIQUE INDEX CONCURRENTLY IF NOT EXISTS schema1.idx ON schema2.t1 USING btree(col1 ASC, col2 DESC NULLS FIRST) CONTAINING (col3, col4) DISTRIBUTE BY HASH(col1) WITH (fillfactor = 70) TABLESPACE ts1 VISIBLE WHERE col1 > 10";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateGlobalIndex(s) => {
            assert!(s.unique);
            assert!(s.concurrent);
            assert!(s.if_not_exists);
            assert_eq!(s.name.as_ref().unwrap().join("."), "schema1.idx");
            assert_eq!(s.table.join("."), "schema2.t1");
            assert_eq!(s.using_method.as_deref(), Some("btree"));
            assert_eq!(s.columns.len(), 2);
            assert_eq!(s.columns[0].name, "col1");
            assert_eq!(s.columns[0].ordering, Some(IndexOrdering::Asc));
            assert_eq!(s.columns[1].name, "col2");
            assert_eq!(s.columns[1].ordering, Some(IndexOrdering::Desc));
            assert_eq!(s.columns[1].nulls, Some(IndexNulls::First));
            assert_eq!(s.containing, vec!["col3", "col4"]);
            assert!(matches!(
                s.distribute_by,
                Some(DistributeClause::Hash { .. })
            ));
            assert_eq!(s.with_options.len(), 1);
            assert_eq!(s.tablespace.as_deref(), Some("ts1"));
            assert_eq!(s.visible, Some(true));
            assert!(s.where_clause.is_some());
        }
        other => panic!("expected CreateGlobalIndex, got {:?}", other),
    }
}

#[test]
fn test_create_global_index_roundtrip() {
    let sql = "CREATE GLOBAL UNIQUE INDEX CONCURRENTLY IF NOT EXISTS idx ON t1 USING btree(col1 ASC, col2 DESC NULLS FIRST) CONTAINING (col3) DISTRIBUTE BY HASH(col1) WITH (fillfactor = 70) TABLESPACE ts1 VISIBLE WHERE col1 > 10";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(stmt, stmt2);
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
                        scroll,
                        query,
                        parsed_query,
                    } => {
                        assert_eq!(scroll, &None);
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
            assert!(matches!(t.columns[0].data_type, DataType::Integer(_)));
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
            assert_eq!(c.scrollability, CursorScrollability::Default);
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
            assert_eq!(c.scrollability, CursorScrollability::Scroll);
            assert!(!c.query.targets.is_empty());
        }
        _ => panic!("expected DeclareCursor, got {:?}", stmt),
    }
}

#[test]
fn test_declare_cursor_no_scroll() {
    let sql = "DECLARE cur NO SCROLL CURSOR FOR SELECT * FROM t";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::DeclareCursor(c) => {
            assert_eq!(c.name, "cur");
            assert_eq!(c.scrollability, CursorScrollability::NoScroll);
            assert_eq!(c.sensitivity, CursorSensitivity::Sensitive);
            assert_eq!(c.holdability, CursorHoldability::Default);
        }
        _ => panic!("expected DeclareCursor"),
    }
}

#[test]
fn test_declare_cursor_insensitive_scroll_with_hold() {
    let sql = "DECLARE cur INSENSITIVE SCROLL CURSOR WITH HOLD FOR SELECT * FROM t";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::DeclareCursor(c) => {
            assert_eq!(c.sensitivity, CursorSensitivity::Insensitive);
            assert_eq!(c.scrollability, CursorScrollability::Scroll);
            assert_eq!(c.holdability, CursorHoldability::WithHold);
        }
        _ => panic!("expected DeclareCursor"),
    }
}

#[test]
fn test_declare_cursor_without_hold() {
    let sql = "DECLARE cur CURSOR WITHOUT HOLD FOR SELECT 1";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::DeclareCursor(c) => {
            assert_eq!(c.holdability, CursorHoldability::WithoutHold);
            assert_eq!(c.scrollability, CursorScrollability::Default);
        }
        _ => panic!("expected DeclareCursor"),
    }
}

#[test]
fn test_declare_cursor_with_return_to_caller() {
    let sql = "DECLARE cur CURSOR WITH RETURN TO CALLER FOR SELECT * FROM t";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::DeclareCursor(c) => {
            assert_eq!(c.returnability, CursorReturnability::WithReturn);
            assert_eq!(c.return_to, CursorReturnTo::ToCaller);
        }
        _ => panic!("expected DeclareCursor"),
    }
}

#[test]
fn test_declare_cursor_without_return_to_client() {
    let sql = "DECLARE cur SCROLL CURSOR WITHOUT RETURN TO CLIENT FOR SELECT 1";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::DeclareCursor(c) => {
            assert_eq!(c.scrollability, CursorScrollability::Scroll);
            assert_eq!(c.returnability, CursorReturnability::WithoutReturn);
            assert_eq!(c.return_to, CursorReturnTo::ToClient);
        }
        _ => panic!("expected DeclareCursor"),
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
fn test_plpgsql_fetch_forward_count() {
    let block = parse_do_block("DO $$ BEGIN FETCH FORWARD 5 FROM cur INTO var; END $$");
    match &block.body[0] {
        PlStatement::Fetch(f) => {
            assert_eq!(f.cursor, "cur");
            assert!(matches!(
                &f.direction,
                Some(plpgsql::FetchDirection::Forward(Some(5)))
            ));
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_plpgsql_fetch_forward_bare() {
    let block = parse_do_block("DO $$ BEGIN FETCH FORWARD FROM cur INTO var; END $$");
    match &block.body[0] {
        PlStatement::Fetch(f) => {
            assert_eq!(f.cursor, "cur");
            assert!(matches!(
                &f.direction,
                Some(plpgsql::FetchDirection::Forward(None))
            ));
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_plpgsql_fetch_forward_all() {
    let block = parse_do_block("DO $$ BEGIN FETCH FORWARD ALL FROM cur INTO var; END $$");
    match &block.body[0] {
        PlStatement::Fetch(f) => {
            assert_eq!(f.cursor, "cur");
            assert!(matches!(
                &f.direction,
                Some(plpgsql::FetchDirection::ForwardAll)
            ));
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_plpgsql_fetch_absolute() {
    let block = parse_do_block("DO $$ BEGIN FETCH ABSOLUTE 10 FROM cur INTO var; END $$");
    match &block.body[0] {
        PlStatement::Fetch(f) => {
            assert_eq!(f.cursor, "cur");
            assert!(matches!(
                &f.direction,
                Some(plpgsql::FetchDirection::Absolute(10))
            ));
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_plpgsql_fetch_absolute_negative() {
    let block = parse_do_block("DO $$ BEGIN FETCH ABSOLUTE -3 FROM cur INTO var; END $$");
    match &block.body[0] {
        PlStatement::Fetch(f) => {
            assert_eq!(f.cursor, "cur");
            assert!(matches!(
                &f.direction,
                Some(plpgsql::FetchDirection::Absolute(-3))
            ));
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_plpgsql_fetch_relative() {
    let block = parse_do_block("DO $$ BEGIN FETCH RELATIVE 5 FROM cur INTO var; END $$");
    match &block.body[0] {
        PlStatement::Fetch(f) => {
            assert_eq!(f.cursor, "cur");
            assert!(matches!(
                &f.direction,
                Some(plpgsql::FetchDirection::Relative(5))
            ));
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_plpgsql_fetch_backward_count() {
    let block = parse_do_block("DO $$ BEGIN FETCH BACKWARD 3 FROM cur INTO var; END $$");
    match &block.body[0] {
        PlStatement::Fetch(f) => {
            assert_eq!(f.cursor, "cur");
            assert!(matches!(
                &f.direction,
                Some(plpgsql::FetchDirection::Backward(Some(3)))
            ));
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_plpgsql_fetch_backward_all() {
    let block = parse_do_block("DO $$ BEGIN FETCH BACKWARD ALL FROM cur INTO var; END $$");
    match &block.body[0] {
        PlStatement::Fetch(f) => {
            assert_eq!(f.cursor, "cur");
            assert!(matches!(
                &f.direction,
                Some(plpgsql::FetchDirection::BackwardAll)
            ));
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_plpgsql_move_forward_count() {
    let block = parse_do_block("DO $$ BEGIN MOVE FORWARD 5 cur; END $$");
    match &block.body[0] {
        PlStatement::Move { cursor, direction } => {
            assert_eq!(cursor, "cur");
            assert!(matches!(
                direction,
                Some(plpgsql::FetchDirection::Forward(Some(5)))
            ));
        }
        _ => panic!("expected Move"),
    }
}

#[test]
fn test_plpgsql_move_absolute() {
    let block = parse_do_block("DO $$ BEGIN MOVE ABSOLUTE 10 cur; END $$");
    match &block.body[0] {
        PlStatement::Move { cursor, direction } => {
            assert_eq!(cursor, "cur");
            assert!(matches!(
                direction,
                Some(plpgsql::FetchDirection::Absolute(10))
            ));
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
                        assert!(matches!(type_name, DataType::Integer(_)));
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
                        assert!(matches!(type_name, DataType::Custom(_, _)));
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

// === GROUPING SETS / ROLLUP / CUBE Tests ===

#[test]
fn test_grouping_sets_basic() {
    let stmt = parse_one("SELECT dept, region, SUM(salary) FROM emp GROUP BY GROUPING SETS ((dept, region), (dept), (region), ())");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.group_by.len(), 1);
            match &s.group_by[0] {
                GroupByItem::GroupingSets(sets) => {
                    assert_eq!(sets.len(), 4);
                    assert_eq!(sets[0].len(), 2); // (dept, region)
                    assert_eq!(sets[1].len(), 1); // (dept)
                    assert_eq!(sets[2].len(), 1); // (region)
                    assert_eq!(sets[3].len(), 0); // ()
                }
                other => panic!("expected GroupingSets, got {:?}", other),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_rollup() {
    let stmt =
        parse_one("SELECT year, month, SUM(amount) FROM sales GROUP BY ROLLUP (year, month)");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.group_by.len(), 1);
            match &s.group_by[0] {
                GroupByItem::Rollup(cols) => {
                    assert_eq!(cols.len(), 2);
                }
                other => panic!("expected Rollup, got {:?}", other),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_cube() {
    let stmt =
        parse_one("SELECT year, product, SUM(amount) FROM sales GROUP BY CUBE (year, product)");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.group_by.len(), 1);
            match &s.group_by[0] {
                GroupByItem::Cube(cols) => {
                    assert_eq!(cols.len(), 2);
                }
                other => panic!("expected Cube, got {:?}", other),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_mixed_group_by() {
    let stmt =
        parse_one("SELECT dept, region, SUM(salary) FROM emp GROUP BY dept, ROLLUP (region)");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.group_by.len(), 2);
            match &s.group_by[0] {
                GroupByItem::Expr(_) => {}
                other => panic!("expected Expr, got {:?}", other),
            }
            match &s.group_by[1] {
                GroupByItem::Rollup(_) => {}
                other => panic!("expected Rollup, got {:?}", other),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_group_by_plain_expr_still_works() {
    let stmt = parse_one("SELECT dept, COUNT(*) FROM emp GROUP BY dept, region");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.group_by.len(), 2);
            assert!(matches!(&s.group_by[0], GroupByItem::Expr(_)));
            assert!(matches!(&s.group_by[1], GroupByItem::Expr(_)));
        }
        _ => panic!("expected Select"),
    }
}

// === CONNECT BY Hierarchical Query Tests ===

#[test]
fn test_connect_by_simple() {
    let stmt = parse_one("SELECT * FROM emp CONNECT BY PRIOR empno = mgr");
    match stmt {
        Statement::Select(s) => {
            let cb = s.connect_by.as_ref().expect("should have CONNECT BY");
            assert!(!cb.nocycle);
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_connect_by_with_start_with() {
    let stmt = parse_one("SELECT * FROM emp START WITH mgr IS NULL CONNECT BY PRIOR empno = mgr");
    match stmt {
        Statement::Select(s) => {
            let cb = s.connect_by.as_ref().expect("should have CONNECT BY");
            assert!(cb.start_with.is_some());
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_connect_by_nocycle() {
    let stmt = parse_one("SELECT * FROM emp CONNECT BY NOCYCLE PRIOR empno = mgr");
    match stmt {
        Statement::Select(s) => {
            let cb = s.connect_by.as_ref().unwrap();
            assert!(cb.nocycle);
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_connect_by_start_with_after() {
    // GaussDB also supports START WITH after CONNECT BY
    let stmt = parse_one("SELECT * FROM emp CONNECT BY PRIOR empno = mgr START WITH mgr IS NULL");
    match stmt {
        Statement::Select(s) => {
            let cb = s.connect_by.as_ref().expect("should have CONNECT BY");
            assert!(cb.start_with.is_some());
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_prior_in_expression() {
    let stmt = parse_one("SELECT PRIOR ename, empno FROM emp CONNECT BY PRIOR empno = mgr");
    match stmt {
        Statement::Select(s) => {
            assert!(s.connect_by.is_some());
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_insert_all_unconditional() {
    let stmt = parse_one(
        "INSERT ALL INTO sales_east VALUES (1, 'a') INTO sales_west VALUES (2, 'b') SELECT * FROM source",
    );
    match stmt {
        Statement::InsertAll(ia) => {
            assert_eq!(ia.targets.len(), 2);
            assert!(ia.conditions.is_empty());
            assert!(ia.else_targets.is_empty());
        }
        _ => panic!("expected InsertAll, got {:?}", stmt),
    }
}

#[test]
fn test_insert_all_conditional() {
    let stmt = parse_one(
        "INSERT ALL WHEN salary > 10000 THEN INTO high_earners VALUES (empno, name) WHEN salary <= 10000 THEN INTO low_earners VALUES (empno, name) SELECT empno, name, salary FROM emp",
    );
    match stmt {
        Statement::InsertAll(ia) => {
            assert!(ia.targets.is_empty());
            assert_eq!(ia.conditions.len(), 2);
        }
        _ => panic!("expected InsertAll"),
    }
}

#[test]
fn test_insert_all_with_else() {
    let stmt = parse_one(
        "INSERT ALL WHEN dept = 'EAST' THEN INTO sales_east VALUES (1, 'a') ELSE INTO sales_other VALUES (3, 'c') SELECT * FROM source",
    );
    match stmt {
        Statement::InsertAll(ia) => {
            assert_eq!(ia.conditions.len(), 1);
            assert_eq!(ia.else_targets.len(), 1);
        }
        _ => panic!("expected InsertAll"),
    }
}

#[test]
fn test_insert_first() {
    let stmt = parse_one(
        "INSERT FIRST WHEN dept = 'EAST' THEN INTO sales_east VALUES (1, 'a') WHEN dept = 'WEST' THEN INTO sales_west VALUES (2, 'b') ELSE INTO sales_other VALUES (3, 'c') SELECT * FROM source",
    );
    match stmt {
        Statement::InsertFirst(if_stmt) => {
            assert_eq!(if_stmt.when_clauses.len(), 2);
            assert_eq!(if_stmt.else_targets.len(), 1);
        }
        _ => panic!("expected InsertFirst"),
    }
}

#[test]
fn test_insert_all_into_with_columns() {
    let stmt = parse_one("INSERT ALL INTO t1 (a, b) VALUES (1, 2) SELECT * FROM src");
    match stmt {
        Statement::InsertAll(ia) => {
            assert_eq!(ia.targets.len(), 1);
            assert_eq!(ia.targets[0].columns, vec!["a", "b"]);
        }
        _ => panic!("expected InsertAll"),
    }
}

#[test]
fn test_pivot() {
    let stmt = parse_one(
        "SELECT * FROM sales PIVOT (SUM(amount) FOR quarter IN ('Q1' AS q1, 'Q2' AS q2))",
    );
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.from.len(), 1);
            match &s.from[0] {
                TableRef::Pivot { source, pivot } => {
                    assert!(matches!(source.as_ref(), TableRef::Table { .. }));
                    assert_eq!(pivot.values.len(), 2);
                    assert_eq!(pivot.values[0].alias.as_deref(), Some("q1"));
                    assert_eq!(pivot.values[1].alias.as_deref(), Some("q2"));
                }
                _ => panic!("expected Pivot TableRef"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_unpivot() {
    let stmt =
        parse_one("SELECT * FROM pivoted UNPIVOT (amount FOR quarter IN (q1 AS 'Q1', q2 AS 'Q2'))");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.from.len(), 1);
            match &s.from[0] {
                TableRef::Unpivot { source, unpivot } => {
                    assert!(matches!(source.as_ref(), TableRef::Table { .. }));
                    assert_eq!(unpivot.columns.len(), 2);
                }
                _ => panic!("expected Unpivot TableRef"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_pivot_with_join() {
    let stmt = parse_one(
        "SELECT * FROM sales JOIN regions ON sales.region_id = regions.id PIVOT (SUM(amount) FOR quarter IN ('Q1', 'Q2'))",
    );
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.from.len(), 1);
            match &s.from[0] {
                TableRef::Pivot { source, .. } => {
                    assert!(matches!(source.as_ref(), TableRef::Join { .. }));
                }
                _ => panic!("expected Pivot wrapping a Join"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_pivot_without_alias() {
    let stmt = parse_one("SELECT * FROM sales PIVOT (SUM(amount) FOR quarter IN ('Q1', 'Q2'))");
    match stmt {
        Statement::Select(s) => match &s.from[0] {
            TableRef::Pivot { pivot, .. } => {
                assert_eq!(pivot.values.len(), 2);
                assert!(pivot.values[0].alias.is_none());
            }
            _ => panic!("expected Pivot"),
        },
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_unpivot_without_alias() {
    let stmt = parse_one("SELECT * FROM pivoted UNPIVOT (amount FOR quarter IN (q1, q2))");
    match stmt {
        Statement::Select(s) => match &s.from[0] {
            TableRef::Unpivot { unpivot, .. } => {
                assert_eq!(unpivot.columns.len(), 2);
                assert!(unpivot.columns[0].alias.is_none());
            }
            _ => panic!("expected Unpivot"),
        },
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_alter_table_add_partition() {
    let stmt = parse_one("ALTER TABLE sales ADD PARTITION p202601 VALUES LESS THAN ('2026-02-01')");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            match &at.actions[0] {
                AlterTableAction::AddPartition { name, values, .. } => {
                    assert_eq!(name, "p202601");
                    assert!(matches!(values, PartitionValues::LessThan(_)));
                }
                _ => panic!("expected AddPartition"),
            }
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_drop_partition() {
    let stmt = parse_one("ALTER TABLE sales DROP PARTITION p202501");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            match &at.actions[0] {
                AlterTableAction::DropPartition {
                    name, if_exists, ..
                } => {
                    assert_eq!(name, "p202501");
                    assert!(!if_exists);
                }
                _ => panic!("expected DropPartition"),
            }
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_truncate_partition() {
    let stmt = parse_one("ALTER TABLE sales TRUNCATE PARTITION p202501");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            match &at.actions[0] {
                AlterTableAction::TruncatePartition { name, cascade, .. } => {
                    assert_eq!(name, "p202501");
                    assert!(!cascade);
                }
                _ => panic!("expected TruncatePartition"),
            }
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_merge_partitions() {
    let stmt =
        parse_one("ALTER TABLE sales MERGE PARTITIONS p202501, p202502 INTO PARTITION p2025q1");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            match &at.actions[0] {
                AlterTableAction::MergePartitions {
                    names, into_name, ..
                } => {
                    assert_eq!(names.len(), 2);
                    assert_eq!(into_name, "p2025q1");
                }
                _ => panic!("expected MergePartitions"),
            }
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_split_partition() {
    let stmt = parse_one(
        "ALTER TABLE sales SPLIT PARTITION p2025q1 AT ('2025-02-01') INTO (PARTITION p202501, PARTITION p202502)",
    );
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            match &at.actions[0] {
                AlterTableAction::SplitPartition {
                    name,
                    at_value,
                    into,
                    ..
                } => {
                    assert_eq!(name, "p2025q1");
                    assert!(at_value.is_some());
                    assert_eq!(into.len(), 2);
                }
                _ => panic!("expected SplitPartition"),
            }
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_exchange_partition() {
    let stmt = parse_one("ALTER TABLE sales EXCHANGE PARTITION p202501 WITH TABLE sales_temp");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            match &at.actions[0] {
                AlterTableAction::ExchangePartition { name, table, .. } => {
                    assert_eq!(name, "p202501");
                    assert_eq!(table.join("."), "sales_temp");
                }
                _ => panic!("expected ExchangePartition"),
            }
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_rename_partition() {
    let stmt = parse_one("ALTER TABLE sales RENAME PARTITION p1 TO p2");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            match &at.actions[0] {
                AlterTableAction::RenamePartition { old_name, new_name } => {
                    assert_eq!(old_name, "p1");
                    assert_eq!(new_name, "p2");
                }
                _ => panic!("expected RenamePartition"),
            }
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_create_table_range_partition_with_values() {
    let stmt = parse_one(
        "CREATE TABLE sales (id INT, sale_date DATE, amount DECIMAL) PARTITION BY RANGE (sale_date) (PARTITION p2025 VALUES LESS THAN ('2026-01-01'), PARTITION p2026 VALUES LESS THAN ('2027-01-01'))",
    );
    match stmt {
        Statement::CreateTable(ct) => {
            assert!(ct.partition_by.is_some());
            match ct.partition_by.as_ref().unwrap() {
                PartitionClause::Range {
                    columns,
                    partitions,
                    ..
                } => {
                    assert_eq!(columns[0].join("."), "sale_date");
                    assert_eq!(partitions.len(), 2);
                    assert_eq!(partitions[0].name, "p2025");
                }
                _ => panic!("expected Range"),
            }
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_filter_clause() {
    let stmt = parse_one("SELECT COUNT(*) FILTER (WHERE status = 'active') FROM users");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, _) => match expr {
                    Expr::FunctionCall { filter, .. } => {
                        assert!(filter.is_some());
                    }
                    _ => panic!("expected FunctionCall"),
                },
                _ => panic!("expected Expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_within_group() {
    let stmt = parse_one("SELECT PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY salary) FROM emp");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, _) => match expr {
                    Expr::FunctionCall { within_group, .. } => {
                        assert_eq!(within_group.len(), 1);
                    }
                    _ => panic!("expected FunctionCall"),
                },
                _ => panic!("expected Expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_filter_with_over() {
    let stmt = parse_one(
        "SELECT COUNT(*) FILTER (WHERE status = 'active') OVER (PARTITION BY dept) FROM users",
    );
    match stmt {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(expr, _) => match expr {
                Expr::FunctionCall { filter, over, .. } => {
                    assert!(filter.is_some());
                    assert!(over.is_some());
                }
                _ => panic!("expected FunctionCall"),
            },
            _ => panic!("expected Expr target"),
        },
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_create_table_interval_partition() {
    let stmt = parse_one(
        "CREATE TABLE t (id INT, created DATE) PARTITION BY RANGE (created) INTERVAL ('1 month') (PARTITION p0 VALUES LESS THAN ('2025-01-01'))",
    );
    match stmt {
        Statement::CreateTable(ct) => match ct.partition_by.as_ref().unwrap() {
            PartitionClause::Range {
                interval,
                partitions,
                ..
            } => {
                assert!(interval.is_some());
                assert_eq!(partitions.len(), 1);
            }
            _ => panic!("expected Range"),
        },
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_list_partition() {
    let stmt = parse_one(
        "CREATE TABLE region_sales (id INT, region VARCHAR(10)) PARTITION BY LIST (region) (PARTITION p_east VALUES IN ('EAST'), PARTITION p_west VALUES IN ('WEST'))",
    );
    match stmt {
        Statement::CreateTable(ct) => match ct.partition_by.as_ref().unwrap() {
            PartitionClause::List {
                columns,
                partitions,
                ..
            } => {
                assert_eq!(columns[0].join("."), "region");
                assert_eq!(partitions.len(), 2);
                assert_eq!(partitions[0].name, "p_east");
            }
            _ => panic!("expected List"),
        },
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_hash_partition() {
    let stmt = parse_one("CREATE TABLE t (id INT) PARTITION BY HASH (id) PARTITIONS 4");
    match stmt {
        Statement::CreateTable(ct) => match ct.partition_by.as_ref().unwrap() {
            PartitionClause::Hash {
                columns,
                partitions_count,
                ..
            } => {
                assert_eq!(columns[0].join("."), "id");
                assert_eq!(*partitions_count, Some(4));
            }
            _ => panic!("expected Hash"),
        },
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_partition_no_defs() {
    let stmt = parse_one("CREATE TABLE t (id INT, dt DATE) PARTITION BY RANGE (dt)");
    match stmt {
        Statement::CreateTable(ct) => match ct.partition_by.as_ref().unwrap() {
            PartitionClause::Range { partitions, .. } => {
                assert!(partitions.is_empty());
            }
            _ => panic!("expected Range"),
        },
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_database_link() {
    let stmt = parse_one(
        "CREATE DATABASE LINK remote_db CONNECT TO user1 IDENTIFIED BY 'pass' USING 'host:port/db'",
    );
    match stmt {
        Statement::CreateDatabaseLink(dbl) => {
            assert_eq!(dbl.name, "remote_db");
            assert!(!dbl.public_link);
            assert_eq!(dbl.user.as_deref(), Some("user1"));
            assert_eq!(dbl.password.as_deref(), Some("pass"));
            assert_eq!(dbl.using_clause.as_deref(), Some("host:port/db"));
        }
        _ => panic!("expected CreateDatabaseLink, got {:?}", stmt),
    }
}

#[test]
fn test_create_public_database_link() {
    let stmt = parse_one(
        "CREATE PUBLIC DATABASE LINK remote_db CONNECT TO admin IDENTIFIED BY 'secret' USING 'oracle_host:1521/orcl'",
    );
    match stmt {
        Statement::CreateDatabaseLink(dbl) => {
            assert!(dbl.public_link);
            assert_eq!(dbl.name, "remote_db");
        }
        _ => panic!("expected CreateDatabaseLink"),
    }
}

#[test]
fn test_create_table_distribute_by_hash() {
    let stmt = parse_one(
        "CREATE TABLE t (id INT, name VARCHAR(100)) DISTRIBUTE BY HASH (id) TO GROUP group1",
    );
    match stmt {
        Statement::CreateTable(ct) => {
            assert!(ct.distribute_by.is_some());
            assert_eq!(ct.to_group.as_deref(), Some("group1"));
            match ct.distribute_by.as_ref().unwrap() {
                DistributeClause::Hash { columns } => {
                    assert_eq!(*columns, vec!["id"]);
                }
                _ => panic!("expected Hash"),
            }
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_distribute_by_replication() {
    let stmt = parse_one("CREATE TABLE t (id INT) DISTRIBUTE BY REPLICATION");
    match stmt {
        Statement::CreateTable(ct) => {
            assert!(matches!(
                ct.distribute_by.as_ref().unwrap(),
                DistributeClause::Replication
            ));
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_with_partition_and_distribute() {
    let stmt = parse_one(
        "CREATE TABLE sales (id INT, dt DATE) PARTITION BY RANGE (dt) DISTRIBUTE BY HASH (id)",
    );
    match stmt {
        Statement::CreateTable(ct) => {
            assert!(ct.partition_by.is_some());
            assert!(ct.distribute_by.is_some());
        }
        _ => panic!("expected CreateTable"),
    }
}

// ========== SUBPARTITION Tests ==========

#[test]
fn test_create_table_subpartition_range_list() {
    let stmt = parse_one(
        "CREATE TABLE t (id INT, name TEXT) PARTITION BY RANGE (id) SUBPARTITION BY LIST (name) (PARTITION p1 VALUES LESS THAN (100) (SUBPARTITION sp1 VALUES IN ('A'), SUBPARTITION sp2 VALUES IN ('B')))",
    );
    match stmt {
        Statement::CreateTable(ct) => {
            assert!(ct.partition_by.is_some());
            assert!(ct.subpartition_by.is_some());
            match ct.subpartition_by.as_ref().unwrap() {
                PartitionClause::List {
                    columns,
                    partitions,
                    ..
                } => {
                    assert_eq!(columns[0].join("."), "name");
                    assert!(partitions.is_empty()); // subpartition defs are in partition defs
                }
                other => panic!("expected List subpartition, got {:?}", other),
            }
            // Check partition defs contain subpartitions
            match ct.partition_by.as_ref().unwrap() {
                PartitionClause::Range { partitions, .. } => {
                    assert_eq!(partitions.len(), 1);
                    assert_eq!(partitions[0].name, "p1");
                    assert_eq!(partitions[0].subpartitions.len(), 2);
                    assert_eq!(partitions[0].subpartitions[0].name, "sp1");
                    assert_eq!(partitions[0].subpartitions[1].name, "sp2");
                }
                other => panic!("expected Range partition, got {:?}", other),
            }
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_subpartition_hash() {
    let stmt = parse_one(
        "CREATE TABLE t (id INT, region VARCHAR(10)) PARTITION BY LIST (region) SUBPARTITION BY HASH (id) SUBPARTITIONS 4 (PARTITION p_east VALUES IN ('EAST') (SUBPARTITION sp1, SUBPARTITION sp2, SUBPARTITION sp3, SUBPARTITION sp4))",
    );
    match stmt {
        Statement::CreateTable(ct) => {
            assert!(ct.subpartition_by.is_some());
            assert_eq!(ct.subpartitions_count, Some(4));
            match ct.subpartition_by.as_ref().unwrap() {
                PartitionClause::Hash {
                    columns,
                    partitions_count,
                    ..
                } => {
                    assert_eq!(columns[0].join("."), "id");
                    assert_eq!(*partitions_count, Some(4));
                }
                other => panic!("expected Hash subpartition, got {:?}", other),
            }
            match ct.partition_by.as_ref().unwrap() {
                PartitionClause::List { partitions, .. } => {
                    assert_eq!(partitions[0].subpartitions.len(), 4);
                }
                other => panic!("expected List partition, got {:?}", other),
            }
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_create_table_subpartition_range() {
    let stmt = parse_one(
        "CREATE TABLE t (id INT, created DATE) PARTITION BY RANGE (created) SUBPARTITION BY RANGE (id) (PARTITION p2025 VALUES LESS THAN ('2026-01-01') (SUBPARTITION sp1 VALUES LESS THAN (100), SUBPARTITION sp2 VALUES LESS THAN (200)))",
    );
    match stmt {
        Statement::CreateTable(ct) => {
            assert!(ct.subpartition_by.is_some());
            match ct.subpartition_by.as_ref().unwrap() {
                PartitionClause::Range { columns, .. } => {
                    assert_eq!(columns[0].join("."), "id");
                }
                other => panic!("expected Range subpartition, got {:?}", other),
            }
            match ct.partition_by.as_ref().unwrap() {
                PartitionClause::Range { partitions, .. } => {
                    assert_eq!(partitions[0].subpartitions.len(), 2);
                }
                other => panic!("expected Range partition, got {:?}", other),
            }
        }
        _ => panic!("expected CreateTable"),
    }
}

#[test]
fn test_alter_table_add_subpartition() {
    let stmt = parse_one("ALTER TABLE t ADD SUBPARTITION sp1 VALUES LESS THAN (50)");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            match &at.actions[0] {
                AlterTableAction::AddSubPartition { name, values, .. } => {
                    assert_eq!(name, "sp1");
                    assert!(values.is_some());
                }
                other => panic!("expected AddSubPartition, got {:?}", other),
            }
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_drop_subpartition() {
    let stmt = parse_one("ALTER TABLE t DROP SUBPARTITION sp1");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            match &at.actions[0] {
                AlterTableAction::DropSubPartition { name, if_exists } => {
                    assert_eq!(name, "sp1");
                    assert!(!if_exists);
                }
                other => panic!("expected DropSubPartition, got {:?}", other),
            }
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_drop_subpartition_if_exists() {
    let stmt = parse_one("ALTER TABLE t DROP SUBPARTITION IF EXISTS sp1");
    match stmt {
        Statement::AlterTable(at) => match &at.actions[0] {
            AlterTableAction::DropSubPartition { name, if_exists } => {
                assert_eq!(name, "sp1");
                assert!(if_exists);
            }
            other => panic!("expected DropSubPartition, got {:?}", other),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_truncate_subpartition() {
    let stmt = parse_one("ALTER TABLE t TRUNCATE SUBPARTITION sp1");
    match stmt {
        Statement::AlterTable(at) => match &at.actions[0] {
            AlterTableAction::TruncateSubPartition { name, cascade } => {
                assert_eq!(name, "sp1");
                assert!(!cascade);
            }
            other => panic!("expected TruncateSubPartition, got {:?}", other),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_truncate_subpartition_cascade() {
    let stmt = parse_one("ALTER TABLE t TRUNCATE SUBPARTITION sp1 CASCADE");
    match stmt {
        Statement::AlterTable(at) => match &at.actions[0] {
            AlterTableAction::TruncateSubPartition { name, cascade } => {
                assert_eq!(name, "sp1");
                assert!(cascade);
            }
            other => panic!("expected TruncateSubPartition, got {:?}", other),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_merge_subpartitions() {
    let stmt = parse_one("ALTER TABLE t MERGE SUBPARTITIONS sp1, sp2 INTO SUBPARTITION sp_merged");
    match stmt {
        Statement::AlterTable(at) => match &at.actions[0] {
            AlterTableAction::MergeSubPartitions { names, into_name } => {
                assert_eq!(names.len(), 2);
                assert_eq!(names[0], "sp1");
                assert_eq!(names[1], "sp2");
                assert_eq!(into_name, "sp_merged");
            }
            other => panic!("expected MergeSubPartitions, got {:?}", other),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_split_subpartition() {
    let stmt = parse_one(
        "ALTER TABLE t SPLIT SUBPARTITION sp1 AT (50) INTO (SUBPARTITION sp1a VALUES LESS THAN (50), SUBPARTITION sp1b)",
    );
    match stmt {
        Statement::AlterTable(at) => match &at.actions[0] {
            AlterTableAction::SplitSubPartition {
                name,
                at_value,
                into,
            } => {
                assert_eq!(name, "sp1");
                assert!(at_value.is_some());
                assert_eq!(into.len(), 2);
                assert_eq!(into[0].name, "sp1a");
                assert_eq!(into[1].name, "sp1b");
            }
            other => panic!("expected SplitSubPartition, got {:?}", other),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_exchange_subpartition() {
    let stmt = parse_one("ALTER TABLE t EXCHANGE SUBPARTITION sp1 WITH TABLE temp_t");
    match stmt {
        Statement::AlterTable(at) => match &at.actions[0] {
            AlterTableAction::ExchangeSubPartition { name, table } => {
                assert_eq!(name, "sp1");
                assert_eq!(table.join("."), "temp_t");
            }
            other => panic!("expected ExchangeSubPartition, got {:?}", other),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_rename_subpartition() {
    let stmt = parse_one("ALTER TABLE t RENAME SUBPARTITION sp1 TO sp1_new");
    match stmt {
        Statement::AlterTable(at) => match &at.actions[0] {
            AlterTableAction::RenameSubPartition { old_name, new_name } => {
                assert_eq!(old_name, "sp1");
                assert_eq!(new_name, "sp1_new");
            }
            other => panic!("expected RenameSubPartition, got {:?}", other),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_move_subpartition() {
    let stmt = parse_one("ALTER TABLE t MOVE SUBPARTITION sp1 TABLESPACE ts1");
    match stmt {
        Statement::AlterTable(at) => match &at.actions[0] {
            AlterTableAction::MoveSubPartition { name, tablespace } => {
                assert_eq!(name, "sp1");
                assert_eq!(tablespace, "ts1");
            }
            other => panic!("expected MoveSubPartition, got {:?}", other),
        },
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_ilm_enable_all() {
    let stmt = parse_one("ALTER TABLE t ILM ENABLE_ALL");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            assert!(matches!(
                at.actions[0],
                AlterTableAction::IlmEnableAllPolicies
            ));
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_ilm_disable_all() {
    let stmt = parse_one("ALTER TABLE t ILM DISABLE_ALL");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            assert!(matches!(
                at.actions[0],
                AlterTableAction::IlmDisableAllPolicies
            ));
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_alter_table_ilm_delete_all() {
    let stmt = parse_one("ALTER TABLE t ILM DELETE_ALL");
    match stmt {
        Statement::AlterTable(at) => {
            assert_eq!(at.actions.len(), 1);
            assert!(matches!(
                at.actions[0],
                AlterTableAction::IlmDeleteAllPolicies
            ));
        }
        _ => panic!("expected AlterTable"),
    }
}

#[test]
fn test_subpartition_format_roundtrip() {
    use crate::formatter::SqlFormatter;
    let sql = "CREATE TABLE t (id INT, name TEXT) PARTITION BY RANGE (id) SUBPARTITION BY LIST (name) (PARTITION p1 VALUES LESS THAN (100) (SUBPARTITION sp1 VALUES IN ('A'), SUBPARTITION sp2 VALUES IN ('B')))";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(stmt, stmt2);
}

#[test]
fn test_json_roundtrip_subpartition() {
    let stmt = parse_one(
        "CREATE TABLE t (id INT, name TEXT) PARTITION BY RANGE (id) SUBPARTITION BY LIST (name) (PARTITION p1 VALUES LESS THAN (100) (SUBPARTITION sp1 VALUES IN ('A'), SUBPARTITION sp2 VALUES IN ('B')))",
    );
    assert_eq!(stmt, json_roundtrip(&stmt));
}

// ========== GaussDB PARTITION Extension Tests ==========

#[test]
fn test_create_table_partition_range_columns() {
    let stmt = parse_one(
        "CREATE TABLE t1 (id INT, name VARCHAR(50)) PARTITION BY RANGE COLUMNS (name) (PARTITION p1 VALUES LESS THAN ('M'), PARTITION p2 VALUES LESS THAN ('Z'))",
    );
    match stmt {
        Statement::CreateTable(ct) => {
            let pb = ct.partition_by.as_ref().expect("expected partition_by");
            match pb {
                PartitionClause::Range {
                    columns,
                    is_columns,
                    partitions,
                    ..
                } => {
                    assert_eq!(*is_columns, true);
                    assert_eq!(columns, &vec![vec!["name".to_string()]]);
                    assert_eq!(partitions.len(), 2);
                }
                other => panic!("expected Range, got {:?}", other),
            }
        }
        other => panic!("expected CreateTable, got {:?}", other),
    }
}

#[test]
fn test_create_table_partition_list_columns() {
    let stmt = parse_one(
        "CREATE TABLE t2 (id INT, region VARCHAR(10)) PARTITION BY LIST COLUMNS (region) (PARTITION p_east VALUES IN ('east'), PARTITION p_west VALUES IN ('west'))",
    );
    match stmt {
        Statement::CreateTable(ct) => {
            let pb = ct.partition_by.as_ref().expect("expected partition_by");
            match pb {
                PartitionClause::List {
                    columns,
                    is_columns,
                    partitions,
                } => {
                    assert_eq!(*is_columns, true);
                    assert_eq!(columns, &vec![vec!["region".to_string()]]);
                    assert_eq!(partitions.len(), 2);
                }
                other => panic!("expected List, got {:?}", other),
            }
        }
        other => panic!("expected CreateTable, got {:?}", other),
    }
}

#[test]
fn test_create_table_partition_range_with_partitions_count() {
    let stmt = parse_one(
        "CREATE TABLE t1 (id INT, dt DATE) PARTITION BY RANGE (dt) PARTITIONS 10 (PARTITION p1 VALUES LESS THAN ('2025-01-01'))",
    );
    match stmt {
        Statement::CreateTable(ct) => {
            let pb = ct.partition_by.as_ref().expect("expected partition_by");
            match pb {
                PartitionClause::Range {
                    partitions_count, ..
                } => {
                    assert_eq!(*partitions_count, Some(10));
                }
                other => panic!("expected Range, got {:?}", other),
            }
        }
        other => panic!("expected CreateTable, got {:?}", other),
    }
}

#[test]
fn test_create_table_partition_start_end_every() {
    let stmt = parse_one(
        "CREATE TABLE t1 (id INT, dt DATE) PARTITION BY RANGE (dt) (PARTITION p1 START('2020-01-01') END('2020-06-01') EVERY('1 month'), PARTITION p2 START('2020-06-01') END('2021-01-01'))",
    );
    match stmt {
        Statement::CreateTable(ct) => {
            let pb = ct.partition_by.as_ref().expect("expected partition_by");
            match pb {
                PartitionClause::Range { partitions, .. } => {
                    assert_eq!(partitions.len(), 2);
                    match &partitions[0].values {
                        Some(PartitionValues::StartEnd { start, end, every }) => {
                            assert!(every.is_some());
                        }
                        other => panic!("expected StartEnd with every, got {:?}", other),
                    }
                    match &partitions[1].values {
                        Some(PartitionValues::StartEnd { every, .. }) => {
                            assert!(every.is_none());
                        }
                        other => panic!("expected StartEnd without every, got {:?}", other),
                    }
                }
                other => panic!("expected Range, got {:?}", other),
            }
        }
        other => panic!("expected CreateTable, got {:?}", other),
    }
}

#[test]
fn test_create_table_partition_list_default() {
    let stmt = parse_one(
        "CREATE TABLE t1 (id INT, region VARCHAR(10)) PARTITION BY LIST (region) (PARTITION p_east VALUES IN ('east'), PARTITION p_default VALUES (DEFAULT))",
    );
    match stmt {
        Statement::CreateTable(ct) => {
            let pb = ct.partition_by.as_ref().expect("expected partition_by");
            match pb {
                PartitionClause::List { partitions, .. } => {
                    assert_eq!(partitions.len(), 2);
                    match &partitions[1].values {
                        Some(PartitionValues::InValues(vals)) => {
                            assert_eq!(vals.len(), 1);
                            assert_eq!(vals[0], Expr::Default);
                        }
                        other => panic!("expected InValues with DEFAULT, got {:?}", other),
                    }
                }
                other => panic!("expected List, got {:?}", other),
            }
        }
        other => panic!("expected CreateTable, got {:?}", other),
    }
}

#[test]
fn test_create_table_partition_values_without_in() {
    let stmt = parse_one(
        "CREATE TABLE t1 (id INT, region VARCHAR(10)) PARTITION BY LIST (region) (PARTITION p_east VALUES ('east'), PARTITION p_west VALUES ('west'))",
    );
    match stmt {
        Statement::CreateTable(ct) => {
            let pb = ct.partition_by.as_ref().expect("expected partition_by");
            match pb {
                PartitionClause::List { partitions, .. } => {
                    assert_eq!(partitions.len(), 2);
                    match &partitions[0].values {
                        Some(PartitionValues::InValues(vals)) => {
                            assert_eq!(vals.len(), 1);
                        }
                        other => panic!("expected InValues, got {:?}", other),
                    }
                }
                other => panic!("expected List, got {:?}", other),
            }
        }
        other => panic!("expected CreateTable, got {:?}", other),
    }
}

#[test]
fn test_create_table_enable_row_movement() {
    let stmt = parse_one("CREATE TABLE t1 (id INT) ENABLE ROW MOVEMENT");
    match stmt {
        Statement::CreateTable(ct) => {
            assert_eq!(ct.row_movement, Some(true));
        }
        other => panic!("expected CreateTable, got {:?}", other),
    }
}

#[test]
fn test_create_table_disable_row_movement() {
    let stmt = parse_one("CREATE TABLE t2 (id INT) DISABLE ROW MOVEMENT");
    match stmt {
        Statement::CreateTable(ct) => {
            assert_eq!(ct.row_movement, Some(false));
        }
        other => panic!("expected CreateTable, got {:?}", other),
    }
}

#[test]
fn test_create_table_enable_row_movement_roundtrip() {
    let sql = "CREATE TABLE t1 (id INTEGER) ENABLE ROW MOVEMENT";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(stmt, stmt2);
}

#[test]
fn test_create_table_disable_row_movement_roundtrip() {
    let sql = "CREATE TABLE t2 (id INTEGER) DISABLE ROW MOVEMENT";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(stmt, stmt2);
}

#[test]
fn test_create_table_range_columns_roundtrip() {
    let sql = "CREATE TABLE t1 (id INTEGER, name VARCHAR(50)) PARTITION BY RANGE COLUMNS (name) (PARTITION p1 VALUES LESS THAN ('M'))";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(stmt, stmt2);
}

#[test]
fn test_create_table_list_columns_roundtrip() {
    let sql = "CREATE TABLE t2 (id INTEGER, region VARCHAR(10)) PARTITION BY LIST COLUMNS (region) (PARTITION p_east VALUES IN ('east'))";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(stmt, stmt2);
}

#[test]
fn test_create_table_start_end_every_roundtrip() {
    let sql = "CREATE TABLE t1 (id INTEGER, dt DATE) PARTITION BY RANGE (dt) (PARTITION p1 START('2020-01-01') END('2020-06-01') EVERY('1 month'))";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(stmt, stmt2);
}

#[test]
fn test_create_table_partition_list_default_roundtrip() {
    let sql = "CREATE TABLE t1 (id INTEGER, region VARCHAR(10)) PARTITION BY LIST (region) (PARTITION p_east VALUES IN ('east'), PARTITION p_default VALUES (DEFAULT))";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(stmt, stmt2);
}

#[test]
fn test_create_table_partition_range_partitions_count_roundtrip() {
    let sql = "CREATE TABLE t1 (id INTEGER, dt DATE) PARTITION BY RANGE (dt) PARTITIONS 10 (PARTITION p1 VALUES LESS THAN ('2025-01-01'))";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(stmt, stmt2);
}

#[test]
fn test_create_table_gaussdb_json_roundtrip() {
    let sql = "CREATE TABLE t1 (id INTEGER, dt DATE) PARTITION BY RANGE COLUMNS (dt) PARTITIONS 4 ENABLE ROW MOVEMENT (PARTITION p1 START('2020-01-01') END('2020-06-01') EVERY('1 month'))";
    let stmt = parse_one(sql);
    assert_eq!(stmt, json_roundtrip(&stmt));
}

// ========== XML Function Tests ==========

#[test]
fn test_xmlelement_simple() {
    let stmt = parse_one("SELECT xmlelement(name foo)");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(Expr::XmlElement { name, .. }, _) => {
                    assert_eq!(name.as_deref(), Some("foo"));
                }
                _ => panic!("expected XmlElement"),
            }
        }
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_xmlelement_with_attributes() {
    let stmt = parse_one("SELECT xmlelement(name foo, xmlattributes('bar' as baz))");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(
                    Expr::XmlElement {
                        attributes: Some(attrs),
                        ..
                    },
                    _,
                ) => {
                    assert_eq!(attrs.items.len(), 1);
                    assert_eq!(attrs.items[0].name.as_deref(), Some("baz"));
                }
                _ => panic!("expected XmlElement with attributes"),
            }
        }
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_xmlelement_noentityescaping_bug() {
    let sql = r#"SELECT xmlelement(" entityescaping <> ", xmlattributes(noentityescaping 'entityescaping<>' " entityescaping <> "))"#;
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(
                Expr::XmlElement {
                    attributes: Some(attrs),
                    ..
                },
                _,
            ) => {
                assert_eq!(attrs.entity_escaping, Some(false));
                assert_eq!(attrs.items.len(), 1);
            }
            _ => panic!("expected XmlElement"),
        },
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_xmlelement_entityescaping() {
    let sql = r#"SELECT xmlelement(entityescaping "entityescaping<>", 'content')"#;
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(
                Expr::XmlElement {
                    entity_escaping: Some(true),
                    name,
                    content,
                    ..
                },
                _,
            ) => {
                assert_eq!(name.as_deref(), Some("entityescaping<>"));
                assert_eq!(content.len(), 1);
            }
            _ => panic!("expected XmlElement"),
        },
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_xmlconcat() {
    let stmts = parse("SELECT xmlconcat(x, y, z)");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(Expr::XmlConcat(exprs), _) => {
                assert_eq!(exprs.len(), 3);
            }
            _ => panic!("expected XmlConcat"),
        },
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_xmlforest() {
    let stmts = parse("SELECT xmlforest('abc' AS foo, 123 AS bar)");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(Expr::XmlForest(items), _) => {
                assert_eq!(items.len(), 2);
                assert_eq!(items[0].alias.as_deref(), Some("foo"));
                assert_eq!(items[1].alias.as_deref(), Some("bar"));
            }
            _ => panic!("expected XmlForest"),
        },
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_xmlparse_document() {
    let stmts = parse("SELECT xmlparse(document '<foo>bar</foo>')");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(
                Expr::XmlParse {
                    option: XmlOption::Document,
                    wellformed: false,
                    ..
                },
                _,
            ) => {}
            _ => panic!("expected XmlParse"),
        },
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_xmlparse_content_wellformed() {
    let stmts = parse("SELECT xmlparse(content '<foo>bar</foo>' wellformed)");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(
                Expr::XmlParse {
                    option: XmlOption::Content,
                    wellformed: true,
                    ..
                },
                _,
            ) => {}
            _ => panic!("expected XmlParse"),
        },
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_xmlpi() {
    let stmts = parse("SELECT xmlpi(name php, 'echo hello')");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(
                Expr::XmlPi {
                    name: Some(n),
                    content: Some(_),
                },
                _,
            ) => {
                assert_eq!(n, "php");
            }
            _ => panic!("expected XmlPi"),
        },
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_xmlpi_no_content() {
    let stmts = parse("SELECT xmlpi(name php)");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(Expr::XmlPi { content: None, .. }, _) => {}
            _ => panic!("expected XmlPi"),
        },
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_xmlroot() {
    let stmts = parse("SELECT xmlroot(x, version '1.0', standalone yes)");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(
                Expr::XmlRoot {
                    version: Some(_),
                    standalone: Some(Some(true)),
                    ..
                },
                _,
            ) => {}
            _ => panic!("expected XmlRoot"),
        },
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_xmlserialize() {
    let stmts = parse("SELECT xmlserialize(content x AS text)");
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(
                Expr::XmlSerialize {
                    option: XmlOption::Content,
                    type_name: _,
                    ..
                },
                _,
            ) => {}
            _ => panic!("expected XmlSerialize"),
        },
        _ => panic!("expected SELECT"),
    }
}

// ── Hint Round-Trip Tests ──

#[test]
fn test_insert_hint_roundtrip() {
    let sql = "INSERT /*+ set(enable_nestloop off) */ INTO t1 (c1) VALUES (1)";
    let stmts = parse(sql);
    let formatter = SqlFormatter::new();
    let output = formatter.format_statement(&stmts[0]);
    assert!(
        output.contains("/*+"),
        "INSERT hint should be preserved in formatter output: {}",
        output
    );
}

#[test]
fn test_update_hint_roundtrip() {
    let sql = "UPDATE /*+ nestloop(t1) */ t1 SET c1 = 1 WHERE c1 > 0";
    let stmts = parse(sql);
    let formatter = SqlFormatter::new();
    let output = formatter.format_statement(&stmts[0]);
    assert!(
        output.contains("/*+"),
        "UPDATE hint should be preserved in formatter output: {}",
        output
    );
}

#[test]
fn test_delete_hint_roundtrip() {
    let sql = "DELETE /*+ indexscan(t1 idx_c1) */ FROM t1 WHERE c1 > 0";
    let stmts = parse(sql);
    let formatter = SqlFormatter::new();
    let output = formatter.format_statement(&stmts[0]);
    assert!(
        output.contains("/*+"),
        "DELETE hint should be preserved in formatter output: {}",
        output
    );
}

#[test]
fn test_merge_hint_roundtrip() {
    let sql = "MERGE /*+ leading(t1 t2) */ INTO t1 USING t2 ON t1.id = t2.id WHEN MATCHED THEN UPDATE SET t1.val = t2.val";
    let stmts = parse(sql);
    let formatter = SqlFormatter::new();
    let output = formatter.format_statement(&stmts[0]);
    assert!(
        output.contains("/*+"),
        "MERGE hint should be preserved in formatter output: {}",
        output
    );
}

#[test]
fn test_select_hint_parsed() {
    let sql = "SELECT /*+ tablescan(t1) */ * FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    match &stmts[0] {
        Statement::Select(s) => assert_eq!(s.hints, vec!["tablescan(t1)"]),
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_select_multi_hint() {
    let sql = "SELECT /*+ tablescan(t1) leading(t1 t2) */ * FROM t1, t2 WHERE t1.id = t2.id";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    match &stmts[0] {
        Statement::Select(s) => {
            assert_eq!(s.hints.len(), 1);
            assert!(s.hints[0].contains("tablescan(t1)"));
            assert!(s.hints[0].contains("leading(t1 t2)"));
        }
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_hint_after_select_keyword() {
    let sql = "SELECT /*+ hashjoin(t1 t2) */ * FROM t1 JOIN t2 ON t1.id = t2.id";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    match &stmts[0] {
        Statement::Select(s) => {
            assert_eq!(s.hints.len(), 1);
            assert!(s.hints[0].contains("hashjoin"));
        }
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_hint_with_queryblock() {
    let sql = "SELECT /*+ tablescan(@sel$1 t1) */ * FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    match &stmts[0] {
        Statement::Select(s) => assert_eq!(s.hints, vec!["tablescan(@sel$1 t1)"]),
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_hint_set_guc() {
    let sql = "SELECT /*+ set(enable_hashjoin off) */ * FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
}

#[test]
fn test_hint_unknown_warning() {
    let sql = "SELECT /*+ nonexistent_hint(t1) */ * FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let warnings: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::Warning { .. }))
        .collect();
    assert!(!warnings.is_empty(), "Should warn about unknown hint");
    assert!(warnings[0].to_string().contains("Unknown hint"));
}

#[test]
fn test_hint_set_missing_value_warning() {
    let sql = "SELECT /*+ set(enable_hashjoin) */ * FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let warnings: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::Warning { .. }))
        .collect();
    assert!(!warnings.is_empty(), "Should warn about malformed set hint");
}

#[test]
fn test_hint_json_roundtrip() {
    let sql = "SELECT /*+ tablescan(t1) leading(t1 t2) */ * FROM t1, t2 WHERE t1.id = t2.id";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    let json = serde_json::to_string(&stmts).unwrap();
    let restored: Vec<Statement> = serde_json::from_str(&json).unwrap();
    let formatter = SqlFormatter::new();
    let output = formatter.format_statement(&restored[0]);
    assert!(
        output.contains("tablescan(t1)"),
        "Hint should survive JSON round-trip"
    );
    assert!(
        output.contains("leading(t1 t2)"),
        "Hint should survive JSON round-trip"
    );
}

#[test]
fn test_func_coalesce_warning() {
    let sql = "SELECT coalesce(a) FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let warnings: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::Warning { .. }))
        .collect();
    assert!(!warnings.is_empty(), "COALESCE with 1 arg should warn");
}

#[test]
fn test_func_window_no_over_warning() {
    let sql = "SELECT row_number() FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let warnings: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::Warning { .. }))
        .collect();
    assert!(!warnings.is_empty(), "row_number without OVER should warn");
}

#[test]
fn test_func_window_with_over_ok() {
    let sql = "SELECT row_number() OVER (ORDER BY a) FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let warnings: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::Warning { .. }))
        .collect();
    assert!(warnings.is_empty(), "row_number with OVER should not warn");
}

#[test]
fn test_into_prefix_alias_standalone_error() {
    let sql = "SELECT to_number(p_in_checkbalance) INTOAAAA v_in_checkbalance FROM sys_dummy;";
    let (_, errors) = Parser::parse_sql(sql);
    let errors: Vec<_> = errors
        .iter()
        .filter(|e| !matches!(e, ParserError::Warning { .. }))
        .collect();
    assert!(!errors.is_empty(), "INTOAAAA should produce a parse error");
    let msg = format!("{}", errors[0]);
    assert!(
        msg.contains("INTOAAAA"),
        "error should point to INTOAAAA: {}",
        msg
    );
}

#[test]
fn test_into_prefix_alias_no_false_positive() {
    let sql = "SELECT id AS intx FROM t1";
    let (_, errors) = Parser::parse_sql(sql);
    let warnings: Vec<_> = errors
        .iter()
        .filter(|e| matches!(e, ParserError::Warning { .. }))
        .collect();
    assert!(
        warnings.is_empty(),
        "intx should not trigger INTO-prefix warning"
    );
}

#[test]
fn test_into_prefix_alias_pl_incomplete_error() {
    let sql = r#"CREATE OR REPLACE PACKAGE BODY test_pkg IS
PROCEDURE p1 IS
  v_balance NUMBER;
BEGIN
  SELECT to_number(p_in_checkbalance) INTOAAAA v_in_checkbalance FROM sys_dummy;
END;
END;"#;
    let (_, errors) = Parser::parse_sql(sql);
    let errors: Vec<_> = errors
        .iter()
        .filter(|e| !matches!(e, ParserError::Warning { .. }))
        .collect();
    assert!(
        !errors.is_empty(),
        "PL context should report error for INTOAAAA"
    );
    let msg = format!("{}", errors[0]);
    assert!(
        msg.contains("INTOAAAA"),
        "error should mention INTOAAAA: {}",
        msg
    );
}

#[test]
fn test_on_conflict_do_nothing() {
    let stmt = parse_one("INSERT INTO t VALUES (1) ON CONFLICT DO NOTHING");
    match stmt {
        Statement::Insert(ins) => {
            let oc = ins.on_conflict.expect("expected on_conflict");
            assert!(matches!(oc, OnConflictAction::Nothing { target: None }));
        }
        _ => panic!("expected Insert"),
    }
}

#[test]
fn test_on_conflict_columns() {
    let stmt = parse_one("INSERT INTO t VALUES (1) ON CONFLICT (id) DO UPDATE SET name = 'x'");
    match stmt {
        Statement::Insert(ins) => {
            let oc = ins.on_conflict.expect("expected on_conflict");
            match oc {
                OnConflictAction::Update {
                    target,
                    assignments,
                    ..
                } => {
                    assert!(
                        matches!(target, Some(OnConflictTarget::Columns(cols)) if cols == vec!["id"])
                    );
                    assert_eq!(assignments.len(), 1);
                }
                _ => panic!("expected Update action"),
            }
        }
        _ => panic!("expected Insert"),
    }
}

#[test]
fn test_on_conflict_on_constraint() {
    let stmt = parse_one("INSERT INTO t VALUES (1) ON CONFLICT ON CONSTRAINT pk DO NOTHING");
    match stmt {
        Statement::Insert(ins) => {
            let oc = ins.on_conflict.expect("expected on_conflict");
            match oc {
                OnConflictAction::Nothing { target } => {
                    assert!(
                        matches!(target, Some(OnConflictTarget::OnConstraint(ref name)) if name == "pk"),
                        "expected OnConstraint(pk), got {:?}",
                        target
                    );
                }
                other => panic!("expected Nothing, got {:?}", other),
            }
        }
        _ => panic!("expected Insert"),
    }
}

// ── Reserved / Non-reserved keyword as identifier tests ──

#[test]
fn test_reserved_keyword_as_table_name_error() {
    let sql = "SELECT * FROM select";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty(), "Should still produce AST (soft error)");
    let reserved_errors: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        !reserved_errors.is_empty(),
        "Reserved keyword 'select' used as table name should error"
    );
    assert!(reserved_errors[0].to_string().contains("select"));
}

#[test]
fn test_reserved_keyword_as_column_name_error() {
    let sql = "SELECT where FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty(), "Should still produce AST (soft error)");
    let reserved_errors: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        !reserved_errors.is_empty(),
        "Reserved keyword 'where' used as column name should error"
    );
}

#[test]
fn test_nonreserved_keyword_as_table_name_no_warning() {
    let sql = "SELECT * FROM action";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "Non-reserved keyword 'action' as table name should not trigger any warning"
    );
}

#[test]
fn test_nonreserved_keyword_as_column_name_no_warning() {
    let sql = "SELECT commit FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "Non-reserved keyword 'commit' as column name should not trigger any warning"
    );
}

#[test]
fn test_colname_keyword_as_identifier_no_warning() {
    let sql = "SELECT bigint FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "ColName keyword 'bigint' as identifier should not trigger any warning"
    );
}

#[test]
fn test_quoted_identifier_no_warning() {
    let sql = "SELECT * FROM \"select\"";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "Quoted identifier should not trigger keyword warnings"
    );
}

#[test]
fn test_normal_identifier_no_warning() {
    let sql = "SELECT my_col FROM my_table";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "Normal identifiers should not trigger keyword warnings"
    );
}

#[test]
fn test_create_table_quoted_reserved_no_error() {
    let sql = "CREATE TABLE t1 (\"select\" VARCHAR(10), \"from\" INT)";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "Quoted identifiers in CREATE TABLE should not trigger errors"
    );
}

// ── Keyword category guard tests (verified against kwlist.h) ──

use crate::token::keyword::{Keyword, KeywordCategory};

/// Helper: assert a keyword's category matches expectation.
fn assert_keyword_category(kw: Keyword, expected: KeywordCategory, label: &str) {
    assert_eq!(
        kw.category(),
        expected,
        "keyword \"{}\" ({}) should be {:?}, got {:?}",
        kw.as_str(),
        label,
        expected,
        kw.category()
    );
}

#[test]
fn test_guard_reserved_keywords_from_kwlist() {
    // Spot-check all RESERVED_KEYWORD entries from kwlist.h that have been
    // historically problematic or are easy to misclassify.
    let reserved: Vec<(Keyword, &str)> = vec![
        (Keyword::ALL, "all"),
        (Keyword::AND, "and"),
        (Keyword::ARRAY, "array"),
        (Keyword::AS, "as"),
        (Keyword::ASC, "asc"),
        (Keyword::ASYMMETRIC, "asymmetric"),
        (Keyword::AUTHID, "authid"),
        (Keyword::BOTH, "both"),
        (Keyword::CASE, "case"),
        (Keyword::CAST, "cast"),
        (Keyword::CHECK, "check"),
        (Keyword::COLLATE, "collate"),
        (Keyword::COLUMN, "column"),
        (Keyword::CONSTRAINT, "constraint"),
        (Keyword::CREATE, "create"),
        (Keyword::CURRENT_CATALOG, "current_catalog"),
        (Keyword::CURRENT_DATE, "current_date"),
        (Keyword::CURRENT_ROLE, "current_role"),
        (Keyword::CURRENT_TIME, "current_time"),
        (Keyword::CURRENT_TIMESTAMP, "current_timestamp"),
        (Keyword::CURRENT_USER, "current_user"),
        (Keyword::DEFAULT, "default"),
        (Keyword::DEFERRABLE, "deferrable"),
        (Keyword::DESC, "desc"),
        (Keyword::DISTINCT, "distinct"),
        (Keyword::DO, "do"),
        (Keyword::ELSE, "else"),
        (Keyword::END_P, "end"),
        (Keyword::EXCEPT, "except"),
        (Keyword::FALSE_P, "false"),
        (Keyword::FETCH, "fetch"),
        (Keyword::FOR, "for"),
        (Keyword::FOREIGN, "foreign"),
        (Keyword::FROM, "from"),
        (Keyword::GRANT, "grant"),
        (Keyword::GROUP_P, "group"),
        (Keyword::HAVING, "having"),
        (Keyword::IN_P, "in"),
        (Keyword::INITIALLY, "initially"),
        (Keyword::INTERSECT, "intersect"),
        (Keyword::INTO, "into"),
        (Keyword::IS, "is"),
        (Keyword::LEADING, "leading"),
        (Keyword::LESS, "less"),
        (Keyword::LIMIT, "limit"),
        (Keyword::LOCALTIME, "localtime"),
        (Keyword::LOCALTIMESTAMP, "localtimestamp"),
        // MAXVALUE was previously misclassified as Unreserved — guard it
        (Keyword::MAXVALUE, "maxvalue"),
        (Keyword::MINUS_P, "minus"),
        (Keyword::MODIFY_P, "modify"),
        (Keyword::NOCYCLE, "nocycle"),
        (Keyword::NOT, "not"),
        (Keyword::NULL_P, "null"),
        (Keyword::OFFSET, "offset"),
        (Keyword::ON, "on"),
        (Keyword::ONLY, "only"),
        (Keyword::OR, "or"),
        (Keyword::ORDER, "order"),
        (Keyword::PERFORMANCE, "performance"),
        (Keyword::PLACING, "placing"),
        (Keyword::PRIMARY, "primary"),
        (Keyword::PROCEDURE, "procedure"),
        (Keyword::REFERENCES, "references"),
        (Keyword::REJECT_P, "reject"),
        (Keyword::RETURNING, "returning"),
        // ROWNUM was in user's test case — guard it
        (Keyword::ROWNUM, "rownum"),
        (Keyword::SELECT, "select"),
        (Keyword::SESSION_USER, "session_user"),
        (Keyword::SHRINK, "shrink"),
        (Keyword::SOME, "some"),
        (Keyword::SYMMETRIC, "symmetric"),
        // SYSDATE was in user's test case — guard it
        (Keyword::SYSDATE, "sysdate"),
        (Keyword::TABLE, "table"),
        (Keyword::THEN, "then"),
        (Keyword::TO, "to"),
        (Keyword::TRAILING, "trailing"),
        (Keyword::TRUE_P, "true"),
        (Keyword::UNION, "union"),
        (Keyword::UNIQUE, "unique"),
        (Keyword::USER, "user"),
        (Keyword::USING, "using"),
        (Keyword::VARIADIC, "variadic"),
        (Keyword::VERIFY, "verify"),
        (Keyword::WHEN, "when"),
        (Keyword::WHERE, "where"),
        (Keyword::WINDOW, "window"),
        (Keyword::WITH, "with"),
    ];
    for (kw, label) in &reserved {
        assert_keyword_category(*kw, KeywordCategory::Reserved, label);
    }
}

#[test]
fn test_guard_colname_keywords_from_kwlist() {
    let colname: Vec<(Keyword, &str)> = vec![
        (Keyword::BETWEEN, "between"),
        (Keyword::BIGINT, "bigint"),
        (Keyword::BIT, "bit"),
        (Keyword::BOOLEAN_P, "boolean"),
        (Keyword::CHAR_P, "char"),
        (Keyword::COALESCE, "coalesce"),
        (Keyword::DATE_P, "date"),
        (Keyword::DECIMAL_P, "decimal"),
        (Keyword::DECODE, "decode"),
        (Keyword::EXISTS, "exists"),
        (Keyword::EXTRACT, "extract"),
        (Keyword::FLOAT_P, "float"),
        (Keyword::GREATEST, "greatest"),
        (Keyword::INTEGER, "integer"),
        (Keyword::INTERVAL, "interval"),
        (Keyword::LEAST, "least"),
        // NAME was in user's test case — guard it (UNRESERVED, not COL_NAME)
        // NVL was in user's test case — guard it
        (Keyword::NVL, "nvl"),
        (Keyword::NUMERIC, "numeric"),
        (Keyword::REAL, "real"),
        (Keyword::ROW, "row"),
        (Keyword::SMALLINT, "smallint"),
        (Keyword::SUBSTRING, "substring"),
        (Keyword::TIME, "time"),
        (Keyword::TIMESTAMP, "timestamp"),
        (Keyword::TREAT, "treat"),
        (Keyword::TRIM, "trim"),
        (Keyword::VALUES, "values"),
        (Keyword::VARCHAR, "varchar"),
    ];
    for (kw, label) in &colname {
        assert_keyword_category(*kw, KeywordCategory::ColName, label);
    }
}

#[test]
fn test_guard_unreserved_keywords_from_kwlist() {
    let unreserved: Vec<(Keyword, &str)> = vec![
        (Keyword::ABORT_P, "abort"),
        (Keyword::ACTION, "action"),
        (Keyword::COMMIT, "commit"),
        (Keyword::FUNCTION, "function"),
        (Keyword::INDEX, "index"),
        (Keyword::INSERT, "insert"),
        (Keyword::MERGE, "merge"),
        // NAME was in user's test case — guard it as UNRESERVED
        (Keyword::NAME_P, "name"),
        (Keyword::SCHEMA, "schema"),
        (Keyword::SET, "set"),
        (Keyword::UPDATE, "update"),
        (Keyword::VACUUM, "vacuum"),
    ];
    for (kw, label) in &unreserved {
        assert_keyword_category(*kw, KeywordCategory::Unreserved, label);
    }
}

#[test]
fn test_guard_type_func_name_keywords_from_kwlist() {
    let typefunc: Vec<(Keyword, &str)> = vec![
        (Keyword::AUTHORIZATION, "authorization"),
        (Keyword::CROSS, "cross"),
        (Keyword::FULL, "full"),
        (Keyword::ILIKE, "ilike"),
        (Keyword::INNER_P, "inner"),
        (Keyword::JOIN, "join"),
        (Keyword::LEFT, "left"),
        (Keyword::LIKE, "like"),
        (Keyword::NATURAL, "natural"),
        (Keyword::OUTER_P, "outer"),
        (Keyword::OVERLAPS, "overlaps"),
        (Keyword::RIGHT, "right"),
        (Keyword::SIMILAR, "similar"),
        (Keyword::VERBOSE, "verbose"),
    ];
    for (kw, label) in &typefunc {
        assert_keyword_category(*kw, KeywordCategory::TypeFuncName, label);
    }
}

/// Regression guard: user's original test case should produce 0 errors + 0 warnings.
/// sysdate/rownum are built-in expressions (RESERVED but valid), nvl is a function call
/// (COL_NAME keyword), name is an alias (UNRESERVED keyword) — all are legitimate uses.
#[test]
fn test_user_reported_sql_no_errors_no_warnings() {
    let sql =
        r#"select c1 as name, to_char(sysdate,"yyyymmdd"), nvl(c3,"01") from t where rownum=1"#;
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty(), "Should produce valid AST");

    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "User's SQL should produce 0 keyword errors, got: {:?}",
        keyword_issues
    );
}

#[test]
fn test_sysdate_as_expression_no_error() {
    let sql = "SELECT sysdate FROM dual";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let reserved_errors: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        reserved_errors.is_empty(),
        "SYSDATE as expression should not produce error, got: {:?}",
        reserved_errors
    );
}

#[test]
fn test_rownum_in_where_no_error() {
    let sql = "SELECT * FROM t WHERE rownum <= 10";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let reserved_errors: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        reserved_errors.is_empty(),
        "ROWNUM in WHERE should not produce error, got: {:?}",
        reserved_errors
    );
}

#[test]
fn test_current_date_as_expression_no_error() {
    let sql = "SELECT current_date";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "CURRENT_DATE as expression should not produce error"
    );
}

#[test]
fn test_current_timestamp_with_precision_no_error() {
    let sql = "SELECT current_timestamp(6)";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "CURRENT_TIMESTAMP(6) should not produce error"
    );
}

#[test]
fn test_nvl_function_call_no_warning() {
    let sql = "SELECT nvl(c1, 0) FROM t";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "nvl() function call should not produce any keyword warning"
    );
}

#[test]
fn test_name_as_alias_no_warning() {
    let sql = "SELECT c1 AS name FROM t";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "name as alias should not produce any keyword warning"
    );
}

// ── Keyword classification tests: value, name, rule, null, minus ──
//
// Summary:
//   value  → Keyword::VALUE_P  → Unreserved  (keyword ✓, reserved ✗)
//   name   → Keyword::NAME_P   → Unreserved  (keyword ✓, reserved ✗)
//   rule   → Keyword::RULE     → Unreserved  (keyword ✓, reserved ✗)
//   null   → Keyword::NULL_P   → Reserved    (keyword ✓, reserved ✓)
//   minus  → Keyword::MINUS_P  → Reserved    (keyword ✓, reserved ✓)

// === Category guard tests ===

#[test]
fn test_value_keyword_is_unreserved() {
    assert_keyword_category(Keyword::VALUE_P, KeywordCategory::Unreserved, "value");
}

#[test]
fn test_name_keyword_is_unreserved() {
    assert_keyword_category(Keyword::NAME_P, KeywordCategory::Unreserved, "name");
}

#[test]
fn test_rule_keyword_is_unreserved() {
    assert_keyword_category(Keyword::RULE, KeywordCategory::Unreserved, "rule");
}

#[test]
fn test_null_keyword_is_reserved() {
    assert_keyword_category(Keyword::NULL_P, KeywordCategory::Reserved, "null");
}

#[test]
fn test_minus_keyword_is_reserved() {
    assert_keyword_category(Keyword::MINUS_P, KeywordCategory::Reserved, "minus");
}

// === Tokenizer recognition tests ===

#[test]
fn test_tokenize_value_as_keyword() {
    let tokens = Tokenizer::new("value").tokenize().unwrap();
    assert!(
        matches!(&tokens[0].token, Token::Keyword(Keyword::VALUE_P)),
        "token 'value' should be recognized as VALUE_P keyword"
    );
}

#[test]
fn test_tokenize_name_as_keyword() {
    let tokens = Tokenizer::new("name").tokenize().unwrap();
    assert!(
        matches!(&tokens[0].token, Token::Keyword(Keyword::NAME_P)),
        "token 'name' should be recognized as NAME_P keyword"
    );
}

#[test]
fn test_tokenize_rule_as_keyword() {
    let tokens = Tokenizer::new("rule").tokenize().unwrap();
    assert!(
        matches!(&tokens[0].token, Token::Keyword(Keyword::RULE)),
        "token 'rule' should be recognized as RULE keyword"
    );
}

#[test]
fn test_tokenize_null_as_keyword() {
    let tokens = Tokenizer::new("null").tokenize().unwrap();
    assert!(
        matches!(&tokens[0].token, Token::Keyword(Keyword::NULL_P)),
        "token 'null' should be recognized as NULL_P keyword"
    );
}

#[test]
fn test_tokenize_minus_as_keyword() {
    let tokens = Tokenizer::new("minus").tokenize().unwrap();
    assert!(
        matches!(&tokens[0].token, Token::Keyword(Keyword::MINUS_P)),
        "token 'minus' should be recognized as MINUS_P keyword"
    );
}

// === Unreserved keywords can be used as identifiers (no error) ===

#[test]
fn test_value_as_table_name_no_error() {
    // value is Unreserved → can be used as table name without error
    let sql = "SELECT * FROM value";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "Unreserved keyword 'value' as table name should not trigger error, got: {:?}",
        keyword_issues
    );
}

#[test]
fn test_value_as_column_name_no_error() {
    // value is Unreserved → can be used as column name
    let sql = "SELECT value FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "Unreserved keyword 'value' as column name should not trigger error"
    );
}

#[test]
fn test_name_as_table_name_no_error() {
    // name is Unreserved → can be used as table name
    let sql = "SELECT * FROM name";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "Unreserved keyword 'name' as table name should not trigger error"
    );
}

#[test]
fn test_rule_as_table_name_no_error() {
    // rule is Unreserved → can be used as table name
    let sql = "SELECT * FROM rule";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "Unreserved keyword 'rule' as table name should not trigger error"
    );
}

// === Reserved keywords used as identifiers should produce error ===

#[test]
fn test_null_as_table_name_reserved_error() {
    // null is Reserved → used as bare table name should error
    let sql = "SELECT * FROM null";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty(), "Should still produce AST (soft error)");
    let reserved_errors: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        !reserved_errors.is_empty(),
        "Reserved keyword 'null' used as table name should error"
    );
    assert!(reserved_errors[0].to_string().contains("null"));
}

#[test]
fn test_minus_as_table_name_reserved_error() {
    // minus is Reserved → used as bare table name should error
    let sql = "SELECT * FROM minus";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty(), "Should still produce AST (soft error)");
    let reserved_errors: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        !reserved_errors.is_empty(),
        "Reserved keyword 'minus' used as table name should error"
    );
    assert!(reserved_errors[0].to_string().contains("minus"));
}

// === Reserved keywords CAN be used when double-quoted ===

#[test]
fn test_null_quoted_as_table_name_no_error() {
    // "null" (quoted) is a valid identifier, no error
    let sql = r#"SELECT * FROM "null""#;
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "Quoted \"null\" should not trigger keyword errors"
    );
}

#[test]
fn test_minus_quoted_as_table_name_no_error() {
    // "minus" (quoted) is a valid identifier, no error
    let sql = r#"SELECT * FROM "minus""#;
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "Quoted \"minus\" should not trigger keyword errors"
    );
}

// === Semantic usage tests: null/minus in valid SQL contexts ===

#[test]
fn test_null_in_select_list_no_error() {
    // NULL as a literal expression (valid use of reserved keyword)
    let sql = "SELECT NULL";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "NULL as expression should not produce keyword error"
    );
}

#[test]
fn test_null_in_where_is_null_no_error() {
    // IS NULL is a valid operator
    let sql = "SELECT * FROM t WHERE c1 IS NULL";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "IS NULL should not produce keyword error"
    );
}

#[test]
fn test_minus_as_set_operator_no_error() {
    // MINUS is a valid set operator (Oracle/GaussDB syntax for EXCEPT)
    let sql = "SELECT id FROM t1 MINUS SELECT id FROM t2";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "MINUS as set operator should not produce keyword error"
    );
}

// === value/rule in domain/rule statements (valid semantic use) ===

#[test]
fn test_value_in_domain_check_no_error() {
    // VALUE is used inside DOMAIN CHECK constraints (valid Unreserved keyword usage)
    let sql = "CREATE DOMAIN d AS INT CHECK (VALUE > 0)";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "VALUE in CHECK constraint should not produce keyword error"
    );
}

#[test]
fn test_rule_statement_parsed_correctly() {
    // RULE is a statement keyword (Unreserved) — used to define rewrite rules
    let sql = "RULE r1 AS ON SELECT TO users DO INSTEAD NOTHING";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    match &stmts[0] {
        Statement::Rule(r) => {
            assert_eq!(r.name, "r1");
        }
        _ => panic!("expected Rule statement"),
    }
}

// === Case-insensitive lookup verification ===

#[test]
fn test_keyword_lookup_case_insensitive() {
    // Verify lookup_keyword works case-insensitively for all 5 keywords
    assert_eq!(lookup_keyword("value"), Some(Keyword::VALUE_P));
    assert_eq!(lookup_keyword("VALUE"), Some(Keyword::VALUE_P));
    assert_eq!(lookup_keyword("Value"), Some(Keyword::VALUE_P));

    assert_eq!(lookup_keyword("name"), Some(Keyword::NAME_P));
    assert_eq!(lookup_keyword("NAME"), Some(Keyword::NAME_P));

    assert_eq!(lookup_keyword("rule"), Some(Keyword::RULE));
    assert_eq!(lookup_keyword("RULE"), Some(Keyword::RULE));

    assert_eq!(lookup_keyword("null"), Some(Keyword::NULL_P));
    assert_eq!(lookup_keyword("NULL"), Some(Keyword::NULL_P));

    assert_eq!(lookup_keyword("minus"), Some(Keyword::MINUS_P));
    assert_eq!(lookup_keyword("MINUS"), Some(Keyword::MINUS_P));
}

// ── Implicit alias tests: non-reserved keywords as column aliases (without AS) ──

#[test]
fn test_unreserved_keyword_name_as_implicit_alias() {
    let sql = "SELECT c1 name FROM t";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(_, alias) => {
                    assert_eq!(alias.as_deref(), Some("name"));
                }
                other => panic!("expected Expr target, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_unreserved_keyword_value_as_implicit_alias() {
    let sql = "SELECT c1 value FROM t";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(_, alias) => {
                    assert_eq!(alias.as_deref(), Some("value"));
                }
                other => panic!("expected Expr target, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_unreserved_keyword_result_as_implicit_alias() {
    let sql = "SELECT c1 result FROM t";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(_, alias) => {
                    assert_eq!(alias.as_deref(), Some("result"));
                }
                other => panic!("expected Expr target, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_unreserved_keyword_rule_as_implicit_alias() {
    let sql = "SELECT c1 rule FROM t";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(_, alias) => {
                    assert_eq!(alias.as_deref(), Some("rule"));
                }
                other => panic!("expected Expr target, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_multiple_unreserved_keyword_aliases() {
    let sql = "SELECT c1 name, c2 as value, c3 result FROM t";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 3);
            match &s.targets[0] {
                SelectTarget::Expr(_, alias) => assert_eq!(alias.as_deref(), Some("name")),
                _ => panic!("expected Expr target"),
            }
            match &s.targets[1] {
                SelectTarget::Expr(_, alias) => assert_eq!(alias.as_deref(), Some("value")),
                _ => panic!("expected Expr target"),
            }
            match &s.targets[2] {
                SelectTarget::Expr(_, alias) => assert_eq!(alias.as_deref(), Some("result")),
                _ => panic!("expected Expr target"),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_subquery_with_unreserved_keyword_aliases() {
    let sql =
        "SELECT name1, value, result FROM (SELECT c1 name1, c2 as value, c3 result FROM t) t1";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 3);
            assert!(!s.from.is_empty());
            match &s.from[0] {
                TableRef::Subquery { alias, .. } => {
                    assert_eq!(alias.as_deref(), Some("t1"));
                }
                other => panic!("expected Subquery, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_unreserved_keyword_as_outer_column_ref() {
    let sql = "SELECT name, value, result FROM t1";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 3);
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_unreserved_keyword_alias_no_keyword_errors() {
    let sql = "SELECT c1 name, c2 value, c3 result FROM t";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let keyword_issues: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        keyword_issues.is_empty(),
        "Unreserved keywords as implicit aliases should not trigger errors, got: {:?}",
        keyword_issues
    );
}

#[test]
fn test_reserved_keyword_null_not_implicit_alias() {
    // NULL is Reserved — should NOT be accepted as implicit alias
    // It gets parsed as a separate expression target, not as c1's alias
    let sql = "SELECT c1 null FROM t";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            // c1 is parsed as target with no alias; null is consumed as NULL literal expression
            // but since NULL doesn't have FROM after it, the parser stops early
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(_, alias) => {
                    assert!(
                        alias.is_none(),
                        "Reserved keyword 'null' should NOT be treated as implicit alias"
                    );
                }
                other => panic!("expected Expr target, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_colname_keyword_as_implicit_alias() {
    // BIGINT is ColName category — should be valid implicit alias
    let sql = "SELECT c1 bigint FROM t";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(_, alias) => {
                    assert_eq!(alias.as_deref(), Some("bigint"));
                }
                other => panic!("expected Expr target, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_typefuncname_keyword_as_implicit_alias() {
    // CROSS is TypeFuncName category — should be valid implicit alias
    let sql = "SELECT c1 cross FROM t";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(_, alias) => {
                    assert_eq!(alias.as_deref(), Some("cross"));
                }
                other => panic!("expected Expr target, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

// ========== Work Unit A: Quick Wins (P0-4 + P0-5) ==========

// --- EXPLAIN PLAN (P0-4: Verify existing implementation) ---

#[test]
fn test_explain_plan_basic() {
    let sql = "EXPLAIN PLAN FOR SELECT * FROM t";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Explain(e) => {
            assert!(e.plan);
            assert!(e.statement_id.is_none());
            match e.query.as_ref() {
                Statement::Select(s) => {
                    assert!(s.targets.len() == 1);
                }
                other => panic!("expected inner Select, got {:?}", other),
            }
        }
        other => panic!("expected Explain, got {:?}", other),
    }
}

#[test]
fn test_explain_plan_with_statement_id() {
    let sql = "EXPLAIN PLAN SET STATEMENT_ID = 'myplan' FOR SELECT * FROM t";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Explain(e) => {
            assert!(e.plan);
            assert_eq!(e.statement_id.as_deref(), Some("myplan"));
            match e.query.as_ref() {
                Statement::Select(s) => {
                    assert!(s.targets.len() == 1);
                }
                other => panic!("expected inner Select, got {:?}", other),
            }
        }
        other => panic!("expected Explain, got {:?}", other),
    }
}

#[test]
fn test_explain_plan_roundtrip() {
    let sql = "EXPLAIN PLAN SET STATEMENT_ID = 'test' FOR SELECT * FROM t";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(stmt, stmt2);
}

// --- SELECT INTO TABLE (P0-5: GaussDB extension) ---

#[test]
fn test_select_into_table() {
    let sql = "SELECT * INTO TABLE new_table FROM t";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert!(
                s.into_targets.is_none(),
                "into_targets should be None for INTO TABLE"
            );
            let into_table = s.into_table.as_ref().expect("expected into_table");
            assert!(!into_table.unlogged);
            assert_eq!(into_table.table_name, vec!["new_table".to_string()]);
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_select_into_unlogged_table() {
    let sql = "SELECT * INTO UNLOGGED TABLE new_table FROM t WHERE id = 1";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert!(s.into_targets.is_none());
            let into_table = s.into_table.as_ref().expect("expected into_table");
            assert!(into_table.unlogged);
            assert_eq!(into_table.table_name, vec!["new_table".to_string()]);
            assert!(s.where_clause.is_some());
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_select_into_table_no_keyword() {
    // GaussDB allows omitting TABLE keyword: SELECT * INTO new_table FROM t
    let sql = "SELECT * INTO new_table FROM t";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert!(s.into_targets.is_none(), "into_targets should be None");
            let into_table = s.into_table.as_ref().expect("expected into_table");
            assert!(!into_table.unlogged);
            assert_eq!(into_table.table_name, vec!["new_table".to_string()]);
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_select_into_table_roundtrip() {
    let sql = "SELECT * INTO TABLE new_table FROM t";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(stmt, stmt2);
}

#[test]
fn test_select_into_unlogged_table_roundtrip() {
    let sql = "SELECT * INTO UNLOGGED TABLE new_table FROM t WHERE id = 1";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(stmt, stmt2);
}

#[test]
fn test_select_into_variables_still_works() {
    let sql = "SELECT col1, col2 INTO v1, v2 FROM t";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert!(
                s.into_table.is_none(),
                "into_table should be None for PL/pgSQL INTO"
            );
            let into_targets = s.into_targets.as_ref().expect("expected into_targets");
            assert_eq!(into_targets.len(), 2);
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

// ========== Utility statement tests ==========

#[test]
fn test_shutdown_bare() {
    let stmt = parse_one("SHUTDOWN");
    match stmt {
        Statement::Shutdown(s) => assert_eq!(s.mode, None),
        other => panic!("expected Shutdown, got {:?}", other),
    }
}

#[test]
fn test_shutdown_fast() {
    let stmt = parse_one("SHUTDOWN FAST");
    match stmt {
        Statement::Shutdown(s) => assert_eq!(s.mode.as_deref(), Some("FAST")),
        other => panic!("expected Shutdown, got {:?}", other),
    }
}

#[test]
fn test_shutdown_immediate() {
    let stmt = parse_one("SHUTDOWN IMMEDIATE");
    match stmt {
        Statement::Shutdown(s) => assert_eq!(s.mode.as_deref(), Some("IMMEDIATE")),
        other => panic!("expected Shutdown, got {:?}", other),
    }
}

#[test]
fn test_barrier() {
    let stmt = parse_one("BARRIER my_barrier");
    match stmt {
        Statement::Barrier(s) => assert_eq!(s.name, "my_barrier"),
        other => panic!("expected Barrier, got {:?}", other),
    }
}

#[test]
fn test_purge_table() {
    let stmt = parse_one("PURGE TABLE my_table");
    match stmt {
        Statement::Purge(s) => match s.target {
            PurgeTarget::Table { ref name } => {
                assert_eq!(name.join("."), "my_table");
            }
            _ => panic!("expected PurgeTarget::Table"),
        },
        other => panic!("expected Purge, got {:?}", other),
    }
}

#[test]
fn test_purge_index() {
    let stmt = parse_one("PURGE INDEX my_idx");
    match stmt {
        Statement::Purge(s) => match s.target {
            PurgeTarget::Index { ref name } => {
                assert_eq!(name.join("."), "my_idx");
            }
            _ => panic!("expected PurgeTarget::Index"),
        },
        other => panic!("expected Purge, got {:?}", other),
    }
}

#[test]
fn test_purge_recyclebin() {
    let stmt = parse_one("PURGE RECYCLEBIN");
    match stmt {
        Statement::Purge(s) => assert!(matches!(s.target, PurgeTarget::RecycleBin)),
        other => panic!("expected Purge, got {:?}", other),
    }
}

#[test]
fn test_snapshot_with_name() {
    let stmt = parse_one("SNAPSHOT snap1");
    match stmt {
        Statement::Snapshot(s) => assert_eq!(s.name.as_deref(), Some("snap1")),
        other => panic!("expected Snapshot, got {:?}", other),
    }
}

#[test]
fn test_snapshot_bare() {
    let stmt = parse_one("SNAPSHOT");
    match stmt {
        Statement::Snapshot(s) => {
            assert_eq!(s.name, None);
            assert!(s.options.is_empty());
        }
        other => panic!("expected Snapshot, got {:?}", other),
    }
}

#[test]
fn test_timecapsule_table() {
    let stmt = parse_one("TIMECAPSULE TABLE t1 TO TIMESTAMP");
    match stmt {
        Statement::TimeCapsule(s) => {
            assert_eq!(s.table_name.join("."), "t1");
            assert!(!s.action.is_empty());
        }
        other => panic!("expected TimeCapsule, got {:?}", other),
    }
}

#[test]
fn test_shrink() {
    let stmt = parse_one("SHRINK SPACE");
    match stmt {
        Statement::Shrink(s) => {
            assert_eq!(s.target.as_deref(), Some("space"));
        }
        other => panic!("expected Shrink, got {:?}", other),
    }
}

#[test]
fn test_verify() {
    let stmt = parse_one("VERIFY TABLE t1");
    match stmt {
        Statement::Verify(s) => assert!(!s.raw_rest.is_empty()),
        other => panic!("expected Verify, got {:?}", other),
    }
}

#[test]
fn test_compile() {
    let stmt = parse_one("COMPILE");
    match stmt {
        Statement::Compile(s) => assert!(s.raw_rest.is_empty()),
        other => panic!("expected Compile, got {:?}", other),
    }
}

#[test]
fn test_clean_conn_all() {
    let stmt = parse_one("CLEAN CONNECTION TO ALL");
    match stmt {
        Statement::CleanConn(s) => {
            assert!(!s.force);
            assert!(s.for_database.is_none());
            assert!(s.to_user.is_none());
        }
        other => panic!("expected CleanConn, got {:?}", other),
    }
}

#[test]
fn test_clean_conn_for_user() {
    let stmt = parse_one("CLEAN CONNECTION TO ALL FOR USER admin");
    match stmt {
        Statement::CleanConn(s) => {
            assert!(!s.force);
            assert_eq!(s.to_user.as_deref(), Some("admin"));
        }
        other => panic!("expected CleanConn, got {:?}", other),
    }
}

#[test]
fn test_sec_label() {
    let stmt = parse_one("SECURITY LABEL TABLE my_table IS 'classified'");
    match stmt {
        Statement::SecLabel(s) => {
            assert_eq!(s.object_type, "table");
            assert_eq!(s.label.as_deref(), Some("classified"));
        }
        other => panic!("expected SecLabel, got {:?}", other),
    }
}

#[test]
fn test_create_conversion() {
    let stmt = parse_one("CREATE CONVERSION myconv FOR latin1 TO utf8 FROM my_func");
    match stmt {
        Statement::CreateConversion(s) => {
            assert_eq!(s.name, "myconv");
            assert_eq!(s.source_encoding, "latin1");
            assert_eq!(s.dest_encoding, "utf8");
            assert_eq!(s.function_name, "my_func");
        }
        other => panic!("expected CreateConversion, got {:?}", other),
    }
}

#[test]
fn test_create_synonym() {
    let stmt = parse_one("CREATE OR REPLACE SYNONYM mysyn FOR public.my_table PUBLIC");
    match stmt {
        Statement::CreateSynonym(s) => {
            assert!(s.replace);
            assert_eq!(s.name, vec!["mysyn".to_string()]);
            assert_eq!(s.target, vec!["public".to_string(), "my_table".to_string()]);
            assert!(s.public);
        }
        other => panic!("expected CreateSynonym, got {:?}", other),
    }
}

#[test]
fn test_create_model() {
    let stmt = parse_one(
        "CREATE MODEL mymodel USING linear FEATURES (col1, col2) TARGET col3 FROM mytable",
    );
    match stmt {
        Statement::CreateModel(s) => {
            assert_eq!(s.name, "mymodel");
            assert!(s.raw_rest.contains("using"));
        }
        other => panic!("expected CreateModel, got {:?}", other),
    }
}

#[test]
fn test_create_am() {
    let stmt = parse_one("CREATE ACCESS METHOD myam TYPE btree HANDLER my_handler");
    match stmt {
        Statement::CreateAm(s) => {
            assert_eq!(s.name, "myam");
            assert_eq!(s.method, "btree");
            assert_eq!(s.handler, "my_handler");
        }
        other => panic!("expected CreateAm, got {:?}", other),
    }
}

#[test]
fn test_create_directory() {
    let stmt = parse_one("CREATE DIRECTORY mydir AS '/tmp/data'");
    match stmt {
        Statement::CreateDirectory(s) => {
            assert_eq!(s.name, "mydir");
            assert_eq!(s.path, "/tmp/data");
        }
        other => panic!("expected CreateDirectory, got {:?}", other),
    }
}

#[test]
fn test_create_data_source() {
    let stmt = parse_one("CREATE DATA SOURCE myds WITH (url = 'localhost', type = 'mysql')");
    match stmt {
        Statement::CreateDataSource(s) => {
            assert_eq!(s.name, "myds");
            assert_eq!(s.options.len(), 2);
        }
        other => panic!("expected CreateDataSource, got {:?}", other),
    }
}

#[test]
fn test_create_event() {
    let stmt = parse_one("CREATE EVENT myevent ON SCHEDULE EVERY 1 DAY DO SELECT 1");
    match stmt {
        Statement::CreateEvent(s) => {
            assert_eq!(s.name, "myevent");
            assert!(s.raw_rest.contains("schedule"));
        }
        other => panic!("expected CreateEvent, got {:?}", other),
    }
}

#[test]
fn test_create_opclass() {
    let stmt = parse_one("CREATE OPERATOR CLASS myop USING btree DEFAULT");
    match stmt {
        Statement::CreateOpClass(s) => {
            assert_eq!(s.name, "myop");
            assert_eq!(s.method, "btree");
        }
        other => panic!("expected CreateOpClass, got {:?}", other),
    }
}

#[test]
fn test_create_opfamily() {
    let stmt = parse_one("CREATE OPERATOR FAMILY myop USING btree");
    match stmt {
        Statement::CreateOpFamily(s) => {
            assert_eq!(s.name, "myop");
            assert_eq!(s.method, "btree");
        }
        other => panic!("expected CreateOpFamily, got {:?}", other),
    }
}

#[test]
fn test_create_contquery() {
    let stmt = parse_one("CREATE CONTINUOUS QUERY mycq AS SELECT * FROM my_stream");
    match stmt {
        Statement::CreateContQuery(s) => {
            assert!(s.raw_rest.contains("mycq"));
        }
        other => panic!("expected CreateContQuery, got {:?}", other),
    }
}

#[test]
fn test_create_stream() {
    let stmt = parse_one("CREATE STREAM mystream (id int, name text)");
    match stmt {
        Statement::CreateStream(s) => {
            assert!(s.raw_rest.contains("mystream"));
        }
        other => panic!("expected CreateStream, got {:?}", other),
    }
}

#[test]
fn test_create_key() {
    let stmt = parse_one("CREATE KEY mykey WITH (algorithm = 'RSA')");
    match stmt {
        Statement::CreateKey(s) => {
            assert!(s.raw_rest.contains("mykey"));
        }
        other => panic!("expected CreateKey, got {:?}", other),
    }
}

#[test]
fn test_alter_foreign_table() {
    let stmt = parse_one("ALTER FOREIGN TABLE ft1 ADD COLUMN c1 INT");
    match stmt {
        Statement::AlterForeignTable(s) => {
            assert_eq!(s.name.join("."), "ft1");
            assert!(!s.raw_rest.is_empty());
        }
        other => panic!("expected AlterForeignTable, got {:?}", other),
    }
}

#[test]
fn test_alter_foreign_server() {
    let stmt = parse_one("ALTER FOREIGN SERVER srv1 OPTIONS (host 'localhost')");
    match stmt {
        Statement::AlterForeignServer(s) => {
            assert_eq!(s.name, "srv1");
            assert!(!s.raw_rest.is_empty());
        }
        other => panic!("expected AlterForeignServer, got {:?}", other),
    }
}

#[test]
fn test_alter_fdw() {
    let stmt = parse_one("ALTER FOREIGN DATA WRAPPER fdw1 HANDLER new_handler");
    match stmt {
        Statement::AlterFdw(s) => {
            assert_eq!(s.name, "fdw1");
            assert!(!s.raw_rest.is_empty());
        }
        other => panic!("expected AlterFdw, got {:?}", other),
    }
}

#[test]
fn test_alter_publication() {
    let stmt = parse_one("ALTER PUBLICATION pub1 ADD TABLE t1");
    match stmt {
        Statement::AlterPublication(s) => {
            assert_eq!(s.name, "pub1");
            assert!(!s.raw_rest.is_empty());
        }
        other => panic!("expected AlterPublication, got {:?}", other),
    }
}

#[test]
fn test_alter_subscription() {
    let stmt = parse_one("ALTER SUBSCRIPTION sub1 CONNECTION 'host=remote'");
    match stmt {
        Statement::AlterSubscription(s) => {
            assert_eq!(s.name, "sub1");
            assert!(!s.raw_rest.is_empty());
        }
        other => panic!("expected AlterSubscription, got {:?}", other),
    }
}

#[test]
fn test_alter_node() {
    let stmt = parse_one("ALTER NODE node1 WITH (host = '127.0.0.1')");
    match stmt {
        Statement::AlterNode(s) => {
            assert_eq!(s.name, "node1");
            assert!(!s.raw_rest.is_empty());
        }
        other => panic!("expected AlterNode, got {:?}", other),
    }
}

#[test]
fn test_alter_node_group() {
    let stmt = parse_one("ALTER NODE GROUP grp1 ADD NODE node2");
    match stmt {
        Statement::AlterNodeGroup(s) => {
            assert_eq!(s.name, "grp1");
            assert!(!s.raw_rest.is_empty());
        }
        other => panic!("expected AlterNodeGroup, got {:?}", other),
    }
}

#[test]
fn test_alter_workload_group() {
    let stmt = parse_one("ALTER WORKLOAD GROUP wg1 SET (cpu_limit = 0.5)");
    match stmt {
        Statement::AlterWorkloadGroup(s) => {
            assert_eq!(s.name, "wg1");
            assert!(!s.raw_rest.is_empty());
        }
        other => panic!("expected AlterWorkloadGroup, got {:?}", other),
    }
}

#[test]
fn test_alter_audit_policy() {
    let stmt = parse_one("ALTER AUDIT POLICY ap1 COMMENTS 'updated'");
    match stmt {
        Statement::AlterAuditPolicy(s) => {
            assert_eq!(s.name, "ap1");
            assert!(!s.raw_rest.is_empty());
        }
        other => panic!("expected AlterAuditPolicy, got {:?}", other),
    }
}

#[test]
fn test_alter_rls_policy() {
    let stmt = parse_one("ALTER POLICY rls1 ON t1 WITH CHECK (true)");
    match stmt {
        Statement::AlterRlsPolicy(s) => {
            assert_eq!(s.name, "rls1");
            assert!(!s.raw_rest.is_empty());
        }
        other => panic!("expected AlterRlsPolicy, got {:?}", other),
    }
}

#[test]
fn test_alter_rls_policy_with_prefix() {
    let stmt = parse_one("ALTER RLS POLICY rls2 ON t2");
    match stmt {
        Statement::AlterRlsPolicy(s) => {
            assert_eq!(s.name, "rls2");
        }
        other => panic!("expected AlterRlsPolicy, got {:?}", other),
    }
}

#[test]
fn test_alter_data_source() {
    let stmt = parse_one("ALTER DATA SOURCE ds1 SET (opt = 'val')");
    match stmt {
        Statement::AlterDataSource(s) => {
            assert_eq!(s.name, "ds1");
            assert!(!s.raw_rest.is_empty());
        }
        other => panic!("expected AlterDataSource, got {:?}", other),
    }
}

#[test]
fn test_alter_event() {
    let stmt = parse_one("ALTER EVENT evt1 ENABLE");
    match stmt {
        Statement::AlterEvent(s) => {
            assert_eq!(s.name, "evt1");
            assert!(!s.raw_rest.is_empty());
        }
        other => panic!("expected AlterEvent, got {:?}", other),
    }
}

#[test]
fn test_alter_opfamily() {
    let stmt = parse_one("ALTER OPERATOR FAMILY of1 USING btree ADD FUNCTION 1 foo(bar)");
    match stmt {
        Statement::AlterOpFamily(s) => {
            assert_eq!(s.name, "of1");
            assert_eq!(s.method, "btree");
            assert!(!s.raw_rest.is_empty());
        }
        other => panic!("expected AlterOpFamily, got {:?}", other),
    }
}

#[test]
fn test_alter_materialized_view() {
    let stmt = parse_one("ALTER MATERIALIZED VIEW mv1 SET (fillfactor = 50)");
    match stmt {
        Statement::AlterMaterializedView(s) => {
            assert_eq!(s.name.join("."), "mv1");
            assert!(!s.raw_rest.is_empty());
        }
        other => panic!("expected AlterMaterializedView, got {:?}", other),
    }
}

#[test]
fn test_fetch_in_keyword() {
    let sql = "FETCH NEXT IN cur1";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::Fetch(f) => {
            assert_eq!(f.cursor_name, "cur1");
            assert_eq!(f.direction, crate::ast::FetchDirection::Next);
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_fetch_bare_forward() {
    let sql = "FETCH FORWARD FROM cur1";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::Fetch(f) => {
            assert_eq!(f.direction, crate::ast::FetchDirection::Forward);
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_fetch_bare_backward_in() {
    let sql = "FETCH BACKWARD IN cur1";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::Fetch(f) => {
            assert_eq!(f.direction, crate::ast::FetchDirection::Backward);
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_fetch_forward_count() {
    let sql = "FETCH FORWARD 5 FROM cur1";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::Fetch(f) => {
            assert_eq!(f.direction, crate::ast::FetchDirection::ForwardCount(5));
        }
        _ => panic!("expected Fetch"),
    }
}

#[test]
fn test_move_next_from() {
    let sql = "MOVE NEXT FROM cur1";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::Move(m) => {
            assert_eq!(m.cursor_name, "cur1");
            assert_eq!(m.direction, crate::ast::FetchDirection::Next);
        }
        _ => panic!("expected Move, got {:?}", &stmts[0]),
    }
}

#[test]
fn test_move_forward_5_in() {
    let sql = "MOVE FORWARD 5 IN cur1";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::Move(m) => {
            assert_eq!(m.direction, crate::ast::FetchDirection::ForwardCount(5));
            assert_eq!(m.cursor_name, "cur1");
        }
        _ => panic!("expected Move"),
    }
}

#[test]
fn test_move_all() {
    let sql = "MOVE ALL FROM cur1";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::Move(m) => {
            assert_eq!(m.direction, crate::ast::FetchDirection::All);
        }
        _ => panic!("expected Move"),
    }
}

#[test]
fn test_move_absolute_negative() {
    let sql = "MOVE ABSOLUTE -3 FROM cur1";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::Move(m) => {
            assert_eq!(m.direction, crate::ast::FetchDirection::Absolute(-3));
        }
        _ => panic!("expected Move"),
    }
}

#[test]
fn test_close_all() {
    let sql = "CLOSE ALL";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::ClosePortal(c) => {
            assert_eq!(c.target, CloseTarget::All);
        }
        _ => panic!("expected ClosePortal"),
    }
}

#[test]
fn test_close_named() {
    let sql = "CLOSE cur1";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::ClosePortal(c) => {
            assert_eq!(c.target, CloseTarget::Name("cur1".to_string()));
        }
        _ => panic!("expected ClosePortal"),
    }
}

#[test]
fn test_update_where_current_of() {
    let sql = "UPDATE accounts SET balance = balance + 100 WHERE CURRENT OF cur_account";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::Update(u) => match &u.where_clause {
            Some(Expr::CurrentOf { cursor_name }) => {
                assert_eq!(cursor_name, "cur_account");
            }
            other => panic!("expected CurrentOf, got {:?}", other),
        },
        _ => panic!("expected Update"),
    }
}

#[test]
fn test_delete_where_current_of() {
    let sql = "DELETE FROM accounts WHERE CURRENT OF cur_account";
    let stmts = parse(sql);
    match &stmts[0] {
        Statement::Delete(d) => match &d.where_clause {
            Some(Expr::CurrentOf { cursor_name }) => {
                assert_eq!(cursor_name, "cur_account");
            }
            other => panic!("expected CurrentOf, got {:?}", other),
        },
        _ => panic!("expected Delete"),
    }
}

#[test]
fn test_delete_partition() {
    let sql = "DELETE FROM range_list PARTITION (p_201901)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Delete(d) => {
            assert_eq!(d.tables.len(), 1);
            match &d.tables[0] {
                TableRef::Table {
                    name, partition, ..
                } => {
                    assert_eq!(name.join("."), "range_list");
                    assert!(partition.is_some());
                    let p = partition.as_ref().unwrap();
                    assert_eq!(p.values, vec!["p_201901"]);
                    assert!(p.for_values.is_none());
                }
                _ => panic!("expected Table"),
            }
        }
        _ => panic!("expected Delete"),
    }
}

#[test]
fn test_delete_partition_for() {
    let sql = "DELETE FROM range_list PARTITION FOR ('201903')";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Delete(d) => {
            assert_eq!(d.tables.len(), 1);
            match &d.tables[0] {
                TableRef::Table { partition, .. } => {
                    assert!(partition.is_some());
                    let p = partition.as_ref().unwrap();
                    assert!(p.for_values.is_some());
                    assert_eq!(p.values.len(), 0);
                }
                _ => panic!("expected Table"),
            }
        }
        _ => panic!("expected Delete"),
    }
}

#[test]
fn test_delete_subpartition() {
    let sql = "DELETE FROM range_list SUBPARTITION (p_201901_a)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Delete(d) => {
            assert_eq!(d.tables.len(), 1);
            match &d.tables[0] {
                TableRef::Table { partition, .. } => {
                    assert!(partition.is_some());
                    let p = partition.as_ref().unwrap();
                    assert_eq!(p.values, vec!["p_201901_a"]);
                }
                _ => panic!("expected Table"),
            }
        }
        _ => panic!("expected Delete"),
    }
}

#[test]
fn test_delete_partition_multiple() {
    let sql = "DELETE FROM range_list PARTITION (p_201901_a, p_201901)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Delete(d) => {
            assert_eq!(d.tables.len(), 1);
            match &d.tables[0] {
                TableRef::Table { partition, .. } => {
                    assert!(partition.is_some());
                    let p = partition.as_ref().unwrap();
                    assert_eq!(p.values, vec!["p_201901_a", "p_201901"]);
                }
                _ => panic!("expected Table"),
            }
        }
        _ => panic!("expected Delete"),
    }
}

#[test]
fn test_delete_with_alias_partition() {
    let sql = "DELETE FROM range_list AS t PARTITION (p_201901_a, p_201901)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Delete(d) => {
            assert_eq!(d.tables.len(), 1);
            match &d.tables[0] {
                TableRef::Table {
                    alias, partition, ..
                } => {
                    assert_eq!(alias.as_deref(), Some("t"));
                    assert!(partition.is_some());
                    let p = partition.as_ref().unwrap();
                    assert_eq!(p.values, vec!["p_201901_a", "p_201901"]);
                }
                _ => panic!("expected Table"),
            }
        }
        _ => panic!("expected Delete"),
    }
}

#[test]
fn test_plpgsql_open_for_execute() {
    let block = parse_do_block("DO $$ BEGIN OPEN cur FOR EXECUTE 'SELECT * FROM t'; END $$");
    match &block.body[0] {
        PlStatement::Open(o) => {
            assert_eq!(o.cursor, "cur");
            match &o.kind {
                PlOpenKind::ForExecute { query, using_args } => {
                    assert!(
                        matches!(query, Expr::Literal(crate::ast::Literal::String(s)) if s == "SELECT * FROM t")
                    );
                    assert!(using_args.is_empty());
                }
                other => panic!("expected ForExecute, got {:?}", other),
            }
        }
        _ => panic!("expected Open"),
    }
}

#[test]
fn test_plpgsql_open_for_execute_using() {
    let block = parse_do_block("DO $$ BEGIN OPEN cur FOR EXECUTE v_query USING 1, 'x'; END $$");
    match &block.body[0] {
        PlStatement::Open(o) => {
            assert_eq!(o.cursor, "cur");
            match &o.kind {
                PlOpenKind::ForExecute { query, using_args } => {
                    assert!(matches!(query, Expr::ColumnRef(_)));
                    assert_eq!(using_args.len(), 2);
                }
                other => panic!("expected ForExecute, got {:?}", other),
            }
        }
        _ => panic!("expected Open"),
    }
}

#[test]
fn test_plpgsql_open_scroll_for() {
    let block = parse_do_block("DO $$ BEGIN OPEN cur SCROLL FOR SELECT * FROM t; END $$");
    match &block.body[0] {
        PlStatement::Open(o) => {
            assert_eq!(o.cursor, "cur");
            match &o.kind {
                PlOpenKind::ForQuery { scroll, query, .. } => {
                    assert_eq!(scroll, &Some(true));
                    assert!(!query.is_empty());
                }
                other => panic!("expected ForQuery, got {:?}", other),
            }
        }
        _ => panic!("expected Open"),
    }
}

#[test]
fn test_plpgsql_open_no_scroll_for() {
    let block = parse_do_block("DO $$ BEGIN OPEN cur NO SCROLL FOR SELECT * FROM t; END $$");
    match &block.body[0] {
        PlStatement::Open(o) => {
            assert_eq!(o.cursor, "cur");
            match &o.kind {
                PlOpenKind::ForQuery { scroll, query, .. } => {
                    assert_eq!(scroll, &Some(false));
                    assert!(!query.is_empty());
                }
                other => panic!("expected ForQuery, got {:?}", other),
            }
        }
        _ => panic!("expected Open"),
    }
}

// ========== Cursor Round-Trip Tests (SQL → AST → JSON → AST → SQL) ==========

/// Full round-trip helper: parse SQL → AST → JSON → AST → format SQL → re-parse → compare ASTs.
fn roundtrip_cursor(sql: &str) {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();

    let json = serde_json::to_string(&stmts).unwrap();
    let restored: Vec<Statement> = serde_json::from_str(&json).unwrap();

    let formatter = SqlFormatter::new();
    let output: Vec<String> = restored
        .iter()
        .map(|s| formatter.format_statement(s))
        .collect();
    let result_sql = output.join(";\n");

    let tokens2 = Tokenizer::new(&result_sql).tokenize().unwrap();
    let stmts2 = Parser::new(tokens2).parse();
    assert_eq!(stmts, stmts2, "Round-trip failed for: {}", sql);
}

#[test]
fn test_cursor_roundtrip_declare() {
    let cases = vec![
        "DECLARE cur CURSOR FOR SELECT * FROM t",
        "DECLARE cur BINARY SCROLL CURSOR WITH HOLD FOR SELECT id FROM users",
        "DECLARE cur NO SCROLL INSENSITIVE CURSOR WITHOUT HOLD FOR SELECT 1",
        "DECLARE cur CURSOR WITH RETURN TO CALLER FOR SELECT * FROM t",
        "DECLARE cur SCROLL CURSOR WITHOUT RETURN TO CLIENT FOR SELECT id FROM t",
    ];
    for sql in cases {
        roundtrip_cursor(sql);
    }
}

#[test]
fn test_cursor_roundtrip_fetch_move() {
    let cases = vec![
        "FETCH NEXT FROM cur1",
        "FETCH FORWARD 5 FROM cur1",
        "FETCH ALL FROM cur1",
        "FETCH PRIOR FROM cur1",
        "FETCH ABSOLUTE 10 FROM cur1",
        "MOVE NEXT FROM cur1",
        "MOVE FORWARD 5 IN cur1",
        "MOVE ALL FROM cur1",
    ];
    for sql in cases {
        roundtrip_cursor(sql);
    }
}

#[test]
fn test_cursor_roundtrip_close() {
    let cases = vec!["CLOSE cur1", "CLOSE ALL"];
    for sql in cases {
        roundtrip_cursor(sql);
    }
}

#[test]
fn test_cursor_roundtrip_current_of() {
    let cases = vec![
        "UPDATE t SET x = 1 WHERE CURRENT OF cur",
        "DELETE FROM t WHERE CURRENT OF cur",
    ];
    for sql in cases {
        roundtrip_cursor(sql);
    }
}

fn parse_with_errors(sql: &str) -> (Vec<Statement>, Vec<ParserError>) {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    let reserved_errors: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .cloned()
        .collect();
    (stmts, reserved_errors)
}

#[test]
fn test_merge_insert_qualified_columns_standalone() {
    let sql = "MERGE INTO t1 USING t2 ON t1.id = t2.id WHEN MATCHED THEN UPDATE SET t1.val = t2.val WHEN NOT MATCHED THEN INSERT (t1.organ_id, t1.acnt_type) VALUES (t2.organ_id, t2.acnt_type)";
    let (stmts, errors) = parse_with_errors(sql);
    assert!(!stmts.is_empty(), "MERGE should produce an AST");
    assert!(
        errors.is_empty(),
        "MERGE INSERT with qualified column names should not produce reserved keyword errors, got: {:?}",
        errors
    );
    match &stmts[0] {
        Statement::Merge(m) => {
            assert_eq!(m.when_clauses.len(), 2, "Should have 2 WHEN clauses");
        }
        _ => panic!("Expected Merge statement, got: {:?}", stmts[0]),
    }
}

#[test]
fn test_merge_insert_qualified_columns_in_procedure() {
    let sql = "CREATE OR REPLACE PROCEDURE test_merge(p_o_code OUT VARCHAR2) IS\n\
               BEGIN\n\
               MERGE INTO t1 USING t2 ON t1.id = t2.id\n\
               WHEN MATCHED THEN\n\
                 UPDATE SET t1.a = t2.a\n\
               WHEN NOT MATCHED THEN\n\
                 INSERT (t1.organ_id) VALUES (t2.organ_id);\n\
               p_o_code := '0';\n\
               END";
    let (stmts, errors) = parse_with_errors(sql);
    assert!(!stmts.is_empty(), "Procedure should produce an AST");
    assert!(
        errors.is_empty(),
        "MERGE WHEN/THEN/NOT in PL/pgSQL should not produce reserved keyword errors, got: {:?}",
        errors
    );
}

#[test]
fn test_merge_insert_qualified_columns_in_procedure_with_subquery() {
    let sql =
        "CREATE OR REPLACE PROCEDURE test_merge(p_i_node VARCHAR2, p_o_code OUT VARCHAR2) IS\n\
               v_count NUMBER;\n\
               BEGIN\n\
               MERGE INTO par_sys_organ_tree_acnt t1\n\
               USING (SELECT a.organ_id FROM par_sys_organ_tree a WHERE a.node = p_i_node) t2\n\
               ON (t1.organ_id = t2.organ_id)\n\
               WHEN MATCHED THEN\n\
                 UPDATE SET t1.acnt_type = t2.acnt_type, t1.acnt_id = t2.acnt_id\n\
               WHEN NOT MATCHED THEN\n\
                 INSERT (t1.organ_id, t1.acnt_type, t1.acnt_id)\n\
                 VALUES (t2.organ_id, t2.acnt_type, t2.acnt_id);\n\
               p_o_code := '0';\n\
               EXCEPTION\n\
                 WHEN OTHERS THEN\n\
                   p_o_code := '1';\n\
               END";
    let (stmts, errors) = parse_with_errors(sql);
    assert!(!stmts.is_empty(), "Procedure should produce an AST");
    assert!(
        errors.is_empty(),
        "Full MERGE in procedure should not produce reserved keyword errors, got: {:?}",
        errors
    );
}

#[test]
fn test_merge_insert_simple_columns_still_works() {
    let sql = "MERGE INTO t1 USING t2 ON t1.id = t2.id WHEN NOT MATCHED THEN INSERT (organ_id, acnt_type) VALUES (t2.organ_id, t2.acnt_type)";
    let (stmts, errors) = parse_with_errors(sql);
    assert!(!stmts.is_empty());
    assert!(
        errors.is_empty(),
        "Simple column names should work fine, got: {:?}",
        errors
    );
}

#[test]
fn test_merge_insert_no_columns_still_works() {
    let sql = "MERGE INTO t1 USING t2 ON t1.id = t2.id WHEN NOT MATCHED THEN INSERT VALUES (t2.id, t2.val)";
    let (stmts, errors) = parse_with_errors(sql);
    assert!(!stmts.is_empty());
    assert!(
        errors.is_empty(),
        "INSERT without column list should work, got: {:?}",
        errors
    );
}

#[test]
fn test_merge_multiple_when_clauses_with_delete() {
    let sql = "MERGE INTO t1 USING t2 ON t1.id = t2.id WHEN MATCHED THEN UPDATE SET t1.val = t2.val WHEN MATCHED AND t1.val IS NULL THEN DELETE";
    let (stmts, errors) = parse_with_errors(sql);
    assert!(!stmts.is_empty());
    let when_then_errors: Vec<_> = errors
        .iter()
        .filter(|e| {
            let s = e.to_string();
            s.contains("\"when\"") || s.contains("\"then\"")
        })
        .collect();
    assert!(
        when_then_errors.is_empty(),
        "WHEN/THEN should not be flagged as reserved keyword misuse: {:?}",
        when_then_errors
    );
}

#[test]
fn test_reserved_keyword_misuse_still_detected_after_merge_fix() {
    let sql = "SELECT * FROM select";
    let (stmts, errors) = parse_with_errors(sql);
    assert!(!stmts.is_empty(), "Should still produce AST (soft error)");
    assert!(
        !errors.is_empty(),
        "Using reserved keyword 'select' as table name should still be caught"
    );
    assert!(errors[0].to_string().contains("select"));
}

#[test]
fn test_scalar_sublink_any() {
    let sql = "SELECT * FROM t1 WHERE a > ANY(SELECT b FROM t2)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => match &s.where_clause {
            Some(Expr::ScalarSublink {
                sublink_type: ScalarSublinkType::Any,
                op,
                ..
            }) => assert_eq!(op, ">"),
            other => panic!("expected ScalarSublink(Any), got {:?}", other),
        },
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_scalar_sublink_all() {
    let sql = "SELECT * FROM t1 WHERE a <= ALL(SELECT b FROM t2)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => match &s.where_clause {
            Some(Expr::ScalarSublink {
                sublink_type: ScalarSublinkType::All,
                op,
                ..
            }) => assert_eq!(op, "<="),
            other => panic!("expected ScalarSublink(All), got {:?}", other),
        },
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_scalar_sublink_some() {
    let sql = "SELECT * FROM t1 WHERE a = SOME(SELECT b FROM t2)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => match &s.where_clause {
            Some(Expr::ScalarSublink {
                sublink_type: ScalarSublinkType::Some,
                op,
                ..
            }) => assert_eq!(op, "="),
            other => panic!("expected ScalarSublink(Some), got {:?}", other),
        },
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_scalar_sublink_with_hint() {
    let sql = "SELECT * FROM t1 WHERE a > ANY(SELECT /*+EXPAND_SUBLINK*/ a FROM t2)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => match &s.where_clause {
            Some(Expr::ScalarSublink {
                sublink_type: ScalarSublinkType::Any,
                ..
            }) => {}
            other => panic!("expected ScalarSublink(Any), got {:?}", other),
        },
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_scalar_sublink_multiple_in_where() {
    let sql = "SELECT * FROM t1 WHERE a > ANY(SELECT a FROM t2) AND b > ANY(SELECT a FROM t3)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => match &s.where_clause {
            Some(Expr::BinaryOp { op, .. }) => assert_eq!(op, "AND"),
            other => panic!("expected BinaryOp(AND), got {:?}", other),
        },
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_scalar_sublink_format_roundtrip() {
    let sql = "SELECT * FROM t1 WHERE a > ANY(SELECT a FROM t2)";
    let stmt = parse_one(sql);
    let formatter = SqlFormatter::new();
    let formatted = formatter.format_statement(&stmt);
    assert!(
        formatted.contains("ANY"),
        "formatted should contain ANY: {}",
        formatted
    );
    assert!(
        formatted.contains("SELECT a FROM t2"),
        "formatted should contain subquery: {}",
        formatted
    );
}

#[test]
fn test_column_constraint_enable_disable() {
    let cases = vec![
        ("CREATE TABLE t (a INT NOT NULL ENABLE)", "NOT NULL ENABLE"),
        ("CREATE TABLE t (a INT NOT NULL DISABLE)", "NOT NULL DISABLE"),
        ("CREATE TABLE t (a INT NULL ENABLE)", "NULL ENABLE"),
        ("CREATE TABLE t (a INT UNIQUE ENABLE)", "UNIQUE ENABLE"),
        ("CREATE TABLE t (a INT PRIMARY KEY ENABLE)", "PRIMARY KEY ENABLE"),
        ("CREATE TABLE t (a INT CHECK (a > 0) ENABLE)", "CHECK ENABLE"),
        (
            "CREATE TABLE tpcds.reason (r_reason_sk INTEGER NOT NULL ENABLE, r_reason_id CHARACTER(16) NOT NULL ENABLE, r_reason_desc CHARACTER(100))",
            "TPC-DS schema",
        ),
    ];
    for (sql, label) in cases {
        let stmts = parse(sql);
        assert_eq!(
            stmts.len(),
            1,
            "{}: expected 1 statement, got {}",
            label,
            stmts.len()
        );
        assert!(
            !matches!(stmts[0], Statement::Empty),
            "{}: parsed as Empty — constraint with ENABLE/DISABLE failed",
            label,
        );
    }
}

// ========== Task 3: Factorial operators ! and !! ==========

#[test]
fn test_postfix_factorial() {
    let stmt = parse_one("SELECT 5 !");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_postfix_factorial_with_alias() {
    let (stmts, errors) = parse_with_errors("SELECT 5 ! AS RESULT");
    assert!(!stmts.is_empty());
    let as_errors: Vec<_> = errors
        .iter()
        .filter(|e| format!("{:?}", e).contains("as"))
        .collect();
    assert!(
        as_errors.is_empty(),
        "Should not error on AS, got: {:?}",
        as_errors
    );
}

#[test]
fn test_prefix_double_bang() {
    let stmt = parse_one("SELECT !! 5");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_factorial_in_expression() {
    let stmt = parse_one("SELECT 4 ! + 1");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

// ========== Task 1: USER as special expression ==========

#[test]
fn test_select_user() {
    let stmt = parse_one("SELECT USER");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, alias) => {
                    assert!(alias.is_none());
                    match expr {
                        Expr::ColumnRef(parts) => {
                            assert_eq!(parts, &vec!["user".to_string()]);
                        }
                        _ => panic!("expected ColumnRef, got {:?}", expr),
                    }
                }
                _ => panic!("expected Expr target"),
            }
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_select_user_no_reserved_error() {
    let (stmts, errors) = parse_with_errors("SELECT USER");
    assert!(!stmts.is_empty(), "should parse SELECT USER");
    assert!(
        errors.is_empty(),
        "USER should not trigger reserved keyword error, got: {:?}",
        errors
    );
}

// ========== Task 2: TRIM direction keywords ==========

#[test]
fn test_trim_both_no_error() {
    let (stmts, errors) = parse_with_errors("SELECT trim(BOTH 'x' FROM 'xTomxx')");
    assert!(!stmts.is_empty());
    assert!(
        errors.is_empty(),
        "BOTH should not trigger reserved keyword error, got: {:?}",
        errors
    );
}

#[test]
fn test_trim_leading_no_error() {
    let (stmts, errors) = parse_with_errors("SELECT trim(LEADING 'x' FROM 'xTomxx')");
    assert!(!stmts.is_empty());
    assert!(
        errors.is_empty(),
        "LEADING should not trigger reserved keyword error, got: {:?}",
        errors
    );
}

#[test]
fn test_trim_trailing_no_error() {
    let (stmts, errors) = parse_with_errors("SELECT trim(TRAILING 'x' FROM 'xTomxx')");
    assert!(!stmts.is_empty());
    assert!(
        errors.is_empty(),
        "TRAILING should not trigger reserved keyword error, got: {:?}",
        errors
    );
}

#[test]
fn test_trim_both_from_ast() {
    let stmt = parse_one("SELECT trim(BOTH 'x' FROM 'xTomxx')");
    match stmt {
        Statement::Select(s) => {
            match &s.targets[0] {
                SelectTarget::Expr(expr, _) => match expr {
                    Expr::SpecialFunction { name, args } => {
                        assert_eq!(name, "trim");
                        assert_eq!(args.len(), 3, "trim(BOTH 'x' FROM 'xTomxx') should have 3 args: [BOTH, 'x', 'xTomxx']");
                    }
                    _ => panic!("expected SpecialFunction, got {:?}", expr),
                },
                _ => panic!("expected Expr target"),
            }
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_trim_leading_with_chars_ast() {
    let stmt = parse_one("SELECT trim(LEADING 'x' FROM 'xTomxx')");
    match stmt {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(expr, _) => match expr {
                Expr::SpecialFunction { name, args } => {
                    assert_eq!(name, "trim");
                    assert_eq!(args.len(), 3);
                }
                _ => panic!("expected SpecialFunction, got {:?}", expr),
            },
            _ => panic!("expected Expr target"),
        },
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

// ========== Task 4: SIMILAR TO operator ==========

#[test]
fn test_similar_to() {
    let stmt = parse_one("SELECT 'abc' SIMILAR TO 'abc'");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_similar_to_ast() {
    let stmt = parse_one("SELECT 'abc' SIMILAR TO '%(b|d)%'");
    match stmt {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(expr, _) => match expr {
                Expr::BinaryOp { op, .. } => {
                    assert_eq!(op, "SIMILAR TO");
                }
                _ => panic!("expected BinaryOp, got {:?}", expr),
            },
            _ => panic!("expected Expr target"),
        },
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_not_similar_to() {
    let stmt = parse_one("SELECT 'abc' NOT SIMILAR TO 'a'");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr(expr, _) => match expr {
                    Expr::BinaryOp { op, .. } => {
                        assert_eq!(op, "NOT SIMILAR TO");
                    }
                    _ => panic!("expected BinaryOp, got {:?}", expr),
                },
                _ => panic!("expected Expr target"),
            }
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_similar_to_no_reserved_error() {
    let (stmts, errors) = parse_with_errors("SELECT 'abc' SIMILAR TO 'abc'");
    assert!(!stmts.is_empty());
    assert!(
        errors.is_empty(),
        "SIMILAR TO should not produce errors, got: {:?}",
        errors
    );
}

// ========== Task 5: LIKE ... ESCAPE clause ==========

#[test]
fn test_like_escape() {
    let stmt = parse_one("SELECT 'AA_BBCC' LIKE '%A@_B%' ESCAPE '@'");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_like_escape_ast() {
    let stmt = parse_one("SELECT 'AA_BBCC' LIKE '%A@_B%' ESCAPE '@'");
    match stmt {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(expr, _) => match expr {
                Expr::Like {
                    escape,
                    negated,
                    case_insensitive,
                    ..
                } => {
                    assert!(!negated);
                    assert!(!case_insensitive);
                    assert!(escape.is_some(), "ESCAPE should be parsed");
                }
                _ => panic!("expected Like, got {:?}", expr),
            },
            _ => panic!("expected Expr target"),
        },
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_not_like_escape() {
    let stmt = parse_one("SELECT 'abc' NOT LIKE 'a%' ESCAPE '!'");
    match stmt {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(expr, _) => match expr {
                Expr::Like {
                    negated, escape, ..
                } => {
                    assert!(negated);
                    assert!(escape.is_some());
                }
                _ => panic!("expected Like, got {:?}", expr),
            },
            _ => panic!("expected Expr target"),
        },
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_ilike_no_escape() {
    // ILIKE without ESCAPE still works
    let stmt = parse_one("SELECT 'abc' ILIKE 'ABC'");
    match stmt {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(expr, _) => match expr {
                Expr::Like {
                    case_insensitive,
                    escape,
                    negated,
                    ..
                } => {
                    assert!(case_insensitive);
                    assert!(!negated);
                    assert!(escape.is_none());
                }
                _ => panic!("expected Like, got {:?}", expr),
            },
            _ => panic!("expected Expr target"),
        },
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_like_escape_no_error() {
    let (stmts, errors) = parse_with_errors("SELECT 'AA_BBCC' LIKE '%A@_B%' ESCAPE '@'");
    assert!(!stmts.is_empty());
    assert!(
        errors.is_empty(),
        "LIKE ESCAPE should not produce errors, got: {:?}",
        errors
    );
}

// ========== Task 6: WINDOW clause ==========

#[test]
fn test_window_clause() {
    let stmt = parse_one("SELECT count(*) OVER w FROM t WINDOW w AS (ORDER BY id)");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            assert_eq!(s.window_clause.len(), 1);
            assert_eq!(s.window_clause[0].name, "w");
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_window_clause_multiple() {
    let stmt = parse_one(
        "SELECT count(*) OVER w1, avg(x) OVER w2 FROM t WINDOW w1 AS (ORDER BY id), w2 AS (PARTITION BY y)",
    );
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 2);
            assert_eq!(s.window_clause.len(), 2);
            assert_eq!(s.window_clause[0].name, "w1");
            assert_eq!(s.window_clause[1].name, "w2");
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_window_clause_with_frame() {
    let stmt = parse_one(
        "SELECT count(*) OVER w FROM t WINDOW w AS (ORDER BY date ASC ROWS BETWEEN 1 PRECEDING AND 1 FOLLOWING)",
    );
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.window_clause.len(), 1);
            assert!(s.window_clause[0].spec.frame.is_some());
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_window_clause_no_error() {
    let (stmts, errors) =
        parse_with_errors("SELECT count(*) OVER w FROM t WINDOW w AS (ORDER BY id)");
    assert!(!stmts.is_empty());
    assert!(
        errors.is_empty(),
        "WINDOW clause should not produce errors, got: {:?}",
        errors
    );
}

// ========== Task 8: Regex operators ~*, !~, !~* ==========

#[test]
fn test_regex_tilde_star() {
    let stmt = parse_one("SELECT 'abc' ~* 'Abc'");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_regex_not_match() {
    let stmt = parse_one("SELECT 'abc' !~ 'Abc'");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_regex_not_match_star() {
    let stmt = parse_one("SELECT 'abc' !~* 'Abc'");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_regex_in_where() {
    let stmt = parse_one("SELECT name FROM users WHERE name ~* '^admin'");
    match stmt {
        Statement::Select(s) => {
            assert!(s.where_clause.is_some());
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

// ========== Task 7: CONVERT(expr USING charset) ==========

#[test]
fn test_convert_using() {
    let stmt = parse_one("SELECT convert('asdas' USING 'gbk')");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_convert_using_ast() {
    let stmt = parse_one("SELECT convert('text_in_utf8' USING 'gbk')");
    match stmt {
        Statement::Select(s) => match &s.targets[0] {
            SelectTarget::Expr(expr, _) => match expr {
                Expr::SpecialFunction { name, args } => {
                    assert_eq!(name, "convert");
                    assert_eq!(args.len(), 2);
                }
                _ => panic!("expected SpecialFunction, got {:?}", expr),
            },
            _ => panic!("expected Expr target"),
        },
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_convert_normal() {
    let stmt = parse_one("SELECT convert('text', 'UTF8', 'GBK')");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

// ========== Task 9: WITHIN GROUP / SEPARATOR ==========

#[test]
fn test_listagg_within_group() {
    let stmt = parse_one("SELECT deptno, listagg(ename, ',') WITHIN GROUP (ORDER BY ename) AS employees FROM emp GROUP BY deptno");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 2);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_group_concat_separator() {
    let stmt = parse_one("SELECT id, group_concat(v SEPARATOR '') FROM t GROUP BY id");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 2);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_group_concat_order_by() {
    let stmt =
        parse_one("SELECT id, group_concat(v ORDER BY v DESC) FROM t GROUP BY id ORDER BY id ASC");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 2);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_group_concat_distinct() {
    let stmt = parse_one("SELECT id, group_concat(DISTINCT v) FROM t GROUP BY id ORDER BY id ASC");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 2);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_percentile_cont_within_group() {
    let stmt = parse_one("SELECT percentile_cont(0) WITHIN GROUP (ORDER BY value) FROM t");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

// ========== Task 10: Geometric operators ==========

#[test]
fn test_geo_distance() {
    let stmt = parse_one("SELECT circle '((0,0),1)' <-> circle '((5,0),1)'");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_geo_overlap() {
    let stmt = parse_one("SELECT box '((0,0),(1,1))' && box '((0,0),(2,2))'");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_geo_left_contains() {
    let stmt = parse_one("SELECT box '((0,0),(3,3))' <<| box '((3,4),(5,5))'");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_geo_right_above() {
    let stmt = parse_one("SELECT box '((3,4),(5,5))' |>> box '((0,0),(3,3))'");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_geo_below_or_equal() {
    let stmt = parse_one("SELECT box '((0,0),(1,1))' &<| box '((0,0),(2,2))'");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_geo_above_or_equal() {
    let stmt = parse_one("SELECT box '((0,0),(3,3))' |&> box '((0,0),(2,2))'");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_geo_tsquery_match() {
    let stmt = parse_one("SELECT to_tsvector('seriousness') @@ to_tsquery('series:*')");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_geo_intersect() {
    let stmt = parse_one("SELECT lseg '((-1,0),(1,0))' ?# box '((-2,-2),(2,2))'");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_geo_same() {
    let stmt = parse_one("SELECT polygon '((0,0),(1,1))' ~= polygon '((1,1),(0,0))'");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_geo_hash_bitwise_xor() {
    let stmt = parse_one("SELECT 17 # 5");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

// ========== Task 11: Network/bit operators ==========

#[test]
fn test_network_shift_left_eq() {
    let stmt = parse_one("SELECT inet '192.168.1/24' <<= inet '192.168.1/24'");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_network_shift_right_eq() {
    let stmt = parse_one("SELECT inet '192.168.1/24' >>= inet '192.168.1/24'");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_range_contains() {
    let stmt = parse_one("SELECT int4range(10, 20) @> 3");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_range_contained_by() {
    let stmt = parse_one("SELECT 3 <@ int4range(10, 20)");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_bitwise_or() {
    let stmt = parse_one("SELECT B '10001' | B '01101'");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_shift_left() {
    let stmt = parse_one("SELECT 1 << 4");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_shift_right() {
    let stmt = parse_one("SELECT 8 >> 2");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}
#[test]
fn test_grant_all_privileges_to_role() {
    let (stmts, errors) = parse_with_errors("GRANT ALL PRIVILEGES TO dev_mask");
    assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    match &stmts[0] {
        Statement::GrantRole(g) => {
            assert_eq!(g.roles, vec!["ALL PRIVILEGES"]);
            assert_eq!(g.grantees, vec!["dev_mask"]);
        }
        _ => panic!("expected GrantRole, got {:?}", stmts[0]),
    }
}

#[test]
fn test_grant_all_privileges_on_table() {
    let stmt = parse_one("GRANT ALL PRIVILEGES ON tpcds.reason TO joe");
    match stmt {
        Statement::Grant(g) => {
            assert!(g.privileges.iter().any(|p| matches!(p, Privilege::All)));
        }
        _ => panic!("expected Grant, got {:?}", stmt),
    }
}

#[test]
fn test_grant_column_level_with_grant_option() {
    let stmt = parse_one(
        "GRANT SELECT (r_reason_sk, r_reason_id) ON tpcds.reason TO joe WITH GRANT OPTION",
    );
    match stmt {
        Statement::Grant(g) => {
            assert!(g.with_grant_option);
            assert!(g
                .privileges
                .iter()
                .any(|p| matches!(p, Privilege::SelectColumns(_))));
        }
        _ => panic!("expected Grant, got {:?}", stmt),
    }
}

#[test]
fn test_grant_create_connect_on_database() {
    let stmt = parse_one("GRANT CREATE,CONNECT ON DATABASE testdb TO joe WITH GRANT OPTION");
    match stmt {
        Statement::Grant(g) => {
            assert!(g.with_grant_option);
            assert!(g.privileges.iter().any(|p| matches!(p, Privilege::Create)));
            assert!(g.privileges.iter().any(|p| matches!(p, Privilege::Connect)));
            match &g.target {
                GrantTarget::Database(dbs) => assert_eq!(dbs, &vec!["testdb"]),
                _ => panic!("expected Database target"),
            }
        }
        _ => panic!("expected Grant, got {:?}", stmt),
    }
}

#[test]
fn test_grant_alter_on_function() {
    let stmt = parse_one("GRANT ALTER ON FUNCTION tpcds.fun1() TO joe");
    match stmt {
        Statement::Grant(g) => {
            assert!(g.privileges.iter().any(|p| matches!(p, Privilege::Alter)));
            match &g.target {
                GrantTarget::Function(funcs) => {
                    assert_eq!(funcs.len(), 1);
                }
                _ => panic!("expected Function target"),
            }
        }
        _ => panic!("expected Grant, got {:?}", stmt),
    }
}

#[test]
fn test_grant_all_on_tablespace() {
    let stmt = parse_one("GRANT ALL ON TABLESPACE tpcds_tbspc TO joe");
    match stmt {
        Statement::Grant(g) => {
            assert!(g.privileges.iter().any(|p| matches!(p, Privilege::All)));
            match &g.target {
                GrantTarget::Tablespace(tbs) => assert_eq!(tbs, &vec!["tpcds_tbspc"]),
                _ => panic!("expected Tablespace target"),
            }
        }
        _ => panic!("expected Grant, got {:?}", stmt),
    }
}

#[test]
fn test_partition_dml_check() {
    let cases = vec![
        ("INSERT INTO range_list PARTITION (p_201901) VALUES('201902', '1', '1', 1)", "INSERT PARTITION name"),
        ("INSERT INTO range_list PARTITION FOR ('201902') VALUES('201902', '1', '1', 1)", "INSERT PARTITION FOR"),
        ("INSERT INTO range_list SUBPARTITION (p_201901_a) VALUES('201902', '1', '1', 1)", "INSERT SUBPARTITION name"),
        ("INSERT INTO range_list SUBPARTITION FOR ('201902','1') VALUES('201902', '1', '1', 1)", "INSERT SUBPARTITION FOR"),
        ("UPDATE range_list PARTITION (p_201901) SET user_no = '2'", "UPDATE PARTITION name"),
        ("UPDATE range_list PARTITION FOR ('201902') SET user_no = '4'", "UPDATE PARTITION FOR"),
        ("UPDATE range_list SUBPARTITION (p_201901_a) SET user_no = '3'", "UPDATE SUBPARTITION name"),
        ("UPDATE range_list SUBPARTITION FOR ('201902','2') SET user_no = '5'", "UPDATE SUBPARTITION FOR"),
        ("DELETE FROM range_list PARTITION (p_201901)", "DELETE PARTITION name"),
        ("DELETE FROM range_list PARTITION FOR ('201903')", "DELETE PARTITION FOR"),
        ("DELETE FROM range_list SUBPARTITION (p_201901_a)", "DELETE SUBPARTITION name"),
        ("DELETE FROM range_list SUBPARTITION FOR ('201903','2')", "DELETE SUBPARTITION FOR"),
        ("DELETE FROM range_list AS t PARTITION (p_201901_a, p_201901)", "DELETE alias PARTITION list"),
        ("SELECT COUNT(*) FROM tpcds.web_returns_p1 PARTITION (P10)", "SELECT PARTITION name"),
        ("SELECT COUNT(*) FROM tpcds.web_returns_p1 PARTITION FOR (2450815)", "SELECT PARTITION FOR"),
        ("UPDATE list_02 PARTITION FOR (100) SET data = ''", "UPDATE PARTITION FOR simple"),
        ("INSERT INTO range_list PARTITION (p_201901) VALUES('201902', '1', '1', 1) ON DUPLICATE KEY UPDATE sales_amt = 5", "INSERT PARTITION ON DUPLICATE"),
    ];

    let mut failures = Vec::new();
    for (sql, desc) in &cases {
        let result = std::panic::catch_unwind(|| parse_one(sql));
        match result {
            Ok(stmt) => {
                if matches!(stmt, Statement::Empty) {
                    failures.push(format!("FAIL (Empty): {} — {}", desc, sql));
                } else {
                    let (_, errors) = parse_with_errors(sql);
                    if !errors.is_empty() {
                        failures.push(format!(
                            "FAIL ({} errors): {} — {}",
                            errors.len(),
                            desc,
                            sql
                        ));
                    }
                }
            }
            Err(_) => {
                failures.push(format!("PANIC: {} — {}", desc, sql));
            }
        }
    }

    if !failures.is_empty() {
        for f in &failures {
            eprintln!("{}", f);
        }
        panic!("{} PARTITION DML test cases failed", failures.len());
    }
}

#[test]
fn test_set_statements_check() {
    let cases: Vec<&str> = vec![
        "SET datestyle = 'YMD'",
        "SET intervalstyle = a",
        "SET intervalstyle = oracle",
        "SET a_format_version = '10c'",
        "SET a_format_dev_version = 's1'",
        "set a_format_dev_version='s2'",
        "set a_format_version='10c'",
        "SET instr_unique_sql_track_type = 'all'",
        "SET track_stmt_stat_level = 'L0,L0'",
        "SET track_stmt_stat_level = 'off,L0'",
        "SET instr_unique_sql_track_type = 'top'",
        "set xmloption=content",
        "set xmloption = document",
        "SET default_text_search_config = 'ts_conf_1'",
        "SET default_text_search_config = 'public.ts_conf'",
        "SET behavior_compat_options ='plpgsql_dependency'",
        "SET DATESTYLE TO postgres, dmy",
        "SET behavior_compat_options='proc_outparam_override'",
        "set default_text_search_config = 'ts_conf_2'",
        "set plan_cache_mode = 'force_generic_plan'",
        "set enable_seqscan=off",
        "SET current_schema = HEAT_MAP_DATA",
        "set enable_hypo_index = on",
        "SET partition_iterator_elimination = on",
        "SET sql_beta_feature = 'disable_merge_append_partition'",
        "SET default_tablespace = 'fastspace'",
        "set enable_fast_query_shipping=off",
        "set enable_mergejoin=off",
        "set enable_nestloop=off",
        "set enable_sort=off",
        "SET behavior_compat_options=''",
        "set behavior_compat_options = 'rownum_type_compat'",
        "set behavior_compat_options = 'char_coerce_compat'",
        "SET behavior_compat_options='truncate_numeric_tail_zero'",
        "SET behavior_compat_options = 'enable_funcname_with_argsname'",
        "SET behavior_compat_options='proc_outparam_override,proc_outparam_transfer_length'",
        "SET behavior_compat_options = 'tableof_elem_constraints'",
        "set behavior_compat_options='current_sysdate'",
        "set behavior_compat_options='allow_function_procedure_replace'",
        "SET behavior_compat_options = 'collection_exception_backcompat'",
        "SET behavior_compat_options='enable_case_when_alias'",
        "set session AUTHORIZATION plsql_rollback2 PASSWORD '********'",
        "set behavior_compat_options='enable_use_ora_timestamptz'",
        "set gs_format_behavior_compat_options='allow_textconcat_null'",
    ];

    let mut failures = Vec::new();
    for sql in &cases {
        let result = std::panic::catch_unwind(|| parse_one(sql));
        match result {
            Ok(stmt) => {
                if matches!(stmt, Statement::Empty) {
                    failures.push(format!("FAIL (Empty): {}", sql));
                } else {
                    let (_, errors) = parse_with_errors(sql);
                    if !errors.is_empty() {
                        failures.push(format!("FAIL ({} errors): {}", errors.len(), sql));
                    }
                }
            }
            Err(_) => {
                failures.push(format!("PANIC: {}", sql));
            }
        }
    }

    if !failures.is_empty() {
        for f in &failures {
            eprintln!("{}", f);
        }
        panic!("{} SET test cases failed", failures.len());
    }
}

#[test]
fn test_set_on_off_values() {
    let (stmts, errors) = parse_with_errors("SET enable_hypo_index = on");
    assert!(
        errors.is_empty(),
        "Expected no errors for SET ... = on, got: {:?}",
        errors
    );
    match &stmts[0] {
        Statement::VariableSet(v) => assert_eq!(v.name, "enable_hypo_index"),
        _ => panic!("expected VariableSet, got {:?}", stmts[0]),
    }

    let (stmts, errors) = parse_with_errors("SET partition_iterator_elimination = on");
    assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    assert!(!stmts.is_empty());

    let (stmts, errors) = parse_with_errors("SET enable_seq_scan = off");
    assert!(
        errors.is_empty(),
        "Expected no errors for SET ... = off, got: {:?}",
        errors
    );
    assert!(!stmts.is_empty());
}

#[test]
fn test_set_role_password() {
    let stmt = parse_one("SET ROLE user01 PASSWORD '********'");
    match stmt {
        Statement::VariableSet(v) => assert_eq!(v.name.to_uppercase(), "ROLE"),
        _ => panic!("expected VariableSet, got {:?}", stmt),
    }
}

#[test]
fn test_set_search_path_to() {
    let stmt = parse_one("SET SEARCH_PATH TO ds, public");
    match stmt {
        Statement::VariableSet(v) => {
            assert_eq!(v.name.to_uppercase(), "SEARCH_PATH");
            assert_eq!(v.value.len(), 2);
        }
        _ => panic!("expected VariableSet, got {:?}", stmt),
    }
}

#[test]
fn test_set_time_zone() {
    let stmt = parse_one("SET TIME ZONE 'PST8PDT'");
    match stmt {
        Statement::VariableSet(v) => {
            assert_eq!(v.name.to_uppercase(), "TIME");
        }
        _ => panic!("expected VariableSet, got {:?}", stmt),
    }
}

#[test]
fn test_half_sql_baseline() {
    let sql = std::fs::read_to_string("GaussDB-2.23.07.210/sql/half-sql.sql").unwrap();
    let tokens = crate::Tokenizer::new(&sql).tokenize().unwrap();
    let mut parser = crate::parser::Parser::new(tokens);
    let stmts = parser.parse();
    let errors = parser.errors();

    let total = stmts.len();
    let empty = stmts
        .iter()
        .filter(|s| matches!(s, crate::ast::Statement::Empty))
        .count();
    let ok = total - empty;

    eprintln!(
        "half-sql.sql: {} total, {} OK, {} Empty, {} parser errors",
        total,
        ok,
        empty,
        errors.len()
    );

    assert!(
        ok >= 470,
        "At least 470 statements should parse OK, got {}",
        ok
    );
}

#[test]
fn test_half_sql_categorize_failures() {
    let sql = std::fs::read_to_string("GaussDB-2.23.07.210/sql/half-sql.sql").unwrap();
    let tokens = crate::Tokenizer::new(&sql).tokenize().unwrap();
    let mut parser = crate::parser::Parser::new(tokens);
    let stmts = parser.parse();

    // Re-tokenize to get line mapping
    let sql_lines: Vec<&str> = sql.lines().collect();

    // Split SQL into statements by semicolons (approximate)
    let mut fail_categories: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    // Simple approach: split by semicolons
    let mut pos = 0;
    let mut stmt_start = 0;
    let mut in_dollar = false;
    let mut dollar_tag = String::new();

    for (i, c) in sql.char_indices() {
        if c == '$' && !in_dollar {
            // Check for dollar-quote start
            let rest = &sql[i..];
            if let Some(end) = rest.find('$') {
                if end > 0 {
                    dollar_tag = rest[..end + 1].to_string();
                    in_dollar = true;
                    continue;
                }
            }
        }
        if in_dollar && c == '$' {
            let rest = &sql[i..];
            if rest.starts_with(&dollar_tag) {
                in_dollar = false;
                dollar_tag.clear();
            }
            continue;
        }
        if c == ';' && !in_dollar {
            let stmt_text = sql[stmt_start..i].trim().to_string();
            if !stmt_text.is_empty() && !stmt_text.starts_with("--") {
                // Get category (first 3 tokens)
                let first_line = stmt_text.lines().next().unwrap_or("");
                let tokens: Vec<&str> = first_line.split_whitespace().take(3).collect();
                let category = tokens.join(" ").to_uppercase();

                // Check if this was parsed as Empty (approximate - count by position)
                fail_categories
                    .entry(category)
                    .or_default()
                    .push(stmt_text.chars().take(200).collect());
            }
            stmt_start = i + 1;
        }
    }

    // Now parse and count actual failures
    let total = stmts.len();
    let empty_count = stmts
        .iter()
        .filter(|s| matches!(s, crate::ast::Statement::Empty))
        .count();

    eprintln!("\n=== half-sql.sql Failure Analysis ===");
    eprintln!(
        "Total: {}, OK: {}, Empty: {}",
        total,
        total - empty_count,
        empty_count
    );
    eprintln!("\nAll statement categories (first 3 tokens):");

    let mut sorted: Vec<_> = fail_categories.iter().collect();
    sorted.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    for (cat, stmts_list) in sorted.iter().take(40) {
        eprintln!("\n  {} ({} stmts)", cat, stmts_list.len());
        for s in stmts_list.iter().take(2) {
            eprintln!("    {}", s);
        }
    }
}

#[test]
fn test_half_sql_failure_categories() {
    let sql = std::fs::read_to_string("GaussDB-2.23.07.210/sql/half-sql.sql").unwrap();

    let mut categories: std::collections::BTreeMap<String, (usize, Vec<String>)> =
        std::collections::BTreeMap::new();
    let mut current = String::new();
    let mut in_dollar = false;
    let mut dollar_tag = String::new();
    let mut stmt_count = 0;

    for line in sql.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("--") || trimmed.starts_with("===") {
            continue;
        }

        let mut chars = trimmed.chars().collect::<Vec<_>>();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '$' {
                let mut j = i + 1;
                while j < chars.len() && (chars[j].is_alphabetic() || chars[j] == '_') {
                    j += 1;
                }
                if j < chars.len() && chars[j] == '$' {
                    let tag: String = chars[i..=j].iter().collect();
                    if !in_dollar {
                        in_dollar = true;
                        dollar_tag = tag;
                    } else if tag == dollar_tag {
                        in_dollar = false;
                        dollar_tag.clear();
                    }
                }
            }
            i += 1;
        }

        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(trimmed);

        if trimmed.ends_with(';') && !in_dollar {
            let stmt_text = current.trim().to_string();
            current.clear();

            if stmt_text.is_empty() {
                continue;
            }

            let words: Vec<&str> = stmt_text.split_whitespace().take(2).collect();
            let cat = if words.len() >= 2 {
                format!("{} {}", words[0].to_uppercase(), words[1].to_uppercase())
            } else {
                words[0].to_uppercase()
            };

            let tok_result = crate::Tokenizer::new(&stmt_text).tokenize();
            match tok_result {
                Ok(toks) => {
                    let mut p = crate::parser::Parser::new(toks);
                    let ss = p.parse();
                    let has_empty = ss.iter().any(|s| matches!(s, crate::ast::Statement::Empty));
                    let errs = p.errors().to_vec();
                    if has_empty || !errs.is_empty() {
                        let entry = categories.entry(cat).or_insert((0, Vec::new()));
                        entry.0 += 1;
                        if entry.1.len() < 3 {
                            entry.1.push(stmt_text.chars().take(150).collect());
                        }
                    }
                }
                Err(_) => {
                    let entry = categories
                        .entry(format!("TOKENIZE_ERR: {}", cat))
                        .or_insert((0, Vec::new()));
                    entry.0 += 1;
                }
            }
            stmt_count += 1;
        }
    }

    eprintln!("\n=== Failing Statement Categories ===");
    eprintln!("Total statements tested: {}", stmt_count);

    let mut sorted: Vec<_> = categories.iter().collect();
    sorted.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));

    for (cat, (count, examples)) in sorted.iter().take(30) {
        eprintln!("\n  [{}] {}", count, cat);
        for ex in examples.iter() {
            eprintln!("    {}", ex);
        }
    }
}

#[test]
fn test_create_index_failures() {
    let cases = vec![
        "CREATE INDEX index_sales ON sales(prod_id) LOCAL (PARTITION idx_p1 ,PARTITION idx_p2)",
        "CREATE INDEX index_part_tab1 ON part_tab1(b) LOCAL ( PARTITION b_index1, PARTITION b_index2, PARTITION b_index 3 )",
        "CREATE INDEX idx_user_no ON subpart_tab1(user_no) LOCAL",
        "CREATE INDEX pgweb_idx_1 ON tsearch.pgweb USING gin ( to_tsvector('english', body) )",
        "CREATE INDEX aa ON test1(col1)",
        "CREATE INDEX idx_test2_col1 ON test2(col1) LOCAL( PARTITION p1, PARTITION p2 )",
        "CREATE UNIQUE INDEX pk_test4_c1 ON test_alt4(c1)",
        "CREATE INDEX idx_test_c1_id ON test_c1 ( id )",
        "CREATE INDEX idx_test1 ON tbl_test1(name) TABLESPACE tbs_index1",
        "CREATE UNIQUE INDEX idx_test2 ON tbl_test1(id)",
        "CREATE INDEX idx_test3 ON tbl_test1(substr(postcode,2))",
        "CREATE INDEX idx_test4 ON tbl_test1(id) WHERE id IS NOT NULL",
        "CREATE INDEX idx_student1 ON student(id) LOCAL",
        "CREATE INDEX idx_student2 ON student(name) GLOBAL",
        "CREATE INDEX tpcds_web_returns_p2_index1 ON web_returns_p2 (ca_address_id) LOCAL",
        "CREATE INDEX tpcds_web_returns_p2_index2 ON web_returns_p2 (ca_address_sk) LOCAL ( PARTITION web_returns_p2_P1_index, PARTITION web_returns_p2_P2_index TABLESPACE example3 ) TABLESPACE example2",
        "CREATE INDEX tpcds_web_returns_p2_global_index ON web_returns_p2 (ca_street_number) GLOBAL",
        "CREATE INDEX tpcds_web_returns_for_p1 ON web_returns_p2 (ca_address_id) LOCAL(partition ind_part for p1)",
        "CREATE INDEX tpcds_web_returns_for_p2 ON web_returns_p2 (ca_address_id) LOCAL(partition ind_part for (5000))",
        "create index t1_range_int_index on t1_range_int(text(c1)) local",
        "create index idx1 on table1 using gin ( to_tsvector(c_text) )",
        "CREATE UNIQUE INDEX ds_reason_index1 ON tpcds.reason(r_reason_sk)",
    ];

    let mut failures = Vec::new();
    for sql in &cases {
        let result = std::panic::catch_unwind(|| parse_one(sql));
        match result {
            Ok(stmt) => {
                if matches!(stmt, Statement::Empty) {
                    failures.push(format!("FAIL (Empty): {}", sql));
                } else {
                    let (_, errors) = parse_with_errors(sql);
                    if !errors.is_empty() {
                        failures.push(format!("FAIL ({} errors): {}", errors.len(), sql));
                    }
                }
            }
            Err(_) => {
                failures.push(format!("PANIC: {}", sql));
            }
        }
    }

    if !failures.is_empty() {
        for f in &failures {
            eprintln!("{}", f);
        }
        panic!("{} CREATE INDEX test cases failed", failures.len());
    }
}

#[test]
fn test_create_resource_label() {
    let sql = "CREATE RESOURCE LABEL mask_lb1 ADD COLUMN ( tb_for_masking . col1 )";
    let (stmts, errors) = parse_with_errors(sql);
    assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    match &stmts[0] {
        Statement::CreatePolicyLabel(p) => {
            assert_eq!(p.name, "mask_lb1");
            assert!(p.add);
        }
        _ => panic!("expected CreatePolicyLabel, got {:?}", stmts[0]),
    }

    let sql2 = "ALTER RESOURCE LABEL table_label ADD COLUMN ( table_for_label . col2 )";
    let (stmts2, errors2) = parse_with_errors(sql2);
    assert!(errors2.is_empty(), "Expected no errors, got: {:?}", errors2);
    match &stmts2[0] {
        Statement::AlterPolicyLabel(p) => {
            assert_eq!(p.name, "table_label");
            assert!(p.add);
        }
        _ => panic!("expected AlterPolicyLabel, got {:?}", stmts2[0]),
    }
}

#[test]
fn test_alter_index_failures() {
    let cases = vec![
        "ALTER INDEX aa RENAME TO idx_test1_col1",
        "ALTER INDEX IF EXISTS idx_test1_col1 SET TABLESPACE tbs_index1",
        "ALTER INDEX IF EXISTS idx_test1_col1 SET (FILLFACTOR = 70)",
        "ALTER INDEX IF EXISTS idx_test1_col1 RESET (FILLFACTOR)",
        "ALTER INDEX IF EXISTS idx_test1_col1 UNUSABLE",
        "ALTER INDEX idx_test1_col1 REBUILD",
        "ALTER INDEX idx_test2_col1 RENAME PARTITION p1 TO p1_test2_idx",
        "ALTER INDEX idx_test2_col1 MOVE PARTITION p1_test2_idx TABLESPACE tbs_index2",
        "ALTER INDEX tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P2_index TABLESPACE example1",
        "ALTER INDEX tpcds_web_returns_p2_index2 RENAME PARTITION web_returns_p2_P8_index TO web_returns_p2_P8_index_new",
        "ALTER INDEX tpcds.tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P2_index TABLESPACE example1",
    ];

    let mut failures = Vec::new();
    for sql in &cases {
        let result = std::panic::catch_unwind(|| parse_one(sql));
        match result {
            Ok(stmt) => {
                if matches!(stmt, Statement::Empty) {
                    failures.push(format!("FAIL (Empty): {}", sql));
                } else {
                    let (_, errors) = parse_with_errors(sql);
                    if !errors.is_empty() {
                        failures.push(format!("FAIL ({} errors): {}", errors.len(), sql));
                    }
                }
            }
            Err(_) => {
                failures.push(format!("PANIC: {}", sql));
            }
        }
    }

    if !failures.is_empty() {
        for f in &failures {
            eprintln!("{}", f);
        }
        panic!("{} ALTER INDEX test cases failed", failures.len());
    }
}

#[test]
fn test_select_expr_failures() {
    let cases = vec![
        "SELECT 8000 + 500 IN ( 10000 , 9000 ) AS RESULT",
        "SELECT 8000 + 500 NOT IN ( 10000 , 9000 ) AS RESULT",
        "SELECT 8000 + 500 < SOME ( array [ 10000 , 9000 ]) AS RESULT",
        "SELECT 8000 + 500 < ANY ( array [ 10000 , 9000 ]) AS RESULT",
        "SELECT 8000 + 500 < ALL ( array [ 10000 , 9000 ]) AS RESULT",
    ];

    let mut failures = Vec::new();
    for sql in &cases {
        let result = std::panic::catch_unwind(|| parse_one(sql));
        match result {
            Ok(stmt) => {
                if matches!(stmt, Statement::Empty) {
                    failures.push(format!("FAIL (Empty): {}", sql));
                } else {
                    let (_, errors) = parse_with_errors(sql);
                    if !errors.is_empty() {
                        failures.push(format!("FAIL ({} errors): {}", errors.len(), sql));
                    }
                }
            }
            Err(_) => {
                failures.push(format!("PANIC: {}", sql));
            }
        }
    }

    if !failures.is_empty() {
        for f in &failures {
            eprintln!("{}", f);
        }
        panic!("{} SELECT expr test cases failed", failures.len());
    }
}

#[test]
fn test_select_quantified_comparison() {
    let sql = "SELECT 8000 + 500 < SOME ( array [ 10000 , 9000 ]) AS RESULT";
    let stmt = parse_one(sql);
    assert!(!matches!(stmt, Statement::Empty));

    let sql2 = "SELECT 8000 + 500 < ANY ( array [ 10000 , 9000 ]) AS RESULT";
    let stmt2 = parse_one(sql2);
    assert!(!matches!(stmt2, Statement::Empty));

    let sql3 = "SELECT 8000 + 500 < ALL ( array [ 10000 , 9000 ]) AS RESULT";
    let stmt3 = parse_one(sql3);
    assert!(!matches!(stmt3, Statement::Empty));
}

#[test]
fn test_alter_index_set_unusable_rebuild() {
    let cases = vec![
        "ALTER INDEX IF EXISTS idx_test1_col1 SET (FILLFACTOR = 70)",
        "ALTER INDEX IF EXISTS idx_test1_col1 UNUSABLE",
        "ALTER INDEX idx_test1_col1 REBUILD",
    ];
    for sql in &cases {
        let (stmts, errors) = parse_with_errors(sql);
        let is_empty = stmts.iter().any(|s| matches!(s, Statement::Empty));
        if is_empty || !errors.is_empty() {
            panic!("FAIL: {} — Empty: {}, Errors: {:?}", sql, is_empty, errors);
        }
    }
}

#[test]
fn test_alter_index_set_options() {
    let sql = "ALTER INDEX IF EXISTS idx_test1_col1 SET (FILLFACTOR = 70)";
    let (stmts, errors) = parse_with_errors(sql);
    assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Statement::AlterIndex(a) => match &a.action {
            AlterIndexAction::Set(opts) => {
                assert_eq!(opts.len(), 1);
                assert_eq!(opts[0].0, "FILLFACTOR");
            }
            other => panic!("expected Set, got {:?}", other),
        },
        _ => panic!("expected AlterIndex, got {:?}", stmts[0]),
    }
}

#[test]
fn test_alter_index_unusable_rebuild() {
    let (stmts, errors) = parse_with_errors("ALTER INDEX IF EXISTS idx_test1_col1 UNUSABLE");
    assert!(errors.is_empty());
    match &stmts[0] {
        Statement::AlterIndex(a) => assert!(matches!(a.action, AlterIndexAction::Unusable)),
        _ => panic!("expected AlterIndex"),
    }

    let (stmts, errors) = parse_with_errors("ALTER INDEX idx_test1_col1 REBUILD");
    assert!(errors.is_empty());
    match &stmts[0] {
        Statement::AlterIndex(a) => assert!(matches!(a.action, AlterIndexAction::Rebuild)),
        _ => panic!("expected AlterIndex"),
    }
}

fn test_set_role_with_password() {
    let sql = "SET ROLE user01 PASSWORD '********'";
    let (stmts, errors) = parse_with_errors(sql);
    assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    match &stmts[0] {
        Statement::VariableSet(v) => assert_eq!(v.name.to_uppercase(), "ROLE"),
        _ => panic!("expected VariableSet, got {:?}", stmts[0]),
    }

    let sql2 = "SET role dev_mask PASSWORD '********'";
    let (_stmts2, errors2) = parse_with_errors(sql2);
    assert!(errors2.is_empty(), "Expected no errors, got: {:?}", errors2);
}

#[test]
fn test_function_default_on_conversion_error() {
    use crate::formatter::SqlFormatter;
    let cases = vec![
        (
            "SELECT to_date('12-jan-2022' default '12-apr-2022' on conversion error)",
            "SELECT to_date('12-jan-2022' DEFAULT '12-apr-2022' ON CONVERSION ERROR)",
        ),
        (
            "SELECT to_date('2022-12-12' default '2022-01-01' on conversion error, 'yyyy-mm-dd')",
            "SELECT to_date('2022-12-12' DEFAULT '2022-01-01' ON CONVERSION ERROR, 'yyyy-mm-dd')",
        ),
        (
            "SELECT to_number('123' default '456-' on conversion error, '999MI')",
            "SELECT to_number('123' DEFAULT '456-' ON CONVERSION ERROR, '999MI')",
        ),
        (
            "SELECT to_timestamp('11-Sep-11' DEFAULT '12-Sep-10 14:10:10.123000' ON CONVERSION ERROR, 'DD-Mon-YY HH24:MI:SS.FF')",
            "SELECT to_timestamp('11-Sep-11' DEFAULT '12-Sep-10 14:10:10.123000' ON CONVERSION ERROR, 'DD-Mon-YY HH24:MI:SS.FF')",
        ),
    ];
    for (input, expected) in cases {
        let stmt = parse_one(input);
        let formatted = SqlFormatter::new().format_statement(&stmt);
        assert_eq!(formatted, expected, "input: {}", input);
    }
}

#[test]
fn test_function_single_arg_overloads() {
    let cases = vec![
        "SELECT to_date('2015-08-14')",
        "SELECT to_char(site) FROM employee",
        "SELECT to_timestamp(200120400)",
    ];
    for sql in cases {
        let (stmts, errors) = parse_with_errors(sql);
        assert!(
            errors.is_empty(),
            "Unexpected errors for '{}': {:?}",
            sql,
            errors
        );
        assert_eq!(stmts.len(), 1, "Expected 1 statement for '{}'", sql);
    }
}

// ── Array type and CHARACTER VARYING tests ──

#[test]
fn test_array_types_simple() {
    let sql = "CREATE TABLE t (a int[], b text[])";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateTable(t) => {
            assert_eq!(t.columns.len(), 2);
            assert!(matches!(
                t.columns[0].data_type,
                DataType::Array(ref inner) if matches!(**inner, DataType::Integer(None))
            ));
            assert!(matches!(
                t.columns[1].data_type,
                DataType::Array(ref inner) if matches!(**inner, DataType::Text)
            ));
        }
        _ => panic!("expected CreateTable, got {:?}", stmt),
    }
}

#[test]
fn test_array_type_varchar_param() {
    let sql = "CREATE TABLE t (a varchar(100)[])";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateTable(t) => {
            assert_eq!(t.columns.len(), 1);
            match &t.columns[0].data_type {
                DataType::Array(inner) => match **inner {
                    DataType::Varchar(Some(100)) => {}
                    ref other => panic!("expected Varchar(Some(100)), got {:?}", other),
                },
                other => panic!("expected Array, got {:?}", other),
            }
        }
        _ => panic!("expected CreateTable, got {:?}", stmt),
    }
}

#[test]
fn test_array_type_multi_dimensional() {
    let sql = "CREATE TABLE t (a int[][])";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateTable(t) => {
            assert_eq!(t.columns.len(), 1);
            match &t.columns[0].data_type {
                DataType::Array(outer) => match **outer {
                    DataType::Array(ref inner) => {
                        assert!(matches!(**inner, DataType::Integer(None)));
                    }
                    ref other => panic!("expected nested Array, got {:?}", other),
                },
                other => panic!("expected Array, got {:?}", other),
            }
        }
        _ => panic!("expected CreateTable, got {:?}", stmt),
    }
}

#[test]
fn test_character_type() {
    let sql = "CREATE TABLE t (a character(10))";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateTable(t) => {
            assert_eq!(t.columns.len(), 1);
            match &t.columns[0].data_type {
                DataType::Char(Some(10)) => {}
                other => panic!("expected Char(Some(10)), got {:?}", other),
            }
        }
        _ => panic!("expected CreateTable, got {:?}", stmt),
    }
}

#[test]
fn test_character_varying() {
    let sql = "CREATE TABLE t (a character varying(100))";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateTable(t) => {
            assert_eq!(t.columns.len(), 1);
            match &t.columns[0].data_type {
                DataType::Varchar(Some(100)) => {}
                other => panic!("expected Varchar(Some(100)), got {:?}", other),
            }
        }
        _ => panic!("expected CreateTable, got {:?}", stmt),
    }
}

#[test]
fn test_character_varying_no_length() {
    let sql = "CREATE TABLE t (a character varying)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateTable(t) => {
            assert_eq!(t.columns.len(), 1);
            match &t.columns[0].data_type {
                DataType::Varchar(None) => {}
                other => panic!("expected Varchar(None), got {:?}", other),
            }
        }
        _ => panic!("expected CreateTable, got {:?}", stmt),
    }
}

#[test]
fn test_character_no_length() {
    let sql = "CREATE TABLE t (a character)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateTable(t) => {
            assert_eq!(t.columns.len(), 1);
            match &t.columns[0].data_type {
                DataType::Char(None) => {}
                other => panic!("expected Char(None), got {:?}", other),
            }
        }
        _ => panic!("expected CreateTable, got {:?}", stmt),
    }
}

#[test]
fn test_array_type_formatter_roundtrip() {
    let cases = vec![
        ("CREATE TABLE t (a INT[])", "CREATE TABLE t (a INTEGER[])"),
        ("CREATE TABLE t (a TEXT[])", "CREATE TABLE t (a TEXT[])"),
        (
            "CREATE TABLE t (a VARCHAR(100)[])",
            "CREATE TABLE t (a VARCHAR(100)[])",
        ),
    ];
    for (input, expected) in cases {
        let stmt = parse_one(input);
        let formatted = SqlFormatter::new().format_statement(&stmt);
        assert_eq!(formatted, expected, "input: {}", input);
    }
}

#[test]
fn test_character_varying_formatter_roundtrip() {
    let cases = vec![
        (
            "CREATE TABLE t (a CHARACTER(10))",
            "CREATE TABLE t (a CHAR(10))",
        ),
        (
            "CREATE TABLE t (a CHARACTER VARYING(100))",
            "CREATE TABLE t (a VARCHAR(100))",
        ),
    ];
    for (input, expected) in cases {
        let stmt = parse_one(input);
        let formatted = SqlFormatter::new().format_statement(&stmt);
        assert_eq!(formatted, expected, "input: {}", input);
    }
}

#[test]
fn test_cast_array_type() {
    let sql = "SELECT CAST(x AS int[]) FROM t";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_sec_label_on_role() {
    let stmt = parse_one("SECURITY LABEL ON ROLE bob IS 'sec_label'");
    match stmt {
        Statement::SecLabel(s) => {
            assert_eq!(s.object_type, "role");
            assert_eq!(s.name, vec!["bob".to_string()]);
            assert_eq!(s.label.as_deref(), Some("sec_label"));
        }
        other => panic!("expected SecLabel, got {:?}", other),
    }
}

#[test]
fn test_sec_label_on_table() {
    let stmt = parse_one("SECURITY LABEL ON TABLE my_table IS 'classified'");
    match stmt {
        Statement::SecLabel(s) => {
            assert_eq!(s.object_type, "table");
            assert_eq!(s.name, vec!["my_table".to_string()]);
            assert_eq!(s.label.as_deref(), Some("classified"));
        }
        other => panic!("expected SecLabel, got {:?}", other),
    }
}

#[test]
fn test_prefix_at_at() {
    let sql = "SELECT @@ circle '((0,0),10)' AS RESULT";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            let target = &s.targets[0];
            match target {
                SelectTarget::Expr(expr, _) => match expr {
                    Expr::UnaryOp { op, .. } => {
                        assert_eq!(op, "@@");
                    }
                    other => panic!("expected UnaryOp, got {:?}", other),
                },
                other => panic!("expected Expr target, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_geometric_lt_caret() {
    let tokens = Tokenizer::new("SELECT box '..' <^ box '..'")
        .tokenize()
        .unwrap();
    let has_op = tokens
        .iter()
        .any(|tws| matches!(&tws.token, Token::Op(op) if op == "<^"));
    assert!(has_op, "expected <^ operator token");
    let stmt = parse_one("SELECT box '..' <^ box '..' AS RESULT");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_geometric_gt_caret() {
    let tokens = Tokenizer::new("SELECT box '..' >^ box '..'")
        .tokenize()
        .unwrap();
    let has_op = tokens
        .iter()
        .any(|tws| matches!(&tws.token, Token::Op(op) if op == ">^"));
    assert!(has_op, "expected >^ operator token");
    let stmt = parse_one("SELECT box '..' >^ box '..' AS RESULT");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_range_adjacent_op() {
    let tokens = Tokenizer::new("SELECT numrange(1.1,2.2) -|- numrange(2.2,3.3)")
        .tokenize()
        .unwrap();
    let has_op = tokens
        .iter()
        .any(|tws| matches!(&tws.token, Token::Op(op) if op == "-|-"));
    assert!(has_op, "expected -|- operator token");
    let stmt = parse_one("SELECT numrange(1.1,2.2) -|- numrange(2.2,3.3) AS RESULT");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_values_in_from() {
    let sql = "SELECT * FROM (VALUES (1), (2)) AS v(value)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.from.len(), 1);
            match &s.from[0] {
                TableRef::Values { alias, .. } => {
                    assert_eq!(alias.as_deref(), Some("v"));
                }
                other => panic!("expected Values table ref, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_values_in_from_multi_row() {
    let sql = "SELECT * FROM (VALUES (1, 'a'), (2, 'b'), (3, 'c')) AS t(id, name)";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.from.len(), 1);
            match &s.from[0] {
                TableRef::Values {
                    values,
                    alias,
                    column_names: _,
                } => {
                    assert_eq!(alias.as_deref(), Some("t"));
                    assert_eq!(values.rows.len(), 3);
                    assert_eq!(values.rows[0].len(), 2);
                    assert_eq!(values.rows[1].len(), 2);
                    assert_eq!(values.rows[2].len(), 2);
                }
                other => panic!("expected Values table ref, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

// ========== TEMPORARY DEBUG TESTS ==========
#[test]
fn test_debug_drop_synonym() {
    let sql = "DROP SYNONYM t1;";
    let (infos, errors) = Parser::parse_sql(sql);
    if !errors.is_empty() {
        for e in &errors {
            eprintln!("ERROR: {:?}", e);
        }
        panic!("DROP SYNONYM failed");
    }
    match &infos[0].statement {
        Statement::Drop(d) => {
            assert_eq!(d.object_type, ObjectType::Synonym);
        }
        _ => panic!("expected Drop, got {:?}", infos[0].statement),
    }
}

#[test]
fn test_debug_drop_public_database_link() {
    let sql = "DROP PUBLIC DATABASE LINK public_dblink;";
    let (infos, errors) = Parser::parse_sql(sql);
    if !errors.is_empty() {
        for e in &errors {
            eprintln!("ERROR: {:?}", e);
        }
        panic!("DROP PUBLIC DATABASE LINK failed");
    }
}

#[test]
fn test_debug_drop_database_link() {
    let sql = "DROP DATABASE LINK private_dblink;";
    let (infos, errors) = Parser::parse_sql(sql);
    if !errors.is_empty() {
        for e in &errors {
            eprintln!("ERROR: {:?}", e);
        }
        panic!("DROP DATABASE LINK failed");
    }
}

#[test]
fn test_debug_drop_user_mapping() {
    let sql = "DROP USER MAPPING FOR bob SERVER my_server;";
    let (infos, errors) = Parser::parse_sql(sql);
    if !errors.is_empty() {
        for e in &errors {
            eprintln!("ERROR: {:?}", e);
        }
        panic!("DROP USER MAPPING failed");
    }
}

#[test]
fn test_debug_create_public_database_link() {
    let sql = "CREATE PUBLIC DATABASE LINK public_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING 'host:port/db';";
    let (infos, errors) = Parser::parse_sql(sql);
    if !errors.is_empty() {
        for e in &errors {
            eprintln!("ERROR: {:?}", e);
        }
        panic!("CREATE PUBLIC DATABASE LINK failed");
    }
}

#[test]
fn test_debug_create_database_link() {
    let sql = "CREATE DATABASE LINK private_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING 'host:port/db';";
    let (infos, errors) = Parser::parse_sql(sql);
    if !errors.is_empty() {
        for e in &errors {
            eprintln!("ERROR: {:?}", e);
        }
        panic!("CREATE DATABASE LINK failed");
    }
}

#[test]
fn test_debug_alter_table_modify_first() {
    let sql = "ALTER TABLE tbl_test MODIFY COLUMN name varchar(25) FIRST;";
    let (infos, errors) = Parser::parse_sql(sql);
    if !errors.is_empty() {
        for e in &errors {
            eprintln!("ERROR: {:?}", e);
        }
        panic!("ALTER TABLE MODIFY COLUMN FIRST failed");
    }
}

#[test]
fn test_debug_alter_table_modify_after() {
    let sql = "ALTER TABLE tbl_test MODIFY COLUMN name varchar(10) AFTER id;";
    let (infos, errors) = Parser::parse_sql(sql);
    if !errors.is_empty() {
        for e in &errors {
            eprintln!("ERROR: {:?}", e);
        }
        panic!("ALTER TABLE MODIFY COLUMN AFTER failed");
    }
}

#[test]
fn test_debug_alter_table_if_exists_star() {
    let sql = "ALTER TABLE IF EXISTS tb5 * ADD COLUMN IF NOT EXISTS c2 char(5) after c1;";
    let (infos, errors) = Parser::parse_sql(sql);
    if !errors.is_empty() {
        for e in &errors {
            eprintln!("ERROR: {:?}", e);
        }
        panic!("ALTER TABLE IF EXISTS * failed");
    }
}

// ========== CREATE/ALTER MASKING POLICY Tests ==========

#[test]
fn test_create_masking_policy_with_function_args() {
    let sql = r"CREATE MASKING POLICY maskpol7 regexpmasking ( '[\d+]' , '*' , 2 , 9 ) ON LABEL ( mask_lb7 );";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateMaskingPolicy(s) => {
            assert_eq!(s.name, "maskpol7");
            assert_eq!(s.masking_function.as_deref(), Some("regexpmasking"));
            assert_eq!(s.function_args.len(), 4);
            assert_eq!(s.labels, vec!["mask_lb7"]);
        }
        _ => panic!("expected CreateMaskingPolicy, got {:?}", stmt),
    }
}

#[test]
fn test_create_masking_policy_with_filter() {
    let sql = "CREATE MASKING POLICY maskpol8 randommasking ON LABEL ( mask_lb8 ) FILTER ON ROLES ( dev_mask , bob_mask ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' );";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateMaskingPolicy(s) => {
            assert_eq!(s.name, "maskpol8");
            assert_eq!(s.masking_function.as_deref(), Some("randommasking"));
            assert_eq!(s.labels, vec!["mask_lb8"]);
            assert_eq!(s.filter_clauses.len(), 3);
            assert_eq!(s.filter_clauses[0].kind, "ROLES");
            assert_eq!(s.filter_clauses[0].values, vec!["dev_mask", "bob_mask"]);
            assert_eq!(s.filter_clauses[1].kind, "APP");
            assert_eq!(s.filter_clauses[1].values, vec!["gsql"]);
            assert_eq!(s.filter_clauses[2].kind, "IP");
            assert_eq!(
                s.filter_clauses[2].values,
                vec!["10.20.30.40", "127.0.0.0/24"]
            );
        }
        _ => panic!("expected CreateMaskingPolicy, got {:?}", stmt),
    }
}

#[test]
fn test_alter_masking_policy_modify_filter() {
    let sql = "ALTER MASKING POLICY maskpol1 MODIFY ( FILTER ON ROLES ( dev_mask , bob_mask ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' ));";
    let stmt = parse_one(sql);
    match stmt {
        Statement::AlterMaskingPolicy(s) => {
            assert_eq!(s.name, "maskpol1");
            match &s.action {
                AlterMaskingPolicyAction::ModifyFilter { filter_clauses } => {
                    assert_eq!(filter_clauses.len(), 3);
                    assert_eq!(filter_clauses[0].kind, "ROLES");
                    assert_eq!(filter_clauses[0].values, vec!["dev_mask", "bob_mask"]);
                    assert_eq!(filter_clauses[1].kind, "APP");
                    assert_eq!(filter_clauses[1].values, vec!["gsql"]);
                    assert_eq!(filter_clauses[2].kind, "IP");
                    assert_eq!(
                        filter_clauses[2].values,
                        vec!["10.20.30.40", "127.0.0.0/24"]
                    );
                }
                other => panic!("expected ModifyFilter action, got {:?}", other),
            }
        }
        _ => panic!("expected AlterMaskingPolicy, got {:?}", stmt),
    }
}

#[test]
fn test_create_masking_policy_basic() {
    let sql = "CREATE MASKING POLICY maskpol1 maskall ON LABEL ( mask_lb1 );";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateMaskingPolicy(s) => {
            assert_eq!(s.name, "maskpol1");
            assert_eq!(s.masking_function.as_deref(), Some("maskall"));
            assert_eq!(s.function_args.len(), 0);
            assert_eq!(s.labels, vec!["mask_lb1"]);
            assert_eq!(s.filter_clauses.len(), 0);
        }
        _ => panic!("expected CreateMaskingPolicy, got {:?}", stmt),
    }
}

// ========== PREDICT BY Expression Tests ==========

#[test]
fn test_predict_by_basic() {
    let sql = "SELECT id, PREDICT BY price_model (FEATURES size,lot) FROM houses";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 2);
            match &s.targets[1] {
                SelectTarget::Expr(expr, None) => match expr {
                    Expr::PredictBy {
                        model_name,
                        features,
                    } => {
                        assert_eq!(model_name, "price_model");
                        assert_eq!(features.len(), 2);
                    }
                    _ => panic!("expected PredictBy expression"),
                },
                _ => panic!("expected Expr target"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_predict_by_with_alias() {
    let sql = r#"SELECT id, PREDICT BY iris_classification (FEATURES sepal_length,sepal_width,petal_length,sepal_width) as "PREDICT" FROM tb_iris limit 3"#;
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 2);
            match &s.targets[1] {
                SelectTarget::Expr(expr, alias) => {
                    assert_eq!(alias.as_deref(), Some("PREDICT"));
                    match expr {
                        Expr::PredictBy {
                            model_name,
                            features,
                        } => {
                            assert_eq!(model_name, "iris_classification");
                            assert_eq!(features.len(), 4);
                        }
                        _ => panic!("expected PredictBy expression"),
                    }
                }
                _ => panic!("expected Expr target with alias"),
            }
            match &s.limit {
                Some(Expr::Literal(Literal::Integer(3))) => {}
                _ => panic!("expected LIMIT 3"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_predict_by_two_features() {
    let sql = "select id, PREDICT BY patient_logistic_regression (FEATURES second_attack,treatment) FROM patients";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => match &s.targets[1] {
            SelectTarget::Expr(
                Expr::PredictBy {
                    model_name,
                    features,
                },
                None,
            ) => {
                assert_eq!(model_name, "patient_logistic_regression");
                assert_eq!(features.len(), 2);
            }
            _ => panic!("expected PredictBy with 2 features"),
        },
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_predict_by_single_feature() {
    let sql =
        "select id, PREDICT BY patient_linear_regression (FEATURES second_attack) FROM patients";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => match &s.targets[1] {
            SelectTarget::Expr(
                Expr::PredictBy {
                    model_name,
                    features,
                },
                None,
            ) => {
                assert_eq!(model_name, "patient_linear_regression");
                assert_eq!(features.len(), 1);
            }
            _ => panic!("expected PredictBy with 1 feature"),
        },
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_predict_by_with_numeric_feature() {
    let sql = "select id, PREDICT BY patient_linear_regression (FEATURES 1,second_attack,treatment) FROM patients";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            match &s.targets[1] {
                SelectTarget::Expr(
                    Expr::PredictBy {
                        model_name,
                        features,
                    },
                    None,
                ) => {
                    assert_eq!(model_name, "patient_linear_regression");
                    assert_eq!(features.len(), 3);
                    // First feature is numeric literal
                    assert!(matches!(&features[0], Expr::Literal(Literal::Integer(1))));
                }
                _ => panic!("expected PredictBy with numeric feature"),
            }
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_predict_by_format_roundtrip() {
    use crate::formatter::SqlFormatter;
    let sql = "SELECT id, PREDICT BY price_model (FEATURES size, lot) FROM houses";
    let stmt = parse_one(sql);
    let formatted = SqlFormatter::new().format_statement(&stmt);
    let stmt2 = parse_one(&formatted);
    assert_eq!(stmt, stmt2);
}

#[test]
fn test_predict_by_json_roundtrip() {
    let sql = "SELECT id, PREDICT BY price_model (FEATURES size, lot) FROM houses";
    let stmt = parse_one(sql);
    assert_eq!(stmt, json_roundtrip(&stmt));
}

// ── False-positive reserved keyword warning suppression tests ──

fn assert_no_reserved_keyword_warnings(sql: &str) {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty(), "Should produce AST: {}", sql);
    let reserved_warnings: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        reserved_warnings.is_empty(),
        "Unexpected reserved keyword warnings for: {}\nWarnings: {:?}",
        sql,
        reserved_warnings
    );
}

#[test]
fn test_set_option_on_not_warning() {
    assert_no_reserved_keyword_warnings("ALTER TABLE t1 SET (enable_tde = on)");
}

#[test]
fn test_security_label_user_not_warning() {
    assert_no_reserved_keyword_warnings("SECURITY LABEL ON USER user1 IS 'label1'");
}

#[test]
fn test_analyze_with_all_not_warning() {
    assert_no_reserved_keyword_warnings("ANALYZE t1_range_int WITH all");
}

#[test]
fn test_alter_modify_on_update_localtimestamp_not_warning() {
    assert_no_reserved_keyword_warnings(
        "ALTER TABLE tb2 MODIFY COLUMN c2 time without time zone ON UPDATE LOCALTIMESTAMP",
    );
}

#[test]
fn test_explain_analyze_on_not_warning() {
    assert_no_reserved_keyword_warnings("EXPLAIN (analyze on, costs off) SELECT * FROM t1");
}

#[test]
fn test_select_current_role_not_warning() {
    assert_no_reserved_keyword_warnings("SELECT CURRENT_ROLE");
}

#[test]
fn test_create_user_mapping_options_user_not_warning() {
    assert_no_reserved_keyword_warnings(
        "CREATE USER MAPPING FOR bob SERVER my_server OPTIONS (user 'bob', password 'secret')",
    );
}

#[test]
fn test_set_option_off_not_warning() {
    assert_no_reserved_keyword_warnings("ALTER TABLE t1 SET (enable_tde = off)");
}

#[test]
fn test_generic_options_with_reserved_keyword_key() {
    assert_no_reserved_keyword_warnings(
        "CREATE SERVER my_server FOREIGN DATA WRAPPER fdw OPTIONS (user 'bob')",
    );
}

#[test]
fn test_true_reserved_keyword_as_table_name_still_warns() {
    let sql = "SELECT * FROM select";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let reserved_errors: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        !reserved_errors.is_empty(),
        "True misuse: 'select' as table name should still warn"
    );
}

#[test]
fn test_true_reserved_keyword_as_column_name_still_warns() {
    let sql = "SELECT where FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let reserved_errors: Vec<_> = parser
        .errors()
        .iter()
        .filter(|e| matches!(e, ParserError::ReservedKeywordAsIdentifier { .. }))
        .collect();
    assert!(
        !reserved_errors.is_empty(),
        "True misuse: 'where' as column name should still warn"
    );
}

#[test]
fn test_alter_table_add_constraint_pk_using_index() {
    let sql = "ALTER TABLE t ADD CONSTRAINT pk_t PRIMARY KEY (id) USING INDEX idx_t PCTFREE 10 INITRANS 2 MAXTRANS 255";
    let stmt = parse_one(sql);
    match stmt {
        Statement::AlterTable(AlterTableStatement { actions, .. }) => {
            assert_eq!(actions.len(), 1);
            match &actions[0] {
                AlterTableAction::AddConstraint { name, constraint } => {
                    assert_eq!(name.as_deref(), Some("pk_t"));
                    match constraint {
                        TableConstraint::PrimaryKey {
                            columns,
                            using_index,
                        } => {
                            assert_eq!(*columns, vec!["id".to_string()]);
                            assert!(using_index.is_some());
                            assert!(using_index.as_ref().unwrap().contains("idx_t"));
                        }
                        _ => panic!("expected PrimaryKey, got {:?}", constraint),
                    }
                }
                _ => panic!("expected AddConstraint, got {:?}", actions[0]),
            }
        }
        _ => panic!("expected AlterTable, got {:?}", stmt),
    }
}

#[test]
fn test_create_table_with_storage_params() {
    let sql = "CREATE TABLE t (id INT, code VARCHAR(1)) PCTFREE 10 INITRANS 2 MAXTRANS 255";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateTable(CreateTableStatement { table_options, .. }) => {
            let keys: Vec<&str> = table_options.iter().map(|(k, _)| k.as_str()).collect();
            assert!(
                keys.contains(&"PCTFREE"),
                "expected PCTFREE in table_options"
            );
            assert!(
                keys.contains(&"INITRANS"),
                "expected INITRANS in table_options"
            );
            assert!(
                keys.contains(&"MAXTRANS"),
                "expected MAXTRANS in table_options"
            );
        }
        _ => panic!("expected CreateTable, got {:?}", stmt),
    }
}

#[test]
fn test_create_index_with_storage_params() {
    let sql = "CREATE INDEX ind1 ON t1 (part_id) INITRANS 2 MAXTRANS 255";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateIndex(CreateIndexStatement { table, columns, .. }) => {
            assert_eq!(table, vec!["t1".to_string()]);
            assert_eq!(columns.len(), 1);
        }
        _ => panic!("expected CreateIndex, got {:?}", stmt),
    }
}

#[test]
fn test_create_index_with_pctfree_and_tablespace() {
    let sql = "CREATE INDEX idx ON t1 (c1) PCTFREE 20 TABLESPACE pg_default";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateIndex(CreateIndexStatement { tablespace, .. }) => {
            assert!(tablespace.is_some());
        }
        _ => panic!("expected CreateIndex, got {:?}", stmt),
    }
}

#[test]
fn test_alter_index_storage_params() {
    let sql = "ALTER INDEX idx PCTFREE 20 INITRANS 4 MAXTRANS 255";
    let stmt = parse_one(sql);
    match stmt {
        Statement::AlterIndex(AlterIndexStatement { name, action, .. }) => {
            assert_eq!(name, vec!["idx".to_string()]);
            assert!(matches!(action, AlterIndexAction::NoOp));
        }
        _ => panic!("expected AlterIndex, got {:?}", stmt),
    }
}

#[test]
fn test_create_table_inline_constraint_using_index_no_name() {
    let sql = "CREATE TABLE t2 (id INT, CONSTRAINT PK_A PRIMARY KEY (id) USING INDEX PCTFREE 10 INITRANS 2 MAXTRANS 255) NOCOMPRESS";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreateTable(CreateTableStatement {
            constraints,
            compress,
            ..
        }) => {
            assert_eq!(constraints.len(), 1);
            match &constraints[0] {
                TableConstraint::PrimaryKey {
                    columns,
                    using_index,
                } => {
                    assert_eq!(*columns, vec!["id".to_string()]);
                    let ui = using_index.as_ref().unwrap();
                    assert!(
                        ui.to_uppercase().contains("PCTFREE 10"),
                        "using_index: {}",
                        ui
                    );
                    assert!(
                        ui.to_uppercase().contains("INITRANS 2"),
                        "using_index: {}",
                        ui
                    );
                }
                _ => panic!("expected PrimaryKey"),
            }
            assert_eq!(compress, Some(false));
        }
        _ => panic!("expected CreateTable, got {:?}", stmt),
    }
}

// ══ Guard tests: each case was a reported parse error, now fixed ══

#[test]
fn guard_alter_table_drop_bare_ident() {
    let sql = "ALTER TABLE t DROP col";
    match parse_one(sql) {
        Statement::AlterTable(a) => {
            let action = a.actions.first().expect("should have action");
            assert!(matches!(action, AlterTableAction::DropColumn { name, .. } if name == "col"),
                "expected DropColumn(col), got {:?}", action);
        }
        other => panic!("expected AlterTable, got {:?}", other),
    }
}

#[test]
fn guard_alter_table_drop_index() {
    let sql = "ALTER TABLE t DROP INDEX idx";
    match parse_one(sql) {
        Statement::AlterTable(a) => {
            let action = a.actions.first().expect("should have action");
            assert!(matches!(action, AlterTableAction::DropIndex { name, if_exists: false } if name == "idx"),
                "expected DropIndex(idx, false), got {:?}", action);
        }
        other => panic!("expected AlterTable, got {:?}", other),
    }
}

#[test]
fn guard_alter_table_drop_index_if_exists() {
    let sql = "ALTER TABLE t DROP INDEX IF EXISTS idx";
    match parse_one(sql) {
        Statement::AlterTable(a) => {
            let action = a.actions.first().expect("should have action");
            assert!(matches!(action, AlterTableAction::DropIndex { name, if_exists: true } if name == "idx"),
                "expected DropIndex(idx, true), got {:?}", action);
        }
        other => panic!("expected AlterTable, got {:?}", other),
    }
}

#[test]
fn guard_alter_table_drop_if_exists_index() {
    let sql = "ALTER TABLE t DROP IF EXISTS INDEX idx";
    match parse_one(sql) {
        Statement::AlterTable(a) => {
            let action = a.actions.first().expect("should have action");
            assert!(matches!(action, AlterTableAction::DropIndex { name, if_exists: true } if name == "idx"),
                "expected DropIndex(idx, true), got {:?}", action);
        }
        other => panic!("expected AlterTable, got {:?}", other),
    }
}

#[test]
fn guard_alter_table_add_constraint_if_not_exists_pk() {
    let sql = "ALTER TABLE t ADD CONSTRAINT IF NOT EXISTS pk PRIMARY KEY (id)";
    match parse_one(sql) {
        Statement::AlterTable(a) => {
            let action = a.actions.first().expect("should have action");
            assert!(matches!(action, AlterTableAction::AddConstraint { .. }),
                "expected AddConstraint, got {:?}", action);
        }
        other => panic!("expected AlterTable, got {:?}", other),
    }
}

#[test]
fn guard_alter_table_add_constraint_if_not_exists_unique() {
    let sql = "ALTER TABLE t ADD CONSTRAINT IF NOT EXISTS uk UNIQUE (col)";
    match parse_one(sql) {
        Statement::AlterTable(a) => {
            let action = a.actions.first().expect("should have action");
            assert!(matches!(action, AlterTableAction::AddConstraint { .. }),
                "expected AddConstraint, got {:?}", action);
        }
        other => panic!("expected AlterTable, got {:?}", other),
    }
}

#[test]
fn guard_alter_table_modify_not_null() {
    let sql = "ALTER TABLE t MODIFY col VARCHAR(100) NOT NULL";
    match parse_one(sql) {
        Statement::AlterTable(a) => {
            let action = a.actions.first().expect("should have action");
            assert!(matches!(action, AlterTableAction::AlterColumn { .. }),
                "expected AlterColumn, got {:?}", action);
        }
        other => panic!("expected AlterTable, got {:?}", other),
    }
}

#[test]
fn guard_alter_trigger_enable() {
    let sql = "ALTER TRIGGER trig ENABLE";
    match parse_one(sql) {
        Statement::AlterTrigger(t) => {
            assert_eq!(t.name, "trig");
            assert_eq!(t.enable, Some(true));
            assert!(t.table.is_none());
        }
        other => panic!("expected AlterTrigger, got {:?}", other),
    }
}

#[test]
fn guard_alter_trigger_disable() {
    let sql = "ALTER TRIGGER trig DISABLE";
    match parse_one(sql) {
        Statement::AlterTrigger(t) => {
            assert_eq!(t.name, "trig");
            assert_eq!(t.enable, Some(false));
            assert!(t.table.is_none());
        }
        other => panic!("expected AlterTrigger, got {:?}", other),
    }
}

#[test]
fn guard_alter_trigger_rename() {
    let sql = "ALTER TRIGGER trig ON tbl RENAME TO trig2";
    match parse_one(sql) {
        Statement::AlterTrigger(t) => {
            assert_eq!(t.name, "trig");
            assert_eq!(t.table, Some(vec!["tbl".to_string()]));
            assert_eq!(t.new_name, Some("trig2".to_string()));
            assert!(t.enable.is_none());
        }
        other => panic!("expected AlterTrigger, got {:?}", other),
    }
}

#[test]
fn guard_alter_index_noparallel() {
    let sql = "ALTER INDEX idx NOPARALLEL";
    match parse_one(sql) {
        Statement::AlterIndex(a) => {
            assert_eq!(a.name, vec!["idx".to_string()]);
            assert!(matches!(a.action, AlterIndexAction::NoOp));
        }
        other => panic!("expected AlterIndex, got {:?}", other),
    }
}

#[test]
fn guard_alter_index_parallel() {
    let sql = "ALTER INDEX idx PARALLEL";
    match parse_one(sql) {
        Statement::AlterIndex(a) => {
            assert_eq!(a.name, vec!["idx".to_string()]);
            assert!(matches!(a.action, AlterIndexAction::NoOp));
        }
        other => panic!("expected AlterIndex, got {:?}", other),
    }
}

#[test]
fn guard_alter_index_logging() {
    let sql = "ALTER INDEX idx LOGGING";
    match parse_one(sql) {
        Statement::AlterIndex(a) => {
            assert!(matches!(a.action, AlterIndexAction::NoOp));
        }
        other => panic!("expected AlterIndex, got {:?}", other),
    }
}

#[test]
fn guard_alter_index_nologging() {
    let sql = "ALTER INDEX idx NOLOGGING";
    match parse_one(sql) {
        Statement::AlterIndex(a) => {
            assert!(matches!(a.action, AlterIndexAction::NoOp));
        }
        other => panic!("expected AlterIndex, got {:?}", other),
    }
}

#[test]
fn guard_alter_index_rebuild_partition() {
    let sql = "ALTER INDEX idx REBUILD PARTITION p1";
    match parse_one(sql) {
        Statement::AlterIndex(a) => {
            assert!(matches!(a.action, AlterIndexAction::RebuildPartition { ref partition_name } if partition_name == "p1"),
                "expected RebuildPartition(p1), got {:?}", a.action);
        }
        other => panic!("expected AlterIndex, got {:?}", other),
    }
}

#[test]
fn guard_create_sequence_noorder() {
    let sql = "CREATE SEQUENCE seq NOORDER";
    match parse_one(sql) {
        Statement::CreateSequence(s) => {
            assert_eq!(s.name, vec!["seq".to_string()]);
        }
        other => panic!("expected CreateSequence, got {:?}", other),
    }
}

#[test]
fn guard_create_sequence_order() {
    let sql = "CREATE SEQUENCE seq ORDER";
    match parse_one(sql) {
        Statement::CreateSequence(s) => {
            assert_eq!(s.name, vec!["seq".to_string()]);
        }
        other => panic!("expected CreateSequence, got {:?}", other),
    }
}

#[test]
fn guard_create_type_table_of() {
    let sql = "CREATE TYPE t AS TABLE OF VARCHAR(100)";
    match parse_one(sql) {
        Statement::CreateType(ct) => {
            match &ct.type_kind {
                TypeKind::Table { element_type } => {
                    assert!(element_type.to_lowercase().contains("varchar"), "expected VARCHAR in element_type, got {}", element_type);
                }
                other => panic!("expected Table kind, got {:?}", other),
            }
        }
        other => panic!("expected CreateType, got {:?}", other),
    }
}

#[test]
fn guard_drop_large_sequence() {
    let sql = "DROP LARGE SEQUENCE seq";
    match parse_one(sql) {
        Statement::Drop(d) => {
            assert_eq!(d.object_type, ObjectType::Sequence);
            assert_eq!(d.names[0], vec!["seq".to_string()]);
        }
        other => panic!("expected Drop, got {:?}", other),
    }
}

#[test]
fn guard_create_trigger_update_of_no_parens() {
    let sql = "CREATE TRIGGER trig BEFORE UPDATE OF col1, col2 ON tbl FOR EACH ROW EXECUTE PROCEDURE func()";
    match parse_one(sql) {
        Statement::CreateTrigger(ct) => {
            assert!(ct.events.iter().any(|e| matches!(e, TriggerEvent::UpdateOf(cols) if cols.len() == 2)),
                "expected UpdateOf with 2 columns, got {:?}", ct.events);
        }
        other => panic!("expected CreateTrigger, got {:?}", other),
    }
}

#[test]
fn guard_unreserved_keyword_table_alias() {
    let sql = "SELECT * FROM table_name CLIENT";
    match parse_one(sql) {
        Statement::Select(s) => {
            match &s.from[0] {
                TableRef::Table { alias, name, .. } => {
                    assert_eq!(name, &vec!["table_name".to_string()]);
                    assert_eq!(alias, &Some("client".to_string()));
                }
                other => panic!("expected Table ref, got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn guard_plpgsql_compound_type_character_varying() {
    let sql = "DO $$ DECLARE v character varying; BEGIN NULL; END $$";
    let stmts = parse(sql);
    assert!(!stmts.is_empty(), "should parse without error");
}

#[test]
fn guard_plpgsql_compound_type_double_precision() {
    let sql = "DO $$ DECLARE v double precision; BEGIN NULL; END $$";
    let stmts = parse(sql);
    assert!(!stmts.is_empty(), "should parse without error");
}

#[test]
fn guard_qualified_overlay_function() {
    let sql = "SELECT DBE_RAW.OVERLAY(data, 1, 2, 3) FROM t";
    match parse_one(sql) {
        Statement::Select(s) => {
            assert!(!s.targets.is_empty(), "should have targets");
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn guard_create_table_pk_include() {
    let sql = "CREATE TABLE t (id INT, PRIMARY KEY (id) INCLUDE (col1))";
    match parse_one(sql) {
        Statement::CreateTable(ct) => {
            assert!(ct.constraints.iter().any(|c| matches!(c, TableConstraint::PrimaryKey { .. })),
                "expected PrimaryKey constraint");
        }
        other => panic!("expected CreateTable, got {:?}", other),
    }
}

#[test]
fn guard_create_table_unique_using_index() {
    let sql = "CREATE TABLE t (id INT, UNIQUE (id) USING INDEX TABLESPACE ts1)";
    let stmts = parse(sql);
    assert!(!stmts.is_empty(), "should parse without error");
}

// ========== P6: SELECT INTO context disambiguation ==========
//
// In PL/pgSQL context, `SELECT col INTO var FROM table` is variable assignment.
// In top-level SQL, `SELECT * INTO table FROM table2` is CREATE TABLE AS.
// The parser must distinguish based on context (PL block vs top-level).

fn parse_do_block_with_source(sql: &str) -> PlBlock {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::with_source(tokens, sql.to_string());
    let stmts = parser.parse();
    let stmt = stmts.into_iter().next().expect("expected at least one statement");
    match stmt {
        Statement::Do(d) => d
            .block
            .expect("DO statement should have parsed a PL/pgSQL block"),
        _ => panic!("expected DO statement"),
    }
}

fn extract_sql_statement_from_block(block: &PlBlock) -> Option<&PlStatement> {
    block.body.iter().find(|s| matches!(s, PlStatement::SqlStatement { .. }))
}

fn extract_select_from_pl(pl: &PlStatement) -> Option<&SelectStatement> {
    match pl {
        PlStatement::SqlStatement { statement, .. } => match statement.as_ref() {
            Statement::Select(s) => Some(s),
            _ => None,
        },
        _ => None,
    }
}

// --- P6-1: PL block — SELECT single col INTO single variable FROM table ---

#[test]
fn test_pl_select_single_into_variable() {
    let block = parse_do_block(
        "DO $$ DECLARE v_status VARCHAR2(30); BEGIN SELECT status INTO v_status FROM users WHERE id = 1; END $$",
    );
    let sql_stmt = extract_sql_statement_from_block(&block).expect("should have a SQL statement");
    let select = extract_select_from_pl(sql_stmt).expect("should have a SELECT");
    assert!(
        select.into_table.is_none(),
        "PL SELECT INTO should NOT parse as SelectIntoTable, got: {:?}",
        select.into_table
    );
    assert!(
        select.into_targets.is_some(),
        "PL SELECT INTO should parse into_targets as variable list"
    );
    let targets = select.into_targets.as_ref().unwrap();
    assert_eq!(targets.len(), 1, "should have exactly 1 INTO target variable");
}

// --- P6-2: PL block — SELECT func(col) INTO variable FROM table (original reproducer) ---

#[test]
fn test_pl_select_func_into_variable() {
    let block = parse_do_block(
        "DO $$ BEGIN SELECT to_number(p_in_checkBalance) INTO v_in_checkBalance FROM sys_dummy; END $$",
    );
    let sql_stmt = extract_sql_statement_from_block(&block).expect("should have a SQL statement");
    let select = extract_select_from_pl(sql_stmt).expect("should have a SELECT");
    assert!(
        select.into_table.is_none(),
        "PL SELECT to_number(..) INTO var FROM table should NOT parse as SelectIntoTable"
    );
    assert!(
        select.into_targets.is_some(),
        "PL SELECT to_number(..) INTO var FROM table should parse into_targets"
    );
}

// --- P6-3: PL block — SELECT multi-col INTO multi-variable FROM table ---

#[test]
fn test_pl_select_multi_into_variables() {
    let block = parse_do_block(
        "DO $$ BEGIN SELECT name, salary INTO v_name, v_salary FROM emp WHERE emp_id = 42; END $$",
    );
    let sql_stmt = extract_sql_statement_from_block(&block).expect("should have a SQL statement");
    let select = extract_select_from_pl(sql_stmt).expect("should have a SELECT");
    assert!(
        select.into_table.is_none(),
        "PL SELECT .. INTO v1, v2 FROM table should NOT parse as SelectIntoTable"
    );
    let targets = select.into_targets.as_ref().expect("should have into_targets");
    assert_eq!(targets.len(), 2, "should have exactly 2 INTO target variables");
}

// --- P6-4: PL block — SELECT INTO variable with expression target ---

#[test]
fn test_pl_select_expr_into_variable() {
    let block = parse_do_block(
        "DO $$ BEGIN SELECT COUNT(*) INTO v_total FROM orders WHERE status = 'active'; END $$",
    );
    let sql_stmt = extract_sql_statement_from_block(&block).expect("should have a SQL statement");
    let select = extract_select_from_pl(sql_stmt).expect("should have a SELECT");
    assert!(
        select.into_table.is_none(),
        "PL SELECT COUNT(*) INTO var FROM table should NOT parse as SelectIntoTable"
    );
    assert!(select.into_targets.is_some());
}

// --- P6-5: PL block — nested BEGIN with SELECT INTO (scope test) ---

#[test]
fn test_pl_select_into_in_nested_block() {
    let block = parse_do_block(
        "DO $$ BEGIN BEGIN SELECT 1 INTO v_x FROM dual; END; END $$",
    );
    // Navigate into the nested block
    let nested_block = block.body.iter().find_map(|s| match s {
        PlStatement::Block(b) => Some(b),
        _ => None,
    }).expect("should have a nested block");
    let sql_stmt = extract_sql_statement_from_block(nested_block).expect("nested block should have SQL");
    let select = extract_select_from_pl(sql_stmt).expect("should have a SELECT");
    assert!(
        select.into_table.is_none(),
        "PL nested block SELECT INTO should NOT parse as SelectIntoTable"
    );
    assert!(select.into_targets.is_some());
}

// --- P6-6: PL block — SELECT INTO in LOOP ---

#[test]
fn test_pl_select_into_in_loop() {
    let block = parse_do_block(
        "DO $$ BEGIN LOOP SELECT balance INTO v_bal FROM accounts WHERE id = v_id; EXIT WHEN v_bal > 100; END LOOP; END $$",
    );
    let loop_stmt = block.body.iter().find_map(|s| match s {
        PlStatement::Loop(l) => Some(l),
        _ => None,
    }).expect("should have a LOOP");
    let sql_stmt = loop_stmt.body.iter().find(|s| matches!(s, PlStatement::SqlStatement { .. }))
        .expect("loop body should have SQL");
    let select = extract_select_from_pl(sql_stmt).expect("should have a SELECT");
    assert!(
        select.into_table.is_none(),
        "PL SELECT INTO inside LOOP should NOT parse as SelectIntoTable"
    );
}

// --- P6-7: PL block — SELECT INTO in EXCEPTION handler ---

#[test]
fn test_pl_select_into_in_exception_handler() {
    let block = parse_do_block(
        "DO $$ BEGIN SELECT val INTO v FROM t; EXCEPTION WHEN OTHERS THEN SELECT 0 INTO v; END $$",
    );
    let handler = block.exception_block.as_ref().expect("should have exception block");
    let sql_stmt = handler.handlers[0].statements.iter()
        .find(|s| matches!(s, PlStatement::SqlStatement { .. }))
        .expect("handler should have SQL");
    let select = extract_select_from_pl(sql_stmt).expect("should have a SELECT");
    assert!(
        select.into_table.is_none(),
        "PL SELECT INTO in exception handler should NOT parse as SelectIntoTable"
    );
}

// --- P6-8: Package body procedure — SELECT INTO variable (original error-sp.sql scenario) ---

#[test]
fn test_package_body_select_into_variable() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS\n\
               PROCEDURE check_balance(p_id IN NUMBER) IS\n\
                 v_balance NUMBER;\n\
               BEGIN\n\
                 SELECT balance INTO v_balance FROM accounts WHERE id = p_id;\n\
                 IF v_balance < 0 THEN\n\
                   RAISE EXCEPTION 'negative balance';\n\
                 END IF;\n\
               END check_balance;\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    match &stmt {
        Statement::CreatePackageBody(p) => {
            let proc = p.items.iter().find_map(|i| match i {
                PackageItem::Procedure(pr) => Some(pr),
                _ => None,
            }).expect("should have a procedure");
            let block = proc.block.as_ref().expect("procedure should have a block");
            let sql_stmt = extract_sql_statement_from_block(block).expect("should have SQL");
            let select = extract_select_from_pl(sql_stmt).expect("should have a SELECT");
            assert!(
                select.into_table.is_none(),
                "Package body SELECT INTO should NOT parse as SelectIntoTable"
            );
            assert!(
                select.into_targets.is_some(),
                "Package body SELECT INTO should parse into_targets"
            );
        }
        _ => panic!("expected CreatePackageBody, got {:?}", stmt),
    }
}

// --- P6-9: Top-level SQL — SELECT INTO TABLE must still work ---

#[test]
fn test_toplevel_select_into_table_preserved() {
    let sql = "SELECT * INTO TABLE new_table FROM source_table";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert!(s.into_targets.is_none(), "top-level INTO TABLE should NOT have into_targets");
            let into_table = s.into_table.as_ref().expect("expected into_table");
            assert_eq!(into_table.table_name, vec!["new_table"]);
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

// --- P6-10: Top-level SQL — SELECT INTO without TABLE keyword must still work ---

#[test]
fn test_toplevel_select_into_bare_table_preserved() {
    let sql = "SELECT * INTO new_table FROM source_table";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert!(s.into_targets.is_none());
            let into_table = s.into_table.as_ref().expect("expected into_table");
            assert_eq!(into_table.table_name, vec!["new_table"]);
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

// --- P6-11: Top-level SQL — SELECT INTO UNLOGGED TABLE must still work ---

#[test]
fn test_toplevel_select_into_unlogged_preserved() {
    let sql = "SELECT * INTO UNLOGGED TABLE new_table FROM source_table WHERE id > 0";
    let stmt = parse_one(sql);
    match stmt {
        Statement::Select(s) => {
            assert!(s.into_targets.is_none());
            let into_table = s.into_table.as_ref().expect("expected into_table");
            assert!(into_table.unlogged);
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

// --- P6-12: Complex real-world procedure — multiple SELECT INTO patterns ---

#[test]
fn test_complex_procedure_multiple_select_into() {
    let sql = r#"CREATE OR REPLACE PACKAGE BODY bank_pkg IS
        PROCEDURE transfer(p_from IN NUMBER, p_to IN NUMBER, p_amount IN NUMBER) IS
            v_from_balance NUMBER;
            v_to_balance NUMBER;
            v_count INTEGER;
            v_status VARCHAR2(30);
        BEGIN
            SELECT balance INTO v_from_balance FROM accounts WHERE id = p_from;
            SELECT balance INTO v_to_balance FROM accounts WHERE id = p_to;
            SELECT COUNT(*) INTO v_count FROM transactions WHERE account_id = p_from;
            SELECT status INTO v_status FROM account_status WHERE account_id = p_from;
            IF v_from_balance < p_amount THEN
                RAISE EXCEPTION 'insufficient funds';
            END IF;
            UPDATE accounts SET balance = balance - p_amount WHERE id = p_from;
            UPDATE accounts SET balance = balance + p_amount WHERE id = p_to;
        END transfer;
    END bank_pkg;"#;
    let stmt = parse_one(sql);
    match &stmt {
        Statement::CreatePackageBody(p) => {
            let proc = p.items.iter().find_map(|i| match i {
                PackageItem::Procedure(pr) => Some(pr),
                _ => None,
            }).expect("should have transfer procedure");
            let block = proc.block.as_ref().expect("should have block");
            let sql_stmts: Vec<_> = block.body.iter()
                .filter_map(|s| match s {
                    PlStatement::SqlStatement { statement, .. } => match statement.as_ref() {
                        Statement::Select(sel) => Some(sel.clone()),
                        _ => None,
                    },
                    _ => None,
                })
                .collect();
            assert!(sql_stmts.len() >= 4, "should have at least 4 SELECT statements, got {}", sql_stmts.len());
            for (i, sel) in sql_stmts.iter().enumerate() {
                assert!(
                    sel.into_table.is_none(),
                    "SELECT #{} in procedure should NOT have into_table",
                    i + 1
                );
                assert!(
                    sel.into_targets.is_some(),
                    "SELECT #{} in procedure should have into_targets",
                    i + 1
                );
            }
        }
        _ => panic!("expected CreatePackageBody, got {:?}", stmt),
    }
}

// --- PlVariable resolution in INTO targets ---

#[test]
fn test_pl_variable_into_single() {
    let block = parse_do_block(
        "DO $$ DECLARE v_name VARCHAR(100); BEGIN SELECT name INTO v_name FROM users WHERE id = 1; END $$",
    );
    let sql_stmt = extract_sql_statement_from_block(&block).expect("should have a SQL statement");
    let select = extract_select_from_pl(sql_stmt).expect("should have a SELECT");
    let into_targets = select.into_targets.as_ref().expect("should have into_targets");
    assert_eq!(into_targets.len(), 1);
    match &into_targets[0] {
        SelectTarget::Expr(Expr::PlVariable(name), None) => {
            assert_eq!(name, &["v_name"]);
        }
        other => panic!("expected PlVariable for v_name, got {:?}", other),
    }
    let select_targets = &select.targets;
    match &select_targets[0] {
        SelectTarget::Expr(Expr::ColumnRef(_), None) => {}
        other => panic!("SELECT list 'name' should be ColumnRef, got {:?}", other),
    }
}

#[test]
fn test_pl_variable_into_multiple() {
    let block = parse_do_block(
        "DO $$ DECLARE v1 INTEGER; v2 TEXT; BEGIN SELECT c1, c2 INTO v1, v2 FROM t; END $$",
    );
    let sql_stmt = extract_sql_statement_from_block(&block).expect("should have a SQL statement");
    let select = extract_select_from_pl(sql_stmt).expect("should have a SELECT");
    let into_targets = select.into_targets.as_ref().expect("should have into_targets");
    assert_eq!(into_targets.len(), 2);
    match &into_targets[0] {
        SelectTarget::Expr(Expr::PlVariable(name), None) => {
            assert_eq!(name, &["v1"]);
        }
        other => panic!("expected PlVariable for v1, got {:?}", other),
    }
    match &into_targets[1] {
        SelectTarget::Expr(Expr::PlVariable(name), None) => {
            assert_eq!(name, &["v2"]);
        }
        other => panic!("expected PlVariable for v2, got {:?}", other),
    }
    match &select.targets[0] {
        SelectTarget::Expr(Expr::ColumnRef(_), _) => {}
        other => panic!("SELECT list c1 should be ColumnRef, got {:?}", other),
    }
    match &select.targets[1] {
        SelectTarget::Expr(Expr::ColumnRef(_), _) => {}
        other => panic!("SELECT list c2 should be ColumnRef, got {:?}", other),
    }
}

#[test]
fn test_pl_variable_undeclared_stays_column_ref() {
    let block = parse_do_block(
        "DO $$ BEGIN SELECT name INTO v_undeclared FROM users WHERE id = 1; END $$",
    );
    let sql_stmt = extract_sql_statement_from_block(&block).expect("should have a SQL statement");
    let select = extract_select_from_pl(sql_stmt).expect("should have a SELECT");
    let into_targets = select.into_targets.as_ref().expect("should have into_targets");
    assert_eq!(into_targets.len(), 1);
    match &into_targets[0] {
        SelectTarget::Expr(Expr::ColumnRef(name), None) => {
            assert_eq!(name, &["v_undeclared"]);
        }
        other => panic!("undeclared variable should stay ColumnRef, got {:?}", other),
    }
}

#[test]
fn test_pl_variable_param_into() {
    let sql = "CREATE OR REPLACE PROCEDURE test_proc(p_name VARCHAR) IS BEGIN SELECT name INTO p_name FROM users WHERE id = 1; END;";
    let stmt = parse_one(sql);
    match &stmt {
        Statement::CreateProcedure(proc) => {
            let block = proc.block.as_ref().expect("should have block");
            let sql_stmt = block.body.iter().find_map(|s| match s {
                PlStatement::SqlStatement { statement, .. } => Some(statement.as_ref()),
                _ => None,
            }).expect("should have SQL statement");
            match sql_stmt {
                Statement::Select(select) => {
                    let into_targets = select.into_targets.as_ref().expect("should have into_targets");
                    assert_eq!(into_targets.len(), 1);
                    match &into_targets[0] {
                        SelectTarget::Expr(Expr::PlVariable(name), None) => {
                            assert_eq!(name, &["p_name"]);
                        }
                        other => panic!("expected PlVariable for p_name parameter, got {:?}", other),
                    }
                }
                other => panic!("expected Select, got {:?}", other),
            }
        }
        other => panic!("expected CreateProcedure, got {:?}", other),
    }
}

#[test]
fn test_pl_variable_for_loop_implicit_scope() {
    let block = parse_do_block(
        "DO $$ BEGIN FOR rec IN SELECT name FROM users LOOP SELECT name INTO rec FROM dual; END LOOP; END $$",
    );
    let for_stmt = block.body.iter().find_map(|s| match s {
        PlStatement::For(f) => Some(f),
        _ => None,
    }).expect("should have FOR statement");

    let sql_stmt = for_stmt.body.iter().find_map(|s| match s {
        PlStatement::SqlStatement { statement, .. } => Some(statement.as_ref()),
        _ => None,
    }).expect("should have SQL statement inside FOR body");

    match sql_stmt {
        Statement::Select(select) => {
            let into_targets = select.into_targets.as_ref().expect("should have into_targets");
            assert_eq!(into_targets.len(), 1);
            match &into_targets[0] {
                SelectTarget::Expr(Expr::PlVariable(name), None) => {
                    assert_eq!(name, &["rec"]);
                }
                other => panic!("expected PlVariable for rec (FOR loop implicit scope), got {:?}", other),
            }
        }
        other => panic!("expected Select, got {:?}", other),
    }
}
