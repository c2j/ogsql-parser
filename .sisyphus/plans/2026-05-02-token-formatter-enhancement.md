# TokenFormatter Enhancement — 对标主流 SQL Formatter 工具

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 将 TokenFormatter 从 440 行的"最小可用"增强为对标 pgFormatter / Prettier SQL / sqlformat 的生产级 SQL 格式化器，支持完整的配置、DML/DDL/PL/pgSQL 结构化格式化。

**Architecture:** 在现有 `TokenFormatter` 基础上引入 `FormatConfig` 配置结构体，支持缩进宽度、关键字大小写、逗号位置、行宽限制等选项。核心格式化逻辑分为三层：配置层（FormatConfig）、结构识别层（识别 SQL 语句边界和子句结构）、格式化输出层（缩进、换行、对齐）。MCP/HTTP 接口统一切换到 TokenFormatter，保持注释保留能力。

**Tech Stack:** Rust，现有 Tokenizer 基础设施，现有 Token/Keyword 类型，serde for 配置序列化。

---

## 背景调研：主流工具能力矩阵

| 能力 | pgFormatter | Prettier SQL | sqlformat | 本项目目标 |
|------|------------|--------------|-----------|-----------|
| 缩进宽度配置 | ✅ spaces/tabs | ✅ indent string | ✅ indent_width | ✅ |
| 关键字大小写 | ✅ 4种模式 | ✅ upper/lower | ✅ upper/lower/capitalize | ✅ 3种 |
| 逗号位置 | ✅ start/end | ✅ before/after/tabular | ✅ comma_first | ✅ |
| 行宽限制 | ✅ wrap-limit | ✅ lineWidth | ✅ wrap_after | ✅ |
| SELECT 子句换行 | ✅ | ✅ | ✅ | ✅ |
| AND/OR 独立行 | ✅ | ✅ | ✅ | ✅ |
| JOIN 格式化 | ✅ | ✅ | ✅ | ✅ |
| CREATE TABLE 列对齐 | ✅ | ✅ | ✅ | ✅ |
| 子查询缩进 | ✅ | ✅ | ✅ | ✅ |
| PL/pgSQL 格式化 | ✅ 完整 | ✅ good | ✅ basic | ✅ 完整 |
| 注释保留 | ✅ | ✅ | ✅ | ✅ 已有 |
| 别名 AS 对齐 | — | ✅ tabulateAlias | — | ✅ |
| 分号新行 | — | ✅ semicolonNewline | — | ✅ |

---

## Part A: FormatConfig 配置系统

### Task 1: 定义 FormatConfig 结构体

**Files:**
- Modify: `src/token_formatter.rs`（在文件顶部添加 FormatConfig 定义）

**Step 1: 写测试**

在 `src/token_formatter.rs` 的 `#[cfg(test)]` 模块中添加：

```rust
#[test]
fn test_format_config_default() {
    let config = FormatConfig::default();
    assert_eq!(config.indent_width, 2);
    assert_eq!(config.keyword_case, KeywordCase::Preserve);
    assert_eq!(config.comma_style, CommaStyle::Trailing);
    assert_eq!(config.line_width, 120);
    assert_eq!(config.uppercase_keywords, false);
    assert_eq!(config.semicolon_newline, true);
}

#[test]
fn test_format_config_custom() {
    let config = FormatConfig {
        indent_width: 4,
        keyword_case: KeywordCase::Upper,
        comma_style: CommaStyle::Leading,
        line_width: 80,
        ..Default::default()
    };
    assert_eq!(config.indent_width, 4);
    assert_eq!(config.keyword_case, KeywordCase::Upper);
}
```

**Step 2: 运行测试确认失败**

Run: `cargo test test_format_config`
Expected: FAIL（类型未定义）

**Step 3: 实现 FormatConfig**

在 `src/token_formatter.rs` 文件顶部（`use` 语句之后、`TokenFormatter` 结构体之前）添加：

```rust
/// Keyword casing mode for SQL formatting
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KeywordCase {
    /// Preserve original casing from source
    Preserve,
    /// Convert all keywords to UPPERCASE
    Upper,
    /// Convert all keywords to lowercase
    Lower,
}

/// Comma positioning style for column lists
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommaStyle {
    /// Comma at end of line: `col1, col2, col3`
    Trailing,
    /// Comma at start of line: `col1\n, col2\n, col3`
    Leading,
}

/// Configuration for SQL formatting
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FormatConfig {
    /// Number of spaces per indentation level (default: 2)
    pub indent_width: usize,
    /// Keyword casing mode (default: Preserve)
    pub keyword_case: KeywordCase,
    /// Comma positioning in lists (default: Trailing)
    pub comma_style: CommaStyle,
    /// Maximum line width before wrapping (default: 120, 0 = no wrapping)
    pub line_width: usize,
    /// Convert keywords to uppercase (legacy compat, overrides keyword_case when true)
    #[serde(default)]
    pub uppercase_keywords: bool,
    /// Put semicolons on their own line (default: true)
    #[serde(default = "default_true")]
    pub semicolon_newline: bool,
    /// Put each SELECT target expression on its own line (default: true)
    #[serde(default = "default_true")]
    pub select_newline: bool,
    /// Put WHERE/AND/OR on new lines (default: true)
    #[serde(default = "default_true")]
    pub logical_operator_newline: bool,
}

fn default_true() -> bool { true }

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            indent_width: 2,
            keyword_case: KeywordCase::Preserve,
            comma_style: CommaStyle::Trailing,
            line_width: 120,
            uppercase_keywords: false,
            semicolon_newline: true,
            select_newline: true,
            logical_operator_newline: true,
        }
    }
}
```

**Step 4: 运行测试确认通过**

Run: `cargo test test_format_config`
Expected: PASS

**Step 5: 提交**

```
feat(formatter): add FormatConfig struct with keyword case, comma style, indent options
```

---

### Task 2: 将 FormatConfig 集成到 TokenFormatter

**Files:**
- Modify: `src/token_formatter.rs`

**Step 1: 写测试**

```rust
#[test]
fn test_indent_width_4() {
    let config = FormatConfig { indent_width: 4, ..Default::default() };
    let input = "BEGIN x := 1; END";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, config).format();
    assert!(output.contains("BEGIN\n    x := 1;\nEND"), "4-space indent: {:?}", output);
}

#[test]
fn test_keyword_case_upper() {
    let config = FormatConfig { keyword_case: KeywordCase::Upper, ..Default::default() };
    let input = "select id from users";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, config).format();
    assert!(output.contains("SELECT"), "Keywords should be uppercase: {:?}", output);
    assert!(!output.contains("select"), "Should not contain lowercase select");
}

#[test]
fn test_keyword_case_lower() {
    let config = FormatConfig { keyword_case: KeywordCase::Lower, ..Default::default() };
    let input = "SELECT id FROM users";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, config).format();
    assert!(output.contains("select"), "Keywords should be lowercase: {:?}", output);
    assert!(!output.contains("SELECT"), "Should not contain uppercase SELECT");
}

#[test]
fn test_keyword_case_preserve() {
    let config = FormatConfig { keyword_case: KeywordCase::Preserve, ..Default::default() };
    let input = "Select id From users";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, config).format();
    assert!(output.contains("Select"), "Original casing should be preserved: {:?}", output);
}
```

**Step 2: 运行测试确认失败**

Run: `cargo test test_indent_width_4 test_keyword_case`
Expected: FAIL（`with_config` 方法不存在，关键字大小写未转换）

**Step 3: 修改 TokenFormatter 结构体**

将 TokenFormatter 改为持有 `FormatConfig`：

```rust
pub struct TokenFormatter<'a> {
    source: &'a str,
    tokens: Vec<TokenWithSpan>,
    pos: usize,
    indent_stack: Vec<IndentKind>,
    needs_line: bool,
    output: String,
    config: FormatConfig,
}

impl<'a> TokenFormatter<'a> {
    /// Create formatter with default configuration (backward compatible)
    pub fn new(source: &'a str, tokens: Vec<TokenWithSpan>) -> Self {
        Self::with_config(source, tokens, FormatConfig::default())
    }

    /// Create formatter with custom configuration
    pub fn with_config(source: &'a str, tokens: Vec<TokenWithSpan>, config: FormatConfig) -> Self {
        Self {
            source,
            tokens,
            pos: 0,
            indent_stack: Vec::new(),
            needs_line: false,
            output: String::new(),
            config,
        }
    }
    // ... rest unchanged for now
}
```

**Step 4: 修改 `emit_indent` 使用配置**

```rust
fn emit_indent(&mut self) {
    let spaces = self.indent_stack.len() * self.config.indent_width;
    for _ in 0..spaces {
        self.output.push(' ');
    }
}
```

**Step 5: 添加关键字大小写转换**

在 `emit_current_token` 中，当 token 是关键字时，根据 `config.keyword_case` 转换：

```rust
fn emit_current_token(&mut self) {
    let tws = &self.tokens[self.pos];
    let text = &self.source[tws.span.start..tws.span.end];

    // Apply keyword casing transformation
    if let Token::Keyword(kw) = &tws.token {
        let transformed = match self.config.keyword_case {
            KeywordCase::Preserve => text.to_string(),
            KeywordCase::Upper => text.to_uppercase(),
            KeywordCase::Lower => text.to_lowercase(),
        };
        self.output.push_str(&transformed);
    } else {
        self.output.push_str(text);
    }
}
```

同样，`emit_default_token` 中的关键字也需要同样处理——提取一个 `transform_keyword` 辅助方法统一处理。

**Step 6: 处理 `uppercase_keywords` 向后兼容**

在 `with_config` 中加入兼容逻辑：
```rust
pub fn with_config(source: &'a str, tokens: Vec<TokenWithSpan>, mut config: FormatConfig) -> Self {
    // Legacy compat: uppercase_keywords=true overrides keyword_case
    if config.uppercase_keywords {
        config.keyword_case = KeywordCase::Upper;
    }
    // ...
}
```

**Step 7: 运行全部测试**

Run: `cargo test`
Expected: ALL pass（含原有测试，new() 默认配置 = Preserve = 保持原行为）

**Step 8: 提交**

```
refactor(formatter): integrate FormatConfig into TokenFormatter, add keyword casing
```

---

## Part B: SELECT / DML 结构化格式化

### Task 3: SELECT 子句换行 — 列列表

**Files:**
- Modify: `src/token_formatter.rs`

**Step 1: 写测试**

```rust
#[test]
fn test_select_columns_each_on_new_line() {
    let config = FormatConfig { select_newline: true, ..Default::default() };
    let input = "SELECT id, name, age FROM users WHERE id = 1";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, config).format();
    // Each column should be on its own line after SELECT
    assert!(output.contains("SELECT\n  id,\n  name,\n  age\nFROM"), "Columns on new lines: {:?}", output);
}

#[test]
fn test_select_columns_inline() {
    let config = FormatConfig { select_newline: false, ..Default::default() };
    let input = "SELECT id, name FROM users";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, config).format();
    // Columns stay on same line
    let compact: String = output.chars().filter(|c| !c.is_whitespace()).collect();
    let input_compact: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!(compact, input_compact);
}
```

**Step 2: 运行测试确认失败**

Run: `cargo test test_select_columns`
Expected: FAIL（当前 SELECT 后不换行）

**Step 3: 实现 SELECT 列表格式化**

核心思路：在 `handle_token` 中，当遇到 `SELECT` 关键字后进入"列列表模式"。在该模式下，每个逗号后面的 token 都换行缩进。

添加状态到 TokenFormatter：
```rust
struct TokenFormatter<'a> {
    // ... existing fields ...
    /// Track if we're inside a SELECT column list (between SELECT and FROM/WHERE/etc.)
    in_select_list: bool,
    /// Parenthesis nesting depth (for subquery detection)
    paren_depth: usize,
}
```

在 `handle_token` 中：
- `SELECT` → 设置 `in_select_list = true`，换行，输出 SELECT，缩进+1
- 在 `in_select_list` 中遇到 `FROM` → 设置 `in_select_list = false`，缩进-1，正常处理 FROM
- 在 `in_select_list` 中遇到逗号 → 输出逗号，换行+缩进
- 在 `in_select_list` 中遇到 `(` → `paren_depth += 1`（进入子查询或函数）
- 在 `in_select_list` 中遇到 `)` → `paren_depth -= 1`

当 `config.select_newline == false` 时，跳过所有列列表换行逻辑。

**Step 4: 运行测试**

Run: `cargo test test_select_columns`
Expected: PASS

**Step 5: 提交**

```
feat(formatter): SELECT column list formatting with per-column newlines
```

---

### Task 4: WHERE / AND / OR 换行

**Files:**
- Modify: `src/token_formatter.rs`

**Step 1: 写测试**

```rust
#[test]
fn test_where_and_or_newline() {
    let config = FormatConfig { logical_operator_newline: true, ..Default::default() };
    let input = "SELECT id FROM users WHERE a = 1 AND b = 2 OR c = 3";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, config).format();
    assert!(output.contains("WHERE a = 1\n  AND b = 2\n  OR c = 3"),
        "AND/OR on new lines: {:?}", output);
}

#[test]
fn test_where_and_or_inline() {
    let config = FormatConfig { logical_operator_newline: false, ..Default::default() };
    let input = "SELECT id FROM users WHERE a = 1 AND b = 2";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, config).format();
    let compact: String = output.chars().filter(|c| !c.is_whitespace()).collect();
    assert!(compact.contains("WHEREa=1ANDb=2"));
}
```

**Step 2: 运行测试确认失败**

**Step 3: 实现 AND/OR 换行**

在 `handle_token` 中添加对 `AND` / `OR` 关键字的特殊处理：

```rust
Token::Keyword(Keyword::AND) | Token::Keyword(Keyword::OR) => {
    if self.config.logical_operator_newline && self.paren_depth == 0 {
        self.emit_line_start();
        self.emit_current_token();
    } else {
        self.emit_default_token();
    }
    self.pos += 1;
}
```

注意：只在顶层（paren_depth == 0）换行，嵌套括号内的 AND/OR 不换行。

**Step 4: 运行测试**

Run: `cargo test test_where_and_or`
Expected: PASS

**Step 5: 提交**

```
feat(formatter): AND/OR logical operator newline formatting
```

---

### Task 5: JOIN 格式化

**Files:**
- Modify: `src/token_formatter.rs`

**Step 1: 写测试**

```rust
#[test]
fn test_join_formatting() {
    let input = "SELECT a.id FROM users a JOIN orders b ON a.id = b.user_id LEFT JOIN products c ON b.product_id = c.id";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, FormatConfig::default()).format();
    // Each JOIN should be on a new line
    assert!(output.contains("\nJOIN orders") || output.contains("JOIN orders"), "JOIN on new line");
    assert!(output.contains("ON a.id = b.user_id"), "ON condition preserved");
}
```

**Step 2: 实现**

在 `handle_token` 中识别 JOIN 组合关键字：

```rust
// JOIN type keywords: INNER JOIN, LEFT JOIN, RIGHT JOIN, FULL JOIN, CROSS JOIN, LEFT OUTER JOIN, etc.
Token::Keyword(Keyword::INNER) | Token::Keyword(Keyword::LEFT) |
Token::Keyword(Keyword::RIGHT) | Token::Keyword(Keyword::FULL) |
Token::Keyword(Keyword::CROSS) | Token::Keyword(Keyword::JOIN) => {
    // If this is the start of a JOIN clause (not inside parens), newline
    if self.in_from_context() && self.paren_depth == 0 {
        self.emit_line_start();
        self.emit_current_token();
        self.pos += 1;
    } else {
        self.emit_default_token();
    }
}
```

需要追踪"FROM 上下文"——在 FROM 之后、WHERE/GROUP BY/ORDER BY 之前。

**Step 3: 运行测试，提交**

```
feat(formatter): JOIN clause formatting with newlines
```

---

### Task 6: 逗号位置控制（Leading/Trailing）

**Files:**
- Modify: `src/token_formatter.rs`

**Step 1: 写测试**

```rust
#[test]
fn test_comma_trailing() {
    let config = FormatConfig { comma_style: CommaStyle::Trailing, select_newline: true, ..Default::default() };
    let input = "SELECT id, name, age FROM t";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, config).format();
    // Trailing: comma at end of line
    assert!(output.contains("id,\n  name,\n  age"), "Trailing comma: {:?}", output);
}

#[test]
fn test_comma_leading() {
    let config = FormatConfig { comma_style: CommaStyle::Leading, select_newline: true, ..Default::default() };
    let input = "SELECT id, name, age FROM t";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, config).format();
    // Leading: comma at start of next line
    assert!(output.contains("id\n  , name\n  , age"), "Leading comma: {:?}", output);
}
```

**Step 2: 实现**

在逗号处理逻辑中，根据 `config.comma_style` 决定逗号位置：

```rust
Token::Comma => {
    match self.config.comma_style {
        CommaStyle::Trailing => {
            self.emit_current_token(); // comma stays
            if self.in_select_list && self.config.select_newline {
                self.needs_line = true; // newline after comma
            } else {
                self.emit_space();
            }
        }
        CommaStyle::Leading => {
            // Newline first, then comma at start of next line
            if self.in_select_list && self.config.select_newline {
                self.needs_line = true;
                self.flush_pending_line();
                self.emit_current_token(); // comma at start
                self.emit_space();
            } else {
                self.emit_current_token();
                self.emit_space();
            }
        }
    }
    self.pos += 1;
}
```

**Step 3: 运行测试，提交**

```
feat(formatter): comma position control (trailing/leading)
```

---

### Task 7: 子查询缩进

**Files:**
- Modify: `src/token_formatter.rs`

**Step 1: 写测试**

```rust
#[test]
fn test_subquery_indentation() {
    let input = "SELECT * FROM (SELECT id FROM users WHERE active = 1) AS subq";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, FormatConfig::default()).format();
    // Subquery SELECT should be indented
    assert!(output.contains("(\n    SELECT") || output.contains("(\n  SELECT"),
        "Subquery indented: {:?}", output);
}
```

**Step 2: 实现**

在遇到 `LParen` 后紧跟 `SELECT` 时，增加缩进层级。遇到对应的 `RParen` 时恢复。

```rust
Token::LParen => {
    self.emit_current_token();
    self.pos += 1;
    // Check if next token is SELECT (subquery)
    if let Some(Token::Keyword(Keyword::SELECT)) = self.peek_token(0) {
        self.indent_stack.push(IndentKind::Select);
        self.needs_line = true;
    }
}
Token::RParen => {
    // If we entered subquery mode, pop indent
    if self.indent_stack.last() == Some(&IndentKind::Select) {
        self.pop_indent_to(IndentKind::Select);
        self.emit_line_start();
    }
    self.emit_current_token();
    self.pos += 1;
}
```

**Step 3: 运行测试，提交**

```
feat(formatter): subquery indentation support
```

---

## Part C: DDL 格式化

### Task 8: CREATE TABLE 列定义格式化

**Files:**
- Modify: `src/token_formatter.rs`

**Step 1: 写测试**

```rust
#[test]
fn test_create_table_columns() {
    let input = "CREATE TABLE users (id INTEGER PRIMARY KEY, name VARCHAR(100) NOT NULL, age INTEGER)";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, FormatConfig::default()).format();
    // Each column definition should be on its own line
    assert!(output.contains("(\n  id INTEGER PRIMARY KEY"), "First column: {:?}", output);
    assert!(output.contains(",\n  name VARCHAR(100) NOT NULL"), "Second column: {:?}", output);
    assert!(output.contains(",\n  age INTEGER\n)"), "Last column: {:?}", output);
}

#[test]
fn test_create_table_with_constraints() {
    let input = "CREATE TABLE t (id INT, CONSTRAINT pk_t PRIMARY KEY (id), FOREIGN KEY (id) REFERENCES other(id))";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, FormatConfig::default()).format();
    // Table-level constraints should also be on new lines
    assert!(output.contains("CONSTRAINT pk_t"), "Constraint preserved: {:?}", output);
}
```

**Step 2: 实现**

添加 `in_create_table` 状态追踪。当 `CREATE TABLE ... (` 后进入列定义模式：

```rust
/// Track if we're inside CREATE TABLE column list
in_create_table_body: bool,
```

逻辑：
- `CREATE` + `TABLE` → 标记即将进入 CREATE TABLE 上下文
- `(` 在 CREATE TABLE 上下文中 → `in_create_table_body = true`，换行+缩进
- 逗号在 `in_create_table_body` 中 → 换行
- `)` 在 `in_create_table_body` 中 → `in_create_table_body = false`，缩进-1，换行

**Step 3: 运行测试，提交**

```
feat(formatter): CREATE TABLE column definition formatting
```

---

### Task 9: CREATE FUNCTION / PROCEDURE 格式化

**Files:**
- Modify: `src/token_formatter.rs`

**Step 1: 写测试**

```rust
#[test]
fn test_create_function_formatting() {
    let input = "CREATE OR REPLACE FUNCTION my_func(p1 INTEGER) RETURNS INTEGER IS BEGIN RETURN p1 + 1; END";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, FormatConfig::default()).format();
    assert!(output.contains("RETURNS INTEGER\nIS"), "IS/AS on new line: {:?}", output);
    assert!(output.contains("BEGIN\n  RETURN p1 + 1;\nEND"), "BEGIN/END formatted: {:?}", output);
}

#[test]
fn test_create_procedure_formatting() {
    let input = "CREATE PROCEDURE my_proc(p1 IN INTEGER) IS v_x INTEGER; BEGIN v_x := p1; END";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, FormatConfig::default()).format();
    assert!(output.contains("IS\n  v_x INTEGER;"), "Declarations indented: {:?}", output);
    assert!(output.contains("BEGIN\n  v_x := p1;\nEND"), "Body indented: {:?}", output);
}
```

**Step 2: 实现**

在 `handle_token` 中对 `CREATE` + `FUNCTION` / `PROCEDURE` 组合检测，然后在 `IS` / `AS` 关键字处换行。

已有的 `is_procedure_or_function_context()` 方法可以复用。增强 `IS`/`AS` 处理，使其在 CREATE FUNCTION/PROCEDURE 的声明段结束后换行进入 BEGIN。

**Step 3: 运行测试，提交**

```
feat(formatter): CREATE FUNCTION/PROCEDURE formatting
```

---

## Part D: PL/pgSQL 格式化增强

### Task 10: 增强现有 PL/pgSQL 格式化

**Files:**
- Modify: `src/token_formatter.rs`

当前已有的 PL/pgSQL 格式化覆盖了 BEGIN/END、IF/THEN、LOOP。需要增强：

**Step 1: 写测试**

```rust
#[test]
fn test_while_loop() {
    let input = "WHILE x > 0 LOOP x := x - 1; END LOOP";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, FormatConfig::default()).format();
    assert!(output.contains("WHILE x > 0 LOOP\n  x := x - 1;\nEND LOOP"), "WHILE LOOP: {:?}", output);
}

#[test]
fn test_for_loop() {
    let input = "FOR i IN 1..10 LOOP x := x + i; END LOOP";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, FormatConfig::default()).format();
    assert!(output.contains("FOR i IN 1..10 LOOP\n  x := x + i;\nEND LOOP"), "FOR LOOP: {:?}", output);
}

#[test]
fn test_forall() {
    let input = "FORALL i IN INDICES OF arr EXECUTE IMMEDIATE 'INSERT INTO t VALUES(:1)' USING arr(i)";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, FormatConfig::default()).format();
    assert!(output.contains("FORALL i IN"), "FORALL preserved: {:?}", output);
}

#[test]
fn test_case_expression() {
    let input = "CASE WHEN x > 0 THEN 'positive' WHEN x < 0 THEN 'negative' ELSE 'zero' END";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, FormatConfig::default()).format();
    assert!(output.contains("CASE\n  WHEN x > 0 THEN\n    'positive'"), "CASE WHEN: {:?}", output);
}

#[test]
fn test_return_statement() {
    let input = "BEGIN RETURN x + 1; END";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, FormatConfig::default()).format();
    assert!(output.contains("BEGIN\n  RETURN x + 1;\nEND"), "RETURN in block: {:?}", output);
}

#[test]
fn test_raise_statement() {
    let input = "BEGIN RAISE NOTICE 'value is %', x; END";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, FormatConfig::default()).format();
    assert!(output.contains("RAISE NOTICE"), "RAISE preserved: {:?}", output);
}
```

**Step 2: 实现**

在 `handle_token` 中增强：

1. **WHILE / FOR**：当 `LOOP` 紧跟在 `WHILE` / `FOR` 条件之后时（已有 LOOP 处理，只需确认 WHILE/FOR 的换行）
2. **CASE WHEN**：`CASE` 推入 `IndentKind::Case`，`WHEN` 换行
3. **FORALL**：类似 LOOP 处理
4. **RETURN / RAISE / EXECUTE / PERFORM**：在 PL 块内作为独立语句换行

```rust
Token::Keyword(Keyword::WHILE) | Token::Keyword(Keyword::FOR) => {
    self.emit_line_start();
    self.emit_current_token();
    self.pos += 1;
    // Continue emitting condition until LOOP is hit
}

Token::Keyword(Keyword::CASE) => {
    self.emit_space();
    self.emit_current_token();
    self.indent_stack.push(IndentKind::Case);
    self.pos += 1;
    self.needs_line = true;
}

Token::Keyword(Keyword::RETURN) | Token::Keyword(Keyword::RAISE) |
Token::Keyword(Keyword::EXECUTE) | Token::Keyword(Keyword::PERFORM) => {
    self.emit_line_start();
    self.emit_current_token();
    self.pos += 1;
}
```

**Step 3: 运行测试，提交**

```
feat(formatter): enhanced PL/pgSQL formatting (WHILE, FOR, CASE, RETURN, RAISE, EXECUTE)
```

---

## Part E: CLI & API 集成

### Task 11: CLI format 子命令参数

**Files:**
- Modify: `src/bin/ogsql.rs`

**Step 1: 修改 Commands enum**

将 `Format` 变体改为带子命令参数：

```rust
#[derive(Subcommand)]
enum Commands {
    /// Format SQL statements with configurable style / 格式化 SQL 语句
    Format {
        /// Indentation width (default: 2)
        #[arg(short = 'i', long, default_value_t = 2)]
        indent: usize,

        /// Keyword casing: preserve, upper, lower (default: preserve)
        #[arg(short = 'k', long, default_value = "preserve")]
        keyword_case: String,

        /// Comma style: trailing, leading (default: trailing)
        #[arg(long, default_value = "trailing")]
        comma: String,

        /// Maximum line width (default: 120, 0 = unlimited)
        #[arg(short = 'w', long, default_value_t = 120)]
        line_width: usize,

        /// Uppercase all keywords (shorthand for --keyword-case upper)
        #[arg(short = 'u', long)]
        uppercase: bool,

        /// Put each SELECT column on its own line
        #[arg(long, default_value_t = true)]
        select_newline: bool,

        /// Put AND/OR on new lines
        #[arg(long, default_value_t = true)]
        logical_newline: bool,

        /// Put semicolons on their own line
        #[arg(long, default_value_t = true)]
        semicolon_newline: bool,
    },
    // ... other commands unchanged
}
```

**Step 2: 修改 cmd_format**

```rust
fn cmd_format(cli: &Cli, indent: usize, keyword_case: &str, comma: &str,
              line_width: usize, uppercase: bool, select_newline: bool,
              logical_newline: bool, semicolon_newline: bool) {
    let sql = read_input(cli.file.as_deref());
    let mut tokenizer = Tokenizer::new(&sql).preserve_comments(true);
    if cli.mybatis { tokenizer = tokenizer.mybatis_params(true); }
    let tokens = match tokenizer.tokenize() {
        Ok(t) => t,
        Err(e) => die!("Tokenization error: {}", e),
    };

    let kw_case = match keyword_case.to_lowercase().as_str() {
        "upper" | "uppercase" => KeywordCase::Upper,
        "lower" | "lowercase" => KeywordCase::Lower,
        _ => KeywordCase::Preserve,
    };
    let comma_style = match comma.to_lowercase().as_str() {
        "leading" | "start" => CommaStyle::Leading,
        _ => CommaStyle::Trailing,
    };

    let config = FormatConfig {
        indent_width: indent,
        keyword_case: if uppercase { KeywordCase::Upper } else { kw_case },
        comma_style,
        line_width,
        uppercase_keywords: uppercase,
        semicolon_newline,
        select_newline,
        logical_operator_newline: logical_newline,
    };

    let formatted = token_formatter::TokenFormatter::with_config(&sql, tokens, config).format();
    if cli.json {
        let out = serde_json::json!({ "formatted": formatted });
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        println!("{}", formatted);
        if !formatted.ends_with('\n') { println!(); }
    }
}
```

**Step 3: 更新 main dispatch**

在 `match cli.command` 中更新 `Format` 分支以传入所有参数。

**Step 4: 手动验证**

```bash
echo "select id, name from users where a = 1 and b = 2" | cargo run -- format --keyword-case upper
echo "select id, name from users" | cargo run -- format --indent 4
echo "select id, name, age from users" | cargo run -- format --comma leading
```

**Step 5: 提交**

```
feat(cli): format command with configurable indent, keyword case, comma style, line width
```

---

### Task 12: MCP format 接口切换到 TokenFormatter + 配置

**Files:**
- Modify: `src/mcp/mod.rs`

**Step 1: 修改 FormatParams**

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct FormatParams {
    /// SQL text to format
    pub sql: String,
    /// Indentation width (default: 2)
    #[serde(default = "default_indent_width")]
    pub indent_width: usize,
    /// Keyword casing: "preserve", "upper", "lower" (default: "preserve")
    #[serde(default = "default_keyword_case")]
    pub keyword_case: String,
    /// Comma style: "trailing", "leading" (default: "trailing")
    #[serde(default)]
    pub comma_style: String,
    /// Maximum line width (default: 120)
    #[serde(default = "default_line_width")]
    pub line_width: usize,
    /// Uppercase all keywords (default: false)
    #[serde(default)]
    pub uppercase: bool,
}

fn default_indent_width() -> usize { 2 }
fn default_keyword_case() -> String { "preserve".to_string() }
fn default_line_width() -> usize { 120 }
```

**Step 2: 修改 format 工具实现**

从 SqlFormatter 切换到 TokenFormatter：

```rust
#[tool(description = "Format SQL with configurable indentation, keyword casing, and style options")]
fn format(
    &self,
    Parameters(params): Parameters<FormatParams>,
) -> String {
    let tokens = match crate::Tokenizer::new(&params.sql).preserve_comments(true).tokenize() {
        Ok(t) => t,
        Err(e) => return format!("{{\"error\": \"{}\"}}", e),
    };

    let kw_case = match params.keyword_case.to_lowercase().as_str() {
        "upper" | "uppercase" => crate::token_formatter::KeywordCase::Upper,
        "lower" | "lowercase" => crate::token_formatter::KeywordCase::Lower,
        _ => crate::token_formatter::KeywordCase::Preserve,
    };
    let comma = match params.comma_style.to_lowercase().as_str() {
        "leading" | "start" => crate::token_formatter::CommaStyle::Leading,
        _ => crate::token_formatter::CommaStyle::Trailing,
    };

    let config = crate::token_formatter::FormatConfig {
        indent_width: params.indent_width,
        keyword_case: if params.uppercase { crate::token_formatter::KeywordCase::Upper } else { kw_case },
        comma_style: comma,
        line_width: params.line_width,
        uppercase_keywords: params.uppercase,
        ..Default::default()
    };

    let formatted = crate::token_formatter::TokenFormatter::with_config(&params.sql, tokens, config).format();

    serde_json::to_string_pretty(&serde_json::json!({
        "formatted": formatted,
    })).unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
}
```

**Step 3: 运行测试，提交**

```
feat(mcp): format tool uses TokenFormatter with configurable options, preserves comments
```

---

### Task 13: HTTP API format 端点增强

**Files:**
- Modify: `src/bin/ogsql.rs`（handle_format 函数和 SqlInput 结构体）

**Step 1: 扩展输入结构体**

```rust
#[derive(Deserialize)]
pub struct FormatInput {
    pub sql: String,
    #[serde(default = "default_indent")]
    pub indent_width: usize,
    #[serde(default)]
    pub keyword_case: String,
    #[serde(default)]
    pub comma_style: String,
    #[serde(default = "default_line_width")]
    pub line_width: usize,
    #[serde(default)]
    pub uppercase: bool,
}
```

**Step 2: 修改 handle_format**

同样从 SqlFormatter 切换到 TokenFormatter + FormatConfig。逻辑与 MCP 类似。

**Step 3: 提交**

```
feat(http): format endpoint uses TokenFormatter with configurable options
```

---

## Part F: 行宽限制 & 自动换行

### Task 14: 行宽检测与自动换行

**Files:**
- Modify: `src/token_formatter.rs`

**Step 1: 写测试**

```rust
#[test]
fn test_line_width_wrap_select_columns() {
    let config = FormatConfig { line_width: 40, select_newline: false, ..Default::default() };
    let input = "SELECT very_long_column_name_1, very_long_column_name_2, very_long_column_name_3 FROM users";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, config).format();
    // Output lines should not exceed 40 chars (approximately)
    for line in output.lines() {
        // Allow some tolerance for unavoidable overflows (identifiers longer than limit)
        assert!(line.len() <= 60, "Line too long ({}): {}", line.len(), line);
    }
}

#[test]
fn test_line_width_unlimited() {
    let config = FormatConfig { line_width: 0, select_newline: false, ..Default::default() };
    let input = "SELECT a, b, c, d, e, f, g, h, i, j FROM very_long_table_name";
    let tokens = crate::Tokenizer::new(input).preserve_comments(true).tokenize().unwrap();
    let output = TokenFormatter::with_config(input, tokens, config).format();
    // Should be single line (no wrapping)
    let compact: String = output.chars().filter(|c| c != '\n').collect();
    assert!(compact.len() > 50, "Should not wrap when line_width=0");
}
```

**Step 2: 实现行宽检测**

在输出每个 token 之前，检查当前行长度是否即将超过 `config.line_width`。如果超过且 token 可以安全换行（如逗号后、SELECT 列表中），则插入换行。

```rust
fn current_line_length(&self) -> usize {
    self.output
        .rfind('\n')
        .map(|pos| self.output.len() - pos - 1)
        .unwrap_or(self.output.len())
}

fn would_exceed_line_width(&self, token_text: &str) -> bool {
    if self.config.line_width == 0 { return false; }
    self.current_line_length() + token_text.len() > self.config.line_width
}
```

行宽检测仅作为"软换行提示"——不影响结构化格式化（如 BEGIN/END 缩进）。仅在 `select_newline == false` 模式下作为补充。

**Step 3: 运行测试，提交**

```
feat(formatter): line width detection with soft wrapping for long lines
```

---

## Part G: 导出与文档

### Task 15: 导出 FormatConfig 和枚举类型

**Files:**
- Modify: `src/lib.rs`

```rust
pub use token_formatter::{FormatConfig, KeywordCase, CommaStyle};
```

**Step 1: 提交**

```
feat: export FormatConfig, KeywordCase, CommaStyle from library root
```

---

## 执行顺序

```
Task 1  (FormatConfig 定义)        ← 无依赖，最先
Task 2  (FormatConfig 集成)        ← 依赖 Task 1
Task 3  (SELECT 列换行)           ← 依赖 Task 2
Task 4  (WHERE/AND/OR 换行)       ← 依赖 Task 2
Task 5  (JOIN 格式化)             ← 依赖 Task 2
Task 6  (逗号位置)                ← 依赖 Task 2, Task 3
Task 7  (子查询缩进)              ← 依赖 Task 2
Task 8  (CREATE TABLE)            ← 依赖 Task 2
Task 9  (CREATE FUNCTION/PROC)    ← 依赖 Task 2
Task 10 (PL/pgSQL 增强)           ← 依赖 Task 2
Task 11 (CLI 参数)                ← 依赖 Task 1-10
Task 12 (MCP 接口)                ← 依赖 Task 11
Task 13 (HTTP 接口)               ← 依赖 Task 11
Task 14 (行宽限制)                ← 依赖 Task 2
Task 15 (导出)                    ← 依赖 Task 1
```

**建议并行分组：**

- **第一波**: Task 1 → Task 2（串行，基础）
- **第二波**: Task 3, 4, 5, 6, 7, 8, 9, 10, 14, 15（全部并行，独立特性）
- **第三波**: Task 11（CLI 集成，需要所有特性完成）
- **第四波**: Task 12, 13（API 集成，并行）

---

## 验收标准

格式化以下 SQL 时，输出应与 pgFormatter 风格一致：

```sql
-- 输入
SELECT id,name,age FROM users WHERE active=1 AND age>18 ORDER BY name

-- 输出 (--keyword-case upper --indent 2)
SELECT
  id,
  name,
  age
FROM users
WHERE active = 1
  AND age > 18
ORDER BY name;
```

```sql
-- 输入
CREATE TABLE users (id INTEGER PRIMARY KEY, name VARCHAR(100) NOT NULL, email VARCHAR(255) UNIQUE)

-- 输出
CREATE TABLE users (
  id INTEGER PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  email VARCHAR(255) UNIQUE
);
```

```sql
-- 输入
CREATE OR REPLACE FUNCTION calc_tax(p_amount IN NUMBER) RETURN NUMBER IS v_tax NUMBER; BEGIN v_tax := p_amount * 0.1; RETURN v_tax; END;

-- 输出
CREATE OR REPLACE FUNCTION calc_tax(p_amount IN NUMBER) RETURN NUMBER
IS
  v_tax NUMBER;
BEGIN
  v_tax := p_amount * 0.1;
  RETURN v_tax;
END;
```
