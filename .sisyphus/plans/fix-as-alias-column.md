# Plan: Fix AS-alias allowlist for SELECT column aliases

## Problem

`b840253` added `is_allowed_as_alias` to `parse_optional_column_alias`'s AS branch —
but SELECT column aliases never reach that code path.

SELECT `AS` is handled inline in `parse_target_el` (select.rs:399-406):

```rust
let alias = if self.match_keyword(Keyword::AS) {
    self.advance();
    Some(self.parse_ident()?)        // ← reserved keywords rejected here
} else {
    self.parse_optional_column_alias()?  // ← only called when NO AS
};
```

Result: `SELECT 1 AS current_user` still produces `ReservedKeywordAsIdentifier` error.

## Fix

**File**: `src/parser/select.rs:399-406`

Add `is_allowed_as_alias` check in the inline AS branch, same pattern as `parse_optional_alias`:

```rust
let alias = if self.match_keyword(Keyword::AS) {
    self.advance();
    if let Token::Keyword(kw) = self.peek() {
        if is_allowed_as_alias(kw) {
            let name = kw.as_str().to_string();
            self.advance();
            Some(crate::ast::Ident::new(name))
        } else {
            Some(self.parse_ident()?)
        }
    } else {
        Some(self.parse_ident()?)
    }
} else {
    self.parse_optional_column_alias()?
};
```

## Regression tests

**File**: `src/parser/tests.rs`

```rust
#[test]
fn test_reserved_keyword_as_column_alias_allowed() {
    assert_valid("SELECT 1 AS current_user");
    assert_valid("SELECT 1 AS cast");
    assert_valid("SELECT 1 AS session_user");
    assert_valid("SELECT 1 AS current_date");
    assert_valid("SELECT 1 AS sysdate");
}

#[test]
fn test_reserved_keyword_as_table_alias_rejects_invalid() {
    // Clausal keywords should still be rejected even with AS
    let (_, errors) = parse_with_errors("SELECT 1 FROM t AS FROM");
    assert!(errors.iter().any(|e| e.to_string().contains("FROM")),
        "FROM after AS should be rejected");
}
```

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings
cargo test --all-features
```

Verify live: `echo "SELECT 1 AS current_user" | ogsql parse` produces 0 errors.
