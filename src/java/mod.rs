//! Java 源码 SQL 提取支持。
//!
//! 从 Java 源文件中提取嵌入在注解、方法调用参数、字符串常量中的 SQL 语句，
//! 并将提取的 SQL 馈入核心 Parser 得到结构化 AST。

pub mod error;
pub mod extract;
pub mod types;

pub use error::JavaError;
pub use types::{
    ExtractedSql, ExtractionMethod, JavaExtractConfig, JavaExtractResult, ParameterStyle, SqlKind,
    SqlOrigin, SqlParseResult,
};

use tree_sitter::Parser;

/// 从 Java 源码字节提取 SQL。
///
/// 接受 UTF-8 字符串，返回提取结果。
pub fn extract_sql_from_java(
    source: &str,
    file_path: &str,
    config: &JavaExtractConfig,
) -> JavaExtractResult {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_java::LANGUAGE.into())
        .expect("Failed to set Java language for tree-sitter");

    let tree = match parser.parse(source, None) {
        Some(tree) => tree,
        None => {
            return JavaExtractResult {
                file_path: file_path.to_string(),
                extractions: Vec::new(),
                errors: vec![JavaError::ParseError {
                    message: "tree-sitter returned no parse tree".to_string(),
                }],
            };
        }
    };

    extract::extract(source, tree.root_node(), file_path, config)
}

#[cfg(test)]
mod tests;
