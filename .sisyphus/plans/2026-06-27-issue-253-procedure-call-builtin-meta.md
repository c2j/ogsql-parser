# Issue #253 — 为 PlProcedureCall 和 CallFuncStatement 附加 builtin 元数据

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 将表达式路径上 `Expr::FunctionCall.builtin` 的元数据填充机制，对称扩展到语句路径的 `PlProcedureCall` 和 `CallFuncStatement`，让下游 visitor 入口即可区分内置包函数调用与用户定义过程调用。

**Architecture:** 方案 B —— 在 `function_registry` 中抽取公共 helper `resolve_builtin_meta(name: &ObjectName) -> Option<BuiltinFuncMeta>`，复用现有 `lookup_builtin_meta_qualified` + `lookup_builtin_meta` 双相查询模式（与 `expr.rs:validate_func` 行为完全一致）。5 个生产构造点调用该 helper，2 个测试构造点用 `None`。两个 AST 结构新增 `builtin: Option<BuiltinFuncMeta>` 字段，带 serde 兼容属性。顺带将 `expr.rs:validate_func` 内联 lookup 替换为调用新 helper（消除现有 6 处重复）。

**Tech Stack:** Rust 2021, serde, 现有 `function_registry` 静态注册表（981 个内置函数，含 dotted package 名）。

**Issue:** [#253](https://github.com/c2j/ogsql-parser/issues/253) — `feat: 为 PlProcedureCall 和 CallFuncStatement 附加 builtin 元数据`

---

## 前置事实（已核对）

- `ObjectName = Vec<Ident>`（`ast/mod.rs:1426`），`Ident: Deref<Target = str>`（`ast/ident.rs:36`），`Ident::from(&str)` 存在 → `vec!["abs".into()]` 合法
- `BuiltinFuncMeta` 定义于 `ast/mod.rs:47-50`，仅 `{ category: String, domain: String }`
- `lookup_builtin_meta(name: &str)` 在 `function_registry.rs:1454` —— 单段名精确匹配（dotted 名作为单个 `&'static str` 键存储）
- `lookup_builtin_meta_qualified(full_name: &str)` 在 `function_registry.rs:1535` —— 两相查询：先精确 dotted 名匹配，失败则回退末段
- 现有 `expr.rs:11-37 validate_func` 是 builtin 填充的参考实现：
  ```rust
  let full_name = name.iter().map(|s| s.to_lowercase()).collect::<Vec<_>>().join(".");
  let last_seg = full_name.split('.').next_back().unwrap_or(&full_name).to_string();
  let builtin = lookup_builtin_meta_qualified(&full_name)
      .or_else(|| lookup_builtin_meta(&last_seg));
  ```
- `function_registry.rs` 测试模块：`#[cfg(test)] mod tests { ... }`（line 1677）
- 仓库 CI 要求：`cargo fmt --all -- --check` + `cargo clippy --all-features -- -D warnings` + `cargo test --all-features` + `cargo audit`

---

## 任务依赖图

```
Task 1 (helper)  ─┬─→ Task 2 (PlProcedureCall)
                   ├─→ Task 3 (CallFuncStatement)
                   └─→ Task 4 (expr.rs DRY refactor)

Task 2, 3 互相独立，可并行
Task 4 依赖 Task 1（使用 helper），与 Task 2/3 无依赖
Task 5 (最终 CI) 依赖前 4 个全部完成
```

---

## Task 1: 新增 `resolve_builtin_meta` helper（TDD）

**Files:**
- Modify: `src/parser/function_registry.rs`（在 `lookup_builtin_meta_qualified` 之后，约 line 1610 附近，添加新 public 函数）
- Modify: `src/parser/function_registry.rs` 的 `#[cfg(test)] mod tests`（line 1677+，添加 4 个单元测试）

### Step 1.1: 编写 4 个失败的单元测试

在 `function_registry.rs` 测试模块末尾（`lookup_builtin_meta_qualified("dbe_application_info.set_action")` 那个测试之后）追加：

```rust
    // ── resolve_builtin_meta：ObjectName 投影 helper ──

    #[test]
    fn test_resolve_builtin_meta_plain_function() {
        let name: crate::ast::ObjectName = vec!["abs".into()];
        let meta = super::resolve_builtin_meta(&name).unwrap();
        assert_eq!(meta.domain, "Math");
    }

    #[test]
    fn test_resolve_builtin_meta_dotted_package_function() {
        let name: crate::ast::ObjectName = vec!["dbe_output".into(), "put_line".into()];
        let meta = super::resolve_builtin_meta(&name).unwrap();
        assert_eq!(meta.domain, "DbeOutput");
    }

    #[test]
    fn test_resolve_builtin_meta_unknown_returns_none() {
        let name: crate::ast::ObjectName = vec!["definitely_not_a_real_func_xyz".into()];
        assert!(super::resolve_builtin_meta(&name).is_none());
    }

    #[test]
    fn test_resolve_builtin_meta_schema_qualified_fallback() {
        // 未知 schema 前缀 + 已知函数名 → 回退末段匹配
        let name: crate::ast::ObjectName = vec!["myschema".into(), "abs".into()];
        let meta = super::resolve_builtin_meta(&name).unwrap();
        assert_eq!(meta.domain, "Math");
    }
```

### Step 1.2: 运行测试验证失败

```bash
cargo test --lib -p ogsql-parser function_registry::tests::test_resolve_builtin_meta -- --nocapture
```

**Expected:** 4 个测试编译失败，错误信息 `cannot find function 'resolve_builtin_meta' in module 'super'`。

### Step 1.3: 实现 `resolve_builtin_meta`

在 `function_registry.rs` 中，紧接 `lookup_builtin_meta_qualified` 函数结束后（约 line 1610）添加：

```rust
/// Resolve built-in function metadata from a (possibly dotted) [`ObjectName`].
///
/// Two-phase lookup replicating the behavior of `Expr::FunctionCall` population
/// in `parser/expr.rs::validate_func`:
///   1. Exact full-qualified dotted-name match (`lookup_builtin_meta_qualified`)
///   2. Fallback to last segment (`lookup_builtin_meta`)
///
/// Handles both plain functions (`abs`) and package-qualified functions
/// (`dbe_output.put_line`) — the registry stores dotted package names as a
/// single `&'static str` key, so phase 1 matches them via exact full-string lookup.
///
/// Also handles schema-qualified user names (`myschema.abs`) via the phase 2
/// last-segment fallback.
pub fn resolve_builtin_meta(
    name: &crate::ast::ObjectName,
) -> Option<crate::ast::BuiltinFuncMeta> {
    let full = name
        .iter()
        .map(|s| s.to_lowercase())
        .collect::<Vec<_>>()
        .join(".");
    let last = full
        .split('.')
        .next_back()
        .unwrap_or(&full)
        .to_string();
    lookup_builtin_meta_qualified(&full).or_else(|| lookup_builtin_meta(&last))
}
```

### Step 1.4: 运行测试验证通过

```bash
cargo test --lib -p ogsql-parser function_registry::tests::test_resolve_builtin_meta -- --nocapture
```

**Expected:** 4 个测试 PASS。

### Step 1.5: 提交

```bash
cargo fmt --all
git add src/parser/function_registry.rs
git commit -m "feat(parser): add resolve_builtin_meta helper for ObjectName-based lookup

Public helper in function_registry that encapsulates the two-phase
lookup pattern used by Expr::FunctionCall population. Will be reused
by PlProcedureCall and CallFuncStatement construction sites in #253."
```

---

## Task 2: 扩展 `PlProcedureCall` 结构并填充 builtin（5 个构造点）

**Files:**
- Modify: `src/ast/plpgsql.rs:225-228`（struct 定义）
- Modify: `src/parser/plpgsql.rs:1270-1273`（`try_parse_pl_procedure_call` 构造点）
- Modify: `src/parser/plpgsql.rs:1305-1308`（`try_parse_pl_procedure_call_from_name` 构造点）
- Modify: `src/parser/plpgsql.rs:2157-2161`（`parse_pl_call` 构造点）
- Modify: `src/ast/visitor.rs:1482-1489`（测试构造点）
- Modify: `src/ast/visitor_tests.rs:245-251`（测试构造点）
- Modify: `src/parser/plpgsql.rs` 测试模块（新增集成测试）

### Step 2.1: 在 struct 定义中添加 `builtin` 字段

`src/ast/plpgsql.rs:224-228` 当前：

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlProcedureCall {
    pub name: crate::ast::ObjectName,
    pub arguments: Vec<crate::ast::Expr>,
}
```

改为：

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlProcedureCall {
    pub name: crate::ast::ObjectName,
    pub arguments: Vec<crate::ast::Expr>,
    /// Built-in function metadata if this call targets a registered built-in
    /// (e.g. `dbe_output.put_line`); `None` for user-defined procedures.
    /// Populated symmetrically with `Expr::FunctionCall.builtin`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub builtin: Option<crate::ast::BuiltinFuncMeta>,
}
```

**注意 `#[serde(default, ...)]`**：必加，否则旧 JSON 反序列化失败（破坏 `tests/roundtrip.rs`）。

### Step 2.2: 在 3 个 parser 构造点填充 builtin

**Site A — `parser/plpgsql.rs:1270-1273`**（`try_parse_pl_procedure_call`）

当前：

```rust
        Some(PlStatement::ProcedureCall(Spanned::new(
            PlProcedureCall { name, arguments },
            Some(SourceSpan { start, end: self.prev_location() }),
        )))
```

改为（注意：在 move `name` 之前先借用它做 lookup）：

```rust
        let builtin = crate::parser::function_registry::resolve_builtin_meta(&name);
        Some(PlStatement::ProcedureCall(Spanned::new(
            PlProcedureCall { name, arguments, builtin },
            Some(SourceSpan { start, end: self.prev_location() }),
        )))
```

**Site B — `parser/plpgsql.rs:1305-1308`**（`try_parse_pl_procedure_call_from_name`）

当前：

```rust
        Some(PlStatement::ProcedureCall(Spanned::new(
            PlProcedureCall { name: vec![name_str.into()], arguments },
            Some(SourceSpan { start, end: self.prev_location() }),
        )))
```

改为（先构造 ObjectName 再 lookup，避免 double-eval）：

```rust
        let name: crate::ast::ObjectName = vec![name_str.into()];
        let builtin = crate::parser::function_registry::resolve_builtin_meta(&name);
        Some(PlStatement::ProcedureCall(Spanned::new(
            PlProcedureCall { name, arguments, builtin },
            Some(SourceSpan { start, end: self.prev_location() }),
        )))
```

**Site C — `parser/plpgsql.rs:2157-2161`**（`parse_pl_call`）

当前：

```rust
        Ok(PlStatement::ProcedureCall(Spanned::new(
            PlProcedureCall { name, arguments },
            Some(SourceSpan { start, end: self.prev_location() }),
        )))
```

改为：

```rust
        let builtin = crate::parser::function_registry::resolve_builtin_meta(&name);
        Ok(PlStatement::ProcedureCall(Spanned::new(
            PlProcedureCall { name, arguments, builtin },
            Some(SourceSpan { start, end: self.prev_location() }),
        )))
```

### Step 2.3: 更新 2 个测试构造点

**`src/ast/visitor.rs:1483-1489`**（`test_walk_pl_statement_procedure_call`）

当前：

```rust
        PlProcedureCall {
            name: vec!["schema".into(), "proc".into()],
            arguments: vec![
                Expr::Literal(crate::ast::Literal::Integer(1)),
                Expr::Literal(crate::ast::Literal::Integer(2)),
            ],
        },
```

追加 `builtin: None`：

```rust
        PlProcedureCall {
            name: vec!["schema".into(), "proc".into()],
            arguments: vec![
                Expr::Literal(crate::ast::Literal::Integer(1)),
                Expr::Literal(crate::ast::Literal::Integer(2)),
            ],
            builtin: None,
        },
```

**`src/ast/visitor_tests.rs:245-251`**

当前：

```rust
    let proc_call = PlStatement::ProcedureCall(PlProcedureCall {
        name: vec!["schema".to_string(), "proc".to_string()],
        arguments: vec![
            Expr::Literal(crate::ast::Literal::Integer(1)),
            Expr::Literal(crate::ast::Literal::Integer(2)),
        ],
    });
```

追加 `builtin: None`：

```rust
    let proc_call = PlStatement::ProcedureCall(PlProcedureCall {
        name: vec!["schema".to_string(), "proc".to_string()],
        arguments: vec![
            Expr::Literal(crate::ast::Literal::Integer(1)),
            Expr::Literal(crate::ast::Literal::Integer(2)),
        ],
        builtin: None,
    });
```

### Step 2.4: 添加集成测试（验证 builtin 被正确填充）

**路径已实测确认**（Momus 评审后修正）：
- 顶层 `BEGIN ... END;` 不存在 `Statement::PlBlock` 变体——PL 块必须包在 `DO $$ ... $$` 中，通过 `Statement::Do(DoStatement { block: Option<PlBlock> })` 暴露
- `DoStatement.block` 是 `Option<PlBlock>`（line 2364），需 `.as_ref()` 解包
- `Spanned<T>` 的内部字段是 `.node`（line 15）

在 `src/parser/plpgsql.rs` 的 `#[cfg(test)] mod tests` 模块末尾追加：

```rust
    #[test]
    fn test_pl_procedure_call_populates_builtin_for_dbe_output() {
        use crate::Tokenizer;
        use crate::ast::plpgsql::{PlStatement, PlProcedureCall};
        use crate::parser::Parser;

        // PL 块必须包在 DO $$ ... $$ 中（无 Statement::PlBlock 顶层变体）
        let sql = "DO $$ BEGIN dbe_output.put_line('hello'); END $$";
        let tokens = Tokenizer::new(sql).tokenize().expect("tokenize failed");
        let stmts = Parser::new(tokens).parse().expect("parse failed");

        // Statement::Do(do_stmt) → do_stmt.node.block.as_ref()?.body → PlStatement::ProcedureCall
        let proc_calls: Vec<&PlProcedureCall> = stmts
            .iter()
            .flat_map(|s| match s {
                crate::ast::Statement::Do(do_stmt) => do_stmt.node.block.as_ref().map(|b| b.body.iter().collect::<Vec<_>>()),
                _ => None,
            })
            .flatten()
            .filter_map(|s| match s {
                PlStatement::ProcedureCall(spanned) => Some(&spanned.node),
                _ => None,
            })
            .collect();

        assert_eq!(proc_calls.len(), 1, "expected exactly one procedure call");
        let builtin = proc_calls[0].builtin.as_ref().expect("builtin should be populated");
        assert_eq!(builtin.domain, "DbeOutput", "domain should be DbeOutput for dbe_output.put_line");
    }

    #[test]
    fn test_pl_procedure_call_builtin_none_for_unknown_procedure() {
        use crate::Tokenizer;
        use crate::ast::plpgsql::PlStatement;
        use crate::parser::Parser;

        let sql = "DO $$ BEGIN my_unknown_proc(); END $$";
        let tokens = Tokenizer::new(sql).tokenize().expect("tokenize failed");
        let stmts = Parser::new(tokens).parse().expect("parse failed");

        let has_none_builtin = stmts.iter().any(|s| match s {
            crate::ast::Statement::Do(do_stmt) => {
                do_stmt.node.block.as_ref().map_or(false, |b| {
                    b.body.iter().any(|s| match s {
                        PlStatement::ProcedureCall(spanned) => spanned.node.builtin.is_none(),
                        _ => false,
                    })
                })
            }
            _ => false,
        });

        assert!(has_none_builtin, "unknown procedure should have builtin == None");
    }
```

### Step 2.5: 运行测试验证

```bash
cargo test --lib -p ogsql-parser plpgsql::tests::test_pl_procedure_call_populates_builtin
cargo test --lib -p ogsql-parser plpgsql::tests::test_pl_procedure_call_builtin_none
cargo test --lib -p ogsql-parser visitor_tests::test_walk_pl_statement_procedure_call
cargo test --lib -p ogsql-parser visitor::tests::test_walk_pl_statement_procedure_call
```

**Expected:** 全部 PASS。

### Step 2.6: 运行回归与 roundtrip 测试

```bash
cargo test --all-features
```

**Expected:** 全部既有测试 PASS（验证 serde 向后兼容、无回归）。如 roundtrip 失败，检查是否漏加 `#[serde(default)]`。

### Step 2.7: 提交

```bash
cargo fmt --all
git add src/ast/plpgsql.rs src/parser/plpgsql.rs src/ast/visitor.rs src/ast/visitor_tests.rs
git commit -m "feat(parser): attach builtin metadata to PlProcedureCall

Extends PlProcedureCall with Option<BuiltinFuncMeta> populated via
function_registry::resolve_builtin_meta at all 3 parser construction
sites. Symmetric with Expr::FunctionCall.builtin and TableRef::FunctionCall.builtin.

Enables downstream visitors to distinguish built-in package procedure
calls (e.g. dbe_output.put_line) from user-defined procedures without
post-hoc lookup.

Closes part of #253."
```

---

## Task 3: 扩展 `CallFuncStatement` 结构并填充 builtin（2 个构造点）

**Files:**
- Modify: `src/ast/mod.rs:1784-1788`（struct 定义）
- Modify: `src/parser/utility/copy_explain.rs:508-555`（`parse_call`，两处构造点）
- Modify: `src/parser/utility/copy_explain.rs` 测试模块（新增集成测试）

### Step 3.1: 在 struct 定义中添加 `builtin` 字段

`src/ast/mod.rs:1784-1788` 当前：

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CallFuncStatement {
    pub func_name: ObjectName,
    pub args: Vec<CallArg>,
}
```

改为：

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CallFuncStatement {
    pub func_name: ObjectName,
    pub args: Vec<CallArg>,
    /// Built-in function metadata if this CALL targets a registered built-in;
    /// `None` for user-defined procedures. Populated symmetrically with
    /// `Expr::FunctionCall.builtin`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub builtin: Option<BuiltinFuncMeta>,
}
```

`BuiltinFuncMeta` 在同文件 line 47 已定义，直接引用无需 import。

### Step 3.2: 在 `parse_call` 中填充 builtin（2 处）

`src/parser/utility/copy_explain.rs:508-555` 当前完整函数：

```rust
    pub(crate) fn parse_call(&mut self) -> Result<CallFuncStatement, ParserError> {
        let func_name = self.parse_object_name()?;
        self.expect_token(&Token::LParen)?;

        if self.match_token(&Token::RParen) {
            self.advance();
            return Ok(CallFuncStatement { func_name, args: vec![] });
        }

        let mut args = Vec::new();
        loop {
            // ... named/positional arg parsing ...
        }
        Ok(CallFuncStatement { func_name, args })
    }
```

改为（在 `parse_object_name?` 之后立即 lookup 一次，两个 return 共用）：

```rust
    pub(crate) fn parse_call(&mut self) -> Result<CallFuncStatement, ParserError> {
        let func_name = self.parse_object_name()?;
        let builtin = crate::parser::function_registry::resolve_builtin_meta(&func_name);
        self.expect_token(&Token::LParen)?;

        if self.match_token(&Token::RParen) {
            self.advance();
            return Ok(CallFuncStatement { func_name, args: vec![], builtin });
        }

        let mut args = Vec::new();
        loop {
            // ... named/positional arg parsing 保持不变 ...
        }
        Ok(CallFuncStatement { func_name, args, builtin })
    }
```

**关键**：只修改函数头（加 `let builtin = ...`）和两个 return 语句的 struct literal。loop 体不动。

### Step 3.3: 添加集成测试

**路径已实测确认**：`Statement::Call(Spanned<CallFuncStatement>)`（line 815），因此 `call.node.builtin`。

在 `src/parser/utility/copy_explain.rs` 的 `#[cfg(test)] mod tests` 模块（若不存在则在文件末尾新建）追加：

```rust
    #[test]
    fn test_call_statement_populates_builtin_for_known_function() {
        use crate::Tokenizer;
        use crate::parser::Parser;
        use crate::ast::Statement;

        // abs 是 Math 域的已知内置函数（已实测 CALL abs(-1) 走 Statement::Call 路径）
        let sql = "CALL abs(-1)";
        let tokens = Tokenizer::new(sql).tokenize().expect("tokenize failed");
        let stmts = Parser::new(tokens).parse().expect("parse failed");

        let builtin = stmts.iter().find_map(|s| match s {
            Statement::Call(call) => call.node.builtin.as_ref(),
            _ => None,
        });

        let builtin = builtin.expect("builtin should be populated for CALL abs(...)");
        assert_eq!(builtin.domain, "Math");
    }

    #[test]
    fn test_call_statement_builtin_none_for_unknown_procedure() {
        use crate::Tokenizer;
        use crate::parser::Parser;
        use crate::ast::Statement;

        let sql = "CALL my_unknown_proc(42)";
        let tokens = Tokenizer::new(sql).tokenize().expect("tokenize failed");
        let stmts = Parser::new(tokens).parse().expect("parse failed");

        let builtin_none = stmts.iter().any(|s| match s {
            Statement::Call(call) => call.node.builtin.is_none(),
            _ => false,
        });

        assert!(builtin_none, "unknown procedure CALL should have builtin == None");
    }

    #[test]
    fn test_call_statement_with_empty_args_does_not_panic() {
        // 覆盖 copy_explain.rs:514 的空参数快路径——只验证不 panic 且 builtin 字段可读
        use crate::Tokenizer;
        use crate::parser::Parser;
        use crate::ast::Statement;

        let sql = "CALL pg_sleep()";
        let tokens = Tokenizer::new(sql).tokenize().expect("tokenize failed");
        let stmts = Parser::new(tokens).parse().expect("parse failed");

        // pg_sleep 是否在注册表中未知——无论结果，至少要验证空参路径不 panic
        let _builtin: Option<_> = stmts.iter().find_map(|s| match s {
            Statement::Call(call) => Some(call.node.builtin.clone()),
            _ => None,
        });
    }
```

### Step 3.4: 运行测试验证

```bash
cargo test --lib -p ogsql-parser utility::copy_explain::tests::test_call_statement
```

**Expected:** 全部 PASS。如 `Statement::Call` 路径错误，按编译器提示修正。

### Step 3.5: 运行完整测试套件验证无回归

```bash
cargo test --all-features
```

### Step 3.6: 提交

```bash
cargo fmt --all
git add src/ast/mod.rs src/parser/utility/copy_explain.rs
git commit -m "feat(parser): attach builtin metadata to CallFuncStatement

Extends CallFuncStatement (top-level CALL statement) with
Option<BuiltinFuncMeta>, populated via function_registry::resolve_builtin_meta
in parse_call. Symmetric with PlProcedureCall and Expr::FunctionCall.

Closes remaining part of #253."
```

---

## Task 4: 重构 `expr.rs::validate_func` 消除内联重复（纯 DRY）

**Files:**
- Modify: `src/parser/expr.rs:11-37`（`validate_func` 函数）

### Step 4.1: 替换内联 lookup 为调用 helper

`src/parser/expr.rs:11-37` 当前：

```rust
    fn validate_func(
        &mut self,
        name: &ObjectName,
        arg_count: usize,
        distinct: bool,
        has_over: bool,
        has_variadic: bool,
    ) -> Option<crate::ast::BuiltinFuncMeta> {
        let full_name = name.iter().map(|s| s.to_lowercase()).collect::<Vec<_>>().join(".");
        let last_seg = full_name.split('.').next_back().unwrap_or(&full_name).to_string();

        let builtin = crate::parser::function_registry::lookup_builtin_meta_qualified(&full_name)
            .or_else(|| crate::parser::function_registry::lookup_builtin_meta(&last_seg));

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

改为：

```rust
    fn validate_func(
        &mut self,
        name: &ObjectName,
        arg_count: usize,
        distinct: bool,
        has_over: bool,
        has_variadic: bool,
    ) -> Option<crate::ast::BuiltinFuncMeta> {
        let builtin = crate::parser::function_registry::resolve_builtin_meta(name);

        // Arg-count validation still needs last_seg for the warning message
        let last_seg = name
            .iter()
            .map(|s| s.to_lowercase())
            .collect::<Vec<_>>()
            .join(".")
            .split('.')
            .next_back()
            .unwrap_or("")
            .to_string();

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

**说明**：`last_seg` 仍需在本地计算，因为 `validate_function_call` 接受 `&str`（函数名单段）而非 ObjectName。若想进一步消除这个重复，可让 `function_registry` 暴露一个接受 `&ObjectName` 的 `validate_function_call_qualified`，但**本任务不做**（YAGNI，超出 issue #253 范围）。

### Step 4.2: 运行完整测试套件验证行为不变

```bash
cargo test --all-features
```

**Expected:** 全部 PASS。此重构是纯行为保持型，任何测试失败都说明 helper 实现与原内联逻辑不一致——回退并核对 `resolve_builtin_meta` 是否 100% 复刻原逻辑。

### Step 4.3: 提交

```bash
cargo fmt --all
git add src/parser/expr.rs
git commit -m "refactor(parser): use resolve_builtin_meta in validate_func

Replaces inline two-phase lookup in expr.rs::validate_func with a call
to the shared function_registry::resolve_builtin_meta helper. Behavior
preserved; arg-count validation still computes last_seg locally for the
warning-message path."
```

---

## Task 5: 最终 CI 全量验证

### Step 5.1: 运行 4 项 CI 检查

```bash
cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings
cargo test --all-features
cargo audit
```

**Expected**: 4 项全部通过。`cargo test --all-features` 必须显示 Task 1-4 新增的全部测试 PASS，且既有 1772+ 测试无回归。

### Step 5.2: 手工 smoke test（验证 JSON 输出）

```bash
# PL 块必须包在 DO $$ ... $$ 中
echo "DO \$\$ BEGIN dbe_output.put_line('hello'); END \$\$" | cargo run --bin ogsql --features cli -- parse -j 2>/dev/null | grep -A2 '"builtin"'
```

**Expected**: PlProcedureCall 节点下出现 `"builtin": {"category": "Scalar", "domain": "DbeOutput"}`（或类似 category 值）。

```bash
echo "CALL abs(-1)" | cargo run --bin ogsql --features cli -- parse -j 2>/dev/null | grep -A2 '"builtin"'
```

**Expected**: CallFuncStatement 节点下出现 `"domain": "Math"`。

### Step 5.3: 无需提交（仅验证）

Task 5 不产生新代码变更，只做最终把关。若 Step 5.1 任一失败，回到对应 Task 修复。

---

## 验收清单

- [ ] `PlProcedureCall` 和 `CallFuncStatement` 均带 `builtin: Option<BuiltinFuncMeta>` 字段
- [ ] 字段带 `#[serde(default, skip_serializing_if = "Option::is_none")]`（向后兼容 + 无 JSON 噪音）
- [ ] 5 个生产构造点全部调用 `resolve_builtin_meta` 填充
- [ ] 2 个测试构造点使用 `builtin: None`
- [ ] `function_registry` 新增 `resolve_builtin_meta` public helper
- [ ] `expr.rs:validate_func` 使用新 helper（无内联重复）
- [ ] 4 项 CI 全部通过：fmt / clippy / test / audit
- [ ] 新增至少 4 个单元测试（helper）+ 2 个集成测试（PlProcedureCall）+ 2 个集成测试（CallFuncStatement）
- [ ] 既有 1772+ 测试全部通过（无回归）
- [ ] 手工 smoke test 确认 JSON 输出包含 builtin 字段

---

## 风险与回退

| 风险 | 可能性 | 缓解 |
|---|---|---|
| `Statement::Call` 变体名不一致 | — | ✅ 已实测确认 `Call(Spanned<CallFuncStatement>)`（mod.rs:815） |
| `Statement::Do` / `DoStatement.block` 路径 | — | ✅ 已实测确认（mod.rs:814, 2361-2365）；`block: Option<PlBlock>` |
| serde `default` 仍导致旧 JSON 失败 | 低 | `tests/roundtrip.rs` 会立即捕获 |
| `pg_sleep` 未注册 | 低 | Task 3.3 第三个测试已注明"仅验证不 panic" |
| 4 个 test 构造点漏改导致编译失败 | 低 | 编译器会立即指出所有缺失点 |

**回退策略**：每个 Task 独立提交，若某 Task 出现回归，`git revert <sha>` 单个 commit 即可隔离回退，不影响其他 Task。

---

## 后续（非本 issue 范围）

- codeweb 侧：移除 `noise_rule` 中事后 `lookup_builtin_meta` 补查逻辑，改为在 `visit_procedure_call` / `visit_call` 入口直接读 `builtin` 字段（issue #253 的下游受益方）
- 可选增强：给 `PlProcedureCall` 和 `CallFuncStatement` 也加 arg-count 校验（类似 `validate_function_call`），但需先确认 GaussDB 是否对过程调用做 arg-count 检查
