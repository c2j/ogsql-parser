# Phase 2: Core DML Parsing Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement expression parsing and full DML statement parsing (SELECT, INSERT, UPDATE, DELETE, MERGE) with proper AST construction, replacing all `skip_to_semicolon()` stubs for these statements.

**Architecture:** Pratt parsing (operator-precedence) for expressions, recursive descent for statement structure. Each parser module lives in its own file under `src/parser/`. The existing `Parser` struct is extended with helper methods. The existing AST types in `src/ast/mod.rs` are expanded with proper structs replacing stubs.

**Tech Stack:** Rust (edition 2021), thiserror for error handling. No external parser dependencies.

**Grammar Reference:** `lib/openGauss-server/src/common/backend/parser/gram.y` (35,325 lines). Key sections:
- Operator precedence: lines 1070-1155
- `a_expr` (expression): lines 28269-28828
- `b_expr` (restricted expression): lines 28829-28924
- `c_expr` (primary expression): lines 28925-29121
- `simple_select`: lines 25194-25250
- `select_no_parens`: lines 25018-25164
- `InsertStmt`: lines 24166-24332
- `UpdateStmt`: lines 24608-24659
- `DeleteStmt`: lines 24498-24545
- `MergeStmt`: lines 24783-24800
- `from_clause` / `table_ref`: lines 26001-26089
- `target_list` / `target_el`: lines 31297-31396
- `func_expr`: lines 29239+
- `sort_clause`: lines 25550-25581
- `with_clause`: lines 25270-25285
- `joined_table`: lines 26599+

**Validation:** After each task, run `cargo build` and `cargo test`. After all tasks, run the regression test: `cargo run --example regression` — all 1,397 SQL files must still pass.

---

### Task 1: Parser Infrastructure & Helper Methods

**Files:**
- Create: `src/parser/expr.rs` (empty placeholder with `use` imports)
- Create: `src/parser/select.rs` (empty placeholder with `use` imports)
- Create: `src/parser/dml.rs` (empty placeholder with `use` imports)
- Modify: `src/parser/mod.rs` — add module declarations and helper methods

**Goal:** Extend the `Parser` struct with the helper methods needed by all subsequent parsers. Create the new module files.

**Step 1: Create module files with imports**

Create `src/parser/expr.rs`:
```rust
use crate::ast::Expr;
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;
```

Create `src/parser/select.rs`:
```rust
use crate::ast::{SelectStatement, SelectTarget, TableRef, WithClause, OrderByItem, Expr, ObjectName};
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;
```

Create `src/parser/dml.rs`:
```rust
use crate::ast::{InsertStatement, UpdateStatement, DeleteStatement, MergeStatement, Expr, ObjectName};
use crate::parser::{Parser, ParserError};
use crate::token::keyword::Keyword;
use crate::token::Token;
```

**Step 2: Add module declarations to `src/parser/mod.rs`**

Add at the top of `mod.rs`:
```rust
mod expr;
mod select;
mod dml;
```

**Step 3: Add helper methods to `Parser` impl block in `mod.rs`**

Add these methods inside the existing `impl Parser` block:

```rust
/// Peek at the current token without advancing.
fn peek(&self) -> &Token {
    self.tokens
        .get(self.pos)
        .map(|t| &t.token)
        .unwrap_or(&Token::Eof)
}

/// Check if the current token matches the given token.
fn match_token(&self, expected: &Token) -> bool {
    self.peek() == expected
}

/// Consume the current token if it matches, return it. Otherwise return None.
fn try_consume(&mut self, expected: &Token) -> Option<Token> {
    if self.match_token(expected) {
        let tok = self.peek().clone();
        self.advance();
        Some(tok)
    } else {
        None
    }
}

/// Expect and consume a specific token, or error.
fn expect_token(&mut self, expected: &Token) -> Result<Token, ParserError> {
    if self.match_token(expected) {
        let tok = self.peek().clone();
        self.advance();
        Ok(tok)
    } else {
        Err(ParserError::UnexpectedToken {
            position: self.pos,
            expected: format!("{:?}", expected),
            got: format!("{:?}", self.peek()),
        })
    }
}

/// Expect and consume a keyword, or error.
/// (Already exists as expect_keyword, no need to add)

/// Check if current token is a keyword.
fn peek_keyword(&self) -> Option<Keyword> {
    if let Token::Keyword(kw) = self.peek() {
        Some(*kw)
    } else {
        None
    }
}

/// Consume an identifier (Ident or Keyword used as identifier).
/// Returns the identifier string.
fn parse_identifier(&mut self) -> Result<String, ParserError> {
    match self.peek().clone() {
        Token::Ident(s) => {
            self.advance();
            Ok(s)
        }
        Token::QuotedIdent(s) => {
            self.advance();
            Ok(s)
        }
        Token::Keyword(kw) => {
            self.advance();
            Ok(format!("{:?}", kw).trim_end_matches("_P").to_lowercase())
        }
        _ => Err(ParserError::UnexpectedToken {
            position: self.pos,
            expected: "identifier".to_string(),
            got: format!("{:?}", self.peek()),
        }),
    }
}

/// Parse a potentially qualified name: `name` or `schema.name` or `catalog.schema.name`.
/// Returns a Vec<String> (ObjectName).
fn parse_object_name(&mut self) -> Result<ObjectName, ParserError> {
    let mut name = vec![self.parse_identifier()?];
    while self.match_token(&Token::Dot) {
        self.advance();
        name.push(self.parse_identifier()?);
    }
    Ok(name)
}

/// Try to consume an optional alias: [AS] identifier
fn parse_optional_alias(&mut self) -> Result<Option<String>, ParserError> {
    // Check for AS keyword
    if self.match_keyword(Keyword::AS) {
        self.advance();
        Ok(Some(self.parse_identifier()?))
    } else {
        // Check if next token could be an alias (identifier-like)
        match self.peek() {
            Token::Ident(_) | Token::QuotedIdent(_) => {
                let alias = self.parse_identifier()?;
                Ok(Some(alias))
            }
            // Some keywords can be used as aliases
            Token::Keyword(kw) if is_alias_compatible_keyword(*kw) => {
                let alias = self.parse_identifier()?;
                Ok(Some(alias))
            }
            _ => Ok(None),
        }
    }
}
```

Also add a free function after the impl block:
```rust
/// Check if a keyword can be used as an alias without AS.
fn is_alias_compatible_keyword(_kw: Keyword) -> bool {
    // Conservative: most keywords cannot be aliases.
    // We'll expand this as needed during testing.
    false
}
```

**Step 4: Verify build**

Run: `cargo build`
Expected: compiles with no errors (warnings OK)

**Step 5: Commit**

```bash
git add src/parser/expr.rs src/parser/select.rs src/parser/dml.rs src/parser/mod.rs
git commit -m "feat: add parser infrastructure, helper methods, and module structure for Phase 2"
```

---

### Task 2: Expression Parser — Primary Expressions (c_expr)

**Files:**
- Modify: `src/parser/expr.rs`
- Modify: `src/parser/mod.rs` — make `parse_expr` available

**Goal:** Implement the lowest level of expression parsing: literals, column references, parameters, parenthesized expressions, function calls, CASE expressions, EXISTS/subqueries, type casts. This is the `c_expr` level in gram.y.

**Context:** The `Expr` enum in `src/ast/mod.rs` already defines:
```rust
pub enum Expr {
    Literal(Literal),
    ColumnRef(ObjectName),  // Vec<String>
    BinaryOp { left, op, right },
    UnaryOp { op, expr },
    FunctionCall { name, args, distinct },
    Case { operand, whens, else_expr },
    Between { expr, low, high, negated },
    InList { expr, list, negated },
    InSubquery { expr, subquery, negated },
    Exists(Box<SelectStatement>),
    Subquery(Box<SelectStatement>),
    IsNull { expr, negated },
    TypeCast { expr, type_name },
    Parameter(i32),
    Array(Vec<Expr>),
    Default,
}
```

**Step 1: Implement `parse_primary_expr` (c_expr equivalent)**

Add to `src/parser/expr.rs`:

```rust
impl Parser {
    /// Parse a primary expression (c_expr in gram.y).
    /// This is the atomic building block: literals, column refs, params, parens, function calls, etc.
    pub(crate) fn parse_primary_expr(&mut self) -> Result<Expr, ParserError> {
        match self.peek().clone() {
            // Integer literal
            Token::Integer(n) => {
                self.advance();
                Ok(Expr::Literal(Literal::Integer(n)))
            }
            // Float literal
            Token::Float(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::Float(s)))
            }
            // String literal
            Token::StringLiteral(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            // Escape string
            Token::EscapeString(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            // Bit string
            Token::BitString(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            // Hex string
            Token::HexString(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            // National string
            Token::NationalString(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            // Dollar-quoted string
            Token::DollarString(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            // Boolean TRUE/FALSE
            Token::Keyword(Keyword::TRUE_P) => {
                self.advance();
                Ok(Expr::Literal(Literal::Boolean(true)))
            }
            Token::Keyword(Keyword::FALSE_P) => {
                self.advance();
                Ok(Expr::Literal(Literal::Boolean(false)))
            }
            // NULL
            Token::Keyword(Keyword::NULL_P) => {
                self.advance();
                Ok(Expr::Literal(Literal::Null))
            }
            // DEFAULT
            Token::Keyword(Keyword::DEFAULT) => {
                self.advance();
                Ok(Expr::Default)
            }
            // Parameter ($1, $2, ...)
            Token::Param(n) => {
                self.advance();
                Ok(Expr::Parameter(n))
            }
            // EXISTS (subquery)
            Token::Keyword(Keyword::EXISTS) => {
                self.advance();
                self.expect_token(&Token::LParen)?;
                let subquery = self.parse_select_statement()?;
                self.expect_token(&Token::RParen)?;
                Ok(Expr::Exists(Box::new(subquery)))
            }
            // CASE expression
            Token::Keyword(Keyword::CASE) => {
                self.parse_case_expr()
            }
            // Parenthesized expression or subquery
            Token::LParen => {
                self.advance();
                // Could be a subquery: (SELECT ...)
                if self.match_keyword(Keyword::SELECT) || self.match_keyword(Keyword::WITH) {
                    let subquery = self.parse_select_statement()?;
                    self.expect_token(&Token::RParen)?;
                    return Ok(Expr::Subquery(Box::new(subquery)));
                }
                // Otherwise: (expr)
                let expr = self.parse_expr()?;
                self.expect_token(&Token::RParen)?;
                Ok(expr)
            }
            // ARRAY
            Token::Keyword(Keyword::ARRAY) => {
                self.advance();
                if self.match_token(&Token::LParen) {
                    // ARRAY(subquery) or ARRAY(expr, ...)
                    self.advance();
                    // Check for subquery
                    if self.match_keyword(Keyword::SELECT) || self.match_keyword(Keyword::WITH) {
                        let subquery = self.parse_select_statement()?;
                        self.expect_token(&Token::RParen)?;
                        // ARRAY(subquery) — represent as subquery for now
                        return Ok(Expr::Subquery(Box::new(subquery)));
                    }
                    // ARRAY(expr, ...)
                    let mut elems = vec![self.parse_expr()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        elems.push(self.parse_expr()?);
                    }
                    self.expect_token(&Token::RParen)?;
                    return Ok(Expr::Array(elems));
                }
                // ARRAY without parens — error or handle ARRAY[...] later
                Err(ParserError::UnexpectedToken {
                    position: self.pos,
                    expected: "'(' after ARRAY".to_string(),
                    got: format!("{:?}", self.peek()),
                })
            }
            // Identifier or keyword-as-identifier — could be column ref or function call
            Token::Ident(_) | Token::QuotedIdent(_) => {
                let name = self.parse_object_name()?;
                // Check for function call: name(...)
                if self.match_token(&Token::LParen) {
                    return self.parse_function_call(name);
                }
                // Otherwise it's a column reference
                Ok(Expr::ColumnRef(name))
            }
            // Keyword that could be an identifier
            Token::Keyword(kw) => {
                // Some keywords can be function names or column names
                let name = self.parse_object_name()?;
                if self.match_token(&Token::LParen) {
                    return self.parse_function_call(name);
                }
                Ok(Expr::ColumnRef(name))
            }
            // @@ variable
            Token::SetIdent(s) => {
                self.advance();
                Ok(Expr::ColumnRef(vec![s]))
            }
            // Star (for SELECT *)
            Token::Star => {
                self.advance();
                Ok(Expr::ColumnRef(vec!["*".to_string()]))
            }
            _ => Err(ParserError::UnexpectedToken {
                position: self.pos,
                expected: "expression".to_string(),
                got: format!("{:?}", self.peek()),
            }),
        }
    }

    /// Parse a function call: name(args) or name(DISTINCT args)
    fn parse_function_call(&mut self, name: ObjectName) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        
        // Check for empty args: name()
        if self.match_token(&Token::RParen) {
            self.advance();
            return Ok(Expr::FunctionCall {
                name,
                args: vec![],
                distinct: false,
            });
        }
        
        // Check for DISTINCT
        let distinct = if self.match_keyword(Keyword::DISTINCT) {
            self.advance();
            true
        } else {
            false
        };
        
        // Check for star: count(*)
        if self.match_token(&Token::Star) {
            self.advance();
            self.expect_token(&Token::RParen)?;
            // Represent * as a special function call with empty args
            return Ok(Expr::FunctionCall {
                name,
                args: vec![Expr::ColumnRef(vec!["*".to_string()])],
                distinct,
            });
        }
        
        // Parse argument list
        let mut args = vec![self.parse_expr()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            args.push(self.parse_expr()?);
        }
        self.expect_token(&Token::RParen)?;
        
        Ok(Expr::FunctionCall { name, args, distinct })
    }

    /// Parse CASE expression
    fn parse_case_expr(&mut self) -> Result<Expr, ParserError> {
        self.expect_keyword(Keyword::CASE)?;
        
        // Optional operand: CASE expr WHEN ... ELSE ... END
        let operand = if !self.match_keyword(Keyword::WHEN) {
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        
        // WHEN clauses
        let mut whens = Vec::new();
        while self.match_keyword(Keyword::WHEN) {
            self.advance();
            let condition = self.parse_expr()?;
            self.expect_keyword(Keyword::THEN)?;
            let result = self.parse_expr()?;
            whens.push(WhenClause { condition, result });
        }
        
        // Optional ELSE
        let else_expr = if self.match_keyword(Keyword::ELSE) {
            self.advance();
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        
        self.expect_keyword(Keyword::END_P)?;
        
        Ok(Expr::Case {
            operand,
            whens,
            else_expr,
        })
    }
}
```

Add `use crate::ast::{Expr, Literal, WhenClause, ObjectName};` to the imports in `expr.rs`.

**Step 2: Implement `parse_expr` with Pratt parsing for operator precedence**

Add to `src/parser/expr.rs`:

```rust
impl Parser {
    /// Main expression parser using Pratt parsing (operator precedence).
    /// This handles the a_expr grammar from gram.y (lines 28269-28828).
    pub(crate) fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        self.parse_expr_with_precedence(0)
    }

    fn parse_expr_with_precedence(&mut self, min_prec: u8) -> Result<Expr, ParserError> {
        let mut left = self.parse_unary_expr()?;
        
        loop {
            let (op_prec, op_str, is_right_assoc) = match self.get_infix_operator() {
                Some(info) => info,
                None => break,
            };
            
            if op_prec < min_prec {
                break;
            }
            
            self.advance(); // consume the operator
            
            let right = self.parse_expr_with_precedence(
                if is_right_assoc { op_prec } else { op_prec + 1 }
            )?;
            
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: op_str,
                right: Box::new(right),
            };
        }
        
        // Handle postfix operations: IS NULL, IS NOT NULL, ISNULL, NOTNULL, ::typecast
        left = self.parse_postfix_ops(left)?;
        
        Ok(left)
    }

    /// Parse unary prefix expressions
    fn parse_unary_expr(&mut self) -> Result<Expr, ParserError> {
        // NOT
        if self.match_keyword(Keyword::NOT) {
            self.advance();
            let expr = self.parse_expr_with_precedence(self.not_precedence())?;
            return Ok(Expr::UnaryOp {
                op: "NOT".to_string(),
                expr: Box::new(expr),
            });
        }
        // Unary minus
        if self.match_token(&Token::Minus) {
            self.advance();
            let expr = self.parse_expr_with_precedence(self.unary_minus_precedence())?;
            return Ok(Expr::UnaryOp {
                op: "-".to_string(),
                expr: Box::new(expr),
            });
        }
        // Unary plus (no-op but parse it)
        if self.match_token(&Token::Plus) {
            self.advance();
            let expr = self.parse_expr_with_precedence(self.unary_minus_precedence())?;
            return Ok(expr);
        }
        // @ (absolute value)
        if self.match_token(&Token::At) {
            self.advance();
            let expr = self.parse_expr_with_precedence(self.unary_minus_precedence())?;
            return Ok(Expr::UnaryOp {
                op: "@".to_string(),
                expr: Box::new(expr),
            });
        }
        
        self.parse_primary_expr()
    }

    /// Get the precedence and string of the current infix operator, if any.
    /// Returns (precedence, operator_string, is_right_associative).
    fn get_infix_operator(&self) -> Option<(u8, String, bool)> {
        match self.peek() {
            // OR (lowest precedence for infix)
            Token::Keyword(Keyword::OR) => Some((5, "OR".to_string(), false)),
            // AND
            Token::Keyword(Keyword::AND) => Some((10, "AND".to_string(), false)),
            // NOT (when used as infix prefix — handled in unary)
            // Comparison operators
            Token::Eq => Some((20, "=".to_string(), false)),
            Token::Lt => Some((20, "<".to_string(), false)),
            Token::Gt => Some((20, ">".to_string(), false)),
            Token::Op(op) => match op.as_str() {
                "<=" | ">=" | "<>" | "!=" => Some((20, op.clone(), false)),
                "||" => Some((30, op.clone(), false)),
                "~" | "!~" | "~~" | "!~~" | "~~*" | "!~~*" => Some((20, op.clone(), false)),
                // Generic operator
                _ => Some((30, op.clone(), false)),
            },
            // Arithmetic operators
            Token::Plus => Some((40, "+".to_string(), false)),
            Token::Minus => Some((40, "-".to_string(), false)),
            Token::Star => Some((50, "*".to_string(), false)),
            Token::Slash => Some((50, "/".to_string(), false)),
            Token::Percent => Some((50, "%".to_string(), false)),
            Token::Caret => Some((55, "^".to_string(), false)),
            // Typecast
            Token::Typecast => Some((90, "::".to_string(), false)),
            // BETWEEN, IN, LIKE, etc. are handled as postfix/special
            // ColonEquals and ParamEquals — skip for now
            _ => None,
        }
    }

    /// Handle postfix operations on an expression: IS NULL, IS NOT NULL, ::type, BETWEEN, IN, etc.
    fn parse_postfix_ops(&mut self, mut left: Expr) -> Result<Expr, ParserError> {
        loop {
            match self.peek() {
                // IS NULL / IS NOT NULL / ISNULL
                Token::Keyword(Keyword::IS) => {
                    self.advance();
                    if self.match_keyword(Keyword::NOT) {
                        self.advance();
                        if self.match_keyword(Keyword::NULL_P) {
                            self.advance();
                            left = Expr::IsNull { expr: Box::new(left), negated: true };
                        } else {
                            // IS NOT something else — not supported, put NOT back
                            // For robustness, just return what we have
                            break;
                        }
                    } else if self.match_keyword(Keyword::NULL_P) {
                        self.advance();
                        left = Expr::IsNull { expr: Box::new(left), negated: false };
                    } else {
                        // IS something else — we don't handle IS TRUE/FALSE etc yet
                        break;
                    }
                }
                Token::Keyword(Keyword::ISNULL) => {
                    self.advance();
                    left = Expr::IsNull { expr: Box::new(left), negated: false };
                }
                Token::Keyword(Keyword::NOTNULL) => {
                    self.advance();
                    left = Expr::IsNull { expr: Box::new(left), negated: true };
                }
                // BETWEEN
                Token::Keyword(Keyword::BETWEEN) => {
                    self.advance();
                    let low = self.parse_expr_with_precedence(40)?;
                    self.expect_keyword(Keyword::AND)?;
                    let high = self.parse_expr_with_precedence(40)?;
                    left = Expr::Between { expr: Box::new(left), low: Box::new(low), high: Box::new(high), negated: false };
                }
                // NOT BETWEEN / NOT IN / NOT LIKE
                Token::Keyword(Keyword::NOT) => {
                    // Peek ahead to see if this is NOT BETWEEN, NOT IN, NOT LIKE, NOT ILIKE, NOT SIMILAR
                    let next_pos = self.pos + 1;
                    if let Some(tws) = self.tokens.get(next_pos) {
                        match &tws.token {
                            Token::Keyword(Keyword::BETWEEN) => {
                                self.advance(); // consume NOT
                                self.advance(); // consume BETWEEN
                                let low = self.parse_expr_with_precedence(40)?;
                                self.expect_keyword(Keyword::AND)?;
                                let high = self.parse_expr_with_precedence(40)?;
                                left = Expr::Between { expr: Box::new(left), low: Box::new(low), high: Box::new(high), negated: true };
                                continue;
                            }
                            Token::Keyword(Keyword::IN_P) => {
                                self.advance(); // consume NOT
                                self.advance(); // consume IN
                                left = self.parse_in_expr(left, true)?;
                                continue;
                            }
                            Token::Keyword(Keyword::LIKE) => {
                                self.advance();
                                self.advance();
                                let pattern = self.parse_primary_expr()?;
                                left = Expr::BinaryOp {
                                    left: Box::new(left),
                                    op: "NOT LIKE".to_string(),
                                    right: Box::new(pattern),
                                };
                                continue;
                            }
                            Token::Keyword(Keyword::ILIKE) => {
                                self.advance();
                                self.advance();
                                let pattern = self.parse_primary_expr()?;
                                left = Expr::BinaryOp {
                                    left: Box::new(left),
                                    op: "NOT ILIKE".to_string(),
                                    right: Box::new(pattern),
                                };
                                continue;
                            }
                            Token::Keyword(Keyword::SIMILAR) => {
                                self.advance();
                                self.advance();
                                self.expect_keyword(Keyword::TO)?;
                                let pattern = self.parse_primary_expr()?;
                                left = Expr::BinaryOp {
                                    left: Box::new(left),
                                    op: "NOT SIMILAR TO".to_string(),
                                    right: Box::new(pattern),
                                };
                                continue;
                            }
                            _ => break, // NOT followed by something else — stop
                        }
                    }
                    break;
                }
                // IN
                Token::Keyword(Keyword::IN_P) => {
                    self.advance();
                    left = self.parse_in_expr(left, false)?;
                }
                // LIKE
                Token::Keyword(Keyword::LIKE) => {
                    self.advance();
                    let pattern = self.parse_primary_expr()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: "LIKE".to_string(),
                        right: Box::new(pattern),
                    };
                }
                // ILIKE
                Token::Keyword(Keyword::ILIKE) => {
                    self.advance();
                    let pattern = self.parse_primary_expr()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: "ILIKE".to_string(),
                        right: Box::new(pattern),
                    };
                }
                // :: typecast handled as infix operator already
                _ => break,
            }
        }
        Ok(left)
    }

    /// Parse IN list or IN subquery: (expr, expr, ...) or (SELECT ...)
    fn parse_in_expr(&mut self, left: Expr, negated: bool) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        if self.match_keyword(Keyword::SELECT) || self.match_keyword(Keyword::WITH) {
            let subquery = self.parse_select_statement()?;
            self.expect_token(&Token::RParen)?;
            return Ok(Expr::InSubquery {
                expr: Box::new(left),
                subquery: Box::new(subquery),
                negated,
            });
        }
        let mut list = vec![self.parse_expr()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            list.push(self.parse_expr()?);
        }
        self.expect_token(&Token::RParen)?;
        Ok(Expr::InList { expr: Box::new(left), list, negated })
    }

    fn not_precedence(&self) -> u8 { 12 }
    fn unary_minus_precedence(&self) -> u8 { 60 }
}
```

**Step 3: Expose `parse_expr` from mod.rs**

In `src/parser/mod.rs`, add a wrapper method that calls into `expr.rs`:
```rust
// Inside impl Parser, add:
fn parse_expr(&mut self) -> Result<crate::ast::Expr, ParserError> {
    self.parse_primary_expr() // temporary: will be replaced with full expr parser
}
```

Actually, the `parse_expr` and `parse_primary_expr` are on the `Parser` impl in `expr.rs`, so they're automatically available via `mod expr;` in `mod.rs`. No extra wrapper needed. But we need to make sure the `parse_select_statement()` method exists (from select.rs) — add a stub for now that returns an error.

**Step 4: Add stub `parse_select_statement` to select.rs**

```rust
impl Parser {
    pub(crate) fn parse_select_statement(&mut self) -> Result<crate::ast::SelectStatement, ParserError> {
        Err(ParserError::UnexpectedToken {
            position: self.pos,
            expected: "SELECT statement (not yet implemented)".to_string(),
            got: format!("{:?}", self.peek()),
        })
    }
}
```

**Step 5: Verify build**

Run: `cargo build`
Expected: compiles with no errors

**Step 6: Commit**

```bash
git add src/parser/expr.rs src/parser/select.rs src/parser/mod.rs
git commit -m "feat: implement expression parser with Pratt parsing and primary expressions"
```

---

### Task 3: SELECT Statement Parser

**Files:**
- Modify: `src/parser/select.rs` — full implementation
- Modify: `src/ast/mod.rs` — expand SelectStatement and add new AST types
- Modify: `src/parser/mod.rs` — wire up parse_statement to call parse_select

**Goal:** Implement full SELECT statement parsing including:
- WITH clause (CTEs)
- DISTINCT / ALL
- Target list (column expressions, aliases, *)
- FROM clause (table references, joins, subqueries)
- WHERE clause
- GROUP BY / HAVING
- ORDER BY (ASC/DESC, NULLS FIRST/LAST)
- LIMIT / OFFSET
- UNION / INTERSECT / EXCEPT (set operations)

**Step 1: Expand AST types in `src/ast/mod.rs`**

Add/modify these types. The `SelectStatement` already exists, but needs enrichment:

```rust
// Add SetOperation to support UNION/INTERSECT/EXCEPT
#[derive(Debug, Clone, PartialEq)]
pub enum SetOperation {
    Union { all: bool },
    Intersect { all: bool },
    Except { all: bool },
}

// Update SelectStatement to support set operations:
#[derive(Debug, Clone, PartialEq)]
pub struct SelectStatement {
    pub with: Option<WithClause>,
    pub distinct: bool,
    pub targets: Vec<SelectTarget>,
    pub from: Vec<TableRef>,
    pub where_clause: Option<Expr>,
    pub group_by: Vec<Expr>,
    pub having: Option<Expr>,
    pub order_by: Vec<OrderByItem>,
    pub limit: Option<Expr>,
    pub offset: Option<Expr>,
    // Set operation support
    pub set_operation: Option<(SetOperation, Box<SelectStatement>)>,
}
```

Update `TableRef`:
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TableRef {
    Table {
        name: ObjectName,
        alias: Option<String>,
    },
    Subquery {
        query: Box<SelectStatement>,
        alias: Option<String>,
    },
    Join {
        left: Box<TableRef>,
        right: Box<TableRef>,
        join_type: JoinType,
        condition: Option<Expr>,
    },
    FunctionCall {
        name: ObjectName,
        alias: Option<String>,
    },
}
```

**Step 2: Implement parse_select_statement in select.rs**

Full implementation covering:
```rust
impl Parser {
    pub(crate) fn parse_select_statement(&mut self) -> Result<SelectStatement, ParserError> {
        let with = self.parse_with_clause()?;
        let mut stmt = self.parse_simple_select()?;
        stmt.with = with;
        
        // Handle set operations (UNION, INTERSECT, EXCEPT, MINUS)
        loop {
            let set_op = match self.peek_keyword() {
                Some(Keyword::UNION) => {
                    self.advance();
                    let all = self.try_consume(&Token::Keyword(Keyword::ALL)).is_some();
                    SetOperation::Union { all }
                }
                Some(Keyword::INTERSECT) => {
                    self.advance();
                    let all = self.try_consume(&Token::Keyword(Keyword::ALL)).is_some();
                    SetOperation::Intersect { all }
                }
                Some(Keyword::EXCEPT) => {
                    self.advance();
                    let all = self.try_consume(&Token::Keyword(Keyword::ALL)).is_some();
                    SetOperation::Except { all }
                }
                Some(Keyword::MINUS_P) => {
                    self.advance();
                    let all = self.try_consume(&Token::Keyword(Keyword::ALL)).is_some();
                    SetOperation::Except { all }
                }
                _ => break,
            };
            
            let right = self.parse_simple_select()?;
            stmt = SelectStatement {
                with: None,
                distinct: false,
                targets: vec![],
                from: vec![],
                where_clause: None,
                group_by: vec![],
                having: None,
                order_by: vec![],
                limit: None,
                offset: None,
                set_operation: Some((set_op, Box::new(stmt))),
            };
            // Re-assign: the left side becomes the `right` parsed just now
            // Actually we need to restructure: stmt.set_operation = Some((set_op, Box::new(prev_stmt)))
            // and the new stmt has the right as the base
        }
        
        // Handle ORDER BY, LIMIT, OFFSET (only valid on the outermost SELECT)
        self.parse_order_limit_offset(&mut stmt)?;
        
        Ok(stmt)
    }
    
    fn parse_with_clause(&mut self) -> Result<Option<WithClause>, ParserError> {
        if !self.match_keyword(Keyword::WITH) {
            return Ok(None);
        }
        self.advance();
        
        let recursive = self.try_consume(&Token::Keyword(Keyword::RECURSIVE)).is_some();
        let mut ctes = Vec::new();
        
        loop {
            let name = self.parse_identifier()?;
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
            self.expect_token(&Token::LParen)?;
            let query = self.parse_select_statement()?;
            self.expect_token(&Token::RParen)?;
            
            ctes.push(Cte { name, columns, query: Box::new(query) });
            
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }
        
        Ok(Some(WithClause { recursive, ctes }))
    }
    
    fn parse_simple_select(&mut self) -> Result<SelectStatement, ParserError> {
        self.expect_keyword(Keyword::SELECT)?;
        
        // DISTINCT / ALL
        let distinct = if self.match_keyword(Keyword::DISTINCT) {
            self.advance();
            // Optional ON (expr_list) — skip for now
            true
        } else {
            if self.match_keyword(Keyword::ALL) {
                self.advance();
            }
            false
        };
        
        // Target list
        let targets = self.parse_target_list()?;
        
        // FROM clause
        let from = self.parse_from_clause()?;
        
        // WHERE clause
        let where_clause = if self.match_keyword(Keyword::WHERE) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        
        // GROUP BY
        let group_by = if self.match_keyword(Keyword::GROUP_P) {
            self.advance();
            self.expect_keyword(Keyword::BY)?;
            let mut items = vec![self.parse_expr()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                items.push(self.parse_expr()?);
            }
            items
        } else {
            vec![]
        };
        
        // HAVING
        let having = if self.match_keyword(Keyword::HAVING) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        
        Ok(SelectStatement {
            with: None,
            distinct,
            targets,
            from,
            where_clause,
            group_by,
            having,
            order_by: vec![],
            limit: None,
            offset: None,
            set_operation: None,
        })
    }
    
    fn parse_target_list(&mut self) -> Result<Vec<SelectTarget>, ParserError> {
        let mut targets = vec![self.parse_target_el()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            targets.push(self.parse_target_el()?);
        }
        Ok(targets)
    }
    
    fn parse_target_el(&mut self) -> Result<SelectTarget, ParserError> {
        // Star: *
        if self.match_token(&Token::Star) {
            self.advance();
            // Check for table.* pattern
            // For simplicity, just return Star with no qualifier
            return Ok(SelectTarget::Star(None));
        }
        
        let expr = self.parse_expr()?;
        
        // Check for alias: [AS] ident
        let alias = if self.match_keyword(Keyword::AS) {
            self.advance();
            Some(self.parse_identifier()?)
        } else {
            self.parse_optional_alias()?
        };
        
        match alias {
            Some(name) => Ok(SelectTarget::Expr(expr, Some(name))),
            None => Ok(SelectTarget::Expr(expr, None)),
        }
    }
    
    fn parse_from_clause(&mut self) -> Result<Vec<TableRef>, ParserError> {
        if !self.match_keyword(Keyword::FROM) {
            return Ok(vec![]);
        }
        self.advance();
        
        let mut tables = vec![self.parse_table_ref()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            tables.push(self.parse_table_ref()?);
        }
        Ok(tables)
    }
    
    fn parse_table_ref(&mut self) -> Result<TableRef, ParserError> {
        let mut left = self.parse_primary_table_ref()?;
        
        // Handle JOINs
        loop {
            let join_type = match self.peek_keyword() {
                Some(Keyword::JOIN) => {
                    self.advance();
                    JoinType::Inner
                }
                Some(Keyword::INNER_P) => {
                    self.advance();
                    self.expect_keyword(Keyword::JOIN)?;
                    JoinType::Inner
                }
                Some(Keyword::LEFT) => {
                    self.advance();
                    self.try_consume(&Token::Keyword(Keyword::OUTER_P));
                    self.expect_keyword(Keyword::JOIN)?;
                    JoinType::Left
                }
                Some(Keyword::RIGHT) => {
                    self.advance();
                    self.try_consume(&Token::Keyword(Keyword::OUTER_P));
                    self.expect_keyword(Keyword::JOIN)?;
                    JoinType::Right
                }
                Some(Keyword::FULL) => {
                    self.advance();
                    self.try_consume(&Token::Keyword(Keyword::OUTER_P));
                    self.expect_keyword(Keyword::JOIN)?;
                    JoinType::Full
                }
                Some(Keyword::CROSS) => {
                    self.advance();
                    self.expect_keyword(Keyword::JOIN)?;
                    JoinType::Cross
                }
                Some(Keyword::NATURAL) => {
                    self.advance();
                    let inner_type = match self.peek_keyword() {
                        Some(Keyword::JOIN) => {
                            self.advance();
                            JoinType::Inner // Natural defaults to inner
                        }
                        Some(Keyword::LEFT) => {
                            self.advance();
                            self.try_consume(&Token::Keyword(Keyword::OUTER_P));
                            self.expect_keyword(Keyword::JOIN)?;
                            JoinType::Left
                        }
                        Some(Keyword::RIGHT) => {
                            self.advance();
                            self.try_consume(&Token::Keyword(Keyword::OUTER_P));
                            self.expect_keyword(Keyword::JOIN)?;
                            JoinType::Right
                        }
                        Some(Keyword::FULL) => {
                            self.advance();
                            self.try_consume(&Token::Keyword(Keyword::OUTER_P));
                            self.expect_keyword(Keyword::JOIN)?;
                            JoinType::Full
                        }
                        _ => return Err(ParserError::UnexpectedToken {
                            position: self.pos,
                            expected: "JOIN after NATURAL".to_string(),
                            got: format!("{:?}", self.peek()),
                        }),
                    };
                    inner_type
                }
                _ => break,
            };
            
            let right = self.parse_primary_table_ref()?;
            
            let condition = if join_type != JoinType::Cross {
                if self.match_keyword(Keyword::ON) {
                    self.advance();
                    Some(self.parse_expr()?)
                } else if self.match_keyword(Keyword::USING) {
                    self.advance();
                    self.expect_token(&Token::LParen)?;
                    // Parse column list — for now, skip to rparen
                    let mut _cols = vec![self.parse_identifier()?];
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        _cols.push(self.parse_identifier()?);
                    }
                    self.expect_token(&Token::RParen)?;
                    None // USING is a special form, represent as None for now
                } else {
                    None
                }
            } else {
                None
            };
            
            left = TableRef::Join {
                left: Box::new(left),
                right: Box::new(right),
                join_type,
                condition,
            };
        }
        
        Ok(left)
    }
    
    fn parse_primary_table_ref(&mut self) -> Result<TableRef, ParserError> {
        // Parenthesized subquery: (SELECT ...)
        if self.match_token(&Token::LParen) {
            self.advance();
            if self.match_keyword(Keyword::SELECT) || self.match_keyword(Keyword::WITH) {
                let query = self.parse_select_statement()?;
                self.expect_token(&Token::RParen)?;
                let alias = self.parse_optional_alias()?;
                return Ok(TableRef::Subquery {
                    query: Box::new(query),
                    alias,
                });
            }
            // Parenthesized join: (t1 JOIN t2 ON ...)
            let table_ref = self.parse_table_ref()?;
            self.expect_token(&Token::RParen)?;
            return Ok(table_ref);
        }
        
        // LATERAL (subquery)
        if self.match_keyword(Keyword::LATERAL_P) {
            self.advance();
            if self.match_token(&Token::LParen) {
                self.advance();
                let query = self.parse_select_statement()?;
                self.expect_token(&Token::RParen)?;
                let alias = self.parse_optional_alias()?;
                return Ok(TableRef::Subquery {
                    query: Box::new(query),
                    alias,
                });
            }
        }
        
        // Table name
        let name = self.parse_object_name()?;
        let alias = self.parse_optional_alias()?;
        
        Ok(TableRef::Table { name, alias })
    }
    
    fn parse_order_limit_offset(&mut self, stmt: &mut SelectStatement) -> Result<(), ParserError> {
        // ORDER BY
        if self.match_keyword(Keyword::ORDER) {
            self.advance();
            self.expect_keyword(Keyword::BY)?;
            let mut items = Vec::new();
            loop {
                let expr = self.parse_expr()?;
                let asc = match self.peek_keyword() {
                    Some(Keyword::ASC) => { self.advance(); Some(true) }
                    Some(Keyword::DESC) => { self.advance(); Some(false) }
                    _ => None,
                };
                let nulls_first = if self.match_keyword(Keyword::NULLS_P) {
                    self.advance();
                    if self.match_keyword(Keyword::FIRST_P) {
                        self.advance();
                        Some(true)
                    } else {
                        self.expect_keyword(Keyword::LAST)?;
                        Some(false)
                    }
                } else {
                    None
                };
                items.push(OrderByItem { expr, asc, nulls_first });
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            stmt.order_by = items;
        }
        
        // LIMIT
        if self.match_keyword(Keyword::LIMIT) {
            self.advance();
            if self.match_keyword(Keyword::ALL) {
                self.advance();
                stmt.limit = None;
            } else {
                stmt.limit = Some(self.parse_expr()?);
            }
        }
        
        // OFFSET
        if self.match_keyword(Keyword::OFFSET) {
            self.advance();
            stmt.offset = Some(self.parse_expr()?);
        }
        
        Ok(())
    }
}
```

**Step 3: Wire up parse_statement in mod.rs**

In `src/parser/mod.rs`, replace the `SELECT`/`WITH` branch in `parse_statement`:
```rust
Token::Keyword(Keyword::SELECT) | Token::Keyword(Keyword::WITH) => {
    let stmt = self.parse_select_statement()?;
    // consume optional semicolon
    if self.match_token(&Token::Semicolon) {
        self.advance();
    }
    Statement::Select(stmt)
}
```

**Step 4: Verify build and run regression test**

Run: `cargo build`
Run: `cargo run --example regression`
Expected: May have some parsing errors — iterate to fix. The key is that all files should still either parse or gracefully error.

**Step 5: Commit**

```bash
git add src/parser/select.rs src/ast/mod.rs src/parser/mod.rs
git commit -m "feat: implement SELECT statement parser with joins, CTEs, set operations, ORDER BY, LIMIT, OFFSET"
```

---

### Task 4: INSERT Statement Parser

**Files:**
- Modify: `src/parser/dml.rs`
- Modify: `src/ast/mod.rs` — replace `InsertStatement` stub with proper struct
- Modify: `src/parser/mod.rs` — wire up

**Goal:** Parse INSERT statements:
```
INSERT INTO table [(col1, col2, ...)] VALUES (v1, v2, ...), (v3, v4, ...)
INSERT INTO table [(col1, col2, ...)] SELECT ...
INSERT INTO table SET col1 = v1, col2 = v2
INSERT INTO table DEFAULT VALUES
```

**Step 1: Define InsertStatement struct in ast/mod.rs**

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct InsertStatement {
    pub table: ObjectName,
    pub columns: Vec<String>,
    pub source: InsertSource,
    pub returning: Vec<SelectTarget>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InsertSource {
    Values(Vec<Vec<Expr>>),
    Select(Box<SelectStatement>),
    DefaultValues,
    Set(Vec<(String, Expr)>),
}
```

Remove `InsertStatement` from the `stub_struct!` macro invocation.

**Step 2: Implement parse_insert in dml.rs**

```rust
impl Parser {
    pub(crate) fn parse_insert(&mut self) -> Result<InsertStatement, ParserError> {
        // INSERT keyword already consumed
        // Optional INTO
        self.try_consume(&Token::Keyword(Keyword::INTO));
        
        let table = self.parse_object_name()?;
        
        // Optional column list: (col1, col2, ...)
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
        
        // Determine source
        let source = if self.match_keyword(Keyword::DEFAULT) {
            self.advance();
            self.expect_keyword(Keyword::VALUES)?;
            InsertSource::DefaultValues
        } else if self.match_keyword(Keyword::VALUES) {
            self.advance();
            let mut rows = Vec::new();
            loop {
                self.expect_token(&Token::LParen)?;
                let mut row = Vec::new();
                if !self.match_token(&Token::RParen) {
                    row.push(self.parse_expr()?);
                    while self.match_token(&Token::Comma) {
                        self.advance();
                        row.push(self.parse_expr()?);
                    }
                }
                self.expect_token(&Token::RParen)?;
                rows.push(row);
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            InsertSource::Values(rows)
        } else if self.match_keyword(Keyword::SELECT) || self.match_keyword(Keyword::WITH) {
            let select = self.parse_select_statement()?;
            InsertSource::Select(Box::new(select))
        } else if self.match_keyword(Keyword::SET) {
            self.advance();
            let mut sets = Vec::new();
            loop {
                let col = self.parse_identifier()?;
                self.expect_token(&Token::Eq)?;
                let val = self.parse_expr()?;
                sets.push((col, val));
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }
            InsertSource::Set(sets)
        } else {
            return Err(ParserError::UnexpectedToken {
                position: self.pos,
                expected: "VALUES, SELECT, DEFAULT VALUES, or SET".to_string(),
                got: format!("{:?}", self.peek()),
            });
        };
        
        // Optional RETURNING
        let returning = if self.match_keyword(Keyword::RETURNING) {
            self.advance();
            self.parse_target_list()?
        } else {
            vec![]
        };
        
        Ok(InsertStatement { table, columns, source, returning })
    }
}
```

**Step 3: Wire up in mod.rs**

Replace the INSERT branch in parse_statement:
```rust
Token::Keyword(Keyword::INSERT) => {
    self.advance();
    let stmt = self.parse_insert()?;
    if self.match_token(&Token::Semicolon) { self.advance(); }
    Statement::Insert(stmt)
}
```

**Step 4: Build and test**

Run: `cargo build && cargo run --example regression`

**Step 5: Commit**

```bash
git add src/parser/dml.rs src/ast/mod.rs src/parser/mod.rs
git commit -m "feat: implement INSERT statement parser with VALUES, SELECT, SET, DEFAULT VALUES"
```

---

### Task 5: UPDATE Statement Parser

**Files:**
- Modify: `src/parser/dml.rs`
- Modify: `src/ast/mod.rs` — replace `UpdateStatement` stub
- Modify: `src/parser/mod.rs` — wire up

**Goal:** Parse UPDATE statements:
```
UPDATE table SET col1 = expr1, col2 = expr2 [FROM ...] [WHERE ...] [RETURNING ...]
UPDATE t1, t2 SET ... (multi-table)
```

**Step 1: Define UpdateStatement struct**

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateStatement {
    pub tables: Vec<TableRef>,
    pub assignments: Vec<UpdateAssignment>,
    pub from: Vec<TableRef>,
    pub where_clause: Option<Expr>,
    pub order_by: Vec<OrderByItem>,
    pub limit: Option<Expr>,
    pub returning: Vec<SelectTarget>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateAssignment {
    pub column: ObjectName,
    pub value: Expr,
}
```

Remove `UpdateStatement` from `stub_struct!` macro.

**Step 2: Implement parse_update in dml.rs**

```rust
pub(crate) fn parse_update(&mut self) -> Result<UpdateStatement, ParserError> {
    // UPDATE keyword already consumed
    let mut tables = vec![self.parse_table_ref()?];
    while self.match_token(&Token::Comma) {
        self.advance();
        tables.push(self.parse_table_ref()?);
    }
    
    self.expect_keyword(Keyword::SET)?;
    
    let mut assignments = Vec::new();
    loop {
        let column = self.parse_object_name()?;
        self.expect_token(&Token::Eq)?;
        let value = self.parse_expr()?;
        assignments.push(UpdateAssignment { column, value });
        if !self.match_token(&Token::Comma) {
            break;
        }
        self.advance();
    }
    
    let from = self.parse_from_clause()?;
    
    let where_clause = if self.match_keyword(Keyword::WHERE) {
        self.advance();
        Some(self.parse_expr()?)
    } else {
        None
    };
    
    // ORDER BY, LIMIT (MySQL compatibility)
    let mut order_by = Vec::new();
    let mut limit = None;
    // Reuse parse_order_limit_offset logic if needed
    
    let returning = if self.match_keyword(Keyword::RETURNING) {
        self.advance();
        self.parse_target_list()?
    } else {
        vec![]
    };
    
    Ok(UpdateStatement { tables, assignments, from, where_clause, order_by, limit, returning })
}
```

**Step 3: Wire up and commit** (same pattern as INSERT)

---

### Task 6: DELETE Statement Parser

**Files:**
- Modify: `src/parser/dml.rs`
- Modify: `src/ast/mod.rs` — replace `DeleteStatement` stub
- Modify: `src/parser/mod.rs` — wire up

**Goal:** Parse DELETE statements:
```
DELETE FROM table [WHERE ...] [RETURNING ...]
DELETE table FROM table2 [WHERE ...] (multi-table)
```

**Step 1: Define DeleteStatement struct**

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteStatement {
    pub tables: Vec<TableRef>,
    pub using: Vec<TableRef>,
    pub where_clause: Option<Expr>,
    pub returning: Vec<SelectTarget>,
}
```

Remove `DeleteStatement` from `stub_struct!` macro.

**Step 2: Implement parse_delete in dml.rs**

```rust
pub(crate) fn parse_delete(&mut self) -> Result<DeleteStatement, ParserError> {
    // DELETE_P keyword already consumed
    // Optional FROM
    let has_from = self.try_consume(&Token::Keyword(Keyword::FROM)).is_some();
    
    let mut tables = vec![self.parse_table_ref()?];
    while self.match_token(&Token::Comma) {
        self.advance();
        tables.push(self.parse_table_ref()?);
    }
    
    // If DELETE FROM table, check for USING or WHERE
    // If DELETE table FROM ..., the FROM here means USING
    let using = if !has_from && self.match_keyword(Keyword::FROM) {
        self.advance();
        self.parse_from_clause()?
    } else if self.match_keyword(Keyword::USING) {
        self.advance();
        self.parse_from_clause()?
    } else {
        vec![]
    };
    
    let where_clause = if self.match_keyword(Keyword::WHERE) {
        self.advance();
        Some(self.parse_expr()?)
    } else {
        None
    };
    
    let returning = if self.match_keyword(Keyword::RETURNING) {
        self.advance();
        self.parse_target_list()?
    } else {
        vec![]
    };
    
    Ok(DeleteStatement { tables, using, where_clause, returning })
}
```

**Step 3: Wire up and commit**

---

### Task 7: MERGE Statement Parser

**Files:**
- Modify: `src/parser/dml.rs`
- Modify: `src/ast/mod.rs` — replace `MergeStatement` stub
- Modify: `src/parser/mod.rs` — wire up

**Goal:** Parse MERGE statements:
```
MERGE INTO target_table USING source_table ON condition
  WHEN MATCHED THEN UPDATE SET ...
  WHEN NOT MATCHED THEN INSERT (...) VALUES (...)
```

**Step 1: Define MergeStatement struct**

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct MergeStatement {
    pub target: TableRef,
    pub source: TableRef,
    pub on_condition: Expr,
    pub when_clauses: Vec<MergeWhenClause>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MergeWhenClause {
    pub matched: bool,
    pub action: MergeAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MergeAction {
    Update(Vec<UpdateAssignment>),
    Delete,
    Insert { columns: Vec<String>, values: Vec<Expr> },
}
```

Remove `MergeStatement` from `stub_struct!` macro.

**Step 2: Implement parse_merge in dml.rs**

```rust
pub(crate) fn parse_merge(&mut self) -> Result<MergeStatement, ParserError> {
    // MERGE keyword already consumed
    self.try_consume(&Token::Keyword(Keyword::INTO));
    
    let target = self.parse_table_ref()?;
    
    self.expect_keyword(Keyword::USING)?;
    let source = self.parse_table_ref()?;
    
    self.expect_keyword(Keyword::ON)?;
    let on_condition = self.parse_expr()?;
    
    let mut when_clauses = Vec::new();
    while self.match_keyword(Keyword::WHEN) {
        self.advance();
        let matched = if self.match_keyword(Keyword::NOT) {
            self.advance();
            self.expect_keyword(Keyword::MATCHED)?;
            false
        } else {
            self.expect_keyword(Keyword::MATCHED)?;
            true
        };
        
        self.expect_keyword(Keyword::THEN)?;
        
        let action = if self.match_keyword(Keyword::UPDATE) {
            self.advance();
            self.expect_keyword(Keyword::SET)?;
            let mut assignments = Vec::new();
            loop {
                let column = self.parse_object_name()?;
                self.expect_token(&Token::Eq)?;
                let value = self.parse_expr()?;
                assignments.push(UpdateAssignment { column, value });
                if !self.match_token(&Token::Comma) { break; }
                self.advance();
            }
            MergeAction::Update(assignments)
        } else if self.match_keyword(Keyword::DELETE_P) {
            self.advance();
            MergeAction::Delete
        } else if self.match_keyword(Keyword::INSERT) {
            self.advance();
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
            self.expect_keyword(Keyword::VALUES)?;
            self.expect_token(&Token::LParen)?;
            let mut values = vec![self.parse_expr()?];
            while self.match_token(&Token::Comma) {
                self.advance();
                values.push(self.parse_expr()?);
            }
            self.expect_token(&Token::RParen)?;
            MergeAction::Insert { columns, values }
        } else {
            return Err(ParserError::UnexpectedToken {
                position: self.pos,
                expected: "UPDATE, DELETE, or INSERT in MERGE WHEN clause".to_string(),
                got: format!("{:?}", self.peek()),
            });
        };
        
        when_clauses.push(MergeWhenClause { matched, action });
    }
    
    Ok(MergeStatement { target, source, on_condition, when_clauses })
}
```

**Step 3: Wire up and commit**

---

### Task 8: Integration Testing & Regression Validation

**Files:**
- Modify: `src/parser/mod.rs` — ensure all DML statements wired correctly
- Modify: `examples/regression.rs` — add detailed reporting

**Goal:** Run the full regression test suite and fix any parsing errors. Ensure all 1,397 SQL files pass.

**Step 1: Add unit tests for expression parsing**

Add to `src/parser/expr.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    fn parse(sql: &str) -> Result<Expr, ParserError> {
        let tokens = crate::token::tokenizer::Tokenizer::new(sql).tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse_expr()
    }
    
    #[test]
    fn test_integer_literal() {
        assert_eq!(parse("42").unwrap(), Expr::Literal(Literal::Integer(42)));
    }
    
    #[test]
    fn test_string_literal() {
        assert_eq!(parse("'hello'").unwrap(), Expr::Literal(Literal::String("hello".to_string())));
    }
    
    #[test]
    fn test_column_ref() {
        assert_eq!(parse("x").unwrap(), Expr::ColumnRef(vec!["x".to_string()]));
    }
    
    #[test]
    fn test_qualified_column() {
        assert_eq!(parse("t.x").unwrap(), Expr::ColumnRef(vec!["t".to_string(), "x".to_string()]));
    }
    
    #[test]
    fn test_binary_op() {
        let result = parse("1 + 2").unwrap();
        match result {
            Expr::BinaryOp { op, .. } => assert_eq!(op, "+"),
            _ => panic!("expected BinaryOp"),
        }
    }
    
    #[test]
    fn test_precedence() {
        // 1 + 2 * 3 should parse as 1 + (2 * 3)
        let result = parse("1 + 2 * 3").unwrap();
        match result {
            Expr::BinaryOp { op, left, .. } => {
                assert_eq!(op, "+");
                assert!(matches!(*left, Expr::Literal(Literal::Integer(1))));
            }
            _ => panic!("expected BinaryOp with +"),
        }
    }
    
    #[test]
    fn test_parentheses() {
        // (1 + 2) * 3 should parse as (1+2) * 3
        let result = parse("(1 + 2) * 3").unwrap();
        match result {
            Expr::BinaryOp { op, left, .. } => {
                assert_eq!(op, "*");
                assert!(matches!(*left, Expr::BinaryOp { op: ref s, .. } if s == "+"));
            }
            _ => panic!("expected BinaryOp with *"),
        }
    }
    
    #[test]
    fn test_function_call() {
        let result = parse("count(*)").unwrap();
        match result {
            Expr::FunctionCall { name, .. } => {
                assert_eq!(name, vec!["count".to_string()]);
            }
            _ => panic!("expected FunctionCall"),
        }
    }
    
    #[test]
    fn test_is_null() {
        let result = parse("x IS NULL").unwrap();
        assert!(matches!(result, Expr::IsNull { negated: false, .. }));
    }
    
    #[test]
    fn test_between() {
        let result = parse("x BETWEEN 1 AND 10").unwrap();
        assert!(matches!(result, Expr::Between { negated: false, .. }));
    }
    
    #[test]
    fn test_in_list() {
        let result = parse("x IN (1, 2, 3)").unwrap();
        assert!(matches!(result, Expr::InList { negated: false, .. }));
    }
}
```

**Step 2: Add unit tests for SELECT parsing**

Add to `src/parser/select.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    fn parse_select(sql: &str) -> Result<SelectStatement, ParserError> {
        let tokens = crate::token::tokenizer::Tokenizer::new(sql).tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse_select_statement()
    }
    
    #[test]
    fn test_simple_select() {
        let result = parse_select("SELECT 1").unwrap();
        assert_eq!(result.targets.len(), 1);
    }
    
    #[test]
    fn test_select_from() {
        let result = parse_select("SELECT * FROM t").unwrap();
        assert_eq!(result.from.len(), 1);
    }
    
    #[test]
    fn test_select_join() {
        let result = parse_select("SELECT * FROM t1 JOIN t2 ON t1.id = t2.id").unwrap();
        assert!(matches!(result.from[0], TableRef::Join { .. }));
    }
    
    #[test]
    fn test_select_where() {
        let result = parse_select("SELECT * FROM t WHERE x > 5").unwrap();
        assert!(result.where_clause.is_some());
    }
    
    #[test]
    fn test_select_order_by() {
        let result = parse_select("SELECT * FROM t ORDER BY x DESC").unwrap();
        assert_eq!(result.order_by.len(), 1);
        assert_eq!(result.order_by[0].asc, Some(false));
    }
    
    #[test]
    fn test_select_limit() {
        let result = parse_select("SELECT * FROM t LIMIT 10").unwrap();
        assert!(result.limit.is_some());
    }
    
    #[test]
    fn test_select_with_cte() {
        let result = parse_select("WITH cte AS (SELECT 1) SELECT * FROM cte").unwrap();
        assert!(result.with.is_some());
    }
}
```

**Step 3: Run all tests**

Run: `cargo test`
Expected: All new tests pass, no regressions

**Step 4: Run regression test**

Run: `cargo run --example regression`
Expected: All 1,397 SQL files pass (tokenize + parse without error)

If any files fail, analyze the error, fix the parser, and re-run. Common issues:
- Keywords not recognized as identifiers in certain contexts
- Operator precedence conflicts
- Missing support for specific SQL syntax (e.g., `FOR UPDATE`, `INTO`, window functions)

**Step 5: Fix any regressions and commit**

```bash
git add .
git commit -m "feat: add unit tests for expression and SELECT parsing, validate regression suite"
```

---

### Task 9: Error Recovery for Unknown Statements

**Files:**
- Modify: `src/parser/mod.rs`

**Goal:** Ensure that any DDL or other statements we haven't implemented yet still use `skip_to_semicolon()` gracefully. The parser should NOT crash on any SQL statement.

**Step 1: Verify all non-DML statements still use skip_to_semicolon**

In `src/parser/mod.rs`, verify that all branches in `parse_statement()`, `dispatch_create()`, `dispatch_alter()`, and `dispatch_drop()` that aren't DML still call `skip_to_semicolon()` and return `Statement::Empty`.

**Step 2: Add a catch-all error recovery in parse_expr**

If `parse_expr()` encounters something it can't handle, it should return an error rather than panicking. The top-level `parse_statement()` should catch this and fall back to `skip_to_semicolon()`.

Update `parse_statement()` in mod.rs to wrap DML parsing in a recovery block:
```rust
Token::Keyword(Keyword::SELECT) | Token::Keyword(Keyword::WITH) => {
    match self.parse_select_statement() {
        Ok(stmt) => {
            if self.match_token(&Token::Semicolon) { self.advance(); }
            Statement::Select(stmt)
        }
        Err(_) => {
            // Fallback: skip to semicolon
            self.skip_to_semicolon()
        }
    }
}
```

Apply the same pattern to INSERT, UPDATE, DELETE, MERGE.

**Step 3: Run regression test**

Run: `cargo run --example regression`
Expected: 1,397/1,397 files pass (some may parse as Statement::Empty if they use unimplemented features)

**Step 4: Commit**

```bash
git add src/parser/mod.rs
git commit -m "feat: add error recovery for DML parsing, graceful fallback to skip_to_semicolon"
```

---

## Summary

| Task | Description | Dependencies |
|------|-------------|-------------|
| 1 | Parser Infrastructure & Helpers | None |
| 2 | Expression Parser (Pratt) | Task 1 |
| 3 | SELECT Statement Parser | Tasks 1, 2 |
| 4 | INSERT Statement Parser | Tasks 1, 2 |
| 5 | UPDATE Statement Parser | Tasks 1, 2 |
| 6 | DELETE Statement Parser | Tasks 1, 2 |
| 7 | MERGE Statement Parser | Tasks 1, 2 |
| 8 | Integration Testing & Regression | Tasks 1-7 |
| 9 | Error Recovery | Tasks 1-8 |

**Execution order:** Tasks 1-3 are sequential (each depends on the previous). Tasks 4-7 depend on Tasks 1-2 and can be done in sequence. Tasks 8-9 are validation/cleanup.

**Total estimated lines of new code:** ~1,500-2,000 lines across all modules.
