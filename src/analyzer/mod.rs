use std::collections::HashMap;

use crate::ast::plpgsql::{PlBlock, PlDeclaration, PlStatement};
use crate::ast::{Expr, Literal, Statement};

// ── 报告类型 ──

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DynamicSqlReport {
    pub execute_findings: Vec<ExecuteFinding>,
    pub variable_traces: Vec<VariableTrace>,
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

#[cfg(test)]
mod tests;
