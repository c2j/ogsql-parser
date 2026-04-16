//! iBatis/MyBatis XML mapper 文件解析支持。
//!
//! 从 XML mapper 文件中提取 SQL 语句，建模动态 SQL 元素，
//! 并将提取的 SQL 馈入核心 Parser 得到结构化 AST。

pub mod error;
pub mod flatten;
pub mod parser;
pub mod resolver;
pub mod types;

pub use error::IbatisError;
pub use types::{
    FlattenedStatement, MapperFile, MapperStatement, ParsedMapper, ParsedStatement, SqlFragment,
    SqlNode, StatementKind,
};

/// 从 XML 字节解析 mapper 文件。
pub fn parse_mapper_bytes(xml: &[u8]) -> ParsedMapper {
    parse_mapper_bytes_with_path(xml, None)
}

/// 从 XML 字节解析 mapper 文件，附带源文件路径。
pub fn parse_mapper_bytes_with_path(xml: &[u8], file_path: Option<&str>) -> ParsedMapper {
    let mut errors = Vec::new();

    let mapper_file = match parser::parse_xml(xml) {
        Ok(m) => m,
        Err(e) => {
            return ParsedMapper {
                file_path: file_path.map(|s| s.to_string()),
                namespace: String::new(),
                statements: Vec::new(),
                errors: vec![e],
            };
        }
    };

    let mapper_file = match resolver::resolve_includes(&mapper_file) {
        Ok(m) => m,
        Err(e) => {
            errors.push(e);
            return ParsedMapper {
                file_path: file_path.map(|s| s.to_string()),
                namespace: mapper_file.namespace,
                statements: Vec::new(),
                errors,
            };
        }
    };

    let mut statements = Vec::new();
    for stmt in &mapper_file.statements {
        let flat_sql = flatten::flatten_sql(&stmt.body);
        let has_dynamic = has_dynamic_elements(&stmt.body);
        let parse_result = if !flat_sql.trim().is_empty() {
            Some(crate::parser::Parser::parse_sql(&flat_sql))
        } else {
            None
        };

        statements.push(ParsedStatement {
            id: stmt.id.clone(),
            kind: stmt.kind,
            flat_sql,
            has_dynamic_elements: has_dynamic,
            line: stmt.line,
            parse_result,
        });
    }

    if statements.is_empty() && errors.is_empty() {
        errors.push(IbatisError::EmptyMapper);
    }

    ParsedMapper {
        file_path: file_path.map(|s| s.to_string()),
        namespace: mapper_file.namespace,
        statements,
        errors,
    }
}

fn has_dynamic_elements(node: &SqlNode) -> bool {
    match node {
        SqlNode::Text { .. } | SqlNode::Parameter { .. } | SqlNode::RawExpr { .. } => false,
        SqlNode::Sequence { children } => children.iter().any(has_dynamic_elements),
        SqlNode::If { .. }
        | SqlNode::Choose { .. }
        | SqlNode::Where { .. }
        | SqlNode::Set { .. }
        | SqlNode::Trim { .. }
        | SqlNode::ForEach { .. }
        | SqlNode::Bind { .. } => true,
    }
}

#[cfg(test)]
mod tests;
