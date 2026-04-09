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
    NestedProcedure(crate::ast::PackageProcedure),
    NestedFunction(crate::ast::PackageFunction),
    Pragma { name: String, arguments: String },
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
}

/// Cursor declaration: cursor_name [([args])] CURSOR [(return_type)] FOR query
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlCursorDecl {
    pub name: String,
    pub arguments: Vec<PlCursorArg>,
    pub return_type: Option<PlDataType>,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed_query: Option<Box<crate::ast::Statement>>,
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
    Assignment { target: String, expression: String },

    /// IF condition THEN stmts [ELSIF condition THEN stmts]... [ELSE stmts] END IF
    If(PlIfStmt),

    /// CASE [expr] WHEN condition THEN stmts... [ELSE stmts] END CASE
    Case(PlCaseStmt),

    /// Basic loop: [label:] LOOP stmts END LOOP [label]
    Loop(PlLoopStmt),

    /// WHILE loop: [label:] WHILE condition LOOP stmts END LOOP [label]
    While(PlWhileStmt),

    /// FOR loop (range, query, or cursor)
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
    Return { expression: Option<String> },

    /// RAISE [level] format [, args...] [USING option = value ...]
    Raise(PlRaiseStmt),

    /// EXECUTE format_string [INTO target] [USING expr, ...]
    Execute(PlExecuteStmt),

    /// PERFORM query
    Perform {
        query: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        parsed_query: Option<Box<crate::ast::Statement>>,
    },

    /// OPEN cursor (simple, for query, or for using)
    Open(PlOpenStmt),

    /// FETCH cursor INTO target
    Fetch(PlFetchStmt),

    /// CLOSE cursor_name
    Close { cursor: String },

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
    Rollback { to_savepoint: Option<String> },

    /// SAVEPOINT name
    Savepoint { name: String },

    /// NULL statement (no-op)
    Null,

    /// GOTO label
    Goto { label: String },

    /// Procedure or function call: name[(args...)]
    ProcedureCall(PlProcedureCall),

    #[serde(rename = "sql_text")]
    Sql(String),

    SqlStatement {
        sql_text: String,
        #[serde(flatten)]
        statement: Box<crate::ast::Statement>,
    },

    /// FORALL statement
    ForAll(PlForAllStmt),

    /// PIPE ROW(expr)
    PipeRow { expression: String },
}

// ── Statement Detail Types ──

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlProcedureCall {
    pub name: crate::ast::ObjectName,
    pub arguments: Vec<String>,
}

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
        #[serde(skip_serializing_if = "Option::is_none")]
        parsed_query: Option<Box<crate::ast::Statement>>,
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
    pub bounds: String,
    pub body: String,
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
    ForQuery {
        query: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        parsed_query: Option<Box<crate::ast::Statement>>,
    },
    /// OPEN cursor FOR USING expr, ...
    ForUsing { expressions: Vec<String> },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PlFetchStmt {
    pub cursor: String,
    pub direction: Option<String>,
    pub into: String,
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
