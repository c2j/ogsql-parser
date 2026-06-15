# 项目结构与贡献指南

本文档描述 Metamorphosis 项目的仓库布局、crate 依赖关系、规则开发流程、测试策略以及提交规范。请在阅读下方的强制编码规则前先熟悉本节内容。

---

## Workspace 布局

```
metamorphosis/
├── crates/
│   ├── core/        # 引擎 + 抽象（types, traits, context, registry, engine, extractor）
│   ├── rules/       # 4 个内置重写规则 + eq_analyzer 共享模块
│   ├── cli/         # CLI 入口（5 个子命令：rewrite/suggest/show-rules/verify/mcp）
│   ├── qed/         # QED 离线验证（嵌入式 Z3，rich schema）
│   ├── verieql/     # 有界等价验证（独立，零 metamorphosis 依赖）
│   └── mcp-server/  # MCP 服务器（5 个工具，stdio 传输）
├── docs/            # 设计文档、贡献指南、最佳实践、实现计划
├── scripts/         # 安装脚本（install-qed-prover.sh, run-qed-verify.sh）
├── testcases/       # 手动测试用例
└── Cargo.toml       # Workspace 根配置
```

## 依赖关系（禁止反向依赖）

```
cli ──► rules ──► core ──► ogsql-parser
  └──► qed ──► core
  └──► mcp-server ──► rules, qed, verieql, core

verieql ──► ogsql-parser (独立，不依赖任何 metamorphosis crate)
```

关键约束：

- `core` 零外部 IO 依赖。
- `verieql` 完全独立，仅依赖 `ogsql-parser` 与 `z3`。
- `mcp-server` 依赖所有其他业务 crate。
- **禁止任何反向依赖**。

## 如何添加新规则

1. 在 `crates/rules/src/` 创建新文件，实现 `RewriteRule` trait。
2. 必须实现的方法：`id()`、`description()`、`category()`、`safety_level()`、`matches() -> MatchResult`、`apply() -> Option<RewriteAction>`。
3. 可选覆盖：`default_enabled() -> bool`。
4. 在 `crates/rules/src/lib.rs` 中注册。
5. 在 `crates/rules/tests/` 添加测试。
6. 如果规则产生 `Replace` 动作，建议添加 QED 等价性验证测试。

最小规则骨架：

```rust
use metamorphosis_core::{RewriteRule, RewriteContext, RewriteAction, SafetyLevel, RuleCategory, MatchResult};

#[derive(Debug)]
pub struct MyRule;

impl RewriteRule for MyRule {
    fn id(&self) -> &'static str { "my-rule" }
    fn description(&self) -> &'static str { "描述" }
    fn category(&self) -> RuleCategory { RuleCategory::Semantic }
    fn safety_level(&self) -> SafetyLevel { SafetyLevel::Safe }
    fn matches(&self, _ctx: &RewriteContext, stmt: &Statement) -> MatchResult {
        // 匹配逻辑
        MatchResult::Matched
    }
    fn apply(&self, ctx: &RewriteContext, stmt: &Statement) -> Option<RewriteAction> {
        // 重写逻辑
        None
    }
}
```

## 测试策略

- 测试金字塔：50% 规则单元测试，30% 引擎单元测试，20% 集成测试。
- 每个规则必须有独立的测试文件 `crates/rules/tests/`。
- Safe / Conditional 规则建议配合 QED E2E 测试验证语义等价性。
- 使用 `cargo test --workspace` 运行全部测试。
- QED E2E 测试运行：`scripts/run-qed-verify.sh`。

## 提交规范

- 采用 Conventional Commits：`feat:`、`fix:`、`docs:`、`test:`、`refactor:`、`chore:`、`style:`。
- 示例：`feat(rules): add eliminate-join-elimination rule`。
- `cargo fmt` 必须通过。
- `cargo clippy -- -D warnings` 必须通过。
- `Cargo.lock` 必须提交。

---

# 文档一：必须遵循的规则（Mandatory）

> **底线要求。不遵守这些规则将直接影响代码安全、可维护性、团队协作效率或生产稳定性。必须在 Code Review 和 CI 中强制检查。**

---

## 1. 项目架构与模块化

| 规则 | 要求 | 来源/依据 |
|------|------|----------|
| **M-ARCH-01** | 使用 Cargo Workspace 组织项目，按职责分层（`core` / `application` / `adapters` / `api`），禁止反向依赖。 | 工程实践 |
| **M-ARCH-02** | `core` 层必须零外部 IO 依赖，保证业务逻辑的平台无关性与可测试性。 | 工程实践 |
| **M-ARCH-03** | 单个 `.rs` 文件不得超过 **600 行**，理想控制在 **400 行以内**。超过必须拆分模块。 | 工程实践 |
| **M-ARCH-04** | 入口文件（`main.rs`、`lib.rs`）尽量不超过 **200 行**，仅做模块聚合与初始化。 | 工程实践 |
| **M-MOD-01** | 一个项目中**禁止混用**不同的模块布局风格（统一使用 `mod.rs` 或统一使用 `module.rs`）。 | G.MOD.04 |
| **M-MOD-02** | 不要在私有模块中将内部类型设为 `pub(crate)`，可见性必须逐层精确控制。 | G.MOD.05 |
| **M-MOD-03** | 作为库对外提供时，`lib.rs` 中必须重新导出对外公开的 API。 | G.MOD.02 |

---

## 2. 代码风格与格式化（CI 强制门禁）

| 规则 | 要求 | 来源/依据 |
|------|------|----------|
| **M-FMT-01** | **强制使用 `rustfmt`** 自动格式化代码，不接受人工风格争论。 | P.FMT.01 |
| **M-FMT-02** | 缩进使用空格而非制表符。 | P.FMT.02 |
| **M-FMT-03** | `extern` 外部函数必须显式指定 `"C"` ABI（`extern "C"`）。 | P.FMT.14 |
| **M-FMT-04** | 具名结构体字段初始化时**不得省略字段名**（除非变量名与字段名完全一致）。 | P.FMT.13 |
| **M-FMT-05** | 导入模块分组必须具有良好的可读性，禁止随便使用通配符 `*`。 | P.FMT.11 / G.MOD.03 |

---

## 3. 命名规范

| 规则 | 要求 | 来源/依据 |
|------|------|----------|
| **M-NAM-01** | 同一个 crate 中标识符命名必须使用**统一的词序**（如全用 `verb_noun` 或全用 `noun_verb`）。 | P.NAM.01 |
| **M-NAM-02** | getter 类方法**禁止使用 `get_` 前缀**（用 `name()` 而非 `get_name()`）。 | P.NAM.05 |
| **M-NAM-03** | 类型转换函数命名遵循所有权语义：`as_`（借用）、`to_`（可能分配）、`into_`（消耗所有权）。 | G.NAM.02 |
| **M-NAM-04** | 全局静态变量必须加前缀 `G_` 以便和常量区分。 | P.NAM.09 |
| **M-NAM-05** | 作用域越大命名越精确，反之应简短。 | P.NAM.04 |

---

## 4. 类型系统与数据安全

| 规则 | 要求 | 来源/依据 |
|------|------|----------|
| **M-TYP-01** | 类型转换**禁止使用裸 `as`**，必须使用安全的转换函数（`try_from`、`into` 等）。 | G.TYP.01 |
| **M-TYP-02** | 数字字面量**必须明确标注类型**（如 `42u64`）。 | G.TYP.02 |
| **M-TYP-03** | 对外导出的公开 Struct 和 Enum **必须添加 `#[non_exhaustive]`**。 | G.TYP.SCT.01 / G.TYP.ENM.05 |
| **M-TYP-04** | 结构体中**超过 3 个布尔字段**时，必须将其独立为新的枚举类型。 | G.TYP.SCT.02 |
| **M-TYP-05** | 禁止将数字类型转换为布尔值，禁止用数字代替布尔值。 | G.TYP.BOL.03 / G.TYP.BOL.06 |
| **M-TYP-06** | 使用数组索引时必须确保不越界，禁止依赖数组边界检查来 Panic。 | G.TYP.ARR.02 / G.EXP.03 |
| **M-TYP-07** | 元组元素不宜超过 3 个，超过应使用结构体。 | G.TYP.TUP.01 |

---

## 5. 错误处理（强制底线）

| 规则 | 要求 | 来源/依据 |
|------|------|----------|
| **M-ERR-01** | **库代码（lib）禁止返回 `anyhow` 等不透明错误**，必须定义具体的错误类型（使用 `thiserror`）。 | 工程实践 |
| **M-ERR-02** | **禁止在库代码中使用 `unwrap()`**。应用代码（bin）也须极度克制。 | G.ERR.01 |
| **M-ERR-03** | 确定不可能为 `None`/`Err` 时，可使用 `expect()`，但信息必须说明"为什么不会失败"。 | P.ERR.02 |
| **M-ERR-04** | 当传入参数超出限制可能导致函数失败时，**必须使用断言（`assert!`）**。 | P.ERR.01 |
| **M-ERR-05** | 公开的返回 `Result` 的函数文档中**必须增加 Error 注释**；可能 Panic 的必须增加 Panic 注释。 | G.CMT.01 / G.CMT.02 |
| **M-ERR-06** | 实现 `From` 而非 `Into`（因为 `Into` 有默认实现）。 | G.TRA.BLN.08 |

---

## 6. 并发与异步（安全底线）

| 规则 | 要求 | 来源/依据 |
|------|------|----------|
| **M-ASY-01** | **禁止在异步块/函数中持有同步互斥锁（`MutexGuard`）跨越 `await` 点**。 | G.ASY.02 |
| **M-ASY-02** | **禁止在异步块/函数中持有 `RefCell` 引用跨越 `await` 点**。 | G.ASY.03 |
| **M-ASY-03** | 异步函数中**禁止包含阻塞操作**（文件 IO、密集计算必须使用 `spawn_blocking`）。 | G.ASY.05 |
| **M-ASY-04** | 异步运行时（tokio/async-std）一旦选定，**全局统一，禁止混用**。 | 工程实践 |
| **M-MTH-01** | 对布尔或引用的并发访问**必须使用原子类型**，禁止用互斥锁。 | G.MTH.LCK.01 |
| **M-MTH-02** | 多线程下必须识别锁争用情况，避免死锁。 | P.MTH.LCK.01 |

---

## 7. Unsafe Rust（绝对红线）

| 规则 | 要求 | 来源/依据 |
|------|------|----------|
| **M-UNS-01** | **禁止为了逃避编译器检查而滥用 Unsafe**。 | P.UNS.01 |
| **M-UNS-02** | **任何 `unsafe` 块之前必须加 `SAFETY` 注释**，说明为什么此处是安全的。 | P.UNS.SAS.09 |
| **M-UNS-03** | 公开的 `unsafe` 函数文档中**必须增加 Safety 注释**。 | G.UNS.SAS.01 |
| **M-UNS-04** | Unsafe 函数中校验边界条件必须使用 `assert!`，**禁止使用 `debug_assert!`**。 | G.UNS.SAS.02 |
| **M-UNS-05** | 禁止在公开 API 中暴露未初始化内存和裸指针。 | P.UNS.SAS.03 / P.UNS.SAS.06 |
| **M-UNS-06** | 禁止将不可变指针手工转换为可变指针。 | G.UNS.PTR.02 |
| **M-UNS-07** | 禁止将裸指针在多线程间共享。 | P.UNS.PTR.01 |

---

## 8. FFI（如涉及 C 互操作）

| 规则 | 要求 | 来源/依据 |
|------|------|----------|
| **M-FFI-01** | 跨越 FFI 边界的函数**必须处理 Panic**（使用 `catch_unwind`）。 | P.UNS.FFI.04 |
| **M-FFI-02** | 使用 `libc` 或标准库提供的可移植类型别名，禁止直接使用平台特定类型。 | P.UNS.FFI.05 |
| **M-FFI-03** | 禁止为传出外部的类型实现 `Drop`。 | P.UNS.FFI.07 |
| **M-FFI-04** | 依赖 C 端传入的参数时，文档中必须声明不变性，并进行合法性检查。 | P.UNS.FFI.12 / P.UNS.FFI.15 |
| **M-FFI-05** | 自定义数据类型必须保证与 C 端一致的数据布局（`#[repr(C)]`）。 | P.UNS.FFI.13 |

---

## 9. 日志与可观测性（底线）

| 规则 | 要求 | 来源/依据 |
|------|------|----------|
| **M-LOG-01** | **统一使用 `tracing`**，禁止使用 `log`。 | 工程实践 |
| **M-LOG-02** | 生产环境日志**必须输出结构化 JSON**，禁止纯文本格式。 | 工程实践 |
| **M-LOG-03** | 日志级别语义必须统一：`ERROR`（需告警）、`WARN`（可自愈异常）、`INFO`（关键生命周期）、`DEBUG/TRACE`（开发调试）。 | 工程实践 |
| **M-LOG-04** | **严禁在日志中记录敏感信息**（密码、Token、PII），必须使用脱敏或 `[REDACTED]`。 | 工程实践 |
| **M-LOG-05** | 每个外部请求入口必须创建 Span，包含 `trace_id` / `request_id`。 | 工程实践 |
| **M-LOG-06** | `ERROR` 级别日志必须包含可行动的上下文（哪里、为什么、影响范围），禁止仅记录 `?err`。 | 工程实践 |

---

## 10. 依赖与构建

| 规则 | 要求 | 来源/依据 |
|------|------|----------|
| **M-DEP-01** | `Cargo.toml` 中依赖版本**禁止使用通配符 `*`**。 | G.CAR.04 |
| **M-DEP-02** | 应用项目必须将 `Cargo.lock` 提交到版本控制。 | 工程实践 |
| **M-DEP-03** | 必须声明 `rust-version`（MSRV）并在 CI 中验证。 | 工程实践 |
| **M-DEP-04** | 使用 `cargo-deny` 在 CI 中检查许可证、安全漏洞（`RUSTSEC`）和禁止的依赖。 | 工程实践 |
| **M-DEP-05** | 使用 `cargo features` 进行条件编译，**禁止使用 `--cfg`**。 | P.CAR.03 |

---

## 11. 文档与注释

| 规则 | 要求 | 来源/依据 |
|------|------|----------|
| **M-DOC-01** | 所有 `pub` API 必须有文档注释，`cargo doc` 无警告。 | 工程实践 |
| **M-DOC-02** | 文档注释中**使用空格代替 tab**。 | G.CMT.03 |
| **M-DOC-03** | 优先使用行注释 `//`，避免使用块注释 `/* */`。 | P.CMT.03 |
| **M-DOC-04** | 代码中保留的 `FIXME` / `TODO` 必须通过任务系统跟踪，禁止无跟踪长期遗留。 | P.CMT.05 |

---

## 12. 信息安全

| 规则 | 要求 | 来源/依据 |
|------|------|----------|
| **M-SEC-01** | 引入第三方库前必须评估维护活跃度、下载量、依赖树深度，防范供应链攻击。 | P.SEC.01 |
| **M-SEC-02** | 代码中禁止出现非法 Unicode 字符（如双向覆盖字符）。 | G.SEC.01 |
---

## 使用建议

1. **文档一（必须遵循）**应直接写入团队的 `CONTRIBUTING.md`，并在 CI 中配置对应的检查工具（`rustfmt`、`clippy`、`cargo-deny`、`cargo-semver-checks` 等）。
2. 文档应**每半年评审一次**，根据项目演进和 Rust 生态发展进行更新。

---

## 附录：实施经验与补充指导（2026-06 审计反馈）

> 以下内容基于 ogsql-parser 项目的实际合规整改经验补充。对已有外部消费者的项目尤其有参考价值。

### A1. M-TYP-03（`#[non_exhaustive]`）的存量处理策略

**问题**：对已通过 git 引用被外部消费的 pub 类型，回头加 `#[non_exhaustive]` 是**破坏性变更**——消费者的结构体字面量和 match 表达式会编译失败。

**分阶段策略**：

| 阶段 | 做法 | 破坏性 |
|------|------|--------|
| 新增类型 | 一律加 `#[non_exhaustive]` | 无 |
| 存量类型 | 文档标注 `# API Stability`，引导消费者使用 `Default::default()` + `..` 更新语法 | 无 |
| 存量类型 | 提供 `Builder` 或 `new()` 构造器，逐步替代直接构造 | 无 |
| 下个大版本（1.0） | 统一为所有 pub 类型补加 `#[non_exhaustive]` | 有（一次性破坏窗口） |

### A2. M-ERR-02（禁止 unwrap）的存量清理路径

**问题**：已有代码库可能包含数百处 `.unwrap()`，一次性清理不现实。

**三层清理策略**：

```
Tier 1: 私有函数内的 unwrap
  → 直接替换为 ? / unwrap_or / unwrap_or_else
  → 零 API 影响

Tier 2: 公开函数内部路径的 unwrap
  → 改实现细节，保持函数签名不变
  → 零 API 影响

Tier 3: 公开函数因 unwrap 而隐式 panic
  → 第一步：加 # Panics 文档标注（告知消费者触发条件）
  → 第二步：在下个版本中改为返回 Result（破坏性，需配合 deprecation）
```

**强制要求**：禁止在 crate 级别 `#![allow(clippy::unwrap_used)]` 而不提供清理计划。每次审查 allow 列表时，必须确认 unwrap 数量在递减。

### A3. M-FMT-05（通配符导入）的务实例外

**问题**：当模块导出超过 50 个类型时（如 AST 定义模块），在每个消费文件中显式列出全部导入会严重影响可读性和维护效率。

**允许的例外**：
- **同 crate 内**消费自身的"命名空间级"模块（如 `use crate::ast::*;`）可保留通配符，但需满足：
  1. 被导入的模块本身是纯数据定义模块（无逻辑、无副作用）
  2. 文件中实际使用了该模块中超过 20 个类型
  3. 在文件头部的模块注释中标注原因
- **跨 crate 引用**时一律禁止通配符导入。
- **非数据定义模块**（如 `use crate::parser::*;`）禁止通配符。

### A4. M-DEP-03（MSRV）的验证要求

**问题**：仅声明 `rust-version` 而不在 CI 中验证，会导致 MSRV 形同虚设。本次审计发现代码使用了 `Option::is_none_or`（Rust 1.82 稳定）但 MSRV 声明为 1.70。

**强制要求**：
1. 声明 `rust-version` 后，必须在 CI 中添加 MSRV 验证 job。
2. 推荐使用 [`cargo-msrv`](https://github.com/foresterre/cargo-msrv) 定期验证。
3. 升级依赖时必须检查是否引入了超出 MSRV 的 API。

### A5. clippy `#![allow]` 的使用规范（新增实践）

**问题**：无注释的 `#![allow(...)]` 会让后续开发者无法判断该豁免是否仍有必要。

**强制要求**：

1. 每个 `#![allow(...)]` 条目必须附带注释说明**为什么需要该豁免**。
2. 区分"永久豁免"（领域特性导致）和"临时豁免"（存量技术债务）：
   - 永久豁免：注释标注 `[PERMANENT]` 并说明领域原因。
   - 临时豁免：注释标注 `[TODO: cleanup]` 并关联 issue 编号。
3. 每季度审查一次 allow 列表，移除已修复的临时豁免。
4. 推荐分批移除临时豁免（见 BEST-PRATICE.md 附录 B4）。

**示例**：

```rust
#![allow(
    // [PERMANENT] Parser pattern: matching different keywords/tokens often
    // leads to identical handling. Each branch is intentionally separate
    // for grammar-rule readability.
    clippy::if_same_then_else,
    // [TODO: cleanup] 193 instances remaining, see issue #XX
    clippy::unwrap_used,
)]
```

### A6. M-ARCH-03（文件行数限制）的领域适配

**问题**：解析器、语法规则等领域特定代码可能合理地超过 600 行限制。

**补充指导**：

| 行数范围 | 要求 |
|----------|------|
| ≤ 600 | 理想目标，无需操作 |
| 601–1000 | 应在 Code Review 中评估是否可拆分 |
| 1001–2000 | 必须拆分，或提供书面理由（如：单文件语法规则不可分割） |
| > 2000 | 强制拆分，无例外 |

**拆分策略**（保持公开 API 不变）：
1. 按职责拆分子模块（如 `parser/ddl/create.rs`, `parser/ddl/alter.rs`）。
2. 在父模块 `mod.rs` 中通过 `pub use` 重新导出，保持外部可见性不变。
3. 拆分后逐文件验证 `cargo doc` 无新增警告。

---

## 相关文档

- 设计文档：`docs/metamorphosis_design_doc.md`
- 最佳实践：`docs/BEST-PRATICE.md`
- 用户手册：`docs/UserGuide.md`
- 开发者指南：`docs/DeveloperGuide.md`
- QED 理论：`docs/QED.md`
