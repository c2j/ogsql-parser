# DDL Phase 3B Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Complete core DDL implementation by wiring up TRUNCATE, implementing CREATE VIEW, CREATE SCHEMA, CREATE DATABASE, CREATE TABLESPACE, and establishing parser unit tests.

**Architecture:** Extend existing recursive descent parser pattern used in `ddl.rs`. AST types currently use `stub_struct!` macro - replace with real struct definitions. Tests follow Rust built-in test framework in same file or `tests/` module.

**Tech Stack:** Rust 2021, thiserror, built-in test framework

**Grammar Reference:** `lib/openGauss-server/src/common/backend/parser/gram.y`
- `CreateSchemaStmt`: lines 5161-5200
- `CreateTableSpaceStmt`: lines 5568-5593
- `CreateSeqStmt`: lines 5237-5264
- `ViewStmt`: lines 5202-5235
- `TruncateStmt`: lines 24760+

---

## Prerequisites

Before starting:
1. Verify project builds: `cargo build` (should pass with warnings)
2. Verify tests pass: `cargo test` (28 tests should pass)
3. Have `gram.y` reference available for syntax questions

---

## Task 1: Wire Up TRUNCATE Statement

**Why first:** Parser already exists but isn't connected to dispatcher. 15-minute win.

**Files:**
- Modify: `src/parser/mod.rs:270-277` (statement dispatch)

**Step 1: Add TRUNCATE branch to parse_statement**

In `src/parser/mod.rs`, after the MERGE branch (around line 256), add:

```rust
Token::Keyword(Keyword::TRUNCATE) => {
    self.advance();
    match self.parse_truncate() {
        Ok(stmt) => {
            self.try_consume_semicolon();
            crate::ast::Statement::Truncate(stmt)
        }
        Err(_) => self.skip_to_semicolon(),
    }
}
```

**Step 2: Import TruncateStatement**

Add to imports at top of `src/parser/mod.rs` if not already there:
```rust
use crate::ast::TruncateStatement;
```

**Step 3: Verify build**

Run: `cargo build`
Expected: Compiles successfully (may have warnings)

**Step 4: Manual test**

Create test file `/tmp/test_truncate.rs`:
```rust
use ogsql_parser::parser::Parser;
use ogsql_parser::token::tokenizer::Tokenizer;

fn main() {
    let sql = "TRUNCATE TABLE users, orders CASCADE";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    println!("{:?}", stmts);
}
```

Run: `cargo run --example /tmp/test_truncate.rs`
Expected: Prints parsed Truncate statement

**Step 5: Commit**

```bash
git add src/parser/mod.rs
git commit -m "feat: wire up TRUNCATE statement parser"
```

---

## Task 2: Implement CREATE VIEW Statement

**Files:**
- Modify: `src/ast/mod.rs:551` - Replace stub with real struct
- Modify: `src/parser/ddl.rs` - Add parse_create_view method
- Modify: `src/parser/mod.rs:302` - Add CREATE VIEW dispatcher

**Step 1: Replace CreateViewStatement stub with real definition**

In `src/ast/mod.rs`, find line 551 in the stub_struct! macro and remove `CreateViewStatement,` from the macro call. Then add before the macro (around line 542):

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct CreateViewStatement {
    pub replace: bool,
    pub temporary: bool,
    pub recursive: bool,
    pub name: ObjectName,
    pub columns: Vec<String>,
    pub query: Box<SelectStatement>,
    pub check_option: Option<CheckOption>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CheckOption {
    Local,
    Cascaded,
}
```

**Step 2: Add CheckOption import to ddl.rs**

In `src/parser/ddl.rs`, add to imports (line 1-8):
```rust
use crate::ast::{
    AlterColumnAction, AlterTableAction, AlterTableStatement, CheckOption, ColumnConstraint, ColumnDef,
    CreateIndexStatement, CreateSequenceStatement, CreateTableStatement, CreateViewStatement,
    DataType, DropStatement, IndexColumn, ObjectType, TableConstraint, TimeZoneInfo, TruncateStatement,
};
```

**Step 3: Implement parse_create_view in ddl.rs**

Add to `src/parser/ddl.rs` after parse_create_sequence (around line 688):

```rust
    // ========== CREATE VIEW ==========

    pub(crate) fn parse_create_view(&mut self) -> Result<CreateViewStatement, ParserError> {
        self.expect_keyword(Keyword::VIEW)?;

        let replace = self.try_consume_keyword(Keyword::OR);
        if replace {
            self.expect_keyword(Keyword::REPLACE)?;
        }

        let temporary = if self.match_keyword(Keyword::TEMPORARY) || self.match_keyword(Keyword::TEMP) {
            self.advance();
            true
        } else {
            false
        };

        let recursive = self.try_consume_keyword(Keyword::RECURSIVE);

        let name = self.parse_object_name()?;

        // Optional column list
        let columns = if self.match_token(&Token::LParen) {
            self.advance();
            let mut cols = vec![self.parse_identifier()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                cols.push(self.parse_identifier()?);
            }
            self.expect_token(&Token::RParen)?;
            cols
        } else {
            vec![]
        };

        self.expect_keyword(Keyword::AS)?;
        let query = Box::new(self.parse_select_statement()?);

        // Optional WITH CHECK OPTION
        let check_option = if self.match_keyword(Keyword::WITH) {
            self.advance();
            if self.match_keyword(Keyword::LOCAL) {
                self.advance();
                self.expect_keyword(Keyword::CHECK)?;
                self.expect_keyword(Keyword::OPTION)?;
                Some(CheckOption::Local)
            } else if self.match_keyword(Keyword::CASCADED) {
                self.advance();
                self.expect_keyword(Keyword::CHECK)?;
                self.expect_keyword(Keyword::OPTION)?;
                Some(CheckOption::Cascaded)
            } else {
                self.expect_keyword(Keyword::CHECK)?;
                self.expect_keyword(Keyword::OPTION)?;
                Some(CheckOption::Cascaded) // Default
            }
        } else {
            None
        };

        Ok(CreateViewStatement {
            replace,
            temporary,
            recursive,
            name,
            columns,
            query,
            check_option,
        })
    }
```

**Step 4: Add VIEW to CREATE dispatcher**

In `src/parser/mod.rs`, in `dispatch_create` method (around line 302), add VIEW case:

```rust
Some(Keyword::VIEW) => match self.parse_create_view() {
    Ok(stmt) => crate::ast::Statement::CreateView(stmt),
    Err(_) => self.skip_to_semicolon(),
},
```

**Step 5: Verify build**

Run: `cargo build`
Expected: Compiles successfully

**Step 6: Commit**

```bash
git add src/ast/mod.rs src/parser/ddl.rs src/parser/mod.rs
git commit -m "feat: implement CREATE VIEW parsing with OR REPLACE, TEMP, RECURSIVE, CHECK OPTION"
```

---

## Task 3: Implement CREATE SCHEMA Statement

**Files:**
- Modify: `src/ast/mod.rs:543` - Replace stub
- Modify: `src/parser/ddl.rs` - Add parse_create_schema
- Modify: `src/parser/mod.rs` - Add CREATE SCHEMA dispatcher

**Step 1: Replace CreateSchemaStatement stub**

Remove `CreateSchemaStatement,` from stub_struct! macro (line 543). Add before macro:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct CreateSchemaStatement {
    pub if_not_exists: bool,
    pub name: Option<String>,
    pub authorization: Option<String>,
    pub elements: Vec<SchemaElement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SchemaElement {
    Table(CreateTableStatement),
    Index(CreateIndexStatement),
    View(CreateViewStatement),
    Sequence(CreateSequenceStatement),
}
```

**Step 2: Update ddl.rs imports**

Add to imports in `src/parser/ddl.rs`:
```rust
use crate::ast::{
    AlterColumnAction, AlterTableAction, AlterTableStatement, CheckOption, ColumnConstraint, ColumnDef,
    CreateIndexStatement, CreateSchemaStatement, CreateSequenceStatement, CreateTableStatement, CreateViewStatement,
    DataType, DropStatement, IndexColumn, ObjectType, SchemaElement, TableConstraint, TimeZoneInfo, TruncateStatement,
};
```

**Step 3: Implement parse_create_schema in ddl.rs**

Add after parse_create_view:

```rust
    // ========== CREATE SCHEMA ==========

    pub(crate) fn parse_create_schema(&mut self) -> Result<CreateSchemaStatement, ParserError> {
        self.expect_keyword(Keyword::SCHEMA)?;

        let if_not_exists = self.parse_if_not_exists();

        // Schema name OR AUTHORIZATION role
        let (name, authorization) = if self.match_keyword(Keyword::AUTHORIZATION) {
            self.advance();
            let auth = Some(self.parse_identifier()?);
            (None, auth)
        } else {
            let schema_name = Some(self.parse_identifier()?);
            let auth = if self.match_keyword(Keyword::AUTHORIZATION) {
                self.advance();
                Some(self.parse_identifier()?)
            } else {
                None
            };
            (schema_name, auth)
        };

        // Optional schema elements
        let mut elements = Vec::new();
        if self.match_token(&Token::LParen) || self.match_keyword(Keyword::CREATE) {
            // Parse schema contents
            loop {
                if self.match_keyword(Keyword::CREATE) {
                    self.advance();
                    let element = match self.peek_keyword() {
                        Some(Keyword::TABLE) => SchemaElement::Table(self.parse_create_table()?),
                        Some(Keyword::INDEX) => SchemaElement::Index(self.parse_create_index()?),
                        Some(Keyword::VIEW) => SchemaElement::View(self.parse_create_view()?),
                        Some(Keyword::SEQUENCE) => SchemaElement::Sequence(self.parse_create_sequence()?),
                        _ => break,
                    };
                    elements.push(element);
                } else {
                    break;
                }
            }
        }

        Ok(CreateSchemaStatement {
            if_not_exists,
            name,
            authorization,
            elements,
        })
    }
```

**Step 4: Add SCHEMA to CREATE dispatcher**

In `src/parser/mod.rs` dispatch_create:

```rust
Some(Keyword::SCHEMA) => match self.parse_create_schema() {
    Ok(stmt) => crate::ast::Statement::CreateSchema(stmt),
    Err(_) => self.skip_to_semicolon(),
},
```

**Step 5: Verify build and commit**

Run: `cargo build`

```bash
git add src/ast/mod.rs src/parser/ddl.rs src/parser/mod.rs
git commit -m "feat: implement CREATE SCHEMA with AUTHORIZATION and nested elements"
```

---

## Task 4: Implement CREATE DATABASE Statement

**Files:**
- Modify: `src/ast/mod.rs:544` - Replace stub
- Modify: `src/parser/ddl.rs` - Add parse_create_database
- Modify: `src/parser/mod.rs` - Add dispatcher

**Step 1: Replace CreateDatabaseStatement stub**

Remove from stub_struct! macro. Add definition:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct CreateDatabaseStatement {
    pub name: String,
    pub owner: Option<String>,
    pub template: Option<String>,
    pub encoding: Option<String>,
    pub locale: Option<String>,
    pub lc_collate: Option<String>,
    pub lc_ctype: Option<String>,
    pub tablespace: Option<String>,
    pub allow_connections: Option<bool>,
    pub connection_limit: Option<i32>,
    pub is_template: Option<bool>,
}
```

**Step 2: Update imports**

Add `CreateDatabaseStatement` to ddl.rs imports.

**Step 3: Implement parse_create_database**

```rust
    // ========== CREATE DATABASE ==========

    pub(crate) fn parse_create_database(&mut self) -> Result<CreateDatabaseStatement, ParserError> {
        self.expect_keyword(Keyword::DATABASE)?;

        let name = self.parse_identifier()?;

        let mut owner = None;
        let mut template = None;
        let mut encoding = None;
        let mut locale = None;
        let mut lc_collate = None;
        let mut lc_ctype = None;
        let mut tablespace = None;
        let mut allow_connections = None;
        let mut connection_limit = None;
        let mut is_template = None;

        // Parse WITH options
        if self.match_keyword(Keyword::WITH) || self.match_token(&Token::LParen) {
            if self.match_keyword(Keyword::WITH) {
                self.advance();
            }

            let has_parens = self.match_token(&Token::LParen);
            if has_parens {
                self.advance();
            }

            loop {
                match self.peek_keyword() {
                    Some(Keyword::OWNER) => {
                        self.advance();
                        self.expect_token(&Token::Eq)?;
                        owner = Some(self.parse_identifier()?);
                    }
                    Some(Keyword::TEMPLATE) => {
                        self.advance();
                        self.expect_token(&Token::Eq)?;
                        template = Some(self.parse_identifier()?);
                    }
                    Some(Keyword::ENCODING) => {
                        self.advance();
                        self.expect_token(&Token::Eq)?;
                        encoding = Some(self.parse_identifier()?);
                    }
                    Some(Keyword::LOCALE) => {
                        self.advance();
                        self.expect_token(&Token::Eq)?;
                        locale = Some(self.parse_identifier()?);
                    }
                    Some(Keyword::LC_COLLATE_P) => {
                        self.advance();
                        self.expect_token(&Token::Eq)?;
                        lc_collate = Some(self.parse_identifier()?);
                    }
                    Some(Keyword::LC_CTYPE_P) => {
                        self.advance();
                        self.expect_token(&Token::Eq)?;
                        lc_ctype = Some(self.parse_identifier()?);
                    }
                    Some(Keyword::TABLESPACE) => {
                        self.advance();
                        self.expect_token(&Token::Eq)?;
                        tablespace = Some(self.parse_identifier()?);
                    }
                    Some(Keyword::ALLOW_CONNECTIONS) => {
                        self.advance();
                        self.expect_token(&Token::Eq)?;
                        allow_connections = Some(self.match_keyword(Keyword::TRUE_P));
                        if !allow_connections.unwrap() {
                            self.expect_keyword(Keyword::FALSE_P)?;
                        }
                    }
                    Some(Keyword::CONNECTION_LIMIT) => {
                        self.advance();
                        self.expect_token(&Token::Eq)?;
                        if let Token::Integer(n) = self.peek() {
                            connection_limit = Some(*n as i32);
                            self.advance();
                        } else {
                            return Err(ParserError::UnexpectedToken {
                                position: self.pos,
                                expected: "integer".to_string(),
                                got: format!("{:?}", self.peek()),
                            });
                        }
                    }
                    Some(Keyword::IS_TEMPLATE) => {
                        self.advance();
                        self.expect_token(&Token::Eq)?;
                        is_template = Some(self.match_keyword(Keyword::TRUE_P));
                        if !is_template.unwrap() {
                            self.expect_keyword(Keyword::FALSE_P)?;
                        }
                    }
                    _ => break,
                }

                if self.match_token(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }

            if has_parens {
                self.expect_token(&Token::RParen)?;
            }
        }

        Ok(CreateDatabaseStatement {
            name,
            owner,
            template,
            encoding,
            locale,
            lc_collate,
            lc_ctype,
            tablespace,
            allow_connections,
            connection_limit,
            is_template,
        })
    }
```

**Step 4: Add to dispatcher**

```rust
Some(Keyword::DATABASE) => match self.parse_create_database() {
    Ok(stmt) => crate::ast::Statement::CreateDatabase(stmt),
    Err(_) => self.skip_to_semicolon(),
},
```

**Step 5: Commit**

```bash
git add src/ast/mod.rs src/parser/ddl.rs src/parser/mod.rs
git commit -m "feat: implement CREATE DATABASE with all PostgreSQL options"
```

---

## Task 5: Implement CREATE TABLESPACE Statement

**Files:**
- Modify: `src/ast/mod.rs:545` - Replace stub
- Modify: `src/parser/ddl.rs` - Add parse_create_tablespace
- Modify: `src/parser/mod.rs` - Add dispatcher

**Step 1: Replace CreateTablespaceStatement stub**

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct CreateTablespaceStatement {
    pub name: String,
    pub owner: Option<String>,
    pub location: String,
}
```

**Step 2: Update imports**

Add `CreateTablespaceStatement` to ddl.rs imports.

**Step 3: Implement parse_create_tablespace**

```rust
    // ========== CREATE TABLESPACE ==========

    pub(crate) fn parse_create_tablespace(&mut self) -> Result<CreateTablespaceStatement, ParserError> {
        self.expect_keyword(Keyword::TABLESPACE)?;

        let name = self.parse_identifier()?;

        let owner = if self.match_keyword(Keyword::OWNER) {
            self.advance();
            Some(self.parse_identifier()?)
        } else {
            None
        };

        self.expect_keyword(Keyword::LOCATION)?;
        let location = match self.peek() {
            Token::StringLiteral(s) | Token::DollarString(s) => {
                let loc = s.clone();
                self.advance();
                loc
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    position: self.pos,
                    expected: "string literal for location".to_string(),
                    got: format!("{:?}", self.peek()),
                });
            }
        };

        Ok(CreateTablespaceStatement {
            name,
            owner,
            location,
        })
    }
```

**Step 4: Add to dispatcher**

```rust
Some(Keyword::TABLESPACE) => match self.parse_create_tablespace() {
    Ok(stmt) => crate::ast::Statement::CreateTablespace(stmt),
    Err(_) => self.skip_to_semicolon(),
},
```

**Step 5: Commit**

```bash
git add src/ast/mod.rs src/parser/ddl.rs src/parser/mod.rs
git commit -m "feat: implement CREATE TABLESPACE parsing"
```

---

## Task 6: Add Parser Unit Tests

**Files:**
- Create: `src/parser/tests.rs`
- Modify: `src/parser/mod.rs` - Add test module

**Step 1: Create test file**

Create `src/parser/tests.rs`:

```rust
//! Parser unit tests

use crate::ast::*;
use crate::parser::Parser;
use crate::token::tokenizer::Tokenizer;

fn parse(sql: &str) -> Vec<Statement> {
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    Parser::new(tokens).parse().unwrap()
}

fn parse_one(sql: &str) -> Statement {
    parse(sql).into_iter().next().expect("Expected at least one statement")
}

// ========== TRUNCATE Tests ==========

#[test]
fn test_truncate_single_table() {
    let stmt = parse_one("TRUNCATE TABLE users");
    match stmt {
        Statement::Truncate(t) => {
            assert_eq!(t.tables, vec![vec!["users".to_string()]]);
            assert!(!t.cascade);
            assert!(!t.restart_identity);
        }
        _ => panic!("Expected TRUNCATE statement"),
    }
}

#[test]
fn test_truncate_multiple_tables() {
    let stmt = parse_one("TRUNCATE users, orders, items");
    match stmt {
        Statement::Truncate(t) => {
            assert_eq!(t.tables.len(), 3);
            assert_eq!(t.tables[0], vec!["users".to_string()]);
            assert_eq!(t.tables[1], vec!["orders".to_string()]);
            assert_eq!(t.tables[2], vec!["items".to_string()]);
        }
        _ => panic!("Expected TRUNCATE statement"),
    }
}

#[test]
fn test_truncate_cascade() {
    let stmt = parse_one("TRUNCATE users CASCADE");
    match stmt {
        Statement::Truncate(t) => {
            assert!(t.cascade);
        }
        _ => panic!("Expected TRUNCATE statement"),
    }
}

// ========== CREATE VIEW Tests ==========

#[test]
fn test_create_simple_view() {
    let stmt = parse_one("CREATE VIEW active_users AS SELECT * FROM users WHERE status = 'active'");
    match stmt {
        Statement::CreateView(v) => {
            assert!(!v.replace);
            assert!(!v.temporary);
            assert_eq!(v.name, vec!["active_users".to_string()]);
            assert!(v.columns.is_empty());
        }
        _ => panic!("Expected CREATE VIEW statement"),
    }
}

#[test]
fn test_create_or_replace_view() {
    let stmt = parse_one("CREATE OR REPLACE VIEW v AS SELECT 1");
    match stmt {
        Statement::CreateView(v) => {
            assert!(v.replace);
        }
        _ => panic!("Expected CREATE VIEW statement"),
    }
}

#[test]
fn test_create_temporary_view() {
    let stmt = parse_one("CREATE TEMPORARY VIEW temp_view AS SELECT * FROM t");
    match stmt {
        Statement::CreateView(v) => {
            assert!(v.temporary);
        }
        _ => panic!("Expected CREATE VIEW statement"),
    }
}

#[test]
fn test_create_view_with_columns() {
    let stmt = parse_one("CREATE VIEW v(a, b, c) AS SELECT x, y, z FROM t");
    match stmt {
        Statement::CreateView(v) => {
            assert_eq!(v.columns, vec!["a", "b", "c"]);
        }
        _ => panic!("Expected CREATE VIEW statement"),
    }
}

#[test]
fn test_create_view_with_check_option() {
    let stmt = parse_one("CREATE VIEW v AS SELECT * FROM t WITH CHECK OPTION");
    match stmt {
        Statement::CreateView(v) => {
            assert_eq!(v.check_option, Some(CheckOption::Cascaded));
        }
        _ => panic!("Expected CREATE VIEW statement"),
    }
}

#[test]
fn test_create_view_with_local_check_option() {
    let stmt = parse_one("CREATE VIEW v AS SELECT * FROM t WITH LOCAL CHECK OPTION");
    match stmt {
        Statement::CreateView(v) => {
            assert_eq!(v.check_option, Some(CheckOption::Local));
        }
        _ => panic!("Expected CREATE VIEW statement"),
    }
}

// ========== CREATE SCHEMA Tests ==========

#[test]
fn test_create_simple_schema() {
    let stmt = parse_one("CREATE SCHEMA myschema");
    match stmt {
        Statement::CreateSchema(s) => {
            assert_eq!(s.name, Some("myschema".to_string()));
            assert!(s.authorization.is_none());
            assert!(s.elements.is_empty());
        }
        _ => panic!("Expected CREATE SCHEMA statement"),
    }
}

#[test]
fn test_create_schema_if_not_exists() {
    let stmt = parse_one("CREATE SCHEMA IF NOT EXISTS myschema");
    match stmt {
        Statement::CreateSchema(s) => {
            assert!(s.if_not_exists);
        }
        _ => panic!("Expected CREATE SCHEMA statement"),
    }
}

#[test]
fn test_create_schema_authorization() {
    let stmt = parse_one("CREATE SCHEMA AUTHORIZATION admin");
    match stmt {
        Statement::CreateSchema(s) => {
            assert!(s.name.is_none());
            assert_eq!(s.authorization, Some("admin".to_string()));
        }
        _ => panic!("Expected CREATE SCHEMA statement"),
    }
}

#[test]
fn test_create_schema_with_authorization() {
    let stmt = parse_one("CREATE SCHEMA myschema AUTHORIZATION admin");
    match stmt {
        Statement::CreateSchema(s) => {
            assert_eq!(s.name, Some("myschema".to_string()));
            assert_eq!(s.authorization, Some("admin".to_string()));
        }
        _ => panic!("Expected CREATE SCHEMA statement"),
    }
}

// ========== CREATE DATABASE Tests ==========

#[test]
fn test_create_simple_database() {
    let stmt = parse_one("CREATE DATABASE mydb");
    match stmt {
        Statement::CreateDatabase(d) => {
            assert_eq!(d.name, "mydb");
            assert!(d.owner.is_none());
        }
        _ => panic!("Expected CREATE DATABASE statement"),
    }
}

#[test]
fn test_create_database_with_owner() {
    let stmt = parse_one("CREATE DATABASE mydb WITH OWNER = admin");
    match stmt {
        Statement::CreateDatabase(d) => {
            assert_eq!(d.owner, Some("admin".to_string()));
        }
        _ => panic!("Expected CREATE DATABASE statement"),
    }
}

#[test]
fn test_create_database_with_encoding() {
    let stmt = parse_one("CREATE DATABASE mydb ENCODING = 'UTF8'");
    match stmt {
        Statement::CreateDatabase(d) => {
            assert_eq!(d.encoding, Some("UTF8".to_string()));
        }
        _ => panic!("Expected CREATE DATABASE statement"),
    }
}

#[test]
fn test_create_database_with_multiple_options() {
    let stmt = parse_one("CREATE DATABASE mydb WITH OWNER = admin ENCODING = 'UTF8' TABLESPACE = pg_default");
    match stmt {
        Statement::CreateDatabase(d) => {
            assert_eq!(d.owner, Some("admin".to_string()));
            assert_eq!(d.encoding, Some("UTF8".to_string()));
            assert_eq!(d.tablespace, Some("pg_default".to_string()));
        }
        _ => panic!("Expected CREATE DATABASE statement"),
    }
}

// ========== CREATE TABLESPACE Tests ==========

#[test]
fn test_create_tablespace() {
    let stmt = parse_one("CREATE TABLESPACE myspace LOCATION '/data/myspace'");
    match stmt {
        Statement::CreateTablespace(t) => {
            assert_eq!(t.name, "myspace");
            assert_eq!(t.location, "/data/myspace");
            assert!(t.owner.is_none());
        }
        _ => panic!("Expected CREATE TABLESPACE statement"),
    }
}

#[test]
fn test_create_tablespace_with_owner() {
    let stmt = parse_one("CREATE TABLESPACE myspace OWNER admin LOCATION '/data/myspace'");
    match stmt {
        Statement::CreateTablespace(t) => {
            assert_eq!(t.name, "myspace");
            assert_eq!(t.owner, Some("admin".to_string()));
            assert_eq!(t.location, "/data/myspace");
        }
        _ => panic!("Expected CREATE TABLESPACE statement"),
    }
}
```

**Step 2: Add test module to mod.rs**

Add to `src/parser/mod.rs` at the end:

```rust
#[cfg(test)]
mod tests;
```

**Step 3: Run tests**

Run: `cargo test`
Expected: All new tests pass plus original 28 tests

**Step 4: Commit**

```bash
git add src/parser/tests.rs src/parser/mod.rs
git commit -m "test: add comprehensive parser unit tests for TRUNCATE, CREATE VIEW/SCHEMA/DATABASE/TABLESPACE"
```

---

## Task 7: Final Verification

**Files:** None (verification only)

**Step 1: Full test suite**

Run: `cargo test`
Expected: All tests pass (28 original + 20+ new)

**Step 2: Build check**

Run: `cargo build --release`
Expected: Clean build with only expected warnings

**Step 3: Regression test**

Run: `cargo run --example regression`
Expected: Same pass rate as before (1409/1409)

**Step 4: Documentation update**

Update `README.md` Phase 3 status to show progress.

**Step 5: Final commit**

```bash
git add README.md
git commit -m "docs: update Phase 3 progress in README"
```

---

## Summary

After completing this plan:

| Statement | Status | Tests |
|-----------|--------|-------|
| TRUNCATE | ✅ Wired up | ✅ 3 tests |
| CREATE VIEW | ✅ Full implementation | ✅ 6 tests |
| CREATE SCHEMA | ✅ Full implementation | ✅ 4 tests |
| CREATE DATABASE | ✅ Full implementation | ✅ 4 tests |
| CREATE TABLESPACE | ✅ Full implementation | ✅ 2 tests |

**Total New Tests:** ~20
**Files Modified:** 3 (ast/mod.rs, parser/ddl.rs, parser/mod.rs)
**Files Created:** 1 (parser/tests.rs)

**Estimated Time:** 6-8 hours for careful implementation
