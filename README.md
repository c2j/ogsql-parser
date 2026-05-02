# OGSQL Parser / OGSQL解析器

> A hand-written recursive descent SQL parser for openGauss/GaussDB written in Rust  
> 使用 Rust 编写的手写递归下降 SQL 解析器，支持 openGauss/GaussDB

---

## Project Overview / 项目概述

This project implements a complete SQL parser for openGauss/GaussDB (an enterprise-class database system based on PostgreSQL). Built from scratch using pure recursive descent parsing techniques without relying on parser generators.

本项目为 openGauss/GaussDB（一款基于 PostgreSQL 的企业级数据库系统）实现了一个完整的 SQL 解析器。采用纯递归下降解析技术从零构建，不依赖解析器生成器。

---

## Completed Features / 已完成特性

### Phase 1: Foundation / 第一阶段：基础框架

| Component | Status | Details |
|-----------|--------|---------|
| Tokenizer / 分词器 | ✅ Complete | 717 keywords, comments, operators, literals |
| Multi-encoding support / 多字符集支持 | ✅ Complete | UTF-8, EUC-JP, EUC-KR, GB18030, BIG5, UTF-16 |
| AST / 抽象语法树 | ✅ Complete | 150+ statement types defined |
| Parser dispatcher / 解析器分发 | ✅ Complete | Top-level statement routing |
| Unit tests / 单元测试 | ✅ Complete | 230 tests |
| Regression tests / 回归测试 | ✅ Complete | 1409/1409 — All openGauss regression tests passing |
| JSON serde / JSON 序列化 | ✅ Complete | Full serde::Serialize + Deserialize on all AST types |

### Phase 2: Core DML / 第二阶段：核心DML

| Component | Status | Details |
|-----------|--------|---------|
| SELECT | ✅ Complete | Full SELECT syntax including CTEs, subqueries, window functions |
| INSERT | ✅ Complete | INSERT VALUES, INSERT SELECT, ON CONFLICT |
| UPDATE | ✅ Complete | UPDATE with SET, FROM, WHERE, RETURNING |
| DELETE | ✅ Complete | DELETE with WHERE, RETURNING |
| MERGE | ✅ Complete | MERGE INTO with WHEN MATCHED/NOT MATCHED |
| Expressions / 表达式 | ✅ Complete | Pratt parser for full expression support |
| Formatter / 格式化器 | ✅ Complete | Configurable SQL formatter: indent, keyword case, comma style, line width, DML/DDL/PL-pgSQL |

### Phase 3: PL/pgSQL Support / 第三阶段：PL/pgSQL支持

| Component | Status | Details |
|-----------|--------|---------|
| DO statement / DO语句 | ✅ Complete | `DO $$ ... $$` with dollar-quoted PL/pgSQL bodies |
| Anonymous blocks / 匿名块 | ✅ Complete | `DECLARE ... BEGIN ... END` and `BEGIN ... END` blocks |
| Declarations / 声明 | ✅ Complete | Variables, constants, cursors, RECORD, TYPE, %TYPE, %ROWTYPE |
| Assignment / 赋值 | ✅ Complete | `variable := expression` |
| IF/ELSIF/ELSE | ✅ Complete | Full conditional branching |
| CASE | ✅ Complete | Searched and plain CASE expressions |
| LOOP / WHILE / FOR / FOREACH | ✅ Complete | All loop constructs with labels |
| EXIT / CONTINUE | ✅ Complete | With optional WHEN conditions and labels |
| RETURN | ✅ Complete | With optional expression |
| RAISE | ✅ Complete | All levels: DEBUG, LOG, INFO, NOTICE, WARNING, EXCEPTION |
| EXECUTE / PERFORM | ✅ Complete | Dynamic SQL execution |
| Cursor operations / 游标操作 | ✅ Complete | OPEN, FETCH, CLOSE, MOVE |
| GET DIAGNOSTICS | ✅ Complete | Including STACKED DIAGNOSTICS |
| Exception handling / 异常处理 | ✅ Complete | WHEN ... THEN handlers |
| GOTO | ✅ Complete | GOTO label |
| Transaction control / 事务控制 | ✅ Complete | COMMIT, ROLLBACK, SAVEPOINT within blocks |
| FORALL / PIPE ROW | ✅ Complete | Bulk operations and pipe row |
| PL/pgSQL Formatter / 格式化 | ✅ Complete | Full formatting for all PL/pgSQL constructs |

### Phase 4: DDL (In Progress) / 第四阶段：DDL（进行中）

| Component | Status | Details |
|-----------|--------|---------|
| CREATE TABLE | ✅ Complete | Columns, constraints, table options |
| CREATE INDEX | ✅ Complete | Index creation with options |
| CREATE VIEW | ✅ Complete | View definitions |
| DROP statements | ✅ Complete | DROP TABLE, INDEX, VIEW |
| TRUNCATE | ✅ Complete | TRUNCATE TABLE |
| ALTER SYSTEM | ✅ Complete | SET/RESET configuration |
| CREATE FUNCTION | ✅ Complete | Full CREATE FUNCTION with parameters, return type, options, PL/pgSQL body |
| CREATE PROCEDURE | ✅ Complete | Full CREATE PROCEDURE with parameters, options, PL/pgSQL body |
| CREATE PACKAGE / PACKAGE BODY | ✅ Complete | Oracle-compatible packages with procedures, functions, cursors |
| CREATE TRIGGER | ✅ Complete | BEFORE/AFTER/INSTEAD OF, events, WHEN condition, EXECUTE |
| ALTER TABLE | 🔄 In Progress | Column operations, constraints |
| Other DDL | 🔄 In Progress | ALTER FUNCTION, ALTER PROCEDURE, etc. |

---

## Project Structure / 项目结构

```
ogsql-parser-by-rust/
├── Cargo.toml              # Project configuration
├── src/
│   ├── lib.rs              # Library exports
│   ├── formatter.rs        # SQL & PL/pgSQL formatter
│   ├── token/
│   │   ├── mod.rs          # Token types and spans
│   │   ├── tokenizer.rs    # Lexical analyzer (898 lines)
│   │   └── keyword.rs      # 717 keyword definitions
│   ├── ast/
│   │   ├── mod.rs          # 150+ AST node definitions
│   │   ├── plpgsql.rs      # PL/pgSQL AST types (40+ types)
│   │   └── visitor.rs      # AST visitor pattern
│   └── parser/
│       ├── mod.rs           # Parser dispatch & main logic
│       ├── select.rs        # SELECT statement parser
│       ├── dml.rs           # INSERT, UPDATE, DELETE, MERGE
│       ├── ddl.rs           # DDL statement parsers
│       ├── expr.rs          # Expression parser (Pratt)
│       ├── plpgsql.rs       # PL/pgSQL parser (25+ statement types)
│       ├── utility.rs       # Utility parsers (types, constraints, etc.)
│       └── tests.rs         # 230 unit tests
├── examples/
│   └── regression.rs       # Run regression tests
├── lib/
│   └── openGauss-server/   # Reference source (git submodule)
└── GaussDB-2.23.07.210/    # Documentation reference
```

---

## Getting Started / 快速开始

### Prerequisites / 环境要求

- Rust 1.70+ with Cargo

### Build / 编译

```bash
# Default build (CLI only)
cargo build --release

# With HTTP API server
cargo build --release --features serve

# With TUI playground
cargo build --release --features tui

# With MCP server
cargo build --release --features mcp

# All features
cargo build --release --features full
```

### Run Tests / 运行测试

```bash
# Unit tests (230 tests)
cargo test

# Regression tests against openGauss test suite
cargo run --example regression
```

### CLI Usage / 命令行使用

```
$ ogsql --help
openGauss/GaussDB SQL Parser

Usage: ogsql [OPTIONS] <COMMAND>

Commands:
  format      Format SQL statements with configurable options
  parse       Parse SQL into AST and print the abstract syntax tree / 解析 SQL 为 AST
  json2sql    Convert JSON (from `parse -j`) back to SQL / 将 JSON 还原为 SQL
  tokenize    Tokenize SQL into a list of tokens / 将 SQL 分词为 token 列表
  validate    Validate SQL syntax and report errors / 校验 SQL 语法
  serve       Start an HTTP API server for parsing SQL [requires: serve feature]
  playground  Launch an interactive terminal UI playground [requires: tui feature]

Options:
  -f, --file <FILE>   Read SQL from file instead of stdin
      --mybatis       Enable MyBatis #{param} and ${expr} placeholder support
  -j, --json          Output in JSON format
  -h, --help          Print help
  -V, --version       Print version
```

#### Format Options / 格式化选项

```
$ ogsql format --help
Format SQL statements with configurable indentation, keyword casing, comma style, and line width.
Supports SELECT, INSERT, DELETE, UPDATE, MERGE, WITH (CTE), CREATE TABLE, and PL/pgSQL.

Options:
  -i, --indent <INDENT>              Indentation width in spaces [default: 2]
  -k, --keyword-case <KEYWORD_CASE>  Keyword casing: preserve, upper, lower [default: preserve]
      --comma <COMMA>                Comma style: trailing, leading [default: trailing]
  -w, --line-width <LINE_WIDTH>      Maximum line width (0 = unlimited) [default: 120]
  -u, --uppercase                    Shorthand for --keyword-case upper
      --no-select-newline            Don't put each SELECT column on its own line
      --no-logical-newline           Don't put AND/OR on new lines
      --no-semicolon-newline         Don't put semicolons on their own line
      --mybatis                      Preserve MyBatis #{param} and ${expr} placeholders
```

#### Examples / 示例

```bash
# Tokenize SQL
echo "SELECT * FROM users" | ogsql tokenize

# Parse SQL to AST (debug format)
echo "SELECT * FROM users" | ogsql parse

# Parse SQL to AST (JSON format)
echo "SELECT * FROM users" | ogsql parse -j

# SQL → JSON → SQL round-trip
echo "SELECT id FROM users WHERE id = 1" | ogsql parse -j | ogsql json2sql

# Convert JSON file back to SQL
ogsql -f ast.json json2sql

# Format SQL (default: 2-space indent, keyword casing preserved)
echo "select id,name from users where id=1" | ogsql format

# Format with uppercase keywords and 4-space indent
echo "select * from users" | ogsql format -u -i 4

# Format with leading comma style
echo "SELECT id, name, age FROM users" | ogsql format --comma leading

# Format MyBatis SQL (preserves #{param} placeholders)
echo "SELECT * FROM users WHERE id = #{userId}" | ogsql format --mybatis

# Format INSERT with MyBatis parameters
echo "insert into t (a,b) values (#{x},#{y})" | ogsql format --mybatis

# Format DML statements
echo "update users set name='Bob', age=30 where id=1" | ogsql format
echo "delete from users where id = 1 and status = 'inactive'" | ogsql format
echo "merge into target t using source s on t.id=s.id when matched then update set t.name=s.name" | ogsql format
echo "with cte as (select id from users) select * from cte" | ogsql format

# Validate SQL syntax
echo "SELECT FROM" | ogsql validate

# Read from file
ogsql -f query.sql parse -j

# Start HTTP API server
ogsql serve --host 0.0.0.0 --port 3000
```

#### HTTP API Endpoints / HTTP API 接口

When built with `--features serve`, the following endpoints are available:

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/health` | Health check |
| POST | `/api/parse` | Parse SQL → AST JSON (body: `{"sql": "..."}`) |
| POST | `/api/json2sql` | JSON → SQL (body: `{"json": "..."}`) |
| POST | `/api/format` | Format SQL with configurable options (body: `{"sql": "...", "indent": 2, "keyword_case": "upper", ...}`) |
| POST | `/api/tokenize` | Tokenize SQL (body: `{"sql": "..."}`) |
| POST | `/api/validate` | Validate SQL (body: `{"sql": "..."}`) |

#### MCP Server / MCP 服务器

When built with `--features mcp`, an MCP (Model Context Protocol) server binary is available:

```bash
# Build MCP server
cargo build --release --features mcp

# Run (stdio transport — for Claude Desktop, Cursor, etc.)
ogsql-mcp
```

##### MCP Tools / MCP 工具

| Tool | Description |
|------|-------------|
| `parse` | Parse SQL → AST JSON (with fingerprints, comments, errors) |
| `tokenize` | SQL → Token list with types, values, positions |
| `format` | Format SQL with configurable indent, keyword case, comma style, line width |
| `validate` | Validate SQL syntax, report errors/warnings |
| `json2sql` | Convert AST JSON back to SQL |
| `parse_xml` | Parse iBatis/MyBatis XML mapper → extracted SQL |
| `parse_java` | Extract SQL from Java source files |

##### Claude Desktop Configuration

Add to `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "ogsql": {
      "command": "/path/to/ogsql-mcp"
    }
  }
}
```

### Use as Library / 作为库使用

#### Basic Parsing / 基本解析

```rust
use ogsql_parser::{Tokenizer, parser::Parser};

fn main() {
    // Parse a simple SQL statement
    let sql = "SELECT id, name FROM users WHERE status = 'active'";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let statements = Parser::new(tokens).parse().unwrap();
    println!("Parsed {} statement(s)", statements.len());

    // Parse a PL/pgSQL DO block
    let plsql = "DO $$ BEGIN RAISE NOTICE 'hello'; END $$";
    let tokens = Tokenizer::new(plsql).tokenize().unwrap();
    let statements = Parser::new(tokens).parse().unwrap();
    println!("Parsed {} statement(s)", statements.len());
}
```

#### JSON Round-Trip (SQL ↔ JSON) / JSON 往返转换

All AST types implement `serde::Serialize` and `serde::Deserialize`, enabling lossless semantic round-tripping: parse SQL to AST, serialize to JSON, deserialize back to AST, and format back to semantically equivalent SQL.

所有 AST 类型实现了 `serde::Serialize` 和 `serde::Deserialize`，支持无损语义往返：将 SQL 解析为 AST，序列化为 JSON，反序列化回 AST，再格式化为语义等价的 SQL。

```
SQL ──parse──▶ AST ──serialize──▶ JSON ──deserialize──▶ AST ──format──▶ SQL'
                                    │                                       │
                                    └──────── semantically equivalent ────────┘
```

```rust
use ogsql_parser::{Tokenizer, Parser, SqlFormatter, Statement};

fn roundtrip(sql: &str) -> String {
    // 1. SQL → AST
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();

    // 2. AST → JSON
    let json = serde_json::to_string(&stmts).unwrap();

    // 3. JSON → AST
    let restored: Vec<Statement> = serde_json::from_str(&json).unwrap();

    // 4. AST → SQL
    let formatter = SqlFormatter::new();
    let output: Vec<String> = restored.iter()
        .map(|s| formatter.format_statement(s))
        .collect();

    output.join(";\n")
}

fn main() {
    // DML round-trip
    let sql = "SELECT id, name FROM users WHERE status = 'active'";
    assert_eq!(roundtrip(sql), "SELECT id, name FROM users WHERE status = 'active'");

    // Special literal types are preserved (E'', B'', X'', N'', $$ $$)
    let sql2 = "SELECT E'\\ttext', B'1010', X'FF', N'unicode'";
    assert_eq!(roundtrip(sql2), "SELECT E'\\ttext', B'1010', X'FF', N'unicode'");

    // DDL round-trip
    let sql3 = "CREATE TABLE t (id INTEGER PRIMARY KEY, name VARCHAR(100) NOT NULL)";
    assert_eq!(roundtrip(sql3), "CREATE TABLE t (id INTEGER PRIMARY KEY, name VARCHAR(100) NOT NULL)");

    // Window functions with frame clauses
    let sql4 = "SELECT ROW_NUMBER() OVER (ORDER BY id ROWS BETWEEN 1 PRECEDING AND 1 FOLLOWING)";
    assert_eq!(roundtrip(sql4), "SELECT ROW_NUMBER() OVER (ORDER BY id ROWS BETWEEN 1 PRECEDING AND 1 FOLLOWING)");
}
```

**What is preserved / 保留内容：**
- All SQL semantics (operators, expressions, subqueries, joins)
- Special literal types: escape strings (`E'...'`), bit strings (`B'...'`), hex strings (`X'...'`), national strings (`N'...'`), dollar-quoted strings (`$$...$$` / `$tag$...$tag$`)
- Type casts with parameterized types (`NUMERIC(10,2)`, `VARCHAR(100)`, `TIMESTAMP(3) WITH TIME ZONE`)
- Window frame specifications (`ROWS BETWEEN ... AND ...`)
- PL/pgSQL control flow (IF/WHILE/CASE/FOR/FOREACH conditions as structured expressions)
- DDL column types, constraints, domain definitions, cast types, RLS policy expressions

**What is NOT preserved / 不保留内容：**
- Comments and whitespace (non-semantic)
- Original keyword casing (formatter normalizes to uppercase)
- Original formatting/layout

---

## Architecture Overview / 架构概览

```
SQL Input
    |
    v
+---------------+     +------------------+     +---------------+     +--------+     +-----------+
|  Tokenizer    | --> |  Token Stream    | --> |    Parser     | --> |  AST   | --> | Formatter |
|  (Lexer)      |     |  (Vec<Token>)    |     |  (Recursive   |     |        |     |           |
|               |     |                  |     |   Descent)    |     |  serde |     |           |
+---------------+     +------------------+     +---------------+     +--------+     +-----------+
                                                                          │  ▲
                                                                          │  │
                                                                    JSON serialize/deserialize
                                                                          │  │
                                                                          ▼  │
                                                                    +-----------+
                                                                    |   JSON    |
                                                                    +-----------+
```

### Key Design Decisions / 关键设计决策

1. **No parser generators / 无解析器生成器**  
   Hand-written recursive descent for full control over error messages and recovery

2. **Pratt parsing for expressions / 使用 Pratt 解析表达式**  
   Handles operator precedence naturally without left-recursion issues

3. **Zero-copy where possible / 尽可能零拷贝**  
   Tokens reference source string slices

4. **Multi-encoding support / 多字符集支持**  
   Automatic detection and conversion of UTF-8, EUC-JP, EUC-KR, GB18030, BIG5, UTF-16

5. **Unified parser core / 统一解析器核心**  
   All interfaces (CLI, HTTP API, TUI) share the same parser core

6. **Full serde support / 完整 serde 支持**  
   All AST types derive `Serialize` + `Deserialize`, enabling lossless JSON round-trip (SQL → JSON → SQL)

---

## Development Roadmap / 开发路线图

| Phase | Focus | Status |
|-------|-------|--------|
| Phase 1 | Foundation: Tokenizer, AST, dispatcher, multi-encoding | ✅ Complete |
| Phase 2 | Core DML: SELECT, INSERT, UPDATE, DELETE, MERGE | ✅ Complete |
| Phase 3 | PL/pgSQL: DO blocks, anonymous blocks, control flow | ✅ Complete |
| Phase 4 | DDL: CREATE, ALTER, DROP statements | 🔄 In Progress |
| Phase 5 | Advanced: Function/procedure bodies, packages, triggers | ✅ Complete |
| Phase 6 | Optimization: Error recovery, performance | 📋 Planned |

---

## Technology Stack / 技术栈

| Component | Choice | Rationale |
|-----------|--------|-----------|
| Language | Rust 2021 | Safety, performance, ecosystem |
| Error handling | `thiserror` 2.0 | Ergonomic error types |
| Encoding | `encoding_rs` 0.8 | Multi-character set support (EUC-JP, EUC-KR, GB18030, etc.) |
| Testing | Built-in + walkdir 2.0 | Regression test file discovery |

---

## References / 参考资料

This project references the following openGauss source files:

- `lib/openGauss-server/src/backend/parser/gram.y` (35,325 lines) — Main SQL grammar specification
- `lib/openGauss-server/src/common/pl/plpgsql/src/gram.y` (15,770 lines) — PL/pgSQL grammar specification
- `lib/openGauss-server/src/backend/parser/scan.l` — Lexer specification
- `lib/openGauss-server/src/include/parser/kwlist.h` — Keyword definitions (717 keywords)
- `lib/openGauss-server/src/test/regress/sql/` — 1,397 regression test files

Documentation reference:
- `GaussDB-2.23.07.210/term/` — GaussDB terminology and specifications

---

## License / 许可证

MIT OR Apache-2.0

---

## Contributing / 贡献

This is an active development project. Phases 1-5 are complete, Phase 4 (DDL) and Phase 6 (Optimization) are in progress.

这是一个活跃的开发项目。第一至第三阶段已完成，第四阶段（DDL）进行中。

---

**Status / 状态**: Phase 5 Complete + JSON Round-Trip / 第五阶段完成 + JSON 往返转换  
**Last Updated / 最后更新**: 2026-04-11
