//! Java 源码 SQL 提取支持。

use tree_sitter::Parser;

pub mod error;
pub mod extract;
pub mod types;

mod annotation;
mod constant;
mod dto_fields;
mod heuristics;
mod mapper_interface;
mod method_call;
mod string_decode;
mod variable;

pub use dto_fields::parse_dto_fields;
pub use error::JavaError;
pub use mapper_interface::{parse_mapper_interface, MapperInterfaceInfo, MapperMethodInfo, MethodParam};
pub use types::{
    CrossFileState, ExtractedSql, ExtractionMethod, JavaExtractConfig, JavaExtractResult, MethodPsBehavior,
    ParameterStyle, SetterPattern, SqlKind, SqlOrigin, SqlParseResult,
};

pub fn extract_sql_from_java(source: &str, file_path: &str, config: &JavaExtractConfig) -> JavaExtractResult {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_java::LANGUAGE.into()).expect("Failed to set Java language for tree-sitter");

    let tree = match parser.parse(source, None) {
        Some(tree) => tree,
        None => {
            return JavaExtractResult {
                file_path: file_path.to_string(),
                extractions: Vec::new(),
                errors: vec![JavaError::ParseError { message: "tree-sitter returned no parse tree".to_string() }],
            };
        }
    };

    extract::extract(source, tree.root_node(), file_path, config)
}

pub fn extract_sql_from_java_files(files: &[(&str, &str)], config: &JavaExtractConfig) -> Vec<JavaExtractResult> {
    let mut state = CrossFileState::default();
    extract_sql_from_java_files_with_state(files, config, &mut state)
}

pub fn extract_sql_from_java_files_with_state(
    files: &[(&str, &str)],
    config: &JavaExtractConfig,
    state: &mut CrossFileState,
) -> Vec<JavaExtractResult> {
    let mut results = Vec::new();
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_java::LANGUAGE.into()).expect("Failed to set Java language for tree-sitter");

    for (file_path, source) in files {
        let tree = match parser.parse(source, None) {
            Some(t) => t,
            None => {
                results.push(JavaExtractResult {
                    file_path: file_path.to_string(),
                    extractions: Vec::new(),
                    errors: vec![JavaError::ParseError { message: "tree-sitter returned no parse tree".to_string() }],
                });
                continue;
            }
        };

        let mut sql_var_patterns_upper = vec!["SQL".to_string()];
        for p in &config.extra_sql_var_patterns {
            let upper = p.to_uppercase();
            if !sql_var_patterns_upper.contains(&upper) {
                sql_var_patterns_upper.push(upper);
            }
        }

        let mut ctx = extract::ExtractContext {
            source,
            file_path,
            extractions: Vec::new(),
            errors: Vec::new(),
            class_name: None,
            method_name: None,
            sql_vars: std::collections::HashMap::new(),
            var_types: std::collections::HashMap::new(),
            jdbc_param_map: std::collections::HashMap::new(),
            ps_var_to_extraction: std::collections::HashMap::new(),
            extra_sql_methods: &config.extra_sql_methods,
            sql_var_patterns_upper,
            method_behaviors: std::mem::take(&mut state.method_behaviors),
            pending_injections: Vec::new(),
            class_ps_to_extraction: std::collections::HashMap::new(),
            dynamic_setters: Vec::new(),
            string_constants: std::mem::take(&mut state.string_constants),
            known_sets: std::collections::HashMap::new(),
            list_sources: std::collections::HashMap::new(),
            string_exprs: std::collections::HashMap::new(),
            local_bool_vars: std::collections::HashMap::new(),
            pending_string_vars: std::collections::HashMap::new(),
        };

        ctx.visit(tree.root_node());
        ctx.apply_pending_injections();

        state.method_behaviors = ctx.method_behaviors;
        state.string_constants = ctx.string_constants;

        let extractions: Vec<ExtractedSql> =
            ctx.extractions.into_iter().filter(|e| extract::starts_with_sql_keyword(&e.sql)).collect();

        results.push(JavaExtractResult { file_path: file_path.to_string(), extractions, errors: ctx.errors });
    }

    results
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_diagnostic;
