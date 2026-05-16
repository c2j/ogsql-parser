# Return Cursor SQL Annotation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Annotate SQL statements that are returned via OUT REFCURSOR parameters (or function RETURNS REFCURSOR) with structured metadata, including branch path tracking for conditional opens.

**Architecture:** A new analyzer module `return_cursor` computes `ReturnCursorAnnotation` structs by walking PL/pgSQL blocks with branch context. The annotation is injected into JSON output (both in-place on Open/ReturnQuery statements and as a summary `routine_analysis` section) and surfaces as a new `ReturnCursorSQL` CSV row type. The core AST types are NOT modified — annotation is purely computed and layered on output.

**Tech Stack:** Rust, serde (JSON serialization), existing analyzer patterns.

---

## Key Files Reference

| File | Role |
|------|------|
| `src/ast/mod.rs` | `RoutineParam`, `CreateFunctionStatement`, `CreateProcedureStatement`, `PackageProcedure`, `PackageFunction`, `StatementInfo` |
| `src/ast/plpgsql.rs` | `PlStatement::Open`, `PlOpenKind`, `PlReturnQueryStmt`, `PlStatement::Return`, `PlBlock`, branch types |
| `src/analyzer/mod.rs` | Existing `find_ref_cursor_queries()`, `RefCursorQuery`, `DynamicSqlReport`, injection patterns |
| `src/analyzer/return_cursor.rs` | **NEW** — Core annotation computation module |
| `src/analyzer/tests.rs` | Existing `parse_proc_block()` helper, ref_cursor tests |
| `src/bin/ogsql.rs` | CSV `ParseCsvRow`, `flatten_statement()`, `collect_pl_stmt_rows()`, JSON injection in `cmd_parse()` |
| `src/lib.rs` | Public re-exports |

---

## Task 1: Define Annotation Data Structures

**Files:**
- Create: `src/analyzer/return_cursor.rs`

**Step 1: Create the module with data structures**

```rust
// src/analyzer/return_cursor.rs
//! Annotation of SQL statements returned via OUT REFCURSOR parameters.
//!
//! When a stored procedure has an OUT parameter of type REFCURSOR,
//! and the body contains `OPEN param FOR SELECT ...`, the SQL is
//! the actual data returned to the caller. This module identifies
//! those SQL statements and annotates them with structured metadata.

use std::collections::HashSet;
use crate::ast::plpgsql::{PlBlock, PlStatement, PlOpenKind};
use crate::ast::RoutineParam;
use serde::{Serialize, Deserialize};

/// Annotation attached to a SQL statement that is returned via a cursor OUT parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnCursorAnnotation {
    /// Name of the OUT parameter (e.g., "out_list") or "<return>" for function return
    pub out_param: String,
    /// 1-based parameter position in the routine signature (for JDBC bind)
    pub out_position: usize,
    /// The cursor type string: "REFCURSOR", "SYS_REFCURSOR", etc.
    pub out_type: String,
    /// Branch path leading to this SQL: "", "IF.then", "IF.elsif#1.then", "IF.else", "LOOP.body"
    pub branch_path: String,
    /// The branch condition expression: "p_flag = 'Y'", "" for top-level or else
    pub branch_condition: String,
    /// JDBC type constant name: "REF_CURSOR" (oracle.jdbc.OracleTypes.CURSOR = -10)
    pub jdbc_type: String,
    /// The SQL text (resolved, with PL variables substituted if possible)
    pub sql: String,
    /// Whether the SQL is static ("static") or dynamic via EXECUTE ("dynamic")
    pub sql_source: String,
    /// Parsed SQL statement (if the SQL is static and parseable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed_query: Option<Box<crate::ast::Statement>>,
    /// Result set column info (if inferrable from the SQL)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result_columns: Vec<ResultColumn>,
}

/// A column in the result set returned by the cursor SQL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultColumn {
    /// Column name or alias
    pub name: String,
    /// Inferred data type (requires schema; None if not inferrable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inferred_type: Option<String>,
    /// Original expression in the SELECT target
    pub expression: String,
}

/// Summary of all return cursor annotations for a single routine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineReturnAnalysis {
    /// Name of the routine
    pub routine_name: String,
    /// "Procedure" or "Function"
    pub routine_kind: String,
    /// All return cursor annotations, grouped by out parameter
    pub return_cursors: Vec<ReturnCursorGroup>,
}

/// A single OUT REFCURSOR parameter and its associated SQL branches.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnCursorGroup {
    /// OUT parameter name
    pub out_param: String,
    /// 1-based position
    pub position: usize,
    /// Type string
    pub cursor_type: String,
    /// JDBC type
    pub jdbc_type: String,
    /// All SQL branches that can be returned via this cursor
    pub branches: Vec<ReturnCursorBranch>,
}

/// One possible SQL that can be returned via a cursor (a branch in the code path).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnCursorBranch {
    /// Branch path: "", "IF.then", "IF.elsif#1.then", "IF.else", "LOOP.body"
    pub path: String,
    /// Branch condition expression
    pub condition: String,
    /// The SQL text
    pub sql: String,
    /// "static" or "dynamic"
    pub sql_source: String,
    /// Dynamic SQL trace info (for EXECUTE-based opens)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<DynamicTrace>,
    /// Result columns
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result_columns: Vec<ResultColumn>,
}

/// Trace information for dynamic SQL returned via cursor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicTrace {
    /// Source variable name
    pub variable: String,
    /// SQL template with placeholders
    pub template: String,
    /// Dynamic parameters
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dynamic_params: Vec<DynamicParam>,
}

// Reuse existing DynamicParam from analyzer if possible, or define our own
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicParam {
    pub source: String,
    pub param_name: String,
}

// ── Context for branch tracking ──

#[derive(Debug, Clone, Default)]
struct BranchContext {
    path: String,
    condition: String,
}

// ── Public API ──

/// Analyze a PL/pgSQL block for return cursor SQL annotations.
///
/// `params` — the routine's parameters
/// `routine_name` — fully qualified routine name (e.g., "PKG_TEST.prc_query")
/// `routine_kind` — "Procedure" or "Function"
/// `return_type` — for functions, the RETURNS type (e.g., "REFCURSOR"); None for procedures
pub fn analyze_return_cursors(
    block: &PlBlock,
    params: &[RoutineParam],
    routine_name: &str,
    routine_kind: &str,
    return_type: Option<&str>,
) -> RoutineReturnAnalysis {
    // Collect OUT REFCURSOR params
    let out_cursors = extract_out_cursors(params);
    let mut annotations: Vec<ReturnCursorAnnotation> = Vec::new();

    collect_annotations(
        &block.body,
        &out_cursors,
        &BranchContext::default(),
        &mut annotations,
    );

    // Also handle function RETURNS REFCURSOR — any OPEN ... FOR in the body
    // where the cursor variable is assigned to a REFCURSOR-typed local variable
    // that gets RETURNed, OR if the function has a local REFCURSOR variable
    // that is RETURNed.
    // For simplicity in this implementation: if return_type contains REFCURSOR,
    // we look for all OPEN ... FOR statements regardless of cursor name
    // and mark them with out_param = "<return>".
    if let Some(rt) = return_type {
        if rt.to_uppercase().contains("REFCURSOR") {
            collect_annotations_for_return(
                &block.body,
                &BranchContext::default(),
                &mut annotations,
            );
        }
    }

    // Group annotations by out_param
    let groups = group_annotations(annotations, params, return_type);

    RoutineReturnAnalysis {
        routine_name: routine_name.to_string(),
        routine_kind: routine_kind.to_string(),
        return_cursors: groups,
    }
}

/// Quick check: does this routine have any OUT REFCURSOR params or return REFCURSOR?
pub fn has_return_cursors(
    params: &[RoutineParam],
    return_type: Option<&str>,
) -> bool {
    params.iter().any(|p| {
        p.mode.as_deref() == Some("OUT")
            && p.data_type.to_uppercase().contains("REFCURSOR")
    }) || return_type.map_or(false, |rt| rt.to_uppercase().contains("REFCURSOR"))
}

// ── Internal ──

type OutCursorSet = Vec<(String, usize, String)>; // (name, position, type)

fn extract_out_cursors(params: &[RoutineParam]) -> OutCursorSet {
    params
        .iter()
        .enumerate()
        .filter(|(_, p)| {
            let is_out = p.mode.as_deref() == Some("OUT")
                || p.mode.as_deref() == Some("INOUT")
                || p.mode.as_deref() == Some("IN OUT");
            let is_cursor = p.data_type.to_uppercase().contains("REFCURSOR");
            is_out && is_cursor
        })
        .map(|(i, p)| {
            (p.name.to_lowercase(), i + 1, p.data_type.clone())
        })
        .collect()
}

fn collect_annotations(
    stmts: &[PlStatement],
    out_cursors: &OutCursorSet,
    branch_ctx: &BranchContext,
    annotations: &mut Vec<ReturnCursorAnnotation>,
) {
    for stmt in stmts {
        match stmt {
            PlStatement::Open(spanned) => {
                let open = &spanned.node;
                if let Some(cursor_name) = extract_cursor_name(&open.cursor) {
                    if let Some((pos, typ)) = find_out_cursor(&cursor_name, out_cursors) {
                        let ann = build_annotation(
                            &cursor_name, pos, &typ,
                            branch_ctx, &open.kind,
                        );
                        annotations.push(ann);
                    }
                }
            }
            PlStatement::If(s) => {
                let cond_str = format_expr(&s.node.condition);
                let then_ctx = BranchContext {
                    path: join_path(&branch_ctx.path, "IF.then"),
                    condition: cond_str.clone(),
                };
                collect_annotations(&s.node.then_stmts, out_cursors, &then_ctx, annotations);
                for (i, elsif) in s.node.elsifs.iter().enumerate() {
                    let elsif_ctx = BranchContext {
                        path: join_path(&branch_ctx.path, &format!("IF.elsif#{}.then", i + 1)),
                        condition: format_expr(&elsif.condition),
                    };
                    collect_annotations(&elsif.stmts, out_cursors, &elsif_ctx, annotations);
                }
                let else_ctx = BranchContext {
                    path: join_path(&branch_ctx.path, "IF.else"),
                    condition: cond_str,
                };
                collect_annotations(&s.node.else_stmts, out_cursors, &else_ctx, annotations);
            }
            PlStatement::Case(s) => {
                for (i, when) in s.node.whens.iter().enumerate() {
                    let ctx = BranchContext {
                        path: join_path(&branch_ctx.path, &format!("CASE.when#{}", i + 1)),
                        condition: format_expr(&when.condition),
                    };
                    collect_annotations(&when.stmts, out_cursors, &ctx, annotations);
                }
                let else_ctx = BranchContext {
                    path: join_path(&branch_ctx.path, "CASE.else"),
                    condition: String::new(),
                };
                collect_annotations(&s.node.else_stmts, out_cursors, &else_ctx, annotations);
            }
            PlStatement::Loop(s) => {
                let ctx = BranchContext {
                    path: join_path(&branch_ctx.path, "LOOP.body"),
                    condition: String::new(),
                };
                collect_annotations(&s.node.body, out_cursors, &ctx, annotations);
            }
            PlStatement::While(s) => {
                let ctx = BranchContext {
                    path: join_path(&branch_ctx.path, "WHILE.body"),
                    condition: format_expr(&s.node.condition),
                };
                collect_annotations(&s.node.body, out_cursors, &ctx, annotations);
            }
            PlStatement::For(s) => {
                let ctx = BranchContext {
                    path: join_path(&branch_ctx.path, "FOR.body"),
                    condition: String::new(),
                };
                collect_annotations(&s.node.body, out_cursors, &ctx, annotations);
            }
            PlStatement::Block(s) => {
                collect_annotations(&s.node.body, out_cursors, branch_ctx, annotations);
                if let Some(ref exc) = s.node.exception_block {
                    for handler in &exc.handlers {
                        let ctx = BranchContext {
                            path: join_path(&branch_ctx.path, "EXCEPTION.handler"),
                            condition: handler.conditions.join(", "),
                        };
                        collect_annotations(&handler.statements, out_cursors, &ctx, annotations);
                    }
                }
            }
            _ => {}
        }
    }
}

/// For functions returning REFCURSOR: collect ALL OPEN ... FOR statements.
fn collect_annotations_for_return(
    stmts: &[PlStatement],
    branch_ctx: &BranchContext,
    annotations: &mut Vec<ReturnCursorAnnotation>,
) {
    for stmt in stmts {
        match stmt {
            PlStatement::Open(spanned) => {
                let open = &spanned.node;
                let cursor_name = extract_cursor_name(&open.cursor).unwrap_or_default();
                let ann = build_annotation(
                    &cursor_name, 0, "SYS_REFCURSOR",
                    branch_ctx, &open.kind,
                );
                // Override out_param to indicate function return
                let mut ann = ann;
                ann.out_param = "<return>".to_string();
                annotations.push(ann);
            }
            PlStatement::ReturnQuery(spanned) => {
                let rq = &spanned.node;
                if !rq.is_dynamic {
                    if let Some(ref parsed) = rq.parsed_query {
                        // handled via Open path — but RETURN QUERY without OPEN
                    }
                    let sql = rq.query.clone();
                    annotations.push(ReturnCursorAnnotation {
                        out_param: "<return>".to_string(),
                        out_position: 0,
                        out_type: "SYS_REFCURSOR".to_string(),
                        branch_path: branch_ctx.path.clone(),
                        branch_condition: branch_ctx.condition.clone(),
                        jdbc_type: "REF_CURSOR".to_string(),
                        sql,
                        sql_source: "static".to_string(),
                        parsed_query: None,
                        result_columns: Vec::new(),
                    });
                }
            }
            // Recurse into branches (same as collect_annotations but without out_cursors filter)
            PlStatement::If(s) => {
                let cond_str = format_expr(&s.node.condition);
                let then_ctx = BranchContext {
                    path: join_path(&branch_ctx.path, "IF.then"),
                    condition: cond_str.clone(),
                };
                collect_annotations_for_return(&s.node.then_stmts, &then_ctx, annotations);
                for (i, elsif) in s.node.elsifs.iter().enumerate() {
                    let elsif_ctx = BranchContext {
                        path: join_path(&branch_ctx.path, &format!("IF.elsif#{}.then", i + 1)),
                        condition: format_expr(&elsif.condition),
                    };
                    collect_annotations_for_return(&elsif.stmts, &elsif_ctx, annotations);
                }
                let else_ctx = BranchContext {
                    path: join_path(&branch_ctx.path, "IF.else"),
                    condition: cond_str,
                };
                collect_annotations_for_return(&s.node.else_stmts, &else_ctx, annotations);
            }
            PlStatement::Block(s) => {
                collect_annotations_for_return(&s.node.body, branch_ctx, annotations);
            }
            PlStatement::Loop(s) => {
                let ctx = BranchContext { path: join_path(&branch_ctx.path, "LOOP.body"), condition: String::new() };
                collect_annotations_for_return(&s.node.body, &ctx, annotations);
            }
            PlStatement::While(s) => {
                let ctx = BranchContext { path: join_path(&branch_ctx.path, "WHILE.body"), condition: format_expr(&s.node.condition) };
                collect_annotations_for_return(&s.node.body, &ctx, annotations);
            }
            PlStatement::For(s) => {
                let ctx = BranchContext { path: join_path(&branch_ctx.path, "FOR.body"), condition: String::new() };
                collect_annotations_for_return(&s.node.body, &ctx, annotations);
            }
            _ => {}
        }
    }
}

fn build_annotation(
    cursor_name: &str,
    position: usize,
    cursor_type: &str,
    branch_ctx: &BranchContext,
    kind: &PlOpenKind,
) -> ReturnCursorAnnotation {
    match kind {
        PlOpenKind::ForQuery { query, parsed_query, .. } => {
            let result_columns = extract_result_columns(parsed_query.as_deref());
            ReturnCursorAnnotation {
                out_param: cursor_name.to_string(),
                out_position: position,
                out_type: cursor_type.to_string(),
                branch_path: branch_ctx.path.clone(),
                branch_condition: branch_ctx.condition.clone(),
                jdbc_type: "REF_CURSOR".to_string(),
                sql: query.clone(),
                sql_source: "static".to_string(),
                parsed_query: parsed_query.clone(),
                result_columns,
            }
        }
        PlOpenKind::ForExecute { query, .. } => {
            let sql = match query {
                crate::ast::Expr::PlVariable(n) | crate::ast::Expr::ColumnRef(n) => n.join("."),
                crate::ast::Expr::Literal(crate::ast::Literal::String(s)) => s.clone(),
                other => format_expr(other),
            };
            ReturnCursorAnnotation {
                out_param: cursor_name.to_string(),
                out_position: position,
                out_type: cursor_type.to_string(),
                branch_path: branch_ctx.path.clone(),
                branch_condition: branch_ctx.condition.clone(),
                jdbc_type: "REF_CURSOR".to_string(),
                sql,
                sql_source: "dynamic".to_string(),
                parsed_query: None,
                result_columns: Vec::new(),
            }
        }
        _ => ReturnCursorAnnotation {
            out_param: cursor_name.to_string(),
            out_position: position,
            out_type: cursor_type.to_string(),
            branch_path: branch_ctx.path.clone(),
            branch_condition: branch_ctx.condition.clone(),
            jdbc_type: "REF_CURSOR".to_string(),
            sql: String::new(),
            sql_source: String::new(),
            parsed_query: None,
            result_columns: Vec::new(),
        }
    }
}

fn extract_cursor_name(expr: &crate::ast::Expr) -> Option<String> {
    match expr {
        crate::ast::Expr::PlVariable(names) | crate::ast::Expr::ColumnRef(names) if names.len() == 1 => {
            Some(names[0].clone())
        }
        _ => None,
    }
}

fn find_out_cursor(name: &str, out_cursors: &OutCursorSet) -> Option<(usize, String)> {
    let name_lower = name.to_lowercase();
    for (n, pos, typ) in out_cursors {
        if n.to_lowercase() == name_lower {
            return Some((*pos, typ.clone()));
        }
    }
    None
}

fn join_path(parent: &str, segment: &str) -> String {
    if parent.is_empty() {
        segment.to_string()
    } else {
        format!("{}.{}", parent, segment)
    }
}

fn format_expr(expr: &crate::ast::Expr) -> String {
    // Use formatter for a readable expression string
    let formatter = crate::formatter::SqlFormatter::new();
    formatter.format_expr(expr)
}

fn extract_result_columns(stmt: Option<&crate::ast::Statement>) -> Vec<ResultColumn> {
    let stmt = match stmt {
        Some(s) => s,
        None => return Vec::new(),
    };
    let select = match stmt {
        crate::ast::Statement::Select(s) => s,
        _ => return Vec::new(),
    };
    select.targets.iter().map(|t| {
        match t {
            crate::ast::SelectTarget::Expr(expr, alias) => {
                let name = alias.clone().unwrap_or_else(|| format_expr(expr));
                let expression = format_expr(expr);
                ResultColumn {
                    name,
                    inferred_type: None,
                    expression,
                }
            }
            _ => ResultColumn {
                name: "*".to_string(),
                inferred_type: None,
                expression: "*".to_string(),
            }
        }
    }).collect()
}

fn group_annotations(
    annotations: Vec<ReturnCursorAnnotation>,
    params: &[RoutineParam],
    return_type: Option<&str>,
) -> Vec<ReturnCursorGroup> {
    // Group by out_param
    let mut param_map: std::collections::HashMap<String, Vec<&ReturnCursorAnnotation>> =
        std::collections::HashMap::new();
    for ann in &annotations {
        param_map
            .entry(ann.out_param.clone())
            .or_default()
            .push(ann);
    }

    let mut groups = Vec::new();

    for (param_name, anns) in param_map {
        let first = anns[0];
        let branches: Vec<ReturnCursorBranch> = anns.iter().map(|a| {
            ReturnCursorBranch {
                path: a.branch_path.clone(),
                condition: a.branch_condition.clone(),
                sql: a.sql.clone(),
                sql_source: a.sql_source.clone(),
                trace: None, // TODO: integrate with DynamicSqlReport for dynamic traces
                result_columns: a.result_columns.clone(),
            }
        }).collect();

        groups.push(ReturnCursorGroup {
            out_param: param_name,
            position: first.out_position,
            cursor_type: first.out_type.clone(),
            jdbc_type: first.jdbc_type.clone(),
            branches,
        });
    }

    groups
}
```

**Step 2: Register the module**

In `src/analyzer/mod.rs`, add at the top:
```rust
pub mod return_cursor;
```

And add the public re-export in `src/lib.rs`:
```rust
pub use analyzer::return_cursor::{
    ReturnCursorAnnotation, RoutineReturnAnalysis, ReturnCursorGroup,
    ReturnCursorBranch, ResultColumn, analyze_return_cursors, has_return_cursors,
};
```

**Step 3: Verify compilation**

Run: `cargo build 2>&1 | head -50`
Expected: Compiles with possible warnings about unused imports. Fix any compile errors.

**Step 4: Commit**

```bash
git add src/analyzer/return_cursor.rs src/analyzer/mod.rs src/lib.rs
git commit -m "feat: add return cursor annotation data structures and core analysis"
```

---

## Task 2: Add Unit Tests for Core Analysis

**Files:**
- Modify: `src/analyzer/return_cursor.rs` (add `#[cfg(test)] mod tests` at bottom)

**Step 1: Write tests**

Append to `src/analyzer/return_cursor.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::Tokenizer;
    use crate::ast::PackageItem;

    /// Helper: parse a package body and extract the first procedure's block + params
    fn parse_proc(sql: &str) -> (PlBlock, Vec<RoutineParam>, String) {
        let tokens = Tokenizer::new(sql).tokenize().unwrap();
        let stmts = Parser::new(tokens).parse();
        match &stmts[0] {
            crate::ast::Statement::CreatePackageBody(pkg) => {
                for item in &pkg.node.items {
                    if let PackageItem::Procedure(p) = item {
                        let block = p.block.as_ref().expect("procedure should have block").clone();
                        return (block, p.parameters.clone(), p.name.join("."));
                    }
                }
                panic!("no procedure found in package body");
            }
            crate::ast::Statement::CreateProcedure(p) => {
                let block = p.block.as_ref().expect("procedure should have block").clone();
                return (block, p.parameters.clone(), p.name.join("."));
            }
            other => panic!("expected package body or procedure, got {:?}", other),
        }
    }

    /// Helper: parse a function and extract block + params + return_type
    fn parse_func(sql: &str) -> (PlBlock, Vec<RoutineParam>, String, Option<String>) {
        let tokens = Tokenizer::new(sql).tokenize().unwrap();
        let stmts = Parser::new(tokens).parse();
        match &stmts[0] {
            crate::ast::Statement::CreateFunction(f) => {
                let block = f.block.as_ref().expect("function should have block").clone();
                return (block, f.parameters.clone(), f.name.join("."), f.return_type.clone());
            }
            other => panic!("expected function, got {:?}", other),
        }
    }

    #[test]
    fn test_basic_out_refcursor_static() {
        let (block, params, name) = parse_proc(
            r#"CREATE OR REPLACE PACKAGE BODY PKG AS
              PROCEDURE get_users(p_status VARCHAR, p_cur OUT REFCURSOR) AS
              BEGIN
                OPEN p_cur FOR SELECT id, name FROM t_users WHERE status = p_status;
              END;
            END PKG;"#
        );
        let analysis = analyze_return_cursors(&block, &params, &name, "Procedure", None);
        assert_eq!(analysis.return_cursors.len(), 1);
        let group = &analysis.return_cursors[0];
        assert_eq!(group.out_param, "p_cur");
        assert_eq!(group.position, 2);
        assert_eq!(group.branches.len(), 1);
        assert!(group.branches[0].sql.contains("t_users"));
        assert_eq!(group.branches[0].path, "");
        assert_eq!(group.branches[0].sql_source, "static");
    }

    #[test]
    fn test_conditional_branches() {
        let (block, params, name) = parse_proc(
            r#"CREATE OR REPLACE PACKAGE BODY PKG AS
              PROCEDURE search(p_type INT, p_cur OUT REFCURSOR) AS
              BEGIN
                IF p_type = 1 THEN
                  OPEN p_cur FOR SELECT id FROM t1;
                ELSIF p_type = 2 THEN
                  OPEN p_cur FOR SELECT id, name FROM t2;
                ELSE
                  OPEN p_cur FOR SELECT * FROM t3;
                END IF;
              END;
            END PKG;"#
        );
        let analysis = analyze_return_cursors(&block, &params, &name, "Procedure", None);
        assert_eq!(analysis.return_cursors.len(), 1);
        let group = &analysis.return_cursors[0];
        assert_eq!(group.branches.len(), 3);
        assert_eq!(group.branches[0].path, "IF.then");
        assert_eq!(group.branches[0].condition, "p_type = 1");
        assert_eq!(group.branches[1].path, "IF.elsif#1.then");
        assert_eq!(group.branches[1].condition, "p_type = 2");
        assert_eq!(group.branches[2].path, "IF.else");
    }

    #[test]
    fn test_in_cursor_not_annotated() {
        let (block, params, name) = parse_proc(
            r#"CREATE OR REPLACE PACKAGE BODY PKG AS
              PROCEDURE proc(cur IN REFCURSOR) AS
              BEGIN
                OPEN cur FOR SELECT 1;
              END;
            END PKG;"#
        );
        let analysis = analyze_return_cursors(&block, &params, &name, "Procedure", None);
        assert!(analysis.return_cursors.is_empty());
    }

    #[test]
    fn test_no_cursor_params() {
        let (block, params, name) = parse_proc(
            r#"CREATE OR REPLACE PACKAGE BODY PKG AS
              PROCEDURE proc(p_id INT) AS
              BEGIN
                NULL;
              END;
            END PKG;"#
        );
        let analysis = analyze_return_cursors(&block, &params, &name, "Procedure", None);
        assert!(analysis.return_cursors.is_empty());
    }

    #[test]
    fn test_function_returns_refcursor() {
        let (block, params, name, return_type) = parse_func(
            r#"CREATE FUNCTION get_user(p_id INT) RETURNS SYS_REFCURSOR AS $$
              DECLARE v_cur SYS_REFCURSOR;
              BEGIN
                OPEN v_cur FOR SELECT id, name FROM t_users WHERE id = p_id;
                RETURN v_cur;
              END;
            $$ LANGUAGE plpgsql"#
        );
        let analysis = analyze_return_cursors(
            &block, &params, &name, "Function", return_type.as_deref(),
        );
        assert_eq!(analysis.return_cursors.len(), 1);
        assert_eq!(analysis.return_cursors[0].out_param, "<return>");
    }

    #[test]
    fn test_dynamic_execute_cursor() {
        let (block, params, name) = parse_proc(
            r#"CREATE OR REPLACE PACKAGE BODY PKG AS
              PROCEDURE dyn_query(p_table VARCHAR, p_cur OUT REFCURSOR) AS
              BEGIN
                OPEN p_cur FOR EXECUTE 'SELECT * FROM ' || p_table;
              END;
            END PKG;"#
        );
        let analysis = analyze_return_cursors(&block, &params, &name, "Procedure", None);
        assert_eq!(analysis.return_cursors.len(), 1);
        assert_eq!(analysis.return_cursors[0].branches[0].sql_source, "dynamic");
    }

    #[test]
    fn test_has_return_cursors() {
        let params_no = vec![RoutineParam {
            name: "p".to_string(), mode: None,
            data_type: "INT".to_string(), default_value: None,
        }];
        assert!(!has_return_cursors(&params_no, None));

        let params_yes = vec![RoutineParam {
            name: "cur".to_string(), mode: Some("OUT".to_string()),
            data_type: "REFCURSOR".to_string(), default_value: None,
        }];
        assert!(has_return_cursors(&params_yes, None));
        assert!(has_return_cursors(&params_no, Some("SYS_REFCURSOR")));
    }

    #[test]
    fn test_result_columns_extracted() {
        let (block, params, name) = parse_proc(
            r#"CREATE OR REPLACE PACKAGE BODY PKG AS
              PROCEDURE get_users(p_cur OUT REFCURSOR) AS
              BEGIN
                OPEN p_cur FOR SELECT id, name AS user_name, email FROM t_users;
              END;
            END PKG;"#
        );
        let analysis = analyze_return_cursors(&block, &params, &name, "Procedure", None);
        let cols = &analysis.return_cursors[0].branches[0].result_columns;
        assert!(cols.len() >= 2);
        // Check that alias is picked up
        assert!(cols.iter().any(|c| c.name == "user_name"));
    }
}
```

**Step 2: Run tests**

Run: `cargo test return_cursor -- --nocapture`
Expected: All 8 tests pass.

**Step 3: Fix any compilation issues**

The `format_expr` method on `SqlFormatter` may or may not exist. If not, check `src/formatter.rs` for the correct method name (likely `format_expression`). Also `SelectTarget` enum variant names may differ — check `src/ast/mod.rs` for exact names.

**Step 4: Commit**

```bash
git add src/analyzer/return_cursor.rs
git commit -m "test: add return cursor annotation unit tests (8 tests)"
```

---

## Task 3: CSV Output — New Row Type

**Files:**
- Modify: `src/bin/ogsql.rs`

**Step 1: Extend ParseCsvRow with branch fields**

Find `ParseCsvRow` struct (around line 604) and add:
```rust
struct ParseCsvRow {
    line: usize,
    stmt_type: String,
    name: String,
    parent: String,
    parameters: String,
    return_type: String,
    sql: String,
    // NEW: branch tracking for ReturnCursorSQL rows
    branch_path: String,
    branch_condition: String,
}
```

**Step 2: Update CSV header**

Find `output_csv_parse_header()` (line 600) and change to:
```rust
fn output_csv_parse_header() {
    println!("file,directory,line,type,name,parent,parameters,return_type,sql,error,warning,branch_path,branch_condition");
}
```

**Step 3: Update CSV row output**

Find `output_csv_parse_rows()` (line 1866). In the print statement, add the two new fields:
```rust
println!(
    "{},{},{},{},{},{},{},{},{},{},{},{},{}",
    csv_escape(file_name),
    csv_escape(rel_dir),
    row.line,
    csv_escape(&row.stmt_type),
    csv_escape(&row.name),
    csv_escape(&row.parent),
    csv_escape(&row.parameters),
    csv_escape(&row.return_type),
    csv_escape(&sql),
    csv_escape(&row_err),
    csv_escape(&row_warn),
    csv_escape(&row.branch_path),
    csv_escape(&row.branch_condition),
);
```

**Step 4: Update ALL ParseCsvRow construction sites**

Every `ParseCsvRow { ... }` in the codebase needs the two new fields. Add `branch_path: String::new(), branch_condition: String::new()` to every existing construction. There are ~30 construction sites — use the compiler to find them all:
```bash
cargo build 2>&1 | grep "missing field"
```

**Step 5: Modify `collect_pl_stmt_rows` to generate ReturnCursorSQL rows**

Add new parameters to the function:
```rust
fn collect_pl_stmt_rows(
    pl_stmt: &ogsql_parser::ast::plpgsql::PlStatement,
    parent_name: &str,
    fallback_line: usize,
    vars: &std::collections::HashMap<String, Option<String>>,
    assigns: &mut std::collections::HashMap<String, Vec<ConcatPart>>,
    rows: &mut Vec<ParseCsvRow>,
    // NEW:
    out_cursors: &std::collections::HashSet<String>,
    branch_path: &str,
    branch_condition: &str,
)
```

In the `PlStatement::Open` arm, when the cursor is in `out_cursors`:
```rust
PlStatement::Open(spanned) => {
    let open = &spanned.node;
    let cursor_name = format_pl_expr(&open_stmt.cursor); // keep existing var name
    let is_return_cursor = out_cursors.contains(&cursor_name.to_lowercase());

    match &open.kind {
        PlOpenKind::ForQuery { scroll: _, query, parsed_query } => {
            if is_return_cursor {
                // NEW: ReturnCursorSQL row (merges Open + SQL into one)
                if let Some(ref stmt) = parsed_query {
                    let formatter = ogsql_parser::SqlFormatter::new();
                    let formatted = formatter.format_statement(stmt);
                    let sql = replace_pl_vars_in_sql(&formatted, vars);
                    rows.push(ParseCsvRow {
                        line,
                        stmt_type: "ReturnCursorSQL".into(),
                        name: cursor_name,
                        parent: parent_name.to_string(),
                        parameters: String::new(),
                        return_type: "REFCURSOR".into(),
                        sql,
                        branch_path: branch_path.to_string(),
                        branch_condition: branch_condition.to_string(),
                    });
                } else {
                    let (_, sql) = resolve_for_query_text(query, vars, assigns);
                    rows.push(ParseCsvRow {
                        line,
                        stmt_type: "ReturnCursorSQL".into(),
                        name: cursor_name,
                        parent: parent_name.to_string(),
                        parameters: String::new(),
                        return_type: "REFCURSOR".into(),
                        sql,
                        branch_path: branch_path.to_string(),
                        branch_condition: branch_condition.to_string(),
                    });
                }
            } else {
                // EXISTING: keep the old Open/ForQuery + Embedded/Select rows unchanged
                // ... (copy existing code)
            }
        }
        PlOpenKind::ForExecute { query, using_args } => {
            if is_return_cursor {
                let dynamic_sql = build_dynamic_sql_from_expr(query, vars, assigns);
                let using_suffix = format_using_args_exprs(using_args);
                let full_sql = if using_suffix.is_empty() {
                    dynamic_sql
                } else {
                    format!("{}\nUSING {}", dynamic_sql, using_suffix)
                };
                rows.push(ParseCsvRow {
                    line,
                    stmt_type: "ReturnCursorSQL".into(),
                    name: cursor_name,
                    parent: parent_name.to_string(),
                    parameters: String::new(),
                    return_type: "REFCURSOR".into(),
                    sql: full_sql,
                    branch_path: branch_path.to_string(),
                    branch_condition: branch_condition.to_string(),
                });
            } else {
                // EXISTING: keep old behavior
            }
        }
        // Simple, ForUsing — keep existing, no change
        _ => { /* existing code */ }
    }
}
```

**Step 6: Thread branch context through recursive calls**

Update all recursive calls to `collect_pl_stmt_rows` within `collect_pl_stmt_rows` to pass branch context. For the `If` arm:
```rust
PlStatement::If(spanned) => {
    let line = spanned_line(&spanned.span).max(fallback_line);
    let if_stmt = &spanned.node;
    let cond_str = format_pl_expr(&if_stmt.condition);
    for s in &if_stmt.then_stmts {
        collect_pl_stmt_rows(s, parent_name, line, vars, assigns, rows, out_cursors,
            &join_branch(branch_path, "IF.then"), &cond_str);
    }
    for (i, elsif) in if_stmt.elsifs.iter().enumerate() {
        let elsif_cond = format_pl_expr(&elsif.condition);
        for s in &elsif.stmts {
            collect_pl_stmt_rows(s, parent_name, line, vars, assigns, rows, out_cursors,
                &join_branch(branch_path, &format!("IF.elsif#{}.then", i + 1)), &elsif_cond);
        }
    }
    for s in &if_stmt.else_stmts {
        collect_pl_stmt_rows(s, parent_name, line, vars, assigns, rows, out_cursors,
            &join_branch(branch_path, "IF.else"), &cond_str);
    }
}
```

Similarly for `Loop`, `While`, `For`, `Case`, `Block`.

**Step 7: Thread out_cursors from flatten_statement**

In `flatten_statement()`, before calling `collect_block_sql_rows`, compute out_cursors:
```rust
Statement::CreateProcedure(s) => {
    let proc_name = s.name.join(".");
    let out_cursors: std::collections::HashSet<String> = s.parameters.iter()
        .filter(|p| {
            let is_out = p.mode.as_deref() == Some("OUT")
                || p.mode.as_deref() == Some("INOUT")
                || p.mode.as_deref() == Some("IN OUT");
            let is_cursor = p.data_type.to_uppercase().contains("REFCURSOR");
            is_out && is_cursor
        })
        .map(|p| p.name.to_lowercase())
        .collect();
    rows.push(ParseCsvRow { /* existing */ });
    if let Some(ref block) = s.block {
        rows.extend(collect_block_sql_rows(block, &proc_name, si.start_line, &vars, &out_cursors));
    }
}
```

Do similarly for `CreateFunction`, `CreatePackageBody` (iterate items), `CreatePackage` (iterate items), `Do`, `AnonyBlock`.

**Step 8: Update `collect_block_sql_rows` signature**

```rust
fn collect_block_sql_rows(
    block: &ogsql_parser::ast::plpgsql::PlBlock,
    parent_name: &str,
    fallback_line: usize,
    vars: &std::collections::HashMap<String, Option<String>>,
    out_cursors: &std::collections::HashSet<String>,
) -> Vec<ParseCsvRow> {
    // ... pass out_cursors to collect_pl_stmt_rows
}
```

**Step 9: Helper function**

```rust
fn join_branch(parent: &str, segment: &str) -> String {
    if parent.is_empty() { segment.to_string() } else { format!("{}.{}", parent, segment) }
}
```

**Step 10: Build and test**

Run: `cargo build 2>&1 | head -50`
Fix all missing field errors. Then:
```bash
echo 'CREATE OR REPLACE PROCEDURE test_proc(p_cur OUT REFCURSOR) AS BEGIN OPEN p_cur FOR SELECT 1; END;' | cargo run --features cli -- parse --csv
```
Expected output includes a `ReturnCursorSQL` row.

**Step 11: Commit**

```bash
git add src/bin/ogsql.rs
git commit -m "feat: ReturnCursorSQL CSV row type with branch path tracking"
```

---

## Task 4: JSON Output — In-Place and Summary Injection

**Files:**
- Modify: `src/bin/ogsql.rs`

**Step 1: Add JSON injection in cmd_parse**

Find the JSON output section in `cmd_parse` (around line 301). After the existing `dynamic_sql_analysis` injection, add:

```rust
// After existing dynamic_sql_analysis injection block:
// Inject return cursor analysis for routines
if has_routine_return_cursors(&si.statement) {
    if let Some(analysis) = compute_routine_analysis(&si.statement) {
        obj.as_object_mut().unwrap().insert(
            "routine_analysis".to_string(),
            serde_json::json!(analysis),
        );
    }
}
```

**Step 2: Implement helper functions**

```rust
fn has_routine_return_cursors(stmt: &ogsql_parser::Statement) -> bool {
    use ogsql_parser::Statement;
    match stmt {
        Statement::CreateProcedure(p) => {
            ogsql_parser::has_return_cursors(&p.parameters, None)
        }
        Statement::CreateFunction(f) => {
            ogsql_parser::has_return_cursors(&f.parameters, f.return_type.as_deref())
        }
        Statement::CreatePackageBody(pkg) => {
            pkg.items.iter().any(|item| match item {
                ogsql_parser::ast::PackageItem::Procedure(p) =>
                    ogsql_parser::has_return_cursors(&p.parameters, None),
                ogsql_parser::ast::PackageItem::Function(f) =>
                    ogsql_parser::has_return_cursors(&f.parameters, f.return_type.as_deref()),
                _ => false,
            })
        }
        _ => false,
    }
}

fn compute_routine_analysis(stmt: &ogsql_parser::Statement) -> Option<serde_json::Value> {
    use ogsql_parser::Statement;
    match stmt {
        Statement::CreateProcedure(p) => {
            let block = p.block.as_ref()?;
            let analysis = ogsql_parser::analyze_return_cursors(
                block, &p.parameters, &p.name.join("."), "Procedure", None,
            );
            Some(serde_json::json!(analysis))
        }
        Statement::CreateFunction(f) => {
            let block = f.block.as_ref()?;
            let analysis = ogsql_parser::analyze_return_cursors(
                block, &f.parameters, &f.name.join("."), "Function",
                f.return_type.as_deref(),
            );
            Some(serde_json::json!(analysis))
        }
        Statement::CreatePackageBody(pkg) => {
            let mut analyses = Vec::new();
            for item in &pkg.items {
                match item {
                    ogsql_parser::ast::PackageItem::Procedure(p) => {
                        if let Some(ref block) = p.block {
                            let analysis = ogsql_parser::analyze_return_cursors(
                                block, &p.parameters, &p.name.join("."),
                                "Procedure", None,
                            );
                            if !analysis.return_cursors.is_empty() {
                                analyses.push(analysis);
                            }
                        }
                    }
                    ogsql_parser::ast::PackageItem::Function(f) => {
                        if let Some(ref block) = f.block {
                            let analysis = ogsql_parser::analyze_return_cursors(
                                block, &f.parameters, &f.name.join("."),
                                "Function", f.return_type.as_deref(),
                            );
                            if !analysis.return_cursors.is_empty() {
                                analyses.push(analysis);
                            }
                        }
                    }
                    _ => {}
                }
            }
            if analyses.is_empty() { None } else { Some(serde_json::json!(analyses)) }
        }
        _ => None,
    }
}
```

**Step 3: Test JSON output**

```bash
echo 'CREATE OR REPLACE PROCEDURE test_proc(p_cur OUT REFCURSOR) AS BEGIN OPEN p_cur FOR SELECT id, name FROM t_users; END;' | cargo run --features cli -- parse -j
```

Expected: JSON output includes `routine_analysis` field with return_cursors array.

```bash
echo 'CREATE OR REPLACE PACKAGE BODY PKG AS PROCEDURE p(p_cur OUT REFCURSOR) AS BEGIN IF f = 1 THEN OPEN p_cur FOR SELECT 1; ELSE OPEN p_cur FOR SELECT 2; END IF; END; END PKG;' | cargo run --features cli -- parse -j
```

Expected: `routine_analysis` with 1 group, 2 branches.

**Step 4: Commit**

```bash
git add src/bin/ogsql.rs
git commit -m "feat: inject routine_analysis into JSON output for cursor return annotations"
```

---

## Task 5: Handle RETURN QUERY and RETURN QUERY EXECUTE

**Files:**
- Modify: `src/analyzer/return_cursor.rs`
- Modify: `src/bin/ogsql.rs`

**Step 1: Add RETURN QUERY handling in analyzer**

In `collect_annotations_for_return()`, enhance the `PlStatement::ReturnQuery` arm:

```rust
PlStatement::ReturnQuery(spanned) => {
    let rq = &spanned.node;
    if rq.is_dynamic {
        // RETURN QUERY EXECUTE expr [USING args]
        let sql = match &rq.dynamic_expr {
            Some(expr) => format_expr(expr),
            None => rq.query.clone(),
        };
        annotations.push(ReturnCursorAnnotation {
            out_param: "<return>".to_string(),
            out_position: 0,
            out_type: "SYS_REFCURSOR".to_string(),
            branch_path: branch_ctx.path.clone(),
            branch_condition: branch_ctx.condition.clone(),
            jdbc_type: "REF_CURSOR".to_string(),
            sql,
            sql_source: "dynamic".to_string(),
            parsed_query: None,
            result_columns: Vec::new(),
        });
    } else {
        // RETURN QUERY SELECT ...
        let result_columns = extract_result_columns(rq.parsed_query.as_ref().map(|b| b.as_ref()));
        annotations.push(ReturnCursorAnnotation {
            out_param: "<return>".to_string(),
            out_position: 0,
            out_type: "SYS_REFCURSOR".to_string(),
            branch_path: branch_ctx.path.clone(),
            branch_condition: branch_ctx.condition.clone(),
            jdbc_type: "REF_CURSOR".to_string(),
            sql: rq.query.clone(),
            sql_source: "static".to_string(),
            parsed_query: None,
            result_columns,
        });
    }
}
```

**Step 2: Add RETURN QUERY handling in CSV**

In `collect_pl_stmt_rows()`, add a new arm:
```rust
PlStatement::ReturnQuery(spanned) => {
    let line = spanned_line(&spanned.span).max(fallback_line);
    let rq = &spanned.node;
    // Check if this is in a function that returns REFCURSOR
    if !out_cursors.is_empty() || /* has function return refcursor flag */ false {
        // ReturnCursorSQL row
        let sql = replace_pl_vars_in_sql(&rq.query, vars);
        rows.push(ParseCsvRow {
            line,
            stmt_type: "ReturnCursorSQL".into(),
            name: "<return>".into(),
            parent: parent_name.to_string(),
            parameters: String::new(),
            return_type: "REFCURSOR".into(),
            sql,
            branch_path: branch_path.to_string(),
            branch_condition: branch_condition.to_string(),
        });
    } else {
        // Normal RETURN QUERY — just show as query row
        rows.push(ParseCsvRow {
            line,
            stmt_type: "ReturnQuery".into(),
            name: String::new(),
            parent: parent_name.to_string(),
            parameters: String::new(),
            return_type: String::new(),
            sql: replace_pl_vars_in_sql(&rq.query, vars),
            branch_path: String::new(),
            branch_condition: String::new(),
        });
    }
}
```

Note: The "has function return refcursor flag" needs to be threaded through. The simplest approach: for functions, add `"<return>"` to the `out_cursors` set when `return_type` contains REFCURSOR.

**Step 3: Commit**

```bash
git add src/analyzer/return_cursor.rs src/bin/ogsql.rs
git commit -m "feat: handle RETURN QUERY and RETURN QUERY EXECUTE as return cursor SQL"
```

---

## Task 6: Integration Test — Full Package Body

**Files:**
- Create test SQL file and verify end-to-end

**Step 1: Create integration test file**

`/tmp/test_return_cursor.sql`:
```sql
CREATE OR REPLACE PACKAGE BODY PKG_USER AS

  PROCEDURE get_users(p_status VARCHAR, p_result OUT REFCURSOR) AS
  BEGIN
    OPEN p_result FOR SELECT id, name, email, status FROM t_users WHERE status = p_status;
  END;

  PROCEDURE search_users(p_type INTEGER, p_cur OUT REFCURSOR) AS
  BEGIN
    IF p_type = 1 THEN
      OPEN p_cur FOR SELECT id, name FROM t_users WHERE status = 'active';
    ELSIF p_type = 2 THEN
      OPEN p_cur FOR SELECT id, name, dept FROM t_users WHERE dept IS NOT NULL;
    ELSE
      OPEN p_cur FOR SELECT id, name FROM t_users_backup;
    END IF;
  END;

  PROCEDURE dynamic_query(p_table VARCHAR, p_cur OUT REFCURSOR) AS
    v_sql VARCHAR;
  BEGIN
    v_sql := 'SELECT * FROM ' || p_table || ' WHERE 1=1';
    OPEN p_cur FOR EXECUTE v_sql;
  END;

  FUNCTION get_user_by_id(p_id INT) RETURN SYS_REFCURSOR AS
    v_cur SYS_REFCURSOR;
  BEGIN
    OPEN v_cur FOR SELECT id, name, email FROM t_users WHERE id = p_id;
    RETURN v_cur;
  END;

END PKG_USER;
/
```

**Step 2: Test CSV output**

```bash
cargo run --features cli -- -f /tmp/test_return_cursor.sql parse --csv
```

Expected:
- Procedure `get_users`: 1 ReturnCursorSQL row, no branch
- Procedure `search_users`: 3 ReturnCursorSQL rows with different branch paths
- Procedure `dynamic_query`: 1 ReturnCursorSQL row with dynamic SQL
- Function `get_user_by_id`: 1 ReturnCursorSQL row with `<return>` param

**Step 3: Test JSON output**

```bash
cargo run --features cli -- -f /tmp/test_return_cursor.sql parse -j | python3 -m json.tool
```

Expected: Each routine has `routine_analysis` with `return_cursors` array containing proper annotations.

**Step 4: Test round-trip (json2sql)**

```bash
cargo run --features cli -- -f /tmp/test_return_cursor.sql parse -j | cargo run --features cli -- json2sql
```

Expected: The `routine_analysis` field is ignored by json2sql (it's an extra field, not part of the core AST), and the SQL is reconstructed correctly.

**Step 5: Commit**

```bash
git commit --allow-empty -m "test: integration test for return cursor annotation"
```

---

## Task 7: MCP Server Integration

**Files:**
- Modify: `src/mcp/mod.rs`

**Step 1: Add return cursor analysis to MCP parse tool**

Find the `parse` tool handler. After the existing JSON output construction, add:

```rust
// Inject routine analysis for statements with return cursors
for value in &mut stmt_values {
    if let Some(stmt_value) = value.get_mut("statement") {
        // Check if this is a routine with return cursors
        // ... (same pattern as Task 4)
    }
}
```

Or simpler: construct the output the same way as the CLI, reusing `compute_routine_analysis`.

**Step 2: Test MCP**

```bash
echo '{"sql":"CREATE PROCEDURE p(cur OUT REFCURSOR) AS BEGIN OPEN cur FOR SELECT 1; END;"}' | cargo run --features mcp -- parse
```

**Step 3: Commit**

```bash
git add src/mcp/mod.rs
git commit -m "feat: return cursor annotation in MCP parse output"
```

---

## Summary

| Task | Description | Key Files |
|------|-------------|-----------|
| 1 | Data structures + core analysis | `src/analyzer/return_cursor.rs` (new) |
| 2 | Unit tests (8 tests) | `src/analyzer/return_cursor.rs` |
| 3 | CSV ReturnCursorSQL row type | `src/bin/ogsql.rs` |
| 4 | JSON routine_analysis injection | `src/bin/ogsql.rs` |
| 5 | RETURN QUERY support | `return_cursor.rs` + `ogsql.rs` |
| 6 | Integration test (full package) | CLI testing |
| 7 | MCP server integration | `src/mcp/mod.rs` |

**Risk areas:**
- `SqlFormatter::format_expr()` — may not exist; need to check method name
- `SelectTarget` enum variants — need to verify exact names in AST
- `PlStatement` variants — ReturnQuery/Return may not be handled in CSV yet (falls through to `_ => {}`)
- Thread safety — all new state is local to function calls, no global state
