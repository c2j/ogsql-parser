//! iBatis/MyBatis XML mapper 文件解析支持。
//!
//! 从 XML mapper 文件中提取 SQL 语句，建模动态 SQL 元素，
//! 并将提取的 SQL 馈入核心 Parser 得到结构化 AST。
//!
//! # 快速选择
//!
//! | 场景 | 推荐 API |
//! |------|---------|
//! | 只解析 XML，不需要 Java 类型推断 | [`parse_mapper_bytes`] |
//! | 需要记录来源文件路径 | [`parse_mapper_bytes_with_path`] |
//! | 已知 Java 源码根目录 | [`parse_mapper_bytes_with_java_src`] |
//! | 只知项目根目录，想自动发现 Java 源码 | [`parse_mapper_bytes_auto`] |
//! | 需要保留动态 SQL 树（if/choose/foreach） | [`parse_mapper_bytes_structured`] |
//!
//! # Java 类型推断（需 `java` feature）
//!
//! XML 中的 `#{param}` 默认不带类型信息，输出为 `__XML_PARAM_param__`。
//! 启用类型推断后可输出 `__XML_PARAM_BIGINT_param__`。
//!
//! 类型来源按优先级：XML 内联 > parameterClass > Mapper 接口 > DTO 字段。
//!
//! ```ignore
//! // 方式一：一步到位，传入项目目录自动检测 Java 源码根
//! let result = ogsql_parser::ibatis::parse_mapper_bytes_auto(
//!     xml_bytes, None, std::path::Path::new("/my-project")
//! );
//!
//! // 方式二：手动指定源码根
//! let result = ogsql_parser::ibatis::parse_mapper_bytes_with_java_src(
//!     xml_bytes, None, vec![std::path::PathBuf::from("/my-project/src/main/java")]
//! );
//!
//! // 方式三：先检测，再决定是否传入
//! let roots = ogsql_parser::ibatis::detect_java_roots(std::path::Path::new("/my-project"));
//! if roots.is_empty() {
//!     // 降级到无类型推断
//! } else {
//!     let result = ogsql_parser::ibatis::parse_mapper_bytes_with_java_src(xml_bytes, None, roots);
//! }
//! ```

mod util;

#[cfg(feature = "java")]
mod java_resolve;

pub mod error;
pub mod expand;
pub mod flatten;
pub mod optimize;
pub mod parser;
pub mod resolver;
pub mod types;

pub use error::IbatisError;
pub use expand::expand_variants;
pub use types::{
    BranchStep, ExpandConfig, ExpandedVariant, FlattenedStatement, IfExpandStrategy, InferenceSource, JdbcType,
    MapperFile, MapperStatement, ParamMeta, ParameterMapDef, ParameterMapEntry, ParsedMapper, ParsedStatement,
    PlaceholderStrategy, SqlFragment, SqlNode, StatementKind, StructuredMapper, StructuredStatement, XmlSourceLocation,
};

#[cfg(feature = "java")]
pub use java_resolve::{detect_java_roots, JavaSourceResolver};

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

#[cfg(feature = "java")]
/// 从 XML 字节解析 mapper 文件，自动从 scan_dir 检测 Java 源码根目录以进行类型推断。
pub fn parse_mapper_bytes_auto(xml: &[u8], file_path: Option<&str>, scan_dir: &std::path::Path) -> ParsedMapper {
    let roots = java_resolve::detect_java_roots(scan_dir);
    parse_mapper_bytes_internal(xml, file_path, roots)
}

/// 从 XML 字节解析 mapper 文件，返回保留完整动态 SQL 树形结构的 AST。
///
/// 与 [`parse_mapper_bytes`] 不同，此函数不做扁平化（`flatten_sql`），
/// 而是直接返回 `SqlNode` 树，使调用方可以：
/// - 枚举所有可能的 SQL 变体（用于 AWR 慢 SQL 指纹匹配）
/// - 实现自定义展开策略
/// - 将动态元素溯源到源码位置
pub fn parse_mapper_bytes_structured(xml: &[u8]) -> StructuredMapper {
    parse_mapper_bytes_structured_with_path(xml, None)
}

/// 带 file_path 的结构化解析，用于在 `XmlSourceLocation` 中记录源文件路径。
pub fn parse_mapper_bytes_structured_with_path(xml: &[u8], file_path: Option<&str>) -> StructuredMapper {
    let mut errors = Vec::new();

    let mapper_file = match parser::parse_xml(xml) {
        Ok(m) => m,
        Err(e) => {
            return StructuredMapper {
                namespace: String::new(),
                statements: Vec::new(),
                fragments: Vec::new(),
                errors: vec![e],
            };
        }
    };

    let mut mapper_file = match resolver::resolve_includes(&mapper_file) {
        Ok(m) => m,
        Err(e) => {
            errors.push(e);
            return StructuredMapper {
                namespace: mapper_file.namespace,
                statements: Vec::new(),
                fragments: Vec::new(),
                errors,
            };
        }
    };

    for stmt in mapper_file.statements.iter_mut() {
        optimize::optimize_exclusive_ifs(&mut stmt.body);
    }

    let statements: Vec<StructuredStatement> = mapper_file
        .statements
        .iter()
        .map(|stmt| {
            let has_dynamic = has_dynamic_elements(&stmt.body);
            let collected = flatten::collect_params(&stmt.body);
            let parameters: Vec<ParamMeta> = collected
                .iter()
                .map(|(name, type_hint, raw)| {
                    let jdbc_type = type_hint.as_ref().and_then(|s| util::jdbc_type_from_str(s));
                    ParamMeta {
                        name: name.clone(),
                        jdbc_type,
                        source: type_hint.as_ref().map(|_| InferenceSource::InlineJdbcType),
                        position: 0,
                        raw: raw.clone(),
                    }
                })
                .collect();

            StructuredStatement {
                id: stmt.id.clone(),
                kind: stmt.kind,
                parameter_type: stmt.parameter_type.clone(),
                result_type: stmt.result_type.clone(),
                body: stmt.body.clone(),
                has_dynamic_elements: has_dynamic,
                location: XmlSourceLocation { file_path: file_path.map(|s| s.to_string()), line: stmt.line },
                parameters,
                database_id: stmt.database_id.clone(),
                statement_type: stmt.statement_type.clone(),
            }
        })
        .collect();

    if statements.is_empty() && errors.is_empty() {
        errors.push(IbatisError::EmptyMapper);
    }

    StructuredMapper { namespace: mapper_file.namespace, statements, fragments: mapper_file.fragments, errors }
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

    let mut mapper_file = match resolver::resolve_includes(&mapper_file) {
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

    for stmt in mapper_file.statements.iter_mut() {
        optimize::optimize_exclusive_ifs(&mut stmt.body);
    }

    let mut statements = Vec::new();
    for stmt in &mapper_file.statements {
        let mut flat_sql = flatten::flatten_sql(&stmt.body);

        let mut collected = flatten::collect_params(&stmt.body);

        apply_parameter_map(&mut flat_sql, &mut collected, &stmt.parameter_type, &mapper_file.parameter_maps);

        #[cfg(feature = "java")]
        let parameters = {
            let resolver = java_resolve::JavaSourceResolver::new(java_source_roots.clone());
            infer_param_types(&mapper_file.namespace, stmt, &collected, &resolver, &mapper_file.type_aliases)
        };
        #[cfg(not(feature = "java"))]
        let parameters: Vec<types::ParamMeta> = collected
            .iter()
            .map(|(name, type_hint, raw)| {
                let jdbc_type = type_hint.as_ref().and_then(|s| util::jdbc_type_from_str(s));
                types::ParamMeta {
                    name: name.clone(),
                    jdbc_type,
                    source: type_hint.as_ref().map(|_| types::InferenceSource::InlineJdbcType),
                    position: 0,
                    raw: raw.clone(),
                }
            })
            .collect();

        for param in &parameters {
            if let Some(jdbc) = &param.jdbc_type {
                let jdbc_str = format!("{:?}", jdbc).to_uppercase();
                for prefix in ["__XML_PARAM_", "__XML_RAW_"] {
                    let untyped = format!("{}{}{}", prefix, param.name, "__");
                    let typed = format!("{}{}_{}{}", prefix, jdbc_str, param.name, "__");
                    flat_sql = flat_sql.replace(&untyped, &typed);
                }
            }
        }

        let has_dynamic = has_dynamic_elements(&stmt.body);
        let parse_sql = if stmt.statement_type.as_deref() == Some("CALLABLE") {
            crate::translate_jdbc_call(&flat_sql)
        } else {
            flat_sql.clone()
        };
        // Trim before parsing to avoid leading whitespace from XML indentation
        // skewing error line numbers.
        let parse_sql_trimmed = parse_sql.trim().to_string();
        let parse_result = if !parse_sql_trimmed.is_empty() {
            Some(crate::parser::Parser::parse_sql(&parse_sql_trimmed))
        } else {
            None
        };

        // Compute the XML line where body content begins (after leading whitespace).
        let leading_newlines = flat_sql.chars().take_while(|c| c.is_whitespace()).filter(|c| *c == '\n').count();
        let body_start_line = stmt.line + leading_newlines;

        // Remap SQL parse error line numbers to XML file line numbers.
        let parse_result = parse_result.map(|(infos, errors)| {
            let errors = errors.into_iter().map(|e| remap_error_line(e, body_start_line as isize - 1)).collect();
            (infos, errors)
        });

        statements.push(ParsedStatement {
            id: stmt.id.clone(),
            kind: stmt.kind,
            parameter_type: stmt.parameter_type.clone(),
            result_type: stmt.result_type.clone(),
            flat_sql,
            parameters,
            has_dynamic_elements: has_dynamic,
            line: stmt.line,
            body_start_line,
            parse_result,
            database_id: stmt.database_id.clone(),
            statement_type: stmt.statement_type.clone(),
        });
    }

    if statements.is_empty() && errors.is_empty() {
        errors.push(IbatisError::EmptyMapper);
    }

    ParsedMapper { file_path: file_path.map(|s| s.to_string()), namespace: mapper_file.namespace, statements, errors }
}

#[cfg(feature = "java")]
fn infer_param_types(
    namespace: &str,
    stmt: &types::MapperStatement,
    collected_params: &[(String, Option<String>, String)],
    resolver: &java_resolve::JavaSourceResolver,
    type_aliases: &[(String, String)],
) -> Vec<types::ParamMeta> {
    use crate::ibatis::java_resolve::{java_type_to_jdbc, jdbc_type_from_str};
    use crate::ibatis::types::{InferenceSource, ParamMeta};

    let resolved_param_type: Option<String> = stmt.parameter_type.as_ref().map(|pt| {
        type_aliases
            .iter()
            .find(|(alias, _)| alias.eq_ignore_ascii_case(pt))
            .map(|(_, fqn)| fqn.clone())
            .unwrap_or_else(|| pt.clone())
    });

    // P0-A: parameterClass is a simple type (e.g. "java.lang.Integer", "int") -> all params inherit it
    let simple_param_jdbc = resolved_param_type.as_ref().and_then(|pt| {
        let jdbc = java_type_to_jdbc(pt);
        if jdbc.is_some() {
            return jdbc;
        }
        // FQN last segment: "java.lang.Integer" -> "Integer"
        pt.rsplit('.').next().and_then(java_type_to_jdbc)
    });

    // Parse Mapper interface (if available)
    let interface_info = resolver.read_source(namespace).and_then(|src| crate::java::parse_mapper_interface(&src));

    // Get method params for this statement
    let method_params = interface_info.as_ref().and_then(|info| info.methods.get(&stmt.id)).map(|m| &m.params);

    // P0-B: short names resolved via typeAlias, then by class name search
    let mut dto_fields: std::collections::HashMap<String, String> = resolved_param_type
        .as_ref()
        .filter(|pt| pt.contains('.') && !pt.eq_ignore_ascii_case("map"))
        .and_then(|pt| resolver.read_source(pt))
        .map(|src| crate::java::parse_dto_fields(&src))
        .unwrap_or_default();

    // P0-B: Try resolve_by_class_name for short names (e.g. "account" -> testdomain.Account)
    if dto_fields.is_empty() {
        if let Some(ref pt) = resolved_param_type {
            if !pt.contains('.') && !pt.eq_ignore_ascii_case("map") {
                if let Some(src) = resolver.read_source_by_class_name(pt) {
                    let fields = crate::java::parse_dto_fields(&src);
                    dto_fields.extend(fields);
                }
            }
        }
    }

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

    collected_params
        .iter()
        .map(|(name, inline_java_type, raw)| {
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

            // Priority 2: parameterClass is a simple type -> all params inherit that type
            if let Some(jdbc) = simple_param_jdbc {
                return ParamMeta {
                    name: name.clone(),
                    jdbc_type: Some(jdbc),
                    source: Some(InferenceSource::ParameterClass),
                    position: 0,
                    raw: raw.clone(),
                };
            }

            // Priority 3: Mapper interface method signature
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

            // Priority 4: DTO fields
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
            ParamMeta { name: name.clone(), jdbc_type: None, source: None, position: 0, raw: raw.clone() }
        })
        .collect()
}

fn apply_parameter_map(
    flat_sql: &mut String,
    collected: &mut Vec<(String, Option<String>, String)>,
    parameter_type: &Option<String>,
    parameter_maps: &[types::ParameterMapDef],
) {
    let Some(ref param_type) = parameter_type else { return };
    let pmap = match parameter_maps.iter().find(|pm| pm.id == *param_type) {
        Some(pm) => pm,
        None => return,
    };
    if pmap.params.is_empty() {
        return;
    }
    let mut idx = 0;
    let mut result = String::with_capacity(flat_sql.len());
    let chars: Vec<char> = flat_sql.chars().collect();
    let mut i = 0;
    let mut in_string = false;
    while i < chars.len() {
        let c = chars[i];
        if c == '\'' && !in_string {
            in_string = true;
            result.push(c);
            i += 1;
            continue;
        }
        if c == '\'' && in_string {
            if i + 1 < chars.len() && chars[i + 1] == '\'' {
                result.push_str("''");
                i += 2;
                continue;
            }
            in_string = false;
            result.push(c);
            i += 1;
            continue;
        }
        if in_string {
            result.push(c);
            i += 1;
            continue;
        }
        if c == '?' {
            if idx < pmap.params.len() {
                let entry = &pmap.params[idx];
                let placeholder = match &entry.jdbc_type {
                    Some(jt) => format!("__XML_PARAM_{}_{}__", jt.to_uppercase(), entry.property),
                    None => format!("__XML_PARAM_{}__", entry.property),
                };
                result.push_str(&placeholder);
                if !collected.iter().any(|(n, _, _)| *n == entry.property) {
                    let raw = match &entry.jdbc_type {
                        Some(jt) => format!("#{{{},jdbcType={}}}", entry.property, jt),
                        None => format!("#{{{}}}", entry.property),
                    };
                    collected.push((entry.property.clone(), entry.jdbc_type.clone(), raw));
                }
                idx += 1;
            } else {
                result.push(c);
            }
            i += 1;
            continue;
        }
        result.push(c);
        i += 1;
    }
    *flat_sql = result;
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

/// Offset a `ParserError`'s line number by `offset` (may be negative).
/// Used to remap SQL-text line numbers to XML file line numbers.
fn remap_error_line(mut err: crate::parser::ParserError, offset: isize) -> crate::parser::ParserError {
    use crate::token::SourceLocation;
    let adjust = |loc: &mut SourceLocation| {
        loc.line = (loc.line as isize + offset).max(1) as usize;
    };
    match &mut err {
        crate::parser::ParserError::UnexpectedToken { location, .. } => adjust(location),
        crate::parser::ParserError::UnexpectedEof { location, .. } => adjust(location),
        crate::parser::ParserError::Warning { location, .. } => adjust(location),
        crate::parser::ParserError::ReservedKeywordAsIdentifier { location, .. } => adjust(location),
        crate::parser::ParserError::UnsupportedSyntax { location, .. } => adjust(location),
        crate::parser::ParserError::TokenizerError(_) => {}
    }
    err
}

#[cfg(test)]
mod tests;
