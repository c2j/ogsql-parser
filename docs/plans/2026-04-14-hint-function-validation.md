# Hint & Function Validation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add structured hint parsing with validation warnings, fix DML formatter hint loss, and add basic function call validation for GaussDB SQL.

**Architecture:** Build a `HintValidator` that parses opaque hint strings into structured types, validates against the 53 known GaussDB hints, and emits `ParserError::Warning` for unknown or malformed hints. Fix the 4 DML formatters (INSERT/UPDATE/DELETE/MERGE) to emit hints. Add a basic `FunctionValidator` that checks known GaussDB function argument constraints (e.g., aggregate functions require exactly 1 arg, window functions need OVER).

**Tech Stack:** Rust, existing parser infrastructure (ParserError::Warning, add_error), existing `sql_plan_hints.json` as reference data.

---

## Task 1: Fix DML Formatter Hint Loss (🔴 Critical)

**Files:**
- Modify: `src/formatter.rs:1159-1200` (format_insert)
- Modify: `src/formatter.rs:1259-1305` (format_update)
- Modify: `src/formatter.rs:1307-1336` (format_delete)
- Modify: `src/formatter.rs:1338-1367` (format_merge)
- Test: `src/parser/tests.rs`

**Why:** INSERT/UPDATE/DELETE/MERGE formatters silently drop hints during SQL→JSON→SQL round-trip. This is a data loss bug.

**Step 1: Write failing tests for DML hint round-trip**

Add tests in `src/parser/tests.rs`:

```rust
#[test]
fn test_insert_hint_roundtrip() {
    let sql = "INSERT /*+ set(enable_nestloop off) */ INTO t1 (c1) VALUES (1)";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    let formatter = SqlFormatter::new();
    let output = formatter.format_statement(&stmts[0]);
    assert!(output.contains("/*+"), "INSERT hint should be preserved in formatter output: {}", output);
}

#[test]
fn test_update_hint_roundtrip() {
    let sql = "UPDATE /*+ nestloop(t1) */ t1 SET c1 = 1 WHERE c1 > 0";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    let formatter = SqlFormatter::new();
    let output = formatter.format_statement(&stmts[0]);
    assert!(output.contains("/*+"), "UPDATE hint should be preserved in formatter output: {}", output);
}

#[test]
fn test_delete_hint_roundtrip() {
    let sql = "DELETE /*+ indexscan(t1 idx_c1) */ FROM t1 WHERE c1 > 0";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    let formatter = SqlFormatter::new();
    let output = formatter.format_statement(&stmts[0]);
    assert!(output.contains("/*+"), "DELETE hint should be preserved in formatter output: {}", output);
}

#[test]
fn test_merge_hint_roundtrip() {
    let sql = "MERGE /*+ leading(t1 t2) */ INTO t1 USING t2 ON t1.id = t2.id WHEN MATCHED THEN UPDATE SET t1.val = t2.val";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    let formatter = SqlFormatter::new();
    let output = formatter.format_statement(&stmts[0]);
    assert!(output.contains("/*+"), "MERGE hint should be preserved in formatter output: {}", output);
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test test_insert_hint_roundtrip test_update_hint_roundtrip test_delete_hint_roundtrip test_merge_hint_roundtrip`
Expected: All 4 tests FAIL because formatted output doesn't contain `/*+`.

**Step 3: Add helper method and fix formatters**

Add a private helper method to `SqlFormatter`:

```rust
fn format_hints(&self, hints: &[String]) -> Option<String> {
    if hints.is_empty() {
        return None;
    }
    let formatted: Vec<String> = hints.iter().map(|h| format!("/*+ {} */", h)).collect();
    Some(formatted.join(" "))
}
```

Then fix each formatter:

`format_insert` — change from:
```rust
fn format_insert(&self, stmt: &InsertStatement) -> String {
    let mut parts = vec![self.kw("INSERT INTO")];
```
to:
```rust
fn format_insert(&self, stmt: &InsertStatement) -> String {
    let mut parts = Vec::new();
    if let Some(hints) = self.format_hints(&stmt.hints) {
        parts.push(hints);
    }
    parts.push(self.kw("INSERT INTO"));
```

`format_update` — change from:
```rust
fn format_update(&self, stmt: &UpdateStatement) -> String {
    let mut parts = vec![self.kw("UPDATE")];
```
to:
```rust
fn format_update(&self, stmt: &UpdateStatement) -> String {
    let mut parts = Vec::new();
    if let Some(hints) = self.format_hints(&stmt.hints) {
        parts.push(hints);
    }
    parts.push(self.kw("UPDATE"));
```

`format_delete` — change from:
```rust
fn format_delete(&self, stmt: &DeleteStatement) -> String {
    let mut parts = vec![self.kw("DELETE FROM")];
```
to:
```rust
fn format_delete(&self, stmt: &DeleteStatement) -> String {
    let mut parts = Vec::new();
    if let Some(hints) = self.format_hints(&stmt.hints) {
        parts.push(hints);
    }
    parts.push(self.kw("DELETE FROM"));
```

`format_merge` — change from:
```rust
fn format_merge(&self, stmt: &MergeStatement) -> String {
    let mut parts = vec![self.kw("MERGE INTO")];
```
to:
```rust
fn format_merge(&self, stmt: &MergeStatement) -> String {
    let mut parts = Vec::new();
    if let Some(hints) = self.format_hints(&stmt.hints) {
        parts.push(hints);
    }
    parts.push(self.kw("MERGE INTO"));
```

**Step 4: Run tests to verify they pass**

Run: `cargo test test_insert_hint_roundtrip test_update_hint_roundtrip test_delete_hint_roundtrip test_merge_hint_roundtrip`
Expected: All 4 tests PASS.

**Step 5: Run full test suite**

Run: `cargo test`
Expected: All existing tests still pass.

**Step 6: Commit**

```bash
git add src/formatter.rs src/parser/tests.rs
git commit -m "fix: preserve hints in INSERT/UPDATE/DELETE/MERGE formatter output"
```

---

## Task 2: Add Hint Validation Infrastructure

**Files:**
- Create: `src/parser/hint_validator.rs`
- Modify: `src/parser/mod.rs` (add module, integrate validation)

**Why:** Currently all hint content is opaque strings with no validation. `/*+ garbage(xyz) */` parses silently. We need to parse hint content and emit warnings for unknown/malformed hints.

**Step 1: Create hint validator module**

Create `src/parser/hint_validator.rs`:

```rust
//! GaussDB SQL Plan Hint Validator
//!
//! Parses opaque hint strings into structured hint types and validates
//! against the 53 documented GaussDB plan hints.

use crate::parser::ParserError;
use crate::token::SourceLocation;

/// All known GaussDB hint names (lowercase), organized by category.
pub const KNOWN_HINTS: &[&str] = &[
    // Scan hints
    "tablescan", "indexscan", "indexonlyscan", "gsi", "gsitable", "bitmapscan",
    // Join method hints
    "nestloop", "hashjoin", "mergejoin",
    // Join order hints
    "leading",
    // Stream hints
    "broadcast", "redistribute", "gather",
    // Aggregation hints
    "use_hash_agg", "use_sort_agg",
    // GUC parameter hints
    "set",
    // Plan selection hints
    "use_cplan", "use_gplan", "choose_adaptive_gplan",
    // Global cache hints
    "no_gpc",
    // Materialization hints
    "material_subplan",
    // Outline hints
    "begin_outline_data", "end_outline_data",
    // Query block hints
    "blockname",
    // WLM hints
    "wlmrule",
    // Query rewrite hints (with NO_ variants)
    "expand_sublink", "no_expand_sublink",
    "expand_sublink_having", "no_expand_sublink_having",
    "expand_sublink_target", "no_expand_sublink_target",
    "expand_sublink_unique_check", "no_expand_sublink_unique_check",
    "sublink_disable_replicated", "no_sublink_disable_replicated",
    "sublink_disable_expr", "no_sublink_disable_expr",
    "enable_sublink_enhanced", "no_enable_sublink_enhanced",
    "use_magic_set", "no_use_magic_set",
    "partial_push", "no_partial_push",
    "reduce_order_by", "no_reduce_order_by",
    "remove_not_null", "no_remove_not_null",
    "lazy_agg", "no_lazy_agg",
    "expand_subquery", "no_expand_subquery",
    "pushdown_having", "no_pushdown_having",
    "inlist_to_join", "no_inlist_to_join",
    "rownum_pushdown", "no_rownum_pushdown",
    // Additional hints found in documentation but not in sql_plan_hints.json
    "rows", "no_expand", "predpush_same_level", "nestloop_index",
    "skew", "no_skew",
    // Parameterized path hints
    "index_paramsel",
];

/// Hints that require parenthesized arguments
const HINTS_WITH_PARENS: &[&str] = &[
    "tablescan", "indexscan", "indexonlyscan", "gsi", "gsitable", "bitmapscan",
    "nestloop", "hashjoin", "mergejoin",
    "leading",
    "broadcast", "redistribute",
    "set",
    "blockname",
    "wlmrule",
    "rows",
    "predpush_same_level", "nestloop_index",
    "skew", "no_skew",
    "inlist_to_join",
    "index_paramsel",
];

/// Hints that support the `no` prefix negation
const HINTS_WITH_NO_PREFIX: &[&str] = &[
    "tablescan", "indexscan", "indexonlyscan", "gsi", "gsitable", "bitmapscan",
    "nestloop", "hashjoin", "mergejoin",
    "broadcast", "redistribute",
];

/// Validate a raw hint string content and return any warnings.
///
/// A single hint string may contain multiple hints separated by whitespace,
/// e.g. "tablescan(t1) leading(t1 t2)".
pub fn validate_hints(hint_content: &str, location: SourceLocation) -> Vec<ParserError> {
    let mut warnings = Vec::new();
    
    // Parse individual hints from the content
    let hints = parse_hint_list(hint_content);
    
    for hint in &hints {
        let hint_warnings = validate_single_hint(hint, location.clone());
        warnings.extend(hint_warnings);
    }
    
    warnings
}

/// A single parsed hint with its name and optional arguments.
#[derive(Debug, Clone)]
pub struct ParsedHint {
    /// The hint name (lowercase), e.g. "tablescan", "no hashjoin"
    pub name: String,
    /// Whether the `no` prefix was present
    pub negated: bool,
    /// Optional queryblock specification, e.g. "@sel$1"
    pub queryblock: Option<String>,
    /// Arguments inside parentheses, e.g. "t1" or "t1 t2"
    pub args: Option<String>,
    /// The raw text of this hint
    pub raw: String,
}

/// Parse a hint content string into individual hint structures.
fn parse_hint_list(content: &str) -> Vec<ParsedHint> {
    let mut hints = Vec::new();
    let mut chars = content.chars().peekable();
    let mut pos = 0;
    
    while pos < content.len() {
        // Skip whitespace
        while let Some(c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
                pos += 1;
            } else {
                break;
            }
        }
        
        if pos >= content.len() {
            break;
        }
        
        // Read hint name (until '(' or whitespace)
        let start = pos;
        let mut name = String::new();
        while let Some(c) = chars.peek() {
            if *c == '(' || c.is_whitespace() || *c == '@' {
                break;
            }
            name.push(chars.next().unwrap());
            pos += 1;
        }
        
        if name.is_empty() {
            break;
        }
        
        // Check for @queryblock
        let mut queryblock = None;
        if chars.peek() == Some(&'@') {
            chars.next();
            pos += 1;
            let mut qb = String::from("@");
            while let Some(c) = chars.peek() {
                if *c == '(' || c.is_whitespace() {
                    break;
                }
                qb.push(chars.next().unwrap());
                pos += 1;
            }
            queryblock = Some(qb);
        }
        
        // Check for parenthesized arguments
        let mut args = None;
        if chars.peek() == Some(&'(') {
            chars.next();
            pos += 1;
            let mut arg_content = String::new();
            let mut depth = 1;
            while depth > 0 {
                match chars.next() {
                    Some('(') => { depth += 1; arg_content.push('('); pos += 1; }
                    Some(')') => {
                        depth -= 1;
                        if depth > 0 { arg_content.push(')'); }
                        pos += 1;
                    }
                    Some(c) => { arg_content.push(c); pos += 1; }
                    None => break,
                }
            }
            args = Some(arg_content);
        }
        
        let raw = content[start..pos].to_string();
        
        // Check for no prefix
        let (actual_name, negated) = if let Some(rest) = name.strip_prefix("no ") {
            (rest.trim().to_string(), true)
        } else if let Some(rest) = name.strip_prefix("no_") {
            // Some hints use no_ prefix as part of the name (like no_expand)
            // Check if the full name with no_ is a known hint
            let full_lower = name.to_lowercase();
            if KNOWN_HINTS.contains(&full_lower.as_str()) {
                (name.to_lowercase(), false)
            } else if let Some(base) = full_lower.strip_prefix("no_") {
                (base.to_string(), true)
            } else {
                (name.to_lowercase(), false)
            }
        } else {
            (name.to_lowercase(), false)
        };
        
        hints.push(ParsedHint {
            name: actual_name,
            negated,
            queryblock,
            args,
            raw,
        });
    }
    
    hints
}

/// Validate a single parsed hint.
fn validate_single_hint(hint: &ParsedHint, location: SourceLocation) -> Vec<ParserError> {
    let mut warnings = Vec::new();
    let name_lower = hint.name.to_lowercase();
    
    // Check if hint name is known
    let is_known = KNOWN_HINTS.contains(&name_lower.as_str());
    
    if !is_known {
        warnings.push(ParserError::Warning {
            message: format!(
                "Unknown hint '{}' — not in GaussDB 2.23.07.210 documented hint list",
                hint.name
            ),
            location: location.clone(),
        });
        return warnings;
    }
    
    // Validate `no` prefix usage
    if hint.negated && !HINTS_WITH_NO_PREFIX.contains(&name_lower.as_str()) {
        warnings.push(ParserError::Warning {
            message: format!(
                "Hint 'no {}' does not support the 'no' prefix negation",
                hint.name
            ),
            location: location.clone(),
        });
    }
    
    // Validate parentheses requirement
    let requires_parens = HINTS_WITH_PARENS.contains(&name_lower.as_str());
    if requires_parens && hint.args.is_none() {
        warnings.push(ParserError::Warning {
            message: format!(
                "Hint '{}' requires parenthesized arguments, e.g. {}(table_name)",
                hint.name, hint.name
            ),
            location: location.clone(),
        });
    }
    
    // Validate hints that should NOT have parentheses
    if !requires_parens && hint.args.is_some() {
        let no_paren_hints = [
            "use_hash_agg", "use_sort_agg", "use_cplan", "use_gplan",
            "choose_adaptive_gplan", "no_gpc", "material_subplan",
            "begin_outline_data", "end_outline_data",
            "no_expand", "expand_sublink", "no_expand_sublink",
            "expand_sublink_having", "no_expand_sublink_having",
            "expand_sublink_target", "no_expand_sublink_target",
            "expand_sublink_unique_check", "no_expand_sublink_unique_check",
            "sublink_disable_replicated", "no_sublink_disable_replicated",
            "sublink_disable_expr", "no_sublink_disable_expr",
            "enable_sublink_enhanced", "no_enable_sublink_enhanced",
            "use_magic_set", "no_use_magic_set",
            "partial_push", "no_partial_push",
            "reduce_order_by", "no_reduce_order_by",
            "remove_not_null", "no_remove_not_null",
            "lazy_agg", "no_lazy_agg",
            "expand_subquery", "no_expand_subquery",
            "pushdown_having", "no_pushdown_having",
            "rownum_pushdown", "no_rownum_pushdown",
        ];
        if no_paren_hints.contains(&name_lower.as_str()) {
            warnings.push(ParserError::Warning {
                message: format!(
                    "Hint '{}' does not take parenthesized arguments",
                    hint.name
                ),
                location: location.clone(),
            });
        }
    }
    
    // Validate wlmrule special format: wlmrule("time_limit,max_execute_time,max_iops")
    if name_lower == "wlmrule" {
        if let Some(ref args) = hint.args {
            if !args.starts_with('"') || !args.ends_with('"') {
                warnings.push(ParserError::Warning {
                    message: "Hint 'wlmrule' requires quoted argument: wlmrule(\"time_limit,max_execute_time,max_iops\")".to_string(),
                    location: location.clone(),
                });
            }
        }
    }
    
    // Validate set hint: set(param value)
    if name_lower == "set" {
        if let Some(ref args) = hint.args {
            let parts: Vec<&str> = args.split_whitespace().collect();
            if parts.len() < 2 {
                warnings.push(ParserError::Warning {
                    message: "Hint 'set' requires parameter name and value: set(param_name value)".to_string(),
                    location: location.clone(),
                });
            }
        }
    }
    
    // Validate leading hint: leading(@qb? table_list or (t1 t2) t3)
    if name_lower == "leading" {
        if let Some(ref args) = hint.args {
            let trimmed = args.trim();
            if trimmed.is_empty() {
                warnings.push(ParserError::Warning {
                    message: "Hint 'leading' requires at least one table name".to_string(),
                    location: location.clone(),
                });
            }
            // Check for unbalanced parentheses
            let open_count = trimmed.chars().filter(|c| *c == '(').count();
            let close_count = trimmed.chars().filter(|c| *c == ')').count();
            if open_count != close_count {
                warnings.push(ParserError::Warning {
                    message: "Hint 'leading' has unbalanced parentheses in table list".to_string(),
                    location: location.clone(),
                });
            }
        }
    }
    
    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_loc() -> SourceLocation {
        SourceLocation { line: 1, column: 1 }
    }

    #[test]
    fn test_parse_simple_hint() {
        let hints = parse_hint_list("tablescan(t1)");
        assert_eq!(hints.len(), 1);
        assert_eq!(hints[0].name, "tablescan");
        assert_eq!(hints[0].args.as_deref(), Some("t1"));
        assert!(!hints[0].negated);
    }

    #[test]
    fn test_parse_multiple_hints() {
        let hints = parse_hint_list("tablescan(t1) leading(t1 t2)");
        assert_eq!(hints.len(), 2);
        assert_eq!(hints[0].name, "tablescan");
        assert_eq!(hints[1].name, "leading");
    }

    #[test]
    fn test_parse_negated_hint() {
        let hints = parse_hint_list("no hashjoin(t1 t2)");
        assert_eq!(hints.len(), 1);
        assert_eq!(hints[0].name, "hashjoin");
        assert!(hints[0].negated);
    }

    #[test]
    fn test_parse_queryblock_hint() {
        let hints = parse_hint_list("tablescan(@sel$1 t1)");
        assert_eq!(hints.len(), 1);
        assert_eq!(hints[0].queryblock.as_deref(), Some("@sel$1"));
        assert_eq!(hints[0].args.as_deref(), Some("t1"));
    }

    #[test]
    fn test_validate_known_hint() {
        let warnings = validate_hints("tablescan(t1)", default_loc());
        assert!(warnings.is_empty(), "Known hint should produce no warnings");
    }

    #[test]
    fn test_validate_unknown_hint() {
        let warnings = validate_hints("invalid_hint(t1)", default_loc());
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].to_string().contains("Unknown hint"));
    }

    #[test]
    fn test_validate_missing_parens() {
        let warnings = validate_hints("tablescan", default_loc());
        assert!(warnings.iter().any(|w| w.to_string().contains("requires parenthesized")));
    }

    #[test]
    fn test_validate_set_hint() {
        let warnings = validate_hints("set(enable_hashjoin off)", default_loc());
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_validate_set_hint_missing_value() {
        let warnings = validate_hints("set(enable_hashjoin)", default_loc());
        assert!(warnings.iter().any(|w| w.to_string().contains("parameter name and value")));
    }

    #[test]
    fn test_validate_wlmrule() {
        let warnings = validate_hints("wlmrule(\"100,500,1\")", default_loc());
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_validate_wlmrule_bad_format() {
        let warnings = validate_hints("wlmrule(100,500,1)", default_loc());
        assert!(warnings.iter().any(|w| w.to_string().contains("quoted argument")));
    }

    #[test]
    fn test_validate_no_prefix_on_non_negatable() {
        let warnings = validate_hints("no leading(t1 t2)", default_loc());
        assert!(warnings.iter().any(|w| w.to_string().contains("does not support")));
    }
}
```

**Step 2: Run hint_validator tests**

Run: `cargo test hint_validator`
Expected: All tests PASS.

**Step 3: Integrate hint validator into parser**

Modify `src/parser/mod.rs`:
- Add `pub(crate) mod hint_validator;` at the top
- In `consume_hints()`, call validation after collecting hints:

```rust
pub(crate) fn consume_hints(&mut self) -> Vec<String> {
    let mut hints = Vec::new();
    let loc = self.current_location();
    while let Token::Hint(h) = self.peek().clone() {
        // Validate hint content and collect warnings
        let warnings = hint_validator::validate_hints(&h, loc.clone());
        for w in warnings {
            self.add_error(w);
        }
        hints.push(h);
        self.advance();
    }
    hints
}
```

**Step 4: Run full test suite**

Run: `cargo test`
Expected: All existing tests still pass.

**Step 5: Commit**

```bash
git add src/parser/hint_validator.rs src/parser/mod.rs
git commit -m "feat: add hint validation with warnings for unknown/malformed GaussDB hints"
```

---

## Task 3: Add Basic Function Call Validation

**Files:**
- Create: `src/parser/function_validator.rs`
- Modify: `src/parser/expr.rs` (integrate validation)

**Why:** Certain function categories have argument constraints that should be validated. For example, aggregate functions like `COUNT` require exactly one argument (or `*`), `COALESCE` requires at least 2 arguments, window functions require OVER clause when used as window functions.

**Step 1: Create function validator module**

Create `src/parser/function_validator.rs`:

```rust
//! Basic GaussDB function call validation.
//!
//! Validates argument counts and basic constraints for known GaussDB functions,
//! emitting warnings for incorrect usage.

use crate::parser::ParserError;
use crate::token::SourceLocation;

/// Functions that require at least 2 arguments
const MIN_2_ARGS: &[&str] = &[
    "coalesce", "nullif", "nvl", "nvl2", "concat", "concat_ws",
    "greatest", "least", "decode",
];

/// Functions that require exactly 1 argument (not counting DISTINCT)
const EXACTLY_1_ARG: &[&str] = &[
    "count", "sum", "avg", "min", "max",
    "stddev", "stddev_pop", "stddev_samp",
    "variance", "var_pop", "var_samp",
    "bit_and", "bit_or",
    "every",
];

/// Functions that take 0 arguments
const ZERO_ARGS: &[&str] = &[
    "now", "current_timestamp", "current_date", "current_time",
    "localtime", "localtimestamp", "clock_timestamp",
    "statement_timestamp", "transaction_timestamp",
    "pi", "version", "pg_backend_pid",
    "currval", "lastval",
];

/// Functions that require exactly 2 arguments
const EXACTLY_2_ARGS: &[&str] = &[
    "nullif", "nvl", "round", "trunc", "power",
    "mod", "div", "atan2",
    "substr", "substring", "left", "right",
    "repeat", "replace", "split_part",
    "lpad", "rpad", "instr",
    "to_char", "to_date", "to_number", "to_timestamp",
];

/// Window functions that should have OVER clause
const WINDOW_FUNCTIONS: &[&str] = &[
    "row_number", "rank", "dense_rank", "percent_rank",
    "cume_dist", "ntile", "lag", "lead",
    "first_value", "last_value", "nth_value",
    "ratio_to_report",
];

/// Aggregate functions that support DISTINCT
const DISTINCT_AGGREGATES: &[&str] = &[
    "count", "sum", "avg", "min", "max",
    "array_agg", "string_agg", "group_concat", "wm_concat",
    "corr", "covar_pop", "covar_samp",
    "regr_avgx", "regr_avgy", "regr_count",
    "regr_intercept", "regr_r2", "regr_slope",
    "stddev", "stddev_pop", "stddev_samp",
    "variance", "var_pop", "var_samp",
];

/// Validate a function call.
///
/// # Parameters
/// - `name`: Function name (lowercase)
/// - `arg_count`: Number of arguments (excluding DISTINCT keyword, including *)
/// - `has_distinct`: Whether DISTINCT modifier was used
/// - `has_over`: Whether OVER clause is present
/// - `location`: Source location for error reporting
pub fn validate_function_call(
    name: &str,
    arg_count: usize,
    has_distinct: bool,
    has_over: bool,
    location: SourceLocation,
) -> Vec<ParserError> {
    let mut warnings = Vec::new();
    let name_lower = name.to_lowercase();
    
    // Validate DISTINCT usage
    if has_distinct && !DISTINCT_AGGREGATES.contains(&name_lower.as_str()) {
        warnings.push(ParserError::Warning {
            message: format!(
                "Function '{}' does not support DISTINCT modifier",
                name
            ),
            location: location.clone(),
        });
    }
    
    // Validate argument counts
    if ZERO_ARGS.contains(&name_lower.as_str()) && arg_count > 0 {
        warnings.push(ParserError::Warning {
            message: format!(
                "Function '{}' takes no arguments but {} were provided",
                name, arg_count
            ),
            location: location.clone(),
        });
    }
    
    if EXACTLY_1_ARG.contains(&name_lower.as_str()) && arg_count != 1 {
        // COUNT(*) is allowed (arg_count could be 1 for star)
        if !(name_lower == "count" && arg_count == 1) {
            warnings.push(ParserError::Warning {
                message: format!(
                    "Function '{}' requires exactly 1 argument but {} were provided",
                    name, arg_count
                ),
                location: location.clone(),
            });
        }
    }
    
    if MIN_2_ARGS.contains(&name_lower.as_str()) && arg_count < 2 {
        warnings.push(ParserError::Warning {
            message: format!(
                "Function '{}' requires at least 2 arguments but {} were provided",
                name, arg_count
            ),
            location: location.clone(),
        });
    }
    
    // Validate window function has OVER clause
    if WINDOW_FUNCTIONS.contains(&name_lower.as_str()) && !has_over {
        warnings.push(ParserError::Warning {
            message: format!(
                "Window function '{}' should have an OVER clause",
                name
            ),
            location: location.clone(),
        });
    }
    
    // Validate OVER clause used only with window/aggregate functions
    // (This is informational only since any aggregate can be used as window function)
    
    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_loc() -> SourceLocation {
        SourceLocation { line: 1, column: 1 }
    }

    #[test]
    fn test_coalesce_too_few_args() {
        let warnings = validate_function_call("coalesce", 1, false, false, default_loc());
        assert!(warnings.iter().any(|w| w.to_string().contains("at least 2")));
    }

    #[test]
    fn test_coalesce_ok() {
        let warnings = validate_function_call("coalesce", 2, false, false, default_loc());
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_count_no_args() {
        let warnings = validate_function_call("count", 0, false, false, default_loc());
        assert!(warnings.iter().any(|w| w.to_string().contains("exactly 1")));
    }

    #[test]
    fn test_window_func_no_over() {
        let warnings = validate_function_call("row_number", 0, false, false, default_loc());
        assert!(warnings.iter().any(|w| w.to_string().contains("OVER clause")));
    }

    #[test]
    fn test_window_func_with_over() {
        let warnings = validate_function_call("row_number", 0, false, true, default_loc());
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_distinct_on_non_aggregate() {
        let warnings = validate_function_call("upper", 1, true, false, default_loc());
        assert!(warnings.iter().any(|w| w.to_string().contains("DISTINCT")));
    }

    #[test]
    fn test_distinct_on_aggregate() {
        let warnings = validate_function_call("count", 1, true, false, default_loc());
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_now_with_args() {
        let warnings = validate_function_call("now", 1, false, false, default_loc());
        assert!(warnings.iter().any(|w| w.to_string().contains("no arguments")));
    }
}
```

**Step 2: Run function_validator tests**

Run: `cargo test function_validator`
Expected: All tests PASS.

**Step 3: Integrate function validator into expr.rs parser**

In `src/parser/expr.rs`, at the end of `parse_function_call()` (where the generic `Expr::FunctionCall` is constructed), add validation:

After the `FunctionCall` struct is constructed (around line 800+), add:

```rust
// Validate function call arguments
{
    let func_name = name.to_string().to_lowercase();
    let last = func_name.split('.').last().unwrap_or(&func_name);
    let func_warnings = crate::parser::function_validator::validate_function_call(
        last,
        args.len(),
        distinct,
        over.is_some(),
        self.current_location(),
    );
    for w in func_warnings {
        self.add_error(w);
    }
}
```

**Step 4: Run full test suite**

Run: `cargo test`
Expected: All existing tests still pass. Some tests may now produce warnings (which is expected and desired).

**Step 5: Commit**

```bash
git add src/parser/function_validator.rs src/parser/expr.rs
git commit -m "feat: add function call validation with warnings for argument constraint violations"
```

---

## Task 4: Add Comprehensive Hint Tests

**Files:**
- Modify: `src/parser/tests.rs`

**Why:** Currently there are zero hint-specific tests. Need coverage for: basic hint parsing, multi-hint, queryblock syntax, hint on each DML type, hint round-trip, hint validation warnings.

**Step 1: Add hint parsing and validation tests**

```rust
// ── Hint Tests ──

#[test]
fn test_select_with_hint() {
    let sql = "SELECT /*+ tablescan(t1) */ * FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    match &stmts[0] {
        Statement::Select(s) => assert_eq!(s.hints, vec!["tablescan(t1)"]),
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_select_with_multiple_hints() {
    let sql = "SELECT /*+ tablescan(t1) */ /*+ leading(t1 t2) */ * FROM t1, t2 WHERE t1.id = t2.id";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    match &stmts[0] {
        Statement::Select(s) => {
            assert_eq!(s.hints.len(), 2);
            assert_eq!(s.hints[0], "tablescan(t1)");
            assert_eq!(s.hints[1], "leading(t1 t2)");
        }
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_select_hint_before_keyword() {
    let sql = "/*+ hashjoin(t1 t2) */ SELECT * FROM t1 JOIN t2 ON t1.id = t2.id";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    match &stmts[0] {
        Statement::Select(s) => assert_eq!(s.hints, vec!["hashjoin(t1 t2)"]),
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_hint_with_queryblock() {
    let sql = "SELECT /*+ tablescan(@sel$1 t1) */ * FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    match &stmts[0] {
        Statement::Select(s) => assert_eq!(s.hints, vec!["tablescan(@sel$1 t1)"]),
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_hint_set_guc() {
    let sql = "SELECT /*+ set(enable_hashjoin off) */ * FROM t1 JOIN t2 ON t1.id = t2.id";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    match &stmts[0] {
        Statement::Select(s) => assert_eq!(s.hints, vec!["set(enable_hashjoin off)"]),
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_hint_validation_unknown_hint() {
    let sql = "SELECT /*+ nonexistent_hint(t1) */ * FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let warnings: Vec<_> = parser.errors().iter()
        .filter(|e| matches!(e, ParserError::Warning { .. }))
        .collect();
    assert!(!warnings.is_empty(), "Should have warning for unknown hint");
    assert!(warnings[0].to_string().contains("Unknown hint"));
}

#[test]
fn test_hint_validation_set_missing_value() {
    let sql = "SELECT /*+ set(enable_hashjoin) */ * FROM t1";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    assert!(!stmts.is_empty());
    let warnings: Vec<_> = parser.errors().iter()
        .filter(|e| matches!(e, ParserError::Warning { .. }))
        .collect();
    assert!(!warnings.is_empty(), "Should have warning for malformed set hint");
}

#[test]
fn test_hint_roundtrip_json() {
    let sql = "SELECT /*+ tablescan(t1) leading(t1 t2) */ * FROM t1, t2 WHERE t1.id = t2.id";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    let json = serde_json::to_string(&stmts).unwrap();
    let restored: Vec<Statement> = serde_json::from_str(&json).unwrap();
    let formatter = SqlFormatter::new();
    let output = formatter.format_statement(&restored[0]);
    assert!(output.contains("tablescan(t1)"), "Hint should survive JSON round-trip");
    assert!(output.contains("leading(t1 t2)"), "Hint should survive JSON round-trip");
}
```

**Step 2: Run all tests**

Run: `cargo test test_select_with_hint test_select_with_multiple test_hint_`
Expected: All tests PASS.

**Step 3: Commit**

```bash
git add src/parser/tests.rs
git commit -m "test: add comprehensive hint parsing and validation tests"
```

---

## Task 5: Run Full Regression Suite

**Files:** None (verification only)

**Step 1: Run cargo test**

Run: `cargo test`
Expected: All tests pass. Warnings may appear for some existing test SQL but this is expected.

**Step 2: Run cargo clippy**

Run: `cargo clippy -- -D warnings`
Expected: No clippy errors.

**Step 3: Run cargo build**

Run: `cargo build`
Expected: Clean build with no errors.

**Step 4: Verify with example SQL files**

Run the parser against the GaussDB hint SQL files:
```bash
cargo run --bin ogsql -- validate -f GaussDB-2.23.07.210/sql/by_file/1004_Hint.sql
cargo run --bin ogsql -- validate -f GaussDB-2.23.07.210/sql/by_file/1012_GUCHint.sql
cargo run --bin ogsql -- validate -f GaussDB-2.23.07.210/sql/by_file/2705_Plan Hint.sql
```

Expected: Files parse successfully (may have warnings which is correct behavior).
