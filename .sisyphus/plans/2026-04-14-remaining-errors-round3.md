# Remaining Parser Errors Analysis (Round 3)

## Current Status: 527/737 pass (71.5%), 199 fail, 626 errors

## Top Error Categories (by count)

### Tier 1: High Impact (94 errors, 1 pattern)
1. **RParen>Keyword(FROM) (94 errors, 3 files)**
   - Root cause: Special function syntax `overlay('hello' placing 'world' from 2 for 3)` and `position('ing' in 'string')` — these functions use FROM/IN instead of commas
   - Also: `substring(str from pos for len)` and `trim(leading from str)` patterns
   - Fix: Add special parsing for these SQL standard functions with non-comma argument separators
   - Files: 1021_Hint.sql, 1072_file_1072.sql, 1073_file_1073.sql

### Tier 2: Medium Impact (23-36 errors each)
2. **RParen>Comma (36 errors, 3+ files)**
   - Multi-arg function calls like `to_binary_double('1,2,3', '9,9,9')` where comma appears inside type definitions or complex expressions
   - Some may overlap with the DEFAULT ON CONVERSION ERROR fix not covering all cases

3. **RParen>Keyword(DEFAULT) (30 errors, 2 files)**
   - `1 e2 default 12 on conversion error` — scientific notation with space (`1 e2`) followed by DEFAULT
   - The current fix handles `DEFAULT <expr> ON CONVERSION ERROR` but `1 e2` is parsed as Integer+Ident, not a single expression

4. **ILM policy (23 errors, 3 files)**
   - `ALTER TABLE SET ilm = on` and `CREATE TABLE ... ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION`
   - ILM is a lifecycle management feature — need ALTER TABLE SET ilm support and ILM policy in CREATE TABLE

### Tier 3: Lower Impact (6-14 errors each)
5. **tokenizer:unterminated_string (14 errors, 14 files)**
   - Tokenizer bugs with dollar-quoted strings or special string syntax — likely hard to fix

6. **LParen>Keyword(USING) (14 errors, 3 files)**
   - `CREATE INDEX ... USING gin(...)` — GIN index with USING clause
   - Fix: Update CREATE INDEX parser to handle USING clause properly

7. **identifier>LParen (13 errors)**
   - Function calls in unusual contexts where identifier followed by LParen isn't expected

8. **RParen>Integer(1) (12 errors)**
   - Likely related to `1 e2` scientific notation or `raw(10)` type arguments

9. **SELECT>Keyword(TABLE) (12 errors)**
   - `SELECT TABLE ...` or TABLE SAMPLE syntax

10. **POOL>LABEL + LABEL>POOL (16 errors combined)**
    - Dispatch edge cases in ALTER RESOURCE LABEL/POOL — some dispatch routes still incorrect

11. **SUBPARTITION>Keyword(PARTITION) (10 errors)**
    - Nested partition/subpartition syntax edge cases

12. **ALTER TABLE action: ILM, DISABLE (14 errors)**
    - Missing ALTER TABLE action types

13. **expression operators: #, ~, >, RBracket (36 errors)**
    - Unrecognized operators in expressions — GaussDB-specific operators

## Recommended Fix Priority (Round 3)

1. **RParen>FROM (94 errors)** — Biggest single win, affects 3 files with many SQL statements each
   - Implement special function parsing for: OVERLAY, POSITION, SUBSTRING, TRIM with FROM/IN/PLACING keywords

2. **ILM policy (23+8=31 errors)** — ALTER TABLE ILM + CREATE TABLE ILM ADD POLICY
   - Add ILM support in ALTER TABLE SET action and CREATE TABLE options

3. **RParen>DEFAULT scientific notation (30 errors)** — Fix `1 e2 default 12 on conversion error`
   - The scientific notation `1 e2` is being parsed as Integer(1) + Ident("e2") separately, then DEFAULT doesn't get consumed properly

4. **CREATE INDEX USING (14 errors)** — Add USING clause in CREATE INDEX parser

5. **POOL/LABEL dispatch fixes (16 errors)** — Fix remaining ALTER RESOURCE POOL/LABEL dispatch

6. **Expression operators (36 errors)** — Add #, ~ operators and RBracket handling
