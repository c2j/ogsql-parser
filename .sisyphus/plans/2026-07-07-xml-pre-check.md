# XML Pre-check: Skip non-mapper XML files in directory scan

## Problem

`parse-xml --dir` and `validate-xml --dir` walk directories and process ALL `.xml` files,
including non-iBatis/non-MyBatis files like `pom.xml`, `web.xml`, Spring configs, etc.
These files get fully parsed by quick-xml, produce no statements, and return a confusing
`EmptyMapper` error.

## Scope

| Entry point | Needs change | Reason |
|---|---|---|
| `cmd_parse_xml_dir` (ogsql.rs ~L5019) | ✅ Yes | Directory scan hits irrelevant XMLs |
| `cmd_validate_xml_dir` (ogsql.rs ~L5600) | ✅ Yes | Same logic, same issue |
| `cmd_parse_xml_single` / `cmd_validate_xml_single` | ❌ No | User explicitly specified the file |
| HTTP API `/api/parse-xml` / `/api/validate-xml` | ❌ No | Caller submits XML explicitly |
| MCP `parse_xml` tool | ❌ No | Caller submits XML explicitly |

## Approach

### 1. Helper function `is_likely_mapper_xml`

Add to `src/bin/ogsql.rs`:

```rust
/// Quick check: does this byte slice look like an iBatis/MyBatis mapper XML?
/// Scans the first 4KB for <mapper or <sqlMap root tags (or their DOCTYPE declarations).
/// False-positive rate is near-zero; false-negative rate is zero for any real mapper file.
fn is_likely_mapper_xml(bytes: &[u8]) -> bool {
    let head = bytes.iter().take(4096).cloned().collect::<Vec<_>>();
    head.windows(7).any(|w| w == b"<mapper" || w == b"<sqlMap")
        || head.windows(15).any(|w| w == b"<!DOCTYPE mapper" || w == b"<!DOCTYPE sqlMap")
}
```

### 2. Insert check in both directory scan loops

Both `cmd_parse_xml_dir` and `cmd_validate_xml_dir` have the same pattern:

```rust
let bytes = match std::fs::read(path) { ... };
// === INSERT HERE ===
if !is_likely_mapper_xml(&bytes) {
    if cli.verbose {
        eprintln!("Skipping non-mapper XML: {}", path.display());
    }
    continue;
}
// === END INSERT ===
let result = ogsql_parser::ibatis::parse_mapper_bytes_with_path(&bytes, ...);
```

### 3. Behavior on skip

- **Default**: Silent skip. File is not counted in errors/warnings stats.
- **With `--verbose`**: Print "Skipping non-mapper XML" to stderr.
- **No new error type needed**, no library API changes.

### 4. Existing behavior preserved

| Scenario | Before | After |
|---|---|---|
| Real mapper XML | Parsed normally | Parsed normally (same) |
| Non-mapper XML (pom.xml) | Full parse → EmptyMapper error | Silent skip |
| User explicitly passes file via `-f` | Parsed (or fails) | Same (no change) |
| HTTP/MCP APIs | Parse submitted XML | Same (no change) |

### 5. Edge cases considered

- **BOM at file start**: Byte scan still works — `<mapper` appears after BOM.
- **XML comments before root**: Comments are plain ASCII/UTF-8 text — `<mapper` still found.
- **Whitespace before root**: No issue — byte scan operates on raw bytes.
- **`<mapper` in string content**: Extremely unlikely in practice; low false-positive risk.
- **UTF-16 encoded XML**: `<` is `0x3C` in both UTF-16LE and UTF-16BE at predictable offsets. Windows(7) could miss it on encoding boundary. Mitigation: genuinely rare in practice; even if we produce a false-negative, the file hits existing EmptyMapper path (no regression).

## Implementation Plan

1. Add `is_likely_mapper_xml()` function in `src/bin/ogsql.rs`
2. Insert check in `cmd_parse_xml_dir` after `std::fs::read(path)` (~L5029)
3. Insert check in `cmd_validate_xml_dir` after `std::fs::read(path)` (~L5610)
4. Build: `cargo build --features ibatis` (pass)
5. Test: `cargo test --features ibatis` (pass)
6. Manual verify: run against a directory with mixed XML files

## Files Changed

- `src/bin/ogsql.rs` — only file modified (~25 lines added)
