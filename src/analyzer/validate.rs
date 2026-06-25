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
                let body_name: String = body.name.iter().map(|s| s.to_lowercase()).collect::<Vec<_>>().join(".");
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
                        let spec_name: String =
                            spec.name.iter().map(|s| s.to_lowercase()).collect::<Vec<_>>().join(".");
                        if spec_name == body_name {
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
}
