/// Function registry for OpenGauss built-in function recognition.
///
/// Provides metadata lookup (`lookup_function`) and validation (`validate_function_call`)
/// for built-in functions. Core layer is compile-time constant; extension layer TBD.
use std::sync::OnceLock;

use crate::parser::ParserError;
use crate::token::SourceLocation;

// ────────────────────────────────────────────────────────────────
// Types
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FuncCategory {
    Aggregate,
    Window,
    Scalar,
    SetReturning,
    Special,
    TypeConstructor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FuncDomain {
    Math,
    String,
    DateTime,
    Aggregate,
    Window,
    Array,
    Json,
    Network,
    Geometric,
    Hash,
    Range,
    TextSearch,
    Crypto,
    System,
    ExceptionContext,
    TypeConversion,
    OracleCompat,
    // ── Oracle 兼容包函数域 ──
    DbeApplicationInfo,
    DbeFile,
    DbeLob,
    DbeMatch,
    DbeOutput,
    DbeRandom,
    DbeRaw,
    DbeScheduler,
    DbeSession,
    DbeSql,
    DbeSqlUtil,
    DbeStats,
    DbeTask,
    DbeUtility,
    DbeXmlDom,
    DbeXmlParser,
    DbmsLob,
    DbmsOutput,
    DbmsScheduler,
    DbmsSql,
    DbmsUtility,
    PkgService,
    PkgUtil,
    UtlFile,
    Xml,
    // ── 其他 ──
    Ai,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CompatMode(u8);

impl CompatMode {
    pub const A_FORMAT: CompatMode = CompatMode(0x01);
    pub const B_FORMAT: CompatMode = CompatMode(0x02);
    pub const PG_FORMAT: CompatMode = CompatMode(0x04);
    pub const ALL: CompatMode = CompatMode(0x07);

    pub fn contains(self, other: CompatMode) -> bool {
        (self.0 & other.0) != 0
    }
}

impl std::ops::BitOr for CompatMode {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        CompatMode(self.0 | rhs.0)
    }
}

/// Distribution origin — which database products ship this function.
/// Separate orthogonal dimension from CompatMode (which tracks A/B/PG syntax format).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Distribution(u8);

impl Distribution {
    /// Present in open-source openGauss
    pub const OPENGAUSS: Distribution = Distribution(0x01);
    /// Present in commercial GaussDB (Huawei Cloud / Stack)
    pub const GAUSSDB: Distribution = Distribution(0x02);
    /// Present in both (the default for existing entries)
    pub const BOTH: Distribution = Distribution(0x03);

    /// Returns true if this distribution includes the given product.
    pub fn contains(self, other: Distribution) -> bool {
        (self.0 & other.0) != 0
    }

    /// True if this function is ONLY in commercial GaussDB, not in open-source openGauss.
    pub fn is_commercial_only(self) -> bool {
        self.0 == Self::GAUSSDB.0
    }
}

impl Default for Distribution {
    fn default() -> Self {
        Distribution::BOTH
    }
}

impl std::ops::BitOr for Distribution {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Distribution(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FuncMeta {
    pub name: &'static str,
    pub category: FuncCategory,
    pub domain: FuncDomain,
    pub min_args: u8,
    pub max_args: Option<u8>,
    pub supports_distinct: bool,
    pub compat: CompatMode,
    pub distribution: Distribution,
}

/// Owned variant of `FuncMeta` for runtime-loaded extension functions.
/// JSON fields map 1:1 to the core metadata type.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FuncMetaOwned {
    pub name: String,
    pub category: FuncCategory,
    pub domain: FuncDomain,
    pub min_args: u8,
    pub max_args: Option<u8>,
    pub supports_distinct: bool,
    pub compat: CompatMode,
    #[serde(default)]
    pub distribution: Distribution,
}

/// Unified lookup result from either the core static registry or runtime extensions.
#[derive(Debug, Clone, Copy)]
pub struct LookupResult {
    pub name: &'static str,
    pub category: FuncCategory,
    pub domain: FuncDomain,
    pub min_args: u8,
    pub max_args: Option<u8>,
    pub supports_distinct: bool,
    pub compat: CompatMode,
    pub distribution: Distribution,
}

impl From<&FuncMeta> for LookupResult {
    fn from(m: &FuncMeta) -> Self {
        LookupResult {
            name: m.name,
            category: m.category,
            domain: m.domain,
            min_args: m.min_args,
            max_args: m.max_args,
            supports_distinct: m.supports_distinct,
            compat: m.compat,
            distribution: m.distribution,
        }
    }
}

impl From<&FuncMetaOwned> for LookupResult {
    fn from(m: &FuncMetaOwned) -> Self {
        LookupResult {
            name: "",
            category: m.category,
            domain: m.domain,
            min_args: m.min_args,
            max_args: m.max_args,
            supports_distinct: m.supports_distinct,
            compat: m.compat,
            distribution: m.distribution,
        }
    }
}

use std::collections::HashMap;

/// Extensible function registry that layers runtime-loaded entries on top of
/// the core compile-time static registry.
pub struct FunctionRegistry {
    extensions: HashMap<String, FuncMetaOwned>,
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionRegistry {
    pub fn new() -> Self {
        FunctionRegistry { extensions: HashMap::new() }
    }

    /// Load extension entries from a JSON string (array of `FuncMetaOwned`).
    ///
    /// # Errors
    ///
    /// Returns `Err(RegistryError)` if the JSON cannot be deserialized.
    pub fn with_extensions_from_json(mut self, json: &str) -> Result<Self, RegistryError> {
        let entries: Vec<FuncMetaOwned> = serde_json::from_str(json)?;
        for entry in entries {
            let key = entry.name.to_ascii_lowercase();
            self.extensions.insert(key, entry);
        }
        Ok(self)
    }

    /// Lookup a function by name. Extensions take priority over core entries.
    pub fn lookup(&self, name: &str) -> Option<LookupResult> {
        let lower = name.to_ascii_lowercase();
        if let Some(ext) = self.extensions.get(&lower) {
            return Some(LookupResult::from(ext));
        }
        lookup_function(name).map(LookupResult::from)
    }

    /// Validate a function call using the merged view (extensions + core).
    pub fn validate(
        &self,
        name: &str,
        arg_count: usize,
        has_distinct: bool,
        has_over: bool,
        has_variadic: bool,
        location: SourceLocation,
    ) -> Vec<ParserError> {
        let Some(meta) = self.lookup(name) else {
            return Vec::new();
        };

        let mut warnings = Vec::new();
        let display_name = name;

        if has_variadic {
        } else {
            match meta.max_args {
                Some(max) if meta.min_args == max && max == 0 => {
                    if arg_count > 0 {
                        warnings.push(ParserError::Warning {
                            message: format!("function {} takes no arguments", display_name),
                            location,
                        });
                    }
                }
                Some(max) if meta.min_args == max => {
                    if arg_count != meta.min_args as usize {
                        warnings.push(ParserError::Warning {
                            message: format!(
                                "function {} requires exactly {} argument(s)",
                                display_name, meta.min_args
                            ),
                            location,
                        });
                    }
                }
                Some(max) => {
                    if arg_count < meta.min_args as usize {
                        warnings.push(ParserError::Warning {
                            message: format!(
                                "function {} requires at least {} argument(s)",
                                display_name, meta.min_args
                            ),
                            location,
                        });
                    }
                    if arg_count > max as usize {
                        warnings.push(ParserError::Warning {
                            message: format!("function {} takes at most {} argument(s)", display_name, max),
                            location,
                        });
                    }
                }
                None => {
                    if arg_count < meta.min_args as usize {
                        warnings.push(ParserError::Warning {
                            message: format!(
                                "function {} requires at least {} argument(s)",
                                display_name, meta.min_args
                            ),
                            location,
                        });
                    }
                }
            }
        }

        if has_distinct && !meta.supports_distinct {
            warnings.push(ParserError::Warning {
                message: format!("DISTINCT is not supported for function {}", display_name),
                location,
            });
        }

        if meta.category == FuncCategory::Window && !has_over {
            warnings.push(ParserError::Warning {
                message: format!("window function {} should have OVER clause", display_name),
                location,
            });
        }

        warnings
    }
}

// ────────────────────────────────────────────────────────────────
// Function registry — compile-time constant sorted array
// ────────────────────────────────────────────────────────────────

const ORACLE_COMPAT: CompatMode = CompatMode(0x05); // A_FORMAT | PG_FORMAT

macro_rules! f {
    ($name:expr, $cat:expr, $dom:expr, $min:expr, $max:expr, $dist:expr) => {
        FuncMeta {
            name: $name,
            category: $cat,
            domain: $dom,
            min_args: $min,
            max_args: $max,
            supports_distinct: $dist,
            compat: CompatMode::ALL,
            distribution: Distribution::BOTH,
        }
    };
}

macro_rules! fo {
    ($name:expr, $cat:expr, $dom:expr, $min:expr, $max:expr, $dist:expr) => {
        FuncMeta {
            name: $name,
            category: $cat,
            domain: $dom,
            min_args: $min,
            max_args: $max,
            supports_distinct: $dist,
            compat: ORACLE_COMPAT,
            distribution: Distribution::BOTH,
        }
    };
}

macro_rules! fop {
    ($name:expr, $cat:expr, $dom:expr, $min:expr, $max:expr, $dist:expr) => {
        FuncMeta {
            name: $name,
            category: $cat,
            domain: $dom,
            min_args: $min,
            max_args: $max,
            supports_distinct: $dist,
            compat: ORACLE_COMPAT,
            distribution: Distribution::BOTH,
        }
    };
}

/// Like `f!` but marks the function as commercial GaussDB only (not in open-source openGauss).
/// CompatMode::ALL + Distribution::GAUSSDB. Use for DBE_RANDOM, etc.
macro_rules! fc {
    ($name:expr, $cat:expr, $dom:expr, $min:expr, $max:expr, $dist:expr) => {
        FuncMeta {
            name: $name,
            category: $cat,
            domain: $dom,
            min_args: $min,
            max_args: $max,
            supports_distinct: $dist,
            compat: CompatMode::ALL,
            distribution: Distribution::GAUSSDB,
        }
    };
}

/// Like `fo!` but marks the function as commercial GaussDB only AND Oracle-compatible.
/// ORACLE_COMPAT + Distribution::GAUSSDB. Use for DBE_XMLDOM, DBE_XMLPARSER, etc.
macro_rules! foc {
    ($name:expr, $cat:expr, $dom:expr, $min:expr, $max:expr, $dist:expr) => {
        FuncMeta {
            name: $name,
            category: $cat,
            domain: $dom,
            min_args: $min,
            max_args: $max,
            supports_distinct: $dist,
            compat: ORACLE_COMPAT,
            distribution: Distribution::GAUSSDB,
        }
    };
}

/// Sorted (by lowercase name) array of all registered built-in functions.
static FUNCTIONS: &[FuncMeta] = &[
    // ── A ───────────────────────────────────────────────────
    f!("abbrev", FuncCategory::Scalar, FuncDomain::Network, 1, Some(1), false),
    f!("abs", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("acos", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    fo!("add_months", FuncCategory::Scalar, FuncDomain::OracleCompat, 2, Some(2), false),
    fc!("adddate", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    fc!("addtime", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    f!("aes_decrypt", FuncCategory::Scalar, FuncDomain::Crypto, 2, Some(2), false),
    f!("aes_encrypt", FuncCategory::Scalar, FuncDomain::Crypto, 2, Some(2), false),
    f!("age", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(2), false),
    f!("area", FuncCategory::Scalar, FuncDomain::Geometric, 1, Some(1), false),
    f!("array_agg", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    f!("array_append", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_cat", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_delete", FuncCategory::Scalar, FuncDomain::Array, 1, Some(1), false),
    f!("array_deleteidx", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_dims", FuncCategory::Scalar, FuncDomain::Array, 1, Some(1), false),
    f!("array_except", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_except_distinct", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_exists", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_extendnull", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_intersect", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_intersect_distinct", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_length", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_lower", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_ndims", FuncCategory::Scalar, FuncDomain::Array, 1, Some(1), false),
    f!("array_next", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    fc!("array_positions", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_prepend", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_prior", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    fc!("array_sort", FuncCategory::Scalar, FuncDomain::Array, 1, Some(1), false),
    f!("array_to_json", FuncCategory::Scalar, FuncDomain::Json, 1, Some(2), false),
    f!("array_to_string", FuncCategory::Scalar, FuncDomain::Array, 2, Some(3), false),
    f!("array_trim", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_union", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_union_distinct", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("array_upper", FuncCategory::Scalar, FuncDomain::Array, 2, Some(2), false),
    f!("ascii", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    fo!("ascii2", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    fo!("asciistr", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    f!("asin", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("atan", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("atan2", FuncCategory::Scalar, FuncDomain::Math, 2, Some(2), false),
    f!("avg", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    // ── B ───────────────────────────────────────────────────
    f!("bit_and", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    f!("bit_length", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    f!("bit_or", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    f!("broadcast", FuncCategory::Scalar, FuncDomain::Network, 1, Some(1), false),
    f!("btrim", FuncCategory::Scalar, FuncDomain::String, 1, Some(2), false),
    // ── C ───────────────────────────────────────────────────
    fc!("cardinality", FuncCategory::Scalar, FuncDomain::Array, 1, Some(1), false),
    f!("cbrt", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("ceil", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("ceiling", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("center", FuncCategory::Scalar, FuncDomain::Geometric, 1, Some(1), false),
    f!("char_length", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    f!("character_length", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    f!("chr", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    f!("circle", FuncCategory::Scalar, FuncDomain::Geometric, 1, Some(1), false),
    f!("clock_timestamp", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(0), false),
    f!("coalesce", FuncCategory::Special, FuncDomain::Other, 2, None, false),
    f!("col_description", FuncCategory::Scalar, FuncDomain::System, 2, Some(3), false),
    f!("concat", FuncCategory::Scalar, FuncDomain::String, 2, None, false),
    f!("concat_ws", FuncCategory::Scalar, FuncDomain::String, 2, None, false),
    f!("convert", FuncCategory::Scalar, FuncDomain::TypeConversion, 2, Some(3), false),
    f!("convert_from", FuncCategory::Scalar, FuncDomain::TypeConversion, 2, Some(2), false),
    f!("convert_to", FuncCategory::Scalar, FuncDomain::TypeConversion, 2, Some(2), false),
    fc!("convert_tz", FuncCategory::Scalar, FuncDomain::DateTime, 3, Some(3), false),
    f!("corr", FuncCategory::Aggregate, FuncDomain::Aggregate, 2, Some(2), true),
    f!("cos", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("cot", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("count", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    f!("covar_pop", FuncCategory::Aggregate, FuncDomain::Aggregate, 2, Some(2), true),
    f!("covar_samp", FuncCategory::Aggregate, FuncDomain::Aggregate, 2, Some(2), true),
    f!("crc32", FuncCategory::Scalar, FuncDomain::Hash, 1, Some(2), false),
    fo!("createxml", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(4), false),
    f!("cume_dist", FuncCategory::Window, FuncDomain::Window, 0, Some(0), false),
    fc!("curdate", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(0), false),
    f!("current_database", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("current_date", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(0), false),
    f!("current_schema", FuncCategory::Scalar, FuncDomain::System, 0, Some(1), false),
    f!("current_setting", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    f!("current_time", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(1), false),
    f!("current_timestamp", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(1), false),
    f!("current_user", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("currval", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    // ── D ───────────────────────────────────────────────────
    f!("cursor_to_xml", FuncCategory::Scalar, FuncDomain::Xml, 5, Some(5), false),
    f!("cursor_to_xmlschema", FuncCategory::Scalar, FuncDomain::Xml, 5, Some(5), false),
    fc!("curtime", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(1), false),
    f!("database_to_xml", FuncCategory::Scalar, FuncDomain::Xml, 4, Some(4), false),
    f!("database_to_xml_and_xmlschema", FuncCategory::Scalar, FuncDomain::Xml, 4, Some(4), false),
    f!("database_to_xmlschema", FuncCategory::Scalar, FuncDomain::Xml, 4, Some(4), false),
    fc!("date_add", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    fc!("date_format", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    f!("date_part", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    fc!("date_sub", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    f!("date_trunc", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    fc!("datediff", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    fc!("dayname", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    fc!("dayofmonth", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    fc!("dayofweek", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    fc!("dayofyear", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    fop!(
        "dbe_application_info.read_client_info",
        FuncCategory::Scalar,
        FuncDomain::DbeApplicationInfo,
        0,
        Some(1),
        false
    ),
    fop!("dbe_application_info.read_module", FuncCategory::Scalar, FuncDomain::DbeApplicationInfo, 0, Some(2), false),
    fop!("dbe_application_info.set_action", FuncCategory::Scalar, FuncDomain::DbeApplicationInfo, 1, Some(1), false),
    fop!(
        "dbe_application_info.set_client_info",
        FuncCategory::Scalar,
        FuncDomain::DbeApplicationInfo,
        1,
        Some(1),
        false
    ),
    fop!("dbe_application_info.set_module", FuncCategory::Scalar, FuncDomain::DbeApplicationInfo, 2, Some(2), false),
    foc!("dbe_compression.get_compression_ratio", FuncCategory::Scalar, FuncDomain::Other, 11, Some(13), false),
    foc!("dbe_compression.get_compression_type", FuncCategory::Scalar, FuncDomain::Other, 4, Some(5), false),
    fop!("dbe_file.close", FuncCategory::Scalar, FuncDomain::DbeFile, 1, Some(1), false),
    fop!("dbe_file.close_all", FuncCategory::Scalar, FuncDomain::DbeFile, 0, Some(0), false),
    fop!("dbe_file.copy", FuncCategory::Scalar, FuncDomain::DbeFile, 3, Some(3), false),
    fop!("dbe_file.flush", FuncCategory::Scalar, FuncDomain::DbeFile, 1, Some(1), false),
    fop!("dbe_file.fopen", FuncCategory::Scalar, FuncDomain::DbeFile, 3, Some(4), false),
    fop!("dbe_file.fopen_nchar", FuncCategory::Scalar, FuncDomain::DbeFile, 3, Some(4), false),
    fop!("dbe_file.format_write", FuncCategory::Scalar, FuncDomain::DbeFile, 2, Some(8), false),
    fop!("dbe_file.format_write_nchar", FuncCategory::Scalar, FuncDomain::DbeFile, 2, Some(7), false),
    fop!("dbe_file.get_attr", FuncCategory::Scalar, FuncDomain::DbeFile, 5, Some(5), false),
    fop!("dbe_file.get_pos", FuncCategory::Scalar, FuncDomain::DbeFile, 1, Some(1), false),
    fop!("dbe_file.get_raw", FuncCategory::Scalar, FuncDomain::DbeFile, 2, Some(3), false),
    fop!("dbe_file.is_close", FuncCategory::Scalar, FuncDomain::DbeFile, 1, Some(1), false),
    fop!("dbe_file.is_open", FuncCategory::Scalar, FuncDomain::DbeFile, 1, Some(1), false),
    fop!("dbe_file.new_line", FuncCategory::Scalar, FuncDomain::DbeFile, 1, Some(2), false),
    fop!("dbe_file.open", FuncCategory::Scalar, FuncDomain::DbeFile, 2, Some(4), false),
    fop!("dbe_file.put_raw", FuncCategory::Scalar, FuncDomain::DbeFile, 2, Some(3), false),
    fop!("dbe_file.read_line", FuncCategory::Scalar, FuncDomain::DbeFile, 2, Some(3), false),
    fop!("dbe_file.read_line_nchar", FuncCategory::Scalar, FuncDomain::DbeFile, 2, Some(3), false),
    fop!("dbe_file.remove", FuncCategory::Scalar, FuncDomain::DbeFile, 2, Some(2), false),
    fop!("dbe_file.rename", FuncCategory::Scalar, FuncDomain::DbeFile, 3, Some(3), false),
    fop!("dbe_file.seek", FuncCategory::Scalar, FuncDomain::DbeFile, 1, Some(3), false),
    fop!("dbe_file.write", FuncCategory::Scalar, FuncDomain::DbeFile, 2, Some(2), false),
    fop!("dbe_file.write_line", FuncCategory::Scalar, FuncDomain::DbeFile, 2, Some(2), false),
    fop!("dbe_file.write_line_nchar", FuncCategory::Scalar, FuncDomain::DbeFile, 2, Some(2), false),
    fop!("dbe_file.write_nchar", FuncCategory::Scalar, FuncDomain::DbeFile, 2, Some(2), false),
    foc!("dbe_heat_map.row_heat_map", FuncCategory::Scalar, FuncDomain::Other, 3, Some(5), false),
    foc!("dbe_ilm.execute_ilm", FuncCategory::Scalar, FuncDomain::Other, 3, Some(6), false),
    foc!("dbe_ilm.stop_ilm", FuncCategory::Scalar, FuncDomain::Other, 0, Some(3), false),
    foc!("dbe_ilm_admin.customize_ilm", FuncCategory::Scalar, FuncDomain::Other, 2, Some(2), false),
    foc!("dbe_ilm_admin.disable_ilm", FuncCategory::Scalar, FuncDomain::Other, 0, Some(0), false),
    foc!("dbe_ilm_admin.enable_ilm", FuncCategory::Scalar, FuncDomain::Other, 0, Some(0), false),
    fop!("dbe_lob.append", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(2), false),
    fop!("dbe_lob.bfileclose", FuncCategory::Scalar, FuncDomain::DbeLob, 1, Some(1), false),
    fop!("dbe_lob.bfilename", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(2), false),
    fop!("dbe_lob.bfileopen", FuncCategory::Scalar, FuncDomain::DbeLob, 1, Some(2), false),
    fop!("dbe_lob.close", FuncCategory::Scalar, FuncDomain::DbeLob, 1, Some(1), false),
    fop!("dbe_lob.compare", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(3), false),
    fop!("dbe_lob.converttoblob", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(5), false),
    fop!("dbe_lob.converttoclob", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(5), false),
    fop!("dbe_lob.copy", FuncCategory::Scalar, FuncDomain::DbeLob, 3, Some(5), false),
    fop!("dbe_lob.create_temporary", FuncCategory::Scalar, FuncDomain::DbeLob, 1, Some(3), false),
    fop!("dbe_lob.erase", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(3), false),
    fop!("dbe_lob.fileclose", FuncCategory::Scalar, FuncDomain::DbeLob, 1, Some(1), false),
    fop!("dbe_lob.fileopen", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(2), false),
    fop!("dbe_lob.freetemporary", FuncCategory::Scalar, FuncDomain::DbeLob, 1, Some(1), false),
    fop!("dbe_lob.get_length", FuncCategory::Scalar, FuncDomain::DbeLob, 1, Some(1), false),
    fop!("dbe_lob.getchunksize", FuncCategory::Scalar, FuncDomain::DbeLob, 1, Some(1), false),
    fop!("dbe_lob.instr", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(4), false),
    fop!("dbe_lob.loadblobfrombfile", FuncCategory::Scalar, FuncDomain::DbeLob, 5, Some(5), false),
    fop!("dbe_lob.loadblobfromfile", FuncCategory::Scalar, FuncDomain::DbeLob, 5, Some(5), false),
    fop!("dbe_lob.loadclobfrombfile", FuncCategory::Scalar, FuncDomain::DbeLob, 5, Some(5), false),
    fop!("dbe_lob.loadclobfromfile", FuncCategory::Scalar, FuncDomain::DbeLob, 5, Some(5), false),
    fop!("dbe_lob.loadfrombfile", FuncCategory::Scalar, FuncDomain::DbeLob, 3, Some(5), false),
    fop!("dbe_lob.loadfromfile", FuncCategory::Scalar, FuncDomain::DbeLob, 5, Some(5), false),
    fop!("dbe_lob.lob_append", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(2), false),
    fop!("dbe_lob.lob_converttoblob", FuncCategory::Scalar, FuncDomain::DbeLob, 5, Some(5), false),
    fop!("dbe_lob.lob_converttoclob", FuncCategory::Scalar, FuncDomain::DbeLob, 5, Some(5), false),
    fop!("dbe_lob.lob_copy", FuncCategory::Scalar, FuncDomain::DbeLob, 3, Some(5), false),
    fop!("dbe_lob.lob_erase", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(3), false),
    fop!("dbe_lob.lob_get_length", FuncCategory::Scalar, FuncDomain::DbeLob, 1, Some(1), false),
    fop!("dbe_lob.lob_read", FuncCategory::Scalar, FuncDomain::DbeLob, 3, Some(3), false),
    fop!("dbe_lob.lob_strip", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(2), false),
    fop!("dbe_lob.lob_substr", FuncCategory::Scalar, FuncDomain::DbeLob, 1, Some(3), false),
    fop!("dbe_lob.lob_write", FuncCategory::Scalar, FuncDomain::DbeLob, 4, Some(4), false),
    fop!("dbe_lob.lob_write_append", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(2), false),
    fop!("dbe_lob.match", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(4), false),
    fop!("dbe_lob.open", FuncCategory::Scalar, FuncDomain::DbeLob, 1, Some(2), false),
    fop!("dbe_lob.read", FuncCategory::Scalar, FuncDomain::DbeLob, 3, Some(3), false),
    fop!("dbe_lob.strip", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(2), false),
    fop!("dbe_lob.substr", FuncCategory::Scalar, FuncDomain::DbeLob, 1, Some(3), false),
    fop!("dbe_lob.trim", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(2), false),
    fop!("dbe_lob.write", FuncCategory::Scalar, FuncDomain::DbeLob, 3, Some(3), false),
    fop!("dbe_lob.write_append", FuncCategory::Scalar, FuncDomain::DbeLob, 2, Some(2), false),
    fop!("dbe_match.edit_distance_similarity", FuncCategory::Scalar, FuncDomain::DbeMatch, 2, Some(2), false),
    fop!("dbe_output.disable", FuncCategory::Scalar, FuncDomain::DbeOutput, 0, Some(0), false),
    fop!("dbe_output.enable", FuncCategory::Scalar, FuncDomain::DbeOutput, 0, Some(1), false),
    fop!("dbe_output.get_line", FuncCategory::Scalar, FuncDomain::DbeOutput, 2, Some(2), false),
    fop!("dbe_output.get_lines", FuncCategory::Scalar, FuncDomain::DbeOutput, 2, Some(2), false),
    fop!("dbe_output.new_line", FuncCategory::Scalar, FuncDomain::DbeOutput, 0, Some(0), false),
    fop!("dbe_output.print", FuncCategory::Scalar, FuncDomain::DbeOutput, 1, Some(1), false),
    fop!("dbe_output.print_line", FuncCategory::Scalar, FuncDomain::DbeOutput, 1, Some(1), false),
    fop!("dbe_output.put", FuncCategory::Scalar, FuncDomain::DbeOutput, 1, Some(1), false),
    fop!("dbe_output.put_line", FuncCategory::Scalar, FuncDomain::DbeOutput, 1, Some(1), false),
    fop!("dbe_output.set_buffer_size", FuncCategory::Scalar, FuncDomain::DbeOutput, 0, Some(1), false),
    fc!("dbe_random.get_value", FuncCategory::Scalar, FuncDomain::DbeRandom, 0, Some(2), false),
    fop!("dbe_raw.bit_and", FuncCategory::Scalar, FuncDomain::DbeRaw, 2, Some(2), false),
    fop!("dbe_raw.bit_complement", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(1), false),
    fop!("dbe_raw.bit_or", FuncCategory::Scalar, FuncDomain::DbeRaw, 2, Some(2), false),
    fop!("dbe_raw.bit_xor", FuncCategory::Scalar, FuncDomain::DbeRaw, 2, Some(2), false),
    fop!("dbe_raw.cast_from_binary_double_to_raw", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(2), false),
    fop!("dbe_raw.cast_from_binary_float_to_raw", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(2), false),
    fop!("dbe_raw.cast_from_binary_integer_to_raw", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(2), false),
    fop!("dbe_raw.cast_from_number_to_raw", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(1), false),
    fop!("dbe_raw.cast_from_raw_to_binary_double", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(2), false),
    fop!("dbe_raw.cast_from_raw_to_binary_float", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(2), false),
    fop!("dbe_raw.cast_from_raw_to_binary_integer", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(2), false),
    fop!("dbe_raw.cast_from_raw_to_number", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(1), false),
    fop!("dbe_raw.cast_from_raw_to_nvarchar2", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(1), false),
    fop!("dbe_raw.cast_from_varchar2_to_raw", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(1), false),
    fop!("dbe_raw.cast_to_varchar2", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(1), false),
    fop!("dbe_raw.compare", FuncCategory::Scalar, FuncDomain::DbeRaw, 2, Some(3), false),
    fop!("dbe_raw.concat", FuncCategory::Scalar, FuncDomain::DbeRaw, 0, Some(12), false),
    fop!("dbe_raw.convert", FuncCategory::Scalar, FuncDomain::DbeRaw, 3, Some(3), false),
    fop!("dbe_raw.copies", FuncCategory::Scalar, FuncDomain::DbeRaw, 2, Some(2), false),
    fop!("dbe_raw.get_length", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(1), false),
    fop!("dbe_raw.overlay", FuncCategory::Scalar, FuncDomain::DbeRaw, 2, Some(5), false),
    fop!("dbe_raw.reverse", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(1), false),
    fop!("dbe_raw.substr", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(3), false),
    fop!("dbe_raw.translate", FuncCategory::Scalar, FuncDomain::DbeRaw, 3, Some(3), false),
    fop!("dbe_raw.transliterate", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(4), false),
    fop!("dbe_raw.xrange", FuncCategory::Scalar, FuncDomain::DbeRaw, 1, Some(2), false),
    fop!("dbe_scheduler.create_credential", FuncCategory::Scalar, FuncDomain::DbeScheduler, 3, Some(3), false),
    fop!("dbe_scheduler.create_job", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, None, false),
    fop!("dbe_scheduler.create_job_class", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(3), false),
    fop!("dbe_scheduler.create_program", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, None, false),
    fop!("dbe_scheduler.create_schedule", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(3), false),
    fop!("dbe_scheduler.define_program_argument", FuncCategory::Scalar, FuncDomain::DbeScheduler, 3, Some(5), false),
    fop!("dbe_scheduler.disable", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(2), false),
    fop!("dbe_scheduler.disable_single", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(2), false),
    fop!("dbe_scheduler.drop_credential", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(1), false),
    fop!("dbe_scheduler.drop_job", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, None, false),
    fop!("dbe_scheduler.drop_job_class", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(1), false),
    fop!("dbe_scheduler.drop_program", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(1), false),
    fop!("dbe_scheduler.drop_schedule", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(1), false),
    fop!("dbe_scheduler.drop_single_job", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(3), false),
    fop!("dbe_scheduler.drop_single_job_class", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(2), false),
    fop!("dbe_scheduler.drop_single_program", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(2), false),
    fop!("dbe_scheduler.drop_single_schedule", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(2), false),
    fop!("dbe_scheduler.enable", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(2), false),
    fop!("dbe_scheduler.enable_single", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(1), false),
    fop!("dbe_scheduler.eval_calendar_string", FuncCategory::Scalar, FuncDomain::DbeScheduler, 2, Some(2), false),
    fop!("dbe_scheduler.evaluate_calendar_string", FuncCategory::Scalar, FuncDomain::DbeScheduler, 2, Some(2), false),
    fop!("dbe_scheduler.generate_job_name", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(1), false),
    fop!("dbe_scheduler.grant_user_authorization", FuncCategory::Scalar, FuncDomain::DbeScheduler, 2, Some(2), false),
    fop!("dbe_scheduler.revoke_user_authorization", FuncCategory::Scalar, FuncDomain::DbeScheduler, 2, Some(2), false),
    fop!("dbe_scheduler.run_backend_job", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(2), false),
    fop!("dbe_scheduler.run_foreground_job", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(2), false),
    fop!("dbe_scheduler.run_job", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(2), false),
    fop!("dbe_scheduler.set_attribute", FuncCategory::Scalar, FuncDomain::DbeScheduler, 3, Some(3), false),
    fop!("dbe_scheduler.set_job_argument_value", FuncCategory::Scalar, FuncDomain::DbeScheduler, 3, Some(3), false),
    fop!("dbe_scheduler.stop_job", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(2), false),
    fop!("dbe_scheduler.stop_single_job", FuncCategory::Scalar, FuncDomain::DbeScheduler, 1, Some(2), false),
    fop!("dbe_session.clear_context", FuncCategory::Scalar, FuncDomain::DbeSession, 2, Some(3), false),
    fop!("dbe_session.set_context", FuncCategory::Scalar, FuncDomain::DbeSession, 3, Some(3), false),
    fop!("dbe_sql.bind_variable", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(4), false),
    fop!("dbe_sql.close_cursor", FuncCategory::Scalar, FuncDomain::DbeSql, 1, Some(1), false),
    fop!("dbe_sql.column_value", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.dbe_sql_get_result_char", FuncCategory::Scalar, FuncDomain::DbeSql, 2, Some(2), false),
    fop!("dbe_sql.dbe_sql_get_result_long", FuncCategory::Scalar, FuncDomain::DbeSql, 2, Some(2), false),
    fop!("dbe_sql.dbe_sql_get_result_raw", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.describe_columns", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.execute", FuncCategory::Scalar, FuncDomain::DbeSql, 1, Some(2), false),
    fop!("dbe_sql.fetch_rows", FuncCategory::Scalar, FuncDomain::DbeSql, 1, Some(1), false),
    fop!("dbe_sql.get_array_result_char", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.get_array_result_int", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.get_array_result_raw", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.get_array_result_text", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.get_result", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.get_result_bytea", FuncCategory::Scalar, FuncDomain::DbeSql, 2, Some(2), false),
    fop!("dbe_sql.get_result_char", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(5), false),
    fop!("dbe_sql.get_result_int", FuncCategory::Scalar, FuncDomain::DbeSql, 2, Some(2), false),
    fop!("dbe_sql.get_result_long", FuncCategory::Scalar, FuncDomain::DbeSql, 6, Some(6), false),
    fop!("dbe_sql.get_result_raw", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(5), false),
    fop!("dbe_sql.get_result_text", FuncCategory::Scalar, FuncDomain::DbeSql, 2, Some(2), false),
    fop!("dbe_sql.get_result_unknown", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.get_results", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.get_results_bytea", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.get_results_char", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.get_results_int", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.get_results_raw", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.get_results_text", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.get_variable_result", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.get_variable_result_char", FuncCategory::Scalar, FuncDomain::DbeSql, 2, Some(2), false),
    fop!("dbe_sql.get_variable_result_int", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.get_variable_result_raw", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.get_variable_result_text", FuncCategory::Scalar, FuncDomain::DbeSql, 2, Some(2), false),
    fop!("dbe_sql.is_active", FuncCategory::Scalar, FuncDomain::DbeSql, 1, Some(1), false),
    fop!("dbe_sql.last_row_count", FuncCategory::Scalar, FuncDomain::DbeSql, 0, Some(0), false),
    fop!("dbe_sql.next_row", FuncCategory::Scalar, FuncDomain::DbeSql, 1, Some(1), false),
    fop!("dbe_sql.open_cursor", FuncCategory::Scalar, FuncDomain::DbeSql, 0, Some(0), false),
    fop!("dbe_sql.register_context", FuncCategory::Scalar, FuncDomain::DbeSql, 0, Some(0), false),
    fop!("dbe_sql.register_variable", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.run_and_next", FuncCategory::Scalar, FuncDomain::DbeSql, 1, Some(1), false),
    fop!("dbe_sql.set_result_type", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(4), false),
    fop!("dbe_sql.set_result_type_bytea", FuncCategory::Scalar, FuncDomain::DbeSql, 4, Some(4), false),
    fop!("dbe_sql.set_result_type_byteas", FuncCategory::Scalar, FuncDomain::DbeSql, 6, Some(6), false),
    fop!("dbe_sql.set_result_type_char", FuncCategory::Scalar, FuncDomain::DbeSql, 4, Some(4), false),
    fop!("dbe_sql.set_result_type_chars", FuncCategory::Scalar, FuncDomain::DbeSql, 6, Some(6), false),
    fop!("dbe_sql.set_result_type_int", FuncCategory::Scalar, FuncDomain::DbeSql, 2, Some(2), false),
    fop!("dbe_sql.set_result_type_ints", FuncCategory::Scalar, FuncDomain::DbeSql, 5, Some(5), false),
    fop!("dbe_sql.set_result_type_long", FuncCategory::Scalar, FuncDomain::DbeSql, 2, Some(2), false),
    fop!("dbe_sql.set_result_type_raw", FuncCategory::Scalar, FuncDomain::DbeSql, 4, Some(4), false),
    fop!("dbe_sql.set_result_type_raws", FuncCategory::Scalar, FuncDomain::DbeSql, 6, Some(6), false),
    fop!("dbe_sql.set_result_type_text", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.set_result_type_texts", FuncCategory::Scalar, FuncDomain::DbeSql, 6, Some(6), false),
    fop!("dbe_sql.set_result_type_unknown", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.set_results_type", FuncCategory::Scalar, FuncDomain::DbeSql, 5, Some(6), false),
    fop!("dbe_sql.sql_bind_array", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(5), false),
    fop!("dbe_sql.sql_bind_variable", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(4), false),
    fop!("dbe_sql.sql_describe_columns", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.sql_get_tableof_values_c", FuncCategory::Scalar, FuncDomain::DbeSql, 4, Some(4), false),
    fop!("dbe_sql.sql_get_values_c", FuncCategory::Scalar, FuncDomain::DbeSql, 4, Some(4), false),
    fop!("dbe_sql.sql_run", FuncCategory::Scalar, FuncDomain::DbeSql, 1, Some(1), false),
    fop!("dbe_sql.sql_set_results_type_c", FuncCategory::Scalar, FuncDomain::DbeSql, 7, Some(7), false),
    fop!("dbe_sql.sql_set_sql", FuncCategory::Scalar, FuncDomain::DbeSql, 3, Some(3), false),
    fop!("dbe_sql.sql_set_tableof_results_type_c", FuncCategory::Scalar, FuncDomain::DbeSql, 7, Some(7), false),
    fop!("dbe_sql.sql_unregister_context", FuncCategory::Scalar, FuncDomain::DbeSql, 1, Some(1), false),
    fop!("dbe_sql_util.create_abort_sql_patch", FuncCategory::Scalar, FuncDomain::DbeSqlUtil, 1, Some(3), false),
    fop!("dbe_sql_util.create_hint_sql_patch", FuncCategory::Scalar, FuncDomain::DbeSqlUtil, 1, Some(3), false),
    fop!("dbe_sql_util.disable_sql_patch", FuncCategory::Scalar, FuncDomain::DbeSqlUtil, 1, Some(1), false),
    fop!("dbe_sql_util.drop_sql_patch", FuncCategory::Scalar, FuncDomain::DbeSqlUtil, 1, Some(1), false),
    fop!("dbe_sql_util.enable_sql_patch", FuncCategory::Scalar, FuncDomain::DbeSqlUtil, 1, Some(1), false),
    fop!("dbe_sql_util.show_sql_patch", FuncCategory::Scalar, FuncDomain::DbeSqlUtil, 0, Some(1), false),
    fop!("dbe_stats.get_stats_history_availability", FuncCategory::Scalar, FuncDomain::DbeStats, 0, Some(0), false),
    fop!("dbe_stats.get_stats_history_retention", FuncCategory::Scalar, FuncDomain::DbeStats, 0, Some(0), false),
    fop!("dbe_stats.lock_column_stats", FuncCategory::Scalar, FuncDomain::DbeStats, 3, Some(3), false),
    fop!("dbe_stats.lock_partition_stats", FuncCategory::Scalar, FuncDomain::DbeStats, 3, Some(3), false),
    fop!("dbe_stats.lock_schema_stats", FuncCategory::Scalar, FuncDomain::DbeStats, 1, Some(1), false),
    fop!("dbe_stats.lock_table_stats", FuncCategory::Scalar, FuncDomain::DbeStats, 1, Some(1), false),
    fop!("dbe_stats.purge_stats", FuncCategory::Scalar, FuncDomain::DbeStats, 1, Some(1), false),
    fop!("dbe_stats.restore_column_stats", FuncCategory::Scalar, FuncDomain::DbeStats, 6, Some(6), false),
    fop!("dbe_stats.restore_partition_stats", FuncCategory::Scalar, FuncDomain::DbeStats, 6, Some(6), false),
    fop!("dbe_stats.restore_schema_stats", FuncCategory::Scalar, FuncDomain::DbeStats, 4, Some(4), false),
    fop!("dbe_stats.restore_table_stats", FuncCategory::Scalar, FuncDomain::DbeStats, 5, Some(5), false),
    fop!("dbe_stats.unlock_column_stats", FuncCategory::Scalar, FuncDomain::DbeStats, 3, Some(3), false),
    fop!("dbe_stats.unlock_partition_stats", FuncCategory::Scalar, FuncDomain::DbeStats, 3, Some(3), false),
    fop!("dbe_stats.unlock_schema_stats", FuncCategory::Scalar, FuncDomain::DbeStats, 1, Some(1), false),
    fop!("dbe_stats.unlock_table_stats", FuncCategory::Scalar, FuncDomain::DbeStats, 1, Some(1), false),
    fop!("dbe_task.cancel", FuncCategory::Scalar, FuncDomain::DbeTask, 1, Some(1), false),
    fop!("dbe_task.change", FuncCategory::Scalar, FuncDomain::DbeTask, 1, Some(4), false),
    fop!("dbe_task.content", FuncCategory::Scalar, FuncDomain::DbeTask, 2, Some(2), false),
    fop!("dbe_task.finish", FuncCategory::Scalar, FuncDomain::DbeTask, 2, Some(3), false),
    fop!("dbe_task.id_submit", FuncCategory::Scalar, FuncDomain::DbeTask, 2, Some(4), false),
    fop!("dbe_task.interval", FuncCategory::Scalar, FuncDomain::DbeTask, 2, Some(2), false),
    fop!("dbe_task.job_submit", FuncCategory::Scalar, FuncDomain::DbeTask, 1, Some(3), false),
    fop!("dbe_task.next_time", FuncCategory::Scalar, FuncDomain::DbeTask, 2, Some(2), false),
    fop!("dbe_task.run", FuncCategory::Scalar, FuncDomain::DbeTask, 1, Some(2), false),
    fop!("dbe_task.submit", FuncCategory::Scalar, FuncDomain::DbeTask, 1, Some(4), false),
    fop!("dbe_task.update", FuncCategory::Scalar, FuncDomain::DbeTask, 4, Some(4), false),
    fop!("dbe_utility.canonicalize", FuncCategory::Scalar, FuncDomain::DbeUtility, 2, Some(3), false),
    fop!("dbe_utility.comma_to_table", FuncCategory::Scalar, FuncDomain::DbeUtility, 3, Some(3), false),
    fop!("dbe_utility.compile_schema", FuncCategory::Scalar, FuncDomain::DbeUtility, 1, Some(3), false),
    fop!("dbe_utility.db_version", FuncCategory::Scalar, FuncDomain::DbeUtility, 1, Some(1), false),
    fop!("dbe_utility.exec_ddl_statement", FuncCategory::Scalar, FuncDomain::DbeUtility, 1, Some(1), false),
    fop!("dbe_utility.expand_sql_text_proc", FuncCategory::Scalar, FuncDomain::DbeUtility, 2, Some(2), false),
    fop!("dbe_utility.format_call_stack", FuncCategory::Scalar, FuncDomain::DbeUtility, 0, Some(0), false),
    fop!("dbe_utility.format_error_backtrace", FuncCategory::Scalar, FuncDomain::DbeUtility, 0, Some(0), false),
    fop!("dbe_utility.format_error_stack", FuncCategory::Scalar, FuncDomain::DbeUtility, 0, Some(0), false),
    fop!("dbe_utility.get_cpu_time", FuncCategory::Scalar, FuncDomain::DbeUtility, 0, Some(0), false),
    fop!("dbe_utility.get_endianness", FuncCategory::Scalar, FuncDomain::DbeUtility, 0, Some(0), false),
    fop!("dbe_utility.get_hash_value", FuncCategory::Scalar, FuncDomain::DbeUtility, 3, Some(3), false),
    fop!("dbe_utility.get_sql_hash", FuncCategory::Scalar, FuncDomain::DbeUtility, 3, Some(3), false),
    fop!("dbe_utility.get_sql_hash_func", FuncCategory::Scalar, FuncDomain::DbeUtility, 3, Some(3), false),
    fop!("dbe_utility.get_time", FuncCategory::Scalar, FuncDomain::DbeUtility, 0, Some(0), false),
    fop!("dbe_utility.is_bit_set", FuncCategory::Scalar, FuncDomain::DbeUtility, 2, Some(2), false),
    fop!("dbe_utility.is_cluster_database", FuncCategory::Scalar, FuncDomain::DbeUtility, 0, Some(0), false),
    fop!("dbe_utility.name_resolve", FuncCategory::Scalar, FuncDomain::DbeUtility, 8, Some(8), false),
    fop!("dbe_utility.name_tokenize", FuncCategory::Scalar, FuncDomain::DbeUtility, 6, Some(6), false),
    fop!("dbe_utility.old_current_schema", FuncCategory::Scalar, FuncDomain::DbeUtility, 0, Some(0), false),
    fop!("dbe_utility.old_current_user", FuncCategory::Scalar, FuncDomain::DbeUtility, 0, Some(0), false),
    fop!("dbe_utility.table_to_comma", FuncCategory::Scalar, FuncDomain::DbeUtility, 3, Some(3), false),
    foc!("dbe_xmldom.appendchild", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 2, Some(2), false),
    foc!("dbe_xmldom.createelement", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 2, Some(3), false),
    foc!("dbe_xmldom.createtextnode", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 2, Some(2), false),
    foc!("dbe_xmldom.freedocument", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.freeelement", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.freenode", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.freenodelist", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.getattribute", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 2, Some(3), false),
    foc!("dbe_xmldom.getattributes", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.getchildnodes", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.getchildrenbytagname", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 2, Some(3), false),
    foc!("dbe_xmldom.getdocumentelement", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.getfirstchild", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.getlastchild", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.getlength", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.getlocalname", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(2), false),
    foc!("dbe_xmldom.getnameditem", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 2, Some(3), false),
    foc!("dbe_xmldom.getnextsibling", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.getnodename", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.getnodetype", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.getnodevalue", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.getparentnode", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.gettagname", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.haschildnodes", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.importnode", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 3, Some(3), false),
    foc!("dbe_xmldom.isnull", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.item", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 2, Some(2), false),
    foc!("dbe_xmldom.makeelement", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.makenode", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 1, Some(1), false),
    foc!("dbe_xmldom.newdomdocument", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 0, Some(1), false),
    foc!("dbe_xmldom.setattribute", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 2, Some(4), false),
    foc!("dbe_xmldom.setcharset", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 2, Some(2), false),
    foc!("dbe_xmldom.setdoctype", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 2, Some(2), false),
    foc!("dbe_xmldom.setnodevalue", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 2, Some(2), false),
    foc!("dbe_xmldom.writetobuffer", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 2, Some(2), false),
    foc!("dbe_xmldom.writetoclob", FuncCategory::Scalar, FuncDomain::DbeXmlDom, 2, Some(2), false),
    foc!("dbe_xmlparser.freeparser", FuncCategory::Scalar, FuncDomain::DbeXmlParser, 1, Some(1), false),
    foc!("dbe_xmlparser.getdocument", FuncCategory::Scalar, FuncDomain::DbeXmlParser, 1, Some(1), false),
    foc!("dbe_xmlparser.getvalidationmode", FuncCategory::Scalar, FuncDomain::DbeXmlParser, 1, Some(1), false),
    foc!("dbe_xmlparser.newparser", FuncCategory::Scalar, FuncDomain::DbeXmlParser, 0, Some(0), false),
    foc!("dbe_xmlparser.parsebuffer", FuncCategory::Scalar, FuncDomain::DbeXmlParser, 2, Some(2), false),
    foc!("dbe_xmlparser.parseclob", FuncCategory::Scalar, FuncDomain::DbeXmlParser, 2, Some(2), false),
    foc!("dbe_xmlparser.setvalidationmode", FuncCategory::Scalar, FuncDomain::DbeXmlParser, 2, Some(2), false),
    fop!("dbms_lob.append", FuncCategory::Scalar, FuncDomain::DbmsLob, 2, Some(2), false),
    fop!("dbms_lob.read", FuncCategory::Scalar, FuncDomain::DbmsLob, 3, Some(3), false),
    fop!("dbms_lob.substr", FuncCategory::Scalar, FuncDomain::DbmsLob, 1, Some(3), false),
    fop!("dbms_lob.write", FuncCategory::Scalar, FuncDomain::DbmsLob, 3, Some(3), false),
    fop!("dbms_output.disable", FuncCategory::Scalar, FuncDomain::DbmsOutput, 0, Some(0), false),
    fop!("dbms_output.enable", FuncCategory::Scalar, FuncDomain::DbmsOutput, 0, Some(1), false),
    fop!("dbms_output.put", FuncCategory::Scalar, FuncDomain::DbmsOutput, 1, Some(1), false),
    fop!("dbms_output.put_line", FuncCategory::Scalar, FuncDomain::DbmsOutput, 1, Some(1), false),
    fop!("dbms_scheduler.create_job", FuncCategory::Scalar, FuncDomain::DbmsScheduler, 1, None, false),
    fop!("dbms_scheduler.drop_job", FuncCategory::Scalar, FuncDomain::DbmsScheduler, 1, None, false),
    fop!("dbms_scheduler.run_job", FuncCategory::Scalar, FuncDomain::DbmsScheduler, 1, Some(2), false),
    fop!("dbms_sql.close_cursor", FuncCategory::Scalar, FuncDomain::DbmsSql, 1, Some(1), false),
    fop!("dbms_sql.column_value", FuncCategory::Scalar, FuncDomain::DbmsSql, 3, Some(3), false),
    fop!("dbms_sql.execute", FuncCategory::Scalar, FuncDomain::DbmsSql, 1, Some(2), false),
    fop!("dbms_sql.fetch_rows", FuncCategory::Scalar, FuncDomain::DbmsSql, 1, Some(1), false),
    fop!("dbms_sql.open_cursor", FuncCategory::Scalar, FuncDomain::DbmsSql, 0, Some(0), false),
    fop!("dbms_utility.format_error_backtrace", FuncCategory::Scalar, FuncDomain::DbmsUtility, 0, Some(0), false),
    fop!("dbms_utility.get_time", FuncCategory::Scalar, FuncDomain::DbmsUtility, 0, Some(0), false),
    fo!("decode", FuncCategory::Special, FuncDomain::OracleCompat, 2, None, false),
    f!("degrees", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("dense_rank", FuncCategory::Window, FuncDomain::Window, 0, Some(0), false),
    f!("diameter", FuncCategory::Scalar, FuncDomain::Geometric, 1, Some(1), false),
    f!("digest", FuncCategory::Scalar, FuncDomain::Crypto, 2, Some(2), false),
    f!("div", FuncCategory::Scalar, FuncDomain::Math, 2, Some(2), false),
    fo!("dump", FuncCategory::Scalar, FuncDomain::String, 1, Some(4), false),
    // ── E ───────────────────────────────────────────────────
    fc!("empty_blob", FuncCategory::Scalar, FuncDomain::Other, 0, Some(0), false),
    fc!("empty_clob", FuncCategory::Scalar, FuncDomain::Other, 0, Some(0), false),
    f!("encode", FuncCategory::Scalar, FuncDomain::String, 2, Some(2), false),
    f!("every", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    fo!("existsnode", FuncCategory::Scalar, FuncDomain::Xml, 2, Some(3), false),
    f!("exp", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("extract", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    fo!("extractvalue", FuncCategory::Scalar, FuncDomain::Xml, 2, Some(3), false),
    fo!("extractxml", FuncCategory::Scalar, FuncDomain::Xml, 2, Some(3), false),
    // ── F ───────────────────────────────────────────────────
    f!("factorial", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("family", FuncCategory::Scalar, FuncDomain::Network, 1, Some(1), false),
    f!("find_in_set", FuncCategory::Scalar, FuncDomain::String, 2, Some(2), false),
    f!("first_value", FuncCategory::Window, FuncDomain::Window, 1, Some(1), false),
    f!("floor", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("format", FuncCategory::Scalar, FuncDomain::String, 2, None, false),
    f!("format_type", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    // ── G ───────────────────────────────────────────────────
    fc!("from_days", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    fc!("from_unixtime", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(2), false),
    f!("gcd", FuncCategory::Scalar, FuncDomain::Math, 2, Some(2), false),
    f!("gen_random_uuid", FuncCategory::Scalar, FuncDomain::Crypto, 0, Some(0), false),
    f!("generate_series", FuncCategory::SetReturning, FuncDomain::Other, 2, Some(3), false),
    f!("generate_subscripts", FuncCategory::SetReturning, FuncDomain::Array, 2, Some(3), false),
    f!("get_bit", FuncCategory::Scalar, FuncDomain::String, 2, Some(2), false),
    f!("get_byte", FuncCategory::Scalar, FuncDomain::String, 2, Some(2), false),
    f!("get_current_ts_config", FuncCategory::Scalar, FuncDomain::TextSearch, 0, Some(0), false),
    fo!("getblobval", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(1), false),
    fo!("getclobval", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(1), false),
    fo!("getnamespace", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(1), false),
    fo!("getnumberval", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(1), false),
    fo!("getrootelement", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(1), false),
    fo!("getstringval", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(1), false),
    f!("greatest", FuncCategory::Special, FuncDomain::Other, 2, None, false),
    fo!("group_concat", FuncCategory::Aggregate, FuncDomain::String, 1, None, true),
    f!("gs_decrypt", FuncCategory::Scalar, FuncDomain::Crypto, 3, Some(3), false),
    f!("gs_decrypt_aes128", FuncCategory::Scalar, FuncDomain::Crypto, 2, Some(2), false),
    f!("gs_encrypt", FuncCategory::Scalar, FuncDomain::Crypto, 3, Some(3), false),
    f!("gs_encrypt_aes128", FuncCategory::Scalar, FuncDomain::Crypto, 2, Some(2), false),
    // ── H ───────────────────────────────────────────────────
    f!("has_schema_privilege", FuncCategory::Scalar, FuncDomain::System, 2, Some(4), false),
    f!("has_table_privilege", FuncCategory::Scalar, FuncDomain::System, 2, Some(4), false),
    f!("hextoraw", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(1), false),
    f!("host", FuncCategory::Scalar, FuncDomain::Network, 1, Some(1), false),
    f!("hostmask", FuncCategory::Scalar, FuncDomain::Network, 1, Some(1), false),
    // ── I ───────────────────────────────────────────────────
    fc!("ifnull", FuncCategory::Special, FuncDomain::OracleCompat, 2, Some(2), false),
    f!("inet_client_addr", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("inet_client_port", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("inet_server_addr", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("inet_server_port", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("initcap", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    fo!("instr", FuncCategory::Scalar, FuncDomain::String, 2, Some(4), false),
    fo!("instrb", FuncCategory::Scalar, FuncDomain::String, 2, Some(4), false),
    fo!("intervaltonum", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(1), false),
    f!("isfinite", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    fo!("isfragment", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(1), false),
    // ── J ───────────────────────────────────────────────────
    f!("json", FuncCategory::TypeConstructor, FuncDomain::Json, 0, Some(1), false),
    f!("json_agg", FuncCategory::Aggregate, FuncDomain::Json, 1, Some(1), true),
    f!("json_append", FuncCategory::Scalar, FuncDomain::Json, 2, None, false),
    fc!("json_array", FuncCategory::Scalar, FuncDomain::Json, 0, None, false),
    f!("json_array_element", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("json_array_element_text", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("json_array_elements", FuncCategory::SetReturning, FuncDomain::Json, 1, Some(1), false),
    f!("json_array_length", FuncCategory::Scalar, FuncDomain::Json, 1, Some(1), false),
    f!("json_build_array", FuncCategory::Scalar, FuncDomain::Json, 0, None, false),
    f!("json_build_object", FuncCategory::Scalar, FuncDomain::Json, 0, None, false),
    fc!("json_contains", FuncCategory::Scalar, FuncDomain::Json, 2, Some(3), false),
    fc!("json_contains_path", FuncCategory::Scalar, FuncDomain::Json, 2, Some(3), false),
    fc!("json_depth", FuncCategory::Scalar, FuncDomain::Json, 1, Some(1), false),
    f!("json_each", FuncCategory::SetReturning, FuncDomain::Json, 1, Some(1), false),
    f!("json_each_text", FuncCategory::SetReturning, FuncDomain::Json, 1, Some(1), false),
    f!("json_extract_path", FuncCategory::Scalar, FuncDomain::Json, 1, None, false),
    f!("json_extract_path_text", FuncCategory::Scalar, FuncDomain::Json, 1, None, false),
    fc!("json_keys", FuncCategory::Scalar, FuncDomain::Json, 1, Some(2), false),
    fc!("json_length", FuncCategory::Scalar, FuncDomain::Json, 1, Some(2), false),
    fc!("json_merge", FuncCategory::Scalar, FuncDomain::Json, 2, None, false),
    f!("json_object", FuncCategory::Scalar, FuncDomain::Json, 1, None, true),
    f!("json_object_agg", FuncCategory::Aggregate, FuncDomain::Json, 2, Some(2), true),
    f!("json_object_field", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("json_object_field_text", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("json_object_keys", FuncCategory::SetReturning, FuncDomain::Json, 1, Some(1), false),
    fc!("json_quote", FuncCategory::Scalar, FuncDomain::Json, 1, Some(1), false),
    fc!("json_remove", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    fc!("json_replace", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    fc!("json_search", FuncCategory::Scalar, FuncDomain::Json, 2, Some(4), false),
    fc!("json_set", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    fc!("json_type", FuncCategory::Scalar, FuncDomain::Json, 1, Some(1), false),
    f!("json_typeof", FuncCategory::Scalar, FuncDomain::Json, 1, Some(1), false),
    fc!("json_unquote", FuncCategory::Scalar, FuncDomain::Json, 1, Some(1), false),
    fc!("json_valid", FuncCategory::Scalar, FuncDomain::Json, 1, Some(1), false),
    f!("jsonb_agg", FuncCategory::Aggregate, FuncDomain::Json, 1, Some(1), true),
    f!("jsonb_array_elements", FuncCategory::SetReturning, FuncDomain::Json, 1, Some(1), false),
    f!("jsonb_array_length", FuncCategory::Scalar, FuncDomain::Json, 1, Some(1), false),
    f!("jsonb_build_array", FuncCategory::Scalar, FuncDomain::Json, 0, None, true),
    f!("jsonb_build_object", FuncCategory::Scalar, FuncDomain::Json, 0, None, true),
    f!("jsonb_cmp", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("jsonb_contained", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("jsonb_contains", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("jsonb_each", FuncCategory::SetReturning, FuncDomain::Json, 1, Some(1), false),
    f!("jsonb_each_text", FuncCategory::SetReturning, FuncDomain::Json, 1, Some(1), false),
    f!("jsonb_eq", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("jsonb_exists", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("jsonb_exists_all", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("jsonb_exists_any", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("jsonb_ge", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("jsonb_gt", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("jsonb_hash", FuncCategory::Scalar, FuncDomain::Json, 1, Some(1), false),
    f!("jsonb_le", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("jsonb_lt", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("jsonb_ne", FuncCategory::Scalar, FuncDomain::Json, 2, Some(2), false),
    f!("jsonb_object_keys", FuncCategory::SetReturning, FuncDomain::Json, 1, Some(1), false),
    f!("jsonb_pretty", FuncCategory::Scalar, FuncDomain::Json, 1, Some(1), false),
    f!("jsonb_set", FuncCategory::Scalar, FuncDomain::Json, 3, Some(4), false),
    f!("jsonb_typeof", FuncCategory::Scalar, FuncDomain::Json, 1, Some(1), false),
    f!("justify_days", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    f!("justify_hours", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    f!("justify_interval", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    // ── L ───────────────────────────────────────────────────
    f!("lag", FuncCategory::Window, FuncDomain::Window, 1, Some(3), false),
    fo!("last_day", FuncCategory::Scalar, FuncDomain::OracleCompat, 1, Some(1), false),
    f!("last_insert_id", FuncCategory::Scalar, FuncDomain::System, 0, Some(1), false),
    f!("last_value", FuncCategory::Window, FuncDomain::Window, 1, Some(1), false),
    f!("lastval", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("lcm", FuncCategory::Scalar, FuncDomain::Math, 2, Some(2), false),
    f!("lead", FuncCategory::Window, FuncDomain::Window, 1, Some(3), false),
    f!("least", FuncCategory::Special, FuncDomain::Other, 2, None, false),
    f!("left", FuncCategory::Scalar, FuncDomain::String, 2, Some(2), false),
    f!("length", FuncCategory::Scalar, FuncDomain::String, 1, Some(2), false),
    f!("lengthb", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    fo!("listagg", FuncCategory::Aggregate, FuncDomain::String, 1, Some(2), true),
    f!("ln", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("localtime", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(1), false),
    f!("localtimestamp", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(1), false),
    f!("log", FuncCategory::Scalar, FuncDomain::Math, 1, Some(2), false),
    f!("log10", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("lower", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    f!("lpad", FuncCategory::Scalar, FuncDomain::String, 2, Some(3), false),
    f!("ltrim", FuncCategory::Scalar, FuncDomain::String, 1, Some(2), false),
    // ── M ───────────────────────────────────────────────────
    f!("make_date", FuncCategory::Scalar, FuncDomain::DateTime, 3, Some(3), false),
    f!("make_time", FuncCategory::Scalar, FuncDomain::DateTime, 3, Some(3), false),
    f!("make_timestamp", FuncCategory::Scalar, FuncDomain::DateTime, 6, Some(6), false),
    f!("make_timestamptz", FuncCategory::Scalar, FuncDomain::DateTime, 6, Some(7), false),
    fc!("makedate", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    fc!("maketime", FuncCategory::Scalar, FuncDomain::DateTime, 3, Some(3), false),
    f!("masklen", FuncCategory::Scalar, FuncDomain::Network, 1, Some(1), false),
    f!("max", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    f!("md5", FuncCategory::Scalar, FuncDomain::Crypto, 1, Some(2), false),
    f!("median", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    f!("min", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    f!("mod", FuncCategory::Scalar, FuncDomain::Math, 2, Some(2), false),
    f!("mode", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    fc!("monthname", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    fo!("months_between", FuncCategory::Scalar, FuncDomain::OracleCompat, 2, Some(2), false),
    // ── N ───────────────────────────────────────────────────
    foc!("nanvl", FuncCategory::Scalar, FuncDomain::OracleCompat, 2, Some(2), false),
    fo!("nchr", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    f!("netmask", FuncCategory::Scalar, FuncDomain::Network, 1, Some(1), false),
    f!("network", FuncCategory::Scalar, FuncDomain::Network, 1, Some(1), false),
    fo!("new_time", FuncCategory::Scalar, FuncDomain::DateTime, 3, Some(3), false),
    fo!("next_day", FuncCategory::Scalar, FuncDomain::OracleCompat, 2, Some(2), false),
    f!("nextval", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    fo!("nls_initcap", FuncCategory::Scalar, FuncDomain::OracleCompat, 1, Some(2), false),
    fo!("nls_lower", FuncCategory::Scalar, FuncDomain::OracleCompat, 1, Some(2), false),
    fo!("nls_sort", FuncCategory::Scalar, FuncDomain::OracleCompat, 1, Some(2), false),
    fo!("nls_upper", FuncCategory::Scalar, FuncDomain::OracleCompat, 1, Some(2), false),
    fo!("nlssort", FuncCategory::Scalar, FuncDomain::OracleCompat, 1, Some(2), false),
    f!("now", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(0), false),
    f!("nth_value", FuncCategory::Window, FuncDomain::Window, 2, Some(2), false),
    f!("ntile", FuncCategory::Window, FuncDomain::Window, 1, Some(1), false),
    f!("nullif", FuncCategory::Special, FuncDomain::Other, 2, Some(2), false),
    f!("numrange", FuncCategory::Scalar, FuncDomain::Range, 2, Some(3), false),
    f!("numtoday", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(1), false),
    f!("numtodsinterval", FuncCategory::Scalar, FuncDomain::TypeConversion, 2, Some(2), false),
    foc!("numtoyminterval", FuncCategory::Scalar, FuncDomain::TypeConversion, 2, Some(2), false),
    fo!("nvl", FuncCategory::Special, FuncDomain::OracleCompat, 2, Some(2), false),
    fo!("nvl2", FuncCategory::Special, FuncDomain::OracleCompat, 3, Some(3), false),
    // ── O ───────────────────────────────────────────────────
    f!("octet_length", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    fc!("ora_hash", FuncCategory::Scalar, FuncDomain::Hash, 1, Some(2), false),
    f!("overlay", FuncCategory::Scalar, FuncDomain::String, 3, Some(4), false),
    // ── P ───────────────────────────────────────────────────
    f!("percent_rank", FuncCategory::Window, FuncDomain::Window, 0, Some(0), false),
    f!("percentile_cont", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(2), true),
    f!("percentile_disc", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(2), true),
    fc!("period_add", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    fc!("period_diff", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    f!("pg_advisory_lock", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    f!("pg_advisory_unlock", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    f!("pg_advisory_xact_lock", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    f!("pg_backend_pid", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("pg_cancel_backend", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_collation_is_visible", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    f!("pg_column_size", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_conf_load_time", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("pg_conversion_is_visible", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_create_logical_replication_slot", FuncCategory::Scalar, FuncDomain::System, 2, Some(3), false),
    f!("pg_create_physical_replication_slot", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    f!("pg_current_xlog_location", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("pg_database_size", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_describe_object", FuncCategory::Scalar, FuncDomain::System, 3, Some(3), false),
    f!("pg_drop_replication_slot", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_exception_context", FuncCategory::Scalar, FuncDomain::ExceptionContext, 0, Some(0), false),
    f!("pg_exception_detail", FuncCategory::Scalar, FuncDomain::ExceptionContext, 0, Some(0), false),
    f!("pg_exception_hint", FuncCategory::Scalar, FuncDomain::ExceptionContext, 0, Some(0), false),
    f!("pg_export_snapshot", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("pg_function_is_visible", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_get_constraintdef", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    f!("pg_get_expr", FuncCategory::Scalar, FuncDomain::System, 2, Some(3), false),
    f!("pg_get_functiondef", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_get_indexdef", FuncCategory::Scalar, FuncDomain::System, 1, Some(3), false),
    f!("pg_get_keywords", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("pg_get_ruledef", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    f!("pg_get_serial_sequence", FuncCategory::Scalar, FuncDomain::System, 2, Some(2), false),
    f!("pg_get_triggerdef", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_get_userbyid", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_get_viewdef", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    f!("pg_has_role", FuncCategory::Scalar, FuncDomain::System, 2, Some(3), false),
    f!("pg_identify_object", FuncCategory::Scalar, FuncDomain::System, 3, Some(3), false),
    f!("pg_indexes_size", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_is_in_recovery", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("pg_is_other_temp_schema", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_last_xact_replay_timestamp", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("pg_listening_channels", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("pg_logical_slot_get_binary_changes", FuncCategory::Scalar, FuncDomain::System, 2, None, false),
    f!("pg_logical_slot_get_changes", FuncCategory::Scalar, FuncDomain::System, 2, None, false),
    f!("pg_logical_slot_peek_binary_changes", FuncCategory::Scalar, FuncDomain::System, 2, None, false),
    f!("pg_logical_slot_peek_changes", FuncCategory::Scalar, FuncDomain::System, 2, None, false),
    f!("pg_ls_dir", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_my_temp_schema", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("pg_opclass_is_visible", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_operator_is_visible", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_opfamily_is_visible", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_postmaster_start_time", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("pg_prepared_statement", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_prepared_xact", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_query_audit", FuncCategory::Scalar, FuncDomain::System, 2, Some(2), false),
    f!("pg_read_binary_file", FuncCategory::Scalar, FuncDomain::System, 1, Some(3), false),
    f!("pg_read_file", FuncCategory::Scalar, FuncDomain::System, 1, Some(3), false),
    f!("pg_relation_filenode", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_relation_filepath", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_relation_size", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    f!("pg_replication_origin_create", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_replication_origin_drop", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_replication_origin_oid", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_replication_origin_progress", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    f!("pg_rotate_logfile", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("pg_size_pretty", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_sleep", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_start_backup", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    f!("pg_stat_file", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_stop_backup", FuncCategory::Scalar, FuncDomain::System, 0, Some(1), false),
    f!("pg_switch_xlog", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("pg_table_is_visible", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    f!("pg_table_size", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_tablespace_databases", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_tablespace_location", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_tablespace_size", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_terminate_backend", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_total_relation_size", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_try_advisory_lock", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    f!("pg_try_advisory_xact_lock", FuncCategory::Scalar, FuncDomain::System, 1, Some(2), false),
    f!("pg_ts_config_is_visible", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_ts_dict_is_visible", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_ts_parser_is_visible", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_ts_template_is_visible", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_type_is_visible", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_typeof", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_xlog_location_diff", FuncCategory::Scalar, FuncDomain::System, 2, Some(2), false),
    f!("pg_xlog_replay_pause", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("pg_xlog_replay_resume", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("pg_xlogfile_name", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pg_xlogfile_name_offset", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("pi", FuncCategory::Scalar, FuncDomain::Math, 0, Some(0), false),
    fop!("pkg_service.isubmit_on_nodes", FuncCategory::Scalar, FuncDomain::PkgService, 5, Some(6), false),
    fop!("pkg_service.job_cancel", FuncCategory::Scalar, FuncDomain::PkgService, 1, Some(1), false),
    fop!("pkg_service.job_finish", FuncCategory::Scalar, FuncDomain::PkgService, 2, Some(3), false),
    fop!("pkg_service.job_submit", FuncCategory::Scalar, FuncDomain::PkgService, 3, Some(5), false),
    fop!("pkg_service.job_update", FuncCategory::Scalar, FuncDomain::PkgService, 4, Some(4), false),
    fop!("pkg_service.sql_cancel", FuncCategory::Scalar, FuncDomain::PkgService, 1, Some(1), false),
    fop!("pkg_service.sql_clean_all_contexts", FuncCategory::Scalar, FuncDomain::PkgService, 0, Some(0), false),
    fop!("pkg_service.sql_get_array_result", FuncCategory::Scalar, FuncDomain::PkgService, 4, Some(4), false),
    fop!("pkg_service.sql_get_value", FuncCategory::Scalar, FuncDomain::PkgService, 3, Some(3), false),
    fop!("pkg_service.sql_get_variable_result", FuncCategory::Scalar, FuncDomain::PkgService, 3, Some(3), false),
    fop!("pkg_service.sql_is_context_active", FuncCategory::Scalar, FuncDomain::PkgService, 1, Some(1), false),
    fop!("pkg_service.sql_next_row", FuncCategory::Scalar, FuncDomain::PkgService, 1, Some(1), false),
    fop!("pkg_service.sql_register_context", FuncCategory::Scalar, FuncDomain::PkgService, 0, Some(0), false),
    fop!("pkg_service.sql_run", FuncCategory::Scalar, FuncDomain::PkgService, 1, Some(1), false),
    fop!("pkg_service.sql_set_result_type", FuncCategory::Scalar, FuncDomain::PkgService, 4, Some(4), false),
    fop!("pkg_service.sql_set_sql", FuncCategory::Scalar, FuncDomain::PkgService, 3, Some(3), false),
    fop!("pkg_service.sql_unregister_context", FuncCategory::Scalar, FuncDomain::PkgService, 1, Some(1), false),
    fop!("pkg_service.submit_on_nodes", FuncCategory::Scalar, FuncDomain::PkgService, 5, Some(6), false),
    foc!("pkg_util.app_read_action", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.app_read_client_info", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.app_read_module", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.app_set_action", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.app_set_client_info", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.app_set_module", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.bfile_close", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.bfile_get_length", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.bfile_open", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(2), false),
    foc!("pkg_util.blob_reset", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(4), false),
    foc!("pkg_util.clob_reset", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(3), false),
    foc!("pkg_util.exception_report_error", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(3), false),
    foc!("pkg_util.file_block_size", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.file_close_all", FuncCategory::Scalar, FuncDomain::PkgUtil, 0, Some(0), false),
    foc!("pkg_util.file_exists", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.file_getpos", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.file_is_close", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.file_newline", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.file_open", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(2), false),
    foc!("pkg_util.file_read", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(3), false),
    foc!("pkg_util.file_read_raw", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(2), false),
    foc!("pkg_util.file_readline", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(3), false),
    foc!("pkg_util.file_remove", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.file_rename", FuncCategory::Scalar, FuncDomain::PkgUtil, 4, Some(5), false),
    foc!("pkg_util.file_seek", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(2), false),
    foc!("pkg_util.file_set_dirname", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.file_set_max_line_size", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.file_size", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.file_write", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(2), false),
    foc!("pkg_util.file_write_raw", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(2), false),
    foc!("pkg_util.file_writeline", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(2), false),
    foc!("pkg_util.gs_compile_schema", FuncCategory::Scalar, FuncDomain::PkgUtil, 0, Some(3), false),
    foc!("pkg_util.io_print", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(2), false),
    foc!("pkg_util.loadblobfromfile", FuncCategory::Scalar, FuncDomain::PkgUtil, 5, Some(5), false),
    foc!("pkg_util.loadclobfromfile", FuncCategory::Scalar, FuncDomain::PkgUtil, 5, Some(5), false),
    foc!("pkg_util.lob_append", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(3), false),
    foc!("pkg_util.lob_append_huge", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(2), false),
    foc!("pkg_util.lob_compare", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(5), false),
    foc!("pkg_util.lob_converttoblob", FuncCategory::Scalar, FuncDomain::PkgUtil, 5, Some(5), false),
    foc!("pkg_util.lob_converttoblob_huge", FuncCategory::Scalar, FuncDomain::PkgUtil, 5, Some(5), false),
    foc!("pkg_util.lob_converttoclob", FuncCategory::Scalar, FuncDomain::PkgUtil, 5, Some(5), false),
    foc!("pkg_util.lob_converttoclob_huge", FuncCategory::Scalar, FuncDomain::PkgUtil, 5, Some(5), false),
    foc!("pkg_util.lob_copy_huge", FuncCategory::Scalar, FuncDomain::PkgUtil, 3, Some(5), false),
    foc!("pkg_util.lob_get_length", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.lob_match", FuncCategory::Scalar, FuncDomain::PkgUtil, 3, Some(4), false),
    foc!("pkg_util.lob_rawtotext", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.lob_read", FuncCategory::Scalar, FuncDomain::PkgUtil, 4, Some(4), false),
    foc!("pkg_util.lob_read_huge", FuncCategory::Scalar, FuncDomain::PkgUtil, 4, Some(4), false),
    foc!("pkg_util.lob_reset", FuncCategory::Scalar, FuncDomain::PkgUtil, 3, Some(4), false),
    foc!("pkg_util.lob_texttoraw", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.lob_write", FuncCategory::Scalar, FuncDomain::PkgUtil, 3, Some(3), false),
    foc!("pkg_util.lob_write_huge", FuncCategory::Scalar, FuncDomain::PkgUtil, 4, Some(4), false),
    foc!("pkg_util.lob_writeappend_huge", FuncCategory::Scalar, FuncDomain::PkgUtil, 3, Some(3), false),
    foc!("pkg_util.match_edit_distance_similarity", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(2), false),
    foc!("pkg_util.modify_package_state", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.random_get_value", FuncCategory::Scalar, FuncDomain::PkgUtil, 0, Some(0), false),
    foc!("pkg_util.random_set_seed", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.raw_cast_from_binary_integer", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(2), false),
    foc!("pkg_util.raw_cast_from_varchar2", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.raw_cast_to_binary_integer", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(2), false),
    foc!("pkg_util.raw_cast_to_varchar2", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.raw_get_length", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.read_bfile_to_blob", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(1), false),
    foc!("pkg_util.session_clear_context", FuncCategory::Scalar, FuncDomain::PkgUtil, 3, Some(3), false),
    foc!("pkg_util.session_search_context", FuncCategory::Scalar, FuncDomain::PkgUtil, 2, Some(2), false),
    foc!("pkg_util.session_set_context", FuncCategory::Scalar, FuncDomain::PkgUtil, 3, Some(3), false),
    foc!("pkg_util.utility_compile_schema", FuncCategory::Scalar, FuncDomain::PkgUtil, 1, Some(3), false),
    foc!("pkg_util.utility_format_call_stack", FuncCategory::Scalar, FuncDomain::PkgUtil, 0, Some(0), false),
    foc!("pkg_util.utility_format_error_backtrace", FuncCategory::Scalar, FuncDomain::PkgUtil, 0, Some(0), false),
    foc!("pkg_util.utility_format_error_stack", FuncCategory::Scalar, FuncDomain::PkgUtil, 0, Some(0), false),
    foc!("pkg_util.utility_get_time", FuncCategory::Scalar, FuncDomain::PkgUtil, 0, Some(0), false),
    f!("plainto_tsquery", FuncCategory::Scalar, FuncDomain::TextSearch, 1, Some(2), false),
    f!("point", FuncCategory::Scalar, FuncDomain::Geometric, 2, Some(2), false),
    f!("polygon", FuncCategory::Scalar, FuncDomain::Geometric, 1, Some(1), false),
    f!("position", FuncCategory::Scalar, FuncDomain::String, 2, Some(2), false),
    f!("power", FuncCategory::Scalar, FuncDomain::Math, 2, Some(2), false),
    // ── Q ───────────────────────────────────────────────────
    f!("query_to_xml", FuncCategory::Scalar, FuncDomain::Xml, 4, Some(4), false),
    f!("query_to_xml_and_xmlschema", FuncCategory::Scalar, FuncDomain::Xml, 4, Some(4), false),
    f!("query_to_xmlschema", FuncCategory::Scalar, FuncDomain::Xml, 4, Some(4), false),
    f!("querytree", FuncCategory::Scalar, FuncDomain::TextSearch, 1, Some(1), false),
    f!("quote_ident", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    f!("quote_literal", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    f!("quote_nullable", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    // ── R ───────────────────────────────────────────────────
    f!("radians", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("radius", FuncCategory::Scalar, FuncDomain::Geometric, 1, Some(1), false),
    fc!("rand", FuncCategory::Scalar, FuncDomain::Math, 0, Some(1), false),
    f!("random", FuncCategory::Scalar, FuncDomain::Math, 0, Some(0), false),
    f!("rank", FuncCategory::Window, FuncDomain::Window, 0, Some(0), false),
    f!("ratio_to_report", FuncCategory::Window, FuncDomain::Window, 1, Some(1), false),
    f!("rawout", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(1), false),
    f!("rawsend", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(1), false),
    f!("rawtohex", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(1), false),
    fo!("rawtohex2", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(1), false),
    f!("regexp_count", FuncCategory::Scalar, FuncDomain::String, 2, Some(4), false),
    f!("regexp_instr", FuncCategory::Scalar, FuncDomain::String, 2, Some(5), false),
    f!("regexp_like", FuncCategory::Scalar, FuncDomain::String, 2, Some(3), false),
    f!("regexp_matches", FuncCategory::Scalar, FuncDomain::String, 2, Some(3), false),
    f!("regexp_replace", FuncCategory::Scalar, FuncDomain::String, 2, Some(6), false),
    f!("regexp_split_to_array", FuncCategory::Scalar, FuncDomain::String, 1, Some(3), false),
    f!("regexp_split_to_table", FuncCategory::SetReturning, FuncDomain::String, 1, Some(3), false),
    f!("regexp_substr", FuncCategory::Scalar, FuncDomain::String, 2, Some(4), false),
    f!("regr_avgx", FuncCategory::Aggregate, FuncDomain::Aggregate, 2, Some(2), true),
    f!("regr_avgy", FuncCategory::Aggregate, FuncDomain::Aggregate, 2, Some(2), true),
    f!("regr_count", FuncCategory::Aggregate, FuncDomain::Aggregate, 2, Some(2), true),
    f!("regr_intercept", FuncCategory::Aggregate, FuncDomain::Aggregate, 2, Some(2), true),
    f!("regr_r2", FuncCategory::Aggregate, FuncDomain::Aggregate, 2, Some(2), true),
    f!("regr_slope", FuncCategory::Aggregate, FuncDomain::Aggregate, 2, Some(2), true),
    f!("regr_sxx", FuncCategory::Aggregate, FuncDomain::Aggregate, 2, Some(2), true),
    f!("regr_sxy", FuncCategory::Aggregate, FuncDomain::Aggregate, 2, Some(2), true),
    f!("regr_syy", FuncCategory::Aggregate, FuncDomain::Aggregate, 2, Some(2), true),
    foc!("remainder", FuncCategory::Scalar, FuncDomain::Math, 2, Some(2), false),
    f!("repeat", FuncCategory::Scalar, FuncDomain::String, 2, Some(2), false),
    f!("replace", FuncCategory::Scalar, FuncDomain::String, 2, Some(3), false),
    f!("reverse", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    f!("right", FuncCategory::Scalar, FuncDomain::String, 2, Some(2), false),
    f!("round", FuncCategory::Scalar, FuncDomain::Math, 1, Some(2), false),
    f!("row_number", FuncCategory::Window, FuncDomain::Window, 0, Some(0), false),
    f!("row_to_json", FuncCategory::Scalar, FuncDomain::Json, 1, Some(2), false),
    fo!("rownum", FuncCategory::Scalar, FuncDomain::OracleCompat, 0, Some(0), false),
    f!("rpad", FuncCategory::Scalar, FuncDomain::String, 2, Some(3), false),
    f!("rtrim", FuncCategory::Scalar, FuncDomain::String, 1, Some(2), false),
    // ── S ───────────────────────────────────────────────────
    f!("schema_to_xml", FuncCategory::Scalar, FuncDomain::Xml, 4, Some(4), false),
    f!("schema_to_xml_and_xmlschema", FuncCategory::Scalar, FuncDomain::Xml, 4, Some(4), false),
    f!("schema_to_xmlschema", FuncCategory::Scalar, FuncDomain::Xml, 4, Some(4), false),
    fc!("sec_to_time", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    f!("session_user", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("set_bit", FuncCategory::Scalar, FuncDomain::String, 3, Some(3), false),
    f!("set_byte", FuncCategory::Scalar, FuncDomain::String, 3, Some(3), false),
    f!("set_config", FuncCategory::Scalar, FuncDomain::System, 2, Some(3), false),
    f!("setseed", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("setval", FuncCategory::Scalar, FuncDomain::System, 2, Some(3), false),
    f!("sha1", FuncCategory::Scalar, FuncDomain::Crypto, 1, Some(1), false),
    f!("sha2", FuncCategory::Scalar, FuncDomain::Crypto, 1, Some(2), false),
    f!("sign", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("sin", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    f!("split_part", FuncCategory::Scalar, FuncDomain::String, 3, Some(3), false),
    f!("sqrt", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    fo!("st_buffer", FuncCategory::Scalar, FuncDomain::Geometric, 2, Some(3), false),
    fo!("st_envelope", FuncCategory::Scalar, FuncDomain::Geometric, 1, Some(1), false),
    fo!("st_makepoint", FuncCategory::Scalar, FuncDomain::Geometric, 2, Some(4), false),
    fo!("st_setsrid", FuncCategory::Scalar, FuncDomain::Geometric, 2, Some(2), false),
    f!("statement_timestamp", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(0), false),
    f!("stddev", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    f!("stddev_pop", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    f!("stddev_samp", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    fc!("str_to_date", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    f!("string_agg", FuncCategory::Aggregate, FuncDomain::String, 2, Some(2), true),
    f!("string_to_array", FuncCategory::Scalar, FuncDomain::Array, 2, Some(3), false),
    f!("strpos", FuncCategory::Scalar, FuncDomain::String, 2, Some(2), false),
    fc!("subdate", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    f!("substr", FuncCategory::Scalar, FuncDomain::String, 2, Some(3), false),
    fo!("substrb", FuncCategory::Scalar, FuncDomain::String, 2, Some(3), false),
    f!("substring", FuncCategory::Scalar, FuncDomain::String, 2, Some(3), false),
    fc!("substring_index", FuncCategory::Scalar, FuncDomain::String, 3, Some(3), false),
    fc!("subtime", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    f!("sum", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    foc!("sys_connect_by_path", FuncCategory::Scalar, FuncDomain::OracleCompat, 2, Some(2), false),
    foc!("sys_context", FuncCategory::Scalar, FuncDomain::System, 2, Some(3), false),
    foc!("sys_extract_utc", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    fo!("sysdate", FuncCategory::Scalar, FuncDomain::OracleCompat, 0, Some(0), false),
    // ── T ───────────────────────────────────────────────────
    f!("table_to_xml", FuncCategory::Scalar, FuncDomain::Xml, 4, Some(4), false),
    f!("table_to_xml_and_xmlschema", FuncCategory::Scalar, FuncDomain::Xml, 4, Some(4), false),
    f!("table_to_xmlschema", FuncCategory::Scalar, FuncDomain::Xml, 4, Some(4), false),
    f!("tan", FuncCategory::Scalar, FuncDomain::Math, 1, Some(1), false),
    fc!("time_format", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    fc!("time_to_sec", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    fc!("timediff", FuncCategory::Scalar, FuncDomain::DateTime, 2, Some(2), false),
    f!("timenow", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(0), false),
    f!("timeofday", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(0), false),
    fc!("timestampadd", FuncCategory::Scalar, FuncDomain::DateTime, 3, Some(3), false),
    fc!("timestampdiff", FuncCategory::Scalar, FuncDomain::DateTime, 3, Some(3), false),
    f!("to_ascii", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(2), false),
    fo!("to_bigint", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(1), false),
    foc!("to_binary_double", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(2), false),
    fo!("to_binary_float", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(2), false),
    fo!("to_blob", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(1), false),
    f!("to_char", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(2), false),
    f!("to_clob", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(1), false),
    f!("to_date", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(2), false),
    fc!("to_days", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    fo!("to_dsinterval", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(1), false),
    f!("to_hex", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(1), false),
    f!("to_json", FuncCategory::Scalar, FuncDomain::Json, 1, Some(1), false),
    f!("to_jsonb", FuncCategory::Scalar, FuncDomain::Json, 1, Some(1), false),
    foc!("to_multi_byte", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    foc!("to_nchar", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(2), false),
    f!("to_number", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(2), false),
    fc!("to_seconds", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    foc!("to_single_byte", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    f!("to_timestamp", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(2), false),
    foc!("to_timestamp_tz", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(2), false),
    f!("to_tsquery", FuncCategory::Scalar, FuncDomain::TextSearch, 1, Some(2), false),
    f!("to_tsvector", FuncCategory::Scalar, FuncDomain::TextSearch, 1, Some(2), false),
    fo!("to_yminterval", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(1), false),
    f!("transaction_timestamp", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(0), false),
    f!("translate", FuncCategory::Scalar, FuncDomain::String, 3, Some(3), false),
    foc!("treat", FuncCategory::Scalar, FuncDomain::TypeConversion, 1, Some(1), false),
    f!("trim", FuncCategory::Scalar, FuncDomain::String, 1, Some(3), false),
    f!("trunc", FuncCategory::Scalar, FuncDomain::Math, 1, Some(2), false),
    f!("ts_headline", FuncCategory::Scalar, FuncDomain::TextSearch, 2, Some(4), false),
    f!("ts_lexize", FuncCategory::Scalar, FuncDomain::TextSearch, 2, Some(2), false),
    f!("ts_parse", FuncCategory::Scalar, FuncDomain::TextSearch, 1, Some(2), false),
    f!("ts_rank", FuncCategory::Scalar, FuncDomain::TextSearch, 1, Some(4), false),
    f!("ts_rank_cd", FuncCategory::Scalar, FuncDomain::TextSearch, 1, Some(4), false),
    f!("ts_rewrite", FuncCategory::Scalar, FuncDomain::TextSearch, 2, Some(3), false),
    f!("ts_stat", FuncCategory::Scalar, FuncDomain::TextSearch, 1, Some(2), false),
    f!("ts_token_type", FuncCategory::Scalar, FuncDomain::TextSearch, 0, Some(1), false),
    f!("tsrange", FuncCategory::Scalar, FuncDomain::Range, 2, Some(3), false),
    f!("tsvector_update_trigger", FuncCategory::Scalar, FuncDomain::TextSearch, 0, Some(0), false),
    f!("txid_current", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("txid_current_snapshot", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("txid_snapshot_xip", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("txid_snapshot_xmax", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("txid_snapshot_xmin", FuncCategory::Scalar, FuncDomain::System, 1, Some(1), false),
    f!("txid_visible_in_snapshot", FuncCategory::Scalar, FuncDomain::System, 2, Some(2), false),
    // ── U ───────────────────────────────────────────────────
    foc!("tz_offset", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    fo!("unistr", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    fc!("unix_timestamp", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(1), false),
    f!("unnest", FuncCategory::SetReturning, FuncDomain::Array, 1, Some(1), false),
    fc!("unnest_table", FuncCategory::SetReturning, FuncDomain::Array, 1, Some(1), false),
    f!("upper", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    f!("user", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    fc!("utc_date", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(0), false),
    fc!("utc_time", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(1), false),
    fc!("utc_timestamp", FuncCategory::Scalar, FuncDomain::DateTime, 0, Some(1), false),
    fop!("utl_file.fclose", FuncCategory::Scalar, FuncDomain::UtlFile, 1, Some(1), false),
    fop!("utl_file.fclose_all", FuncCategory::Scalar, FuncDomain::UtlFile, 0, Some(0), false),
    fop!("utl_file.fopen", FuncCategory::Scalar, FuncDomain::UtlFile, 2, Some(4), false),
    fop!("utl_file.get_line", FuncCategory::Scalar, FuncDomain::UtlFile, 1, Some(2), false),
    fop!("utl_file.put_line", FuncCategory::Scalar, FuncDomain::UtlFile, 1, Some(2), false),
    // ── V ───────────────────────────────────────────────────
    fc!("uuid_short", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    f!("var_pop", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    f!("var_samp", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    f!("variance", FuncCategory::Aggregate, FuncDomain::Aggregate, 1, Some(1), true),
    f!("version", FuncCategory::Scalar, FuncDomain::System, 0, Some(0), false),
    fo!("vsize", FuncCategory::Scalar, FuncDomain::String, 1, Some(1), false),
    // ── W ───────────────────────────────────────────────────
    fc!("weekday", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    fc!("weekofyear", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(1), false),
    f!("width", FuncCategory::Scalar, FuncDomain::Geometric, 1, Some(1), false),
    f!("width_bucket", FuncCategory::Scalar, FuncDomain::Math, 3, Some(4), false),
    fo!("wm_concat", FuncCategory::Aggregate, FuncDomain::String, 1, None, true),
    // ── X ───────────────────────────────────────────────────
    f!("xml_is_well_formed", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(1), false),
    f!("xml_is_well_formed_content", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(2), false),
    f!("xml_is_well_formed_document", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(2), false),
    fop!("xmlagg", FuncCategory::Aggregate, FuncDomain::Xml, 1, Some(1), true),
    fop!("xmlattributes", FuncCategory::Scalar, FuncDomain::Xml, 1, None, false),
    fop!("xmlcomment", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(1), false),
    fop!("xmlconcat", FuncCategory::Scalar, FuncDomain::Xml, 1, None, false),
    fop!("xmlelement", FuncCategory::Scalar, FuncDomain::Xml, 1, None, false),
    f!("xmlexists", FuncCategory::Scalar, FuncDomain::Xml, 2, Some(2), false),
    fop!("xmlforest", FuncCategory::Scalar, FuncDomain::Xml, 1, None, false),
    fop!("xmlparse", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(2), false),
    fop!("xmlpi", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(2), false),
    fop!("xmlquery", FuncCategory::Scalar, FuncDomain::Xml, 2, Some(3), false),
    fc!("xmlroot", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(3), false),
    fo!("xmlsequence", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(1), false),
    fop!("xmlserialize", FuncCategory::Scalar, FuncDomain::Xml, 2, Some(3), false),
    fop!("xmltype", FuncCategory::Scalar, FuncDomain::Xml, 1, Some(1), false),
    f!("xpath", FuncCategory::Scalar, FuncDomain::Xml, 2, Some(2), false),
    f!("xpath_exists", FuncCategory::Scalar, FuncDomain::Xml, 2, Some(2), false),
    fc!("yearweek", FuncCategory::Scalar, FuncDomain::DateTime, 1, Some(2), false),
];

/// Look up a built-in function by name (case-insensitive, binary search).
pub fn lookup_function(name: &str) -> Option<&'static FuncMeta> {
    let lower = name.to_ascii_lowercase();
    let idx = FUNCTIONS.partition_point(|m| m.name < lower.as_str());
    if idx < FUNCTIONS.len() && FUNCTIONS[idx].name == lower {
        Some(&FUNCTIONS[idx])
    } else {
        None
    }
}

pub fn lookup_builtin_meta(name: &str) -> Option<crate::ast::BuiltinFuncMeta> {
    lookup_function(name).map(|m| crate::ast::BuiltinFuncMeta {
        category: match m.category {
            FuncCategory::Aggregate => "Aggregate",
            FuncCategory::Window => "Window",
            FuncCategory::Scalar => "Scalar",
            FuncCategory::SetReturning => "SetReturning",
            FuncCategory::Special => "Special",
            FuncCategory::TypeConstructor => "TypeConstructor",
        }
        .to_string(),
        domain: match m.domain {
            FuncDomain::Math => "Math",
            FuncDomain::String => "String",
            FuncDomain::DateTime => "DateTime",
            FuncDomain::Aggregate => "Aggregate",
            FuncDomain::Window => "Window",
            FuncDomain::Array => "Array",
            FuncDomain::Json => "Json",
            FuncDomain::Network => "Network",
            FuncDomain::Geometric => "Geometric",
            FuncDomain::Hash => "Hash",
            FuncDomain::Range => "Range",
            FuncDomain::TextSearch => "TextSearch",
            FuncDomain::Crypto => "Crypto",
            FuncDomain::System => "System",
            FuncDomain::ExceptionContext => "ExceptionContext",
            FuncDomain::TypeConversion => "TypeConversion",
            FuncDomain::OracleCompat => "OracleCompat",
            // ── Oracle 兼容包函数域 ──
            FuncDomain::DbeApplicationInfo => "DbeApplicationInfo",
            FuncDomain::DbeFile => "DbeFile",
            FuncDomain::DbeLob => "DbeLob",
            FuncDomain::DbeMatch => "DbeMatch",
            FuncDomain::DbeOutput => "DbeOutput",
            FuncDomain::DbeRandom => "DbeRandom",
            FuncDomain::DbeRaw => "DbeRaw",
            FuncDomain::DbeScheduler => "DbeScheduler",
            FuncDomain::DbeSession => "DbeSession",
            FuncDomain::DbeSql => "DbeSql",
            FuncDomain::DbeSqlUtil => "DbeSqlUtil",
            FuncDomain::DbeStats => "DbeStats",
            FuncDomain::DbeTask => "DbeTask",
            FuncDomain::DbeUtility => "DbeUtility",
            FuncDomain::DbeXmlDom => "DbeXmlDom",
            FuncDomain::DbeXmlParser => "DbeXmlParser",
            FuncDomain::DbmsLob => "DbmsLob",
            FuncDomain::DbmsOutput => "DbmsOutput",
            FuncDomain::DbmsScheduler => "DbmsScheduler",
            FuncDomain::DbmsSql => "DbmsSql",
            FuncDomain::DbmsUtility => "DbmsUtility",
            FuncDomain::PkgService => "PkgService",
            FuncDomain::PkgUtil => "PkgUtil",
            FuncDomain::UtlFile => "UtlFile",
            FuncDomain::Xml => "Xml",
            // ── 其他 ──
            FuncDomain::Ai => "Ai",
            FuncDomain::Other => "Other",
        }
        .to_string(),
    })
}

/// System schemas where built-in functions actually reside (always trusted
/// for last-segment fallback). Derived from openGauss system catalog layout.
const SYSTEM_SCHEMAS: &[&str] = &["pg_catalog", "sys"];

static SYSTEM_PREFIXES: OnceLock<Vec<&'static str>> = OnceLock::new();

fn system_prefixes() -> &'static [&'static str] {
    SYSTEM_PREFIXES.get_or_init(|| {
        let mut v: Vec<&'static str> =
            FUNCTIONS.iter().filter_map(|m| m.name.split_once('.').map(|(p, _)| p)).collect();
        v.extend_from_slice(SYSTEM_SCHEMAS);
        v.sort_unstable();
        v.dedup();
        v
    })
}

fn is_known_system_prefix(prefix: &str) -> bool {
    system_prefixes().binary_search(&prefix).is_ok()
}

/// Two-phase lookup: exact full-qualified name, then fallback to last segment
/// only if the first segment is a known system package/schema (e.g. `pg_catalog`,
/// `dbe_output`, `dbms_lob`). User-defined schemas never trigger the fallback.
pub fn lookup_function_qualified(full_name: &str) -> Option<&'static FuncMeta> {
    let lower = full_name.to_ascii_lowercase();
    let idx = FUNCTIONS.partition_point(|m| m.name < lower.as_str());
    if idx < FUNCTIONS.len() && FUNCTIONS[idx].name == lower {
        return Some(&FUNCTIONS[idx]);
    }
    let last_seg = lower.split('.').next_back().unwrap_or(&lower);
    if last_seg.len() == lower.len() {
        return None;
    }
    let first_seg = lower.split('.').next().unwrap_or(&lower);
    if !is_known_system_prefix(first_seg) {
        return None;
    }
    let idx2 = FUNCTIONS.partition_point(|m| m.name < last_seg);
    if idx2 < FUNCTIONS.len() && FUNCTIONS[idx2].name == last_seg {
        return Some(&FUNCTIONS[idx2]);
    }
    None
}

pub fn lookup_builtin_meta_qualified(full_name: &str) -> Option<crate::ast::BuiltinFuncMeta> {
    lookup_function_qualified(full_name).map(|m| crate::ast::BuiltinFuncMeta {
        category: match m.category {
            FuncCategory::Aggregate => "Aggregate",
            FuncCategory::Window => "Window",
            FuncCategory::Scalar => "Scalar",
            FuncCategory::SetReturning => "SetReturning",
            FuncCategory::Special => "Special",
            FuncCategory::TypeConstructor => "TypeConstructor",
        }
        .to_string(),
        domain: match m.domain {
            FuncDomain::Math => "Math",
            FuncDomain::String => "String",
            FuncDomain::DateTime => "DateTime",
            FuncDomain::Aggregate => "Aggregate",
            FuncDomain::Window => "Window",
            FuncDomain::Array => "Array",
            FuncDomain::Json => "Json",
            FuncDomain::Network => "Network",
            FuncDomain::Geometric => "Geometric",
            FuncDomain::Hash => "Hash",
            FuncDomain::Range => "Range",
            FuncDomain::TextSearch => "TextSearch",
            FuncDomain::Crypto => "Crypto",
            FuncDomain::System => "System",
            FuncDomain::ExceptionContext => "ExceptionContext",
            FuncDomain::TypeConversion => "TypeConversion",
            FuncDomain::OracleCompat => "OracleCompat",
            FuncDomain::DbeApplicationInfo => "DbeApplicationInfo",
            FuncDomain::DbeFile => "DbeFile",
            FuncDomain::DbeLob => "DbeLob",
            FuncDomain::DbeMatch => "DbeMatch",
            FuncDomain::DbeOutput => "DbeOutput",
            FuncDomain::DbeRandom => "DbeRandom",
            FuncDomain::DbeRaw => "DbeRaw",
            FuncDomain::DbeScheduler => "DbeScheduler",
            FuncDomain::DbeSession => "DbeSession",
            FuncDomain::DbeSql => "DbeSql",
            FuncDomain::DbeSqlUtil => "DbeSqlUtil",
            FuncDomain::DbeStats => "DbeStats",
            FuncDomain::DbeTask => "DbeTask",
            FuncDomain::DbeUtility => "DbeUtility",
            FuncDomain::DbeXmlDom => "DbeXmlDom",
            FuncDomain::DbeXmlParser => "DbeXmlParser",
            FuncDomain::DbmsLob => "DbmsLob",
            FuncDomain::DbmsOutput => "DbmsOutput",
            FuncDomain::DbmsScheduler => "DbmsScheduler",
            FuncDomain::DbmsSql => "DbmsSql",
            FuncDomain::DbmsUtility => "DbmsUtility",
            FuncDomain::PkgService => "PkgService",
            FuncDomain::PkgUtil => "PkgUtil",
            FuncDomain::UtlFile => "UtlFile",
            FuncDomain::Xml => "Xml",
            FuncDomain::Ai => "Ai",
            FuncDomain::Other => "Other",
        }
        .to_string(),
    })
}

/// Resolve built-in function metadata from a [`ObjectName`], handling both
/// plain (`abs`) and dotted package names (`dbe_output.put_line`).
pub fn resolve_builtin_meta(name: &crate::ast::ObjectName) -> Option<crate::ast::BuiltinFuncMeta> {
    let full = name.iter().map(|s| s.to_lowercase()).collect::<Vec<_>>().join(".");
    lookup_builtin_meta_qualified(&full)
}

/// Validate a function call and return a list of warnings (if any).
pub fn validate_function_call(
    name: &str,
    arg_count: usize,
    has_distinct: bool,
    has_over: bool,
    has_variadic: bool,
    location: SourceLocation,
) -> Vec<ParserError> {
    let mut warnings = Vec::new();
    let Some(meta) = lookup_function(name) else {
        return warnings;
    };

    // VARIADIC expands at runtime — argument count validation is not meaningful
    if !has_variadic {
        match meta.max_args {
            Some(max) if meta.min_args == max && max == 0 => {
                if arg_count > 0 {
                    warnings.push(ParserError::Warning {
                        message: format!("function {} takes no arguments", meta.name),
                        location,
                    });
                }
            }
            Some(max) if meta.min_args == max => {
                if arg_count != meta.min_args as usize {
                    warnings.push(ParserError::Warning {
                        message: format!("function {} requires exactly {} argument(s)", meta.name, meta.min_args),
                        location,
                    });
                }
            }
            Some(max) => {
                if arg_count < meta.min_args as usize {
                    warnings.push(ParserError::Warning {
                        message: format!("function {} requires at least {} argument(s)", meta.name, meta.min_args),
                        location,
                    });
                }
                if arg_count > max as usize {
                    warnings.push(ParserError::Warning {
                        message: format!("function {} takes at most {} argument(s)", meta.name, max),
                        location,
                    });
                }
            }
            None => {
                // Variadic: min_args..∞
                if arg_count < meta.min_args as usize {
                    warnings.push(ParserError::Warning {
                        message: format!("function {} requires at least {} argument(s)", meta.name, meta.min_args),
                        location,
                    });
                }
            }
        }
    }

    if has_distinct && !meta.supports_distinct {
        warnings.push(ParserError::Warning {
            message: format!("DISTINCT is not supported for function {}", meta.name),
            location,
        });
    }

    // Window function missing OVER clause
    if meta.category == FuncCategory::Window && !has_over {
        warnings.push(ParserError::Warning {
            message: format!("window function {} should have OVER clause", meta.name),
            location,
        });
    }

    warnings
}

// ────────────────────────────────────────────────────────────────
// Tests
// ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn loc() -> SourceLocation {
        SourceLocation::default()
    }

    // ── Type correctness ──────────────────────────────────────

    #[test]
    fn test_compat_mode_bitops() {
        let combined = CompatMode::A_FORMAT | CompatMode::B_FORMAT;
        assert!(combined.contains(CompatMode::A_FORMAT));
        assert!(combined.contains(CompatMode::B_FORMAT));
        assert!(!combined.contains(CompatMode::PG_FORMAT));

        assert!(CompatMode::ALL.contains(CompatMode::A_FORMAT));
        assert!(CompatMode::ALL.contains(CompatMode::B_FORMAT));
        assert!(CompatMode::ALL.contains(CompatMode::PG_FORMAT));
    }

    #[test]
    fn test_compat_mode_single() {
        assert!(CompatMode::A_FORMAT.contains(CompatMode::A_FORMAT));
        assert!(!CompatMode::A_FORMAT.contains(CompatMode::B_FORMAT));
    }

    // ── Lookup: known functions ────────────────────────────────

    #[test]
    fn test_lookup_aggregate_count() {
        let meta = lookup_function("count").expect("count should be registered");
        assert_eq!(meta.category, FuncCategory::Aggregate);
        assert_eq!(meta.domain, FuncDomain::Aggregate);
        assert_eq!(meta.min_args, 1);
        assert_eq!(meta.max_args, Some(1));
        assert!(meta.supports_distinct);
    }

    #[test]
    fn test_lookup_aggregate_sum() {
        let meta = lookup_function("sum").expect("sum should be registered");
        assert_eq!(meta.category, FuncCategory::Aggregate);
        assert!(meta.supports_distinct);
    }

    #[test]
    fn test_lookup_aggregate_avg() {
        let meta = lookup_function("avg").expect("avg should be registered");
        assert_eq!(meta.category, FuncCategory::Aggregate);
        assert!(meta.supports_distinct);
    }

    #[test]
    fn test_lookup_aggregate_min_max() {
        let min_meta = lookup_function("min").expect("min should be registered");
        let max_meta = lookup_function("max").expect("max should be registered");
        assert_eq!(min_meta.category, FuncCategory::Aggregate);
        assert_eq!(max_meta.category, FuncCategory::Aggregate);
        assert!(min_meta.supports_distinct);
        assert!(max_meta.supports_distinct);
    }

    #[test]
    fn test_lookup_aggregate_string_agg() {
        let meta = lookup_function("string_agg").expect("string_agg should be registered");
        assert_eq!(meta.category, FuncCategory::Aggregate);
        assert_eq!(meta.domain, FuncDomain::String);
        assert!(meta.supports_distinct);
    }

    #[test]
    fn test_lookup_aggregate_listagg() {
        let meta = lookup_function("listagg").expect("listagg should be registered");
        assert_eq!(meta.category, FuncCategory::Aggregate);
        assert!(meta.supports_distinct);
    }

    // ── Lookup: window functions ───────────────────────────────

    #[test]
    fn test_lookup_window_row_number() {
        let meta = lookup_function("row_number").expect("row_number should be registered");
        assert_eq!(meta.category, FuncCategory::Window);
        assert_eq!(meta.domain, FuncDomain::Window);
        assert_eq!(meta.min_args, 0);
        assert!(!meta.supports_distinct);
    }

    #[test]
    fn test_lookup_window_rank() {
        let meta = lookup_function("rank").expect("rank should be registered");
        assert_eq!(meta.category, FuncCategory::Window);
    }

    #[test]
    fn test_lookup_window_dense_rank() {
        let meta = lookup_function("dense_rank").expect("dense_rank should be registered");
        assert_eq!(meta.category, FuncCategory::Window);
    }

    #[test]
    fn test_lookup_window_lag_lead() {
        let lag = lookup_function("lag").expect("lag should be registered");
        let lead = lookup_function("lead").expect("lead should be registered");
        assert_eq!(lag.category, FuncCategory::Window);
        assert_eq!(lead.category, FuncCategory::Window);
    }

    #[test]
    fn test_lookup_window_nth_value() {
        let meta = lookup_function("nth_value").expect("nth_value should be registered");
        assert_eq!(meta.category, FuncCategory::Window);
    }

    // ── Lookup: scalar string functions ────────────────────────

    #[test]
    fn test_lookup_scalar_upper_lower() {
        let upper = lookup_function("upper").expect("upper should be registered");
        let lower = lookup_function("lower").expect("lower should be registered");
        assert_eq!(upper.category, FuncCategory::Scalar);
        assert_eq!(lower.category, FuncCategory::Scalar);
        assert_eq!(upper.domain, FuncDomain::String);
        assert_eq!(lower.domain, FuncDomain::String);
        assert!(!upper.supports_distinct);
        assert!(!lower.supports_distinct);
    }

    #[test]
    fn test_lookup_scalar_substring() {
        // Both "substring" and "substr" should be registered
        let sub = lookup_function("substring").expect("substring should be registered");
        assert_eq!(sub.category, FuncCategory::Scalar);
        assert_eq!(sub.domain, FuncDomain::String);
    }

    #[test]
    fn test_lookup_scalar_substr() {
        let meta = lookup_function("substr").expect("substr should be registered");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::String);
    }

    #[test]
    fn test_lookup_scalar_replace() {
        let meta = lookup_function("replace").expect("replace should be registered");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::String);
        assert_eq!(meta.min_args, 2);
    }

    #[test]
    fn test_lookup_scalar_trim_ltrim_rtrim() {
        let trim = lookup_function("trim").expect("trim should be registered");
        let ltrim = lookup_function("ltrim").expect("ltrim should be registered");
        let rtrim = lookup_function("rtrim").expect("rtrim should be registered");
        assert_eq!(trim.category, FuncCategory::Scalar);
        assert_eq!(ltrim.category, FuncCategory::Scalar);
        assert_eq!(rtrim.category, FuncCategory::Scalar);
    }

    #[test]
    fn test_lookup_scalar_concat_concat_ws() {
        let concat = lookup_function("concat").expect("concat should be registered");
        let concat_ws = lookup_function("concat_ws").expect("concat_ws should be registered");
        assert_eq!(concat.category, FuncCategory::Scalar);
        assert_eq!(concat_ws.category, FuncCategory::Scalar);
        // concat is variadic
        assert!(concat.max_args.is_none());
        assert!(concat_ws.max_args.is_none());
        assert_eq!(concat.min_args, 2);
        assert_eq!(concat_ws.min_args, 2);
    }

    #[test]
    fn test_lookup_scalar_length() {
        let meta = lookup_function("length").expect("length should be registered");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::String);
    }

    // ── Lookup: scalar math functions ──────────────────────────

    #[test]
    fn test_lookup_scalar_abs() {
        let meta = lookup_function("abs").expect("abs should be registered");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::Math);
        assert_eq!(meta.min_args, 1);
        assert_eq!(meta.max_args, Some(1));
    }

    #[test]
    fn test_lookup_scalar_round_ceil_floor() {
        let round = lookup_function("round").expect("round should be registered");
        let ceil = lookup_function("ceil").expect("ceil should be registered");
        let floor = lookup_function("floor").expect("floor should be registered");
        assert_eq!(round.category, FuncCategory::Scalar);
        assert_eq!(round.domain, FuncDomain::Math);
        assert_eq!(ceil.category, FuncCategory::Scalar);
        assert_eq!(ceil.domain, FuncDomain::Math);
        assert_eq!(floor.category, FuncCategory::Scalar);
        assert_eq!(floor.domain, FuncDomain::Math);
    }

    #[test]
    fn test_lookup_scalar_mod() {
        let meta = lookup_function("mod").expect("mod should be registered");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::Math);
        assert_eq!(meta.min_args, 2);
    }

    #[test]
    fn test_lookup_scalar_pi() {
        let meta = lookup_function("pi").expect("pi should be registered");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::Math);
        assert_eq!(meta.min_args, 0);
    }

    // ── Lookup: scalar datetime functions ──────────────────────

    #[test]
    fn test_lookup_scalar_now() {
        let meta = lookup_function("now").expect("now should be registered");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::DateTime);
        assert_eq!(meta.min_args, 0);
    }

    #[test]
    fn test_lookup_scalar_date_trunc() {
        let meta = lookup_function("date_trunc").expect("date_trunc should be registered");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::DateTime);
        assert_eq!(meta.min_args, 2);
    }

    #[test]
    fn test_lookup_scalar_to_char() {
        let meta = lookup_function("to_char").expect("to_char should be registered");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::TypeConversion);
    }

    #[test]
    fn test_lookup_scalar_extract() {
        let meta = lookup_function("extract").expect("extract should be registered");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::DateTime);
    }

    // ── Lookup: set-returning functions ────────────────────────

    #[test]
    fn test_lookup_set_returning_generate_series() {
        let meta = lookup_function("generate_series").expect("generate_series should be registered");
        assert_eq!(meta.category, FuncCategory::SetReturning);
    }

    #[test]
    fn test_lookup_set_returning_unnest() {
        let meta = lookup_function("unnest").expect("unnest should be registered");
        assert_eq!(meta.category, FuncCategory::SetReturning);
        assert_eq!(meta.domain, FuncDomain::Array);
    }

    // ── Lookup: special functions ──────────────────────────────

    #[test]
    fn test_lookup_special_coalesce() {
        let meta = lookup_function("coalesce").expect("coalesce should be registered");
        assert_eq!(meta.category, FuncCategory::Special);
        assert!(meta.max_args.is_none());
        assert_eq!(meta.min_args, 2);
    }

    #[test]
    fn test_lookup_special_nullif() {
        let meta = lookup_function("nullif").expect("nullif should be registered");
        assert_eq!(meta.category, FuncCategory::Special);
        assert_eq!(meta.min_args, 2);
        assert_eq!(meta.max_args, Some(2));
    }

    #[test]
    fn test_lookup_special_greatest_least() {
        let greatest = lookup_function("greatest").expect("greatest should be registered");
        let least = lookup_function("least").expect("least should be registered");
        assert_eq!(greatest.category, FuncCategory::Special);
        assert_eq!(least.category, FuncCategory::Special);
        assert!(greatest.max_args.is_none());
        assert!(least.max_args.is_none());
    }

    // ── Lookup: Oracle-compatible functions ────────────────────

    #[test]
    fn test_lookup_oracle_compat_nvl() {
        let meta = lookup_function("nvl").expect("nvl should be registered");
        assert_eq!(meta.category, FuncCategory::Special);
        assert_eq!(meta.domain, FuncDomain::OracleCompat);
    }

    #[test]
    fn test_lookup_oracle_compat_nvl2() {
        let meta = lookup_function("nvl2").expect("nvl2 should be registered");
        assert_eq!(meta.category, FuncCategory::Special);
        assert_eq!(meta.domain, FuncDomain::OracleCompat);
    }

    #[test]
    fn test_lookup_oracle_compat_add_months() {
        let meta = lookup_function("add_months").expect("add_months should be registered");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::OracleCompat);
    }

    #[test]
    fn test_lookup_oracle_compat_last_day() {
        let meta = lookup_function("last_day").expect("last_day should be registered");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::OracleCompat);
    }

    #[test]
    fn test_lookup_oracle_compat_decode() {
        let meta = lookup_function("decode").expect("decode should be registered");
        assert_eq!(meta.category, FuncCategory::Special);
        assert!(meta.max_args.is_none());
        assert_eq!(meta.min_args, 2);
    }

    // ── Lookup: case insensitivity ─────────────────────────────

    #[test]
    fn test_lookup_case_insensitive() {
        assert!(lookup_function("COUNT").is_some());
        assert!(lookup_function("Count").is_some());
        assert!(lookup_function("UPPER").is_some());
        assert!(lookup_function("Upper").is_some());
    }

    // ── Lookup: unknown function returns None ──────────────────

    #[test]
    fn test_lookup_unknown_returns_none() {
        assert!(lookup_function("my_custom_func").is_none());
        assert!(lookup_function("xyz123").is_none());
        assert!(lookup_function("").is_none());
    }

    // ── Validation: arg count warnings ─────────────────────────

    #[test]
    fn test_validate_coalesce_too_few() {
        let w = validate_function_call("coalesce", 1, false, false, false, loc());
        assert!(w.iter().any(|e| e.to_string().contains("at least 2")));
    }

    #[test]
    fn test_validate_coalesce_ok() {
        assert!(validate_function_call("coalesce", 2, false, false, false, loc()).is_empty());
    }

    #[test]
    fn test_validate_count_no_args() {
        let w = validate_function_call("count", 0, false, false, false, loc());
        assert!(w.iter().any(|e| e.to_string().contains("exactly 1")));
    }

    #[test]
    fn test_validate_now_with_args() {
        let w = validate_function_call("now", 1, false, false, false, loc());
        assert!(w.iter().any(|e| e.to_string().contains("no arguments")));
    }

    // ── Validation: DISTINCT on non-aggregate ──────────────────

    #[test]
    fn test_validate_distinct_on_non_aggregate() {
        let w = validate_function_call("upper", 1, true, false, false, loc());
        assert!(w.iter().any(|e| e.to_string().contains("DISTINCT")));
    }

    #[test]
    fn test_validate_distinct_on_aggregate() {
        assert!(validate_function_call("count", 1, true, false, false, loc()).is_empty());
    }

    // ── Validation: window function missing OVER ───────────────

    #[test]
    fn test_validate_window_no_over() {
        let w = validate_function_call("row_number", 0, false, false, false, loc());
        assert!(w.iter().any(|e| e.to_string().contains("OVER clause")));
    }

    #[test]
    fn test_validate_window_with_over() {
        assert!(validate_function_call("row_number", 0, false, true, false, loc()).is_empty());
    }

    #[test]
    fn test_validate_lag_one_arg_ok() {
        assert!(validate_function_call("lag", 1, false, true, false, loc()).is_empty());
    }

    #[test]
    fn test_validate_lag_two_args_ok() {
        assert!(validate_function_call("lag", 2, false, true, false, loc()).is_empty());
    }

    #[test]
    fn test_validate_lag_three_args_ok() {
        assert!(validate_function_call("lag", 3, false, true, false, loc()).is_empty());
    }

    #[test]
    fn test_validate_lead_one_arg_ok() {
        assert!(validate_function_call("lead", 1, false, true, false, loc()).is_empty());
    }

    #[test]
    fn test_validate_lag_four_args_warns() {
        let w = validate_function_call("lag", 4, false, true, false, loc());
        assert!(w.iter().any(|e| e.to_string().contains("at most 3")));
    }

    // ── Validation: unknown function produces no warnings ──────

    #[test]
    fn test_validate_unknown_no_warnings() {
        assert!(validate_function_call("my_func", 5, true, false, false, loc()).is_empty());
    }

    // ── Validation: case insensitivity ─────────────────────────

    #[test]
    fn test_validate_case_insensitive() {
        // UPPER with DISTINCT should warn even if name is lowercase
        let w = validate_function_call("UPPER", 1, true, false, false, loc());
        assert!(w.iter().any(|e| e.to_string().contains("DISTINCT")));
    }

    // ── Validation: variadic functions ─────────────────────────

    #[test]
    fn test_validate_concat_many_args_ok() {
        // concat accepts 2+ args
        assert!(validate_function_call("concat", 5, false, false, false, loc()).is_empty());
    }

    #[test]
    fn test_validate_greatest_many_args_ok() {
        assert!(validate_function_call("greatest", 10, false, false, false, loc()).is_empty());
    }

    // ══════════════════════════════════════════════════════════════
    // Extension layer tests (FunctionRegistry)
    // ══════════════════════════════════════════════════════════════

    // ── Owned metadata type ────────────────────────────────────

    #[test]
    fn test_func_meta_owned_from_json() {
        let json = r#"{
            "name": "my_func",
            "category": "Scalar",
            "domain": "String",
            "min_args": 1,
            "max_args": 3,
            "supports_distinct": false,
            "compat": 7
        }"#;
        let meta: FuncMetaOwned = serde_json::from_str(json).unwrap();
        assert_eq!(meta.name, "my_func");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::String);
        assert_eq!(meta.min_args, 1);
        assert_eq!(meta.max_args, Some(3));
        assert!(!meta.supports_distinct);
        assert!(meta.compat.contains(CompatMode::A_FORMAT));
        assert!(meta.compat.contains(CompatMode::B_FORMAT));
        assert!(meta.compat.contains(CompatMode::PG_FORMAT));
    }

    #[test]
    fn test_func_meta_owned_variadic() {
        let json = r#"{
            "name": "multi_concat",
            "category": "Scalar",
            "domain": "String",
            "min_args": 2,
            "max_args": null,
            "supports_distinct": false,
            "compat": 7
        }"#;
        let meta: FuncMetaOwned = serde_json::from_str(json).unwrap();
        assert!(meta.max_args.is_none());
    }

    // ── FunctionRegistry: core lookup passthrough ──────────────

    #[test]
    fn test_registry_core_lookup() {
        let reg = FunctionRegistry::new();
        let meta = reg.lookup("count").expect("count should be found via core");
        assert_eq!(meta.category, FuncCategory::Aggregate);
    }

    #[test]
    fn test_registry_core_lookup_case_insensitive() {
        let reg = FunctionRegistry::new();
        assert!(reg.lookup("COUNT").is_some());
        assert!(reg.lookup("Upper").is_some());
    }

    #[test]
    fn test_registry_unknown_returns_none() {
        let reg = FunctionRegistry::new();
        assert!(reg.lookup("nonexistent_func_xyz").is_none());
    }

    // ── FunctionRegistry: extension loading ────────────────────

    #[test]
    fn test_registry_load_from_json_str() {
        let json = r#"[
            {
                "name": "custom_encrypt",
                "category": "Scalar",
                "domain": "Crypto",
                "min_args": 1,
                "max_args": 2,
                "supports_distinct": false,
                "compat": 7
            }
        ]"#;
        let reg = FunctionRegistry::new().with_extensions_from_json(json).unwrap();
        let meta = reg.lookup("custom_encrypt").expect("extension func should be found");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::Crypto);
        assert_eq!(meta.min_args, 1);
        assert_eq!(meta.max_args, Some(2));
    }

    #[test]
    fn test_registry_extension_overrides_core() {
        // Extension can override a core function (e.g., user-defined count)
        let json = r#"[
            {
                "name": "count",
                "category": "Scalar",
                "domain": "Other",
                "min_args": 0,
                "max_args": null,
                "supports_distinct": false,
                "compat": 7
            }
        ]"#;
        let reg = FunctionRegistry::new().with_extensions_from_json(json).unwrap();
        let meta = reg.lookup("count").expect("count should be found");
        // Extension overrides core
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::Other);
        assert!(meta.max_args.is_none());
    }

    #[test]
    fn test_registry_multiple_extensions() {
        let json = r#"[
            {
                "name": "func_a",
                "category": "Scalar",
                "domain": "Math",
                "min_args": 1,
                "max_args": 1,
                "supports_distinct": false,
                "compat": 1
            },
            {
                "name": "func_b",
                "category": "Aggregate",
                "domain": "Aggregate",
                "min_args": 1,
                "max_args": null,
                "supports_distinct": true,
                "compat": 7
            }
        ]"#;
        let reg = FunctionRegistry::new().with_extensions_from_json(json).unwrap();
        let a = reg.lookup("func_a").unwrap();
        assert_eq!(a.category, FuncCategory::Scalar);
        assert_eq!(a.domain, FuncDomain::Math);
        assert!(a.compat.contains(CompatMode::A_FORMAT));
        assert!(!a.compat.contains(CompatMode::B_FORMAT));

        let b = reg.lookup("func_b").unwrap();
        assert_eq!(b.category, FuncCategory::Aggregate);
        assert!(b.supports_distinct);
    }

    // ── FunctionRegistry: invalid JSON ─────────────────────────

    #[test]
    fn test_registry_invalid_json_returns_error() {
        let reg = FunctionRegistry::new().with_extensions_from_json("not json");
        assert!(reg.is_err());
    }

    #[test]
    fn test_registry_empty_json_array() {
        let reg = FunctionRegistry::new().with_extensions_from_json("[]").unwrap();
        // Core still works
        assert!(reg.lookup("count").is_some());
        assert!(reg.lookup("custom_nonexistent").is_none());
    }

    // ── FunctionRegistry: validation with extensions ───────────

    #[test]
    fn test_registry_validate_extension_func() {
        let json = r#"[
            {
                "name": "my_agg",
                "category": "Aggregate",
                "domain": "Aggregate",
                "min_args": 1,
                "max_args": 2,
                "supports_distinct": true,
                "compat": 7
            }
        ]"#;
        let reg = FunctionRegistry::new().with_extensions_from_json(json).unwrap();
        let w = reg.validate("my_agg", 5, false, false, false, loc());
        assert!(w.iter().any(|e| e.to_string().contains("at most 2")));
    }

    #[test]
    fn test_registry_validate_extension_window_no_over() {
        let json = r#"[
            {
                "name": "custom_window",
                "category": "Window",
                "domain": "Window",
                "min_args": 0,
                "max_args": 0,
                "supports_distinct": false,
                "compat": 7
            }
        ]"#;
        let reg = FunctionRegistry::new().with_extensions_from_json(json).unwrap();
        let w = reg.validate("custom_window", 0, false, false, false, loc());
        assert!(w.iter().any(|e| e.to_string().contains("OVER")));
    }

    #[test]
    fn test_registry_validate_core_func_via_registry() {
        let reg = FunctionRegistry::new();
        let w = reg.validate("upper", 1, true, false, false, loc());
        assert!(w.iter().any(|e| e.to_string().contains("DISTINCT")));
    }

    #[test]
    fn test_registry_validate_unknown_no_warnings() {
        let reg = FunctionRegistry::new();
        assert!(reg.validate("nonexistent_xyz", 5, true, true, false, loc()).is_empty());
    }

    // ── Oracle Package Function Domain Tests ──

    #[test]
    fn test_qualified_lookup_dbe_lob_append() {
        let meta = super::lookup_function_qualified("dbe_lob.append").unwrap();
        assert_eq!(meta.name, "dbe_lob.append");
        assert_eq!(meta.domain, FuncDomain::DbeLob);
        assert_eq!(meta.category, FuncCategory::Scalar);
    }

    #[test]
    fn test_qualified_lookup_dbms_lob_append() {
        let meta = super::lookup_function_qualified("dbms_lob.append").unwrap();
        assert_eq!(meta.domain, FuncDomain::DbmsLob);
    }

    #[test]
    fn test_qualified_lookup_distinguishes_dbe_vs_dbms_lob() {
        let dbe = super::lookup_function_qualified("dbe_lob.append").unwrap();
        let dbms = super::lookup_function_qualified("dbms_lob.append").unwrap();
        assert_ne!(dbe.domain, dbms.domain);
        assert_eq!(dbe.domain, FuncDomain::DbeLob);
        assert_eq!(dbms.domain, FuncDomain::DbmsLob);
    }

    #[test]
    fn test_qualified_lookup_dbe_output() {
        let meta = super::lookup_function_qualified("dbe_output.put_line").unwrap();
        assert_eq!(meta.domain, FuncDomain::DbeOutput);
    }

    #[test]
    fn test_qualified_lookup_dbms_output() {
        let meta = super::lookup_function_qualified("dbms_output.put_line").unwrap();
        assert_eq!(meta.domain, FuncDomain::DbmsOutput);
    }

    #[test]
    fn test_qualified_lookup_dbe_sql() {
        let meta = super::lookup_function_qualified("dbe_sql.execute").unwrap();
        assert_eq!(meta.domain, FuncDomain::DbeSql);
    }

    #[test]
    fn test_qualified_lookup_dbms_sql() {
        let meta = super::lookup_function_qualified("dbms_sql.execute").unwrap();
        assert_eq!(meta.domain, FuncDomain::DbmsSql);
    }

    #[test]
    fn test_qualified_lookup_dbe_file() {
        let meta = super::lookup_function_qualified("dbe_file.open").unwrap();
        assert_eq!(meta.domain, FuncDomain::DbeFile);
    }

    #[test]
    fn test_qualified_lookup_utl_file() {
        let meta = super::lookup_function_qualified("utl_file.fopen").unwrap();
        assert_eq!(meta.domain, FuncDomain::UtlFile);
    }

    #[test]
    fn test_qualified_lookup_dbe_scheduler() {
        let meta = super::lookup_function_qualified("dbe_scheduler.create_job").unwrap();
        assert_eq!(meta.domain, FuncDomain::DbeScheduler);
    }

    #[test]
    fn test_qualified_lookup_dbms_scheduler() {
        let meta = super::lookup_function_qualified("dbms_scheduler.create_job").unwrap();
        assert_eq!(meta.domain, FuncDomain::DbmsScheduler);
    }

    #[test]
    fn test_qualified_lookup_dbe_utility() {
        let meta = super::lookup_function_qualified("dbe_utility.get_time").unwrap();
        assert_eq!(meta.domain, FuncDomain::DbeUtility);
    }

    #[test]
    fn test_qualified_lookup_dbe_stats() {
        let meta = super::lookup_function_qualified("dbe_stats.lock_table_stats").unwrap();
        assert_eq!(meta.domain, FuncDomain::DbeStats);
    }

    #[test]
    fn test_qualified_lookup_dbe_session() {
        let meta = super::lookup_function_qualified("dbe_session.set_context").unwrap();
        assert_eq!(meta.domain, FuncDomain::DbeSession);
    }

    #[test]
    fn test_qualified_lookup_dbms_utility() {
        let meta = super::lookup_function_qualified("dbms_utility.get_time").unwrap();
        assert_eq!(meta.domain, FuncDomain::DbmsUtility);
    }

    #[test]
    fn test_qualified_lookup_pkg_service() {
        let meta = super::lookup_function_qualified("pkg_service.sql_cancel").unwrap();
        assert_eq!(meta.domain, FuncDomain::PkgService);
    }

    #[test]
    fn test_lookup_xml_xmlelement() {
        let meta = lookup_function("xmlelement").unwrap();
        assert_eq!(meta.domain, FuncDomain::Xml);
        assert_eq!(meta.category, FuncCategory::Scalar);
    }

    #[test]
    fn test_lookup_xml_xmlagg() {
        let meta = lookup_function("xmlagg").unwrap();
        assert_eq!(meta.domain, FuncDomain::Xml);
        assert_eq!(meta.category, FuncCategory::Aggregate);
        assert!(meta.supports_distinct);
    }

    #[test]
    fn test_builtin_meta_xml() {
        let meta = lookup_builtin_meta("xmlelement").unwrap();
        assert_eq!(meta.category, "Scalar");
        assert_eq!(meta.domain, "Xml");
    }

    #[test]
    fn test_builtin_meta_qualified_dbe_lob() {
        let meta = super::lookup_builtin_meta_qualified("dbe_lob.append").unwrap();
        assert_eq!(meta.category, "Scalar");
        assert_eq!(meta.domain, "DbeLob");
    }

    #[test]
    fn test_qualified_lookup_fallback_to_last_segment() {
        let meta = super::lookup_function_qualified("pg_catalog.upper");
        assert!(meta.is_some());
        let m = meta.unwrap();
        assert_eq!(m.domain, FuncDomain::String);
    }

    #[test]
    fn test_qualified_lookup_unknown_returns_none() {
        let meta = super::lookup_function_qualified("nonexistent_pkg.nonexistent_func");
        assert!(meta.is_none());
    }

    #[test]
    fn test_qualified_lookup_no_dot_returns_exact() {
        let meta = super::lookup_function_qualified("upper");
        assert!(meta.is_some());
    }

    #[test]
    fn test_functions_array_sorting_invariant() {
        for i in 1..FUNCTIONS.len() {
            assert!(
                FUNCTIONS[i - 1].name < FUNCTIONS[i].name,
                "FUNCTIONS array not sorted: {:?} >= {:?} at index {}",
                FUNCTIONS[i - 1].name,
                FUNCTIONS[i].name,
                i
            );
        }
    }

    // ── Exception context function tests ──

    #[test]
    fn test_lookup_exception_context_functions() {
        for name in &["pg_exception_context", "pg_exception_detail", "pg_exception_hint"] {
            let meta = lookup_function(name).unwrap_or_else(|| panic!("{} not found", name));
            assert_eq!(meta.domain, FuncDomain::ExceptionContext, "{} domain", name);
            assert_eq!(meta.category, FuncCategory::Scalar, "{} category", name);
            assert_eq!(meta.min_args, 0, "{} min_args", name);
            assert_eq!(meta.max_args, Some(0), "{} max_args", name);
        }
    }

    #[test]
    fn test_builtin_meta_exception_context() {
        let meta = lookup_builtin_meta("pg_exception_detail").unwrap();
        assert_eq!(meta.category, "Scalar");
        assert_eq!(meta.domain, "ExceptionContext");

        let meta = lookup_builtin_meta("pg_exception_hint").unwrap();
        assert_eq!(meta.category, "Scalar");
        assert_eq!(meta.domain, "ExceptionContext");

        let meta = lookup_builtin_meta("pg_exception_context").unwrap();
        assert_eq!(meta.category, "Scalar");
        assert_eq!(meta.domain, "ExceptionContext");
    }

    // ── JSON function tests ──

    #[test]
    fn test_lookup_json_type_constructor() {
        let meta = lookup_function("json").unwrap();
        assert_eq!(meta.category, FuncCategory::TypeConstructor);
        assert_eq!(meta.domain, FuncDomain::Json);
        assert_eq!(meta.min_args, 0);
        assert_eq!(meta.max_args, Some(1));
    }

    #[test]
    fn test_lookup_json_aggregate_functions() {
        let json_agg = lookup_function("json_agg").unwrap();
        assert_eq!(json_agg.category, FuncCategory::Aggregate);
        assert_eq!(json_agg.domain, FuncDomain::Json);
        assert!(json_agg.supports_distinct);

        let jsonb_agg = lookup_function("jsonb_agg").unwrap();
        assert_eq!(jsonb_agg.category, FuncCategory::Aggregate);
        assert_eq!(jsonb_agg.domain, FuncDomain::Json);
        assert!(jsonb_agg.supports_distinct);
    }

    #[test]
    fn test_lookup_json_conversion_functions() {
        for name in &["to_json", "to_jsonb", "array_to_json", "row_to_json"] {
            let meta = lookup_function(name).unwrap_or_else(|| panic!("{} not found", name));
            assert_eq!(meta.category, FuncCategory::Scalar, "{} category", name);
            assert_eq!(meta.domain, FuncDomain::Json, "{} domain", name);
        }
    }

    #[test]
    fn test_lookup_json_typeof_functions() {
        for name in &["json_typeof", "jsonb_typeof", "jsonb_pretty"] {
            let meta = lookup_function(name).unwrap_or_else(|| panic!("{} not found", name));
            assert_eq!(meta.category, FuncCategory::Scalar, "{} category", name);
            assert_eq!(meta.domain, FuncDomain::Json, "{} domain", name);
        }
    }

    #[test]
    fn test_builtin_meta_json_type_constructor() {
        let meta = lookup_builtin_meta("json").unwrap();
        assert_eq!(meta.category, "TypeConstructor");
        assert_eq!(meta.domain, "Json");
    }

    // ── P1 system function tests ──

    #[test]
    fn test_lookup_p1_system_functions() {
        let funcs = [
            ("col_description", 2, Some(3)),
            ("current_setting", 1, Some(2)),
            ("has_schema_privilege", 2, Some(4)),
            ("has_table_privilege", 2, Some(4)),
            ("pg_cancel_backend", 1, Some(1)),
            ("pg_database_size", 1, Some(1)),
            ("pg_get_userbyid", 1, Some(1)),
            ("pg_relation_size", 1, Some(2)),
            ("pg_table_is_visible", 1, Some(2)),
            ("pg_table_size", 1, Some(1)),
            ("pg_terminate_backend", 1, Some(1)),
            ("pg_total_relation_size", 1, Some(1)),
            ("pg_typeof", 1, Some(1)),
            ("set_config", 2, Some(3)),
        ];
        for (name, min, max) in &funcs {
            let meta = lookup_function(name).unwrap_or_else(|| panic!("{} not found", name));
            assert_eq!(meta.domain, FuncDomain::System, "{} domain", name);
            assert_eq!(meta.category, FuncCategory::Scalar, "{} category", name);
            assert_eq!(meta.min_args, *min, "{} min_args", name);
            assert_eq!(meta.max_args, *max, "{} max_args", name);
        }
    }

    #[test]
    fn test_builtin_meta_p1_system_functions() {
        for name in &["pg_typeof", "current_setting", "set_config", "pg_database_size"] {
            let meta = lookup_builtin_meta(name).unwrap();
            assert_eq!(meta.category, "Scalar", "{} category", name);
            assert_eq!(meta.domain, "System", "{} domain", name);
        }
    }

    // ── Distribution type tests ────────────────────────────────

    #[test]
    fn test_distribution_bitops() {
        let combined = Distribution::OPENGAUSS | Distribution::GAUSSDB;
        assert_eq!(combined, Distribution::BOTH);
        assert!(Distribution::BOTH.contains(Distribution::OPENGAUSS));
        assert!(Distribution::BOTH.contains(Distribution::GAUSSDB));
        assert!(!Distribution::OPENGAUSS.contains(Distribution::GAUSSDB));
        assert!(!Distribution::GAUSSDB.contains(Distribution::OPENGAUSS));
    }

    #[test]
    fn test_distribution_commercial_only() {
        assert!(Distribution::GAUSSDB.is_commercial_only());
        assert!(!Distribution::BOTH.is_commercial_only());
        assert!(!Distribution::OPENGAUSS.is_commercial_only());
    }

    #[test]
    fn test_distribution_default() {
        assert_eq!(Distribution::default(), Distribution::BOTH);
    }

    #[test]
    fn test_func_meta_owned_backward_compat_no_distribution() {
        // Old-format JSON without `distribution` field — must still deserialize
        let json = r#"{
            "name": "legacy_func",
            "category": "Scalar",
            "domain": "Math",
            "min_args": 1,
            "max_args": 1,
            "supports_distinct": false,
            "compat": 7
        }"#;
        let meta: FuncMetaOwned = serde_json::from_str(json).unwrap();
        assert_eq!(meta.name, "legacy_func");
        assert_eq!(meta.distribution, Distribution::BOTH);
    }

    #[test]
    fn test_func_meta_owned_with_distribution() {
        // New-format JSON with `distribution` field
        let json = r#"{
            "name": "commercial_func",
            "category": "Scalar",
            "domain": "Math",
            "min_args": 1,
            "max_args": 1,
            "supports_distinct": false,
            "compat": 7,
            "distribution": 2
        }"#;
        let meta: FuncMetaOwned = serde_json::from_str(json).unwrap();
        assert_eq!(meta.name, "commercial_func");
        assert_eq!(meta.distribution, Distribution::GAUSSDB);
        assert!(meta.distribution.is_commercial_only());
    }

    #[test]
    fn test_all_functions_have_valid_distribution() {
        for meta in FUNCTIONS {
            assert!(
                meta.distribution.contains(Distribution::OPENGAUSS)
                    || meta.distribution.contains(Distribution::GAUSSDB),
                "Function {} has invalid (zero) distribution",
                meta.name
            );
        }
    }

    #[test]
    fn test_commercial_only_functions_registered() {
        let commercial: Vec<&FuncMeta> = FUNCTIONS.iter().filter(|m| m.distribution.is_commercial_only()).collect();
        assert!(commercial.len() >= 2, "Expected at least 2 commercial-only functions, found {}", commercial.len());
        assert!(commercial.iter().any(|m| m.name == "dbe_random.get_value"));
        assert!(commercial.iter().any(|m| m.name == "dbe_xmldom.makenode"));
    }

    // ── Smoke test: fc! macro (commercial-only, ALL compat) ─────

    #[test]
    fn test_lookup_dbe_random_get_value() {
        let meta = lookup_function("dbe_random.get_value").expect("dbe_random.get_value should be registered");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::DbeRandom);
        assert_eq!(meta.min_args, 0); // both params have defaults
        assert_eq!(meta.max_args, Some(2));
        assert!(meta.distribution.is_commercial_only());
        assert!(!meta.distribution.contains(Distribution::OPENGAUSS));
        assert!(meta.compat.contains(CompatMode::A_FORMAT));
        assert!(meta.compat.contains(CompatMode::B_FORMAT));
        assert!(meta.compat.contains(CompatMode::PG_FORMAT));
    }

    #[test]
    fn test_qualified_lookup_dbe_random_get_value() {
        let meta = lookup_function_qualified("dbe_random.get_value").unwrap();
        assert_eq!(meta.domain, FuncDomain::DbeRandom);
        assert_eq!(meta.distribution, Distribution::GAUSSDB);
    }

    #[test]
    fn test_builtin_meta_dbe_random() {
        let meta = lookup_builtin_meta_qualified("dbe_random.get_value").unwrap();
        assert_eq!(meta.domain, "DbeRandom");
    }

    // ── Smoke test: foc! macro (commercial-only, ORACLE_COMPAT) ──

    #[test]
    fn test_lookup_dbe_xmldom_makenode() {
        let meta = lookup_function("dbe_xmldom.makenode").expect("dbe_xmldom.makenode should be registered");
        assert_eq!(meta.category, FuncCategory::Scalar);
        assert_eq!(meta.domain, FuncDomain::DbeXmlDom);
        assert_eq!(meta.min_args, 1);
        assert_eq!(meta.max_args, Some(1));
        assert!(meta.distribution.is_commercial_only());
        assert!(!meta.distribution.contains(Distribution::OPENGAUSS));
        assert!(meta.compat.contains(CompatMode::A_FORMAT));
        assert!(meta.compat.contains(CompatMode::PG_FORMAT));
        assert!(!meta.compat.contains(CompatMode::B_FORMAT));
    }

    #[test]
    fn test_qualified_lookup_dbe_xmldom_makenode() {
        let meta = lookup_function_qualified("dbe_xmldom.makenode").unwrap();
        assert_eq!(meta.domain, FuncDomain::DbeXmlDom);
        assert_eq!(meta.distribution, Distribution::GAUSSDB);
    }

    #[test]
    fn test_builtin_meta_dbe_xmldom() {
        let meta = lookup_builtin_meta_qualified("dbe_xmldom.makenode").unwrap();
        assert_eq!(meta.domain, "DbeXmlDom");
    }

    // ── Phase 2: batch registration guard tests ─────────────────

    #[test]
    fn test_phase2_dbe_output_completion() {
        for name in &["dbe_output.print_line", "dbe_output.set_buffer_size"] {
            let meta = lookup_function(name).unwrap_or_else(|| panic!("{} not found", name));
            assert_eq!(meta.domain, FuncDomain::DbeOutput);
        }
    }

    #[test]
    fn test_phase2_dbe_stats_completion() {
        for name in &[
            "dbe_stats.lock_partition_stats",
            "dbe_stats.lock_column_stats",
            "dbe_stats.lock_schema_stats",
            "dbe_stats.unlock_partition_stats",
            "dbe_stats.unlock_column_stats",
            "dbe_stats.unlock_schema_stats",
            "dbe_stats.restore_table_stats",
            "dbe_stats.restore_partition_stats",
            "dbe_stats.restore_column_stats",
            "dbe_stats.restore_schema_stats",
            "dbe_stats.purge_stats",
            "dbe_stats.get_stats_history_retention",
            "dbe_stats.get_stats_history_availability",
        ] {
            let meta = lookup_function(name).unwrap_or_else(|| panic!("{} not found", name));
            assert_eq!(meta.domain, FuncDomain::DbeStats);
        }
    }

    #[test]
    fn test_phase2_dbe_task_registered() {
        for name in &[
            "dbe_task.submit",
            "dbe_task.id_submit",
            "dbe_task.cancel",
            "dbe_task.run",
            "dbe_task.finish",
            "dbe_task.update",
            "dbe_task.change",
            "dbe_task.content",
            "dbe_task.next_time",
            "dbe_task.interval",
            "dbe_task.job_submit",
        ] {
            let meta = lookup_function(name).unwrap_or_else(|| panic!("{} not found", name));
            assert_eq!(meta.domain, FuncDomain::DbeTask);
        }
    }

    #[test]
    fn test_builtin_meta_dbe_task() {
        let meta = lookup_builtin_meta_qualified("dbe_task.submit").unwrap();
        assert_eq!(meta.domain, "DbeTask");
    }

    #[test]
    fn test_phase2_pkg_service_completion() {
        for name in &[
            "pkg_service.isubmit_on_nodes",
            "pkg_service.job_cancel",
            "pkg_service.job_finish",
            "pkg_service.job_submit",
            "pkg_service.job_update",
            "pkg_service.sql_clean_all_contexts",
            "pkg_service.sql_get_array_result",
            "pkg_service.sql_get_value",
            "pkg_service.sql_get_variable_result",
            "pkg_service.sql_is_context_active",
            "pkg_service.sql_next_row",
            "pkg_service.sql_register_context",
            "pkg_service.sql_run",
            "pkg_service.sql_set_result_type",
            "pkg_service.sql_set_sql",
            "pkg_service.sql_unregister_context",
            "pkg_service.submit_on_nodes",
        ] {
            let meta = lookup_function(name).unwrap_or_else(|| panic!("{} not found", name));
            assert_eq!(meta.domain, FuncDomain::PkgService);
        }
    }

    #[test]
    fn test_phase2_dbe_lob_completion() {
        for name in &[
            "dbe_lob.bfileclose",
            "dbe_lob.bfileopen",
            "dbe_lob.bfilename",
            "dbe_lob.close",
            "dbe_lob.converttoblob",
            "dbe_lob.converttoclob",
            "dbe_lob.fileclose",
            "dbe_lob.fileopen",
            "dbe_lob.getchunksize",
            "dbe_lob.loadblobfrombfile",
            "dbe_lob.loadblobfromfile",
            "dbe_lob.loadclobfrombfile",
            "dbe_lob.loadclobfromfile",
            "dbe_lob.loadfrombfile",
            "dbe_lob.loadfromfile",
            "dbe_lob.lob_append",
            "dbe_lob.lob_converttoblob",
            "dbe_lob.lob_converttoclob",
            "dbe_lob.lob_copy",
            "dbe_lob.lob_erase",
            "dbe_lob.lob_get_length",
            "dbe_lob.lob_read",
            "dbe_lob.lob_strip",
            "dbe_lob.lob_substr",
            "dbe_lob.lob_write",
            "dbe_lob.lob_write_append",
            "dbe_lob.match",
            "dbe_lob.open",
            "dbe_lob.strip",
            "dbe_lob.write_append",
        ] {
            let meta = lookup_function(name).unwrap_or_else(|| panic!("{} not found", name));
            assert_eq!(meta.domain, FuncDomain::DbeLob);
        }
    }

    #[test]
    fn test_phase2_dbe_lob_lob_converttoblob_signature() {
        let meta = lookup_function("dbe_lob.lob_converttoblob").unwrap();
        assert_eq!(meta.min_args, 5);
        assert_eq!(meta.max_args, Some(5));
    }

    #[test]
    fn test_phase2_dbe_lob_converttoblob_signature() {
        let meta = lookup_function("dbe_lob.converttoblob").unwrap();
        assert_eq!(meta.min_args, 2);
        assert_eq!(meta.max_args, Some(5));
    }

    // ── Phase 3: XML/DOM packages (commercial-only, O-compatible) ──

    #[test]
    fn test_phase3_dbe_xmldom_registered() {
        for name in &[
            "dbe_xmldom.appendchild",
            "dbe_xmldom.createelement",
            "dbe_xmldom.createtextnode",
            "dbe_xmldom.freedocument",
            "dbe_xmldom.freeelement",
            "dbe_xmldom.freenode",
            "dbe_xmldom.freenodelist",
            "dbe_xmldom.getattribute",
            "dbe_xmldom.getattributes",
            "dbe_xmldom.getchildnodes",
            "dbe_xmldom.getchildrenbytagname",
            "dbe_xmldom.getdocumentelement",
            "dbe_xmldom.getfirstchild",
            "dbe_xmldom.getlastchild",
            "dbe_xmldom.getlength",
            "dbe_xmldom.getlocalname",
            "dbe_xmldom.getnameditem",
            "dbe_xmldom.getnextsibling",
            "dbe_xmldom.getnodename",
            "dbe_xmldom.getnodetype",
            "dbe_xmldom.getnodevalue",
            "dbe_xmldom.getparentnode",
            "dbe_xmldom.gettagname",
            "dbe_xmldom.haschildnodes",
            "dbe_xmldom.importnode",
            "dbe_xmldom.isnull",
            "dbe_xmldom.item",
            "dbe_xmldom.makeelement",
            "dbe_xmldom.makenode",
            "dbe_xmldom.newdomdocument",
            "dbe_xmldom.setattribute",
            "dbe_xmldom.setcharset",
            "dbe_xmldom.setdoctype",
            "dbe_xmldom.setnodevalue",
            "dbe_xmldom.writetobuffer",
            "dbe_xmldom.writetoclob",
        ] {
            let meta = lookup_function(name).unwrap_or_else(|| panic!("{} not found", name));
            assert_eq!(meta.domain, FuncDomain::DbeXmlDom);
            assert!(meta.distribution.is_commercial_only());
            assert!(!meta.compat.contains(CompatMode::B_FORMAT));
        }
    }

    #[test]
    fn test_phase3_dbe_xmlparser_registered() {
        for name in &[
            "dbe_xmlparser.freeparser",
            "dbe_xmlparser.getdocument",
            "dbe_xmlparser.getvalidationmode",
            "dbe_xmlparser.newparser",
            "dbe_xmlparser.parsebuffer",
            "dbe_xmlparser.parseclob",
            "dbe_xmlparser.setvalidationmode",
        ] {
            let meta = lookup_function(name).unwrap_or_else(|| panic!("{} not found", name));
            assert_eq!(meta.domain, FuncDomain::DbeXmlParser);
            assert!(meta.distribution.is_commercial_only());
            assert!(!meta.compat.contains(CompatMode::B_FORMAT));
        }
    }

    #[test]
    fn test_builtin_meta_dbe_xmlparser() {
        let meta = lookup_builtin_meta_qualified("dbe_xmlparser.parsebuffer").unwrap();
        assert_eq!(meta.domain, "DbeXmlParser");
    }

    #[test]
    fn test_phase3_xmldom_newdomdocument_signature() {
        let meta = lookup_function("dbe_xmldom.newdomdocument").unwrap();
        assert_eq!(meta.min_args, 0);
        assert_eq!(meta.max_args, Some(1));
    }

    // ── Phase 4: RAW, APPLICATION_INFO, SCHEDULER, MATCH, SQL_UTIL ──

    #[test]
    fn test_phase4_dbe_application_info_registered() {
        for name in &[
            "dbe_application_info.read_client_info",
            "dbe_application_info.read_module",
            "dbe_application_info.set_action",
            "dbe_application_info.set_client_info",
            "dbe_application_info.set_module",
        ] {
            let meta = lookup_function(name).unwrap_or_else(|| panic!("{} not found", name));
            assert_eq!(meta.domain, FuncDomain::DbeApplicationInfo);
        }
    }

    #[test]
    fn test_phase4_dbe_match_registered() {
        let meta = lookup_function("dbe_match.edit_distance_similarity").unwrap();
        assert_eq!(meta.domain, FuncDomain::DbeMatch);
        assert_eq!(meta.min_args, 2);
        assert_eq!(meta.max_args, Some(2));
    }

    #[test]
    fn test_phase4_dbe_raw_registered() {
        for name in &[
            "dbe_raw.bit_and",
            "dbe_raw.bit_complement",
            "dbe_raw.bit_or",
            "dbe_raw.bit_xor",
            "dbe_raw.cast_from_binary_double_to_raw",
            "dbe_raw.cast_from_binary_float_to_raw",
            "dbe_raw.cast_from_binary_integer_to_raw",
            "dbe_raw.cast_from_number_to_raw",
            "dbe_raw.cast_from_raw_to_binary_double",
            "dbe_raw.cast_from_raw_to_binary_float",
            "dbe_raw.cast_from_raw_to_binary_integer",
            "dbe_raw.cast_from_raw_to_nvarchar2",
            "dbe_raw.cast_from_raw_to_number",
            "dbe_raw.cast_from_varchar2_to_raw",
            "dbe_raw.cast_to_varchar2",
            "dbe_raw.compare",
            "dbe_raw.concat",
            "dbe_raw.convert",
            "dbe_raw.copies",
            "dbe_raw.get_length",
            "dbe_raw.overlay",
            "dbe_raw.reverse",
            "dbe_raw.substr",
            "dbe_raw.translate",
            "dbe_raw.transliterate",
            "dbe_raw.xrange",
        ] {
            let meta = lookup_function(name).unwrap_or_else(|| panic!("{} not found", name));
            assert_eq!(meta.domain, FuncDomain::DbeRaw);
        }
    }

    #[test]
    fn test_phase4_dbe_scheduler_completion() {
        for name in &[
            "dbe_scheduler.create_credential",
            "dbe_scheduler.create_job_class",
            "dbe_scheduler.create_program",
            "dbe_scheduler.create_schedule",
            "dbe_scheduler.define_program_argument",
            "dbe_scheduler.disable",
            "dbe_scheduler.drop_credential",
            "dbe_scheduler.drop_job_class",
            "dbe_scheduler.drop_program",
            "dbe_scheduler.drop_schedule",
            "dbe_scheduler.enable",
            "dbe_scheduler.eval_calendar_string",
            "dbe_scheduler.generate_job_name",
            "dbe_scheduler.grant_user_authorization",
            "dbe_scheduler.revoke_user_authorization",
            "dbe_scheduler.run_backend_job",
            "dbe_scheduler.run_foreground_job",
            "dbe_scheduler.set_attribute",
            "dbe_scheduler.set_job_argument_value",
            "dbe_scheduler.stop_job",
        ] {
            let meta = lookup_function(name).unwrap_or_else(|| panic!("{} not found", name));
            assert_eq!(meta.domain, FuncDomain::DbeScheduler);
        }
    }

    #[test]
    fn test_phase4_dbe_sql_util_registered() {
        for name in &[
            "dbe_sql_util.create_abort_sql_patch",
            "dbe_sql_util.create_hint_sql_patch",
            "dbe_sql_util.disable_sql_patch",
            "dbe_sql_util.drop_sql_patch",
            "dbe_sql_util.enable_sql_patch",
            "dbe_sql_util.show_sql_patch",
        ] {
            let meta = lookup_function(name).unwrap_or_else(|| panic!("{} not found", name));
            assert_eq!(meta.domain, FuncDomain::DbeSqlUtil);
        }
    }

    #[test]
    fn test_phase4_domain_string_mappings() {
        assert_eq!(
            lookup_builtin_meta_qualified("dbe_application_info.set_action").unwrap().domain,
            "DbeApplicationInfo"
        );
        assert_eq!(lookup_builtin_meta_qualified("dbe_match.edit_distance_similarity").unwrap().domain, "DbeMatch");
        assert_eq!(lookup_builtin_meta_qualified("dbe_raw.substr").unwrap().domain, "DbeRaw");
        assert_eq!(lookup_builtin_meta_qualified("dbe_sql_util.show_sql_patch").unwrap().domain, "DbeSqlUtil");
    }

    #[test]
    fn test_registry_extension_commercial_only() {
        let json = r#"[
            {
                "name": "dbe_test_commercial",
                "category": "Scalar",
                "domain": "Other",
                "min_args": 1,
                "max_args": 1,
                "supports_distinct": false,
                "compat": 7,
                "distribution": 2
            }
        ]"#;
        let reg = FunctionRegistry::new().with_extensions_from_json(json).unwrap();
        let meta = reg.lookup("dbe_test_commercial").unwrap();
        assert!(meta.distribution.is_commercial_only());
        assert!(!meta.distribution.contains(Distribution::OPENGAUSS));
    }

    #[test]
    fn test_registry_extension_defaults_to_both() {
        // Extension without distribution field should default to BOTH
        let json = r#"[
            {
                "name": "dbe_test_default",
                "category": "Scalar",
                "domain": "Other",
                "min_args": 1,
                "max_args": 1,
                "supports_distinct": false,
                "compat": 7
            }
        ]"#;
        let reg = FunctionRegistry::new().with_extensions_from_json(json).unwrap();
        let meta = reg.lookup("dbe_test_default").unwrap();
        assert_eq!(meta.distribution, Distribution::BOTH);
    }

    #[test]
    fn test_resolve_builtin_meta_plain_function() {
        let name: crate::ast::ObjectName = vec!["abs".into()];
        let meta = super::resolve_builtin_meta(&name).unwrap();
        assert_eq!(meta.domain, "Math");
    }

    #[test]
    fn test_resolve_builtin_meta_dotted_package_function() {
        let name: crate::ast::ObjectName = vec!["dbe_output".into(), "put_line".into()];
        let meta = super::resolve_builtin_meta(&name).unwrap();
        assert_eq!(meta.domain, "DbeOutput");
    }

    #[test]
    fn test_resolve_builtin_meta_unknown_returns_none() {
        let name: crate::ast::ObjectName = vec!["definitely_not_a_real_func_xyz".into()];
        assert!(super::resolve_builtin_meta(&name).is_none());
    }

    #[test]
    fn test_resolve_builtin_meta_schema_qualified_fallback() {
        let name: crate::ast::ObjectName = vec!["pg_catalog".into(), "abs".into()];
        let meta = super::resolve_builtin_meta(&name).unwrap();
        assert_eq!(meta.domain, "Math");
    }

    // ── Issue #258: user packages must not be misidentified as builtin ──

    #[test]
    fn test_issue_258_pack_log_log_not_builtin() {
        assert!(super::lookup_function_qualified("pack_log.log").is_none());
        let name: crate::ast::ObjectName = vec!["pack_log".into(), "log".into()];
        assert!(super::resolve_builtin_meta(&name).is_none());
    }

    #[test]
    fn test_issue_258_bigfund_pack_log_count_not_builtin() {
        assert!(super::lookup_function_qualified("bigfund.pack_log.count").is_none());
        let name: crate::ast::ObjectName = vec!["bigfund".into(), "pack_log".into(), "count".into()];
        assert!(super::resolve_builtin_meta(&name).is_none());
    }

    #[test]
    fn test_issue_258_pckg_ctp_lg_public_log_not_builtin() {
        assert!(super::lookup_function_qualified("pckg_ctp_lg_public.log").is_none());
    }

    #[test]
    fn test_issue_258_my_utils_substr_not_builtin() {
        assert!(super::lookup_function_qualified("my_utils.substr").is_none());
        let name: crate::ast::ObjectName = vec!["my_utils".into(), "substr".into()];
        assert!(super::resolve_builtin_meta(&name).is_none());
    }

    #[test]
    fn test_issue_258_user_schema_upper_not_builtin() {
        assert!(super::lookup_function_qualified("some_schema.upper").is_none());
        assert!(super::lookup_function_qualified("myschema.abs").is_none());
    }

    #[test]
    fn test_issue_258_system_prefix_still_resolves() {
        let meta = super::lookup_function_qualified("pg_catalog.upper");
        assert!(meta.is_some());
        assert_eq!(meta.unwrap().domain, FuncDomain::String);

        let meta = super::lookup_function_qualified("sys.abs");
        assert!(meta.is_some());
        assert_eq!(meta.unwrap().domain, FuncDomain::Math);
    }

    #[test]
    fn test_issue_258_oracle_compat_package_still_resolves() {
        let meta = super::lookup_function_qualified("dbe_output.put_line");
        assert!(meta.is_some());
        assert_eq!(meta.unwrap().domain, FuncDomain::DbeOutput);

        let meta = super::lookup_function_qualified("dbms_output.put_line");
        assert!(meta.is_some());
    }
}
