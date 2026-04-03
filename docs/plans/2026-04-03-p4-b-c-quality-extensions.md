# P4-B + P4-C: Code Quality & openGauss Extensions Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Enhance the openGauss SQL parser with improved error reporting, AST visitor pattern, formatter completion, streaming API, and real parsing for openGauss-specific statement types (foreign data, replication, distributed/resource management, security policies).

**Architecture:** Infrastructure improvements (P4-B) are implemented first as they're independent and provide foundation for later work. openGauss extensions (P4-C) follow in functional clusters, each wave replacing stub structs with real AST types, implementing parsers, updating dispatch, and extending the formatter.

**Tech Stack:** Rust 2021, thiserror 2.0, existing recursive descent parser infrastructure.

---

## Current State

- **97 unit tests**, **1409/1409 regression tests pass**
- **~20 statements fully parsed** (SELECT, INSERT, UPDATE, DELETE, MERGE, CREATE/ALTER/DROP TABLE/INDEX/SEQUENCE/VIEW/SCHEMA/DATABASE/TABLESPACE, COPY, EXPLAIN, CALL, SET/SHOW/RESET, Transaction, DISCARD, TRUNCATE, CHECKPOINT)
- **32 statements stub-dispatched** (recognized keyword, skip_to_semicolon_as with stub struct)
- **~70+ statements missing dispatch** (hit default `_ => self.skip_to_semicolon()`, return Statement::Empty)
- **ParserError** has position (usize token index) but no line/column
- **Formatter** has real formatting for ~23 statement types, stubs for the rest
- **No AST visitor pattern** exists
- **No streaming parse API** exists

---

## Wave Dependency Graph

```
Wave 1 (B1: Error Enhancement) ─────────────────────┐
Wave 1 (B2: AST Visitor) ───────────────────────────┤ Independent, parallel
                                                      │
Wave 2 (B4: Streaming Parse API) ───────────────────┤ Independent
                                                      │
Wave 3 (B3: Formatter Completion) ──────────────────┘ Uses Visitor optionally
                                                      │
Wave 4 (C1: Foreign Data Wrappers) ─────────────────┐
Wave 5 (C2: Replication) ───────────────────────────┤ Sequential, each builds
Wave 6 (C3: Distributed/Resource) ──────────────────┤ on patterns established
Wave 7 (C4: Security Policies) ─────────────────────┘ by previous waves
```

---

## Wave 1: Infrastructure — Error Enhancement + AST Visitor

**Two independent tasks, can run in parallel.**

### Task 1a: Error Position Enhancement (B1)

**Files:**
- Modify: `src/parser/mod.rs` — ParserError enum, add `current_span()` helper
- Modify: `src/token/tokenizer.rs` — Add line/column tracking during tokenization
- Modify: `src/token/mod.rs` — Add `SourceLocation` struct
- Test: `src/parser/tests.rs`

**Approach:**

1. Add `SourceLocation` to `src/token/mod.rs`:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLocation {
    pub line: usize,    // 1-based
    pub column: usize,  // 1-based
    pub offset: usize,  // 0-based byte offset
}
```

2. Add `location: SourceLocation` to `TokenWithSpan` (keep existing `span: Span`):
```rust
pub struct TokenWithSpan {
    pub token: Token,
    pub span: Span,
    pub location: SourceLocation,  // NEW
}
```

3. Modify `Tokenizer` in `src/token/tokenizer.rs` to track line/column while scanning:
- Track `line: usize` (starts at 1) and `column: usize` (starts at 1)
- Increment line on `\n`, reset column
- Compute column as `current_pos - line_start_pos`

4. Enhance `ParserError` in `src/parser/mod.rs`:
```rust
pub enum ParserError {
    #[error("unexpected token at line {location.line}, column {location.column}: expected {expected}, got {got}")]
    UnexpectedToken {
        location: SourceLocation,  // was: position: usize
        expected: String,
        got: String,
    },
    #[error("unexpected end of input at line {location.line}, column {location.column}: expected {0}")]
    UnexpectedEof(String, SourceLocation),  // was: UnexpectedEof(String)
    #[error("{0}")]
    TokenizerError(#[from] crate::token::tokenizer::TokenizerError),
}
```

5. Add `fn current_location(&self) -> SourceLocation` helper to Parser that reads from current TokenWithSpan.

6. Update all error construction sites in parser to use `current_location()` instead of `self.pos`.

**Tests:**
- Test that `parse_one("SELECT")` error includes line/column info
- Test multi-line SQL error reports correct line

**Commit:** `feat(P4-B1): add line/column to error reporting`

---

### Task 1b: AST Visitor Trait (B2)

**Files:**
- Create: `src/ast/visitor.rs` — Visitor trait definition
- Modify: `src/ast/mod.rs` — Add `pub mod visitor;`
- Test: `src/parser/tests.rs`

**Approach:**

1. Create `src/ast/visitor.rs` with a Visitor trait:

```rust
/// Result type for visitor operations.
/// Control flow: Continue visiting, Skip children, or Stop entirely.
pub enum VisitorResult {
    Continue,
    SkipChildren,
    Stop,
}

/// AST Visitor trait for traversal and analysis.
/// Default implementations return Continue (visit everything).
pub trait Visitor {
    fn visit_statement(&mut self, stmt: &Statement) -> VisitorResult {
        VisitorResult::Continue
    }
    fn visit_expr(&mut self, expr: &Expr) -> VisitorResult {
        VisitorResult::Continue
    }
    fn visit_select(&mut self, select: &SelectStatement) -> VisitorResult {
        VisitorResult::Continue
    }
    fn visit_create_table(&mut self, table: &CreateTableStatement) -> VisitorResult {
        VisitorResult::Continue
    }
    fn visit_insert(&mut self, insert: &InsertStatement) -> VisitorResult {
        VisitorResult::Continue
    }
    fn visit_update(&mut self, update: &UpdateStatement) -> VisitorResult {
        VisitorResult::Continue
    }
    fn visit_delete(&mut self, delete: &DeleteStatement) -> VisitorResult {
        VisitorResult::Continue
    }
    // ... one method per fully-parsed statement type
}

/// Walk the AST using a visitor.
pub fn walk_statement(visitor: &mut dyn Visitor, stmt: &Statement) -> VisitorResult {
    let result = visitor.visit_statement(stmt);
    if result != VisitorResult::Continue {
        return result;
    }
    match stmt {
        Statement::Select(s) => walk_select(visitor, s),
        Statement::Insert(s) => walk_insert(visitor, s),
        // ... for each variant with real data
        _ => VisitorResult::Continue,
    }
}
```

2. Implement walk functions for each AST node type that recursively visit children.

**Tests:**
- Test a counting visitor that counts all Statement types
- Test a visitor that stops early (Stop result)
- Test SkipChildren behavior

**Commit:** `feat(P4-B2): add AST visitor trait with walk functions`

---

## Wave 2: Streaming Parse API (B4)

**Files:**
- Modify: `src/parser/mod.rs` — Add `StatementIter` and `parse_next()`
- Test: `src/parser/tests.rs`

**Approach:**

1. Add `StatementIter` that wraps a Parser and implements `Iterator`:

```rust
pub struct StatementIter {
    parser: Parser,
    done: bool,
}

impl Iterator for StatementIter {
    type Item = Result<Statement, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        loop {
            match self.parser.peek() {
                Token::Eof => {
                    self.done = true;
                    return None;
                }
                Token::Semicolon => {
                    self.parser.advance();
                    continue;
                }
                _ => {
                    let result = self.parser.parse_statement();
                    match result {
                        Ok(stmt) => return Some(Ok(stmt)),
                        Err(e) => {
                            self.done = true;
                            return Some(Err(e));
                        }
                    }
                }
            }
        }
    }
}
```

2. Add methods to `Parser`:

```rust
impl Parser {
    /// Create an iterator over statements from pre-tokenized input.
    pub fn into_iter(self) -> StatementIter { ... }

    /// Parse next statement from token stream.
    /// Returns None at EOF.
    pub fn parse_next(&mut self) -> Option<Result<Statement, ParserError>> { ... }
}
```

3. Also add `Tokenizer::tokenize_iter()` that returns a lazy token iterator (if feasible) — but this is optional since most use cases can tokenize upfront.

**Tests:**
- Test parsing multiple statements via iterator
- Test `parse_next()` returns None at EOF
- Test error during iteration stops the iterator

**Commit:** `feat(P4-B4): add streaming StatementIter and parse_next()`

---

## Wave 3: Formatter Completion (B3)

**Files:**
- Modify: `src/formatter.rs` — Replace stub formatting for already-parsed statement types
- Test: `src/parser/tests.rs`

**Approach:**

The formatter already handles these fully-parsed statements (format_statement lines 17-43):
- Select, Insert, Update, Delete, Merge
- CreateTable, AlterTable, Drop, Truncate
- CreateIndex, CreateSchema, CreateDatabase, CreateTablespace, CreateView, CreateSequence
- Transaction, Copy, Explain
- VariableSet, VariableShow, VariableReset, Discard, Call, Checkpoint

**What to do:**
1. Verify each existing format method produces valid/reasonable SQL
2. For any methods that are stubs or incomplete — complete them
3. Add round-trip tests: parse SQL → format → verify output is valid SQL (can be re-parsed)

**Specific improvements:**
- Ensure `format_expr` handles all `Expr` variants (check for missing ones)
- Ensure `format_data_type` handles all `DataType` variants
- Ensure `format_table_refs` handles all `TableRef` variants
- Add indent-aware formatting for nested structures

**Tests:**
- Round-trip test for each fully-parsed statement type (parse → format → parse again)
- At least 10 formatter tests

**Commit:** `feat(P4-B3): complete formatter for all parsed statement types + round-trip tests`

---

## Wave 4: Foreign Data Wrappers (C1)

**Files:**
- Modify: `src/ast/mod.rs` — Replace 6 stub structs with real AST types
- Modify: `src/parser/mod.rs` — Update dispatch_create + add dispatch_alter entries
- Create or modify: `src/parser/utility.rs` or new `src/parser/openGauss.rs` — Parser methods
- Modify: `src/formatter.rs` — Add format methods
- Test: `src/parser/tests.rs`

**Statements:**
- `CREATE FOREIGN TABLE name (...) SERVER server_name OPTIONS (...)`
- `CREATE SERVER name FOREIGN DATA WRAPPER fdw_name OPTIONS (...)`
- `CREATE FOREIGN DATA WRAPPER name OPTIONS (...)`
- `ALTER FOREIGN TABLE name ...` (add/drop column, options)
- `ALTER SERVER name OPTIONS (...)`
- `ALTER FOREIGN DATA WRAPPER name OPTIONS (...)`

**AST structs:**

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct CreateForeignTableStatement {
    pub name: ObjectName,
    pub columns: Vec<ColumnDef>,
    pub server_name: String,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateForeignServerStatement {
    pub name: String,
    pub fdw_name: Option<String>,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateFdwStatement {
    pub name: String,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AlterForeignTableStatement { _stub: () }  // Keep stub for now, implement later

#[derive(Debug, Clone, PartialEq)]
pub struct AlterForeignServerStatement { _stub: () }

#[derive(Debug, Clone, PartialEq)]
pub struct AlterFdwStatement { _stub: () }
```

**Note:** For ALTER variants, we can keep stub structs initially and only implement CREATE parsing. ALTER can be added in a follow-up wave.

**Parser approach:**
- CREATE FOREIGN TABLE: Parse like CREATE TABLE but with SERVER clause
- CREATE SERVER: Parse name + optional TYPE/VERSION + FOREIGN DATA WRAPPER + OPTIONS
- CREATE FOREIGN DATA WRAPPER: Parse name + HANDLER/VALIDATOR + OPTIONS
- Use existing `parse_options_list()` or create `parse_generic_options()` for key=value pairs

**Tests:**
- Test each CREATE statement with realistic SQL
- Test with and without OPTIONS
- Test error on malformed input

**Commit:** `feat(P4-C1): implement foreign data wrapper statement parsing`

---

## Wave 5: Replication — PUBLICATION / SUBSCRIPTION (C2)

**Files:**
- Modify: `src/ast/mod.rs` — Replace 4 stub structs
- Modify: `src/parser/mod.rs` — Update dispatch
- Modify: `src/parser/utility.rs` or openGauss module
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**Statements:**
- `CREATE PUBLICATION name FOR TABLE table_name [, ...] | FOR ALL TABLES`
- `CREATE PUBLICATION name FOR ALL TABLES WITH (option = value, ...)`
- `CREATE SUBSCRIPTION name CONNECTION 'conninfo' PUBLICATION pub_name WITH (option = value, ...)`
- `ALTER PUBLICATION name ADD TABLE / SET TABLE / DROP TABLE`
- `ALTER SUBSCRIPTION name CONNECTION / PUBLICATION / SET / REFRESH`

**AST structs:**

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct CreatePublicationStatement {
    pub name: String,
    pub for_tables: Option<Vec<ObjectName>>,  // None = ALL TABLES
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateSubscriptionStatement {
    pub name: String,
    pub connection: String,
    pub publications: Vec<String>,
    pub options: Vec<(String, String)>,
}

// ALTER variants keep stub structs for now
```

**Tests:**
- CREATE PUBLICATION for specific tables
- CREATE PUBLICATION FOR ALL TABLES
- CREATE SUBSCRIPTION with connection and publication
- ALTER variants (stub-level)

**Commit:** `feat(P4-C2): implement PUBLICATION/SUBSCRIPTION parsing`

---

## Wave 6: Distributed / Resource Management (C3)

**Files:**
- Modify: `src/ast/mod.rs` — Replace 8 stub structs
- Modify: `src/parser/mod.rs` — Update dispatch
- Modify parser module
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**Statements:**
- `CREATE NODE name WITH (option = value, ...)`
- `CREATE NODE GROUP name WITH (node_name, ...) DISTRIBUTE BY ...`
- `CREATE RESOURCE POOL name WITH (option = value, ...)`
- `CREATE WORKLOAD GROUP name WITH (option = value, ...)`
- ALTER variants for each

**AST structs:**

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct CreateNodeStatement {
    pub name: String,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateNodeGroupStatement {
    pub name: String,
    pub nodes: Vec<String>,
    pub distribute: Option<String>,  // DISTRIBUTE BY clause
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateResourcePoolStatement {
    pub name: String,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateWorkloadGroupStatement {
    pub name: String,
    pub options: Vec<(String, String)>,
}

// ALTER variants keep stub structs for now
```

**Tests:**
- Each CREATE statement with realistic options
- ALTER variants (stub-level)

**Commit:** `feat(P4-C3): implement NODE/NODE GROUP/RESOURCE POOL/WORKLOAD GROUP parsing`

---

## Wave 7: Security Policies (C4)

**Files:**
- Modify: `src/ast/mod.rs` — Replace 6 stub structs
- Modify: `src/parser/mod.rs` — Update dispatch
- Modify parser module
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**Statements:**
- `CREATE AUDIT POLICY name [DEFAULT] PRIVILEGES privilege_list ON object_type FILTER BY filter_expr`
- `CREATE MASKING POLICY name (column_name data_type [, ...]) WITH (option = value)`
- `CREATE RLS POLICY name ON table_name [AS restrictive/permissive] USING (expr) WITH CHECK (expr)`
- ALTER variants for each

**AST structs:**

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct CreateAuditPolicyStatement {
    pub name: String,
    pub privileges: Vec<String>,
    pub on_object: Option<String>,
    pub filter: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateMaskingPolicyStatement {
    pub name: String,
    pub columns: Vec<(String, String)>,  // (name, type)
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateRlsPolicyStatement {
    pub name: String,
    pub table: ObjectName,
    pub permissive: Option<bool>,  // Some(true)=PERMISSIVE, Some(false)=RESTRICTIVE, None=default
    pub using_expr: Option<String>,
    pub with_check_expr: Option<String>,
}

// ALTER variants keep stub structs for now
```

**Tests:**
- Each CREATE statement with realistic SQL
- ALTER variants (stub-level)

**Commit:** `feat(P4-C4): implement AUDIT/MASKING/RLS POLICY parsing`

---

## Cross-Wave Requirements

### After each wave:
1. Run `cargo test` — all tests pass
2. Run `cargo run --example regression` — 1409/1409 still pass
3. Run `lsp_diagnostics` on all modified files — no errors
4. Update `src/formatter.rs` for any newly-parsed statements
5. Commit with descriptive message

### Final verification:
1. Count `skip_to_semicolon_as` calls — should decrease with each P4-C wave
2. Count stub_struct! entries — should decrease
3. Formatter coverage should increase
4. All 1409 regression tests still pass
5. `cargo clippy` passes with no warnings

---

## Summary

| Wave | Items | New Parsed Statements | Est. Tests Added |
|------|-------|----------------------|-----------------|
| 1 | B1 + B2 | 0 (infrastructure) | ~6 |
| 2 | B4 | 0 (API) | ~4 |
| 3 | B3 | 0 (formatter) | ~10 |
| 4 | C1 | 3 CREATE + 3 ALTER stubs | ~8 |
| 5 | C2 | 2 CREATE + 2 ALTER stubs | ~6 |
| 6 | C3 | 4 CREATE + 4 ALTER stubs | ~8 |
| 7 | C4 | 3 CREATE + 3 ALTER stubs | ~6 |
| **Total** | **P4-B + P4-C** | **12 new parsed CREATE statements** | **~48 new tests** |
