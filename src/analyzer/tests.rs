use super::*;
use crate::parser::Parser;

fn parse_block(sql: &str) -> crate::ast::plpgsql::PlBlock {
    let tokens = crate::Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    match &stmts[0] {
        crate::ast::Statement::Do(d) => d.block.as_ref().expect("block should parse").clone(),
        crate::ast::Statement::AnonyBlock(ab) => ab.block.clone(),
        _ => panic!("expected DO or AnonyBlock, got {:?}", stmts[0]),
    }
}

#[test]
fn test_literal_assignment_to_execute() {
    let block = parse_block(
        "DO $$ BEGIN plsql_block := 'call calc_stats($1, $1, $2, $1)'; EXECUTE IMMEDIATE plsql_block USING a, b; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    let finding = &report.execute_findings[0];
    assert_eq!(finding.expression_desc, "plsql_block");
    assert_eq!(
        finding.resolved_value.as_deref(),
        Some("call calc_stats($1, $1, $2, $1)")
    );
    assert!(finding.parsed_statement.is_some());
    match &finding.trace {
        TraceChain::VariableCopy {
            source_var,
            source_chain,
        } => {
            assert_eq!(source_var, "plsql_block");
            assert!(matches!(
                source_chain.as_ref(),
                TraceChain::LiteralAssignment { .. }
            ));
        }
        other => panic!("expected VariableCopy, got {:?}", other),
    }
}

#[test]
fn test_variable_chain() {
    let block = parse_block("DO $$ BEGIN a := 'SELECT 1'; b := a; EXECUTE IMMEDIATE b; END $$");
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    let finding = &report.execute_findings[0];
    assert_eq!(finding.resolved_value.as_deref(), Some("SELECT 1"));
    assert!(finding.parsed_statement.is_some());
    match &finding.trace {
        TraceChain::VariableCopy {
            source_var,
            source_chain,
        } => {
            assert_eq!(source_var, "b");
            match source_chain.as_ref() {
                TraceChain::VariableCopy { source_var, .. } => {
                    assert_eq!(source_var, "a")
                }
                other => panic!("expected nested VariableCopy, got {:?}", other),
            }
        }
        other => panic!("expected VariableCopy, got {:?}", other),
    }
}

#[test]
fn test_concatenation() {
    let block = parse_block(
        "DO $$ BEGIN pfx := 'SELECT * FROM '; sfx := 'users'; full_sql := pfx || sfx; EXECUTE IMMEDIATE full_sql; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    let finding = &report.execute_findings[0];
    assert_eq!(
        finding.resolved_value.as_deref(),
        Some("SELECT * FROM users")
    );
    assert!(finding.parsed_statement.is_some());
}

#[test]
fn test_concat_with_literal_and_variable() {
    let block = parse_block(
        "DO $$ BEGIN tab := 'users'; EXECUTE IMMEDIATE 'SELECT * FROM ' || tab; END $$",
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    let finding = &report.execute_findings[0];
    assert_eq!(
        finding.resolved_value.as_deref(),
        Some("SELECT * FROM users")
    );
    assert!(finding.parsed_statement.is_some());
}

#[test]
fn test_unknown_variable() {
    let block = parse_block("DO $$ BEGIN EXECUTE unknown_var; END $$");
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    assert!(report.execute_findings[0].resolved_value.is_none());
    assert!(matches!(
        report.execute_findings[0].trace,
        TraceChain::Unknown
    ));
}

#[test]
fn test_assignment_in_if_branch() {
    let block = parse_block(
        "DO $$ BEGIN IF true THEN sql_text := 'DROP TABLE temp'; END IF; EXECUTE IMMEDIATE sql_text; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    let finding = &report.execute_findings[0];
    assert_eq!(finding.resolved_value.as_deref(), Some("DROP TABLE temp"));
}

#[test]
fn test_multiple_executes() {
    let block = parse_block(
        "DO $$ BEGIN a := 'SELECT 1'; b := 'SELECT 2'; EXECUTE IMMEDIATE a; EXECUTE IMMEDIATE b; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 2);
    assert_eq!(
        report.execute_findings[0].resolved_value.as_deref(),
        Some("SELECT 1")
    );
    assert_eq!(
        report.execute_findings[1].resolved_value.as_deref(),
        Some("SELECT 2")
    );
}

#[test]
fn test_variable_traces_recorded() {
    let block = parse_block(
        "DO $$ BEGIN x := 'hello'; y := x; z := y || ' world'; EXECUTE IMMEDIATE z; END $$",
    );
    let report = analyze_pl_block(&block);
    assert!(report.variable_traces.len() >= 3);
    assert!(report
        .variable_traces
        .iter()
        .any(|t| t.variable_name == "x" && t.value == "hello"));
    assert!(report
        .variable_traces
        .iter()
        .any(|t| t.variable_name == "y" && t.value == "hello"));
    assert!(report
        .variable_traces
        .iter()
        .any(|t| t.variable_name == "z" && t.value == "hello world"));
}

#[test]
fn test_nested_block_inner_scope() {
    let block =
        parse_block("DO $$ BEGIN BEGIN inner := 'SELECT 1'; END; EXECUTE IMMEDIATE inner; END $$");
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    // inner scope variables don't leak to outer scope
    assert!(report.execute_findings[0].resolved_value.is_none());
}

#[test]
fn test_literal_execute_still_works() {
    let block = parse_block("DO $$ BEGIN EXECUTE 'SELECT 42'; END $$");
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    let finding = &report.execute_findings[0];
    // Direct literal — resolved by analyzer (not parser's parsed_query)
    assert_eq!(finding.resolved_value.as_deref(), Some("SELECT 42"));
    assert!(finding.parsed_statement.is_some());
}

#[test]
fn test_statement_path_tracking() {
    let block = parse_block("DO $$ BEGIN a := 'SELECT 1'; EXECUTE IMMEDIATE a; END $$");
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings[0].statement_path, vec![1]);
}

#[test]
fn test_for_loop_variable_in_scope() {
    let block = parse_block(
        "DO $$ BEGIN FOR rec IN 1..10 LOOP EXECUTE IMMEDIATE rec; END LOOP; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    // rec is declared in FOR scope but has no known string value
    assert!(report.execute_findings[0].resolved_value.is_none());
    // Should NOT be Unknown — it's a declared variable with no known value
    match &report.execute_findings[0].trace {
        TraceChain::VariableCopy { source_var, .. } => {
            assert_eq!(source_var, "rec");
        }
        other => panic!("expected VariableCopy for rec, got {:?}", other),
    }
}

#[test]
fn test_variable_no_default_in_scope() {
    let block = parse_block(
        "DO $$ DECLARE v_sql VARCHAR(100); BEGIN v_sql := 'SELECT 1'; EXECUTE IMMEDIATE v_sql; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    assert_eq!(
        report.execute_findings[0].resolved_value.as_deref(),
        Some("SELECT 1")
    );
    match &report.execute_findings[0].trace {
        TraceChain::VariableCopy { source_var, .. } => {
            assert_eq!(source_var, "v_sql");
        }
        other => panic!("expected VariableCopy for v_sql, got {:?}", other),
    }
}

#[test]
fn test_cursor_declaration_in_scope() {
    let block = parse_block(
        "DO $$ DECLARE cur CURSOR FOR SELECT 1; BEGIN NULL; END $$"
    );
    // Cursor is registered in scope — no crash, no execute findings
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 0);
}

#[test]
fn test_foreach_loop_variable_in_scope() {
    let block = parse_block(
        "DO $$ DECLARE arr INT[]; BEGIN FOREACH item IN ARRAY arr LOOP EXECUTE IMMEDIATE item; END LOOP; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    assert!(report.execute_findings[0].resolved_value.is_none());
    match &report.execute_findings[0].trace {
        TraceChain::VariableCopy { source_var, .. } => {
            assert_eq!(source_var, "item");
        }
        other => panic!("expected VariableCopy for item, got {:?}", other),
    }
}

#[test]
fn test_for_loop_variable_does_not_leak() {
    let block = parse_block(
        "DO $$ BEGIN FOR rec IN 1..10 LOOP NULL; END LOOP; EXECUTE IMMEDIATE rec; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    assert!(report.execute_findings[0].resolved_value.is_none());
    assert!(matches!(
        report.execute_findings[0].trace,
        TraceChain::Unknown
    ));
}

fn parse_procedure_block(sql: &str) -> (crate::ast::plpgsql::PlBlock, Vec<(String, String, Option<String>)>) {
    let tokens = crate::Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse();
    if let Some(crate::ast::Statement::CreatePackageBody(pkg)) = stmts.into_iter().next() {
        for item in &pkg.node.items {
            if let crate::ast::PackageItem::Procedure(proc) = item {
                let params: Vec<_> = proc.parameters.iter().map(|p| {
                    (p.name.clone(), p.data_type.clone(), p.mode.clone())
                }).collect();
                let block = proc.block.as_ref().expect("procedure should have block").clone();
                return (block, params);
            }
        }
    }
    panic!("expected package body with procedure");
}

#[test]
fn test_ref_cursor_out_param_detected() {
    let (block, params) = parse_procedure_block(
        r#"CREATE OR REPLACE PACKAGE BODY PKG_TEST AS
  PROCEDURE prc_query(p_acnt VARCHAR, out_list OUT REFCURSOR) AS
  BEGIN
    OPEN out_list FOR SELECT * FROM t_users WHERE accno = p_acnt;
  END prc_query;
END PKG_TEST;
/"#
    );
    let queries = find_ref_cursor_queries(&block, &params);
    assert_eq!(queries.len(), 1);
    assert_eq!(queries[0].out_param_name, "out_list");
    assert!(queries[0].query.is_some());
    assert!(queries[0].parsed_query.is_some());
}

#[test]
fn test_ref_cursor_not_matched_for_non_out() {
    let (block, params) = parse_procedure_block(
        r#"CREATE OR REPLACE PACKAGE BODY PKG_TEST AS
  PROCEDURE prc_query(cur IN REFCURSOR) AS
  BEGIN
    OPEN cur FOR SELECT 1;
  END prc_query;
END PKG_TEST;
/"#
    );
    let queries = find_ref_cursor_queries(&block, &params);
    assert!(queries.is_empty());
}

#[test]
fn test_ref_cursor_for_execute() {
    let (block, params) = parse_procedure_block(
        r#"CREATE OR REPLACE PACKAGE BODY PKG_TEST AS
  PROCEDURE prc_query(p_acnt VARCHAR, out_list OUT REFCURSOR) AS
    v_sql VARCHAR;
  BEGIN
    v_sql := 'SELECT * FROM t_users WHERE accno = :1';
    OPEN out_list FOR EXECUTE v_sql;
  END prc_query;
END PKG_TEST;
/"#
    );
    let queries = find_ref_cursor_queries(&block, &params);
    assert_eq!(queries.len(), 1);
    assert_eq!(queries[0].out_param_name, "out_list");
    assert_eq!(queries[0].query.as_deref(), Some("v_sql"));
}

#[test]
fn test_no_ref_cursor_params() {
    let (block, params) = parse_procedure_block(
        r#"CREATE OR REPLACE PACKAGE BODY PKG_TEST AS
  PROCEDURE prc_query(p_acnt VARCHAR) AS
  BEGIN
    NULL;
  END prc_query;
END PKG_TEST;
/"#
    );
    let queries = find_ref_cursor_queries(&block, &params);
    assert!(queries.is_empty());
}

fn parse_statements(sql: &str) -> Vec<crate::ast::Statement> {
    let tokens = crate::Tokenizer::new(sql).tokenize().unwrap();
    crate::parser::Parser::new(tokens).parse()
}

#[test]
fn test_fingerprint_identical_queries() {
    let stmts = parse_statements(
        "SELECT * FROM t_users WHERE accno = 'admin'; SELECT * FROM t_users WHERE accno = 'admin'"
    );
    let fps = compute_query_fingerprints(&stmts);
    assert_eq!(fps.len(), 1, "identical queries should have same fingerprint");
    assert_eq!(fps[0].occurrences.len(), 2);
}

#[test]
fn test_fingerprint_different_queries() {
    let stmts = parse_statements(
        "SELECT * FROM t_users; SELECT * FROM t_orders"
    );
    let fps = compute_query_fingerprints(&stmts);
    assert_eq!(fps.len(), 2, "different queries should have different fingerprints");
}

#[test]
fn test_fingerprint_deterministic() {
    let stmts1 = parse_statements("SELECT id, name FROM users WHERE active = true");
    let stmts2 = parse_statements("SELECT id, name FROM users WHERE active = true");
    let fps1 = compute_query_fingerprints(&stmts1);
    let fps2 = compute_query_fingerprints(&stmts2);
    assert_eq!(fps1[0].fingerprint, fps2[0].fingerprint, "same SQL should produce same fingerprint");
}

#[test]
fn test_fingerprint_format() {
    let stmts = parse_statements("SELECT 1");
    let fps = compute_query_fingerprints(&stmts);
    assert!(fps[0].fingerprint.starts_with("fp_"), "fingerprint should start with fp_ prefix");
    assert_eq!(fps[0].fingerprint.len(), 19, "fingerprint should be fp_ + 16 hex chars");
}
