# ALTER TABLE Batch 1+2: Sub-actions + ILM/Compress Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add 12 ALTER TABLE sub-actions (Batch 1) and 5 CREATE TABLE / standalone DDL features (Batch 2) to reach ~90% ALTER TABLE coverage and resolve regression test blockers.

**Architecture:** All Batch 1 items follow the identical pattern: add variant to `AlterTableAction` enum in `src/ast/mod.rs`, add match arm in `parse_alter_table_action()` in `src/parser/ddl/alter.rs`, add formatting in `src/formatter.rs`. Batch 2 adds a shared `IlmPolicy` struct for ILM, compress flags in `CreateTableStatement`, and a standalone `AlterTablespaceStatement`.

**Tech Stack:** Rust, tree-sitter (for tests only), clap (CLI not touched).

---

## Task 1: ALTER TABLE DISABLE/ENABLE TRIGGER

**Files:**
- Modify: `src/ast/mod.rs` — `AlterTableAction` enum (line ~263)
- Modify: `src/parser/ddl/alter.rs` — `parse_alter_table_action()` ENABLE_P/DISABLE_P branches (lines 486-503)
- Modify: `src/formatter.rs` — `format_alter_table_action()`

**Step 1: Add AST variants**

In `src/ast/mod.rs`, add to `AlterTableAction` enum after `DisableRowLevelSecurity` (line 264):

```rust
    EnableTrigger {
        name: Option<String>,
    },
    DisableTrigger {
        name: Option<String>,
    },
```

**Step 2: Update parser**

In `src/parser/ddl/alter.rs`, modify the `ENABLE_P` branch (line 486-493). Change from:

```rust
Some(Keyword::ENABLE_P) => {
    self.advance();
    if self.match_keyword(Keyword::ROW) {
        self.advance();
        self.expect_keyword(Keyword::LEVEL)?;
        self.expect_keyword(Keyword::SECURITY)?;
    }
    Ok(AlterTableAction::EnableRowLevelSecurity)
}
```

To:

```rust
Some(Keyword::ENABLE_P) => {
    self.advance();
    if self.match_keyword(Keyword::ROW) {
        self.advance();
        self.expect_keyword(Keyword::LEVEL)?;
        self.expect_keyword(Keyword::SECURITY)?;
        Ok(AlterTableAction::EnableRowLevelSecurity)
    } else if self.match_keyword(Keyword::TRIGGER) {
        self.advance();
        let name = if !self.match_keyword(Keyword::ALL) && !self.match_keyword(Keyword::USER) {
            Some(self.parse_identifier()?)
        } else {
            self.advance();
            None
        };
        Ok(AlterTableAction::EnableTrigger { name })
    } else {
        Ok(AlterTableAction::EnableRowLevelSecurity)
    }
}
```

Apply the same pattern to the `DISABLE_P` branch (line 495-503), adding `DisableTrigger` handling.

**Step 3: Update formatter**

In `src/formatter.rs`, find the `format_alter_table_action` method and add formatting for `EnableTrigger` and `DisableTrigger` variants.

**Step 4: Run tests**

Run: `cargo test`
Expected: All existing tests pass.

---

## Task 2: ALTER TABLE VALIDATE CONSTRAINT

**Files:** Same as Task 1 pattern.

**Step 1: Add AST variant**

```rust
    ValidateConstraint {
        name: String,
    },
```

**Step 2: Update parser**

Add a new arm in `parse_alter_table_action()` for `Keyword::VALIDATE`:

```rust
Some(Keyword::VALIDATE) => {
    self.advance();
    self.expect_keyword(Keyword::CONSTRAINT)?;
    let name = self.parse_identifier()?;
    Ok(AlterTableAction::ValidateConstraint { name })
}
```

**Step 3: Update formatter + tests**

---

## Task 3: ALTER TABLE ADD CONSTRAINT USING INDEX

**Step 1: Add AST variant**

```rust
    AddConstraintUsingIndex {
        name: String,
        index_name: String,
    },
```

**Step 2: Update parser**

In the `ADD_P` branch (line 41), after the existing constraint handling, add check for `UNIQUE ... USING INDEX`:

Inside the `ADD_P => { ... }` block, after existing `AddConstraint` logic, add a check:
When we see `ADD CONSTRAINT name UNIQUE USING INDEX index_name`, produce `AddConstraintUsingIndex`.

**Step 3: Update formatter + tests**

---

## Task 4: ALTER TABLE INHERIT / NO INHERIT

**Step 1: Add AST variants**

```rust
    Inherit {
        parent: ObjectName,
    },
    NoInherit {
        parent: ObjectName,
    },
```

**Step 2: Update parser**

Add arms:
```rust
Some(Keyword::INHERIT) => {
    self.advance();
    let parent = self.parse_object_name()?;
    Ok(AlterTableAction::Inherit { parent })
}
Some(Keyword::NO) => {
    self.advance();
    self.expect_keyword(Keyword::INHERIT)?;
    let parent = self.parse_object_name()?;
    Ok(AlterTableAction::NoInherit { parent })
}
```

Note: `NO` keyword needs careful handling — it's also used in `NO FORCE ROW LEVEL SECURITY` (Task 10). Use a peek-ahead or nested match.

---

## Task 5: ALTER TABLE CLUSTER ON / SET WITHOUT CLUSTER

**Step 1: Add AST variants**

```rust
    ClusterOn {
        index_name: String,
    },
    SetWithoutCluster,
```

**Step 2: Update parser**

In the `SET` branch (line 239), add after `SET WITHOUT OIDS`:
```rust
} else if self.match_keyword(Keyword::WITHOUT_P) {
    self.advance();
    if self.match_keyword(Keyword::CLUSTER) {
        self.advance();
        return Ok(AlterTableAction::SetWithoutCluster);
    } else {
        self.expect_keyword(Keyword::OIDS)?;
        return Ok(AlterTableAction::SetWithoutOids);
    }
}
```

Add a new arm for `Keyword::CLUSTER`:
```rust
Some(Keyword::CLUSTER) => {
    self.advance();
    self.expect_keyword(Keyword::ON)?;
    let index_name = self.parse_identifier()?;
    Ok(AlterTableAction::ClusterOn { index_name })
}
```

---

## Task 6: ALTER TABLE REPLICA IDENTITY

**Step 1: Add AST type + variant**

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ReplicaIdentity {
    Default,
    Nothing,
    Full,
    Index { name: String },
}
```

Add to `AlterTableAction`:
```rust
    ReplicaIdentity(ReplicaIdentity),
```

**Step 2: Parser**

Add arm for `Keyword::REPLICA`:
```rust
Some(Keyword::REPLICA) => {
    self.advance();
    self.expect_ident_str("IDENTITY")?;
    let identity = if self.match_keyword(Keyword::DEFAULT) {
        self.advance();
        ReplicaIdentity::Default
    } else if self.match_ident_str("NOTHING") {
        self.advance();
        ReplicaIdentity::Nothing
    } else if self.match_keyword(Keyword::FULL) {
        self.advance();
        ReplicaIdentity::Full
    } else if self.match_keyword(Keyword::USING) {
        self.advance();
        self.expect_keyword(Keyword::INDEX)?;
        let name = self.parse_identifier()?;
        ReplicaIdentity::Index { name }
    } else {
        return Err(ParserError::UnexpectedToken { ... });
    };
    Ok(AlterTableAction::ReplicaIdentity(identity))
}
```

---

## Task 7: ALTER TABLE SET COMPRESS / NOCOMPRESS

**Step 1: Add AST variant**

```rust
    SetCompress,
    SetNoCompress,
```

**Step 2: Parser**

In the `SET` branch, add:
```rust
} else if self.match_keyword(Keyword::COMPRESS) {
    self.advance();
    return Ok(AlterTableAction::SetCompress);
}
```

Add `NOCOMPRESS` handling in a `NO` branch or in the catch-all.

---

## Task 8: ALTER TABLE MODIFY (MySQL compat)

**Step 1: Add AST variant**

```rust
    ModifyColumn(ColumnDef),
```

**Step 2: Parser**

The `MODIFY_P` keyword is already handled at line 470 but produces `AlterColumn`. Need to check if it should produce a new `ModifyColumn` variant instead for MySQL compat, where `MODIFY col_name data_type [constraints]` redefines the column entirely.

For now, the existing `MODIFY_P` handling at line 470 already parses `MODIFY column_name data_type`. If it needs to parse full column constraints (not just type/null), enhance the existing branch to parse `parse_column_def()` instead of just type.

---

## Task 9: ALTER TABLE FORCE / NO FORCE ROW LEVEL SECURITY

**Step 1: Add AST variants**

```rust
    ForceRowLevelSecurity,
    NoForceRowLevelSecurity,
```

**Step 2: Parser**

Add `FORCE` keyword handling and `NO FORCE` handling:
```rust
// In FORCE branch:
Some(Keyword::FORCE) => {
    self.advance();
    self.expect_keyword(Keyword::ROW)?;
    self.expect_keyword(Keyword::LEVEL)?;
    self.expect_keyword(Keyword::SECURITY)?;
    Ok(AlterTableAction::ForceRowLevelSecurity)
}
```

For `NO FORCE`, extend the `NO` keyword handling from Task 4.

---

## Task 10: ALTER TABLE OF type_name / NOT OF

**Step 1: Add AST variants**

```rust
    OfType {
        type_name: ObjectName,
    },
    NotOfType {
        type_name: ObjectName,
    },
```

**Step 2: Parser**

```rust
Some(Keyword::OF) => {
    self.advance();
    let type_name = self.parse_object_name()?;
    // Distinguish from "OF type_name" (ALTER TABLE) vs "OF" in other contexts
    Ok(AlterTableAction::OfType { type_name })
}
```

For `NOT OF`, handle in the `NO`/`NOT` branch.

---

## Task 11: ALTER TABLE ADD/DELETE NODE

**Step 1: Add AST variants**

```rust
    AddNode {
        node_name: String,
    },
    DeleteNode {
        node_name: String,
    },
```

**Step 2: Parser**

In the `ADD_P` branch, after existing checks for PARTITION/SUBPARTITION/COLUMN/CONSTRAINT:
```rust
} else if self.match_keyword(Keyword::NODE) {
    self.advance();
    let node_name = self.parse_identifier()?;
    Ok(AlterTableAction::AddNode { node_name })
}
```

In the `DROP` branch, similarly add NODE handling.

---

## Task 12: ALTER TABLE COMMENT

**Step 1: Add AST variant**

```rust
    SetComment {
        comment: String,
    },
```

**Step 2: Parser**

Add arm for `Keyword::COMMENT`:
```rust
Some(Keyword::COMMENT) => {
    self.advance();
    if self.match_token(&Token::Eq) {
        self.advance();
    }
    let comment = self.parse_string_literal()?;
    Ok(AlterTableAction::SetComment { comment })
}
```

---

## Task 13: CREATE TABLE ILM ADD POLICY (Batch 2)

**Files:**
- Modify: `src/ast/mod.rs` — Add `IlmPolicy` struct + field in `CreateTableStatement`
- Modify: `src/parser/ddl/table.rs` — Parse ILM clause after table options
- Modify: `src/formatter.rs`

**Step 1: Add IlmPolicy struct**

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IlmPolicy {
    pub after_n: i64,
    pub unit: String,      // "day" | "month" | "year"
    pub condition: Option<Expr>,
}
```

Add field to `CreateTableStatement`:
```rust
pub ilm: Option<IlmPolicy>,
```

**Step 2: Parser**

In CREATE TABLE parsing (`src/parser/ddl/table.rs`), after COMPRESS/NOCOMPRESS handling, add ILM detection:

```rust
if self.match_ident_str("ILM") {
    self.advance();
    self.expect_keyword(Keyword::ADD_P)?;
    self.expect_ident_str("POLICY")?;
    // consume "ROW STORE COMPRESS ADVANCED ROW"
    while !self.match_keyword(Keyword::AFTER) && !self.peek_is_eof() {
        self.advance();
    }
    self.expect_keyword(Keyword::AFTER)?;
    let after_n = self.parse_integer()?.parse::<i64>().unwrap_or(0);
    let unit = self.parse_identifier()?;
    self.expect_keyword(Keyword::OF)?;
    // consume "NO MODIFICATION"
    self.advance(); // NO
    self.advance(); // MODIFICATION
    let condition = if self.match_keyword(Keyword::ON) {
        self.advance();
        self.expect_token(&Token::LParen)?;
        let expr = self.parse_expr()?;
        self.expect_token(&Token::RParen)?;
        Some(expr)
    } else {
        None
    };
    ilm = Some(IlmPolicy { after_n, unit, condition });
}
```

---

## Task 14: ALTER TABLE ILM ADD POLICY (Batch 2)

**Step 1: Add AST variant**

```rust
    IlmAddPolicy(IlmPolicy),
    IlmEnablePolicy,
    IlmDisablePolicy,
    IlmDeletePolicy,
```

**Step 2: Parser**

Add arm for `match_ident_str("ILM")`:
```rust
_ if self.peek_ident_str("ILM") => {
    self.advance();
    if self.match_ident_str("ENABLE") {
        self.advance();
        self.expect_ident_str("POLICY")?;
        Ok(AlterTableAction::IlmEnablePolicy)
    } else if self.match_ident_str("DISABLE") {
        self.advance();
        self.expect_ident_str("POLICY")?;
        Ok(AlterTableAction::IlmDisablePolicy)
    } else if self.match_ident_str("DELETE") {
        self.advance();
        self.expect_ident_str("POLICY")?;
        Ok(AlterTableAction::IlmDeletePolicy)
    } else if self.match_keyword(Keyword::ADD_P) {
        self.advance();
        self.expect_ident_str("POLICY")?;
        // parse ILM policy details (same as Task 13)
        Ok(AlterTableAction::IlmAddPolicy(ilm_policy))
    }
}
```

---

## Task 15: CREATE TABLE compress_mode + COMPRESS/NOCOMPRESS (Batch 2)

**Step 1: Add fields to CreateTableStatement**

```rust
pub compress: Option<bool>,  // Some(true) = COMPRESS, Some(false) = NOCOMPRESS, None = default
```

Add `compress_mode` to `ColumnDef`:
```rust
pub compress_mode: Option<String>,  // DELTA/PREFIX/DICTIONARY/NUMSTR/NOCOMPRESS
```

**Step 2: Parser**

In CREATE TABLE column definition parsing, after data type, check for compress_mode keywords:
```rust
let compress_mode = if self.match_ident_str("DELTA") || self.match_ident_str("PREFIX")
    || self.match_ident_str("DICTIONARY") || self.match_ident_str("NUMSTR")
    || self.match_ident_str("NOCOMPRESS") {
    let mode = self.node_text(...);
    self.advance();
    Some(mode)
} else {
    None
};
```

In CREATE TABLE body, after WITH clause, check for COMPRESS/NOCOMPRESS:
```rust
if self.match_keyword(Keyword::COMPRESS) {
    self.advance();
    compress = Some(true);
} else if self.match_keyword(Keyword::NOCOMPRESS) {
    self.advance();
    compress = Some(false);
}
```

---

## Task 16: ALTER TABLESPACE (Batch 2)

**Files:**
- Modify: `src/ast/mod.rs` — Add `AlterTablespaceStatement` (promote from stub or add new)
- Modify: `src/parser/mod.rs` — Add ALTER TABLESPACE dispatch
- Modify: `src/parser/ddl/create.rs` or new file — Parse ALTER TABLESPACE
- Modify: `src/formatter.rs`

**Step 1: Add AST**

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterTablespaceStatement {
    pub name: String,
    pub action: AlterTablespaceAction,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlterTablespaceAction {
    RenameTo { new_name: String },
    OwnerTo { new_owner: String },
    SetOptions { options: Vec<(String, String)> },
    ResetOptions { options: Vec<String> },
}
```

Add to `Statement` enum: `AlterTablespace(AlterTablespaceStatement)`.

**Step 2: Parser dispatch**

In `parse_alter()` in `src/parser/mod.rs`, add:
```rust
Some(Keyword::TABLESPACE) => {
    self.advance();
    self.parse_alter_tablespace().map(Statement::AlterTablespace)
}
```

**Step 3: Implement `parse_alter_tablespace()`**

GaussDB syntax:
```
ALTER TABLESPACE tablespace_name RENAME TO new_name;
ALTER TABLESPACE tablespace_name OWNER TO new_owner;
ALTER TABLESPACE tablespace_name SET (option = value [, ...]);
ALTER TABLESPACE tablespace_name RESET (option [, ...]);
```

---

## Implementation Notes

### Keyword Availability Check

Before implementing each task, verify the keyword exists in `src/token/keyword.rs`. Keywords needed:
- `VALIDATE` — check if exists, may need `match_ident_str("VALIDATE")`
- `CLUSTER` — exists as `Keyword::CLUSTER`
- `REPLICA` — check if exists
- `COMMENT` — exists as `Keyword::COMMENT`
- `ILM` — non-reserved, use `match_ident_str("ILM")`
- `COMPRESS` / `NOCOMPRESS` — exist in keyword.rs

### Formatter Pattern

All new `AlterTableAction` variants need formatting in `format_alter_table_action()`. Follow the existing pattern in `src/formatter.rs`.

### Testing

For each task, add unit tests in `src/parser/tests.rs`:
```rust
#[test]
fn test_alter_table_enable_trigger() {
    let sql = "ALTER TABLE t ENABLE TRIGGER trg_name";
    let result = parse_sql(sql);
    assert!(result.is_ok());
}
```

### Execution Order

Tasks 1-12 (Batch 1) can be parallelized in groups:
- **Group A** (independent keywords): Tasks 1, 2, 5, 6, 7, 10, 11, 12
- **Group B** (shares `NO`/`NOT` keyword): Tasks 4, 9, 10 — must be coordinated
- **Group C** (complex): Tasks 3, 8

Tasks 13-16 (Batch 2) depend on ILM design from Task 13.
