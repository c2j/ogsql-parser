# Parameterized SQL Output — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Extend `DynamicSqlAnalyzer` to produce `parameterized_sql` and `parameter_bindings` from `TraceChain` trees, including support for NULL variable values.

**Architecture:** Add a post-processing step that walks the `TraceChain` tree to generate parameterized SQL. `LiteralAssignment` nodes become static SQL text; `VariableCopy` nodes become `:param` placeholders. Detect quote-wrapping patterns (when a `LiteralAssignment` ending in `'` precedes a `VariableCopy` followed by a `LiteralAssignment` starting with `'`). NULL variables are handled naturally because parameterization doesn't require actual values.

**Tech Stack:** Rust, serde (JSON serialization), existing analyzer infrastructure

**Related Issues:** #32 (parameterized_sql), #33 (NULL variable handling)

---

## Background

The `DynamicSqlAnalyzer` (`src/analyzer/mod.rs`, 345 lines) traces variable assignments through PL/pgSQL blocks and resolves dynamic SQL in `EXECUTE IMMEDIATE` statements. It produces `ExecuteFinding` with `resolved_value` (concrete SQL) and `trace` (TraceChain tree).

Currently, downstream tools (sp2java code generator) must traverse the trace tree themselves to distinguish static SQL fragments from variable injection points. This plan adds `parameterized_sql` and `parameter_bindings` directly to `ExecuteFinding`.

### TraceChain Types (existing)

```rust
enum TraceChain {
    LiteralAssignment { value: String },           // Static text
    VariableCopy { source_var: String, source_chain: Box<TraceChain> }, // Variable ref
    Concatenation { parts: Vec<TraceChain> },      // || concatenation
    DeclarationDefault { value: String },           // Default from DECLARE
    Unknown,                                        // Cannot resolve
}
```

### Parameterization Rules

1. `LiteralAssignment` → keep as static SQL text
2. `VariableCopy` → replace with `:variable_name` placeholder
3. `DeclarationDefault` → keep as static SQL text (treated like a literal)
4. `Unknown` → replace with `:?` placeholder (unknown variable)
5. **Quote wrapping**: When `LiteralAssignment` before a `VariableCopy` ends with `'` and the `LiteralAssignment` after it starts with `'`, detect wrapping pattern and:
   - Strip the trailing `'` from the preceding literal
   - Strip the leading `'` from the following literal
   - Mark `wrapping: "'...'"` in bindings
6. **Multiple occurrences** of same variable → each gets its own binding entry with position

### Example Transformation

Input trace for: `v_sql := 'SELECT * FROM t WHERE id=''' || p_id || ''''`

```
Concatenation {
    parts: [
        LiteralAssignment { value: "SELECT * FROM t WHERE id='" },
        VariableCopy { source_var: "p_id", source_chain: LiteralAssignment { value: "admin" } },
        LiteralAssignment { value: "'" }
    ]
}
```

Output:
```json
{
  "parameterized_sql": "SELECT * FROM t WHERE id= :p_id",
  "parameter_bindings": [
    { "position": 1, "variable": "p_id", "wrapping": "'...'" }
  ]
}
```

---

## Task 1: Add ParameterBinding type and Extend ExecuteFinding

**Files:**
- Modify: `src/analyzer/mod.rs` (type definitions at lines 8-46)

**Step 1: Write the failing test**

Add to `src/analyzer/tests.rs`:

```rust
#[test]
fn test_parameterized_sql_simple_variable() {
    let block = parse_block(
        "DO $$ BEGIN v_sql := 'SELECT * FROM t WHERE id=''' || p_id || ''''; EXECUTE IMMEDIATE v_sql; END $$"
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    let finding = &report.execute_findings[0];
    // resolved_value is None because p_id is unknown
    assert!(finding.resolved_value.is_none());
    // But parameterized_sql should still be generated
    assert_eq!(
        finding.parameterized_sql.as_deref(),
        Some("SELECT * FROM t WHERE id= :p_id")
    );
    assert_eq!(finding.parameter_bindings.len(), 1);
    assert_eq!(finding.parameter_bindings[0].variable, "p_id");
    assert_eq!(finding.parameter_bindings[0].wrapping, Some("'...'".to_string()));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_parameterized_sql_simple_variable`
Expected: FAIL — `parameterized_sql` and `parameter_bindings` fields don't exist yet

**Step 3: Add ParameterBinding struct and extend ExecuteFinding**

In `src/analyzer/mod.rs`, add after `VariableTrace` (around line 28):

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParameterBinding {
    pub position: usize,
    pub variable: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapping: Option<String>,
}
```

Modify `ExecuteFinding` to add two new fields:

```rust
pub struct ExecuteFinding {
    pub statement_path: Vec<usize>,
    pub expression_desc: String,
    pub resolved_value: Option<String>,
    pub parsed_statement: Option<Box<Statement>>,
    pub trace: TraceChain,
    // New fields:
    pub parameterized_sql: Option<String>,
    pub parameter_bindings: Vec<ParameterBinding>,
}
```

**Step 4: Run test to verify it still fails (field initialization)**

Run: `cargo test test_parameterized_sql_simple_variable`
Expected: FAIL — constructor needs new fields

**Step 5: Update ExecuteFinding construction sites**

In `process_statement` -> `PlStatement::Execute` branch (around line 181), add default values for new fields temporarily:

```rust
self.findings.push(ExecuteFinding {
    statement_path: self.path.clone(),
    expression_desc: desc,
    resolved_value: resolved,
    parsed_statement: parsed,
    trace,
    parameterized_sql: None,           // TODO: implement
    parameter_bindings: Vec::new(),    // TODO: implement
});
```

**Step 6: Run all existing tests to verify no regression**

Run: `cargo test`
Expected: All 965 tests pass

**Step 7: Commit**

```bash
git add src/analyzer/mod.rs src/analyzer/tests.rs
git commit -m "feat(#32): add ParameterBinding type and extend ExecuteFinding with parameterized fields"
```

---

## Task 2: Implement parameterize_trace function

**Files:**
- Modify: `src/analyzer/mod.rs` (new function after `resolve_expr`, around line 318)

**Step 1: Write the failing tests**

Add to `src/analyzer/tests.rs`:

```rust
#[test]
fn test_parameterized_sql_literal_only() {
    // Pure literal — parameterized_sql == resolved_value, no bindings
    let block = parse_block("DO $$ BEGIN EXECUTE 'SELECT 1'; END $$");
    let report = analyze_pl_block(&block);
    let finding = &report.execute_findings[0];
    assert_eq!(finding.parameterized_sql.as_deref(), Some("SELECT 1"));
    assert!(finding.parameter_bindings.is_empty());
}

#[test]
fn test_parameterized_sql_concat_no_quotes() {
    // Variable without quote wrapping
    let block = parse_block(
        "DO $$ BEGIN v_sql := 'SELECT * FROM ' || tab_name; EXECUTE IMMEDIATE v_sql; END $$"
    );
    let report = analyze_pl_block(&block);
    let finding = &report.execute_findings[0];
    assert_eq!(
        finding.parameterized_sql.as_deref(),
        Some("SELECT * FROM  :tab_name")
    );
    assert_eq!(finding.parameter_bindings.len(), 1);
    assert_eq!(finding.parameter_bindings[0].variable, "tab_name");
    assert!(finding.parameter_bindings[0].wrapping.is_none());
}

#[test]
fn test_parameterized_sql_multiple_vars_with_quotes() {
    // Issue #32 example: multiple variables with quote wrapping
    let block = parse_block(
        r#"DO $$
DECLARE
    v_sql VARCHAR;
    p_acnt VARCHAR := '12345';
    p_name VARCHAR := 'test';
BEGIN
    v_sql := 'SELECT * FROM t_users WHERE 1=1';
    v_sql := v_sql || ' AND accno = ''' || p_acnt || '''';
    v_sql := v_sql || ' AND (''' || p_name || ''' IS NULL OR name = ''' || p_name || ''')';
    EXECUTE IMMEDIATE v_sql;
END $$"#
    );
    let report = analyze_pl_block(&block);
    let finding = &report.execute_findings[0];
    assert!(finding.parameterized_sql.is_some());
    let psql = finding.parameterized_sql.as_ref().unwrap();
    assert!(psql.contains(":p_acnt"));
    assert!(psql.contains(":p_name"));
    assert_eq!(finding.parameter_bindings.len(), 3); // p_acnt once, p_name twice

    // p_acnt has wrapping
    let acnt_binding = finding.parameter_bindings.iter().find(|b| b.variable == "p_acnt").unwrap();
    assert_eq!(acnt_binding.wrapping, Some("'...'".to_string()));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_parameterized_sql`
Expected: FAIL — `parameterized_sql` is still `None`

**Step 3: Implement `parameterize_trace` function**

Add new function in `impl DynamicSqlAnalyzer` block (or as a free function since it only uses `&TraceChain`):

```rust
/// Result of parameterizing a TraceChain tree.
struct ParameterizedResult {
    sql: String,
    bindings: Vec<ParameterBinding>,
}

/// Walk the TraceChain tree and produce parameterized SQL.
/// `LiteralAssignment` → static text
/// `VariableCopy` → `:var_name` placeholder
/// `Concatenation` → recurse into parts
/// `DeclarationDefault` → static text
/// `Unknown` → `:?` placeholder
fn parameterize_trace(trace: &TraceChain) -> ParameterizedResult {
    let mut bindings = Vec::new();
    let sql = parameterize_trace_inner(trace, &mut bindings);
    ParameterizedResult { sql, bindings }
}

fn parameterize_trace_inner(trace: &TraceChain, bindings: &mut Vec<ParameterBinding>) -> String {
    match trace {
        TraceChain::LiteralAssignment { value } => value.clone(),
        TraceChain::DeclarationDefault { value } => value.clone(),
        TraceChain::VariableCopy { source_var, .. } => {
            let pos = bindings.len() + 1;
            bindings.push(ParameterBinding {
                position: pos,
                variable: source_var.clone(),
                wrapping: None, // will be fixed up by quote detection
            });
            format!(" :{}", source_var)
        }
        TraceChain::Unknown => " :?".to_string(),
        TraceChain::Concatenation { parts } => {
            parts.iter().map(|p| parameterize_trace_inner(p, bindings)).collect()
        }
    }
}
```

**Step 4: Implement quote-wrapping detection**

Add a post-processing step that detects and handles the quote-wrapping pattern. This is a separate function that examines the `Concatenation` structure:

```rust
/// Detect quote wrapping in Concatenation parts and fix up both the SQL string and bindings.
///
/// Pattern: LiteralAssignment ending with `'` + VariableCopy + LiteralAssignment starting with `'`
/// Action: Strip quotes, mark wrapping on binding
fn detect_quote_wrapping(trace: &TraceChain, sql: &mut String, bindings: &mut Vec<ParameterBinding>) {
    // Walk the tree to find wrapping patterns
    detect_wrapping_in_trace(trace, bindings);
    // Fix up the SQL string by removing paired quotes around placeholders
    *sql = fix_wrapping_quotes(sql, bindings);
}

fn detect_wrapping_in_trace(trace: &TraceChain, bindings: &mut Vec<ParameterBinding>) {
    if let TraceChain::Concatenation { parts } = trace {
        for i in 0..parts.len() {
            if i > 0 && i + 1 < parts.len() {
                let before = &parts[i - 1];
                let current = &parts[i];
                let after = &parts[i + 1];

                if let (TraceChain::LiteralAssignment { value: before_val }, 
                         TraceChain::VariableCopy { source_var, .. },
                         TraceChain::LiteralAssignment { value: after_val }) = 
                    (before, current, after) 
                {
                    if before_val.ends_with('\'') && after_val.starts_with('\'') {
                        // Found wrapping pattern
                        // Find the corresponding binding and set wrapping
                        let binding = bindings.iter_mut()
                            .find(|b| b.variable == *source_var && b.wrapping.is_none());
                        if let Some(b) = binding {
                            b.wrapping = Some("'...'".to_string());
                        }
                    }
                }
            }
            // Recurse into nested Concatenation
            detect_wrapping_in_trace(&parts[i], bindings);
        }
    }
}

fn fix_wrapping_quotes(sql: &str, bindings: &[ParameterBinding]) -> String {
    let mut result = sql.to_string();
    for binding in bindings {
        if binding.wrapping.is_some() {
            // Replace `' :var_name'` with ` :var_name` (remove the quote before placeholder)
            // And also handle the quote after
            let placeholder = format!(" :{}", binding.variable);
            // The pattern in SQL is: `' :var_name'` → ` :var_name`
            // We need to remove the quote before and the quote after
            result = result.replace(&format!("'{}", placeholder), placeholder);
            result = result.replace(&format!("{}'", placeholder), placeholder);
        }
    }
    result
}
```

**Step 5: Wire it into ExecuteFinding construction**

In `process_statement` → `PlStatement::Execute` branch:

```rust
PlStatement::Execute(exec) => {
    let (resolved, trace) = self.resolve_expr(&exec.string_expr);
    let parsed = resolved
        .as_ref()
        .and_then(|s| crate::parser::Parser::parse_statement_from_str(s));
    let desc = self.expr_to_string(&exec.string_expr);
    
    // Generate parameterized SQL
    let param_result = parameterize_trace(&trace);
    
    self.findings.push(ExecuteFinding {
        statement_path: self.path.clone(),
        expression_desc: desc,
        resolved_value: resolved,
        parsed_statement: parsed,
        trace,
        parameterized_sql: if param_result.sql.is_empty() { None } else { Some(param_result.sql) },
        parameter_bindings: param_result.bindings,
    });
}
```

**Step 6: Run all tests**

Run: `cargo test`
Expected: All tests pass, including new parameterized SQL tests

**Step 7: Commit**

```bash
git add src/analyzer/mod.rs src/analyzer/tests.rs
git commit -m "feat(#32): implement parameterize_trace with quote-wrapping detection"
```

---

## Task 3: Handle NULL variable values (#33)

**Files:**
- Modify: `src/analyzer/mod.rs` (evaluate_expr, process_declarations)

**Step 1: Write the failing test**

Add to `src/analyzer/tests.rs`:

```rust
#[test]
fn test_parameterized_sql_with_null_variable() {
    // Issue #33: NULL variable should still produce parameterized_sql
    let block = parse_block(
        r#"DO $$
DECLARE
    v_sql VARCHAR;
    p_acnt VARCHAR := '12345';
    p_name VARCHAR := NULL;
BEGIN
    v_sql := 'SELECT * FROM t_users WHERE 1=1';
    v_sql := v_sql || ' AND accno = ''' || p_acnt || '''';
    v_sql := v_sql || ' AND (''' || p_name || ''' IS NULL OR name = ''' || p_name || ''')';
    EXECUTE IMMEDIATE v_sql;
END $$"#
    );
    let report = analyze_pl_block(&block);
    assert_eq!(report.execute_findings.len(), 1);
    let finding = &report.execute_findings[0];
    
    // resolved_value should be None (because p_name is NULL, can't resolve full SQL)
    assert!(finding.resolved_value.is_none());
    
    // But parameterized_sql should still be generated
    assert!(finding.parameterized_sql.is_some());
    let psql = finding.parameterized_sql.as_ref().unwrap();
    assert!(psql.contains(":p_acnt"));
    assert!(psql.contains(":p_name"), "should contain :p_name placeholder, got: {}", psql);
    
    // Should have 3 bindings: p_acnt (1), p_name (2)
    let acnt_bindings: Vec<_> = finding.parameter_bindings.iter()
        .filter(|b| b.variable == "p_acnt").collect();
    let name_bindings: Vec<_> = finding.parameter_bindings.iter()
        .filter(|b| b.variable == "p_name").collect();
    assert_eq!(acnt_bindings.len(), 1);
    assert_eq!(name_bindings.len(), 2);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_parameterized_sql_with_null_variable`
Expected: FAIL — `p_name` is NULL so `resolved_value` is None, and `parameterized_sql` may also be None or missing `:p_name`

**Step 3: Trace the issue — NULL literal handling in evaluate_expr**

Currently in `evaluate_expr`, there's no handling for `Literal::Null`. When `p_name VARCHAR := NULL`, the default expression is `Literal::Null`, which falls through to the `_ =>` catch-all that returns `VarState { known_value: None, trace: TraceChain::Unknown }`.

Fix: Add `Literal::Null` handling that preserves the variable name in the trace while keeping `known_value: None`:

In `evaluate_expr`, add a case for `Expr::Literal(Literal::Null)`:

```rust
Expr::Literal(Literal::Null) => VarState {
    known_value: None,
    trace: TraceChain::Unknown, // NULL values are unknown for SQL resolution
},
```

Wait — this is already what happens. The issue is that when `p_name := NULL`, the variable gets registered with `trace: TraceChain::Unknown`, so when it appears in a concatenation like `'...' || p_name || '...'`, the VariableCopy is generated correctly but the Concatenation can't resolve the full value.

Let me check: does `parameterize_trace` handle `Unknown` → `:?` correctly? Actually, `p_name` IS declared in scope with a known default of NULL. So when `evaluate_expr` sees `Expr::PlVariable(["p_name"])`, it should find it in scope with `trace: TraceChain::Unknown`. But `parameterize_trace_inner` for `Unknown` outputs ` :?` — not ` :p_name`.

**The real fix**: When `p_name` has `known_value: None` but IS a declared variable, the trace should be `VariableCopy { source_var: "p_name", source_chain: Unknown }` (not bare `Unknown`). Let me check if this is already the case...

Looking at `evaluate_expr` line 277-292: When `PlVariable` is found via `lookup_var`, it creates `VariableCopy { source_var, source_chain: Box::new(state.trace.clone()) }`. So if `p_name` was registered with `trace: TraceChain::Unknown`, evaluating `p_name` in an expression gives `VariableCopy { source_var: "p_name", source_chain: Unknown }`. This is correct!

The issue is that `process_declarations` registers NULL-default variables at line 119-126 with `trace: TraceChain::Unknown`. But the `VariableCopy` wrapping preserves the variable name. So `parameterize_trace` should already produce ` :p_name` for this case.

Let me verify by looking at the `DeclarationDefault` variant — when `p_name VARCHAR := NULL`, the parser produces a default expression of `Literal::Null`. In `evaluate_expr`, this falls through to the catch-all and returns `VarState { known_value: None, trace: TraceChain::Unknown }`. The variable is registered via `set_var` at line 118.

So when `p_name` is later used in `''' || p_name || '''`:
- `evaluate_expr` sees `PlVariable(["p_name"])`
- Looks it up → finds `VarState { known_value: None, trace: Unknown }`
- Returns `VariableCopy { source_var: "p_name", source_chain: Unknown }`

This means `parameterize_trace` should correctly produce `:p_name`. The test should pass once Task 2 is implemented.

But wait — the `known_value: None` causes the Concatenation to produce `known_value: None` (line 297-299), so `resolved_value` is `None`. But `parameterized_sql` doesn't depend on `known_value` — it depends on the trace tree structure.

**So the fix for #33 might already work with Task 2's implementation!** Let me verify with the test.

**Step 4: Run the test**

Run: `cargo test test_parameterized_sql_with_null_variable`
Expected: May PASS if Task 2 implementation is correct, or may need minor fixes

**Step 5: If needed, fix any remaining issues**

If the test fails because the NULL-declared variable isn't tracked as a `VariableCopy`, add explicit handling in `process_declarations`:

```rust
PlDeclaration::Variable(var_decl) => {
    if let Some(expr) = &var_decl.default {
        let state = self.evaluate_expr(expr);
        // ... existing code ...
        self.set_var(&var_decl.name, state);
    } else {
        self.set_var(&var_decl.name, VarState {
            known_value: None,
            trace: TraceChain::Unknown,
        });
    }
}
```

This is already the current code. The key insight is: even with `trace: TraceChain::Unknown` stored for the variable, when it's accessed via `evaluate_expr`, it wraps in `VariableCopy`. So parameterization should work.

**Step 6: Run all tests**

Run: `cargo test`
Expected: All tests pass

**Step 7: Commit**

```bash
git add src/analyzer/tests.rs
git commit -m "feat(#33): verify parameterized SQL generation works with NULL variables"
```

---

## Task 4: Edge cases and integration test

**Files:**
- Modify: `src/analyzer/tests.rs`

**Step 1: Write edge case tests**

```rust
#[test]
fn test_parameterized_sql_deeply_nested() {
    let block = parse_block(
        "DO $$ BEGIN a := 'SELECT *'; b := a || ' FROM t'; c := b || ' WHERE x=''' || p_x || ''''; EXECUTE IMMEDIATE c; END $$"
    );
    let report = analyze_pl_block(&block);
    let finding = &report.execute_findings[0];
    let psql = finding.parameterized_sql.as_ref().unwrap();
    assert!(psql.contains("SELECT * FROM t WHERE x="));
    assert!(psql.contains(":p_x"));
}

#[test]
fn test_parameterized_sql_same_var_multiple_times() {
    let block = parse_block(
        "DO $$ BEGIN v := 'BETWEEN ''' || p_val || ''' AND ''' || p_val || ''''; EXECUTE IMMEDIATE v; END $$"
    );
    let report = analyze_pl_block(&block);
    let finding = &report.execute_findings[0];
    let psql = finding.parameterized_sql.as_ref().unwrap();
    // Should contain :p_val twice
    assert_eq!(psql.matches(":p_val").count(), 2);
    assert_eq!(finding.parameter_bindings.len(), 2);
    assert!(finding.parameter_bindings.iter().all(|b| b.variable == "p_val"));
}
```

**Step 2: Run all tests**

Run: `cargo test`
Expected: All pass

**Step 3: Commit**

```bash
git add src/analyzer/tests.rs
git commit -m "test(#32): add edge case tests for parameterized SQL output"
```

---

## Summary

After completing all tasks, the `DynamicSqlReport` JSON output will include:

```json
{
  "execute_findings": [
    {
      "statement_path": [1],
      "expression_desc": "v_sql",
      "resolved_value": "SELECT * FROM t WHERE id='admin'",
      "parameterized_sql": "SELECT * FROM t WHERE id= :p_id",
      "parameter_bindings": [
        { "position": 1, "variable": "p_id", "wrapping": "'...'" }
      ],
      "parsed_statement": { "Select": { ... } },
      "trace": { ... }
    }
  ],
  "variable_traces": [...]
}
```

This enables downstream code generators (sp2java) to directly convert dynamic SQL to MyBatis parameterized queries without traversing the trace tree themselves.
