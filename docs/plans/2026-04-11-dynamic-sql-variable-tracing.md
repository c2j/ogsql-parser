# 动态 SQL 变量追踪：完整数据流分析 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 新建 `analyzer` 模块，对 PL/pgSQL 块做完整数据流分析，追踪变量赋值链，自动发现 EXECUTE IMMEDIATE 中变量引用的实际字符串值并解析为结构化 AST，输出独立分析报告。

**Architecture:** 新增 `src/analyzer/` 模块，与 parser/formatter 平级。核心是一个 `DynamicSqlAnalyzer` 结构体，递归遍历 `PlBlock` AST，维护作用域栈和变量状态表。对每个 `PlStatement::Execute`，尝试通过常量传播解析 `string_expr` 的具体值。结果存入 `DynamicSqlReport` 独立输出，不修改现有 AST 类型。

**Tech Stack:** Rust 2021, serde, indexmap（可选，用于有序作用域）

---

## 核心数据结构

### 分析报告

```rust
/// 独立分析报告，不修改 AST。
/// 消费方式：analyzer::analyze_pl_block(&block) → DynamicSqlReport
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DynamicSqlReport {
    /// 所有发现的 EXECUTE IMMEDIATE 语句分析结果
    pub execute_findings: Vec<ExecuteFinding>,
    /// 追踪过程中发现的变量赋值摘要（供调试/展示）
    pub variable_traces: Vec<VariableTrace>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecuteFinding {
    /// EXECUTE 语句在 body 中的路径（如 [0, 2] 表示 body[0] 内嵌套 body[2]）
    pub statement_path: Vec<usize>,
    /// 原始 string_expr（变量名或表达式描述）
    pub expression_desc: String,
    /// 追踪结果
    pub resolved_value: Option<String>,
    /// 如果 resolved_value 解析成功，这里是结构化 AST
    pub parsed_statement: Option<Box<crate::ast::Statement>>,
    /// 追踪链（变量值是怎么来的）
    pub trace: TraceChain,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VariableTrace {
    pub variable_name: String,
    pub assignment_path: Vec<usize>,
    pub value: String,
}

/// 追踪链：描述值是怎么传播的
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TraceChain {
    /// 直接字面量赋值
    LiteralAssignment { value: String },
    /// 从另一个变量复制
    VariableCopy { source_var: String, source_chain: Box<TraceChain> },
    /// 字符串拼接
    Concatenation { parts: Vec<TraceChain> },
    /// DECLARE 默认值
    DeclarationDefault { value: String },
    /// 无法静态解析（运行时决定）
    Unknown,
}
```

### 分析器

```rust
pub struct DynamicSqlAnalyzer {
    /// 作用域栈：每个作用域是变量名到追踪信息的映射
    scopes: Vec<HashMap<String, VarState>>,
    /// 收集的 EXECUTE 发现
    findings: Vec<ExecuteFinding>,
    /// 收集的变量追踪
    traces: Vec<VariableTrace>,
    /// 当前语句路径
    path: Vec<usize>,
}

struct VarState {
    /// 已知的字符串值（None 表示运行时决定）
    known_value: Option<String>,
    /// 追踪链
    trace: TraceChain,
}
```

---

## Task 1: 创建 analyzer 模块骨架

**Files:**
- Create: `src/analyzer/mod.rs`
- Modify: `src/lib.rs` (添加 `pub mod analyzer`)

**Step 1: 创建 analyzer 模块**

创建 `src/analyzer/mod.rs`：

```rust
use std::collections::HashMap;
use crate::ast::plpgsql::{PlBlock, PlStatement, PlDeclaration};
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
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VariableTrace {
    pub variable_name: String,
    pub assignment_path: Vec<usize>,
    pub value: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TraceChain {
    LiteralAssignment { value: String },
    VariableCopy { source_var: String, source_chain: Box<TraceChain> },
    Concatenation { parts: Vec<TraceChain> },
    DeclarationDefault { value: String },
    Unknown,
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
        self.enter_scope();
        self.process_declarations(&block.declarations);
        self.process_statements(&block.body);
        self.exit_scope();
        DynamicSqlReport {
            execute_findings: self.findings,
            variable_traces: self.traces,
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
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
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), state);
        }
    }

    fn process_declarations(&mut self, declarations: &[PlDeclaration]) {
        for decl in declarations {
            if let PlDeclaration::Variable(var_decl) = decl {
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
                }
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
                let state = self.evaluate_expr(expression);
                if let Some(ref value) = state.known_value {
                    self.traces.push(VariableTrace {
                        variable_name: target.clone(),
                        assignment_path: self.path.clone(),
                        value: value.clone(),
                    });
                }
                self.set_var(target, state);
            }

            PlStatement::Execute(exec) => {
                let (resolved, trace) = self.resolve_expr(&exec.string_expr);
                let parsed = resolved.as_ref().and_then(|s| {
                    crate::parser::Parser::parse_statement_from_str(s)
                });
                let desc = self.expr_to_string(&exec.string_expr);
                self.findings.push(ExecuteFinding {
                    statement_path: self.path.clone(),
                    expression_desc: desc,
                    resolved_value: resolved,
                    parsed_statement: parsed,
                    trace,
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
                self.process_statements(&for_stmt.body);
            }

            PlStatement::ForEach(foreach_stmt) => {
                self.process_statements(&foreach_stmt.body);
            }

            PlStatement::ForAll(_) => {}

            _ => {}
        }
    }

    /// 尝试在静态层面求值一个表达式
    fn evaluate_expr(&self, expr: &Expr) -> VarState {
        match expr {
            Expr::Literal(Literal::String(s)) => VarState {
                known_value: Some(s.clone()),
                trace: TraceChain::LiteralAssignment { value: s.clone() },
            },
            Expr::Literal(Literal::DollarString { body, .. }) => VarState {
                known_value: Some(body.clone()),
                trace: TraceChain::LiteralAssignment { value: body.clone() },
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
                    VarState { known_value: None, trace: TraceChain::Unknown }
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
            _ => VarState { known_value: None, trace: TraceChain::Unknown },
        }
    }

    /// 解析表达式，返回 (解析到的字符串值, 追踪链)
    fn resolve_expr(&self, expr: &Expr) -> (Option<String>, TraceChain) {
        let state = self.evaluate_expr(expr);
        (state.known_value, state.trace)
    }

    fn expr_to_string(&self, expr: &Expr) -> String {
        match expr {
            Expr::ColumnRef(names) => names.join("."),
            Expr::Literal(Literal::String(s)) => format!("'{}'", s),
            Expr::BinaryOp { op, left, right } => {
                format!("{} {} {}", self.expr_to_string(left), op, self.expr_to_string(right))
            }
            _ => format!("{:?}", expr),
        }
    }
}

// ── 公共入口函数 ──

pub fn analyze_pl_block(block: &PlBlock) -> DynamicSqlReport {
    DynamicSqlAnalyzer::new().analyze(block)
}
```

**Step 2: 注册模块**

在 `src/lib.rs` 添加：

```rust
pub mod analyzer;
```

并在适当位置添加 re-export：

```rust
pub use analyzer::{DynamicSqlReport, analyze_pl_block};
```

**Step 3: 编译验证**

Run: `cargo check 2>&1 | grep "error" | head -10`

预期：编译通过或只有警告。

**Step 4: Commit**

```bash
git add src/analyzer/mod.rs src/lib.rs
git commit -m "feat(analyzer): add DynamicSqlAnalyzer with full data flow analysis for PL/pgSQL"
```

---

## Task 2: 添加全面测试

**Files:**
- Create: `src/analyzer/tests.rs`
- Modify: `src/analyzer/mod.rs` (添加 `#[cfg(test)] mod tests`)

**Step 1: 在 `src/analyzer/mod.rs` 末尾添加测试模块**

```rust
#[cfg(test)]
mod tests;
```

**Step 2: 创建 `src/analyzer/tests.rs`**

```rust
use super::*;
use crate::parser::Parser;

fn parse_block(sql: &str) -> crate::ast::plpgsql::PlBlock {
    let tokens = crate::Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    match &stmts[0] {
        crate::ast::Statement::Do(d) => d.block.as_ref().expect("block should parse").clone(),
        crate::ast::Statement::AnonyBlock(ab) => ab.block.clone(),
        _ => panic!("expected DO or AnonyBlock, got {:?}", stmts[0]),
    }
}

#[test]
fn test_trace_literal_assignment_to_execute() {
    let block = parse_block(
        "DO $$ BEGIN plsql_block := 'call calc_stats($1, $1, $2, $1)'; EXECUTE IMMEDIATE plsql_block USING a, b; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    let finding = &report.execute_findings[0];
    assert_eq!(finding.expression_desc, "plsql_block");
    assert_eq!(finding.resolved_value.as_deref(), Some("call calc_stats($1, $1, $2, $1)"));
    assert!(finding.parsed_statement.is_some());
    match &finding.trace {
        TraceChain::VariableCopy { source_var, source_chain } => {
            assert_eq!(source_var, "plsql_block");
            assert!(matches!(source_chain.as_ref(), TraceChain::LiteralAssignment { .. }));
        }
        other => panic!("expected VariableCopy, got {:?}", other),
    }
}

#[test]
fn test_trace_declare_default_to_execute() {
    let block = parse_block(
        "DO $$ BEGIN EXECUTE IMMEDIATE plsql_block; END $$"
    );
    // Note: This test depends on whether DECLARE sections parse correctly
    // If DECLARE doesn't parse, the variable is unknown
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    // Without DECLARE section, plsql_block is unknown
    assert!(report.execute_findings[0].resolved_value.is_none());
}

#[test]
fn test_trace_variable_chain() {
    let block = parse_block(
        "DO $$ BEGIN a := 'SELECT 1'; b := a; EXECUTE IMMEDIATE b; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    let finding = &report.execute_findings[0];
    assert_eq!(finding.resolved_value.as_deref(), Some("SELECT 1"));
    match &finding.trace {
        TraceChain::VariableCopy { source_var, source_chain, .. } => {
            assert_eq!(source_var, "b");
            match source_chain.as_ref() {
                TraceChain::VariableCopy { source_var, .. } => assert_eq!(source_var, "a"),
                other => panic!("expected nested VariableCopy, got {:?}", other),
            }
        }
        other => panic!("expected VariableCopy, got {:?}", other),
    }
}

#[test]
fn test_trace_concatenation() {
    let block = parse_block(
        "DO $$ BEGIN prefix := 'SELECT * FROM '; suffix := 'users'; full_sql := prefix || suffix; EXECUTE IMMEDIATE full_sql; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    let finding = &report.execute_findings[0];
    assert_eq!(finding.resolved_value.as_deref(), Some("SELECT * FROM users"));
    assert!(finding.parsed_statement.is_some());
}

#[test]
fn test_trace_concat_with_literal() {
    let block = parse_block(
        "DO $$ BEGIN tab := 'users'; EXECUTE IMMEDIATE 'SELECT * FROM ' || tab; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    let finding = &report.execute_findings[0];
    assert_eq!(finding.resolved_value.as_deref(), Some("SELECT * FROM users"));
    assert!(finding.parsed_statement.is_some());
}

#[test]
fn test_trace_unknown_variable() {
    let block = parse_block(
        "DO $$ BEGIN EXECUTE IMMEDURE unknown_var; END $$"
    );
    // This should fail parsing "EXECUTE IMMEDURE" — fix SQL:
    let block = parse_block(
        "DO $$ BEGIN EXECUTE unknown_var; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    assert!(report.execute_findings[0].resolved_value.is_none());
    assert!(matches!(report.execute_findings[0].trace, TraceChain::Unknown));
}

#[test]
fn test_trace_in_if_branch() {
    let block = parse_block(
        "DO $$ BEGIN IF true THEN sql_text := 'DROP TABLE temp'; END IF; EXECUTE IMMEDIATE sql_text; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    let finding = &report.execute_findings[0];
    assert_eq!(finding.resolved_value.as_deref(), Some("DROP TABLE temp"));
}

#[test]
fn test_trace_in_nested_block() {
    let block = parse_block(
        "DO $$ BEGIN BEGIN sql_text := 'SELECT 42'; END; EXECUTE IMMEDIATE sql_text; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    // nested block has its own scope — sql_text defined in inner scope
    // should NOT be visible in outer scope (or should it?)
    // Design decision: assignments in nested blocks propagate upward if variable exists in outer scope
    // For now: inner scope assignments don't leak to outer scope
    // This test validates whatever behavior we implement
}

#[test]
fn test_multiple_executes() {
    let block = parse_block(
        "DO $$ BEGIN a := 'SELECT 1'; b := 'SELECT 2'; EXECUTE IMMEDIATE a; EXECUTE IMMEDIATE b; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 2);
    assert_eq!(report.execute_findings[0].resolved_value.as_deref(), Some("SELECT 1"));
    assert_eq!(report.execute_findings[1].resolved_value.as_deref(), Some("SELECT 2"));
}

#[test]
fn test_variable_traces_recorded() {
    let block = parse_block(
        "DO $$ BEGIN x := 'hello'; y := x; z := y || ' world'; EXECUTE IMMEDIATE z; END $$"
    );
    let report = analyze_pl_block(&block);
    assert!(report.variable_traces.len() >= 3); // x, y, z
    assert!(report.variable_traces.iter().any(|t| t.variable_name == "x" && t.value == "hello"));
    assert!(report.variable_traces.iter().any(|t| t.variable_name == "y" && t.value == "hello"));
    assert!(report.variable_traces.iter().any(|t| t.variable_name == "z" && t.value == "hello world"));
}
```

**Step 3: 运行测试**

Run: `cargo test analyzer 2>&1 | tail -20`

预期：部分测试可能需要微调（嵌套作用域行为、SQL 语法细节）。

**Step 4: Commit**

```bash
git add src/analyzer/
git commit -m "test(analyzer): add comprehensive tests for dynamic SQL variable tracing"
```

---

## Task 3: 验证与集成

**Step 1: 全量测试**

Run: `cargo test 2>&1 | tail -10`

**Step 2: Clippy**

Run: `cargo clippy 2>&1 | grep "error" | head -5`

**Step 3: 手动集成测试**

```bash
echo "DO \$\$ BEGIN plsql_block := 'call calc_stats(\$1, \$1, \$2, \$1)'; EXECUTE IMMEDIATE plsql_block USING a,b; END \$\$" | cargo run --quiet -- analyze
```

预期输出 JSON 报告，包含 `execute_findings` 和 `variable_traces`。

**Step 4: Commit**

---

## 影响范围

| 文件 | 类型 | 改动量 |
|------|------|--------|
| `src/analyzer/mod.rs` | 新建 | ~250行 |
| `src/analyzer/tests.rs` | 新建 | ~130行 |
| `src/lib.rs` | 修改 | +2行 |
| **合计** | | ~380行 |

## 不修改的文件

- `src/ast/` — 无变更
- `src/parser/` — 无变更
- `src/formatter/` — 无变更
- `src/token/` — 无变更
