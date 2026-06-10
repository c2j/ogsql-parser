# Contributing to OGSQL Parser

本文档面向 ogsql-parser 项目的潜在合作开发者和贡献者，涵盖项目架构、构建流程、开发规范和贡献指南。

---

## 目录

- [1. 架构概览](#1-架构概览)
- [2. 从源码构建](#2-从源码构建)
- [3. 开发与测试](#3-开发与测试)
- [4. CI 要求](#4-ci-要求)
- [5. 技术栈](#5-技术栈)
- [6. 参考源](#6-参考源)
- [7. 开发路线图](#7-开发路线图)
- [8. 如何贡献](#8-如何贡献)

---

## 1. 架构概览

### 1.1 核心流程

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

### 1.2 核心设计原则

1. **手写递归下降**：不使用解析器生成器，完全掌控错误消息和错误恢复
2. **Pratt 解析器处理表达式**：自然处理运算符优先级，无左递归问题
3. **尽可能零拷贝**：Token 引用源字符串切片，减少内存分配
4. **多字符集支持**：自动检测和转换多种字符编码
5. **统一解析核心**：所有接口（CLI、HTTP API、TUI、MCP）共享同一解析核心
6. **完整 serde 支持**：所有 AST 类型支持 `Serialize` + `Deserialize`

### 1.3 模块结构

```
ogsql-parser/
├── Cargo.toml              # 项目配置
├── src/
│   ├── lib.rs              # 库导出
│   ├── bin/
│   │   ├── ogsql.rs        # CLI 入口
│   │   └── ogsql-mcp.rs    # MCP 服务器入口
│   ├── formatter/          # SQL & PL/pgSQL 格式化器
│   ├── token_formatter.rs  # Token 级格式化器
│   ├── token/
│   │   ├── mod.rs          # Token 类型和 Span
│   │   ├── tokenizer.rs    # 词法分析器
│   │   ├── keyword.rs      # 717 个关键字定义
│   │   └── encoding.rs     # 多字符集支持
│   ├── ast/
│   │   ├── mod.rs          # 180+ AST 节点定义
│   │   ├── plpgsql.rs      # PL/pgSQL AST 类型 (40+ 类型)
│   │   └── visitor.rs      # AST Visitor 模式
│   ├── parser/
│   │   ├── mod.rs          # 解析器分发和主逻辑
│   │   ├── select.rs       # SELECT 解析器
│   │   ├── dml.rs          # INSERT/UPDATE/DELETE/MERGE
│   │   ├── ddl/            # DDL 解析器（含子模块）
│   │   ├── expr.rs         # 表达式解析器（Pratt）
│   │   ├── plpgsql.rs      # PL/pgSQL 解析器
│   │   ├── hint_validator.rs    # Hint 验证
│   │   ├── function_registry.rs # 内置函数注册表 (449 个函数)
│   │   └── utility/        # 工具解析器（类型、约束等）
│   ├── analyzer/
│   │   ├── mod.rs          # 语义分析主模块
│   │   ├── schema.rs       # Schema 加载和解析
│   │   └── return_cursor.rs # 返回游标分析
│   ├── linter/             # SQL 反模式检测
│   │   ├── mod.rs          # Linter 主模块
│   │   ├── rules_prohibition.rs  # 禁止项规则 (R001-R009)
│   │   ├── rules_performance.rs  # 性能规则 (P001-P022)
│   │   ├── rules_caution.rs      # 注意事项规则 (C001-C018)
│   │   └── rules_suggestion.rs   # 建议规则 (S001-S008)
│   ├── ibatis/             # iBatis/MyBatis XML 解析
│   ├── java/               # Java 源文件 SQL 提取
│   └── mcp/                # MCP 服务器实现
├── examples/
│   ├── regression.rs       # 运行回归测试
│   ├── test_issues.rs      # 问题专项测试
│   └── test_loc.rs         # 行列精度测试
├── tests/
│   ├── roundtrip.rs        # SQL ↔ JSON 往返测试
│   ├── plpgsql.rs          # PL/pgSQL 集成测试
│   ├── multi_statement.rs  # 多语句解析测试
│   └── error_handling.rs   # 错误处理测试
├── docs/
│   ├── user-guide.md       # 用户指南
│   ├── crate-guide.md      # Crate 开发者指南
│   ├── ast-json-reference.md # AST JSON 结构参考
│   └── plans/              # 实现计划
├── lib/
│   └── openGauss-server/   # 参考源码 (git submodule)
└── GaussDB-2.23.07.210/    # 文档参考
```

---

## 2. 从源码构建

### 2.1 环境要求

- **Rust 1.70+** 及 Cargo
- Git（克隆仓库及 submodule）
- （可选）Rust nightly 工具链，用于 Windows 7 目标构建

### 2.2 克隆仓库

```bash
git clone <repository-url>
cd ogsql-parser
git submodule update --init --recursive
```

### 2.3 Feature 特性

OGSQL Parser 使用 Cargo Feature 来控制可选功能模块：

| Feature | 说明 | 额外依赖 |
|---------|------|----------|
| `cli` | 命令行工具 | clap, walkdir |
| `ibatis` | iBatis/MyBatis XML Mapper 解析 | quick-xml, walkdir |
| `java` | Java 源文件 SQL 提取 | tree-sitter, tree-sitter-java, walkdir |
| `serve` | HTTP API 服务器 | axum, tokio, tower-http, utoipa |
| `tui` | 交互式终端 UI | ratatui, crossterm |
| `mcp` | MCP 服务器 | rmcp, schemars, tokio + ibatis + java |
| `full` | 启用所有功能 | 所有上述依赖 |

**编译示例：**

```bash
# 仅编译 CLI
cargo build --release --features cli

# 编译 CLI + HTTP API
cargo build --release --features serve

# 编译全部功能
cargo build --release --features full

# 编译 MCP 服务器（自动包含 ibatis 和 java）
cargo build --release --features mcp
```

### 2.4 Windows 7 构建

Rust 1.78+ 移除了 Windows 7 支持（stdlib 无条件调用 `GetSystemTimePreciseAsFileTime`，该 API 仅在 Windows 8+ 存在）。使用官方 Tier 3 目标配合 nightly 工具链和 `-Zbuild-std` 从源码编译标准库：

```bash
# 前置条件
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly

# 构建
cargo +nightly build --release --features cli --target x86_64-win7-windows-msvc -Zbuild-std
cargo +nightly build --release --features full --target x86_64-win7-windows-msvc -Zbuild-std
```

产出物在 `target/x86_64-win7-windows-msvc/release/ogsql.exe`。

32 位目标同理：`i686-win7-windows-msvc`。

`.cargo/config.toml` 已配置两个 Win7 目标的 `+crt-static` flag，确保静态链接 CRT。

---

## 3. 开发与测试

### 3.1 运行测试

```bash
# 单元测试
cargo test

# 全部测试（含 feature gated）
cargo test --all-features

# 回归测试（对照 openGauss 测试套件）
cargo run --example regression
```

### 3.2 常用开发命令

```bash
# 代码格式化检查
cargo fmt --all -- --check

# Clippy 检查
cargo clippy --all-features -- -D warnings

# 安全审计
cargo audit
```

---

## 4. CI 要求

CI 定义在 `.github/workflows/ci.yml`，包含以下 job，全部必须通过：

| Job | 命令 | 说明 |
|-----|------|------|
| Format | `cargo fmt --all -- --check` | 代码格式一致 |
| Clippy | `cargo clippy --all-features -- -D warnings` | 零 lint 警告 |
| Test | `cargo test --all-features` | 全部测试通过 |
| Security Audit | `cargo audit` | 无已知漏洞依赖 |

### 提交前本地验证

```bash
cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings
cargo test --all-features
```

---

## 5. 技术栈

| 组件 | 选择 | 理由 |
|------|------|------|
| 语言 | Rust 2021 | 安全性、性能、生态 |
| 错误处理 | `thiserror` 2.0 | 人体工学错误类型 |
| 字符编码 | `encoding_rs` 0.8 | 多字符集支持 |
| CLI | `clap` 4 | Derive 宏参数解析 |
| HTTP API | `axum` 0.7 | 异步 Web 框架 |
| MCP | `rmcp` 1.5 | Model Context Protocol 服务器 |
| TUI | `ratatui` 0.29 | 终端 UI 框架 |
| XML 解析 | `quick-xml` 0.39 | 快速 XML 解析（iBatis） |
| Java 解析 | `tree-sitter-java` 0.23 | 增量 Java 解析 |
| 测试 | Built-in + walkdir 2.0 | 回归测试文件发现 |
| Win7 构建 | nightly + `-Zbuild-std` | Tier 3 Windows 7 目标 |

---

## 6. 参考源

本项目参考以下 openGauss 源文件实现语法解析：

- `lib/openGauss-server/src/backend/parser/gram.y` (35,325 行) — 主 SQL 语法规范
- `lib/openGauss-server/src/common/pl/plpgsql/src/gram.y` (15,770 行) — PL/pgSQL 语法规范
- `lib/openGauss-server/src/backend/parser/scan.l` — 词法规范
- `lib/openGauss-server/src/include/parser/kwlist.h` — 关键字定义 (717 个)
- `lib/openGauss-server/src/test/regress/sql/` — 1,397 回归测试文件

文档参考：
- `GaussDB-2.23.07.210/term/` — GaussDB 术语和规范

---

## 7. 开发路线图

| 阶段 | 重点 | 状态 |
|------|------|------|
| Phase 1 | 基础：Tokenizer, AST, dispatcher, 多编码 | ✅ 完成 |
| Phase 2 | 核心 DML：SELECT, INSERT, UPDATE, DELETE, MERGE | ✅ 完成 |
| Phase 3 | PL/pgSQL：DO 块、匿名块、控制流 | ✅ 完成 |
| Phase 4 | DDL：CREATE, ALTER, DROP 语句 | ✅ 完成 |
| Phase 5 | 高级：语义分析、动态 SQL、MERGE 校验 | ✅ 完成 |
| Phase 6 | 集成：iBatis XML, Java SQL, MCP, 严格校验 | ✅ 完成 |
| Phase 7 | 优化：错误恢复、性能 | 📋 规划中 |

---

## 8. 如何贡献

欢迎通过 Issue 和 Pull Request 参与贡献。

1. Fork 仓库并创建功能分支
2. 确保代码通过 `cargo fmt` 和 `cargo clippy` 检查
3. 添加/更新测试用例
4. 运行 `cargo test --all-features` 确保全部通过
5. 提交 Pull Request

### 许可证

MIT OR Apache-2.0 双许可。您的贡献默认以相同许可证发布。
