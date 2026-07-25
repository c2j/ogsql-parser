# Changelog

All notable changes to ogsql-parser will be documented in this file.

## [0.8.32]

### Added
- Oracle `(+)` outer-join marker preserved in AST for full round-trip fidelity
- `Ident` quote-style preservation (`ObjectName` → `Vec<Ident>`) retains quoted-identifier style across parse/format round-trip
- JDBC `?` placeholder token (`JdbcParam`) for prepared-statement SQL
- iBatis/MyBatis XML mapper parsing with dynamic SQL AST
- Java source SQL extraction (tree-sitter based)
- MCP (Model Context Protocol) server for Claude Desktop, Cursor, etc.
- HTTP API server (`serve` / `serve-minimal` features) with RESTful endpoints
- Interactive TUI playground (`tui` feature)
- SQL anti-pattern linter with 53 rules (4 severity levels)
- Strict validation mode (detect undefined functions in PL blocks)
- MERGE semantic validation (non-deterministic/invalid pattern detection)
- Dynamic SQL analysis with variable tracing and EXECUTE IMMEDIATE resolution
- Schema loading and resolution from JSON
- Return cursor analysis for PL/pgSQL
- PL variable validation
- Query fingerprint computation
- Transaction analysis for PL blocks
- Package consistency validation (PACKAGE vs PACKAGE BODY)
- AST Visitor pattern (walk statements, PL blocks, expressions)
- Windows 7 support via Tier 3 target `x86_64-win7-windows-msvc`
- iBatis callable stored procedure support
- Full PL/pgSQL support (DO blocks, anonymous blocks, control flow, exception handling, GOTO)
- Complete DDL: CREATE/ALTER/DROP for all GaussDB object types
- Two-stage SQL formatter: structured AST pretty-print + configurable token formatter
- Multi-encoding support: UTF-8, EUC-JP, EUC-KR, GB18030, BIG5, UTF-16
- Token-level formatter with FormatConfig (indent, keyword case, comma style, line width)
- JSON serde round-trip (SQL → AST → JSON → AST → SQL)
- Benchmark suite comparing ogsql-parser vs sqlparser-rs, pglast, JSqlParser

### Fixed
- Numerous parser coverage improvements for GaussDB-specific syntax

---

[0.8.32]: https://github.com/c2j/ogsql-parser/releases/tag/v0.8.32
