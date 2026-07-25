# EXECUTE IMMEDIATE 字符串内容二次解析 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 当 EXECUTE IMMEDIATE 的动态 SQL 是字符串字面量时，自动提取内容并二次解析为结构化 AST，使 `PlExecuteStmt.parsed_query` 可用于分析。

**Architecture:** 遵循代码库中已有的 `parsed_query` 双字段模式（PlCursorDecl/Perform/PlForKind/PrepareStatement 同款）。新增 `parse_statement_from_str()` 工具函数（参考已有的 `parse_pl_block_from_str`），在 `parse_pl_execute()` 中检测 `string_expr` 是否为字符串字面量，是则提取内容重新 tokenize+parse。

**Tech Stack:** Rust 2021, serde

---

## 背景知识

### 已有模式（5 处一致）

代码库中已有 5 个地方使用了"原始文本 + 可选解析 AST"双字段模式：

```
PlCursorDecl      { query: String,          parsed_query: Option<Box<Statement>> }
PlForKind::Query   { query: String,          parsed_query: Option<Box<Statement>> }
PlStatement::Perform { query: String,        parsed_query: Option<Box<Statement>> }
PlOpenKind::ForQuery { query: String,        parsed_query: Option<Box<Statement>> }
PrepareStatement   { statement: String,      parsed_statement: Option<Box<Statement>> }
```

### 关键函数

- `parse_pl_block_from_str(input: &str)` — utility.rs:2803 — 已有的子字符串重解析（tokenize + parse PL block）
- `Parser::parse_sql(input: &str)` — mod.rs:54 — tokenize + parse 完整 SQL（返回 Vec<StatementInfo>）
- `parse_statement()` — mod.rs:567 — 顶层语句分发器（支持所有 130+ 语句类型，包括 CALL）

### string_expr 三种情况

| 类型 | 能否解析 | AST 形式 |
|------|---------|---------|
| 字符串字面量 `'call ...'` | ✅ | `Expr::Literal(Literal::String(s))` |
| 变量 `plsql_block` | ❌ | `Expr::ColumnRef(["plsql_block"])` |
| 拼接 `'SEL' \|\| tab` | ❌ | `Expr::BinaryOp { .. }` |

只有第一种情况 `parsed_query` 有值。

---

## Task 1: AST — 为 PlExecuteStmt 添加 parsed_query 字段

**Files:**
- Modify: `src/ast/plpgsql.rs:357-369` (PlExecuteStmt)

**Step 1: 添加 parsed_query 字段**

在 `src/ast/plpgsql.rs` 中，将 `PlExecuteStmt` 从：

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlExecuteStmt {
    /// Whether the IMMEDIATE keyword was present
    pub immediate: bool,
    /// The dynamic SQL string expression (may be a concatenation)
    pub string_expr: crate::ast::Expr,
    /// INTO target variables for query results
    #[serde(default)]
    pub into_targets: Vec<crate::ast::Expr>,
    /// USING arguments with IN/OUT/INOUT mode
    #[serde(default)]
    pub using_args: Vec<PlUsingArg>,
}
```

改为：

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlExecuteStmt {
    /// Whether the IMMEDIATE keyword was present
    pub immediate: bool,
    /// The dynamic SQL string expression (may be a concatenation)
    pub string_expr: crate::ast::Expr,
    /// INTO target variables for query results
    #[serde(default)]
    pub into_targets: Vec<crate::ast::Expr>,
    /// USING arguments with IN/OUT/INOUT mode
    #[serde(default)]
    pub using_args: Vec<PlUsingArg>,
    /// Re-parsed AST when string_expr is a literal string (None for variables/concatenations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed_query: Option<Box<crate::ast::Statement>>,
}
```

**Serde 说明：** `#[serde(skip_serializing_if = "Option::is_none")]` 确保旧 JSON（没有 `parsed_query` 字段）反序列化时得到 `None`，新 JSON 在值为 `None` 时不输出该字段。这是代码库中 `PlCursorDecl.parsed_query` 等字段的完全一致用法。

**Step 2: 编译验证**

Run: `cargo check 2>&1 | grep "error" | head -10`

预期：编译报错指向 `parse_pl_execute()` 中构造 `PlExecuteStmt` 的地方（缺少 `parsed_query` 字段）。这是正常的，Task 2 修复。

**Step 3: Commit**

```bash
git add src/ast/plpgsql.rs
git commit -m "feat(ast): add parsed_query field to PlExecuteStmt for dynamic SQL re-parsing"
```

---

## Task 2: 新增 parse_statement_from_str 工具函数

**Files:**
- Modify: `src/parser/utility.rs` (在 `parse_pl_block_from_str` 旁边)

**Step 1: 添加函数**

在 `src/parser/utility.rs` 的 `parse_pl_block_from_str`（第 2803 行）之后添加：

```rust
    pub(crate) fn parse_statement_from_str(
        input: &str,
    ) -> Option<Box<crate::ast::Statement>> {
        let tokens = match crate::token::tokenizer::Tokenizer::new(input).tokenize() {
            Ok(t) => t,
            Err(_) => return None,
        };
        let mut parser = Parser::new(tokens);
        match parser.parse_statement() {
            Ok(stmt) => Some(Box::new(stmt)),
            Err(_) => None,
        }
    }
```

**设计要点：**
- 返回 `Option` 而非 `Result` — 解析失败是正常的（字符串内容可能不是有效 SQL）
- 使用 `parse_statement()` 而非 `try_parse_dml_statement()` — 前者支持所有 130+ 语句类型（包括 CALL、DDL 等），后者只支持 DML
- 不消费尾部分号 — `parse_statement()` 内部已处理

**Step 2: 编译验证**

Run: `cargo check 2>&1 | grep "error" | head -5`

**Step 3: Commit**

```bash
git add src/parser/utility.rs
git commit -m "feat(parser): add parse_statement_from_str utility for re-parsing string literals"
```

---

## Task 3: 增强 parse_pl_execute 实现字符串内容二次解析

**Files:**
- Modify: `src/parser/plpgsql.rs:1119-1177` (parse_pl_execute)

**Step 1: 添加测试**

在 `src/parser/tests.rs` 的 EXECUTE 测试组（`test_plpgsql_execute_concat_expr` 之后）添加：

```rust
#[test]
fn test_plpgsql_execute_string_literal_parsed() {
    let block = parse_do_block(
        "DO $$ BEGIN EXECUTE IMMEDIATE 'call calc_stats($1, $1, $2, $1)'; END $$"
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert!(e.parsed_query.is_some(), "string literal should be re-parsed");
            let inner = e.parsed_query.as_ref().unwrap();
            match inner.as_ref() {
                crate::ast::Statement::Call(c) => {
                    assert_eq!(c.name, vec!["calc_stats".to_string()]);
                    assert_eq!(c.args.len(), 4);
                }
                other => panic!("expected Call statement, got {:?}", other),
            }
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_variable_not_parsed() {
    let block = parse_do_block(
        "DO $$ BEGIN EXECUTE IMMEDIATE plsql_block USING a, b; END $$"
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert!(e.parsed_query.is_none(), "variable should NOT be re-parsed");
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_concat_not_parsed() {
    let block = parse_do_block(
        "DO $$ BEGIN EXECUTE IMMEDIATE 'SELECT * FROM ' || tab_name; END $$"
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.immediate);
            assert!(e.parsed_query.is_none(), "concatenation should NOT be re-parsed");
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_dml_string_parsed() {
    let block = parse_do_block(
        "DO $$ BEGIN EXECUTE 'SELECT id, name FROM users WHERE id = 1'; END $$"
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(!e.immediate);
            assert!(e.parsed_query.is_some());
            let inner = e.parsed_query.as_ref().unwrap();
            assert!(matches!(inner.as_ref(), crate::ast::Statement::Select(_)));
        }
        _ => panic!("expected Execute"),
    }
}

#[test]
fn test_plpgsql_execute_invalid_sql_string_not_parsed() {
    let block = parse_do_block(
        "DO $$ BEGIN EXECUTE 'not valid sql at all !!!'; END $$"
    );
    match &block.body[0] {
        PlStatement::Execute(e) => {
            assert!(e.parsed_query.is_none(), "invalid SQL should gracefully fall back to None");
        }
        _ => panic!("expected Execute"),
    }
}
```

**Step 2: 运行测试确认失败**

Run: `cargo test test_plpgsql_execute_string_literal_parsed 2>&1 | tail -10`

预期：编译失败（`PlExecuteStmt` 缺少 `parsed_query` 字段）。

**Step 3: 修改 parse_pl_execute**

将 `src/parser/plpgsql.rs` 的 `parse_pl_execute()` 函数中，在 `let string_expr = self.parse_expr()?;` 之后、`let mut into_targets` 之前，插入字符串内容提取和二次解析逻辑。

完整替换 `parse_pl_execute` 函数：

```rust
    fn parse_pl_execute(&mut self) -> Result<PlStatement, ParserError> {
        self.advance(); // consume "execute"

        let immediate = self.try_consume_ident_str("immediate");

        let string_expr = self.parse_expr()?;

        // Re-parse string literal contents into structured AST
        let parsed_query = match &string_expr {
            Expr::Literal(Literal::String(s)) => {
                Self::parse_statement_from_str(s)
            }
            Expr::Literal(Literal::DollarString { body, .. }) => {
                Self::parse_statement_from_str(body)
            }
            _ => None, // variable reference, concatenation, etc.
        };

        let mut into_targets = Vec::new();
        if self.match_ident_str("into") {
            self.advance();
            loop {
                into_targets.push(self.parse_expr()?);
                if self.match_token(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let mut using_args = Vec::new();
        if self.match_ident_str("using") {
            self.advance();
            loop {
                let mode = if self.match_ident_str("in") {
                    self.advance();
                    if self.match_ident_str("out") {
                        self.advance();
                        PlUsingMode::InOut
                    } else {
                        PlUsingMode::In
                    }
                } else if self.match_ident_str("out") {
                    self.advance();
                    PlUsingMode::Out
                } else {
                    PlUsingMode::In
                };
                using_args.push(PlUsingArg {
                    mode,
                    argument: self.parse_expr()?,
                });
                if self.match_token(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.try_consume_semicolon();

        Ok(PlStatement::Execute(PlExecuteStmt {
            immediate,
            string_expr,
            into_targets,
            using_args,
            parsed_query,
        }))
    }
```

**关键变化：** 在 `parse_expr()` 之后，通过 pattern match 检查是否为 `Literal::String(s)` 或 `Literal::DollarString { body, .. }`，是则调用 `Self::parse_statement_from_str()` 二次解析。其他情况（变量引用、拼接表达式）`parsed_query` 为 `None`。

**Step 4: 运行测试**

Run: `cargo test test_plpgsql_execute 2>&1 | grep -E "^test|failures|test result"`

预期：所有 EXECUTE 测试通过（包括新增的 5 个）。

**Step 5: Commit**

```bash
git add src/parser/plpgsql.rs src/parser/tests.rs
git commit -m "feat(parser): re-parse EXECUTE IMMEDIATE string literal contents into structured AST"
```

---

## Task 4: 全量回归测试

**Files:**
- No modifications

**Step 1: 运行全部单元测试**

Run: `cargo test 2>&1 | tail -10`

预期：所有 247+ 测试通过。

**Step 2: 运行 clippy**

Run: `cargo clippy 2>&1 | grep "error" | head -5`

预期：零错误。

**Step 3: 验证 JSON serde 往返**

Run: 
```bash
echo "DO \$\$ BEGIN EXECUTE IMMEDIATE 'call calc_stats(\$1, \$1, \$2, \$1)'; END \$\$" | cargo run --quiet -- parse -j 2>&1 | grep -E '"parsed_query"|"Call"|"name"|"args"'
```

预期：JSON 输出中包含 `"parsed_query"` 字段，其值为 Call 语句的结构化 AST。

**Step 4: Commit（如有修复）**

---

## 影响范围总结

| 文件 | 改动类型 | 改动量 |
|------|---------|--------|
| `src/ast/plpgsql.rs` | 添加 `parsed_query` 字段 | +3行 |
| `src/parser/utility.rs` | 新增 `parse_statement_from_str()` | +15行 |
| `src/parser/plpgsql.rs` | `parse_pl_execute()` 增加二次解析逻辑 | +8行 |
| `src/parser/tests.rs` | 新增 5 个测试 | +60行 |
| **合计** | **4 个文件** | **~86行** |

## 不需要改动的文件

- `src/formatter.rs` — `parsed_query` 仅供分析用，格式化输出仍用 `string_expr`
- `src/ast/visitor.rs` — 已通过 `PlStatement::Execute` 覆盖
- `src/token/tokenizer.rs` — 无需新 Token 类型
- `Cargo.toml` — 无新依赖
