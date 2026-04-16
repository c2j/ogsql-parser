pub(crate) mod alter;
pub(crate) mod create;
pub(crate) mod drop;
pub(crate) mod table;

use crate::ast::DataType;

pub(crate) fn format_data_type(dt: &DataType) -> String {
    match dt {
        DataType::Boolean => "boolean".to_string(),
        DataType::TinyInt => "tinyint".to_string(),
        DataType::SmallInt => "smallint".to_string(),
        DataType::Integer => "integer".to_string(),
        DataType::BigInt => "bigint".to_string(),
        DataType::Real => "real".to_string(),
        DataType::Float(Some(n)) => format!("float({})", n),
        DataType::Float(None) => "float".to_string(),
        DataType::Double => "double precision".to_string(),
        DataType::Serial => "serial".to_string(),
        DataType::SmallSerial => "smallserial".to_string(),
        DataType::BigSerial => "bigserial".to_string(),
        DataType::BinaryFloat => "binary_float".to_string(),
        DataType::BinaryDouble => "binary_double".to_string(),
        DataType::Text => "text".to_string(),
        DataType::Char(Some(n)) => format!("char({})", n),
        DataType::Char(None) => "char".to_string(),
        DataType::Varchar(Some(n)) => format!("varchar({})", n),
        DataType::Varchar(None) => "varchar".to_string(),
        DataType::Numeric(Some(p), Some(s)) => format!("numeric({},{})", p, s),
        DataType::Numeric(Some(p), None) => format!("numeric({})", p),
        DataType::Numeric(None, _) => "numeric".to_string(),
        DataType::Date => "date".to_string(),
        DataType::Timestamp(Some(p), _) => format!("timestamp({})", p),
        DataType::Timestamp(None, _) => "timestamp".to_string(),
        DataType::Uuid => "uuid".to_string(),
        DataType::Json => "json".to_string(),
        DataType::Jsonb => "jsonb".to_string(),
        DataType::Bytea => "bytea".to_string(),
        DataType::Custom(obj, _) => obj.join("."),
        other => format!("{:?}", other).to_lowercase(),
    }
}
