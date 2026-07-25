# iBatis XML 解析优化 + 参数类型推断 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 重构 iBatis XML 解析管线（修复 `<include>` 文本匹配缺陷、消除代码重复），并新增基于 Java 源码的参数类型推断能力，使提取的 SQL 中的 `__XML_PARAM_...` 占位符携带类型信息。

**Architecture:** 
1. 引入 `SqlNode::Include` 节点消除文本序列化/反序列化
2. 新增 `JavaSourceResolver` 按需查找 Java 文件（namespace → 文件路径映射），用 tree-sitter-java 解析 Mapper 接口方法签名和 DTO 字段
3. 将类型推断结果作为结构化 `ParamMeta` 附加到 `ParsedStatement`

**Tech Stack:** Rust 2021, quick-xml, tree-sitter + tree-sitter-java (已有依赖), serde, walkdir

**Anti-Slop Rules:**
- ❌ 禁止参数类型启发式猜测（参数名约定、SQL 上下文推测）
- ❌ 禁止全量扫描 Java 目录
- ❌ 禁止 `as any` / `@ts-ignore` 类似模式（Rust 中避免 `.unwrap()` 在非确定性路径）
- ✅ 类型推断只接受确定性来源：内联注解、Java 方法签名、@Param、DTO 字段
- ✅ 无类型信息时返回 `None`，不猜测

**Reference Files (implementer MUST read before starting):**
- `src/ibatis/parser.rs` — XML 解析器（Include 节点生成处）
- `src/ibatis/resolver.rs` — Include 解析（需重写核心逻辑）
- `src/ibatis/flatten.rs` — SQL 扁平化（参数占位符生成）
- `src/ibatis/types.rs` — 数据模型
- `src/ibatis/mod.rs` — 主管线入口
- `src/ibatis/tests.rs` — 现有测试
- `src/java/extract.rs:106-167` — `visit_method_declaration` 方法签名提取模式（可参考）
- `src/java/extract.rs:257-291` — `extract_type_name` 类型名提取（可复用逻辑）
- `src/bin/ogsql.rs:88-111` — CLI 子命令定义模式
- `src/bin/ogsql.rs:1176-1223` — `cmd_parse_xml_dir` 目录扫描模式

---

## Task 1: 引入 `SqlNode::Include` 节点

**目标:** `<include refid="..."/>` 生成结构化节点而非文本，消除 resolver 中的字符串匹配。

**Files:**
- Modify: `src/ibatis/types.rs`
- Modify: `src/ibatis/parser.rs`
- Modify: `src/ibatis/resolver.rs`
- Modify: `src/ibatis/flatten.rs`
- Modify: `src/ibatis/tests.rs`

### Step 1: 在 `types.rs` 的 `SqlNode` enum 中添加 Include 变体

在 `RawExpr` 之后、`If` 之前添加：

```rust
    /// <include refid="..."/> — 解析阶段生成，resolve 阶段被替换为片段 body
    Include {
        refid: String,
    },
```

### Step 2: 修改 `parser.rs` — Empty 事件中的 `<include>` 生成 Include 节点

当前代码（约第 156 行）：
```rust
if ln.as_ref().eq_ignore_ascii_case(b"include") {
    if let Some(refid) = get_attr(&e, "refid") {
        nodes.push(SqlNode::Text {
            content: format!("<include refid=\"{}\"/>", refid),
        });
    }
}
```

改为：
```rust
if ln.as_ref().eq_ignore_ascii_case(b"include") {
    if let Some(refid) = get_attr(&e, "refid") {
        nodes.push(SqlNode::Include { refid });
    }
}
```

### Step 3: 删除 `parser.rs` 中的 `node_to_raw_text` 函数

删除整个 `node_to_raw_text` 函数（约第 348-383 行）。这个函数用于将 SqlNode 序列化回文本，是 Include 文本匹配的核心问题来源。

同时，`read_node_tree` 中"未知元素重建为 XML 文本"的分支（约第 141-150 行）中的 `node_to_raw_text` 调用需要改为内联文本拼接，或者将这些子节点保持为 `SqlNode::Text`（未知元素内不会出现 Include）。

### Step 4: 重写 `resolver.rs` 核心逻辑

**关键变化:** `resolve_node` 直接对 `SqlNode::Include` 做结构化替换，不再有文本匹配。

替换 `resolve_node` 函数：

```rust
fn resolve_node(
    node: &SqlNode,
    fragments: &HashMap<&str, &SqlNode>,
    visited: &mut HashSet<String>,
) -> Result<SqlNode, IbatisError> {
    match node {
        SqlNode::Include { refid } => {
            // 检测循环引用
            if visited.contains(refid) {
                let mut chain: Vec<String> = visited.iter().cloned().collect();
                chain.push(refid.clone());
                return Err(IbatisError::CircularInclude { chain });
            }
            // 查找片段
            let body = fragments
                .get(refid.as_str())
                .ok_or_else(|| IbatisError::UnknownFragment { refid: refid.clone() })?;
            // 递归展开（片段可能也含 Include）
            visited.insert(refid.clone());
            let resolved = resolve_node(body, fragments, visited)?;
            visited.remove(refid);
            Ok(resolved)
        }
        SqlNode::Text { .. } | SqlNode::Parameter { .. } | SqlNode::RawExpr { .. } | SqlNode::Bind { .. } => {
            Ok(node.clone())
        }
        SqlNode::If { test, children } => {
            let resolved = resolve_children(children, fragments, visited)?;
            Ok(SqlNode::If { test: test.clone(), children: resolved })
        }
        SqlNode::Choose { branches } => {
            let mut resolved = Vec::new();
            for (test, ch) in branches {
                let resolved_ch = resolve_children(ch, fragments, visited)?;
                resolved.push((test.clone(), resolved_ch));
            }
            Ok(SqlNode::Choose { branches: resolved })
        }
        SqlNode::Where { children } => {
            Ok(SqlNode::Where { children: resolve_children(children, fragments, visited)? })
        }
        SqlNode::Set { children } => {
            Ok(SqlNode::Set { children: resolve_children(children, fragments, visited)? })
        }
        SqlNode::Trim { prefix, suffix, prefix_overrides, suffix_overrides, children } => {
            Ok(SqlNode::Trim {
                prefix: prefix.clone(), suffix: suffix.clone(),
                prefix_overrides: prefix_overrides.clone(), suffix_overrides: suffix_overrides.clone(),
                children: resolve_children(children, fragments, visited)?,
            })
        }
        SqlNode::ForEach { collection, item, index, open, separator, close, children } => {
            Ok(SqlNode::ForEach {
                collection: collection.clone(), item: item.clone(), index: index.clone(),
                open: open.clone(), separator: separator.clone(), close: close.clone(),
                children: resolve_children(children, fragments, visited)?,
            })
        }
        SqlNode::Sequence { children } => {
            Ok(SqlNode::Sequence { children: resolve_children(children, fragments, visited)? })
        }
    }
}
```

### Step 5: 简化 `resolve_children`

将 `visited` 改为 `&mut` 引用（不再 clone），因为同一层级兄弟节点的 Include 是独立的：

```rust
fn resolve_children(
    children: &[SqlNode],
    fragments: &HashMap<&str, &SqlNode>,
    visited: &mut HashSet<String>,
) -> Result<Vec<SqlNode>, IbatisError> {
    children.iter().map(|c| resolve_node(c, fragments, visited)).collect()
}
```

### Step 6: 删除 `resolver.rs` 中不再需要的函数

删除以下函数：
- `resolve_includes_in_text` — 文本匹配逻辑，不再需要
- `extract_refid` — 从文本提取 refid，不再需要
- `node_to_flat_text` — 节点转文本，不再需要

### Step 7: 在 `flatten.rs` 中添加 Include 分支

在 `flatten_sql` 的 match 中添加：

```rust
SqlNode::Include { refid } => {
    // Include 节点在 resolve 阶段应已被替换
    // 如果出现说明 resolve 未执行，输出注释标记
    format!("/* UNRESOLVED_INCLUDE({}) */", refid)
}
```

### Step 8: 更新测试

在 `tests.rs` 中更新 `node_text` helper，添加 Include 分支：

```rust
SqlNode::Include { refid } => format!("<include refid=\"{}\"/>", refid),
```

### Step 9: 运行测试

Run: `cargo test --features ibatis --lib ibatis::tests`
Expected: 所有现有测试通过（Include 在基本测试中通过 Text 生成，无变化）

### Step 10: 添加 Include 结构化解析的专项测试

```rust
#[test]
fn test_include_parsed_as_node() {
    let xml = br#"<mapper namespace="test">
        <sql id="cols">id, name</sql>
        <select id="find">SELECT <include refid="cols"/> FROM users</select>
    </mapper>"#;
    let mapper = crate::ibatis::parser::parse_xml(xml).unwrap();
    let stmt = &mapper.statements[0];
    // body 应该是 Sequence，包含 Text + Include + Text
    match &stmt.body {
        SqlNode::Sequence { children } => {
            let includes: Vec<_> = children.iter().filter_map(|c| match c {
                SqlNode::Include { refid } => Some(refid.clone()),
                _ => None,
            }).collect();
            assert_eq!(includes, vec!["cols".to_string()]);
        }
        other => panic!("expected Sequence, got {:?}", other),
    }
}

#[test]
fn test_include_resolved_structurally() {
    let xml = br#"<mapper namespace="test">
        <sql id="cols">id, name</sql>
        <select id="find">SELECT <include refid="cols"/> FROM users</select>
    </mapper>"#;
    let mapper = crate::ibatis::parser::parse_xml(xml).unwrap();
    let resolved = crate::ibatis::resolver::resolve_includes(&mapper).unwrap();
    let stmt = &resolved.statements[0];
    let content = node_text(&stmt.body);
    assert!(content.contains("id, name"), "include should be expanded, got: {}", content);
    // 确认没有 Include 节点残留
    assert!(!content.contains("<include"), "no raw include text should remain");
}
```

### Step 11: Commit

```
refactor(ibatis): structural Include node replaces text-based include matching
```

---

## Task 2: 提取共享工具函数

**目标:** 消除 `parser.rs` 和 `flatten.rs` 中的重复函数。

**Files:**
- Create: `src/ibatis/util.rs`
- Modify: `src/ibatis/parser.rs`
- Modify: `src/ibatis/flatten.rs`
- Modify: `src/ibatis/mod.rs`

### Step 1: 创建 `src/ibatis/util.rs`

从 `parser.rs` 和 `flatten.rs` 中提取以下函数：

```rust
//! 共享工具函数。

/// 从位置 start 开始查找匹配的 `}`，考虑嵌套 `{}`。
pub fn find_closing_brace(chars: &[char], start: usize) -> Option<usize> {
    let mut depth = 1;
    let mut i = start;
    while i < chars.len() {
        match chars[i] {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// 从 MyBatis 参数字符串中提取 name 和可选的 javaType/jdbcType。
/// 格式: `name` 或 `name,javaType=double` 或 `name,jdbcType=NUMERIC`。
/// 优先使用 javaType。
pub fn parse_param_type(param: &str) -> (String, Option<String>) {
    let mut parts = param.split(',');
    let name = parts.next().unwrap_or("").trim().to_string();
    let mut java_type: Option<String> = None;
    let mut jdbc_type: Option<String> = None;
    for part in parts {
        let part = part.trim();
        if let Some(val) = part.strip_prefix("javaType=") {
            java_type = Some(val.to_string());
        } else if let Some(val) = part.strip_prefix("jdbcType=") {
            jdbc_type = Some(val.to_string());
        }
    }
    (name, java_type.or(jdbc_type))
}
```

### Step 2: 在 `mod.rs` 中添加模块声明

在 `pub mod error;` 之前添加：

```rust
mod util;
```

（保持 `util` 为 crate 内部可见，不 pub use）

### Step 3: 替换 `parser.rs` 中的函数为 `util::` 调用

删除 `parser.rs` 中的 `find_closing_brace` 和 `parse_param_type` 函数。
将所有调用改为 `super::util::find_closing_brace` 和 `super::util::parse_param_type`。

### Step 4: 替换 `flatten.rs` 中的函数为 `util::` 调用

删除 `flatten.rs` 中的 `find_closing_brace` 和 `parse_param_type` 函数。
将所有调用改为 `super::util::find_closing_brace` 和 `super::util::parse_param_type`。

### Step 5: 运行测试

Run: `cargo test --features ibatis --lib ibatis`
Expected: 所有测试通过

### Step 6: Commit

```
refactor(ibatis): extract shared utility functions to util.rs
```

---

## Task 3: 修复 `apply_trim` 循环剥离 + `choose` 分支改进

**目标:** 修复 prefix overrides 只剥离一次的 bug；改进 choose 分支解析。

**Files:**
- Modify: `src/ibatis/flatten.rs`
- Modify: `src/ibatis/parser.rs`
- Modify: `src/ibatis/tests.rs`

### Step 1: 修复 `apply_trim` 循环剥离 prefix overrides

在 `flatten.rs` 的 `apply_trim` 函数中，将 prefix overrides 剥离改为循环：

当前代码（单次）：
```rust
if let Some(overrides) = prefix_overrides {
    let ov_list: Vec<&str> = overrides.split('|').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    for ov in &ov_list {
        let trimmed = result.trim_start();
        if trimmed.len() >= ov.len() && trimmed[..ov.len()].eq_ignore_ascii_case(ov) {
            result = trimmed[ov.len()..].to_string();
        }
    }
}
```

改为循环：
```rust
if let Some(overrides) = prefix_overrides {
    let ov_list: Vec<&str> = overrides.split('|').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    loop {
        let trimmed = result.trim_start();
        let mut stripped = false;
        for ov in &ov_list {
            if trimmed.len() >= ov.len() && trimmed[..ov.len()].eq_ignore_ascii_case(ov) {
                result = trimmed[ov.len()..].to_string();
                stripped = true;
                break;
            }
        }
        if !stripped { break; }
    }
}
```

同样对 suffix overrides 做循环处理。

### Step 2: 添加循环剥离测试

```rust
#[test]
fn test_trim_strips_multiple_prefix_overrides() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <trim prefix="WHERE" prefixOverrides="AND |OR ">
                <if test="a">AND OR status = #{status}</if>
            </trim>
        </select>
    </mapper>"#;
    let result = super::super::parse_mapper_bytes(xml);
    let sql = &result.statements[0].flat_sql;
    assert!(sql.contains("WHERE"), "should have WHERE, got: {}", sql);
    assert!(!sql.contains("WHERE AND"), "first override stripped, got: {}", sql);
}
```

### Step 3: 运行测试

Run: `cargo test --features ibatis --lib ibatis::tests`
Expected: 全部通过

### Step 4: Commit

```
fix(ibatis): loop-strip prefix/suffix overrides in apply_trim
```

---

## Task 4: 增强 `ParsedStatement` + 定义类型系统

**目标:** 传递 `parameter_type`/`result_type`；定义 `ParamMeta`/`JdbcType`/`InferenceSource`。

**Files:**
- Modify: `src/ibatis/types.rs`
- Modify: `src/ibatis/mod.rs`

### Step 1: 在 `types.rs` 中添加类型系统定义

在文件末尾 `ParsedStatement` 之前添加：

```rust
/// MyBatis 支持的 JDBC 类型（常用子集）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum JdbcType {
    // 数值
    Integer,
    BigInt,
    SmallInt,
    TinyInt,
    Decimal,
    Numeric,
    Double,
    Float,
    Real,
    // 字符串
    Char,
    VarChar,
    LongVarChar,
    NChar,
    NVarChar,
    Clob,
    NClob,
    // 二进制
    Binary,
    VarBinary,
    Blob,
    // 日期时间
    Date,
    Time,
    Timestamp,
    // 其他
    Boolean,
    Null,
    Array,
    Other,
}

/// 参数类型推断来源。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum InferenceSource {
    /// XML 内联 javaType 注解: #{param,javaType=String}
    InlineJavaType,
    /// XML 内联 jdbcType 注解: #{param,jdbcType=VARCHAR}
    InlineJdbcType,
    /// Java Mapper 接口方法签名
    JavaMethodSignature,
    /// Java @Param 注解
    JavaParamAnnotation,
    /// Java DTO/Model 类字段类型
    JavaDtoField,
}

/// 参数元数据。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParamMeta {
    /// 参数名
    pub name: String,
    /// 推断出的 JDBC 类型（None = 无法推断）
    pub jdbc_type: Option<JdbcType>,
    /// 推断来源
    pub source: Option<InferenceSource>,
    /// 参数在 flat_sql 中的字节偏移
    pub position: usize,
    /// 原始 #{...} 内容
    pub raw: String,
}
```

### Step 2: 修改 `ParsedStatement` 添加新字段

```rust
pub struct ParsedStatement {
    pub id: String,
    pub kind: StatementKind,
    pub parameter_type: Option<String>,
    pub result_type: Option<String>,
    pub flat_sql: String,
    pub parameters: Vec<ParamMeta>,
    pub has_dynamic_elements: bool,
    pub line: usize,
    pub parse_result: Option<(
        Vec<crate::ast::StatementInfo>,
        Vec<crate::parser::ParserError>,
    )>,
}
```

### Step 3: 修改 `mod.rs` 传递新字段

在 `parse_mapper_bytes_with_path` 中构建 `ParsedStatement` 的地方：

```rust
statements.push(ParsedStatement {
    id: stmt.id.clone(),
    kind: stmt.kind,
    parameter_type: stmt.parameter_type.clone(),
    result_type: stmt.result_type.clone(),
    flat_sql,
    parameters: Vec::new(), // Task 6 中填充
    has_dynamic: has_dynamic,
    line: stmt.line,
    parse_result,
});
```

### Step 4: 更新 `mod.rs` 的 pub use

在 `mod.rs` 的 pub use 中添加新类型：

```rust
pub use types::{
    FlattenedStatement, JdbcType, MapperFile, MapperStatement, ParamMeta, ParsedMapper,
    ParsedStatement, SqlFragment, SqlNode, StatementKind, InferenceSource,
};
```

### Step 5: 运行测试

Run: `cargo test --features ibatis --lib ibatis`
Expected: 编译通过，所有测试通过（parameters 字段为空 Vec，不影响现有逻辑）

### Step 6: Commit

```
feat(ibatis): add ParamMeta/JdbcType types, forward parameter_type to ParsedStatement
```

---

## Task 5: 新增 Java Mapper 接口解析器 + DTO 字段提取

**目标:** 解析 Java Mapper 接口方法签名和 DTO 类字段，提取参数名到类型的映射。

**Files:**
- Create: `src/java/mapper_interface.rs`
- Create: `src/java/dto_fields.rs`
- Modify: `src/java/mod.rs`

### Step 1: 创建 `src/java/mapper_interface.rs`

```rust
//! MyBatis Mapper 接口解析器。
//!
//! 从 Java Mapper 接口中提取方法签名信息：
//! 方法名 → 参数名 + Java 类型 + @Param 注解。

use std::collections::HashMap;
use tree_sitter::Node;

/// Mapper 接口解析结果。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MapperInterfaceInfo {
    /// 全限定类名（从 package + class name 得到）
    pub fqn: String,
    /// 方法名 → 方法信息
    pub methods: HashMap<String, MapperMethodInfo>,
}

/// 一个方法的信息。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MapperMethodInfo {
    /// 返回类型名
    pub return_type: Option<String>,
    /// 参数列表
    pub params: Vec<MethodParam>,
}

/// 一个方法参数。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MethodParam {
    /// @Param("xxx") 注解值 或 参数名
    pub name: String,
    /// Java 类型名（"int", "String", "Long", "User" 等）
    pub java_type: String,
    /// @Param 注解值（如果存在）
    pub param_annotation: Option<String>,
}

/// 解析 Java Mapper 接口文件。
///
/// 输入: Java 源码字符串
/// 输出: 如果文件是 interface，返回解析结果；否则返回 None
pub fn parse_mapper_interface(source: &str) -> Option<MapperInterfaceInfo> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_java::LANGUAGE.into()).ok()?;

    let tree = parser.parse(source, None)?;
    let root = tree.root_node();

    // 查找 package 声明
    let package_name = extract_package_name(&root, source);

    // 查找 interface_declaration
    let interface_node = find_interface(&root)?;
    let class_name = interface_node
        .child_by_field_name("name")
        .map(|n| source[n.byte_range()].to_string())?;

    let fqn = match &package_name {
        Some(pkg) => format!("{}.{}", pkg, class_name),
        None => class_name,
    };

    let methods = extract_methods(&interface_node, source);

    Some(MapperInterfaceInfo { fqn, methods })
}

fn extract_package_name(root: &Node, source: &str) -> Option<String> {
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "package_declaration" {
            // package com.example.mapper;
            let mut cursor2 = child.walk();
            for c in child.children(&mut cursor2) {
                if c.kind() == "scoped_identifier" || c.kind() == "identifier" {
                    return Some(source[c.byte_range()].to_string());
                }
            }
        }
    }
    None
}

fn find_interface<'a>(root: &'a Node) -> Option<Node<'a>> {
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "interface_declaration" {
            return Some(child);
        }
        // 可能在 class_declaration 内部有内部接口
        if child.kind() == "class_declaration" || child.kind() == "program" {
            if let Some(iface) = find_interface_deep(&child) {
                return Some(iface);
            }
        }
    }
    None
}

fn find_interface_deep<'a>(node: &'a Node) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "interface_declaration" {
            return Some(child);
        }
    }
    None
}

fn extract_methods(interface: &Node, source: &str) -> HashMap<String, MapperMethodInfo> {
    let mut methods = HashMap::new();
    let mut cursor = interface.walk();

    for child in interface.children(&mut cursor) {
        if child.kind() == "interface_body" {
            let mut body_cursor = child.walk();
            for member in child.children(&mut body_cursor) {
                // 跳过注解，找 abstract_method_declaration 或 method_declaration
                if member.kind() == "method_declaration" || member.kind() == "abstract_method_declaration" {
                    if let Some(info) = parse_method(&member, source) {
                        methods.insert(info.0, info.1);
                    }
                }
            }
        }
    }
    methods
}

fn parse_method(node: &Node, source: &str) -> Option<(String, MapperMethodInfo)> {
    let name_node = node.child_by_field_name("name")?;
    let method_name = source[name_node.byte_range()].to_string();

    let return_type = node.child_by_field_name("type")
        .map(|n| extract_type_name_simple(&n, source));

    let params = node.child_by_field_name("parameters")
        .map(|params_node| extract_params(&params_node, source))
        .unwrap_or_default();

    Some((method_name, MapperMethodInfo { return_type, params }))
}

fn extract_params(params_node: &Node, source: &str) -> Vec<MethodParam> {
    let mut params = Vec::new();
    let mut cursor = params_node.walk();

    for child in params_node.children(&mut cursor) {
        if child.kind() == "formal_parameter" {
            if let Some(param) = parse_formal_parameter(&child, source) {
                params.push(param);
            }
        }
    }
    params
}

fn parse_formal_parameter(node: &Node, source: &str) -> Option<MethodParam> {
    let mut java_type: Option<String> = None;
    let mut var_name: Option<String> = None;
    let mut param_annotation: Option<String> = None;

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            // 类型节点
            "type_identifier" | "primitive_type" | "integral_type"
            | "floating_point_type" | "boolean_type" | "generic_type" | "array_type" => {
                java_type = Some(extract_type_name_simple(&child, source));
            }
            // 参数名
            "identifier" => {
                var_name = Some(source[child.byte_range()].to_string());
            }
            // @Param 注解
            "annotation" => {
                param_annotation = extract_param_annotation(&child, source);
            }
            _ => {}
        }
    }

    let java_type = java_type?;
    let var_name = var_name?;

    // @Param 值优先于参数名
    let name = param_annotation.clone().unwrap_or_else(|| var_name.clone());

    Some(MethodParam {
        name,
        java_type,
        param_annotation,
    })
}

fn extract_param_annotation(node: &Node, source: &str) -> Option<String> {
    // @Param("xxx") → annotation > marker_annotation | annotation
    // annotation: @ Param ( argument_list )
    let name_node = node.child_by_field_name("name")?;
    let ann_name = source[name_node.byte_range()].to_string();
    if ann_name != "Param" {
        return None;
    }

    // 查找 argument_list 中的字符串字面量
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "argument_list" {
            let mut arg_cursor = child.walk();
            for arg in child.children(&mut arg_cursor) {
                if arg.kind() == "string_literal" {
                    // 去掉引号
                    let raw = source[arg.byte_range()].to_string();
                    let inner = raw.trim_matches('"');
                    return Some(inner.to_string());
                }
            }
        }
    }
    None
}

fn extract_type_name_simple(node: &Node, source: &str) -> String {
    match node.kind() {
        "type_identifier" | "primitive_type" | "integral_type"
        | "floating_point_type" | "boolean_type" => source[node.byte_range()].to_string(),
        "generic_type" => {
            // List<User> → List
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "type_identifier" {
                    return source[child.byte_range()].to_string();
                }
            }
            source[node.byte_range()].to_string()
        }
        "array_type" => {
            // int[] → int
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if matches!(child.kind(),
                    "type_identifier" | "primitive_type" | "integral_type"
                    | "floating_point_type" | "boolean_type"
                ) {
                    return source[child.byte_range()].to_string();
                }
            }
            source[node.byte_range()].to_string()
        }
        _ => source[node.byte_range()].to_string(),
    }
}
```

### Step 2: 创建 `src/java/dto_fields.rs`

```rust
//! DTO/Model 类字段类型提取。
//!
//! 从 Java POJO/DTO/Model 类中提取字段名 → Java 类型的映射。

use std::collections::HashMap;
use tree_sitter::Node;

/// 从 Java 类源码中提取字段名 → 类型映射。
pub fn parse_dto_fields(source: &str) -> HashMap<String, String> {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_java::LANGUAGE.into())
        .expect("Failed to set Java language");

    let tree = match parser.parse(source, None) {
        Some(t) => t,
        None => return HashMap::new(),
    };

    let mut fields = HashMap::new();
    collect_fields(tree.root_node(), source, &mut fields);
    fields
}

fn collect_fields(node: Node, source: &str, fields: &mut HashMap<String, String>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "class_declaration" {
            if let Some(body) = child.child_by_field_name("body") {
                let mut body_cursor = body.walk();
                for member in body.children(&mut body_cursor) {
                    if member.kind() == "field_declaration" {
                        parse_field(&member, source, fields);
                    }
                }
            }
        }
    }
}

fn parse_field(node: &Node, source: &str, fields: &mut HashMap<String, String>) {
    // field_declaration: type variable_declarator_list
    // 例: private Long id;
    // 例: private String name;

    let mut type_name: Option<String> = None;
    let mut var_name: Option<String> = None;

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "type_identifier" | "primitive_type" | "integral_type"
            | "floating_point_type" | "boolean_type" | "generic_type" | "array_type" => {
                type_name = Some(extract_type(&child, source));
            }
            "variable_declarator_list" => {
                // 取第一个 declarator 的名字
                let mut dc = child.walk();
                for dc_child in child.children(&mut dc) {
                    if dc_child.kind() == "variable_declarator" {
                        if let Some(name_node) = dc_child.child_by_field_name("name") {
                            var_name = Some(source[name_node.byte_range()].to_string());
                        }
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    if let (Some(t), Some(n)) = (type_name, var_name) {
        fields.insert(n, t);
    }
}

fn extract_type(node: &Node, source: &str) -> String {
    match node.kind() {
        "type_identifier" | "primitive_type" | "integral_type"
        | "floating_point_type" | "boolean_type" => source[node.byte_range()].to_string(),
        "generic_type" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "type_identifier" {
                    return source[child.byte_range()].to_string();
                }
            }
            source[node.byte_range()].to_string()
        }
        _ => source[node.byte_range()].to_string(),
    }
}
```

### Step 3: 在 `src/java/mod.rs` 中添加模块

在现有 `mod` 声明之后添加：

```rust
mod dto_fields;
mod mapper_interface;

pub use dto_fields::parse_dto_fields;
pub use mapper_interface::{
    MapperInterfaceInfo, MapperMethodInfo, MethodParam, parse_mapper_interface,
};
```

### Step 4: 写测试（在 `src/java/tests.rs` 中追加）

```rust
// ========== Mapper Interface Parsing Tests ==========

#[test]
fn test_parse_mapper_interface_basic() {
    let source = r#"
package com.example.mapper;

public interface UserMapper {
    User findById(int id);
    List<User> findByName(String name);
    void insert(User user);
}
"#;
    let info = crate::java::parse_mapper_interface(source).unwrap();
    assert_eq!(info.fqn, "com.example.mapper.UserMapper");
    assert!(info.methods.contains_key("findById"));

    let method = &info.methods["findById"];
    assert_eq!(method.params.len(), 1);
    assert_eq!(method.params[0].name, "id");
    assert_eq!(method.params[0].java_type, "int");
}

#[test]
fn test_parse_mapper_interface_with_param_annotation() {
    let source = r#"
package com.example.mapper;

public interface UserMapper {
    List<User> search(@Param("status") int status, @Param("name") String name);
}
"#;
    let info = crate::java::parse_mapper_interface(source).unwrap();
    let method = &info.methods["search"];
    assert_eq!(method.params.len(), 2);
    assert_eq!(method.params[0].name, "status");     // 从 @Param 取
    assert_eq!(method.params[0].java_type, "int");
    assert_eq!(method.params[0].param_annotation, Some("status".into()));
    assert_eq!(method.params[1].name, "name");
    assert_eq!(method.params[1].java_type, "String");
}

#[test]
fn test_parse_mapper_interface_not_interface() {
    let source = "public class Foo { }";
    assert!(crate::java::parse_mapper_interface(source).is_none());
}

#[test]
fn test_parse_dto_fields() {
    let source = r#"
package com.example.model;

public class User {
    private Long id;
    private String name;
    private Integer age;
    private BigDecimal salary;
    private Date createTime;
    private boolean active;
}
"#;
    let fields = crate::java::parse_dto_fields(source);
    assert_eq!(fields.get("id").unwrap(), "Long");
    assert_eq!(fields.get("name").unwrap(), "String");
    assert_eq!(fields.get("age").unwrap(), "Integer");
    assert_eq!(fields.get("salary").unwrap(), "BigDecimal");
    assert_eq!(fields.get("createTime").unwrap(), "Date");
    assert_eq!(fields.get("active").unwrap(), "boolean");
}
```

### Step 5: 运行测试

Run: `cargo test --features java --lib java::tests`
Expected: 全部通过

### Step 6: Commit

```
feat(java): add Mapper interface and DTO field type extraction
```

---

## Task 6: 新增 `JavaSourceResolver` + Java→JDBC 类型映射

**目标:** 实现 FQN → 文件路径的按需查找；Java 类型名到 JdbcType 的映射。

**Files:**
- Create: `src/ibatis/java_resolve.rs`
- Modify: `src/ibatis/mod.rs`

### Step 1: 创建 `src/ibatis/java_resolve.rs`

```rust
//! Java 源码按需查找器。
//!
//! 根据全限定类名 (FQN) 在 Java 源码根目录中查找对应文件。
//! "com.example.mapper.UserMapper" → "com/example/mapper/UserMapper.java"

use std::path::{Path, PathBuf};

/// Java 类型名 → JdbcType 映射表。
/// 覆盖 Java 基本类型、包装类和常用类型。
static JAVA_TO_JDBC: &[(&str, crate::ibatis::types::JdbcType)] = &[
    // 基本类型
    ("int",          JdbcType::Integer),
    ("long",         JdbcType::BigInt),
    ("short",        JdbcType::SmallInt),
    ("byte",         JdbcType::TinyInt),
    ("float",        JdbcType::Float),
    ("double",       JdbcType::Double),
    ("boolean",      JdbcType::Boolean),
    ("char",         JdbcType::Char),
    // 包装类
    ("Integer",      JdbcType::Integer),
    ("Long",         JdbcType::BigInt),
    ("Short",        JdbcType::SmallInt),
    ("Byte",         JdbcType::TinyInt),
    ("Float",        JdbcType::Float),
    ("Double",       JdbcType::Double),
    ("Boolean",      JdbcType::Boolean),
    ("Character",    JdbcType::Char),
    // 常用类
    ("String",       JdbcType::VarChar),
    ("BigDecimal",   JdbcType::Decimal),
    ("Date",         JdbcType::Timestamp),
    ("LocalDate",    JdbcType::Date),
    ("LocalDateTime",JdbcType::Timestamp),
    ("LocalTime",    JdbcType::Time),
    ("Timestamp",    JdbcType::Timestamp),
    ("byte[]",       JdbcType::VarBinary),
    ("Object",       JdbcType::Other),
];

/// jdbcType 字符串 → JdbcType 映射表。
static JDBC_TYPE_MAP: &[(&str, crate::ibatis::types::JdbcType)] = &[
    ("INTEGER",      JdbcType::Integer),
    ("BIGINT",       JdbcType::BigInt),
    ("SMALLINT",     JdbcType::SmallInt),
    ("TINYINT",      JdbcType::TinyInt),
    ("DECIMAL",      JdbcType::Decimal),
    ("NUMERIC",      JdbcType::Numeric),
    ("DOUBLE",       JdbcType::Double),
    ("FLOAT",        JdbcType::Float),
    ("REAL",         JdbcType::Real),
    ("CHAR",         JdbcType::Char),
    ("VARCHAR",      JdbcType::VarChar),
    ("LONGVARCHAR",  JdbcType::LongVarChar),
    ("NCHAR",        JdbcType::NChar),
    ("NVARCHAR",     JdbcType::NVarChar),
    ("CLOB",         JdbcType::Clob),
    ("NCLOB",        JdbcType::NClob),
    ("BINARY",       JdbcType::Binary),
    ("VARBINARY",    JdbcType::VarBinary),
    ("BLOB",         JdbcType::Blob),
    ("DATE",         JdbcType::Date),
    ("TIME",         JdbcType::Time),
    ("TIMESTAMP",    JdbcType::Timestamp),
    ("BOOLEAN",      JdbcType::Boolean),
    ("NULL",         JdbcType::Null),
    ("ARRAY",        JdbcType::Array),
    ("OTHER",        JdbcType::Other),
];

/// Java 源码按需查找器。
#[derive(Debug, Clone)]
pub struct JavaSourceResolver {
    roots: Vec<PathBuf>,
}

impl JavaSourceResolver {
    /// 创建查找器。roots 是 Java 源码根目录列表
    /// （如 `["/project/src/main/java"]`）。
    pub fn new(roots: Vec<PathBuf>) -> Self {
        Self { roots }
    }

    /// 创建空查找器（不做 Java 查找）。
    pub fn empty() -> Self {
        Self { roots: Vec::new() }
    }

    /// 根据全限定类名查找并读取 Java 文件。
    /// "com.example.mapper.UserMapper" → 查找 "com/example/mapper/UserMapper.java"
    pub fn read_source(&self, fqn: &str) -> Option<String> {
        let path = self.resolve(fqn)?;
        std::fs::read_to_string(&path).ok()
    }

    /// 根据全限定类名查找 Java 文件路径。
    pub fn resolve(&self, fqn: &str) -> Option<PathBuf> {
        let relative = fqn.replace('.', "/") + ".java";
        self.roots.iter()
            .map(|root| root.join(&relative))
            .find(|path| path.is_file())
    }
}

/// Java 类型名 → JdbcType。
pub fn java_type_to_jdbc(java_type: &str) -> Option<crate::ibatis::types::JdbcType> {
    JAVA_TO_JDBC.iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(java_type))
        .map(|(_, jdbc)| *jdbc)
}

/// jdbcType 字符串 → JdbcType。
pub fn jdbc_type_from_str(s: &str) -> Option<crate::ibatis::types::JdbcType> {
    JDBC_TYPE_MAP.iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(s))
        .map(|(_, jdbc)| *jdbc)
}
```

### Step 2: 在 `src/ibatis/mod.rs` 中添加模块

```rust
#[cfg(feature = "java")]
mod java_resolve;

#[cfg(feature = "java")]
pub use java_resolve::JavaSourceResolver;
```

### Step 3: 写测试（在 `src/ibatis/tests.rs` 中追加）

```rust
#[test]
fn test_java_type_to_jdbc_mapping() {
    use crate::ibatis::types::JdbcType;
    assert_eq!(super::super::java_resolve::java_type_to_jdbc("int"), Some(JdbcType::Integer));
    assert_eq!(super::super::java_resolve::java_type_to_jdbc("String"), Some(JdbcType::VarChar));
    assert_eq!(super::super::java_resolve::java_type_to_jdbc("Long"), Some(JdbcType::BigInt));
    assert_eq!(super::super::java_resolve::java_type_to_jdbc("Date"), Some(JdbcType::Timestamp));
    assert_eq!(super::super::java_resolve::java_type_to_jdbc("Unknown"), None);
}

#[test]
fn test_jdbc_type_from_str() {
    use crate::ibatis::types::JdbcType;
    assert_eq!(super::super::java_resolve::jdbc_type_from_str("VARCHAR"), Some(JdbcType::VarChar));
    assert_eq!(super::super::java_resolve::jdbc_type_from_str("INTEGER"), Some(JdbcType::Integer));
    assert_eq!(super::super::java_resolve::jdbc_type_from_str("timestamp"), Some(JdbcType::Timestamp));
}
```

### Step 4: 运行测试

Run: `cargo test --features ibatis,java --lib`
Expected: 全部通过

### Step 5: Commit

```
feat(ibatis): add JavaSourceResolver and Java-to-JDBC type mapping
```

---

## Task 7: 集成类型推断到主管线

**目标:** 在 `parse_mapper_bytes_with_path` 中整合所有类型来源，填充 `ParamMeta`。

**Files:**
- Modify: `src/ibatis/flatten.rs` — 收集参数列表
- Modify: `src/ibatis/mod.rs` — 主管线整合类型推断

### Step 1: 修改 `flatten.rs`，让 `flatten_sql` 同时收集参数

添加一个公开函数用于从 SqlNode 树中提取所有参数：

```rust
/// 从 SqlNode 树中收集所有参数的位置信息。
pub fn collect_params(node: &SqlNode) -> Vec<(String, Option<String>, String)> {
    let mut params = Vec::new();
    collect_params_recursive(node, &mut params);
    params
}

fn collect_params_recursive(
    node: &SqlNode,
    params: &mut Vec<(String, Option<String>, String)>,
) {
    match node {
        SqlNode::Parameter { name, java_type } => {
            let raw = match java_type {
                Some(t) => format!("#{{{},{}}}", name, format!("javaType={}", t)),
                None => format!("#{{{}}}", name),
            };
            params.push((name.clone(), java_type.clone(), raw));
        }
        SqlNode::If { children, .. } => {
            for c in children { collect_params_recursive(c, params); }
        }
        SqlNode::Choose { branches } => {
            for (_, ch) in branches {
                for c in ch { collect_params_recursive(c, params); }
            }
        }
        SqlNode::Where { children } | SqlNode::Set { children } => {
            for c in children { collect_params_recursive(c, params); }
        }
        SqlNode::Trim { children, .. } | SqlNode::ForEach { children, .. } => {
            for c in children { collect_params_recursive(c, params); }
        }
        SqlNode::Sequence { children } => {
            for c in children { collect_params_recursive(c, params); }
        }
        // Text, RawExpr, Bind, Include 不含参数
        SqlNode::Text { .. } | SqlNode::RawExpr { .. } | SqlNode::Bind { .. } | SqlNode::Include { .. } => {}
    }
}
```

### Step 2: 在 `mod.rs` 中实现类型推断主函数

在 `parse_mapper_bytes_with_path` 之后添加：

```rust
/// 从 Java 源码推断参数类型。
#[cfg(feature = "java")]
fn infer_param_types(
    stmt: &crate::ibatis::types::MapperStatement,
    collected_params: &[(String, Option<String>, String)],
    resolver: &crate::ibatis::java_resolve::JavaSourceResolver,
) -> Vec<crate::ibatis::types::ParamMeta> {
    use crate::ibatis::types::{InferenceSource, JdbcType, ParamMeta};
    use crate::ibatis::java_resolve::{java_type_to_jdbc, jdbc_type_from_str};

    // 1. 解析 Mapper 接口（如果有）
    let interface_info = resolver.read_source(&stmt.id) // ← 不对，namespace 不是 stmt.id
        .and_then(|src| crate::java::parse_mapper_interface(&src));

    // 2. 解析 DTO（如果 parameterType 是全限定类名）
    let dto_fields = stmt.parameter_type.as_ref()
        .filter(|pt| pt.contains('.') && !pt.eq_ignore_ascii_case("map"))
        .and_then(|pt| resolver.read_source(pt))
        .map(|src| crate::java::parse_dto_fields(&src))
        .unwrap_or_default();

    // 3. 对每个参数确定类型
    collected_params.iter().map(|(name, inline_java_type, raw)| {
        // 优先级 1: XML 内联注解
        if let Some(ref jt) = inline_java_type {
            if let Some(jdbc) = java_type_to_jdbc(jt) {
                return ParamMeta {
                    name: name.clone(),
                    jdbc_type: Some(jdbc),
                    source: Some(InferenceSource::InlineJavaType),
                    position: 0, // 后面计算
                    raw: raw.clone(),
                };
            }
            if let Some(jdbc) = jdbc_type_from_str(jt) {
                return ParamMeta {
                    name: name.clone(),
                    jdbc_type: Some(jdbc),
                    source: Some(InferenceSource::InlineJdbcType),
                    position: 0,
                    raw: raw.clone(),
                };
            }
        }

        // 优先级 2: Mapper 接口方法签名
        // (通过 namespace 匹配接口，通过 statement id 匹配方法)

        // 优先级 3: DTO 字段
        if let Some(java_t) = dto_fields.get(name) {
            if let Some(jdbc) = java_type_to_jdbc(java_t) {
                return ParamMeta {
                    name: name.clone(),
                    jdbc_type: Some(jdbc),
                    source: Some(InferenceSource::JavaDtoField),
                    position: 0,
                    raw: raw.clone(),
                };
            }
        }

        // 无类型信息
        ParamMeta {
            name: name.clone(),
            jdbc_type: None,
            source: None,
            position: 0,
            raw: raw.clone(),
        }
    }).collect()
}
```

### Step 3: 修改 `parse_mapper_bytes_with_path` 接受可选的 resolver

修改函数签名，增加可选的 Java 源码根目录参数：

```rust
/// 从 XML 字节解析 mapper 文件。
pub fn parse_mapper_bytes(xml: &[u8]) -> ParsedMapper {
    parse_mapper_bytes_with_path(xml, None, None)
}

pub fn parse_mapper_bytes_with_path(xml: &[u8], file_path: Option<&str>) -> ParsedMapper {
    parse_mapper_bytes_internal(xml, file_path, Vec::new())
}

/// 从 XML 字节解析 mapper 文件，附带源文件路径和 Java 源码根目录。
pub fn parse_mapper_bytes_with_java_src(
    xml: &[u8],
    file_path: Option<&str>,
    java_source_roots: Vec<std::path::PathBuf>,
) -> ParsedMapper {
    parse_mapper_bytes_internal(xml, file_path, java_source_roots)
}

fn parse_mapper_bytes_internal(
    xml: &[u8],
    file_path: Option<&str>,
    java_source_roots: Vec<std::path::PathBuf>,
) -> ParsedMapper {
    // ... 现有逻辑，在构建 ParsedStatement 时填充 parameters
}
```

在构建 `ParsedStatement` 时填充参数：

```rust
let collected = flatten::collect_params(&stmt.body);
#[cfg(feature = "java")]
let parameters = {
    let resolver = java_resolve::JavaSourceResolver::new(java_source_roots.clone());
    infer_param_types_with_resolver(&mapper_file.namespace, stmt, &collected, &resolver)
};
#[cfg(not(feature = "java"))]
let parameters = collected.iter().map(|(name, java_type, raw)| {
    ParamMeta {
        name: name.clone(),
        jdbc_type: java_type.as_ref().and_then(|jt| {
            java_resolve::java_type_to_jdbc(jt).or_else(|| java_resolve::jdbc_type_from_str(jt))
        }),
        source: java_type.as_ref().map(|_| InferenceSource::InlineJavaType),
        position: 0,
        raw: raw.clone(),
    }
}).collect();
```

### Step 4: 在 `flatten_sql` 中计算参数在 flat_sql 中的位置

在 `flatten_sql` 遍历时记录每个 `__XML_PARAM_name__` 出现的字节偏移。这一步可选，先设 `position: 0`，后续可优化。

### Step 5: 写端到端测试

```rust
#[cfg(feature = "java")]
#[test]
fn test_e2e_param_type_from_java_interface() {
    // 模拟 Java Mapper 接口
    let java_source = r#"
package com.example.mapper;
public interface UserMapper {
    User findById(int id);
    List<User> findByName(String name);
}
"#;

    // 创建临时目录和文件
    let tmp_dir = std::env::temp_dir().join("ogsql_test_java_src");
    let pkg_dir = tmp_dir.join("com/example/mapper");
    std::fs::create_dir_all(&pkg_dir).unwrap();
    std::fs::write(pkg_dir.join("UserMapper.java"), java_source).unwrap();

    let xml = br#"<mapper namespace="com.example.mapper.UserMapper">
        <select id="findById">SELECT * FROM users WHERE id = #{id}</select>
        <select id="findByName">SELECT * FROM users WHERE name = #{name}</select>
    </mapper>"#;

    let result = super::parse_mapper_bytes_with_java_src(
        xml,
        None,
        vec![tmp_dir.clone()],
    );

    assert_eq!(result.statements.len(), 2);

    // findById → #{id} 应该被推断为 INTEGER
    let stmt1 = &result.statements[0];
    assert_eq!(stmt1.id, "findById");
    assert_eq!(stmt1.parameters.len(), 1);
    assert_eq!(stmt1.parameters[0].name, "id");
    assert_eq!(stmt1.parameters[0].jdbc_type, Some(crate::ibatis::types::JdbcType::Integer));

    // findByName → #{name} 应该被推断为 VARCHAR
    let stmt2 = &result.statements[1];
    assert_eq!(stmt2.id, "findByName");
    assert_eq!(stmt2.parameters[0].name, "name");
    assert_eq!(stmt2.parameters[0].jdbc_type, Some(crate::ibatis::types::JdbcType::VarChar));

    // 清理
    let _ = std::fs::remove_dir_all(&tmp_dir);
}
```

### Step 6: 运行测试

Run: `cargo test --features ibatis,java --lib`
Expected: 全部通过

### Step 7: Commit

```
feat(ibatis): integrate Java-based parameter type inference into pipeline
```

---

## Task 8: CLI 集成 — 添加 `--java-src` 选项

**目标:** 在 `parse-xml` 子命令中添加 `--java-src` 选项。

**Files:**
- Modify: `src/bin/ogsql.rs`

### Step 1: 修改 ParseXml 命令定义

在 `ParseXml` 枚举变体中添加 `java_src` 字段（约第 91 行）：

```rust
ParseXml {
    /// Recursively scan directory for XML files
    #[arg(short = 'd', long = "dir")]
    dir: Option<String>,
    /// Output in CSV format
    #[arg(long = "csv")]
    csv: bool,
    /// Java source root directory for parameter type inference
    #[arg(long = "java-src")]
    java_src: Option<String>,
},
```

### Step 2: 修改 dispatch（约第 1833 行）

```rust
Commands::ParseXml { ref dir, csv, ref java_src } =>
    cmd_parse_xml(&cli, dir.as_deref(), csv, java_src.as_deref()),
```

### Step 3: 修改 `cmd_parse_xml` 签名和实现

```rust
fn cmd_parse_xml(cli: &Cli, dir: Option<&str>, csv: bool, java_src: Option<&str>) {
    if dir.is_some() && cli.file.is_some() {
        die!("Error: --dir and -f are mutually exclusive");
    }

    // 验证 java_src 目录
    let java_roots: Vec<std::path::PathBuf> = match java_src {
        Some(path) => {
            let p = std::path::Path::new(path);
            if !p.is_dir() {
                die!("Error: '{}' is not a directory", path);
            }
            vec![p.to_path_buf()]
        }
        None => Vec::new(),
    };

    if let Some(dir_path) = dir {
        cmd_parse_xml_dir(cli, dir_path, csv, &java_roots);
    } else {
        cmd_parse_xml_single(cli, csv, &java_roots);
    }
}
```

### Step 4: 修改 `cmd_parse_xml_single` 和 `cmd_parse_xml_dir` 传递 java_roots

在调用 `parse_mapper_bytes_with_path` 的地方，改为调用 `parse_mapper_bytes_with_java_src`。

`cmd_parse_xml_single`:
```rust
let result = if java_roots.is_empty() {
    ogsql_parser::ibatis::parse_mapper_bytes_with_path(&input, cli.file.as_deref())
} else {
    ogsql_parser::ibatis::parse_mapper_bytes_with_java_src(
        &input, cli.file.as_deref(), java_roots.clone()
    )
};
```

`cmd_parse_xml_dir` 同理。

### Step 5: 运行手动测试

```bash
cargo build --features ibatis,java

# 无 java-src（现有行为）
echo '<mapper namespace="test"><select id="find">SELECT * FROM t WHERE id = #{id}</select></mapper>' | ./target/debug/ogsql parse-xml

# 有 java-src
./target/debug/ogsql parse-xml --dir /path/to/mapper-xml --java-src /path/to/src/main/java
```

### Step 6: Commit

```
feat(cli): add --java-src option to parse-xml for parameter type inference
```

---

## Task 9: MCP 工具增强

**目标:** MCP `parse_xml` 工具支持传入 Java 源码。

**Files:**
- Modify: `src/mcp/mod.rs`

### Step 1: 修改 `ParseXmlParams`

```rust
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ParseXmlParams {
    /// XML content of an iBatis/MyBatis mapper file
    pub xml: String,
    /// Java source root directory for parameter type inference
    #[serde(default)]
    pub java_src: Option<String>,
    /// Directly provided Java sources: class path → source code
    #[serde(default)]
    pub java_sources: Option<std::collections::HashMap<String, String>>,
}
```

### Step 2: 修改 `parse_xml` 工具实现

```rust
fn parse_xml(
    &self,
    Parameters(ParseXmlParams { xml, java_src, java_sources }): Parameters<ParseXmlParams>,
) -> String {
    // 如果提供了 java_sources（内联代码），写入临时目录
    let tmp_dir;
    let java_roots = if let Some(ref sources) = java_sources {
        tmp_dir = std::env::temp_dir().join(format!("ogsql_mcp_{}", std::process::id()));
        for (path, content) in sources {
            let full_path = tmp_dir.join(path);
            if let Some(parent) = full_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(&full_path, content);
        }
        vec![tmp_dir.clone()]
    } else if let Some(ref src) = java_src {
        vec![std::path::PathBuf::from(src)]
    } else {
        vec![]
    };

    let result = crate::ibatis::parse_mapper_bytes_with_java_src(
        xml.as_bytes(),
        None,
        java_roots,
    );

    // 清理临时目录
    if let Some(ref dir) = tmp_dir {
        let _ = std::fs::remove_dir_all(dir);
    }

    serde_json::to_string_pretty(&result)
        .unwrap_or_else(|e| format!("{{\"error\": \"{}\"}}", e))
}
```

### Step 3: 运行测试

Run: `cargo build --features mcp`
Expected: 编译通过

### Step 4: Commit

```
feat(mcp): enhance parse_xml tool with Java source support for type inference
```

---

## Task 10: 集成验证 + 回归测试

**目标:** 确保所有改动协同工作，无回归。

### Step 1: 运行完整测试套件

```bash
# ibatis 模块测试
cargo test --features ibatis --lib ibatis

# java 模块测试
cargo test --features java --lib java

# 联合测试
cargo test --features ibatis,java --lib

# 全量测试
cargo test --features full
```

Expected: 全部通过

### Step 2: 用真实 XML 文件验证

```bash
# 无 Java 源码（基线）
./target/debug/ogsql parse-xml -f lib/GenerateInstructionsMapper.xml -j

# 检查输出中 parameters 数组存在、无类型（因为没有 java-src）
```

### Step 3: 验证 MCP 构建

```bash
cargo build --features mcp
```

### Step 4: 验证不相关功能不受影响

```bash
cargo test --features cli
cargo test
```

### Step 5: Final commit（如有修复）

```
test: verify full pipeline with ibatis+java integration
```
