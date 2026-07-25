# 跨文件解析增强 实施计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 修复 9 个失败的跨文件/控制流诊断测试，将通过率从 82/91 提升到 89+/91。

**Architecture:** 在现有 `CrossFileState` 基础上，新增 `field_access` 常量解析、参数名替换、传递式方法行为记录、继承方法导入四项能力。所有改动集中在 `extract.rs`、`method_call.rs`、`variable.rs`、`mod.rs`、`types.rs` 五个文件。

**Tech Stack:** Rust, tree-sitter-java

---

## 当前状态

| 分组 | 通过 | 失败 | 说明 |
|------|------|------|------|
| A-J (原始) | 40/40 | 0 | P0/P1/P2 改进全部通过 |
| K-O (控制流) | 32/34 | 2 | m4(Java14 switch), o10(return SQL) |
| P-Q (跨文件) | 10/17 | 7 | 本次目标 |
| **总计** | **82/91** | **9** | |

## 失败根因分类

| 根因 | 影响测试 | 修复复杂度 | ROI |
|------|---------|-----------|-----|
| A: `field_access` 常量查找缺失 | P4, P5, P6, P11 | 低 | **最高（4 测试）** |
| B: 跨文件 PS 参数名替换 | P7 | 中 | 高 |
| C: 传递式方法行为记录 | P8 | 中 | 高 |
| D: 继承方法导入 | P12 | 高 | 中 |
| E: Java14 switch 表达式 | m4 | 中 | 低（边缘场景） |
| F: return 语句中的 SQL | o10 | 中 | 低（边缘场景） |

## 修复优先级

| 优先级 | 修复项 | 预期效果 | 预计通过率 |
|--------|--------|---------|-----------|
| **XP0** | field_access 常量解析 | +4 测试 | 86/91 (95%) |
| **XP1** | 跨文件参数名映射 | +1 测试 | 87/91 (96%) |
| **XP2** | 传递式方法行为 | +1 测试 | 88/91 (97%) |
| **XP3** | 继承方法导入 | +1 测试 | 89/91 (98%) |
| ~~XP4~~ | ~~Java14 switch~~ | ~~+1 测试~~ | 暂缓 |
| ~~XP5~~ | ~~return SQL~~ | ~~+1 测试~~ | 暂缓 |

---

## Task 1: `field_access` 常量解析 (XP0)

**目标:** 让 `string_constants` 能解析 `SqlConstants.USER_QUERY` 形式的跨文件常量引用。

**原理:** `string_constants` HashMap 的 key 当前是简单变量名（如 `USER_QUERY`）。跨文件场景中 tree-sitter 生成 `field_access` 节点，结构为：

```
field_access "SqlConstants.USER_QUERY"
  identifier "SqlConstants"
  . "."
  identifier "USER_QUERY"
```

只需要取最后一段 `identifier`（`USER_QUERY`）去 `string_constants` 中查找即可。

**Files:**
- Modify: `src/java/extract.rs` — `collect_concat_parts` (2处 identifier 分支)
- Modify: `src/java/method_call.rs` — `find_first_string_arg` (1处 identifier 分支)

**Step 1: 添加 `resolve_field_access_constant` 辅助函数**

在 `extract.rs` 的 `ExtractContext` impl 中添加：

```rust
/// 从 field_access 节点（如 `SqlConstants.USER_QUERY`）提取最后一段标识符，
/// 然后在 string_constants 中查找对应的字符串值。
fn resolve_field_access_constant(&self, node: Node) -> Option<String> {
    if node.kind() != "field_access" {
        return None;
    }
    // field_access 子节点: identifier "." identifier
    // 取最后一个 identifier 作为常量名
    let last_ident = node.child_by_field_name("field")
        .unwrap_or_else(|| {
            let mut cursor = node.walk();
            node.children(&mut cursor)
                .filter(|c| c.kind() == "identifier")
                .last()
                .unwrap_or(node)
        });
    let name = self.node_text(last_ident);
    self.string_constants.get(&name).cloned()
}
```

**注意:** tree-sitter-java 的 `field_access` 节点有 `field` 字段名指向最后一个 identifier。需验证。如果 `child_by_field_name("field")` 返回 None，则 fallback 到遍历子节点取最后一个 identifier。

**Step 2: 在 `collect_concat_parts` 中添加 `field_access` 分支**

位置：`extract.rs` 的 `collect_concat_parts` 函数，left 和 right 两处 match 各添加一个分支：

```rust
// left 侧（约 line 306 之后）
"field_access" => {
    if let Some(val) = self.resolve_field_access_constant(left) {
        parts.push((val, false));
    } else {
        parts.push((self.make_placeholder_for_node(left), false));
    }
}

// right 侧（约 line 330 之后）
"field_access" => {
    if let Some(val) = self.resolve_field_access_constant(right) {
        parts.push((val, false));
    } else {
        parts.push((self.make_placeholder_for_node(right), false));
    }
}
```

**Step 3: 在 `find_first_string_arg` 中添加 `field_access` 分支**

位置：`method_call.rs` 约 line 230

```rust
"field_access" => {
    if let Some(val) = self.resolve_field_access_constant(child) {
        return Some((val, false));
    }
}
```

**Step 4: 运行测试验证**

```bash
cargo test --features java --lib "diag_p4\|diag_p5\|diag_p6\|diag_p11" -- --nocapture
```

预期：4 个测试全部通过。

**Step 5: 全量回归测试**

```bash
cargo test --features java --lib "diag_" 2>&1 | tail -5
```

预期：86 passed; 5 failed（P7, P8, P12, m4, o10）

**Step 6: Commit**

```bash
git add src/java/extract.rs src/java/method_call.rs
git commit -m "feat(java): add field_access constant resolution for cross-file scenarios

Adds field_access node handling in collect_concat_parts and
find_first_string_arg to resolve cross-file constant references
like SqlConstants.USER_QUERY via string_constants lookup.

Fixes: P4, P5, P6, P11 diagnostic tests (82/91 → 86/91)"
```

---

## Task 2: 跨文件 PS 参数名映射 (XP1)

**目标:** 当 `UserService.save(name, email)` 调用 `JdbcHelper.bindParams(ps, a, b)` 时，PS 的占位符应使用调用者的实参名（`name`, `email`），而非被调用者的形参名（`a`, `b`）。

**原理:** 当前 `apply_pending_injections` 从 `MethodPsBehavior.setter_patterns` 中直接取 `var_name`（形参名 `a`, `b`）。跨文件场景需要做 **形参→实参 的映射替换**。

当前数据流：
```
JdbcHelper.bindParams(ps, a, b):
  record_method_behavior → key="bindParams:3", setter_patterns=[
    Literal{index:1, var_name:"a"},
    Literal{index:2, var_name:"b"}
  ]

UserService.save(name, email):
  visit_method_invocation("JdbcHelper.bindParams(ps, name, email)")
  → PendingInjection{method_name:"bindParams", arg_count:3}

apply_pending_injections:
  查找 "bindParams:3" → 得到 setter_patterns
  用 var_name "a"/"b" 生成占位符 ← 错误！应该是 "name"/"email"
```

**核心思路:** `PendingInjection` 需要记录调用时的**实参列表**。`apply_pending_injections` 时，将 `setter_patterns` 中的形参名替换为对应位置的实参名。

**Files:**
- Modify: `src/java/method_call.rs` — `PendingInjection` 结构体、`visit_method_invocation`、`apply_pending_injections`

**Step 1: 扩展 `PendingInjection` 结构体**

```rust
pub(super) struct PendingInjection {
    pub(super) ps_var: String,
    pub(super) method_name: String,
    pub(super) arg_count: usize,
    pub(super) extraction_idx: Option<usize>,
    // 新增：记录调用时传入的非 PS 参数的表达式文本
    pub(super) arg_expressions: Vec<String>,
}
```

**Step 2: 在 `visit_method_invocation` 中收集实参**

当前创建 `PendingInjection` 的代码（约 line 56-83）需要扩展，在遍历 arguments 时记录每个非标点参数的文本：

```rust
if let Some(args_node) = node.child_by_field_name("arguments") {
    if args_node.kind() == "argument_list" {
        let mut cursor = args_node.walk();
        let mut found_ps_var: Option<String> = None;
        let mut arg_count = 0usize;
        let mut arg_expressions: Vec<String> = Vec::new();
        for child in args_node.children(&mut cursor) {
            if child.kind() == "," || child.kind() == "(" || child.kind() == ")" {
                continue;
            }
            arg_count += 1;
            let arg_text = self.node_text(child);
            if child.kind() == "identifier" {
                if self.ps_var_to_extraction.contains_key(&arg_text) {
                    found_ps_var = Some(arg_text.clone());
                }
            }
            arg_expressions.push(arg_text);
        }
        if let Some(ps_var) = found_ps_var {
            let ext_idx = self.ps_var_to_extraction.get(&ps_var).copied();
            self.pending_injections.push(PendingInjection {
                ps_var,
                method_name: method_name.clone(),
                arg_count,
                extraction_idx: ext_idx,
                arg_expressions,
            });
        }
    }
}
```

**Step 3: 在 `apply_pending_injections` 中做参数名替换**

关键逻辑：查找 `MethodPsBehavior` 中 `ps_param_index`（PS 参数在第几位），从 `arg_expressions` 中取对应位置的实参名。然后构建 `形参名→实参名` 的映射表，替换 `setter_patterns` 中的 `var_name`。

```rust
// 在 apply_pending_injections 中，获取 behavior 之后：
let behavior = match behavior {
    Some(b) => b,
    None => {
        self.apply_fallback_dynamic(ext_idx);
        continue;
    }
};

// 新增：构建 形参名→实参名 映射
let param_to_arg = self.build_param_arg_mapping(&behavior, &injection.arg_expressions);

// 然后在遍历 setter_patterns 时，用映射替换 var_name
for pattern in &behavior.setter_patterns {
    match pattern {
        SetterPattern::Literal { index, java_type, var_name } => {
            let resolved_name = var_name.as_ref()
                .and_then(|name| param_to_arg.get(name).cloned())
                .or_else(|| var_name.clone());
            // ... 用 resolved_name 替代 var_name 生成占位符
        }
        // DynamicLoop 不变
    }
}
```

**需要新增辅助函数 `build_param_arg_mapping`：**

```rust
/// 根据 MethodPsBehavior 的形参信息和调用时的实参列表，
/// 构建形参名→实参名的映射。
fn build_param_arg_mapping(
    &self,
    behavior: &MethodPsBehavior,
    arg_expressions: &[String],
) -> HashMap<String, String> {
    // behavior.ps_param_index: PS 变量在参数列表中的位置
    // behavior.setter_patterns 中 Literal 的 var_name: 形参名
    // arg_expressions: 调用时传入的实参文本列表
    
    let mut mapping = HashMap::new();
    for pattern in &behavior.setter_patterns {
        if let SetterPattern::Literal { var_name: Some(param_name), .. } = pattern {
            // 形参名 param_name 对应哪个实参位置？
            // 这个信息在 MethodPsBehavior 中没有直接记录。
            // 需要扩展 MethodPsBehavior 来记录每个 setter 的形参索引。
        }
    }
    mapping
}
```

**问题：** 当前 `SetterPattern::Literal` 只存储 `var_name`（形参名），不存储形参在方法签名中的位置索引。无法做形参→实参映射。

**解决方案：** 扩展 `SetterPattern::Literal`，增加 `param_index: Option<usize>` 字段，在 `record_method_behavior` 时记录。

**Step 3a: 扩展 `SetterPattern::Literal`**

```rust
// types.rs
pub enum SetterPattern {
    Literal {
        index: usize,        // JDBC 参数索引 (ps.setInt 的第一个参数)
        java_type: String,   // Java 类型 (String, int 等)
        var_name: Option<String>,  // 形参名
        param_index: Option<usize>, // 新增：该形参在方法签名中的位置（0-based）
    },
    DynamicLoop {
        java_type: String,
    },
}
```

**Step 3b: 修改 `record_method_behavior` 记录 `param_index`**

在 `record_method_behavior`（extract.rs 约 line 634）中，遍历 `jdbc_param_map` 时，`var_name` 已经知道（就是形参名如 `a`、`b`）。需要在遍历 `formal_parameter` 时记录每个参数名到其索引的映射：

```rust
// 在 record_method_behavior 开头，遍历参数时：
let mut param_name_to_index: HashMap<String, usize> = HashMap::new();
// ... 在 for child in params_node.children 循环中 ...
if let (Some(t), Some(v)) = (&type_name, &var_name) {
    param_name_to_index.insert(v.clone(), param_idx);
    // ...
}

// 然后在构建 setter_patterns 时：
for ((var_name, _), info) in &self.jdbc_param_map {
    if var_name == &ps_var {
        let param_idx = param_name_to_index.get(&info.var_name).copied();
        setter_patterns.push(SetterPattern::Literal {
            index: info.index,
            java_type: info.java_type.clone(),
            var_name: Some(info.var_name.clone()),
            param_index: param_idx,  // 新增
        });
    }
}
```

**Step 3c: 完整的 `build_param_arg_mapping`**

```rust
fn build_param_arg_mapping(
    &self,
    behavior: &MethodPsBehavior,
    arg_expressions: &[String],
) -> HashMap<String, String> {
    let mut mapping = HashMap::new();
    for pattern in &behavior.setter_patterns {
        if let SetterPattern::Literal {
            var_name: Some(ref param_name),
            param_index: Some(idx),
            ..
        } = pattern {
            if let Some(arg_expr) = arg_expressions.get(*idx) {
                // arg_expr 可能是 "name" 或 "email" 这样的简单标识符
                // 也可能是复杂表达式如 "user.getName()"
                // 对于简单标识符，直接使用；复杂表达式取最后一个段
                let resolved = if arg_expr.contains('.') {
                    arg_expr.rsplit('.').next().unwrap_or(arg_expr)
                } else {
                    arg_expr.as_str()
                };
                mapping.insert(param_name.clone(), resolved.to_string());
            }
        }
    }
    mapping
}
```

**Step 4: 运行测试**

```bash
cargo test --features java --lib "diag_p7" -- --nocapture
```

预期：P7 通过，占位符为 `__JAVA_VAR_String_name__` 和 `__JAVA_VAR_String_email__`。

**Step 5: 全量回归**

```bash
cargo test --features java --lib "diag_" 2>&1 | tail -5
```

预期：87 passed; 4 failed（P8, P12, m4, o10）

**Step 6: Commit**

```bash
git commit -m "feat(java): cross-file PS parameter name substitution

Extends PendingInjection with arg_expressions, adds param_index
to SetterPattern::Literal, and builds param→arg mapping in
apply_pending_injections for correct cross-file placeholder names.

Fixes: P7 diagnostic test (86/91 → 87/91)"
```

---

## Task 3: 传递式方法行为记录 (XP2)

**目标:** 当 `UserService.process(ps, name, age)` 调用 `ParamBinder.bindUser(ps, name, age)` 时，自动为 `process` 方法生成 `method_behaviors` 条目，使其行为等同于 `bindUser`。

**原理:** 当前 `record_method_behavior` 只记录**直接**包含 `ps.setXxx()` 调用的方法。委托方法（只调用其他 helper 方法）不会被记录。

核心思路：在 `visit_method_invocation` 处理方法调用时，如果发现调用的目标方法已在 `method_behaviors` 中有记录，且当前方法有 PS 参数，则为当前方法创建一个"转发"的行为记录。

**Files:**
- Modify: `src/java/extract.rs` — `record_method_behavior` 或新增函数

**Step 1: 在 `visit_method_invocation` 中检测委托调用**

在 `method_call.rs` 的 `visit_method_invocation` 中，处理完 SB 操作和 PendingInjection 后，检查：

```rust
// 在 visit_method_invocation 末尾，检查是否需要记录委托方法行为
self.maybe_record_delegation(node, &method_name);
```

新增函数：

```rust
/// 检查当前方法是否在委托调用另一个有 PS 行为的方法。
/// 如果是，且当前方法也有 PS 参数，则为当前方法创建转发行为记录。
fn maybe_record_delegation(&mut self, node: Node, called_method_name: &str) {
    // 必须在方法体内才能记录
    let current_method = match &self.method_name {
        Some(m) => m.clone(),
        None => return,
    };

    // 被调用方法必须在 method_behaviors 中有记录
    let called_key = format!("{}:{}", called_method_name, self.count_args(node));
    let called_behavior = self.method_behaviors.get(&called_key).cloned()
        .or_else(|| self.method_behaviors.get(called_method_name).cloned());
    let called_behavior = match called_behavior {
        Some(b) => b,
        None => return,
    };

    // 当前方法的参数中必须有 PreparedStatement
    // 检查调用参数中是否有 PS 变量
    let args_node = match node.child_by_field_name("arguments") {
        Some(n) if n.kind() == "argument_list" => n,
        _ => return,
    };

    let mut cursor = args_node.walk();
    let mut arg_exprs: Vec<String> = Vec::new();
    let mut has_ps_arg = false;
    for child in args_node.children(&mut cursor) {
        if child.kind() == "," || child.kind() == "(" || child.kind() == ")" {
            continue;
        }
        let text = self.node_text(child);
        if child.kind() == "identifier" && self.ps_var_to_extraction.contains_key(&text) {
            has_ps_arg = true;
        }
        arg_exprs.push(text);
    }

    if !has_ps_arg {
        return;
    }

    // 为当前方法创建转发行为
    // 当前方法签名中 PS 参数的位置和名字需要在当前方法上下文中推断
    // 简化方案：直接克隆被调用方法的行为，但修改 ps_param_name 为当前方法中的 PS 变量名
    let ps_var_in_call = arg_exprs.get(called_behavior.ps_param_index)
        .cloned()
        .unwrap_or_default();

    // 构建参数映射：被调用方法形参 → 当前调用实参
    let mut forwarded_patterns = called_behavior.setter_patterns.clone();
    for pattern in &mut forwarded_patterns {
        if let SetterPattern::Literal { var_name: Some(ref mut name), param_index, .. } = pattern {
            if let Some(idx) = *param_index {
                if let Some(arg) = arg_exprs.get(idx) {
                    // 替换形参名为实参名
                    let resolved = if arg.contains('.') {
                        arg.rsplit('.').next().unwrap_or(arg)
                    } else {
                        arg.as_str()
                    };
                    *name = resolved.to_string();
                }
            }
        }
    }

    // 计算当前方法的参数总数（需要从方法声明中获取，简化处理）
    // 使用当前已知的 arg_count 作为 key
    let total_args = arg_exprs.len();

    // 记录为当前方法的行为
    let key = format!("{}:{}", current_method, total_args);
    self.method_behaviors.insert(key, MethodPsBehavior {
        ps_param_index: called_behavior.ps_param_index, // 可能不准确，但已足够
        ps_param_name: ps_var_in_call.clone(),
        setter_patterns: forwarded_patterns,
    });
}

fn count_args(&self, node: Node) -> usize {
    let args_node = match node.child_by_field_name("arguments") {
        Some(n) if n.kind() == "argument_list" => n,
        _ => return 0,
    };
    let mut cursor = args_node.walk();
    args_node.children(&mut cursor)
        .filter(|c| c.kind() != "," && c.kind() != "(" && c.kind() != ")")
        .count()
}
```

**注意:** 这里的关键简化是：委托方法的行为直接从被委托方法克隆，形参名替换为实参名。这不需要解析当前方法的完整签名——因为我们已知调用时传入了什么参数。

**Step 2: 运行测试**

```bash
cargo test --features java --lib "diag_p8" -- --nocapture
```

预期：P8 通过。

**Step 3: 全量回归**

```bash
cargo test --features java --lib "diag_" 2>&1 | tail -5
```

预期：88 passed; 3 failed（P12, m4, o10）

**Step 4: Commit**

```bash
git commit -m "feat(java): transitive method behavior recording for delegation chains

When a method delegates to another method with known PS behavior,
automatically records a forwarding behavior for the delegating method.
Enables 3+ layer cross-file parameter resolution.

Fixes: P8 diagnostic test (87/91 → 88/91)"
```

---

## Task 4: 继承方法导入 (XP3)

**目标:** 当 `UserDao extends BaseDao` 时，`UserDao` 能使用 `BaseDao` 中定义的方法行为。

**原理:** tree-sitter-java 的 `class_declaration` 节点有 `superclass` 字段（类型为 `superclass`，内容如 `extends BaseDao`）。需要：

1. 在 `CrossFileState` 中新增 `class_methods` 映射：`class_name → Vec<(method_name, param_count)>`
2. 处理文件时，检测 `extends`，将父类的 `method_behaviors` 导入当前上下文
3. 需要按类名限定 method_behaviors 的 key（如 `"BaseDao.buildSelect:0"`）

**核心难点:** 当前 `method_behaviors` 的 key 是 `"methodName:paramCount"`，没有类名限定。如果多个类有同名方法会产生冲突。需要扩展 key 格式。

**Files:**
- Modify: `src/java/types.rs` — `CrossFileState` 新增字段
- Modify: `src/java/mod.rs` — 文件处理后收集类→方法映射
- Modify: `src/java/extract.rs` — `visit_type_declaration` 捕获 extends
- Modify: `src/java/method_call.rs` — `apply_pending_injections` 查找父类方法

**Step 1: 扩展 `CrossFileState`**

```rust
// mod.rs
pub struct CrossFileState {
    pub method_behaviors: HashMap<String, MethodPsBehavior>,
    pub string_constants: HashMap<String, String>,
    /// 类名 → 该类的父类名
    pub class_parents: HashMap<String, String>,
    /// 类名 → 该类定义的方法行为 key 列表
    pub class_method_keys: HashMap<String, Vec<String>>,
}
```

**Step 2: 在 `visit_type_declaration` 中捕获 extends**

```rust
// extract.rs — visit_type_declaration
pub(super) fn visit_type_declaration(&mut self, node: Node) {
    let old_class = self.class_name.clone();
    let old_parent = self.current_parent_class.clone();

    if let Some(name_node) = node.child_by_field_name("name") {
        self.class_name = Some(self.node_text(name_node));

        // 新增：捕获 extends
        if let Some(sc_node) = node.child_by_field_name("superclass") {
            // sc_node 是 "superclass" 类型，子节点: "extends" keyword + type_identifier
            let mut cursor = sc_node.walk();
            for child in sc_node.children(&mut cursor) {
                if child.kind() == "type_identifier" {
                    self.current_parent_class = Some(self.node_text(child));
                    break;
                }
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        self.visit(child);
    }

    self.class_name = old_class;
    self.current_parent_class = old_parent;
}
```

**Step 3: 在 `ExtractContext` 中新增字段**

```rust
// extract.rs
pub(super) struct ExtractContext<'a> {
    // ... 现有字段 ...
    /// 当前类的父类名
    pub(super) current_parent_class: Option<String>,
    /// 当前类定义的方法行为 key（在 record_method_behavior 中收集）
    pub(super) recorded_method_keys: Vec<(String, String)>, // (class_name, method_key)
}
```

**Step 4: 修改 `record_method_behavior` 记录类名**

```rust
// 在 record_method_behavior 末尾，insert 之后：
if let Some(class_name) = &self.class_name {
    self.recorded_method_keys.push((class_name.clone(), key.clone()));
}
```

**Step 5: 文件处理后更新 `CrossFileState`**

在 `extract_sql_from_java_files_with_state` 中，文件处理完成后：

```rust
// 更新 class_parents
if let Some(class_name) = &ctx.class_name {
    if let Some(parent) = &ctx.current_parent_class {
        state.class_parents.insert(class_name.clone(), parent.clone());
    }
}

// 更新 class_method_keys
for (class_name, method_key) in &ctx.recorded_method_keys {
    state.class_method_keys
        .entry(class_name.clone())
        .or_default()
        .push(method_key.clone());
}
```

**Step 6: 在 `apply_pending_injections` 中查找父类方法**

当在当前类的 `method_behaviors` 中找不到时，尝试查找父类的：

```rust
// 在 apply_pending_injections 中，behavior lookup 失败后：
let behavior = self.method_behaviors.get(&behavior_key).cloned()
    .or_else(|| self.method_behaviors.get(&injection.method_name).cloned())
    .or_else(|| {
        // 尝试从父类方法中查找
        self.lookup_inherited_behavior(&injection.method_name, injection.arg_count)
    });
```

新增 `lookup_inherited_behavior`：

```rust
fn lookup_inherited_behavior(&self, method_name: &str, arg_count: usize) -> Option<MethodPsBehavior> {
    // 需要访问 class_parents 和 class_method_keys
    // 但 ExtractContext 没有直接持有这些跨文件信息
    // 解决方案：在文件处理开始时，将父类的 method_behaviors 合并进来
    // 这需要在 extract_sql_from_java_files_with_state 中做预处理
    unimplemented!()
}
```

**更好的方案：预处理导入**

在 `extract_sql_from_java_files_with_state` 中，创建 `ExtractContext` 之前，检查当前文件是否 extends 了某个已知类，如果是，将该类的 `method_behaviors` 复制一份（key 改为不带类名前缀）合并到当前 context：

```rust
// 在 for (file_path, source) in files 循环中，创建 ctx 之前：
let mut extra_behaviors = HashMap::new();
// 预解析文件获取 class 和 extends 信息
if let Some(pre_tree) = parser.parse(source, None) {
    if let Some(class_decl) = find_first_class_declaration(pre_tree.root_node()) {
        if let Some(parent_name) = extract_superclass_name(class_decl, source) {
            // 查找父类的方法
            if let Some(parent_methods) = state.class_method_keys.get(&parent_name) {
                for method_key in parent_methods {
                    if let Some(behavior) = state.method_behaviors.get(method_key) {
                        // 用去掉类名前缀的 key 存储
                        let short_key = method_key.split(':').last()
                            .unwrap_or(method_key)
                            .to_string();
                        extra_behaviors.insert(short_key, behavior.clone());
                    }
                }
            }
        }
    }
}
```

但这需要额外一次 parse（预解析），成本较高。

**更优方案：在 CrossFileState 中用限定 key 存储，查找时 fallback**

1. `record_method_behavior` 用限定 key `"ClassName.methodName:paramCount"` 存储
2. 同时保留不带类名的 key 作为 fallback
3. 查找时先查 `"ClassName.methodName:paramCount"`，再查 `"methodName:paramCount"`，再查父类的

这样只需修改存储格式，不需要预解析。

**Step 7: 运行测试**

```bash
cargo test --features java --lib "diag_p12" -- --nocapture
```

预期：P12 通过。

**Step 8: 全量回归**

```bash
cargo test --features java --lib "diag_" 2>&1 | tail -5
```

预期：89 passed; 2 failed（m4, o10）

**Step 9: Commit**

```bash
git commit -m "feat(java): inheritance method behavior import

Tracks extends relationships in CrossFileState, qualifies
method_behavior keys with class name, and falls back to parent
class behaviors when resolving method calls in child classes.

Fixes: P12 diagnostic test (88/91 → 89/91)"
```

---

## Task 5: 全量验证与收尾

**Step 1: 全量测试**

```bash
cargo test --features java --lib "diag_" 2>&1 | tail -5
```

预期：89 passed; 2 failed (m4, o10)

**Step 2: 原始 1313 测试回归**

```bash
cargo test --features java --lib 2>&1 | grep "^test result" | head -1
```

预期：全部通过，无回归。

**Step 3: 验证已知限制仍然正确记录**

```bash
cargo test --features java --lib "diag_m4\|diag_o10" -- --nocapture
```

确认 m4 和 o10 仍然失败，且失败消息准确描述了限制。

**Step 4: 最终 Commit**

```bash
git commit -m "test(java): verify cross-file improvements — 89/91 pass

89 pass / 2 known limitations (Java 14 switch expression, return SQL).
No regressions in 1313 original tests."
```

---

## 关键设计决策

1. **`resolve_field_access_constant` 只取最后一段 identifier** — 不做类名匹配，因为同一文件内常量名通常唯一。如果未来有同名常量冲突，可扩展为全限定名匹配。

2. **`SetterPattern::Literal` 新增 `param_index`** — 这是 Task 2 的基础，也是 Task 3 委托记录的基础。一次性添加，后续任务直接使用。

3. **委托方法行为记录采用"克隆+替换"策略** — 不需要解析当前方法的完整签名，只利用调用时的实参信息。简化实现但可能遗漏边缘情况（如当前方法有额外的 PS 操作）。

4. **继承采用"限定 key + fallback"策略** — 避免预解析文件的额外成本。限定 key 格式为 `"ClassName.methodName:paramCount"`，查找时逐级 fallback。

5. **m4 和 o10 暂不修复** — Java 14 switch expression 和 return-SQL 属于边缘场景，ROI 不高，记录为已知限制。

## 风险与缓解

| 风险 | 概率 | 缓解 |
|------|------|------|
| `param_index` 添加导致现有代码编译错误 | 低 | 所有构造 `SetterPattern::Literal` 的地方都需要更新 |
| 委托记录与直接 PS 操作冲突 | 中 | 如果方法既有直接 setXxx 又有委托调用，以直接操作为准 |
| 继承 fallback 找到错误父类方法 | 低 | 限定 key 包含类名，fallback 只在非限定查找失败时触发 |
| field_access 的 `field` 字段名在不同 tree-sitter-java 版本不同 | 低 | 添加 fallback 逻辑（遍历子节点取最后一个 identifier） |
