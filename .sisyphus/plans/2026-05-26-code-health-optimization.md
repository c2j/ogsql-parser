# Code Health Optimization Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 将项目健康度从 6.6/10 提升到 8.0+/10，消除可靠性隐患、完善 CI/CD、清理仓库垃圾。

**Architecture:** 按优先级分 3 个 Iteration。Iteration-1 修复可靠性崩溃点 + 清理仓库（P0），Iteration-2 建立 CI/CD 质量门禁（P1），Iteration-3 重构臃肿文件 + 消除重复（P2）。每个 Iteration 独立可合并。

**Tech Stack:** Rust 2021, GitHub Actions, clippy, rustfmt

---

## Iteration-1: 可靠性修复 + 仓库清理 (P0)

### Task 1.1: 清理仓库调试产物

**Files:**
- Delete: `error.log`, `error-1.log`, `error-2.log`, `error-3.log`, `error-latest.log`, `error-20260511.log`, `error-4.txt`, `error-5.txt`, `err-report.txt`
- Delete: `lib/error_java1.java`, `lib/error-execute.sql`, `lib/PKG_CURSOR.sql`, `lib/PKG_FOR.sql`
- Delete: `tmp/`
- Modify: `.gitignore` (添加 `.DS_Store`, `.vscode/`, `.idea/`, `*.swp`)

**Step 1: 删除错误日志文件**

```bash
git rm error*.log error*.txt err-report.txt
git rm lib/error_java1.java lib/error-execute.sql lib/PKG_CURSOR.sql lib/PKG_FOR.sql
rm -rf tmp/
```

**Step 2: 更新 .gitignore**

添加遗漏的忽略规则：
- `.DS_Store`
- `.vscode/`
- `.idea/`
- `*.swp`, `*.swo`
- `*~`

**Step 3: 提交**

```bash
git add .gitignore
git commit -m "chore: remove debug artifacts and update .gitignore"
```

---

### Task 1.2: 修复解析器核心 .unwrap() 崩溃点

**Files:**
- Modify: `src/parser/mod.rs` (lines 171, 269, 299, 2407, 2974)

**Step 1: 修复 line 171 — parse 返回 0 语句时崩溃**

```rust
// BEFORE:
1 => Ok((infos.into_iter().next().unwrap(), parser.errors().to_vec())),

// AFTER:
1 => {
    let info = infos.into_iter().next().ok_or_else(|| {
        let location = parser.current_location();
        ParserError::UnexpectedEof {
            expected: "SQL statement".to_string(),
            location,
        }
    })?;
    Ok((info, parser.errors().to_vec()))
}
```

**Step 2: 修复 line 269 — errors.pop() 空 Vec**

检查错误列表是否为空，如果为空则返回合适的错误：
```rust
let last_error = self.errors.pop()
    .ok_or_else(|| ParserError::Warning {
        message: "internal: expected error in list".to_string(),
        location: self.current_location(),
    })?;
```

**Step 3: 修复 line 299 — tokens.last() 空 Vec**

```rust
// BEFORE:
let last = self.tokens.last().unwrap();

// AFTER:
let last = self.tokens.last().ok_or(ParserError::UnexpectedEof {
    expected: "token".to_string(),
    location: SourceLocation::default(),
})?;
```

**Step 4: 修复 line 2407, 2974 — peek_keyword() unwrap**

确保 `peek_keyword()` 返回 `Result` 或添加 `ok_or_else` 转换。

**Step 5: 运行测试验证**

```bash
cargo test
```
Expected: 所有 1,234 测试通过。

**Step 6: 提交**

```bash
git commit -m "fix(parser): replace unwrap() with proper error handling in core parser"
```

---

### Task 1.3: 修复 tokenizer .unwrap() 崩溃点

**Files:**
- Modify: `src/token/tokenizer.rs` (lines 541, 543, 974, 1026, 1028)

**Step 1: 修复 advance() unwrap (lines 541, 543, 1026, 1028)**

```rust
// BEFORE:
full.push(self.advance().unwrap());

// AFTER:
match self.advance() {
    Some(token) => full.push(token),
    None => return Err(TokenizerError::UnexpectedEof {
        expected: "character".to_string(),
        position: self.position(),
    }),
}
```

> 注意: 需要检查 `TokenizerError` 枚举是否有 `UnexpectedEof` 变体，如果没有则需要添加。

**Step 2: 添加 TokenizerError::UnexpectedEof 变体**

在 `token/mod.rs` 或 `token/tokenizer.rs` 的错误枚举中添加：
```rust
#[error("unexpected end of input at position {position}: expected {expected}")]
UnexpectedEof {
    expected: String,
    position: usize,
},
```

**Step 3: 修复 line 974 — window.pop_front() unwrap**

```rust
// BEFORE:
let token = window.pop_front().unwrap();

// AFTER:
let token = window.pop_front().ok_or(TokenizerError::Internal {
    message: "expected token in window".to_string(),
})?;
```

**Step 4: 运行测试验证**

```bash
cargo test
cargo test --features full
```
Expected: 所有测试通过。

**Step 5: 提交**

```bash
git commit -m "fix(tokenizer): replace unwrap() with proper error handling"
```

---

### Task 1.4: 修复 formatter 可选字段崩溃点

**Files:**
- Modify: `src/formatter.rs` (lines 4426, 4434, 4442, 4450, 4458, 7232, 7234, 7593, 7596)

**Step 1: 修复 savepoint_name / transaction_id unwrap (lines 4426-4458)**

```rust
// BEFORE:
self.quote_identifier(stmt.savepoint_name.as_ref().unwrap())

// AFTER:
match &stmt.savepoint_name {
    Some(name) => self.quote_identifier(name),
    None => String::new(), // or return error
}
```

**Step 2: 修复 table / new_name unwrap (lines 7232, 7234)**

同样用 `match` 替换 `.as_ref().unwrap()`。

**Step 3: 修复 tokenize/parse unwrap (lines 7593, 7596)**

将 `Tokenizer::new(sql).tokenize().unwrap()` 替换为 `?` 传播。

**Step 4: 运行测试验证**

```bash
cargo test
```
Expected: 所有测试通过。

**Step 5: 提交**

```bash
git commit -m "fix(formatter): replace unwrap() on optional fields with safe match"
```

---

### Task 1.5: 修复 parser/expr.rs 表达式解析崩溃点

**Files:**
- Modify: `src/parser/expr.rs` (lines 1250, 1335, 1585, 1907)

**Step 1: 修复 name.last().unwrap() / obj_name.last().unwrap()**

```rust
// BEFORE:
name.last().unwrap()

// AFTER:
name.last().ok_or(ParserError::UnexpectedEof {
    expected: "identifier".to_string(),
    location: self.current_location(),
})?
```

**Step 2: 修复 args.pop().unwrap()**

同样替换为 `ok_or()` 错误返回。

**Step 3: 修复 peek_keyword().unwrap()**

使用 `ok_or_else()` 或确保 `peek_keyword` 返回 `Result`。

**Step 4: 运行测试验证**

```bash
cargo test
```
Expected: 所有测试通过。

**Step 5: 提交**

```bash
git commit -m "fix(expr): replace unwrap() with proper error handling in expression parser"
```

---

### Task 1.6: 添加索引边界检查（选择性修复）

**Files:**
- 优先修复: `src/parser/mod.rs`, `src/parser/select.rs`, `src/parser/ddl/alter.rs`, `src/parser/plpgsql.rs`

**Step 1: 修复高频索引位置**

在 `mod.rs` 和 `select.rs` 中找到最关键的 `tokens[pos]` 和 `tokens[pos + 1]` 访问，替换为：

```rust
// BEFORE:
let next = &self.tokens[self.pos + 1].token;

// AFTER:
let next = self.tokens.get(self.pos + 1)
    .map(|t| &t.token)
    .ok_or_else(|| ParserError::UnexpectedEof {
        expected: "token".to_string(),
        location: self.current_location(),
    })?;
```

**Step 2: 运行测试验证**

```bash
cargo test
```
Expected: 所有测试通过。

**Step 3: 提交**

```bash
git commit -m "fix(parser): add bounds checking for token index operations"
```

---

### Task 1.7: 初始化缺失的 git 子模块

**Files:**
- Submodule: `lib/openGauss-server`

**Step 1: 初始化子模块**

```bash
git submodule update --init lib/openGauss-server
```

**Step 2: 提交（如果需要）**

如果子模块指针有变化：
```bash
git add lib/openGauss-server
git commit -m "chore: initialize openGauss-server submodule"
```

---

## Iteration-2: CI/CD 质量门禁 (P1)

### Task 2.1: 添加 CI 测试 + Lint 工作流

**Files:**
- Create: `.github/workflows/ci.yml`

**Step 1: 创建 CI 工作流文件**

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: true
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: true
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - run: cargo clippy --all-features -- -D warnings

  fmt:
    name: Format
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --all -- --check
```

**Step 2: 提交**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: add test, clippy, and rustfmt checks"
```

---

### Task 2.2: 添加 rustfmt.toml 和 clippy.toml

**Files:**
- Create: `rustfmt.toml`
- Create: `clippy.toml`
- Modify: `Cargo.toml` (添加 `[lints]` 配置)

**Step 1: 创建 rustfmt.toml**

```toml
edition = "2021"
max_width = 120
tab_spaces = 4
use_small_heuristics = "Max"
```

**Step 2: 创建 clippy.toml**

```toml
too-many-arguments-threshold = 8
```

**Step 3: 在 Cargo.toml 添加 [lints] 配置**

```toml
[lints.clippy]
unwrap_used = "warn"
expect_used = "warn"
```

**Step 4: 提交**

```bash
git add rustfmt.toml clippy.toml Cargo.toml
git commit -m "chore: add rustfmt and clippy configuration"
```

---

## Iteration-3: 重构与去重 (P2)

### Task 3.1: 提取逗号分隔列表解析的通用辅助函数

**Files:**
- Modify: `src/parser/mod.rs` (添加通用函数)
- Modify: `src/parser/ddl/alter.rs` (替换 4 处重复)
- Modify: `src/parser/ddl/create.rs` (替换重复)

**Step 1: 在 mod.rs 添加通用辅助函数**

```rust
impl<'a> Parser<'a> {
    /// 解析逗号分隔的列表: item, item, item
    pub(crate) fn parse_comma_separated<T, F>(
        &mut self,
        parse_item: F,
        terminator: Option<Keyword>,
    ) -> Result<Vec<T>, ParserError>
    where
        F: Fn(&mut Self) -> Result<T, ParserError>,
    {
        let mut items = Vec::new();
        loop {
            items.push(parse_item(self)?);
            if !self.consume_token(&Token::Comma) {
                break;
            }
            // 检查是否到达终止关键字
            if let Some(kw) = terminator {
                if self.peek_keyword() == Some(kw) {
                    break;
                }
            }
        }
        Ok(items)
    }
}
```

**Step 2: 在 alter.rs 中使用新函数替换 4 处重复模式**

**Step 3: 运行测试验证**

```bash
cargo test
```
Expected: 所有测试通过。

**Step 4: 提交**

```bash
git commit -m "refactor(parser): extract comma-separated list parsing helper"
```

---

### Task 3.2: 统一 IF [NOT] EXISTS 解析

**Files:**
- Modify: `src/parser/mod.rs` (添加 `parse_if_exists` 和 `parse_if_not_exists`)
- Modify: `src/parser/ddl/alter.rs:33-42` (删除私有版本，使用 mod.rs 的)
- Modify: `src/parser/ddl/table.rs:773-785` (删除私有版本，使用 mod.rs 的)

**Step 1: 将 parse_if_exists/parse_if_not_exists 提升到 mod.rs 作为 pub(crate)**

**Step 2: 删除 alter.rs 和 table.rs 中的私有副本，改用 mod.rs 的版本**

**Step 3: 运行测试验证**

```bash
cargo test
```
Expected: 所有测试通过。

**Step 4: 提交**

```bash
git commit -m "refactor(parser): consolidate IF [NOT] EXISTS parsing helpers"
```

---

### Task 3.3: 为 MCP 模块添加测试

**Files:**
- Create: `src/mcp/tests.rs`
- Modify: `src/mcp/mod.rs` (添加 `#[cfg(test)] mod tests;`)

**Step 1: 创建测试框架**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_tool_list() {
        // 验证所有 MCP tool 已注册
    }

    #[test]
    fn test_mcp_parse_request() {
        // 验证 parse tool 的参数解析
    }

    #[test]
    fn test_mcp_format_request() {
        // 验证 format tool 的参数解析
    }
}
```

**Step 2: 在 mod.rs 顶部添加**

```rust
#[cfg(test)]
mod tests;
```

**Step 3: 运行测试验证**

```bash
cargo test --features mcp
```
Expected: 新测试通过。

**Step 4: 提交**

```bash
git commit -m "test(mcp): add basic tests for MCP module"
```

---

## 执行摘要

| Iteration | 任务数 | 预计时间 | 风险 |
|-----------|--------|----------|------|
| Iteration-1 (P0) | 7 tasks | 2-3h | 🟡 中 — 错误处理变更需要仔细测试 |
| Iteration-2 (P1) | 2 tasks | 30min | 🟢 低 — 纯配置文件变更 |
| Iteration-3 (P2) | 3 tasks | 2h | 🟢 低 — 重构，有测试保护 |
| **总计** | **12 tasks** | **4-6h** | — |

**预期效果**: 项目健康度从 6.6/10 → 8.2/10
