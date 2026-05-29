use std::collections::HashMap;

use crate::ast::plpgsql::{
    PlBlock, PlCaseStmt, PlForStmt, PlIfStmt, PlLoopStmt, PlOpenKind, PlOpenStmt, PlReturnQueryStmt, PlStatement,
    PlWhileStmt,
};
use crate::ast::{Expr, Literal, RoutineParam, SelectTarget, Spanned, Statement};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReturnCursorAnnotation {
    pub out_param: String,
    pub out_position: usize,
    pub out_type: String,
    pub branch_path: String,
    pub branch_condition: String,
    pub jdbc_type: String,
    pub sql: String,
    pub sql_source: String,
    pub parsed_query: Option<Box<Statement>>,
    pub result_columns: Vec<ResultColumn>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResultColumn {
    pub name: String,
    pub inferred_type: Option<String>,
    pub expression: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RoutineReturnAnalysis {
    pub routine_name: String,
    pub routine_kind: String,
    pub return_cursors: Vec<ReturnCursorGroup>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReturnCursorGroup {
    pub out_param: String,
    pub position: usize,
    pub cursor_type: String,
    pub jdbc_type: String,
    pub branches: Vec<ReturnCursorBranch>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReturnCursorBranch {
    pub path: String,
    pub condition: String,
    pub sql: String,
    pub sql_source: String,
    pub result_columns: Vec<ResultColumn>,
}

struct BranchContext {
    path: String,
    condition: String,
}

struct OutCursorInfo {
    name: String,
    position: usize,
    cursor_type: String,
}

fn is_out_refcursor(param: &RoutineParam) -> bool {
    let mode_ok = param
        .mode
        .as_ref()
        .map(|m| {
            let upper = m.to_uppercase();
            upper == "OUT" || upper == "INOUT" || upper == "IN OUT"
        })
        .unwrap_or(false);
    let type_ok = param.data_type.to_uppercase().contains("REFCURSOR");
    mode_ok && type_ok
}

fn is_return_refcursor(return_type: Option<&str>) -> bool {
    return_type.map(|rt| rt.to_uppercase().contains("REFCURSOR")).unwrap_or(false)
}

pub fn has_return_cursors(params: &[RoutineParam], return_type: Option<&str>) -> bool {
    params.iter().any(is_out_refcursor) || is_return_refcursor(return_type)
}

fn extract_cursor_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::PlVariable(names) | Expr::ColumnRef(names) if names.len() == 1 => Some(names[0].clone()),
        _ => None,
    }
}

fn format_expr_brief(expr: &Expr) -> String {
    match expr {
        Expr::Literal(Literal::String(s)) => format!("'{}'", s),
        Expr::Literal(lit) => format!("{:?}", lit),
        Expr::ColumnRef(names) | Expr::PlVariable(names) => names.join("."),
        Expr::BinaryOp { left, op, right } => {
            format!("{} {} {}", format_expr_brief(left), op, format_expr_brief(right))
        }
        Expr::UnaryOp { op, expr: inner } => format!("{}{}", op, format_expr_brief(inner)),
        Expr::Parenthesized(inner) => format!("({})", format_expr_brief(inner)),
        Expr::FunctionCall { name, args, .. } => {
            let arg_strs: Vec<String> = args.iter().map(format_expr_brief).collect();
            format!("{}({})", name.join("."), arg_strs.join(", "))
        }
        Expr::IsNull { expr: inner, negated } => {
            if *negated {
                format!("{} IS NOT NULL", format_expr_brief(inner))
            } else {
                format!("{} IS NULL", format_expr_brief(inner))
            }
        }
        Expr::TypeCast { expr: inner, type_name, .. } => {
            format!("{}::{:?}", format_expr_brief(inner), type_name)
        }
        _ => format!("{:?}", expr),
    }
}

fn extract_result_columns(stmt: &Statement) -> Vec<ResultColumn> {
    let select = match stmt {
        Statement::Select(s) => &s.node,
        _ => return Vec::new(),
    };
    let mut columns = Vec::new();
    for target in &select.targets {
        match target {
            SelectTarget::Expr(expr, alias) => {
                let name = alias.clone().or_else(|| infer_column_name(expr)).unwrap_or_else(|| "?".to_string());
                columns.push(ResultColumn { name, inferred_type: None, expression: format_expr_brief(expr) });
            }
            SelectTarget::Star(table) => {
                let name = table.as_ref().map(|t| format!("{}.*", t)).unwrap_or_else(|| "*".to_string());
                columns.push(ResultColumn { name, inferred_type: None, expression: "*".to_string() });
            }
        }
    }
    columns
}

fn infer_column_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::ColumnRef(names) => names.last().cloned(),
        Expr::PlVariable(names) => names.last().cloned(),
        Expr::Literal(Literal::String(_)) => Some("?column?".to_string()),
        _ => None,
    }
}

fn make_annotation(
    out_param: &str,
    position: usize,
    cursor_type: &str,
    branch_ctx: &BranchContext,
    sql: String,
    sql_source: &str,
    parsed_query: Option<Box<Statement>>,
    result_columns: Vec<ResultColumn>,
) -> ReturnCursorAnnotation {
    ReturnCursorAnnotation {
        out_param: out_param.to_string(),
        out_position: position,
        out_type: cursor_type.to_string(),
        branch_path: branch_ctx.path.clone(),
        branch_condition: branch_ctx.condition.clone(),
        jdbc_type: "REF_CURSOR".to_string(),
        sql,
        sql_source: sql_source.to_string(),
        parsed_query,
        result_columns,
    }
}

fn process_open_stmt(
    open: &Spanned<PlOpenStmt>,
    out_cursors: &[OutCursorInfo],
    is_func_return: bool,
    return_cursor_type: Option<&str>,
    branch_ctx: &BranchContext,
    annotations: &mut Vec<ReturnCursorAnnotation>,
) {
    let cursor_name = extract_cursor_name(&open.cursor);

    let matched_out =
        cursor_name.as_ref().and_then(|name| out_cursors.iter().find(|c| c.name.eq_ignore_ascii_case(name)));

    if matched_out.is_none() && !is_func_return {
        return;
    }

    let (out_param, position, cursor_type) = if let Some(info) = matched_out {
        (info.name.clone(), info.position, info.cursor_type.clone())
    } else if is_func_return {
        ("<return>".to_string(), 0, return_cursor_type.unwrap_or("REFCURSOR").to_string())
    } else {
        return;
    };

    match &open.kind {
        PlOpenKind::ForQuery { query, parsed_query, .. } => {
            let result_columns = parsed_query.as_ref().map(|pq| extract_result_columns(pq)).unwrap_or_default();
            annotations.push(make_annotation(
                &out_param,
                position,
                &cursor_type,
                branch_ctx,
                query.clone(),
                "static",
                parsed_query.clone(),
                result_columns,
            ));
        }
        PlOpenKind::ForExecute { query, using_args } => {
            let query_str = format_expr_brief(query);
            let parsed = crate::parser::Parser::parse_statement_from_str(&query_str);
            let result_columns = parsed.as_ref().map(|pq| extract_result_columns(pq)).unwrap_or_default();
            annotations.push(make_annotation(
                &out_param,
                position,
                &cursor_type,
                branch_ctx,
                query_str,
                "dynamic",
                parsed,
                result_columns,
            ));
            let _ = using_args;
        }
        PlOpenKind::Simple { .. } | PlOpenKind::ForUsing { .. } => {}
    }
}

fn process_return_query(
    rq: &Spanned<PlReturnQueryStmt>,
    return_cursor_type: Option<&str>,
    branch_ctx: &BranchContext,
    annotations: &mut Vec<ReturnCursorAnnotation>,
) {
    let sql_source = if rq.is_dynamic { "dynamic" } else { "static" };
    let sql = if rq.is_dynamic {
        rq.dynamic_expr.as_ref().map(format_expr_brief).unwrap_or_else(|| rq.query.clone())
    } else {
        rq.query.clone()
    };
    let parsed = crate::parser::Parser::parse_statement_from_str(&sql);
    let result_columns = parsed.as_ref().map(|pq| extract_result_columns(pq)).unwrap_or_default();
    annotations.push(make_annotation(
        "<return>",
        0,
        return_cursor_type.unwrap_or("REFCURSOR"),
        branch_ctx,
        sql,
        sql_source,
        parsed,
        result_columns,
    ));
}

fn walk_statements(
    stmts: &[PlStatement],
    out_cursors: &[OutCursorInfo],
    is_func_return: bool,
    return_cursor_type: Option<&str>,
    branch_ctx: &BranchContext,
    annotations: &mut Vec<ReturnCursorAnnotation>,
) {
    for stmt in stmts {
        match stmt {
            PlStatement::Open(open) => {
                process_open_stmt(open, out_cursors, is_func_return, return_cursor_type, branch_ctx, annotations);
            }
            PlStatement::ReturnQuery(rq) if is_func_return => {
                process_return_query(rq, return_cursor_type, branch_ctx, annotations);
            }
            PlStatement::If(if_stmt) => {
                walk_if(if_stmt, out_cursors, is_func_return, return_cursor_type, branch_ctx, annotations);
            }
            PlStatement::Case(case_stmt) => {
                walk_case(case_stmt, out_cursors, is_func_return, return_cursor_type, branch_ctx, annotations);
            }
            PlStatement::Loop(loop_stmt) => {
                walk_loop(loop_stmt, out_cursors, is_func_return, return_cursor_type, branch_ctx, annotations);
            }
            PlStatement::While(while_stmt) => {
                walk_while(while_stmt, out_cursors, is_func_return, return_cursor_type, branch_ctx, annotations);
            }
            PlStatement::For(for_stmt) => {
                walk_for(for_stmt, out_cursors, is_func_return, return_cursor_type, branch_ctx, annotations);
            }
            PlStatement::Block(block) => {
                walk_statements(&block.body, out_cursors, is_func_return, return_cursor_type, branch_ctx, annotations);
                if let Some(ref eb) = block.exception_block {
                    for handler in &eb.handlers {
                        walk_statements(
                            &handler.statements,
                            out_cursors,
                            is_func_return,
                            return_cursor_type,
                            branch_ctx,
                            annotations,
                        );
                    }
                }
            }
            _ => {}
        }
    }
}

fn push_path(parent: &str, segment: &str) -> String {
    if parent.is_empty() {
        segment.to_string()
    } else {
        format!("{}.{}", parent, segment)
    }
}

fn walk_if(
    if_stmt: &PlIfStmt,
    out_cursors: &[OutCursorInfo],
    is_func_return: bool,
    return_cursor_type: Option<&str>,
    parent_ctx: &BranchContext,
    annotations: &mut Vec<ReturnCursorAnnotation>,
) {
    let then_path = push_path(&parent_ctx.path, "IF.then");
    let then_ctx = BranchContext { path: then_path, condition: format_expr_brief(&if_stmt.condition) };
    walk_statements(&if_stmt.then_stmts, out_cursors, is_func_return, return_cursor_type, &then_ctx, annotations);

    for (i, elsif) in if_stmt.elsifs.iter().enumerate() {
        let elsif_path = push_path(&parent_ctx.path, &format!("IF.elsif#{}.then", i + 1));
        let elsif_ctx = BranchContext { path: elsif_path, condition: format_expr_brief(&elsif.condition) };
        walk_statements(&elsif.stmts, out_cursors, is_func_return, return_cursor_type, &elsif_ctx, annotations);
    }

    if !if_stmt.else_stmts.is_empty() {
        let else_path = push_path(&parent_ctx.path, "IF.else");
        let else_ctx = BranchContext { path: else_path, condition: String::new() };
        walk_statements(&if_stmt.else_stmts, out_cursors, is_func_return, return_cursor_type, &else_ctx, annotations);
    }
}

fn walk_case(
    case_stmt: &PlCaseStmt,
    out_cursors: &[OutCursorInfo],
    is_func_return: bool,
    return_cursor_type: Option<&str>,
    parent_ctx: &BranchContext,
    annotations: &mut Vec<ReturnCursorAnnotation>,
) {
    for (i, when) in case_stmt.whens.iter().enumerate() {
        let when_path = push_path(&parent_ctx.path, &format!("CASE.when#{}", i + 1));
        let when_ctx = BranchContext { path: when_path, condition: format_expr_brief(&when.condition) };
        walk_statements(&when.stmts, out_cursors, is_func_return, return_cursor_type, &when_ctx, annotations);
    }

    if !case_stmt.else_stmts.is_empty() {
        let else_path = push_path(&parent_ctx.path, "CASE.else");
        let else_ctx = BranchContext { path: else_path, condition: String::new() };
        walk_statements(&case_stmt.else_stmts, out_cursors, is_func_return, return_cursor_type, &else_ctx, annotations);
    }
}

fn walk_loop(
    loop_stmt: &PlLoopStmt,
    out_cursors: &[OutCursorInfo],
    is_func_return: bool,
    return_cursor_type: Option<&str>,
    parent_ctx: &BranchContext,
    annotations: &mut Vec<ReturnCursorAnnotation>,
) {
    let path = push_path(&parent_ctx.path, "LOOP.body");
    let ctx = BranchContext { path, condition: String::new() };
    walk_statements(&loop_stmt.body, out_cursors, is_func_return, return_cursor_type, &ctx, annotations);
}

fn walk_while(
    while_stmt: &PlWhileStmt,
    out_cursors: &[OutCursorInfo],
    is_func_return: bool,
    return_cursor_type: Option<&str>,
    parent_ctx: &BranchContext,
    annotations: &mut Vec<ReturnCursorAnnotation>,
) {
    let path = push_path(&parent_ctx.path, "WHILE.body");
    let ctx = BranchContext { path, condition: format_expr_brief(&while_stmt.condition) };
    walk_statements(&while_stmt.body, out_cursors, is_func_return, return_cursor_type, &ctx, annotations);
}

fn walk_for(
    for_stmt: &PlForStmt,
    out_cursors: &[OutCursorInfo],
    is_func_return: bool,
    return_cursor_type: Option<&str>,
    parent_ctx: &BranchContext,
    annotations: &mut Vec<ReturnCursorAnnotation>,
) {
    let path = push_path(&parent_ctx.path, "FOR.body");
    let ctx = BranchContext { path, condition: String::new() };
    walk_statements(&for_stmt.body, out_cursors, is_func_return, return_cursor_type, &ctx, annotations);
}

fn group_annotations(annotations: Vec<ReturnCursorAnnotation>) -> Vec<ReturnCursorGroup> {
    let mut map: HashMap<String, ReturnCursorGroup> = HashMap::new();
    for ann in annotations {
        let key = format!("{}:{}", ann.out_param, ann.out_position);
        let group = map.entry(key).or_insert_with(|| ReturnCursorGroup {
            out_param: ann.out_param.clone(),
            position: ann.out_position,
            cursor_type: ann.out_type.clone(),
            jdbc_type: ann.jdbc_type.clone(),
            branches: Vec::new(),
        });
        group.branches.push(ReturnCursorBranch {
            path: ann.branch_path,
            condition: ann.branch_condition,
            sql: ann.sql,
            sql_source: ann.sql_source,
            result_columns: ann.result_columns,
        });
    }
    let mut groups: Vec<ReturnCursorGroup> = map.into_values().collect();
    groups.sort_by(|a, b| a.position.cmp(&b.position).then_with(|| a.out_param.cmp(&b.out_param)));
    groups
}

pub fn analyze_return_cursors(
    block: &PlBlock,
    params: &[RoutineParam],
    routine_name: &str,
    routine_kind: &str,
    return_type: Option<&str>,
) -> RoutineReturnAnalysis {
    let mut out_cursors = Vec::new();
    for (i, param) in params.iter().enumerate() {
        if is_out_refcursor(param) {
            out_cursors.push(OutCursorInfo {
                name: param.name.clone(),
                position: i + 1,
                cursor_type: param.data_type.clone(),
            });
        }
    }

    let is_func_return = is_return_refcursor(return_type);
    let return_cursor_type =
        return_type.map(|rt| if rt.to_uppercase().contains("REFCURSOR") { rt.to_string() } else { rt.to_string() });

    let root_ctx = BranchContext { path: String::new(), condition: String::new() };

    let mut annotations = Vec::new();
    walk_statements(
        &block.body,
        &out_cursors,
        is_func_return,
        return_cursor_type.as_deref(),
        &root_ctx,
        &mut annotations,
    );

    let groups = group_annotations(annotations);

    RoutineReturnAnalysis {
        routine_name: routine_name.to_string(),
        routine_kind: routine_kind.to_string(),
        return_cursors: groups,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::token::tokenizer::Tokenizer;

    fn parse_pl(sql: &str) -> Vec<crate::ast::Statement> {
        let tokens = Tokenizer::new(sql).tokenize().unwrap();
        Parser::new(tokens).parse()
    }

    fn first_stmt(stmts: &[crate::ast::Statement]) -> Option<&crate::ast::Statement> {
        stmts.first()
    }

    fn extract_block(stmts: &[crate::ast::Statement]) -> Option<PlBlock> {
        match first_stmt(stmts)? {
            crate::ast::Statement::CreateProcedure(p) => p.block.clone(),
            crate::ast::Statement::CreateFunction(f) => f.block.clone(),
            _ => None,
        }
    }

    fn extract_params(stmts: &[crate::ast::Statement]) -> Vec<RoutineParam> {
        match first_stmt(stmts) {
            Some(crate::ast::Statement::CreateProcedure(p)) => p.parameters.clone(),
            Some(crate::ast::Statement::CreateFunction(f)) => f.parameters.clone(),
            _ => Vec::new(),
        }
    }

    fn extract_return_type(stmts: &[crate::ast::Statement]) -> Option<String> {
        match first_stmt(stmts)? {
            crate::ast::Statement::CreateFunction(f) => f.return_type.clone(),
            _ => None,
        }
    }

    fn extract_name(stmts: &[crate::ast::Statement]) -> String {
        match first_stmt(stmts) {
            Some(crate::ast::Statement::CreateProcedure(p)) => p.name.join("."),
            Some(crate::ast::Statement::CreateFunction(f)) => f.name.join("."),
            _ => "unknown".to_string(),
        }
    }

    fn extract_kind(stmts: &[crate::ast::Statement]) -> String {
        match first_stmt(stmts) {
            Some(crate::ast::Statement::CreateProcedure(_)) => "Procedure".to_string(),
            Some(crate::ast::Statement::CreateFunction(_)) => "Function".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    #[test]
    fn test_basic_out_refcursor_static() {
        let sql = r#"
            CREATE PROCEDURE get_users(p_cur OUT SYS_REFCURSOR) AS
            BEGIN
                OPEN p_cur FOR SELECT id, name FROM users;
            END;
        "#;
        let stmts = parse_pl(sql);
        let block = extract_block(&stmts).unwrap();
        let params = extract_params(&stmts);
        let name = extract_name(&stmts);
        let kind = extract_kind(&stmts);

        let result = analyze_return_cursors(&block, &params, &name, &kind, None);

        assert_eq!(result.routine_name, "get_users");
        assert_eq!(result.routine_kind, "Procedure");
        assert_eq!(result.return_cursors.len(), 1);
        let group = &result.return_cursors[0];
        assert_eq!(group.out_param, "p_cur");
        assert_eq!(group.position, 1);
        assert_eq!(group.cursor_type.to_uppercase(), "SYS_REFCURSOR");
        assert_eq!(group.jdbc_type, "REF_CURSOR");
        assert_eq!(group.branches.len(), 1);
        let branch = &group.branches[0];
        assert_eq!(branch.path, "");
        assert_eq!(branch.sql_source, "static");
        assert!(branch.sql.to_uppercase().contains("SELECT"));
        assert!(branch.sql.to_uppercase().contains("USERS"));
    }

    #[test]
    fn test_conditional_branches() {
        let sql = r#"
            CREATE PROCEDURE get_data(p_cur OUT SYS_REFCURSOR, p_type INTEGER) AS
            BEGIN
                IF p_type = 1 THEN
                    OPEN p_cur FOR SELECT id, name FROM users;
                ELSIF p_type = 2 THEN
                    OPEN p_cur FOR SELECT id, title FROM posts;
                ELSE
                    OPEN p_cur FOR SELECT id, value FROM config;
                END IF;
            END;
        "#;
        let stmts = parse_pl(sql);
        let block = extract_block(&stmts).unwrap();
        let params = extract_params(&stmts);

        let result = analyze_return_cursors(&block, &params, "get_data", "Procedure", None);

        assert_eq!(result.return_cursors.len(), 1);
        let group = &result.return_cursors[0];
        assert_eq!(group.branches.len(), 3);

        assert_eq!(group.branches[0].path, "IF.then");
        assert!(group.branches[0].sql.to_uppercase().contains("USERS"));

        assert_eq!(group.branches[1].path, "IF.elsif#1.then");
        assert!(group.branches[1].sql.to_uppercase().contains("POSTS"));

        assert_eq!(group.branches[2].path, "IF.else");
        assert!(group.branches[2].sql.to_uppercase().contains("CONFIG"));
    }

    #[test]
    fn test_in_cursor_not_annotated() {
        let sql = r#"
            CREATE PROCEDURE process_cursor(p_cur IN SYS_REFCURSOR) AS
            BEGIN
                NULL;
            END;
        "#;
        let stmts = parse_pl(sql);
        let block = extract_block(&stmts).unwrap();
        let params = extract_params(&stmts);

        let result = analyze_return_cursors(&block, &params, "process_cursor", "Procedure", None);

        assert_eq!(result.return_cursors.len(), 0);
    }

    #[test]
    fn test_no_cursor_params() {
        let sql = r#"
            CREATE PROCEDURE no_cursors(p_id INTEGER) AS
            BEGIN
                NULL;
            END;
        "#;
        let stmts = parse_pl(sql);
        let block = extract_block(&stmts).unwrap();
        let params = extract_params(&stmts);

        let result = analyze_return_cursors(&block, &params, "no_cursors", "Procedure", None);

        assert_eq!(result.return_cursors.len(), 0);
    }

    #[test]
    fn test_function_returns_refcursor() {
        let sql = r#"
            CREATE FUNCTION get_users_func RETURN SYS_REFCURSOR AS
            BEGIN
                OPEN rc FOR SELECT id, name FROM users;
                RETURN rc;
            END;
        "#;
        let stmts = parse_pl(sql);
        let block = extract_block(&stmts).unwrap();
        let params = extract_params(&stmts);
        let ret_type = extract_return_type(&stmts);

        let result = analyze_return_cursors(&block, &params, "get_users_func", "Function", ret_type.as_deref());

        assert_eq!(result.return_cursors.len(), 1);
        let group = &result.return_cursors[0];
        assert_eq!(group.out_param, "<return>");
        assert_eq!(group.position, 0);
        assert_eq!(group.branches.len(), 1);
        assert_eq!(group.branches[0].sql_source, "static");
        assert!(group.branches[0].sql.to_uppercase().contains("SELECT"));
    }

    #[test]
    fn test_dynamic_execute_cursor() {
        let sql = r#"
            CREATE PROCEDURE dyn_cursor(p_cur OUT SYS_REFCURSOR, v_sql VARCHAR) AS
            BEGIN
                OPEN p_cur FOR EXECUTE v_sql;
            END;
        "#;
        let stmts = parse_pl(sql);
        let block = extract_block(&stmts).unwrap();
        let params = extract_params(&stmts);

        let result = analyze_return_cursors(&block, &params, "dyn_cursor", "Procedure", None);

        assert_eq!(result.return_cursors.len(), 1);
        let group = &result.return_cursors[0];
        assert_eq!(group.branches.len(), 1);
        assert_eq!(group.branches[0].sql_source, "dynamic");
    }

    #[test]
    fn test_has_return_cursors() {
        let out_cursor = RoutineParam {
            name: "p_cur".to_string(),
            mode: Some("OUT".to_string()),
            data_type: "SYS_REFCURSOR".to_string(),
            default_value: None,
        };
        assert!(has_return_cursors(&[out_cursor.clone()], None));

        let in_cursor = RoutineParam {
            name: "p_cur".to_string(),
            mode: Some("IN".to_string()),
            data_type: "SYS_REFCURSOR".to_string(),
            default_value: None,
        };
        assert!(!has_return_cursors(&[in_cursor], None));

        let no_cursor = RoutineParam {
            name: "p_id".to_string(),
            mode: Some("IN".to_string()),
            data_type: "INTEGER".to_string(),
            default_value: None,
        };
        assert!(!has_return_cursors(&[no_cursor], None));

        assert!(has_return_cursors(&[], Some("SYS_REFCURSOR")));
        assert!(!has_return_cursors(&[], Some("INTEGER")));
    }

    #[test]
    fn test_result_columns_extracted() {
        let sql = r#"
            CREATE PROCEDURE get_cols(p_cur OUT SYS_REFCURSOR) AS
            BEGIN
                OPEN p_cur FOR SELECT id, name AS user_name FROM users;
            END;
        "#;
        let stmts = parse_pl(sql);
        let block = extract_block(&stmts).unwrap();
        let params = extract_params(&stmts);

        let result = analyze_return_cursors(&block, &params, "get_cols", "Procedure", None);

        assert_eq!(result.return_cursors.len(), 1);
        let group = &result.return_cursors[0];
        assert_eq!(group.branches.len(), 1);
        let cols = &group.branches[0].result_columns;
        assert_eq!(cols.len(), 2);
        assert_eq!(cols[0].name, "id");
        assert_eq!(cols[1].name, "user_name");
    }
}
