# P5: Parser Coverage Expansion — Deepen + New Statements

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Expand parser coverage from ~18% to ~60% by deepening existing parsers and implementing high-impact stubs.

**Architecture:** All changes go through the existing Parser struct in `src/parser/`. AST types in `src/ast/mod.rs` get expanded/replaced. Each wave adds AST types + parser methods + tests + formatter methods. All three interfaces (CLI/HTTP/TUI) automatically benefit since they share the same parser core.

**Tech Stack:** Rust, thiserror, serde (for JSON serialization)

---

## Wave 1: Deepen SELECT — FOR UPDATE/SHARE, FETCH FIRST

**Files:**
- Modify: `src/ast/mod.rs` — Add `LockClause` to `SelectStatement`
- Modify: `src/parser/select.rs` — Parse `FOR UPDATE/SHARE` and `FETCH FIRST/N ROWS`
- Modify: `src/formatter.rs` — Format new fields
- Test: `src/parser/tests.rs`

**AST changes:**
```rust
// Add to SelectStatement
pub lock_clause: Option<LockClause>,

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum LockClause {
    Update { tables: Vec<ObjectName>, nowait: bool },
    Share { tables: Vec<ObjectName>, nowait: bool },
    NoKeyUpdate { tables: Vec<ObjectName>, nowait: bool },
    KeyShare { tables: Vec<ObjectName>, nowait: bool },
}
```

**Parser changes (select.rs `parse_order_limit_offset`):**
- After OFFSET, parse optional `FOR UPDATE/SHARE/NO KEY UPDATE/KEY SHARE`
- Parse optional `OF table_list` and `NOWAIT/SKIP LOCKED`
- In `parse_order_limit_offset`, also handle `FETCH FIRST/N ROWS ONLY` as alternative to LIMIT

**Tests:**
- `SELECT * FROM t FOR UPDATE`
- `SELECT * FROM t FOR SHARE OF t1, t2 NOWAIT`
- `SELECT * FROM t FOR KEY SHARE SKIP LOCKED`
- `SELECT * FROM t FETCH FIRST 10 ROWS ONLY`
- `SELECT * FROM t FETCH FIRST ROW WITH TIES`

---

## Wave 2: Deepen INSERT — ON CONFLICT

**Files:**
- Modify: `src/ast/mod.rs` — Add `OnConflict` to `InsertStatement`
- Modify: `src/parser/dml.rs` — Parse `ON CONFLICT ... DO NOTHING/UPDATE SET`
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**AST changes:**
```rust
pub struct InsertStatement {
    // existing fields...
    pub conflict_action: Option<OnConflictAction>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum OnConflictAction {
    Nothing,
    Update { target: Option<OnConflictTarget>, assignments: Vec<UpdateAssignment>, where_clause: Option<Expr> },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum OnConflictTarget {
    Columns(Vec<String>),
    OnConstraint(String),
}
```

**Tests:**
- `INSERT INTO t (a) VALUES (1) ON CONFLICT DO NOTHING`
- `INSERT INTO t (a, b) VALUES (1, 2) ON CONFLICT (a) DO UPDATE SET b = EXCLUDED.b`
- `INSERT INTO t (a, b) VALUES (1, 2) ON CONFLICT ON CONSTRAINT pk_t DO UPDATE SET b = t.b + 1 WHERE t.c > 0`

---

## Wave 3: Deepen CREATE TABLE — TEMP/UNLOGGED, WITH, PARTITION BY, TABLESPACE, INHERITS, LIKE

**Files:**
- Modify: `src/ast/mod.rs` — Expand `CreateTableStatement`
- Modify: `src/parser/ddl.rs` — Parse new clauses
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**AST changes:**
```rust
pub struct CreateTableStatement {
    pub temporary: bool,         // TEMP | TEMPORARY
    pub unlogged: bool,          // UNLOGGED
    pub if_not_exists: bool,
    pub name: ObjectName,
    pub columns: Vec<ColumnDef>,
    pub constraints: Vec<TableConstraint>,
    pub inherits: Vec<ObjectName>,
    pub partition_by: Option<PartitionClause>,
    pub tablespace: Option<String>,
    pub on_commit: Option<OnCommitAction>,
    pub options: Vec<(String, String)>,  // WITH (...)
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum PartitionClause {
    Range { column: ObjectName },
    List { column: ObjectName },
    Hash { column: ObjectName },
    Interval { column: ObjectName },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum OnCommitAction {
    PreserveRows,
    DeleteRows,
    Drop,
}
```

**Tests:**
- `CREATE TEMP TABLE t (id INT)`
- `CREATE UNLOGGED TABLE t (id INT)`
- `CREATE TABLE t (id INT) WITH (fillfactor=70, autovacuum_enabled=false)`
- `CREATE TABLE t (id INT) TABLESPACE ts1`
- `CREATE TABLE t (id INT) INHERITS (parent1, parent2)`
- `CREATE TABLE t (id INT) LIKE other_table INCLUDING DEFAULTS`
- `CREATE TABLE t (id INT, dt DATE) PARTITION BY RANGE (dt)`
- `CREATE TABLE t (id INT) ON COMMIT DELETE ROWS`

---

## Wave 4: Deepen ALTER TABLE — ADD/DROP CONSTRAINT, RENAME, OWNER, SET SCHEMA

**Files:**
- Modify: `src/ast/mod.rs` — Expand `AlterTableAction`
- Modify: `src/parser/ddl.rs` — Parse new actions
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**AST changes:**
```rust
pub enum AlterTableAction {
    // existing: AddColumn, DropColumn, AlterColumn
    AddConstraint { name: Option<String>, constraint: TableConstraint },
    DropConstraint { name: String, if_exists: bool, cascade: bool },
    RenameColumn { old: String, new: String },
    RenameTo { new_name: String },
    OwnerTo { owner: String },
    SetSchema { schema: String },
}
```

**Tests:**
- `ALTER TABLE t ADD CONSTRAINT pk PRIMARY KEY (id)`
- `ALTER TABLE t ADD UNIQUE (email)`
- `ALTER TABLE t DROP CONSTRAINT ck CHECK (x > 0) CASCADE`
- `ALTER TABLE t RENAME COLUMN old_name TO new_name`
- `ALTER TABLE t RENAME TO new_table`
- `ALTER TABLE t OWNER TO admin`
- `ALTER TABLE t SET SCHEMA new_schema`

---

## Wave 5: Expand DROP + CREATE INDEX

**Files:**
- Modify: `src/ast/mod.rs` — Expand `ObjectType`, `CreateIndexStatement`
- Modify: `src/parser/ddl.rs` — More DROP types, INDEX CONCURRENTLY/TABLESPACE
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**AST changes:**
```rust
pub enum ObjectType {
    Table, Index, Sequence, View, Schema, Database,
    Tablespace, Function, Procedure, Trigger, Extension,
    MaterializedView, ForeignTable, ForeignServer, Fdw,
}
```

**Tests:**
- `DROP TABLESPACE ts1`
- `DROP FUNCTION func1(INT, TEXT)`
- `DROP TRIGGER trig1 ON table1`
- `DROP MATERIALIZED VIEW mv1`
- `CREATE INDEX CONCURRENTLY idx ON t (col)`
- `CREATE INDEX idx ON t (col) TABLESPACE ts1`

---

## Wave 6: GRANT / REVOKE

**Files:**
- Modify: `src/ast/mod.rs` — Replace stub `GrantStatement`/`RevokeStatement`
- Modify: `src/parser/mod.rs` — Replace stub dispatch with real parsers
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**AST changes:**
```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct GrantStatement {
    pub privileges: Vec<Privilege>,
    pub target: GrantTarget,
    pub grantees: Vec<String>,
    pub with_grant_option: bool,
    pub granted_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum Privilege {
    All,
    Select,
    Insert,
    Update,
    Delete,
    Usage,
    Create,
    Connect,
    Temporary,
    Execute,
    Trigger,
    References,
    // ...more as needed
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum GrantTarget {
    Table(Vec<ObjectName>),
    Schema(Vec<String>),
    Database(Vec<String>),
    Function(Vec<ObjectName>),
    AllTablesInSchema(Vec<String>),
    AllFunctionsInSchema(Vec<String>),
    AllSequencesInSchema(Vec<String>),
}

// Similar structure for RevokeStatement
```

**Tests:**
- `GRANT SELECT, INSERT ON table1 TO user1, user2`
- `GRANT ALL PRIVILEGES ON SCHEMA public TO admin WITH GRANT OPTION`
- `GRANT USAGE, CREATE ON DATABASE mydb TO dev`
- `GRANT EXECUTE ON FUNCTION func1(INT) TO public`
- `GRANT SELECT ON ALL TABLES IN SCHEMA public TO reader`
- `REVOKE INSERT ON table1 FROM user1`
- `REVOKE ALL ON SCHEMA public FROM admin CASCADE`

---

## Wave 7: CREATE FUNCTION / PROCEDURE

**Files:**
- Modify: `src/ast/mod.rs` — Replace stubs
- Modify: `src/parser/mod.rs` — Replace stub dispatch
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**AST changes:**
```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct CreateFunctionStatement {
    pub or_replace: bool,
    pub name: ObjectName,
    pub args: Vec<FuncArg>,
    pub returns: Option<CreateFuncReturn>,
    pub language: String,
    pub body: String,           // dollar-quoted or string literal
    pub volatility: Option<FuncVolatility>,
    pub strict: bool,
    pub called_on_null: Option<FuncNullBehavior>,
    pub security: Option<FuncSecurity>,
    pub set_options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum FuncArg {
    { name: Option<String>, data_type: DataType, default: Option<Expr>, mode: FuncArgMode },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum FuncArgMode { In, Out, InOut, Variadic }

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum FuncVolatility { Volatile, Stable, Immutable }

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum FuncNullBehavior { CalledOnNullInput, ReturnsNullOnNullInput, Strict }

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum FuncSecurity { Definer, Invoker }

// Similar for CreateProcedureStatement (no RETURNS)
```

**Tests:**
- `CREATE OR REPLACE FUNCTION add(a INT, b INT) RETURNS INT AS $$ SELECT a + b $$ LANGUAGE SQL`
- `CREATE FUNCTION get_users() RETURNS SETOF users LANGUAGE plpgsql AS $$ BEGIN RETURN QUERY SELECT * FROM users; END; $$`
- `CREATE PROCEDURE audit_log(msg TEXT) LANGUAGE plpgsql AS $$ BEGIN INSERT INTO logs(msg) VALUES(msg); END; $$`

---

## Wave 8: CREATE TRIGGER + CREATE MATERIALIZED VIEW

**Files:**
- Modify: `src/ast/mod.rs` — Replace stubs
- Modify: `src/parser/mod.rs` — Replace stub dispatch
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**AST changes:**
```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct CreateTriggerStatement {
    pub name: String,
    pub or_replace: bool,
    pub constraint: bool,
    pub table: ObjectName,
    pub events: Vec<TriggerEvent>,
    pub for_each: TriggerForEach,
    pub when: Option<Expr>,
    pub func_name: ObjectName,
    pub func_args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum TriggerEvent { Insert, Update, UpdateOf(Vec<String>), Delete, Truncate }

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum TriggerForEach { Row, Statement }

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct CreateMaterializedViewStatement {
    pub if_not_exists: bool,
    pub name: ObjectName,
    pub columns: Vec<String>,
    pub query: Box<SelectStatement>,
    pub tablespace: Option<String>,
    pub with_data: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct RefreshMatViewStatement {
    pub concurrent: bool,
    pub name: ObjectName,
}
```

**Tests:**
- `CREATE TRIGGER trg AFTER INSERT OR UPDATE ON t FOR EACH ROW EXECUTE PROCEDURE func1()`
- `CREATE TRIGGER trg BEFORE DELETE ON t FOR EACH ROW WHEN (OLD.status = 'active') EXECUTE PROCEDURE func2()`
- `CREATE MATERIALIZED VIEW mv AS SELECT * FROM t WITH DATA`
- `CREATE MATERIALIZED VIEW IF NOT EXISTS mv AS SELECT * FROM t TABLESPACE ts1`
- `REFRESH MATERIALIZED VIEW mv`
- `REFRESH MATERIALIZED VIEW CONCURRENTLY mv`

---

## Wave 9: VACUUM / ANALYZE / COMMENT ON / LOCK TABLE

**Files:**
- Modify: `src/ast/mod.rs` — Replace stubs
- Modify: `src/parser/mod.rs` — Replace stub dispatch
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**AST changes:**
```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct VacuumStatement {
    pub full: bool,
    pub verbose: bool,
    pub analyze: bool,
    pub freeze: bool,
    pub tables: Vec<VacuumTarget>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct VacuumTarget {
    pub name: ObjectName,
    pub columns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct AnalyzeStatement {
    pub verbose: bool,
    pub tables: Vec<VacuumTarget>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct CommentStatement {
    pub object_type: String,
    pub name: ObjectName,
    pub comment: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct LockStatement {
    pub tables: Vec<ObjectName>,
    pub mode: String,
    pub nowait: bool,
}
```

**Tests:**
- `VACUUM`
- `VACUUM FULL VERBOSE ANALYZE t1, t2`
- `VACUUM (FREEZE) t1 (col1, col2)`
- `ANALYZE`
- `ANALYZE VERBOSE t1`
- `COMMENT ON TABLE t IS 'my table'`
- `COMMENT ON COLUMN t.id IS 'primary key'`
- `LOCK TABLE t1, t2 IN ACCESS EXCLUSIVE MODE NOWAIT`

---

## Wave 10: PREPARE / EXECUTE / DEALLOCATE / DO

**Files:**
- Modify: `src/ast/mod.rs` — Replace stubs
- Modify: `src/parser/mod.rs` — Replace stub dispatch
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**AST changes:**
```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct PrepareStatement {
    pub name: String,
    pub data_types: Vec<DataType>,
    pub statement: Box<Statement>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ExecuteStatement {
    pub name: String,
    pub params: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct DeallocateStatement {
    pub name: String,
    pub all: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct DoStatement {
    pub language: Option<String>,
    pub code: String,
}
```

**Tests:**
- `PREPARE stmt (INT, TEXT) AS SELECT * FROM t WHERE id = $1 AND name = $2`
- `EXECUTE stmt(1, 'test')`
- `DEALLOCATE stmt`
- `DEALLOCATE PREPARE stmt`
- `DEALLOCATE ALL`
- `DO $$ BEGIN CREATE TABLE t (id INT); END $$`
- `DO LANGUAGE plpgsql $$ BEGIN RAISE NOTICE 'hello'; END $$`

---

## Wave 11: ALTER DATABASE / SCHEMA / SEQUENCE / FUNCTION / ROLE / USER / SYSTEM

**Files:**
- Modify: `src/ast/mod.rs` — Replace stubs with real types
- Modify: `src/parser/mod.rs` — Implement `dispatch_alter` entries
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**New parsers for `dispatch_alter`:**
- `ALTER DATABASE name SET/RESET config_param`
- `ALTER DATABASE name RENAME TO new_name`
- `ALTER SCHEMA name RENAME TO new_name`
- `ALTER SCHEMA name OWNER TO user`
- `ALTER SEQUENCE name INCREMENT BY / RESTART WITH / ...`
- `ALTER FUNCTION name(...) RENAME TO / OWNER TO / SET SCHEMA`
- `ALTER ROLE name WITH options`
- `ALTER USER name WITH options`
- `ALTER SYSTEM SET config_param = value`

**Tests:**
- `ALTER DATABASE mydb SET search_path TO public`
- `ALTER DATABASE mydb RENAME TO newdb`
- `ALTER SCHEMA myschema RENAME TO newschema`
- `ALTER SEQUENCE seq INCREMENT BY 2 RESTART WITH 100`
- `ALTER FUNCTION add(INT, INT) RENAME TO sum_it`
- `ALTER ROLE admin WITH PASSWORD 'secret'`
- `ALTER SYSTEM SET max_connections = 200`

---

## Wave 12: Remaining stubs — CURSOR / LISTEN / NOTIFY / RULE / CLUSTER / REINDEX

**Files:**
- Modify: `src/ast/mod.rs` — Replace stubs
- Modify: `src/parser/mod.rs` — Replace stub dispatch
- Modify: `src/formatter.rs`
- Test: `src/parser/tests.rs`

**AST changes:**
```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct DeclareCursorStatement {
    pub name: String,
    pub args: Vec<Expr>,
    pub query: Box<SelectStatement>,
    pub hold: bool,        // WITH HOLD
    pub binary: bool,      // BINARY
    pub scroll: bool,      // SCROLL / NO SCROLL / INSENSITIVE
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct FetchStatement {
    pub direction: FetchDirection,
    pub cursor_name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum FetchDirection {
    Next, Prior, First, Last,
    Absolute(i64),
    Relative(i64),
    ForwardAll, BackwardAll,
    Forward(i64), Backward(i64),
    Count(i64),   // just a number
    All,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ClosePortalStatement { pub name: String }

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ListenStatement { pub channel: String }
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct NotifyStatement { pub channel: String, pub payload: Option<String> }
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct UnlistenStatement { pub channel: Option<String> }

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct RuleStatement {
    pub name: String,
    pub table: ObjectName,
    pub event: String,
    pub condition: Option<Expr>,
    pub instead: bool,
    pub actions: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ClusterStatement { pub table: Option<ObjectName>, pub verbose: bool }
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ReindexStatement { pub target: ReindexTarget, pub verbose: bool, pub concurrent: bool }
```

**Tests:**
- `DECLARE cur CURSOR FOR SELECT * FROM t`
- `DECLARE cur BINARY SCROLL CURSOR WITH HOLD FOR SELECT * FROM t`
- `FETCH NEXT FROM cur`
- `FETCH 10 FROM cur`
- `FETCH ALL FROM cur`
- `CLOSE cur`
- `LISTEN mychannel`
- `NOTIFY mychannel, 'payload'`
- `UNLISTEN mychannel`
- `CLUSTER VERBOSE t1`
- `REINDEX TABLE t1`
- `REINDEX INDEX CONCURRENTLY idx1`
