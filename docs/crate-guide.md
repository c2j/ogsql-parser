# OGSQL Parser Crate 开发者指南

本文档面向将 ogsql-parser 作为 Rust crate 使用，或需要程序化处理其 AST JSON 输出的开发者。

---

## 目录

- [1. 快速开始](#1-快速开始)
  - [1.1 添加依赖](#11-添加依赖)
  - [1.2 Feature 特性](#12-feature-特性)
- [2. API 使用](#2-api-使用)
  - [2.1 基本解析](#21-基本解析)
  - [2.2 解析 PL/pgSQL](#22-解析-plpgsql)
  - [2.3 解析选项](#23-解析选项)
  - [2.4 SQL 格式化](#24-sql-格式化)
  - [2.5 JSON 往返转换](#25-json-往返转换)
  - [2.6 SQL 校验](#26-sql-校验)
  - [2.7 iBatis/MyBatis XML 解析](#27-ibatismybatis-xml-解析)
  - [2.8 Java 源文件 SQL 提取](#28-java-源文件-sql-提取)
  - [2.9 AST Visitor 模式](#29-ast-visitor-模式)
  - [2.10 语义分析](#210-语义分析)
- [3. AST JSON 参考](#3-ast-json-参考)
- [4. 错误与警告类型参考](#4-错误与警告类型参考)
  - [4.1 错误与警告分类](#41-错误与警告分类)
  - [4.2 解析错误 (ParserError)](#42-解析错误-parsererror)
  - [4.3 分词错误 (TokenizerError)](#43-分词错误-tokenizererror)
  - [4.4 解析警告](#44-解析警告)
- [5. SQL Lint (库 API)](#5-sql-lint-库-api)
- [6. 语义校验规则](#6-语义校验规则)
  - [6.1 PL 变量校验](#61-pl-变量校验)
  - [6.2 MERGE 语义校验](#62-merge-语义校验)
  - [6.3 包一致性校验](#63-包一致性校验)

---

## 1. 快速开始

### 1.1 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
ogsql-parser = "0.6.18"

# 按需启用 feature
# ogsql-parser = { version = "0.6.18", features = ["ibatis"] }     # iBatis XML 解析
# ogsql-parser = { version = "0.6.18", features = ["java"] }       # Java 源文件支持
# ogsql-parser = { version = "0.6.18", features = ["ibatis", "java"] }  # 同时启用
```

### 1.2 Feature 特性

| Feature | 说明 | 额外依赖 |
|---------|------|----------|
| `ibatis` | iBatis/MyBatis XML Mapper 解析 | quick-xml, walkdir |
| `java` | Java 源文件 SQL 提取 | tree-sitter, tree-sitter-java, walkdir |

---

## 2. API 使用

### 2.1 基本解析

```rust
use ogsql_parser::{Tokenizer, Parser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sql = "SELECT id, name FROM users WHERE status = 'active'";

    // 1. 分词
    let tokens = Tokenizer::new(sql).tokenize()?;

    // 2. 解析
    let statements = Parser::new(tokens).parse()?;

    println!("解析了 {} 条语句", statements.len());

    for stmt in &statements {
        println!("{:?}", stmt);
    }

    Ok(())
}
```

### 2.2 解析 PL/pgSQL

```rust
use ogsql_parser::{Tokenizer, Parser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sql = "DO $$ BEGIN RAISE NOTICE 'hello'; END $$";
    let tokens = Tokenizer::new(sql).tokenize()?;
    let statements = Parser::new(tokens).parse()?;

    println!("解析了 {} 条语句", statements.len());
    Ok(())
}
```

### 2.3 解析选项

```rust
use ogsql_parser::{Tokenizer, Parser, ParseOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sql = "SELECT * FROM users WHERE id = #{userId}";

    let tokens = Tokenizer::new(sql).tokenize()?;
    let options = ParseOptions {
        mybatis: true,        // 启用 MyBatis 占位符支持
        ..Default::default()
    };
    let statements = Parser::new(tokens).parse_with_options(options)?;

    Ok(())
}
```

### 2.4 SQL 格式化

**使用 `SqlFormatter`（简单接口）：**

```rust
use ogsql_parser::{Tokenizer, Parser, SqlFormatter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sql = "select id,name from users where id=1";
    let tokens = Tokenizer::new(sql).tokenize()?;
    let statements = Parser::new(tokens).parse()?;

    let formatter = SqlFormatter::new();
    for stmt in &statements {
        let formatted = formatter.format_statement(stmt);
        println!("{}", formatted);
    }

    Ok(())
}
```

**使用 `FormatConfig`（精细控制）：**

```rust
use ogsql_parser::{Tokenizer, Parser, token_formatter::{FormatConfig, KeywordCase, CommaStyle}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sql = "select id,name from users where id=1";
    let tokens = Tokenizer::new(sql).tokenize()?;
    let statements = Parser::new(tokens).parse()?;

    let config = FormatConfig {
        indent: 4,
        keyword_case: KeywordCase::Upper,
        comma_style: CommaStyle::Leading,
        line_width: 100,
        ..Default::default()
    };

    for stmt in &statements {
        let formatted = config.format_statement(stmt);
        println!("{}", formatted);
    }

    Ok(())
}
```

### 2.5 JSON 往返转换

所有 AST 类型实现了 `serde::Serialize` 和 `serde::Deserialize`，支持 SQL → JSON → SQL 的无损语义往返。

```rust
use ogsql_parser::{Tokenizer, Parser, SqlFormatter, Statement};
use serde_json;

fn roundtrip(sql: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 1. SQL → AST
    let tokens = Tokenizer::new(sql).tokenize()?;
    let stmts = Parser::new(tokens).parse()?;

    // 2. AST → JSON
    let json = serde_json::to_string(&stmts)?;

    // 3. JSON → AST
    let restored: Vec<Statement> = serde_json::from_str(&json)?;

    // 4. AST → SQL
    let formatter = SqlFormatter::new();
    let output: Vec<String> = restored.iter()
        .map(|s| formatter.format_statement(s))
        .collect();

    Ok(output.join(";\n"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // DML 往返
    let sql = "SELECT id, name FROM users WHERE status = 'active'";
    assert_eq!(roundtrip(sql)?, "SELECT id, name FROM users WHERE status = 'active'");

    // 特殊字面量类型被保留
    let sql2 = "SELECT E'\\ttext', B'1010', X'FF', N'unicode'";
    assert_eq!(roundtrip(sql2)?, "SELECT E'\\ttext', B'1010', X'FF', N'unicode'");

    // DDL 往返
    let sql3 = "CREATE TABLE t (id INTEGER PRIMARY KEY, name VARCHAR(100) NOT NULL)";
    let result = roundtrip(sql3)?;
    assert!(result.contains("CREATE TABLE"));

    println!("所有往返测试通过！");
    Ok(())
}
```

**往返转换保留的内容：**
- 所有 SQL 语义（运算符、表达式、子查询、连接）
- 特殊字面量类型：转义字符串 (`E'...'`)、位串 (`B'...'`)、十六进制串 (`X'...'`)、国际化字符串 (`N'...'`)、美元符号引用字符串 (`$$...$$`)
- 带参数的类型转换（`NUMERIC(10,2)`、`VARCHAR(100)`、`TIMESTAMP(3) WITH TIME ZONE`）
- 窗口函数帧规范（`ROWS BETWEEN ... AND ...`）
- PL/pgSQL 控制流
- DDL 列类型、约束、域定义

**往返转换不保留的内容：**
- 注释和空白（非语义信息）
- 原始关键字大小写（格式化器默认转为大写）
- 原始格式/布局

### 2.6 SQL 校验

```rust
use ogsql_parser::{Tokenizer, Parser};

fn validate_sql(sql: &str) -> Result<(), String> {
    match Tokenizer::new(sql).tokenize() {
        Ok(tokens) => match Parser::new(tokens).parse() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("解析错误: {:?}", e)),
        },
        Err(e) => Err(format!("分词错误: {:?}", e)),
    }
}
```

**区分错误和警告：**

使用 `parse_sql` 一次性方法可以获取完整的错误列表和语句列表：

```rust
use ogsql_parser::Parser;

let (statements, errors) = Parser::parse_sql("SELECT * FROM t");

for err in &errors {
    match err {
        ParserError::Warning { message, .. } => {
            println!("警告: {}", message);
        }
        ParserError::ReservedKeywordAsIdentifier { keyword, .. } => {
            println!("警告: 保留关键字 '{}' 被用作标识符", keyword);
        }
        _ => {
            println!("错误: {}", err);
        }
    }
}
```

**MERGE 语义校验：**

```rust
use ogsql_parser::{Parser, validate_merge_semantics};

let (stmts, _) = Parser::parse_sql("MERGE INTO target t USING source s ON t.id = s.id WHEN MATCHED THEN DELETE");
for stmt in &stmts {
    if let Some(errors) = validate_merge_semantics(&stmt.statement) {
        for err in errors {
            println!("MERGE 错误: {:?}", err.kind);
        }
    }
}
```

**PL 变量校验：**

```rust
use ogsql_parser::{Parser, validate_pl_variables_with_extra_vars_and_funcs};

let (stmts, _) = Parser::parse_sql("CREATE FUNCTION test() RETURNS VOID AS $$ BEGIN PERFORM my_func(); END $$ LANGUAGE plpgsql");

for si in &stmts {
    if let ogsql_parser::ast::Statement::CreateFunction(f) = &si.statement {
        if let Some(ref block) = f.block {
            let extra_funcs: Vec<&str> = vec!["my_func"];  // 自定义已知函数
            let errors = validate_pl_variables_with_extra_vars_and_funcs(
                block, &f.parameters, &[], &extra_funcs, false,
            );
            for err in &errors {
                println!("未定义变量: {} (位置: {:?})", err.name, err.location);
            }
        }
    }
}
```

**包一致性校验：**

```rust
use ogsql_parser::{Parser, validate_package_consistency};

let (stmts, _) = Parser::parse_sql("CREATE PACKAGE pkg AS ... END; CREATE PACKAGE BODY pkg AS ... END;");
let errors = validate_package_consistency(&stmts);
for err in &errors {
    println!("包不一致: {} — {:?}", err.package_name, err.kind);
}
```

**综合校验 (validate_statements)：**

一次性运行 PACKAGE 一致性、MERGE 语义、PL 变量校验，保留 typed errors：

```rust
use ogsql_parser::{Parser, validate_statements};

let (stmts, _) = Parser::parse_sql("MERGE INTO t USING s ON t.id = s.id WHEN MATCHED THEN DELETE");
let report = validate_statements(&stmts, &[], /* strict */ false);

println!("PACKAGE 错误: {}", report.package_errors.len());
println!("MERGE 错误: {}", report.merge_errors.len());
println!("PL 变量错误: {}", report.undefined_variable_errors.len());

if report.is_empty() {
    println!("校验通过");
}
```

`extra_funcs` 参数可注入外部已知函数名，避免跨包调用被误报为未定义：

```rust
let extra: Vec<String> = vec!["external_func".into()];
let report = validate_statements(&stmts, &extra, true);
```

### 2.7 iBatis/MyBatis XML 解析

需要启用 `ibatis` feature。

```rust
use ogsql_parser::ibatis;

fn main() {
    let xml = br#"<mapper namespace="com.example.UserMapper">
        <select id="findById" parameterType="com.example.User">
            SELECT * FROM users WHERE id = #{id} AND name = #{name}
        </select>
        <insert id="insertUser">
            INSERT INTO users (id, name) VALUES (#{id}, #{name})
        </insert>
    </mapper>"#;

    let result = ibatis::parse_mapper_bytes(xml);

    for stmt in &result.statements {
        println!("语句 ID: {}", stmt.id);
        println!("SQL: {}", stmt.flat_sql);
        for param in &stmt.parameters {
            println!("  参数: {} -> {:?}", param.name, param.jdbc_type);
        }
    }
}
```

**结合 Java 源码推断参数类型（需要同时启用 `ibatis` 和 `java` feature）：**

```rust
use ogsql_parser::ibatis;
use std::path::PathBuf;

fn main() {
    let xml = br#"<mapper namespace="com.example.UserMapper">
        <select id="findById" parameterType="com.example.User">
            SELECT * FROM users WHERE id = #{id}
        </select>
    </mapper>"#;

    let result = ibatis::parse_mapper_bytes_with_java_src(
        xml,
        None,
        vec![PathBuf::from("/project/src/main/java")]
    );

    for stmt in &result.statements {
        println!("{}: {}", stmt.id, stmt.flat_sql);
        for param in &stmt.parameters {
            println!("  {} -> {:?}", param.name, param.jdbc_type);
        }
    }
}
```

### 2.8 Java 源文件 SQL 提取

需要启用 `java` feature。

```rust
use ogsql_parser::java;

fn main() {
    let source = r#"
        public class UserDao {
            public void findUser() {
                String sql = "SELECT * FROM users WHERE id = 1";
                jdbcTemplate.query(sql);
            }
        }
    "#;

    // 提取并解析嵌入的 SQL
    // 具体用法请参考 java 模块的 API 文档
}
```

### 2.9 AST Visitor 模式

OGSQL Parser 提供了 Visitor 模式用于遍历 AST。

```rust
use ogsql_parser::{
    Tokenizer, Parser, Statement, Expr,
    visitor::{Visitor, VisitorResult, walk_statement, walk_pl_statement},
};

// 自定义 Visitor：统计所有函数调用
struct FunctionCollector {
    functions: Vec<String>,
}

impl Visitor for FunctionCollector {
    fn visit_expr(&mut self, expr: &Expr) -> VisitorResult {
        match expr {
            Expr::FunctionCall { name, .. } => {
                let full_name = name.join(".");
                self.functions.push(full_name);
            }
            _ => {}
        }
        VisitorResult::Continue
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sql = "SELECT count(*), max(score) FROM users GROUP BY dept";
    let tokens = Tokenizer::new(sql).tokenize()?;
    let statements = Parser::new(tokens).parse()?;

    let mut collector = FunctionCollector { functions: vec![] };
    for stmt in &statements {
        walk_statement(&mut collector, stmt);
    }

    println!("发现的函数: {:?}", collector.functions);
    // 输出: 发现的函数: ["count", "max"]

    Ok(())
}
```

### 2.10 语义分析

```rust
use ogsql_parser::{
    Tokenizer, Parser,
    validate_pl_variables, validate_merge_semantics,
    compute_query_fingerprints, analyze_transactions,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sql = "MERGE INTO target t USING source s ON t.id = s.id
               WHEN MATCHED THEN UPDATE SET t.name = s.name
               WHEN NOT MATCHED THEN INSERT (id, name) VALUES (s.id, s.name)";

    let tokens = Tokenizer::new(sql).tokenize()?;
    let statements = Parser::new(tokens).parse()?;

    // MERGE 语义校验
    for stmt in &statements {
        if let Some(errors) = validate_merge_semantics(stmt) {
            for err in &errors {
                println!("MERGE 错误: {:?}", err);
            }
        }
    }

    // 计算 SQL 指纹（用于 SQL 唯一性识别）
    let fingerprints = compute_query_fingerprints(&statements);
    for fp in &fingerprints {
        println!("指纹: {}", fp);
    }

    Ok(())
}
```

---

## 3. AST JSON 参考

`ogsql parse -j` 输出的 JSON 结构详见 [AST JSON Reference](./ast-json-reference.md)，涵盖：

- 顶层结构 (`statements`, `errors`)
- `StatementInfo` 格式
- 序列化约定（enum tagging, `ObjectName`, 可选字段）
- 150+ 种语句变体及关键字段
- 表达式节点 (`Expr`) 完整参考及 JSON 形态
- `FunctionCall` 与 `SpecialFunction` 的区别
- 字面量类型 (`Literal`)
- 数据类型 (`DataType`)
- 表引用 (`TableRef`)
- 常见使用模式（识别函数调用、表引用、列引用等）

**重要提示：** 遍历 AST 查找所有函数调用时，需同时处理 `FunctionCall` 和 `SpecialFunction` 两种节点类型。

---

## 4. 错误与警告类型参考

### 4.1 错误与警告分类

OGSQL Parser 将诊断信息分为两类：

| 分类 | 来源 | 说明 |
|------|------|------|
| **错误 (Error)** | 语法/语义检查 | 阻止正常解析或影响 SQL 正确性的问题 |
| **警告 (Warning)** | 语法/语义检查 | 可能是问题但不阻止解析，或使用不推荐写法的提示 |

**判断标准：** `ParserError` 的以下变体被视为警告，其余为错误：

- `ParserError::Warning` — 显式警告信息（如嵌套深度超限、包不一致）
- `ParserError::ReservedKeywordAsIdentifier` — 保留关键字被用作标识符

### 4.2 解析错误 (ParserError)

`ParserError` 是解析阶段的核心错误类型，共 6 个变体：

| 变体 | 说明 | 输出示例 |
|------|------|----------|
| `UnexpectedToken` | 解析到不符合预期的 Token | `unexpected token at line 1, column 8: expected FROM, got *` |
| `UnexpectedEof` | 语句不完整，提前到达文件末尾 | `unexpected end of input at line 5, column 1: expected )` |
| `Warning` | 警告信息（不阻止解析） | `nesting depth exceeded 256 — skipping` |
| `ReservedKeywordAsIdentifier` | 保留关键字被用作标识符 | `reserved keyword "select" cannot be used as identifier at line 3, column 5` |
| `TokenizerError` | 词法分析错误（见 4.3） | `unterminated string literal at position 42` |
| `UnsupportedSyntax` | 不支持的语法（语义校验产生） | `unsupported syntax at line 10, column 1: MERGE (GaussDB does not support ...)` |

**位置信息：** 所有变体（除 `TokenizerError` 外）都包含 `SourceLocation`，包括 `line` 和 `column`（均为 1-based）。

### 4.3 分词错误 (TokenizerError)

分词在解析之前进行，`TokenizerError` 包含 6 个变体：

| 变体 | 说明 |
|------|------|
| `UnterminatedString` | 字符串未闭合（如 `'unterminated`） |
| `UnterminatedComment` | 块注释 `/* ... */` 未闭合 |
| `UnterminatedDollarString` | 美元符引用字符串 `$$...` 未闭合 |
| `UnterminatedQuotedIdentifier` | 引号标识符 `"name` 未闭合 |
| `InvalidCharacter` | 输入中包含无效字符 |
| `UnexpectedEof` | 在期望更多输入时到达文件末尾 |

> **注意：** 分词错误通过 `TokenizerError` 上的 `#[from]` 宏自动转换为 `ParserError::TokenizerError`，因此即使分词阶段出错，解析器仍能统一收集并报告。

### 4.4 解析警告

除 `ParserError::Warning` 和 `ReservedKeywordAsIdentifier` 外，以下情况也会产生警告：

| 来源 | 说明 |
|------|------|
| 嵌套深度超限 | 表达式/语句嵌套超过 256 层时，解析器跳过后续 |
| 包不一致 | PACKAGE 与 PACKAGE BODY 中定义的子程序不匹配（见 6.3） |
| 保留关键字 | 在标识符位置使用了 SQL 保留关键字 |
| PL 变量未定义 | PL 块中引用未声明的变量（见 6.1） |

---

## 5. SQL Lint (库 API)

```rust
use ogsql_parser::linter::{SqlLinter, LintConfig, WarningLevel, Confidence};
use ogsql_parser::Parser;

// 使用默认配置创建 linter
let config = LintConfig::default();
let linter = SqlLinter::with_default_rules(config);

// 解析 SQL
let (stmts, _) = Parser::parse_sql("SELECT * FROM users");

// 运行 lint
let warnings = linter.lint(&stmts, None, None);
for w in &warnings {
    println!("[{}] {}: {}", w.level, w.rule_id, w.message);
    if let Some(suggestion) = &w.suggestion {
        println!("  建议: {}", suggestion);
    }
}

// 生成摘要
let summary = ogsql_parser::linter::build_lint_summary(&warnings);
println!("{}", serde_json::to_string_pretty(&summary).unwrap());
```

**结构化 Mapper Lint（foreach C018）** `[requires: ibatis feature]`：

扁平 SQL 会将 `<foreach>` 塌缩为单次迭代，导致 C018 漏检。`lint_structured_mapper` 在 `SqlNode` 树（展开前）上运行，检测 INSERT VALUES 中的 foreach 动态批量插入：

```rust
use ogsql_parser::ibatis;
use ogsql_parser::linter::structured::lint_structured_mapper;
use ogsql_parser::linter::LintConfig;

let xml = br#"<mapper namespace="t">
    <insert id="batch">
        INSERT INTO t (a, b, c, d, e) VALUES
        <foreach collection="rows" item="r" separator=",">
            (#{r.a}, #{r.b}, #{r.c}, #{r.d}, #{r.e})
        </foreach>
    </insert>
</mapper>"#;

let structured = ibatis::parse_mapper_bytes_structured(xml);
let mut config = LintConfig::default();
config.max_insert_values_rows = 1000; // 阈值：总绑定参数上限
config.foreach_estimated_rows = 1000;  // foreach 集合估算行数

let warnings = lint_structured_mapper(&structured, &config);
for w in &warnings {
    println!("[{}] {}", w.rule_id, w.message);
}
```

**从 TOML 文件加载配置：**

```rust
use ogsql_parser::linter::LintConfig;

// 自动查找 .ogsql-lint.toml
if let Ok(Some(config)) = LintConfig::find_and_load() {
    let linter = ogsql_parser::linter::SqlLinter::with_default_rules(config);
    // ...
}

// 从指定文件加载
let config = LintConfig::load_from_file(std::path::Path::new("./.ogsql-lint.toml"))?;
```

**结合 Schema 和索引信息的 Lint：**

```rust
use ogsql_parser::analyzer::schema::{SchemaMap, IndexMapV2};
use ogsql_parser::linter::IndexInfo;
use std::collections::{HashMap, HashSet};

// 加载 schema
let schema: SchemaMap = /* ... */;

// 构建索引信息
let mut column_indexes: HashMap<String, HashSet<String>> = HashMap::new();
// ... 填充索引信息 ...
let index_info = IndexInfo {
    indexes: IndexMapV2::new(),
    column_indexes,
};

// 运行带 schema 的 lint（R005, R006, R007, S007 等规则会更精确）
let warnings = linter.lint(&stmts, Some(&schema), Some(&index_info));
```

> 完整的 Lint 规则清单和 CLI 使用方式，请参见 [用户指南](./user-guide.md#11-sql-反模式检测-lint)。

---

## 6. 语义校验规则

除语法解析外，OGSQL Parser 提供多种语义校验，在 `validate` 命令和库 API 中可用。

### 6.1 PL 变量校验

检测 PL/pgSQL 块中引用了未声明的变量或函数。

**正常模式（默认）：** 只检查变量是否已声明（检测 `DECLARE` 块中定义的变量）。

**严格模式（`--strict`）：** 同时检查函数调用是否在已知函数列表中。已知函数包括：
- OGSQL 内置的 449 个函数（通过 `function_registry` 注册）
- 文件中已 `CREATE FUNCTION` / `CREATE PROCEDURE` 定义的函数
- 文件中 PACKAGE / PACKAGE BODY 中定义的子程序

`UndefinedVariableError` 包含以下字段：
- `name` — 未定义的变量/函数名
- `location` — 出现位置
- `kind` — `Variable` 或 `Function`

### 6.2 MERGE 语义校验

检测 MERGE 语句的 GaussDB 兼容性问题：

| 种类 | 说明 |
|------|------|
| `DeleteNotSupported` | GaussDB 不支持 `MERGE ... WHEN MATCHED THEN DELETE` |
| `OnColumnUpdated` | GaussDB 不允许在 WHEN MATCHED 中更新 ON 子句引用的列 |

### 6.3 包一致性校验

检查 PACKAGE 与 PACKAGE BODY 定义的一致性：

| 种类 | 说明 |
|------|------|
| `MissingInBody` | PACKAGE 中声明了子程序，但 BODY 中缺少实现 |
| `ExtraInBody` | BODY 中实现了子程序，但 PACKAGE 中未声明 |
| `SignatureMismatch` | 子程序签名（参数类型/数量）不一致 |
| `CursorMismatch` | PACKAGE 中声明的游标在 BODY 中不匹配 |

---

## 相关文档

- [用户指南](./user-guide.md) — 面向 CLI/HTTP/MCP 终端用户
- [AST JSON 参考](./ast-json-reference.md) — `ogsql parse -j` 输出的完整 JSON 结构参考
- [贡献指南](../CONTRIBUTING.md) — 面向项目贡献者的架构与开发指南
