//! Public orchestration API for the `validate` CLI command.
//!
//! Runs PACKAGE consistency, MERGE semantics, and PL variable validation on
//! already-parsed statements, preserving typed errors (no folding into
//! `ParserError`). This is the library-level entry point that the `validate`,
//! `validate-xml`, and `validate-java` CLI commands build on.

use crate::{MergeSemanticError, PackageConsistencyError, UndefinedVariableError};

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
