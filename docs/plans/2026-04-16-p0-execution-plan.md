# P0 GaussDB Syntax Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement all P0 (high priority) items from the GaussDB syntax gap analysis: 5 specific syntax extensions + 37 stub statement parsers.

**Architecture:** Hand-written recursive descent parser. Each statement needs: (1) AST struct definition in `src/ast/mod.rs`, (2) Parser method in appropriate `src/parser/` file, (3) Dispatch wiring in `src/parser/mod.rs`, (4) Formatter support in `src/formatter.rs`. Tests in `src/parser/tests.rs`.

**Tech Stack:** Rust 2021, thiserror 2.0, serde (Serialize/Deserialize)

---

## Key Files Reference

| File | Purpose | Lines |
|------|---------|-------|
| `src/ast/mod.rs` | All AST type definitions + Statement enum | 2256 |
| `src/parser/mod.rs` | Top-level dispatch + helper methods | 2866 |
| `src/parser/ddl/create.rs` | CREATE INDEX/SEQUENCE/VIEW/etc. parsers | 956 |
| `src/parser/ddl/table.rs` | CREATE TABLE parser (with PARTITION) | 907 |
| `src/parser/ddl/alter.rs` | ALTER TABLE parser (with PARTITION actions) | 970 |
| `src/parser/ddl/drop.rs` | DROP statement parser | 347 |
| `src/parser/select.rs` | SELECT parser | 708 |
| `src/parser/utility/copy_explain.rs` | COPY/EXPLAIN parsers | 903 |
| `src/parser/utility/statements.rs` | TRIGGER, COMMENT, LOCK, etc. | 1561 |
| `src/parser/utility/grant.rs` | GRANT/REVOKE parsers | ~300 |
| `src/parser/utility/functions.rs` | Helper functions | ~500 |
| `src/formatter.rs` | SQL formatter for all AST types | 5311 |
| `src/parser/tests.rs` | Unit tests (5581 lines, 230+ tests) |

## Patterns to Follow

### AST Definition Pattern
```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct XxxStatement {
    pub name: String,
    pub action: Option<XxxAction>,
}
```

### Parser Pattern (simple)
```rust
pub(crate) fn parse_xxx(&mut self) -> Result<XxxStatement, ParserError> {
    // consume expected tokens, parse fields
    Ok(XxxStatement { ... })
}
```

### Dispatch Pattern (in parse_statement/dispatch_create/dispatch_alter)
```rust
Some(Keyword::XXX) => {
    match self.parse_xxx() {
        Ok(stmt) => { self.try_consume_semicolon(); Statement::Xxx(stmt) }
        Err(e) => { self.add_error(e); self.skip_to_semicolon() }
    }
}
```

### Test Pattern
```rust
#[test]
fn test_parse_xxx() {
    let stmt = parse_one("XXX ...");
    match stmt {
        Statement::Xxx(s) => { /* assertions */ }
        _ => panic!("expected Xxx"),
    }
}
```

---

## Work Unit A: Quick Wins (P0-4 Verify + P0-5 SELECT INTO)

**Dependencies:** None (can start immediately)

### Task A-1: Verify EXPLAIN PLAN already works

**Files:**
- Test: `src/parser/tests.rs`
- Doc SQL: `GaussDB-2.23.07.210/sql/by_file/1333_EXPLAIN PLAN.sql`

**Step 1:** Read the existing EXPLAIN PLAN parser at `src/parser/utility/copy_explain.rs:536-596` and verify it handles `EXPLAIN PLAN [SET STATEMENT_ID = name] FOR statement`

**Step 2:** Add test cases from GaussDB docs:
```rust
#[test]
fn test_explain_plan_basic() {
    let stmt = parse_one("EXPLAIN PLAN FOR SELECT * FROM t");
    match stmt {
        Statement::Explain(e) => {
            assert!(e.plan);
            assert!(e.statement_id.is_none());
        }
        _ => panic!("expected Explain"),
    }
}

#[test]
fn test_explain_plan_with_statement_id() {
    let stmt = parse_one("EXPLAIN PLAN SET STATEMENT_ID = 'myplan' FOR SELECT * FROM t");
    match stmt {
        Statement::Explain(e) => {
            assert!(e.plan);
            assert_eq!(e.statement_id.as_deref(), Some("myplan"));
        }
        _ => panic!("expected Explain"),
    }
}
```

**Step 3:** Run `cargo test test_explain_plan` — should pass

**Step 4:** Mark P0-4 as COMPLETE in the gap analysis doc

### Task A-2: Add SELECT INTO TABLE support

**Files:**
- Modify: `src/ast/mod.rs` (SelectStatement AST)
- Modify: `src/parser/select.rs:139-144` (INTO parsing)
- Modify: `src/formatter.rs` (format SELECT INTO)
- Test: `src/parser/tests.rs`

**Current state:** `SelectStatement.into_targets: Option<Vec<SelectTarget>>` handles PL/pgSQL `SELECT INTO var1, var2` form only.

**Step 1:** Add `SelectIntoTable` type to AST:
```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SelectIntoTable {
    pub unlogged: bool,
    pub table_name: ObjectName,
}
```
Add field to `SelectStatement`:
```rust
pub into_table: Option<SelectIntoTable>,  // GaussDB: INTO [UNLOGGED] [TABLE] new_table
```

**Step 2:** Modify parser at `src/parser/select.rs:139-144`:
```rust
let into_table = if self.match_keyword(Keyword::INTO) {
    self.advance();
    if self.match_keyword(Keyword::TABLE) || self.match_keyword(Keyword::UNLOGGED) {
        // GaussDB SELECT INTO TABLE form
        let unlogged = self.try_consume_keyword(Keyword::UNLOGGED);
        self.try_consume_keyword(Keyword::TABLE); // optional TABLE keyword
        let table_name = self.parse_object_name()?;
        Some(SelectIntoTable { unlogged, table_name })
    } else {
        None
    }
} else {
    None
};
// Keep existing into_targets for PL/pgSQL form (variables after INTO)
```

**Step 3:** Add test:
```rust
#[test]
fn test_select_into_table() {
    let stmt = parse_one("SELECT * INTO new_table FROM t");
    match stmt {
        Statement::Select(s) => {
            assert!(s.into_table.is_some());
            assert_eq!(s.into_table.unwrap().table_name.to_string(), "new_table");
        }
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_select_into_unlogged_table() {
    let stmt = parse_one("SELECT * INTO UNLOGGED TABLE new_table FROM t WHERE id = 1");
    match stmt {
        Statement::Select(s) => {
            let t = s.into_table.unwrap();
            assert!(t.unlogged);
            assert_eq!(t.table_name.to_string(), "new_table");
        }
        _ => panic!("expected Select"),
    }
}
```

**Step 4:** Update formatter to handle `into_table`

**Step 5:** `cargo test` — all pass

---

## Work Unit B: CREATE GLOBAL INDEX (P0-3)

**Dependencies:** None (can start immediately, parallel with A)

### Task B-1: Define AST for CreateGlobalIndexStatement

**Files:** Modify: `src/ast/mod.rs`

**Step 1:** Add `GlobalIndexColumn` (enhanced IndexColumn for GSI features):
```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GlobalIndexColumn {
    pub name: String,
    pub length: Option<u32>,
    pub collation: Option<String>,
    pub opclass: Option<String>,
    pub ordering: Option<IndexOrdering>,
    pub nulls: Option<IndexNulls>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum IndexOrdering { Asc, Desc }

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum IndexNulls { First, Last }
```

**Step 2:** Add `CreateGlobalIndexStatement`:
```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateGlobalIndexStatement {
    pub unique: bool,
    pub concurrent: bool,
    pub if_not_exists: bool,
    pub name: Option<ObjectName>,
    pub table: ObjectName,
    pub using_method: Option<String>,
    pub columns: Vec<GlobalIndexColumn>,
    pub containing: Vec<String>,
    pub distribute_by: Option<DistributeClause>,
    pub with_options: Vec<(String, String)>,
    pub tablespace: Option<String>,
    pub visible: Option<bool>,
    pub where_clause: Option<Expr>,
}
```

**Step 3:** Add `Statement::CreateGlobalIndex(CreateGlobalIndexStatement)` variant to `Statement` enum

### Task B-2: Implement parser for CREATE GLOBAL INDEX

**Files:**
- Modify: `src/parser/ddl/create.rs` (add parser method)
- Modify: `src/parser/mod.rs` (add dispatch in `dispatch_create`)

**Step 1:** Add `parse_create_global_index()` method to `impl Parser` in `create.rs`:
- Parse `GLOBAL` keyword (already consumed by dispatch)
- Parse `UNIQUE`, `CONCURRENTLY`, `IF NOT EXISTS`
- Parse index name (optional)
- Parse `ON table_name`
- Parse optional `USING method`
- Parse column list `(col1, col2, ...)` with per-column options
- Parse `CONTAINING (col1, col2, ...)`
- Parse `DISTRIBUTE BY HASH(col1, ...)`
- Parse `WITH (key = value, ...)`
- Parse `TABLESPACE name`
- Parse `VISIBLE | INVISIBLE`
- Parse `WHERE predicate`

**Step 2:** Wire into `dispatch_create()`:
```rust
Some(Keyword::GLOBAL) => {
    self.advance(); // consume GLOBAL
    match self.parse_create_global_index() {
        Ok(stmt) => { self.try_consume_semicolon(); Statement::CreateGlobalIndex(stmt) }
        Err(e) => { self.add_error(e); self.skip_to_semicolon() }
    }
}
```

**Step 3:** Add tests:
```rust
#[test]
fn test_create_global_index_basic() {
    let stmt = parse_one("CREATE GLOBAL INDEX idx ON t1(col1)");
    // assertions
}

#[test]
fn test_create_global_index_full() {
    let stmt = parse_one("CREATE GLOBAL UNIQUE INDEX CONCURRENTLY IF NOT EXISTS idx ON t1 USING btree (col1 ASC NULLS FIRST, col2 DESC) CONTAINING (col3) DISTRIBUTE BY HASH(col1) TABLESPACE ts1 VISIBLE WHERE col1 > 0");
    // full assertions
}
```

### Task B-3: Add formatter support

**Files:** Modify: `src/formatter.rs`

**Step 1:** Add `format_create_global_index()` method following the existing `format_create_index` pattern

**Step 2:** Wire into `format_statement()` match arm

**Step 3:** `cargo test` — verify round-trip

---

## Work Unit C: ALTER TABLE PARTITION Extensions (P0-1)

**Dependencies:** None (can start immediately, parallel with A and B)

### Task C-1: Add `UPDATE GLOBAL INDEX` clause support

**Files:**
- Modify: `src/ast/mod.rs` (add field to relevant AlterTableAction variants)
- Modify: `src/parser/ddl/alter.rs` (parse the clause after partition actions)
- Modify: `src/formatter.rs`

**Missing clauses to add:**
- `update_global_index: bool` on DropPartition, TruncatePartition, MergePartitions, SplitPartition, ExchangePartition
- `update_distributed_global_index: Option<bool>` — `Some(true)` = UPDATE DISTRIBUTED GLOBAL INDEX, `Some(false)` = NO UPDATE DISTRIBUTED GLOBAL INDEX, `None` = not specified

**Step 1:** Extend AST action variants in `src/ast/mod.rs` to include `update_global_index: bool` and `update_distributed_global_index: Option<bool>` fields

**Step 2:** Update parser in `alter.rs` — after parsing each partition action, check for `UPDATE GLOBAL INDEX` and `UPDATE DISTRIBUTED GLOBAL INDEX | NO UPDATE DISTRIBUTED GLOBAL INDEX`

**Step 3:** Update formatter

**Step 4:** Add tests:
```rust
#[test]
fn test_alter_table_drop_partition_update_global_index() {
    let stmt = parse_one("ALTER TABLE t1 DROP PARTITION p1 UPDATE GLOBAL INDEX");
    // assert update_global_index == true
}
```

### Task C-2: Add `FOR (partition_value)` form to MOVE/ADD/SPLIT

**Files:**
- Modify: `src/parser/ddl/alter.rs`
- Modify: `src/ast/mod.rs` (if needed)

**Step 1:** In `parse_alter_table_action()` for MOVE PARTITION, add:
```rust
let name_or_for = if self.match_keyword(Keyword::FOR) {
    self.advance();
    self.expect_token(&Token::LParen)?;
    let values = self.parse_expr_list()?;
    self.expect_token(&Token::RParen)?;
    PartitionRef::For(values)
} else {
    PartitionRef::Name(self.parse_identifier()?)
};
```

**Step 2:** Add `PartitionRef` enum to AST if not exists (similar to existing `DmlPartitionClause` pattern)

**Step 3:** Update all relevant actions (MOVE, ADD, SPLIT, etc.)

### Task C-3: Add ENABLE/DISABLE ROW MOVEMENT, WITH/WITHOUT VALIDATION, EXCHANGE extensions

**Files:** Modify: `src/ast/mod.rs`, `src/parser/ddl/alter.rs`, `src/formatter.rs`

**Step 1:** Add `AlterTableAction::EnableRowMovement` and `AlterTableAction::DisableRowMovement`

**Step 2:** Add `with_validation: Option<bool>` and `verbose: bool` to ExchangePartition

**Step 3:** Add `ONLY` / `*` table forms to ExchangePartition target

**Step 4:** Tests + formatter updates

### Task C-4: Add MODIFY PARTITION and RESET PARTITION

**Step 1:** Add `AlterTableAction::ModifyPartition { name, action: ModifyPartitionAction }` with `UnusableLocalIndexes`, `RebuildUnusableLocalIndexes` sub-actions

**Step 2:** Add `AlterTableAction::ResetPartition`

**Step 3:** Tests + formatter

---

## Work Unit D: CREATE TABLE PARTITION Extensions (P0-2)

**Dependencies:** None (can start immediately, parallel with A, B, C)

### Task D-1: Add RANGE COLUMNS / LIST COLUMNS support

**Files:** Modify: `src/ast/mod.rs`, `src/parser/ddl/table.rs`

**Step 1:** Add `is_columns: bool` field to `PartitionClause::Range` and `PartitionClause::List` variants

**Step 2:** Update parser to consume optional `COLUMNS` keyword after `RANGE`/`LIST`

### Task D-2: Add PARTITIONS integer for RANGE/LIST

**Step 1:** Add `partitions_count: Option<u32>` to `PartitionClause::Range` and `PartitionClause::List`

**Step 2:** Update parser — after column spec, check for `PARTITIONS N`

### Task D-3: Add START/END/EVERY partition syntax

**Step 1:** Add new `PartitionValues::StartEnd { start: Vec<Expr>, end: Vec<Expr>, every: Option<Expr> }` variant (or check if already exists)

**Step 2:** Implement `parse_partition_start_end_item()` in `table.rs`:
```
PARTITION name { START(expr) END(expr) [EVERY(expr)] } [TABLESPACE ts]
```

**Step 3:** Tests with GaussDB examples from `1273_CREATE TABLE PARTITION.sql`

### Task D-4: Add ENABLE/DISABLE ROW MOVEMENT at table level

**Step 1:** Add `row_movement: Option<bool>` to `CreateTableStatement`

**Step 2:** Parse in the post-parentheses option loop in `parse_create_table()`

### Task D-5: Add DISTRIBUTE BY RANGE/LIST with SLICE syntax

**Step 1:** Extend `DistributeClause` enum:
```rust
Range { columns: Vec<String>, slices: Vec<SliceDef> },
List { columns: Vec<String>, slices: Vec<SliceDef> },
```

**Step 2:** Implement `parse_slice_def()` and `parse_slice_references()`

### Task D-6: Add TO { GROUP | NODE } clause

**Step 1:** Already has `to_group: Option<String>`, extend to support `TO NODE(n1, n2)`:
```rust
pub to_group: Option<String>,
pub to_node: Option<Vec<String>>,
```

### Task D-7: Update formatter for all new partition features

---

## Work Unit E: Simple Stub Statements (P0-6, Batch 1 — Top-level keywords)

**Dependencies:** None (can start immediately)

These are simple statements that appear as top-level keywords in `parse_statement()`:

| # | Statement | Keyword | AST | Syntax |
|---|-----------|---------|-----|--------|
| 1 | SHUTDOWN | `SHUTDOWN` | `ShutdownStatement` | `SHUTDOWN [FAST | IMMEDIATE]` |
| 2 | BARRIER | `BARRIER` | `BarrierStatement` | `BARRIER barrier_name` |
| 3 | PURGE | `PURGE` | `PurgeStatement` | `PURGE { TABLE ... \| INDEX ... \| RECYCLEBIN }` |
| 4 | COMPILE | `COMPILE` | `CompileStatement` | Complex PL/SQL schema object |
| 5 | VERIFY | `VERIFY` | `VerifyStatement` | `VERIFY [FAST] table_name ...` |
| 6 | SNAPSHOT | `SNAPSHOT` | `SnapshotStatement` | `SNAPSHOT { AS OF ... \| ...}` |
| 7 | TIMECAPSULE | `TIMECAPSULE TABLE` | `TimeCapsuleStatement` | `TIMECAPSULE TABLE name TO {TIMESTAMP \| SCN} expr` |
| 8 | SHRINK | `SHRINK` | `ShrinkStatement` | `SHRINK SPACE ...` |
| 9 | CLEAN CONNECTION | `CLEAN CONNECTION` | `CleanConnStatement` | `CLEAN CONNECTION TO ALL [FOR USER name]` |
| 10 | SECURITY LABEL | `SECURITY LABEL` | `SecLabelStatement` | `SECURITY LABEL [FOR provider] ON ... IS 'label'` |

### For each statement:

**Step 1:** Replace stub struct with real AST definition in `src/ast/mod.rs`
**Step 2:** Implement parser method (in `src/parser/utility/statements.rs` or `mod.rs`)
**Step 3:** Wire dispatch in `parse_statement()` or appropriate sub-dispatcher
**Step 4:** Add formatter support in `src/formatter.rs`
**Step 5:** Add 2-3 tests in `src/parser/tests.rs`
**Step 6:** `cargo test` to verify

---

## Work Unit F: CREATE Stub Statements (P0-6, Batch 2)

**Dependencies:** None (can start immediately, parallel with E)

These are dispatched via `dispatch_create()`:

| # | Statement | AST | Key Syntax Elements |
|---|-----------|-----|---------------------|
| 1 | CREATE CONVERSION | `CreateConversionStatement` | `FOR src_encoding TO dest_encoding FROM func` |
| 2 | CREATE SYNONYM | `CreateSynonymStatement` | `syn_name FOR obj_name [PUBLIC]` |
| 3 | CREATE MODEL | `CreateModelStatement` | `model_name USING algorithm FEATURES (...) TARGET ... FROM table` |
| 4 | CREATE AM | `CreateAmStatement` | `name TYPE index_method HANDLER handler_func` |
| 5 | CREATE DIRECTORY | `CreateDirectoryStatement` | `dir_name AS 'path'` |
| 6 | CREATE DATA SOURCE | `CreateDataSourceStatement` | `name [WITH (options)]` |
| 7 | CREATE EVENT | `CreateEventStatement` | `name ON SCHEDULE ... [DO stmt]` |
| 8 | CREATE OPCLASS | `CreateOpClassStatement` | `name USING method [TYPE type] ...` |
| 9 | CREATE OPFAMILY | `CreateOpFamilyStatement` | `name USING method` |
| 10 | CREATE CONTQUERY | `CreateContQueryStatement` | Complex |
| 11 | CREATE STREAM | `CreateStreamStatement` | Complex |
| 12 | CREATE KEY | `CreateKeyStatement` | `key_name WITH algorithm ENCRYPTION` |

### Implementation approach:
Same pattern as Work Unit E — replace stub, implement parser, wire dispatch, format, test.

Reference: GaussDB docs in `GaussDB-2.23.07.210/` directory for exact syntax.

---

## Work Unit G: ALTER Stub Statements (P0-6, Batch 3)

**Dependencies:** None (can start immediately, parallel with E, F)

These are dispatched via `dispatch_alter()`:

| # | Statement | AST | Pattern |
|---|-----------|-----|---------|
| 1 | ALTER FOREIGN TABLE | `AlterForeignTableStatement` | Similar to ALTER TABLE |
| 2 | ALTER FOREIGN SERVER | `AlterForeignServerStatement` | name + options/version |
| 3 | ALTER FDW | `AlterFdwStatement` | name + options/handler/validator |
| 4 | ALTER PUBLICATION | `AlterPublicationStatement` | name + ADD/DROP/SET tables |
| 5 | ALTER SUBSCRIPTION | `AlterSubscriptionStatement` | name + CONNECTION/PUBLICATION/ENABLE/DISABLE |
| 6 | ALTER NODE | `AlterNodeStatement` | name + HOST/PORT/... options |
| 7 | ALTER NODE GROUP | `AlterNodeGroupStatement` | name + ADD/DELETE/DROP/SET nodes |
| 8 | ALTER WORKLOAD GROUP | `AlterWorkloadGroupStatement` | name + options |
| 9 | ALTER AUDIT POLICY | `AlterAuditPolicyStatement` | name + ADD/REMOVE/MODIFY actions |
| 10 | ALTER RLS POLICY | `AlterRlsPolicyStatement` | name + ADD/REMOVE/RENAME actions |
| 11 | ALTER DATA SOURCE | `AlterDataSourceStatement` | name + SET/RESET options |
| 12 | ALTER EVENT | `AlterEventStatement` | name + ON SCHEDULE/DO/ENABLE/DISABLE |
| 13 | ALTER OPFAMILY | `AlterOpFamilyStatement` | name USING method + ADD/DROP operators |
| 14 | ALTER MATERIALIZED VIEW | NEW — needs AST | Various ALTER actions on mat view |

### Implementation approach:
- Most follow the "name + action enum" pattern similar to `AlterDatabaseStatement`
- ALTER MATERIALIZED VIEW may need a new AST struct (not currently in stub list, but in the gap analysis)

---

## Execution Order & Parallelism

```
Phase 1 (all parallel):
  ├── Work Unit A: Quick Wins (P0-4 + P0-5)     — 1-2 hours
  ├── Work Unit B: CREATE GLOBAL INDEX            — 2-3 hours
  ├── Work Unit C: ALTER TABLE PARTITION extensions — 3-4 hours
  └── Work Unit D: CREATE TABLE PARTITION extensions — 3-4 hours

Phase 2 (all parallel, after Phase 1 done):
  ├── Work Unit E: Simple stubs (10 items)        — 3-4 hours
  ├── Work Unit F: CREATE stubs (12 items)        — 4-6 hours
  └── Work Unit G: ALTER stubs (14 items)         — 4-6 hours
```

**Total estimated effort:** 20-30 hours of implementation

---

## Testing Strategy

1. **Unit tests**: Each new parser method gets 2-3 tests in `src/parser/tests.rs`
2. **Regression tests**: Run `cargo run --example regression` — must still pass 1409/1409
3. **Round-trip tests**: SQL → Parse → JSON → Deserialize → Format → SQL (must be semantically equivalent)
4. **GaussDB doc SQL**: Use example SQL from `GaussDB-2.23.07.210/sql/by_file/` as test cases

## Verification Commands

```bash
# Build check
cargo build 2>&1 | head -50

# All unit tests
cargo test 2>&1 | tail -20

# Regression tests
cargo run --example regression 2>&1 | tail -20

# Check specific test
cargo test test_name -- --nocapture
```
