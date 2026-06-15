// Pre-existing code issues that became deny-by-warning in Rust 1.93.
// These are style issues & pre-existing dead code (not bugs). Fix gradually.
#![allow(
    // Parser pattern: matching different keywords/tokens often leads to identical
    // handling (advance + same action). Each branch is intentionally separate for
    // grammar-rule readability. Combining conditions would obscure the grammar.
    clippy::if_same_then_else,
    clippy::unwrap_used,
    clippy::large_enum_variant,
    clippy::ptr_arg,
    clippy::should_implement_trait,
    clippy::unnecessary_literal_unwrap,
    clippy::result_large_err,
    unexpected_cfgs,
    unreachable_patterns,
    dead_code,
    unused_assignments,
    unused_macros
)]

pub mod analyzer;
pub mod ast;
pub mod formatter;
pub mod linter;
pub mod parser;
pub mod token;
pub mod token_formatter;

pub use analyzer::return_cursor::{
    analyze_return_cursors, has_return_cursors, ResultColumn, ReturnCursorAnnotation, ReturnCursorBranch,
    ReturnCursorGroup, RoutineReturnAnalysis,
};
pub use analyzer::schema::{
    collect_ddl_schema, load_full_schema, load_schema, resolve_schema, FullSchema, IndexMapV2, SchemaMap,
    SchemaResolutionReport,
};
pub use analyzer::{
    analyze_pl_block, analyze_transactions, compute_query_fingerprints, validate_merge_semantics,
    validate_package_consistency, validate_pl_variables, validate_pl_variables_with_extra_vars,
    validate_pl_variables_with_extra_vars_and_funcs, DynamicSqlReport, MergeSemanticError, MergeSemanticErrorKind,
    PackageConsistencyError, PackageConsistencyErrorKind, QueryFingerprint, TransactionReport, UndefinedRefKind,
    UndefinedVariableError,
};

pub use ast::visitor::{walk_pl_block, walk_pl_declaration, walk_pl_statement, walk_statement, Visitor, VisitorResult};
pub use ast::StatementInfo;
pub use ast::{
    AlterTableAction, AlterTableStatement, ColumnConstraint, ColumnDef, CopyStatement, CreateDatabaseStatement,
    CreateGlobalIndexStatement, CreateIndexStatement, CreateLanguageStatement, CreateSchemaStatement,
    CreateTableStatement, CreateTablespaceStatement, CreateViewStatement, CreateWeakPasswordDictStatement, DataType,
    DeleteStatement, DistributeClause, DropStatement, ExecuteDirectStatement, ExplainStatement, Expr,
    GlobalIndexColumn, IndexNulls, IndexOrdering, InsertStatement, Literal, MergeStatement, MoveStatement, ObjectName,
    PartitionClause, PartitionDef, PredictByStatement, SelectIntoTable, SelectStatement, Statement, TableConstraint,
    TableRef, TransactionKind, TransactionStatement, TruncateStatement, UpdateStatement, ValuesStatement, WindowSpec,
};
pub use formatter::SqlFormatter;
pub use parser::{CommentInfo, ParseOptions, ParseOutput, Parser, ParserError, StatementIter};
pub use token::tokenizer::{Tokenizer, TokenizerError};
pub use token::{Keyword, SourceLocation, Span, Token, TokenWithSpan};
pub use token_formatter::{CommaStyle, FormatConfig, KeywordCase};

#[cfg(feature = "ibatis")]
pub mod ibatis;

#[cfg(feature = "java")]
pub mod java;

#[cfg(feature = "mcp")]
pub mod mcp;
