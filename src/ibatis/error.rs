//! iBatis XML 解析错误类型。

use crate::parser::ParserError;

/// iBatis XML mapper 解析过程中可能产生的错误。
#[derive(Debug, Clone, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum IbatisError {
    /// XML 格式错误
    #[error("XML parse error at line {line}: {message}")]
    XmlError { line: usize, message: String },

    /// 找不到引用的 SQL 片段
    #[error("unknown sql fragment: {refid}")]
    UnknownFragment { refid: String },

    /// 循环引用检测
    #[error("circular include detected: {chain:?}")]
    CircularInclude { chain: Vec<String> },

    /// 必需属性缺失
    #[error("missing required attribute '{attribute}' on <{element}>")]
    MissingAttribute { element: String, attribute: String },

    /// mapper 文件为空或没有有效内容
    #[error("empty mapper: no statements found")]
    EmptyMapper,

    /// SQL 解析错误（由核心 Parser 产生）
    #[error("SQL parse error: {0}")]
    SqlParseError(ParserError),
}
