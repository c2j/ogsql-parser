//! Shared helpers for regression test files.
//!
//! Each `tests/regression_{module}.rs` imports this module:
//! ```rust
//! mod common;
//! use common::{load_issue, RegressFixture};
//! ```

use std::fs;
use std::path::PathBuf;

/// Parsed regression fixture.
pub struct RegressFixture {
    pub id: String,
    pub description: String,
    pub expect: String,
    pub content: String,
    pub file_type: FileType,
}

pub enum FileType {
    Sql,
    Xml,
    Java,
}

/// Load all fixture files matching `{typ}_{id}_{module}.*` from `tests/regress/`.
pub fn load_all(typ: &str, id: &str, module: &str) -> Vec<RegressFixture> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("regress");
    let prefix = format!("{}_{}_{}", typ, id, module);

    let mut results = Vec::new();
    let entries = match fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => return results,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let fname = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if fname != prefix {
            continue;
        }

        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let file_type = match ext {
            "sql" => FileType::Sql,
            "xml" => FileType::Xml,
            "java" => FileType::Java,
            _ => continue,
        };

        let raw = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let (meta, content) = parse_metadata(&raw, &file_type);
        results.push(RegressFixture {
            id: meta.get("Issue").cloned().unwrap_or_else(|| id.to_string()),
            description: meta.get("Description").cloned().unwrap_or_default(),
            expect: meta.get("Expect").cloned().unwrap_or_default(),
            content,
            file_type,
        });
    }

    results
}

/// Load fixtures for an issue.
pub fn load_issue(id: u32, module: &str) -> Vec<RegressFixture> {
    load_all("issue", &id.to_string(), module)
}

/// Load fixtures for a named feature or bug (when no issue number exists yet).
pub fn load_named(name: &str, module: &str) -> Vec<RegressFixture> {
    load_all("feat", name, module)
}

/// Parse metadata comments from the beginning of file content.
///
/// Metadata lines are consecutive comment lines at the file start,
/// matching `Key: Value` format. First non-comment, non-empty line
/// ends the metadata block and everything after is `content`.
fn parse_metadata(raw: &str, file_type: &FileType) -> (std::collections::HashMap<String, String>, String) {
    let comment_prefix = match file_type {
        FileType::Sql => "-- ",
        FileType::Java => "// ",
        FileType::Xml => "<!-- ",
    };
    let xml_suffix = match file_type {
        FileType::Xml => Some(" -->"),
        _ => None,
    };

    let mut meta = std::collections::HashMap::new();
    let mut lines = raw.lines();
    let mut content_start = 0usize;

    for line in lines.by_ref() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            content_start += line.len() + 1; // +1 for newline
            continue;
        }

        let stripped = if trimmed.starts_with(comment_prefix) {
            let mut s = &trimmed[comment_prefix.len()..];
            if let Some(suffix) = xml_suffix {
                if s.ends_with(suffix) {
                    s = &s[..s.len() - suffix.len()];
                }
            }
            s.trim()
        } else {
            // Not a comment → metadata block ends
            break;
        };

        if let Some((key, value)) = stripped.split_once(':') {
            meta.insert(key.trim().to_string(), value.trim().to_string());
        }
        content_start += line.len() + 1;
    }

    let content = if content_start < raw.len() {
        raw[content_start..].to_string()
    } else {
        String::new()
    };

    (meta, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sql_metadata() {
        let raw = "-- Issue: #246\n-- Description: test\n-- Expect: ok\n\nSELECT 1;\n";
        let (meta, content) = parse_metadata(raw, &FileType::Sql);
        assert_eq!(meta.get("Issue").unwrap(), "#246");
        assert_eq!(meta.get("Description").unwrap(), "test");
        assert_eq!(content.trim(), "SELECT 1;");
    }

    #[test]
    fn test_parse_xml_metadata() {
        let raw = "<!-- Issue: #300 -->\n<!-- Description: test -->\n\n<root/>\n";
        let (meta, content) = parse_metadata(raw, &FileType::Xml);
        assert_eq!(meta.get("Issue").unwrap(), "#300");
        assert_eq!(content.trim(), "<root/>");
    }

    #[test]
    fn test_parse_java_metadata() {
        let raw = "// Issue: #301\n// Description: test\n\npublic class T {}\n";
        let (meta, content) = parse_metadata(raw, &FileType::Java);
        assert_eq!(meta.get("Issue").unwrap(), "#301");
        assert!(content.contains("public class T"));
    }
}
