# Oracle 兼容包函数 Domain 扩展实施计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 在 `function_registry` 中新增 16 个 `FuncDomain` 变体，注册 ~80 个 Oracle 兼容包函数（dbe_\*/dbms_\*/utl_\*/xml/pkg_service），使 `Expr::FunctionCall.builtin` 携带精确的 domain 信息，供下游 cobweb 项目做迁移兼容性分析。

**Architecture:** 采用两阶段查找策略——先尝试全限定名精确匹配（如 `"dbe_lob.append"`），再 fallback 到最后段名匹配（如 `"append"`），保持向后兼容。包函数使用全限定名注册到 `FUNCTIONS` 数组，新增 `fop!` 宏简化 Oracle 包函数注册。所有改动集中在 `function_registry.rs` 和 `expr.rs` 两个文件。

**Tech Stack:** Rust，现有 parser 基础设施（FuncMeta、CompatMode、ParserError::Warning）。

---

## 当前状态速览

| 维度 | 现状 |
|---|---|
| `FuncDomain` 变体数 | 16 个（Math, String, DateTime, Aggregate, Window, Array, Json, Network, Geometric, TextSearch, Crypto, System, TypeConversion, OracleCompat, Ai, Other） |
| `OracleCompat` 函数数 | ~20 个（add_months, decode, nvl, nvl2, last_day, months_between, next_day, nls_\*, nlssort, rownum, sysdate, instr, listagg, substrb, translate, wm_concat, group_concat） |
| 注册机制 | `FUNCTIONS` 静态数组 + 二分查找（按小写名字母序） |
| 查找逻辑 | `expr.rs:validate_func` 取 `ObjectName.last()` 再 `split('.').last()` → 只匹配最后段名 |
| 影响范围 | `FuncDomain` 仅在 `function_registry.rs` 中使用；`BuiltinFuncMeta` 在 `ast/mod.rs` 定义（category/domain 为 String，无需改动） |

---

## Task 1: 扩展 FuncDomain 枚举

**Files:**
- Modify: `src/parser/function_registry.rs:21-39`（FuncDomain 枚举定义）
- Modify: `src/parser/function_registry.rs:1864-1881`（lookup_builtin_meta 中 match 分支）

**Why:** 新增 16 个 domain 变体，使包函数有独立的分类标识。

**Step 1: 扩展 FuncDomain 枚举**

在 `FuncDomain::OracleCompat` 之后、`Ai` 之前，按字母序插入新变体：

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FuncDomain {
    Math,
    String,
    DateTime,
    Aggregate,
    Window,
    Array,
    Json,
    Network,
    Geometric,
    TextSearch,
    Crypto,
    System,
    TypeConversion,
    OracleCompat,
    // ── Oracle 兼容包函数域 ──
    DbeFile,
    DbeLob,
    DbeOutput,
    DbeScheduler,
    DbeSession,
    DbeSql,
    DbeStats,
    DbeUtility,
    DbmsLob,
    DbmsOutput,
    DbmsScheduler,
    DbmsSql,
    DbmsUtility,
    PkgService,
    UtlFile,
    Xml,
    // ── 其他 ──
    Ai,
    Other,
}
```

> **注意：** 变体顺序不影响功能，但按字母序排列便于维护。现有 `OracleCompat` 保留给非包函数（nvl, decode 等）。

**Step 2: 更新 lookup_builtin_meta 的 match 分支**

在 `lookup_builtin_meta` 函数（约 line 1864）的 `match m.domain` 中，在 `OracleCompat` 之后、`Ai` 之前插入：

```rust
FuncDomain::DbeFile => "DbeFile",
FuncDomain::DbeLob => "DbeLob",
FuncDomain::DbeOutput => "DbeOutput",
FuncDomain::DbeScheduler => "DbeScheduler",
FuncDomain::DbeSession => "DbeSession",
FuncDomain::DbeSql => "DbeSql",
FuncDomain::DbeStats => "DbeStats",
FuncDomain::DbeUtility => "DbeUtility",
FuncDomain::DbmsLob => "DbmsLob",
FuncDomain::DbmsOutput => "DbmsOutput",
FuncDomain::DbmsScheduler => "DbmsScheduler",
FuncDomain::DbmsSql => "DbmsSql",
FuncDomain::DbmsUtility => "DbmsUtility",
FuncDomain::PkgService => "PkgService",
FuncDomain::UtlFile => "UtlFile",
FuncDomain::Xml => "Xml",
```

**Step 3: 验证编译**

Run: `cargo build 2>&1 | head -30`
Expected: 编译通过（无其他代码引用新增的枚举变体，不会报错）。

**Step 4: Commit**

```bash
git add src/parser/function_registry.rs
git commit -m "feat(registry): add 16 FuncDomain variants for Oracle package functions"
```

---

## Task 2: 新增 fop! 宏用于 Oracle 包函数注册

**Files:**
- Modify: `src/parser/function_registry.rs:277-289`（在 fo! 宏之后）

**Why:** 包函数使用全限定名注册（如 `"dbe_lob.append"`），需要专用宏。与 `f!`（通用）和 `fo!`（Oracle 兼容非包函数）区分开。

**Step 1: 在 fo! 宏定义之后新增 fop! 宏**

在 `fo!` 宏之后（约 line 289）插入：

```rust
macro_rules! fop {
    ($name:expr, $cat:expr, $dom:expr, $min:expr, $max:expr, $dist:expr) => {
        FuncMeta {
            name: $name,
            category: $cat,
            domain: $dom,
            min_args: $min,
            max_args: $max,
            supports_distinct: $dist,
            compat: ORACLE_COMPAT,
        }
    };
}
```

> **说明：** `fop!` 与 `fo!` 功能相同（都使用 `ORACLE_COMPAT` 模式），但独立出来是为了语义清晰——`fop` 表示 "function, oracle package"。如果未来包函数需要额外的 compat 标记（如仅 A_FORMAT），可以只改 `fop!` 而不影响 `fo!`。

**Step 2: 验证编译**

Run: `cargo build`
Expected: 编译通过。

**Step 3: Commit**

```bash
git add src/parser/function_registry.rs
git commit -m "feat(registry): add fop! macro for Oracle package function registration"
```

---

## Task 3: 改造查找逻辑支持两阶段匹配

**Files:**
- Modify: `src/parser/function_registry.rs:1844-1853`（lookup_function）
- Modify: `src/parser/expr.rs:11-33`（validate_func）

**Why:** 当前查找只匹配函数名的最后一段（`append`），无法区分 `dbe_lob.append` 和 `dbms_lob.append`。需要改造为：先全限定名精确匹配，再 fallback 到最后段名。

**Step 1: 新增 lookup_function_qualified 函数**

在 `lookup_function` 之后（约 line 1853）新增：

```rust
/// Look up a built-in function by name, supporting two-phase matching:
/// 1. Exact match on the full name (e.g., "dbe_lob.append")
/// 2. Fallback to last segment (e.g., "append" from "dbe_lob.append")
pub fn lookup_function_qualified(full_name: &str) -> Option<&'static FuncMeta> {
    // Phase 1: try exact full qualified name
    let lower = full_name.to_ascii_lowercase();
    let idx = FUNCTIONS.partition_point(|m| m.name < lower.as_str());
    if idx < FUNCTIONS.len() && FUNCTIONS[idx].name == lower {
        return Some(&FUNCTIONS[idx]);
    }
    // Phase 2: fallback to last segment
    let last_seg = lower.split('.').last().unwrap_or(&lower);
    // Avoid redundant lookup if no dot in name
    if last_seg.len() == lower.len() {
        return None;
    }
    let idx2 = FUNCTIONS.partition_point(|m| m.name < last_seg);
    if idx2 < FUNCTIONS.len() && FUNCTIONS[idx2].name == last_seg {
        return Some(&FUNCTIONS[idx2]);
    }
    None
}
```

**Step 2: 新增 lookup_builtin_meta_qualified 函数**

在 `lookup_builtin_meta` 之后新增：

```rust
/// Qualified variant of `lookup_builtin_meta` that supports package-qualified names.
pub fn lookup_builtin_meta_qualified(full_name: &str) -> Option<crate::ast::BuiltinFuncMeta> {
    lookup_function_qualified(full_name).map(|m| crate::ast::BuiltinFuncMeta {
        category: match m.category {
            FuncCategory::Aggregate => "Aggregate",
            FuncCategory::Window => "Window",
            FuncCategory::Scalar => "Scalar",
            FuncCategory::SetReturning => "SetReturning",
            FuncCategory::Special => "Special",
        }.to_string(),
        domain: match m.domain {
            FuncDomain::Math => "Math",
            FuncDomain::String => "String",
            FuncDomain::DateTime => "DateTime",
            FuncDomain::Aggregate => "Aggregate",
            FuncDomain::Window => "Window",
            FuncDomain::Array => "Array",
            FuncDomain::Json => "Json",
            FuncDomain::Network => "Network",
            FuncDomain::Geometric => "Geometric",
            FuncDomain::TextSearch => "TextSearch",
            FuncDomain::Crypto => "Crypto",
            FuncDomain::System => "System",
            FuncDomain::TypeConversion => "TypeConversion",
            FuncDomain::OracleCompat => "OracleCompat",
            FuncDomain::DbeFile => "DbeFile",
            FuncDomain::DbeLob => "DbeLob",
            FuncDomain::DbeOutput => "DbeOutput",
            FuncDomain::DbeScheduler => "DbeScheduler",
            FuncDomain::DbeSession => "DbeSession",
            FuncDomain::DbeSql => "DbeSql",
            FuncDomain::DbeStats => "DbeStats",
            FuncDomain::DbeUtility => "DbeUtility",
            FuncDomain::DbmsLob => "DbmsLob",
            FuncDomain::DbmsOutput => "DbmsOutput",
            FuncDomain::DbmsScheduler => "DbmsScheduler",
            FuncDomain::DbmsSql => "DbmsSql",
            FuncDomain::DbmsUtility => "DbmsUtility",
            FuncDomain::PkgService => "PkgService",
            FuncDomain::UtlFile => "UtlFile",
            FuncDomain::Xml => "Xml",
            FuncDomain::Ai => "Ai",
            FuncDomain::Other => "Other",
        }.to_string(),
    })
}
```

**Step 3: 修改 validate_func 使用全限定名查找**

修改 `src/parser/expr.rs` 中 `validate_func` 函数（约 line 11-33）：

```rust
fn validate_func(
    &mut self,
    name: &ObjectName,
    arg_count: usize,
    distinct: bool,
    has_over: bool,
    has_variadic: bool,
) -> Option<crate::ast::BuiltinFuncMeta> {
    // Construct full qualified name from ObjectName parts
    let full_name = name.0.iter()
        .map(|i| i.value.to_lowercase())
        .collect::<Vec<_>>()
        .join(".");
    let last_seg = full_name.split('.').last().unwrap_or(&full_name).to_string();

    // Try qualified lookup first (e.g., "dbe_lob.append"), then fallback to last segment
    let builtin = crate::parser::function_registry::lookup_builtin_meta_qualified(&full_name)
        .or_else(|| {
            crate::parser::function_registry::lookup_builtin_meta(&last_seg)
        });

    let warnings = crate::parser::function_registry::validate_function_call(
        &last_seg,
        arg_count,
        distinct,
        has_over,
        has_variadic,
        self.current_location(),
    );
    for w in warnings {
        self.add_error(w);
    }
    builtin
}
```

> **关键设计决策：** `validate_function_call` 仍使用 last segment 查找验证（因为参数校验与包前缀无关），只有 `builtin` 元数据查找使用全限定名。这确保了向后兼容——已有的非包函数（如 `nvl`, `decode`）查找路径不变。

**Step 4: 编写两阶段查找的测试**

在 `src/parser/function_registry.rs` 的 `mod tests` 中新增：

```rust
#[test]
fn test_lookup_qualified_dbe_lob_append() {
    // 全限定名精确匹配
    let meta = crate::parser::function_registry::lookup_function_qualified("dbe_lob.append");
    assert!(meta.is_some(), "dbe_lob.append should be found via qualified lookup");
    let m = meta.unwrap();
    assert_eq!(m.domain, FuncDomain::DbeLob);
    assert_eq!(m.category, FuncCategory::Scalar);
}

#[test]
fn test_lookup_qualified_fallback_to_last_segment() {
    // 非全限定名 fallback 到 last segment
    let meta = crate::parser::function_registry::lookup_function_qualified("some_schema.upper");
    assert!(meta.is_some(), "should fallback to lookup 'upper'");
    let m = meta.unwrap();
    assert_eq!(m.domain, FuncDomain::String);
}

#[test]
fn test_lookup_qualified_unknown() {
    let meta = crate::parser::function_registry::lookup_function_qualified("nonexistent_pkg.nonexistent_func");
    assert!(meta.is_none());
}

#[test]
fn test_lookup_qualified_no_dot() {
    // 无点号时应返回 None（因为 lookup_function 已处理无点号的情况）
    let meta = crate::parser::function_registry::lookup_function_qualified("upper");
    // "upper" 无点号，Phase 1 精确匹配到 "upper"，返回 Some
    assert!(meta.is_some());
}
```

**Step 5: 验证编译**

Run: `cargo test test_lookup_qualified_ 2>&1 | tail -20`
Expected: 编译通过（测试会 FAIL 因为函数条目尚未注册，这正常）。

**Step 6: Commit**

```bash
git add src/parser/function_registry.rs src/parser/expr.rs
git commit -m "feat(registry): add two-phase qualified lookup for package functions"
```

---

## Task 4: 注册 XML 域函数

**Files:**
- Modify: `src/parser/function_registry.rs`（FUNCTIONS 数组，按字母序插入）

**Why:** XML 函数（xmlelement, xmlforest 等）在 openGauss 中广泛使用，且解析器已有 xmlelement 的专门测试。

**Step 1: 在 FUNCTIONS 数组中注册 XML 函数**

在 `FUNCTIONS` 数组中，按字母序找到正确位置（`x` 开头在数组末尾），在 `wm_concat` 之后、数组结束 `];` 之前插入：

```rust
    // ── X ───────────────────────────────────────────────────────
    fop!(
        "xmlagg",
        FuncCategory::Aggregate,
        FuncDomain::Xml,
        1,
        Some(1),
        true
    ),
    fop!(
        "xmlattributes",
        FuncCategory::Scalar,
        FuncDomain::Xml,
        1,
        None,
        false
    ),
    fop!(
        "xmlcomment",
        FuncCategory::Scalar,
        FuncDomain::Xml,
        1,
        Some(1),
        false
    ),
    fop!(
        "xmlconcat",
        FuncCategory::Scalar,
        FuncDomain::Xml,
        1,
        None,
        false
    ),
    fop!(
        "xmlelement",
        FuncCategory::Scalar,
        FuncDomain::Xml,
        1,
        None,
        false
    ),
    fop!(
        "xmlforest",
        FuncCategory::Scalar,
        FuncDomain::Xml,
        1,
        None,
        false
    ),
    fop!(
        "xmlparse",
        FuncCategory::Scalar,
        FuncDomain::Xml,
        1,
        Some(2),
        false
    ),
    fop!(
        "xmlpi",
        FuncCategory::Scalar,
        FuncDomain::Xml,
        1,
        Some(2),
        false
    ),
    fop!(
        "xmlquery",
        FuncCategory::Scalar,
        FuncDomain::Xml,
        2,
        Some(3),
        false
    ),
    fop!(
        "xmlserialize",
        FuncCategory::Scalar,
        FuncDomain::Xml,
        2,
        Some(3),
        false
    ),
    fop!(
        "xmltype",
        FuncCategory::Scalar,
        FuncDomain::Xml,
        1,
        Some(1),
        false
    ),
```

**Step 2: 编写 XML domain 测试**

```rust
#[test]
fn test_lookup_xml_xmlelement() {
    let meta = lookup_function("xmlelement").unwrap();
    assert_eq!(meta.domain, FuncDomain::Xml);
    assert_eq!(meta.category, FuncCategory::Scalar);
}

#[test]
fn test_lookup_xml_xmlagg() {
    let meta = lookup_function("xmlagg").unwrap();
    assert_eq!(meta.domain, FuncDomain::Xml);
    assert_eq!(meta.category, FuncCategory::Aggregate);
    assert!(meta.supports_distinct);
}

#[test]
fn test_builtin_meta_xml() {
    let meta = lookup_builtin_meta("xmlelement").unwrap();
    assert_eq!(meta.category, "Scalar");
    assert_eq!(meta.domain, "Xml");
}
```

**Step 3: 验证**

Run: `cargo test test_lookup_xml_ test_builtin_meta_xml 2>&1`
Expected: 3 个测试 PASS。

**Step 4: Commit**

```bash
git add src/parser/function_registry.rs
git commit -m "feat(registry): register XML domain functions (xmlelement, xmlagg, etc.)"
```

---

## Task 5: 注册 DBE_LOB 域函数

**Files:**
- Modify: `src/parser/function_registry.rs`（FUNCTIONS 数组）

**Why:** LOB 操作是存储过程迁移中最常用的包函数之一。

**Step 1: 在 FUNCTIONS 数组中按字母序插入 DBE_LOB 函数**

`dbe_lob.*` 函数以 `"dbe_lob."` 前缀开头。字母序 `"dbe" < "dec"`（b < e），因此所有 `dbe_*` 条目位于 `date_trunc` 和 `decode` 之间。`dbe_lob` 在新块内部排在 `dbe_file` 之后、`dbe_output` 之前（`"dbe_f" < "dbe_l" < "dbe_o"`）。

```rust
    fop!(
        "dbe_lob.append",
        FuncCategory::Scalar,
        FuncDomain::DbeLob,
        2,
        Some(2),
        false
    ),
    fop!(
        "dbe_lob.compare",
        FuncCategory::Scalar,
        FuncDomain::DbeLob,
        2,
        Some(3),
        false
    ),
    fop!(
        "dbe_lob.copy",
        FuncCategory::Scalar,
        FuncDomain::DbeLob,
        3,
        Some(5),
        false
    ),
    fop!(
        "dbe_lob.createtemporary",
        FuncCategory::Scalar,
        FuncDomain::DbeLob,
        1,
        Some(3),
        false
    ),
    fop!(
        "dbe_lob.erase",
        FuncCategory::Scalar,
        FuncDomain::DbeLob,
        2,
        Some(3),
        false
    ),
    fop!(
        "dbe_lob.freetemporary",
        FuncCategory::Scalar,
        FuncDomain::DbeLob,
        1,
        Some(1),
        false
    ),
    fop!(
        "dbe_lob.getlength",
        FuncCategory::Scalar,
        FuncDomain::DbeLob,
        1,
        Some(1),
        false
    ),
    fop!(
        "dbe_lob.instr",
        FuncCategory::Scalar,
        FuncDomain::DbeLob,
        2,
        Some(4),
        false
    ),
    fop!(
        "dbe_lob.read",
        FuncCategory::Scalar,
        FuncDomain::DbeLob,
        3,
        Some(3),
        false
    ),
    fop!(
        "dbe_lob.substr",
        FuncCategory::Scalar,
        FuncDomain::DbeLob,
        1,
        Some(3),
        false
    ),
    fop!(
        "dbe_lob.trim",
        FuncCategory::Scalar,
        FuncDomain::DbeLob,
        2,
        Some(2),
        false
    ),
    fop!(
        "dbe_lob.write",
        FuncCategory::Scalar,
        FuncDomain::DbeLob,
        3,
        Some(3),
        false
    ),
```

**Step 2: 编写 DBE_LOB 测试**

```rust
#[test]
fn test_lookup_qualified_dbe_lob_detailed() {
    let meta = crate::parser::function_registry::lookup_function_qualified("dbe_lob.append").unwrap();
    assert_eq!(meta.name, "dbe_lob.append");
    assert_eq!(meta.domain, FuncDomain::DbeLob);
    assert_eq!(meta.category, FuncCategory::Scalar);
    assert_eq!(meta.min_args, 2);
    assert_eq!(meta.max_args, Some(2));
}

#[test]
fn test_lookup_dbe_lob_getlength() {
    let meta = crate::parser::function_registry::lookup_function_qualified("dbe_lob.getlength").unwrap();
    assert_eq!(meta.domain, FuncDomain::DbeLob);
}

#[test]
fn test_builtin_meta_dbe_lob() {
    let meta = lookup_builtin_meta_qualified("dbe_lob.append").unwrap();
    assert_eq!(meta.category, "Scalar");
    assert_eq!(meta.domain, "DbeLob");
}
```

**Step 3: 验证**

Run: `cargo test test_lookup_dbe_lob test_builtin_meta_dbe_lob 2>&1`
Expected: PASS。

**Step 4: Commit**

```bash
git add src/parser/function_registry.rs
git commit -m "feat(registry): register DBE_LOB domain functions (12 functions)"
```

---

## Task 6: 注册 DBE_OUTPUT / DBMS_OUTPUT 域函数

**Files:**
- Modify: `src/parser/function_registry.rs`（FUNCTIONS 数组）

**Step 1: 注册 DBE_OUTPUT 函数**

所有 `dbe_*`/`dbms_*` 条目位于 `date_trunc`（line 610）和 `decode`（line 618）之间。`dbe_output` 在新块内部排在 `dbe_lob` 之后、`dbe_scheduler` 之前。`dbms_output` 排在 `dbms_lob` 之后、`dbms_scheduler` 之前。

```rust
    fop!(
        "dbe_output.disable",
        FuncCategory::Scalar,
        FuncDomain::DbeOutput,
        0,
        Some(0),
        false
    ),
    fop!(
        "dbe_output.enable",
        FuncCategory::Scalar,
        FuncDomain::DbeOutput,
        0,
        Some(1),
        false
    ),
    fop!(
        "dbe_output.get_line",
        FuncCategory::Scalar,
        FuncDomain::DbeOutput,
        2,
        Some(2),
        false
    ),
    fop!(
        "dbe_output.get_lines",
        FuncCategory::Scalar,
        FuncDomain::DbeOutput,
        2,
        Some(2),
        false
    ),
    fop!(
        "dbe_output.new_line",
        FuncCategory::Scalar,
        FuncDomain::DbeOutput,
        0,
        Some(0),
        false
    ),
    fop!(
        "dbe_output.print",
        FuncCategory::Scalar,
        FuncDomain::DbeOutput,
        1,
        Some(1),
        false
    ),
    fop!(
        "dbe_output.put",
        FuncCategory::Scalar,
        FuncDomain::DbeOutput,
        1,
        Some(1),
        false
    ),
    fop!(
        "dbe_output.put_line",
        FuncCategory::Scalar,
        FuncDomain::DbeOutput,
        1,
        Some(1),
        false
    ),
```

**Step 2: 注册 DBMS_OUTPUT 函数**

```rust
    fop!(
        "dbms_output.disable",
        FuncCategory::Scalar,
        FuncDomain::DbmsOutput,
        0,
        Some(0),
        false
    ),
    fop!(
        "dbms_output.enable",
        FuncCategory::Scalar,
        FuncDomain::DbmsOutput,
        0,
        Some(1),
        false
    ),
    fop!(
        "dbms_output.put",
        FuncCategory::Scalar,
        FuncDomain::DbmsOutput,
        1,
        Some(1),
        false
    ),
    fop!(
        "dbms_output.put_line",
        FuncCategory::Scalar,
        FuncDomain::DbmsOutput,
        1,
        Some(1),
        false
    ),
```

**Step 3: 编写测试**

```rust
#[test]
fn test_lookup_dbe_output() {
    let meta = crate::parser::function_registry::lookup_function_qualified("dbe_output.put_line").unwrap();
    assert_eq!(meta.domain, FuncDomain::DbeOutput);
}

#[test]
fn test_lookup_dbms_output() {
    let meta = crate::parser::function_registry::lookup_function_qualified("dbms_output.put_line").unwrap();
    assert_eq!(meta.domain, FuncDomain::DbmsOutput);
}

#[test]
fn test_dbe_vs_dbms_output_distinct() {
    // 确保 dbe_output 和 dbms_output 的 put_line 被区分开
    let dbe = crate::parser::function_registry::lookup_function_qualified("dbe_output.put_line").unwrap();
    let dbms = crate::parser::function_registry::lookup_function_qualified("dbms_output.put_line").unwrap();
    assert_ne!(dbe.domain, dbms.domain, "dbe_output.put_line and dbms_output.put_line should have different domains");
}
```

**Step 4: 验证**

Run: `cargo test test_lookup_dbe_output test_lookup_dbms_output test_dbe_vs_dbms 2>&1`
Expected: PASS。

**Step 5: Commit**

```bash
git add src/parser/function_registry.rs
git commit -m "feat(registry): register DBE_OUTPUT and DBMS_OUTPUT domain functions"
```

---

## Task 7: 注册 DBE_SQL / DBMS_SQL 域函数

**Files:**
- Modify: `src/parser/function_registry.rs`（FUNCTIONS 数组）

**Step 1: 注册 DBE_SQL 函数**

```rust
    fop!(
        "dbe_sql.close_cursor",
        FuncCategory::Scalar,
        FuncDomain::DbeSql,
        1,
        Some(1),
        false
    ),
    fop!(
        "dbe_sql.column_value",
        FuncCategory::Scalar,
        FuncDomain::DbeSql,
        3,
        Some(3),
        false
    ),
    fop!(
        "dbe_sql.execute",
        FuncCategory::Scalar,
        FuncDomain::DbeSql,
        1,
        Some(2),
        false
    ),
    fop!(
        "dbe_sql.fetch_rows",
        FuncCategory::Scalar,
        FuncDomain::DbeSql,
        1,
        Some(1),
        false
    ),
    fop!(
        "dbe_sql.open_cursor",
        FuncCategory::Scalar,
        FuncDomain::DbeSql,
        0,
        Some(0),
        false
    ),
    fop!(
        "dbe_sql.register_variable",
        FuncCategory::Scalar,
        FuncDomain::DbeSql,
        3,
        Some(3),
        false
    ),
```

**Step 2: 注册 DBMS_SQL 函数**

```rust
    fop!(
        "dbms_sql.close_cursor",
        FuncCategory::Scalar,
        FuncDomain::DbmsSql,
        1,
        Some(1),
        false
    ),
    fop!(
        "dbms_sql.column_value",
        FuncCategory::Scalar,
        FuncDomain::DbmsSql,
        3,
        Some(3),
        false
    ),
    fop!(
        "dbms_sql.execute",
        FuncCategory::Scalar,
        FuncDomain::DbmsSql,
        1,
        Some(2),
        false
    ),
    fop!(
        "dbms_sql.fetch_rows",
        FuncCategory::Scalar,
        FuncDomain::DbmsSql,
        1,
        Some(1),
        false
    ),
    fop!(
        "dbms_sql.open_cursor",
        FuncCategory::Scalar,
        FuncDomain::DbmsSql,
        0,
        Some(0),
        false
    ),
```

**Step 3: 编写测试**

```rust
#[test]
fn test_lookup_dbe_sql() {
    let meta = crate::parser::function_registry::lookup_function_qualified("dbe_sql.execute").unwrap();
    assert_eq!(meta.domain, FuncDomain::DbeSql);
}

#[test]
fn test_lookup_dbms_sql() {
    let meta = crate::parser::function_registry::lookup_function_qualified("dbms_sql.execute").unwrap();
    assert_eq!(meta.domain, FuncDomain::DbmsSql);
}
```

**Step 4: 验证并 Commit**

Run: `cargo test test_lookup_dbe_sql test_lookup_dbms_sql 2>&1`

```bash
git add src/parser/function_registry.rs
git commit -m "feat(registry): register DBE_SQL and DBMS_SQL domain functions"
```

---

## Task 8: 注册 DBE_FILE / UTL_FILE 域函数

**Files:**
- Modify: `src/parser/function_registry.rs`（FUNCTIONS 数组）

**Step 1: 注册 DBE_FILE 函数**

`dbe_file.*` 位于 `date_trunc`（line 610）和 `decode`（line 618）之间，是 `dbe_*`/`dbms_*` 块中的第一组。

```rust
    fop!(
        "dbe_file.close",
        FuncCategory::Scalar,
        FuncDomain::DbeFile,
        1,
        Some(1),
        false
    ),
    fop!(
        "dbe_file.copy",
        FuncCategory::Scalar,
        FuncDomain::DbeFile,
        3,
        Some(3),
        false
    ),
    fop!(
        "dbe_file.open",
        FuncCategory::Scalar,
        FuncDomain::DbeFile,
        2,
        Some(4),
        false
    ),
    fop!(
        "dbe_file.read_line",
        FuncCategory::Scalar,
        FuncDomain::DbeFile,
        2,
        Some(3),
        false
    ),
    fop!(
        "dbe_file.remove",
        FuncCategory::Scalar,
        FuncDomain::DbeFile,
        2,
        Some(2),
        false
    ),
    fop!(
        "dbe_file.rename",
        FuncCategory::Scalar,
        FuncDomain::DbeFile,
        3,
        Some(3),
        false
    ),
    fop!(
        "dbe_file.write_line",
        FuncCategory::Scalar,
        FuncDomain::DbeFile,
        2,
        Some(2),
        false
    ),
```

**Step 2: 注册 UTL_FILE 函数**

`utl_file.*` 字母序位于 `user`（line 1785）和 `var_pop`（line 1794）之间（`"use" < "utl" < "var"`）。

```rust
    fop!(
        "utl_file.fclose",
        FuncCategory::Scalar,
        FuncDomain::UtlFile,
        1,
        Some(1),
        false
    ),
    fop!(
        "utl_file.fclose_all",
        FuncCategory::Scalar,
        FuncDomain::UtlFile,
        0,
        Some(0),
        false
    ),
    fop!(
        "utl_file.fopen",
        FuncCategory::Scalar,
        FuncDomain::UtlFile,
        2,
        Some(4),
        false
    ),
    fop!(
        "utl_file.get_line",
        FuncCategory::Scalar,
        FuncDomain::UtlFile,
        1,
        Some(2),
        false
    ),
    fop!(
        "utl_file.put_line",
        FuncCategory::Scalar,
        FuncDomain::UtlFile,
        1,
        Some(2),
        false
    ),
```

**Step 3: 编写测试**

```rust
#[test]
fn test_lookup_dbe_file() {
    let meta = crate::parser::function_registry::lookup_function_qualified("dbe_file.open").unwrap();
    assert_eq!(meta.domain, FuncDomain::DbeFile);
}

#[test]
fn test_lookup_utl_file() {
    let meta = crate::parser::function_registry::lookup_function_qualified("utl_file.fopen").unwrap();
    assert_eq!(meta.domain, FuncDomain::UtlFile);
}
```

**Step 4: 验证并 Commit**

```bash
git add src/parser/function_registry.rs
git commit -m "feat(registry): register DBE_FILE and UTL_FILE domain functions"
```

---

## Task 9: 注册 DBE_SCHEDULER / DBMS_SCHEDULER 域函数

**Files:**
- Modify: `src/parser/function_registry.rs`（FUNCTIONS 数组）

**Step 1: 注册 DBE_SCHEDULER 函数**

```rust
    fop!(
        "dbe_scheduler.create_job",
        FuncCategory::Scalar,
        FuncDomain::DbeScheduler,
        1,
        None,
        false
    ),
    fop!(
        "dbe_scheduler.drop_job",
        FuncCategory::Scalar,
        FuncDomain::DbeScheduler,
        1,
        None,
        false
    ),
    fop!(
        "dbe_scheduler.run_job",
        FuncCategory::Scalar,
        FuncDomain::DbeScheduler,
        1,
        Some(2),
        false
    ),
```

**Step 2: 注册 DBMS_SCHEDULER 函数**

```rust
    fop!(
        "dbms_scheduler.create_job",
        FuncCategory::Scalar,
        FuncDomain::DbmsScheduler,
        1,
        None,
        false
    ),
    fop!(
        "dbms_scheduler.drop_job",
        FuncCategory::Scalar,
        FuncDomain::DbmsScheduler,
        1,
        None,
        false
    ),
    fop!(
        "dbms_scheduler.run_job",
        FuncCategory::Scalar,
        FuncDomain::DbmsScheduler,
        1,
        Some(2),
        false
    ),
```

**Step 3: 编写测试**

```rust
#[test]
fn test_lookup_dbe_scheduler() {
    let meta = crate::parser::function_registry::lookup_function_qualified("dbe_scheduler.create_job").unwrap();
    assert_eq!(meta.domain, FuncDomain::DbeScheduler);
}

#[test]
fn test_lookup_dbms_scheduler() {
    let meta = crate::parser::function_registry::lookup_function_qualified("dbms_scheduler.create_job").unwrap();
    assert_eq!(meta.domain, FuncDomain::DbmsScheduler);
}
```

**Step 4: 验证并 Commit**

```bash
git add src/parser/function_registry.rs
git commit -m "feat(registry): register DBE_SCHEDULER and DBMS_SCHEDULER domain functions"
```

---

## Task 10: 注册剩余域函数（DBE_UTILITY, DBE_STATS, DBE_SESSION, DBMS_LOB, DBMS_UTILITY, PKG_SERVICE）

**Files:**
- Modify: `src/parser/function_registry.rs`（FUNCTIONS 数组）

**Why:** 完成所有 16 个域的函数注册。

**Step 1: 注册 DBE_UTILITY 函数**

```rust
    fop!(
        "dbe_utility.get_time",
        FuncCategory::Scalar,
        FuncDomain::DbeUtility,
        0,
        Some(0),
        false
    ),
    fop!(
        "dbe_utility.format_error_backtrace",
        FuncCategory::Scalar,
        FuncDomain::DbeUtility,
        0,
        Some(0),
        false
    ),
    fop!(
        "dbe_utility.format_error_stack",
        FuncCategory::Scalar,
        FuncDomain::DbeUtility,
        0,
        Some(0),
        false
    ),
```

**Step 2: 注册 DBE_STATS 函数**

```rust
    fop!(
        "dbe_stats.lock_table_stats",
        FuncCategory::Scalar,
        FuncDomain::DbeStats,
        1,
        Some(1),
        false
    ),
    fop!(
        "dbe_stats.unlock_table_stats",
        FuncCategory::Scalar,
        FuncDomain::DbeStats,
        1,
        Some(1),
        false
    ),
```

**Step 3: 注册 DBE_SESSION 函数**

```rust
    fop!(
        "dbe_session.set_context",
        FuncCategory::Scalar,
        FuncDomain::DbeSession,
        3,
        Some(3),
        false
    ),
    fop!(
        "dbe_session.clear_context",
        FuncCategory::Scalar,
        FuncDomain::DbeSession,
        2,
        Some(3),
        false
    ),
```

**Step 4: 注册 DBMS_LOB 函数**

```rust
    fop!(
        "dbms_lob.append",
        FuncCategory::Scalar,
        FuncDomain::DbmsLob,
        2,
        Some(2),
        false
    ),
    fop!(
        "dbms_lob.read",
        FuncCategory::Scalar,
        FuncDomain::DbmsLob,
        3,
        Some(3),
        false
    ),
    fop!(
        "dbms_lob.substr",
        FuncCategory::Scalar,
        FuncDomain::DbmsLob,
        1,
        Some(3),
        false
    ),
    fop!(
        "dbms_lob.write",
        FuncCategory::Scalar,
        FuncDomain::DbmsLob,
        3,
        Some(3),
        false
    ),
```

**Step 5: 注册 DBMS_UTILITY 函数**

```rust
    fop!(
        "dbms_utility.get_time",
        FuncCategory::Scalar,
        FuncDomain::DbmsUtility,
        0,
        Some(0),
        false
    ),
    fop!(
        "dbms_utility.format_error_backtrace",
        FuncCategory::Scalar,
        FuncDomain::DbmsUtility,
        0,
        Some(0),
        false
    ),
```

**Step 6: 注册 PKG_SERVICE 函数**

`pkg_service.sql_cancel` 字母序位于 `pi`（line 1308）和 `position`（line 1316）之间（`"pi" < "pkg" < "pos"`）。

```rust
    fop!(
        "pkg_service.sql_cancel",
        FuncCategory::Scalar,
        FuncDomain::PkgService,
        1,
        Some(1),
        false
    ),
```

**Step 7: 编写测试**

```rust
#[test]
fn test_lookup_dbe_utility() {
    let meta = crate::parser::function_registry::lookup_function_qualified("dbe_utility.get_time").unwrap();
    assert_eq!(meta.domain, FuncDomain::DbeUtility);
}

#[test]
fn test_lookup_dbe_stats() {
    let meta = crate::parser::function_registry::lookup_function_qualified("dbe_stats.lock_table_stats").unwrap();
    assert_eq!(meta.domain, FuncDomain::DbeStats);
}

#[test]
fn test_lookup_dbe_session() {
    let meta = crate::parser::function_registry::lookup_function_qualified("dbe_session.set_context").unwrap();
    assert_eq!(meta.domain, FuncDomain::DbeSession);
}

#[test]
fn test_lookup_dbms_lob() {
    let meta = crate::parser::function_registry::lookup_function_qualified("dbms_lob.append").unwrap();
    assert_eq!(meta.domain, FuncDomain::DbmsLob);
}

#[test]
fn test_lookup_dbms_utility() {
    let meta = crate::parser::function_registry::lookup_function_qualified("dbms_utility.get_time").unwrap();
    assert_eq!(meta.domain, FuncDomain::DbmsUtility);
}

#[test]
fn test_lookup_pkg_service() {
    let meta = crate::parser::function_registry::lookup_function_qualified("pkg_service.sql_cancel").unwrap();
    assert_eq!(meta.domain, FuncDomain::PkgService);
}

#[test]
fn test_dbe_lob_vs_dbms_lob_distinct() {
    let dbe = crate::parser::function_registry::lookup_function_qualified("dbe_lob.append").unwrap();
    let dbms = crate::parser::function_registry::lookup_function_qualified("dbms_lob.append").unwrap();
    assert_ne!(dbe.domain, dbms.domain, "dbe_lob.append and dbms_lob.append should have different domains");
    assert_eq!(dbe.domain, FuncDomain::DbeLob);
    assert_eq!(dbms.domain, FuncDomain::DbmsLob);
}
```

**Step 8: 验证并 Commit**

Run: `cargo test 2>&1 | tail -30`
Expected: 全部测试 PASS。

```bash
git add src/parser/function_registry.rs
git commit -m "feat(registry): register remaining domain functions (DbeUtility, DbeStats, DbeSession, DbmsLob, DbmsUtility, PkgService)"
```

---

## Task 11: 端到端验证 — 解析器集成测试

**Files:**
- Modify: `src/parser/tests.rs` 或 `src/parser/tests_plsql_fixes.rs`（已有 dbms_output 测试的文件）

**Why:** 验证完整的 SQL 解析 → 函数识别 → domain 标注流程。

**Step 1: 编写端到端集成测试**

```rust
#[test]
fn test_e2e_dbe_lob_append_builtin_meta() {
    let sql = "SELECT dbe_lob.append(v_lob, v_data)";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    // 从 AST 中提取 FunctionCall 的 builtin 字段
    let select = &stmts[0];
    // ... 遍历 AST 找到 FunctionCall 节点，验证 builtin.domain == "DbeLob"
    // 具体断言取决于 AST 结构
}

#[test]
fn test_e2e_xmlelement_builtin_meta() {
    let sql = "SELECT xmlelement(name \"a\", \"hello\")";
    let tokens = Tokenizer::new(sql).tokenize().unwrap();
    let stmts = Parser::new(tokens).parse().unwrap();
    // 验证 builtin.domain == "Xml"
}

#[test]
fn test_e2e_dbms_output_builtin_meta() {
    let sql = "CALL dbms_output.put_line('hello')";
    // 或使用 PL/pgSQL 上下文中的 dbms_output.put_line
    // 验证 builtin.domain == "DbmsOutput"
}
```

> **注意：** 此任务的具体测试代码取决于 FunctionCall 的 builtin 字段在解析后如何被访问。实现时需检查现有的 `parse_function_call` 流程中 `builtin` 是如何被赋值的，确保 `validate_func` 改造后正确传递了 domain 信息。

**Step 2: 运行全量测试**

Run: `cargo test 2>&1 | tail -40`
Expected: 所有现有测试 + 新增测试均 PASS，无回归。

**Step 3: Commit**

```bash
git add src/parser/tests.rs
git commit -m "test(registry): add end-to-end integration tests for package function domains"
```

---

## Task 12: 全局验证与清理

**Files:**
- All changed files

**Step 1: 运行全量测试套件**

Run: `cargo test 2>&1`
Expected: 0 failures。

**Step 2: 检查编译警告**

Run: `cargo build 2>&1 | grep -i warning`
Expected: 无新增 warning（特别是 unused import、dead code）。

**Step 3: 验证 FUNCTIONS 数组排序不变量**

Run: `cargo test test_lookup 2>&1`
Expected: 所有 lookup 测试通过（二分查找正确的前提是数组有序）。

**Step 4: LSP 诊断**

对 `function_registry.rs` 和 `expr.rs` 运行 `lsp_diagnostics`，确认无 error/warning。

**Step 5: 最终 Commit（如有修复）**

```bash
git add -A
git commit -m "chore(registry): final cleanup and verification"
```

---

## 排序不变量检查清单

`FUNCTIONS` 数组必须严格按 name 的 ASCII 小写字母序排列。新增条目时需确认插入位置：

| 前缀 | 插入位置（在...之间） | 排序验证 |
|---|---|---|
| `dbe_file.*` | `date_trunc` 和 `decode` 之间 | `"dat" < "dbe" < "dec"` ✅ |
| `dbe_lob.*` | `dbe_file.write_line` 和 `dbe_output.*` 之间 | `"dbe_f" < "dbe_l" < "dbe_o"` ✅ |
| `dbe_output.*` | `dbe_lob.write` 和 `dbe_scheduler.*` 之间 | `"dbe_l" < "dbe_o" < "dbe_s"` ✅ |
| `dbe_scheduler.*` | `dbe_output.put_line` 和 `dbe_session.*` 之间 | `"dbe_o" < "dbe_sc" < "dbe_se"` ✅ |
| `dbe_session.*` | `dbe_scheduler.run_job` 和 `dbe_sql.*` 之间 | `"dbe_sc" < "dbe_se" < "dbe_sq"` ✅ |
| `dbe_sql.*` | `dbe_session.set_context` 和 `dbe_stats.*` 之间 | `"dbe_se" < "dbe_sq" < "dbe_st"` ✅ |
| `dbe_stats.*` | `dbe_sql.register_variable` 和 `dbe_utility.*` 之间 | `"dbe_sq" < "dbe_st" < "dbe_u"` ✅ |
| `dbe_utility.*` | `dbe_stats.unlock_table_stats` 和 `dbms_lob.*` 之间 | `"dbe_s" < "dbe_u" < "dbm"` ✅ |
| `dbms_lob.*` | `dbe_utility.format_error_stack` 和 `dbms_output.*` 之间 | `"dbe" < "dbms_l" < "dbms_o"` ✅ |
| `dbms_output.*` | `dbms_lob.write` 和 `dbms_scheduler.*` 之间 | `"dbms_l" < "dbms_o" < "dbms_s"` ✅ |
| `dbms_scheduler.*` | `dbms_output.put_line` 和 `dbms_sql.*` 之间 | `"dbms_o" < "dbms_sc" < "dbms_sq"` ✅ |
| `dbms_sql.*` | `dbms_scheduler.run_job` 和 `dbms_utility.*` 之间 | `"dbms_sc" < "dbms_sq" < "dbms_u"` ✅ |
| `dbms_utility.*` | `dbms_sql.open_cursor` 和 `decode` 之间 | `"dbms_s" < "dbms_u" < "dec"` ✅ |
| `pkg_service.*` | `pi` 和 `position` 之间 | `"pi" < "pkg" < "pos"` ✅ |
| `utl_file.*` | `user` 和 `var_pop` 之间 | `"use" < "utl" < "var"` ✅ |
| `xml*` | `wm_concat` 之后，数组末尾 | `"wm" < "xml"` ✅ |

> **⚠️ 关键排序说明：** 所有 `dbe_*` 和 `dbms_*` 条目作为一个整体块，插入到现有 `date_trunc`（line 610）和 `decode`（line 618）之间。因为 `"dbe" < "dbm" < "dec"`（b < m? 不对——`dbe` vs `dbm`：d=d, b=b, e vs m, e < m ✅）。在新块内部，各前缀按 `dbe_file < dbe_lob < dbe_output < dbe_scheduler < dbe_session < dbe_sql < dbe_stats < dbe_utility < dbms_lob < dbms_output < dbms_scheduler < dbms_sql < dbms_utility` 排序。

> **实现时务必验证：** 每个 `fop!` 条目插入后，其 `name` 值与前一条目和后一条目比较，确保严格递增。

---

## 撤销策略

如果任何 Task 导致回归，可通过 `git revert` 单独回滚该 commit。每个 Task 都是独立 commit，互不影响。

---

## 预估工作量

| Task | 预估时间 | 复杂度 |
|---|---|---|
| Task 1: FuncDomain 枚举扩展 | 5 min | 低 |
| Task 2: fop! 宏 | 3 min | 低 |
| Task 3: 两阶段查找逻辑 | 15 min | **中** |
| Task 4: XML 函数 | 10 min | 低 |
| Task 5: DBE_LOB 函数 | 10 min | 低 |
| Task 6: DBE_OUTPUT/DBMS_OUTPUT | 10 min | 低 |
| Task 7: DBE_SQL/DBMS_SQL | 8 min | 低 |
| Task 8: DBE_FILE/UTL_FILE | 8 min | 低 |
| Task 9: DBE_SCHEDULER/DBMS_SCHEDULER | 5 min | 低 |
| Task 10: 剩余域函数 | 15 min | 低 |
| Task 11: 端到端集成测试 | 15 min | 中 |
| Task 12: 全局验证 | 5 min | 低 |
| **总计** | **~110 min** | |
