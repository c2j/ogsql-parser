# Ident quote_style Preservation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Preserve identifier quoting information through the AST so that `SqlFormatter` round-trips SQL without changing identifier semantics.

**Architecture:** Introduce an `Ident` struct with `value: String` + `quote_style: Option<char>`. Change `ObjectName` from `Vec<String>` to `Vec<Ident>`. The key innovation is implementing `Deref<Target=str>`, `Borrow<str>`, `Display`, and `From<String>` on `Ident`, which makes the vast majority of existing call sites compile without modification. Custom serde keeps JSON backward-compatible.

**Tech Stack:** Rust 2021, serde, thiserror

---

## Background — Why This Matters

In openGauss/GaussDB, `FROM MyTable` (unquoted) and `FROM "MyTable"` (quoted) can resolve to **different physical tables**. The current `SqlFormatter` adds quotes to unquoted identifiers containing uppercase letters (because it uses a PostgreSQL case-folding heuristic), and drops quotes from quoted all-lowercase identifiers (because the AST has no quote_style field). Both directions silently change query semantics.

Root cause chain:
```
Tokenizer (correctly distinguishes Token::Ident vs Token::QuotedIdent)
  → Parser (parse_identifier merges both into String, quote_style lost)
    → AST (ObjectName = Vec<String>, no field to carry quote_style)
      → Formatter (quote_identifier uses uppercase heuristic, guesses wrong)
```

---

## Design: The Ident Struct

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    /// The identifier value without quotes.
    pub value: String,
    /// The starting quote if any. `None` = unquoted, `Some('"')` = double-quoted.
    pub quote_style: Option<char>,
}
```

### Trait Implementations (the key to low blast radius)

| Trait | Why | Effect |
|-------|-----|--------|
| `Deref<Target=str>` | Delegate all `str` methods | `.to_lowercase()`, `.len()`, `.starts_with()` etc. just work |
| `Borrow<str>` | Make `Vec<Ident>.join(".")` work | All `.join(".")` calls compile unchanged |
| `Display` | Format with quotes if needed | `format!("{}", ident)` works, outputs quoted form |
| `From<String>` | Construction | `"foo".to_string()` contexts can use `Ident::from(s)` |
| `From<&str>` | Construction | `"foo".into()` works |
| `PartialEq<str>` | Comparison | `ident == "foo"` works |
| `PartialEq<&str>` | Comparison | `ident == &"foo"` works |
| `Default` | `unwrap_or_default()` | `name.last().cloned().unwrap_or_default()` compiles unchanged |
| Custom `Serialize`/`Deserialize` | JSON backward compat | Unquoted → `"foo"`, quoted → `{"value":"Foo","quote_style":"\""}` |

### Serde Strategy (Critical)

**Current JSON** (ObjectName = Vec<String>):
```json
["schema", "MyTable"]
```

**New JSON** (ObjectName = Vec<Ident>) — backward compatible:
```json
["schema", {"value": "MyTable", "quote_style": "\""}]
```

- Unquoted `Ident { value: "schema", quote_style: None }` → serializes as `"schema"` (plain string)
- Quoted `Ident { value: "MyTable", quote_style: Some('"') }` → serializes as `{"value": "MyTable", "quote_style": "\""}`
- Deserialization accepts both forms (backward compatible with old JSON)

---

## Phase Breakdown

### Phase 1: Foundation — Define Ident + Trait Impls (Tasks 1-3)

Create the `Ident` struct with all trait implementations. No existing code changes yet.

### Phase 2: Parser Changes (Tasks 4-6)

Change `parse_identifier` and `consume_any_identifier` to return `Ident`. Change `ObjectName` type alias. The `Deref`/`Borrow`/`Display` impls mean most of the 509+ call sites compile without changes.

### Phase 3: Formatter Changes (Tasks 7-9)

Replace the heuristic-based `quote_identifier` with `quote_style`-aware `format_ident`. Remove the uppercase-letter guessing.

### Phase 4: Fix Breaking Sites (Tasks 10-13)

Fix the ~20 truly breaking patterns: `.cloned()` returning Ident instead of String, direct ObjectName construction, test assertions.

### Phase 5: Tests + Verification (Tasks 14-16)

Add round-trip tests for quoted identifiers. Run full test suite.

---

## Task 1: Define Ident Struct

**Files:**
- Create: `src/ast/ident.rs`
- Modify: `src/ast/mod.rs:1` (add `pub mod ident;` and re-export)

**Step 1: Create `src/ast/ident.rs`**

```rust
use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

/// An identifier, decomposed into its value and quote style.
///
/// `quote_style` is `None` for unquoted identifiers (e.g. `mytable`),
/// `Some('"')` for double-quoted identifiers (e.g. `"MyTable"`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    /// The identifier value without quotes.
    pub value: String,
    /// The starting quote if any. `None` = unquoted, `Some('"')` = double-quoted.
    pub quote_style: Option<char>,
}

impl Ident {
    /// Create a new unquoted identifier.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            quote_style: None,
        }
    }

    /// Create a new quoted identifier.
    pub fn quoted(value: impl Into<String>, quote_style: char) -> Self {
        Self {
            value: value.into(),
            quote_style: Some(quote_style),
        }
    }

    /// Returns the identifier value as `&str`.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

// ── Deref<Target=str> ──
// Makes all `str` methods available on `Ident`: `.to_lowercase()`, `.len()`,
// `.starts_with()`, `.is_empty()`, etc. This is the primary mechanism that
// lets existing code compiled against `Vec<String>` continue to work with
// `Vec<Ident>` without modification.

impl Deref for Ident {
    type Target = str;
    fn deref(&self) -> &str {
        &self.value
    }
}

// ── Borrow<str> ──
// Enables `Vec<Ident>.join(".")` — the slice::join method requires
// `S: Borrow<str>` for `[S]::join(&str)`.

impl Borrow<str> for Ident {
    fn borrow(&self) -> &str {
        &self.value
    }
}

// ── Display ──
// Outputs the identifier with quotes if `quote_style` is set.
// Used by the formatter and any `format!("{}", ident)` call.

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.quote_style {
            None => f.write_str(&self.value),
            Some(q) => {
                let escaped = self.value.replace(q, &format!("{q}{q}"));
                write!(f, "{q}{escaped}{q}")
            }
        }
    }
}

// ── From impls ──

impl From<String> for Ident {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for Ident {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<&String> for Ident {
    fn from(s: &String) -> Self {
        Self::new(s)
    }
}

// ── PartialEq with str variants ──

impl PartialEq<str> for Ident {
    fn eq(&self, other: &str) -> bool {
        self.value == other
    }
}

impl PartialEq<&str> for Ident {
    fn eq(&self, other: &&str) -> bool {
        self.value == *other
    }
}

// ── Default ──
// Enables `Option<Ident>::unwrap_or_default()`.

impl Default for Ident {
    fn default() -> Self {
        Self {
            value: String::new(),
            quote_style: None,
        }
    }
}
```

**Step 2: Add module declaration in `src/ast/mod.rs:1`**

Add after `pub mod plpgsql;`:
```rust
pub mod ident;
```

And near the `ObjectName` definition (line 1419), add re-export:
```rust
pub use ident::Ident;
```

**Step 3: Run `cargo check`**

```bash
cargo check --all-features
```
Expected: PASS (no existing code depends on Ident yet).

**Step 4: Commit**

```bash
git add src/ast/ident.rs src/ast/mod.rs
git commit -m "feat(ast): add Ident struct with Deref/Borrow/Display trait impls"
```

---

## Task 2: Implement Custom Serde for Ident

**Files:**
- Modify: `src/ast/ident.rs` (add serde impls)
- Modify: `src/ast/ident.rs` header (add `#[cfg_attr(feature = "serde", ...)]` or direct derives)

**Step 1: Write the failing test**

Add to `src/ast/ident.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_unquoted_roundtrip() {
        let ident = Ident::new("mytable");
        let json = serde_json::to_string(&ident).unwrap();
        // Unquoted should serialize as plain string
        assert_eq!(json, r#""mytable""#);
        let de: Ident = serde_json::from_str(&json).unwrap();
        assert_eq!(de, ident);
    }

    #[test]
    fn test_serde_quoted_roundtrip() {
        let ident = Ident::quoted("MyTable", '"');
        let json = serde_json::to_string(&ident).unwrap();
        // Quoted should serialize as object
        assert_eq!(json, r#"{"value":"MyTable","quote_style":"\""}"#);
        let de: Ident = serde_json::from_str(&json).unwrap();
        assert_eq!(de, ident);
    }

    #[test]
    fn test_deserialize_plain_string_backward_compat() {
        // Old JSON format: plain string
        let de: Ident = serde_json::from_str(r#""mytable""#).unwrap();
        assert_eq!(de.value, "mytable");
        assert_eq!(de.quote_style, None);
    }
}
```

**Step 2: Run test to verify it fails**

```bash
cargo test --features serde ast::ident::tests -- --nocapture
```
Expected: FAIL — Ident doesn't implement Serialize/Deserialize yet.

**Step 3: Implement custom serde**

Add to `src/ast/ident.rs`:

```rust
use serde::{Deserialize, Serialize, Deserializer, Serializer};
use serde::de::{self, Visitor};

// Custom Serialize: unquoted → plain string, quoted → object
impl Serialize for Ident {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.quote_style {
            None => serializer.serialize_str(&self.value),
            Some(q) => {
                use serde::ser::SerializeStruct;
                let mut s = serializer.serialize_struct("Ident", 2)?;
                s.serialize_field("value", &self.value)?;
                s.serialize_field("quote_style", &q)?;
                s.end()
            }
        }
    }
}

// Custom Deserialize: accept both plain string and object
impl<'de> Deserialize<'de> for Ident {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct IdentVisitor;

        impl<'de> Visitor<'de> for IdentVisitor {
            type Value = Ident;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a string or an object with value and quote_style")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Ident, E> {
                Ok(Ident::new(value.to_string()))
            }

            fn visit_string<E: de::Error>(self, value: String) -> Result<Ident, E> {
                Ok(Ident::new(value))
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Ident, A::Error> {
                let mut value = None;
                let mut quote_style = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "value" => value = Some(map.next_value()?),
                        "quote_style" => quote_style = Some(map.next_value()?),
                        _ => {
                            return Err(de::Error::unknown_field(&key, &["value", "quote_style"]))
                        }
                    }
                }
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                Ok(Ident { value, quote_style })
            }
        }

        deserializer.deserialize_any(IdentVisitor)
    }
}
```

**Step 4: Run tests to verify they pass**

```bash
cargo test --features serde ast::ident::tests -- --nocapture
```
Expected: PASS

**Step 5: Commit**

```bash
git add src/ast/ident.rs
git commit -m "feat(ast): custom serde for Ident — backward compatible JSON"
```

---

## Task 3: Change ObjectName Type Alias

**Files:**
- Modify: `src/ast/mod.rs:1419`

**Step 1: Change the type alias**

```rust
// Before:
pub type ObjectName = Vec<String>;

// After:
pub type ObjectName = Vec<Ident>;
```

**Step 2: Export Ident from lib.rs**

In `src/lib.rs:81`, add `Ident` to the re-export list:

```rust
// Before:
GlobalIndexColumn, IndexNulls, IndexOrdering, InsertStatement, Literal, MergeStatement, MoveStatement, ObjectName,

// After:
GlobalIndexColumn, Ident, IndexNulls, IndexOrdering, InsertStatement, Literal, MergeStatement, MoveStatement, ObjectName,
```

**Step 3: Run `cargo check`**

```bash
cargo check --all-features 2>&1 | head -80
```

Expected: Many errors — this is expected. The errors will fall into categories:
- `parse_identifier` returns `String` but ObjectName needs `Ident` (parser)
- `.cloned()` returns `Ident` where `String` expected (consumers)
- Test assertions comparing with `Vec<String>` (tests)

**Do NOT fix these yet.** We'll address them in the following tasks. Commit just the type change as a checkpoint.

**Step 4: Commit**

```bash
git add src/ast/mod.rs src/lib.rs
git commit -m "refactor(ast): change ObjectName from Vec<String> to Vec<Ident>"
```

---

## Phase 2: Parser Changes

## Task 4: Update parse_identifier to Return Ident

**Files:**
- Modify: `src/parser/mod.rs:823-879` (parse_identifier, consume_any_identifier, parse_object_name)

**Step 1: Change `parse_identifier`**

```rust
// src/parser/mod.rs:843-869

// Before:
fn parse_identifier(&mut self) -> Result<String, ParserError> {
    match self.peek().clone() {
        Token::Ident(s) => { self.advance(); Ok(s) }
        Token::QuotedIdent(s) => { self.advance(); Ok(s) }
        Token::Keyword(kw) => {
            ...
            Ok(name)
        }
        ...
    }
}

// After:
fn parse_identifier(&mut self) -> Result<crate::ast::Ident, ParserError> {
    match self.peek().clone() {
        Token::Ident(s) => {
            self.advance();
            Ok(crate::ast::Ident::new(s))
        }
        Token::QuotedIdent(s) => {
            self.advance();
            Ok(crate::ast::Ident::quoted(s, '"'))
        }
        Token::Keyword(kw) => {
            let location = self.current_location();
            self.advance();
            let name = kw.as_str().to_string();
            if kw.category() == crate::token::keyword::KeywordCategory::Reserved {
                self.add_error(ParserError::ReservedKeywordAsIdentifier { keyword: name.clone(), location });
            }
            Ok(crate::ast::Ident::new(name))
        }
        _ => Err(ParserError::UnexpectedToken {
            location: self.current_location(),
            expected: "identifier".to_string(),
            got: format!("{:?}", self.peek()),
        }),
    }
}
```

**Step 2: Change `consume_any_identifier`**

```rust
// src/parser/mod.rs:823-839

// Before:
fn consume_any_identifier(&mut self) -> Result<String, ParserError> {
    match self.peek().clone() {
        Token::Ident(s) | Token::QuotedIdent(s) => {
            self.advance();
            Ok(s)
        }
        Token::Keyword(kw) => {
            self.advance();
            Ok(kw.as_str().to_string())
        }
        ...
    }
}

// After:
fn consume_any_identifier(&mut self) -> Result<crate::ast::Ident, ParserError> {
    match self.peek().clone() {
        Token::Ident(s) => {
            self.advance();
            Ok(crate::ast::Ident::new(s))
        }
        Token::QuotedIdent(s) => {
            self.advance();
            Ok(crate::ast::Ident::quoted(s, '"'))
        }
        Token::Keyword(kw) => {
            self.advance();
            Ok(crate::ast::Ident::new(kw.as_str().to_string()))
        }
        ...
    }
}
```

**Step 3: Verify `parse_object_name` still compiles**

```rust
// src/parser/mod.rs:872-879 — NO CHANGES NEEDED
// parse_identifier now returns Ident, vec![Ident] == ObjectName == Vec<Ident>
fn parse_object_name(&mut self) -> Result<crate::ast::ObjectName, ParserError> {
    let mut name = vec![self.parse_identifier()?];  // Ident → Vec<Ident> ✓
    while self.match_token(&Token::Dot) {
        self.advance();
        name.push(self.parse_identifier()?);  // Ident → Vec<Ident> ✓
    }
    Ok(name)
}
```

**Step 4: Run `cargo check`**

```bash
cargo check --all-features 2>&1 | grep "^error" | wc -l
```

Note the error count. Many call sites that use `parse_identifier()?` as a `String` (passing to functions expecting `&str` or `String`) will fail.

**Common fix patterns for call sites:**

| Pattern | Fix |
|---------|-----|
| `let s: String = parse_identifier()?;` | `let s = parse_identifier()?;` (s is now Ident, Deref handles str usage) |
| `self.parse_identifier()?.to_lowercase()` | Works via Deref — no change |
| `format!("{}", parse_identifier()?)` | Works via Display — no change |
| Passing to function expecting `String` | Add `.value` or `.to_string()` |
| Passing to function expecting `&str` | Add `.as_str()` or use deref coercion `&*ident` |
| `parse_identifier()? == "foo"` | Works via PartialEq<str> — no change |

**Step 5: Fix call sites iteratively**

Focus on errors in this order (by file, most calls first):
1. `src/parser/utility/statements.rs` (~96 calls)
2. `src/parser/ddl/alter.rs` (~88 calls)
3. `src/parser/ddl/create.rs` (~83 calls)
4. `src/parser/mod.rs` (~60 calls)
5. `src/parser/plpgsql.rs` (~30 calls)
6. `src/parser/select.rs` (~22 calls)
7. `src/parser/ddl/table.rs` (~30 calls)
8. `src/parser/utility/grant.rs` (~31 calls)
9. `src/parser/expr.rs` (~13 calls)
10. `src/parser/utility/copy_explain.rs` (~12 calls)
11. `src/parser/utility/functions.rs` (~6 calls)
12. `src/parser/dml.rs` (~12 calls)
13. `src/parser/ddl/drop.rs` (~1 call)

Most will compile without changes due to Deref/Borrow/Display. The ones that need changes typically involve:
- Assigning to a `String` typed variable: `let name: String = ...` → `let name = ...` or add `.value`
- Passing `Ident` where `String` is expected: add `.to_string()` or `.value`

**Step 6: Commit after each file compiles**

```bash
git add -A && git commit -m "refactor(parser): update parse_identifier to return Ident — <filename>"
```

---

## Task 5: Update parse_pl_variable_or_column

**Files:**
- Modify: `src/parser/mod.rs:93`

**Context:** This function checks `name[0]` against declared variables.

```rust
// Before:
let base = if !name.is_empty() && self.is_var_declared(&name[0]) {

// After (name[0] is Ident, Deref<Target=str> makes &name[0] coerce to &str):
let base = if !name.is_empty() && self.is_var_declared(&name[0]) {
```

This should compile unchanged because `Ident: Deref<Target=str>`, so `&name[0]` (which is `&Ident`) coerces to `&str`.

**Verify:** `cargo check --all-features`

---

## Task 6: Update Sequence Detection in expr.rs

**Files:**
- Modify: `src/parser/expr.rs:1183-1198`

```rust
// Before:
if name.len() >= 2 {
    let last = name.last().expect("...").to_lowercase();
    if last == "nextval" || last == "currval" {
        let func = if last == "nextval" { SequenceFunc::Nextval } else { SequenceFunc::Currval };
        let mut seq_parts = name.clone();
        seq_parts.pop();

// After — NO CHANGES NEEDED
// name.last() returns Option<&Ident>, .to_lowercase() works via Deref
// name.clone() returns Vec<Ident>, seq_parts.pop() returns Option<Ident>
// All compile unchanged.
```

**Verify:** `cargo check --all-features`

---

## Phase 3: Formatter Changes

## Task 7: Add format_ident and Update format_object_name

**Files:**
- Modify: `src/formatter/mod.rs:1431-1459`

**Step 1: Write failing test**

```rust
#[cfg(test)]
mod ident_quote_tests {
    use crate::{Tokenizer, Parser, SqlFormatter};

    #[test]
    fn test_unquoted_uppercase_not_quoted() {
        let (stmts, _) = Parser::parse_sql("SELECT * FROM MyTable");
        let out = SqlFormatter::new().format_statement(&stmts[0].statement);
        assert!(
            !out.contains("\"MyTable\""),
            "Unquoted identifier should not gain quotes. Got: {out}"
        );
    }

    #[test]
    fn test_quoted_preserved() {
        let (stmts, _) = Parser::parse_sql("SELECT * FROM \"MyTable\"");
        let out = SqlFormatter::new().format_statement(&stmts[0].statement);
        assert!(
            out.contains("\"MyTable\""),
            "Quoted identifier should keep quotes. Got: {out}"
        );
    }

    #[test]
    fn test_quoted_lowercase_preserved() {
        let (stmts, _) = Parser::parse_sql("SELECT * FROM \"mytable\"");
        let out = SqlFormatter::new().format_statement(&stmts[0].statement);
        assert!(
            out.contains("\"mytable\""),
            "Quoted lowercase identifier should keep quotes. Got: {out}"
        );
    }
}
```

**Step 2: Run tests to verify they fail**

```bash
cargo test ident_quote_tests -- --nocapture
```
Expected: FAIL.

**Step 3: Add `format_ident` and update `format_object_name`**

```rust
// Add after quote_identifier (around line 1454):

/// Format an Ident using its quote_style, not a heuristic.
fn format_ident(&self, ident: &crate::ast::Ident) -> String {
    match ident.quote_style {
        None => ident.value.clone(),
        Some(q) => {
            let escaped = ident.value.replace(q, &format!("{q}{q}"));
            format!("{q}{escaped}{q}")
        }
    }
}

// Update format_object_name:
// Before:
fn format_object_name(&self, name: &ObjectName) -> String {
    let parts: Vec<String> = name.iter().map(|s| self.quote_identifier(s)).collect();
    parts.join(".")
}

// After:
fn format_object_name(&self, name: &ObjectName) -> String {
    let parts: Vec<String> = name.iter().map(|i| self.format_ident(i)).collect();
    parts.join(".")
}
```

**Step 4: Update other formatter sites that use `quote_identifier` on ObjectName elements**

The formatter has patterns like:
```rust
// Lines using quote_identifier on ObjectName parts:
name.iter().map(|s| self.quote_identifier(s))  // → format_ident
```

Search for all sites where `quote_identifier` is called on elements of an `ObjectName`/`Vec<Ident>`:

```bash
grep -n "quote_identifier" src/formatter/mod.rs | grep -v "fn quote_identifier"
```

Each of these needs to be changed to `format_ident` IF the argument is an `Ident`. Sites where the argument is a plain `String`/`&str` (column aliases, constraint names, etc.) should stay as `quote_identifier`.

**Key pattern to fix:**

```rust
// Before (many sites):
name.iter().map(|s| self.quote_identifier(s)).collect()

// After:
name.iter().map(|i| self.format_ident(i)).collect()
```

This appears at approximately lines: 691, 730, 748, 783, 968, 1624, 2565, 2616, 2708, 2985, 3055, 3362, 4291, 4297.

**Note:** Some of these lines call `quote_identifier_relaxed` — update those to use a `format_ident_relaxed` variant OR just use `format_ident` since relaxed quoting was a workaround for the same bug.

**Step 5: Run formatter tests**

```bash
cargo test ident_quote_tests -- --nocapture
```
Expected: PASS.

**Step 6: Run full test suite**

```bash
cargo test --all-features 2>&1 | tail -20
```

**Step 7: Commit**

```bash
git add src/formatter/mod.rs
git commit -m "feat(formatter): use quote_style from Ident instead of uppercase heuristic"
```

---

## Task 8: Keep quote_identifier for Non-ObjectName Strings

**Files:**
- Modify: `src/formatter/mod.rs:1431-1443`

`quote_identifier(&str)` is still needed for strings that are NOT ObjectName elements — column aliases, constraint names, index names stored as `String` in the AST.

**No changes needed to the function itself.** It stays as-is for non-identifier strings. The key change is that `format_object_name` and other ObjectName-related formatting now use `format_ident` instead.

**Verify:** `cargo check --all-features`

---

## Task 9: Update QualifiedStar and ColumnRef Formatting

**Files:**
- Modify: `src/formatter/mod.rs` — search for `Expr::ColumnRef` and `Expr::QualifiedStar` formatting

```rust
// Before (ColumnRef formatting):
Expr::ColumnRef(name) => {
    let parts: Vec<String> = name.iter().map(|s| self.quote_identifier(s)).collect();
    parts.join(".")
}

// After:
Expr::ColumnRef(name) => {
    let parts: Vec<String> = name.iter().map(|i| self.format_ident(i)).collect();
    parts.join(".")
}
```

Same pattern for `Expr::PlVariable`, `Expr::FunctionCall { name, .. }`, `Expr::SequenceValue { sequence, .. }`, and `TableRef::Table { name, .. }`.

**Verify:** `cargo check --all-features`

---

## Phase 4: Fix Breaking Sites

## Task 10: Fix `.cloned()` Patterns in Analyzer/Linter/CLI

**Files:**
- `src/analyzer/return_cursor.rs:145-146`
- `src/analyzer/schema.rs:233,339`
- `src/analyzer/mod.rs:617,1840,2278,2322,2401`
- `src/linter/rules_caution.rs:324,335,354`
- `src/linter/rules_prohibition.rs:327,336,421,479,511`
- `src/linter/rules_performance.rs:352,409,609,1056,636`
- `src/bin/ogsql.rs:748,776,815,2221`

**Pattern:** `name.last().cloned()` returns `Option<Ident>` instead of `Option<String>`.

**Fix strategy — three options (pick based on usage context):**

```rust
// Option A: If used as string via Deref → works unchanged
name.last().cloned().unwrap_or_default().to_lowercase()  // ✓ Deref handles it

// Option B: If explicitly needs String
name.last().map(|i| i.value.clone()).unwrap_or_default()  // explicit

// Option C: If used in format!
format!("{}", name.last().unwrap())  // ✓ Display handles it
```

**Most common pattern — `.cloned().unwrap_or_default().to_lowercase()`:**

This pattern appears frequently in the linter:
```rust
tables.push(name.last().cloned().unwrap_or_default().to_lowercase());
```
With `Ident: Default + Deref<Target=str>`, this compiles unchanged:
- `.cloned()` → `Option<Ident>`
- `.unwrap_or_default()` → `Ident::default()` (empty value, None quote_style)
- `.to_lowercase()` → `str::to_lowercase()` via Deref → returns `String`

**So most of these sites need NO CHANGES.** Only fix sites where the `Ident` is used in a context that truly requires `String` (function arguments, struct fields).

**Verify:** `cargo check --all-features` and fix remaining errors.

---

## Task 11: Fix `name.last().map(|s| s.as_str())` Patterns

**Files:**
- `src/linter/rules_prohibition.rs:479`
- `src/bin/ogsql.rs` (various)

```rust
// Before:
name.last().map(|s| s.as_str())

// After:
name.last().map(|i| i.as_str())
```

Since `Ident` has an `as_str()` method, this works with a one-character change (`s` → `i` or just rename).

**Verify:** `cargo check --all-features`

---

## Task 12: Fix Direct ObjectName Construction

**Files:**
- `src/ast/visitor.rs:1379`
- `src/ast/visitor_tests.rs:246,258`
- `src/parser/utility/copy_explain.rs:861`

```rust
// Before:
Expr::ColumnRef(crate::ast::ObjectName::from(vec!["x".to_string()]))

// After:
Expr::ColumnRef(vec![crate::ast::Ident::new("x")])

// Before:
name: vec!["schema".to_string(), "proc".to_string()],

// After:
name: vec!["schema".into(), "proc".into()],
```

**Verify:** `cargo check --all-features`

---

## Task 13: Fix type_helpers.rs Indexing

**Files:**
- `src/linter/type_helpers.rs:37,60,65,77,93`

```rust
// Before:
let lookup_name = alias.as_deref().unwrap_or_else(|| name.last().unwrap_or(&name[0]));
let full = format!("{}.{}", name[0].to_lowercase(), name[1].to_lowercase());
let single = name.last().expect("...").to_lowercase();
if col == &name[0].to_lowercase() { ... }
let table = obj_name[0].to_lowercase();

// After — NO CHANGES NEEDED
// All of these work via Deref<Target=str>:
// - name.last() returns Option<&Ident>, unwrap_or(&name[0]) works (both &Ident)
// - name[0].to_lowercase() works via Deref
// - &name[0].to_lowercase() works via Deref
```

**Verify:** `cargo check --all-features`

---

## Phase 5: Tests + Verification

## Task 14: Fix Existing Test Assertions

**Files:**
- `src/parser/tests.rs:957,961,1007,1010`
- `src/ast/visitor_tests.rs:151,258,350`

```rust
// Before (parser/tests.rs):
assert_eq!(name.as_slice(), &["my_func"]);
assert_eq!(var_name.as_slice(), &["v_param"]);

// After:
assert_eq!(name.iter().map(|i| i.as_str()).collect::<Vec<_>>(), &["my_func"]);
// OR:
assert_eq!(name.len(), 1);
assert_eq!(name[0].as_str(), "my_func");
```

```rust
// Before (visitor_tests.rs):
assert_eq!(visitor.procedure_calls[0], vec!["schema".to_string(), "proc".to_string()]);

// After:
assert_eq!(
    visitor.procedure_calls[0].iter().map(|i| i.as_str()).collect::<Vec<_>>(),
    vec!["schema", "proc"]
);
```

**Verify:** `cargo test --all-features`

---

## Task 15: Add Round-Trip Tests for Quoted Identifiers

**Files:**
- Create: `tests/quoted_identifier_roundtrip.rs`

```rust
use ogsql_parser::{Tokenizer, Parser, SqlFormatter};

fn format_sql(sql: &str) -> String {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    let formatter = SqlFormatter::new();
    stmts.iter().map(|s| formatter.format_statement(&s.statement)).collect::<Vec<_>>().join(";\n")
}

#[test]
fn unquoted_uppercase_stays_unquoted() {
    let sql = "SELECT * FROM MyTable";
    let out = format_sql(sql);
    assert!(!out.contains("\"MyTable\""), "Got: {out}");
}

#[test]
fn quoted_identifier_preserved() {
    let sql = "SELECT * FROM \"MyTable\"";
    let out = format_sql(sql);
    assert!(out.contains("\"MyTable\""), "Got: {out}");
}

#[test]
fn quoted_lowercase_preserved() {
    let sql = "SELECT * FROM \"mytable\"";
    let out = format_sql(sql);
    assert!(out.contains("\"mytable\""), "Got: {out}");
}

#[test]
fn mixed_quoted_unqualified() {
    let sql = "SELECT id FROM userDetails WHERE id = 1";
    let out = format_sql(sql);
    assert!(!out.contains("\""), "No quotes expected. Got: {out}");
}

#[test]
fn qualified_name_with_quotes() {
    let sql = "SELECT * FROM public.\"MyTable\"";
    let out = format_sql(sql);
    assert!(out.contains("\"MyTable\""), "Got: {out}");
}

#[test]
fn column_alias_with_uppercase_no_quotes() {
    let sql = "SELECT col AS MyAlias FROM t";
    let out = format_sql(sql);
    // Alias is String, not Ident — may still be quoted by quote_identifier.
    // This test documents current behavior.
}

#[test]
fn create_table_quoted_name() {
    let sql = "CREATE TABLE \"MyTable\" (id INT)";
    let out = format_sql(sql);
    assert!(out.contains("\"MyTable\""), "Got: {out}");
}

#[test]
fn function_call_quoted_name() {
    let sql = "SELECT \"myFunc\"(col) FROM t";
    let out = format_sql(sql);
    assert!(out.contains("\"myFunc\""), "Got: {out}");
}

#[test]
fn json_roundtrip_unquoted() {
    let sql = "SELECT * FROM MyTable";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    let json = serde_json::to_string(&stmts).unwrap();
    // Should serialize as plain string array (backward compat)
    assert!(json.contains("\"MyTable\"") || json.contains("MyTable"));
    let restored: Vec<ogsql_parser::Statement> = serde_json::from_str(&json).unwrap();
    let formatter = SqlFormatter::new();
    let out = formatter.format_statement(&restored[0].statement);
    assert!(!out.contains("\"MyTable\""), "Got: {out}");
}
```

**Verify:**
```bash
cargo test --test quoted_identifier_roundtrip -- --nocapture
```

---

## Task 16: Full Verification

**Step 1: Format check**

```bash
cargo fmt --all -- --check
```

**Step 2: Clippy**

```bash
cargo clippy --all-features -- -D warnings
```

**Step 3: Full test suite**

```bash
cargo test --all-features
```

**Step 4: Regression tests**

```bash
cargo run --example regression 2>&1 | tail -5
```

Expected: All 1409 regression tests pass.

**Step 5: JSON round-trip manual test**

```bash
echo 'SELECT * FROM "MyTable"' | cargo run --features cli -- parse -j | cargo run --features cli -- json2sql
```

Expected: Output preserves quotes: `SELECT * FROM "MyTable"`

**Step 6: Commit all remaining changes**

```bash
git add -A
git commit -m "test: add quoted identifier round-trip tests and fix assertions"
```

---

## Risk Assessment

| Risk | Likelihood | Mitigation |
|------|-----------|------------|
| Deref<Target=str> has edge cases | Low | Rust's Deref is well-understood; the main gotcha is Deref coercion doesn't apply in all generic contexts |
| Serde custom impl breaks edge cases | Low | Tests cover both formats; `deserialize_any` handles both string and object |
| Hidden call sites outside src/ | Medium | `cargo check --all-features` catches all; also check examples/ and tests/ |
| Regression test SQL uses quoted identifiers | Low | Check by running regression suite |
| Performance impact | Negligible | Ident is a String + Option<char>, no heap allocation overhead vs String |

---

## Migration Checklist

- [ ] Task 1: Ident struct defined with all trait impls
- [ ] Task 2: Custom serde for Ident (backward compatible)
- [ ] Task 3: ObjectName changed to Vec<Ident>
- [ ] Task 4: parse_identifier returns Ident
- [ ] Task 5: parse_pl_variable_or_column verified
- [ ] Task 6: Sequence detection verified
- [ ] Task 7: format_ident added, format_object_name updated
- [ ] Task 8: quote_identifier kept for non-ObjectName strings
- [ ] Task 9: All ObjectName formatting sites updated
- [ ] Task 10: `.cloned()` patterns fixed
- [ ] Task 11: `.as_str()` patterns fixed
- [ ] Task 12: Direct ObjectName construction fixed
- [ ] Task 13: type_helpers.rs verified
- [ ] Task 14: Test assertions fixed
- [ ] Task 15: Round-trip tests added
- [ ] Task 16: Full verification passes (fmt, clippy, test, regression)
