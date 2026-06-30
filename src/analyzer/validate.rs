//! Public orchestration API for the `validate` CLI command.
//!
//! Runs PACKAGE consistency, MERGE semantics, and PL variable validation on
//! already-parsed statements, preserving typed errors (no folding into
//! `ParserError`). This is the library-level entry point that the `validate`,
//! `validate-xml`, and `validate-java` CLI commands build on.

use crate::{MergeSemanticError, PackageConsistencyError, UndefinedVariableError};

/// Walk `StatementInfo` slices, extract `PlBlock`s from each variant
/// (`CreateProcedure`, `CreateFunction`, `Do`, `AnonyBlock`, `CreatePackageBody`),
/// and run PL variable/function validation per block.
///
/// For package bodies, cross-references package-spec variables so they are
/// recognised as defined during validation of the body's subprograms.
pub fn validate_pl_variables_from_stmts(
    stmts: &[crate::ast::StatementInfo],
    known_funcs: &[String],
    strict: bool,
) -> Vec<crate::UndefinedVariableError> {
    use crate::ast::Statement;
    let mut warnings = Vec::new();
    let funcs_str: Vec<&str> = known_funcs.iter().map(|s| s.as_str()).collect();
    for si in stmts {
        match &si.statement {
            Statement::CreateProcedure(proc) => {
                if let Some(ref block) = proc.block {
                    let vars = crate::validate_pl_variables_with_extra_vars_and_funcs(
                        block,
                        &proc.parameters,
                        &[],
                        &funcs_str,
                        strict,
                    );
                    warnings.extend(vars);
                }
            }
            Statement::CreateFunction(func) => {
                if let Some(ref block) = func.block {
                    let vars = crate::validate_pl_variables_with_extra_vars_and_funcs(
                        block,
                        &func.parameters,
                        &[],
                        &funcs_str,
                        strict,
                    );
                    warnings.extend(vars);
                }
            }
            Statement::Do(do_stmt) => {
                if let Some(ref block) = do_stmt.block {
                    let vars =
                        crate::validate_pl_variables_with_extra_vars_and_funcs(block, &[], &[], &funcs_str, strict);
                    warnings.extend(vars);
                }
            }
            Statement::CreatePackageBody(body) => {
                let body_key = crate::analyzer::last_name_lower(&body.name);
                let mut pkg_vars: Vec<&str> = body
                    .items
                    .iter()
                    .filter_map(|item| match item {
                        crate::ast::PackageItem::Variable(v) => Some(v.name.as_str()),
                        _ => None,
                    })
                    .collect();
                for other_si in stmts {
                    if let Statement::CreatePackage(spec) = &other_si.statement {
                        let spec_key = crate::analyzer::last_name_lower(&spec.name);
                        if spec_key == body_key {
                            for item in &spec.items {
                                if let crate::ast::PackageItem::Variable(v) = item {
                                    pkg_vars.push(v.name.as_str());
                                }
                            }
                        }
                    }
                }
                for item in &body.items {
                    match item {
                        crate::ast::PackageItem::Procedure(proc) => {
                            if let Some(ref block) = proc.block {
                                let vars = crate::validate_pl_variables_with_extra_vars_and_funcs(
                                    block,
                                    &proc.parameters,
                                    &pkg_vars,
                                    &funcs_str,
                                    strict,
                                );
                                warnings.extend(vars);
                            }
                        }
                        crate::ast::PackageItem::Function(func) => {
                            if let Some(ref block) = func.block {
                                let vars = crate::validate_pl_variables_with_extra_vars_and_funcs(
                                    block,
                                    &func.parameters,
                                    &pkg_vars,
                                    &funcs_str,
                                    strict,
                                );
                                warnings.extend(vars);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    warnings
}

/// Collect the names of all routines (functions, procedures) and package-level types
/// defined in a slice of statements. Used by the PL variable validator to treat
/// locally-defined routines as known functions.
///
/// Walks `StatementInfo` looking at `CreateFunction`, `CreateProcedure`,
/// `CreatePackage` (spec), and `CreatePackageBody` — for each subprogram found,
/// the last component of its (possibly schema-qualified) name is collected in
/// lowercase. Duplicates and sort order are handled by the caller.
pub fn collect_defined_routine_names(stmts: &[crate::ast::StatementInfo]) -> Vec<String> {
    use crate::ast::Statement;
    let mut names = Vec::new();
    for si in stmts {
        match &si.statement {
            Statement::CreateFunction(func) => {
                if let Some(last) = func.name.last() {
                    names.push(last.to_lowercase());
                }
            }
            Statement::CreateProcedure(proc) => {
                if let Some(last) = proc.name.last() {
                    names.push(last.to_lowercase());
                }
            }
            Statement::CreatePackage(spec) => {
                for item in &spec.items {
                    match item {
                        crate::ast::PackageItem::Function(f) => {
                            if let Some(last) = f.name.last() {
                                names.push(last.to_lowercase());
                            }
                        }
                        crate::ast::PackageItem::Procedure(p) => {
                            if let Some(last) = p.name.last() {
                                names.push(last.to_lowercase());
                            }
                        }
                        crate::ast::PackageItem::Type(t) => {
                            let name = match t {
                                crate::ast::plpgsql::PlTypeDecl::Record { name, .. } => name,
                                crate::ast::plpgsql::PlTypeDecl::TableOf { name, .. } => name,
                                crate::ast::plpgsql::PlTypeDecl::VarrayOf { name, .. } => name,
                                crate::ast::plpgsql::PlTypeDecl::RefCursor { name } => name,
                            };
                            names.push(name.to_lowercase());
                        }
                        _ => {}
                    }
                }
            }
            Statement::CreatePackageBody(body) => {
                for item in &body.items {
                    match item {
                        crate::ast::PackageItem::Function(f) => {
                            if let Some(last) = f.name.last() {
                                names.push(last.to_lowercase());
                            }
                        }
                        crate::ast::PackageItem::Procedure(p) => {
                            if let Some(last) = p.name.last() {
                                names.push(last.to_lowercase());
                            }
                        }
                        crate::ast::PackageItem::Type(t) => {
                            let name = match t {
                                crate::ast::plpgsql::PlTypeDecl::Record { name, .. } => name,
                                crate::ast::plpgsql::PlTypeDecl::TableOf { name, .. } => name,
                                crate::ast::plpgsql::PlTypeDecl::VarrayOf { name, .. } => name,
                                crate::ast::plpgsql::PlTypeDecl::RefCursor { name } => name,
                            };
                            names.push(name.to_lowercase());
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    names.sort();
    names.dedup();
    names
}

/// Aggregate result of running all three validators on a slice of statements.
///
/// Each bucket is independent — e.g. `merge_errors` may be empty while
/// `package_errors` is non-empty. Use [`ValidationReport::is_empty`] to check
/// whether any validator produced findings.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ValidationReport {
    /// PACKAGE spec vs PACKAGE BODY mismatches.
    pub package_errors: Vec<PackageConsistencyError>,
    /// Non-deterministic / invalid MERGE patterns.
    pub merge_errors: Vec<MergeSemanticError>,
    /// Undefined variables / functions in PL/pgSQL blocks.
    pub undefined_variable_errors: Vec<UndefinedVariableError>,
}

/// Run PACKAGE consistency, MERGE semantics, and PL variable validation on
/// already-parsed statements. Returns typed errors in three independent
/// buckets — no folding into `ParserError` (that's a CLI output concern).
///
/// This is the library-level equivalent of the `validate` / `validate-xml` /
/// `validate-java` CLI commands' shared pipeline.
///
/// # Arguments
/// * `stmts` - Already-parsed SQL statements.
/// * `extra_funcs` - Additional function names to treat as defined (e.g.
///   routines declared in external packages the consumer knows about).
/// * `strict` - When `true`, flag undefined function calls in PL blocks.
pub fn validate_statements(
    stmts: &[crate::ast::StatementInfo],
    extra_funcs: &[String],
    strict: bool,
) -> ValidationReport {
    let package_errors = crate::validate_package_consistency(stmts);
    let merge_errors = crate::validate_merge_semantics(stmts);

    let mut all_funcs: Vec<String> = extra_funcs.to_vec();
    all_funcs.extend(collect_defined_routine_names(stmts));
    all_funcs.sort();
    all_funcs.dedup();

    let undefined_variable_errors = validate_pl_variables_from_stmts(stmts, &all_funcs, strict);

    ValidationReport { package_errors, merge_errors, undefined_variable_errors }
}

impl ValidationReport {
    /// `true` when every bucket is empty (no findings from any validator).
    pub fn is_empty(&self) -> bool {
        self.package_errors.is_empty() && self.merge_errors.is_empty() && self.undefined_variable_errors.is_empty()
    }

    /// Total number of findings across all buckets.
    pub fn total_count(&self) -> usize {
        self.package_errors.len() + self.merge_errors.len() + self.undefined_variable_errors.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_default_is_empty() {
        let r = ValidationReport::default();
        assert!(r.is_empty());
        assert_eq!(r.total_count(), 0);
    }

    #[test]
    fn report_total_count_sums_all_buckets() {
        let r = ValidationReport {
            package_errors: vec![PackageConsistencyError {
                package_name: "p".into(),
                subprogram_name: "s".into(),
                kind: crate::PackageConsistencyErrorKind::MissingInBody,
                detail: None,
            }],
            merge_errors: vec![MergeSemanticError {
                kind: crate::MergeSemanticErrorKind::DeleteNotSupported,
                detail: None,
                location: crate::SourceLocation::default(),
            }],
            undefined_variable_errors: vec![UndefinedVariableError {
                variable_name: "x".into(),
                location: None,
                context: "ctx".into(),
                kind: crate::UndefinedRefKind::Variable,
            }],
        };
        assert!(!r.is_empty());
        assert_eq!(r.total_count(), 3);
    }

    #[test]
    fn validate_statements_empty_input_yields_empty_report() {
        let report = validate_statements(&[], &[], false);
        assert!(report.is_empty());
    }

    #[test]
    fn validate_statements_detects_merge_error() {
        // Non-deterministic MERGE: DELETE in WHEN MATCHED.
        let stmts = parse_stmts("MERGE INTO t USING s ON t.id = s.id WHEN MATCHED THEN DELETE");
        let report = validate_statements(&stmts, &[], false);
        assert!(!report.merge_errors.is_empty(), "expected merge errors");
    }

    #[test]
    fn validate_statements_detects_undefined_variable_in_strict_mode() {
        // undefined_func() is called in a RETURN expression (PERFORM is not checked).
        let sql =
            "CREATE OR REPLACE FUNCTION f() RETURNS INT AS $$ BEGIN RETURN undefined_func(); END; $$ LANGUAGE plpgsql";
        let stmts = parse_stmts(sql);
        let report = validate_statements(&stmts, &[], true);
        assert!(!report.undefined_variable_errors.is_empty(), "expected undefined-func errors in strict mode");
    }

    #[test]
    fn validate_statements_extra_funcs_suppresses_undefined_error() {
        let sql =
            "CREATE OR REPLACE FUNCTION f() RETURNS INT AS $$ BEGIN RETURN known_func(); END; $$ LANGUAGE plpgsql";
        let stmts = parse_stmts(sql);
        let report = validate_statements(&stmts, &["known_func".to_string()], true);
        assert!(report.undefined_variable_errors.is_empty(), "known_func in extra_funcs should suppress error");
    }

    #[test]
    fn package_schema_mismatch_emits_warning() {
        let sql = "CREATE OR REPLACE PACKAGE pkg_a AS v_flag NUMBER; PROCEDURE prc_test; END pkg_a;\n\
                   CREATE OR REPLACE PACKAGE BODY myschema.pkg_a AS \
                   PROCEDURE prc_test IS BEGIN v_flag := 1; END prc_test; END pkg_a;";
        let stmts = parse_stmts(sql);
        let report = validate_statements(&stmts, &[], false);
        assert!(
            report
                .package_errors
                .iter()
                .any(|e| matches!(e.kind, crate::PackageConsistencyErrorKind::SchemaMismatch { .. })),
            "should emit SchemaMismatch when spec and body have different schema qualification"
        );
    }

    #[test]
    fn package_schema_mismatch_still_resolves_variables() {
        let sql = "CREATE OR REPLACE PACKAGE pkg_b AS v_flag NUMBER; PROCEDURE prc_test; END pkg_b;\n\
                   CREATE OR REPLACE PACKAGE BODY myschema.pkg_b AS \
                   PROCEDURE prc_test IS BEGIN v_flag := 1; END prc_test; END pkg_b;";
        let stmts = parse_stmts(sql);
        let report = validate_statements(&stmts, &[], false);
        assert!(
            report.undefined_variable_errors.is_empty(),
            "should NOT report undefined variable when spec/body schemas differ but package name matches"
        );
    }

    #[test]
    fn package_schema_mismatch_reversed_still_resolves_variables() {
        let sql = "CREATE OR REPLACE PACKAGE myschema.pkg_c AS v_flag NUMBER; PROCEDURE prc_test; END pkg_c;\n\
                   CREATE OR REPLACE PACKAGE BODY pkg_c AS \
                   PROCEDURE prc_test IS BEGIN v_flag := 1; END prc_test; END pkg_c;";
        let stmts = parse_stmts(sql);
        let report = validate_statements(&stmts, &[], false);
        assert!(
            report.undefined_variable_errors.is_empty(),
            "should NOT report undefined variable when spec has schema but body doesn't"
        );
    }

    #[test]
    fn package_same_schema_no_warning() {
        let sql = "CREATE OR REPLACE PACKAGE myschema.pkg_d AS v_flag NUMBER; PROCEDURE prc_test; END pkg_d;\n\
                   CREATE OR REPLACE PACKAGE BODY myschema.pkg_d AS \
                   PROCEDURE prc_test IS BEGIN v_flag := 1; END prc_test; END pkg_d;";
        let stmts = parse_stmts(sql);
        let report = validate_statements(&stmts, &[], false);
        assert!(
            !report
                .package_errors
                .iter()
                .any(|e| matches!(e.kind, crate::PackageConsistencyErrorKind::SchemaMismatch { .. })),
            "should NOT emit SchemaMismatch when spec and body have same schema"
        );
        assert!(report.undefined_variable_errors.is_empty(), "should NOT report undefined variable");
    }

    /// Helper: parse SQL text into Vec<StatementInfo>.
    fn parse_stmts(sql: &str) -> Vec<crate::ast::StatementInfo> {
        crate::parser::Parser::parse_sql(sql).0
    }
}
