use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::ast::plpgsql::{PlBlock, PlDeclaration, PlOpenKind, PlStatement};
use crate::ast::{Expr, Literal, Statement};

// ── 报告类型 ──

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DynamicSqlReport {
    pub execute_findings: Vec<ExecuteFinding>,
    pub variable_traces: Vec<VariableTrace>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ref_cursor_queries: Vec<RefCursorQuery>,
}

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
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParameterBinding {
    pub position: usize,
    pub variable: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapping: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VariableTrace {
    pub variable_name: String,
    pub assignment_path: Vec<usize>,
    pub value: String,
}

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
pub struct QueryFingerprint {
    pub fingerprint: String,
    pub occurrences: Vec<FingerprintOccurrence>,
    pub normalized_sql: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

// ── 内部状态 ──

struct VarState {
    known_value: Option<String>,
    trace: TraceChain,
}

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

pub fn analyze_pl_block(block: &PlBlock) -> DynamicSqlReport {
    DynamicSqlAnalyzer::new().analyze(block)
}

// ── Transaction Analysis ──

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransactionReport {
    pub has_explicit_commit: bool,
    pub has_explicit_rollback: bool,
    pub has_autonomous_transaction: bool,
    pub transaction_segments: Vec<TransactionSegment>,
    pub cross_procedure_calls: Vec<CrossProcedureCall>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransactionSegment {
    pub index: usize,
    pub start_reason: TransactionBoundary,
    pub end_reason: TransactionBoundary,
    pub statement_range: (usize, usize),
    pub sub_transactions: Vec<SubTransaction>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransactionBoundary {
    ProcedureEntry,
    PostCommit,
    PostRollback,
    Commit,
    Rollback,
    ProcedureExit,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubTransaction {
    pub block_path: Vec<usize>,
    pub implicit_savepoint: bool,
    pub body_range: (usize, usize),
    pub exception_handlers: Vec<ExceptionHandlerInfo>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExceptionHandlerInfo {
    pub conditions: Vec<String>,
    pub statement_count: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CrossProcedureCall {
    pub call_path: Vec<usize>,
    pub callee: String,
    pub callee_may_commit: bool,
}

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

#[cfg(test)]
mod tests;
