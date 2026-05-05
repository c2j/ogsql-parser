use std::path::{Path, PathBuf};

static JAVA_TO_JDBC: &[(&str, crate::ibatis::types::JdbcType)] = &[
    ("int",          crate::ibatis::types::JdbcType::Integer),
    ("long",         crate::ibatis::types::JdbcType::BigInt),
    ("short",        crate::ibatis::types::JdbcType::SmallInt),
    ("byte",         crate::ibatis::types::JdbcType::TinyInt),
    ("float",        crate::ibatis::types::JdbcType::Float),
    ("double",       crate::ibatis::types::JdbcType::Double),
    ("boolean",      crate::ibatis::types::JdbcType::Boolean),
    ("char",         crate::ibatis::types::JdbcType::Char),
    ("Integer",      crate::ibatis::types::JdbcType::Integer),
    ("Long",         crate::ibatis::types::JdbcType::BigInt),
    ("Short",        crate::ibatis::types::JdbcType::SmallInt),
    ("Byte",         crate::ibatis::types::JdbcType::TinyInt),
    ("Float",        crate::ibatis::types::JdbcType::Float),
    ("Double",       crate::ibatis::types::JdbcType::Double),
    ("Boolean",      crate::ibatis::types::JdbcType::Boolean),
    ("Character",    crate::ibatis::types::JdbcType::Char),
    ("String",       crate::ibatis::types::JdbcType::VarChar),
    ("BigDecimal",   crate::ibatis::types::JdbcType::Decimal),
    ("Date",         crate::ibatis::types::JdbcType::Timestamp),
    ("LocalDate",    crate::ibatis::types::JdbcType::Date),
    ("LocalDateTime",crate::ibatis::types::JdbcType::Timestamp),
    ("LocalTime",    crate::ibatis::types::JdbcType::Time),
    ("Timestamp",    crate::ibatis::types::JdbcType::Timestamp),
    ("byte[]",       crate::ibatis::types::JdbcType::VarBinary),
    ("Object",       crate::ibatis::types::JdbcType::Other),
];

static JDBC_TYPE_MAP: &[(&str, crate::ibatis::types::JdbcType)] = &[
    ("INTEGER",      crate::ibatis::types::JdbcType::Integer),
    ("BIGINT",       crate::ibatis::types::JdbcType::BigInt),
    ("SMALLINT",     crate::ibatis::types::JdbcType::SmallInt),
    ("TINYINT",      crate::ibatis::types::JdbcType::TinyInt),
    ("DECIMAL",      crate::ibatis::types::JdbcType::Decimal),
    ("NUMERIC",      crate::ibatis::types::JdbcType::Numeric),
    ("DOUBLE",       crate::ibatis::types::JdbcType::Double),
    ("FLOAT",        crate::ibatis::types::JdbcType::Float),
    ("REAL",         crate::ibatis::types::JdbcType::Real),
    ("CHAR",         crate::ibatis::types::JdbcType::Char),
    ("VARCHAR",      crate::ibatis::types::JdbcType::VarChar),
    ("LONGVARCHAR",  crate::ibatis::types::JdbcType::LongVarChar),
    ("NCHAR",        crate::ibatis::types::JdbcType::NChar),
    ("NVARCHAR",     crate::ibatis::types::JdbcType::NVarChar),
    ("CLOB",         crate::ibatis::types::JdbcType::Clob),
    ("NCLOB",        crate::ibatis::types::JdbcType::NClob),
    ("BINARY",       crate::ibatis::types::JdbcType::Binary),
    ("VARBINARY",    crate::ibatis::types::JdbcType::VarBinary),
    ("BLOB",         crate::ibatis::types::JdbcType::Blob),
    ("DATE",         crate::ibatis::types::JdbcType::Date),
    ("TIME",         crate::ibatis::types::JdbcType::Time),
    ("TIMESTAMP",    crate::ibatis::types::JdbcType::Timestamp),
    ("BOOLEAN",      crate::ibatis::types::JdbcType::Boolean),
    ("NULL",         crate::ibatis::types::JdbcType::Null),
    ("ARRAY",        crate::ibatis::types::JdbcType::Array),
    ("OTHER",        crate::ibatis::types::JdbcType::Other),
];

#[derive(Debug, Clone)]
pub struct JavaSourceResolver {
    roots: Vec<PathBuf>,
}

impl JavaSourceResolver {
    pub fn new(roots: Vec<PathBuf>) -> Self {
        Self { roots }
    }

    pub fn empty() -> Self {
        Self { roots: Vec::new() }
    }

    pub fn read_source(&self, fqn: &str) -> Option<String> {
        let path = self.resolve(fqn)?;
        std::fs::read_to_string(&path).ok()
    }

    pub fn resolve(&self, fqn: &str) -> Option<PathBuf> {
        let relative = fqn.replace('.', "/") + ".java";
        self.roots.iter()
            .map(|root| root.join(&relative))
            .find(|path| path.is_file())
    }
}

pub fn java_type_to_jdbc(java_type: &str) -> Option<crate::ibatis::types::JdbcType> {
    JAVA_TO_JDBC.iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(java_type))
        .map(|(_, jdbc)| *jdbc)
}

pub fn jdbc_type_from_str(s: &str) -> Option<crate::ibatis::types::JdbcType> {
    JDBC_TYPE_MAP.iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(s))
        .map(|(_, jdbc)| *jdbc)
}
