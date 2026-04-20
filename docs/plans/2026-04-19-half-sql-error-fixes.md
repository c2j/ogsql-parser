# Half-SQL Error Fix Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reduce parse errors in GaussDB-2.23.07.210/sql/half-sql.sql from 876 to under 200

**Architecture:** Fix cascading error recovery first (biggest bang), then fix primary parse failures in priority order. The 663 "reserved keyword" errors are mostly cascading from ~5 root-cause failures. The 214 "unexpected token" errors are real unsupported SQL constructs.

**Baseline:** 876 errors (214 unexpected token + 662 reserved keyword cascade)

---

## Task 1: Fix DECLARE-without-BEGIN cascade in find_statement_end_pos

**Files:**
- Modify: `src/parser/mod.rs` — `find_statement_end_pos()` method (~line 244)

**Problem:** `DECLARE` on its own line causes `find_statement_end_pos()` to skip all subsequent semicolons (because `in_declare_section` is true), scanning to EOF. This swallows all subsequent SQL into the PL/pgSQL parser, producing hundreds of cascading "reserved keyword" errors.

**Step 1:** In `find_statement_end_pos()`, after detecting `in_declare_section`, scan ahead to check if BEGIN follows within a reasonable distance. If no BEGIN is found before the next semicolon, treat the DECLARE as a standalone statement (not a block).

**Step 2:** Run `cargo test --lib` to verify no regressions.

**Step 3:** Run validate on half-sql.sql to verify error reduction (~338 errors expected to be eliminated).

---

## Task 2: Fix "reserved keyword" errors from parse_identifier() in real contexts

**Files:**
- Modify: `src/parser/mod.rs` — partition value parsing (MAXVALUE)
- Modify: `src/parser/ddl/create.rs` — CREATE USER role keywords
- Modify: `src/parser/utility/copy_explain.rs` — SET ... TO value parsing

**Problem:** `parse_identifier()` emits non-fatal errors for reserved keywords (MAXVALUE, ON, OFF, TO, etc.) when they're used in positions where keywords should be accepted.

**Step 1:** Find all `parse_identifier()` call sites that encounter reserved keywords and replace with `consume_any_identifier()` where appropriate.

**Step 2:** Test each change individually and run full test suite.

---

## Task 3: Fix composite table alias syntax (AS alias(col type, ...))

**Files:**
- Modify: `src/parser/select.rs` — table reference parsing for function calls

**Problem:** `FROM function_call() AS alias(col1 TYPE, col2 TYPE)` — the `(col1 TYPE, ...)` column definitions for the alias are not consumed by the parser, leaving unconsumed tokens.

**Impact:** This is a major cascade trigger — unconsumed tokens flow into the next statement.

---

## Task 4: Expression operator extensions

**Files:**
- Modify: `src/parser/expr.rs` — `try_postfix_op()` and operator parsing

**Operators to add:**
- `IS DISTINCT FROM` / `IS NOT DISTINCT FROM` (5 errors)
- `<=>` null-safe equality (already partially done?)
- `@@` text search match (2 errors)
- `<^` `>^` box operators (2 errors)
- `-|-` range adjacency (1 error)

---

## Task 5: Named parameters (=>) in function calls

**Files:**
- Modify: `src/parser/expr.rs` — function call argument parsing

**Problem:** `func(param => value)` syntax not supported. `=>` is tokenized as `ParamEquals`.

---

## Task 6: PARTITION/SUBPARTITION in FROM clause

**Files:**
- Modify: `src/parser/select.rs` — `parse_table_ref()` method

**Problem:** `FROM table PARTITION(name)` and `FROM table SUBPARTITION FOR(...)` not supported (17 errors).

---

## Task 7: Remaining statement-level extensions (lower priority)

Categories with 3-6 errors each:
- VARIADIC keyword in expressions (3)
- ALTER TABLE extensions: MODIFY COLUMN, AFTER, wildcard *, SET options (14)
- GaussDB-specific DDL stubs (AUDIT POLICY, MASKING POLICY, DATA SOURCE, SECURITY LABEL, etc.) (~40)
- TEXT SEARCH DICTIONARY (7)
- DATABASE LINK (6)
- TIMECAPSULE (6)
- PREDICT BY (5)
- SET TRANSACTION variants (5)
- ALTER PROCEDURE COMPILE (4)
- USER MAPPING (3)
- And many singleton categories
