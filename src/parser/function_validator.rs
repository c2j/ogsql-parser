use crate::parser::ParserError;
use crate::token::SourceLocation;

const MIN_2_ARGS: &[&str] = &[
    "coalesce",
    "concat",
    "concat_ws",
    "greatest",
    "least",
    "decode",
    "nullif",
    "nvl",
    "nvl2",
];

const EXACTLY_1_ARG: &[&str] = &[
    "count",
    "sum",
    "avg",
    "min",
    "max",
    "stddev",
    "stddev_pop",
    "stddev_samp",
    "variance",
    "var_pop",
    "var_samp",
    "bit_and",
    "bit_or",
    "every",
];

const ZERO_ARGS: &[&str] = &[
    "now",
    "current_timestamp",
    "current_date",
    "current_time",
    "localtime",
    "localtimestamp",
    "clock_timestamp",
    "statement_timestamp",
    "transaction_timestamp",
    "pi",
    "version",
    "pg_backend_pid",
    "lastval",
];

const WINDOW_FUNCTIONS: &[&str] = &[
    "row_number",
    "rank",
    "dense_rank",
    "percent_rank",
    "cume_dist",
    "ntile",
    "lag",
    "lead",
    "first_value",
    "last_value",
    "nth_value",
    "ratio_to_report",
];

const DISTINCT_AGGREGATES: &[&str] = &[
    "count",
    "sum",
    "avg",
    "min",
    "max",
    "array_agg",
    "string_agg",
    "group_concat",
    "wm_concat",
    "corr",
    "covar_pop",
    "covar_samp",
    "regr_avgx",
    "regr_avgy",
    "regr_count",
    "regr_intercept",
    "regr_r2",
    "regr_slope",
    "stddev",
    "stddev_pop",
    "stddev_samp",
    "variance",
    "var_pop",
    "var_samp",
];

pub fn validate_function_call(
    name: &str,
    arg_count: usize,
    has_distinct: bool,
    has_over: bool,
    location: SourceLocation,
) -> Vec<ParserError> {
    let mut warnings = Vec::new();
    let name_lower = name.to_lowercase();

    if has_distinct && !DISTINCT_AGGREGATES.contains(&name_lower.as_str()) {
        warnings.push(ParserError::Warning {
            message: format!("Function '{}' does not support DISTINCT modifier", name),
            location: location.clone(),
        });
    }

    if ZERO_ARGS.contains(&name_lower.as_str()) && arg_count > 0 {
        warnings.push(ParserError::Warning {
            message: format!(
                "Function '{}' takes no arguments but {} were provided",
                name, arg_count
            ),
            location: location.clone(),
        });
    }

    if EXACTLY_1_ARG.contains(&name_lower.as_str()) && arg_count != 1 {
        warnings.push(ParserError::Warning {
            message: format!(
                "Function '{}' requires exactly 1 argument but {} were provided",
                name, arg_count
            ),
            location: location.clone(),
        });
    }

    if MIN_2_ARGS.contains(&name_lower.as_str()) && arg_count < 2 {
        warnings.push(ParserError::Warning {
            message: format!(
                "Function '{}' requires at least 2 arguments but {} were provided",
                name, arg_count
            ),
            location: location.clone(),
        });
    }

    if WINDOW_FUNCTIONS.contains(&name_lower.as_str()) && !has_over {
        warnings.push(ParserError::Warning {
            message: format!("Window function '{}' should have an OVER clause", name),
            location: location.clone(),
        });
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loc() -> SourceLocation {
        SourceLocation::default()
    }

    #[test]
    fn test_coalesce_too_few() {
        let w = validate_function_call("coalesce", 1, false, false, loc());
        assert!(w.iter().any(|e| e.to_string().contains("at least 2")));
    }

    #[test]
    fn test_coalesce_ok() {
        assert!(validate_function_call("coalesce", 2, false, false, loc()).is_empty());
    }

    #[test]
    fn test_count_no_args() {
        let w = validate_function_call("count", 0, false, false, loc());
        assert!(w.iter().any(|e| e.to_string().contains("exactly 1")));
    }

    #[test]
    fn test_window_no_over() {
        let w = validate_function_call("row_number", 0, false, false, loc());
        assert!(w.iter().any(|e| e.to_string().contains("OVER clause")));
    }

    #[test]
    fn test_window_with_over() {
        assert!(validate_function_call("row_number", 0, false, true, loc()).is_empty());
    }

    #[test]
    fn test_distinct_on_non_aggregate() {
        let w = validate_function_call("upper", 1, true, false, loc());
        assert!(w.iter().any(|e| e.to_string().contains("DISTINCT")));
    }

    #[test]
    fn test_distinct_on_aggregate() {
        assert!(validate_function_call("count", 1, true, false, loc()).is_empty());
    }

    #[test]
    fn test_now_with_args() {
        let w = validate_function_call("now", 1, false, false, loc());
        assert!(w.iter().any(|e| e.to_string().contains("no arguments")));
    }
}
