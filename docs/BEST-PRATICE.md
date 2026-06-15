
# 文档二：可选遵循的规则（Recommended / Optional）

> **质量提升建议。根据项目阶段、团队成熟度和性能要求选择性采纳。不强制在 CI 中阻断，但应在 Code Review 中鼓励。**

---

## 1. 架构与代码组织

| 规则 | 建议 | 来源/依据 |
|------|------|----------|
| **R-ARCH-01** | 使用类型状态机（Type State Pattern）编码合法状态流转，防止非法状态出现。 | 工程实践 |
| **R-ARCH-02** | 使用 Newtype 模式（如 `struct UserId(Uuid)`）防止基本类型滥用。 | 工程实践 |
| **R-ARCH-03** | 使用 Sealed Trait 模式防止外部实现你不希望扩展的接口。 | 工程实践 |
| **R-ARCH-04** | 将模块的测试代码移动到单独的文件（`tests/` 目录或独立 `#[cfg(test)]` 文件），提升编译速度。 | P.MOD.02 |
| **R-ARCH-05** | 使用消息传递（channel/Actor）替代共享可变状态，降低锁复杂度。 | 工程实践 |

---

## 2. 代码风格与可读性

| 规则 | 建议 | 来源/依据 |
|------|------|----------|
| **R-FMT-01** | 多行表达式操作时，操作符置于行首（方便 diff 阅读）。 | P.FMT.06 |
| **R-FMT-02** | 函数参数超过 5 个或导入模块超过 4 个时换行。 | P.FMT.08 |
| **R-FMT-03** | 枚举变体和结构体字段左对齐。 | P.FMT.07 |
| **R-FMT-04** | 不要将派生宏中多个不相关的 trait 合并为同一行。 | P.FMT.16 |
| **R-FMT-05** | 解构元组时允许使用 `..` 指代剩余元素。 | P.FMT.15 |

---

## 3. 命名与 API 设计

| 规则 | 建议 | 来源/依据 |
|------|------|----------|
| **R-NAM-01** | 标识符命名符合阅读习惯，避免无意义占位词。 | P.NAM.03 |
| **R-NAM-02** | 遵循 `iter` / `iter_mut` / `into_iter` 规范生成迭代器。 | P.NAM.06 |
| **R-NAM-03** | 避免使用语言内置保留字、关键字作为标识符。 | P.NAM.07 |
| **R-API-01** | 函数参数超过 5 个时，考虑封装为结构体。 | G.FUD.01 |
| **R-API-02** | 函数参数出现太多 `bool` 时，封装为自定义结构体或枚举。 | G.FUD.03 |
| **R-API-03** | 函数参数若实现 `Copy` 且值较小（如 `u64` 以下），优先按值传入而非引用。 | G.FUD.04 |
| **R-API-04** | 不要总是为函数指定 `#[inline(always)]`，让编译器自行决定。 | G.FUD.05 |
| **R-API-05** | 函数返回值不要使用 `return`（提前返回除外）。 | P.FUD.02 |

---

## 4. 类型系统精细化

| 规则 | 建议 | 来源/依据 |
|------|------|----------|
| **R-TYP-01** | 必要时使用自定义类型表达更明确的语义，而非直接使用原生类型。 | P.TYP.01 |
| **R-TYP-02** | 使用切片迭代器代替手工索引。 | P.TYP.SLC.01 |
| **R-TYP-03** | 使用切片模式提升代码可读性。 | P.TYP.SLC.02 |
| **R-TYP-04** | 使用结构体功能更新语法（`..default()`）提升可读性。 | G.TYP.SCT.03 |
| **R-TYP-05** | Enum 内变体的大小差异不宜过大（避免内存浪费）。 | G.TYP.ENM.06 |
| **R-TYP-06** | 如需依赖 Enum 变体的序数，应显式为变体设置明确的数值。 | G.TYP.ENM.07 |
| **R-TYP-07** | 不宜在 `use` 语句中引入 Enum 的全部变体（避免命名空间污染）。 | G.TYP.ENM.04 |
| **R-TYP-08** | 科学计算中涉及浮点数近似值的常量，宜使用预定义常量（如 `std::f64::consts::PI`）。 | G.CNS.01 |
| **R-TYP-09** | 对于适用 `const fn` 的函数或方法，宜尽可能使用 `const fn`。 | G.CNS.05 |

---

## 5. 集合与字符串优化

| 规则 | 建议 | 来源/依据 |
|------|------|----------|
| **R-COL-01** | 创建 `Vec` / `HashMap` / `VecDeque` / `String` 时，宜预先分配足够容量，避免多次分配。 | P.TYP.VEC.02 / P.STR.02 / P.CLT.01 |
| **R-COL-02** | 非必要情况下不要使用 `LinkedList`，而用 `Vec` 或 `VecDeque` 代替。 | G.CLT.01 |
| **R-STR-01** | 处理字符串元素时优先按字节处理而非字符（当已知为 ASCII 时）。 | P.STR.01 |
| **R-STR-02** | 拼接字符串时优先使用 `format!`。 | P.STR.05 |
| **R-STR-03** | 追加字符串时使用 `push_str` 方法。 | G.STR.02 |

---

## 6. 错误处理精细化

| 规则 | 建议 | 来源/依据 |
|------|------|----------|
| **R-ERR-01** | 不要滥用 `expect`，优先考虑 `unwrap_or` / `unwrap_or_default` / `unwrap_or_else` 系列。 | G.ERR.02 |
| **R-ERR-02** | 应用代码（bin）可自由使用 `anyhow` 或 `eyre` 进行错误兜底与上下文传播。 | 工程实践 |
| **R-ERR-03** | 跨越架构边界时，使用 `From` 转换错误，避免在业务层写 `map_err`。 | 工程实践 |

---

## 7. 并发与异步优化

| 规则 | 建议 | 来源/依据 |
|------|------|----------|
| **R-MTH-01** | 宜使用 `parking_lot` 替代标准库 `std::sync` 中的同步原语（更快、更健壮）。 | G.MTH.LCK.03 |
| **R-MTH-02** | 宜使用 `crossbeam` 替代标准库 `std::sync::mpsc`（性能更好、支持 select）。 | G.MTH.LCK.04 |
| **R-MTH-03** | 宜使用 `Arc<[T]>` 代替 `Arc<Vec<T>>`（避免双重间接）。 | G.MTH.LCK.02 |
| **R-ASY-01** | 异步编程并不适合所有场景，计算密集型场景应考虑同步编程。 | P.ASY.01 |
| **R-ASY-02** | 避免定义不必要的异步函数（纯计算型函数不应 async）。 | G.ASY.04 |
| **R-ASY-03** | `Mutex` 内保护的数据应足够小，避免长时间持有锁；`RwLock` 适用于读多写少场景。 | 工程实践 |
| **R-ASY-04** | 所有长时间运行的异步操作必须正确处理 `tokio::select!` 中的取消信号。 | 工程实践 |

---

## 8. 泛型与 Trait

| 规则 | 建议 | 来源/依据 |
|------|------|----------|
| **R-GEN-01** | 用泛型来抽象公共语义，但泛型参数和 trait 限定不宜过多（影响编译时间）。 | P.GEN.01 / P.GEN.03 |
| **R-GEN-02** | 不要随便使用 `impl Trait` 语法替代泛型限定（公开 API 中尤其注意）。 | P.GEN.02 |
| **R-GEN-03** | 为泛型类型实现方法时，`impl` 中声明的泛型参数一定要被用到。 | P.GEN.04 |
| **R-TRA-01** | 根据场景合理选择使用 trait 对象（动态分发）或泛型（静态分发）。 | P.TRA.OBJ.01 |
| **R-TRA-02** | 除非必要，避免自定义虚表。 | P.TRA.OBJ.02 |
| **R-TRA-03** | 不要随便使用 `Deref` trait 来模拟继承。 | G.TRA.BLN.10 |
| **R-TRA-04** | 对实现 `Copy` 的可迭代类型，要通过迭代器拷贝所有元素时，使用 `copied()` 而非 `cloned()`。 | G.TRA.BLN.07 |
| **R-TRA-05** | 一般情况下不要给 `Copy` 类型手工实现 `Clone`（派生即可）。 | G.TRA.BLN.09 |
| **R-TRA-06** | 使用派生宏自动实现 `Default` trait，不要手工实现。 | G.TRA.BLN.03 |

---

## 9. Unsafe 进阶

| 规则 | 建议 | 来源/依据 |
|------|------|----------|
| **R-UNS-01** | 不要为了提升性能而盲目使用 Unsafe Rust。 | P.UNS.02 |
| **R-UNS-02** | 建议使用 `NonNull<T>` 替代 `*mut T`（保证非空，允许编译器优化）。 | P.UNS.PTR.02 |
| **R-UNS-03** | 使用指针类型构造泛型结构体时，使用 `PhantomData<T>` 指定协变和所有权。 | P.UNS.PTR.03 |
| **R-UNS-04** | 尽量使用 `pointer::cast` 代替 `as` 强转指针。 | G.UNS.PTR.03 |
| **R-UNS-05** | 除了与 C 交互，尽量不要使用 Union。 | P.UNS.UNI.01 |
| **R-UNS-06** | 使用 `MaybeUninit<T>` 处理未初始化的内存。 | G.UNS.MEM.01 |
| **R-UNS-07** | 在抽象安全方法的同时，建议为性能考虑增加相应的 `unsafe` 方法（如 `get_unchecked`）。 | P.UNS.SAS.07 |

---

## 10. 宏与代码生成

| 规则 | 建议 | 来源/依据 |
|------|------|----------|
| **R-MAC-01** | 不要轻易使用宏（增加认知负担、破坏 IDE 体验）。 | P.MAC.01 |
| **R-MAC-02** | `dbg!()` 宏只应该用于调试代码，禁止提交到生产代码。 | G.MAC.01 |
| **R-MAC-03** | 实现宏语法时尽量贴近 Rust 原生语法。 | P.MAC.02 |
| **R-MAC-04** | 使用宏时考虑宏展开对编译文件体积的影响。 | G.MAC.02 |
| **R-CGN-01** | 代码生成按情况选择使用过程宏还是 `build.rs`。 | P.CGN.01 |
| **R-CGN-02** | `build.rs` 生成的代码必须保证没有任何警告。 | P.CGN.02 |

---

## 11. I/O 与性能

| 规则 | 建议 | 来源/依据 |
|------|------|----------|
| **R-IO-01** | 文件读取建议使用 `BufReader` / `BufWriter` 替代裸 `Reader` / `Writer`。 | G.FIO.01 |
| **R-IO-02** | 使用 `read_to_end` / `read_to_string` 时注意文件大小能否一次性读入内存。 | P.FIO.01 |
| **R-PERF-01** | 使用 `Cow<'a, B>` 选择合理场景以最大化优化性能。 | P.STR.04 |
| **R-PERF-02** | 使用标准库内置方法处理浮点数计算。 | G.TYP.FLT.04 |

---

## 12. 测试与质量

| 规则 | 建议 | 来源/依据 |
|------|------|----------|
| **R-TST-01** | 使用依赖注入（Trait 抽象）提升可测试性，测试时注入 Mock 实现。 | 工程实践 |
| **R-TST-02** | 对解析器、序列化逻辑使用 `proptest` 进行属性测试。 | 工程实践 |
| **R-TST-03** | 对 FFI 边界或 unsafe 代码使用 `cargo-fuzz` 进行模糊测试。 | 工程实践 |
| **R-TST-04** | 使用 `cargo-semver-checks` 在 CI 中自动检测 API 兼容性（库项目）。 | 工程实践 |
| **R-TST-05** | 使用 `cargo-udeps` 检测未使用的依赖。 | 附录 E |

---

## 13. 日志与可观测性（增强）

| 规则 | 建议 | 来源/依据 |
|------|------|----------|
| **R-OBS-01** | 使用 `tracing-appender` 的非阻塞 writer，避免磁盘 IO 阻塞 async 运行时。 | 工程实践 |
| **R-OBS-02** | 对 `DEBUG` 级别日志在高流量场景下进行采样（如 1%），防止日志风暴。 | 工程实践 |
| **R-OBS-03** | 使用 `metrics` crate 暴露关键业务指标（QPS、延迟、错误率）。 | 工程实践 |
| **R-OBS-04** | 在 `api` 层统一注入 trace-id，实现全链路追踪。 | 工程实践 |
| **R-OBS-05** | 制定日志分级保留策略：ERROR 永久、INFO 30 天、DEBUG 7 天。 | 工程实践 |

---

## 14. 文档与工程文化

| 规则 | 建议 | 来源/依据 |
|------|------|----------|
| **R-DOC-01** | 维护架构决策记录（ADR），记录重大技术选型上下文。 | 工程实践 |
| **R-DOC-02** | 在 `lib.rs` 或 `README.md` 中提供架构概览图。 | 工程实践 |
| **R-DOC-03** | 制定 Rust Edition 迁移策略（新版本发布后 6-12 个月内评估迁移）。 | 工程实践 |
| **R-DOC-04** | 代码能做到自注释，文档要干练简洁。 | P.CMT.01 |
| **R-DOC-05** | 注释应有宽度限制（建议 80~100 列）。 | P.CMT.02 |

---

## 使用建议

1. **文档二（可选遵循）**作为团队内部的 **Rust 最佳实践手册**，在新人培训和 Code Review 中引用。
2. 文档应**每半年评审一次**，根据项目演进和 Rust 生态发展进行更新。

---

## 附录 B：实施经验与补充指导（2026-06 审计反馈）

> 以下内容基于 ogsql-parser 项目的实际合规整改经验补充。

### B1. 已有外部消费者的 API 演进策略

**场景**：库已通过 git 引用被外部产品消费，API 变更需要向后兼容。

**原则**：新增合规，存量渐进。

| 场景 | 做法 |
|------|------|
| 新增 pub 类型 | 一律加 `#[non_exhaustive]`，一律提供 `Default` 或 `Builder` |
| 新增 pub 函数 | 一律返回 `Result`，禁止隐式 panic |
| 废弃旧 API | `#[deprecated(since = "0.x", note = "use `new_api` instead")]`，保留旧 API 至下个大版本 |
| 破坏性变更 | 积累到 1.0 版本统一处理，提前在 CHANGELOG 中预告 |

### B2. `expect()` 消息质量标准

好的 expect 消息回答 **"为什么不会失败"**，而非 **"正在做什么"**。

```rust
// 好 — 解释了不变式
phf_map.get(key).expect("phf map is compile-time built; all keys are known at build time")

// 好 — 解释了前置条件已保证
tokens.last().expect("parse() guarantees at least one token (EOF sentinel)")

// 坏 — 只描述了操作，没解释为什么安全
keyword.expect("parse keyword")

// 坏 — 空消息
result.expect("")
```

### B3. 领域特定的 lint 策略

不同领域的代码可能需要不同的 clippy 策略。以下是常见例外场景：

| 领域 | 可考虑 allow 的 lint | 理由 |
|------|---------------------|------|
| 解析器 / 编译器 | `if_same_then_else` | 不同 token 匹配常导致相同处理逻辑 |
| 解析器 / 编译器 | `too_many_arguments` | 递归下降解析器的上下文传递 |
| FFI 绑定 | `missing_safety_doc` | 自动生成的绑定 |
| 测试代码 | `unwrap_used` | 测试中 panic 是期望行为 |

**注意**：每个 allow 仍需注释说明，且应定期审查是否可移除。

### B4. clippy 豁免清理方法论

清理已有 `#![allow(...)]` 时，推荐分三批进行：

**第一批：纯风格类（低风险，可自动修复）**
```
needless_return, let_and_return, redundant_closure, useless_format,
format_in_format_args, collapsible_if, collapsible_match, single_match,
needless_late_init, unnecessary_to_owned
```
→ 使用 `cargo clippy --fix --lib --allow-dirty` 自动修复大部分。
→ 手动修复剩余（通常涉及 match 重构）。
→ 运行完整测试套件验证。

**第二批：需手动修复（中风险）**
```
while_let_loop, field_reassign_with_default, manual_strip,
match_like_matches_macro, bind_instead_of_map, map_unwrap_or
```
→ 逐个文件手动修复。
→ 每个 lint 修复后运行 `cargo clippy` 确认该 lint 清零。

**第三批：高风险（需逐个审查）**
```
unwrap_used, large_enum_variant, ptr_arg, should_implement_trait,
unnecessary_literal_unwrap, result_large_err
```
→ 可能涉及 API 变更或内存布局调整。
→ 每个 fix 需要 Code Review。
→ 建议关联 issue 跟踪进度。

**每批完成后**：
1. `cargo fmt --all -- --check` 必须通过。
2. `cargo clippy` 无新增警告。
3. `cargo test --all-features` 必须通过。

### B5. 通配符导入（`use ...::*;`）的判断流程

```
是否跨 crate？
  ├─ 是 → 禁止通配符，必须显式导入
  └─ 否 → 被导入的模块性质？
           ├─ 纯数据定义（如 AST 类型）→ 可保留，加注释
           ├─ 含逻辑/副作用 → 禁止通配符
           └─ 使用类型 < 20 个 → 禁止通配符，显式导入
```

### B6. 大文件拆分的安全检查清单

拆分超过 600 行的文件时，确保：

- [ ] 拆分前后 `cargo doc --no-deps` 输出无差异（公开 API 不变）
- [ ] 拆分前后 `cargo test` 全部通过
- [ ] 拆分前后 `grep -r 'pub use' src/lib.rs` 的导出列表不变
- [ ] 新文件的模块声明（`mod xxx;`）已添加到父 `mod.rs`
- [ ] 如有 `pub(crate)` 项，确认跨模块可见性未因拆分而中断
