//! PL/pgSQL AST types for procedural language blocks.

use serde::{Deserialize, Serialize};
use std::fmt;

// ── Block Structure ──

/// A PL/pgSQL block: [label:] [DECLARE decls] BEGIN stmts [EXCEPTION handlers] END [label]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlBlock {
    pub label: Option<String>,
    pub declarations: Vec<PlDeclaration>,
    pub body: Vec<PlStatement>,
    pub exception_block: Option<PlExceptionBlock>,
    pub end_label: Option<String>,
}

// ── Declarations ──

/// Declaration in a DECLARE section.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlVarDecl {
    pub name: String,
    pub data_type: PlDataType,
    pub default: Option<crate::ast::Expr>,
    pub constant: bool,
    pub not_null: bool,
    pub collate: Option<String>,
}

/// PL/pgSQL data types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlCursorDecl {
    pub name: String,
    pub arguments: Vec<PlCursorArg>,
    pub return_type: Option<PlDataType>,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed_query: Option<Box<crate::ast::Statement>>,
    pub scrollable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlCursorArg {
    pub name: String,
    pub data_type: PlDataType,
    pub mode: PlArgMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlArgMode {
    In,
    Out,
    InOut,
}

/// Record type declaration: name RECORD
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlRecordDecl {
    pub name: String,
}

/// TYPE declaration (composite type): name IS RECORD (fields)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlTypeDecl {
    pub name: String,
    pub fields: Vec<PlTypeField>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlTypeField {
    pub name: String,
    pub data_type: PlDataType,
}

// ── Statements ──

/// A PL/pgSQL statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlStatement {
    /// Nested block
    Block(PlBlock),

    /// Assignment: target := expr
    Assignment {
        target: String,
        expression: crate::ast::Expr,
    },

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
        condition: Option<crate::ast::Expr>,
    },

    /// CONTINUE [label] [WHEN condition]
    Continue {
        label: Option<String>,
        condition: Option<crate::ast::Expr>,
    },

    /// RETURN [expression]
    Return {
        expression: Option<crate::ast::Expr>,
    },

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
        direction: Option<FetchDirection>,
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
    PipeRow { expression: crate::ast::Expr },
}

// ── Statement Detail Types ──

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlProcedureCall {
    pub name: crate::ast::ObjectName,
    pub arguments: Vec<crate::ast::Expr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlIfStmt {
    pub condition: crate::ast::Expr,
    pub then_stmts: Vec<PlStatement>,
    pub elsifs: Vec<PlElsif>,
    pub else_stmts: Vec<PlStatement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlElsif {
    pub condition: crate::ast::Expr,
    pub stmts: Vec<PlStatement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlCaseStmt {
    pub expression: Option<crate::ast::Expr>,
    pub whens: Vec<PlCaseWhen>,
    pub else_stmts: Vec<PlStatement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlCaseWhen {
    pub condition: crate::ast::Expr,
    pub stmts: Vec<PlStatement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlLoopStmt {
    pub label: Option<String>,
    pub body: Vec<PlStatement>,
    pub end_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlWhileStmt {
    pub label: Option<String>,
    pub condition: crate::ast::Expr,
    pub body: Vec<PlStatement>,
    pub end_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlForStmt {
    pub label: Option<String>,
    pub variable: String,
    pub kind: PlForKind,
    pub body: Vec<PlStatement>,
    pub end_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlForKind {
    /// FOR i IN low..high [BY step] LOOP
    Range {
        low: crate::ast::Expr,
        high: crate::ast::Expr,
        step: Option<crate::ast::Expr>,
        reverse: bool,
    },
    /// FOR rec IN query LOOP
    Query {
        query: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        parsed_query: Option<Box<crate::ast::Statement>>,
        #[serde(default)]
        using_args: Vec<PlUsingArg>,
    },
    /// FOR rec IN cursor_name [([args])] LOOP
    Cursor {
        cursor_name: String,
        arguments: Vec<crate::ast::Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlForEachStmt {
    pub label: Option<String>,
    pub variable: String,
    pub expression: crate::ast::Expr,
    pub slice: Option<i32>,
    pub body: Vec<PlStatement>,
    pub end_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlForAllStmt {
    pub variable: String,
    pub bounds: String,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlRaiseStmt {
    pub level: Option<RaiseLevel>,
    pub message: Option<String>,
    pub options: Vec<RaiseOption>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RaiseLevel {
    Debug,
    Log,
    Info,
    Notice,
    Warning,
    Exception,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RaiseOption {
    pub name: String,
    pub value: String,
}

/// Parameter passing mode for EXECUTE IMMEDIATE ... USING arguments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlUsingMode {
    In,
    Out,
    InOut,
}

/// A single argument in a USING clause with its passing mode.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlUsingArg {
    pub mode: PlUsingMode,
    pub argument: crate::ast::Expr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlExecuteStmt {
    /// Whether the IMMEDIATE keyword was present
    pub immediate: bool,
    /// The dynamic SQL string expression (may be a concatenation)
    pub string_expr: crate::ast::Expr,
    /// INTO target variables for query results
    #[serde(default)]
    pub into_targets: Vec<crate::ast::Expr>,
    /// USING arguments with IN/OUT/INOUT mode
    #[serde(default)]
    pub using_args: Vec<PlUsingArg>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlOpenStmt {
    pub cursor: String,
    pub kind: PlOpenKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlOpenKind {
    /// OPEN cursor [([args])]
    Simple { arguments: Vec<crate::ast::Expr> },
    /// OPEN cursor FOR query
    ForQuery {
        query: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        parsed_query: Option<Box<crate::ast::Statement>>,
    },
    /// OPEN cursor FOR USING expr, ...
    ForUsing { expressions: Vec<crate::ast::Expr> },
}

/// Direction keyword for FETCH and MOVE statements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FetchDirection {
    Next,
    Prior,
    First,
    Last,
    Forward,
    Backward,
    Absolute,
    Relative,
    All,
}

impl fmt::Display for FetchDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchDirection::Next => write!(f, "NEXT"),
            FetchDirection::Prior => write!(f, "PRIOR"),
            FetchDirection::First => write!(f, "FIRST"),
            FetchDirection::Last => write!(f, "LAST"),
            FetchDirection::Forward => write!(f, "FORWARD"),
            FetchDirection::Backward => write!(f, "BACKWARD"),
            FetchDirection::Absolute => write!(f, "ABSOLUTE"),
            FetchDirection::Relative => write!(f, "RELATIVE"),
            FetchDirection::All => write!(f, "ALL"),
        }
    }
}

/// GET DIAGNOSTICS item kinds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetDiagItemKind {
    RowCount,
    ResultStatus,
    ReturnedSqlstate,
    MessageText,
    Detail,
    Hint,
    Context,
    SchemaName,
    TableName,
    ColumnName,
    DatatypeName,
    ConstraintName,
    PgExceptionContext,
}

impl fmt::Display for GetDiagItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetDiagItemKind::RowCount => write!(f, "ROW_COUNT"),
            GetDiagItemKind::ResultStatus => write!(f, "RESULT_STATUS"),
            GetDiagItemKind::ReturnedSqlstate => write!(f, "RETURNED_SQLSTATE"),
            GetDiagItemKind::MessageText => write!(f, "MESSAGE_TEXT"),
            GetDiagItemKind::Detail => write!(f, "DETAIL"),
            GetDiagItemKind::Hint => write!(f, "HINT"),
            GetDiagItemKind::Context => write!(f, "CONTEXT"),
            GetDiagItemKind::SchemaName => write!(f, "SCHEMA_NAME"),
            GetDiagItemKind::TableName => write!(f, "TABLE_NAME"),
            GetDiagItemKind::ColumnName => write!(f, "COLUMN_NAME"),
            GetDiagItemKind::DatatypeName => write!(f, "DATATYPE_NAME"),
            GetDiagItemKind::ConstraintName => write!(f, "CONSTRAINT_NAME"),
            GetDiagItemKind::PgExceptionContext => write!(f, "PG_EXCEPTION_CONTEXT"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlFetchStmt {
    pub cursor: String,
    pub direction: Option<FetchDirection>,
    pub into: crate::ast::Expr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlGetDiagStmt {
    pub stacked: bool,
    pub items: Vec<PlGetDiagItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlGetDiagItem {
    pub target: String,
    pub item: GetDiagItemKind,
}

// ── Exception Handling ──

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlExceptionBlock {
    pub handlers: Vec<PlExceptionHandler>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlExceptionHandler {
    pub conditions: Vec<String>,
    pub statements: Vec<PlStatement>,
}
