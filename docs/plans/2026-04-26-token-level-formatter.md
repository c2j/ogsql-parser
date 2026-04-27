# Token-Level Formatter & SqlFormatter Bug Fixes

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Separate `format` (token-level, whitespace-only) from `json2sql` (AST reconstruction), and fix SqlFormatter's 4 semantic bugs.

**Architecture:** The `format` command switches from AST-reconstruction to token-level formatting. Add `Comment` token to the tokenizer (opt-in via flag) so the formatter can preserve comments. SqlFormatter gets 4 targeted bug fixes for its use by `json2sql`. Both share no code path.

**Tech Stack:** Rust, existing Tokenizer infrastructure, serde for tests.

---

## Background: Current vs Target

### Current (broken)
```
format:   SQL → parse → AST → SqlFormatter → SQL   (changes content!)
json2sql: JSON → AST → SqlFormatter → SQL           (same formatter, same bugs)
```

### Target (correct)
```
format:   SQL → Tokenizer(preserve_comments=true) → TokenFormatter → SQL   (whitespace only)
json2sql: JSON → AST → SqlFormatter(bugs fixed) → SQL                     (AST reconstruction)
```

---

## Part A: Tokenizer Comment Preservation

### Task 1: Add `Comment` token variant and tokenizer flag

**Files:**
- Modify: `src/token/mod.rs` (add `Comment(String)` to Token enum)
- Modify: `src/token/tokenizer.rs` (add `preserve_comments` flag)

**Step 1: Write the failing test**

In `src/token/tokenizer.rs`, add test:

```rust
#[test]
fn test_comment_preservation_single_line() {
    let tokens = Tokenizer::new("SELECT -- this is a comment\nFROM dual").preserve_comments(true).tokenize().unwrap();
    assert!(tokens.iter().any(|t| matches!(t.token, Token::Comment(_))));
    let comment_token = tokens.iter().find(|t| matches!(t.token, Token::Comment(_))).unwrap();
    assert!(matches!(&comment_token.token, Token::Comment(c) if c.contains("this is a comment")));
}

#[test]
fn test_comment_preservation_block_comment() {
    let tokens = Tokenizer::new("SELECT /* block\ncomment */ FROM dual").preserve_comments(true).tokenize().unwrap();
    assert!(tokens.iter().any(|t| matches!(t.token, Token::Comment(_))));
    let comment_token = tokens.iter().find(|t| matches!(t.token, Token::Comment(_))).unwrap();
    assert!(matches!(&comment_token.token, Token::Comment(c) if c.contains("block") && c.contains("comment")));
}

#[test]
fn test_comment_not_preserved_by_default() {
    let tokens = Tokenizer::new("SELECT -- comment\nFROM dual").tokenize().unwrap();
    assert!(!tokens.iter().any(|t| matches!(t.token, Token::Comment(_))));
}

#[test]
fn test_comment_ordering() {
    let tokens = Tokenizer::new("SELECT /* c1 */ a, /* c2 */ b FROM t").preserve_comments(true).tokenize().unwrap();
    let kinds: Vec<String> = tokens.iter().filter_map(|t| match &t.token {
        Token::Keyword(k) => Some(format!("{:?}", k)),
        Token::Comment(c) => Some(format!("Comment({})", c)),
        _ => None,
    }).collect();
    // SELECT should come before Comment(c1), then before a, etc.
    assert!(kinds[0].contains("SELECT"));
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test test_comment_preservation`
Expected: FAIL (no `preserve_comments` method, no `Comment` variant)

**Step 3: Implement**

Add to `src/token/mod.rs` Token enum (after `Hint`):
```rust
/// SQL comment (-- single-line or /* block */)
Comment(String),
```

Add to `src/token/tokenizer.rs` Tokenizer struct:
```rust
pub struct Tokenizer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    pos: usize,
    line: usize,
    line_start: usize,
    pending_hint: Option<String>,
    preserve_comments: bool,  // NEW
}
```

Add builder method and update constructor:
```rust
pub fn new(input: &'a str) -> Self {
    Self {
        input,
        chars: input.chars().peekable(),
        pos: 0,
        line: 1,
        line_start: 0,
        pending_hint: None,
        preserve_comments: false,  // default: skip comments (backward compat)
    }
}

pub fn preserve_comments(mut self, yes: bool) -> Self {
    self.preserve_comments = yes;
    self
}
```

Modify `skip_whitespace_and_comments` to emit Comment tokens when flag is on. Replace the comment-skipping sections with:

```rust
fn skip_whitespace_and_comments(&mut self) -> Result<(), TokenizerError> {
    loop {
        match self.chars.peek().copied() {
            None => return Ok(()),
            Some(c) if c.is_whitespace() => {
                self.advance();
            }
            Some('-') => {
                if self.peek_byte_at(1) == Some(b'-') {
                    if self.preserve_comments {
                        self.advance();
                        self.advance();
                        let content = self.collect_line_comment();
                        return Err(TokenizerError::__Comment(content));
                    } else {
                        self.advance();
                        self.advance();
                        self.skip_while(|c| c != '\n');
                    }
                } else {
                    return Ok(());
                }
            }
            // ... block comment similar
        }
    }
}
```

Actually, returning a Comment via `Err` is ugly. Better approach: have `next_token` handle comments. Refactor `skip_whitespace_and_comments` to optionally return a Comment token:

```rust
/// Returns None if done, Some(Comment token) if a comment was found, or continues
/// After calling this, next_token should check for pending_comment.
fn skip_whitespace_and_comments(&mut self) -> Result<Option<TokenWithSpan>, TokenizerError> {
    // ... whitespace loop ...
    // when comment found AND preserve_comments:
    //   return Ok(Some(comment_token_with_span))
    // when comment found AND !preserve_comments:
    //   skip it, continue loop
}
```

**Step 4: Run all tests**

Run: `cargo test`
Expected: ALL 911+ tests pass (preserve_comments=false is default, backward compat)

**Step 5: Commit**

```
feat: add Comment token variant and preserve_comments flag to Tokenizer
```

---

## Part B: Token-Level Formatter

### Task 2: Create TokenFormatter skeleton with structural context

**Files:**
- Create: `src/token_formatter.rs`
- Modify: `src/lib.rs` (add `pub mod token_formatter;`)

**Step 1: Write the failing test**

In `src/token_formatter.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn format_sql(input: &str) -> String {
        let tokens = crate::token::Tokenizer::new(input)
            .preserve_comments(true)
            .tokenize()
            .unwrap();
        TokenFormatter::new(input, tokens).format()
    }

    #[test]
    fn test_simple_select_preserves_content() {
        let input = "SELECT id, name FROM users WHERE id = 1";
        let output = format_sql(input);
        // Token content must be identical - only whitespace changes
        assert_eq!(output.replace(char::is_whitespace, ""), input.replace(char::is_whitespace, ""));
        // Keywords should appear in output
        assert!(output.contains("SELECT"));
        assert!(output.contains("FROM"));
        assert!(output.contains("WHERE"));
    }

    #[test]
    fn test_preserves_quoted_identifiers() {
        let input = r#"SELECT "BIGFUND"."PKG_BM_2" FROM dual"#;
        let output = format_sql(input);
        assert!(output.contains(r#""BIGFUND""#));
        assert!(output.contains(r#""PKG_BM_2""#));
    }

    #[test]
    fn test_preserves_unquoted_identifiers() {
        let input = "SELECT BIGFUND.PKG_BM_2 FROM dual";
        let output = format_sql(input);
        // Should NOT add quotes
        assert!(output.contains("BIGFUND.PKG_BM_2"));
        assert!(!output.contains(r#""BIGFUND""#));
    }

    #[test]
    fn test_preserves_single_line_comment() {
        let input = "SELECT -- this is a comment\na FROM t";
        let output = format_sql(input);
        assert!(output.contains("-- this is a comment"));
    }

    #[test]
    fn test_preserves_block_comment() {
        let input = "SELECT /* block comment */ a FROM t";
        let output = format_sql(input);
        assert!(output.contains("/* block comment */"));
    }

    #[test]
    fn test_begin_end_indentation() {
        let input = "BEGIN p_out := 0; END";
        let output = format_sql(input);
        let expected = "BEGIN\n  p_out := 0;\nEND";
        assert_eq!(output.trim(), expected);
    }

    #[test]
    fn test_nested_begin_end() {
        let input = "BEGIN BEGIN x := 1; END; END";
        let output = format_sql(input);
        assert!(output.contains("BEGIN\n    x := 1;\n  END"));
    }

    #[test]
    fn test_exception_block() {
        let input = "BEGIN x := 1; EXCEPTION WHEN OTHERS THEN x := 0; END";
        let output = format_sql(input);
        assert!(output.contains("EXCEPTION\n  WHEN OTHERS THEN\n    x := 0;"));
    }

    #[test]
    fn test_if_then_end_if() {
        let input = "IF x > 0 THEN y := 1; END IF";
        let output = format_sql(input);
        assert!(output.contains("IF x > 0 THEN\n  y := 1;\nEND IF"));
    }

    #[test]
    fn test_loop_end_loop() {
        let input = "LOOP x := x + 1; END LOOP";
        let output = format_sql(input);
        assert!(output.contains("LOOP\n  x := x + 1;\nEND LOOP"));
    }

    #[test]
    fn test_preserves_end_label() {
        let input = "END pkg_batchpay_management_2";
        let output = format_sql(input);
        assert!(output.contains("END pkg_batchpay_management_2"));
    }

    #[test]
    fn test_package_body_structure() {
        let input = "CREATE OR REPLACE PACKAGE BODY pkg1 IS PROCEDURE proc1 IS BEGIN x := 1; END; END pkg1";
        let output = format_sql(input);
        // Procedure should be indented inside package body
        assert!(output.contains("PROCEDURE proc1 IS\n    BEGIN\n      x := 1;\n    END;"));
        assert!(output.contains("END pkg1"));
    }

    #[test]
    fn test_no_declare_for_named_procedure() {
        // Token formatter never adds DECLARE - it only preserves what's there
        let input = "PROCEDURE proc1 IS v_x NUMBER; BEGIN v_x := 1; END";
        let output = format_sql(input);
        // Should NOT contain DECLARE (it wasn't in the input)
        assert!(!output.contains("DECLARE"));
    }

    #[test]
    fn test_semicolons_preserved() {
        let input = "BEGIN a := 1; b := 2; END";
        let output = format_sql(input);
        assert!(output.contains("a := 1;"));
        assert!(output.contains("b := 2;"));
    }

    #[test]
    fn test_string_literals_preserved() {
        let input = "SELECT 'hello world' FROM dual";
        let output = format_sql(input);
        assert!(output.contains("'hello world'"));
    }
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test token_formatter`
Expected: FAIL (module doesn't exist)

**Step 3: Implement TokenFormatter**

Create `src/token_formatter.rs`:

```rust
use crate::token::{Token, TokenWithSpan, Keyword};

pub struct TokenFormatter<'a> {
    source: &'a str,
    tokens: Vec<TokenWithSpan>,
    pos: usize,
    indent: usize,
    output: String,
}

impl<'a> TokenFormatter<'a> {
    pub fn new(source: &'a str, tokens: Vec<TokenWithSpan>) -> Self {
        Self {
            source,
            tokens,
            pos: 0,
            indent: 0,
            output: String::new(),
        }
    }

    pub fn format(mut self) -> String {
        while self.pos < self.tokens.len() {
            let tws = &self.tokens[self.pos];
            match &tws.token {
                Token::Eof => break,
                Token::Comment(c) => {
                    self.emit_newline_indent();
                    self.output.push_str(c);
                    self.advance();
                }
                _ => {
                    self.emit_token(tws);
                    self.advance();
                }
            }
        }
        self.output
    }

    fn emit_newline_indent(&mut self) {
        self.output.push('\n');
        self.output.push_str(&"  ".repeat(self.indent));
    }

    fn emit_token(&mut self, tws: &TokenWithSpan) {
        // Use original source text for this token (preserves quoting, casing, etc.)
        let original_text = &self.source[tws.span.start..tws.span.end];
        // ... structural context tracking and indentation logic ...
        self.output.push_str(original_text);
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
}
```

The structural context tracking is the core logic. It decides:
- Before which tokens to add a newline + indent
- When to increase/decrease indent
- When to add a space vs newline

Key indent triggers:
| Token Pattern | Action |
|---------------|--------|
| `BEGIN` | After: push indent |
| `END` | Before: pop indent |
| `THEN` (after IF) | After: push indent |
| `END IF` | Before: pop indent |
| `LOOP` | After: push indent |
| `END LOOP` | Before: pop indent |
| `EXCEPTION` | Before: pop to matching BEGIN level |
| `PROCEDURE`/`FUNCTION` in package | Before: indent+1 |
| `IS`/`AS` after procedure header | After: newline |
| `;` inside block | After: newline+indent |

**Step 4: Run tests iteratively**

Run: `cargo test token_formatter`
Fix each test failure one at a time.

**Step 5: Commit**

```
feat: add TokenFormatter for whitespace-only SQL formatting
```

---

### Task 3: Wire TokenFormatter into CLI format command

**Files:**
- Modify: `src/bin/ogsql.rs` (cmd_format uses TokenFormatter)

**Step 1: Write the failing test**

Integration test (manual):

```bash
# Format should preserve unquoted identifiers
echo "SELECT BIGFUND.PKG_BM_2 FROM dual" | cargo run --features full -- format
# Expected: no double quotes added

# Format should preserve comments
echo "SELECT -- my comment\na FROM t" | cargo run --features full -- format
# Expected: -- my comment appears in output
```

**Step 2: Modify cmd_format**

```rust
fn cmd_format(cli: &Cli) {
    let sql = read_input(cli.file.as_deref());
    let tokens = match Tokenizer::new(&sql).preserve_comments(true).tokenize() {
        Ok(t) => t,
        Err(e) => { eprintln!("error: {}", e); return; }
    };
    let formatted = ogsql_parser::token_formatter::TokenFormatter::new(&sql, tokens).format();
    println!("{}", formatted);
}
```

**Step 3: Verify**

Run: `cargo run --features full -- format -f GaussDB-2.23.07.210/sql/errsp1.sql`
Expected: properly indented, all original content preserved (comments, quotes, labels)

**Step 4: Commit**

```
refactor: format command uses token-level formatter instead of AST reconstruction
```

---

## Part C: SqlFormatter Bug Fixes (for json2sql)

### Task 4: Fix identifier quoting in SqlFormatter

**Files:**
- Modify: `src/formatter.rs` (`format_object_name` method)

**Step 1: Write the failing test**

In `src/formatter.rs` tests:

```rust
#[test]
fn test_format_object_name_no_unnecessary_quotes() {
    let f = SqlFormatter::new();
    let name = ObjectName(vec![
        Ident { value: "BIGFUND".into(), quote_style: None },
        Ident { value: "PKG_BM_2".into(), quote_style: None },
    ]);
    let output = f.format_object_name(&name);
    assert_eq!(output, "BIGFUND.PKG_BM_2"); // NO double quotes
}

#[test]
fn test_format_object_name_preserves_quotes() {
    let f = SqlFormatter::new();
    let name = ObjectName(vec![
        Ident { value: "BIGFUND".into(), quote_style: Some('"') },
    ]);
    let output = f.format_object_name(&name);
    assert_eq!(output, r#""BIGFUND""#); // preserves original quotes
}
```

**Step 2: Fix `format_object_name`**

Only add quotes when the Ident originally had `quote_style: Some(...)`.

**Step 3: Run tests**

Run: `cargo test`
Expected: ALL pass

**Step 4: Commit**

```
fix: SqlFormatter only quotes identifiers that were originally quoted
```

---

### Task 5: Fix DECLARE in named procedures

**Files:**
- Modify: `src/formatter.rs` (`format_pl_block`, `format_package_procedure`, `format_package_function`)

**Step 1: Write the failing test**

```rust
#[test]
fn test_format_package_procedure_no_declare() {
    // Named procedures use IS, not DECLARE
    let input = r#"CREATE OR REPLACE PACKAGE BODY pkg1 IS PROCEDURE proc1 IS v_x NUMBER; BEGIN v_x := 1; END; END"#;
    let tokens = Tokenizer::new(input).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    let f = SqlFormatter::new();
    let output = stmts.iter().map(|s| f.format_statement(s)).collect::<Vec<_>>().join(";\n");
    // Should NOT contain "IS DECLARE" or "IS \n DECLARE"
    let without_spaces: String = output.chars().filter(|c| !c.is_whitespace()).collect();
    assert!(!without_spaces.contains("ISDECLARE"));
}
```

**Step 2: Fix**

Add `named_block: bool` parameter to `format_pl_block`. When true, skip `DECLARE` keyword output.

**Step 3: Run tests**

**Step 4: Commit**

```
fix: SqlFormatter skips DECLARE in named procedure/function blocks
```

---

### Task 6: Fix missing semicolon after procedure END in package body

**Files:**
- Modify: `src/formatter.rs` (`format_package_procedure`, `format_package_function`)

**Step 1: Write the failing test**

```rust
#[test]
fn test_format_package_procedure_end_semicolon() {
    let input = "CREATE OR REPLACE PACKAGE BODY pkg1 IS PROCEDURE proc1 IS BEGIN x := 1; END; END";
    // ... parse ...
    let output = f.format_statement(&stmt);
    // After END of proc1, there should be a semicolon before next PROCEDURE or final END
    assert!(output.contains("END;"));
}
```

**Step 2: Fix**

In `format_package_procedure`, when there's a block, append `;` after `format_pl_block`.

**Step 3: Commit**

```
fix: SqlFormatter adds semicolon after procedure END in package body
```

---

### Task 7: Fix lost end_label

**Files:**
- Modify: `src/formatter.rs` (`format_create_package`, `format_create_package_body`)

**Step 1: Write the failing test**

```rust
#[test]
fn test_format_package_preserves_end_label() {
    let input = "CREATE OR REPLACE PACKAGE pkg1 IS PROCEDURE proc1; END pkg1";
    // ... parse ...
    let output = f.format_statement(&stmt);
    assert!(output.contains("END pkg1"));
}
```

**Step 2: Fix**

Use the `end_label` field (check if it exists in the AST struct) in `format_create_package` / `format_create_package_body`.

**Step 3: Commit**

```
fix: SqlFormatter preserves package end_label
```

---

## Part D: End-to-End Verification

### Task 8: Verify full pipeline

**Step 1: Verify format preserves content**

```bash
cargo run --features full -- format -f GaussDB-2.23.07.210/sql/errsp1.sql 2>/dev/null > /tmp/formatted.sql
```

Check:
- [ ] `BIGFUND.PKG_BM_2` (no quotes)
- [ ] `-- comment` preserved
- [ ] `end pkg_batchpay_management_2` label preserved
- [ ] No `DECLARE` added after `IS`
- [ ] Proper indentation for BEGIN/END/EXCEPTION/IF

**Step 2: Verify json2sql round-trip**

```bash
cargo run --features full -- parse -f GaussDB-2.23.07.210/sql/errsp1.sql -j | cargo run --features full -- json2sql 2>/dev/null
```

Check:
- [ ] No parse errors
- [ ] Identifier quoting is correct (only originally-quoted ids have quotes)
- [ ] No `IS DECLARE`
- [ ] `END;` after each procedure

**Step 3: Run full test suite**

```bash
cargo test
```

Expected: ALL tests pass (911+)

**Step 4: Final commit**

```
chore: verification of token-level formatter and SqlFormatter fixes
```

---

## Execution Order

1. **Task 1** (Tokenizer Comments) → foundational, no dependencies
2. **Task 2** (TokenFormatter) → depends on Task 1
3. **Task 3** (CLI wiring) → depends on Task 2
4. **Tasks 4-7** (SqlFormatter fixes) → independent of Tasks 1-3, can run in parallel
5. **Task 8** (E2E verification) → depends on all above

Recommended: Tasks 1-3 sequential, Tasks 4-7 in parallel, Task 8 last.
