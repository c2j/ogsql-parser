//! 共享工具函数。

/// 从位置 start 开始查找匹配的 `}`，考虑嵌套 `{}`。
pub fn find_closing_brace(chars: &[char], start: usize) -> Option<usize> {
    let mut depth = 1;
    let mut i = start;
    while i < chars.len() {
        match chars[i] {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// 从 MyBatis 参数字符串中提取 name 和可选的 javaType/jdbcType。
/// 格式: `name` or `name,javaType=double` or `name,jdbcType=NUMERIC`.
/// Prefers javaType over jdbcType.
pub fn parse_param_type(param: &str) -> (String, Option<String>) {
    let mut parts = param.split(',');
    let name = parts.next().unwrap_or("").trim().to_string();
    let mut java_type: Option<String> = None;
    let mut jdbc_type: Option<String> = None;
    for part in parts {
        let part = part.trim();
        if let Some(val) = part.strip_prefix("javaType=") {
            java_type = Some(val.to_string());
        } else if let Some(val) = part.strip_prefix("jdbcType=") {
            jdbc_type = Some(val.to_string());
        }
    }
    (name, java_type.or(jdbc_type))
}
