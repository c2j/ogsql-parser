# iBatis Callable Stored Procedure Mapper XML Parsing Fixes

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix 6 bugs in parse-xml for MyBatis mapper XML containing stored procedure calls with `<foreach>`, `<if>`, `#{param,mode=IN,jdbcType=VARCHAR}` style parameters.

**Architecture:** Changes span the iBatis module's type model, XML parser, parameter extraction, flattening, and module orchestration. We follow TDD: add guard tests first, then fix each issue.

**Tech Stack:** Rust, quick-xml, serde, ogsql-parser ibatis module.

---

## Bug Summary

| # | Issue | Files | Severity |
|---|-------|-------|----------|
| 1 | ForEach `separator` dropped in `flatten_sql` | flatten.rs | HIGH |
| 2 | `ParamMeta.jdbc_type` always null (non-java path) | mod.rs | HIGH |
| 3 | `databaseId` attribute not captured | types.rs, parser.rs, mod.rs | MEDIUM |
| 4 | `java_type/jdbcType` priority confusion — both lost when both present | util.rs, types.rs, flatten.rs, mod.rs, expand.rs, parser.rs | MEDIUM |
| 5 | `statementType` attribute not captured | types.rs, parser.rs, mod.rs | LOW |
| 6 | `mode`, `resultMap` param attrs silently dropped | util.rs, types.rs | LOW |

---

## Task 1: Add Guard Test Cases

**Files:**
- Modify: `src/ibatis/tests.rs` (append after existing tests)

**Step 1:** Add a helper function and the guard test using the user's full mapper XML.

The test should verify:
- `database_id` is captured for `databaseId="gauss"` statements
- `statement_type` is `"CALLABLE"` for all statements
- `Parameter` nodes have separate `jdbc_type` field
- `mode` is preserved in param attrs
- `ForEach` flattened SQL contains separator `,`
- `ParamMeta.jdbc_type` is not null when XML has `jdbcType=`

Write the test with assertions that will FAIL against current code, confirming each bug.

**Step 2:** Run `cargo test --lib ibatis::tests` to verify tests compile but fail.

---

## Task 2: Update Data Model — Add `database_id`, `statement_type` to Statement Types

**Files:**
- Modify: `src/ibatis/types.rs`

**Step 1:** Add to `MapperStatement`:
```rust
pub database_id: Option<String>,
pub statement_type: Option<String>,
```

**Step 2:** Add to `ParsedStatement`:
```rust
pub database_id: Option<String>,
pub statement_type: Option<String>,
```

**Step 3:** Add to `StructuredStatement`:
```rust
pub database_id: Option<String>,
pub statement_type: Option<String>,
```

---

## Task 3: Capture `databaseId` and `statementType` in XML Parser

**Files:**
- Modify: `src/ibatis/parser.rs`

**Step 1:** In `parse_xml()` where `MapperStatement` is constructed (around line 49), extract:
```rust
let database_id = get_attr(&e, "databaseId");
let statement_type = get_attr(&e, "statementType");
```

Add both fields to the `MapperStatement` initializer.

---

## Task 4: Fix Parameter Parsing — Separate jdbcType/javaType, Preserve mode/resultMap

**Files:**
- Modify: `src/ibatis/util.rs`
- Modify: `src/ibatis/types.rs`
- Modify: `src/ibatis/parser.rs`
- Modify: `src/ibatis/flatten.rs`
- Modify: `src/ibatis/expand.rs`
- Modify: `src/ibatis/mod.rs`

### Step 1 (util.rs): Add `ParamAttrs` struct, update `parse_param_type`

```rust
/// Parsed attributes from a MyBatis #{...} parameter.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ParamAttrs {
    pub java_type: Option<String>,
    pub jdbc_type: Option<String>,
    pub mode: Option<String>,
    pub result_map: Option<String>,
}

pub fn parse_param_attrs(param: &str) -> (String, ParamAttrs) { ... }
```

Keep old `parse_param_type` as compatibility wrapper returning `(String, Option<String>)` using `jdbc_type.or(java_type)`.

### Step 2 (types.rs): Add `jdbc_type` to `SqlNode::Parameter` and `SqlNode::RawExpr`

```rust
Parameter {
    name: String,
    java_type: Option<String>,  // kept for backward compat
    jdbc_type: Option<String>,  // NEW: actual JDBC type
    mode: Option<String>,       // NEW
},
RawExpr {
    expr: String,
    java_type: Option<String>,
    jdbc_type: Option<String>,  // NEW
},
```

### Step 3 (parser.rs): Use `parse_param_attrs` in `parse_text_to_nodes`

Update `#{...}` and `${...}` handling to use new function and populate all fields.

### Step 4 (flatten.rs): Update `flatten_sql`, `collect_params`, `simple_node_to_text`, `replace_params`

- Use `jdbc_type.or(java_type)` for placeholder formatting (prefer JDBC type)
- Update `collect_params` to return `(String, ParamAttrs, String)` or similar
- Update `simple_node_to_text` to reconstruct with proper jdbcType=

### Step 5 (expand.rs): Update `expand_node` for new Parameter/RawExpr fields

### Step 6 (mod.rs): Wire new fields through `parse_mapper_bytes_internal` and `parse_mapper_bytes_structured_with_path`

---

## Task 5: Fix ForEach Separator in Flatten

**Files:**
- Modify: `src/ibatis/flatten.rs`

**Step 1:** In `flatten_sql`, update `ForEach` arm:

Current code ignores separator. Fix: when flattening, join children with separator for a single "most complete" iteration. The separator should appear between the flattened children of the foreach body.

```rust
SqlNode::ForEach { open, separator, close, prepend, children, .. } => {
    let content = flatten_children(children);
    let sep = separator.as_deref().unwrap_or("");
    // For "most complete" strategy: use separator between non-empty child segments
    let body = format!("{}{}{}", open_str, content, close_str);
    apply_prepend(prepend, &body)
}
```

Actually the issue is more nuanced. The "most complete" strategy takes ALL if-branches. For stored proc calls, each `<if>` represents a mutually exclusive param type. The flattened result should ideally pick one branch.

But since flattening always takes all branches, the separator won't help between exclusive branches. The real fix: insert separator between **non-empty** flattened child segments of the foreach body.

**Strategy:** Split the foreach children into segments separated by dynamic elements. For each segment, flatten it. Join segments with separator. Only include non-empty segments.

---

## Task 6: Fix ParamMeta.jdbc_type Always Null

**Files:**
- Modify: `src/ibatis/mod.rs`

**Step 1:** In `parse_mapper_bytes_structured_with_path` (line ~139), convert `jdbc_type` string to `JdbcType`:

```rust
let jdbc_type = attrs.jdbc_type.as_ref()
    .and_then(|s| jdbc_type_from_str(s))
    .or_else(|| attrs.java_type.as_ref().and_then(|s| jdbc_type_from_str(s)));
```

**Step 2:** In non-java flat path (line ~216), same conversion.

**Step 3:** Add `jdbc_type_from_str` to `util.rs` (it already exists in `java_resolve.rs` but we need it accessible without java feature).

---

## Task 7: Run All Tests, Verify

**Step 1:** `cargo test --lib ibatis::tests`
**Step 2:** `cargo test` (full suite)
**Step 3:** Run parse-xml on the test mapper and verify all 6 issues are fixed.
