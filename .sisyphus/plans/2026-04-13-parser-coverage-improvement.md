# Parser Coverage Improvement Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Improve ogsql-parser pass rate from 50.8% to 80%+ on 758 GaussDB SQL test files by fixing 6 major error categories.

**Architecture:** The parser is a hand-written recursive descent parser in Rust. Main files:
- `src/ast/mod.rs` (2045 lines) — AST types (Statement enum, all structs)
- `src/ast/plpgsql.rs` (491 lines) — PL/pgSQL AST types
- `src/parser/mod.rs` (2302 lines) — Parser core + dispatch (needs modularization)
- `src/parser/ddl.rs` (2516 lines) — DDL parsers (CREATE/ALTER/DROP, needs modularization)
- `src/parser/expr.rs` (992 lines) — Expression parser
- `src/parser/plpgsql.rs` (1828 lines) — PL/pgSQL parser
- `src/parser/utility.rs` (3884 lines) — COPY/EXPLAIN/GRANT/etc (needs modularization)
- `src/parser/select.rs` (682 lines) — SELECT parser
- `src/parser/dml.rs` (401 lines) — INSERT/UPDATE/DELETE/MERGE
- `src/token/mod.rs` (125 lines) — Token enum
- `src/token/tokenizer.rs` (986 lines) — Lexer

**Tech Stack:** Rust 2021, thiserror 2, serde/serde_json, clap 4

**Verification command:** `bash /tmp/batch_verify.sh` (reruns all 758 files)

---

## Modularization Strategy

Current problem files that need splitting:
- `parser/mod.rs` (2302 lines) — Split dispatch into `parser/dispatch.rs`
- `parser/ddl.rs` (2516 lines) — Split into `parser/ddl_table.rs`, `parser/ddl_other.rs`
- `parser/utility.rs` (3884 lines) — Split into `parser/utility_copy.rs`, `parser/utility_grant.rs`, `parser/utility_other.rs`

**Rule:** Any .rs file exceeding ~1500 lines should be split. New files stay under 1000 lines.

---

## Task 1: Extend DROP statement dispatch (P0 — +125 files)

**Files:**
- Modify: `src/parser/ddl.rs:1463-1588` (parse_drop + parse_drop_statement_with_type)
- Modify: `src/ast/mod.rs:286-314` (ObjectType enum)

**What:** Add missing DROP object types to ObjectType enum and parse_drop match. Currently missing:
  - User, Role, Group, Tablespace (already in enum but not parsed)
  - ResourcePool, WorkloadGroup, AuditPolicy, MaskingPolicy, RlsPolicy
  - DataSource, Directory, Event, Publication, Subscription, Server
  - TextSearchConfig, TextSearchDict, Domain, Rule, Aggregate, Cast, Conversion
  - Operator, OperatorClass, OperatorFamily, UserMapping, Synonym, Model
  - PolicyLabel, WeakPasswordDictionary, ClientMasterKey, ColumnEncryption
  - Global, App, Node, NodeGroup, OpClass, OpFamily

**Step 1:** Add all missing ObjectType variants to `ObjectType` enum in `src/ast/mod.rs`

```rust
pub enum ObjectType {
    // existing...
    User,
    Role,
    Group,
    ResourcePool,
    WorkloadGroup,
    AuditPolicy,
    MaskingPolicy,
    RlsPolicy,
    DataSource,
    Directory,
    Event,
    Publication,
    Subscription,
    TextSearchConfig,
    TextSearchDict,
    Domain,
    Rule,
    Aggregate,
    Cast,
    Conversion,
    Operator,
    OperatorClass,
    OperatorFamily,
    UserMapping,
    Synonym,
    Model,
    PolicyLabel,
    WeakPasswordDictionary,
    ClientMasterKey,
    ColumnEncryption,
    Global,
    App,
    Node,
    NodeGroup,
    OpClass,
    OpFamily,
    SecurityLabel,
    ForeignDataWrapper,
    Language,
}
```

**Step 2:** Add keyword-to-ObjectType mappings in `parse_drop()` in `src/parser/ddl.rs`. Pattern:
```rust
Some(Keyword::USER) => { self.advance(); ObjectType::User }
Some(Keyword::ROLE) => { self.advance(); ObjectType::Role }
// ... etc
```

For multi-word types like `DROP TEXT SEARCH CONFIGURATION`:
```rust
Some(Keyword::TEXT_P) => {
    self.advance();
    self.expect_keyword(Keyword::SEARCH)?;
    if self.match_keyword(Keyword::CONFIGURATION) {
        self.advance(); ObjectType::TextSearchConfig
    } else {
        self.expect_keyword(Keyword::DICTIONARY)?;
        ObjectType::TextSearchDict
    }
}
```

**Step 3:** Handle `DROP OWNED BY name` as special case in dispatch_drop before parse_drop.

**Step 4:** Run `bash /tmp/batch_verify.sh` and verify ~125 more files pass.

---

## Task 2: Support ARRAY[...] bracket syntax (P0 — +50 files)

**Files:**
- Modify: `src/parser/expr.rs:344-366` (ARRAY parsing in parse_primary_expr)
- Modify: `src/parser/expr.rs:116-245` (try_postfix_op — add subscript)

**What:** Add `ARRAY[...]` support alongside `ARRAY(...)`, and add `expr[index]` subscript.

**Step 1:** In `parse_primary_expr` at the ARRAY branch (line 344), add LBracket handling:

```rust
Token::Keyword(Keyword::ARRAY) => {
    self.advance();
    if self.match_token(&Token::LParen) {
        // existing ARRAY(...) code stays
    } else if self.match_token(&Token::LBracket) {
        self.advance();
        if self.match_keyword(Keyword::SELECT) || self.match_keyword(Keyword::WITH) {
            let subquery = self.parse_select_statement()?;
            self.expect_token(&Token::RBracket)?;
            return Ok(Expr::Subquery(Box::new(subquery)));
        }
        let mut elems = vec![self.parse_expr()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            elems.push(self.parse_expr()?);
        }
        self.expect_token(&Token::RBracket)?;
        return Ok(Expr::Array(elems));
    }
    // ... error case
}
```

**Step 2:** In `try_postfix_op`, add LBracket subscript handling:

```rust
Token::LBracket => {
    self.advance();
    let index = self.parse_expr()?;
    self.expect_token(&Token::RBracket)?;
    // Store as function call "array_index" or new Expr variant
    *expr = Expr::FunctionCall { name: ObjectName::from_parts(vec![]), args: vec![...] };
    return Ok(true);
}
```

**Step 3:** Verify with: `echo "SELECT unnest(ARRAY[1,2])" | cargo run -- parse -j`

---

## Task 3: PL/pgSQL type declarations — IS TABLE OF / VARRAY (P0 — +60 files)

**Files:**
- Modify: `src/parser/plpgsql.rs:275-304` (parse_pl_type_decl)
- Modify: `src/ast/plpgsql.rs` (add PlTypeDecl variants)

**What:** Extend `parse_pl_type_decl` to handle:
- `TYPE name IS TABLE OF type [INDEX BY type]` (nested table / index-by table)
- `TYPE name IS VARRAY(n) OF type` (varray)
- `TYPE name IS RECORD(...)` (already partially handled)

**Step 1:** Add new AST variants in `src/ast/plpgsql.rs`:

```rust
pub enum PlTypeDecl {
    Record { name: String, fields: Vec<PlTypeField> },  // existing, rename
    TableOf { name: String, elem_type: PlDataType, index_by: Option<PlDataType> },
    VarrayOf { name: String, size: Expr, elem_type: PlDataType },
}
```

**Step 2:** Rewrite `parse_pl_type_decl` in plpgsql.rs to branch after `IS`:
```rust
fn parse_pl_type_decl(&mut self, name: String) -> Result<PlDeclaration, ParserError> {
    self.expect_keyword(Keyword::TYPE_P)?;
    self.expect_ident_str("is")?;

    if self.match_ident_str("record") {
        // existing record parsing...
    } else if self.match_ident_str("table") {
        self.advance();
        self.expect_ident_str("of")?;
        let elem_type = self.parse_pl_data_type()?;
        let mut index_by = None;
        if self.match_ident_str("index") {
            self.advance();
            self.expect_ident_str("by")?;
            index_by = Some(self.parse_pl_data_type()?);
        }
        self.try_consume_semicolon();
        Ok(PlDeclaration::Type(PlTypeDecl::TableOf { name, elem_type, index_by }))
    } else if self.match_ident_str("varray") || self.match_ident_str("varying") {
        // VARRAY(n) OF type
        self.advance();
        self.expect_token(&Token::LParen)?;
        let size = self.parse_expr()?;
        self.expect_token(&Token::RParen)?;
        self.expect_ident_str("of")?;
        let elem_type = self.parse_pl_data_type()?;
        self.try_consume_semicolon();
        Ok(PlDeclaration::Type(PlTypeDecl::VarrayOf { name, size, elem_type }))
    } else {
        // fallback: skip to semicolon, return as raw type
        Err(...)
    }
}
```

---

## Task 4: GRANT/REVOKE syntax extensions (P1 — +15 files)

**Files:**
- Modify: `src/parser/utility.rs:1905-2370` (GRANT/REVOKE parsing)
- Modify: `src/ast/mod.rs:1461-1470` (GrantTarget enum)

**What:** Add missing GRANT target types and syntax patterns:

**Step 1:** Add to GrantTarget enum:
```rust
pub enum GrantTarget {
    // existing...
    Tablespace(Vec<String>),
    ForeignServer(Vec<String>),
    ForeignDataWrapper(Vec<String>),
    AllTablesInTablespace(Vec<String>),
}
```

**Step 2:** In `parse_grant_target`, add TABLESPACE/SERVER/FDW cases.

**Step 3:** Fix GRANT role dispatch in `parse_statement` (mod.rs around line 903) — the condition that decides between `parse_grant` vs `parse_grant_role` needs to handle `GRANT ALL PRIVILEGES TO user` (role grant without ON).

**Step 4:** Add column-level privilege support: `GRANT SELECT(col1, col2) ON table TO user`

---

## Task 5: CREATE TABLESPACE RELATIVE LOCATION (P1 — +40 errors)

**Files:**
- Modify: `src/parser/ddl.rs:2034-2066` (parse_create_tablespace)

**What:** Add `RELATIVE` keyword support before `LOCATION` in CREATE TABLESPACE.

**Step 1:** Add `relative: bool` field to `CreateTablespaceStatement` struct.

**Step 2:** In parse_create_tablespace, consume optional `RELATIVE` before `LOCATION`.

---

## Task 6: EXPLAIN variants (P2 — +12 errors)

**Files:**
- Modify: `src/parser/utility.rs:458-518` (parse_explain)

**What:** Fix EXPLAIN parsing for:
1. `EXPLAIN SELECT ...` (no options, no parens) — currently works
2. `EXPLAIN(costs off)SELECT ...` (no space after parens) — should work already
3. `EXPLAIN PLAN SET STATEMENT_ID = 'id' FOR SELECT ...` — need STATEMENT keyword fix

**Step 1:** Check if `Keyword::STATEMENT` vs `Keyword::STATEMENT_ID` mismatch exists. The error says `STATEMENT_ID` was expected but `Keyword(STATEMENT)` was found — this is a keyword mapping issue.

---

## Task 7: Modularize parser files (ongoing)

**Files to split:**
- `parser/utility.rs` (3884 lines) → `parser/copy.rs`, `parser/grant.rs`, `parser/utility.rs` (remaining)
- `parser/ddl.rs` (2516 lines) → `parser/ddl_table.rs`, `parser/ddl_index.rs`, `parser/ddl_other.rs`
- `parser/mod.rs` (2302 lines) → extract dispatch_create/dispatch_alter/dispatch_drop to `parser/dispatch.rs`

**Rule:** Keep each file under 1000 lines. Use `pub(crate) mod` and `impl Parser` in each.

---

## Task 8: Final verification

**Step 1:** Run `bash /tmp/batch_verify.sh`
**Step 2:** Verify pass rate > 80%
**Step 3:** Run `cargo test` to ensure no regressions
**Step 4:** Run `cargo build --release` to verify clean build
