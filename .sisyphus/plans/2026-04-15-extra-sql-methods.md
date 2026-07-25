# `--extra-sql-methods` CLI Argument Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Allow users to extend the built-in `SQL_METHOD_UNAMBIGUOUS` list via a `--extra-sql-methods` CLI argument on the `parse-java` subcommand.

**Architecture:** Add a `JavaExtractConfig` struct that holds extra method names, thread it from CLI → public API → `ExtractContext`. The config is optional — when not provided, behavior is identical to the current hardcoded constants. At the match site, extra methods are checked alongside the built-in `SQL_METHOD_UNAMBIGUOUS` slice.

**Tech Stack:** Rust, clap v4 (derive), existing tree-sitter Java extraction module.

---

### Task 1: Add `JavaExtractConfig` struct

**Files:**
- Modify: `src/java/types.rs` (append new struct)

**Step 1: Add the config struct to types.rs**

Add at the end of `src/java/types.rs`:

```rust
/// Configuration for Java SQL extraction.
///
/// Allows extending the built-in method name lists at runtime.
#[derive(Debug, Clone, Default)]
pub struct JavaExtractConfig {
    /// Additional method names to treat as unambiguous SQL carriers
    /// (appended to the built-in `SQL_METHOD_UNAMBIGUOUS` list).
    pub extra_sql_methods: Vec<String>,
}
```

**Step 2: Update `mod.rs` to export `JavaExtractConfig`**

In `src/java/mod.rs`, add `JavaExtractConfig` to the `pub use types::` re-export line (line 12):

```rust
pub use types::{
    ExtractedSql, ExtractionMethod, JavaExtractConfig, JavaExtractResult, ParameterStyle, SqlKind, SqlOrigin,
    SqlParseResult,
};
```

**Step 3: Verify compilation**

Run: `cargo build`
Expected: Clean compilation (no usages yet, just the type definition).

---

### Task 2: Thread config through `extract_sql_from_java` → `extract` → `ExtractContext`

**Files:**
- Modify: `src/java/mod.rs` — change `extract_sql_from_java` signature
- Modify: `src/java/extract.rs` — change `extract` signature and `ExtractContext` struct

**Step 1: Update `extract_sql_from_java` in mod.rs**

Change from:
```rust
pub fn extract_sql_from_java(source: &str, file_path: &str) -> JavaExtractResult {
```
To:
```rust
pub fn extract_sql_from_java(source: &str, file_path: &str, config: &JavaExtractConfig) -> JavaExtractResult {
```

And pass config to extract:
```rust
extract::extract(source, tree.root_node(), file_path, config)
```

Add import at top of mod.rs:
```rust
use types::JavaExtractConfig;
```

**Step 2: Update `extract` function signature in extract.rs**

Change from:
```rust
pub fn extract(source: &str, root: Node, file_path: &str) -> JavaExtractResult {
    let mut ctx = ExtractContext {
        source,
        file_path,
        extractions: Vec::new(),
        errors: Vec::new(),
        class_name: None,
        method_name: None,
        sql_vars: HashMap::new(),
        var_types: HashMap::new(),
    };
```
To:
```rust
pub fn extract(source: &str, root: Node, file_path: &str, config: &JavaExtractConfig) -> JavaExtractResult {
    let mut ctx = ExtractContext {
        source,
        file_path,
        extractions: Vec::new(),
        errors: Vec::new(),
        class_name: None,
        method_name: None,
        sql_vars: HashMap::new(),
        var_types: HashMap::new(),
        extra_sql_methods: &config.extra_sql_methods,
    };
```

Add import at top of extract.rs:
```rust
use super::types::JavaExtractConfig;
```

**Step 3: Add `extra_sql_methods` field to `ExtractContext`**

Add a field to `ExtractContext` struct:
```rust
struct ExtractContext<'a> {
    source: &'a str,
    file_path: &'a str,
    extractions: Vec<ExtractedSql>,
    errors: Vec<JavaError>,
    class_name: Option<String>,
    method_name: Option<String>,
    sql_vars: HashMap<String, TrackedVar>,
    /// Maps variable name to Java type name (e.g., "tableName" to "String")
    var_types: HashMap<String, String>,
    /// Extra unambiguous SQL method names from CLI config.
    extra_sql_methods: &'a [String],
}
```

**Step 4: Update all callers of `extract_sql_from_java`**

In `src/java/tests.rs`, every call to `extract_sql_from_java(java, "...")` needs a third argument. Add a helper at the top of the test file:

```rust
use crate::java::JavaExtractConfig;

/// Default config for tests (no extra methods).
fn default_config() -> JavaExtractConfig {
    JavaExtractConfig::default()
}
```

Then replace every `extract_sql_from_java(java, "X.java")` with `extract_sql_from_java(java, "X.java", &default_config())`.

There are ~20 test functions that call `extract_sql_from_java`. Update ALL of them.

**Step 5: Verify compilation**

Run: `cargo build`
Expected: Clean compilation (CLI not yet updated, but tests should compile).

---

### Task 3: Use `extra_sql_methods` in the method matching logic

**Files:**
- Modify: `src/java/extract.rs` — `visit_method_invocation` method

**Step 1: Update `visit_method_invocation`**

In `visit_method_invocation` (around line 345-362), change the unambiguous check from:

```rust
let is_unambiguous = SQL_METHOD_UNAMBIGUOUS.contains(&method_name.as_str());
```

To:

```rust
let is_unambiguous = SQL_METHOD_UNAMBIGUOUS.contains(&method_name.as_str())
    || self.extra_sql_methods.iter().any(|m| m == &method_name);
```

The rest of the logic remains unchanged — extra methods are treated exactly like the built-in unambiguous ones (no `looks_like_sql()` check needed).

**Step 2: Verify with existing tests**

Run: `cargo test --features java`
Expected: All existing tests pass (extra_sql_methods is empty `&[]` by default, so behavior is identical).

---

### Task 4: Write test for extra_sql_methods

**Files:**
- Modify: `src/java/tests.rs` — add new test

**Step 1: Add test for custom method name**

```rust
#[test]
fn test_extra_sql_methods() {
    let java = r#"
        public class CustomDao {
            public void findUsers() {
                db.findNativeQuery("SELECT * FROM users WHERE active = 1");
            }
        }
    "#;

    // Without extra methods — should NOT extract
    let result = extract_sql_from_java(java, "CustomDao.java", &JavaExtractConfig::default());
    let method_extractions: Vec<_> = result
        .extractions
        .iter()
        .filter(|e| e.origin.method == ExtractionMethod::MethodCall)
        .collect();
    assert!(method_extractions.is_empty(), "Should not extract without extra methods");

    // With extra methods — should extract
    let config = JavaExtractConfig {
        extra_sql_methods: vec!["findNativeQuery".to_string()],
    };
    let result = extract_sql_from_java(java, "CustomDao.java", &config);
    let method_extractions: Vec<_> = result
        .extractions
        .iter()
        .filter(|e| e.origin.method == ExtractionMethod::MethodCall)
        .collect();
    assert_eq!(method_extractions.len(), 1);
    assert_eq!(
        method_extractions[0].origin.api_method_name.as_deref(),
        Some("findNativeQuery")
    );
    assert!(method_extractions[0].sql.contains("SELECT * FROM users"));
}
```

**Step 2: Run new test**

Run: `cargo test --features java test_extra_sql_methods`
Expected: PASS

**Step 3: Run all tests**

Run: `cargo test --features java`
Expected: All tests pass.

---

### Task 5: Wire up CLI argument

**Files:**
- Modify: `src/bin/ogsql.rs` — add arg to `ParseJava` variant and pass to function

**Step 1: Add CLI argument to `ParseJava` command**

Change `Commands::ParseJava` from:
```rust
#[cfg(feature = "java")]
/// Extract and parse SQL from Java source files / 从 Java 源文件中提取并解析 SQL
#[command(name = "parse-java")]
ParseJava,
```

To:
```rust
#[cfg(feature = "java")]
/// Extract and parse SQL from Java source files / 从 Java 源文件中提取并解析 SQL
#[command(name = "parse-java")]
ParseJava {
    /// Additional unambiguous SQL method names to extract (comma-separated or repeatable)
    /// 追加的无歧义 SQL 方法名（逗号分隔或多次使用）
    #[arg(long = "extra-sql-methods", value_delimiter = ',')]
    extra_sql_methods: Vec<String>,
},
```

**Step 2: Update `cmd_parse_java` function**

Change function signature from:
```rust
fn cmd_parse_java(cli: &Cli) {
```
To:
```rust
fn cmd_parse_java(cli: &Cli, extra_sql_methods: &[String]) {
```

And change the call inside from:
```rust
let result = ogsql_parser::java::extract_sql_from_java(&source, &file_path);
```
To:
```rust
let config = ogsql_parser::java::JavaExtractConfig {
    extra_sql_methods: extra_sql_methods.to_vec(),
};
let result = ogsql_parser::java::extract_sql_from_java(&source, &file_path, &config);
```

**Step 3: Update main() dispatch**

Change from:
```rust
Commands::ParseJava => cmd_parse_java(&cli),
```
To:
```rust
Commands::ParseJava { extra_sql_methods } => cmd_parse_java(&cli, &extra_sql_methods),
```

**Step 4: Build and verify CLI**

Run: `cargo build --features java`
Expected: Clean build.

Run: `ogsql parse-java --help`
Expected: Shows `--extra-sql-methods` argument.

**Step 5: Integration test with CLI**

Create a test Java file and run:
```bash
echo 'public class T { public void f() { db.findSql("SELECT 1"); } }' > /tmp/test.java
ogsql parse-java -f /tmp/test.java
# Should show: No SQL statements found

ogsql parse-java -f /tmp/test.java --extra-sql-methods findSql
# Should show: 1 extraction with "SELECT 1"
```

---

### Summary of Changes

| File | Change |
|------|--------|
| `src/java/types.rs` | Add `JavaExtractConfig` struct |
| `src/java/mod.rs` | Export `JavaExtractConfig`, update `extract_sql_from_java` signature |
| `src/java/extract.rs` | Update `extract` signature, add `extra_sql_methods` field to `ExtractContext`, update match logic |
| `src/java/tests.rs` | Update all ~20 existing test calls + add 1 new test |
| `src/bin/ogsql.rs` | Add `--extra-sql-methods` arg to `ParseJava`, wire through to extraction |
