//! SQL heuristics and detection utilities for Java SQL extraction.

use super::constant::SQL_KEYWORDS;
use crate::java::types::{ParameterStyle, SqlKind};

pub(super) enum NativeQueryFlag {
    True,
    False,
    NotPresent,
}

pub(super) fn looks_like_sql(text: &str) -> bool {
    let upper = text.to_uppercase();
    SQL_KEYWORDS.iter().any(|kw| upper.contains(kw))
}

pub(super) fn detect_sql_kind_from_content(sql: &str) -> SqlKind {
    let upper = sql.trim().to_uppercase();
    let prefix = upper.split_whitespace().next().unwrap_or("");
    match prefix {
        "CREATE" | "ALTER" | "DROP" | "TRUNCATE" => SqlKind::Ddl,
        _ => SqlKind::NativeSql,
    }
}

pub(super) fn detect_parameter_style(sql: &str) -> ParameterStyle {
    let mut has_question = false;
    let mut has_numbered = false;
    let mut has_named_colon = false;
    let mut has_hash = false;

    let chars: Vec<char> = sql.chars().collect();
    let len = chars.len();
    let mut in_string = false;
    let mut i = 0;

    while i < len {
        if chars[i] == '\'' {
            in_string = !in_string;
            i += 1;
            continue;
        }
        if in_string {
            i += 1;
            continue;
        }
        if chars[i] == '?' && i + 1 < len && chars[i + 1].is_ascii_digit() {
            has_numbered = true;
        } else if chars[i] == '?' {
            has_question = true;
        } else if chars[i] == ':' && i + 1 < len && chars[i + 1].is_ascii_alphabetic() {
            has_named_colon = true;
        } else if chars[i] == '#' && i + 1 < len && chars[i + 1] == '{' {
            has_hash = true;
        }
        i += 1;
    }

    if has_hash {
        ParameterStyle::NamedHash
    } else if has_numbered {
        ParameterStyle::PositionalNumbered
    } else if has_named_colon {
        ParameterStyle::NamedColon
    } else if has_question {
        ParameterStyle::PositionalQuestion
    } else {
        ParameterStyle::None
    }
}

pub(super) fn convert_placeholders(sql: &str) -> String {
    let chars: Vec<char> = sql.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(sql.len() + sql.len() / 4);
    let mut i = 0;
    let mut question_counter: usize = 0;

    while i < len {
        if chars[i] == '\'' {
            result.push(chars[i]);
            i += 1;
            while i < len {
                if chars[i] == '\'' {
                    result.push(chars[i]);
                    i += 1;
                    break;
                }
                result.push(chars[i]);
                i += 1;
            }
            continue;
        }

        if chars[i] == '?' {
            question_counter += 1;
            let digit_start = i + 1;
            let mut digit_end = digit_start;
            while digit_end < len && chars[digit_end].is_ascii_digit() {
                digit_end += 1;
            }
            let param_num = if digit_end > digit_start {
                let num_str: String = chars[digit_start..digit_end].iter().collect();
                num_str.parse::<usize>().unwrap_or(question_counter)
            } else {
                question_counter
            };
            result.push_str(&format!("__JAVA_VAR_JDBC_PARAM_{}__", param_num));
            i = digit_end;
            continue;
        }

        if chars[i] == ':'
            && i + 1 < len
            && (chars[i + 1].is_ascii_alphabetic() || chars[i + 1] == '_')
        {
            let name_start = i + 1;
            let mut name_end = name_start;
            while name_end < len
                && (chars[name_end].is_ascii_alphanumeric() || chars[name_end] == '_')
            {
                name_end += 1;
            }
            let name: String = chars[name_start..name_end].iter().collect();
            result.push_str(&format!("__JAVA_VAR_{}__", name));
            i = name_end;
            continue;
        }

        result.push(chars[i]);
        i += 1;
    }

    result
}
