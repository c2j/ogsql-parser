//! Java string decoding helpers for SQL extraction.

use super::extract::ExtractContext;

impl<'a> ExtractContext<'a> {
    pub(super) fn decode_java_string(&self, raw: &str, is_text_block: bool) -> String {
        if is_text_block {
            self.decode_text_block(raw)
        } else {
            self.decode_regular_string(raw)
        }
    }

    pub(super) fn decode_regular_string(&self, raw: &str) -> String {
        let inner = raw.strip_prefix('"').and_then(|s| s.strip_suffix('"'));
        match inner {
            Some(s) => self.process_escape_sequences(s),
            None => raw.to_string(),
        }
    }

    pub(super) fn decode_text_block(&self, raw: &str) -> String {
        let inner = raw
            .strip_prefix("\"\"\"")
            .and_then(|s| s.strip_suffix("\"\"\""));
        let inner = match inner {
            Some(s) => s,
            None => return raw.to_string(),
        };

        let lines: Vec<&str> = inner.lines().collect();
        if lines.is_empty() {
            return String::new();
        }

        let start = if lines.first().map(|l| l.trim().is_empty()).unwrap_or(false) {
            1
        } else {
            0
        };

        let effective_lines = &lines[start..];
        let min_indent = effective_lines
            .iter()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.chars().take_while(|c| *c == ' ' || *c == '\t').count())
            .min()
            .unwrap_or(0);

        let result: Vec<String> = effective_lines
            .iter()
            .map(|l| {
                if l.len() >= min_indent {
                    l[min_indent..].to_string()
                } else {
                    l.trim_end().to_string()
                }
            })
            .collect();

        let mut joined = result.join("\n");
        joined = joined.trim().to_string();
        self.process_escape_sequences(&joined)
    }

    pub(super) fn process_escape_sequences(&self, s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '\\' && i + 1 < chars.len() {
                match chars[i + 1] {
                    'n' => {
                        result.push('\n');
                        i += 2;
                    }
                    't' => {
                        result.push('\t');
                        i += 2;
                    }
                    'r' => {
                        result.push('\r');
                        i += 2;
                    }
                    '"' => {
                        result.push('"');
                        i += 2;
                    }
                    '\'' => {
                        result.push('\'');
                        i += 2;
                    }
                    '\\' => {
                        result.push('\\');
                        i += 2;
                    }
                    _ => {
                        result.push(chars[i]);
                        i += 1;
                    }
                }
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }
        result
    }
}
