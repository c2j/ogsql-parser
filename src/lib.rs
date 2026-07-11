//! A hand-written recursive descent SQL parser for openGauss/GaussDB.
//!
//! Supports the full openGauss SQL dialect including DML, DDL, PL/pgSQL,
//! and GaussDB-specific extensions. All AST types implement [`serde::Serialize`]
//! and [`serde::Deserialize`] for lossless JSON round-trip.
//!
//! # Quick start
//!
//! ```
//! use ogsql_parser::{Tokenizer, parser::Parser};
//!
//! let sql = "SELECT id, name FROM users WHERE status = 'active'";
//! let tokens = Tokenizer::new(sql).tokenize()?;
//! let statements = Parser::new(tokens).parse();
//! # Ok::<(), ogsql_parser::TokenizerError>(())
//! ```
//!
//! # Validation
//!
//! Run PACKAGE consistency, MERGE semantics, and PL variable validation in one call,
//! with typed errors preserved (no folding into `ParserError`):
//!
//! ```
//! use ogsql_parser::{Parser, validate_statements};
//!
//! let (stmts, _) = Parser::parse_sql(
//!     "MERGE INTO t USING s ON t.id = s.id WHEN MATCHED THEN DELETE",
//! );
//! let report = validate_statements(&stmts, &[], false);
//! if !report.merge_errors.is_empty() {
//!     println!("MERGE issues: {}", report.merge_errors.len());
//! }
//! ```
//!
//! # Features
//!
//! - **Default**: Library only (tokenizer, parser, AST, formatter, analyzer, linter)
//! - `cli`: Command-line binary with `parse`, `format`, `validate`, `tokenize`
//! - `ibatis`: iBatis/MyBatis XML mapper parsing
//! - `java`: Java source SQL extraction (tree-sitter)
//! - `serve`: HTTP API server (axum)
//! - `tui`: Interactive terminal playground (ratatui)
//! - `mcp`: Model Context Protocol server for AI tools
//!
//! See the [project README](https://github.com/User/ogsql-parser) for full documentation.

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
pub use analyzer::validate::{
    collect_defined_routine_names, validate_pl_variables_from_stmts, validate_statements, ValidationReport,
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
    GlobalIndexColumn, Ident, IndexNulls, IndexOrdering, InsertStatement, Literal, MergeStatement, MoveStatement,
    ObjectName, PartitionClause, PartitionDef, PredictByStatement, SelectIntoTable, SelectStatement, Statement,
    TableConstraint, TableRef, TransactionKind, TransactionStatement, TruncateStatement, UpdateStatement,
    ValuesStatement, WindowSpec,
};
pub use formatter::SqlFormatter;
pub use parser::{CommentInfo, ParseOptions, ParseOutput, Parser, ParserError, StatementIter};
pub use token::tokenizer::{Tokenizer, TokenizerError};
pub use token::{Keyword, SourceLocation, Span, Token, TokenWithSpan};
pub use token_formatter::{CommaStyle, FormatConfig, KeywordCase};

/// Returns `true` if a `ParserError` is a warning (not a hard error).
///
/// Warnings are `ParserError::Warning { .. }` and
/// `ParserError::ReservedKeywordAsIdentifier { .. }`.
/// Everything else is considered a hard error.
pub fn is_warning(e: &ParserError) -> bool {
    matches!(e, ParserError::Warning { .. } | ParserError::ReservedKeywordAsIdentifier { .. })
}

/// Translate JDBC `{call ...}` / `{? = call ...}` escape syntax to native `CALL` SQL.
///
/// Also handles MyBatis/iBatis flattened parameter placeholders in place of `?`:
/// - `{__XML_PARAM_VARCHAR_result__ = call func(args)}` → `CALL func(args)`
/// - `{__XML_RAW_procName__ = call func(args)}` → `CALL func(args)`
///
/// The core SQL parser only understands bare `CALL pkg.proc(args)`. JDBC escape
/// wrappers (`{call ...}`, `{? = call ...}`) must be stripped before parsing.
///
/// Tolerates whitespace/line-breaks between tokens (e.g. `{\ncall proc()}`,
/// `{?\n= call proc()}`), which can appear in iBatis/MyBatis XML mapper text.
///
/// Idempotent — if the input does not start with a JDBC escape pattern, it is
/// returned unchanged.
/// Check that `call` is followed by a word boundary (whitespace, `(`, `}`, or end-of-string).
/// Prevents false matches on identifiers like `callpkg` that start with "call".
fn is_jdbc_call_boundary(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | b'(' | b'}')
}

/// Match the JDBC `call` keyword with mandatory word boundary.
fn try_match_call(s: &str) -> bool {
    s.len() >= 4 && s[..4].eq_ignore_ascii_case("call") && (s.len() == 4 || is_jdbc_call_boundary(s.as_bytes()[4]))
}

/// Check if `s` starts with a flattened parameter placeholder (`__XML_PARAM_...__` or
/// `__XML_RAW_...__`), returning the text after the trailing `__` suffix if found.
fn try_skip_param_placeholder(s: &str) -> Option<&str> {
    let after_prefix =
        if let Some(rest) = s.strip_prefix("__XML_PARAM_") { rest } else { s.strip_prefix("__XML_RAW_")? };
    // Find the closing `__` suffix
    let end = after_prefix.find("__")?;
    Some(&after_prefix[end + 2..])
}

pub fn translate_jdbc_call(sql: &str) -> String {
    let trimmed = sql.trim_start();

    if !trimmed.starts_with('{') {
        return sql.to_string();
    }
    let after_brace = &trimmed[1..];

    // {? = call func(args)}  → CALL func(args)
    let trimmed_after_brace = after_brace.trim_start();
    if let Some(rest) = trimmed_after_brace.strip_prefix('?') {
        let rest = rest.trim_start();
        let after_call = if let Some(after_eq) = rest.strip_prefix('=') {
            after_eq.trim_start()
        } else {
            return sql.to_string();
        };
        if try_match_call(after_call) {
            let body = &after_call[4..].trim_start();
            return strip_trailing_brace(body);
        }
        return sql.to_string();
    }

    // {__XML_PARAM_*__ = call func(args)}  → CALL func(args)
    // {__XML_RAW_*__ = call func(args)}    → CALL func(args)
    if let Some(after_placeholder) = try_skip_param_placeholder(trimmed_after_brace) {
        let after_eq = after_placeholder.trim_start();
        if let Some(after_call) = after_eq.strip_prefix('=') {
            let after_call = after_call.trim_start();
            if try_match_call(after_call) {
                let body = &after_call[4..].trim_start();
                return strip_trailing_brace(body);
            }
        }
        return sql.to_string();
    }

    // {call proc(args)}  → CALL proc(args)
    if try_match_call(trimmed_after_brace) {
        let body = &trimmed_after_brace[4..].trim_start();
        return strip_trailing_brace(body);
    }

    sql.to_string()
}

fn strip_trailing_brace(body: &str) -> String {
    let inner = if let Some(pos) = body.rfind('}') { body[..pos].trim() } else { body.trim() };
    // If the procedure call has no arguments (no parentheses), add empty ()
    // e.g. {call pkg.proc} → CALL pkg.proc()
    if inner.contains('(') {
        format!("CALL {}", inner)
    } else {
        format!("CALL {}()", inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_jdbc_call_plain_sql_idempotent() {
        // Plain SQL without JDBC escape → unchanged
        assert_eq!(translate_jdbc_call("SELECT 1"), "SELECT 1");
        assert_eq!(translate_jdbc_call("CALL proc()"), "CALL proc()");
        assert_eq!(translate_jdbc_call(""), "");
    }

    #[test]
    fn test_translate_jdbc_call_basic() {
        // {call proc(args)} → CALL proc(args)
        assert_eq!(translate_jdbc_call("{call proc()}"), "CALL proc()");
        assert_eq!(translate_jdbc_call("{call proc(1, 2)}"), "CALL proc(1, 2)");
    }

    #[test]
    fn test_translate_jdbc_call_no_args_adds_parens() {
        // {call pkg.proc} → CALL pkg.proc()  (adds empty parens)
        assert_eq!(translate_jdbc_call("{call pkg.proc}"), "CALL pkg.proc()");
        assert_eq!(translate_jdbc_call("{call my_proc}"), "CALL my_proc()");
    }

    #[test]
    fn test_translate_jdbc_call_literal_question_mark() {
        // {? = call func(args)} → CALL func(args)
        assert_eq!(translate_jdbc_call("{? = call func()}"), "CALL func()");
        assert_eq!(translate_jdbc_call("{? = call func(1, 2)}"), "CALL func(1, 2)");
        assert_eq!(translate_jdbc_call("{? = call pkg.func(1)}"), "CALL pkg.func(1)");
    }

    #[test]
    fn test_translate_jdbc_call_literal_question_mark_no_args() {
        // {? = call func} → CALL func()  (adds empty parens)
        assert_eq!(translate_jdbc_call("{? = call func}"), "CALL func()");
        assert_eq!(translate_jdbc_call("{? = call pkg.func}"), "CALL pkg.func()");
    }

    #[test]
    fn test_translate_jdbc_call_whitespace_tolerant() {
        // Whitespace/line-breaks between tokens
        assert_eq!(translate_jdbc_call("{\ncall proc()}"), "CALL proc()");
        assert_eq!(translate_jdbc_call("{\n? = call proc()}"), "CALL proc()");
        assert_eq!(translate_jdbc_call("{?\n= call proc()}"), "CALL proc()");
        assert_eq!(translate_jdbc_call("{call\nproc()}"), "CALL proc()");
    }

    #[test]
    fn test_translate_jdbc_call_param_placeholder() {
        // {__XML_PARAM_VARCHAR_result__ = call func(args)} → CALL func(args)
        assert_eq!(translate_jdbc_call("{__XML_PARAM_VARCHAR_result__ = call func()}"), "CALL func()");
        assert_eq!(
            translate_jdbc_call("{__XML_PARAM_VARCHAR_result__ = call func(__XML_PARAM_VARCHAR_scdm__)}"),
            "CALL func(__XML_PARAM_VARCHAR_scdm__)"
        );
    }

    #[test]
    fn test_translate_jdbc_call_param_placeholder_no_type() {
        // Placeholder without jdbcType prefix
        assert_eq!(translate_jdbc_call("{__XML_PARAM_result__ = call func()}"), "CALL func()");
        assert_eq!(
            translate_jdbc_call("{__XML_PARAM_result__ = call func(__XML_PARAM_scdm__, __XML_PARAM_day__)}"),
            "CALL func(__XML_PARAM_scdm__, __XML_PARAM_day__)"
        );
    }

    #[test]
    fn test_translate_jdbc_call_raw_placeholder() {
        // {__XML_RAW_procName__ = call func(args)} → CALL func(args)
        assert_eq!(translate_jdbc_call("{__XML_RAW_procName__ = call func()}"), "CALL func()");
    }

    #[test]
    fn test_translate_jdbc_call_full_user_example() {
        // Exact user pattern with #{result,mode=OUT,jdbcType=VARCHAR}
        let input = concat!(
            "{__XML_PARAM_VARCHAR_result__ = call fnc_com_getday(\n",
            "        __XML_PARAM_VARCHAR_scdm__,\n",
            "        __XML_PARAM_VARCHAR_day__,\n",
            "        __XML_PARAM_INTEGER_feed__,\n",
            "        __XML_PARAM_VARCHAR_coinCode__\n",
            "        )}"
        );
        let expected = concat!(
            "CALL fnc_com_getday(\n",
            "        __XML_PARAM_VARCHAR_scdm__,\n",
            "        __XML_PARAM_VARCHAR_day__,\n",
            "        __XML_PARAM_INTEGER_feed__,\n",
            "        __XML_PARAM_VARCHAR_coinCode__\n",
            "        )"
        );
        assert_eq!(translate_jdbc_call(input), expected);
    }

    #[test]
    fn test_translate_jdbc_call_idempotent_non_jdbc() {
        // Non-JDBC input with { ... } but not matching patterns → unchanged
        assert_eq!(translate_jdbc_call("{ SELECT 1 }"), "{ SELECT 1 }");
        assert_eq!(translate_jdbc_call("{ ? = 1 }"), "{ ? = 1 }");
    }

    #[test]
    fn test_translate_jdbc_call_no_match_fallback() {
        // { ? = something_else } → unchanged (no "call")
        assert_eq!(translate_jdbc_call("{ ? = some_expr }"), "{ ? = some_expr }");
        // {__XML_PARAM_x__ = something } → unchanged
        assert_eq!(translate_jdbc_call("{__XML_PARAM_x__ = something}"), "{__XML_PARAM_x__ = something}");
    }
}

#[cfg(feature = "ibatis")]
pub mod ibatis;

#[cfg(feature = "java")]
pub mod java;

#[cfg(feature = "mcp")]
pub mod mcp;
