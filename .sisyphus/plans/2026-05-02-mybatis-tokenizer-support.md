# MyBatis Parameter Tokenizer Support

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Make the tokenizer recognize MyBatis `#{param}` and `${expr}` as single tokens when `--mybatis` flag is enabled, so format/parse/validate/tokenize commands handle them correctly.

**Architecture:** Add opt-in `mybatis_params` flag to Tokenizer. When enabled, `#{...}` scans as `Token::MyBatisParam(String)` and `${...}` scans as `Token::MyBatisRawExpr(String)`. Parser treats them as value expressions. Formatter outputs them verbatim. CLI exposes `--mybatis` global flag.

**Tech Stack:** Rust, existing token/parser/formatter infrastructure

---

## Touch Points Map

| File | Change |
|------|--------|
| `src/token/mod.rs` | Add `MyBatisParam(String)` and `MyBatisRawExpr(String)` to `Token` enum |
| `src/token/tokenizer.rs` | Add `mybatis_params` flag; handle `#{` and `${` scanning |
| `src/ast/mod.rs` | Add `MyBatisParam(String)` and `MyBatisRawExpr(String)` to `Expr` enum |
| `src/parser/expr.rs` | Parse new tokens as expressions (like `Token::Param`) |
| `src/parser/mod.rs` | Add new tokens to "value-producing" token list |
| `src/parser/utility/functions.rs` | Add token_name/token_to_string for new tokens |
| `src/formatter.rs` | Format new Expr variants verbatim |
| `src/token_formatter.rs` | No changes needed (uses source span, automatic) |
| `src/bin/ogsql.rs` | Add `--mybatis` global CLI flag, pass to Tokenizer |

---

### Task 1: Add Token Variants

**Files:**
- Modify: `src/token/mod.rs:48-146` (Token enum)

**Step 1: Add token variants**

In `src/token/mod.rs`, add after the existing `Param(i32)` variant (line 83):

```rust
// --- Parameters (continued) ---
/// MyBatis parameter placeholder: #{param, jdbcType=BIGINT}
/// Content includes everything between #{ and }
MyBatisParam(String),
/// MyBatis raw expression: ${expr}
/// Content includes everything between ${ and }
MyBatisRawExpr(String),
```

**Step 2: Build to verify**

Run: `cargo build 2>&1 | head -40`
Expected: Many compile errors in match arms (expected — we'll fix them in subsequent tasks). Just verify the enum definition itself has no syntax errors.

---

### Task 2: Tokenizer — Add Flag and Scanning Logic

**Files:**
- Modify: `src/token/tokenizer.rs`

**Step 1: Add `mybatis_params` field to Tokenizer struct**

In `src/token/tokenizer.rs`, add field to the struct (around line 29):

```rust
pub struct Tokenizer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    pos: usize,
    line: usize,
    line_start: usize,
    pending_hint: Option<String>,
    preserve_comments: bool,
    pending_comment: Option<String>,
    after_dml_keyword: bool,
    mybatis_params: bool,  // NEW
}
```

In `Tokenizer::new` (line 32), add `mybatis_params: false` to the init.

Add builder method after `preserve_comments` (after line 48):

```rust
pub fn mybatis_params(mut self, yes: bool) -> Self {
    self.mybatis_params = yes;
    self
}
```

**Step 2: Handle `#{` in the `#` match arm**

The `#` character is currently in the catch-all operator arm at line 699:

```rust
'~' | '&' | '|' | '`' | '#' | '?' => {
```

Remove `'#'` from that arm. Add a new dedicated arm **before** it:

```rust
'#' => {
    if self.mybatis_params && self.peek() == Some('{') {
        self.scan_mybatis_param()
    } else {
        // Existing behavior: scan as operator
        self.advance();
        let start = self.pos - '#'.len_utf8();
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
}
```

**Step 3: Add `scan_mybatis_param` method**

Add this method to `impl Tokenizer` (near the other `scan_*` methods, around line 870):

```rust
fn scan_mybatis_param(&mut self) -> Result<Token, TokenizerError> {
    let start = self.pos;
    // Current char is '#', next is '{'
    self.advance(); // consume '#'
    self.advance(); // consume '{'
    let mut depth = 1;
    while let Some(&c) = self.chars.peek() {
        self.chars.next();
        self.pos += c.len_utf8();
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    // Content is between #{ and }
                    let content = self.input[start + 2..self.pos - 1].to_string();
                    return Ok(Token::MyBatisParam(content));
                }
            }
            _ => {}
        }
    }
    Err(TokenizerError::UnterminatedString(start))
}
```

**Step 4: Handle `${` in `scan_dollar_or_param`**

Modify `scan_dollar_or_param` (line 854). Add MyBatis check **before** the existing digit check:

```rust
fn scan_dollar_or_param(&mut self) -> Result<Token, TokenizerError> {
    // MyBatis raw expression: ${expr}
    if self.mybatis_params && self.peek() == Some('{') {
        self.advance(); // consume '{'
        let mut depth = 1;
        while let Some(&c) = self.chars.peek() {
            self.chars.next();
            self.pos += c.len_utf8();
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        // Content is between ${ and }
                        // Start is the position of '$', +2 to skip '${'
                        let param_start = self.pos - self.input[..self.pos].len();
                        // Re-derive content from input
                        // Actually, let's track it properly
                        // The '$' was at self.pos before we entered this function
                        // We need to save it. Let me restructure...
                        todo!("restructure to track start position")
                    }
                }
                _ => {}
            }
        }
    }
    // ... existing code
}
```

Wait, this needs proper position tracking. Let me reconsider. The `scan_dollar_or_param` is called when `$` has already been consumed (line ~580 in next_token). Let me check.

Actually, looking at the tokenizer more carefully — `$` is matched in the main `next_token` match and calls `scan_dollar_or_param`. The `$` has been consumed already when entering. So `self.pos` is already past `$`.

Let me restructure the approach:

**Revised Step 4:** Add MyBatis check at the top of `scan_dollar_or_param`, before existing logic:

```rust
fn scan_dollar_or_param(&mut self) -> Result<Token, TokenizerError> {
    // MyBatis raw expression: ${expr}
    if self.mybatis_params && self.peek() == Some('{') {
        let dollar_pos = self.pos - 1; // position of '$' (already consumed)
        self.advance(); // consume '{'
        let content_start = self.pos;
        let mut depth = 1;
        while let Some(&c) = self.chars.peek() {
            self.chars.next();
            self.pos += c.len_utf8();
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        let content = self.input[content_start..self.pos - 1].to_string();
                        return Ok(Token::MyBatisRawExpr(content));
                    }
                }
                _ => {}
            }
        }
        return Err(TokenizerError::UnterminatedString(dollar_pos));
    }

    // Existing code follows...
    if let Some(&c) = self.chars.peek() {
        if c.is_ascii_digit() {
            // ...
```

**Step 5: Add tokenizer unit tests**

Add tests at the end of `src/token/tokenizer.rs` (in the existing `#[cfg(test)]` module, around line 1100+):

```rust
#[test]
fn test_mybatis_param_simple() {
    let tokens = Tokenizer::new("SELECT #{id}")
        .mybatis_params(true)
        .tokenize()
        .unwrap();
    assert!(matches!(&tokens[1].token, Token::MyBatisParam(s) if s == "id"));
}

#[test]
fn test_mybatis_param_with_type() {
    let tokens = Tokenizer::new("SELECT #{name, jdbcType=VARCHAR, javaType=String}")
        .mybatis_params(true)
        .tokenize()
        .unwrap();
    assert!(matches!(&tokens[1].token, Token::MyBatisParam(s) if s.contains("jdbcType=VARCHAR")));
}

#[test]
fn test_mybatis_raw_expr() {
    let tokens = Tokenizer::new("SELECT ${col}")
        .mybatis_params(true)
        .tokenize()
        .unwrap();
    assert!(matches!(&tokens[1].token, Token::MyBatisRawExpr(s) if s == "col"));
}

#[test]
fn test_mybatis_disabled_hash_is_op() {
    let tokens = Tokenizer::new("SELECT #{id}")
        .mybatis_params(false)
        .tokenize()
        .unwrap();
    // '#' should be Op("#"), '{' should be separate token
    assert!(tokens.iter().any(|t| matches!(&t.token, Token::Op(s) if s == "#")));
}

#[test]
fn test_mybatis_disabled_dollar_is_param() {
    let tokens = Tokenizer::new("SELECT $1")
        .mybatis_params(true)
        .tokenize()
        .unwrap();
    assert!(matches!(&tokens[1].token, Token::Param(1)));
}

#[test]
fn test_mybatis_multiple_params() {
    let sql = "INSERT INTO t (a, b) VALUES (#{x}, #{y})";
    let tokens = Tokenizer::new(sql)
        .mybatis_params(true)
        .tokenize()
        .unwrap();
    let params: Vec<_> = tokens.iter()
        .filter(|t| matches!(&t.token, Token::MyBatisParam(_)))
        .collect();
    assert_eq!(params.len(), 2);
}

#[test]
fn test_mybatis_in_string_literal() {
    // #{...} inside string literals should NOT be scanned as MyBatisParam
    // because the tokenizer handles string literals first in the match arm
    let tokens = Tokenizer::new("SELECT 'item = #{price}'")
        .mybatis_params(true)
        .tokenize()
        .unwrap();
    // The #{price} should be inside the StringLiteral, not a separate token
    assert!(tokens.iter().any(|t| matches!(&t.token, Token::StringLiteral(s) if s.contains("#{price}"))));
    assert!(!tokens.iter().any(|t| matches!(&t.token, Token::MyBatisParam(_))));
}
```

**Step 6: Build and run tokenizer tests**

Run: `cargo test --lib token::tokenizer -- tests 2>&1 | tail -20`
Expected: All existing tests + new MyBatis tests pass.

---

### Task 3: AST — Add Expr Variants

**Files:**
- Modify: `src/ast/mod.rs:1271` (Expr enum)

**Step 1: Add Expr variants**

After `Parameter(i32)` (line 1271), add:

```rust
MyBatisParam(String),
MyBatisRawExpr(String),
```

**Step 2: Build to verify enum compiles**

Run: `cargo build 2>&1 | head -20`
Expected: Errors in formatter.rs and parser — expected, we fix those next.

---

### Task 4: Parser — Handle New Tokens

**Files:**
- Modify: `src/parser/expr.rs:681`
- Modify: `src/parser/mod.rs:1096`

**Step 1: Add expression parsing in expr.rs**

In `src/parser/expr.rs`, after the `Token::Param(n)` arm (line 681-684), add:

```rust
Token::MyBatisParam(content) => {
    self.advance();
    Ok(Expr::MyBatisParam(content.clone()))
}
Token::MyBatisRawExpr(content) => {
    self.advance();
    Ok(Expr::MyBatisRawExpr(content.clone()))
}
```

**Step 2: Add to value-producing token list in mod.rs**

In `src/parser/mod.rs`, after `Token::Param(_)` (line 1096), add:

```rust
| Token::MyBatisParam(_)
| Token::MyBatisRawExpr(_)
```

**Step 3: Add token name/to_string in utility/functions.rs**

In `src/parser/utility/functions.rs`, find where `Token::Param(n)` is handled:
- In `token_name` function (around line 344-356): add arms for `MyBatisParam` and `MyBatisRawExpr`
- In `token_to_string` function (around line 824-836): add arms

For `token_name`:
```rust
Token::MyBatisParam(_) => Cow::Borrowed("MyBatisParam"),
Token::MyBatisRawExpr(_) => Cow::Borrowed("MyBatisRawExpr"),
```

For `token_to_string`:
```rust
Token::MyBatisParam(s) => format!("#{{{}}}", s),
Token::MyBatisRawExpr(s) => format!("${{{}}}", s),
```

**Step 4: Build**

Run: `cargo build 2>&1 | head -20`
Expected: Should compile (remaining errors only in formatter).

---

### Task 5: Formatter — Output MyBatis Params Verbatim

**Files:**
- Modify: `src/formatter.rs:1200`

**Step 1: Add Expr formatting**

In `src/formatter.rs`, after `Expr::Parameter(n) => format!("${}", n)` (line 1200), add:

```rust
Expr::MyBatisParam(content) => format!("#{{{}}}", content),
Expr::MyBatisRawExpr(content) => format!("${{{}}}", content),
```

**Step 2: Build**

Run: `cargo build 2>&1 | tail -5`
Expected: Clean build, no errors.

---

### Task 6: CLI — Add `--mybatis` Flag

**Files:**
- Modify: `src/bin/ogsql.rs`

**Step 1: Add CLI flag**

In the `Cli` struct (line 9), add after the `comments` field:

```rust
#[arg(long, global = true)]
mybatis: bool,
```

**Step 2: Pass flag to Tokenizer in cmd_format**

In `cmd_format` (line 141), the Tokenizer is created on line 143. Change:

```rust
fn cmd_format(cli: &Cli) {
    let sql = read_input(cli.file.as_deref());
    let mut tokenizer = Tokenizer::new(&sql).preserve_comments(true);
    if cli.mybatis {
        tokenizer = tokenizer.mybatis_params(true);
    }
    let tokens = match tokenizer.tokenize() {
```

**Step 3: Pass flag to Tokenizer in cmd_tokenize**

In `cmd_tokenize` (line 262), similarly:

```rust
fn cmd_tokenize(cli: &Cli) {
    let sql = read_input(cli.file.as_deref());
    let mut tokenizer = Tokenizer::new(&sql);
    if cli.comments {
        tokenizer = tokenizer.preserve_comments(true);
    }
    if cli.mybatis {
        tokenizer = tokenizer.mybatis_params(true);
    }
    let tokens = match tokenizer.tokenize() {
```

**Step 4: Pass flag through to Parser in cmd_parse and cmd_validate**

These functions use `parse_input` which calls `Parser::parse_sql_with_options`. The `ParseOptions` doesn't have a mybatis flag — the flag is on the Tokenizer, not the Parser. So we need to tokenize first with the flag, then parse.

In `cmd_parse` (line 162), change from using `parse_input` to explicit tokenize+parse:

```rust
fn cmd_parse(cli: &Cli) {
    let sql = read_input(cli.file.as_deref());
    let output = if cli.mybatis {
        let mut tokenizer = Tokenizer::new(&sql);
        if cli.comments {
            tokenizer = tokenizer.preserve_comments(true);
        }
        tokenizer = tokenizer.mybatis_params(true);
        let tokens = match tokenizer.tokenize() {
            Ok(t) => t,
            Err(e) => die!("Tokenizer error: {}", e),
        };
        Parser::parse_sql_tokens_with_options(tokens, sql, ParseOptions { preserve_comments: cli.comments })
    } else {
        parse_input(&sql, cli.comments)
    };
    // ... rest unchanged
```

Same pattern for `cmd_validate` (line 354).

**IMPORTANT**: Check if `Parser::parse_sql_tokens_with_options` or equivalent exists. If not, we may need to add it or refactor `parse_input` to accept a tokenizer flag. Check `src/lib.rs` for the public API.

Alternative simpler approach: Modify `parse_input` to accept a mybatis flag:

```rust
fn parse_input_ex(sql: &str, preserve_comments: bool, mybatis: bool) -> ogsql_parser::ParseOutput {
    let mut tokenizer = Tokenizer::new(sql);
    if preserve_comments {
        tokenizer = tokenizer.preserve_comments(true);
    }
    if mybatis {
        tokenizer = tokenizer.mybatis_params(true);
    }
    let tokens = match tokenizer.tokenize() {
        Ok(t) => t,
        Err(e) => { /* handle */ }
    };
    let options = ogsql_parser::ParseOptions { preserve_comments };
    ogsql_parser::Parser::parse_with_options(tokens, options)
}
```

Check what `Parser::parse_sql_with_options` does internally — it may create its own Tokenizer. If so, we need a lower-level entry point that takes pre-tokenized tokens.

**Step 5: Build and test CLI**

Run: `cargo build 2>&1 | tail -5`
Expected: Clean build.

Test manually:
```bash
echo "SELECT #{id} FROM t" | cargo run -- format --mybatis
# Expected: SELECT #{id} FROM t  (preserved as-is)

echo "SELECT #{id} FROM t" | cargo run -- format
# Expected: SELECT # { id } FROM t  (broken, existing behavior)

echo "INSERT INTO t (a,b) VALUES (#{x, jdbcType=INT}, #{y})" | cargo run -- format --mybatis
# Expected: #{x, jdbcType=INT} and #{y} preserved
```

---

### Task 7: End-to-End Test with User's Original SQL

**Step 1: Test the exact SQL from the bug report**

```bash
ogsql format --mybatis <<'EOF'
insert into t_mapper_approval (order_id,approver,action,reason) values (#{vOrder_orderId, jdbcType=BIGINT, javaType=Long},#{pApprover, jdbcType=VARCHAR, javaType=String},'ITEM_REVIEW','Item total=' || #{vItem_lineAmount, jdbcType=NUMERIC, javaType=java.math.BigDecimal} || ' for product=' || #{vItem_productName, jdbcType=VARCHAR, javaType=String})
EOF
```

Expected: All `#{...}` preserved exactly. Keywords uppercased/formatted. String literals preserved.

**Step 2: Test parse with --mybatis**

```bash
echo "SELECT #{id} FROM t WHERE name = #{name}" | ogsql parse --mybatis
```

Expected: AST shows MyBatisParam expressions in the WHERE clause.

**Step 3: Test validate with --mybatis**

```bash
echo "SELECT #{id} FROM t WHERE name = #{name}" | ogsql validate --mybatis
```

Expected: VALID.

**Step 4: Test tokenize with --mybatis -j**

```bash
echo "SELECT #{id}" | ogsql tokenize --mybatis -j
```

Expected: JSON shows `"type": "MyBatisParam"` token.

---

### Task 8: JSON Serialization Compatibility

**Step 1: Verify serde round-trip**

The Token and Expr enums derive `Serialize` and `Deserialize`. Verify the new variants serialize correctly:

```bash
echo "SELECT #{id, jdbcType=BIGINT}" | ogsql parse --mybatis -j | ogsql json2sql
```

Expected: JSON contains `"MyBatisParam": "id, jdbcType=BIGINT"` and json2sql reconstructs it correctly.

If json2sql fails, add the new Expr variants to the SqlFormatter in `src/formatter.rs` (already done in Task 5).

**Step 2: Check token_formatter.rs**

The `TokenFormatter` uses `emit_current_token()` which reads from source span. Since `scan_mybatis_param` tracks the span from `#` to `}` (inclusive), the token_formatter should output the complete `#{...}` correctly without any code changes.

Verify:
```bash
echo "SELECT #{id} FROM t" | ogsql format --mybatis
```

If the output is garbled, check that `scan_mybatis_param` sets the span correctly (start should be position of `#`, end should be position after `}`).

---

### Task 9: Commit

```bash
git add src/token/mod.rs src/token/tokenizer.rs src/ast/mod.rs src/parser/expr.rs src/parser/mod.rs src/parser/utility/functions.rs src/formatter.rs src/bin/ogsql.rs
git commit -m "feat: add MyBatis #{}/${} tokenizer support with --mybatis flag

Tokenizer recognizes #{param} and ${expr} as single tokens when
--mybatis flag is enabled. Parser handles them as value expressions.
Formatter outputs them verbatim. Default behavior unchanged.

Closes: MyBatis parameter formatting bug"
```
