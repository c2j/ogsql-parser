# XML Functions (XMLELEMENT/XMLATTRIBUTES etc.) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add full GaussDB-compatible XML function parsing support (XMLELEMENT, XMLATTRIBUTES, XMLCONCAT, XMLFOREST, XMLPARSE, XMLPI, XMLROOT, XMLSERIALIZE) with ENTITYESCAPING/NOENTITYESCAPING, EVALNAME, and WELLFORMED keywords.

**Architecture:** Add new XML-specific AST types, XML-specific parsing in expr.rs (before the generic keyword→function fallback), new keywords, formatter support, and visitor support. Each XML function gets its own parsing method.

**Tech Stack:** Rust, recursive descent parser, existing token/AST/parser/formatter/visitor infrastructure.

**Reference:**
- GaussDB syntax: `GaussDB-2.23.07.210/` documentation (search for "xmlelement")
- openGauss grammar: `lib/openGauss-server/src/common/backend/parser/gram.y` lines 30365-30485
- Current parser: `src/parser/expr.rs` parse_primary_expr (line 270) and parse_function_call (line 432)

---

## Task 1: Add Missing Keywords

**Files:**
- Modify: `src/token/keyword.rs`

**Step 1: Add new keyword variants to the Keyword enum**

Insert after `ENCRYPTION_TYPE` (line 210), in alphabetical order:

```rust
    // After ENCRYPTION_TYPE (line 210):
    ENTITYESCAPING,
    // Before ENUM_P
```

Insert after `NOCYCLE` (find exact line), in alphabetical order:

```rust
    // After NOCYCLE:
    NOENTITYESCAPING,
    // Before NOEXTEND
```

Insert after `ESCAPING` (line 220), in alphabetical order:

```rust
    // After ESCAPING:
    EVALNAME,
    // Before EVENT
```

Insert after `WEAK` (find exact line), in alphabetical order:

```rust
    // After WEAK:
    WELLFORMED,
    // BEFORE WHEN
```

**Step 2: Add as_str() mappings**

In the `as_str()` match, after `Keyword::ENCRYPTION_TYPE => "encryption_type"` (line 933):

```rust
            Keyword::ENTITYESCAPING => "entityescaping",
```

After the NOCYCLE mapping, before NOEXTEND:

```rust
            Keyword::NOENTITYESCAPING => "noentityescaping",
```

After `Keyword::ESCAPING => "escaping"` (line 943):

```rust
            Keyword::EVALNAME => "evalname",
```

After the WEAK mapping, before WHEN:

```rust
            Keyword::WELLFORMED => "wellformeded",
```

**Step 3: Add keyword category**

These are all `Unreserved` keywords per the GaussDB docs. Add them to the `category()` match in the appropriate groups:

- `ENTITYESCAPING` → group with `ENCRYPTION_TYPE`/`ENDS`/`ENFORCED`/`ENUM_P` in Unreserved
- `NOENTITYESCAPING` → group with NOCYCLE in Unreserved
- `EVALNAME` → group with `ESCAPING` in Unreserved
- `WELLFORMED` → group with `WEAK` in Unreserved

**Step 4: Add to lookup table**

In the sorted `KEYWORDS` array, insert in alphabetical order:

```rust
        ("entityescaping", Keyword::ENTITYESCAPING),
        // ...
        ("evalname", Keyword::EVALNAME),
        // ...
        ("noentityescaping", Keyword::NOENTITYESCAPING),
        // ...
        ("wellformeded", Keyword::WELLFORMED),
```

**Step 5: Verify**

Run: `cargo build`
Expected: Compiles with no errors.

Run: `echo "SELECT noentityescaping" | ./target/debug/ogsql tokenize`
Expected: `Keyword(NOENTITYESCAPING)` instead of `Ident("noentityescaping")`

**Step 6: Commit**

```
feat: add ENTITYESCAPING, NOENTITYESCAPING, EVALNAME, WELLFORMED keywords
```

---

## Task 2: Add XML AST Types

**Files:**
- Modify: `src/ast/mod.rs`

**Step 1: Define XML expression types**

Add before the `Expr` enum (before line 705):

```rust
/// XML expression operator type
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum XmlExprOp {
    XmlConcat,
    XmlElement,
    XmlForest,
    XmlParse,
    XmlPi,
    XmlRoot,
    XmlSerialize,
}

/// XML DOCUMENT/CONTENT option
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum XmlOption {
    Document,
    Content,
}

/// A named XML attribute (value AS alias)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct XmlAttribute {
    pub value: Expr,
    pub name: Option<String>,
}
```

**Step 2: Add Expr::XmlExpr variant**

Add to the `Expr` enum (after `Default` at line 760, before the closing `}`):

```rust
    XmlElement {
        entity_escaping: Option<bool>,  // None=default, Some(true)=ENTITYESCAPING, Some(false)=NOENTITYESCAPING
        name_expr: Option<Box<Expr>>,   // EVALNAME expr → Some, NAME ident or bare ident → None (name is in element_name)
        element_name: Option<String>,   // The element name (from NAME ident or bare ident)
        attributes: Option<XmlAttributes>,
        content: Vec<XmlContent>,
    },
```

Wait, actually the design needs to be cleaner. Let me reconsider.

The full syntax is:
```
XMLELEMENT( [ENTITYESCAPING|NOENTITYESCAPING] { [NAME] ident | EVALNAME expr } [, XMLATTRIBUTES(...)] [, content [AS alias]]* )
XMLATTRIBUTES( [ENTITYESCAPING|NOENTITYESCAPING] value [[AS] name | AS EVALNAME expr] [, ...] )
XMLCONCAT( expr, expr, ... )
XMLFOREST( content [AS name] [, ...] )
XMLPARSE( {DOCUMENT|CONTENT} expr [WELLFORMED] )
XMLPI( NAME ident [, content] )
XMLROOT( expr, VERSION expr [, STANDALONE {YES|NO|NO VALUE}] )
XMLSERIALIZE( {DOCUMENT|CONTENT} expr AS type )
```

Here's a cleaner design:

```rust
/// XML attribute list (from XMLATTRIBUTES())
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct XmlAttributes {
    pub entity_escaping: Option<bool>,  // None=default, Some(true)=ENTITYESCAPING, Some(false)=NOENTITYESCAPING
    pub items: Vec<XmlAttribute>,
}

/// A single XML attribute item
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct XmlAttribute {
    pub value: Expr,
    pub name: Option<String>,           // [AS] attname → Some, AS EVALNAME expr not supported as struct field
}

/// XML content item (in XMLELEMENT content list)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct XmlContent {
    pub expr: Expr,
    pub alias: Option<String>,          // [AS] alias
}
```

And the `Expr` variant:

```rust
    XmlElement {
        entity_escaping: Option<bool>,   // None=default, Some(true)=ENTITYESCAPING, Some(false)=NOENTITYESCAPING
        evalname: Option<Box<Expr>>,     // EVALNAME expr → Some, NAME ident → None
        name: Option<String>,            // element name (when NAME or bare ident), None when EVALNAME
        attributes: Option<XmlAttributes>,
        content: Vec<XmlContent>,
    },
    XmlAttributes {
        entity_escaping: Option<bool>,
        items: Vec<XmlAttribute>,
    },
    XmlConcat(Vec<Expr>),
    XmlForest(Vec<XmlContent>),
    XmlParse {
        option: XmlOption,
        expr: Box<Expr>,
        wellformed: bool,
    },
    XmlPi {
        name: Option<String>,
        content: Option<Box<Expr>>,
    },
    XmlRoot {
        expr: Box<Expr>,
        version: Option<Box<Expr>>,
        standalone: Option<Option<bool>>,  // None=no standalone, Some(None)=NO VALUE, Some(Some(true))=YES, Some(Some(false))=NO
    },
    XmlSerialize {
        option: XmlOption,
        expr: Box<Expr>,
        type_name: DataType,
    },
```

**Step 3: Verify**

Run: `cargo build`
Expected: Compiles. (There will be warnings about unused variants, that's fine.)

**Step 4: Commit**

```
feat: add XML AST types (XmlElement, XmlAttributes, XmlConcat, etc.)
```

---

## Task 3: Add XML Parsing in expr.rs

**Files:**
- Modify: `src/parser/expr.rs`

**Step 1: Update imports**

Add new AST types to the import at the top:

```rust
use crate::ast::{
    DataType, Expr, Literal, ObjectName, OrderByItem, SelectStatement, WhenClause, WindowFrame,
    WindowFrameBound, WindowFrameDirection, WindowFrameMode, WindowSpec,
    XmlAttribute, XmlAttributes, XmlContent, XmlOption,
};
```

**Step 2: Add XML parsing dispatch in parse_primary_expr**

In `parse_primary_expr`, in the `Token::Keyword(kw)` arm (around line 381), BEFORE the generic `parse_object_name()` call, add:

```rust
            Token::Keyword(kw) => {
                // XML function special parsing
                if kw == Keyword::XMLELEMENT {
                    self.advance();
                    return self.parse_xml_element();
                }
                if kw == Keyword::XMLCONCAT {
                    self.advance();
                    return self.parse_xml_concat();
                }
                if kw == Keyword::XMLFOREST {
                    self.advance();
                    return self.parse_xml_forest();
                }
                if kw == Keyword::XMLPARSE {
                    self.advance();
                    return self.parse_xml_parse();
                }
                if kw == Keyword::XMLPI {
                    self.advance();
                    return self.parse_xml_pi();
                }
                if kw == Keyword::XMLROOT {
                    self.advance();
                    return self.parse_xml_root();
                }
                if kw == Keyword::XMLSERIALIZE {
                    self.advance();
                    return self.parse_xml_serialize();
                }
                // ... existing CAST handling ...
```

**Step 3: Implement parse_xml_element**

```rust
    fn parse_xml_element(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;

        // Optional ENTITYESCAPING / NOENTITYESCAPING
        let entity_escaping = if self.try_consume_keyword(Keyword::ENTITYESCAPING) {
            Some(true)
        } else if self.try_consume_keyword(Keyword::NOENTITYESCAPING) {
            Some(false)
        } else {
            None
        };

        // Element name: NAME ident | EVALNAME expr | bare_ident | "quoted_ident"
        let (evalname, name) = if self.try_consume_keyword(Keyword::NAME) {
            // NAME identifier
            (None, Some(self.parse_identifier()?))
        } else if self.try_consume_keyword(Keyword::EVALNAME) {
            // EVALNAME expression
            let expr = self.parse_expr()?;
            (Some(Box::new(expr)), None)
        } else {
            // bare identifier or quoted identifier (element name)
            (None, Some(self.parse_identifier()?))
        };

        // Optional: , XMLATTRIBUTES(...)
        let attributes = if self.match_token(&Token::Comma) {
            if self.match_keyword(Keyword::XMLATTRIBUTES) {
                self.advance();
                Some(self.parse_xml_attributes_inner()?)
            } else {
                None
            }
        } else {
            None
        };

        // Remaining args are content items: content [AS alias] [, ...]
        let mut content = Vec::new();
        loop {
            if self.match_token(&Token::RParen) {
                break;
            }
            if content.is_empty() && attributes.is_none() {
                // First content item after element name
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            } else if !content.is_empty() {
                // Subsequent content items
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            } else if content.is_empty() && attributes.is_some() {
                // After attributes
                if !self.match_token(&Token::Comma) {
                    break;
                }
                self.advance();
            }

            let expr = self.parse_expr()?;
            let alias = self.parse_optional_alias()?;
            content.push(XmlContent { expr, alias });
        }

        self.expect_token(&Token::RParen)?;

        Ok(Expr::XmlElement {
            entity_escaping,
            evalname,
            name,
            attributes,
            content,
        })
    }
```

Wait - this logic has a problem with the comma handling after attributes vs content. Let me reconsider.

The grammar is:
```
XMLELEMENT( [escaping] name [, xmlattributes(...) ] [, content_items...] )
```

So the flow is:
1. Parse optional escaping
2. Parse element name
3. If comma → could be XMLATTRIBUTES or first content
4. If was XMLATTRIBUTES and another comma → content items
5. Until RParen

Let me rewrite more carefully:

```rust
    fn parse_xml_element(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;

        // Optional ENTITYESCAPING / NOENTITYESCAPING
        let entity_escaping = if self.try_consume_keyword(Keyword::ENTITYESCAPING) {
            Some(true)
        } else if self.try_consume_keyword(Keyword::NOENTITYESCAPING) {
            Some(false)
        } else {
            None
        };

        // Element name: [NAME] ident | EVALNAME expr
        let (evalname, name) = if self.try_consume_keyword(Keyword::EVALNAME) {
            let expr = self.parse_expr()?;
            (Some(Box::new(expr)), None)
        } else {
            let _ = self.try_consume_keyword(Keyword::NAME); // optional NAME keyword
            (None, Some(self.parse_identifier()?))
        };

        // Parse optional comma-separated items (could be xmlattributes or content)
        let mut attributes: Option<XmlAttributes> = None;
        let mut content: Vec<XmlContent> = Vec::new();

        while self.match_token(&Token::Comma) {
            self.advance();

            // Try XMLATTRIBUTES
            if self.match_keyword(Keyword::XMLATTRIBUTES) && attributes.is_none() {
                self.advance();
                attributes = Some(self.parse_xml_attributes_inner()?);
                continue;
            }

            // Otherwise it's a content expression [AS alias]
            let expr = self.parse_expr()?;
            let alias = self.parse_optional_alias()?;
            content.push(XmlContent { expr, alias });
        }

        self.expect_token(&Token::RParen)?;

        Ok(Expr::XmlElement {
            entity_escaping,
            evalname,
            name,
            attributes,
            content,
        })
    }

    fn parse_xml_attributes_inner(&mut self) -> Result<XmlAttributes, ParserError> {
        self.expect_token(&Token::LParen)?;

        let entity_escaping = if self.try_consume_keyword(Keyword::ENTITYESCAPING) {
            Some(true)
        } else if self.try_consume_keyword(Keyword::NOENTITYESCAPING) {
            Some(false)
        } else {
            None
        };

        let mut items = Vec::new();
        loop {
            let expr = self.parse_expr()?;
            let name = self.parse_optional_alias()?;
            items.push(XmlAttribute { value: expr, name });
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }

        self.expect_token(&Token::RParen)?;
        Ok(XmlAttributes { entity_escaping, items })
    }
```

**Step 4: Implement parse_xml_concat**

```rust
    fn parse_xml_concat(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        let mut args = vec![self.parse_expr()?];
        while self.match_token(&Token::Comma) {
            self.advance();
            args.push(self.parse_expr()?);
        }
        self.expect_token(&Token::RParen)?;
        Ok(Expr::XmlConcat(args))
    }
```

**Step 5: Implement parse_xml_forest**

```rust
    fn parse_xml_forest(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        let mut items = Vec::new();
        loop {
            let expr = self.parse_expr()?;
            let alias = self.parse_optional_alias()?;
            items.push(XmlContent { expr, alias });
            if !self.match_token(&Token::Comma) {
                break;
            }
            self.advance();
        }
        self.expect_token(&Token::RParen)?;
        Ok(Expr::XmlForest(items))
    }
```

**Step 6: Implement parse_xml_parse**

```rust
    fn parse_xml_parse(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        let option = if self.try_consume_keyword(Keyword::DOCUMENT) {
            XmlOption::Document
        } else {
            self.expect_keyword(Keyword::CONTENT)?;
            XmlOption::Content
        };
        let expr = self.parse_expr()?;
        let wellformed = self.try_consume_keyword(Keyword::WELLFORMED);
        self.expect_token(&Token::RParen)?;
        Ok(Expr::XmlParse {
            option,
            expr: Box::new(expr),
            wellformed,
        })
    }
```

**Step 7: Implement parse_xml_pi**

```rust
    fn parse_xml_pi(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        let _ = self.try_consume_keyword(Keyword::NAME); // optional NAME keyword
        let name = self.parse_identifier()?;
        let content = if self.match_token(&Token::Comma) {
            self.advance();
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        self.expect_token(&Token::RParen)?;
        Ok(Expr::XmlPi {
            name: Some(name),
            content,
        })
    }
```

**Step 8: Implement parse_xml_root**

```rust
    fn parse_xml_root(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        let expr = self.parse_expr()?;
        self.expect_token(&Token::Comma)?;
        self.expect_keyword(Keyword::VERSION_P)?;
        let version = if self.try_consume_keyword(Keyword::NO_P) {
            None
        } else {
            Some(Box::new(self.parse_expr()?))
        };
        let standalone = if self.match_token(&Token::Comma) {
            self.advance();
            self.expect_keyword(Keyword::STANDALONE_P)?;
            if self.try_consume_keyword(Keyword::YES_P) {
                Some(Some(true))
            } else if self.try_consume_keyword(Keyword::NO_P) {
                Some(Some(false))
            } else {
                self.expect_keyword(Keyword::NO_P)?;
                // "NO VALUE" - hmm, this is ambiguous. Let me check the grammar.
                // Actually: STANDALONE_P YES_P | STANDALONE_P NO_P | STANDALONE_P NO_P VALUE_P
                unreachable!()
            }
        } else {
            None
        };
        self.expect_token(&Token::RParen)?;
        Ok(Expr::XmlRoot {
            expr: Box::new(expr),
            version,
            standalone,
        })
    }
```

Actually let me reconsider the standalone parsing. From gram.y:
```
opt_xml_root_standalone: ',' STANDALONE_P YES_P      { $$ = INTCONST(1); }
            | ',' STANDALONE_P NO_P            { $$ = INTCONST(0); }
            | ',' STANDALONE_P NO_P VALUE_P     { $$ = INTCONST(-1); }  /* no value */
            | /* EMPTY */                       { $$ = INTCONST(0); }
```

So the options are: YES, NO, NO VALUE, or absent.

```rust
    fn parse_xml_root(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        let expr = self.parse_expr()?;
        self.expect_token(&Token::Comma)?;
        self.expect_keyword(Keyword::VERSION_P)?;
        let version = if self.match_keyword(Keyword::NO_P) {
            // Check if it's "NO VALUE" or standalone "NO"
            // In the VERSION context: "VERSION NO VALUE" means no version
            // But actually in gram.y: xml_root_version: VERSION_P a_expr | VERSION_P NO_P VALUE_P
            // So it could be VERSION expr or VERSION NO VALUE
            self.advance(); // consume NO
            if self.try_consume_keyword(Keyword::VALUE_P) {
                None // VERSION NO VALUE
            } else {
                // VERSION NO → hmm, this shouldn't happen normally
                // Let's treat "NO" as an expression (column ref)
                Some(Box::new(Expr::ColumnRef(vec!["no".to_string()])))
            }
        } else {
            Some(Box::new(self.parse_expr()?))
        };
        let standalone = if self.match_token(&Token::Comma) {
            self.advance();
            self.expect_keyword(Keyword::STANDALONE_P)?;
            if self.try_consume_keyword(Keyword::YES_P) {
                Some(Some(true))
            } else if self.try_consume_keyword(Keyword::NO_P) {
                if self.try_consume_keyword(Keyword::VALUE_P) {
                    Some(None) // NO VALUE
                } else {
                    Some(Some(false)) // NO
                }
            } else {
                None
            }
        } else {
            None
        };
        self.expect_token(&Token::RParen)?;
        Ok(Expr::XmlRoot {
            expr: Box::new(expr),
            version,
            standalone,
        })
    }
```

**Step 9: Implement parse_xml_serialize**

```rust
    fn parse_xml_serialize(&mut self) -> Result<Expr, ParserError> {
        self.expect_token(&Token::LParen)?;
        let option = if self.try_consume_keyword(Keyword::DOCUMENT) {
            XmlOption::Document
        } else {
            self.expect_keyword(Keyword::CONTENT)?;
            XmlOption::Content
        };
        let expr = self.parse_expr()?;
        self.expect_keyword(Keyword::AS)?;
        let type_name = self.parse_data_type()?;
        self.expect_token(&Token::RParen)?;
        Ok(Expr::XmlSerialize {
            option,
            expr: Box::new(expr),
            type_name,
        })
    }
```

**Step 10: Verify**

Run: `cargo build`
Expected: Compiles.

Run with the original failing test:
```
echo "SELECT xmlelement(\" entityescaping <> \", xmlattributes(noentityescaping 'entityescaping<>' \" entityescaping <> \"));" | ./target/debug/ogsql parse
```
Expected: Successfully parses (no error).

**Step 11: Commit**

```
feat: add XML function parsing (XMLELEMENT, XMLATTRIBUTES, XMLCONCAT, XMLFOREST, XMLPARSE, XMLPI, XMLROOT, XMLSERIALIZE)
```

---

## Task 4: Add Formatter Support

**Files:**
- Modify: `src/formatter.rs`

**Step 1: Add XML formatting in format_expr**

In the `format_expr` function, add arms for all new Expr variants before the `_ => {}` catch-all:

```rust
            Expr::XmlElement {
                entity_escaping,
                evalname,
                name,
                attributes,
                content,
            } => {
                let mut result = self.kw("XMLELEMENT") + "(";
                if let Some(true) = entity_escaping {
                    result += &format!(" {} ", self.kw("ENTITYESCAPING"));
                } else if let Some(false) = entity_escaping {
                    result += &format!(" {} ", self.kw("NOENTITYESCAPING"));
                }
                if let Some(expr) = evalname {
                    result += &format!("{} {}", self.kw("EVALNAME"), self.format_expr(expr));
                } else if let Some(n) = name {
                    result += &self.quote_identifier_relaxed(n);
                }
                if let Some(attrs) = attributes {
                    result += &format!(", {}", self.format_xml_attributes(attrs));
                }
                for item in content {
                    result += &format!(", {}", self.format_expr(&item.expr));
                    if let Some(alias) = &item.alias {
                        result += &format!(" {} {}", self.kw("AS"), self.quote_identifier_relaxed(alias));
                    }
                }
                result + ")"
            }
            Expr::XmlAttributes { entity_escaping, items } => {
                let mut result = self.kw("XMLATTRIBUTES") + "(";
                if let Some(true) = entity_escaping {
                    result += &format!("{} ", self.kw("ENTITYESCAPING"));
                } else if let Some(false) = entity_escaping {
                    result += &format!("{} ", self.kw("NOENTITYESCAPING"));
                }
                let parts: Vec<String> = items.iter().map(|a| {
                    let mut s = self.format_expr(&a.value);
                    if let Some(name) = &a.name {
                        s += &format!(" {} {}", self.kw("AS"), self.quote_identifier_relaxed(name));
                    }
                    s
                }).collect();
                result += &parts.join(", ");
                result + ")"
            }
            Expr::XmlConcat(exprs) => {
                format!("{}({})", self.kw("XMLCONCAT"), self.format_exprs(exprs))
            }
            Expr::XmlForest(items) => {
                let parts: Vec<String> = items.iter().map(|item| {
                    let mut s = self.format_expr(&item.expr);
                    if let Some(alias) = &item.alias {
                        s += &format!(" {} {}", self.kw("AS"), self.quote_identifier_relaxed(alias));
                    }
                    s
                }).collect();
                format!("{}({})", self.kw("XMLFOREST"), parts.join(", "))
            }
            Expr::XmlParse { option, expr, wellformed } => {
                let opt_str = match option {
                    XmlOption::Document => self.kw("DOCUMENT"),
                    XmlOption::Content => self.kw("CONTENT"),
                };
                let mut result = format!("{}({} {}", self.kw("XMLPARSE"), opt_str, self.format_expr(expr));
                if *wellformed {
                    result += &format!(" {}", self.kw("WELLFORMED"));
                }
                result + ")"
            }
            Expr::XmlPi { name, content } => {
                let mut result = format!("{}({}", self.kw("XMLPI"), self.kw("NAME"));
                if let Some(n) = name {
                    result += &format!(" {}", self.quote_identifier_relaxed(n));
                }
                if let Some(c) = content {
                    result += &format!(", {}", self.format_expr(c));
                }
                result + ")"
            }
            Expr::XmlRoot { expr, version, standalone } => {
                let mut result = format!("{}({}, {}", self.kw("XMLROOT"), self.format_expr(expr), self.kw("VERSION"));
                if let Some(v) = version {
                    result += &format!(" {}", self.format_expr(v));
                } else {
                    result += &format!(" {} {}", self.kw("NO"), self.kw("VALUE"));
                }
                if let Some(s) = standalone {
                    result += &format!(", {}", self.kw("STANDALONE"));
                    match s {
                        Some(true) => result += &format!(" {}", self.kw("YES")),
                        Some(false) => result += &format!(" {}", self.kw("NO")),
                        None => result += &format!(" {} {}", self.kw("NO"), self.kw("VALUE")),
                    }
                }
                result + ")"
            }
            Expr::XmlSerialize { option, expr, type_name } => {
                let opt_str = match option {
                    XmlOption::Document => self.kw("DOCUMENT"),
                    XmlOption::Content => self.kw("CONTENT"),
                };
                format!("{}({} {} {} {})", self.kw("XMLSERIALIZE"), opt_str, self.format_expr(expr), self.kw("AS"), self.format_data_type(type_name))
            }
```

**Step 2: Add helper format_xml_attributes**

```rust
    fn format_xml_attributes(&self, attrs: &XmlAttributes) -> String {
        let mut result = self.kw("XMLATTRIBUTES") + "(";
        if let Some(true) = attrs.entity_escaping {
            result += &format!("{} ", self.kw("ENTITYESCAPING"));
        } else if let Some(false) = attrs.entity_escaping {
            result += &format!("{} ", self.kw("NOENTITYESCAPING"));
        }
        let parts: Vec<String> = attrs.items.iter().map(|a| {
            let mut s = self.format_expr(&a.value);
            if let Some(name) = &a.name {
                s += &format!(" {} {}", self.kw("AS"), self.quote_identifier_relaxed(name));
            }
            s
        }).collect();
        result += &parts.join(", ");
        result + ")"
    }
```

**Step 3: Verify**

Run: `cargo build`
Expected: Compiles.

Run: `echo "SELECT xmlelement(name foo);" | ./target/debug/ogsql parse`
Run: `echo "SELECT xmlelement(\" entityescaping <> \", xmlattributes(noentityescaping 'entityescaping<>' \" entityescaping <> \"));" | ./target/debug/ogsql parse`
Expected: Both parse and format correctly.

**Step 4: Commit**

```
feat: add XML function formatting
```

---

## Task 5: Add Visitor Support

**Files:**
- Modify: `src/ast/visitor.rs`

**Step 1: Update walk_expr to handle XML variants**

In the `walk_expr` function, add the XML expression handling in the match:

```rust
        Expr::XmlElement { evalname, attributes, content, .. } => {
            if let Some(expr) = evalname {
                if walk_expr(visitor, expr) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
            if let Some(attrs) = attributes {
                for item in &attrs.items {
                    if walk_expr(visitor, &item.value) == VisitorResult::Stop {
                        return VisitorResult::Stop;
                    }
                }
            }
            for item in content {
                if walk_expr(visitor, &item.expr) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::XmlAttributes { items, .. } => {
            for item in items {
                if walk_expr(visitor, &item.value) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::XmlConcat(exprs) => {
            for expr in exprs {
                if walk_expr(visitor, expr) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::XmlForest(items) => {
            for item in items {
                if walk_expr(visitor, &item.expr) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::XmlParse { expr, .. } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
        Expr::XmlPi { content, .. } => {
            if let Some(c) = content {
                if walk_expr(visitor, c) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::XmlRoot { expr, version, .. } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
            if let Some(v) = version {
                if walk_expr(visitor, v) == VisitorResult::Stop {
                    return VisitorResult::Stop;
                }
            }
        }
        Expr::XmlSerialize { expr, .. } => {
            if walk_expr(visitor, expr) == VisitorResult::Stop {
                return VisitorResult::Stop;
            }
        }
```

**Step 2: Verify**

Run: `cargo build`
Expected: Compiles.

**Step 3: Commit**

```
feat: add XML expression visitor support
```

---

## Task 6: Add Unit Tests

**Files:**
- Modify: `src/parser/tests.rs`

**Step 1: Add XML function tests**

Add tests at the end of the file:

```rust
// ========== XML Function Tests ==========

#[test]
fn test_xmlelement_simple() {
    let stmt = parse_one("SELECT xmlelement(name foo)");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
            // verify it's an XmlElement variant
        }
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_xmlelement_with_attributes() {
    let stmt = parse_one("SELECT xmlelement(name foo, xmlattributes('bar' as baz))");
    match stmt {
        Statement::Select(s) => {
            assert_eq!(s.targets.len(), 1);
        }
        _ => panic!("expected SELECT"),
    }
}

#[test]
fn test_xmlelement_noentityescaping() {
    // This is the exact failing case from the bug report
    let sql = r#"SELECT xmlelement(" entityescaping <> ", xmlattributes(noentityescaping 'entityescaping<>' " entityescaping <> "))"#;
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_xmlelement_entityescaping() {
    let sql = r#"SELECT xmlelement(entityescaping "entityescaping<>", 'content')"#;
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_xmlattributes_noentityescaping() {
    let sql = r#"SELECT xmlelement(name foo, xmlattributes(noentityescaping 'bar' as baz))"#;
    let stmts = parse(sql);
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_xmlconcat() {
    let stmts = parse("SELECT xmlconcat(x, y, z)");
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_xmlforest() {
    let stmts = parse("SELECT xmlforest('abc' AS foo, 123 AS bar)");
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_xmlparse() {
    let stmts = parse("SELECT xmlparse(document '<foo>bar</foo>')");
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_xmlparse_wellformed() {
    let stmts = parse("SELECT xmlparse(content '<foo>bar</foo>' wellformed)");
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_xmlpi() {
    let stmts = parse("SELECT xmlpi(name php, 'echo hello')");
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_xmlroot() {
    let stmts = parse("SELECT xmlroot(x, version '1.0', standalone yes)");
    assert_eq!(stmts.len(), 1);
}

#[test]
fn test_xmlserialize() {
    let stmts = parse("SELECT xmlserialize(content x AS text)");
    assert_eq!(stmts.len(), 1);
}
```

**Step 2: Run tests**

Run: `cargo test`
Expected: All tests pass.

**Step 3: Commit**

```
test: add XML function parsing tests
```

---

## Task 7: Verify Regression Tests Pass

**Step 1: Run existing test suite**

Run: `cargo test`
Expected: All existing tests still pass + new XML tests pass.

**Step 2: Run regression tests**

Run: `cargo run --example regression`
Expected: All 1409 regression tests pass (same as before, no regressions).

**Step 3: Test the original bug case**

Run: `echo "SELECT xmlelement(\" entityescaping <> \", xmlattributes(noentityescaping 'entityescaping<>' \" entityescaping <> \"));" | ./target/debug/ogsql parse`
Expected: Successfully parses with correct AST output.

**Step 4: Final commit if any fixes needed**

```
fix: address XML parsing edge cases from regression testing
```
