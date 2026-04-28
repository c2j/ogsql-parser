# AST Semantic Enhancements Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add semantic AST nodes for sequence access (NEXTVAL/CURRVAL), system date (SYSDATE), schema-resolved types (%TYPE), and dynamic SQL templates — enabling precise SQL→Java migration.

**Architecture:** Each feature adds a new Expr variant or analyzer output, following the existing pattern (AST → Parser → Formatter → Visitor → Tests). All features maintain backward compatibility — unrecognized constructs fall back to existing behavior.

**Tech Stack:** Rust, serde, clap, existing ogsql-parser architecture

---

## Task 1: P0 — CLI Integration for transaction_analysis (#49) ✅ DONE

**Status:** Completed. Added 4 lines to `src/bin/ogsql.rs:177-181` calling `analyze_transactions(block)` and inserting into JSON output.

---

## Task 2: P1a — SequenceValue AST Node for NEXTVAL/CURRVAL (#60)

**Files:**
- Modify: `src/ast/mod.rs` (~line 1178 — add SequenceFunc enum, ~line 1317 — add Expr variant)
- Modify: `src/parser/expr.rs` (~line 1122 — detect sequence access in dot-qualified names)
- Modify: `src/formatter.rs` (~line 1360 — add format arm)
- Modify: `src/ast/visitor.rs` (~line 1103 — add walk arm)
- Modify: `src/parser/tests.rs` — add tests

**Key Design Decision:** `seq_name.NEXTVAL` currently parses as `Expr::ColumnRef(["seq_name", "nextval"])`. After change, it becomes `Expr::SequenceValue { sequence: ["seq_name"], function: Nextval }`. Function call syntax `nextval('seq')` is UNCHANGED (remains FunctionCall).

**Steps:**
1. Add `SequenceFunc` enum to `src/ast/mod.rs`
2. Add `Expr::SequenceValue` variant
3. Modify `parse_column_ref_or_qualified_star()` to detect last component being "nextval"/"currval"
4. Add formatter arm
5. Add visitor arm
6. Write tests (SELECT with NEXTVAL, CURRVAL, 3-part name, function call syntax, round-trip)
7. Verify all tests pass

---

## Task 3: P1b — SysDate AST Node for SYSDATE (#61)

**Files:**
- Modify: `src/ast/mod.rs` (~line 1317 — add Expr::SysDate variant)
- Modify: `src/parser/expr.rs` (~line 983-996 — move SYSDATE from ColumnRef to SysDate)
- Modify: `src/formatter.rs` (~line 1360 — add format arm)
- Modify: `src/ast/visitor.rs` (~line 1103 — add walk arm)
- Modify: `src/parser/tests.rs` — add tests

**Key Design Decision:** SYSDATE currently parses as `Expr::ColumnRef(["sysdate"])`. After change, it becomes `Expr::SysDate`. Date arithmetic like `SYSDATE + 1` is handled automatically by BinaryOp since the left operand is now Expr::SysDate instead of ColumnRef. No DateArithmetic node needed yet — that's a future enhancement.

**Steps:**
1. Add `Expr::SysDate` variant to Expr enum
2. In `parse_primary_expr()`, move SYSDATE from the ColumnRef match arm (line 985) to produce `Expr::SysDate` instead
3. Add formatter arm: `Expr::SysDate => "SYSDATE".to_string()`
4. Add visitor arm: `Expr::SysDate => {}`
5. Update existing tests that expect `ColumnRef(["sysdate"])` to expect `SysDate`
6. Add new tests for SYSDATE in various contexts (SELECT, WHERE, PL/pgSQL assignment)
7. Verify all tests pass

---

## Task 4: P2 — --schema-json Parameter for %TYPE Resolution (#62)

**Files:**
- Modify: `src/bin/ogsql.rs` — add `--schema-json` CLI arg
- Create: `src/analyzer/schema.rs` — schema loading + resolution logic
- Modify: `src/analyzer/mod.rs` — export new module
- Modify: `src/lib.rs` — export new public API
- Modify: `src/ast/plpgsql.rs` — (optional) add resolved_type field to PercentType

**Key Design Decision:** Follow the `dynamic_sql_analysis` / `transaction_analysis` pattern — output as a separate analysis report in CLI JSON, NOT inline in AST nodes. This keeps the AST pure.

**Schema JSON Format:**
```json
{
  "table_name": {
    "column_name": "varchar2(200)"
  }
}
```
Keys are lowercase table names, values are column→type maps.

**Output Format:**
```json
{
  "schema_resolution": {
    "resolved_types": [
      {"table": "DB_LOG", "column": "PROC_NAME", "resolved_type": "varchar2(200)"}
    ],
    "unresolved": [
      {"table": "UNKNOWN_TABLE", "column": "COL"}
    ]
  }
}
```

**Steps:**
1. Add `--schema-json <PATH>` arg to CLI (clap)
2. Create `src/analyzer/schema.rs` with `SchemaResolver` struct
3. Implement schema JSON loading (serde deserialize)
4. Implement `resolve_percent_types(block: &PlBlock, schema: &SchemaMap)` → `SchemaResolutionReport`
5. Walk PlBlock AST, find all `PercentType`/`PercentRowType` nodes, resolve against schema
6. Integrate into CLI: load schema, call resolver, attach to JSON output
7. Add tests for resolution logic
8. Verify all tests pass

---

## Task 5: P3 — Dynamic SQL Template Decomposition (#63)

**Files:**
- Modify: `src/analyzer/mod.rs` — add DynamicTemplate struct and extraction logic
- Modify: `src/ast/plpgsql.rs` — (optional) or keep template in analyzer output only

**Key Design Decision:** The `DynamicSqlAnalyzer` already has `TraceChain::Concatenation` that tracks how dynamic SQL strings are built. The template extraction reuses this — decomposing Concatenation into `static_parts` (string literals) and `dynamic_params` (variable references). Additionally, it correlates IF conditions with the concatenation arms to produce `conditions` for MyBatis `<if>` generation.

**DynamicTemplate Structure:**
```json
{
  "dynamic_template": {
    "static_parts": ["SELECT * FROM t WHERE 1=1", " AND status = ", " AND created_at >= "],
    "dynamic_params": [
      {"source": "using_arg", "index": 0, "param_name": "p_status"},
      {"source": "using_arg", "index": 1, "param_name": "p_date_from"}
    ],
    "conditions": [
      {"param": "p_status", "operator": "IS NOT NULL"},
      {"param": "p_date_from", "operator": "IS NOT NULL"}
    ]
  }
}
```

**Steps:**
1. Define `DynamicTemplate`, `StaticPart`, `DynamicParam`, `Condition` structs in analyzer
2. Add `dynamic_template: Option<DynamicTemplate>` to `ExecuteFinding`
3. Implement `extract_template(chain: &TraceChain, block: &PlBlock)` that walks Concatenation chains
4. Detect IF...THEN pattern where condition is `param IS NOT NULL` and body appends to SQL variable
5. Correlate concatenation arms with IF conditions
6. Fallback: return None when template extraction fails (complex cases)
7. Add tests with typical dynamic SQL patterns
8. Verify all tests pass

---

## Execution Order

1. ✅ P0 (#49) — DONE
2. 🔄 P1a (#60) — In progress (delegated)
3. ⏳ P1b (#61) — After P1a completes (shared Expr enum)
4. ⏳ P2 (#62) — After P1b (independent module)
5. ⏳ P3 (#63) — After P2 (builds on analyzer)

## Notes

- Issue #43 was verified as NOT a bug — the reported priority ordering issue doesn't exist in current code
- Each task maintains backward compatibility — fallback behavior unchanged
- All tasks follow the project's existing patterns (AST → Parser → Formatter → Visitor → Tests)
- Tests use `cargo test --features cli`
