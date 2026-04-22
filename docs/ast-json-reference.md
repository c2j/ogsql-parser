# AST JSON Reference

This document describes the JSON structure produced by `ogsql parse -j`.

## Top-Level Structure

```json
{
  "statements": [ ... ],
  "errors": [ ... ]
}
```

- `statements` — array of `StatementInfo` objects, one per SQL statement
- `errors` — array of parse error/warning strings

### StatementInfo

Each element in `statements`:

```json
{
  "Select": { ... },
  "sql_text": "SELECT ...",
  "start_line": 1,
  "start_col": 1,
  "end_line": 1,
  "end_col": 34
}
```

| Field | Type | Description |
|-------|------|-------------|
| Statement variant | object | The actual statement, keyed by variant name (see below) |
| `sql_text` | string | Original SQL text of this statement |
| `start_line` / `start_col` | number | 1-based source position |
| `end_line` / `end_col` | number | 1-based end position |

The statement variant is flattened via `#[serde(flatten)]`, so `Select`, `Insert`, etc. appear as top-level keys alongside `sql_text`.

---

## Serialization Conventions

### Enum Tagging

All Rust enums use **externally tagged** serialization (serde default). This means:

```json
// Unit variant (no data)
"Checkpoint"

// Newtype variant (wraps a struct)
{ "Select": { "targets": [...], "from": [...] } }

// Struct variant (named fields)
{ "BinaryOp": { "left": {...}, "op": "+", "right": {...} } }
```

### ObjectName

`ObjectName` is `Vec<String>` — an array of dot-separated identifier components.

| SQL | JSON |
|-----|------|
| `users` | `["users"]` |
| `public.users` | `["public", "users"]` |
| `db.schema.func` | `["db", "schema", "func"]` |

### Optional Fields

`Option<T>` fields serialize as `null` when absent (not omitted):

```json
{ "alias": null, "where_clause": null }
```

### Post-Processing Fields

Fields prefixed with `_` are injected after parsing and are **not** part of the AST serde model. They are ignored by `json2sql`.

| Field | Location | Description |
|-------|----------|-------------|
| `_meta` | `FunctionCall` | Built-in function metadata (see Function Metadata section) |
| `dynamic_sql_analysis` | `StatementInfo` | Dynamic SQL findings from PL/pgSQL blocks |

---

## Statement Variants

The `Statement` enum has 150+ variants. The most commonly used:

| Variant | SQL | Key Fields |
|---------|-----|------------|
| `Select` | `SELECT ...` | `targets`, `from`, `where_clause`, `group_by`, `order_by`, `limit`, `with` |
| `Insert` | `INSERT INTO ...` | `table`, `columns`, `source` (Values or Query), `on_conflict`, `returning` |
| `Update` | `UPDATE ... SET ...` | `table`, `assignments`, `from`, `where_clause`, `returning` |
| `Delete` | `DELETE FROM ...` | `table`, `where_clause`, `returning` |
| `Merge` | `MERGE INTO ...` | `target`, `source`, `when_matched`, `when_not_matched` |
| `CreateTable` | `CREATE TABLE ...` | `name`, `columns`, `constraints`, `partition_by` |
| `CreateIndex` | `CREATE INDEX ...` | `name`, `table`, `columns`, `unique` |
| `CreateView` | `CREATE VIEW ...` | `name`, `query` |
| `CreateFunction` | `CREATE FUNCTION ...` | `name`, `parameters`, `return_type`, `block` |
| `CreateProcedure` | `CREATE PROCEDURE ...` | `name`, `parameters`, `block` |
| `Do` | `DO $$ ... $$` | `block` (PL/pgSQL AST) |
| `Call` | `CALL proc(...)` | `func_name`, `args` |
| `Drop` | `DROP TABLE/INDEX/...` | `object_type`, `names`, `if_exists`, `cascade` |
| `Transaction` | `BEGIN/COMMIT/ROLLBACK` | `action` |
| `Explain` | `EXPLAIN ...` | `statement`, `analyze`, `verbose`, `format` |
| `VariableSet` | `SET name = value` | `name`, `value` |
| `Grant` / `Revoke` | `GRANT/REVOKE ...` | `privileges`, `targets`, `grantees` |

Full list of all variants: `Select`, `Insert`, `InsertAll`, `InsertFirst`, `Update`, `Delete`, `Merge`, `CreateTable`, `CreateTableAs`, `AlterTable`, `AlterTablespace`, `Drop`, `Truncate`, `CreateIndex`, `CreateGlobalIndex`, `CreateSchema`, `CreateDatabase`, `CreateDatabaseLink`, `CreateTablespace`, `CreateFunction`, `CreateProcedure`, `CreateType`, `AlterIndex`, `CreatePackage`, `CreatePackageBody`, `CreateView`, `CreateMaterializedView`, `CreateSequence`, `CreateTrigger`, `CreateExtension`, `CreateRole`, `CreateUser`, `CreateGroup`, `Grant`, `Revoke`, `Transaction`, `Copy`, `Explain`, `Vacuum`, `VariableSet`, `VariableShow`, `VariableReset`, `Do`, `Call`, `Prepare`, `Execute`, `Deallocate`, `Comment`, `Lock`, `DeclareCursor`, `ClosePortal`, `Fetch`, `Checkpoint`, `Discard`, `Cluster`, `Reindex`, `Listen`, `Notify`, `Unlisten`, `Rule`, `DropRule`, `CreateCast`, `CreateConversion`, `CreateDomain`, `AlterDomain`, `CreateForeignTable`, `CreateForeignServer`, `CreateFdw`, `CreatePublication`, `CreateSubscription`, `CreateSynonym`, `CreateModel`, `CreateAm`, `CreateDirectory`, `CreateNode`, `CreateNodeGroup`, `CreateResourcePool`, `CreateWorkloadGroup`, `CreateAuditPolicy`, `CreateMaskingPolicy`, `CreateRlsPolicy`, `CreateDataSource`, `CreateEvent`, `CreateOpClass`, `CreateOpFamily`, `CreateContQuery`, `CreateStream`, `CreateKey`, `AlterFunction`, `AlterProcedure`, `AlterSchema`, `AlterDatabase`, `AlterRole`, `AlterUser`, `AlterGroup`, `CreateAggregate`, `CreateOperator`, `AlterDefaultPrivileges`, `CreateUserMapping`, `AlterUserMapping`, `DropUserMapping`, `AlterSequence`, `AlterExtension`, `AlterCompositeType`, `AlterView`, `AlterTrigger`, `AlterForeignTable`, `AlterForeignServer`, `AlterFdw`, `AlterPublication`, `AlterSubscription`, `AlterNode`, `AlterNodeGroup`, `AlterResourcePool`, `AlterWorkloadGroup`, `AlterAuditPolicy`, `AlterMaskingPolicy`, `AlterRlsPolicy`, `AlterDataSource`, `AlterEvent`, `AlterOpFamily`, `AlterMaterializedView`, `AlterGlobalConfig`, `RefreshMaterializedView`, `Shutdown`, `Barrier`, `Purge`, `TimeCapsule`, `Snapshot`, `Shrink`, `Verify`, `CleanConn`, `Compile`, `GetDiag`, `ShowEvent`, `AnonyBlock`, `RemovePackage`, `SecLabel`, `CreateWeakPasswordDictionary`, `DropWeakPasswordDictionary`, `CreatePolicyLabel`, `AlterPolicyLabel`, `DropPolicyLabel`, `GrantRole`, `RevokeRole`, `Analyze`, `Abort`, `Values`, `ExecuteDirect`, `AlterSynonym`, `AlterTextSearchConfig`, `AlterTextSearchDict`, `AlterCoordinator`, `AlterAppWorkloadGroupMapping`, `AlterDatabaseLink`, `AlterDirectory`, `AlterLanguage`, `AlterLargeObject`, `AlterPackage`, `AlterSession`, `AlterSystemKillSession`, `CreateLanguage`, `CreateWeakPasswordDictionaryWithValues`, `PredictBy`, `Replace`, `Move`, `LockBuckets`, `MarkBuckets`, `SetSessionAuthorization`, `CreateAppWorkloadGroupMapping`, `DropAppWorkloadGroupMapping`, `CreateTextSearchConfig`, `CreateTextSearchDict`, `AlterTextSearchConfigFull`, `AlterTextSearchDictFull`, `ExpdpDatabase`, `ExpdpTable`, `ImpdpDatabase`, `ImpdpTable`, `ReassignOwned`, `Empty`.

---

## Expression Nodes (Expr)

`Expr` is the core recursive type for all SQL expressions. All variants:

| Variant | SQL Example | JSON Shape |
|---------|-------------|------------|
| `Literal` | `42`, `'hello'`, `TRUE` | `{ "Literal": { "Integer": 42 } }` |
| `ColumnRef` | `col`, `t.col` | `{ "ColumnRef": ["col"] }` or `{ "ColumnRef": ["t", "col"] }` |
| `QualifiedStar` | `t.*` | `{ "QualifiedStar": "t" }` |
| `BinaryOp` | `a + b` | `{ "BinaryOp": { "left": {...}, "op": "+", "right": {...} } }` |
| `UnaryOp` | `-x`, `NOT x` | `{ "UnaryOp": { "op": "-", "expr": {...} } }` |
| `FunctionCall` | `count(x)` | See dedicated section below |
| `Case` | `CASE WHEN ...` | `{ "Case": { "operand": null, "whens": [...], "else_expr": {...} } }` |
| `Between` | `x BETWEEN a AND b` | `{ "Between": { "expr": {...}, "low": {...}, "high": {...}, "negated": false } }` |
| `InList` | `x IN (1,2,3)` | `{ "InList": { "expr": {...}, "list": [...], "negated": false } }` |
| `InSubquery` | `x IN (SELECT ...)` | `{ "InSubquery": { "expr": {...}, "subquery": {...}, "negated": false } }` |
| `Exists` | `EXISTS (SELECT ...)` | `{ "Exists": { "Select": {...} } }` |
| `Subquery` | `(SELECT ...)` | `{ "Subquery": { "Select": {...} } }` |
| `IsNull` | `x IS NULL` | `{ "IsNull": { "expr": {...}, "negated": false } }` |
| `TypeCast` | `x::INT`, `CAST(x AS INT)` | `{ "TypeCast": { "expr": {...}, "type_name": {...}, "default": null, "format": null } }` |
| `Parameter` | `$1` | `{ "Parameter": 1 }` |
| `Array` | `ARRAY[1,2,3]` | `{ "Array": [...] }` |
| `Subscript` | `arr[1]` | `{ "Subscript": { "object": {...}, "index": {...} } }` |
| `Parenthesized` | `(a + b)` | `{ "Parenthesized": {...} }` |
| `RowConstructor` | `(1, 'a', TRUE)` | `{ "RowConstructor": [...] }` |
| `Prior` | `PRIOR x` (hierarchical query) | `{ "Prior": {...} }` |
| `Default` | `DEFAULT` | `"Default"` |
| `SpecialFunction` | `EXTRACT(YEAR FROM d)`, `SUBSTRING(s FROM 1 FOR 3)` | `{ "SpecialFunction": { "name": "extract", "args": [...] } }` |
| `CurrentOf` | `WHERE CURRENT OF cursor` | `{ "CurrentOf": { "cursor_name": "c1" } }` |
| `XmlElement` | `XMLELEMENT(...)` | `{ "XmlElement": { "name": ..., "content": [...] } }` |
| `XmlConcat` | `XMLCONCAT(...)` | `{ "XmlConcat": [...] }` |
| `XmlForest` | `XMLFOREST(...)` | `{ "XmlForest": [...] }` |
| `XmlParse` | `XMLPARSE(...)` | `{ "XmlParse": { "option": "Document", "expr": {...}, "wellformed": false } }` |
| `XmlPi` | `XMLPI(...)` | `{ "XmlPi": { "name": ..., "content": {...} } }` |
| `XmlRoot` | `XMLROOT(...)` | `{ "XmlRoot": { "expr": {...}, "version": {...}, "standalone": null } }` |
| `XmlSerialize` | `XMLSERIALIZE(...)` | `{ "XmlSerialize": { "option": "Content", "expr": {...}, "type_name": {...} } }` |

---

## FunctionCall

The most complex and important expression node:

```json
{
  "FunctionCall": {
    "name": ["count"],
    "args": [{ "ColumnRef": ["*"] }],
    "distinct": false,
    "filter": null,
    "over": null,
    "within_group": [],
    "_meta": {
      "builtin": true,
      "category": "Aggregate",
      "domain": "Aggregate"
    }
  }
}
```

| Field | Type | Description |
|-------|------|-------------|
| `name` | `ObjectName` (string array) | Function name, last element is the function name, preceding elements are schema qualifiers |
| `args` | `Expr[]` | Arguments. `COUNT(*)` has `[{ "ColumnRef": ["*"] }]` |
| `distinct` | boolean | `true` if `DISTINCT` modifier was present |
| `filter` | `Expr \| null` | `FILTER (WHERE ...)` clause |
| `over` | `WindowSpec \| null` | `OVER (...)` or `OVER window_name` |
| `within_group` | `OrderByItem[]` | `WITHIN GROUP (ORDER BY ...)` clause |
| `_meta` | object or absent | **Post-processing field.** Only present for built-in functions |

### Function Metadata (`_meta`)

Only present when the function is recognized as a built-in OpenGauss function. Absent = user-defined function or unknown built-in.

| Field | Type | Values |
|-------|------|--------|
| `builtin` | boolean | Always `true` when present |
| `category` | string | `"Aggregate"`, `"Window"`, `"Scalar"`, `"SetReturning"`, `"Special"` |
| `domain` | string | `"Math"`, `"String"`, `"DateTime"`, `"Aggregate"`, `"Window"`, `"Array"`, `"Json"`, `"Network"`, `"Geometric"`, `"TextSearch"`, `"Crypto"`, `"System"`, `"TypeConversion"`, `"OracleCompat"`, `"Ai"`, `"Other"` |

**How to distinguish built-in vs user-defined:**
- `_meta` exists → built-in function
- `_meta` absent → user-defined function (or built-in but not in registry yet)

### WindowSpec

```json
{
  "window_name": null,
  "partition_by": [{ "ColumnRef": ["dept"] }],
  "order_by": [{ "expr": { "ColumnRef": ["salary"] }, "asc": false, "nulls_first": null }],
  "frame": {
    "mode": "Rows",
    "start": { "direction": "UnboundedPreceding" },
    "end": { "direction": "UnboundedFollowing" }
  }
}
```

When `window_name` is set, it's a reference to a named window (`OVER w`), and other fields are empty.

### SpecialFunction

Functions with keyword-separated syntax (not comma-separated):

| Function | SQL Syntax | `name` value |
|----------|-----------|--------------|
| `EXTRACT` | `EXTRACT(year FROM date_col)` | `"extract"` |
| `SUBSTRING` | `SUBSTRING(str FROM 1 FOR 3)` | `"substring"` |
| `OVERLAY` | `OVERLAY(str PLACING 'x' FROM 2 FOR 1)` | `"overlay"` |
| `POSITION` | `POSITION('x' IN str)` | `"position"` |
| `TRIM` | `TRIM(LEADING ' ' FROM str)` | `"trim"` |

`SpecialFunction` nodes do **not** have `_meta` annotation — they are recognized by the `name` field.

---

## Literal Types

| Variant | SQL | JSON |
|---------|-----|------|
| `Integer` | `42` | `{ "Integer": 42 }` |
| `Float` | `3.14` | `{ "Float": "3.14" }` |
| `String` | `'hello'` | `{ "String": "hello" }` |
| `EscapeString` | `E'\ttext'` | `{ "EscapeString": "\\ttext" }` |
| `BitString` | `B'1010'` | `{ "BitString": "1010" }` |
| `HexString` | `X'FF'` | `{ "HexString": "FF" }` |
| `NationalString` | `N'unicode'` | `{ "NationalString": "unicode" }` |
| `DollarString` | `$$ body $$` or `$tag$ body $tag$` | `{ "DollarString": { "tag": null, "body": " body " } }` |
| `Boolean` | `TRUE`, `FALSE` | `{ "Boolean": true }` |
| `Null` | `NULL` | `"Null"` |

Note: `Float` and `Integer` preserve original representation as strings where precision matters.

---

## DataType

| Variant | SQL | JSON |
|---------|-----|------|
| `Boolean` | `BOOLEAN` | `"Boolean"` |
| `SmallInt` | `SMALLINT` | `"SmallInt"` |
| `Integer` | `INTEGER` | `"Integer"` |
| `BigInt` | `BIGINT` | `"BigInt"` |
| `Real` | `REAL` | `"Real"` |
| `Float` | `FLOAT(24)` | `{ "Float": 24 }` |
| `Double` | `DOUBLE PRECISION` | `"Double"` |
| `Numeric` | `NUMERIC(10,2)` | `{ "Numeric": [10, 2] }` |
| `Char` | `CHAR(10)` | `{ "Char": 10 }` |
| `Varchar` | `VARCHAR(100)` | `{ "Varchar": 100 }` |
| `Text` | `TEXT` | `"Text"` |
| `Timestamp` | `TIMESTAMP(3) WITH TIME ZONE` | `{ "Timestamp": [3, { "WithTimeZone": true }] }` |
| `Date` | `DATE` | `"Date"` |
| `Json` / `Jsonb` | `JSON`, `JSONB` | `"Json"` / `"Jsonb"` |
| `Uuid` | `UUID` | `"Uuid"` |
| `Serial` | `SERIAL` | `"Serial"` |
| `Custom` | any other type | `{ "Custom": [["my_type"], []] }` |

---

## Select Targets

```json
// Expression with optional alias
{ "Expr": [{ "ColumnRef": ["id"] }, "user_id"] }

// * or t.*
{ "Star": "t" }    // qualified: t.*
{ "Star": null }    // unqualified: *
```

---

## Table References (FROM clause)

```json
// Simple table
{ "Table": { "name": ["users"], "alias": "u" } }

// Table function
{ "FunctionCall": { "name": ["generate_series"], "args": [...], "alias": "s" } }

// Subquery
{ "Subquery": { "query": { "Select": {...} }, "alias": "sq" } }

// JOIN
{ "Join": {
    "left": { "Table": { "name": ["users"], "alias": null } },
    "right": { "Table": { "name": ["orders"], "alias": null } },
    "join_type": "Left",
    "condition": { "BinaryOp": { "left": {...}, "op": "=", "right": {...} } },
    "natural": false,
    "using_columns": []
} }

// PIVOT / UNPIVOT
{ "Pivot": { "source": {...}, "pivot": {...} } }
{ "Unpivot": { "source": {...}, "unpivot": {...} } }
```

Join types: `"Inner"`, `"Left"`, `"Right"`, `"Full"`, `"Cross"`.

---

## Set Operations

```json
{ "set_operation": {
    "Union": { "all": false, "right": { "Select": {...} } }
} }
```

Variants: `Union`, `Intersect`, `Except`. Each has `all` (boolean) and `right` (SelectStatement).

---

## Common Patterns

### Identify all function calls

Walk the JSON tree recursively. Any object with a `FunctionCall` key contains a function call node. The `name` array's last element is the function name.

### Check if a function is built-in

```
if (node.FunctionCall._meta && node.FunctionCall._meta.builtin) {
    // Built-in function: category and domain available
    category = node.FunctionCall._meta.category;  // "Aggregate", "Window", "Scalar", ...
    domain = node.FunctionCall._meta.domain;       // "Math", "String", "DateTime", ...
} else {
    // User-defined function (or unrecognized built-in)
}
```

### Find all table references

Walk the JSON tree for `Table` keys inside `TableRef` nodes. The `name` field is an ObjectName array.

### Extract column references

Walk for `ColumnRef` keys in `Expr` nodes. Single-element arrays are unqualified names (`["col"]`), multi-element are qualified (`["table", "col"]`).

### Detect aggregate vs window functions

```
// Aggregate function (no OVER clause)
{ "FunctionCall": { "name": ["sum"], "over": null, "_meta": { "category": "Aggregate" } } }

// Window function (has OVER clause)
{ "FunctionCall": { "name": ["row_number"], "over": { "partition_by": [], "order_by": [...] }, "_meta": { "category": "Window" } } }
```
