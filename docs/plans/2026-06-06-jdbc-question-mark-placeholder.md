# JDBC `?` Placeholder Support Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Support JDBC-style `?` positional parameter placeholders when `mybatis_params: true` is enabled.

**Architecture:** Mirror the existing MyBatis param pattern — new `JdbcParam` token/expr variant, conditional tokenizer branching, parser match arm, formatter output. No new flags; reuse `mybatis_params` to control all three placeholder types (`#{}`, `${}`, `?`).

**Tech Stack:** Rust, recursive descent parser, existing token/AST infrastructure.

---

### Task 1: Token Layer — Add `JdbcParam` variant

**Files:**
- Modify: `src/token/mod.rs:100-103`

**Step 1: Add token variant**

In `src/token/mod.rs`, after `MyBatisRawExpr(String)` (line 103), add:

```rust
    /// JDBC-style positional parameter placeholder: ?
    JdbcParam,
```

This goes in the `// --- Parameters ---` section alongside `Param`, `MyBatisParam`, `MyBatisRawExpr`.

**Step 2: Verify compilation**

Run: `cargo check 2>&1 | head -30`
Expected: Compilation errors in downstream files (tokenizer, parser, etc.) because they don't handle `JdbcParam` yet — that's expected and will be fixed in subsequent tasks.

---

### Task 2: Tokenizer — Conditional `?` handling

**Files:**
- Modify: `src/token/tokenizer.rs:708` (operator branch)

**Step 1: Split `?` from the operator branch**

Current code at line 708:
```rust
'~' | '&' | '|' | '`' | '?' => {
    self.advance();
    let start = self.pos - c.len_utf8();
    // ... scan multi-char op ...
}
```

Change to:
```rust
'~' | '&' | '|' | '`' => {
    self.advance();
    let start = self.pos - c.len_utf8();
    while let Some(&nc) = self.chars.peek() {
        if is_op_char(nc) {
            self.chars.next();
            self.pos += nc.len_utf8();
        } else {
            break;
        }
    }
    let op_str = &self.input[start..self.pos];
    if op_str == "||" {
        Token::OpConcat
    } else {
        Token::Op(op_str.to_string())
    }
}

'?' => {
    if self.mybatis_params {
        self.advance();
        Token::JdbcParam
    } else {
        self.advance();
        let start = self.pos - 1;
        while let Some(&nc) = self.chars.peek() {
            if is_op_char(nc) {
                self.chars.next();
                self.pos += nc.len_utf8();
            } else {
                break;
            }
        }
        let op_str = &self.input[start..self.pos];
        Token::Op(op_str.to_string())
    }
}
```

Note: The `'~' | '&' | '|' | '\`'` branch body is identical to the original combined branch body (just copy it). The `'?'` branch is new with conditional logic.

**Step 2: Verify compilation**

Run: `cargo check 2>&1 | head -30`
Expected: Fewer errors than Task 1 — tokenizer is resolved.

---

### Task 3: AST Layer — Add `JdbcParam` expr variant

**Files:**
- Modify: `src/ast/mod.rs:1267`

**Step 1: Add Expr variant**

After `MyBatisRawExpr(String)` (line 1267), add:

```rust
    /// JDBC-style positional parameter: ?
    JdbcParam,
```

**Step 2: Verify compilation**

Run: `cargo check 2>&1 | head -30`
Expected: Errors in parser/formatter/linter/analyzer that need to handle the new variant.

---

### Task 4: Parser — Expression start + primary expr

**Files:**
- Modify: `src/parser/mod.rs:1081` (is_expr_start)
- Modify: `src/parser/expr.rs:791` (parse_primary_expr)

**Step 1: Add to is_expr_start**

In `src/parser/mod.rs` at line 1081, after `Token::MyBatisRawExpr(_)`, add:

```rust
                | Token::JdbcParam
```

**Step 2: Add to parse_primary_expr**

In `src/parser/expr.rs` after the `Token::MyBatisRawExpr` match arm (line 791), add:

```rust
            Token::JdbcParam => {
                self.advance();
                Ok(Expr::JdbcParam)
            }
```

**Step 3: Verify compilation**

Run: `cargo check 2>&1 | head -30`
Expected: Errors in formatter/linter/analyzer remain.

---

### Task 5: Formatter — Format JdbcParam back to `?`

**Files:**
- Modify: `src/formatter/mod.rs:1216`

**Step 1: Add formatter case**

After `Expr::MyBatisRawExpr(content) => format!("${{{}}}", content),` (line 1216), add:

```rust
            Expr::JdbcParam => "?".to_string(),
```

**Step 2: Verify compilation**

Run: `cargo check 2>&1 | head -30`
Expected: Fewer errors — formatter resolved.

---

### Task 6: Linter + Analyzer — Add JdbcParam handling

**Files:**
- Modify: `src/linter/mod.rs:651` (match arm)
- Modify: `src/analyzer/mod.rs:1986` (match arm)

**Step 1: Add to linter**

In `src/linter/mod.rs` at line 651, after `Expr::MyBatisRawExpr(_)`, add on the same match arm (pipe-separated):

```rust
        | Expr::JdbcParam
```

**Step 2: Add to analyzer**

In `src/analyzer/mod.rs` at line 1986, after `Expr::MyBatisRawExpr(_) => {}`, add:

```rust
            Expr::JdbcParam => {}
```

**Step 3: Verify compilation**

Run: `cargo check 2>&1 | head -30`
Expected: Fewer errors.

---

### Task 7: CLI + Utility — Token display mapping

**Files:**
- Modify: `src/bin/ogsql.rs:392` (token display)
- Modify: `src/parser/utility/functions.rs:371,848` (token to string)

**Step 1: Add to CLI token display**

In `src/bin/ogsql.rs` after `Token::MyBatisRawExpr(s) => ("MyBatisRawExpr".into(), format!("${{{}}}", s)),` (line 392), add:

```rust
        Token::JdbcParam => ("JdbcParam".into(), "?".to_string()),
```

**Step 2: Add to utility functions (two locations)**

In `src/parser/utility/functions.rs`:

Location 1 (~line 371): After `Token::MyBatisRawExpr(s) => format!("${{{}}}", s),`, add:
```rust
            Token::JdbcParam => Cow::Borrowed("?"),
```

Location 2 (~line 848): After `Token::MyBatisRawExpr(s) => format!("${{{}}}", s),`, add:
```rust
        Token::JdbcParam => "?".to_string(),
```

**Step 3: Verify full compilation**

Run: `cargo check 2>&1 | tail -5`
Expected: Clean compilation (no errors).

---

### Task 8: Tests

**Files:**
- Modify: `src/token/tokenizer.rs` (add tests at end of test module)

**Step 1: Add tokenizer tests**

At the end of the `#[cfg(test)]` module in `src/token/tokenizer.rs`, add:

```rust
    #[test]
    fn test_jdbc_param_simple() {
        let tokens = Tokenizer::new("SELECT * FROM t WHERE id = ?")
            .mybatis_params(true)
            .tokenize()
            .unwrap();
        assert!(tokens.iter().any(|t| matches!(&t.token, Token::JdbcParam)));
    }

    #[test]
    fn test_jdbc_param_multiple() {
        let tokens = Tokenizer::new("INSERT INTO t (a, b) VALUES (?, ?)")
            .mybatis_params(true)
            .tokenize()
            .unwrap();
        let params: Vec<_> = tokens.iter().filter(|t| matches!(&t.token, Token::JdbcParam)).collect();
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_jdbc_param_disabled() {
        let tokens = Tokenizer::new("SELECT ?")
            .mybatis_params(false)
            .tokenize()
            .unwrap();
        assert!(tokens.iter().any(|t| matches!(&t.token, Token::Op(_))));
        assert!(!tokens.iter().any(|t| matches!(&t.token, Token::JdbcParam)));
    }

    #[test]
    fn test_jdbc_param_parse_expr() {
        use crate::parser::{Parser, ParseOptions};
        let output = Parser::parse_sql_with_options(
            "SELECT * FROM t WHERE id = ? AND name = ?",
            ParseOptions { preserve_comments: false, mybatis_params: true },
        );
        assert!(output.errors.is_empty(), "Parse errors: {:?}", output.errors);
        assert_eq!(output.statements.len(), 1);
    }
```

**Step 2: Run tests**

Run: `cargo test test_jdbc_param`
Expected: All 4 tests pass.

---

### Task 9: Final verification

**Step 1: Run clippy**

Run: `cargo clippy --all-features -- -D warnings 2>&1 | tail -20`
Expected: No warnings.

**Step 2: Run all tests**

Run: `cargo test --all-features 2>&1 | tail -20`
Expected: All tests pass.

**Step 3: Run fmt check**

Run: `cargo fmt --all -- --check`
Expected: No output (clean formatting).
