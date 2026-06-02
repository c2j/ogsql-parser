//! 共享工具函数。

use crate::ibatis::types::JdbcType;

/// 从 `#{...}` 中解析出的参数属性。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ParamAttrs {
    pub java_type: Option<String>,
    pub jdbc_type: Option<String>,
    pub mode: Option<String>,
    pub result_map: Option<String>,
}

/// 从位置 start 开始查找匹配的 `}`，考虑嵌套 `{}`。
pub fn find_closing_brace(chars: &[char], start: usize) -> Option<usize> {
    let mut depth = 1;
    let mut i = start;
    while i < chars.len() {
        match chars[i] {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// 完整解析 MyBatis #{...} 参数字符串，返回 (name, ParamAttrs)。
pub fn parse_param_attrs(param: &str) -> (String, ParamAttrs) {
    if param.contains(',') {
        parse_param_attrs_mybatis3(param)
    } else if param.contains(':') {
        parse_param_attrs_ibatis2(param)
    } else {
        (param.trim().to_string(), ParamAttrs::default())
    }
}

fn parse_param_attrs_mybatis3(param: &str) -> (String, ParamAttrs) {
    let mut parts = param.split(',');
    let name = parts.next().unwrap_or("").trim().to_string();
    let mut attrs = ParamAttrs::default();
    for part in parts {
        let part = part.trim();
        if let Some(val) = part.strip_prefix("javaType=") {
            attrs.java_type = Some(val.to_string());
        } else if let Some(val) = part.strip_prefix("jdbcType=") {
            attrs.jdbc_type = Some(val.to_string());
        } else if let Some(val) = part.strip_prefix("mode=") {
            attrs.mode = Some(val.to_string());
        } else if let Some(val) = part.strip_prefix("resultMap=") {
            attrs.result_map = Some(val.to_string());
        }
    }
    (name, attrs)
}

fn parse_param_attrs_ibatis2(param: &str) -> (String, ParamAttrs) {
    let mut parts = param.splitn(3, ':');
    let name = parts.next().unwrap_or("").trim().to_string();
    let jdbc_type = parts.next().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    (name, ParamAttrs { jdbc_type, ..ParamAttrs::default() })
}

/// 从 MyBatis 参数字符串中提取 name 和可选的 javaType/jdbcType。
/// 格式:
/// - MyBatis 3: `name` or `name,javaType=double` or `name,jdbcType=NUMERIC`
/// - iBatis 2.x: `name` or `name:jdbcType` or `name:jdbcType:nullValue`
pub fn parse_param_type(param: &str) -> (String, Option<String>) {
    let (name, attrs) = parse_param_attrs(param);
    (name, attrs.jdbc_type.or(attrs.java_type))
}

/// 将 JDBC 类型字符串（如 "VARCHAR"）转换为 `JdbcType` 枚举。
pub fn jdbc_type_from_str(s: &str) -> Option<JdbcType> {
    match s.to_uppercase().as_str() {
        "INTEGER" | "INT" => Some(JdbcType::Integer),
        "BIGINT" => Some(JdbcType::BigInt),
        "SMALLINT" => Some(JdbcType::SmallInt),
        "TINYINT" => Some(JdbcType::TinyInt),
        "DECIMAL" => Some(JdbcType::Decimal),
        "NUMERIC" => Some(JdbcType::Numeric),
        "DOUBLE" => Some(JdbcType::Double),
        "FLOAT" => Some(JdbcType::Float),
        "REAL" => Some(JdbcType::Real),
        "CHAR" => Some(JdbcType::Char),
        "VARCHAR" => Some(JdbcType::VarChar),
        "LONGVARCHAR" => Some(JdbcType::LongVarChar),
        "NCHAR" => Some(JdbcType::NChar),
        "NVARCHAR" => Some(JdbcType::NVarChar),
        "CLOB" => Some(JdbcType::Clob),
        "NCLOB" => Some(JdbcType::NClob),
        "BINARY" => Some(JdbcType::Binary),
        "VARBINARY" => Some(JdbcType::VarBinary),
        "BLOB" => Some(JdbcType::Blob),
        "DATE" => Some(JdbcType::Date),
        "TIME" => Some(JdbcType::Time),
        "TIMESTAMP" => Some(JdbcType::Timestamp),
        "BOOLEAN" => Some(JdbcType::Boolean),
        "NULL" => Some(JdbcType::Null),
        "ARRAY" => Some(JdbcType::Array),
        "OTHER" => Some(JdbcType::Other),
        "CURSOR" => Some(JdbcType::Cursor),
        _ => None,
    }
}
