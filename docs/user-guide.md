# OGSQL Parser 用户指南

> 版本 0.6.6 | 适用于 openGauss / GaussDB SQL 解析器

---

## 目录

- [1. 简介](#1-简介)
- [2. 安装与构建](#2-安装与构建)
  - [2.1 环境要求](#21-环境要求)
  - [2.2 编译](#22-编译)
  - [2.3 Feature 特性](#23-feature-特性)
- [3. 命令行工具 (CLI)](#3-命令行工具-cli)
  - [3.1 总览](#31-总览)
  - [3.2 parse — 解析 SQL](#32-parse--解析-sql)
  - [3.3 tokenize — 分词](#33-tokenize--分词)
  - [3.4 format — 格式化](#34-format--格式化)
  - [3.5 validate — 语法校验](#35-validate--语法校验)
  - [3.6 json2sql — JSON 还原 SQL](#36-json2sql--json-还原-sql)
  - [3.7 parse-xml — 解析 iBatis/MyBatis XML](#37-parse-xml--解析-ibatismybatis-xml)
  - [3.8 parse-java — 提取 Java 中的 SQL](#38-parse-java--提取-java-中的-sql)
  - [3.9 serve — HTTP API 服务器](#39-serve--http-api-服务器)
  - [3.10 playground — 交互式终端](#310-playground--交互式终端)
- [4. MCP 服务器](#4-mcp-服务器)
  - [4.1 构建与启动](#41-构建与启动)
  - [4.2 可用工具](#42-可用工具)
  - [4.3 配置 Claude Desktop](#43-配置-claude-desktop)
- [5. 作为 Rust 库使用](#5-作为-rust-库使用)
  - [5.1 添加依赖](#51-添加依赖)
  - [5.2 基本解析](#52-基本解析)
  - [5.3 格式化 SQL](#53-格式化-sql)
  - [5.4 JSON 往返转换](#54-json-往返转换)
  - [5.5 SQL 校验](#55-sql-校验)
  - [5.6 iBatis/MyBatis XML 解析](#56-ibatismybatis-xml-解析)
  - [5.7 Java 源文件 SQL 提取](#57-java-源文件-sql-提取)
  - [5.8 AST Visitor 模式](#58-ast-visitor-模式)
  - [5.9 语义分析](#59-语义分析)
- [6. AST JSON 参考](#6-ast-json-参考)
  - [6.1 顶层结构](#61-顶层结构)
  - [6.2 语句类型 (Statement)](#62-语句类型-statement)
  - [6.3 表达式 (Expr)](#63-表达式-expr)
  - [6.4 字面量类型 (Literal)](#64-字面量类型-literal)
  - [6.5 数据类型 (DataType)](#65-数据类型-datatype)
  - [6.6 表引用 (TableRef)](#66-表引用-tableref)
  - [6.7 函数调用](#67-函数调用)
- [7. 支持的 SQL 语法](#7-支持的-sql-语法)
  - [7.1 DML 语句](#71-dml-语句)
  - [7.2 DDL 语句](#72-ddl-语句)
  - [7.3 PL/pgSQL 语句](#73-plpgsql-语句)
  - [7.4 其他语句](#74-其他语句)
- [8. 多字符集支持](#8-多字符集支持)
- [9. 架构概览](#9-架构概览)
- [10. 常见问题 (FAQ)](#10-常见问题-faq)

---

## 1. 简介

OGSQL Parser 是一个使用 Rust 编写的 SQL 解析器，专为 openGauss / GaussDB（基于 PostgreSQL 的企业级数据库）设计。它采用手写递归下降解析技术，不依赖任何解析器生成器，提供以下核心能力：

- **SQL 解析**：将 SQL 文本解析为抽象语法树 (AST)，支持 150+ 种语句类型
- **SQL 格式化**：可配置缩进、关键字大小写、逗号风格、行宽等
- **SQL 校验**：检测语法错误并报告详细的错误位置
- **分词 (Tokenize)**：将 SQL 拆解为 Token 序列，支持 717 个关键字
- **JSON 往返转换**：SQL → AST → JSON → AST → SQL 的无损语义转换
- **PL/pgSQL 支持**：完整的 PL/pgSQL 过程化语言解析，包括控制流、游标、异常处理等
- **iBatis/MyBatis XML 解析**：从 XML Mapper 文件中提取 SQL，支持参数类型推断
- **Java 源文件 SQL 提取**：从 Java 代码中提取并解析嵌入的 SQL 语句
- **多字符集支持**：自动检测和转换 UTF-8、EUC-JP、EUC-KR、GB18030、BIG5、UTF-16
- **MCP 服务器**：作为 AI 工具服务器，与 Claude Desktop、Cursor 等 AI 编辑器集成

该项目提供了多种使用方式：命令行工具、Rust 库、HTTP API 服务、交互式终端以及 MCP 服务器。

---

## 2. 安装与构建

### 2.1 环境要求

- **Rust 1.70+** 及 Cargo
- （可选）Git，用于克隆仓库

### 2.2 编译

```bash
# 克隆仓库
git clone <repository-url>
cd ogsql-parser

# 默认编译（仅 CLI）
cargo build --release

# 编译完成后，二进制文件位于 target/release/ogsql
```

### 2.3 Feature 特性

OGSQL Parser 使用 Cargo Feature 来控制可选功能模块。可根据需要选择编译：

| Feature | 说明 | 额外依赖 |
|---------|------|----------|
| `cli` | 命令行工具（默认不含，需显式启用） | clap, walkdir |
| `ibatis` | iBatis/MyBatis XML Mapper 解析 | quick-xml, walkdir |
| `java` | Java 源文件 SQL 提取 | tree-sitter, tree-sitter-java, walkdir |
| `serve` | HTTP API 服务器 | axum, tokio, tower-http, utoipa |
| `tui` | 交互式终端 UI | ratatui, crossterm |
| `mcp` | MCP 服务器（AI 工具集成） | rmcp, schemars, tokio + ibatis + java |
| `full` | 启用所有功能 | 所有上述依赖 |

**编译示例：**

```bash
# 仅编译 CLI
cargo build --release --features cli

# 编译 CLI + HTTP API
cargo build --release --features serve

# 编译 CLI + iBatis XML 解析
cargo build --release --features "cli,ibatis"

# 编译全部功能
cargo build --release --features full

# 编译 MCP 服务器（会自动包含 ibatis 和 java）
cargo build --release --features mcp
```

---

## 3. 命令行工具 (CLI)

需要使用 `--features cli` 或包含 `cli` 的 feature（如 `serve`、`full`）编译。

### 3.1 总览

```
$ ogsql --help

   ██████╗  ██████╗ ███████╗ ██████╗ ██╗      ██████╗  █████╗ ██████╗ ███████╗███████╗██████╗
  ██╔═══██╗██╔════╝ ██╔════╝██╔═══██╗██║      ██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔════╝██╔══██╗
  ██║   ██║██║  ███╗███████╗██║   ██║██║      ██████╔╝███████║██████╔╝███████╗███████╗██████╔╝
  ██║   ██║██║   ██║╚════██║██║   ██║██║      ██╔═══╝ ██╔══██║██╔══██╗╚════██║██╔═══╝ ██╔══██╗
  ╚██████╔╝╚██████╔╝███████║╚██████╔╝███████╗ ██║     ██║  ██║██║  ██║███████║███████╗██║  ██║
   ╚═════╝  ╚═════╝ ╚══════╝ ╚═════╝ ╚══════╝ ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝

ogsql 0.6.7
openGauss/GaussDB SQL Parser

Usage: ogsql [OPTIONS] <COMMAND>

Commands:
  format      格式化 SQL 语句
  parse       解析 SQL 为 AST（抽象语法树）
  json2sql    将 JSON 还原为 SQL
  tokenize    将 SQL 分词为 token 列表
  validate    校验 SQL 语法
  parse-xml   解析 iBatis/MyBatis XML Mapper [需要 ibatis feature]
  parse-java  从 Java 源文件中提取 SQL [需要 java feature]
  serve       启动 HTTP API 服务器 [需要 serve feature]
  playground  启动交互式终端 [需要 tui feature]

Options:
  -f, --file <FILE>     从文件读取 SQL（可多次指定）
  -j, --json            以 JSON 格式输出
  -v, --verbose         详细输出模式
      --comments        输出中包含注释信息
      --mybatis         启用 MyBatis #{param} 和 ${expr} 占位符支持
      --schema-json     指定 schema JSON 文件（用于语义分析）
  -h, --help            显示帮助信息
  -V, --version         显示版本号
```

**全局选项说明：**

| 选项 | 说明 |
|------|------|
| `-f, --file <FILE>` | 从文件读取 SQL 而非标准输入，可多次指定处理多个文件 |
| `-j, --json` | 输出 JSON 格式（适用于 parse、tokenize 等命令） |
| `-v, --verbose` | 启用详细输出，显示更多解析过程信息 |
| `--comments` | 在输出中包含注释信息 |
| `--mybatis` | 启用 MyBatis 占位符支持，格式化时保留 `#{param}` 和 `${expr}` |
| `--schema-json <FILE>` | 指定数据库 schema JSON 文件，用于启用语义分析功能 |

---

### 3.2 parse — 解析 SQL

将 SQL 文本解析为 AST（抽象语法树），输出结构化的语法树信息。

```bash
# 基本用法：从标准输入解析
echo "SELECT id, name FROM users WHERE status = 'active'" | ogsql parse

# 以 JSON 格式输出（推荐用于程序化处理）
echo "SELECT * FROM users" | ogsql parse -j

# 从文件解析
ogsql -f query.sql parse -j

# 解析包含 MyBatis 占位符的 SQL
echo "SELECT * FROM users WHERE id = #{userId}" | ogsql --mybatis parse -j

# 解析 PL/pgSQL
echo "DO \$\$ BEGIN RAISE NOTICE 'hello'; END \$\$" | ogsql parse -j

# 解析多语句文件，输出带详细信息
ogsql -f procedures.sql parse -j -v --comments
```

**JSON 输出结构：**

```json
{
  "statements": [
    {
      "Select": {
        "targets": [...],
        "from": [...],
        "where_clause": {...}
      },
      "sql_text": "SELECT id, name FROM users WHERE status = 'active'",
      "start_line": 1,
      "start_col": 1,
      "end_line": 1,
      "end_col": 52
    }
  ],
  "errors": []
}
```

每个语句包含：
- 语句变体（如 `Select`、`Insert` 等）及其结构化字段
- `sql_text`：原始 SQL 文本
- `start_line`/`start_col`/`end_line`/`end_col`：源码位置信息

---

### 3.3 tokenize — 分词

将 SQL 文本拆分为 Token 序列，显示每个 Token 的类型、值和位置。

```bash
# 基本分词
echo "SELECT * FROM users WHERE id = 1" | ogsql tokenize

# JSON 格式输出
echo "SELECT * FROM users" | ogsql tokenize -j
```

**输出示例（JSON）：**

```json
[
  {
    "token": "SELECT",
    "kind": "Keyword",
    "span": { "start": 0, "end": 6, "line": 1, "col": 1 }
  },
  {
    "token": "*",
    "kind": "Star",
    "span": { "start": 7, "end": 8, "line": 1, "col": 8 }
  }
]
```

---

### 3.4 format — 格式化

将 SQL 文本格式化为标准化的、可读性强的格式。支持 DML、DDL 和 PL/pgSQL。

```bash
# 默认格式化（2 空格缩进，保留原始大小写）
echo "select id,name from users where id=1" | ogsql format

# 关键字大写 + 4 空格缩进
echo "select * from users" | ogsql format -u -i 4

# 使用 leading 逗号风格
echo "SELECT id, name, age FROM users" | ogsql format --comma leading

# 设置行宽限制
echo "SELECT * FROM users" | ogsql format -w 80

# 保留 MyBatis 占位符
echo "SELECT * FROM users WHERE id = #{userId}" | ogsql format --mybatis

# 紧凑模式（不换行）
echo "SELECT id, name FROM users" | ogsql format --no-select-newline --no-logical-newline

# 格式化 DML 语句
echo "update users set name='Bob',age=30 where id=1" | ogsql format
echo "delete from users where id = 1 and status = 'inactive'" | ogsql format
echo "merge into target t using source s on t.id=s.id when matched then update set t.name=s.name" | ogsql format

# 格式化 CTE（WITH 语句）
echo "with cte as (select id from users) select * from cte" | ogsql format

# 从文件读取并格式化
ogsql -f query.sql format -u -i 4
```

**格式化选项详解：**

| 选项 | 默认值 | 说明 |
|------|--------|------|
| `-i, --indent <N>` | `2` | 缩进空格数 |
| `-k, --keyword-case <CASE>` | `preserve` | 关键字大小写：`preserve`（保留原样）、`upper`（大写）、`lower`（小写） |
| `--comma <STYLE>` | `trailing` | 逗号风格：`trailing`（行尾逗号）、`leading`（行首逗号） |
| `-w, --line-width <N>` | `120` | 最大行宽，`0` 表示不限制 |
| `-u, --uppercase` | — | `--keyword-case upper` 的快捷方式 |
| `--no-select-newline` | — | 不将每个 SELECT 列放在单独的行 |
| `--no-logical-newline` | — | 不将 AND/OR 放在新行 |
| `--no-semicolon-newline` | — | 不将分号放在单独的行 |
| `--mybatis` | — | 保留 MyBatis `#{param}` 和 `${expr}` 占位符 |

---

### 3.5 validate — 语法校验

检查 SQL 语法是否正确，报告错误和警告信息。

```bash
# 校验正确 SQL
echo "SELECT * FROM users" | ogsql validate
# 输出: OK

# 校验错误 SQL
echo "SELECT FROM" | ogsql validate
# 输出: 错误信息，包含位置和原因

# 从文件校验
ogsql -f procedures.sql validate
```

---

### 3.6 json2sql — JSON 还原 SQL

将 `parse -j` 生成的 JSON 转换回 SQL 文本。支持从标准输入或文件读取 JSON。

```bash
# 管道往返：SQL → JSON → SQL
echo "SELECT id FROM users WHERE id = 1" | ogsql parse -j | ogsql json2sql

# 从 JSON 文件还原
ogsql -f ast.json json2sql
```

**注意：** 还原后的 SQL 在语义上与原始 SQL 等价，但格式可能不同（注释和原始大小写不会保留）。

---

### 3.7 parse-xml — 解析 iBatis/MyBatis XML

从 iBatis/MyBatis XML Mapper 文件中提取 SQL 语句及其参数信息。需要 `--features ibatis`。

```bash
# 解析单个 XML（从标准输入）
echo '<mapper namespace="test"><select id="find">SELECT * FROM t WHERE id = #{id}</select></mapper>' | ogsql parse-xml

# 解析单个文件，CSV 输出
ogsql parse-xml --csv -f mapper/UserMapper.xml

# 解析单个文件，JSON 输出
ogsql parse-xml -f mapper/UserMapper.xml -j

# 解析目录下所有 XML
ogsql parse-xml --dir ./mapper-xml -j

# 结合 Java 源码进行参数类型推断
ogsql parse-xml -d /path/to/mapper-xml --java-src /path/to/src/main/java --csv

# 指定多个 Java 源码目录
ogsql parse-xml --dir ./mapper-xml --java-src ./src/main/java --java-src ./lib/src -j
```

**输出说明：**

每个提取的 SQL 语句包含：
- `id`：Mapper 语句 ID（对应 XML 中的 `id` 属性）
- `flat_sql`：展开动态标签后的平坦 SQL
- `parameters`：参数列表，包含名称和推断的 JDBC 类型

---

### 3.8 parse-java — 提取 Java 中的 SQL

从 Java 源文件中提取嵌入的 SQL 语句。需要 `--features java`。

```bash
# 从单个 Java 文件提取
ogsql parse-java -f src/main/java/com/example/UserDao.java

# 从目录批量提取
ogsql parse-java --dir src/main/java -j
```

---

### 3.9 serve — HTTP API 服务器

启动 HTTP API 服务器，通过网络接口访问解析功能。需要 `--features serve`。

```bash
# 启动服务器（默认 0.0.0.0:3000）
ogsql serve

# 自定义主机和端口
ogsql serve --host 127.0.0.1 --port 8080
```

**可用 API 端点：**

| 方法 | 端点 | 说明 | 请求体 |
|------|------|------|--------|
| GET | `/api/health` | 健康检查 | — |
| POST | `/api/parse` | 解析 SQL → AST JSON | `{"sql": "..."}` |
| POST | `/api/json2sql` | JSON → SQL | `{"json": "..."}` |
| POST | `/api/format` | 格式化 SQL | `{"sql": "...", "indent": 2, "keyword_case": "upper", ...}` |
| POST | `/api/tokenize` | SQL 分词 | `{"sql": "..."}` |
| POST | `/api/validate` | 校验 SQL | `{"sql": "..."}` |

**请求示例：**

```bash
# 解析 SQL
curl -X POST http://localhost:3000/api/parse \
  -H "Content-Type: application/json" \
  -d '{"sql": "SELECT id, name FROM users WHERE id = 1"}'

# 格式化 SQL
curl -X POST http://localhost:3000/api/format \
  -H "Content-Type: application/json" \
  -d '{"sql": "select id,name from users", "indent": 4, "keyword_case": "upper"}'

# 校验 SQL
curl -X POST http://localhost:3000/api/validate \
  -H "Content-Type: application/json" \
  -d '{"sql": "SELECT FROM"}'
```

**format 请求体参数：**

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `sql` | string | — | 必填，要格式化的 SQL |
| `indent` | number | `2` | 缩进空格数 |
| `keyword_case` | string | `"preserve"` | 关键字大小写：`preserve`、`upper`、`lower` |
| `comma` | string | `"trailing"` | 逗号风格：`trailing`、`leading` |
| `line_width` | number | `120` | 最大行宽 |
| `mybatis` | boolean | `false` | 是否保留 MyBatis 占位符 |

---

### 3.10 playground — 交互式终端

启动交互式终端 UI（TUI），实时输入 SQL 并查看解析结果。需要 `--features tui`。

```bash
ogsql playground
```

在 playground 中可以：
- 实时输入 SQL 语句
- 查看解析后的 AST 结构
- 实时查看格式化结果
- 查看分词结果

---

## 4. MCP 服务器

OGSQL Parser 可作为 MCP (Model Context Protocol) 服务器运行，为 AI 编辑器和助手提供 SQL 解析工具。需要 `--features mcp`。

### 4.1 构建与启动

```bash
# 编译 MCP 服务器
cargo build --release --features mcp

# 运行（使用 stdio 传输，适用于 Claude Desktop、Cursor 等）
./target/release/ogsql-mcp
```

### 4.2 可用工具

MCP 服务器提供以下工具供 AI 助手调用：

| 工具名 | 说明 | 参数 |
|--------|------|------|
| `parse` | 解析 SQL → AST JSON（含指纹、注释、错误） | `sql` |
| `tokenize` | SQL → Token 列表（含类型、值、位置） | `sql` |
| `format` | 格式化 SQL | `sql`, 可选: `indent`, `keyword_case`, `comma`, `line_width` |
| `validate` | 校验 SQL 语法，报告错误/警告 | `sql` |
| `json2sql` | 将 AST JSON 转换回 SQL | `json` |
| `parse_xml` | 解析 iBatis/MyBatis XML Mapper | `xml` 或 `dir`, 可选: `java_src`/`java_sources` |
| `parse_java` | 从 Java 源文件提取 SQL | `path` 或 `dir` |

### 4.3 配置 Claude Desktop

在 Claude Desktop 的配置文件 `claude_desktop_config.json` 中添加：

```json
{
  "mcpServers": {
    "ogsql": {
      "command": "/path/to/ogsql-mcp"
    }
  }
}
```

配置完成后，Claude 即可使用 SQL 解析工具来分析和处理 SQL 语句。

---

## 5. 作为 Rust 库使用

### 5.1 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
ogsql-parser = "0.6.7"

# 按需启用 feature
[features]
# 如果需要 iBatis XML 解析
# ogsql-parser = { version = "0.6.7", features = ["ibatis"] }
# 如果需要 Java 源文件支持
# ogsql-parser = { version = "0.6.7", features = ["java"] }
```

### 5.2 基本解析

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

**解析 PL/pgSQL：**

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

**使用解析选项：**

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

### 5.3 格式化 SQL

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

**使用 Token Formatter（更精细的控制）：**

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

### 5.4 JSON 往返转换

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

### 5.5 SQL 校验

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

### 5.6 iBatis/MyBatis XML 解析

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

### 5.7 Java 源文件 SQL 提取

需要启用 `java` feature。

```rust
use ogsql_parser::java;

fn main() {
    // 从 Java 源文件提取 SQL
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

### 5.8 AST Visitor 模式

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

### 5.9 语义分析

OGSQL Parser 提供多种语义分析功能：

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

## 6. AST JSON 参考

本节描述 `ogsql parse -j` 生成的 JSON 结构，供需要程序化处理 AST 的开发者参考。

### 6.1 顶层结构

```json
{
  "statements": [ ... ],
  "errors": [ ... ]
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `statements` | `StatementInfo[]` | 语句数组，每条 SQL 语句对应一个元素 |
| `errors` | `string[]` | 解析错误/警告数组 |

**StatementInfo 结构：**

```json
{
  "Select": { ... },
  "sql_text": "SELECT ...",
  "start_line": 1,
  "start_col": 1,
  "end_line": 1,
  "end_col": 34
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| 语句变体 | object | 实际语句内容，以变体名（如 `Select`、`Insert`）为键 |
| `sql_text` | string | 原始 SQL 文本 |
| `start_line` / `start_col` | number | 起始位置（1-based） |
| `end_line` / `end_col` | number | 结束位置（1-based） |

### 6.2 语句类型 (Statement)

常见语句变体：

| 变体 | SQL | 关键字段 |
|------|-----|----------|
| `Select` | `SELECT ...` | `targets`, `from`, `where_clause`, `group_by`, `order_by`, `limit`, `with` |
| `Insert` | `INSERT INTO ...` | `table`, `columns`, `source`, `on_conflict`, `returning` |
| `Update` | `UPDATE ... SET ...` | `table`, `assignments`, `from`, `where_clause`, `returning` |
| `Delete` | `DELETE FROM ...` | `table`, `where_clause`, `returning` |
| `Merge` | `MERGE INTO ...` | `target`, `source`, `when_matched`, `when_not_matched` |
| `CreateTable` | `CREATE TABLE ...` | `name`, `columns`, `constraints`, `partition_by` |
| `CreateIndex` | `CREATE INDEX ...` | `name`, `table`, `columns`, `unique` |
| `CreateView` | `CREATE VIEW ...` | `name`, `query` |
| `CreateFunction` | `CREATE FUNCTION ...` | `name`, `parameters`, `return_type`, `block` |
| `CreateProcedure` | `CREATE PROCEDURE ...` | `name`, `parameters`, `block` |
| `Do` | `DO $$ ... $$` | `block`（PL/pgSQL AST） |
| `Drop` | `DROP TABLE/INDEX/...` | `object_type`, `names`, `if_exists`, `cascade` |
| `Transaction` | `BEGIN/COMMIT/ROLLBACK` | `action` |
| `Explain` | `EXPLAIN ...` | `statement`, `analyze`, `verbose`, `format` |
| `Grant` / `Revoke` | `GRANT/REVOKE ...` | `privileges`, `targets`, `grantees` |

### 6.3 表达式 (Expr)

`Expr` 是 SQL 表达式的核心递归类型：

| 变体 | SQL 示例 | JSON 形式 |
|------|----------|-----------|
| `Literal` | `42`, `'hello'` | `{ "Literal": { "Integer": 42 } }` |
| `ColumnRef` | `col`, `t.col` | `{ "ColumnRef": ["col"] }` |
| `BinaryOp` | `a + b` | `{ "BinaryOp": { "left": {...}, "op": "+", "right": {...} } }` |
| `UnaryOp` | `-x`, `NOT x` | `{ "UnaryOp": { "op": "-", "expr": {...} } }` |
| `FunctionCall` | `count(x)` | `{ "FunctionCall": { "name": ["count"], "args": [...] } }` |
| `Case` | `CASE WHEN ...` | `{ "Case": { "operand": null, "whens": [...] } }` |
| `Between` | `x BETWEEN a AND b` | `{ "Between": { "expr": {...}, "low": {...}, "high": {...} } }` |
| `InList` | `x IN (1,2,3)` | `{ "InList": { "expr": {...}, "list": [...] } }` |
| `InSubquery` | `x IN (SELECT ...)` | `{ "InSubquery": { "expr": {...}, "subquery": {...} } }` |
| `Exists` | `EXISTS (SELECT ...)` | `{ "Exists": {...} }` |
| `Subquery` | `(SELECT ...)` | `{ "Subquery": {...} }` |
| `IsNull` | `x IS NULL` | `{ "IsNull": { "expr": {...}, "negated": false } }` |
| `TypeCast` | `x::INT` | `{ "TypeCast": { "expr": {...}, "type_name": {...} } }` |
| `Parameter` | `$1` | `{ "Parameter": 1 }` |
| `Array` | `ARRAY[1,2,3]` | `{ "Array": [...] }` |
| `Parenthesized` | `(a + b)` | `{ "Parenthesized": {...} }` |

### 6.4 字面量类型 (Literal)

| 变体 | SQL | JSON |
|------|-----|------|
| `Integer` | `42` | `{ "Integer": 42 }` |
| `Float` | `3.14` | `{ "Float": "3.14" }` |
| `String` | `'hello'` | `{ "String": "hello" }` |
| `EscapeString` | `E'\ttext'` | `{ "EscapeString": "\\ttext" }` |
| `BitString` | `B'1010'` | `{ "BitString": "1010" }` |
| `HexString` | `X'FF'` | `{ "HexString": "FF" }` |
| `NationalString` | `N'unicode'` | `{ "NationalString": "unicode" }` |
| `DollarString` | `$$ body $$` | `{ "DollarString": { "tag": null, "body": " body " } }` |
| `Boolean` | `TRUE` | `{ "Boolean": true }` |
| `Null` | `NULL` | `"Null"` |

### 6.5 数据类型 (DataType)

| 变体 | SQL | JSON |
|------|-----|------|
| `Boolean` | `BOOLEAN` | `"Boolean"` |
| `SmallInt` | `SMALLINT` | `"SmallInt"` |
| `Integer` | `INTEGER` | `"Integer"` |
| `BigInt` | `BIGINT` | `"BigInt"` |
| `Real` | `REAL` | `"Real"` |
| `Float` | `FLOAT(24)` | `{ "Float": 24 }` |
| `Double` | `DOUBLE PRECISION` | `"Double"` |
| `Numeric` | `NUMERIC(10,2)` | `{ "Numeric": [10, 2] }` |
| `Char` | `CHAR(10)` | `{ "Char": 10 }` |
| `Varchar` | `VARCHAR(100)` | `{ "Varchar": 100 }` |
| `Text` | `TEXT` | `"Text"` |
| `Timestamp` | `TIMESTAMP(3) WITH TIME ZONE` | `{ "Timestamp": [3, { "WithTimeZone": true }] }` |
| `Date` | `DATE` | `"Date"` |
| `Json` / `Jsonb` | `JSON`, `JSONB` | `"Json"` / `"Jsonb"` |
| `Uuid` | `UUID` | `"Uuid"` |
| `Serial` | `SERIAL` | `"Serial"` |
| `Custom` | 自定义类型 | `{ "Custom": [["my_type"], []] }` |

### 6.6 表引用 (TableRef)

```json
// 简单表
{ "Table": { "name": ["users"], "alias": "u" } }

// 子查询
{ "Subquery": { "query": { "Select": {...} }, "alias": "sq" } }

// JOIN
{ "Join": {
    "left": { "Table": { "name": ["users"] } },
    "right": { "Table": { "name": ["orders"] } },
    "join_type": "Left",
    "condition": { "BinaryOp": { "left": {...}, "op": "=", "right": {...} } }
} }
```

JOIN 类型：`"Inner"`、`"Left"`、`"Right"`、`"Full"`、`"Cross"`

### 6.7 函数调用

**FunctionCall：**

```json
{
  "FunctionCall": {
    "name": ["count"],
    "args": [{ "ColumnRef": ["*"] }],
    "distinct": false,
    "filter": null,
    "over": null,
    "within_group": [],
    "_meta": {
      "builtin": true,
      "category": "Aggregate",
      "domain": "Aggregate"
    }
  }
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | `string[]` | 函数名（最后一个是函数名，前面的是 schema 限定符） |
| `args` | `Expr[]` | 参数列表 |
| `distinct` | boolean | 是否有 `DISTINCT` 修饰符 |
| `filter` | `Expr?` | `FILTER (WHERE ...)` 子句 |
| `over` | `WindowSpec?` | 窗口函数 `OVER (...)` 子句 |
| `within_group` | `OrderByItem[]` | `WITHIN GROUP (ORDER BY ...)` 子句 |
| `_meta` | object? | 内置函数元数据（仅内置函数存在） |

**内置函数元数据 (`_meta`)：**

| 字段 | 类型 | 说明 |
|------|------|------|
| `builtin` | boolean | 始终为 `true` |
| `category` | string | `"Aggregate"`、`"Window"`、`"Scalar"`、`"SetReturning"`、`"Special"` |
| `domain` | string | `"Math"`、`"String"`、`"DateTime"`、`"Aggregate"` 等 |

**SpecialFunction：** 部分使用关键字分隔语法的函数（如 `EXTRACT`、`SUBSTRING`、`SUBSTR`）以 `SpecialFunction` 节点表示。遍历 AST 查找所有函数调用时，需同时处理 `FunctionCall` 和 `SpecialFunction` 两种节点。

---

## 7. 支持的 SQL 语法

### 7.1 DML 语句

| 语句 | 说明 |
|------|------|
| `SELECT` | 完整语法，包括 CTE（WITH）、子查询、窗口函数、PIVOT/UNPIVOT |
| `INSERT` | INSERT VALUES、INSERT SELECT、ON CONFLICT |
| `UPDATE` | UPDATE SET、FROM、WHERE、RETURNING |
| `DELETE` | DELETE WHERE、RETURNING |
| `MERGE` | MERGE INTO、WHEN MATCHED / NOT MATCHED |
| `WITH (CTE)` | 通用表表达式，递归 CTE |
| `VALUES` | VALUES 列表 |

### 7.2 DDL 语句

| 语句 | 状态 |
|------|------|
| `CREATE TABLE` | ✅ 已完成（含分区、约束、表选项） |
| `CREATE TABLE AS` | ✅ 已完成 |
| `CREATE INDEX` | ✅ 已完成 |
| `CREATE GLOBAL INDEX` | ✅ 已完成 |
| `CREATE VIEW` | ✅ 已完成 |
| `CREATE FUNCTION` | ✅ 已完成（含 PL/pgSQL 体） |
| `CREATE PROCEDURE` | ✅ 已完成（含 PL/pgSQL 体） |
| `CREATE PACKAGE / PACKAGE BODY` | ✅ 已完成（Oracle 兼容包） |
| `CREATE TRIGGER` | ✅ 已完成 |
| `CREATE SCHEMA` | ✅ 已完成 |
| `CREATE DATABASE` | ✅ 已完成 |
| `CREATE SEQUENCE` | ✅ 已完成 |
| `CREATE TYPE` | ✅ 已完成 |
| `CREATE DOMAIN` | ✅ 已完成 |
| `CREATE CAST` | ✅ 已完成 |
| `DROP TABLE / INDEX / VIEW ...` | ✅ 已完成 |
| `TRUNCATE` | ✅ 已完成 |
| `ALTER TABLE` | 🔄 进行中 |
| `ALTER SYSTEM SET/RESET` | ✅ 已完成 |

### 7.3 PL/pgSQL 语句

| 语句 | 说明 |
|------|------|
| `DO $$ ... $$` | DO 语句 |
| 匿名块 | `DECLARE ... BEGIN ... END` |
| 变量声明 | 变量、常量、游标、RECORD、TYPE、%TYPE、%ROWTYPE |
| 赋值 | `variable := expression` |
| `IF / ELSIF / ELSE` | 条件分支 |
| `CASE` | 搜索型和简单 CASE 表达式 |
| `LOOP / WHILE / FOR / FOREACH` | 所有循环结构（支持标签） |
| `EXIT / CONTINUE` | 带可选 WHEN 条件和标签 |
| `RETURN` | 带可选表达式 |
| `RAISE` | 所有级别：DEBUG、LOG、INFO、NOTICE、WARNING、EXCEPTION |
| `EXECUTE / PERFORM` | 动态 SQL 执行 |
| 游标操作 | OPEN、FETCH、CLOSE、MOVE |
| `GET DIAGNOSTICS` | 包括 STACKED DIAGNOSTICS |
| 异常处理 | `WHEN ... THEN` 处理器 |
| `GOTO` | GOTO 标签 |
| 事务控制 | 块内 COMMIT、ROLLBACK、SAVEPOINT |
| `FORALL / PIPE ROW` | 批量操作 |

### 7.4 其他语句

- `EXPLAIN` / `EXPLAIN ANALYZE`
- `GRANT` / `REVOKE`
- `BEGIN` / `COMMIT` / `ROLLBACK` / `SAVEPOINT`
- `COPY`
- `VACUUM` / `REINDEX` / `CLUSTER`
- `SET` / `SHOW` / `RESET`
- `CALL`
- `PREPARE` / `EXECUTE` / `DEALLOCATE`
- `COMMENT`
- `LOCK`
- `DECLARE CURSOR` / `FETCH` / `CLOSE`
- `CHECKPOINT` / `DISCARD`
- `LISTEN` / `NOTIFY` / `UNLISTEN`
- 以及更多 openGauss/GaussDB 特有语句

---

## 8. 多字符集支持

OGSQL Parser 支持多种字符编码的 SQL 文本，自动检测和转换：

| 编码 | 说明 |
|------|------|
| UTF-8 | 默认编码 |
| EUC-JP | 日语 |
| EUC-KR | 韩语 |
| GB18030 | 中文（包含 GB2312 和 GBK） |
| BIG5 | 繁体中文 |
| UTF-16 | Unicode 双字节 |

使用库 API 时，`Tokenizer` 会自动检测编码并处理。CLI 工具同样自动检测输入文件编码。

---

## 9. 架构概览

```
SQL 输入
    |
    v
┌───────────┐    ┌──────────────┐    ┌───────────┐    ┌──────┐    ┌──────────┐
│  分词器    │ -> │  Token 流    │ -> │   解析器  │ -> │  AST │ -> │ 格式化器 │
│ (Tokenizer)│    │ (Vec<Token>) │    │  (Parser) │    │      │    │(Formatter)│
│           │    │              │    │ (递归下降) │    │      │    │          │
└───────────┘    └──────────────┘    └───────────┘    └──────┘    └──────────┘
                                                          │  ▲
                                                          │  │
                                                    JSON 序列化/反序列化
                                                          │  │
                                                          ▼  │
                                                    ┌──────────┐
                                                    │   JSON   │
                                                    └──────────┘
```

**核心设计原则：**

1. **手写递归下降**：不使用解析器生成器，完全掌控错误消息和错误恢复
2. **Pratt 解析器处理表达式**：自然处理运算符优先级，无左递归问题
3. **尽可能零拷贝**：Token 引用源字符串切片，减少内存分配
4. **多字符集支持**：自动检测和转换多种字符编码
5. **统一解析核心**：所有接口（CLI、HTTP API、TUI、MCP）共享同一解析核心
6. **完整 serde 支持**：所有 AST 类型支持 `Serialize` + `Deserialize`

**模块结构：**

```
src/
├── lib.rs              # 库导出
├── bin/
│   ├── ogsql.rs        # CLI 入口
│   └── ogsql-mcp.rs    # MCP 服务器入口
├── token/
│   ├── mod.rs          # Token 类型和 Span
│   ├── tokenizer.rs    # 词法分析器
│   ├── keyword.rs      # 717 个关键字定义
│   └── encoding.rs     # 多字符集支持
├── ast/
│   ├── mod.rs          # 150+ AST 节点定义
│   ├── plpgsql.rs      # PL/pgSQL AST 类型
│   └── visitor.rs      # AST Visitor 模式
├── parser/
│   ├── mod.rs          # 解析器分发和主逻辑
│   ├── select.rs       # SELECT 解析器
│   ├── dml.rs          # INSERT/UPDATE/DELETE/MERGE
│   ├── ddl/            # DDL 解析器（含子模块）
│   ├── expr.rs         # 表达式解析器（Pratt）
│   ├── plpgsql.rs      # PL/pgSQL 解析器
│   └── utility/        # 工具解析器（类型、约束等）
├── formatter/          # SQL 格式化器
├── token_formatter.rs  # Token 级格式化器
├── analyzer/           # 语义分析
│   ├── mod.rs          # 分析器主模块
│   ├── schema.rs       # Schema 加载和解析
│   └── return_cursor.rs # 返回游标分析
├── ibatis/             # iBatis/MyBatis XML 解析
├── java/               # Java 源文件 SQL 提取
└── mcp/                # MCP 服务器实现
```

---

## 10. 常见问题 (FAQ)

### Q: OGSQL Parser 支持标准 PostgreSQL SQL 吗？

是的。由于 openGauss/GaussDB 基于 PostgreSQL，OGSQL Parser 支持绝大多数 PostgreSQL SQL 语法，同时包含 openGauss/GaussDB 特有的扩展。

### Q: 解析遇到错误时会怎样？

解析器会尽可能继续解析后续语句，将遇到的错误收集到 `errors` 数组中返回，而不是直接中止。这意味着即使部分语句有语法错误，仍然可以获取其他正确解析的语句。

### Q: JSON 往返转换是真正无损的吗？

是语义无损，而非文本无损。往返转换保留所有 SQL 语义信息，但不保留：
- 注释
- 原始关键字大小写
- 原始格式和空白

还原后的 SQL 在语义上与原始 SQL 完全等价。

### Q: 如何处理 MyBatis 的动态 SQL 标签（如 `<if>`、`<foreach>`）？

`parse-xml` 命令会将 MyBatis 动态标签展开为平坦 SQL。对于条件性标签（如 `<if>`），所有分支都会被展开；对于循环标签（如 `<foreach>`），会生成代表性输出。

### Q: 支持哪些 openGauss 版本？

参考的是 openGauss 的 gram.y（35,325 行）和 PL/pgSQL gram.y（15,770 行），涵盖当前 openGauss/GaussDB 的主流语法。所有 1,409 个回归测试均通过。

### Q: 解析性能如何？

使用 Rust 编写，采用零拷贝 Token 和手写递归下降，性能优异。具体数据取决于 SQL 复杂度，但在大多数场景下，单条语句的解析耗时在微秒级别。

### Q: 如何贡献代码？

这是一个活跃开发的项目。欢迎通过 Issue 和 Pull Request 参与贡献。DDL（Phase 4）和性能优化（Phase 6）仍在进行中。

### Q: 许可证是什么？

MIT OR Apache-2.0 双许可，您可以选择其中任一使用。
