//! Java 源码 SQL 提取错误类型。

use crate::parser::ParserError;

/// Java SQL 提取过程中可能产生的错误。
#[derive(Debug, Clone, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum JavaError {
    /// tree-sitter 解析错误
    #[error("tree-sitter parse error: {message}")]
    ParseError { message: String },

    /// 提取的 SQL 解析错误（由核心 Parser 产生）
    #[error("SQL parse error in {origin}: {error}")]
    SqlParseError { origin: String, error: ParserError },

    /// 文件读取错误
    #[error("IO error: {0}")]
    IoError(String),

    /// 编码错误
    #[error("encoding error: {0}")]
    EncodingError(String),
}
