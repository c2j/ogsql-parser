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
| Tokenizer / 分词器 | Complete | 717 keywords, comments, operators, literals |
| Multi-encoding support / 多字符集支持 | Complete | UTF-8, EUC-JP, EUC-KR, GB18030, BIG5, UTF-16 |
| AST / 抽象语法树 | Complete | 150+ statement types defined |
| Parser dispatcher / 解析器分发 | Complete | Top-level statement routing |
| Unit tests / 单元测试 | 25 tests | Core functionality covered |
| Regression tests / 回归测试 | 1409/1409 | All openGauss regression tests passing |

---

## Project Structure / 项目结构

```
ogsql-parser-by-rust/
├── Cargo.toml           # Project configuration
├── src/
│   ├── lib.rs           # Library exports
│   ├── token/
│   │   ├── mod.rs       # Token types and spans
│   │   ├── tokenizer.rs # Lexical analyzer
│   │   └── keyword.rs   # 717 keyword definitions
│   ├── ast/
│   │   └── mod.rs       # 150+ AST node definitions
│   └── parser/
│       └── mod.rs       # Parser dispatch logic
├── examples/
│   └── regression.rs    # Run regression tests
├── lib/
│   └── openGauss-server/    # Reference source (git submodule)
└── GaussDB-2.23.07.210/     # Documentation reference
```

---

## Getting Started / 快速开始

### Prerequisites / 环境要求

- Rust 1.70+ with Cargo

### Build / 编译

```bash
cargo build --release
```

### Run Tests / 运行测试

```bash
# Unit tests
cargo test

# Regression tests against openGauss test suite
cargo run --example regression
```

### Use as Library / 作为库使用

```rust
use ogsql_parser::{Tokenizer, parser::Parser};

fn main() {
    let sql = "SELECT id, name FROM users WHERE status = 'active'";
    
    // Tokenize
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    
    // Parse
    let statements = Parser::new(tokens).parse().unwrap();
    
    println!("Parsed {} statement(s)", statements.len());
}
```

---

## Architecture Overview / 架构概览

```
SQL Input
    |
    v
+---------------+     +------------------+     +---------------+     +--------+
|  Tokenizer    | --> |  Token Stream    | --> |    Parser     | --> |  AST   |
|  (Lexer)      |     |  (Vec<Token>)    |     |  (Recursive   |     |        |
|               |     |                  |     |   Descent)    |     |        |
+---------------+     +------------------+     +---------------+     +--------+
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

---

## Development Roadmap / 开发路线图

| Phase | Focus | Target |
|-------|-------|--------|
| Phase 1 | Foundation | Tokenizer, AST, dispatcher, multi-encoding (Complete) |
| Phase 2 | Core DML | SELECT, INSERT, UPDATE, DELETE, MERGE (Complete) |
| Phase 3 | DDL | CREATE, ALTER, DROP statements |
| Phase 4 | Advanced Features | Window functions, CTEs, subqueries |
| Phase 5 | openGauss Extensions | GaussDB-specific syntax |
| Phase 6 | Optimization | Error recovery, performance |

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

- `lib/openGauss-server/src/backend/parser/gram.y` (35,325 lines) — Grammar specification
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

This is an active development project. Phases 1-2 are complete.

这是一个活跃的开发项目。第一阶段和第二阶段已完成。

---

**Status / 状态**: Phase 2 Complete (DML parsing) / 第二阶段完成（DML解析）  
**Last Updated / 最后更新**: 2026-04-01
