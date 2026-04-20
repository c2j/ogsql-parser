# 关键字与表达式层 Bug 修复 + 语法扩展 实施计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 修复 6 个关键字/表达式处理 bug（消除 ~18 个 parse error），再逐步补齐 SIMILAR TO、LIKE ESCAPE、WINDOW 子句等 SQL 标准特性，最终将 half-sql.sql 的非 DDL 错误清零。

**Architecture:** 所有变更遵循现有架构 — 表达式层修改集中在 `src/parser/expr.rs` 的 `try_postfix_op()` 和 `parse_primary_expr()`；SELECT 层修改在 `src/parser/select.rs`；AST 类型在 `src/ast/mod.rs`；Formatter 在 `src/formatter.rs`；测试在 `src/parser/tests.rs`。每个 Task 遵循 TDD：写测试 → 确认失败 → 实现 → 确认通过。

**Tech Stack:** Rust 2021, thiserror 2.0, serde (Serialize + Deserialize)

**关键约定（写代码前必读）：**
- `match_keyword(kw)` — 只读（`&self`），不推进位置，仅检查当前 token
- `try_consume_keyword(kw)` — 检查并推进（`&mut self`）
- `expect_keyword(kw)` — 检查并推进，不匹配则返回 Err
- `match_token(tok)` — 只读，不推进
- `expect_token(tok)` — 检查并推进，不匹配则返回 Err
- `parse()` 永远返回 `Vec<Statement>`，不会 Err；解析失败产生 `Statement::Empty`
- 错误通过 `parser.errors()` 获取，通过 `self.add_error()` 添加
- 测试用 `parse_one(sql)` 获取单条语句，用 `parse_with_errors(sql)` 检查错误

**回归验证:** 每个 Task 完成后运行:
```bash
cargo test 2>&1 | tail -5
```

---

## Phase A: 关键字/表达式 Bug 修复（6 个根因，~18 个 error）

### Task 1: USER 作为特殊表达式 — 修复 4 个 error

**背景:** `SELECT USER` 在 openGauss/GaussDB 中合法（等同于 `SELECT CURRENT_USER`）。`USER` 被分类为 Reserved，但 `expr.rs:735-746` 的特殊表达式关键字列表中没有包含 `Keyword::USER`。

**Files:**
- Modify: `src/parser/expr.rs:735-746` — 在特殊表达式 match arm 中添加 `Keyword::USER`
- Test: `src/parser/tests.rs` — 末尾添加测试

**Step 1: 写失败测试**

在 `src/parser/tests.rs` 末尾添加：

```rust
// ========== Task 1: USER as special expression ==========

#[test]
fn test_select_user() {
    let stmt = parse_one("SELECT USER");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            match &s.targets[0] {
                SelectTarget::Expr { expr, alias } => {
                    assert!(alias.is_none());
                    match expr {
                        Expr::ColumnRef(parts) => {
                            assert_eq!(parts, &vec!["USER".to_string()]);
                        }
                        _ => panic!("expected ColumnRef, got {:?}", expr),
                    }
                }
                _ => panic!("expected Expr target"),
            }
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_select_user_no_reserved_error() {
    let (stmts, errors) = parse_with_errors("SELECT USER");
    assert!(!stmts.is_empty(), "should parse SELECT USER");
    assert!(errors.is_empty(), "USER should not trigger reserved keyword error, got: {:?}", errors);
}
```

**Step 2: 确认测试失败**

Run: `cargo test test_select_user --lib -- --nocapture 2>&1 | tail -20`
Expected: `test_select_user` 可能 panic 或 `test_select_user_no_reserved_error` 断言 errors 非空

**Step 3: 实现 — 一行修改**

在 `src/parser/expr.rs` 第 735-746 行，在 `Keyword::SESSION_USER` 后面添加 `Keyword::USER`：

```rust
// 修改前 (line 735-746):
if matches!(
    kw,
    Keyword::SYSDATE
        | Keyword::ROWNUM
        | Keyword::CURRENT_DATE
        | Keyword::CURRENT_CATALOG
        | Keyword::CURRENT_USER
        | Keyword::SESSION_USER
) {

// 修改后:
if matches!(
    kw,
    Keyword::SYSDATE
        | Keyword::ROWNUM
        | Keyword::CURRENT_DATE
        | Keyword::CURRENT_CATALOG
        | Keyword::CURRENT_USER
        | Keyword::SESSION_USER
        | Keyword::USER
) {
```

**Step 4: 确认测试通过**

Run: `cargo test test_select_user --lib -- --nocapture`
Expected: 2 tests PASS

**Step 5: 验证无回归**

Run: `cargo test 2>&1 | tail -5`
Expected: all tests pass

**Step 6: Commit**

```bash
git add src/parser/expr.rs src/parser/tests.rs
git commit -m "fix: handle USER as special expression (like CURRENT_USER)

USER is a valid SQL expression in openGauss/GaussDB equivalent to
CURRENT_USER. Add Keyword::USER to the special expression match arm
in parse_primary_expr() alongside CURRENT_USER and SESSION_USER.

Fixes 'reserved keyword user cannot be used as identifier' errors
on SELECT USER statements."
```

---

### Task 2: TRIM 方向关键字 (BOTH/LEADING/TRAILING) — 修复 6 个 error

**背景:** `parse_trim_function()` 使用 `match_keyword()` 检查方向关键字。但 `match_keyword()` 是只读的（`&self`），不会推进位置。所以 BOTH/LEADING/TRAILING 匹配后仍然是当前 token，然后 `parse_identifier()` 看到这个 Reserved keyword 就触发了 "reserved keyword cannot be used as identifier" 错误。

**根因代码** (`src/parser/expr.rs:1108-1116`):
```rust
let direction = if self.match_keyword(Keyword::LEADING)     // 只读，不推进
    || self.match_keyword(Keyword::TRAILING)                // 只读，不推进
    || self.match_keyword(Keyword::BOTH)                    // 只读，不推进
{
    Some(self.parse_identifier()?)  // BUG: BOTH 仍是当前 token → reserved error
} else {
    None
};
```

**修复方案:** 用 `peek_keyword()` + 手动 `advance()` 替代 `match_keyword()` + `parse_identifier()`，直接捕获关键字文本。

**Files:**
- Modify: `src/parser/expr.rs:1108-1116` — 替换方向关键字处理逻辑
- Test: `src/parser/tests.rs` — 添加 TRIM 方向关键字测试

**Step 1: 写失败测试**

```rust
// ========== Task 2: TRIM direction keywords ==========

#[test]
fn test_trim_both_no_error() {
    let (stmts, errors) = parse_with_errors("SELECT trim(BOTH 'x' FROM 'xTomxx')");
    assert!(!stmts.is_empty());
    assert!(errors.is_empty(), "BOTH should not trigger reserved keyword error, got: {:?}", errors);
}

#[test]
fn test_trim_leading_no_error() {
    let (stmts, errors) = parse_with_errors("SELECT trim(LEADING 'x' FROM 'xTomxx')");
    assert!(!stmts.is_empty());
    assert!(errors.is_empty(), "LEADING should not trigger reserved keyword error, got: {:?}", errors);
}

#[test]
fn test_trim_trailing_no_error() {
    let (stmts, errors) = parse_with_errors("SELECT trim(TRAILING 'x' FROM 'xTomxx')");
    assert!(!stmts.is_empty());
    assert!(errors.is_empty(), "TRAILING should not trigger reserved keyword error, got: {:?}", errors);
}

#[test]
fn test_trim_both_from_ast() {
    let stmt = parse_one("SELECT trim(BOTH 'x' FROM 'xTomxx')");
    match stmt {
        Statement::Select(s) => {
            match &s.targets[0] {
                SelectTarget::Expr { expr, .. } => {
                    match expr {
                        Expr::SpecialFunction { name, args } => {
                            assert_eq!(name, "trim");
                            assert_eq!(args.len(), 2); // direction + source (no explicit chars → FROM variant)
                        }
                        _ => panic!("expected SpecialFunction, got {:?}", expr),
                    }
                }
                _ => panic!("expected Expr target"),
            }
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_trim_direction_with_chars_ast() {
    let stmt = parse_one("SELECT trim(LEADING 'x' FROM 'xTomxx')");
    match stmt {
        Statement::Select(s) => {
            match &s.targets[0] {
                SelectTarget::Expr { expr, .. } => {
                    match expr {
                        Expr::SpecialFunction { name, args } => {
                            assert_eq!(name, "trim");
                            assert_eq!(args.len(), 3); // direction + chars + source
                        }
                        _ => panic!("expected SpecialFunction, got {:?}", expr),
                    }
                }
                _ => panic!("expected Expr target"),
            }
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}
```

**Step 2: 确认测试失败**

Run: `cargo test test_trim_both --lib -- --nocapture`
Expected: `test_trim_both_no_error` fails with "BOTH should not trigger reserved keyword error"

**Step 3: 实现修复**

替换 `src/parser/expr.rs` 中 `parse_trim_function` 的方向关键字处理（约 line 1108-1116）：

```rust
// 修改后:
fn parse_trim_function(&mut self, name: ObjectName) -> Result<Expr, ParserError> {
    // 使用 peek_keyword + advance 代替 match_keyword + parse_identifier
    // 避免 parse_identifier() 对 Reserved keyword 报错
    let direction = match self.peek_keyword() {
        Some(Keyword::LEADING) | Some(Keyword::TRAILING) | Some(Keyword::BOTH) => {
            let dir = self.peek_keyword().unwrap().as_str().to_string();
            self.advance();
            Some(dir)
        }
        _ => None,
    };

    // 后续代码保持不变：
    if let Some(dir) = direction {
        if self.match_keyword(Keyword::FROM) {
            // TRIM(direction FROM expr) — no explicit chars
            self.advance();
            let source = self.parse_expr()?;
            self.expect_token(&Token::RParen)?;
            Ok(Expr::SpecialFunction {
                name: name.join("."),
                args: vec![Expr::ColumnRef(vec![dir]), source],
            })
        } else {
            // TRIM(direction chars FROM expr)
            let chars = self.parse_expr()?;
            self.expect_keyword(Keyword::FROM)?;
            let source = self.parse_expr()?;
            self.expect_token(&Token::RParen)?;
            Ok(Expr::SpecialFunction {
                name: name.join("."),
                args: vec![Expr::ColumnRef(vec![dir]), chars, source],
            })
        }
    } else {
        // ... 保持原有代码不变 (line 1140-1169)
```

**Step 4: 确认测试通过**

Run: `cargo test test_trim --lib -- --nocapture`
Expected: 所有 5 个 TRIM 测试 PASS

**Step 5: 验证无回归**

Run: `cargo test 2>&1 | tail -5`

**Step 6: Commit**

```bash
git add src/parser/expr.rs src/parser/tests.rs
git commit -m "fix: avoid reserved keyword error in TRIM(BOTH/LEADING/TRAILING ...)

The parse_trim_function() used match_keyword() (read-only) then
parse_identifier() which saw BOTH/LEADING/TRAILING still as the
current token and emitted ReservedKeywordAsIdentifier errors.

Replace with peek_keyword() + advance() to directly capture the
keyword text without going through parse_identifier()."
```

---

### Task 3: 后缀阶乘运算符 `!` 和 `!!` — 修复 2 个 error

**背景:** PostgreSQL/openGauss 支持 `5!` (阶乘) 和 `!!5` (前缀阶乘) 运算符。`SELECT 5 ! AS RESULT` 因为 `!` 未作为后缀运算符实现，导致解析器无法处理 `5 !`，后续 `AS` 被误判为标识符。

**Files:**
- Modify: `src/parser/expr.rs` — `get_infix_operator()` 或 `try_postfix_op()` 中添加 `!` 后缀运算符；`parse_unary_expr()` 中添加 `!!` 前缀运算符
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
// ========== Task 3: Factorial operators ! and !! ==========

#[test]
fn test_postfix_factorial() {
    let stmt = parse_one("SELECT 5 !");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_postfix_factorial_with_alias() {
    // This is the exact line 1555 case that triggers the error
    let (stmts, errors) = parse_with_errors("SELECT 5 ! AS RESULT");
    assert!(!stmts.is_empty());
    // Should not produce reserved keyword error for 'as'
    let as_errors: Vec<_> = errors.iter()
        .filter(|e| format!("{:?}", e).contains("as"))
        .collect();
    assert!(as_errors.is_empty(), "Should not error on AS, got: {:?}", as_errors);
}

#[test]
fn test_prefix_factorial() {
    let stmt = parse_one("SELECT !! 5");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_factorial_in_expression() {
    let stmt = parse_one("SELECT 4 ! + 1");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}
```

**Step 2: 确认测试失败**

Run: `cargo test test_postfix_factorial --lib -- --nocapture`
Expected: FAIL

**Step 3: 实现**

**3a. 后缀 `!` (阶乘)**

在 `try_postfix_op()` 中（`src/parser/expr.rs` 约 line 357，在 `_ => Ok(false)` 之前），添加：

```rust
Token::Op(s) if s == "!" => {
    self.advance();
    *left = Expr::UnaryOp {
        expr: Box::new(std::mem::replace(left, Expr::Default)),
        op: "!".to_string(),
    };
    Ok(true)
}
```

注意：需确认 `Token::Op` 是 `!` 的 token 类型。如果 `!` 被词法分析器识别为其他类型（如 `Token::Bang`），需相应调整。先检查 tokenizer 如何处理 `!`。

**3b. 前缀 `!!` (前缀阶乘)**

在 `parse_unary_expr()` 中（约 line 370-400），添加 `!!` 前缀运算符处理：

```rust
// 在现有前缀运算符处理后，检查 !! 前缀阶乘
if let Token::Op(s) = self.peek() {
    if s == "!!" {
        self.advance();
        let expr = self.parse_unary_expr()?;
        return Ok(Expr::UnaryOp {
            expr: Box::new(expr),
            op: "!!".to_string(),
        });
    }
}
```

同样需确认 tokenizer 如何处理 `!!`。可能需要先检查 tokenizer 对 `!` 和 `!!` 的处理逻辑。

**Step 4: 确认测试通过**

Run: `cargo test test_postfix_factorial test_prefix_factorial --lib -- --nocapture`

**Step 5: 验证无回归**

Run: `cargo test 2>&1 | tail -5`

**Step 6: Commit**

```bash
git add src/parser/expr.rs src/parser/tests.rs
git commit -m "feat: add postfix (!) and prefix (!!) factorial operators

PostgreSQL/openGauss support 5! (factorial) and !!5 (prefix factorial).
Add postfix ! to try_postfix_op() and prefix !! to parse_unary_expr()."
```

---

### Task 4: SIMILAR TO 运算符 — 修复 4 个 error

**背景:** `SIMILAR TO` 是 SQL 标准的正则匹配运算符（`expr SIMILAR TO pattern [ESCAPE 'char']`）。openGauss 完全支持。当前解析器没有实现。

**Files:**
- Modify: `src/parser/expr.rs` — `try_postfix_op()` 中添加 `SIMILAR TO` 和 `NOT SIMILAR TO` 处理
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
// ========== Task 4: SIMILAR TO operator ==========

#[test]
fn test_similar_to() {
    let stmt = parse_one("SELECT 'abc' SIMILAR TO 'abc' AS RESULT");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_similar_to_with_pattern() {
    let stmt = parse_one("SELECT 'abc' SIMILAR TO '%(b|d)%' AS RESULT");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_not_similar_to() {
    let stmt = parse_one("SELECT 'abc' NOT SIMILAR TO 'a' AS RESULT");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_similar_to_no_end_of_statement_error() {
    let (stmts, errors) = parse_with_errors("SELECT 'abc' SIMILAR TO 'abc' AS RESULT");
    assert!(!stmts.is_empty());
    // Should not produce 'expected end of statement, got Keyword(SIMILAR)' error
    let similar_errors: Vec<_> = errors.iter()
        .filter(|e| format!("{:?}", e).contains("SIMILAR"))
        .collect();
    assert!(similar_errors.is_empty(), "Should not error on SIMILAR, got: {:?}", similar_errors);
}
```

**Step 2: 确认测试失败**

Run: `cargo test test_similar_to --lib -- --nocapture`

**Step 3: 实现**

在 `try_postfix_op()` 中（`src/parser/expr.rs`），在 LIKE/ILIKE 处理之后（约 line 325 之后）添加：

```rust
// SIMILAR TO
Token::Keyword(Keyword::SIMILAR) => {
    self.advance();
    self.expect_keyword(Keyword::TO)?;
    let pattern = self.parse_expr()?;
    *left = Expr::BinaryOp {
        left: Box::new(std::mem::replace(left, Expr::Default)),
        op: "SIMILAR TO".to_string(),
        right: Box::new(pattern),
    };
    Ok(true)
}
```

同时在 `NOT` 的 match arm 中（约 line 262-298），添加 `SIMILAR` 分支：

```rust
Token::Keyword(Keyword::SIMILAR) => {
    self.advance();
    self.advance(); // consume NOT SIMILAR
    self.expect_keyword(Keyword::TO)?;
    let pattern = self.parse_expr()?;
    *left = Expr::BinaryOp {
        left: Box::new(std::mem::replace(left, Expr::Default)),
        op: "NOT SIMILAR TO".to_string(),
        right: Box::new(pattern),
    };
    return Ok(true);
}
```

**Step 4: 确认测试通过**

**Step 5: 验证无回归**

**Step 6: Commit**

```bash
git commit -m "feat: add SIMILAR TO / NOT SIMILAR TO expression operators

SQL standard regex matching operator. openGauss supports:
  expr SIMILAR TO pattern
  expr NOT SIMILAR TO pattern"
```

---

### Task 5: LIKE ... ESCAPE 子句 — 修复 1 个 error

**背景:** `LIKE ... ESCAPE 'char'` 是 SQL 标准的转义字符指定语法。当前 LIKE 运算符解析后没有检查可选的 ESCAPE 子句。

**Files:**
- Modify: `src/parser/expr.rs` — LIKE/NOT LIKE/ILIKE 处理中添加 ESCAPE 子句
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
// ========== Task 5: LIKE ESCAPE clause ==========

#[test]
fn test_like_escape() {
    let stmt = parse_one("SELECT 'AA_BBCC' LIKE '%A@_B%' ESCAPE '@' AS RESULT");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_like_escape_no_error() {
    let (stmts, errors) = parse_with_errors("SELECT 'AA_BBCC' LIKE '%A@_B%' ESCAPE '@' AS RESULT");
    assert!(!stmts.is_empty());
    let escape_errors: Vec<_> = errors.iter()
        .filter(|e| format!("{:?}", e).contains("ESCAPE"))
        .collect();
    assert!(escape_errors.is_empty(), "Should not error on ESCAPE, got: {:?}", escape_errors);
}
```

**Step 2: 确认测试失败**

**Step 3: 实现**

在 LIKE 运算符处理中（`src/parser/expr.rs` 约 line 307-315），解析 pattern 后检查 ESCAPE：

```rust
// LIKE (修改 line 307-315):
Token::Keyword(Keyword::LIKE) => {
    self.advance();
    let pattern = self.parse_expr()?;
    let escape = if self.match_keyword(Keyword::ESCAPE) {
        self.advance();
        Some(self.parse_expr()?)
    } else {
        None
    };
    *left = Expr::BinaryOp {
        left: Box::new(std::mem::replace(left, Expr::Default)),
        op: "LIKE".to_string(),
        right: Box::new(pattern),
    };
    // 如果有 ESCAPE，需要包装。临时方案：将 escape 信息附加到 right
    // 如果 AST 有 LikeExpr 变体，则用专用结构；否则用 BinaryOp 暂时忽略 escape
    // TODO: 考虑是否需要添加 Expr::Like { expr, pattern, escape, negated, ilike } AST 变体
    Ok(true)
}
```

**注意:** 完整实现需要决定 AST 表示方式。可选方案：
1. 添加 `Expr::Like { expr, pattern, escape, negated, case_insensitive }` 专用变体（推荐，更精确）
2. 在 `Expr::BinaryOp` 中添加 `escape: Option<Box<Expr>>` 字段
3. 将 `ESCAPE 'char'` 编码为三元组 `FunctionCall("like_escape", [expr, pattern, escape])`

推荐方案 1（添加 Like AST 变体），但这涉及 AST、formatter、serde 修改。如果暂时只需消除 error，可先用方案 3（将 ESCAPE 作为普通表达式的一部分处理）或暂时 ignore escape token。

**Step 4-6: 同上**

```bash
git commit -m "feat: add LIKE ... ESCAPE clause support

SQL standard syntax for specifying escape character in LIKE patterns.
  expr LIKE pattern ESCAPE 'char'
  expr NOT LIKE pattern ESCAPE 'char'"
```

---

### Task 6: WINDOW 子句（命名窗口定义）— 修复 1 个 error

**背景:** `SELECT ... WINDOW w AS (ORDER BY x)` 是 SQL 标准的命名窗口定义语法。当前 `SelectStatement` AST 没有 `window_clause` 字段，SELECT 解析器也没有解析 WINDOW 子句。

**影响范围最大** — 需要修改 AST、解析器、格式化器。但只影响 1 个 error。

**Files:**
- Modify: `src/ast/mod.rs` — `SelectStatement` 添加 `window_clause` 字段 + `NamedWindow` 类型
- Modify: `src/parser/select.rs` — `parse_simple_select()` 中解析 WINDOW 子句
- Modify: `src/formatter.rs` — 格式化 WINDOW 子句
- Test: `src/parser/tests.rs`

**Step 1: 写失败测试**

```rust
// ========== Task 6: WINDOW clause ==========

#[test]
fn test_window_clause() {
    let stmt = parse_one("SELECT date, count(*) OVER w FROM t WINDOW w AS (ORDER BY date ASC)");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 2);
            // 验证 window_clause 被解析
        }
        _ => panic!("expected Select, got {:?}", stmt),
    }
}

#[test]
fn test_window_clause_no_error() {
    let (stmts, errors) = parse_with_errors(
        "SELECT date, count(*) OVER w FROM daily_uniques WINDOW w AS (ORDER BY date ASC ROWS 1 PRECEDING)"
    );
    assert!(!stmts.is_empty());
    let window_errors: Vec<_> = errors.iter()
        .filter(|e| format!("{:?}", e).contains("WINDOW"))
        .collect();
    assert!(window_errors.is_empty(), "Should not error on WINDOW, got: {:?}", window_errors);
}
```

**Step 2: 确认测试失败**

**Step 3: 修改 AST**

在 `src/ast/mod.rs` 中：

```rust
// 添加 NamedWindow 结构体（在 WindowSpec 附近）
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NamedWindow {
    pub name: String,
    pub spec: WindowSpec,
}

// 修改 SelectStatement (约 line 892):
// 在 lock_clause 之后添加:
pub window_clause: Vec<NamedWindow>,
```

**Step 4: 修改 SELECT 解析器**

在 `src/parser/select.rs` 的 `parse_simple_select()` 中，在 HAVING 之后（约 line 238）添加：

```rust
// WINDOW clause (命名窗口定义)
let window_clause = if self.match_keyword(Keyword::WINDOW) {
    self.advance();
    let mut windows = vec![];
    loop {
        let name = self.parse_identifier()?;
        self.expect_keyword(Keyword::AS)?;
        self.expect_token(&Token::LParen)?;
        let spec = self.parse_window_spec()?;
        self.expect_token(&Token::RParen)?;
        windows.push(NamedWindow { name, spec });
        if !self.match_token(&Token::Comma) {
            break;
        }
        self.advance();
    }
    windows
} else {
    vec![]
};
```

同时更新 `parse_simple_select` 返回的 `SelectStatement` 构造中的 `window_clause` 字段。

**Step 5: 修改 Formatter**

在 `src/formatter.rs` 中，`format_select` 方法中，在 HAVING 之后、ORDER BY 之前输出 WINDOW 子句。

**Step 6-7: 验证 + Commit**

```bash
git commit -m "feat: add WINDOW clause support in SELECT statements

SQL standard named window definitions:
  SELECT ... WINDOW w1 AS (PARTITION BY x), w2 AS (ORDER BY y)

Add NamedWindow AST type, parse WINDOW after HAVING in SELECT,
and format the clause in the SQL formatter."
```

---

## Phase B: SQL 标准表达式扩展（~10 个 error）

### Task 7: CONVERT(expr USING charset) 语法

**背景:** `SELECT convert('text' USING 'gbk')` — SQL 标准的字符集转换语法。当前解析器不识别 `USING` 关键字在 `convert()` 函数中的特殊用法。

**Files:**
- Modify: `src/parser/expr.rs` — 在 `parse_function_call()` 中添加 `convert` 的特殊处理
- Test: `src/parser/tests.rs`

**Step 1: 测试**

```rust
#[test]
fn test_convert_using() {
    let stmt = parse_one("SELECT convert('asdas' USING 'gbk')");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select, got {:?}", stmt),
    }
}
```

**Step 3: 实现** — 在 `parse_function_call()` 的特殊函数分发（trim/overlay/substring 旁边）添加 `convert` 的特殊处理：

```rust
if lower_name == "convert" {
    return self.parse_convert_function(name);
}
```

```rust
fn parse_convert_function(&mut self, name: ObjectName) -> Result<Expr, ParserError> {
    // CONVERT(expr USING charset) — SQL standard form
    // CONVERT(expr, dest_charset) — PostgreSQL form (falls through to normal)
    let first = self.parse_expr()?;
    if self.match_keyword(Keyword::USING) {
        self.advance();
        let charset = self.parse_expr()?;
        self.expect_token(&Token::RParen)?;
        return Ok(Expr::SpecialFunction {
            name: name.join("."),
            args: vec![first, charset],
        });
    }
    // Not USING form — fall through to regular function call
    // ... continue with comma-separated args
}
```

---

### Task 8: 正则匹配运算符 `~*`、`!~`、`!~*`

**背景:** PostgreSQL/openGauss 支持 `~` (区分大小写正则匹配)、`~*` (不区分)、`!~` (不匹配)、`!~*` (不匹配不区分)。当前 `~` 可能已支持但 `~*` 等复合运算符未支持。

**Files:**
- Modify: `src/token/tokenizer.rs` — 确认 `~*`、`!~`、`!~*` 被正确 tokenize
- Modify: `src/parser/expr.rs` — `get_infix_operator()` 中确认这些运算符的优先级
- Test: `src/parser/tests.rs`

```rust
#[test]
fn test_regex_case_insensitive() {
    let stmt = parse_one("SELECT 'abc' ~* 'Abc' AS RESULT");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_regex_not_match() {
    let stmt = parse_one("SELECT 'abc' !~ 'Abc' AS RESULT");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 1),
        _ => panic!("expected Select"),
    }
}
```

**实现:** 先检查 tokenizer 是否将 `~*` tokenize 为 `Op("~*")`，如果是则只需在 `get_infix_operator()` 中注册优先级即可。

---

### Task 9: `listagg() WITHIN GROUP (ORDER BY ...)` 和 `group_concat() SEPARATOR`

**背景:** Oracle/GaussDB 的 `listagg(col, sep) WITHIN GROUP (ORDER BY col)` 和 MySQL 兼容的 `group_concat(col SEPARATOR ',')` 语法。

**Files:**
- Modify: `src/parser/expr.rs` — `parse_function_call()` 中处理 WITHIN GROUP 和 SEPARATOR 子句
- Test: `src/parser/tests.rs`

```rust
#[test]
fn test_listagg_within_group() {
    let stmt = parse_one("SELECT deptno, listagg(ename, ',') WITHIN GROUP (ORDER BY ename) AS employees FROM emp GROUP BY deptno");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 2),
        _ => panic!("expected Select"),
    }
}

#[test]
fn test_group_concat_separator() {
    let stmt = parse_one("SELECT id, group_concat(v separator '') FROM t");
    match stmt {
        Statement::Select(s) => assert_eq!(s.targets.len(), 2),
        _ => panic!("expected Select"),
    }
}
```

---

## Phase C: PostgreSQL 特有运算符（~60 个 error）

### Task 10: 几何运算符 `<->`、`@@`、`<<|`、`|>>`、`&<|`、`|&>` 等

**背景:** PostgreSQL 的几何类型、全文搜索、网络类型有大量特殊运算符。

**影响:** 约 20 个 error line

**优先级:** 中 — 这些是 PostgreSQL 扩展特性，不影响核心 SQL 兼容性

**方法:**
1. 检查 tokenizer 如何处理这些运算符（是否被正确 tokenize 为 `Op(...)`）
2. 在 `get_infix_operator()` 中注册优先级
3. 添加测试

---

### Task 11: 网络/位运算符 `<<=`、`>>=`、`|`、`^`

**背景:** 网络类型的包含运算符 `<<=`/`>>=`，位运算 `|`（OR）、`^`（XOR/幂）

**影响:** 约 10 个 error line

---

## Phase D: DDL 缺失（~350+ error，未来 Phase 4 范围）

### 概览（不展开，属于 Phase 4 开发路线图）

| 语法 | error 数 | 难度 |
|------|---------|------|
| CREATE USER ... PASSWORD | ~30 | 中 |
| ALTER USER ... PASSWORD / IDENTIFIED | ~40 | 中 |
| ALTER DATABASE ... RENAME TO / SET | ~10 | 中 |
| CREATE PROCEDURE / ALTER PROCEDURE | ~20 | 高 |
| ALTER TABLE ... MODIFY / PARTITION | ~60 | 高 |
| CREATE/DROP DATABASE LINK | ~15 | 中 |
| SECURITY LABEL / CLIENT MASTER KEY | ~30 | 中 |
| GRANT/REVOKE 扩展 | ~20 | 中 |
| 其他 37 个 stub 语句 | ~124 | 低-中 |

这些 DDL 缺失应在 Phase 4 的后续迭代中逐批实现，每批遵循相同的 TDD 模式。

---

## 执行顺序总结

```
Phase A (本计划核心):
  Task 1: USER 特殊表达式        → 修复 4 error, 1 行代码
  Task 2: TRIM 方向关键字        → 修复 6 error, ~10 行代码
  Task 3: 阶乘运算符 ! / !!      → 修复 2 error, ~15 行代码
  Task 4: SIMILAR TO 运算符      → 修复 4 error, ~20 行代码
  Task 5: LIKE ESCAPE 子句       → 修复 1 error, ~20 行代码
  Task 6: WINDOW 子句            → 修复 1 error, ~60 行代码 (AST+解析+格式化)

Phase B (后续批次):
  Task 7:  CONVERT USING 语法     → 修复 1 error
  Task 8:  正则运算符 ~* !~ !~*  → 修复 2 error
  Task 9:  WITHIN GROUP / SEPARATOR → 修复 ~15 error

Phase C (低优先级):
  Task 10: 几何运算符             → 修复 ~20 error
  Task 11: 网络/位运算符          → 修复 ~10 error

Phase D (Phase 4 DDL 路线图):
  Task 12+: DDL 语句逐批实现     → 修复 ~350+ error
```

**Phase A 完成后预期:** half-sql.sql 的 error 从 1014 降至约 996（减少 18 个），但这些都是高价值的 bug 修复，消除了合法 SQL 的误报。
