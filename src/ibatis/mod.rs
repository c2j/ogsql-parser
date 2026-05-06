//! iBatis/MyBatis XML mapper 文件解析支持。
//!
//! 从 XML mapper 文件中提取 SQL 语句，建模动态 SQL 元素，
//! 并将提取的 SQL 馈入核心 Parser 得到结构化 AST。

mod util;

#[cfg(feature = "java")]
mod java_resolve;

pub mod error;
pub mod flatten;
pub mod parser;
pub mod resolver;
pub mod types;

pub use error::IbatisError;
pub use types::{
    FlattenedStatement, InferenceSource, JdbcType, MapperFile, MapperStatement, ParamMeta,
    ParsedMapper, ParsedStatement, SqlFragment, SqlNode, StatementKind,
};

#[cfg(feature = "java")]
pub use java_resolve::JavaSourceResolver;

/// 从 XML 字节解析 mapper 文件。
pub fn parse_mapper_bytes(xml: &[u8]) -> ParsedMapper {
    parse_mapper_bytes_internal(xml, None, Vec::new())
}

/// 从 XML 字节解析 mapper 文件，附带源文件路径。
pub fn parse_mapper_bytes_with_path(xml: &[u8], file_path: Option<&str>) -> ParsedMapper {
    parse_mapper_bytes_internal(xml, file_path, Vec::new())
}

#[cfg(feature = "java")]
/// 从 XML 字节解析 mapper 文件，附带 Java 源码根目录以进行类型推断。
pub fn parse_mapper_bytes_with_java_src(
    xml: &[u8],
    file_path: Option<&str>,
    java_source_roots: Vec<std::path::PathBuf>,
) -> ParsedMapper {
    parse_mapper_bytes_internal(xml, file_path, java_source_roots)
}

fn parse_mapper_bytes_internal(
    xml: &[u8],
    file_path: Option<&str>,
    #[allow(unused_variables)] java_source_roots: Vec<std::path::PathBuf>,
) -> ParsedMapper {
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
        let mut flat_sql = flatten::flatten_sql(&stmt.body);

        let collected = flatten::collect_params(&stmt.body);

        #[cfg(feature = "java")]
        let parameters = {
            let resolver = java_resolve::JavaSourceResolver::new(java_source_roots.clone());
            infer_param_types(&mapper_file.namespace, stmt, &collected, &resolver)
        };
        #[cfg(not(feature = "java"))]
        let parameters = collected.iter().map(|(name, java_type, raw)| {
            types::ParamMeta {
                name: name.clone(),
                jdbc_type: None,
                source: java_type.as_ref().map(|_| types::InferenceSource::InlineJavaType),
                position: 0,
                raw: raw.clone(),
            }
        }).collect();

        for param in &parameters {
            if let Some(jdbc) = &param.jdbc_type {
                let untyped = format!("{}{}{}", "__XML_PARAM_", param.name, "__");
                let typed = format!("{}{}_{}{}", "__XML_PARAM_", format!("{:?}", jdbc).to_uppercase(), param.name, "__");
                flat_sql = flat_sql.replace(&untyped, &typed);
            }
        }

        let has_dynamic = has_dynamic_elements(&stmt.body);
        let parse_result = if !flat_sql.trim().is_empty() {
            Some(crate::parser::Parser::parse_sql(&flat_sql))
        } else {
            None
        };

        statements.push(ParsedStatement {
            id: stmt.id.clone(),
            kind: stmt.kind,
            parameter_type: stmt.parameter_type.clone(),
            result_type: stmt.result_type.clone(),
            flat_sql,
            parameters,
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

#[cfg(feature = "java")]
fn infer_param_types(
    namespace: &str,
    stmt: &types::MapperStatement,
    collected_params: &[(String, Option<String>, String)],
    resolver: &java_resolve::JavaSourceResolver,
) -> Vec<types::ParamMeta> {
    use crate::ibatis::types::{InferenceSource, ParamMeta};
    use crate::ibatis::java_resolve::{java_type_to_jdbc, jdbc_type_from_str};

    // Parse Mapper interface (if available)
    let interface_info = resolver.read_source(namespace)
        .and_then(|src| crate::java::parse_mapper_interface(&src));

    // Get method params for this statement
    let method_params = interface_info.as_ref()
        .and_then(|info| info.methods.get(&stmt.id))
        .map(|m| &m.params);

    // Parse DTO from XML parameterType or from method parameter type
    let mut dto_fields: std::collections::HashMap<String, String> = stmt.parameter_type.as_ref()
        .filter(|pt| pt.contains('.') && !pt.eq_ignore_ascii_case("map"))
        .and_then(|pt| resolver.read_source(pt))
        .map(|src| crate::java::parse_dto_fields(&src))
        .unwrap_or_default();

    if dto_fields.is_empty() {
        if let Some(ref info) = interface_info {
            if let Some(method) = info.methods.get(&stmt.id) {
                for param in &method.params {
                    if java_type_to_jdbc(&param.java_type).is_none() {
                        if let Some(src) = resolver.read_source_by_class_name(&param.java_type) {
                            let fields = crate::java::parse_dto_fields(&src);
                            dto_fields.extend(fields);
                        }
                    }
                }
            }
        }
    }

    collected_params.iter().map(|(name, inline_java_type, raw)| {
        // Priority 1: XML inline annotation
        if let Some(ref jt) = inline_java_type {
            if let Some(jdbc) = java_type_to_jdbc(jt) {
                return ParamMeta {
                    name: name.clone(),
                    jdbc_type: Some(jdbc),
                    source: Some(InferenceSource::InlineJavaType),
                    position: 0,
                    raw: raw.clone(),
                };
            }
            if let Some(jdbc) = jdbc_type_from_str(jt) {
                return ParamMeta {
                    name: name.clone(),
                    jdbc_type: Some(jdbc),
                    source: Some(InferenceSource::InlineJdbcType),
                    position: 0,
                    raw: raw.clone(),
                };
            }
        }

        // Priority 2: Mapper interface method signature
        if let Some(params) = method_params {
            if let Some(param) = params.iter().find(|p| p.name == *name) {
                if let Some(jdbc) = java_type_to_jdbc(&param.java_type) {
                    let source = if param.param_annotation.is_some() {
                        InferenceSource::JavaParamAnnotation
                    } else {
                        InferenceSource::JavaMethodSignature
                    };
                    return ParamMeta {
                        name: name.clone(),
                        jdbc_type: Some(jdbc),
                        source: Some(source),
                        position: 0,
                        raw: raw.clone(),
                    };
                }
            }
        }

        // Priority 3: DTO fields
        if let Some(java_t) = dto_fields.get(name) {
            if let Some(jdbc) = java_type_to_jdbc(java_t) {
                return ParamMeta {
                    name: name.clone(),
                    jdbc_type: Some(jdbc),
                    source: Some(InferenceSource::JavaDtoField),
                    position: 0,
                    raw: raw.clone(),
                };
            }
        }

        // No type info
        ParamMeta {
            name: name.clone(),
            jdbc_type: None,
            source: None,
            position: 0,
            raw: raw.clone(),
        }
    }).collect()
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
        | SqlNode::Bind { .. }
        | SqlNode::Include { .. } => true,
    }
}

#[cfg(test)]
mod tests;
