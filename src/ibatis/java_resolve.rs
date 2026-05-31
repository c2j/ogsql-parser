use std::collections::HashSet;
use std::io::Read;
use std::path::{Path, PathBuf};

static JAVA_TO_JDBC: &[(&str, crate::ibatis::types::JdbcType)] = &[
    ("int", crate::ibatis::types::JdbcType::Integer),
    ("long", crate::ibatis::types::JdbcType::BigInt),
    ("short", crate::ibatis::types::JdbcType::SmallInt),
    ("byte", crate::ibatis::types::JdbcType::TinyInt),
    ("float", crate::ibatis::types::JdbcType::Float),
    ("double", crate::ibatis::types::JdbcType::Double),
    ("boolean", crate::ibatis::types::JdbcType::Boolean),
    ("char", crate::ibatis::types::JdbcType::Char),
    ("Integer", crate::ibatis::types::JdbcType::Integer),
    ("Long", crate::ibatis::types::JdbcType::BigInt),
    ("Short", crate::ibatis::types::JdbcType::SmallInt),
    ("Byte", crate::ibatis::types::JdbcType::TinyInt),
    ("Float", crate::ibatis::types::JdbcType::Float),
    ("Double", crate::ibatis::types::JdbcType::Double),
    ("Boolean", crate::ibatis::types::JdbcType::Boolean),
    ("Character", crate::ibatis::types::JdbcType::Char),
    ("String", crate::ibatis::types::JdbcType::VarChar),
    ("BigDecimal", crate::ibatis::types::JdbcType::Decimal),
    ("Date", crate::ibatis::types::JdbcType::Timestamp),
    ("LocalDate", crate::ibatis::types::JdbcType::Date),
    ("LocalDateTime", crate::ibatis::types::JdbcType::Timestamp),
    ("LocalTime", crate::ibatis::types::JdbcType::Time),
    ("Timestamp", crate::ibatis::types::JdbcType::Timestamp),
    ("byte[]", crate::ibatis::types::JdbcType::VarBinary),
    ("Object", crate::ibatis::types::JdbcType::Other),
];

static JDBC_TYPE_MAP: &[(&str, crate::ibatis::types::JdbcType)] = &[
    ("INTEGER", crate::ibatis::types::JdbcType::Integer),
    ("BIGINT", crate::ibatis::types::JdbcType::BigInt),
    ("SMALLINT", crate::ibatis::types::JdbcType::SmallInt),
    ("TINYINT", crate::ibatis::types::JdbcType::TinyInt),
    ("DECIMAL", crate::ibatis::types::JdbcType::Decimal),
    ("NUMERIC", crate::ibatis::types::JdbcType::Numeric),
    ("DOUBLE", crate::ibatis::types::JdbcType::Double),
    ("FLOAT", crate::ibatis::types::JdbcType::Float),
    ("REAL", crate::ibatis::types::JdbcType::Real),
    ("CHAR", crate::ibatis::types::JdbcType::Char),
    ("VARCHAR", crate::ibatis::types::JdbcType::VarChar),
    ("LONGVARCHAR", crate::ibatis::types::JdbcType::LongVarChar),
    ("NCHAR", crate::ibatis::types::JdbcType::NChar),
    ("NVARCHAR", crate::ibatis::types::JdbcType::NVarChar),
    ("CLOB", crate::ibatis::types::JdbcType::Clob),
    ("NCLOB", crate::ibatis::types::JdbcType::NClob),
    ("BINARY", crate::ibatis::types::JdbcType::Binary),
    ("VARBINARY", crate::ibatis::types::JdbcType::VarBinary),
    ("BLOB", crate::ibatis::types::JdbcType::Blob),
    ("DATE", crate::ibatis::types::JdbcType::Date),
    ("TIME", crate::ibatis::types::JdbcType::Time),
    ("TIMESTAMP", crate::ibatis::types::JdbcType::Timestamp),
    ("BOOLEAN", crate::ibatis::types::JdbcType::Boolean),
    ("NULL", crate::ibatis::types::JdbcType::Null),
    ("ARRAY", crate::ibatis::types::JdbcType::Array),
    ("OTHER", crate::ibatis::types::JdbcType::Other),
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
        self.roots.iter().map(|root| root.join(&relative)).find(|path| path.is_file())
    }

    pub fn resolve_by_class_name(&self, class_name: &str) -> Option<PathBuf> {
        let target = format!("{}.java", class_name);
        for root in &self.roots {
            let found = walkdir::WalkDir::new(root)
                .into_iter()
                .filter_map(|e| e.ok())
                .find(|e| e.file_name().to_str().map(|n| n == target).unwrap_or(false));
            if let Some(entry) = found {
                return Some(entry.into_path());
            }
        }
        None
    }

    pub fn read_source_by_class_name(&self, class_name: &str) -> Option<String> {
        let path = self.resolve_by_class_name(class_name)?;
        std::fs::read_to_string(&path).ok()
    }
}

pub fn java_type_to_jdbc(java_type: &str) -> Option<crate::ibatis::types::JdbcType> {
    JAVA_TO_JDBC.iter().find(|(name, _)| name.eq_ignore_ascii_case(java_type)).map(|(_, jdbc)| *jdbc)
}

pub fn jdbc_type_from_str(s: &str) -> Option<crate::ibatis::types::JdbcType> {
    JDBC_TYPE_MAP.iter().find(|(name, _)| name.eq_ignore_ascii_case(s)).map(|(_, jdbc)| *jdbc)
}

fn extract_package_decl(content: &str) -> Option<String> {
    let package_start = content.find("package ")?;
    let after_keyword = package_start + "package ".len();
    let semicolon_pos = content[after_keyword..].find(';')?;
    let package_name = content[after_keyword..after_keyword + semicolon_pos].trim();
    if package_name.is_empty() {
        return None;
    }
    Some(package_name.to_string())
}

fn infer_root_from_java_file(path: &Path) -> Option<PathBuf> {
    let file = std::fs::File::open(path).ok()?;
    let mut limited = file.take(2000);
    let mut content = String::new();
    std::io::Read::read_to_string(&mut limited, &mut content).ok()?;

    let package = extract_package_decl(&content)?;
    let package_path = package.replace('.', std::path::MAIN_SEPARATOR_STR);

    let path_str = path.to_str()?;
    let package_idx = path_str.rfind(&package_path)?;
    let root_str = &path_str[..package_idx];
    let root = PathBuf::from(root_str);

    if walkdir::WalkDir::new(&root)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
        .any(|e| e.file_name().to_str().map(|n| n.ends_with(".java")).unwrap_or(false))
    {
        Some(root)
    } else {
        None
    }
}

pub fn detect_java_roots(scan_dir: &Path) -> Vec<PathBuf> {
    let standard_dirs = [
        "src/main/java",
        "src/test/java",
        "src/main/generated/java",
    ];

    let mut standard_roots = Vec::new();
    for dir in &standard_dirs {
        let candidate = scan_dir.join(dir);
        if candidate.is_dir() {
            standard_roots.push(candidate);
        }
    }

    if !standard_roots.is_empty() {
        return standard_roots;
    }

    let mut roots = HashSet::new();
    let mut sampled = 0;
    const MAX_SAMPLES: usize = 50;

    for entry in walkdir::WalkDir::new(scan_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_str().map(|n| n.ends_with(".java")).unwrap_or(false))
    {
        if sampled >= MAX_SAMPLES {
            break;
        }
        sampled += 1;

        if let Some(root) = infer_root_from_java_file(entry.path()) {
            roots.insert(root);
        }
    }

    roots.into_iter().collect()
}
