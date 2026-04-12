//! Java SQL 提取数据模型。

use crate::ast::StatementInfo;
use crate::parser::ParserError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JavaExtractResult {
    pub file_path: String,
    pub extractions: Vec<ExtractedSql>,
    pub errors: Vec<crate::java::error::JavaError>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExtractedSql {
    pub sql: String,
    pub origin: SqlOrigin,
    pub sql_kind: SqlKind,
    pub parameter_style: ParameterStyle,
    pub is_concatenated: bool,
    pub is_text_block: bool,
    pub parse_result: Option<SqlParseResult>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SqlParseResult {
    pub statements: Vec<StatementInfo>,
    pub errors: Vec<ParserError>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SqlOrigin {
    pub method: ExtractionMethod,
    pub class_name: Option<String>,
    pub method_name: Option<String>,
    pub annotation_name: Option<String>,
    pub api_method_name: Option<String>,
    pub variable_name: Option<String>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ExtractionMethod {
    Annotation,
    MethodCall,
    Constant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SqlKind {
    NativeSql,
    Jpql,
    Ddl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ParameterStyle {
    PositionalQuestion,
    PositionalNumbered,
    NamedColon,
    NamedHash,
    None,
}
