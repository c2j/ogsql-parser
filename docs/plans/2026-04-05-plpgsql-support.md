# PL/pgSQL Support Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement PL/pgSQL block parsing with full statement-level AST support for anonymous blocks, DO statements, and function/procedure bodies.

**Architecture:** PL/pgSQL is a separate procedural language with its own 15,770-line grammar in openGauss. Our parser already tokenizes PL/pgSQL-specific tokens (`:=`, `..`, `%`, `<<`, `>>`). We add a dedicated `src/parser/plpgsql.rs` module and `src/ast/plpgsql.rs` module. The main parser dispatches to the PL/pgSQL parser when it detects DO statements, anonymous blocks (`DECLARE...BEGIN...END` or `BEGIN...END` at top level), and optionally function/procedure bodies.

**Tech Stack:** Rust, existing tokenizer (no changes needed), recursive descent parsing.

**Key Architectural Insight:** openGauss uses a two-stage approach — main gram.y extracts body text, then PL/pgSQL gram.y parses it with its own scanner. We do the same in one pass: the main parser recognizes PL/pgSQL contexts and delegates to `parse_pl_block()`.

---

## Tokenizer: No Changes Needed

The existing tokenizer already produces all tokens needed for PL/pgSQL:
- `Token::ColonEquals` (`:=`) — PL/pgSQL assignment
- `Token::DotDot` (`..`) — range operator in FOR loops
- `Token::Percent` (`%`) — for `%TYPE` and `%ROWTYPE`
- `Token::Op("<<")` / `Token::Op(">>")` — for labels `<<label>>`
- `Token::DollarString(String)` — dollar-quoted strings
- All SQL keywords needed (IF, THEN, ELSE, BEGIN, END, LOOP, etc.)

---

## Wave 1: Block Structure & Variable Declarations

### Task 1: Define PL/pgSQL AST Types (Block & Declarations)

**Files:**
- Create: `src/ast/plpgsql.rs`
- Modify: `src/ast/mod.rs` (add `pub mod plpgsql;`)

**Step 1: Create `src/ast/plpgsql.rs` with block and declaration types**

```rust
//! PL/pgSQL AST types for procedural language blocks.

use serde::Serialize;

// ── Block Structure ──

/// A PL/pgSQL block: [label:] [DECLARE decls] BEGIN stmts [EXCEPTION handlers] END [label]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlBlock {
    pub label: Option<String>,
    pub declarations: Vec<PlDeclaration>,
    pub body: Vec<PlStatement>,
    pub exception_block: Option<PlExceptionBlock>,
    pub end_label: Option<String>,
}

// ── Declarations ──

/// Declaration in a DECLARE section.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PlDeclaration {
    Variable(PlVarDecl),
    Cursor(PlCursorDecl),
    Record(PlRecordDecl),
    Type(PlTypeDecl),
    Constant(PlConstantDecl),
}

/// Variable declaration: name [CONSTANT] type [NOT NULL] [:= expr | DEFAULT expr] [COLLATE name]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlVarDecl {
    pub name: String,
    pub data_type: PlDataType,
    pub default: Option<String>,
    pub constant: bool,
    pub not_null: bool,
    pub collate: Option<String>,
}

/// Constant declaration: name CONSTANT type [NOT NULL] := expr
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlConstantDecl {
    pub name: String,
    pub data_type: PlDataType,
    pub value: String,
    pub not_null: bool,
}

/// PL/pgSQL data types.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PlDataType {
    /// Regular type name (e.g., INTEGER, TEXT, VARCHAR(100))
    TypeName(String),
    /// Anchored type: table.column%TYPE
    PercentType { table: String, column: String },
    /// Anchored row type: table%ROWTYPE
    PercentRowType(String),
    /// RECORD type
    Record,
    /// CURSOR type (for cursor variables)
    Cursor,
    /// REFCURSOR type
    RefCursor,
    /// TYPE OF expression
    TypeOf(String),
}

/// Cursor declaration: cursor_name [([args])] [SCROLL] CURSOR [(return_type)] FOR query
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlCursorDecl {
    pub name: String,
    pub arguments: Vec<PlCursorArg>,
    pub return_type: Option<PlDataType>,
    pub query: String,
    pub scrollable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlCursorArg {
    pub name: String,
    pub data_type: PlDataType,
    pub mode: PlArgMode,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PlArgMode {
    In,
    Out,
    InOut,
}

/// Record type declaration: name RECORD
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlRecordDecl {
    pub name: String,
}

/// TYPE declaration (composite type): name IS RECORD (fields)
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlTypeDecl {
    pub name: String,
    pub fields: Vec<PlTypeField>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlTypeField {
    pub name: String,
    pub data_type: PlDataType,
}

// ── Statements ──

/// A PL/pgSQL statement.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PlStatement {
    /// Nested block
    Block(PlBlock),

    /// Assignment: target := expr
    Assignment {
        target: String,
        expression: String,
    },

    /// IF condition THEN stmts [ELSIF condition THEN stmts]... [ELSE stmts] END IF
    If(PlIfStmt),

    /// CASE [expr] WHEN condition THEN stmts... [ELSE stmts] END CASE
    Case(PlCaseStmt),

    /// Basic loop: [label:] LOOP stmts END LOOP [label]
    Loop(PlLoopStmt),

    /// WHILE loop: [label:] WHILE condition LOOP stmts END LOOP [label]
    While(PlWhileStmt),

    /// FOR loop: [label:] FOR var IN [REVERSE] low..high [BY step] LOOP stmts END LOOP [label]
    /// or: [label:] FOR var IN cursor [([args])] LOOP stmts END LOOP [label]
    /// or: [label:] FOR var IN [REVERSE] query LOOP stmts END LOOP [label]
    For(PlForStmt),

    /// FOREACH loop: [label:] FOREACH var IN ARRAY expr [SLICE n] LOOP stmts END LOOP [label]
    ForEach(PlForEachStmt),

    /// EXIT [label] [WHEN condition]
    Exit {
        label: Option<String>,
        condition: Option<String>,
    },

    /// CONTINUE [label] [WHEN condition]
    Continue {
        label: Option<String>,
        condition: Option<String>,
    },

    /// RETURN [expression]
    Return {
        expression: Option<String>,
    },

    /// RAISE [level] format [, args...] [USING option = value ...]
    Raise(PlRaiseStmt),

    /// EXECUTE format_string [INTO target] [USING expr, ...]
    Execute(PlExecuteStmt),

    /// PERFORM query
    Perform {
        query: String,
    },

    /// OPEN cursor_name [([args])]
    /// OPEN cursor_name FOR query
    /// OPEN cursor_name FOR USING expr, ...
    Open(PlOpenStmt),

    /// FETCH cursor_name INTO target
    /// FETCH [direction [FROM | IN]] cursor INTO target
    Fetch(PlFetchStmt),

    /// CLOSE cursor_name
    Close {
        cursor: String,
    },

    /// MOVE [direction [FROM | IN]] cursor
    Move {
        cursor: String,
        direction: Option<String>,
    },

    /// GET [STACKED] DIAGNOSTICS var = item [, var = item ...]
    GetDiagnostics(PlGetDiagStmt),

    /// COMMIT [WORK]
    Commit,

    /// ROLLBACK [WORK | TO savepoint_name]
    Rollback {
        to_savepoint: Option<String>,
    },

    /// SAVEPOINT name
    Savepoint {
        name: String,
    },

    /// NULL statement (no-op)
    Null,

    /// GOTO label
    Goto {
        label: String,
    },

    /// Raw SQL statement (SELECT, INSERT, UPDATE, DELETE, etc.)
    Sql(String),

    /// FORALL statement
    ForAll(PlForAllStmt),

    /// PIPE ROW(expr)
    PipeRow {
        expression: String,
    },
}

// ── Statement Detail Types ──

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlIfStmt {
    pub condition: String,
    pub then_stmts: Vec<PlStatement>,
    pub elsifs: Vec<PlElsif>,
    pub else_stmts: Vec<PlStatement>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlElsif {
    pub condition: String,
    pub stmts: Vec<PlStatement>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlCaseStmt {
    /// None for searched CASE (CASE WHEN ... THEN ...)
    pub expression: Option<String>,
    pub whens: Vec<PlCaseWhen>,
    pub else_stmts: Vec<PlStatement>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlCaseWhen {
    pub condition: String,
    pub stmts: Vec<PlStatement>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlLoopStmt {
    pub label: Option<String>,
    pub body: Vec<PlStatement>,
    pub end_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlWhileStmt {
    pub label: Option<String>,
    pub condition: String,
    pub body: Vec<PlStatement>,
    pub end_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlForStmt {
    pub label: Option<String>,
    pub variable: String,
    pub kind: PlForKind,
    pub body: Vec<PlStatement>,
    pub end_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PlForKind {
    /// FOR i IN low..high [BY step] LOOP
    Range {
        low: String,
        high: String,
        step: Option<String>,
        reverse: bool,
    },
    /// FOR rec IN query LOOP
    Query {
        query: String,
    },
    /// FOR rec IN cursor_name [([args])] LOOP
    Cursor {
        cursor_name: String,
        arguments: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlForEachStmt {
    pub label: Option<String>,
    pub variable: String,
    pub expression: String,
    pub slice: Option<i32>,
    pub body: Vec<PlStatement>,
    pub end_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlForAllStmt {
    pub variable: String,
    pub bounds: String,  // low..high
    pub body: String,    // the DML statement
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlRaiseStmt {
    pub level: Option<RaiseLevel>,
    pub message: Option<String>,
    pub options: Vec<RaiseOption>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RaiseLevel {
    Debug,
    Log,
    Info,
    Notice,
    Warning,
    Exception,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RaiseOption {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlExecuteStmt {
    pub string_expr: String,
    pub into_target: Option<String>,
    pub using_args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlOpenStmt {
    pub cursor: String,
    pub kind: PlOpenKind,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PlOpenKind {
    /// OPEN cursor [([args])]
    Simple { arguments: Vec<String> },
    /// OPEN cursor FOR query
    ForQuery { query: String },
    /// OPEN cursor FOR USING expr, ...
    ForUsing { expressions: Vec<String> },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlFetchStmt {
    pub cursor: String,
    pub direction: Option<String>,  // NEXT, PRIOR, FIRST, LAST, ABSOLUTE n, RELATIVE n, FORWARD n, BACKWARD n, ALL
    pub into: String,  // target variable(s)
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlGetDiagStmt {
    pub stacked: bool,
    pub items: Vec<PlGetDiagItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlGetDiagItem {
    pub target: String,
    pub item: String,
}

// ── Exception Handling ──

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlExceptionBlock {
    pub handlers: Vec<PlExceptionHandler>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlExceptionHandler {
    pub conditions: Vec<String>,
    pub statements: Vec<PlStatement>,
}
```

**Step 2: Add `pub mod plpgsql;` to `src/ast/mod.rs`**

At the top of `src/ast/mod.rs`, add:
```rust
pub mod plpgsql;
```

**Step 3: Verify it compiles**

Run: `cargo check`
Expected: No errors (the module is declared but nothing uses it yet)

**Step 4: Commit**

```
git add src/ast/plpgsql.rs src/ast/mod.rs
git commit -m "feat(plpgsql): define PL/pgSQL AST types for blocks, declarations, statements"
```

---

### Task 2: PL/pgSQL Parser Skeleton — Block & Declaration Parsing

**Files:**
- Create: `src/parser/plpgsql.rs`
- Modify: `src/parser/mod.rs` (add `pub mod plpgsql;`)

**Step 1: Create `src/parser/plpgsql.rs` with core parsing functions**

This is the largest file. Key functions:

```rust
//! PL/pgSQL block parser.
//!
//! Parses PL/pgSQL procedural language blocks including:
//! - Anonymous blocks (DECLARE ... BEGIN ... END)
//! - DO statement bodies
//! - Function/procedure bodies

use crate::ast::plpgsql::*;
use crate::token::keyword::Keyword;
use crate::token::{Token, TokenWithSpan};
use crate::parser::ParserError;

/// Extension trait for PL/pgSQL-specific token matching on Parser.
pub(crate) trait PlpgsqlParse {
    /// Parse a complete PL/pgSQL block: [label:] [DECLARE decls] BEGIN stmts [EXCEPTION handlers] END [label]
    fn parse_pl_block(&mut self) -> Result<PlBlock, ParserError>;

    /// Parse the DECLARE section (variable, cursor, type declarations)
    fn parse_pl_declarations(&mut self) -> Result<Vec<PlDeclaration>, ParserError>;

    /// Parse a single variable declaration
    fn parse_pl_var_decl(&mut self) -> Result<PlDeclaration, ParserError>;

    /// Parse a PL/pgSQL data type
    fn parse_pl_data_type(&mut self) -> Result<PlDataType, ParserError>;

    /// Parse statement list until a terminator (END, ELSE, ELSIF, EXCEPTION, or EOF)
    fn parse_pl_statements(&mut self) -> Result<Vec<PlStatement>, ParserError>;

    /// Parse a single PL/pgSQL statement
    fn parse_pl_statement(&mut self) -> Result<PlStatement, ParserError>;

    /// Check if current token is a label (<<ident>>)
    fn try_parse_pl_label(&mut self) -> Option<String>;

    /// Check if current position is a DECLARE keyword followed by variable declarations
    /// (vs a cursor declaration like DECLARE cursor_name CURSOR)
    fn is_pl_declare_block(&mut self) -> bool;
}
```

**Step 2: Implement `parse_pl_block()`**

```rust
impl PlpgsqlParse for crate::parser::Parser {
    fn parse_pl_block(&mut self) -> Result<PlBlock, ParserError> {
        // 1. Optional label: <<label>>
        let label = self.try_parse_pl_label();

        // 2. Optional DECLARE section
        let declarations = if self.peek_keyword() == Some(Keyword::DECLARE) {
            self.advance(); // consume DECLARE
            self.parse_pl_declarations()?
        } else {
            Vec::new()
        };

        // 3. BEGIN keyword
        self.expect_keyword(Keyword::BEGIN_P)?;

        // 4. Statement body (until EXCEPTION or END)
        let mut body = Vec::new();
        let mut exception_block = None;

        loop {
            match self.peek_keyword() {
                Some(Keyword::EXCEPTION) => {
                    self.advance();
                    exception_block = Some(self.parse_pl_exception_block()?);
                }
                Some(Keyword::END_P) => {
                    self.advance();
                    break;
                }
                _ => {
                    let stmt = self.parse_pl_statement()?;
                    body.push(stmt);
                }
            }
        }

        // 5. Optional END label
        let end_label = self.try_parse_pl_label();

        Ok(PlBlock {
            label,
            declarations,
            body,
            exception_block,
            end_label,
        })
    }

    fn try_parse_pl_label(&mut self) -> Option<String> {
        if self.match_token(&Token::Op("<<".to_string())) {
            self.advance();
            let label = self.parse_identifier().ok()?;
            if self.match_token(&Token::Op(">>".to_string())) {
                self.advance();
                return Some(label);
            }
        }
        None
    }
}
```

**Step 3: Implement `parse_pl_declarations()`**

Handle these patterns:
- `name [CONSTANT] type [NOT NULL] [:= expr | DEFAULT expr] [COLLATE name]`
- `name CURSOR [([args])] [FOR query]`
- `name RECORD`
- `name TYPE IS RECORD (field type, ...)`
- `name table%ROWTYPE`
- `name table.column%TYPE`

**Step 4: Implement `parse_pl_statements()` and `parse_pl_statement()` dispatch**

Dispatch on the next keyword/token to the appropriate statement parser:
- `IF` → `parse_pl_if()`
- `CASE` → `parse_pl_case()`
- `LOOP` → `parse_pl_loop()`
- `WHILE` → `parse_pl_while()`
- `FOR` → `parse_pl_for()`
- `FOREACH` → `parse_pl_foreach()`
- `EXIT` → `parse_pl_exit()`
- `CONTINUE` → `parse_pl_continue()`
- `RETURN` → `parse_pl_return()`
- `RAISE` → `parse_pl_raise()`
- `EXECUTE` → `parse_pl_execute()`
- `PERFORM` → `parse_pl_perform()`
- `OPEN` → `parse_pl_open()`
- `FETCH` → `parse_pl_fetch()`
- `CLOSE` → `parse_pl_close()`
- `MOVE` → `parse_pl_move()`
- `GET` → `parse_pl_get_diagnostics()`
- `COMMIT` → `PlStatement::Commit`
- `ROLLBACK` → `parse_pl_rollback()`
- `SAVEPOINT` → `parse_pl_savepoint()`
- `NULL` → `PlStatement::Null`
- `GOTO` → `parse_pl_goto()`
- `FORALL` → `parse_pl_forall()`
- `PIPE` → `parse_pl_pipe_row()`
- `<<` → label prefix, then parse labeled statement
- Otherwise → parse as SQL statement (assignment or raw SQL)

**Step 5: Add `pub mod plpgsql;` to `src/parser/mod.rs`**

**Step 6: Verify compilation**

Run: `cargo check`
Expected: Compile errors for unimplemented statement parsers — that's fine, we implement them in Wave 2-3.

**Step 7: Commit**

```
git add src/parser/plpgsql.rs src/parser/mod.rs
git commit -m "feat(plpgsql): implement block and declaration parsing skeleton"
```

---

## Wave 2: Control Flow Statements

### Task 3: IF / CASE / LOOP / WHILE / FOR / FOREACH

**Files:**
- Modify: `src/parser/plpgsql.rs`

Implement each control flow parser:

**IF statement:**
```
IF condition THEN
    stmts
[ELSIF condition THEN
    stmts]...
[ELSE
    stmts]
END IF
```

**CASE statement:**
```
CASE [expression]
    WHEN condition THEN stmts
    ...
    [ELSE stmts]
END CASE
```

**LOOP / WHILE / FOR:**
```
[label:] LOOP stmts END LOOP [label]
[label:] WHILE condition LOOP stmts END LOOP [label]
[label:] FOR var IN [REVERSE] low..high [BY step] LOOP stmts END LOOP [label]
[label:] FOR var IN cursor LOOP stmts END LOOP [label]
[label:] FOR var IN query LOOP stmts END LOOP [label]
[label:] FOREACH var IN ARRAY expr [SLICE n] LOOP stmts END LOOP [label]
```

**EXIT / CONTINUE / RETURN:**
```
EXIT [label] [WHEN condition]
CONTINUE [label] [WHEN condition]
RETURN [expression]
```

**Test cases (add to `src/parser/tests.rs`):**
```rust
#[test]
fn test_plpgsql_if_simple() {
    let sql = "DO $$ BEGIN IF x > 0 THEN y := 1; END IF; END $$";
    let stmt = parse_one(sql);
    assert!(matches!(stmt, Statement::Do(_)));
}

#[test]
fn test_plpgsql_if_elsif_else() {
    let sql = "DO $$ BEGIN IF x > 0 THEN y := 1; ELSIF x = 0 THEN y := 0; ELSE y := -1; END IF; END $$";
    let stmt = parse_one(sql);
    assert!(matches!(stmt, Statement::Do(_)));
}

#[test]
fn test_plpgsql_loop() {
    let sql = "DO $$ BEGIN LOOP EXIT WHEN x > 10; x := x + 1; END LOOP; END $$";
    let stmt = parse_one(sql);
    assert!(matches!(stmt, Statement::Do(_)));
}

#[test]
fn test_plpgsql_while() {
    let sql = "DO $$ BEGIN WHILE x < 10 LOOP x := x + 1; END LOOP; END $$";
    let stmt = parse_one(sql);
    assert!(matches!(stmt, Statement::Do(_)));
}

#[test]
fn test_plpgsql_for_range() {
    let sql = "DO $$ BEGIN FOR i IN 1..10 LOOP x := x + i; END LOOP; END $$";
    let stmt = parse_one(sql);
    assert!(matches!(stmt, Statement::Do(_)));
}

#[test]
fn test_plpgsql_for_reverse() {
    let sql = "DO $$ BEGIN FOR i IN REVERSE 10..1 LOOP x := x + i; END LOOP; END $$";
    let stmt = parse_one(sql);
    assert!(matches!(stmt, Statement::Do(_)));
}

#[test]
fn test_plpgsql_for_query() {
    let sql = "DO $$ BEGIN FOR rec IN SELECT * FROM t LOOP x := rec.id; END LOOP; END $$";
    let stmt = parse_one(sql);
    assert!(matches!(stmt, Statement::Do(_)));
}

#[test]
fn test_plpgsql_case() {
    let sql = "DO $$ BEGIN CASE x WHEN 1 THEN y := 'a'; WHEN 2 THEN y := 'b'; ELSE y := 'c'; END CASE; END $$";
    let stmt = parse_one(sql);
    assert!(matches!(stmt, Statement::Do(_)));
}

#[test]
fn test_plpgsql_foreach() {
    let sql = "DO $$ BEGIN FOREACH x IN ARRAY arr LOOP y := y + x; END LOOP; END $$";
    let stmt = parse_one(sql);
    assert!(matches!(stmt, Statement::Do(_)));
}

#[test]
fn test_plpgsql_labeled_loop() {
    let sql = "DO $$ <<outer>> BEGIN FOR i IN 1..3 LOOP <<inner>> LOOP EXIT outer WHEN i = 2; END LOOP inner; END LOOP; END $$";
    let stmt = parse_one(sql);
    assert!(matches!(stmt, Statement::Do(_)));
}
```

---

## Wave 3: Advanced Features

### Task 4: Exception Handling, RAISE, EXECUTE, Cursors, Transaction Statements

**Files:**
- Modify: `src/parser/plpgsql.rs`

**Exception handling:**
```
EXCEPTION
    WHEN condition [OR condition]... THEN
        stmts
    ...
```

**RAISE:**
```
RAISE [DEBUG | LOG | INFO | NOTICE | WARNING | EXCEPTION] 'format' [, args...]
RAISE EXCEPTION 'error: % is too small', val USING ERRCODE = '22012';
RAISE condition_name;
RAISE;
```

**EXECUTE:**
```
EXECUTE 'SELECT * FROM ' || table_name;
EXECUTE 'INSERT INTO t VALUES ($1)' USING val;
EXECUTE 'SELECT * FROM t' INTO rec;
```

**Cursor operations:**
```
OPEN cursor_name;
OPEN cursor_name FOR SELECT ...;
OPEN cursor_name USING val1, val2;
FETCH cursor_name INTO var;
FETCH NEXT FROM cursor INTO var;
CLOSE cursor_name;
MOVE ABSOLUTE 5 IN cursor;
```

**Transaction in blocks:**
```
COMMIT;
ROLLBACK;
ROLLBACK TO savepoint_name;
SAVEPOINT sp_name;
```

**GET DIAGNOSTICS:**
```
GET DIAGNOSTICS var = ROW_COUNT;
GET STACKED DIAGNOSTICS var = MESSAGE_TEXT, var2 = PG_EXCEPTION_DETAIL;
```

**Test cases:**
```rust
#[test]
fn test_plpgsql_exception_handling() {
    let sql = "DO $$ BEGIN INSERT INTO t VALUES (1); EXCEPTION WHEN unique_violation THEN RAISE NOTICE 'duplicate'; END $$";
    let stmt = parse_one(sql);
    assert!(matches!(stmt, Statement::Do(_)));
}

#[test]
fn test_plpgsql_raise() {
    let sql = "DO $$ BEGIN RAISE NOTICE 'value is %', x; END $$";
    let stmt = parse_one(sql);
    assert!(matches!(stmt, Statement::Do(_)));
}

#[test]
fn test_plpgsql_raise_exception() {
    let sql = "DO $$ BEGIN RAISE EXCEPTION 'not found' USING ERRCODE = '02000'; END $$";
    let stmt = parse_one(sql);
    assert!(matches!(stmt, Statement::Do(_)));
}

#[test]
fn test_plpgsql_execute() {
    let sql = "DO $$ BEGIN EXECUTE 'CREATE TABLE ' || tbl || ' (id int)'; END $$";
    let stmt = parse_one(sql);
    assert!(matches!(stmt, Statement::Do(_)));
}

#[test]
fn test_plpgsql_cursor_open_fetch_close() {
    let sql = "DO $$ DECLARE c CURSOR FOR SELECT id FROM t; r RECORD; BEGIN OPEN c; FETCH c INTO r; CLOSE c; END $$";
    let stmt = parse_one(sql);
    assert!(matches!(stmt, Statement::Do(_)));
}
```

---

## Wave 4: Integration & Formatting

### Task 5: Wire Up DO Statement, Anonymous Blocks, Function Bodies

**Files:**
- Modify: `src/ast/mod.rs` (update `DoStatement`, `AnonyBlockStatement`)
- Modify: `src/parser/mod.rs` (update dispatch for BEGIN_P, DECLARE, DO)
- Modify: `src/parser/utility.rs` (update `parse_do()`)
- Modify: `src/formatter.rs` (add PL/pgSQL formatters)

**Step 1: Update `DoStatement` to use parsed PL/pgSQL block**

In `src/ast/mod.rs`, change `DoStatement`:
```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct DoStatement {
    pub language: Option<String>,
    pub block: Option<crate::ast::plpgsql::PlBlock>,
    /// Raw code string (used when block parsing fails or language is not plpgsql)
    pub code: Option<String>,
}
```

**Step 2: Replace `AnonyBlockStatement` stub with real struct**

Remove from `stub_struct!` macro and add:
```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct AnonyBlockStatement {
    pub has_declare: bool,
    pub block: crate::ast::plpgsql::PlBlock,
}
```

**Step 3: Update `parse_do()` in `src/parser/utility.rs`**

```rust
pub(crate) fn parse_do(&mut self) -> Result<DoStatement, ParserError> {
    let mut language = None;

    if self.try_consume_keyword(Keyword::LANGUAGE) {
        language = Some(self.parse_identifier()?);
    }

    // Try to parse the body as a PL/pgSQL block
    if let Some(code) = self.try_consume_dollar_string() {
        // Parse the dollar-quoted content as PL/pgSQL
        // (re-tokenize the inner content and parse as PlBlock)
        let block = self.parse_pl_block_from_string(&code);
        return Ok(DoStatement { language, block: Some(block), code: None });
    }

    // Fallback: collect raw code
    let code = self.skip_to_semicolon_and_collect();
    Ok(DoStatement { language, block: None, code: Some(code) })
}
```

**Step 4: Update `BEGIN_P` dispatch in `src/parser/mod.rs`**

The key challenge: distinguish `BEGIN;` (transaction) from `BEGIN ... END` (PL/pgSQL block).

Heuristic: After `BEGIN`, if the next token is `END_P` or a statement keyword (not `;`), it's a PL/pgSQL block.

```rust
Token::Keyword(Keyword::BEGIN_P) | Token::Keyword(Keyword::START) => {
    self.advance();
    // Check if this is a transaction BEGIN or a PL/pgSQL block
    // Transaction: BEGIN; or BEGIN TRANSACTION; or BEGIN ISOLATION LEVEL ...
    // PL/pgSQL block: BEGIN <statement>...END;
    if self.match_token(&Token::Semicolon) || self.peek_keyword() == Some(Keyword::TRANSACTION)
        || self.peek_keyword() == Some(Keyword::ISOLATION)
        || self.peek_keyword() == Some(Keyword::DEFERRABLE)
        || self.peek_keyword() == Some(Keyword::READ) {
        // Transaction
        match self.parse_transaction_begin() {
            Ok(stmt) => {
                self.try_consume_semicolon();
                crate::ast::Statement::Transaction(stmt)
            }
            Err(e) => {
                self.add_error(e);
                self.skip_to_semicolon()
            }
        }
    } else {
        // PL/pgSQL anonymous block (BEGIN ... END)
        match self.parse_anonymous_block(false) {
            Ok(stmt) => {
                self.try_consume_semicolon();
                crate::ast::Statement::AnonyBlock(stmt)
            }
            Err(e) => {
                self.add_error(e);
                self.skip_to_semicolon()
            }
        }
    }
}
```

**Step 5: Update `DECLARE` dispatch similarly**

After `DECLARE`, check if it's a cursor declaration or an anonymous block.

Heuristic: `DECLARE cursor_name CURSOR` = cursor. `DECLARE var_name type` = anonymous block.

**Step 6: Add PL/pgSQL formatter methods**

Add `format_pl_block()`, `format_pl_statement()`, etc. to `src/formatter.rs`.

---

### Task 6: Comprehensive Tests

**Files:**
- Modify: `src/parser/tests.rs`

Add 20+ tests covering:
1. Simple anonymous blocks (DECLARE...BEGIN...END, BEGIN...END)
2. DO statements with dollar-quoted PL/pgSQL
3. Variable declarations (all types)
4. All control flow (IF, CASE, LOOP, WHILE, FOR range/query/cursor, FOREACH)
5. Exception handling
6. RAISE, EXECUTE, PERFORM
7. Cursor operations (OPEN, FETCH, CLOSE)
8. Nested blocks
9. Labeled blocks and GOTO
10. Transaction statements in blocks
11. Complex real-world examples from openGauss test suite

---

## Real-World Test Cases (from openGauss regression tests)

These are actual PL/pgSQL patterns from the openGauss test suite that the parser should handle:

```sql
-- Simple trigger function (from plpgsql.sql)
CREATE FUNCTION tg_room_au() RETURNS trigger AS $$
BEGIN
    IF new.roomno != old.roomno THEN
        UPDATE WSlot SET roomno = new.roomno WHERE roomno = old.roomno;
    END IF;
    RETURN new;
END;
$$ LANGUAGE plpgsql;

-- With DECLARE section (from plpgsql.sql)
CREATE FUNCTION tg_pslot_biu() RETURNS trigger AS $$
DECLARE
    pfrec RECORD;
    ps ALIAS FOR new;
BEGIN
    SELECT INTO pfrec * FROM PField WHERE name = ps.pfname;
    IF NOT FOUND THEN
        RAISE EXCEPTION 'Patchfield "%" does not exist', ps.pfname;
    END IF;
    RETURN ps;
END;
$$ LANGUAGE plpgsql;

-- Exception handling (from plpgsql.sql)
CREATE FUNCTION tg_iface_biu() RETURNS trigger AS $$
DECLARE
    sname TEXT;
    sysrec RECORD;
BEGIN
    SELECT INTO sysrec * FROM system WHERE name = new.sysname;
    IF NOT FOUND THEN
        RAISE EXCEPTION 'system "%" does not exist', new.sysname;
    END IF;
    sname := 'IF.' || new.sysname;
    sname := sname || '.';
    sname := sname || new.ifname;
    IF length(sname) > 20 THEN
        RAISE EXCEPTION 'IFace slotname "%" too long (20 char max)', sname;
    END IF;
    new.slotname := sname;
    RETURN new;
EXCEPTION
    WHEN no_data_found THEN
        RAISE EXCEPTION 'system "%" does not exist', new.sysname;
END;
$$ LANGUAGE plpgsql;

-- FOR integer range loop
CREATE FUNCTION tg_hub_adjustslots(hname bpchar, oldnslots integer, newnslots integer)
RETURNS integer AS $$
BEGIN
    FOR i IN oldnslots + 1 .. newnslots LOOP
        INSERT INTO HSlot (slotname, hubname, slotno, slotlink)
        VALUES ('HS.dummy', hname, i, '');
    END LOOP;
    RETURN 0;
END;
$$ LANGUAGE plpgsql;

-- DO statement
DO $$ DECLARE r RECORD; BEGIN FOR r IN SELECT * FROM t LOOP RAISE NOTICE '%', r.id; END LOOP; END $$;

-- Anonymous block
DECLARE v_count INTEGER; BEGIN SELECT COUNT(*) INTO v_count FROM t; RAISE NOTICE 'count: %', v_count; END;

-- WHILE loop
DO $$ BEGIN
    DECLARE x INT := 0;
    WHILE x < 10 LOOP
        x := x + 1;
    END LOOP;
END $$;
```

---

## Execution Strategy

**Total estimated effort:** ~4 waves, each wave ~1-2 hours of implementation

**Dependencies:**
- Wave 1 (Tasks 1-2) must be done first
- Wave 2 (Task 3) depends on Wave 1
- Wave 3 (Task 4) depends on Wave 1
- Wave 2 and Wave 3 can be done in parallel
- Wave 4 (Tasks 5-6) depends on Waves 2+3

**Parallel execution opportunity:** Tasks 3 and 4 (control flow + advanced features) are independent and can be parallelized.
