# Strict Validation Mode (--strict) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add `--strict` flag to `ogsql validate` that detects undefined function calls in PL blocks.

**Architecture:** The strict flag flows from CLI → `validate_sql()` → `validate_pl_variables_from_stmts()` → `validate_pl_variables_with_extra_vars_and_funcs()` → `PlVariableValidator`. In strict mode, the `FunctionCall` branch in `check_expr()` validates the function name against the registry + known_funcs + PL builtins.

**Tech Stack:** Rust, clap (CLI), existing analyzer infrastructure

---

### Task 1: Add `strict` field to `PlVariableValidator` and thread through public API

**Files:**
- Modify: `src/analyzer/mod.rs:1576-1591` (struct + constructor)
- Modify: `src/analyzer/mod.rs:1537-1574` (public functions)

**Step 1: Add `strict` field to struct and constructor**

In `src/analyzer/mod.rs`, modify the struct at line 1576:

```rust
struct PlVariableValidator {
    scope_stack: Vec<std::collections::HashSet<String>>,
    known_funcs: std::collections::HashSet<String>,
    errors: Vec<UndefinedVariableError>,
    current_span: Option<crate::ast::SourceSpan>,
    strict: bool,
}
```

Update constructor at line 1584:

```rust
fn new(strict: bool) -> Self {
    Self {
        scope_stack: vec![std::collections::HashSet::new()],
        known_funcs: std::collections::HashSet::new(),
        errors: Vec::new(),
        current_span: None,
        strict,
    }
}
```

**Step 2: Thread `strict` through public API**

Update all three public entry functions to accept `strict: bool`:

`validate_pl_variables` (line 1537):
```rust
pub fn validate_pl_variables(
    block: &PlBlock,
    params: &[crate::ast::RoutineParam],
) -> Vec<UndefinedVariableError> {
    validate_pl_variables_with_extra_vars_and_funcs(block, params, &[], &[], false)
}
```

`validate_pl_variables_with_extra_vars` (line 1546):
```rust
pub fn validate_pl_variables_with_extra_vars(
    block: &PlBlock,
    params: &[crate::ast::RoutineParam],
    extra_vars: &[&str],
) -> Vec<UndefinedVariableError> {
    validate_pl_variables_with_extra_vars_and_funcs(block, params, extra_vars, &[], false)
}
```

`validate_pl_variables_with_extra_vars_and_funcs` (line 1556):
```rust
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
```

**Step 3: Run `cargo check`**

Run: `cargo check`
Expected: Compile errors in `src/bin/ogsql.rs` because calls to `validate_pl_variables_with_extra_vars_and_funcs` now need a 5th argument.

---

### Task 2: Update CLI to add `--strict` flag

**Files:**
- Modify: `src/bin/ogsql.rs:116-130` (Validate variant)

**Step 1: Add `strict` field to Validate command**

```rust
Validate {
    /// Recursively scan directory for SQL files (can specify multiple times)
    /// 递归扫描目录中的 SQL 文件（可多次指定）
    #[arg(short = 'd', long = "dir")]
    dir: Vec<String>,
    /// File extensions to scan, comma-separated (default: sql) / 扫描的文件扩展名，逗号分隔
    #[arg(short = 'e', long = "ext", value_delimiter = ',', default_value = "sql")]
    ext: Vec<String>,
    /// Output validation results in CSV format / 以 CSV 格式输出校验结果
    #[arg(long = "csv")]
    csv: bool,
    /// Print statistics after directory processing
    #[arg(long)]
    stats: bool,
    /// Enable strict mode: detect undefined function calls in PL blocks / 启用严格模式：检测 PL 块中的未定义函数调用
    #[arg(long)]
    strict: bool,
},
```

**Step 2: Update `validate_sql` to accept and pass `strict`**

Change signature at line 2924:
```rust
fn validate_sql(
    sql: &str,
    mybatis: bool,
    extra_funcs: &[String],
    strict: bool,
) -> (Vec<ogsql_parser::StatementInfo>, Vec<ogsql_parser::ParserError>, Vec<ogsql_parser::PackageConsistencyError>, Vec<ogsql_parser::UndefinedVariableError>) {
```

Change call at line 2964:
```rust
let var_errors = validate_pl_variables_from_stmts(&stmts, &all_funcs, strict);
```

**Step 3: Update `validate_pl_variables_from_stmts` signature**

Change signature at line 2969:
```rust
fn validate_pl_variables_from_stmts(stmts: &[ogsql_parser::StatementInfo], known_funcs: &[String], strict: bool) -> Vec<ogsql_parser::UndefinedVariableError> {
```

Update ALL calls to `validate_pl_variables_with_extra_vars_and_funcs` inside this function to pass `strict` as the last argument. There are 5 call sites (lines 2977, 2983, 2989, 3017, 3023). Example:

```rust
// Before:
let vars = ogsql_parser::validate_pl_variables_with_extra_vars_and_funcs(block, &proc.parameters, &[], &funcs_str);
// After:
let vars = ogsql_parser::validate_pl_variables_with_extra_vars_and_funcs(block, &proc.parameters, &[], &funcs_str, strict);
```

**Step 4: Update all callers of `validate_sql` and `cmd_validate*`**

Search for all callers of `validate_sql` and `cmd_validate`:

1. `cmd_validate_single` (line 3047): pass `strict`
2. `cmd_validate_files` (line 3136): pass `strict`
3. `cmd_validate_dir` (line 3252): pass `strict`
4. Main dispatch (line 5438-5446): extract `strict` from CLI

Update `cmd_validate` signature:
```rust
fn cmd_validate(cli: &Cli, csv: bool, strict: bool) {
    if cli.file.len() <= 1 {
        cmd_validate_single(cli, cli.file.first().map(|s| s.as_str()), csv, strict);
    } else {
        cmd_validate_files(cli, csv, strict);
    }
}
```

Update `cmd_validate_single`:
```rust
fn cmd_validate_single(cli: &Cli, file_path: Option<&str>, csv: bool, strict: bool) {
    let sql = read_input(file_path);
    let (stmts, errors, pkg_errors, var_errors) = validate_sql(&sql, cli.mybatis, &[], strict);
```

Update `cmd_validate_files`: thread `strict` to `validate_sql` calls.

Update `cmd_validate_dir`: thread `strict` to internal validate calls.

Update main dispatch (line 5438):
```rust
Commands::Validate { ref dir, ref ext, csv, stats, strict } => {
    if !dir.is_empty() && !cli.file.is_empty() {
        die!("Error: --dir and -f are mutually exclusive");
    }
    if !dir.is_empty() {
        cmd_validate_dir(&cli, dir, ext, csv, stats, strict);
    } else {
        cmd_validate(&cli, csv, strict);
    }
}
```

**Step 5: Run `cargo check`**

Run: `cargo check`
Expected: Compiles successfully.

---

### Task 3: Implement function name validation in `check_expr` (core logic)

**Files:**
- Modify: `src/analyzer/mod.rs:1929-1945` (FunctionCall branch)

**Step 1: Modify the `FunctionCall` branch in `check_expr`**

Replace lines 1929-1945 with:

```rust
Expr::FunctionCall { name, args, agg_from, builtin, .. } => {
    // ── strict mode: validate function name existence ──
    if self.strict {
        if let Some(fname) = name.last() {
            let is_known = builtin.is_some()
                || is_known_function(fname)
                || self.is_known_func(fname)
                || is_pl_builtin(fname);
            if !is_known {
                self.errors.push(UndefinedVariableError {
                    variable_name: format!("函数 '{}' 未定义", fname),
                    location: self.current_span.clone(),
                    context: context.to_string(),
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
```

**Key design decisions:**
- `builtin.is_some()` is the fast path — already resolved during parsing
- `is_known_function(fname)` checks the function registry (430 functions)
- `self.is_known_func(fname)` checks user/package-defined routines
- `is_pl_builtin(fname)` checks PL built-in values (SYSDATE etc.)
- Only runs when `self.strict` is true — zero overhead in default mode

**Step 2: Run `cargo check`**

Run: `cargo check`
Expected: Compiles successfully.

**Step 3: Run all tests**

Run: `cargo test --lib`
Expected: All 1252+ tests pass.

---

### Task 4: Write tests for strict mode

**Files:**
- Modify: `src/analyzer/mod.rs` (add test module entries at end of existing tests)

**Step 1: Add strict mode test cases**

Add these tests in the `#[cfg(test)]` module in `src/analyzer/mod.rs`:

```rust
// ── Strict mode tests ──

#[test]
fn test_strict_mode_detects_undefined_function() {
    use crate::ast::{PlBlock, PlStatement, Expr, PlAssignment};
    // Simulate: BEGIN x := unknown_func(1); END;
    // The FunctionCall with name "unknown_func" should be flagged in strict mode
    let sql = "CREATE OR REPLACE PROCEDURE test_strict AS $$\n\
               BEGIN\n\
                   result := unknown_func(1);\n\
               END; $$ LANGUAGE plpgsql";
    let tokens = crate::Tokenizer::new(sql).tokenize().unwrap();
    let stmts = crate::Parser::new(tokens).parse().unwrap();
    use crate::ast::Statement;
    for si in &stmts {
        if let Statement::CreateProcedure(proc) = &si.statement {
            if let Some(ref block) = proc.block {
                // Non-strict: should NOT flag unknown_func
                let non_strict = validate_pl_variables_with_extra_vars_and_funcs(
                    block, &proc.parameters, &[], &[], false
                );
                let func_errors: Vec<_> = non_strict.iter()
                    .filter(|e| e.variable_name.contains("未定义") || e.variable_name.contains("unknown_func"))
                    .collect();
                assert!(func_errors.is_empty(), "non-strict should not flag function calls: {:?}", func_errors);

                // Strict: should flag unknown_func
                let strict = validate_pl_variables_with_extra_vars_and_funcs(
                    block, &proc.parameters, &[], &[], true
                );
                let func_errors: Vec<_> = strict.iter()
                    .filter(|e| e.variable_name.contains("未定义") || e.variable_name.contains("unknown_func"))
                    .collect();
                assert!(!func_errors.is_empty(), "strict mode should flag unknown_func");
            }
        }
    }
}

#[test]
fn test_strict_mode_allows_known_functions() {
    // Built-in functions like abs(), count(), now() should NOT be flagged
    let sql = "CREATE OR REPLACE PROCEDURE test_known AS $$\n\
               BEGIN\n\
                   result := abs(-5);\n\
                   cnt := count(*);\n\
                   t := now();\n\
               END; $$ LANGUAGE plpgsql";
    let tokens = crate::Tokenizer::new(sql).tokenize().unwrap();
    let stmts = crate::Parser::new(tokens).parse().unwrap();
    use crate::ast::Statement;
    for si in &stmts {
        if let Statement::CreateProcedure(proc) = &si.statement {
            if let Some(ref block) = proc.block {
                let strict = validate_pl_variables_with_extra_vars_and_funcs(
                    block, &proc.parameters, &[], &[], true
                );
                let func_errors: Vec<_> = strict.iter()
                    .filter(|e| e.variable_name.contains("未定义"))
                    .collect();
                assert!(func_errors.is_empty(), "known functions should not be flagged: {:?}", func_errors);
            }
        }
    }
}

#[test]
fn test_strict_mode_allows_user_defined_functions() {
    // Functions passed as extra_funcs should NOT be flagged
    let sql = "CREATE OR REPLACE PROCEDURE test_user_func AS $$\n\
               BEGIN\n\
                   result := my_custom_func(1);\n\
               END; $$ LANGUAGE plpgsql";
    let tokens = crate::Tokenizer::new(sql).tokenize().unwrap();
    let stmts = crate::Parser::new(tokens).parse().unwrap();
    use crate::ast::Statement;
    for si in &stmts {
        if let Statement::CreateProcedure(proc) = &si.statement {
            if let Some(ref block) = proc.block {
                let strict = validate_pl_variables_with_extra_vars_and_funcs(
                    block, &proc.parameters, &[], &["my_custom_func"], true
                );
                let func_errors: Vec<_> = strict.iter()
                    .filter(|e| e.variable_name.contains("my_custom_func") || e.variable_name.contains("未定义"))
                    .collect();
                assert!(func_errors.is_empty(), "user-defined functions should not be flagged: {:?}", func_errors);
            }
        }
    }
}
```

**Step 2: Run the new tests**

Run: `cargo test --lib test_strict`
Expected: All 3 tests pass.

**Step 3: Run full test suite**

Run: `cargo test --lib`
Expected: All tests pass.

---

### Task 5: Update JSON output to include strict mode info

**Files:**
- Modify: `src/bin/ogsql.rs` (validate JSON output section, around line 3082)

**Step 1: Add `strict` field to JSON output**

In the JSON output section of `cmd_validate_single`, add a `strict` field:

Find where the JSON output is built (around line 3082-3100) and add:
```rust
if strict {
    out.as_object_mut().unwrap().insert(
        "strict_mode".to_string(),
        serde_json::json!(true),
    );
}
```

This is optional but helps API consumers know strict mode was active.

**Step 2: Run `cargo check`**

Run: `cargo check`
Expected: Compiles.

---

### Task 6: Final verification

**Step 1: Run full test suite**

Run: `cargo test`
Expected: All tests pass.

**Step 2: Manual CLI test**

```bash
# Non-strict (default) - should pass
echo "CREATE OR REPLACE PROCEDURE test AS \$\$ BEGIN result := unknown_func(1); END; \$\$ LANGUAGE plpgsql" | cargo run -- validate

# Strict - should report undefined function
echo "CREATE OR REPLACE PROCEDURE test AS \$\$ BEGIN result := unknown_func(1); END; \$\$ LANGUAGE plpgsql" | cargo run -- validate --strict

# Strict with known function - should pass
echo "CREATE OR REPLACE PROCEDURE test AS \$\$ BEGIN result := abs(-5); END; \$\$ LANGUAGE plpgsql" | cargo run -- validate --strict
```

**Step 3: Commit**

```bash
git add -A
git commit -m "feat(validate): add --strict mode for undefined function detection (#181)"
```
