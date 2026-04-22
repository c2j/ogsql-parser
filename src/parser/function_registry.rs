/// Function registry for OpenGauss built-in function recognition.
///
/// Provides metadata lookup (`lookup_function`) and validation (`validate_function_call`)
/// for built-in functions. Core layer is compile-time constant; extension layer TBD.
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
    TextSearch,
    Crypto,
    System,
    TypeConversion,
    OracleCompat,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FuncMeta {
    pub name: &'static str,
    pub category: FuncCategory,
    pub domain: FuncDomain,
    pub min_args: u8,
    pub max_args: Option<u8>,
    pub supports_distinct: bool,
    pub compat: CompatMode,
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

impl FunctionRegistry {
    pub fn new() -> Self {
        FunctionRegistry {
            extensions: HashMap::new(),
        }
    }

    /// Load extension entries from a JSON string (array of `FuncMetaOwned`).
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
                            location: location.clone(),
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
                            location: location.clone(),
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
                            location: location.clone(),
                        });
                    }
                    if arg_count > max as usize {
                        warnings.push(ParserError::Warning {
                            message: format!(
                                "function {} takes at most {} argument(s)",
                                display_name, max
                            ),
                            location: location.clone(),
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
                            location: location.clone(),
                        });
                    }
                }
            }
        }

        if has_distinct && !meta.supports_distinct {
            warnings.push(ParserError::Warning {
                message: format!("DISTINCT is not supported for function {}", display_name),
                location: location.clone(),
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
        }
    };
}

/// Sorted (by lowercase name) array of all registered built-in functions.
static FUNCTIONS: &[FuncMeta] = &[
    // ── A ───────────────────────────────────────────────────────
    f!(
        "abs",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "acos",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    fo!(
        "add_months",
        FuncCategory::Scalar,
        FuncDomain::OracleCompat,
        2,
        Some(2),
        false
    ),
    f!(
        "age",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        1,
        Some(2),
        false
    ),
    f!(
        "array_agg",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    f!(
        "ascii",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(1),
        false
    ),
    f!(
        "asin",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "atan",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "atan2",
        FuncCategory::Scalar,
        FuncDomain::Math,
        2,
        Some(2),
        false
    ),
    f!(
        "avg",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    // ── B ───────────────────────────────────────────────────────
    f!(
        "bit_and",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    f!(
        "bit_length",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(1),
        false
    ),
    f!(
        "bit_or",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    f!(
        "btrim",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(2),
        false
    ),
    // ── C ───────────────────────────────────────────────────────
    f!(
        "cbrt",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "ceil",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "ceiling",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "char_length",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(1),
        false
    ),
    f!(
        "character_length",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(1),
        false
    ),
    f!(
        "chr",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(1),
        false
    ),
    f!(
        "clock_timestamp",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        0,
        Some(0),
        false
    ),
    f!(
        "coalesce",
        FuncCategory::Special,
        FuncDomain::Other,
        2,
        None,
        false
    ),
    f!(
        "concat",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        None,
        false
    ),
    f!(
        "concat_ws",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        None,
        false
    ),
    f!(
        "convert",
        FuncCategory::Scalar,
        FuncDomain::TypeConversion,
        2,
        Some(3),
        false
    ),
    f!(
        "corr",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        2,
        Some(2),
        true
    ),
    f!(
        "cos",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "cot",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "count",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    f!(
        "covar_pop",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        2,
        Some(2),
        true
    ),
    f!(
        "covar_samp",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        2,
        Some(2),
        true
    ),
    f!(
        "cume_dist",
        FuncCategory::Window,
        FuncDomain::Window,
        0,
        Some(0),
        false
    ),
    f!(
        "current_database",
        FuncCategory::Scalar,
        FuncDomain::System,
        0,
        Some(0),
        false
    ),
    f!(
        "current_date",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        0,
        Some(0),
        false
    ),
    f!(
        "current_schema",
        FuncCategory::Scalar,
        FuncDomain::System,
        0,
        Some(1),
        false
    ),
    f!(
        "current_time",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        0,
        Some(1),
        false
    ),
    f!(
        "current_timestamp",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        0,
        Some(1),
        false
    ),
    f!(
        "current_user",
        FuncCategory::Scalar,
        FuncDomain::System,
        0,
        Some(0),
        false
    ),
    // ── D ───────────────────────────────────────────────────────
    f!(
        "date_part",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        2,
        Some(2),
        false
    ),
    f!(
        "date_trunc",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        2,
        Some(2),
        false
    ),
    fo!(
        "decode",
        FuncCategory::Special,
        FuncDomain::OracleCompat,
        2,
        None,
        false
    ),
    f!(
        "degrees",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "dense_rank",
        FuncCategory::Window,
        FuncDomain::Window,
        0,
        Some(0),
        false
    ),
    f!(
        "div",
        FuncCategory::Scalar,
        FuncDomain::Math,
        2,
        Some(2),
        false
    ),
    // ── E ───────────────────────────────────────────────────────
    f!(
        "encode",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(2),
        false
    ),
    f!(
        "every",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    f!(
        "exp",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "extract",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        2,
        Some(2),
        false
    ),
    // ── F ───────────────────────────────────────────────────────
    f!(
        "factorial",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "first_value",
        FuncCategory::Window,
        FuncDomain::Window,
        1,
        Some(1),
        false
    ),
    f!(
        "floor",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "format",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        None,
        false
    ),
    // ── G ───────────────────────────────────────────────────────
    f!(
        "gcd",
        FuncCategory::Scalar,
        FuncDomain::Math,
        2,
        Some(2),
        false
    ),
    f!(
        "generate_series",
        FuncCategory::SetReturning,
        FuncDomain::Other,
        2,
        Some(3),
        false
    ),
    f!(
        "generate_subscripts",
        FuncCategory::SetReturning,
        FuncDomain::Array,
        2,
        Some(3),
        false
    ),
    f!(
        "greatest",
        FuncCategory::Special,
        FuncDomain::Other,
        2,
        None,
        false
    ),
    fo!(
        "group_concat",
        FuncCategory::Aggregate,
        FuncDomain::String,
        1,
        None,
        true
    ),
    // ── I ───────────────────────────────────────────────────────
    f!(
        "inet_client_addr",
        FuncCategory::Scalar,
        FuncDomain::System,
        0,
        Some(0),
        false
    ),
    f!(
        "inet_client_port",
        FuncCategory::Scalar,
        FuncDomain::System,
        0,
        Some(0),
        false
    ),
    f!(
        "inet_server_addr",
        FuncCategory::Scalar,
        FuncDomain::System,
        0,
        Some(0),
        false
    ),
    f!(
        "inet_server_port",
        FuncCategory::Scalar,
        FuncDomain::System,
        0,
        Some(0),
        false
    ),
    f!(
        "initcap",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(1),
        false
    ),
    fo!(
        "instr",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(4),
        false
    ),
    fo!(
        "instrb",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(4),
        false
    ),
    f!(
        "isfinite",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        1,
        Some(1),
        false
    ),
    // ── J ───────────────────────────────────────────────────────
    f!(
        "json_array_elements",
        FuncCategory::SetReturning,
        FuncDomain::Json,
        1,
        Some(1),
        false
    ),
    f!(
        "json_each",
        FuncCategory::SetReturning,
        FuncDomain::Json,
        1,
        Some(1),
        false
    ),
    f!(
        "json_each_text",
        FuncCategory::SetReturning,
        FuncDomain::Json,
        1,
        Some(1),
        false
    ),
    f!(
        "json_object_keys",
        FuncCategory::SetReturning,
        FuncDomain::Json,
        1,
        Some(1),
        false
    ),
    f!(
        "jsonb_array_elements",
        FuncCategory::SetReturning,
        FuncDomain::Json,
        1,
        Some(1),
        false
    ),
    f!(
        "jsonb_each",
        FuncCategory::SetReturning,
        FuncDomain::Json,
        1,
        Some(1),
        false
    ),
    f!(
        "jsonb_each_text",
        FuncCategory::SetReturning,
        FuncDomain::Json,
        1,
        Some(1),
        false
    ),
    f!(
        "jsonb_object_keys",
        FuncCategory::SetReturning,
        FuncDomain::Json,
        1,
        Some(1),
        false
    ),
    f!(
        "justify_days",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        1,
        Some(1),
        false
    ),
    f!(
        "justify_hours",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        1,
        Some(1),
        false
    ),
    f!(
        "justify_interval",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        1,
        Some(1),
        false
    ),
    // ── L ───────────────────────────────────────────────────────
    f!(
        "lag",
        FuncCategory::Window,
        FuncDomain::Window,
        2,
        Some(3),
        false
    ),
    fo!(
        "last_day",
        FuncCategory::Scalar,
        FuncDomain::OracleCompat,
        1,
        Some(1),
        false
    ),
    f!(
        "last_value",
        FuncCategory::Window,
        FuncDomain::Window,
        1,
        Some(1),
        false
    ),
    f!(
        "lastval",
        FuncCategory::Scalar,
        FuncDomain::System,
        0,
        Some(0),
        false
    ),
    f!(
        "lcm",
        FuncCategory::Scalar,
        FuncDomain::Math,
        2,
        Some(2),
        false
    ),
    f!(
        "lead",
        FuncCategory::Window,
        FuncDomain::Window,
        2,
        Some(3),
        false
    ),
    f!(
        "least",
        FuncCategory::Special,
        FuncDomain::Other,
        2,
        None,
        false
    ),
    f!(
        "left",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(2),
        false
    ),
    f!(
        "length",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(2),
        false
    ),
    fo!(
        "listagg",
        FuncCategory::Aggregate,
        FuncDomain::String,
        1,
        Some(2),
        true
    ),
    f!(
        "ln",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "localtime",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        0,
        Some(1),
        false
    ),
    f!(
        "localtimestamp",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        0,
        Some(1),
        false
    ),
    f!(
        "log",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(2),
        false
    ),
    f!(
        "log10",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "lower",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(1),
        false
    ),
    f!(
        "lpad",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(3),
        false
    ),
    f!(
        "ltrim",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(2),
        false
    ),
    // ── M ───────────────────────────────────────────────────────
    f!(
        "make_date",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        3,
        Some(3),
        false
    ),
    f!(
        "make_time",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        3,
        Some(3),
        false
    ),
    f!(
        "make_timestamp",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        6,
        Some(6),
        false
    ),
    f!(
        "make_timestamptz",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        6,
        Some(7),
        false
    ),
    f!(
        "max",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    f!(
        "median",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    f!(
        "min",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    f!(
        "mod",
        FuncCategory::Scalar,
        FuncDomain::Math,
        2,
        Some(2),
        false
    ),
    f!(
        "mode",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    fo!(
        "months_between",
        FuncCategory::Scalar,
        FuncDomain::OracleCompat,
        2,
        Some(2),
        false
    ),
    // ── N ───────────────────────────────────────────────────────
    fo!(
        "next_day",
        FuncCategory::Scalar,
        FuncDomain::OracleCompat,
        2,
        Some(2),
        false
    ),
    fo!(
        "nls_initcap",
        FuncCategory::Scalar,
        FuncDomain::OracleCompat,
        1,
        Some(2),
        false
    ),
    fo!(
        "nls_lower",
        FuncCategory::Scalar,
        FuncDomain::OracleCompat,
        1,
        Some(2),
        false
    ),
    fo!(
        "nls_sort",
        FuncCategory::Scalar,
        FuncDomain::OracleCompat,
        1,
        Some(2),
        false
    ),
    fo!(
        "nls_upper",
        FuncCategory::Scalar,
        FuncDomain::OracleCompat,
        1,
        Some(2),
        false
    ),
    fo!(
        "nlssort",
        FuncCategory::Scalar,
        FuncDomain::OracleCompat,
        1,
        Some(2),
        false
    ),
    f!(
        "now",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        0,
        Some(0),
        false
    ),
    f!(
        "nth_value",
        FuncCategory::Window,
        FuncDomain::Window,
        2,
        Some(2),
        false
    ),
    f!(
        "ntile",
        FuncCategory::Window,
        FuncDomain::Window,
        1,
        Some(1),
        false
    ),
    f!(
        "nullif",
        FuncCategory::Special,
        FuncDomain::Other,
        2,
        Some(2),
        false
    ),
    fo!(
        "nvl",
        FuncCategory::Special,
        FuncDomain::OracleCompat,
        2,
        Some(2),
        false
    ),
    fo!(
        "nvl2",
        FuncCategory::Special,
        FuncDomain::OracleCompat,
        3,
        Some(3),
        false
    ),
    // ── O ───────────────────────────────────────────────────────
    f!(
        "octet_length",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(1),
        false
    ),
    f!(
        "overlay",
        FuncCategory::Scalar,
        FuncDomain::String,
        3,
        Some(4),
        false
    ),
    // ── P ───────────────────────────────────────────────────────
    f!(
        "percent_rank",
        FuncCategory::Window,
        FuncDomain::Window,
        0,
        Some(0),
        false
    ),
    f!(
        "percentile_cont",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(2),
        true
    ),
    f!(
        "percentile_disc",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(2),
        true
    ),
    f!(
        "pg_backend_pid",
        FuncCategory::Scalar,
        FuncDomain::System,
        0,
        Some(0),
        false
    ),
    f!(
        "pg_conf_load_time",
        FuncCategory::Scalar,
        FuncDomain::System,
        0,
        Some(0),
        false
    ),
    f!(
        "pg_postmaster_start_time",
        FuncCategory::Scalar,
        FuncDomain::System,
        0,
        Some(0),
        false
    ),
    f!(
        "pg_sleep",
        FuncCategory::Scalar,
        FuncDomain::System,
        1,
        Some(1),
        false
    ),
    f!(
        "pi",
        FuncCategory::Scalar,
        FuncDomain::Math,
        0,
        Some(0),
        false
    ),
    f!(
        "position",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(2),
        false
    ),
    f!(
        "power",
        FuncCategory::Scalar,
        FuncDomain::Math,
        2,
        Some(2),
        false
    ),
    // ── Q ───────────────────────────────────────────────────────
    f!(
        "quote_ident",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(1),
        false
    ),
    f!(
        "quote_literal",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(1),
        false
    ),
    f!(
        "quote_nullable",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(1),
        false
    ),
    // ── R ───────────────────────────────────────────────────────
    f!(
        "radians",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "rank",
        FuncCategory::Window,
        FuncDomain::Window,
        0,
        Some(0),
        false
    ),
    f!(
        "ratio_to_report",
        FuncCategory::Window,
        FuncDomain::Window,
        1,
        Some(1),
        false
    ),
    f!(
        "regexp_count",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(4),
        false
    ),
    f!(
        "regexp_instr",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(5),
        false
    ),
    f!(
        "regexp_like",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(3),
        false
    ),
    f!(
        "regexp_matches",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(3),
        false
    ),
    f!(
        "regexp_replace",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(6),
        false
    ),
    f!(
        "regexp_substr",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(4),
        false
    ),
    f!(
        "regr_avgx",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        2,
        Some(2),
        true
    ),
    f!(
        "regr_avgy",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        2,
        Some(2),
        true
    ),
    f!(
        "regr_count",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        2,
        Some(2),
        true
    ),
    f!(
        "regr_intercept",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        2,
        Some(2),
        true
    ),
    f!(
        "regr_r2",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        2,
        Some(2),
        true
    ),
    f!(
        "regr_slope",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        2,
        Some(2),
        true
    ),
    f!(
        "repeat",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(2),
        false
    ),
    f!(
        "replace",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(3),
        false
    ),
    f!(
        "reverse",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(1),
        false
    ),
    f!(
        "right",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(2),
        false
    ),
    f!(
        "round",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(2),
        false
    ),
    f!(
        "row_number",
        FuncCategory::Window,
        FuncDomain::Window,
        0,
        Some(0),
        false
    ),
    fo!(
        "rownum",
        FuncCategory::Scalar,
        FuncDomain::OracleCompat,
        0,
        Some(0),
        false
    ),
    f!(
        "rpad",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(3),
        false
    ),
    f!(
        "rtrim",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(2),
        false
    ),
    // ── S ───────────────────────────────────────────────────────
    f!(
        "session_user",
        FuncCategory::Scalar,
        FuncDomain::System,
        0,
        Some(0),
        false
    ),
    f!(
        "sign",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "sin",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "split_part",
        FuncCategory::Scalar,
        FuncDomain::String,
        3,
        Some(3),
        false
    ),
    f!(
        "sqrt",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "statement_timestamp",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        0,
        Some(0),
        false
    ),
    f!(
        "stddev",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    f!(
        "stddev_pop",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    f!(
        "stddev_samp",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    f!(
        "string_agg",
        FuncCategory::Aggregate,
        FuncDomain::String,
        2,
        Some(2),
        true
    ),
    f!(
        "strpos",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(2),
        false
    ),
    f!(
        "substr",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(3),
        false
    ),
    fo!(
        "substrb",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(3),
        false
    ),
    f!(
        "substring",
        FuncCategory::Scalar,
        FuncDomain::String,
        2,
        Some(3),
        false
    ),
    f!(
        "sum",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    fo!(
        "sysdate",
        FuncCategory::Scalar,
        FuncDomain::OracleCompat,
        0,
        Some(0),
        false
    ),
    // ── T ───────────────────────────────────────────────────────
    f!(
        "tan",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(1),
        false
    ),
    f!(
        "to_ascii",
        FuncCategory::Scalar,
        FuncDomain::TypeConversion,
        1,
        Some(2),
        false
    ),
    f!(
        "to_char",
        FuncCategory::Scalar,
        FuncDomain::TypeConversion,
        1,
        Some(2),
        false
    ),
    f!(
        "to_date",
        FuncCategory::Scalar,
        FuncDomain::TypeConversion,
        1,
        Some(2),
        false
    ),
    f!(
        "to_hex",
        FuncCategory::Scalar,
        FuncDomain::TypeConversion,
        1,
        Some(1),
        false
    ),
    f!(
        "to_number",
        FuncCategory::Scalar,
        FuncDomain::TypeConversion,
        1,
        Some(2),
        false
    ),
    f!(
        "to_timestamp",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        1,
        Some(2),
        false
    ),
    f!(
        "transaction_timestamp",
        FuncCategory::Scalar,
        FuncDomain::DateTime,
        0,
        Some(0),
        false
    ),
    f!(
        "translate",
        FuncCategory::Scalar,
        FuncDomain::String,
        3,
        Some(3),
        false
    ),
    f!(
        "trim",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(3),
        false
    ),
    f!(
        "trunc",
        FuncCategory::Scalar,
        FuncDomain::Math,
        1,
        Some(2),
        false
    ),
    // ── U ───────────────────────────────────────────────────────
    f!(
        "unnest",
        FuncCategory::SetReturning,
        FuncDomain::Array,
        1,
        Some(1),
        false
    ),
    f!(
        "upper",
        FuncCategory::Scalar,
        FuncDomain::String,
        1,
        Some(1),
        false
    ),
    f!(
        "user",
        FuncCategory::Scalar,
        FuncDomain::System,
        0,
        Some(0),
        false
    ),
    // ── V ───────────────────────────────────────────────────────
    f!(
        "var_pop",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    f!(
        "var_samp",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    f!(
        "variance",
        FuncCategory::Aggregate,
        FuncDomain::Aggregate,
        1,
        Some(1),
        true
    ),
    f!(
        "version",
        FuncCategory::Scalar,
        FuncDomain::System,
        0,
        Some(0),
        false
    ),
    // ── W ───────────────────────────────────────────────────────
    f!(
        "width_bucket",
        FuncCategory::Scalar,
        FuncDomain::Math,
        3,
        Some(4),
        false
    ),
    fo!(
        "wm_concat",
        FuncCategory::Aggregate,
        FuncDomain::String,
        1,
        None,
        true
    ),
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
        }.to_string(),
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
            FuncDomain::TextSearch => "TextSearch",
            FuncDomain::Crypto => "Crypto",
            FuncDomain::System => "System",
            FuncDomain::TypeConversion => "TypeConversion",
            FuncDomain::OracleCompat => "OracleCompat",
            FuncDomain::Ai => "Ai",
            FuncDomain::Other => "Other",
        }.to_string(),
    })
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
                        message: format!(
                            "function {} requires exactly {} argument(s)",
                            meta.name, meta.min_args
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
                            meta.name, meta.min_args
                        ),
                        location,
                    });
                }
                if arg_count > max as usize {
                    warnings.push(ParserError::Warning {
                        message: format!(
                            "function {} takes at most {} argument(s)",
                            meta.name, max
                        ),
                        location,
                    });
                }
            }
            None => {
                // Variadic: min_args..∞
                if arg_count < meta.min_args as usize {
                    warnings.push(ParserError::Warning {
                        message: format!(
                            "function {} requires at least {} argument(s)",
                            meta.name, meta.min_args
                        ),
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
        let meta =
            lookup_function("generate_series").expect("generate_series should be registered");
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
        let reg = FunctionRegistry::new()
            .with_extensions_from_json(json)
            .unwrap();
        let meta = reg
            .lookup("custom_encrypt")
            .expect("extension func should be found");
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
        let reg = FunctionRegistry::new()
            .with_extensions_from_json(json)
            .unwrap();
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
        let reg = FunctionRegistry::new()
            .with_extensions_from_json(json)
            .unwrap();
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
        let reg = FunctionRegistry::new()
            .with_extensions_from_json("[]")
            .unwrap();
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
        let reg = FunctionRegistry::new()
            .with_extensions_from_json(json)
            .unwrap();
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
        let reg = FunctionRegistry::new()
            .with_extensions_from_json(json)
            .unwrap();
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
        assert!(reg
            .validate("nonexistent_xyz", 5, true, true, false, loc())
            .is_empty());
    }
}
