# PACKAGE BODY Parsing Optimization Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix the ogsql parser to correctly parse Oracle/GaussDB PACKAGE and PACKAGE BODY statements, including nested procedures, functions, variable declarations, and inner DML.

**Architecture:** Three-phase approach: P0 fixes statement boundary detection so PACKAGE is treated as a single statement; P1 adds recognition of bare PROCEDURE/FUNCTION within package bodies; P2 redesigns the AST to hold structured sub-items and reuses the existing PL/pgSQL parser to recursively parse procedure bodies.

**Tech Stack:** Rust 2021, hand-written recursive descent parser, existing PL/pgSQL parser infrastructure in `src/parser/plpgsql.rs`.

---

## P0: Fix Statement Boundary Detection for PACKAGE

**Goal:** `find_statement_end_pos()` and `collect_until_end_boundary()` must correctly identify the full extent of `CREATE [OR REPLACE] PACKAGE [BODY] name AS|IS ... END name; /` as a single statement, regardless of internal semicolons and nested `BEGIN...END` blocks.

**Root Cause:** `find_statement_end_pos()` stops at the first semicolon at depth 0 (inside package spec declarations). `collect_until_end_boundary()` exits when BEGIN/END depth returns to 0 (at the end of the first inner procedure, not the package).

### Task P0-1: Write failing test for multi-procedure PACKAGE SPEC

**Files:**
- Modify: `src/parser/tests.rs` (after line 1155)

**Step 1: Write the failing test**

Add to `src/parser/tests.rs` after the `test_create_package_authid_definer` test:

```rust
#[test]
fn test_create_package_spec_multi_procs() {
    let sql = "CREATE OR REPLACE PACKAGE my_pkg IS\n\
               PROCEDURE proc1(i_date IN VARCHAR2, o_flag OUT VARCHAR2);\n\
               PROCEDURE proc2(i_date IN VARCHAR2);\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackage(p) => {
            assert_eq!(p.name, vec!["my_pkg"]);
            assert!(p.body.contains("proc1"));
            assert!(p.body.contains("proc2"));
        }
        _ => panic!("expected CreatePackage, got {:?}", stmt),
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_create_package_spec_multi_procs`
Expected: FAIL — body will only contain `proc1` (truncated at first semicolon)

### Task P0-2: Write failing test for PACKAGE BODY with nested procedures

**Files:**
- Modify: `src/parser/tests.rs` (after test from P0-1)

**Step 1: Write the failing test**

```rust
#[test]
fn test_create_package_body_multi_procedures() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS\n\
               PROCEDURE proc1(i_date IN VARCHAR2) IS\n\
                 v_x NUMBER;\n\
               BEGIN\n\
                 DELETE FROM t1 WHERE id = 1;\n\
               END proc1;\n\
               PROCEDURE proc2 IS\n\
               BEGIN\n\
                 INSERT INTO t2 VALUES(1);\n\
               END proc2;\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            assert_eq!(p.name, vec!["my_pkg"]);
            assert!(p.body.contains("proc1"));
            assert!(p.body.contains("proc2"));
            assert!(p.body.contains("DELETE FROM"));
            assert!(p.body.contains("INSERT INTO"));
        }
        _ => panic!("expected CreatePackageBody, got {:?}", stmt),
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_create_package_body_multi_procedures`
Expected: FAIL — body will be truncated after `END proc1;`

### Task P0-3: Fix `find_statement_end_pos()` to recognize PACKAGE boundaries

**Files:**
- Modify: `src/parser/mod.rs:192-226` (function `find_statement_end_pos`)

**Step 1: Add `is_creating_package` detection**

The function needs to know whether we're inside a `CREATE PACKAGE [BODY]` statement so it doesn't split at internal semicolons. Add a boolean flag that gets set when the token sequence `CREATE [OR REPLACE] PACKAGE [BODY]` is detected at the start, and only clears when `END <name>;` is found at depth 0.

Replace `find_statement_end_pos()` (lines 192-226) with:

```rust
fn find_statement_end_pos(&self) -> usize {
    let mut depth = 0i32;
    let mut begin_depth = 0i32;
    let mut in_package = false;
    let mut package_body = false;
    let mut saw_end_at_zero = false;

    for i in self.pos..self.tokens.len() {
        match &self.tokens[i].token {
            Token::Eof => return if i > 0 { i - 1 } else { 0 },
            Token::LParen => depth += 1,
            Token::RParen => depth = (depth - 1).max(0),
            Token::DollarString(_) => {}
            Token::Keyword(Keyword::BEGIN_P) => begin_depth += 1,
            Token::Keyword(Keyword::END_P) if begin_depth > 0 => {
                let next_is_compound = (i + 1) < self.tokens.len()
                    && matches!(
                        self.tokens[i + 1].token,
                        Token::Keyword(Keyword::LOOP)
                            | Token::Keyword(Keyword::IF_P)
                            | Token::Keyword(Keyword::CASE)
                    );
                if !next_is_compound {
                    begin_depth -= 1;
                }
                // In package body: the outer END (begin_depth==0 after decrement) ends the package
                if in_package && package_body && begin_depth == 0 {
                    // consume trailing identifier (package name)
                    let mut j = i + 1;
                    while j < self.tokens.len()
                        && matches!(self.tokens[j].token, Token::Ident(_) | Token::Keyword(_))
                    {
                        // Only consume one identifier (the package name)
                        break;
                    }
                    // Look for semicolon or slash after END name
                    let mut end_pos = i;
                    for k in (i + 1)..self.tokens.len() {
                        match &self.tokens[k].token {
                            Token::Ident(_) | Token::Keyword(_) => {
                                end_pos = k;
                            }
                            Token::Semicolon => {
                                // END name ; pattern
                                if let Some(slash_pos) = self.find_slash_after(k) {
                                    return slash_pos;
                                }
                                return k;
                            }
                            Token::Slash => return k,
                            _ => break,
                        }
                    }
                    return end_pos;
                }
            }
            Token::Keyword(Keyword::PACKAGE) => {
                // Check if this is CREATE PACKAGE [BODY]
                if i > 0 {
                    if let Token::Keyword(Keyword::CREATE) = &self.tokens[i - 1].token {
                        in_package = true;
                    }
                }
            }
            Token::Keyword(Keyword::BODY_P) => {
                // Check if this is PACKAGE BODY
                if in_package && i > 0 {
                    if let Token::Keyword(Keyword::PACKAGE) = &self.tokens[i - 1].token {
                        package_body = true;
                    }
                }
            }
            Token::Semicolon if depth <= 0 && begin_depth <= 0 => {
                // PACKAGE SPEC: semicolons inside are NOT statement terminators
                if in_package && !package_body {
                    continue; // skip internal semicolons in package spec
                }
                // Oracle-style: END; / pattern — skip the ; and look for /
                if let Some(slash_pos) = self.find_slash_after(i) {
                    return slash_pos;
                }
                return i;
            }
            Token::Slash if depth <= 0 && begin_depth <= 0 => return i,
            _ => {}
        }
    }
    self.tokens.len().saturating_sub(1)
}
```

**Step 2: Run P0-1 test**

Run: `cargo test test_create_package_spec_multi_procs`
Expected: PASS — package spec now captures all procedure declarations

**Step 3: Run existing tests**

Run: `cargo test`
Expected: ALL existing tests still pass

### Task P0-4: Fix `collect_until_end_boundary()` for nested procedures

**Files:**
- Modify: `src/parser/utility.rs:1133-1204` (function `collect_until_end_boundary`)

**Step 1: Add procedure/function depth tracking**

The key insight: Oracle nested blocks use `PROCEDURE/FUNCTION name IS|AS ... BEGIN ... END name;` which forms a complete scope. We need to track `proc_depth` separately from `begin_depth`.

Replace `collect_until_end_boundary()` (lines 1133-1204) with:

```rust
fn collect_until_end_boundary(&mut self) -> String {
    let mut collected = String::new();
    let mut depth = 0i32;
    let mut proc_depth = 0i32; // tracks PROCEDURE/FUNCTION ... IS/AS ... END nesting

    loop {
        match self.peek() {
            Token::Eof => break,
            Token::LParen => {
                depth += 1;
                if !collected.is_empty() {
                    collected.push(' ');
                }
                collected.push_str(&self.token_to_string());
                self.advance();
            }
            Token::RParen => {
                depth = (depth - 1).max(0);
                if !collected.is_empty() {
                    collected.push(' ');
                }
                collected.push_str(&self.token_to_string());
                self.advance();
            }
            Token::Keyword(Keyword::BEGIN_P) => {
                if !collected.is_empty() {
                    collected.push(' ');
                }
                collected.push_str(&self.token_to_string());
                self.advance();
            }
            Token::Keyword(Keyword::END_P) => {
                // Check if next_is_compound (END IF, END LOOP, END CASE)
                let next_is_compound = self.lookahead_is_compound_end();

                if proc_depth > 0 {
                    // We're inside a nested PROCEDURE/FUNCTION
                    // This END closes it — but we need to check if it's
                    // closing a BEGIN block or the entire procedure
                    // Consume the END and any trailing name
                    if !collected.is_empty() {
                        collected.push(' ');
                    }
                    collected.push_str(&self.token_to_string());
                    self.advance();
                    if !next_is_compound {
                        // consume trailing procedure/function name identifier
                        loop {
                            match self.peek() {
                                Token::Ident(_) => {
                                    if !collected.is_empty() {
                                        collected.push(' ');
                                    }
                                    collected.push_str(&self.token_to_string());
                                    self.advance();
                                }
                                _ => break,
                            }
                        }
                        // consume optional semicolon
                        if self.match_token(&Token::Semicolon) {
                            collected.push(';');
                            self.advance();
                        }
                        proc_depth -= 1;
                    } else {
                        self.try_consume_ident_str("IF");
                    }
                    continue;
                }

                // proc_depth == 0: this is the outermost END (closes the package)
                self.advance();
                if !next_is_compound {
                    self.try_consume_ident_str("IF");
                }
                loop {
                    match self.peek() {
                        Token::Ident(_) => self.advance(),
                        _ => break,
                    }
                }
                break;
            }
            // Detect PROCEDURE or FUNCTION keyword that starts a nested subprogram
            Token::Keyword(Keyword::PROCEDURE) | Token::Keyword(Keyword::FUNCTION) => {
                if !collected.is_empty() {
                    collected.push(' ');
                }
                collected.push_str(&self.token_to_string());
                self.advance();
                // Check if this is a definition (has IS/AS) vs a declaration (ends with ;)
                // Peek ahead to see if we find IS/AS before a semicolon
                if self.looks_like_subprogram_definition() {
                    proc_depth += 1;
                }
            }
            Token::Semicolon if depth == 0 && proc_depth == 0 => {
                self.advance();
                break;
            }
            Token::Slash if depth == 0 && proc_depth == 0 => {
                self.advance();
                break;
            }
            _ => {
                if !collected.is_empty() {
                    collected.push(' ');
                }
                collected.push_str(&self.token_to_string());
                self.advance();
            }
        }
    }

    collected.trim().to_string()
}
```

**Step 2: Add helper `looks_like_subprogram_definition()`**

Add this helper method to the Parser impl (in `src/parser/utility.rs`, before `collect_until_end_boundary`):

```rust
/// Peek ahead from current position to check if we're looking at a subprogram
/// definition (has IS/AS body) vs a declaration (just parameters and semicolon).
/// Current position should be right after the PROCEDURE/FUNCTION keyword.
fn looks_like_subprogram_definition(&self) -> bool {
    let mut i = self.pos;
    let mut paren_depth = 0i32;
    while i < self.tokens.len() {
        match &self.tokens[i].token {
            Token::LParen => paren_depth += 1,
            Token::RParen => paren_depth = (paren_depth - 1).max(0),
            Token::Keyword(Keyword::IS) | Token::Keyword(Keyword::AS)
                if paren_depth == 0 =>
            {
                return true;
            }
            Token::Semicolon if paren_depth == 0 => return false,
            Token::Eof => return false,
            _ => {}
        }
        i += 1;
    }
    false
}
```

**Step 3: Run P0-2 test**

Run: `cargo test test_create_package_body_multi_procedures`
Expected: PASS — body now contains both procedures and their inner DML

**Step 4: Run ALL tests**

Run: `cargo test`
Expected: ALL tests pass, including the two new ones

**Step 5: Verify with b.sql**

Run: `cargo build && ./target/debug/ogsql parse -f /Users/c2j/Projects/Desktop_Projects/DB/SP-Complexity-Evaluator/sql_samples/gauss/b.sql`
Expected:
- `CREATE PACKAGE PKG_FACC_DATAPROC` body contains all 3 procedure declarations
- `CREATE PACKAGE BODY PKG_FACC_DATAPROC` body contains all 3 procedure definitions with full BEGIN...END blocks
- No more `Empty` statements for procedure definitions that are inside the package body
- The 3 errors about "expected BEGIN_P, got Ident(v_step_no)" should be gone

**Step 6: Commit**

```bash
git add src/parser/mod.rs src/parser/utility.rs src/parser/tests.rs
git commit -m "fix: correct PACKAGE/PACKAGE BODY boundary detection for nested procedures"
```

---

## P1: Add Bare PROCEDURE/FUNCTION Recognition

**Goal:** When PACKAGE BODY is properly captured (P0 complete), the tokens inside the body string are not re-parsed. However, bare `PROCEDURE`/`FUNCTION` tokens at the top level of `parse_statement()` (outside any recognized statement) currently fall into the catch-all branch and produce `Statement::Empty`. This P1 ensures that top-level bare procedure/function definitions within package bodies are recognized — but more importantly, it also addresses the case where individual procedure definitions leak out of the package body collection.

**Note:** After P0, the package body is collected as a single opaque string, so inner procedures don't leak. P1 is about future-proofing: if a bare `PROCEDURE`/`FUNCTION` appears outside a package (e.g., standalone Oracle procedure without CREATE keyword), it should be gracefully handled rather than producing `Empty`.

### Task P1-1: Write failing test for bare procedure

**Files:**
- Modify: `src/parser/tests.rs`

**Step 1: Write the test**

```rust
#[test]
fn test_bare_procedure_definition() {
    // Oracle-style bare procedure (no CREATE keyword)
    let sql = "PROCEDURE my_proc(i_date IN VARCHAR2) IS\n\
               v_x NUMBER;\n\
               BEGIN\n\
                 DELETE FROM t1 WHERE id = 1;\n\
               END my_proc;";
    let result = parse_one_with_errors(sql);
    // Should not be Empty — should be recognized as some form of procedure
    assert!(
        !matches!(result.0.statement, Statement::Empty),
        "bare PROCEDURE should not parse as Empty"
    );
}
```

Add helper to tests.rs if not present:

```rust
fn parse_one_with_errors(input: &str) -> (Statement, Vec<ParserError>) {
    match Parser::parse_one(input) {
        Ok(pair) => pair,
        Err(e) => panic!("parse failed: {:?}", e),
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_bare_procedure_definition`
Expected: FAIL — bare PROCEDURE falls into catch-all → `Empty`

### Task P1-2: Add PROCEDURE/FUNCTION dispatch in parse_statement()

**Files:**
- Modify: `src/parser/mod.rs` in `parse_statement()` (around line 611 where BEGIN_P is handled)

**Step 1: Add PROCEDURE and FUNCTION handlers**

In `parse_statement()` (around line 611-636), add BEFORE the `BEGIN_P` handler:

```rust
Token::Keyword(Keyword::PROCEDURE) => {
    self.advance();
    match self.parse_create_procedure() {
        Ok(stmt) => {
            self.try_consume_semicolon();
            crate::ast::Statement::CreateProcedure(stmt)
        }
        Err(e) => {
            self.add_error(e);
            self.skip_to_semicolon()
        }
    }
}
Token::Keyword(Keyword::FUNCTION) => {
    self.advance();
    match self.parse_create_function() {
        Ok(stmt) => {
            self.try_consume_semicolon();
            crate::ast::Statement::CreateFunction(stmt)
        }
        Err(e) => {
            self.add_error(e);
            self.skip_to_semicolon()
        }
    }
}
```

**Step 2: Run test**

Run: `cargo test test_bare_procedure_definition`
Expected: PASS — bare PROCEDURE now recognized as `CreateProcedure`

**Step 3: Run all tests**

Run: `cargo test`
Expected: ALL tests pass

**Step 4: Commit**

```bash
git add src/parser/mod.rs src/parser/tests.rs
git commit -m "feat: recognize bare PROCEDURE/FUNCTION keywords in statement dispatch"
```

---

## P2: Structured AST + Recursive PL/SQL Parsing for PACKAGE BODY

**Goal:** Replace the opaque `body: String` in `CreatePackageBodyStatement` with structured `items: Vec<PackageItem>` where each procedure/function body is parsed using the existing PL/pgSQL parser (`parse_pl_block()`). This enables downstream tools to analyze individual DML statements within stored procedures.

### Task P2-1: Design and implement new AST types

**Files:**
- Modify: `src/ast/mod.rs`

**Step 1: Add PackageItem enum and related types**

Add after `CreatePackageBodyStatement` (around line 1001):

```rust
/// An item within a PACKAGE or PACKAGE BODY.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum PackageItem {
    /// Procedure declaration (spec) or definition (body)
    Procedure(PackageProcedure),
    /// Function declaration (spec) or definition (body)
    Function(PackageFunction),
    /// Variable declaration within package body
    Variable(PackageVariableDecl),
    /// Type declaration
    Type(PackageTypeDecl),
    /// Cursor declaration
    Cursor(PackageCursorDecl),
    /// Unparsed item (for items not yet fully supported)
    Raw(String),
}

/// Procedure within a package.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct PackageProcedure {
    pub name: ObjectName,
    pub parameters: Vec<FunctionParameter>,
    pub block: Option<PlBlock>, // None for declarations (spec), Some for definitions (body)
}

/// Function within a package.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct PackageFunction {
    pub name: ObjectName,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Option<String>,
    pub block: Option<PlBlock>, // None for declarations (spec), Some for definitions (body)
}

/// Variable declaration within package.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct PackageVariableDecl {
    pub name: String,
    pub data_type: String,
    pub default_value: Option<String>,
}

/// Type declaration within package.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct PackageTypeDecl {
    pub name: String,
    pub definition: String,
}

/// Cursor declaration within package.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct PackageCursorDecl {
    pub name: String,
    pub definition: String,
}
```

**Step 2: Update CreatePackageBodyStatement**

Change `body: String` to include both structured items and raw text:

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct CreatePackageBodyStatement {
    pub replace: bool,
    pub name: ObjectName,
    pub items: Vec<PackageItem>,
    /// Keep the raw body text for backward compatibility and fallback
    pub body: String,
}
```

Similarly update `CreatePackageStatement`:

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct CreatePackageStatement {
    pub replace: bool,
    pub name: ObjectName,
    pub authid: Option<PackageAuthid>,
    pub items: Vec<PackageItem>,
    pub body: String,
}
```

**Step 3: Fix compilation errors**

All places that construct `CreatePackageStatement` and `CreatePackageBodyStatement` will need updating. Set `items: vec![]` initially and keep `body` as before. This is a non-breaking intermediate step.

### Task P2-2: Implement `parse_package_body_items()`

**Files:**
- Modify: `src/parser/utility.rs`

**Step 1: Add `parse_package_body_items()` method**

This new method replaces `collect_until_end_boundary()` for the structured path. It iterates over tokens inside the package body, recognizing sub-programs and parsing their bodies using `parse_pl_block()`.

```rust
/// Parse items within a PACKAGE BODY until the final END package_name.
/// Returns (items, raw_body_text) for backward compatibility.
pub(crate) fn parse_package_body_items(&mut self) -> (Vec<PackageItem>, String) {
    let mut items = Vec::new();
    let mut raw_body = String::new();

    loop {
        match self.peek() {
            Token::Eof => break,
            Token::Keyword(Keyword::END_P) => {
                // Final END of the package
                self.advance();
                // consume package name
                loop {
                    match self.peek() {
                        Token::Ident(_) | Token::Keyword(_) => self.advance(),
                        _ => break,
                    }
                }
                break;
            }
            Token::Keyword(Keyword::PROCEDURE) => {
                let start_pos = self.pos;
                match self.parse_package_procedure() {
                    Ok(proc) => {
                        items.push(PackageItem::Procedure(proc));
                    }
                    Err(_) => {
                        // Fallback: collect as raw text
                        self.pos = start_pos;
                        let raw = self.collect_one_package_item();
                        items.push(PackageItem::Raw(raw.clone()));
                        raw_body.push_str(&raw);
                        raw_body.push(' ');
                    }
                }
            }
            Token::Keyword(Keyword::FUNCTION) => {
                let start_pos = self.pos;
                match self.parse_package_function() {
                    Ok(func) => {
                        items.push(PackageItem::Function(func));
                    }
                    Err(_) => {
                        self.pos = start_pos;
                        let raw = self.collect_one_package_item();
                        items.push(PackageItem::Raw(raw.clone()));
                        raw_body.push_str(&raw);
                        raw_body.push(' ');
                    }
                }
            }
            _ => {
                // Skip comments, whitespace tokens, etc.
                // Collect as raw item (variable, type, cursor declarations)
                let token_str = self.token_to_string();
                if !raw_body.is_empty() {
                    raw_body.push(' ');
                }
                raw_body.push_str(&token_str);
                self.advance();
            }
        }
    }

    (items, raw_body.trim().to_string())
}
```

**Step 2: Implement `parse_package_procedure()`**

```rust
fn parse_package_procedure(&mut self) -> Result<PackageProcedure, ParserError> {
    self.expect_keyword(Keyword::PROCEDURE)?; // consume PROCEDURE
    let name = self.parse_object_name()?;

    let mut parameters = Vec::new();
    if self.match_token(&Token::LParen) {
        self.advance();
        if !self.match_token(&Token::RParen) {
            loop {
                let param = self.parse_function_parameter()?;
                parameters.push(param);
                if self.match_token(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect_token(&Token::RParen)?;
    }

    // Check if this is a definition (has IS/AS) or just a declaration (semicolons)
    let has_body = if self.match_keyword(Keyword::IS) || self.match_keyword(Keyword::AS) {
        self.advance();
        true
    } else {
        self.try_consume_semicolon();
        false
    };

    let block = if has_body {
        // Parse the procedure body as a PL/pgSQL block
        // The block has: [declarations] BEGIN ... END [name];
        Some(self.parse_procedure_body()?)
    } else {
        None
    };

    Ok(PackageProcedure {
        name,
        parameters,
        block,
    })
}
```

**Step 3: Implement `parse_procedure_body()`**

This reuses the existing PL/pgSQL block parsing infrastructure:

```rust
/// Parse a procedure/function body: [declarations] BEGIN ... END [name];
/// This is Oracle-style where declarations come before BEGIN without a DECLARE keyword.
fn parse_procedure_body(&mut self) -> Result<PlBlock, ParserError> {
    // Parse variable declarations until BEGIN
    let mut declarations = Vec::new();
    while !self.match_keyword(Keyword::BEGIN_P) && !matches!(self.peek(), Token::Eof) {
        // Parse Oracle-style variable declaration: name type [:= default];
        match self.parse_oracle_var_decl() {
            Some(decl) => declarations.push(decl),
            None => {
                // Skip unrecognized tokens
                self.advance();
            }
        }
    }

    self.expect_keyword(Keyword::BEGIN_P)?;

    // Reuse existing PL/pgSQL statement parsing
    let mut body = Vec::new();
    let mut exception_block = None;

    loop {
        if self.match_ident_str("exception") {
            self.advance();
            exception_block = Some(self.parse_pl_exception_block()?);
        } else if self.peek_keyword() == Some(Keyword::END_P) {
            self.advance();
            // consume trailing procedure name
            loop {
                match self.peek() {
                    Token::Ident(_) => {
                        self.advance();
                    }
                    _ => break,
                }
            }
            self.try_consume_semicolon();
            break;
        } else if matches!(self.peek(), Token::Eof) {
            break;
        } else {
            let stmt = self.parse_pl_statement()?;
            body.push(stmt);
        }
    }

    Ok(PlBlock {
        label: None,
        declarations,
        body,
        exception_block,
        end_label: None,
    })
}
```

**Step 4: Implement `parse_oracle_var_decl()` helper**

```rust
/// Try to parse an Oracle-style variable declaration.
/// Pattern: var_name type[%TYPE | %ROWTYPE] [:= expr | DEFAULT expr] ;
fn parse_oracle_var_decl(&mut self) -> Option<PlDeclaration> {
    // Must start with an identifier
    if !matches!(self.peek(), Token::Ident(_)) {
        return None;
    }
    let start_pos = self.pos;
    let name = match self.peek() {
        Token::Ident(s) => s.clone(),
        _ => return None,
    };
    self.advance();

    // If next is := or ; or IDENT (type), this is likely a declaration
    // Otherwise, backtrack
    let type_str = match self.peek() {
        Token::Ident(_) | Token::Keyword(_) => {
            // Collect the type name (could be qualified: schema.type)
            let mut ty = self.token_to_string();
            self.advance();
            while self.match_token(&Token::Dot) {
                ty.push('.');
                self.advance();
                ty.push_str(&self.token_to_string());
                self.advance();
            }
            // Check for %TYPE or %ROWTYPE
            // These come as separate tokens: % TYPE
            ty
        }
        _ => {
            self.pos = start_pos;
            return None;
        }
    };

    // Check for %TYPE / %ROWTYPE
    let data_type = if matches!(self.peek(), Token::Op(ref s) if s == "%") {
        self.advance(); // consume %
        let attr = self.parse_identifier().unwrap_or_default();
        if attr.eq_ignore_ascii_case("type") {
            // Simplified: store as TypeName
            PlDataType::TypeName(format!("{}.%TYPE", type_str))
        } else if attr.eq_ignore_ascii_case("rowtype") {
            PlDataType::PercentRowType(type_str)
        } else {
            PlDataType::TypeName(type_str)
        }
    } else {
        PlDataType::TypeName(type_str)
    };

    // Optional default value
    let default = if self.match_token(&Token::Op(":=".to_string()))
        || self.match_keyword(Keyword::DEFAULT)
    {
        self.advance();
        let val = self.skip_to_semicolon_or_keyword();
        Some(val)
    } else {
        None
    };

    self.try_consume_semicolon();

    Some(PlDeclaration::Variable(PlVarDecl {
        name,
        data_type,
        default,
        constant: false,
        not_null: false,
        collate: None,
    }))
}
```

### Task P2-3: Wire up structured parsing in `parse_create_package()`

**Files:**
- Modify: `src/parser/utility.rs` (function `parse_create_package`)

**Step 1: Update `parse_create_package()` to use structured parsing**

In the `parse_create_package()` function (around line 1115), change:

```rust
// OLD:
let body = self.collect_until_end_boundary();

if is_body {
    Ok(Statement::CreatePackageBody(CreatePackageBodyStatement {
        replace,
        name,
        body,
    }))
} else {
    Ok(Statement::CreatePackage(CreatePackageStatement {
        replace,
        name,
        authid,
        body,
    }))
}
```

To:

```rust
// NEW: use structured parsing for package body
let (items, body) = if is_body {
    self.parse_package_body_items()
} else {
    // Package spec: use existing string collection
    let raw = self.collect_until_end_boundary();
    (vec![], raw)
};

if is_body {
    Ok(Statement::CreatePackageBody(CreatePackageBodyStatement {
        replace,
        name,
        items,
        body,
    }))
} else {
    Ok(Statement::CreatePackage(CreatePackageStatement {
        replace,
        name,
        authid,
        items,
        body,
    }))
}
```

### Task P2-4: Update formatter for new AST fields

**Files:**
- Modify: `src/formatter.rs`

**Step 1: Update formatter to handle new `items` field**

Find the `CreatePackage` and `CreatePackageBody` handling in the formatter and add `items: vec![]` to the formatting output, or just ignore the new field if the formatter uses `..` spreading.

### Task P2-5: Write comprehensive tests for P2

**Files:**
- Modify: `src/parser/tests.rs`

**Step 1: Write tests for structured package body**

```rust
#[test]
fn test_package_body_structured_procedure() {
    let sql = "CREATE OR REPLACE PACKAGE BODY my_pkg IS\n\
               PROCEDURE proc1(i_date IN VARCHAR2) IS\n\
                 v_x NUMBER;\n\
               BEGIN\n\
                 DELETE FROM t1 WHERE id = 1;\n\
                 INSERT INTO t2 VALUES(1);\n\
               END proc1;\n\
               END my_pkg;";
    let stmt = parse_one(sql);
    match stmt {
        Statement::CreatePackageBody(p) => {
            assert_eq!(p.name, vec!["my_pkg"]);
            assert!(!p.items.is_empty(), "package body should have structured items");

            // Find the procedure
            let proc = p.items.iter().find_map(|item| match item {
                PackageItem::Procedure(p) => Some(p),
                _ => None,
            }).expect("should have a procedure");

            assert_eq!(proc.name, vec!["proc1"]);
            assert!(proc.block.is_some(), "procedure should have a body block");

            let block = proc.block.as_ref().unwrap();
            // Should have DML statements in the body
            assert!(!block.body.is_empty(), "procedure body should have statements");
        }
        _ => panic!("expected CreatePackageBody, got {:?}", stmt),
    }
}
```

**Step 2: Run all tests**

Run: `cargo test`
Expected: ALL tests pass

### Task P2-6: Final verification with b.sql

**Step 1: Build and test against b.sql**

Run: `cargo build && ./target/debug/ogsql parse -f /Users/c2j/Projects/Desktop_Projects/DB/SP-Complexity-Evaluator/sql_samples/gauss/b.sql -j`
Expected: JSON output showing structured PACKAGE BODY with individual procedures, each containing parsed DML statements.

**Step 2: Verify no regressions**

Run: `cargo test`
Run: `cargo run --example regression` (if regression test suite available)
Expected: All existing tests pass, regression test count unchanged.

**Step 3: Commit**

```bash
git add -A
git commit -m "feat: structured AST for PACKAGE BODY with recursive PL/SQL parsing"
```

---

## Dependency Graph

```
P0-1 ──→ P0-3 ──→ P0-4 ──→ P0-Verify ──→ P0-Commit
   └──→ P0-2 ──┘

P0-Commit ──→ P1-1 ──→ P1-2 ──→ P1-Commit

P1-Commit ──→ P2-1 ──→ P2-2 ──→ P2-3 ──→ P2-4 ──→ P2-5 ──→ P2-6 ──→ P2-Commit
```

Each P-level depends on the previous one. Tasks within a P-level are sequential (each builds on the prior).
