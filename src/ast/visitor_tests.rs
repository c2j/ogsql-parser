#[cfg(test)]
mod visitor_tests {
    use crate::ast::plpgsql::{
        PlBlock, PlDeclaration, PlExceptionBlock, PlExceptionHandler, PlForKind, PlForStmt,
        PlIfStmt, PlLoopStmt, PlOpenKind, PlOpenStmt, PlProcedureCall, PlStatement, PlWhileStmt,
    };
    use crate::ast::visitor::{
        walk_expr, walk_pl_block, walk_pl_declaration, walk_pl_statement, walk_select,
        walk_statement, walk_table_ref, Visitor, VisitorResult,
    };
    use crate::ast::{
        CallFuncStatement, CreateFunctionStatement, CreatePackageBodyStatement,
        CreatePackageStatement, CreateProcedureStatement, DoStatement, Expr, ObjectName,
        SelectStatement, Statement, TableRef,
    };
    use crate::parser::Parser;
    use crate::tokenizer::Tokenizer;

    // ── Test Helpers ──

    fn parse(sql: &str) -> Vec<Statement> {
        let tokens = Tokenizer::new(sql).tokenize().unwrap();
        Parser::new(tokens).parse()
    }

    fn parse_single(sql: &str) -> Statement {
        let mut stmts = parse(sql);
        assert_eq!(stmts.len(), 1, "Expected exactly one statement");
        stmts.remove(0)
    }

    fn parse_expr(sql: &str) -> Expr {
        // Parse as SELECT expr and extract it
        let stmt = parse_single(&format!("SELECT {}", sql));
        match stmt {
            Statement::Select(ref select) => {
                match &select.targets[0] {
                    crate::ast::SelectTarget::Expr(expr, _) => expr.clone(),
                    _ => panic!("Expected expression target"),
                }
            }
            _ => panic!("Expected SELECT statement"),
        }
    }

    // ── Test Visitor: Collects visited node types ──

    #[derive(Debug, Default)]
    struct TestVisitor {
        statements: Vec<String>,
        exprs: Vec<String>,
        pl_blocks: usize,
        pl_statements: usize,
        pl_declarations: usize,
        pl_exception_handlers: usize,
        calls: Vec<ObjectName>,
        procedure_calls: Vec<ObjectName>,
        selects: usize,
    }

    impl Visitor for TestVisitor {
        fn visit_statement(&mut self, stmt: &Statement) -> VisitorResult {
            self.statements.push(format!("{:?}", std::mem::discriminant(stmt)));
            VisitorResult::Continue
        }

        fn visit_expr(&mut self, _expr: &Expr) -> VisitorResult {
            self.exprs.push("expr".to_string());
            VisitorResult::Continue
        }

        fn visit_pl_block(&mut self, _block: &PlBlock) -> VisitorResult {
            self.pl_blocks += 1;
            VisitorResult::Continue
        }

        fn visit_pl_statement(&mut self, _stmt: &PlStatement) -> VisitorResult {
            self.pl_statements += 1;
            VisitorResult::Continue
        }

        fn visit_pl_declaration(&mut self, _decl: &PlDeclaration) -> VisitorResult {
            self.pl_declarations += 1;
            VisitorResult::Continue
        }

        fn visit_pl_exception_handler(&mut self, _handler: &PlExceptionHandler) -> VisitorResult {
            self.pl_exception_handlers += 1;
            VisitorResult::Continue
        }

        fn visit_call(&mut self, call: &CallFuncStatement) -> VisitorResult {
            self.calls.push(call.func_name.clone());
            VisitorResult::Continue
        }

        fn visit_procedure_call(&mut self, call: &PlProcedureCall) -> VisitorResult {
            self.procedure_calls.push(call.name.clone());
            VisitorResult::Continue
        }

        fn visit_select(&mut self, _select: &SelectStatement) -> VisitorResult {
            self.selects += 1;
            VisitorResult::Continue
        }
    }

    // ── P0 Tests: PL/pgSQL Visitor ──

    #[test]
    fn test_pl_block_visitor_methods_exist() {
        // Verify all new trait methods compile with default implementations
        let mut visitor = TestVisitor::default();
        let block = PlBlock {
            label: None,
            declarations: vec![],
            body: vec![],
            exception_block: None,
            end_label: None,
        };
        assert_eq!(visitor.visit_pl_block(&block), VisitorResult::Continue);
        assert_eq!(visitor.visit_pl_statement(&PlStatement::Null), VisitorResult::Continue);
        assert_eq!(visitor.visit_pl_declaration(&PlDeclaration::Record(crate::ast::plpgsql::PlRecordDecl { name: "x".to_string() })), VisitorResult::Continue);
        assert_eq!(visitor.visit_pl_exception_handler(&PlExceptionHandler { conditions: vec![], statements: vec![] }
        ), VisitorResult::Continue);
    }

    #[test]
    fn test_walk_pl_block_basic() {
        let block = PlBlock {
            label: None,
            declarations: vec![
                PlDeclaration::Variable(crate::ast::plpgsql::PlVarDecl {
                    name: "x".to_string(),
                    data_type: crate::ast::plpgsql::PlDataType::TypeName("INTEGER".to_string()),
                    default: Some(Expr::Literal(crate::ast::Literal::Integer(42))),
                    constant: false,
                    not_null: false,
                    collate: None,
                }),
            ],
            body: vec![
                PlStatement::Assignment {
                    target: "x".to_string(),
                    expression: Expr::Literal(crate::ast::Literal::Integer(1)),
                },
            ],
            exception_block: Some(PlExceptionBlock {
                handlers: vec![
                    PlExceptionHandler {
                        conditions: vec!["OTHERS".to_string()],
                        statements: vec![PlStatement::Null],
                    },
                ],
            }),
            end_label: None,
        };

        let mut visitor = TestVisitor::default();
        walk_pl_block(&mut visitor, &block);

        assert_eq!(visitor.pl_blocks, 1);
        assert_eq!(visitor.pl_declarations, 1);
        assert_eq!(visitor.pl_statements, 3); // Assignment + Null (in handler) + handler itself
        assert_eq!(visitor.pl_exception_handlers, 1);
        assert_eq!(visitor.exprs.len(), 2); // default(42) + assignment(1)
    }

    #[test]
    fn test_walk_pl_statement_if() {
        let if_stmt = PlStatement::If(PlIfStmt {
            condition: Expr::Literal(crate::ast::Literal::Boolean(true)),
            then_stmts: vec![PlStatement::Null],
            elsifs: vec![crate::ast::plpgsql::PlElsif {
                condition: Expr::Literal(crate::ast::Literal::Boolean(false)),
                stmts: vec![PlStatement::Null],
            }],
            else_stmts: vec![PlStatement::Null],
        });

        let mut visitor = TestVisitor::default();
        walk_pl_statement(&mut visitor, &if_stmt);

        assert_eq!(visitor.pl_statements, 4); // If + 3 Nulls
        assert_eq!(visitor.exprs.len(), 2); // condition + elsif condition
    }

    #[test]
    fn test_walk_pl_statement_loop() {
        let loop_stmt = PlStatement::Loop(PlLoopStmt {
            label: None,
            body: vec![
                PlStatement::Null,
                PlStatement::Null,
            ],
            end_label: None,
        });

        let mut visitor = TestVisitor::default();
        walk_pl_statement(&mut visitor, &loop_stmt);

        assert_eq!(visitor.pl_statements, 3); // Loop + 2 Nulls
    }

    #[test]
    fn test_walk_pl_statement_while() {
        let while_stmt = PlStatement::While(PlWhileStmt {
            label: None,
            condition: Expr::Literal(crate::ast::Literal::Boolean(true)),
            body: vec![PlStatement::Null],
            end_label: None,
        });

        let mut visitor = TestVisitor::default();
        walk_pl_statement(&mut visitor, &while_stmt);

        assert_eq!(visitor.pl_statements, 2); // While + Null
        assert_eq!(visitor.exprs.len(), 1); // condition
    }

    #[test]
    fn test_walk_pl_statement_for_range() {
        let for_stmt = PlStatement::For(PlForStmt {
            label: None,
            variable: "i".to_string(),
            kind: PlForKind::Range {
                low: Expr::Literal(crate::ast::Literal::Integer(1)),
                high: Expr::Literal(crate::ast::Literal::Integer(10)),
                step: None,
                reverse: false,
            },
            body: vec![PlStatement::Null],
            end_label: None,
        });

        let mut visitor = TestVisitor::default();
        walk_pl_statement(&mut visitor, &for_stmt);

        assert_eq!(visitor.pl_statements, 2); // For + Null
        assert_eq!(visitor.exprs.len(), 2); // low + high
    }

    #[test]
    fn test_walk_pl_statement_procedure_call() {
        let proc_call = PlStatement::ProcedureCall(PlProcedureCall {
            name: vec!["schema".to_string(), "proc".to_string()],
            arguments: vec![
                Expr::Literal(crate::ast::Literal::Integer(1)),
                Expr::Literal(crate::ast::Literal::Integer(2)),
            ],
        });

        let mut visitor = TestVisitor::default();
        walk_pl_statement(&mut visitor, &proc_call);

        assert_eq!(visitor.pl_statements, 1);
        assert_eq!(visitor.procedure_calls.len(), 1);
        assert_eq!(visitor.procedure_calls[0], vec!["schema".to_string(), "proc".to_string()]);
        assert_eq!(visitor.exprs.len(), 2); // 2 arguments
    }

    #[test]
    fn test_walk_pl_statement_exception() {
        let block = PlBlock {
            label: None,
            declarations: vec![],
            body: vec![PlStatement::Null],
            exception_block: Some(PlExceptionBlock {
                handlers: vec![
                    PlExceptionHandler {
                        conditions: vec!["NO_DATA_FOUND".to_string()],
                        statements: vec![PlStatement::Null],
                    },
                    PlExceptionHandler {
                        conditions: vec!["OTHERS".to_string()],
                        statements: vec![PlStatement::Null],
                    },
                ],
            }),
            end_label: None,
        };

        let mut visitor = TestVisitor::default();
        walk_pl_block(&mut visitor, &block);

        assert_eq!(visitor.pl_blocks, 1);
        assert_eq!(visitor.pl_exception_handlers, 2);
        assert_eq!(visitor.pl_statements, 3); // body Null + 2 handler Nulls
    }

    // ── P0 Tests: walk_statement PL/pgSQL carriers ──

    #[test]
    fn test_walk_statement_create_function() {
        let sql = "CREATE FUNCTION foo() RETURNS INTEGER AS $$ BEGIN RETURN 1; END; $$ LANGUAGE plpgsql";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.pl_blocks, 1);
        assert_eq!(visitor.pl_statements, 1); // Return
    }

    #[test]
    fn test_walk_statement_create_procedure() {
        let sql = "CREATE PROCEDURE bar() AS $$ BEGIN NULL; END; $$";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.pl_blocks, 1);
        assert_eq!(visitor.pl_statements, 1); // Null
    }

    #[test]
    fn test_walk_statement_do() {
        let sql = "DO $$ BEGIN PERFORM 1; END $$";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.pl_blocks, 1);
        assert_eq!(visitor.pl_statements, 1); // Perform
    }

    #[test]
    fn test_walk_statement_anony_block() {
        let sql = "BEGIN PERFORM 1; END";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.pl_blocks, 1);
        assert_eq!(visitor.pl_statements, 1); // Perform
    }

    #[test]
    fn test_walk_statement_call() {
        let sql = "CALL schema.proc_name(1, 2)";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.calls.len(), 1);
        assert_eq!(visitor.calls[0], vec!["schema".to_string(), "proc_name".to_string()]);
    }

    // ── P1 Tests: walk_expr coverage ──

    #[test]
    fn test_walk_expr_like() {
        let expr = parse_expr("name LIKE '%test%'");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 2); // name + pattern
    }

    #[test]
    fn test_walk_expr_between() {
        let expr = parse_expr("x BETWEEN 1 AND 10");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 3); // x + low + high
    }

    #[test]
    fn test_walk_expr_in_list() {
        let expr = parse_expr("x IN (1, 2, 3)");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 4); // x + 3 items
    }

    #[test]
    fn test_walk_expr_exists() {
        let expr = parse_expr("EXISTS (SELECT 1 FROM t)");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert_eq!(visitor.selects, 1); // subquery
    }

    #[test]
    fn test_walk_expr_typecast() {
        let expr = parse_expr("x::INTEGER");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 1); // x
    }

    #[test]
    fn test_walk_expr_array() {
        let expr = parse_expr("ARRAY[1, 2, 3]");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 3); // 3 elements
    }

    #[test]
    fn test_walk_expr_subscript() {
        let expr = parse_expr("arr[1]");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 2); // arr + index
    }

    #[test]
    fn test_walk_expr_field_access() {
        let expr = parse_expr("rec.field");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 1); // rec
    }

    #[test]
    fn test_walk_expr_parenthesized() {
        let expr = parse_expr("(1 + 2)");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 1); // inner expr
    }

    #[test]
    fn test_walk_expr_row_constructor() {
        let expr = parse_expr("ROW(1, 2, 3)");
        let mut visitor = TestVisitor::default();
        walk_expr(&mut visitor, &expr);
        assert!(visitor.exprs.len() >= 3); // 3 elements
    }

    // ── P1 Tests: walk_select coverage ──

    #[test]
    fn test_walk_select_with_cte() {
        let sql = "WITH cte AS (SELECT id FROM users) SELECT * FROM cte";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.selects, 2); // outer + CTE
    }

    #[test]
    fn test_walk_select_group_by() {
        let sql = "SELECT dept, COUNT(*) FROM employees GROUP BY dept HAVING COUNT(*) > 5";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert!(visitor.exprs.len() >= 3); // dept + COUNT(*) + HAVING
    }

    #[test]
    fn test_walk_select_order_by() {
        let sql = "SELECT * FROM t ORDER BY x, y DESC";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert!(visitor.exprs.len() >= 2); // x + y
    }

    #[test]
    fn test_walk_select_limit_offset() {
        let sql = "SELECT * FROM t LIMIT 10 OFFSET 5";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert!(visitor.exprs.len() >= 2); // LIMIT + OFFSET
    }

    #[test]
    fn test_walk_select_union() {
        let sql = "SELECT 1 UNION ALL SELECT 2";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.selects, 2); // left + right
    }

    // ── P1 Tests: walk_table_ref coverage ──

    #[test]
    fn test_walk_table_ref_join() {
        let sql = "SELECT * FROM a JOIN b ON a.id = b.id";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert!(visitor.exprs.len() >= 2); // a.id + b.id
    }

    #[test]
    fn test_walk_table_ref_function_call() {
        let sql = "SELECT * FROM generate_series(1, 10)";
        let stmt = parse_single(sql);

        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert!(visitor.exprs.len() >= 2); // 1 + 10
    }

    // ── P2 Tests: SkipChildren behavior ──

    #[derive(Debug, Default)]
    struct SkipChildrenVisitor {
        expr_count: usize,
    }

    impl Visitor for SkipChildrenVisitor {
        fn visit_expr(&mut self, _expr: &Expr) -> VisitorResult {
            self.expr_count += 1;
            VisitorResult::SkipChildren
        }
    }

    #[test]
    fn test_skip_children_expr() {
        let expr = parse_expr("1 + 2");
        let mut visitor = SkipChildrenVisitor::default();
        walk_expr(&mut visitor, &expr);

        // Should visit the BinaryOp expr but skip its children (1 and 2)
        assert_eq!(visitor.expr_count, 1);
    }

    #[derive(Debug, Default)]
    struct StopVisitor {
        expr_count: usize,
    }

    impl Visitor for StopVisitor {
        fn visit_expr(&mut self, _expr: &Expr) -> VisitorResult {
            self.expr_count += 1;
            if self.expr_count >= 2 {
                VisitorResult::Stop
            } else {
                VisitorResult::Continue
            }
        }
    }

    #[test]
    fn test_stop_expr() {
        let expr = parse_expr("1 + 2 + 3");
        let mut visitor = StopVisitor::default();
        let result = walk_expr(&mut visitor, &expr);

        assert_eq!(result, VisitorResult::Stop);
        assert_eq!(visitor.expr_count, 2);
    }

    // ── Integration Tests ──

    #[test]
    fn test_complex_pl_block() {
        let sql = r#"
            CREATE FUNCTION complex_func(p_id INT) RETURNS INT AS $$
            DECLARE
                v_count INT := 0;
            BEGIN
                IF p_id > 0 THEN
                    PERFORM process_record(p_id);
                    v_count := v_count + 1;
                ELSE
                    RAISE NOTICE 'Invalid id: %', p_id;
                END IF;
                
                FOR i IN 1..10 LOOP
                    PERFORM helper(i);
                END LOOP;
                
                RETURN v_count;
            EXCEPTION
                WHEN OTHERS THEN
                    log_error(SQLERRM);
                    RETURN -1;
            END;
            $$ LANGUAGE plpgsql
        "#;

        let stmt = parse_single(sql);
        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.pl_blocks, 1);
        assert_eq!(visitor.pl_declarations, 1); // v_count
        assert!(visitor.pl_statements >= 5); // If + Loop + Return + 2 Performs
        assert_eq!(visitor.pl_exception_handlers, 1);
        assert_eq!(visitor.procedure_calls.len(), 3); // process_record + helper + log_error
    }

    #[test]
    fn test_nested_blocks() {
        let sql = r#"
            DO $$
            BEGIN
                BEGIN
                    PERFORM inner_proc();
                END;
                PERFORM outer_proc();
            END;
            $$
        "#;

        let stmt = parse_single(sql);
        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.pl_blocks, 2); // outer + inner
        assert_eq!(visitor.procedure_calls.len(), 2); // inner_proc + outer_proc
    }

    #[test]
    fn test_create_package_body() {
        let sql = r#"
            CREATE PACKAGE BODY pkg_api AS
                PROCEDURE inner_proc IS
                BEGIN
                    helper.do_stuff();
                END;
                
                FUNCTION get_val RETURN NUMBER IS
                BEGIN
                    RETURN compute_val();
                END;
            END pkg_api
        "#;

        let stmt = parse_single(sql);
        let mut visitor = TestVisitor::default();
        walk_statement(&mut visitor, &stmt);

        assert_eq!(visitor.pl_blocks, 2); // inner_proc + get_val
        assert_eq!(visitor.procedure_calls.len(), 2); // helper.do_stuff + compute_val
    }
}
