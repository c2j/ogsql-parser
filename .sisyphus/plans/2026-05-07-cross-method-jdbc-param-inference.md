# Cross-Method JDBC Parameter Inference

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Support JDBC `?` placeholder type/name inference across three patterns: `String[]` argument mapping (Pattern C), cross-method PS passing (Pattern B), and dynamic loop indexing.

**Architecture:** Two-phase approach. Phase 1 (P1) handles `String[]` array argument resolution within a single method call — when `DbService.executeQuery(sql, new String[]{node})` is seen, map array elements to `?` in the referenced SQL constant. Phase 2 (P2) adds cross-method PS flow via a file-scoped method behavior index — each method records what it does with PS parameters, then call sites resolve and inject.

**Tech Stack:** Rust, tree-sitter (Java CST), existing `ExtractContext` architecture.

**Branch:** `feat/cross-method-jdbc-param-inference` (from `main`)

---

## Phase 1 (P1): String[] Argument Mapping

Handle `DbService.executeQuery("ORACLEJDBC", SQL_CONSTANT, new String[] {node})` — parameters are in the same call expression as the SQL reference.

### Task 1: Resolve tracked SQL variable in unambiguous method arguments

Currently `find_first_string_arg` only matches `string_literal` / `binary_expression`. When the argument is an `identifier` that references a tracked SQL variable (e.g., `QUERY_MENU_SQL`), we need to recognize it and link to the existing extraction.

**Files:**
- Modify: `src/java/method_call.rs:55-94` (the `find_first_string_arg` / extraction push block)
- Test: `src/java/tests.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn test_sql_constant_referenced_in_method_call() {
    let java = r#"
        public class Dao {
            private static final String SQL = "SELECT * FROM t WHERE id = ?";
            public void query(String nodeId) {
                List list = DbService.executeQuery("ORACLEJDBC", SQL, new String[] {nodeId});
            }
        }
    "#;
    let mut config = JavaExtractConfig::default();
    config.extra_sql_methods = vec!["executeQuery".to_string()];
    let result = extract_sql_from_java(java, "Dao.java", &config);
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert!(ext.sql.contains("__JAVA_VAR_String_nodeId__"), "SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"), "SQL: {}", ext.sql);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test --features java test_sql_constant_referenced_in_method_call`
Expected: FAIL — `__JAVA_VAR_JDBC_PARAM_1__` remains unresolved

**Step 3: Implement SQL variable argument resolution**

In `visit_method_invocation`, after the `find_first_string_arg` block (line 94), add a new block: when `pushed_extraction_idx` is `None` and an argument is an `identifier` found in `sql_vars`, resolve to that tracked variable's extraction index. Do NOT create a duplicate extraction — reuse the existing one.

Then, scan remaining arguments for `array_creation_expression` patterns like `new String[] {var1, var2}` or `new Object[] {var1, var2}`. Extract element names from the array initializer. Map them positionally to `__JAVA_VAR_JDBC_PARAM_N__` in the linked extraction. Re-parse the modified SQL.

**Step 4: Run test to verify it passes**

Run: `cargo test --features java test_sql_constant_referenced_in_method_call`
Expected: PASS

**Step 5: Commit**

```bash
git add src/java/method_call.rs src/java/tests.rs
git commit -m "feat(java): resolve SQL constant + String[] args in utility method calls"
```

### Task 2: Handle `new String[] {var}` array argument parsing

Add a helper to extract variable names from `new String[] {a, b, c}` and `new Object[] {a, b, c}` array creation expressions.

**Files:**
- Modify: `src/java/method_call.rs`
- Test: `src/java/tests.rs`

**Step 1: Write failing tests for array arg variants**

```rust
#[test]
fn test_array_args_multiple_params() {
    let java = r#"
        public class Dao {
            private static final String SQL = "SELECT * FROM t WHERE a = ? AND b = ?";
            public void query(String x, String y) {
                DbService.executeQuery("DB", SQL, new String[] {x, y});
            }
        }
    "#;
    let mut config = JavaExtractConfig::default();
    config.extra_sql_methods = vec!["executeQuery".to_string()];
    let result = extract_sql_from_java(java, "Dao.java", &config);
    let ext = result.extractions.iter().find(|e| e.sql.contains("SELECT")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_String_x__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_y__"), "SQL: {}", ext.sql);
}

#[test]
fn test_array_args_object_array() {
    let java = r#"
        public class Dao {
            private static final String SQL = "SELECT * FROM t WHERE id = ?";
            public void query(int id) {
                DbService.executeQuery("DB", SQL, new Object[] {id});
            }
        }
    "#;
    let mut config = JavaExtractConfig::default();
    config.extra_sql_methods = vec!["executeQuery".to_string()];
    let result = extract_sql_from_java(java, "Dao.java", &config);
    let ext = result.extractions.iter().find(|e| e.sql.contains("SELECT")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_int_id__"), "SQL: {}", ext.sql);
}
```

**Step 2: Run tests — expect FAIL**

**Step 3: Implement `extract_array_element_names` helper**

Parse `array_creation_expression` nodes: walk into the `element_list` child, extract identifiers from each element. For elements that aren't simple identifiers (e.g., method calls, field access), use the existing `make_placeholder_for_node`.

**Step 4: Run tests — expect PASS**

**Step 5: Commit**

```bash
git commit -m "feat(java): parse String[]/Object[] array args for parameter inference"
```

### Task 3: Inline SQL + inline array args

Handle the case where SQL is an inline string literal (not a constant) AND parameters are an inline array: `DbService.executeQuery("DB", "SELECT * FROM t WHERE id = ?", new String[]{x})`.

**Files:**
- Modify: `src/java/method_call.rs`
- Test: `src/java/tests.rs`

**Step 1: Write failing test**

```rust
#[test]
fn test_inline_sql_with_inline_array_args() {
    let java = r#"
        public class Dao {
            public void query(String nodeId) {
                DbService.executeQuery("DB", "SELECT * FROM t WHERE id = ?", new String[] {nodeId});
            }
        }
    "#;
    let mut config = JavaExtractConfig::default();
    config.extra_sql_methods = vec!["executeQuery".to_string()];
    let result = extract_sql_from_java(java, "Dao.java", &config);
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert!(ext.sql.contains("__JAVA_VAR_String_nodeId__"), "SQL: {}", ext.sql);
}
```

**Step 2: Run test — expect FAIL** (currently creates extraction with unresolved JDBC_PARAM)

**Step 3: Extend the extraction push block**

After creating a new extraction from an inline string arg, also scan remaining arguments for `array_creation_expression`. If found, immediately apply the array elements as backfill to the just-pushed extraction.

**Step 4: Run test — expect PASS**

**Step 5: Commit**

```bash
git commit -m "feat(java): backfill params from inline String[] in same call"
```

### Task 4: Verify Phase 1 with real-world pattern

Test the exact user scenario: `static final` SQL constants + `DbService.executeQuery` + `String[]` args.

**Files:**
- Test: `src/java/tests.rs`

**Step 1: Write integration test**

```rust
#[test]
fn test_ebms_pattern_two_sql_constants() {
    let java = r#"
        public class EBMSHandler {
            private static final String SWITCH_SQL = "SELECT KIND_ID FROM ebk_dic_all_kind t WHERE t.operation_kind = 'PI00168_SWITCH'";
            private static final String QUERY_MENU_SQL = "SELECT t.node_id, t.node_name, t.edition FROM par_netuser_menu_tree t WHERE t.node_id = ?";

            public int execute(String node) throws Exception {
                List list1 = DbService.executeQuery("ORACLEJDBC", SWITCH_SQL);
                List list2 = DbService.executeQuery("ORACLEJDBC", QUERY_MENU_SQL, new String[] {node});
                return 0;
            }
        }
    "#;
    let mut config = JavaExtractConfig::default();
    config.extra_sql_methods = vec!["executeQuery".to_string()];
    let result = extract_sql_from_java(java, "EBMSHandler.java", &config);
    assert_eq!(result.extractions.len(), 2);

    let switch = result.extractions.iter().find(|e| e.sql.contains("SWITCH") || e.sql.contains("ebk_dic_all_kind")).unwrap();
    assert!(!switch.sql.contains("__JAVA_VAR_JDBC_PARAM_"), "SWITCH_SQL has no params: {}", switch.sql);

    let menu = result.extractions.iter().find(|e| e.sql.contains("par_netuser_menu_tree")).unwrap();
    assert!(menu.sql.contains("__JAVA_VAR_String_node__"), "QUERY_MENU_SQL should resolve ?: {}", menu.sql);
    assert!(!menu.sql.contains("__JAVA_VAR_JDBC_PARAM_"), "QUERY_MENU_SQL should have no unresolved: {}", menu.sql);
}
```

**Step 2: Run all tests**

Run: `cargo test --features java`
Expected: ALL PASS

**Step 3: Commit**

```bash
git commit -m "test(java): integration test for DbService.executeQuery pattern"
```

---

## Phase 2 (P2): Cross-Method PreparedStatement Flow

Handle `ps = conn.prepareStatement(sql); submitData(ps, list);` where `submitData` calls `ps.setString` in another method.

### Task 5: Add file-scoped method behavior index

Introduce a `HashMap<String, MethodPsBehavior>` to `ExtractContext` that persists across method boundaries (not saved/restored in `visit_method_declaration`).

**Files:**
- Modify: `src/java/types.rs` — add `MethodPsBehavior`, `SetterPattern` types
- Modify: `src/java/extract.rs` — add `method_behaviors` field to `ExtractContext`, NOT included in save/restore

**Step 1: Define types**

In `src/java/types.rs`:

```rust
/// Recorded behavior of a method on its PreparedStatement parameter.
#[derive(Debug, Clone)]
pub(super) struct MethodPsBehavior {
    pub(super) ps_param_index: usize,
    pub(super) ps_param_name: String,
    pub(super) setter_patterns: Vec<SetterPattern>,
}

/// Pattern of a setXxx call on a PS parameter.
#[derive(Debug, Clone)]
pub(super) enum SetterPattern {
    /// Literal index: ps.setString(1, name)
    Literal {
        index: usize,
        java_type: String,
        var_name: Option<String>,
    },
    /// Dynamic loop: ps.setString(i+1, arr[i])
    DynamicLoop {
        java_type: String,
    },
}
```

**Step 2: Add field to ExtractContext**

In `src/java/extract.rs`, add:
```rust
pub(super) method_behaviors: HashMap<String, MethodPsBehavior>,
```

Initialize in `extract()` as `HashMap::new()`. Do NOT include in `visit_method_declaration`'s save/restore (`old_*` / restore) — this field is file-scoped.

**Step 3: Compile check**

Run: `cargo build --features java`
Expected: compiles (field unused is OK)

**Step 4: Commit**

```bash
git commit -m "feat(java): add MethodPsBehavior types and file-scoped method_behaviors index"
```

### Task 6: Record method behavior for PS-receiving methods

When a method has a `PreparedStatement` parameter and makes `setXxx` calls on it, record the behavior in `method_behaviors`.

**Files:**
- Modify: `src/java/extract.rs` — at end of `visit_method_declaration`, after `backfill_jdbc_params()`
- Modify: `src/java/method_call.rs` — extend `visit_setter_call` to also record into a per-method accumulator

**Step 1: Write failing test**

```rust
#[test]
fn test_cross_method_ps_passing_literal_setters() {
    let java = r#"
        public class Dao {
            public void process(String name, String email) {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO t (name, email) VALUES (?, ?)");
                insertData(ps, name, email);
            }

            public static void insertData(PreparedStatement ps, String name, String email) {
                ps.setString(1, name);
                ps.setString(2, email);
                ps.execute();
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    let ext = result.extractions.iter().find(|e| e.sql.contains("INSERT INTO t")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_String_name__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_email__"), "SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"), "SQL: {}", ext.sql);
}
```

**Step 2: Run test — expect FAIL**

**Step 3: Implement behavior recording + call site injection**

Part A — Recording: In `visit_method_declaration`, after backfill, scan `jdbc_param_map` for entries where the PS var name matches a method parameter of type `PreparedStatement`. Collect those into a `MethodPsBehavior` and insert into `method_behaviors`.

Part B — Injection: In `visit_method_invocation`, when we encounter a method call `someMethod(ps, ...)` where:
1. `ps` is in `ps_var_to_extraction` (tracked PS variable)
2. The method name is found in `method_behaviors`
3. The argument position matches the `ps_param_index`

Then apply the behavior's `setter_patterns` to the extraction referenced by `ps_var_to_extraction[ps_var]`.

**Step 4: Run test — expect PASS**

**Step 5: Commit**

```bash
git commit -m "feat(java): cross-method PS parameter inference via method behavior index"
```

### Task 7: Handle dynamic loop setter pattern

Record `ps.setString(i+1, s[i])` as `DynamicLoop { java_type: "String" }` instead of ignoring it.

**Files:**
- Modify: `src/java/method_call.rs` — extend `visit_setter_call` for dynamic index detection
- Test: `src/java/tests.rs`

**Step 1: Write failing test**

```rust
#[test]
fn test_cross_method_dynamic_loop_setter() {
    let java = r#"
        public class Dao {
            public void process(List list) {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO t (a,b,c) VALUES (?,?,?)");
                submitData(ps, list);
            }

            public static void submitData(PreparedStatement ps, List list) throws Exception {
                for (Iterator it = list.iterator(); it.hasNext();) {
                    String[] s = (String[]) it.next();
                    for (int i = 0; i < s.length; i++) {
                        ps.setString(i + 1, s[i]);
                    }
                    ps.addBatch();
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    let ext = result.extractions.iter().find(|e| e.sql.contains("INSERT INTO t")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_String_DYNAMIC_1__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_DYNAMIC_2__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_DYNAMIC_3__"), "SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"), "SQL: {}", ext.sql);
}
```

**Step 2: Run test — expect FAIL**

**Step 3: Implement dynamic index detection**

In `visit_setter_call`, when `args[0].trim().parse::<usize>()` fails, check if the index argument is a `binary_expression` like `i + 1` or `i + 1)`. If so, record as `SetterPattern::DynamicLoop` in a per-method accumulator (separate from `jdbc_param_map` which requires a literal index).

In the behavior recording step (Task 6 Part A), include `DynamicLoop` entries.

In the injection step (Task 6 Part B), when applying `DynamicLoop`, generate `__JAVA_VAR_{java_type}_DYNAMIC_N__` for each `?` in the SQL that wasn't already covered by a `Literal` pattern.

**Step 4: Run test — expect PASS**

**Step 5: Commit**

```bash
git commit -m "feat(java): dynamic loop setter pattern inference across methods"
```

### Task 8: Real-world integration test — full user scenario

Test the exact `submitData` + `process` scenario from the user's code.

**Files:**
- Test: `src/java/tests.rs`

**Step 1: Write test**

```rust
#[test]
fn test_user_scenario_submit_data_cross_method() {
    let java = r#"
        public class DataProcessor {
            public static void submitData(PreparedStatement ps, List list) throws Exception {
                try {
                    for (Iterator it = list.iterator(); it.hasNext();) {
                        String[] s = (String[]) it.next();
                        for (int i = 0; i < s.length; i++) {
                            ps.setString(i + 1, s[i]);
                        }
                        ps.addBatch();
                    }
                    ps.executeBatch();
                } catch (Exception e) {
                    e.printStackTrace();
                }
            }

            public void process(List list, String accno) throws Exception {
                ps = conn.prepareStatement("insert into dat_clnt_fv_tran " +
                        "(ACCNO,BUSIDATE,SERIALNO,TRXCODE,DRCRF,SUMMARY,AMOUNT,BALANCE,RECIPACC,RECIPNAM,NOTES)" +
                        "VALUES(lpad(" + accno + ",19,0),?,?,?,?,?,?,?,?,?,?)");
                if(list.size()>0){
                    submitData(ps, list);
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "DataProcessor.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert!(ext.sql.contains("__JAVA_VAR_String_accno__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_DYNAMIC_1__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_DYNAMIC_10__"), "SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"), "SQL: {}", ext.sql);
}
```

**Step 2: Run all tests**

Run: `cargo test --features java`
Expected: ALL PASS

**Step 3: Commit**

```bash
git commit -m "test(java): integration test for cross-method submitData scenario"
```

---

## Task Dependency Graph

```
Task 1 ──► Task 2 ──► Task 3 ──► Task 4   (Phase 1: String[] args)
                                           │
                                           ▼
                    Task 5 ──► Task 6 ──► Task 7 ──► Task 8   (Phase 2: cross-method)
```

Phase 1 and Phase 2 are independent — Phase 2 does not depend on Phase 1. But they should land on the same branch.

## Files Changed (Summary)

| File | Tasks | Purpose |
|---|---|---|
| `src/java/method_call.rs` | 1,2,3,6,7 | Core logic: array arg parsing, SQL var resolution, cross-method injection |
| `src/java/extract.rs` | 5,6 | ExtractContext extension, method behavior recording |
| `src/java/types.rs` | 5 | New types: MethodPsBehavior, SetterPattern |
| `src/java/tests.rs` | 1-8 | All tests |

## Estimated Size

| Phase | New/Modified LOC | Complexity |
|---|---|---|
| Phase 1 (Tasks 1-4) | ~120-150 lines | Low — single call expression scope |
| Phase 2 (Tasks 5-8) | ~250-350 lines | Medium — file-scoped index, two-pass data flow |
| **Total** | ~370-500 lines | |
