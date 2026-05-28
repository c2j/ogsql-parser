pub mod return_cursor;
pub mod schema;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::ast::plpgsql::{PlBlock, PlDeclaration, PlOpenKind, PlStatement};
use crate::ast::{Expr, Literal, SourceSpan, Statement, StatementInfo};

// ── 报告类型 ──

/// Result of analyzing dynamic SQL usage in a PL/pgSQL block.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DynamicSqlReport {
    pub execute_findings: Vec<ExecuteFinding>,
    pub variable_traces: Vec<VariableTrace>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ref_cursor_queries: Vec<RefCursorQuery>,
}

/// A single `EXECUTE IMMEDIATE` finding with resolved SQL and trace chain.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecuteFinding {
    pub statement_path: Vec<usize>,
    pub expression_desc: String,
    pub resolved_value: Option<String>,
    pub parsed_statement: Option<Box<Statement>>,
    pub trace: TraceChain,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameterized_sql: Option<String>,
    pub parameter_bindings: Vec<ParameterBinding>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub optional_filters: Vec<OptionalFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_template: Option<DynamicTemplate>,
}

/// Maps a dynamic parameter to its source variable and position in parameterized SQL.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParameterBinding {
    pub position: usize,
    pub variable: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapping: Option<String>,
}

/// Records a variable's assigned value and the PL statement path where it was set.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VariableTrace {
    pub variable_name: String,
    pub assignment_path: Vec<usize>,
    pub value: String,
}

/// Provenance chain tracing how a dynamic SQL string was constructed.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TraceChain {
    LiteralAssignment {
        value: String,
    },
    VariableCopy {
        source_var: String,
        source_chain: Box<TraceChain>,
    },
    Concatenation {
        parts: Vec<TraceChain>,
    },
    DeclarationDefault {
        value: String,
    },
    Unknown,
}

// ── Dynamic SQL Template Decomposition ──

/// Decomposed template of a dynamic SQL string with static parts and dynamic parameters.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// A single dynamic parameter extracted from a SQL template.
pub struct DynamicTemplate {
    /// Static SQL fragments interleaved with dynamic params
    /// Template reconstruction: static_parts[0] + dynamic_params[0] + static_parts[1] + ...
    pub static_parts: Vec<String>,
    pub dynamic_params: Vec<DynamicParam>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<TemplateCondition>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

/// /// A single dynamic parameter extracted from a SQL template.
pub struct DynamicParam {
    /// Source variable name or expression
    pub source: String,
    /// Parameter name (for MyBatis #{param})
    pub param_name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

/// /// An IF condition guarding a dynamic parameter in a template.
pub struct TemplateCondition {
/// An IF condition guarding a dynamic parameter in a template.
    /// The variable checked in the IF condition
    pub param: String,
    /// The operator: "IS NOT NULL", "IS NULL", "= value", etc.
    pub operator: String,
}

// ── 参数化 SQL 生成 ──

struct ParameterizedResult {
    sql: String,
    bindings: Vec<ParameterBinding>,
}

fn parameterize_trace(trace: &TraceChain) -> ParameterizedResult {
    let mut bindings = Vec::new();
    let mut sql = build_parameterized_sql(trace, &mut bindings);
    detect_wrapping_in_sql(&mut sql, &mut bindings);
    ParameterizedResult { sql, bindings }
}

fn detect_wrapping_in_sql(sql: &mut String, bindings: &mut Vec<ParameterBinding>) {
    for binding in bindings.iter_mut() {
        let placeholder = format!(" :{}", binding.variable);
        let has_quote_before = sql.contains(&format!("'{}", placeholder));
        let has_quote_after = sql.contains(&format!("{}'", placeholder));
        if has_quote_before && has_quote_after {
            binding.wrapping = Some("'...'".to_string());
        }
    }
    let mut result = sql.clone();
    for binding in bindings.iter() {
        if binding.wrapping.is_some() {
            let placeholder = format!(" :{}", binding.variable);
            result = result.replace(&format!("'{}", placeholder), &placeholder);
            result = result.replace(&format!("{}'", placeholder), &placeholder);
        }
    }
    *sql = result;
}

fn build_parameterized_sql(trace: &TraceChain, bindings: &mut Vec<ParameterBinding>) -> String {
    match trace {
        TraceChain::LiteralAssignment { value }
        | TraceChain::DeclarationDefault { value } => value.clone(),

        TraceChain::VariableCopy { source_chain, .. } => {
            build_parameterized_sql(source_chain, bindings)
        }

        TraceChain::Unknown => " :?".to_string(),

        TraceChain::Concatenation { parts } => parts
            .iter()
            .map(|p| build_concat_part(p, bindings))
            .collect(),
    }
}

fn build_concat_part(trace: &TraceChain, bindings: &mut Vec<ParameterBinding>) -> String {
    match trace {
        TraceChain::LiteralAssignment { value }
        | TraceChain::DeclarationDefault { value } => value.clone(),

        TraceChain::VariableCopy {
            source_var,
            source_chain,
        } => match source_chain.as_ref() {
            TraceChain::Concatenation { .. } => {
                build_parameterized_sql(source_chain, bindings)
            }
            TraceChain::VariableCopy { .. } => {
                build_parameterized_sql(source_chain, bindings)
            }
            _ => {
                let pos = bindings.len() + 1;
                bindings.push(ParameterBinding {
                    position: pos,
                    variable: source_var.clone(),
                    wrapping: None,
                });
                format!(" :{}", source_var)
            }
        },

        TraceChain::Unknown => " :?".to_string(),

        TraceChain::Concatenation { parts } => parts
            .iter()
            .map(|p| build_concat_part(p, bindings))
            .collect(),
    }
}

// ── Optional filter detection ──

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

/// /// A conditional WHERE filter that becomes active when a parameter is non-NULL.
pub struct OptionalFilter {
    pub parameter: String,
    pub column: Vec<String>,
    pub operator: String,
}

fn extract_where_clause(stmt: &Statement) -> Option<&Expr> {
    match stmt {
        Statement::Select(sel) => sel.node.where_clause.as_ref(),
        Statement::Update(update) => update.node.where_clause.as_ref(),
        Statement::Delete(delete) => delete.node.where_clause.as_ref(),
        _ => None,
    }
}

fn extract_var_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::PlVariable(names) | Expr::ColumnRef(names) if names.len() == 1 => {
            Some(names[0].clone())
        }
        _ => None,
    }
}

fn strip_parens(expr: &Expr) -> &Expr {
    match expr {
        Expr::Parenthesized(inner) => strip_parens(inner),
        _ => expr,
    }
}

fn detect_optional_filters(expr: &Expr) -> Vec<OptionalFilter> {
    let mut filters = Vec::new();
    collect_optional_filters(expr, &mut filters);
    filters
}

fn collect_optional_filters(expr: &Expr, filters: &mut Vec<OptionalFilter>) {
    match expr {
        Expr::BinaryOp { op, left, right } if op == "OR" => {
            if let Some(f) = try_match_optional_filter(left, right) {
                filters.push(f);
                return;
            }
            if let Some(f) = try_match_optional_filter(right, left) {
                filters.push(f);
                return;
            }
        }
        Expr::Parenthesized(inner) => {
            collect_optional_filters(inner, filters);
            return;
        }
        Expr::BinaryOp { op, left, right } if op == "AND" => {
            collect_optional_filters(left, filters);
            collect_optional_filters(right, filters);
            return;
        }
        _ => {}
    }
}

fn try_match_optional_filter(is_null_side: &Expr, comparison_side: &Expr) -> Option<OptionalFilter> {
    let is_null = strip_parens(is_null_side);
    let comparison = strip_parens(comparison_side);

    if let Expr::IsNull { expr: param_expr, negated: false } = is_null {
        let param_name = extract_var_name(param_expr)?;

        if let Expr::Like { expr, pattern, negated: false, .. } = comparison {
            if extract_var_name(pattern).as_ref() == Some(&param_name) {
                let column = match expr.as_ref() {
                    Expr::ColumnRef(names) => names.clone(),
                    _ => return None,
                };
                return Some(OptionalFilter {
                    parameter: param_name,
                    column,
                    operator: "LIKE".to_string(),
                });
            }
        }

        if let Expr::BinaryOp { op, left, right } = comparison {
            if op == "=" || op == ">" || op == "<" || op == ">=" || op == "<=" || op == "<>" {
                let (col_expr, param_expr2) = if extract_var_name(right).as_ref() == Some(&param_name) {
                    (left, right)
                } else if extract_var_name(left).as_ref() == Some(&param_name) {
                    (right, left)
                } else {
                    return None;
                };

                if extract_var_name(param_expr2)? != param_name {
                    return None;
                }

                let column = match col_expr.as_ref() {
                    Expr::ColumnRef(names) => names.clone(),
                    _ => return None,
                };

                return Some(OptionalFilter {
                    parameter: param_name,
                    column,
                    operator: op.clone(),
                });
            }
        }
    }
    None
}

// ── REF CURSOR query detection ──

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

/// /// A REF CURSOR out parameter and its associated query.
pub struct RefCursorQuery {
    pub out_param_name: String,
    pub query: Option<String>,
    pub parsed_query: Option<Box<Statement>>,
}

fn extract_cursor_name(expr: &Expr) -> Option<String> {
    match expr {
        Expr::PlVariable(names) | Expr::ColumnRef(names) if names.len() == 1 => {
            Some(names[0].clone())
        }
        _ => None,
    }
}

fn collect_ref_cursor_queries(
    stmts: &[PlStatement],
    ref_cursor_params: &std::collections::HashSet<String>,
) -> Vec<RefCursorQuery> {
    let mut queries = Vec::new();
    for stmt in stmts {
        if let PlStatement::Open(open) = stmt {
            if let Some(cursor_name) = extract_cursor_name(&open.node.cursor) {
                if ref_cursor_params.contains(&cursor_name) {
                    match &open.node.kind {
                        PlOpenKind::ForQuery { query, parsed_query, .. } => {
                            queries.push(RefCursorQuery {
                                out_param_name: cursor_name,
                                query: Some(query.clone()),
                                parsed_query: parsed_query.clone(),
                            });
                        }
                        PlOpenKind::ForExecute { query: q, .. } => {
                            let query_str = match q {
                                Expr::PlVariable(n) | Expr::ColumnRef(n) => Some(n.join(".")),
                                Expr::Literal(Literal::String(s)) => Some(s.clone()),
                                _ => None,
                            };
                            queries.push(RefCursorQuery {
                                out_param_name: cursor_name,
                                query: query_str,
                                parsed_query: None,
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
        match stmt {
            PlStatement::Block(b) => {
                queries.extend(collect_ref_cursor_queries(&b.body, ref_cursor_params));
            }
            PlStatement::If(i) => {
                queries.extend(collect_ref_cursor_queries(&i.then_stmts, ref_cursor_params));
                for elsif in &i.elsifs {
                    queries.extend(collect_ref_cursor_queries(&elsif.stmts, ref_cursor_params));
                }
                queries.extend(collect_ref_cursor_queries(&i.else_stmts, ref_cursor_params));
            }
            PlStatement::Loop(l) => {
                queries.extend(collect_ref_cursor_queries(&l.body, ref_cursor_params));
            }
            PlStatement::While(w) => {
                queries.extend(collect_ref_cursor_queries(&w.body, ref_cursor_params));
            }
            PlStatement::For(f) => {
                queries.extend(collect_ref_cursor_queries(&f.body, ref_cursor_params));
            }
            _ => {}
        }
    }
    queries
}


/// /// Find all REF CURSOR out parameters and their associated queries in a PL block.
pub fn find_ref_cursor_queries(
    block: &PlBlock,
    params: &[(String, String, Option<String>)],
) -> Vec<RefCursorQuery> {
    let ref_cursor_params: std::collections::HashSet<String> = params
        .iter()
        .filter(|(_, data_type, mode)| {
            data_type.to_uppercase().contains("REFCURSOR") && mode.as_deref() == Some("OUT")
        })
        .map(|(name, _, _)| name.clone())
        .collect();

    if ref_cursor_params.is_empty() {
        return Vec::new();
    }

    collect_ref_cursor_queries(&block.body, &ref_cursor_params)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

/// /// A normalized SQL fingerprint with all occurrences in the input.
pub struct QueryFingerprint {
    pub fingerprint: String,
    pub occurrences: Vec<FingerprintOccurrence>,
    pub normalized_sql: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

/// /// Location where a specific fingerprint was found.
pub struct FingerprintOccurrence {
    pub location: String,
}

fn fingerprint_statement(stmt: &Statement) -> Option<String> {
    let formatter = crate::formatter::SqlFormatter::new();
    let normalized_sql = formatter.format_statement(stmt);
    let mut hasher = DefaultHasher::new();
    normalized_sql.hash(&mut hasher);
    Some(format!("fp_{:016x}", hasher.finish()))
}


/// /// Compute structural fingerprints for all SQL statements, grouping identical queries.
pub fn compute_query_fingerprints(stmts: &[Statement]) -> Vec<QueryFingerprint> {
    let mut fingerprint_map: HashMap<String, QueryFingerprint> = HashMap::new();

    for (i, stmt) in stmts.iter().enumerate() {
        collect_fingerprints_recursive(stmt, &format!("statement_{}", i), &mut fingerprint_map);
    }

    let mut results: Vec<_> = fingerprint_map.into_values().collect();
    results.sort_by(|a, b| a.fingerprint.cmp(&b.fingerprint));
    results
}

fn collect_fingerprints_recursive(
    stmt: &Statement,
    location: &str,
    map: &mut HashMap<String, QueryFingerprint>,
) {
    if let Some(fp) = fingerprint_statement(stmt) {
        let formatter = crate::formatter::SqlFormatter::new();
        let normalized_sql = formatter.format_statement(stmt);
        let entry = map.entry(fp.clone()).or_insert_with(|| QueryFingerprint {
            fingerprint: fp,
            occurrences: Vec::new(),
            normalized_sql,
        });
        entry.occurrences.push(FingerprintOccurrence {
            location: location.to_string(),
        });
    }
}

fn extract_template(trace: &TraceChain) -> Option<DynamicTemplate> {
    match trace {
        TraceChain::Concatenation { parts } => {
            build_template_from_parts(parts)
        }
        TraceChain::VariableCopy { source_chain, .. } => {
            extract_template(source_chain)
        }
        _ => None,
    }
}

/// Collect static text from a trace chain (recursively flattens literals).
/// Used when a nested Concatenation has no dynamic params — its static text
/// must still be appended to the parent template.
fn collect_static_text(trace: &TraceChain) -> String {
    match trace {
        TraceChain::LiteralAssignment { value }
        | TraceChain::DeclarationDefault { value } => value.clone(),
        TraceChain::VariableCopy { source_chain, .. } => collect_static_text(source_chain),
        TraceChain::Concatenation { parts } => {
            parts.iter().map(|p| collect_static_text(p)).collect()
        }
        TraceChain::Unknown => String::new(),
    }
}

/// Merge a sub-template's static parts and dynamic params into the parent
/// template being built. Handles both cases: sub-template present (has dynamic
/// params) or absent (all-static — collect text only).
fn merge_sub_template(
    sub_parts: &[TraceChain],
    static_parts: &mut Vec<String>,
    dynamic_params: &mut Vec<DynamicParam>,
) {
    if let Some(sub_template) = build_template_from_parts(sub_parts) {
        if let Some(last) = static_parts.last_mut() {
            last.push_str(sub_template.static_parts.first().unwrap_or(&String::new()));
        }
        for (i, param) in sub_template.dynamic_params.iter().enumerate() {
            dynamic_params.push(param.clone());
            if i + 1 < sub_template.static_parts.len() {
                static_parts.push(sub_template.static_parts[i + 1].clone());
            } else {
                static_parts.push(String::new());
            }
        }
    } else {
        // All-static concatenation: collect literal text and append
        if let Some(last) = static_parts.last_mut() {
            let text: String = sub_parts.iter().map(|p| collect_static_text(p)).collect();
            last.push_str(&text);
        }
    }
}

fn build_template_from_parts(parts: &[TraceChain]) -> Option<DynamicTemplate> {
    let mut static_parts = Vec::new();
    let mut dynamic_params = Vec::new();

    static_parts.push(String::new());

    for part in parts {
        match part {
            TraceChain::LiteralAssignment { value }
            | TraceChain::DeclarationDefault { value } => {
                if let Some(last) = static_parts.last_mut() {
                    last.push_str(value);
                }
            }
            TraceChain::VariableCopy { source_var, source_chain } => {
                match source_chain.as_ref() {
                    TraceChain::LiteralAssignment { value }
                    | TraceChain::DeclarationDefault { value } => {
                        if let Some(last) = static_parts.last_mut() {
                            last.push_str(value);
                        }
                    }
                    TraceChain::Concatenation { parts: sub_parts } => {
                        merge_sub_template(sub_parts, &mut static_parts, &mut dynamic_params);
                    }
                    _ => {
                        dynamic_params.push(DynamicParam {
                            source: source_var.clone(),
                            param_name: source_var.clone(),
                        });
                        static_parts.push(String::new());
                    }
                }
            }
            TraceChain::Concatenation { parts: sub_parts } => {
                merge_sub_template(sub_parts, &mut static_parts, &mut dynamic_params);
            }
            TraceChain::Unknown => {
                dynamic_params.push(DynamicParam {
                    source: "?".to_string(),
                    param_name: "?".to_string(),
                });
                static_parts.push(String::new());
            }
        }
    }

    if dynamic_params.is_empty() {
        return None;
    }

    for part in &mut static_parts {
        *part = part.trim().to_string();
    }

    Some(DynamicTemplate {
        static_parts,
        dynamic_params,
        conditions: Vec::new(),
    })
}

// ── 内部状态 ──

struct VarState {
    known_value: Option<String>,
    trace: TraceChain,
}


/// /// Tracks variable assignments and traces dynamic SQL construction in PL/pgSQL.
pub struct DynamicSqlAnalyzer {
    scopes: Vec<HashMap<String, VarState>>,
    findings: Vec<ExecuteFinding>,
    traces: Vec<VariableTrace>,
    path: Vec<usize>,
}

impl DynamicSqlAnalyzer {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            findings: Vec::new(),
            traces: Vec::new(),
            path: Vec::new(),
        }
    }

    pub fn analyze(mut self, block: &PlBlock) -> DynamicSqlReport {
        self.process_declarations(&block.declarations);
        self.process_statements(&block.body);
        DynamicSqlReport {
            execute_findings: self.findings,
            variable_traces: self.traces,
            ref_cursor_queries: Vec::new(),
        }
    }

    fn lookup_var(&self, name: &str) -> Option<&VarState> {
        for scope in self.scopes.iter().rev() {
            if let Some(state) = scope.get(name) {
                return Some(state);
            }
        }
        None
    }

    fn set_var(&mut self, name: &str, state: VarState) {
        // 写入当前作用域（最顶层的）
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), state);
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn process_declarations(&mut self, declarations: &[PlDeclaration]) {
        for decl in declarations {
            match decl {
                PlDeclaration::Variable(var_decl) => {
                    if let Some(expr) = &var_decl.default {
                        let state = self.evaluate_expr(expr);
                        if let Some(ref value) = state.known_value {
                            self.traces.push(VariableTrace {
                                variable_name: var_decl.name.clone(),
                                assignment_path: self.path.clone(),
                                value: value.clone(),
                            });
                        }
                        self.set_var(&var_decl.name, state);
                    } else {
                        // Register the variable even without a default value
                        // so it's known in scope (important for PlVariable resolution)
                        self.set_var(&var_decl.name, VarState {
                            known_value: None,
                            trace: TraceChain::Unknown,
                        });
                    }
                }
                PlDeclaration::Cursor(cursor_decl) => {
                    // Register cursor name in scope
                    self.set_var(&cursor_decl.name, VarState {
                        known_value: None,
                        trace: TraceChain::Unknown,
                    });
                }
                PlDeclaration::Record(record_decl) => {
                    // Register record name in scope
                    self.set_var(&record_decl.name, VarState {
                        known_value: None,
                        trace: TraceChain::Unknown,
                    });
                }
                _ => {} // Type, NestedProcedure, NestedFunction, Pragma — not variables
            }
        }
    }

    fn process_statements(&mut self, stmts: &[PlStatement]) {
        for (i, stmt) in stmts.iter().enumerate() {
            self.path.push(i);
            self.process_statement(stmt);
            self.path.pop();
        }
    }

    fn process_statement(&mut self, stmt: &PlStatement) {
        match stmt {
            PlStatement::Assignment { target, expression } => {
                let target_name = match target {
                    Expr::PlVariable(n) | Expr::ColumnRef(n) => {
                        n.last().cloned().unwrap_or_default()
                    }
                    _ => String::new(),
                };
                let state = self.evaluate_expr(expression);
                if let Some(ref value) = state.known_value {
                    self.traces.push(VariableTrace {
                        variable_name: target_name.clone(),
                        assignment_path: self.path.clone(),
                        value: value.clone(),
                    });
                }
                self.set_var(&target_name, state);
            }

            PlStatement::Execute(exec) => {
                let (resolved, trace) = self.resolve_expr(&exec.string_expr);
                let parsed = resolved
                    .as_ref()
                    .and_then(|s| crate::parser::Parser::parse_statement_from_str(s));
                let desc = self.expr_to_string(&exec.string_expr);
                let param_result = parameterize_trace(&trace);
                let dynamic_template = extract_template(&trace);

                let optional_filters = parsed
                    .as_ref()
                    .and_then(|stmt| extract_where_clause(stmt))
                    .map(|where_clause| detect_optional_filters(where_clause))
                    .unwrap_or_default();

                self.findings.push(ExecuteFinding {
                    statement_path: self.path.clone(),
                    expression_desc: desc,
                    resolved_value: resolved,
                    parsed_statement: parsed,
                    trace,
                    parameterized_sql: if param_result.sql.trim().is_empty() {
                        None
                    } else {
                        Some(param_result.sql)
                    },
                    parameter_bindings: param_result.bindings,
                    optional_filters,
                    dynamic_template,
                });
            }

            PlStatement::Block(block) => {
                self.enter_scope();
                self.process_declarations(&block.declarations);
                self.process_statements(&block.body);
                self.exit_scope();
            }

            PlStatement::If(if_stmt) => {
                self.process_statements(&if_stmt.then_stmts);
                for elsif in &if_stmt.elsifs {
                    self.process_statements(&elsif.stmts);
                }
                self.process_statements(&if_stmt.else_stmts);
            }

            PlStatement::Case(case_stmt) => {
                for when in &case_stmt.whens {
                    self.process_statements(&when.stmts);
                }
                self.process_statements(&case_stmt.else_stmts);
            }

            PlStatement::Loop(loop_stmt) => {
                self.process_statements(&loop_stmt.body);
            }

            PlStatement::While(while_stmt) => {
                self.process_statements(&while_stmt.body);
            }

            PlStatement::For(for_stmt) => {
                self.enter_scope();
                self.set_var(&for_stmt.variable, VarState {
                    known_value: None,
                    trace: TraceChain::Unknown,
                });
                self.process_statements(&for_stmt.body);
                self.exit_scope();
            }

            PlStatement::ForEach(foreach_stmt) => {
                self.enter_scope();
                self.set_var(&foreach_stmt.variable, VarState {
                    known_value: None,
                    trace: TraceChain::Unknown,
                });
                self.process_statements(&foreach_stmt.body);
                self.exit_scope();
            }

            _ => {}
        }
    }

    fn evaluate_expr(&self, expr: &Expr) -> VarState {
        match expr {
            Expr::Literal(Literal::String(s)) => VarState {
                known_value: Some(s.clone()),
                trace: TraceChain::LiteralAssignment { value: s.clone() },
            },
            Expr::Literal(Literal::DollarString { body, .. }) => VarState {
                known_value: Some(body.clone()),
                trace: TraceChain::LiteralAssignment {
                    value: body.clone(),
                },
            },
            Expr::Literal(Literal::EscapeString(s)) => VarState {
                known_value: Some(s.clone()),
                trace: TraceChain::LiteralAssignment { value: s.clone() },
            },
            Expr::ColumnRef(names) if names.len() == 1 => {
                let var_name = &names[0];
                if let Some(state) = self.lookup_var(var_name) {
                    VarState {
                        known_value: state.known_value.clone(),
                        trace: TraceChain::VariableCopy {
                            source_var: var_name.clone(),
                            source_chain: Box::new(state.trace.clone()),
                        },
                    }
                } else {
                    VarState {
                        known_value: None,
                        trace: TraceChain::Unknown,
                    }
                }
            }
            Expr::PlVariable(names) if names.len() == 1 => {
                let var_name = &names[0];
                if let Some(state) = self.lookup_var(var_name) {
                    VarState {
                        known_value: state.known_value.clone(),
                        trace: TraceChain::VariableCopy {
                            source_var: var_name.clone(),
                            source_chain: Box::new(state.trace.clone()),
                        },
                    }
                } else {
                    VarState {
                        known_value: None,
                        trace: TraceChain::Unknown,
                    }
                }
            }
            Expr::BinaryOp { op, left, right } if op == "||" => {
                let left_state = self.evaluate_expr(left);
                let right_state = self.evaluate_expr(right);
                let known_value = match (&left_state.known_value, &right_state.known_value) {
                    (Some(l), Some(r)) => Some(format!("{}{}", l, r)),
                    _ => None,
                };
                VarState {
                    known_value,
                    trace: TraceChain::Concatenation {
                        parts: vec![left_state.trace, right_state.trace],
                    },
                }
            }
            _ => VarState {
                known_value: None,
                trace: TraceChain::Unknown,
            },
        }
    }

    fn resolve_expr(&self, expr: &Expr) -> (Option<String>, TraceChain) {
        let state = self.evaluate_expr(expr);
        (state.known_value, state.trace)
    }

    fn expr_to_string(&self, expr: &Expr) -> String {
        match expr {
            Expr::ColumnRef(names) => names.join("."),
            Expr::PlVariable(names) => names.join("."),
            Expr::Literal(Literal::String(s)) => format!("'{}'", s),
            Expr::BinaryOp {
                op, left, right, ..
            } => format!(
                "{} {} {}",
                self.expr_to_string(left),
                op,
                self.expr_to_string(right)
            ),
            _ => format!("{:?}", expr),
        }
    }
}

// ── 公共入口函数 ──


/// /// Analyze a PL block for dynamic SQL patterns, variable tracing, and ref cursor queries.
pub fn analyze_pl_block(block: &PlBlock) -> DynamicSqlReport {
    DynamicSqlAnalyzer::new().analyze(block)
}

// ── Transaction Analysis ──

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

/// /// Result of analyzing transaction control flow in a PL/pgSQL block.
pub struct TransactionReport {
    pub has_explicit_commit: bool,
    pub has_explicit_rollback: bool,
    pub has_autonomous_transaction: bool,
    pub transaction_segments: Vec<TransactionSegment>,
    pub cross_procedure_calls: Vec<CrossProcedureCall>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

/// /// A contiguous range of statements between two transaction boundaries.
pub struct TransactionSegment {
    pub index: usize,
    pub start_reason: TransactionBoundary,
    pub end_reason: TransactionBoundary,
    pub statement_range: (usize, usize),
    pub sub_transactions: Vec<SubTransaction>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

/// /// Reason why a transaction segment starts or ends.
pub enum TransactionBoundary {
    ProcedureEntry,
    PostCommit,
    PostRollback,
    Commit,
    Rollback,
    ProcedureExit,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

/// /// A nested block that forms an implicit sub-transaction.
pub struct SubTransaction {
    pub block_path: Vec<usize>,
    pub implicit_savepoint: bool,
    pub body_range: (usize, usize),
    pub exception_handlers: Vec<ExceptionHandlerInfo>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

/// /// Information about an exception handler within a sub-transaction.
pub struct ExceptionHandlerInfo {
    pub conditions: Vec<String>,
    pub statement_count: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

/// /// A call to another procedure/function that may affect transaction state.
pub struct CrossProcedureCall {
    pub call_path: Vec<usize>,
    pub callee: String,
    pub callee_may_commit: bool,
}


/// /// Analyze transaction boundaries, segments, and cross-procedure calls in a PL block.
pub fn analyze_transactions(block: &PlBlock) -> TransactionReport {
    let mut analyzer = TransactionAnalyzer::new();
    analyzer.analyze(block)
}

struct TransactionAnalyzer {
    has_explicit_commit: bool,
    has_explicit_rollback: bool,
    has_autonomous_transaction: bool,
    segments: Vec<TransactionSegment>,
    cross_procedure_calls: Vec<CrossProcedureCall>,
    current_segment_start: usize,
    current_segment_start_reason: TransactionBoundary,
    current_segment_stmts: Vec<(usize, SubTransactionInfo)>,
    global_idx: usize,
}

struct SubTransactionInfo {
    block_path: Vec<usize>,
    implicit_savepoint: bool,
    body_range: (usize, usize),
    handlers: Vec<ExceptionHandlerInfo>,
}

impl TransactionAnalyzer {
    fn new() -> Self {
        Self {
            has_explicit_commit: false,
            has_explicit_rollback: false,
            has_autonomous_transaction: false,
            segments: Vec::new(),
            cross_procedure_calls: Vec::new(),
            current_segment_start: 0,
            current_segment_start_reason: TransactionBoundary::ProcedureEntry,
            current_segment_stmts: Vec::new(),
            global_idx: 0,
        }
    }

    fn analyze(&mut self, block: &PlBlock) -> TransactionReport {
        for decl in &block.declarations {
            if let PlDeclaration::Pragma { name, .. } = decl {
                if name.eq_ignore_ascii_case("AUTONOMOUS_TRANSACTION") {
                    self.has_autonomous_transaction = true;
                }
            }
        }
        self.scan_statements(&block.body, &[]);
        self.flush_segment(TransactionBoundary::ProcedureExit);
        TransactionReport {
            has_explicit_commit: self.has_explicit_commit,
            has_explicit_rollback: self.has_explicit_rollback,
            has_autonomous_transaction: self.has_autonomous_transaction,
            transaction_segments: std::mem::take(&mut self.segments),
            cross_procedure_calls: std::mem::take(&mut self.cross_procedure_calls),
        }
    }

    fn scan_statements(&mut self, stmts: &[PlStatement], path: &[usize]) {
        for (i, stmt) in stmts.iter().enumerate() {
            let mut sub_tx = None;
            self.scan_statement(stmt, &path.iter().copied().chain(std::iter::once(i)).collect::<Vec<_>>(), &mut sub_tx);
            self.current_segment_stmts.push((self.global_idx, sub_tx.unwrap_or(SubTransactionInfo {
                block_path: path.iter().copied().chain(std::iter::once(i)).collect(),
                implicit_savepoint: false,
                body_range: (0, 0),
                handlers: Vec::new(),
            })));
            self.global_idx += 1;
        }
    }

    fn scan_statement(&mut self, stmt: &PlStatement, path: &[usize], sub_tx: &mut Option<SubTransactionInfo>) {
        match stmt {
            PlStatement::Commit { .. } => {
                self.has_explicit_commit = true;
                let end = if self.global_idx > 0 { self.global_idx } else { 0 };
                let sub_transactions = self.drain_sub_transactions();
                self.segments.push(TransactionSegment {
                    index: self.segments.len(),
                    start_reason: std::mem::replace(&mut self.current_segment_start_reason, TransactionBoundary::PostCommit),
                    end_reason: TransactionBoundary::Commit,
                    statement_range: (self.current_segment_start, end),
                    sub_transactions,
                });
                self.current_segment_start = self.global_idx + 1;
            }
            PlStatement::Rollback { .. } => {
                self.has_explicit_rollback = true;
                let end = if self.global_idx > 0 { self.global_idx } else { 0 };
                let sub_transactions = self.drain_sub_transactions();
                self.segments.push(TransactionSegment {
                    index: self.segments.len(),
                    start_reason: std::mem::replace(&mut self.current_segment_start_reason, TransactionBoundary::PostRollback),
                    end_reason: TransactionBoundary::Rollback,
                    statement_range: (self.current_segment_start, end),
                    sub_transactions,
                });
                self.current_segment_start = self.global_idx + 1;
            }
            PlStatement::ProcedureCall(call) => {
                let callee = call.name.join(".");
                self.cross_procedure_calls.push(CrossProcedureCall {
                    call_path: path.to_vec(),
                    callee: callee.clone(),
                    callee_may_commit: false,
                });
            }
            PlStatement::Block(inner_block) => {
                if inner_block.exception_block.is_some() {
                    let handler_count = inner_block.exception_block.as_ref().map_or(0, |eb| eb.handlers.len());
                    let handlers: Vec<ExceptionHandlerInfo> = inner_block
                        .exception_block
                        .as_ref()
                        .map(|eb| {
                            eb.handlers
                                .iter()
                                .map(|h| ExceptionHandlerInfo {
                                    conditions: h.conditions.clone(),
                                    statement_count: h.statements.len(),
                                })
                                .collect()
                        })
                        .unwrap_or_default();
                    *sub_tx = Some(SubTransactionInfo {
                        block_path: path.to_vec(),
                        implicit_savepoint: true,
                        body_range: (0, inner_block.body.len().saturating_sub(1)),
                        handlers,
                    });
                }
                self.scan_statements(&inner_block.body, path);
            }
            PlStatement::If(if_stmt) => {
                self.scan_statements(&if_stmt.then_stmts, path);
                for elsif in &if_stmt.elsifs {
                    self.scan_statements(&elsif.stmts, path);
                }
                self.scan_statements(&if_stmt.else_stmts, path);
            }
            PlStatement::Case(case_stmt) => {
                for when in &case_stmt.whens {
                    self.scan_statements(&when.stmts, path);
                }
                self.scan_statements(&case_stmt.else_stmts, path);
            }
            PlStatement::Loop(loop_stmt) => {
                self.scan_statements(&loop_stmt.body, path);
            }
            PlStatement::While(while_stmt) => {
                self.scan_statements(&while_stmt.body, path);
            }
            PlStatement::For(for_stmt) => {
                self.scan_statements(&for_stmt.body, path);
            }
            PlStatement::ForEach(foreach_stmt) => {
                self.scan_statements(&foreach_stmt.body, path);
            }
            _ => {}
        }
    }

    fn drain_sub_transactions(&mut self) -> Vec<SubTransaction> {
        self.current_segment_stmts
            .drain(..)
            .filter_map(|(_, info)| {
                if info.implicit_savepoint {
                    Some(SubTransaction {
                        block_path: info.block_path,
                        implicit_savepoint: true,
                        body_range: info.body_range,
                        exception_handlers: info.handlers,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn flush_segment(&mut self, end_reason: TransactionBoundary) {
        if !self.current_segment_stmts.is_empty() || !self.segments.is_empty() {
            let end = self.global_idx.saturating_sub(1);
            let sub_transactions = self.drain_sub_transactions();
            self.segments.push(TransactionSegment {
                index: self.segments.len(),
                start_reason: std::mem::replace(&mut self.current_segment_start_reason, TransactionBoundary::ProcedureEntry),
                end_reason,
                statement_range: (self.current_segment_start, end),
                sub_transactions,
            });
        }
    }
}

// ── Package Spec vs Body Consistency Validation ──

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

/// /// A mismatch between a package spec and its body.
pub struct PackageConsistencyError {
    pub package_name: String,
    pub subprogram_name: String,
    pub kind: PackageConsistencyErrorKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]

/// /// Category of package spec vs body inconsistency.
pub enum PackageConsistencyErrorKind {
    /// A subprogram exists in the spec but is missing from the body
    MissingInBody,
    /// A subprogram exists in the body but is not declared in the spec
    ExtraInBody,
    /// Parameter count differs between spec and body
    ParamCountMismatch { spec_count: usize, body_count: usize },
    /// A parameter has a default value in the spec but not in the body (or vice versa)
    DefaultMismatch { param_name: String, spec_default: Option<String>, body_default: Option<String> },
    /// A parameter has a different data type in spec vs body
    TypeMismatch { param_name: String, spec_type: String, body_type: String },
}

/// Validate that all package specs and bodies in the parsed statements are consistent.
/// Returns a list of errors/warnings for each mismatch found.
pub fn validate_package_consistency(
    stmts: &[crate::ast::StatementInfo],
) -> Vec<PackageConsistencyError> {
    use crate::ast::{Statement, CreatePackageStatement, CreatePackageBodyStatement};

    let mut specs: std::collections::HashMap<String, &CreatePackageStatement> =
        std::collections::HashMap::new();
    let mut bodies: std::collections::HashMap<String, &CreatePackageBodyStatement> =
        std::collections::HashMap::new();

    for si in stmts {
        match &si.statement {
            Statement::CreatePackage(spec) => {
                let name = object_name_str(&spec.name);
                specs.insert(name, spec);
            }
            Statement::CreatePackageBody(body) => {
                let name = object_name_str(&body.name);
                bodies.insert(name, body);
            }
            _ => {}
        }
    }

    let mut errors = Vec::new();

    for (pkg_name, spec) in &specs {
        if let Some(body) = bodies.get(pkg_name) {
            validate_single_package(pkg_name, spec, body, &mut errors);
        }
    }

    errors
}

fn object_name_str(name: &crate::ast::ObjectName) -> String {
    name.iter()
        .map(|s| s.to_uppercase())
        .collect::<Vec<_>>()
        .join(".")
}

fn validate_single_package(
    pkg_name: &str,
    spec: &crate::ast::CreatePackageStatement,
    body: &crate::ast::CreatePackageBodyStatement,
    errors: &mut Vec<PackageConsistencyError>,
) {
    use crate::ast::PackageItem;

    let spec_funcs: std::collections::HashMap<String, &crate::ast::PackageFunction> = spec.items
        .iter()
        .filter_map(|item| match item {
            PackageItem::Function(f) => {
                let name = object_name_str(&f.name);
                Some((name, f))
            }
            _ => None,
        })
        .collect();

    let spec_procs: std::collections::HashMap<String, &crate::ast::PackageProcedure> = spec.items
        .iter()
        .filter_map(|item| match item {
            PackageItem::Procedure(p) => {
                let name = object_name_str(&p.name);
                Some((name, p))
            }
            _ => None,
        })
        .collect();

    let body_funcs: std::collections::HashMap<String, &crate::ast::PackageFunction> = body.items
        .iter()
        .filter_map(|item| match item {
            PackageItem::Function(f) => {
                let name = object_name_str(&f.name);
                Some((name, f))
            }
            _ => None,
        })
        .collect();

    let body_procs: std::collections::HashMap<String, &crate::ast::PackageProcedure> = body.items
        .iter()
        .filter_map(|item| match item {
            PackageItem::Procedure(p) => {
                let name = object_name_str(&p.name);
                Some((name, p))
            }
            _ => None,
        })
        .collect();

    for (func_name, spec_func) in &spec_funcs {
        if let Some(body_func) = body_funcs.get(func_name) {
            compare_params(
                pkg_name,
                func_name,
                &spec_func.parameters,
                &body_func.parameters,
                errors,
            );
        } else {
            errors.push(PackageConsistencyError {
                package_name: pkg_name.to_string(),
                subprogram_name: func_name.clone(),
                kind: PackageConsistencyErrorKind::MissingInBody,
                detail: Some("FUNCTION declared in package spec but not defined in package body".to_string()),
            });
        }
    }

    for (proc_name, spec_proc) in &spec_procs {
        if let Some(body_proc) = body_procs.get(proc_name) {
            compare_params(
                pkg_name,
                proc_name,
                &spec_proc.parameters,
                &body_proc.parameters,
                errors,
            );
        } else {
            errors.push(PackageConsistencyError {
                package_name: pkg_name.to_string(),
                subprogram_name: proc_name.clone(),
                kind: PackageConsistencyErrorKind::MissingInBody,
                detail: Some("PROCEDURE declared in package spec but not defined in package body".to_string()),
            });
        }
    }

    for func_name in body_funcs.keys() {
        if !spec_funcs.contains_key(func_name) {
            errors.push(PackageConsistencyError {
                package_name: pkg_name.to_string(),
                subprogram_name: func_name.clone(),
                kind: PackageConsistencyErrorKind::ExtraInBody,
                detail: Some("FUNCTION defined in package body but not declared in package spec".to_string()),
            });
        }
    }

    for proc_name in body_procs.keys() {
        if !spec_procs.contains_key(proc_name) {
            errors.push(PackageConsistencyError {
                package_name: pkg_name.to_string(),
                subprogram_name: proc_name.clone(),
                kind: PackageConsistencyErrorKind::ExtraInBody,
                detail: Some("PROCEDURE defined in package body but not declared in package spec".to_string()),
            });
        }
    }
}

fn compare_params(
    pkg_name: &str,
    subprogram_name: &str,
    spec_params: &[crate::ast::RoutineParam],
    body_params: &[crate::ast::RoutineParam],
    errors: &mut Vec<PackageConsistencyError>,
) {
    if spec_params.len() != body_params.len() {
        errors.push(PackageConsistencyError {
            package_name: pkg_name.to_string(),
            subprogram_name: subprogram_name.to_string(),
            kind: PackageConsistencyErrorKind::ParamCountMismatch {
                spec_count: spec_params.len(),
                body_count: body_params.len(),
            },
            detail: Some(format!(
                "parameter count mismatch: spec has {}, body has {}",
                spec_params.len(), body_params.len()
            )),
        });
    }

    let max_len = spec_params.len().max(body_params.len());
    for i in 0..max_len {
        let spec_p = match spec_params.get(i) {
            Some(p) => p,
            None => continue,
        };
        let body_p = match body_params.get(i) {
            Some(p) => p,
            None => continue,
        };

        match (&spec_p.default_value, &body_p.default_value) {
            (Some(_), None) => {
                errors.push(PackageConsistencyError {
                    package_name: pkg_name.to_string(),
                    subprogram_name: subprogram_name.to_string(),
                    kind: PackageConsistencyErrorKind::DefaultMismatch {
                        param_name: spec_p.name.clone(),
                        spec_default: spec_p.default_value.clone(),
                        body_default: None,
                    },
                    detail: Some(format!(
                        "Parameter '{}' has DEFAULT value '{}' in spec but no DEFAULT in body",
                        spec_p.name,
                        spec_p.default_value.as_deref().unwrap_or("")
                    )),
                });
            }
            (None, Some(_)) => {
                errors.push(PackageConsistencyError {
                    package_name: pkg_name.to_string(),
                    subprogram_name: subprogram_name.to_string(),
                    kind: PackageConsistencyErrorKind::DefaultMismatch {
                        param_name: body_p.name.clone(),
                        spec_default: None,
                        body_default: body_p.default_value.clone(),
                    },
                    detail: Some(format!(
                        "Parameter '{}' has no DEFAULT in spec but has DEFAULT '{}' in body",
                        body_p.name,
                        body_p.default_value.as_deref().unwrap_or("")
                    )),
                });
            }
            (Some(spec_def), Some(body_def)) => {
                if !default_values_equivalent(spec_def, body_def) {
                    errors.push(PackageConsistencyError {
                        package_name: pkg_name.to_string(),
                        subprogram_name: subprogram_name.to_string(),
                        kind: PackageConsistencyErrorKind::DefaultMismatch {
                            param_name: spec_p.name.clone(),
                            spec_default: Some(spec_def.clone()),
                            body_default: Some(body_def.clone()),
                        },
                        detail: Some(format!(
                            "Parameter '{}' DEFAULT value differs: spec has '{}', body has '{}'",
                            spec_p.name, spec_def, body_def
                        )),
                    });
                }
            }
            (None, None) => {}
        }

        let spec_type = spec_p.data_type.to_uppercase().replace(' ', "");
        let body_type = body_p.data_type.to_uppercase().replace(' ', "");
        if spec_type != body_type {
            errors.push(PackageConsistencyError {
                package_name: pkg_name.to_string(),
                subprogram_name: subprogram_name.to_string(),
                kind: PackageConsistencyErrorKind::TypeMismatch {
                    param_name: spec_p.name.clone(),
                    spec_type: spec_p.data_type.clone(),
                    body_type: body_p.data_type.clone(),
                },
                detail: Some(format!(
                    "parameter '{}' type mismatch: spec has '{}', body has '{}'",
                    spec_p.name, spec_p.data_type, body_p.data_type
                )),
            });
        }
    }
}

/// Compare default values for semantic equivalence.
/// Handles common cases like "NULL" vs "null", extra whitespace, etc.
fn default_values_equivalent(a: &str, b: &str) -> bool {
    let normalize = |s: &str| s.trim().to_uppercase();
    normalize(a) == normalize(b)
}

// ── PL Undefined Variable Validation ──

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UndefinedRefKind {
    Variable,
    Function,
}

/// A warning about a potentially undefined variable reference in PL/pgSQL code.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UndefinedVariableError {
    /// The unresolved variable name.
    pub variable_name: String,
    /// Source location of the reference, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<crate::ast::SourceSpan>,
    /// Description of the PL context where the reference was found
    /// (e.g., "EXECUTE IMMEDIATE", "assignment", "IF condition").
    pub context: String,
    #[serde(default = "default_ref_kind")]
    pub kind: UndefinedRefKind,
}

fn default_ref_kind() -> UndefinedRefKind {
    UndefinedRefKind::Variable
}

/// SQL built-in values that are valid references in PL expressions.
/// These appear as `ColumnRef` in the AST but are NOT undefined variables.
const PL_BUILTIN_VALUES: &[&str] = &[
    "SYSDATE",
    "SYSTIMESTAMP",
    "CURRENT_DATE",
    "CURRENT_TIME",
    "CURRENT_TIMESTAMP",
    "LOCALTIME",
    "LOCALTIMESTAMP",
    "USER",
    "CURRENT_USER",
    "SESSION_USER",
    "UID",
    "SESSIONTIMEZONE",
    "DBTIMEZONE",
    "ROWID",
    "ROWNUM",
    "LEVEL",
    "NEXTVAL",
    "CURRVAL",
    "FOUND",
    "NOT_FOUND",
    "ROW_COUNT",
    "SQLCODE",
    "SQLERRM",
    "SQLSTATE",
    "YEAR", "MONTH", "DAY", "HOUR", "MINUTE", "SECOND",
    "EPOCH", "DOW", "DOY", "ISODOW", "ISOYEAR", "QUARTER", "WEEK",
    "CENTURY", "DECADE", "MILLISECOND", "MILLISECONDS", "MICROSECOND", "MICROSECONDS",
    "TIMEZONE", "TIMEZONE_HOUR", "TIMEZONE_MINUTE",
    "BOTH", "LEADING", "TRAILING",
];

fn is_pl_builtin(name: &str) -> bool {
    PL_BUILTIN_VALUES.iter().any(|&b| b.eq_ignore_ascii_case(name))
}

fn is_known_function(name: &str) -> bool {
    crate::parser::function_registry::lookup_function(name).is_some()
}

/// Validate that all variable references in a PL block resolve to declared variables.
///
/// This performs a semantic analysis pass over the PL block, checking that every
/// `ColumnRef` in PL-level expressions (not embedded SQL) refers to either a
/// declared variable/parameter or a known SQL built-in value.
///
/// # Arguments
/// * `block` - The PL block to validate
/// * `params` - Procedure/function parameter names (added to scope)
///
/// # Returns
/// A list of warnings for potentially undefined variable references.
pub fn validate_pl_variables(
    block: &PlBlock,
    params: &[crate::ast::RoutineParam],
) -> Vec<UndefinedVariableError> {
    validate_pl_variables_with_extra_vars_and_funcs(block, params, &[], &[], false)
}


/// /// Like validate_pl_variables but with additional pre-declared variable names.
pub fn validate_pl_variables_with_extra_vars(
    block: &PlBlock,
    params: &[crate::ast::RoutineParam],
    extra_vars: &[&str],
) -> Vec<UndefinedVariableError> {
    validate_pl_variables_with_extra_vars_and_funcs(block, params, extra_vars, &[], false)
}


/// /// Like validate_pl_variables but with extra variable names and known function names.
pub fn validate_pl_variables_with_extra_vars_and_funcs(
    block: &PlBlock,
    params: &[crate::ast::RoutineParam],
    extra_vars: &[&str],
    extra_funcs: &[&str],
    strict: bool,
) -> Vec<UndefinedVariableError> {
    let mut validator = PlVariableValidator::new(strict);
    for p in params {
        validator.declare(&p.name);
    }
    for v in extra_vars {
        validator.declare(v);
    }
    for f in extra_funcs {
        validator.declare_func(f);
    }
    validator.process_block(block);
    validator.errors
}

struct PlVariableValidator {
    scope_stack: Vec<std::collections::HashSet<String>>,
    known_funcs: std::collections::HashSet<String>,
    errors: Vec<UndefinedVariableError>,
    current_span: Option<crate::ast::SourceSpan>,
    strict: bool,
}

impl PlVariableValidator {
    fn new(strict: bool) -> Self {
        Self {
            scope_stack: vec![std::collections::HashSet::new()],
            known_funcs: std::collections::HashSet::new(),
            errors: Vec::new(),
            current_span: None,
            strict,
        }
    }

    fn declare(&mut self, name: &str) {
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.insert(name.to_lowercase());
        }
    }

    fn declare_func(&mut self, name: &str) {
        self.known_funcs.insert(name.to_lowercase());
    }

    fn is_known_func(&self, name: &str) -> bool {
        self.known_funcs.contains(&name.to_lowercase())
    }

    fn is_declared(&self, name: &str) -> bool {
        let lower = name.to_lowercase();
        self.scope_stack.iter().rev().any(|s| s.contains(&lower))
    }

    fn enter_scope(&mut self) {
        self.scope_stack.push(std::collections::HashSet::new());
    }

    fn exit_scope(&mut self) {
        self.scope_stack.pop();
    }

    fn process_block(&mut self, block: &PlBlock) {
        self.process_declarations(&block.declarations);
                self.process_statements(&block.body);
                if let Some(ref eb) = block.exception_block {
            for handler in &eb.handlers {
                self.process_statements(&handler.statements);
            }
        }
    }

    fn process_declarations(&mut self, declarations: &[PlDeclaration]) {
        use crate::ast::plpgsql::{PlDeclaration, PlTypeDecl};
        for decl in declarations {
            match decl {
                PlDeclaration::Variable(v) => self.declare(&v.name),
                PlDeclaration::Cursor(c) => self.declare(&c.name),
                PlDeclaration::Record(r) => self.declare(&r.name),
                PlDeclaration::Type(t) => {
                    let name = match t {
                        PlTypeDecl::Record { name, .. } => name,
                        PlTypeDecl::TableOf { name, .. } => name,
                        PlTypeDecl::VarrayOf { name, .. } => name,
                        PlTypeDecl::RefCursor { name } => name,
                    };
                    self.declare(name);
                }
                PlDeclaration::NestedProcedure(p) => self.declare(&p.name.join("_")),
                PlDeclaration::NestedFunction(f) => self.declare(&f.name.join("_")),
                PlDeclaration::Pragma { .. } => {}
            }
        }
    }

    fn process_statements(&mut self, stmts: &[PlStatement]) {
        for stmt in stmts {
            self.process_statement(stmt);
        }
    }

    fn process_statement(&mut self, stmt: &PlStatement) {
        let saved_span = self.current_span.take();
        match stmt {
            // ── PL expressions: CHECK for undefined variables ──

            PlStatement::Assignment { target, expression } => {
                // Assignment has no Spanned wrapper — keep parent span for location.
                self.current_span = saved_span.clone();
                self.check_expr(target, "assignment target");
                self.check_expr(expression, "assignment expression");
            }

            PlStatement::Execute(exec) => {
                self.current_span = exec.span.clone();
                self.check_expr(&exec.node.string_expr, "EXECUTE IMMEDIATE");
                for target in &exec.node.into_targets {
                    self.check_expr(target, "EXECUTE IMMEDIATE INTO");
                }
                for arg in &exec.node.using_args {
                    self.check_expr(&arg.argument, "EXECUTE IMMEDIATE USING");
                }
            }

            PlStatement::If(if_stmt) => {
                self.current_span = if_stmt.span.clone();
                self.check_expr(&if_stmt.node.condition, "IF condition");
                self.process_statements(&if_stmt.node.then_stmts);
                for elsif in &if_stmt.node.elsifs {
                    self.check_expr(&elsif.condition, "ELSIF condition");
                    self.process_statements(&elsif.stmts);
                }
                self.process_statements(&if_stmt.node.else_stmts);
            }

            PlStatement::Case(case_stmt) => {
                self.current_span = case_stmt.span.clone();
                if let Some(ref expr) = case_stmt.node.expression {
                    self.check_expr(expr, "CASE expression");
                }
                for when in &case_stmt.node.whens {
                    self.check_expr(&when.condition, "CASE WHEN condition");
                    self.process_statements(&when.stmts);
                }
                self.process_statements(&case_stmt.node.else_stmts);
            }

            PlStatement::While(while_stmt) => {
                self.current_span = while_stmt.span.clone();
                self.check_expr(&while_stmt.node.condition, "WHILE condition");
                self.process_statements(&while_stmt.node.body);
            }

            PlStatement::For(for_stmt) => {
                self.current_span = for_stmt.span.clone();
                self.enter_scope();
                self.declare(&for_stmt.node.variable);
                match &for_stmt.node.kind {
                    crate::ast::plpgsql::PlForKind::Range { low, high, step, .. } => {
                        self.check_expr(low, "FOR loop lower bound");
                        self.check_expr(high, "FOR loop upper bound");
                        if let Some(s) = step {
                            self.check_expr(s, "FOR loop step");
                        }
                    }
                    crate::ast::plpgsql::PlForKind::Query { using_args, .. } => {
                        for arg in using_args {
                            self.check_expr(&arg.argument, "FOR IN SELECT USING");
                        }
                    }
                    crate::ast::plpgsql::PlForKind::Cursor { cursor_name, arguments } => {
                        self.check_expr(cursor_name, "FOR IN cursor");
                        for arg in arguments {
                            self.check_expr(arg, "FOR IN cursor arguments");
                        }
                    }
                }
                self.process_statements(&for_stmt.node.body);
                self.exit_scope();
            }

            PlStatement::ForEach(foreach_stmt) => {
                self.current_span = foreach_stmt.span.clone();
                self.enter_scope();
                self.declare(&foreach_stmt.node.variable);
                self.check_expr(&foreach_stmt.node.expression, "FOREACH expression");
                self.process_statements(&foreach_stmt.node.body);
                self.exit_scope();
            }

            PlStatement::Loop(loop_stmt) => {
                self.current_span = loop_stmt.span.clone();
                self.process_statements(&loop_stmt.node.body);
            }

            PlStatement::Exit { condition, .. } => {
                if let Some(ref expr) = condition {
                    self.check_expr(expr, "EXIT WHEN condition");
                }
            }

            PlStatement::Continue { condition, .. } => {
                if let Some(ref expr) = condition {
                    self.check_expr(expr, "CONTINUE WHEN condition");
                }
            }

            PlStatement::Return { expression } => {
                if let Some(ref expr) = expression {
                    self.check_expr(expr, "RETURN expression");
                }
            }

            PlStatement::ReturnNext { expression } => {
                self.check_expr(expression, "RETURN NEXT expression");
            }

            PlStatement::ReturnQuery(rq) => {
                self.current_span = rq.span.clone();
                if let Some(ref expr) = rq.node.dynamic_expr {
                    self.check_expr(expr, "RETURN QUERY EXECUTE");
                }
                for arg in &rq.node.using_args {
                    self.check_expr(&arg.argument, "RETURN QUERY USING");
                }
            }

            PlStatement::Raise(raise) => {
                self.current_span = raise.span.clone();
                for param in &raise.node.params {
                    self.check_expr(param, "RAISE parameter");
                }
                for opt in &raise.node.options {
                    self.check_expr(&opt.value, "RAISE option");
                }
            }

            PlStatement::Open(open) => {
                self.current_span = open.span.clone();
                self.check_expr(&open.node.cursor, "OPEN cursor");
                match &open.node.kind {
                    PlOpenKind::Simple { arguments } => {
                        for arg in arguments {
                            self.check_expr(arg, "OPEN cursor arguments");
                        }
                    }
                    PlOpenKind::ForExecute { query, using_args } => {
                        self.check_expr(query, "OPEN FOR EXECUTE");
                        for arg in using_args {
                            self.check_expr(arg, "OPEN FOR EXECUTE USING");
                        }
                    }
                    PlOpenKind::ForUsing { expressions } => {
                        for expr in expressions {
                            self.check_expr(expr, "OPEN FOR USING");
                        }
                    }
                    PlOpenKind::ForQuery { .. } => {
                        // SQL query — skip column refs
                    }
                }
            }

            PlStatement::Fetch(fetch) => {
                self.current_span = fetch.span.clone();
                self.check_expr(&fetch.node.cursor, "FETCH cursor");
                for target in &fetch.node.into {
                    self.check_expr(target, "FETCH INTO target");
                }
            }

            PlStatement::Close { cursor } => {
                self.check_expr(cursor, "CLOSE cursor");
            }

            PlStatement::Move { cursor, .. } => {
                self.check_expr(cursor, "MOVE cursor");
            }

            PlStatement::GetDiagnostics(gd) => {
                self.current_span = gd.span.clone();
                for item in &gd.node.items {
                    self.check_expr(&item.target, "GET DIAGNOSTICS target");
                }
            }

            PlStatement::ProcedureCall(call) => {
                self.current_span = call.span.clone();
                for arg in &call.node.arguments {
                    self.check_expr_proc_arg(arg);
                }
            }

            PlStatement::PipeRow { expression } => {
                self.check_expr(expression, "PIPE ROW expression");
            }

            PlStatement::Block(inner_block) => {
                self.current_span = inner_block.span.clone();
                self.enter_scope();
                self.process_block(&inner_block.node);
                self.exit_scope();
            }

            // ── SQL contexts: SKIP ──
            PlStatement::SqlStatement { .. } => {}
            PlStatement::Perform { .. } => {}
            PlStatement::Sql(_) => {}

            // ── No PL expressions to check ──
            PlStatement::Null => {}
            PlStatement::Goto { .. } => {}
            PlStatement::Commit { .. } => {}
            PlStatement::Rollback { .. } => {}
            PlStatement::Savepoint { .. } => {}
            PlStatement::ReleaseSavepoint { .. } => {}
            PlStatement::SetTransaction { .. } => {}
            PlStatement::ForAll(_) => {} // bounds are strings, not expressions
            PlStatement::VariableSet(_) => {}
            PlStatement::VariableReset(_) => {}
        }
        self.current_span = saved_span;
    }

    fn check_expr(&mut self, expr: &Expr, context: &str) {
        match expr {
            Expr::ColumnRef(names) | Expr::PlVariable(names) if names.len() == 1 => {
                let name = &names[0];
                if !self.is_declared(name) && !is_pl_builtin(name) && !is_known_function(name) && !self.is_known_func(name) {
                    self.errors.push(UndefinedVariableError {
                        variable_name: name.clone(),
                        location: self.current_span.clone(),
                        context: context.to_string(),
                        kind: UndefinedRefKind::Variable,
                    });
                }
            }

            Expr::BinaryOp { left, right, .. } => {
                self.check_expr(left, context);
                self.check_expr(right, context);
            }
            Expr::UnaryOp { expr: inner, .. } => {
                self.check_expr(inner, context);
            }
            Expr::IsNull { expr: inner, .. } => {
                self.check_expr(inner, context);
            }
            Expr::IsBoolean { expr: inner, .. } => {
                self.check_expr(inner, context);
            }
            Expr::InList { expr: inner, list, .. } => {
                self.check_expr(inner, context);
                for item in list {
                    self.check_expr(item, context);
                }
            }
            Expr::InSubquery { expr: inner, .. } => {
                self.check_expr(inner, context);
            }
            Expr::Between { expr: inner, low, high, .. } => {
                self.check_expr(inner, context);
                self.check_expr(low, context);
                self.check_expr(high, context);
            }
            Expr::Like { expr: inner, pattern, escape, .. } => {
                self.check_expr(inner, context);
                self.check_expr(pattern, context);
                if let Some(ref e) = escape {
                    self.check_expr(e, context);
                }
            }
            Expr::FunctionCall { name, args, agg_from, builtin, .. } => {
                if self.strict {
                    if let Some(fname) = name.last() {
                        if builtin.is_none()
                            && !is_known_function(fname)
                            && !self.is_known_func(fname)
                            && !is_pl_builtin(fname)
                        {
                            self.errors.push(UndefinedVariableError {
                                variable_name: fname.clone(),
                                location: self.current_span.clone(),
                                context: context.to_string(),
                                kind: UndefinedRefKind::Function,
                            });
                        }
                    }
                }
                // Aggregate FROM clause (e.g., SUM(expr FROM generate_series(1, N) AS i))
                // introduces SQL-level aliases that are valid references in the aggregate args.
                if agg_from.is_some() {
                    self.enter_scope();
                    for item in agg_from.as_ref().unwrap() {
                        self.declare_table_ref_alias(item);
                        self.check_table_ref_exprs(item, context);
                    }
                }
                for arg in args {
                    self.check_expr(arg, context);
                }
                if agg_from.is_some() {
                    self.exit_scope();
                }
            }
            Expr::SpecialFunction { args, .. } => {
                for arg in args {
                    self.check_expr(arg, context);
                }
            }
            Expr::Case { operand, whens, else_expr } => {
                if let Some(ref op) = operand {
                    self.check_expr(op, context);
                }
                for when in whens {
                    self.check_expr(&when.condition, context);
                    self.check_expr(&when.result, context);
                }
                if let Some(ref el) = else_expr {
                    self.check_expr(el, context);
                }
            }
            Expr::TypeCast { expr: inner, .. } => {
                self.check_expr(inner, context);
            }
            Expr::Treat { expr: inner, .. } => {
                self.check_expr(inner, context);
            }
            Expr::CollationFor { expr: inner } => {
                self.check_expr(inner, context);
            }
            Expr::Parenthesized(inner) => {
                self.check_expr(inner, context);
            }
            Expr::Subscript { object, lower, upper, .. } => {
                self.check_expr(object, context);
                if let Some(l) = lower { self.check_expr(l, context); }
                if let Some(u) = upper { self.check_expr(u, context); }
            }
            Expr::FieldAccess { object, .. } => {
                self.check_expr(object, context);
            }
            Expr::Array(exprs) => {
                for e in exprs {
                    self.check_expr(e, context);
                }
            }
            Expr::RowConstructor(exprs) => {
                for e in exprs {
                    self.check_expr(e, context);
                }
            }
            Expr::Prior(inner) => {
                self.check_expr(inner, context);
            }
            Expr::ScalarSublink { expr: inner, .. } => {
                self.check_expr(inner, context);
            }
            Expr::CursorAttribute { cursor, .. } => {
                if let Expr::ColumnRef(names) | Expr::PlVariable(names) = cursor.as_ref() {
                    if names.len() == 1 && names[0].eq_ignore_ascii_case("SQL") {
                        return;
                    }
                }
                self.check_expr(cursor, context);
            }
            Expr::XmlElement { evalname, content, .. } => {
                if let Some(ref e) = evalname {
                    self.check_expr(e, context);
                }
                for c in content {
                    self.check_expr(&c.expr, context);
                }
            }
            Expr::XmlConcat(exprs) => {
                for e in exprs {
                    self.check_expr(e, context);
                }
            }
            Expr::XmlForest(items) => {
                for item in items {
                    self.check_expr(&item.expr, context);
                }
            }
            Expr::XmlParse { expr: inner, .. } => {
                self.check_expr(inner, context);
            }
            Expr::XmlPi { content, .. } => {
                if let Some(ref c) = content {
                    self.check_expr(c, context);
                }
            }
            Expr::XmlRoot { expr: inner, version, .. } => {
                self.check_expr(inner, context);
                if let Some(ref v) = version {
                    self.check_expr(v, context);
                }
            }
            Expr::XmlSerialize { expr: inner, .. } => {
                self.check_expr(inner, context);
            }
            Expr::PredictBy { features, .. } => {
                for f in features {
                    self.check_expr(f, context);
                }
            }
            Expr::SequenceValue { .. } => {}

            Expr::ColumnRef(_) | Expr::PlVariable(_) => {}
            Expr::Literal(_) => {}
            Expr::QualifiedStar(_) => {}
            Expr::Exists(_) => {}
            Expr::Subquery(_) => {}
            Expr::Parameter(_) => {}
            Expr::MyBatisParam(_) => {}
            Expr::MyBatisRawExpr(_) => {}
            Expr::Default => {}
            Expr::SysDate => {}
            Expr::CurrentOf { .. } => {}
        }
    }

    fn declare_table_ref_alias(&mut self, table_ref: &crate::ast::TableRef) {
        let alias = match table_ref {
            crate::ast::TableRef::Table { alias, .. } => alias.as_ref(),
            crate::ast::TableRef::FunctionCall { alias, .. } => alias.as_ref(),
            crate::ast::TableRef::Subquery { alias, .. } => alias.as_ref(),
            crate::ast::TableRef::Values { alias, .. } => alias.as_ref(),
            crate::ast::TableRef::Join { .. } => None,
            crate::ast::TableRef::Pivot { .. } => None,
            crate::ast::TableRef::Unpivot { .. } => None,
        };
        if let Some(a) = alias {
            self.declare(a);
        }
    }

    fn check_table_ref_exprs(&mut self, table_ref: &crate::ast::TableRef, context: &str) {
        match table_ref {
            crate::ast::TableRef::FunctionCall { args, .. } => {
                for arg in args {
                    self.check_expr(arg, context);
                }
            }
            crate::ast::TableRef::Table { tablesample, .. } => {
                if let Some(ref ts) = tablesample {
                    for arg in &ts.arguments {
                        self.check_expr(arg, context);
                    }
                    if let Some(ref r) = ts.repeatable {
                        self.check_expr(r, context);
                    }
                }
            }
            crate::ast::TableRef::Subquery { .. } | crate::ast::TableRef::Values { .. } => {}
            crate::ast::TableRef::Join { left, right, condition, .. } => {
                self.check_table_ref_exprs(left, context);
                self.check_table_ref_exprs(right, context);
                if let Some(ref c) = condition {
                    self.check_expr(c, context);
                }
            }
            crate::ast::TableRef::Pivot { source, pivot } => {
                self.check_table_ref_exprs(source, context);
                self.check_expr(&pivot.aggregate, context);
            }
            crate::ast::TableRef::Unpivot { source, .. } => {
                self.check_table_ref_exprs(source, context);
            }
        }
    }

    fn check_expr_proc_arg(&mut self, expr: &Expr) {
        match expr {
            Expr::ColumnRef(_) | Expr::PlVariable(_) => {
                // Procedure call arguments may reference SQL columns or external
                // parameterless functions — skip validation for single-component names.
            }
            Expr::BinaryOp { left, right, .. } => {
                self.check_expr_proc_arg(left);
                self.check_expr_proc_arg(right);
            }
            Expr::UnaryOp { expr: inner, .. } => {
                self.check_expr_proc_arg(inner);
            }
            Expr::Parenthesized(inner) => {
                self.check_expr_proc_arg(inner);
            }
            Expr::FunctionCall { args, .. } | Expr::SpecialFunction { args, .. } => {
                for arg in args {
                    self.check_expr_proc_arg(arg);
                }
            }
            Expr::TypeCast { expr: inner, .. } => {
                self.check_expr_proc_arg(inner);
            }
            Expr::Case { operand, whens, else_expr } => {
                if let Some(ref op) = operand {
                    self.check_expr_proc_arg(op);
                }
                for when in whens {
                    self.check_expr_proc_arg(&when.condition);
                    self.check_expr_proc_arg(&when.result);
                }
                if let Some(ref el) = else_expr {
                    self.check_expr_proc_arg(el);
                }
            }
            _ => {}
        }
    }
}

// ── MERGE Semantic Validation for GaussDB ──

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

/// /// A GaussDB semantic restriction violation in a MERGE statement.
pub struct MergeSemanticError {
    pub kind: MergeSemanticErrorKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default)]
    pub location: crate::token::SourceLocation,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]

/// /// Category of MERGE semantic restriction.
pub enum MergeSemanticErrorKind {
    /// GaussDB does not support WHEN MATCHED THEN DELETE
    DeleteNotSupported,
    /// Columns referenced in ON clause cannot be updated
    OnColumnUpdated,
    /// DUAL table does not exist in GaussDB
    DualTableNotSupported,
}

/// Validate MERGE statements against GaussDB semantic restrictions.
///
/// Returns a list of errors for each violation found across all MERGE statements
/// in the provided statement list (including MERGE inside PL/pgSQL blocks).
pub fn validate_merge_semantics(stmts: &[StatementInfo]) -> Vec<MergeSemanticError> {
    let mut errors = Vec::new();
    for si in stmts {
        let loc = crate::token::SourceLocation {
            line: si.start_line,
            column: si.start_col,
            offset: 0,
        };
        collect_merge_errors(&si.statement, &mut errors, loc);
    }
    errors
}

fn span_to_location(span: Option<&crate::ast::SourceSpan>) -> crate::token::SourceLocation {
    span.map(|s| s.start.clone())
        .unwrap_or_default()
}

fn collect_merge_errors(
    stmt: &Statement,
    errors: &mut Vec<MergeSemanticError>,
    fallback_loc: crate::token::SourceLocation,
) {
    match stmt {
        Statement::Merge(ref merge_stmt) => {
            let loc = if merge_stmt.span.is_some() {
                span_to_location(merge_stmt.span.as_ref())
            } else {
                fallback_loc
            };
            errors.extend(validate_single_merge(&merge_stmt.node, loc));
        }
        Statement::CreateProcedure(ref proc) => {
            if let Some(ref block) = proc.block {
                scan_pl_block(block, errors);
            }
        }
        Statement::CreateFunction(ref func) => {
            if let Some(ref block) = func.block {
                scan_pl_block(block, errors);
            }
        }
        Statement::Do(ref do_stmt) => {
            if let Some(ref block) = do_stmt.block {
                scan_pl_block(block, errors);
            }
        }
        Statement::AnonyBlock(ref ab) => {
            scan_pl_block(&ab.block, errors);
        }
        Statement::CreatePackageBody(ref body) => {
            for item in &body.items {
                match item {
                    crate::ast::PackageItem::Procedure(ref p) => {
                        if let Some(ref block) = p.block {
                            scan_pl_block(block, errors);
                        }
                    }
                    crate::ast::PackageItem::Function(ref f) => {
                        if let Some(ref block) = f.block {
                            scan_pl_block(block, errors);
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn scan_pl_block(block: &crate::ast::plpgsql::PlBlock, errors: &mut Vec<MergeSemanticError>) {
    for stmt in &block.body {
        scan_pl_stmt(stmt, errors);
    }
    if let Some(ref exc) = block.exception_block {
        for handler in &exc.handlers {
            for stmt in &handler.statements {
                scan_pl_stmt(stmt, errors);
            }
        }
    }
}

fn scan_pl_stmt(stmt: &crate::ast::plpgsql::PlStatement, errors: &mut Vec<MergeSemanticError>) {
    use crate::ast::plpgsql::PlStatement as Ps;
    match stmt {
        Ps::SqlStatement { statement, span, .. } => {
            let loc = if let Some(ref sp) = span {
                sp.start.clone()
            } else if let Statement::Merge(ref ms) = statement.as_ref() {
                span_to_location(ms.span.as_ref())
            } else {
                crate::token::SourceLocation::default()
            };
            collect_merge_errors(statement, errors, loc);
        }
        Ps::Block(ref inner) => {
            scan_pl_block(&inner.node, errors);
        }
        Ps::If(ref if_stmt) => {
            for s in &if_stmt.node.then_stmts {
                scan_pl_stmt(s, errors);
            }
            for elsif in &if_stmt.node.elsifs {
                for s in &elsif.stmts {
                    scan_pl_stmt(s, errors);
                }
            }
            for s in &if_stmt.node.else_stmts {
                scan_pl_stmt(s, errors);
            }
        }
        Ps::Case(ref case_stmt) => {
            for when in &case_stmt.node.whens {
                for s in &when.stmts {
                    scan_pl_stmt(s, errors);
                }
            }
            for s in &case_stmt.node.else_stmts {
                scan_pl_stmt(s, errors);
            }
        }
        Ps::Loop(ref loop_stmt) => {
            for s in &loop_stmt.node.body {
                scan_pl_stmt(s, errors);
            }
        }
        Ps::While(ref while_stmt) => {
            for s in &while_stmt.node.body {
                scan_pl_stmt(s, errors);
            }
        }
        Ps::For(ref for_stmt) => {
            for s in &for_stmt.node.body {
                scan_pl_stmt(s, errors);
            }
        }
        Ps::ForEach(ref foreach_stmt) => {
            for s in &foreach_stmt.node.body {
                scan_pl_stmt(s, errors);
            }
        }
        Ps::ForAll(ref _forall_stmt) => {
            // ForAll body is a String (raw SQL), not structured PlStatement
        }
        _ => {}
    }
}

fn validate_single_merge(stmt: &crate::ast::MergeStatement, loc: crate::token::SourceLocation) -> Vec<MergeSemanticError> {
    let mut errors = Vec::new();

    for clause in &stmt.when_clauses {
        // Check 1: WHEN MATCHED THEN DELETE is not supported
        if clause.matched && matches!(clause.action, crate::ast::MergeAction::Delete) {
            errors.push(MergeSemanticError {
                kind: MergeSemanticErrorKind::DeleteNotSupported,
                detail: Some("GaussDB does not support MERGE ... WHEN MATCHED THEN DELETE".to_string()),
                location: loc.clone(),
            });
        }

        // Check 2: Columns referenced in ON clause cannot be updated
        if let crate::ast::MergeAction::Update(ref assignments) = clause.action {
            let on_columns = extract_target_columns_from_expr(&stmt.on_condition, &stmt.target);
            if !on_columns.is_empty() {
                let mut conflicting = Vec::new();
                for assign in assignments {
                    for col_name in &assign.columns {
                        let col_lower = col_name.last().map(|s| s.to_lowercase());
                        if let Some(ref col) = col_lower {
                            if on_columns.contains(col) {
                                if !conflicting.contains(col) {
                                    conflicting.push(col.clone());
                                }
                            }
                        }
                    }
                }
                if !conflicting.is_empty() {
                    errors.push(MergeSemanticError {
                        kind: MergeSemanticErrorKind::OnColumnUpdated,
                        detail: Some(format!(
                            "Columns referenced in the ON clause cannot be updated: {}",
                            conflicting.join(", ")
                        )),
                        location: loc.clone(),
                    });
                }
            }
        }
    }

    // Check 3: DUAL table does not exist in GaussDB
    if uses_dual_table(&stmt.source) {
        errors.push(MergeSemanticError {
            kind: MergeSemanticErrorKind::DualTableNotSupported,
            detail: Some("GaussDB does not have a DUAL table; use a VALUES clause or bare SELECT instead".to_string()),
            location: loc.clone(),
        });
    }

    errors
}

/// Extract column names from an expression that belong to the target table.
///
/// The target table is identified by its alias (if present) or its unqualified name.
/// Column names are returned in lowercase for case-insensitive comparison.
fn extract_target_columns_from_expr(
    expr: &Expr,
    target: &crate::ast::TableRef,
) -> std::collections::HashSet<String> {
    let target_id = match target {
        crate::ast::TableRef::Table { name, alias, .. } => {
            if let Some(ref a) = alias {
                a.to_lowercase()
            } else {
                name.last().map(|s| s.to_lowercase()).unwrap_or_default()
            }
        }
        _ => return std::collections::HashSet::new(),
    };

    let mut cols = std::collections::HashSet::new();
    collect_target_columns(expr, &target_id, &mut cols);
    cols
}

fn collect_target_columns(
    expr: &Expr,
    target_id: &str,
    cols: &mut std::collections::HashSet<String>,
) {
    match expr {
        Expr::ColumnRef(parts) => {
            if parts.len() == 2 && parts[0].to_lowercase() == target_id {
                cols.insert(parts[1].to_lowercase());
            } else if parts.len() == 1 {
                // Unqualified column — conservatively include as potential target column
                cols.insert(parts[0].to_lowercase());
            }
        }
        Expr::BinaryOp { left, right, .. } => {
            collect_target_columns(left, target_id, cols);
            collect_target_columns(right, target_id, cols);
        }
        Expr::Parenthesized(inner) => {
            collect_target_columns(inner, target_id, cols);
        }
        Expr::FunctionCall { args, .. } => {
            for arg in args {
                collect_target_columns(arg, target_id, cols);
            }
        }
        Expr::Like { expr: e, pattern, .. } => {
            collect_target_columns(e, target_id, cols);
            collect_target_columns(pattern, target_id, cols);
        }
        Expr::Between { expr: e, low, high, .. } => {
            collect_target_columns(e, target_id, cols);
            collect_target_columns(low, target_id, cols);
            collect_target_columns(high, target_id, cols);
        }
        Expr::InList { expr: e, list, .. } => {
            collect_target_columns(e, target_id, cols);
            for item in list {
                collect_target_columns(item, target_id, cols);
            }
        }
        Expr::IsNull { expr: e, .. } => {
            collect_target_columns(e, target_id, cols);
        }
        Expr::IsBoolean { expr: e, .. } => {
            collect_target_columns(e, target_id, cols);
        }
        Expr::UnaryOp { expr: e, .. } => {
            collect_target_columns(e, target_id, cols);
        }
        Expr::TypeCast { expr: e, .. } => {
            collect_target_columns(e, target_id, cols);
        }
        Expr::Case { operand, whens, else_expr } => {
            if let Some(ref op) = operand {
                collect_target_columns(op, target_id, cols);
            }
            for when in whens {
                collect_target_columns(&when.condition, target_id, cols);
                collect_target_columns(&when.result, target_id, cols);
            }
            if let Some(ref el) = else_expr {
                collect_target_columns(el, target_id, cols);
            }
        }
        _ => {}
    }
}

/// Check if a TableRef (or its nested subqueries) references the DUAL table.
fn uses_dual_table(table_ref: &crate::ast::TableRef) -> bool {
    match table_ref {
        crate::ast::TableRef::Table { name, .. } => {
            name.last().map_or(false, |n| n.eq_ignore_ascii_case("dual"))
        }
        crate::ast::TableRef::Subquery { query, .. } => {
            for tr in &query.from {
                if uses_dual_table(tr) {
                    return true;
                }
            }
            false
        }
        crate::ast::TableRef::Join { left, right, .. } => {
            uses_dual_table(left) || uses_dual_table(right)
        }
        crate::ast::TableRef::Pivot { source, .. } => uses_dual_table(source),
        crate::ast::TableRef::Unpivot { source, .. } => uses_dual_table(source),
        _ => false,
    }
}

#[cfg(test)]
mod tests;
