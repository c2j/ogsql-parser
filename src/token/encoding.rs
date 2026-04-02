//! Character encoding support for SQL files.
//!
//! openGauss supports multiple character encodings. This module provides
//! detection and conversion to UTF-8 for tokenization.

use encoding_rs::{CoderResult, Encoding};

/// Detect and decode SQL file content to UTF-8.
///
/// Tries multiple encodings in order of likelihood:
/// 1. UTF-8 (most common)
/// 2. EUC-JP (Japanese)
/// 3. EUC-KR (Korean)
/// 4. GB18030 / EUC-CN (Chinese Simplified)
/// 5. EUC-TW (Chinese Traditional)
/// 6. UTF-16 LE/BE
///
/// Returns the decoded string and the encoding name.
pub fn decode_sql_file(bytes: &[u8]) -> Result<(String, &'static str), std::io::Error> {
    // First try UTF-8 (most common case)
    if let Ok(s) = String::from_utf8(bytes.to_vec()) {
        return Ok((s, "UTF-8"));
    }

    // Try various Asian encodings commonly used in openGauss
    let encodings: &[(&'static Encoding, &'static str)] = &[
        (encoding_rs::EUC_JP, "EUC-JP"),
        (encoding_rs::EUC_KR, "EUC-KR"),
        (encoding_rs::GB18030, "GB18030"),
        (encoding_rs::BIG5, "BIG5"),
        (encoding_rs::UTF_16LE, "UTF-16LE"),
        (encoding_rs::UTF_16BE, "UTF-16BE"),
    ];

    for (encoding, name) in encodings {
        let (cow, _, had_errors) = encoding.decode(bytes);
        if !had_errors {
            return Ok((cow.into_owned(), name));
        }
    }

    // Fallback: try lossy UTF-8 conversion
    Ok((String::from_utf8_lossy(bytes).into_owned(), "UTF-8-lossy"))
}

/// Decode with a specific encoding.
pub fn decode_with_encoding(bytes: &[u8], encoding_name: &str) -> Result<String, std::io::Error> {
    let encoding = match encoding_name.to_uppercase().as_str() {
        "UTF-8" | "UTF8" => {
            return String::from_utf8(bytes.to_vec())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e));
        }
        "EUC-JP" | "EUCJP" | "EUC_JP" => encoding_rs::EUC_JP,
        "EUC-KR" | "EUCKR" | "EUC_KR" => encoding_rs::EUC_KR,
        "EUC-CN" | "EUCCN" | "EUC_CN" | "GB2312" | "GBK" | "GB18030" => encoding_rs::GB18030,
        "EUC-TW" | "EUCTW" | "EUC_TW" => encoding_rs::BIG5,
        "BIG5" => encoding_rs::BIG5,
        "UTF-16" | "UTF16" => {
            // Try LE first, then BE
            let (cow, _, had_errors) = encoding_rs::UTF_16LE.decode(bytes);
            if !had_errors {
                return Ok(cow.into_owned());
            }
            let (cow, _, had_errors) = encoding_rs::UTF_16BE.decode(bytes);
            if !had_errors {
                return Ok(cow.into_owned());
            }
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid UTF-16",
            ));
        }
        "UTF-16LE" | "UTF16LE" => encoding_rs::UTF_16LE,
        "UTF-16BE" | "UTF16BE" => encoding_rs::UTF_16BE,
        "SHIFT_JIS" | "SHIFT-JIS" | "SJIS" => encoding_rs::SHIFT_JIS,
        "ISO-2022-JP" | "ISO2022JP" => encoding_rs::ISO_2022_JP,
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Unsupported encoding: {}", encoding_name),
            ));
        }
    };

    let (cow, _, had_errors) = encoding.decode(bytes);
    if had_errors {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid {} data", encoding_name),
        ))
    } else {
        Ok(cow.into_owned())
    }
}

/// Check if bytes are valid UTF-8.
pub fn is_utf8(bytes: &[u8]) -> bool {
    std::str::from_utf8(bytes).is_ok()
}

/// Try to detect encoding by checking file content patterns.
/// This is a simple heuristic and not 100% accurate.
pub fn detect_encoding(bytes: &[u8]) -> &'static str {
    if is_utf8(bytes) {
        return "UTF-8";
    }

    // Check for UTF-16 BOM
    if bytes.starts_with(&[0xFF, 0xFE]) {
        return "UTF-16LE";
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        return "UTF-16BE";
    }

    // Try encodings and pick first that decodes without errors
    let encodings = [
        (encoding_rs::EUC_JP, "EUC-JP"),
        (encoding_rs::EUC_KR, "EUC-KR"),
        (encoding_rs::GB18030, "GB18030"),
        (encoding_rs::BIG5, "BIG5"),
    ];

    for (encoding, name) in encodings {
        let (_, _, had_errors) = encoding.decode(bytes);
        if !had_errors {
            return name;
        }
    }

    // Default to UTF-8 with lossy conversion
    "UTF-8"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8() {
        let bytes = b"SELECT * FROM table WHERE name = 'test';";
        let (s, enc) = decode_sql_file(bytes).unwrap();
        assert_eq!(enc, "UTF-8");
        assert!(s.contains("SELECT"));
    }

    #[test]
    fn test_empty() {
        let (s, enc) = decode_sql_file(b"").unwrap();
        assert_eq!(enc, "UTF-8");
        assert_eq!(s, "");
    }

    #[test]
    fn test_specific_encoding() {
        // Test decoding with specific encoding
        let bytes = b"SELECT * FROM table;";
        let s = decode_with_encoding(bytes, "UTF-8").unwrap();
        assert_eq!(s, "SELECT * FROM table;");
    }
}
