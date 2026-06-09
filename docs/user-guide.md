# OGSQL Parser 用户指南

> 版本 0.6.10 | 适用于 openGauss / GaussDB SQL 解析器

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
- [11. SQL 反模式检测 (Lint)](#11-sql-反模式检测-lint)
  - [11.1 快速开始](#111-快速开始)
  - [11.2 命令行选项](#112-命令行选项)
  - [11.3 配置文件](#113-配置文件)
  - [11.4 规则清单](#114-规则清单)
  - [11.5 Rust 库 API](#115-rust-库-api)
- [12. 错误与警告类型参考](#12-错误与警告类型参考)
  - [12.1 错误与警告分类](#121-错误与警告分类)
  - [12.2 解析错误 (ParserError)](#122-解析错误-parsererror)
  - [12.3 分词错误 (TokenizerError)](#123-分词错误-tokenizererror)
  - [12.4 解析警告](#124-解析警告)
  - [12.5 输出格式](#125-输出格式)
- [13. 语义校验规则](#13-语义校验规则)
  - [13.1 PL 变量校验（严格模式）](#131-pl-变量校验严格模式)
  - [13.2 MERGE 语义校验](#132-merge-语义校验)
  - [13.3 包一致性校验](#133-包一致性校验)

---

## 1. 简介

OGSQL Parser 是一个使用 Rust 编写的 SQL 解析器，专为 openGauss / GaussDB（基于 PostgreSQL 的企业级数据库）设计。它采用手写递归下降解析技术，不依赖任何解析器生成器，提供以下核心能力：

- **SQL 解析**：将 SQL 文本解析为抽象语法树 (AST)，支持 180+ 种语句类型
- **SQL 格式化**：可配置缩进、关键字大小写、逗号风格、行宽等
- **SQL 校验**：检测语法错误并报告详细的错误位置
- **分词 (Tokenize)**：将 SQL 拆解为 Token 序列，支持 717 个关键字
- **JSON 往返转换**：SQL → AST → JSON → AST → SQL 的无损语义转换
- **PL/pgSQL 支持**：完整的 PL/pgSQL 过程化语言解析，包括控制流、游标、异常处理等
- **iBatis/MyBatis XML 解析**：从 XML Mapper 文件中提取 SQL，支持参数类型推断
- **Java 源文件 SQL 提取**：从 Java 代码中提取并解析嵌入的 SQL 语句
- **多字符集支持**：自动检测和转换 UTF-8、EUC-JP、EUC-KR、GB18030、BIG5、UTF-16
- **MCP 服务器**：作为 AI 工具服务器，与 Claude Desktop、Cursor 等 AI 编辑器集成
- **Windows 7 支持**：通过 Tier 3 目标 `x86_64-win7-windows-msvc` 支持在 Windows 7 环境运行

该项目提供了多种使用方式：命令行工具、Rust 库、HTTP API 服务、交互式终端以及 MCP 服务器。

---

## 2. 安装与构建

### 2.1 环境要求

- **Rust 1.70+** 及 Cargo
- （可选）Git，用于克隆仓库
- （可选）Rust nightly 工具链，用于 Windows 7 目标构建（`x86_64-win7-windows-msvc`）

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

ogsql 0.6.10
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

检查 SQL 语法是否正确，报告错误和警告信息。支持单文件、目录批量扫描、严格模式和多种输出格式。

```bash
# 校验单条 SQL
echo "SELECT * FROM users" | ogsql validate
# 输出: OK

# 校验错误 SQL
echo "SELECT FROM" | ogsql validate
# 输出: 错误信息，包含位置和原因

# 从文件校验
ogsql -f procedures.sql validate

# 批量扫描目录
ogsql validate -d ./sql-files

# 启用严格模式（检测 PL 块中未定义函数调用）
echo "CREATE OR REPLACE FUNCTION test() RETURNS VOID AS \$\$ BEGIN PERFORM unknown_func(); END; \$\$ LANGUAGE plpgsql" | ogsql validate --strict

# 批量扫描 + CSV 输出 + 统计
ogsql validate -d ./sql-files --csv --stats

# 批量扫描 + JSON 输出 + 统计
ogsql validate -d ./sql-files -j --stats

# 校验并生成错误日志（需 -v）
ogsql -f procedures.sql validate -v

# 配合 --lint 启用 SQL 反模式检测
ogsql validate -d ./sql-files --lint --stats
```

**校验选项：**

| 选项 | 说明 |
|------|------|
| `-d, --dir <DIR>` | 递归扫描目录中的 SQL 文件（可多次指定） |
| `-e, --ext <EXT>` | 扫描的文件扩展名，逗号分隔（默认: `sql`） |
| `--csv` | 以 CSV 格式输出校验结果（每行一条语句） |
| `--stats` | 输出统计摘要（文件数、错误数、语句类型分布等） |
| `--strict` | 启用严格模式：检测 PL 块中未定义的函数调用（详见 [13.1](#131-pl-变量校验严格模式)） |

**校验流程：**

`validate` 命令依次执行以下检查：

1. **分词** — 检查字符串、注释等是否正确闭合
2. **语法解析** — 检查 SQL 语法是否正确，报告解析错误和警告
3. **MERGE 语义校验** — 检查 MERGE 语句的 GaussDB 兼容性（详见 [13.2](#132-merge-语义校验)）
4. **包一致性校验** — 检查 PACKAGE 与 PACKAGE BODY 是否匹配（详见 [13.3](#133-包一致性校验)）
5. **PL 变量校验** — 检查 PL 块中是否使用了未声明的变量（`--strict` 时还检查未定义函数，详见 [13.1](#131-pl-变量校验严格模式)）

**结果分类：**

- **错误 (error)** — 包括 `UnexpectedToken`、`UnexpectedEof`、`TokenizerError`、`UnsupportedSyntax` 等，表示语法层面存在问题
- **警告 (warning)** — 包括 `ParserError::Warning` 和 `ReservedKeywordAsIdentifier`，表示潜在问题但不阻止解析
- **严格模式错误** — PL 块中使用未定义函数时报告（仅 `--strict`）

**退出码：**

- `0` — 所有文件校验通过（可能有警告）
- `1` — 存在错误（`--csv` 或批量模式时）

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
ogsql-parser = "0.6.10"

# 按需启用 feature
[features]
# 如果需要 iBatis XML 解析
# ogsql-parser = { version = "0.6.10", features = ["ibatis"] }
# 如果需要 Java 源文件支持
# ogsql-parser = { version = "0.6.10", features = ["java"] }
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

**区分错误和警告：**

解析器返回的 `ParserError` 包含错误和警告两种。使用 `parse_sql` 一次性方法可以获取完整的错误列表和语句列表：

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
| `CREATE TABLE` | ✅ 已完成（含分区、分布、约束、表选项、ILM策略、压缩） |
| `CREATE TABLE AS` | ✅ 已完成 |
| `CREATE INDEX` | ✅ 已完成 |
| `CREATE GLOBAL INDEX` | ✅ 已完成 |
| `CREATE VIEW` | ✅ 已完成 |
| `CREATE MATERIALIZED VIEW` | ✅ 已完成（含 REFRESH MATERIALIZED VIEW） |
| `CREATE FUNCTION` | ✅ 已完成（含 PL/pgSQL 体、参数、返回类型） |
| `CREATE PROCEDURE` | ✅ 已完成（含 PL/pgSQL 体） |
| `CREATE PACKAGE / PACKAGE BODY` | ✅ 已完成（Oracle 兼容包） |
| `CREATE TRIGGER` | ✅ 已完成 |
| `CREATE SCHEMA` | ✅ 已完成 |
| `CREATE DATABASE` | ✅ 已完成 |
| `CREATE DATABASE LINK` | ✅ 已完成 |
| `CREATE TABLESPACE` | ✅ 已完成 |
| `CREATE SEQUENCE` | ✅ 已完成 |
| `CREATE TYPE` | ✅ 已完成（复合、枚举、范围类型） |
| `CREATE DOMAIN` | ✅ 已完成 |
| `CREATE CAST` | ✅ 已完成 |
| `CREATE EXTENSION` | ✅ 已完成 |
| `CREATE ROLE / USER / GROUP` | ✅ 已完成 |
| `CREATE FOREIGN TABLE / SERVER / FDW` | ✅ 已完成 |
| `CREATE PUBLICATION / SUBSCRIPTION` | ✅ 已完成 |
| `CREATE SYNONYM` | ✅ 已完成（Oracle 兼容） |
| `CREATE AGGREGATE / OPERATOR` | ✅ 已完成 |
| `CREATE MODEL` | ✅ 已完成（AI 模型管理） |
| GaussDB 专有 CREATE | ✅ 已完成（NODE、NODE GROUP、RESOURCE POOL、WORKLOAD GROUP、AUDIT/MASKING/RLS POLICY、DATA SOURCE、EVENT、OP CLASS/FAMILY、STREAM、KEY、DIRECTORY、LANGUAGE、WEAK PASSWORD DICTIONARY、TEXT SEARCH CONFIG/DICT、CONT QUERY、APP WORKLOAD GROUP MAPPING） |
| `ALTER TABLE` | ✅ 已完成 |
| `ALTER INDEX / SEQUENCE / VIEW` | ✅ 已完成 |
| `ALTER FUNCTION / PROCEDURE` | ✅ 已完成 |
| `ALTER SCHEMA / DATABASE / TABLESPACE` | ✅ 已完成 |
| `ALTER ROLE / USER / GROUP` | ✅ 已完成 |
| `ALTER TRIGGER / EXTENSION` | ✅ 已完成 |
| `ALTER FOREIGN TABLE / SERVER / FDW` | ✅ 已完成 |
| `ALTER PUBLICATION / SUBSCRIPTION` | ✅ 已完成 |
| `ALTER TYPE / DOMAIN` | ✅ 已完成 |
| GaussDB 专有 ALTER | ✅ 已完成（NODE、NODE GROUP、RESOURCE POOL、WORKLOAD GROUP、AUDIT/MASKING/RLS POLICY、DATA SOURCE、EVENT、OP FAMILY、OPERATOR、MATERIALIZED VIEW、GLOBAL CONFIG、SESSION、DATABASE LINK、DIRECTORY、LANGUAGE、PACKAGE、COORDINATOR、APP WORKLOAD GROUP MAPPING、SYNONYM、TEXT SEARCH CONFIG/DICT） |
| `DROP` 语句 | ✅ 已完成（支持 30+ 种对象类型） |
| `TRUNCATE` | ✅ 已完成 |
| `COMMENT` | ✅ 已完成 |
| `GRANT / REVOKE` | ✅ 已完成（含角色授权） |

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
- `GRANT` / `REVOKE` / `GRANT ROLE` / `REVOKE ROLE`
- `BEGIN` / `COMMIT` / `ROLLBACK` / `SAVEPOINT`
- `COPY`
- `VACUUM` / `REINDEX` / `CLUSTER` / `ANALYZE`
- `SET` / `SHOW` / `RESET`
- `CALL`
- `PREPARE` / `EXECUTE` / `DEALLOCATE`
- `COMMENT`
- `LOCK`
- `DECLARE CURSOR` / `FETCH` / `CLOSE`
- `CHECKPOINT` / `DISCARD`
- `LISTEN` / `NOTIFY` / `UNLISTEN`
- `SHUTDOWN` / `BARRIER`
- `PURGE` / `TIMECAPSULE` / `SNAPSHOT`
- `SHRINK` / `VERIFY` / `CLEAN CONNECTION`
- `COMPILE`
- `REPLACE`（Oracle 兼容 INSERT OR REPLACE）
- `INSERT ALL / INSERT FIRST`（Oracle 多表插入）
- `PREDICT BY`（AI 预测）
- `EXECUTE DIRECT`（分布式直接执行）
- `SECURITY LABEL`
- `EXPDP DATABASE/TABLE` / `IMPDP DATABASE/TABLE`（数据导入导出）
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
├── linter/             # SQL 反模式检测
│   ├── mod.rs          # Linter 主模块 (SqlWarning, LintConfig)
│   ├── rules_prohibition.rs  # 禁止项规则 (R001-R009)
│   ├── rules_performance.rs  # 性能规则 (P001-P022)
│   ├── rules_caution.rs      # 注意事项规则 (C001-C018)
│   └── rules_suggestion.rs   # 建议规则 (S001-S008)
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

**错误与警告的区别：** 解析器将诊断信息分为错误和警告两类。错误（如 `UnexpectedToken`、`TokenizerError`）表示语法问题；警告（如 `ReservedKeywordAsIdentifier`、`Warning` 变体）表示潜在问题但不阻止解析。详见 [12. 错误与警告类型参考](#12-错误与警告类型参考)。

### Q: validate 命令和 parse 命令有什么不同？

`parse` 专注于输出 AST 结构，适合程序化处理和 JSON 往返转换。`validate` 专为语法校验设计，额外执行以下检查：

1. **MERGE 语义校验** — 检测 GaussDB 不支持的 MERGE 模式
2. **包一致性校验** — 检查 PACKAGE 与 PACKAGE BODY 是否匹配
3. **PL 变量校验** — 检测 PL 块中未声明的变量
4. **严格模式** — `--strict` 下额外检测未定义函数调用

`validate` 更适合 CI/CD 流水线中作为质量门禁使用。

### Q: 如何在 CI/CD 中使用校验功能？

```bash
# 批量校验 SQL 文件目录，CSV 输出 + 统计
ogsql validate -d ./sql-files --csv --stats

# 启用严格校验 + Lint（自动成为质量门禁，有错误时退出码 1）
ogsql validate -d ./sql-files --strict --lint --min-level caution --csv --stats

# 使用自定义 lint 配置
ogsql validate -d ./sql-files --lint --lint-config ./ci/.ogsql-lint.toml --csv --stats
```

有错误时退出码为 `1`，CI 可据此判断是否阻断流水线。警告不会导致非零退出码。

### Q: 什么是严格模式 (--strict)？什么时候应该使用？

`--strict` 模式在正常 PL 变量校验之外，额外检测 PL 块中调用的函数是否在已知函数列表中。已知函数包括：

- OGSQL 内置的 449 个函数（完整 function_registry）
- 同一文件中通过 `CREATE FUNCTION` / `CREATE PROCEDURE` / `PACKAGE` 定义的子程序

**使用场景：**
- **推荐使用** — 当 SQL 文件是完整的功能单元（包含所有依赖定义），严格模式可以捕获拼写错误和缺失依赖
- **谨慎使用** — 当文件引用了外部模块的函数时，严格模式可能产生误报（需要配合 `--schema-json` 或预先扫描依赖文件）

### Q: Lint 规则太多，如何只关注重要的？

使用 `--min-level` 和 `--suppress` 精细化控制：

```bash
# 只报告 Prohibition（禁止项）和 Performance（性能）级别
ogsql validate -d ./sql-files --lint --min-level performance

# 排除特定规则
ogsql validate -d ./sql-files --lint --suppress R001,P012,S006
```

也可以通过配置文件永久设置，见 [11.3 配置文件](#113-配置文件)。

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

这是一个活跃开发的项目。欢迎通过 Issue 和 Pull Request 参与贡献。性能优化（Phase 7）仍在规划中。

### Q: 许可证是什么？

MIT OR Apache-2.0 双许可，您可以选择其中任一使用。

---

## 11. SQL 反模式检测 (Lint)

OGSQL Parser 内置 SQL 静态检测引擎 (`SqlLinter`)，可识别 47 条反模式规则，覆盖**禁止项 (Prohibition)**、**性能 (Performance)**、**注意事项 (Caution)** 和**建议 (Suggestion)** 四个等级。

### 11.1 快速开始

通过 `--lint` 全局选项启用，可配合 `parse`、`validate`、`parse-xml`、`parse-java` 使用：

```bash
# 单文件 lint
echo "SELECT * FROM users" | ogsql parse -j --lint

# 目录批量 lint + 统计
ogsql validate -d ./sql-files --lint --stats

# 只报告 Prohibition 和 Performance 级别
ogsql validate -d ./sql-files --lint --min-level performance

# 只报告 Full confidence 的结果（排除 iBatis 动态 SQL 的 Partial 结果）
ogsql validate -d ./sql-files --lint --min-confidence full

# 禁用指定规则（逗号分隔）
ogsql validate -d ./sql-files --lint --suppress R001,S007

# 使用自定义配置文件（详见 11.3）
ogsql validate -d ./sql-files --lint --lint-config ./.ogsql-lint.toml
```

### 11.2 命令行选项

| 选项 | 值 | 默认 | 说明 |
|------|-----|------|------|
| `--lint` | — | 关闭 | 启用 SQL 反模式检测 |
| `--min-level` | `prohibition`, `performance`, `caution`, `suggestion` | `suggestion` | 最低报告级别 |
| `--min-confidence` | `full`, `partial` | `partial` | 最低可信度（full 排除 iBatis 动态 SQL） |
| `--suppress` | 逗号分隔规则 ID | — | 禁用指定规则（如 `R001,S007`） |
| `--in-list-threshold` | 正整数 | `500` | P003: IN 列表大小阈值 |
| `--subquery-depth-limit` | 正整数 | `3` | P014: 子查询嵌套深度阈值 |
| `--non-equi-join-limit` | 正整数 | `2` | P007: 非等值 JOIN 数量阈值 |
| `--lint-config` | 文件路径 | 自动查找 | 配置文件路径（详见 11.3） |

### 11.3 配置文件

OGSQL Lint 支持通过 `.ogsql-lint.toml` 配置文件自定义规则行为。文件搜索顺序：

1. 当前工作目录下的 `.ogsql-lint.toml`
2. `~/.config/ogsql/lint.toml` (XDG 约定)

**配置示例：**

```toml
# .ogsql-lint.toml
min_level = "caution"       # 只报告 Caution 及以上级别
min_confidence = "full"     # 只报告 Full confidence
suppress = ["R001", "S007"] # 禁用 SELECT * 和字面量类型建议
in_list_threshold = 200     # IN 列表超过 200 个值时报告
subquery_depth_limit = 5    # 子查询嵌套超过 5 层时报告
sql_length_limit = 4096     # SQL 超过 4096 字符时建议拆分（默认值，与 GaussDB track_activity_query_size 对齐）
non_equi_join_limit = 3     # 非等值 JOIN 超过 3 个时报告
group_by_column_limit = 10  # GROUP BY 超过 10 列时报告
max_insert_values_rows = 65535 # INSERT VALUES 行数×列数阈值（默认值，对齐数据库绑定参数上限）
```

配置项的优先级：**命令行参数 > 配置文件 > 默认值**。

### 11.4 规则清单

#### 禁止项 (Prohibition) — 违反 GaussDB 编码规范

| ID | 名称 | 适用语句 | 说明 |
|----|------|----------|------|
| R001 | `select-star` | SELECT | `SELECT *` 违反编码规范，表结构变化时可能导致不兼容 |
| R002 | `large-column-sort` | SELECT | GROUP BY / ORDER BY 超过阈值列的表达式，可能导致性能问题 |
| R003 | `lock-table` | All | `LOCK TABLE` 可能导致死锁风险 |
| R004 | `drop-cascade` | All | `DROP ... CASCADE` 可能误删依赖对象 |
| R005 | `implicit-type-conversion` | SELECT | WHERE 中可能存在隐式类型转换，导致索引失效 |
| R006 | `function-on-where-column` | DML | WHERE 中对有索引的列使用函数或表达式 |
| R007 | `like-leading-wildcard` | DML | `LIKE '%...'` 前导通配符导致无法使用索引，触发全表扫描 |
| R008 | `same-table-column-compare` | DML | 同表列比较：可能未正确使用索引 |
| R009 | `scalar-subquery-in-select` | SELECT | SELECT 列中包含标量子查询，每行都会执行一次子查询 |

#### 性能 (Performance) — 可识别的性能陷阱

| ID | 名称 | 适用语句 | 说明 |
|----|------|----------|------|
| P001 | `union-without-all` | SELECT | `UNION` 未使用 `ALL`，存在不必要的去重排序 |
| P002 | `not-in-subquery` | DML | `NOT IN (子查询)` 性能较差，NULL 值会导致结果不符合预期 |
| P003 | `in-list-too-large` | DML | IN 列表超过阈值，导致解析缓慢 |
| P004 | `or-to-union-all` | DML | WHERE 顶层为 OR 条件，可能导致优化器放弃索引 |
| P005 | `now-function-non-pushable` | DML | `now()` / `sysdate` 不可下推，导致分布式查询性能下降 |
| P006 | `count-star-large-table` | SELECT | `COUNT(*)` 在大表上性能较差 |
| P007 | `too-many-non-equi-joins` | SELECT | 非等值 JOIN 条件过多，性能较差 |
| P008 | `group-by-without-hashagg` | SELECT | GROUP BY 操作未使用 `hash_agg` 提示 |
| P009 | `function-instead-of-case` | DML | `NVL`/`NVL2`/`DECODE`/`IIF` 函数可用 CASE 替代，可能更高效 |
| P010 | `multi-column-update-subquery` | UPDATE | 多列 UPDATE 使用子查询效率较低 |
| P011 | `correlated-subquery` | DML | 关联子查询可能导致每行执行一次子查询 |
| P012 | `unnecessary-distinct` | SELECT | DISTINCT 存在，需结合唯一键判断是否必要 |
| P013 | `cartesian-product` | SELECT | CROSS JOIN 或缺少 JOIN 条件，可能产生笛卡尔积 |
| P014 | `deeply-nested-subquery` | DML | 子查询嵌套深度超过阈值，性能可能较差 |
| P015 | `range-equals-same-value` | DML | `BETWEEN` 上下界相同，应简化为等号条件 |
| P016 | `update-from-no-join-condition` | UPDATE | `UPDATE FROM` 无 WHERE 子句，可能产生笛卡尔积更新 |
| P017 | `merge-without-unique-index` | MERGE | MERGE 语句 ON 条件需要唯一索引保证确定性 |
| P018 | `insert-select-no-columns` | INSERT | `INSERT INTO ... SELECT` 未指定目标列名，依赖列顺序 |
| P019 | `multi-table-update` | UPDATE | 多表 UPDATE，可能产生非预期结果 |
| P020 | `insert-all-multi-table` | All | `INSERT ALL` / `INSERT FIRST` 多表插入 |
| P021 | `row-by-row-insert-in-loop` | PL Block | 循环体内包含 INSERT，应使用 FORALL 批量操作替代 |
| P022 | `explain-in-production` | All | EXPLAIN 语句不应出现在生产代码中 |
| P023 | `connect-by-performance` | All | CONNECT BY 层级查询在数据量大或递归层次深时性能可能显著下降；缺少 START WITH 可能导致全表扫描。考虑使用 WITH RECURSIVE CTE 替代，使用 NOCYCLE 避免死循环 |

#### 注意事项 (Caution) — 合法但需谨慎

| ID | 名称 | 适用语句 | 说明 |
|----|------|----------|------|
| C001 | `hint-unknown` | DML | Hint 不在 GaussDB 已知 Hint 列表中，将被静默忽略 |
| C005 | `hint-contradictory` | DML | Hint 矛盾（如 `tablescan` 与 `indexscan` 同时存在） |
| C006 | `hint-table-not-in-from` | DML | Hint 引用的表不在 FROM 子句中 |
| C007 | `update-without-where` | UPDATE | `UPDATE` 无 WHERE 子句，可能影响全表数据 |
| C008 | `delete-without-where` | DELETE | `DELETE` 无 WHERE 子句，将删除全表数据 |
| C009 | `insert-no-column-list` | INSERT | INSERT 未指定目标列名，依赖表定义顺序 |
| C010 | `unlogged-table` | DDL | `UNLOGGED TABLE` 在故障恢复时数据会丢失 |
| C011 | `goto-statement` | PL Block | PL/pgSQL 中使用了 `GOTO` 语句，不符合结构化编程建议 |
| C012 | `execute-concat-sql-injection` | PL Block | `EXECUTE IMMEDIATE` 中使用字符串拼接，可能存在 SQL 注入风险 |
| C013 | `exception-swallow` | PL Block | `WHEN OTHERS THEN` 异常处理中未重新抛出异常，可能静默吞错 |
| C014 | `pl-commit-rollback` | PL Block | PL/pgSQL 块中包含 COMMIT/ROLLBACK，可能复杂化事务控制 |
| C015 | `select-for-update-blocking` | SELECT | `SELECT ... FOR UPDATE` 未使用 NOWAIT/SKIP LOCKED，可能长时间阻塞 |
| C016 | `autonomous-transaction` | PL Block | `PRAGMA AUTONOMOUS_TRANSACTION` 会创建独立事务，性能开销较大 |
| C017 | `raise-in-exception-clears-variables` | PL Block | RAISE 在 EXCEPTION 块中会清空所有局部变量值（包括 OUT 参数） |
| C018 | `excessive-insert-values` | INSERT | `INSERT VALUES` 的 `行数 × 列数` 超过阈值（默认 65535，对齐数据库绑定参数上限），大批量可能导致长事务和锁竞争 |

#### 建议 (Suggestion) — 改善可维护性

| ID | 名称 | 适用语句 | 说明 |
|----|------|----------|------|
| S001 | `delete-full-table-use-truncate` | DELETE | DELETE 全表可用 TRUNCATE 替代，释放空间更快 |
| S002 | `limit-offset-use-cursor` | SELECT | OFFSET 分页在大偏移时性能较差，考虑使用游标翻页 |
| S005 | `prefer-percent-type` | PL Block | PL/pgSQL 变量使用普通类型而非 `%TYPE`/`%ROWTYPE` 锚定 |
| S006 | `limit-without-order-by` | SELECT | `LIMIT` 无 `ORDER BY`，结果顺序不确定 |
| S007 | `explicit-type-for-literals` | DML | WHERE 中字符串常量与列比较时类型不匹配（仅在有 schema 且列类型可解析为非同族类型时报告，无 schema 不告警） |
| S008 | `complex-sql-consider-split` | All | SQL 文本长度超过阈值（默认 4096 字符，与 GaussDB `track_activity_query_size` 对齐），建议拆分简化或使用 CTE |

### 11.5 Rust 库 API

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

// 加载 schema（详见 5.9 节）
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

---

## 12. 错误与警告类型参考

### 12.1 错误与警告分类

OGSQL Parser 将诊断信息分为两类：

| 分类 | 来源 | 说明 |
|------|------|------|
| **错误 (Error)** | 语法/语义检查 | 阻止正常解析或影响 SQL 正确性的问题 |
| **警告 (Warning)** | 语法/语义检查 | 可能是问题但不阻止解析，或使用不推荐写法的提示 |

**判断标准：** `ParserError` 的以下变体被视为警告，其余为错误：

- `ParserError::Warning` — 显式警告信息（如嵌套深度超限、包不一致）
- `ParserError::ReservedKeywordAsIdentifier` — 保留关键字被用作标识符

### 12.2 解析错误 (ParserError)

`ParserError` 是解析阶段的核心错误类型，共 6 个变体：

| 变体 | 说明 | 输出示例 |
|------|------|----------|
| `UnexpectedToken` | 解析到不符合预期的 Token | `unexpected token at line 1, column 8: expected FROM, got *` |
| `UnexpectedEof` | 语句不完整，提前到达文件末尾 | `unexpected end of input at line 5, column 1: expected )` |
| `Warning` | 警告信息（不阻止解析） | `nesting depth exceeded 256 — skipping` |
| `ReservedKeywordAsIdentifier` | 保留关键字被用作标识符 | `reserved keyword "select" cannot be used as identifier at line 3, column 5` |
| `TokenizerError` | 词法分析错误（见 [12.3](#123-分词错误-tokenizererror)） | `unterminated string literal at position 42` |
| `UnsupportedSyntax` | 不支持的语法（语义校验产生） | `unsupported syntax at line 10, column 1: MERGE (GaussDB does not support MERGE ... WHEN MATCHED THEN DELETE)` |

**位置信息：** 所有变体（除 `TokenizerError` 外）都包含 `SourceLocation`，包括 `line` 和 `column`（均为 1-based）。

### 12.3 分词错误 (TokenizerError)

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

### 12.4 解析警告

除 `ParserError::Warning` 和 `ReservedKeywordAsIdentifier` 外，以下情况也会产生警告：

| 来源 | 说明 |
|------|------|
| 嵌套深度超限 | 表达式/语句嵌套超过 256 层时，解析器跳过后续 |
| 包不一致 | PACKAGE 与 PACKAGE BODY 中定义的子程序不匹配（见 [13.3](#133-包一致性校验)） |
| 保留关键字 | 在标识符位置使用了 SQL 保留关键字 |
| PL 变量未定义 | PL 块中引用未声明的变量（见 [13.1](#131-pl-变量校验严格模式)） |

### 12.5 输出格式

**JSON 输出（`-j`）：**

```json
{
  "valid": false,
  "total_files": 10,
  "total_errors": 3,
  "total_warnings": 7,
  "files": [
    {
      "file": "query.sql",
      "directory": "sql/",
      "valid": false,
      "error_count": 2,
      "warning_count": 1,
      "errors": [
        "unexpected token at line 5, column 12: expected FROM, got ,",
        "reserved keyword \"select\" cannot be used as identifier at line 3, column 5"
      ]
    }
  ],
  "strict_mode": true
}
```

**CSV 输出（`--csv`）：** 每行一条语句，字段包括文件路径、语句类型、有效性、错误数、警告数、错误详情等。

**统计输出（`--stats`）：** 在所有结果之后打印汇总信息，包括：
- 总文件数、有错误的文件数、有警告的文件数
- 总语句数及按语句类型分布
- 错误类型分布 (error kinds)
- 警告类型分布 (warning kinds)

---

## 13. 语义校验规则

除语法解析外，OGSQL Parser 提供多种语义校验，在 `validate` 命令和库 API 中自动启用。

### 13.1 PL 变量校验（严格模式）

检测 PL/pgSQL 块中引用了未声明的变量或函数。

**正常模式（默认）：** 只检查变量是否已声明（检测 `DECLARE` 块中定义的变量）。

**严格模式（`--strict`）：** 同时检查函数调用是否在已知函数列表中。已知函数包括：
- OGSQL 内置的 449 个函数（通过 `function_registry` 注册）
- 文件中已 `CREATE FUNCTION` / `CREATE PROCEDURE` 定义的函数
- 文件中 PACKAGE / PACKAGE BODY 中定义的子程序

**示例：**

```bash
# 正常模式：只报告未声明变量
echo "CREATE FUNCTION test() RETURNS VOID AS \$\$
BEGIN
  result := undeclared_var;  -- 报告: 未声明变量
  PERFORM known_func();      -- 不报告（可能是外部函数）
END; \$\$ LANGUAGE plpgsql" | ogsql validate

# 严格模式：同时报告未定义函数
echo "CREATE FUNCTION test() RETURNS VOID AS \$\$
BEGIN
  PERFORM unknown_func();    -- 报告: 函数不在已知列表中
END; \$\$ LANGUAGE plpgsql" | ogsql validate --strict
```

`UndefinedVariableError` 包含以下字段：
- `name` — 未定义的变量/函数名
- `location` — 出现位置
- `kind` — `Variable` 或 `Function`

### 13.2 MERGE 语义校验

检测 MERGE 语句的 GaussDB 兼容性问题：

| 种类 | 说明 |
|------|------|
| `DeleteNotSupported` | GaussDB 不支持 `MERGE ... WHEN MATCHED THEN DELETE` |
| `OnColumnUpdated` | GaussDB 不允许在 WHEN MATCHED 中更新 ON 子句引用的列 |
| `DualTableNotSupported` | GaussDB 没有 `DUAL` 表 |

```bash
# 检测 MERGE 语义错误
echo "MERGE INTO t USING s ON t.id = s.id WHEN MATCHED THEN DELETE" | ogsql validate
# → MERGE (GaussDB does not support MERGE ... WHEN MATCHED THEN DELETE)
```

### 13.3 包一致性校验

检查 PACKAGE 与 PACKAGE BODY 定义的一致性：

| 种类 | 说明 |
|------|------|
| `MissingInBody` | PACKAGE 中声明了子程序，但 BODY 中缺少实现 |
| `ExtraInBody` | BODY 中实现了子程序，但 PACKAGE 中未声明 |
| `SignatureMismatch` | 子程序签名（参数类型/数量）不一致 |
| `CursorMismatch` | PACKAGE 中声明的游标在 BODY 中不匹配 |

```bash
# 校验包一致性
ogsql -f package.sql validate
# → package pkg: proc_name — MissingInBody
```
