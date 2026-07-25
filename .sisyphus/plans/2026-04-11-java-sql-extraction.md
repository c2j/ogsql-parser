# Java SQL Extraction Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 为 ogsql-parser 新增 Java 源代码 SQL 提取能力：从 `.java` 文件中识别并提取嵌入在注解（`@Query`、`@NamedQuery`、`@SqlUpdate` 等）、方法调用参数（`createNativeQuery`、`prepareStatement` 等）和字符串常量中的 SQL 语句，将提取的 SQL 馈入现有 Parser 管线得到结构化 AST。

**Architecture:** 新增 `src/java/` 模块（与 `ibatis/` 平级），作为独立的前置预处理层。`.java` → tree-sitter CST → 模式匹配提取 → 字符串拼接合并 → 现有 `Parser::parse_sql()` → AST。不修改任何现有核心模块代码。通过 Cargo feature gate `java` 控制，依赖 `tree-sitter` + `tree-sitter-java` 做 CST 解析。

**Tech Stack:** Rust 2021, tree-sitter 0.24, tree-sitter-java 0.23, serde, thiserror

**Design Decisions:**
- 使用 tree-sitter-java 解析 Java，不手写 Java 解析器
- P0 优先处理注解声明式 SQL（覆盖 ~60% 场景）
- P1 处理方法调用参数 SQL（覆盖 ~20%）
- P2 处理字符串常量 SQL（覆盖 ~15%）
- P3（暂不实现）StringBuilder 动态拼接
- JPQL vs native SQL 必须区分标注
- 参数标记保留原样（`?`、`:param`、`?1`、`#{}`），不做替换
- 字符串拼接（`"str1" + "str2"`）合并后再输出
- Java 15+ text block（`""" ... """`）支持

**Anti-Slop Rules:**
- ❌ 禁止 trait 层次结构 / `dyn` / builder pattern / visitor pattern
- ❌ 禁止完整 Java 数据流分析
- ❌ 禁止 async
- ❌ 禁止 JPQL 解析（只标注，不转换）
- ✅ 使用 flat enum + match（跟随 `src/ibatis/types.rs` 模式）
- ✅ 所有公开类型 derive `Debug, Clone, Serialize, Deserialize`
- ✅ 错误收集模式（跟随 `src/parser/mod.rs` 和 `src/ibatis/mod.rs`）

---

## 模块结构

```
src/java/
├── mod.rs          # 公开 API + 模块导出 (~80 行)
├── types.rs        # 数据模型: ExtractedSql, SqlOrigin, JavaExtractResult 等 (~180 行)
├── error.rs        # JavaError (~40 行)
├── extract.rs      # CST 遍历 + SQL 提取核心逻辑 (~600 行)
└── tests.rs        # 单元测试 (~500 行)
```

---

## Task 1: 基础设施 — Cargo.toml + 模块骨架 + 错误类型 + 数据模型

**Files:**
- Modify: `Cargo.toml`
- Create: `src/java/mod.rs`
- Create: `src/java/error.rs`
- Create: `src/java/types.rs`
- Modify: `src/lib.rs`

**Step 1: 在 Cargo.toml 中添加依赖和 feature**

在 `[dependencies]` 中添加:
```toml
tree-sitter = { version = "0.24", optional = true }
tree-sitter-java = { version = "0.23", optional = true }
```

在 `[features]` 中修改:
```toml
java = ["dep:tree-sitter", "dep:tree-sitter-java"]
full = ["cli", "ibatis", "java", "serve", "tui"]
```

**Step 2: 创建 `src/java/error.rs` — 错误类型**

```rust
//! Java 源码 SQL 提取错误类型。

use crate::parser::ParserError;

/// Java SQL 提取过程中可能产生的错误。
#[derive(Debug, Clone, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum JavaError {
    /// tree-sitter 解析错误
    #[error("tree-sitter parse error: {message}")]
    ParseError { message: String },

    /// 提取的 SQL 解析错误（由核心 Parser 产生）
    #[error("SQL parse error in {origin}: {0}")]
    SqlParseError { origin: String, error: ParserError },

    /// 文件读取错误
    #[error("IO error: {0}")]
    IoError(String),

    /// 编码错误
    #[error("encoding error: {0}")]
    EncodingError(String),
}
```

**Step 3: 创建 `src/java/types.rs` — 数据模型**

```rust
//! Java SQL 提取数据模型。

use crate::parser::ParserError;
use crate::ast::StatementInfo;

// ── 顶层结果 ──

/// Java 文件 SQL 提取的完整结果。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JavaExtractResult {
    /// 源文件路径（可能为空，如从 stdin 读取）
    pub file_path: String,
    /// 提取到的 SQL 列表
    pub extractions: Vec<ExtractedSql>,
    /// 错误列表
    pub errors: Vec<crate::java::error::JavaError>,
}

// ── 单条提取结果 ──

/// 从 Java 代码中提取的一条 SQL。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExtractedSql {
    /// 提取出的 SQL 文本（已合并字符串拼接）
    pub sql: String,
    /// 来源信息
    pub origin: SqlOrigin,
    /// SQL 类型
    pub sql_kind: SqlKind,
    /// 参数标记风格
    pub parameter_style: ParameterStyle,
    /// 是否由多段字符串拼接而成
    pub is_concatenated: bool,
    /// 是否来自 Java 15+ text block (`""" ... """`)
    pub is_text_block: bool,
    /// SQL 解析结果（可选，解析失败时为 None）
    pub parse_result: Option<SqlParseResult>,
}

/// SQL 解析结果。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SqlParseResult {
    /// 解析成功的语句列表
    pub statements: Vec<StatementInfo>,
    /// 解析错误列表
    pub errors: Vec<ParserError>,
}

// ── 来源信息 ──

/// SQL 在 Java 源码中的来源。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SqlOrigin {
    /// 提取方式
    pub method: ExtractionMethod,
    /// 所在 Java 类名
    pub class_name: Option<String>,
    /// 所在方法签名
    pub method_name: Option<String>,
    /// 注解名（仅注解模式）
    pub annotation_name: Option<String>,
    /// 被调用的 API 方法名（仅方法调用模式）
    pub api_method_name: Option<String>,
    /// 变量名（仅常量模式）
    pub variable_name: Option<String>,
    /// 源码行号（1-based）
    pub line: usize,
    /// 源码列号（0-based）
    pub column: usize,
}

// ── 枚举 ──

/// SQL 提取方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ExtractionMethod {
    /// 注解: @Query, @NamedQuery, @SqlUpdate 等
    Annotation,
    /// API 方法调用: createNativeQuery, prepareStatement 等
    MethodCall,
    /// 字符串常量: private static final String SQL = "..."
    Constant,
}

/// SQL 方言类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SqlKind {
    /// 原生 SQL
    NativeSql,
    /// JPQL (Java Persistence Query Language)
    Jpql,
    /// DDL (CREATE TABLE, ALTER TABLE 等)
    Ddl,
}

/// 参数标记风格。
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ParameterStyle {
    /// JDBC 位置参数: ?
    PositionalQuestion,
    /// JPA 位置参数: ?1, ?2
    PositionalNumbered,
    /// 命名参数: :paramName
    NamedColon,
    /// MyBatis/JDBI 风格: #{param} 或 ${param}
    NamedHash,
    /// 未检测到参数或无法确定
    None,
}
```

**Step 4: 创建 `src/java/mod.rs` — 模块骨架**

```rust
//! Java 源码 SQL 提取支持。
//!
//! 从 Java 源文件中提取嵌入在注解、方法调用参数、字符串常量中的 SQL 语句，
//! 并将提取的 SQL 馈入核心 Parser 得到结构化 AST。

pub mod error;
pub mod extract;
pub mod types;

pub use error::JavaError;
pub use types::{
    ExtractedSql, ExtractionMethod, JavaExtractResult, ParameterStyle, SqlKind, SqlOrigin,
    SqlParseResult,
};

use tree_sitter::Parser;

/// 从 Java 源码字节提取 SQL。
///
/// 接受原始字节（UTF-8），返回提取结果。
/// 内部使用 tree-sitter-java 构建 CST，然后遍历提取 SQL。
pub fn extract_sql_from_java(source: &str, file_path: &str) -> JavaExtractResult {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_java::LANGUAGE.into())
        .expect("Failed to set Java language for tree-sitter");

    let tree = match parser.parse(source, None) {
        Some(tree) => tree,
        None => {
            return JavaExtractResult {
                file_path: file_path.to_string(),
                extractions: Vec::new(),
                errors: vec![JavaError::ParseError {
                    message: "tree-sitter returned no parse tree".to_string(),
                }],
            };
        }
    };

    extract::extract(source, tree.root_node(), file_path)
}
```

**Step 5: 创建 `src/java/extract.rs` — 占位**

```rust
//! Java CST 遍历与 SQL 提取。

use tree_sitter::Node;

use crate::java::error::JavaError;
use crate::java::types::*;

/// 从 tree-sitter 根节点提取 SQL。
pub fn extract(source: &str, root: Node, file_path: &str) -> JavaExtractResult {
    let mut ctx = ExtractContext {
        source,
        file_path,
        extractions: Vec::new(),
        errors: Vec::new(),
        class_name: None,
        method_name: None,
    };

    ctx.visit(root);

    JavaExtractResult {
        file_path: file_path.to_string(),
        extractions: ctx.extractions,
        errors: ctx.errors,
    }
}

struct ExtractContext<'a> {
    source: &'a str,
    file_path: &'a str,
    extractions: Vec<ExtractedSql>,
    errors: Vec<JavaError>,
    class_name: Option<String>,
    method_name: Option<String>,
}

impl<'a> ExtractContext<'a> {
    fn visit(&mut self, node: Node) {
        // 占位，后续 Task 填充
        let _ = node;
    }

    /// 从 tree-sitter Node 提取文本。
    fn node_text(&self, node: Node) -> String {
        self.source[node.byte_range()].to_string()
    }
}
```

**Step 6: 修改 `src/lib.rs` — 注册 java 模块**

在 `src/lib.rs` 末尾（`#[cfg(feature = "ibatis")]` 之后）添加:

```rust
#[cfg(feature = "java")]
pub mod java;
```

**Step 7: 编译验证**

```bash
cargo build --features java 2>&1
```

预期: 编译成功，无错误。

**Step 8: Commit**

```bash
git add Cargo.toml src/lib.rs src/java/
git commit -m "feat(java): scaffold java module with tree-sitter dependency"
```

---

## Task 2: P0 — 注解 SQL 提取

**Files:**
- Modify: `src/java/extract.rs`
- Create: `src/java/tests.rs`

**目标:** 提取 `@Query`、`@NamedQuery`、`@SqlUpdate`、`@SqlQuery` 等注解中的 SQL。

这是覆盖面最广的模式（~60%）。tree-sitter-java 将注解解析为 `annotation` 节点，包含 `identifier`（注解名）和 `annotation_argument_list`（参数）。

**关键 tree-sitter-java 节点类型:**

```
(annotation
  name: (identifier) @name              ; 注解名如 "Query"
  arguments: (annotation_argument_list   ; 参数列表
    (element_value_pair
      key: (identifier)                  ; "value" / "query" / "nativeQuery"
      value: (string_literal ...))))     ; SQL 文本

(marker_annotation
  name: (identifier))                    ; 无参注解如 @Override

; @NamedQuery 和 @NamedQueries 是特殊的
; @NamedQuery(name="...", query="...")
; @NamedQueries({...}) 内含多个 @NamedQuery
```

**Java 字符串拼接处理:**

在注解中 SQL 常被拆成多段:
```java
@Query("SELECT * FROM users " +
       "WHERE status = :status")
```

tree-sitter-java 将 `"str1" + "str2"` 解析为:
```
(binary_expression
  left: (string_literal)
  operator: "+"
  right: (string_literal))
```

所以提取函数需要递归处理 `binary_expression` 中连续的 `string_literal` 节点。

**Java text block 处理:**

Java 15+ 的 `""" ... """` 被 tree-sitter-java 解析为 `string_literal` 节点（内部含 `multiline_string_fragment`），但首尾各有3个双引号。提取时需要:
1. 识别是否以 `"""` 开头
2. 去掉首尾的 `"""`
3. 去掉共有的缩进前缀（Java text block 的 Incidental White Space 算法）

**Step 1: 写测试 — `src/java/tests.rs`**

```rust
//! Java SQL 提取单元测试。

use crate::java::{extract_sql_from_java, ExtractionMethod, SqlKind};

#[test]
fn test_query_annotation_native_sql() {
    let java = r#"
        public interface UserRepository {
            @Query(value = "SELECT * FROM users WHERE status = :status", nativeQuery = true)
            List<User> findByStatus(@Param("status") int status);
        }
    "#;
    let result = extract_sql_from_java(java, "UserRepository.java");
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(ext.sql.trim(), "SELECT * FROM users WHERE status = :status");
    assert_eq!(ext.origin.method, ExtractionMethod::Annotation);
    assert_eq!(ext.origin.annotation_name.as_deref(), Some("Query"));
    assert_eq!(ext.sql_kind, SqlKind::NativeSql);
}

#[test]
fn test_query_annotation_jpql() {
    let java = r#"
        public interface UserRepository {
            @Query("SELECT u FROM User u WHERE u.status = :status")
            List<User> findByStatus(@Param("status") int status);
        }
    "#;
    let result = extract_sql_from_java(java, "UserRepository.java");
    assert!(result.errors.is_empty());
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(result.extractions[0].sql_kind, SqlKind::Jpql);
}

#[test]
fn test_query_annotation_string_concatenation() {
    let java = r#"
        public interface UserRepository {
            @Query(value = "SELECT * FROM users " +
                   "WHERE status = :status", nativeQuery = true)
            List<User> findByStatus(@Param("status") int status);
        }
    "#;
    let result = extract_sql_from_java(java, "UserRepository.java");
    assert!(result.errors.is_empty());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(ext.sql.trim(), "SELECT * FROM users WHERE status = :status");
    assert!(ext.is_concatenated);
}

#[test]
fn test_query_annotation_text_block() {
    let java = r#"
        public interface UserRepository {
            @Query(value = """
                SELECT *
                FROM users
                WHERE status = :status
                """, nativeQuery = true)
            List<User> findByStatus(@Param("status") int status);
        }
    "#;
    let result = extract_sql_from_java(java, "UserRepository.java");
    assert!(result.errors.is_empty());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert!(ext.is_text_block);
    assert!(ext.sql.contains("SELECT"));
    assert!(ext.sql.contains("FROM users"));
}

#[test]
fn test_named_query_annotation() {
    let java = r#"
        @NamedQuery(name = "User.findById", query = "SELECT u FROM User u WHERE u.id = :id")
        public class User { }
    "#;
    let result = extract_sql_from_java(java, "User.java");
    assert!(result.errors.is_empty());
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(result.extractions[0].origin.annotation_name.as_deref(), Some("NamedQuery"));
    assert_eq!(result.extractions[0].sql_kind, SqlKind::Jpql);
}

#[test]
fn test_sql_update_annotation() {
    let java = r#"
        public interface UserDao {
            @SqlUpdate("INSERT INTO users (name, email) VALUES (:name, :email)")
            void insert(@Bind("name") String name, @Bind("email") String email);
        }
    "#;
    let result = extract_sql_from_java(java, "UserDao.java");
    assert!(result.errors.is_empty());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert!(ext.sql.contains("INSERT INTO users"));
    assert_eq!(ext.origin.annotation_name.as_deref(), Some("SqlUpdate"));
    assert_eq!(ext.sql_kind, SqlKind::NativeSql);
}

#[test]
fn test_multiple_annotations_in_class() {
    let java = r#"
        public interface UserRepository {
            @Query("SELECT u FROM User u WHERE u.id = :id")
            User findById(@Param("id") Long id);

            @Query(value = "SELECT * FROM users WHERE name = :name", nativeQuery = true)
            User findByName(@Param("name") String name);

            @SqlUpdate("UPDATE users SET status = 'inactive' WHERE id = :id")
            void deactivate(@Bind("id") Long id);
        }
    "#;
    let result = extract_sql_from_java(java, "UserRepository.java");
    assert!(result.errors.is_empty());
    assert_eq!(result.extractions.len(), 3);
}

#[test]
fn test_no_sql_in_file() {
    let java = r#"
        public class Calculator {
            public int add(int a, int b) { return a + b; }
        }
    "#;
    let result = extract_sql_from_java(java, "Calculator.java");
    assert!(result.errors.is_empty());
    assert!(result.extractions.is_empty());
}
```

**Step 2: 运行测试确认失败**

```bash
cargo test --features java -- java::tests 2>&1
```

预期: 编译成功但测试断言失败（`extract` 尚未实现提取逻辑）。

**Step 3: 实现 `src/java/extract.rs` — 注解 SQL 提取**

完整实现 `ExtractContext`:

```rust
//! Java CST 遍历与 SQL 提取。

use tree_sitter::Node;

use crate::java::error::JavaError;
use crate::java::types::*;

/// P0 注解名白名单: 直接包含 SQL 的注解
const SQL_ANNOTATIONS: &[&str] = &[
    "Query",
    "NamedQuery",
    "SqlUpdate",
    "SqlQuery",
    "Modifying",
];

/// "nativeQuery = true" 检查结果
#[derive(PartialEq)]
enum NativeQueryFlag {
    True,
    False,
    NotPresent,
}

/// 从 tree-sitter 根节点提取 SQL。
pub fn extract(source: &str, root: Node, file_path: &str) -> JavaExtractResult {
    let mut ctx = ExtractContext {
        source,
        file_path,
        extractions: Vec::new(),
        errors: Vec::new(),
        class_name: None,
        method_name: None,
    };

    ctx.visit(root);

    JavaExtractResult {
        file_path: file_path.to_string(),
        extractions: ctx.extractions,
        errors: ctx.errors,
    }
}

struct ExtractContext<'a> {
    source: &'a str,
    file_path: &'a str,
    extractions: Vec<ExtractedSql>,
    errors: Vec<JavaError>,
    class_name: Option<String>,
    method_name: Option<String>,
}

impl<'a> ExtractContext<'a> {
    fn visit(&mut self, node: Node) {
        match node.kind() {
            "class_declaration" | "interface_declaration" | "enum_declaration" => {
                self.visit_type_declaration(node);
            }
            "method_declaration" => {
                self.visit_method_declaration(node);
            }
            "annotation" => {
                self.visit_annotation(node);
            }
            _ => {
                // 递归遍历子节点
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    self.visit(child);
                }
            }
        }
    }

    /// 遍历类/接口/枚举声明 — 记录类名
    fn visit_type_declaration(&mut self, node: Node) {
        // 提取类名
        let old_class = self.class_name.clone();
        if let Some(name_node) = node.child_by_field_name("name") {
            self.class_name = Some(self.node_text(name_node));
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.visit(child);
        }

        self.class_name = old_class;
    }

    /// 遍历方法声明 — 记录方法名，处理方法上的注解
    fn visit_method_declaration(&mut self, node: Node) {
        let old_method = self.method_name.clone();
        if let Some(name_node) = node.child_by_field_name("name") {
            self.method_name = Some(self.node_text(name_node));
        }

        // 处理方法上的注解（在 modifiers 中）
        if let Some(modifiers) = node.child_by_field_name("modifiers") {
            let mut cursor = modifiers.walk();
            for child in modifiers.children(&mut cursor) {
                if child.kind() == "annotation" {
                    self.visit_annotation(child);
                }
            }
        }

        // 递归处理方法体（P1/P2 需要）
        if let Some(body) = node.child_by_field_name("body") {
            let mut cursor = body.walk();
            for child in body.children(&mut cursor) {
                self.visit(child);
            }
        }

        self.method_name = old_method;
    }

    /// 处理注解节点 — P0 核心
    fn visit_annotation(&mut self, node: Node) {
        let name_node = match node.child_by_field_name("name") {
            Some(n) => n,
            None => return,
        };
        let annotation_name = self.node_text(name_node);

        // 分发处理
        match annotation_name.as_str() {
            "NamedQueries" => self.visit_named_queries(node),
            name if SQL_ANNOTATIONS.contains(&name) => {
                self.visit_sql_annotation(node, &annotation_name);
            }
            _ => {} // 非SQL注解，跳过
        }
    }

    /// 处理 @NamedQueries({...}) — 包含多个内部 @NamedQuery
    fn visit_named_queries(&mut self, node: Node) {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "annotation_argument_list" {
                let mut inner_cursor = child.walk();
                for inner_child in child.children(&mut inner_cursor) {
                    if inner_child.kind() == "annotation" {
                        self.visit_annotation(inner_child);
                    }
                }
            }
        }
    }

    /// 处理包含 SQL 的注解 (@Query, @NamedQuery, @SqlUpdate, @SqlQuery)
    fn visit_sql_annotation(&mut self, node: Node, annotation_name: &str) {
        let args_node = match node.child_by_field_name("arguments") {
            Some(n) if n.kind() == "annotation_argument_list" => n,
            _ => return,
        };

        // 判断 nativeQuery 标志
        let native_flag = self.check_native_query_flag(&args_node);

        // 查找 SQL 字符串值
        let sql_text = self.find_sql_value_in_annotation(&args_node, annotation_name);

        if let Some((sql_text, is_text_block)) = sql_text {
            let sql_kind = match native_flag {
                NativeQueryFlag::True => SqlKind::NativeSql,
                NativeQueryFlag::False => SqlKind::Jpql,
                NativeQueryFlag::NotPresent => {
                    // 无 nativeQuery 标志时，根据注解类型判断
                    match annotation_name {
                        "NamedQuery" => SqlKind::Jpql,  // @NamedQuery 总是 JPQL
                        "SqlUpdate" | "SqlQuery" => SqlKind::NativeSql,  // JDBI 总是 native
                        "Query" => SqlKind::Jpql,  // @Query 默认 JPQL
                        "Modifying" => SqlKind::Jpql,
                        _ => SqlKind::NativeSql,
                    }
                }
            };

            // 检测参数风格
            let param_style = detect_parameter_style(&sql_text);

            // 尝试用核心 Parser 解析（仅 NativeSql）
            let parse_result = if sql_kind == SqlKind::NativeSql {
                let flat_sql = sql_text.trim().to_string();
                if !flat_sql.is_empty() {
                    let (stmts, errors) = crate::parser::Parser::parse_sql(&flat_sql);
                    Some(SqlParseResult { statements: stmts, errors })
                } else {
                    None
                }
            } else {
                None
            };

            self.extractions.push(ExtractedSql {
                sql: sql_text,
                origin: SqlOrigin {
                    method: ExtractionMethod::Annotation,
                    class_name: self.class_name.clone(),
                    method_name: self.method_name.clone(),
                    annotation_name: Some(annotation_name.to_string()),
                    api_method_name: None,
                    variable_name: None,
                    line: node.start_position().row + 1,
                    column: node.start_position().column,
                },
                sql_kind,
                parameter_style: param_style,
                is_concatenated: false, // TODO: 检测拼接
                is_text_block,
                parse_result,
            });
        }
    }

    /// 在注解参数中查找 SQL 值。
    ///
    /// 对于 @Query: 查找 "value" 键对应的字符串值
    /// 对于 @NamedQuery: 查找 "query" 键对应的字符串值
    /// 对于 @SqlUpdate/@SqlQuery: 注解参数就是直接的字符串（无键值对）
    fn find_sql_value_in_annotation(
        &self,
        args_node: Node,
        annotation_name: &str,
    ) -> Option<(String, bool)> {
        let target_key = match annotation_name {
            "NamedQuery" => "query",
            _ => "value",
        };

        let mut cursor = args_node.walk();
        for child in args_node.children(&mut cursor) {
            match child.kind() {
                "element_value_pair" => {
                    let key_node = child.child_by_field_name("key")?;
                    let key_text = self.node_text(key_node);
                    if key_text == target_key {
                        let value_node = child.child_by_field_name("value")?;
                        return self.extract_string_value(value_node);
                    }
                }
                // 直接字符串参数（如 @SqlUpdate("INSERT ...")）
                "string_literal" => {
                    return self.extract_string_value(child);
                }
                _ => {}
            }
        }

        // 对于 @Query，如果没找到 "value" 键，尝试直接取第一个字符串参数
        // 即 @Query("SELECT ...") 而不是 @Query(value = "SELECT ...")
        if annotation_name == "Query" {
            let mut cursor = args_node.walk();
            for child in args_node.children(&mut cursor) {
                if child.kind() == "string_literal" {
                    return self.extract_string_value(child);
                }
            }
        }

        None
    }

    /// 从 string_literal 或 binary_expression (+ 拼接) 中提取字符串值。
    ///
    /// 返回 (合并后的字符串, 是否是 text block)
    fn extract_string_value(&self, node: Node) -> Option<(String, bool)> {
        match node.kind() {
            "string_literal" => {
                let raw = self.node_text(node);
                let is_text_block = raw.starts_with("\"\"\"");
                let content = self.decode_java_string(&raw, is_text_block);
                Some((content, is_text_block))
            }
            "binary_expression" => {
                // "str1" + "str2" 拼接
                let parts = self.collect_concat_parts(node);
                if parts.is_empty() {
                    return None;
                }
                let combined: String = parts.into_iter()
                    .filter_map(|(s, _)| Some(s))
                    .collect();
                Some((combined, false))
            }
            _ => None,
        }
    }

    /// 递归收集 binary_expression 中通过 + 连接的字符串片段。
    ///
    /// `"SELECT " + "FROM " + tableName` → ["SELECT ", "FROM ", ???]
    /// 遇到非字符串字面量的节点时，插入 `__JAVA_VAR__` 占位符。
    fn collect_concat_parts(&self, node: Node) -> Vec<(String, bool)> {
        let op = node.child_by_field_name("operator")
            .map(|n| self.node_text(n));

        if op.as_deref() != Some("+") {
            return vec![];
        }

        let mut parts = Vec::new();

        if let Some(left) = node.child_by_field_name("left") {
            match left.kind() {
                "string_literal" => {
                    let raw = self.node_text(left);
                    let is_tb = raw.starts_with("\"\"\"");
                    parts.push((self.decode_java_string(&raw, is_tb), is_tb));
                }
                "binary_expression" => {
                    parts.extend(self.collect_concat_parts(left));
                }
                _ => {
                    // 变量引用，插入占位符
                    parts.push(("__JAVA_VAR__".to_string(), false));
                }
            }
        }

        if let Some(right) = node.child_by_field_name("right") {
            match right.kind() {
                "string_literal" => {
                    let raw = self.node_text(right);
                    let is_tb = raw.starts_with("\"\"\"");
                    parts.push((self.decode_java_string(&raw, is_tb), is_tb));
                }
                "binary_expression" => {
                    parts.extend(self.collect_concat_parts(right));
                }
                _ => {
                    parts.push(("__JAVA_VAR__".to_string(), false));
                }
            }
        }

        parts
    }

    /// 检查注解参数中是否有 nativeQuery = true。
    fn check_native_query_flag(&self, args_node: &Node) -> NativeQueryFlag {
        let mut cursor = args_node.walk();
        for child in args_node.children(&mut cursor) {
            if child.kind() == "element_value_pair" {
                if let Some(key) = child.child_by_field_name("key") {
                    if self.node_text(key) == "nativeQuery" {
                        if let Some(value) = child.child_by_field_name("value") {
                            let val_text = self.node_text(value);
                            return if val_text == "true" {
                                NativeQueryFlag::True
                            } else {
                                NativeQueryFlag::False
                            };
                        }
                    }
                }
            }
        }
        NativeQueryFlag::NotPresent
    }

    /// 解码 Java 字符串字面量。
    ///
    /// 普通字符串: `"hello"` → `hello`
    /// Text block: `"""line1\nline2"""` → `line1\nline2`（去除共有缩进）
    fn decode_java_string(&self, raw: &str, is_text_block: bool) -> String {
        if is_text_block {
            self.decode_text_block(raw)
        } else {
            self.decode_regular_string(raw)
        }
    }

    /// 解码普通 Java 字符串: 去掉首尾双引号，处理转义序列。
    fn decode_regular_string(&self, raw: &str) -> String {
        // 去掉首尾的 "
        let inner = raw.strip_prefix('"').and_then(|s| s.strip_suffix('"'));
        let inner = match inner {
            Some(s) => s,
            None => return raw.to_string(),
        };

        self.process_escape_sequences(inner)
    }

    /// 解码 Java text block: 去掉首尾的 """，去除共有缩进。
    fn decode_text_block(&self, raw: &str) -> String {
        // 去掉首尾的 """
        let inner = raw.strip_prefix("\"\"\"").and_then(|s| s.strip_suffix("\"\"\""));
        let inner = match inner {
            Some(s) => s,
            None => return raw.to_string(),
        };

        // 去除共有缩进（Java text block incidental whitespace 算法）
        let lines: Vec<&str> = inner.lines().collect();
        if lines.is_empty() {
            return String::new();
        }

        // 第一个换行后的内容有效，如果 """ 后紧跟换行
        let start = if lines.first().map(|l| l.trim().is_empty()).unwrap_or(false) {
            1
        } else {
            0
        };

        let effective_lines = &lines[start..];

        // 计算最小缩进
        let min_indent = effective_lines.iter()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.chars().take_while(|c| c == &' ' || c == &'\t').count())
            .min()
            .unwrap_or(0);

        // 去除缩进并合并
        let result: Vec<String> = effective_lines.iter()
            .map(|l| {
                if l.len() >= min_indent {
                    l[min_indent..].to_string()
                } else {
                    l.trim_end().to_string()
                }
            })
            .collect();

        let mut joined = result.join("\n");
        // 去除首尾空白
        joined = joined.trim().to_string();
        self.process_escape_sequences(&joined)
    }

    /// 处理常见 Java 转义序列。
    fn process_escape_sequences(&self, s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '\\' && i + 1 < chars.len() {
                match chars[i + 1] {
                    'n' => { result.push('\n'); i += 2; }
                    't' => { result.push('\t'); i += 2; }
                    'r' => { result.push('\r'); i += 2; }
                    '"' => { result.push('"'); i += 2; }
                    '\'' => { result.push('\''); i += 2; }
                    '\\' => { result.push('\\'); i += 2; }
                    _ => { result.push(chars[i]); i += 1; }
                }
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }
        result
    }

    /// 从 tree-sitter Node 提取文本。
    fn node_text(&self, node: Node) -> String {
        self.source[node.byte_range()].to_string()
    }
}

/// 检测 SQL 文本中的参数标记风格。
fn detect_parameter_style(sql: &str) -> ParameterStyle {
    let mut has_question = false;
    let mut has_numbered = false;
    let mut has_named_colon = false;
    let mut has_hash = false;

    let chars: Vec<char> = sql.chars().collect();
    let len = chars.len();
    let mut in_string = false;
    let mut i = 0;

    while i < len {
        if chars[i] == '\'' {
            in_string = !in_string;
            i += 1;
            continue;
        }
        if in_string {
            i += 1;
            continue;
        }

        if chars[i] == '?' && i + 1 < len && chars[i + 1].is_ascii_digit() {
            has_numbered = true;
        } else if chars[i] == '?' {
            has_question = true;
        } else if chars[i] == ':' && i + 1 < len && chars[i + 1].is_ascii_alphabetic() {
            has_named_colon = true;
        } else if chars[i] == '#' && i + 1 < len && chars[i + 1] == '{' {
            has_hash = true;
        }
        i += 1;
    }

    if has_hash {
        ParameterStyle::NamedHash
    } else if has_numbered {
        ParameterStyle::PositionalNumbered
    } else if has_named_colon {
        ParameterStyle::NamedColon
    } else if has_question {
        ParameterStyle::PositionalQuestion
    } else {
        ParameterStyle::None
    }
}
```

**Step 4: 在 `src/java/mod.rs` 中注册 tests 模块**

在 `src/java/mod.rs` 末尾添加:
```rust
#[cfg(test)]
mod tests;
```

**Step 5: 运行测试**

```bash
cargo test --features java -- java::tests 2>&1
```

预期: 8 个测试全部通过。

**Step 6: Commit**

```bash
git add src/java/
git commit -m "feat(java): P0 annotation SQL extraction — @Query, @NamedQuery, @SqlUpdate, @SqlQuery"
```

---

## Task 3: P1 — 方法调用参数 SQL 提取

**Files:**
- Modify: `src/java/extract.rs`
- Modify: `src/java/tests.rs`

**目标:** 提取以下 API 方法调用中第一个字符串参数中的 SQL:

| 方法名 | 来源框架 | SQL 类型 |
|--------|---------|---------|
| `createNativeQuery` | JPA/Hibernate | NativeSql |
| `createQuery` | JPA/Hibernate | Jpql |
| `prepareStatement` | JDBC | NativeSql |
| `prepareCall` | JDBC | NativeSql |
| `query` | Spring JdbcTemplate | NativeSql |
| `update` | Spring JdbcTemplate | NativeSql |
| `execute` | Spring JdbcTemplate | NativeSql |
| `executeQuery` | JDBC Statement | NativeSql |
| `executeUpdate` | JDBC Statement | NativeSql |

**tree-sitter-java 节点结构:**

```
(method_invocation
  object: (field_access/identifier)   ; 可选: em, jdbcTemplate 等
  name: (identifier)                  ; 方法名
  arguments: (argument_list
    (string_literal ...)))            ; 第一个字符串参数 = SQL
```

**挑战:** 方法名如 `query`/`update`/`execute` 太通用了（可能不是 SQL 相关），需要启发式过滤:
1. 检查字符串参数是否包含 SQL 关键字（SELECT, INSERT, UPDATE, DELETE, WITH, CREATE, ALTER）
2. 或者参数是 text block

**Step 1: 添加测试**

在 `src/java/tests.rs` 中追加:

```rust
// ── P1: Method Call Tests ──

#[test]
fn test_create_native_query() {
    let java = r#"
        public class UserService {
            public User findUser(String id) {
                return em.createNativeQuery("SELECT * FROM users WHERE id = ?", User.class)
                    .setParameter(1, id)
                    .getSingleResult();
            }
        }
    "#;
    let result = extract_sql_from_java(java, "UserService.java");
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(ext.sql.trim(), "SELECT * FROM users WHERE id = ?");
    assert_eq!(ext.origin.method, ExtractionMethod::MethodCall);
    assert_eq!(ext.origin.api_method_name.as_deref(), Some("createNativeQuery"));
    assert_eq!(ext.sql_kind, SqlKind::NativeSql);
}

#[test]
fn test_prepare_statement() {
    let java = r#"
        public class UserService {
            public void deleteUser(String id) throws SQLException {
                PreparedStatement ps = conn.prepareStatement("DELETE FROM users WHERE id = ?");
                ps.setString(1, id);
                ps.executeUpdate();
            }
        }
    "#;
    let result = extract_sql_from_java(java, "UserService.java");
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert!(ext.sql.contains("DELETE FROM users"));
    assert_eq!(ext.origin.api_method_name.as_deref(), Some("prepareStatement"));
}

#[test]
fn test_jdbc_template_query() {
    let java = r#"
        public class UserRepository {
            public List<User> findAll() {
                return jdbcTemplate.query("SELECT id, name FROM users",
                    (rs, rowNum) -> new User(rs.getLong("id"), rs.getString("name")));
            }
        }
    "#;
    let result = extract_sql_from_java(java, "UserRepository.java");
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert!(ext.sql.contains("SELECT id, name FROM users"));
    assert_eq!(ext.origin.api_method_name.as_deref(), Some("query"));
}

#[test]
fn test_generic_method_name_filtered() {
    // "query" 方法名太通用，如果字符串不含 SQL 关键字则不应提取
    let java = r#"
        public class MyService {
            public void doSomething() {
                query("http://example.com/api");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "MyService.java");
    // "http://example.com/api" 不含 SQL 关键字，不应被提取
    let sql_extractions: Vec<_> = result.extractions.iter()
        .filter(|e| e.origin.method == ExtractionMethod::MethodCall)
        .collect();
    assert!(sql_extractions.is_empty());
}

#[test]
fn test_create_query_jpql() {
    let java = r#"
        public class UserService {
            public List<User> findActive() {
                return em.createQuery("SELECT u FROM User u WHERE u.status = 'active'")
                    .getResultList();
            }
        }
    "#;
    let result = extract_sql_from_java(java, "UserService.java");
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(result.extractions[0].sql_kind, SqlKind::Jpql);
}
```

**Step 2: 运行测试确认新增测试失败**

```bash
cargo test --features java -- java::tests::test_create_native_query 2>&1
```

**Step 3: 实现 P1 提取逻辑**

在 `src/java/extract.rs` 中:

1. 添加方法调用白名单常量:

```rust
/// P1 方法调用白名单: 第一个参数为 SQL 的 API 方法
const SQL_METHOD_UNAMBIGUOUS: &[&str] = &[
    "createNativeQuery",
    "createQuery",
    "prepareStatement",
    "prepareCall",
    "executeQuery",
    "executeUpdate",
];

/// P1 方法调用白名单: 需要启发式检查（方法名太通用）
const SQL_METHOD_AMBIGUOUS: &[&str] = &[
    "query",
    "update",
    "execute",
];

/// SQL 关键字（用于启发式过滤）
const SQL_KEYWORDS: &[&str] = &[
    "SELECT ", "INSERT ", "UPDATE ", "DELETE ",
    "WITH ", "CREATE ", "ALTER ", "DROP ",
    "MERGE ", "TRUNCATE ", "CALL ",
];
```

2. 在 `ExtractContext::visit` 方法中添加 `method_invocation` 分支:

```rust
fn visit(&mut self, node: Node) {
    match node.kind() {
        "class_declaration" | "interface_declaration" | "enum_declaration" => {
            self.visit_type_declaration(node);
        }
        "method_declaration" => {
            self.visit_method_declaration(node);
        }
        "annotation" => {
            self.visit_annotation(node);
        }
        "method_invocation" => {
            self.visit_method_invocation(node);
        }
        _ => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                self.visit(child);
            }
        }
    }
}
```

3. 添加 `visit_method_invocation` 方法:

```rust
/// 处理方法调用 — P1 核心
fn visit_method_invocation(&mut self, node: Node) {
    let name_node = match node.child_by_field_name("name") {
        Some(n) => n,
        None => return,
    };
    let method_name = self.node_text(name_node);

    let is_sql_method = if SQL_METHOD_UNAMBIGUOUS.contains(&method_name.as_str()) {
        true
    } else if SQL_METHOD_AMBIGUOUS.contains(&method_name.as_str()) {
        false // 需要启发式检查
    } else {
        // 不是目标方法，但仍需递归（方法调用可能嵌套）
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.visit(child);
        }
        return;
    };

    let args_node = match node.child_by_field_name("arguments") {
        Some(n) if n.kind() == "argument_list" => n,
        _ => return,
    };

    // 提取第一个字符串参数
    let sql_value = self.find_first_string_arg(&args_node);

    if let Some((sql_text, is_text_block)) = sql_value {
        // 如果是模糊方法名，做启发式检查
        if !is_sql_method && !looks_like_sql(&sql_text) {
            return;
        }

        let sql_kind = match method_name.as_str() {
            "createQuery" => SqlKind::Jpql,
            _ => SqlKind::NativeSql,
        };

        let param_style = detect_parameter_style(&sql_text);

        let parse_result = if sql_kind == SqlKind::NativeSql {
            let flat_sql = sql_text.trim().to_string();
            if !flat_sql.is_empty() {
                let (stmts, errors) = crate::parser::Parser::parse_sql(&flat_sql);
                Some(SqlParseResult { statements: stmts, errors })
            } else {
                None
            }
        } else {
            None
        };

        self.extractions.push(ExtractedSql {
            sql: sql_text,
            origin: SqlOrigin {
                method: ExtractionMethod::MethodCall,
                class_name: self.class_name.clone(),
                method_name: self.method_name.clone(),
                annotation_name: None,
                api_method_name: Some(method_name),
                variable_name: None,
                line: node.start_position().row + 1,
                column: node.start_position().column,
            },
            sql_kind,
            parameter_style: param_style,
            is_concatenated: false,
            is_text_block,
            parse_result,
        });
    }

    // 继续递归（方法参数中可能嵌套其他方法调用）
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        self.visit(child);
    }
}

/// 在参数列表中找第一个字符串字面量参数。
fn find_first_string_arg(&self, args_node: &Node) -> Option<(String, bool)> {
    let mut cursor = args_node.walk();
    for child in args_node.children(&mut cursor) {
        match child.kind() {
            "string_literal" => {
                return self.extract_string_value(child);
            }
            "binary_expression" => {
                return self.extract_string_value(child);
            }
            _ => continue,
        }
    }
    None
}
```

4. 添加启发式函数:

```rust
/// 启发式判断字符串是否看起来像 SQL。
fn looks_like_sql(text: &str) -> bool {
    let upper = text.to_uppercase();
    SQL_KEYWORDS.iter().any(|kw| upper.contains(kw))
}
```

**Step 4: 运行测试**

```bash
cargo test --features java -- java::tests 2>&1
```

预期: 所有测试通过（P0 的 8 个 + P1 的 5 个 = 13 个）。

**Step 5: Commit**

```bash
git add src/java/
git commit -m "feat(java): P1 method call SQL extraction — createNativeQuery, prepareStatement, jdbcTemplate"
```

---

## Task 4: P2 — 字符串常量 SQL 提取

**Files:**
- Modify: `src/java/extract.rs`
- Modify: `src/java/tests.rs`

**目标:** 提取 `static final String` 常量中看起来像 SQL 的字符串。

**匹配模式:**
1. 变量名包含 SQL 关键字: `SQL_*`, `QUERY_*`, `INSERT_*`, `UPDATE_*`, `DELETE_*`, `SELECT_*`
2. 或者字符串内容看起来像 SQL（`looks_like_sql` 启发式）

**tree-sitter-java 节点结构:**

```
(field_declaration
  (modifiers
    (public)
    (static)
    (final))
  type: (type_identifier)    ; String
  declarator: (variable_declarator
    name: (identifier)       ; 变量名
    value: (string_literal))) ; SQL 字符串

(local_variable_declaration
  type: (type_identifier)
  declarator: (variable_declarator
    name: (identifier)
    value: (string_literal)))
```

**Step 1: 添加测试**

在 `src/java/tests.rs` 中追加:

```rust
// ── P2: Constant SQL Tests ──

#[test]
fn test_static_final_sql_constant() {
    let java = r#"
        public class UserQueries {
            private static final String SQL_FIND_BY_ID =
                "SELECT id, name, email FROM users WHERE id = ?";
            private static final String SQL_INSERT =
                "INSERT INTO users (name, email) VALUES (?, ?)";
        }
    "#;
    let result = extract_sql_from_java(java, "UserQueries.java");
    let constants: Vec<_> = result.extractions.iter()
        .filter(|e| e.origin.method == ExtractionMethod::Constant)
        .collect();
    assert_eq!(constants.len(), 2);
    assert!(constants[0].sql.contains("SELECT"));
    assert!(constants[1].sql.contains("INSERT"));
}

#[test]
fn test_sql_constant_with_concatenation() {
    let java = r#"
        public class UserQueries {
            private static final String SQL_FIND_ACTIVE =
                "SELECT * FROM users " +
                "WHERE status = 'active' " +
                "ORDER BY name";
        }
    "#;
    let result = extract_sql_from_java(java, "UserQueries.java");
    let constants: Vec<_> = result.extractions.iter()
        .filter(|e| e.origin.method == ExtractionMethod::Constant)
        .collect();
    assert_eq!(constants.len(), 1);
    assert!(constants[0].sql.contains("SELECT * FROM users"));
    assert!(constants[0].sql.contains("WHERE status = 'active'"));
    assert!(constants[0].is_concatenated);
}

#[test]
fn test_non_sql_constant_not_extracted() {
    let java = r#"
        public class Config {
            private static final String NAME = "ogsql-parser";
            private static final String VERSION = "1.0.0";
        }
    "#;
    let result = extract_sql_from_java(java, "Config.java");
    let constants: Vec<_> = result.extractions.iter()
        .filter(|e| e.origin.method == ExtractionMethod::Constant)
        .collect();
    assert!(constants.is_empty());
}

#[test]
fn test_sql_constant_name_heuristic() {
    // 变量名包含 SQL，即使内容较短也应提取
    let java = r#"
        public class Q {
            private static final String DELETE_SQL = "DELETE FROM temp";
        }
    "#;
    let result = extract_sql_from_java(java, "Q.java");
    let constants: Vec<_> = result.extractions.iter()
        .filter(|e| e.origin.method == ExtractionMethod::Constant)
        .collect();
    assert_eq!(constants.len(), 1);
}
```

**Step 2: 运行测试确认新增测试失败**

```bash
cargo test --features java -- java::tests::test_static_final_sql_constant 2>&1
```

**Step 3: 实现 P2 提取逻辑**

在 `src/java/extract.rs` 中:

1. 添加变量名启发式常量:

```rust
/// 变量名中暗示 SQL 的关键字
const SQL_NAME_KEYWORDS: &[&str] = &[
    "SQL", "QUERY", "INSERT", "UPDATE", "DELETE", "SELECT",
    "CREATE", "ALTER", "DROP", "MERGE",
];
```

2. 在 `ExtractContext::visit` 方法中添加 `field_declaration` 和 `local_variable_declaration` 分支:

```rust
fn visit(&mut self, node: Node) {
    match node.kind() {
        "class_declaration" | "interface_declaration" | "enum_declaration" => {
            self.visit_type_declaration(node);
        }
        "method_declaration" => {
            self.visit_method_declaration(node);
        }
        "annotation" => {
            self.visit_annotation(node);
        }
        "method_invocation" => {
            self.visit_method_invocation(node);
        }
        "field_declaration" => {
            self.visit_field_declaration(node);
        }
        "local_variable_declaration" => {
            self.visit_local_variable_declaration(node);
        }
        _ => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                self.visit(child);
            }
        }
    }
}
```

3. 添加字段声明和局部变量声明处理方法:

```rust
/// 处理字段声明 — P2: static final String SQL = "..."
fn visit_field_declaration(&mut self, node: Node) {
    self.check_string_declaration(node, true);

    // 继续递归
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        self.visit(child);
    }
}

/// 处理局部变量声明 — P2: String sql = "SELECT ..."
fn visit_local_variable_declaration(&mut self, node: Node) {
    self.check_string_declaration(node, false);

    // 继续递归
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        self.visit(child);
    }
}

/// 检查变量声明是否包含 SQL 字符串。
fn check_string_declaration(&mut self, node: Node, is_field: bool) {
    // 找到 variable_declarator
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "variable_declarator" {
            self.check_declarator(child, is_field);
        }
    }
}

fn check_declarator(&mut self, declarator: Node, _is_field: bool) {
    let name_node = match declarator.child_by_field_name("name") {
        Some(n) => n,
        None => return,
    };
    let value_node = match declarator.child_by_field_name("value") {
        Some(n) => n,
        None => return,
    };

    let var_name = self.node_text(name_node);

    // 提取字符串值
    let (sql_text, is_text_block) = match self.extract_string_value(value_node) {
        Some(v) => v,
        None => return,
    };

    // 判断是否是 SQL:
    // 1. 变量名包含 SQL 关键字
    // 2. 或者字符串内容看起来像 SQL
    let var_name_upper = var_name.to_uppercase();
    let name_looks_like_sql = SQL_NAME_KEYWORDS.iter()
        .any(|kw| var_name_upper.contains(kw));

    if !name_looks_like_sql && !looks_like_sql(&sql_text) {
        return;
    }

    let sql_kind = detect_sql_kind_from_content(&sql_text);
    let param_style = detect_parameter_style(&sql_text);
    let is_concatenated = value_node.kind() == "binary_expression";

    let parse_result = if sql_kind == SqlKind::NativeSql {
        let flat_sql = sql_text.trim().to_string();
        if !flat_sql.is_empty() {
            let (stmts, errors) = crate::parser::Parser::parse_sql(&flat_sql);
            Some(SqlParseResult { statements: stmts, errors })
        } else {
            None
        }
    } else {
        None
    };

    self.extractions.push(ExtractedSql {
        sql: sql_text,
        origin: SqlOrigin {
            method: ExtractionMethod::Constant,
            class_name: self.class_name.clone(),
            method_name: self.method_name.clone(),
            annotation_name: None,
            api_method_name: None,
            variable_name: Some(var_name),
            line: declarator.start_position().row + 1,
            column: declarator.start_position().column,
        },
        sql_kind,
        parameter_style: param_style,
        is_concatenated,
        is_text_block,
        parse_result,
    });
}
```

4. 添加辅助函数:

```rust
/// 根据 SQL 内容判断类型。
fn detect_sql_kind_from_content(sql: &str) -> SqlKind {
    let upper = sql.trim().to_uppercase();
    let prefix = upper.split_whitespace().next().unwrap_or("");
    match prefix {
        "SELECT" | "INSERT" | "UPDATE" | "DELETE" | "WITH" | "MERGE" => SqlKind::NativeSql,
        "CREATE" | "ALTER" | "DROP" | "TRUNCATE" => SqlKind::Ddl,
        _ => SqlKind::NativeSql, // 默认
    }
}
```

**Step 4: 运行测试**

```bash
cargo test --features java -- java::tests 2>&1
```

预期: 所有测试通过（P0: 8 + P1: 5 + P2: 4 = 17 个）。

**Step 5: Commit**

```bash
git add src/java/
git commit -m "feat(java): P2 constant SQL extraction — static final String SQL_*"
```

---

## Task 5: CLI 集成 — `parse-java` 子命令

**Files:**
- Modify: `src/bin/ogsql.rs`

**目标:** 添加 `ogsql parse-java` CLI 子命令，与 `parse-xml` 完全对称。

**Step 1: 在 Commands enum 中添加 ParseJava 变体**

在 `src/bin/ogsql.rs` 的 `Commands` enum 中，在 `#[cfg(feature = "ibatis")]` 块之后添加:

```rust
#[cfg(feature = "java")]
/// Extract SQL from Java source files / 从 Java 源文件提取 SQL
#[command(name = "parse-java")]
ParseJava,
```

**Step 2: 实现 cmd_parse_java 函数**

在文件末尾（`cmd_parse_xml` 函数之后）添加:

```rust
#[cfg(feature = "java")]
fn cmd_parse_java(cli: &Cli) {
    let input = match cli.file.as_deref() {
        Some(path) => {
            let bytes = std::fs::read(path)
                .unwrap_or_else(|e| die!("Error reading {}: {}", path, e));
            token::decode_sql_file(&bytes)
                .unwrap_or_else(|e| die!("Error decoding {}: {}", cli.file.as_deref().unwrap(), e))
                .0
        }
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)
                .unwrap_or_else(|e| die!("Error reading stdin: {}", e));
            buf
        }
    };

    let file_path = cli.file.as_deref().unwrap_or("<stdin>");
    let result = ogsql_parser::java::extract_sql_from_java(&input, file_path);

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        if !result.errors.is_empty() {
            eprintln!("{} error(s):", result.errors.len());
            for e in &result.errors {
                eprintln!("  {}", e);
            }
        }

        for ext in &result.extractions {
            let kind_tag = match ext.sql_kind {
                ogsql_parser::java::SqlKind::NativeSql => "SQL",
                ogsql_parser::java::SqlKind::Jpql => "JPQL",
                ogsql_parser::java::SqlKind::Ddl => "DDL",
            };
            let method_tag = match ext.origin.method {
                ogsql_parser::java::ExtractionMethod::Annotation => {
                    format!("@{}", ext.origin.annotation_name.as_deref().unwrap_or("?"))
                }
                ogsql_parser::java::ExtractionMethod::MethodCall => {
                    format!("{}", ext.origin.api_method_name.as_deref().unwrap_or("?"))
                }
                ogsql_parser::java::ExtractionMethod::Constant => {
                    format!("const {}", ext.origin.variable_name.as_deref().unwrap_or("?"))
                }
            };
            println!("── [{}] {} (L{}) ──", kind_tag, method_tag, ext.origin.line);
            println!("{}", ext.sql.trim());
            if ext.is_text_block {
                println!("  [text block]");
            }
            if ext.is_concatenated {
                println!("  [concatenated]");
            }
            if let Some(pr) = &ext.parse_result {
                if !pr.errors.is_empty() {
                    eprintln!("  {} parse error(s):", pr.errors.len());
                    for e in &pr.errors {
                        eprintln!("    {}", e);
                    }
                } else {
                    println!("  ✓ Parsed successfully ({} statement(s))", pr.statements.len());
                }
            }
            println!();
        }

        println!("Total: {} extraction(s) from '{}'", result.extractions.len(), file_path);
    }
}
```

**Step 3: 在 main match 中添加分支**

在 `main` 函数的 `match cli.command` 中添加:

```rust
#[cfg(feature = "java")]
Commands::ParseJava => cmd_parse_java(&cli),
```

**Step 4: 编译验证**

```bash
cargo build --features java 2>&1
```

**Step 5: 手动测试 CLI**

```bash
# 创建测试文件
cat > /tmp/TestRepo.java << 'EOF'
public interface UserRepository extends JpaRepository<User, Long> {
    @Query(value = "SELECT * FROM users WHERE status = :status", nativeQuery = true)
    List<User> findByStatus(@Param("status") int status);

    @Query("SELECT u FROM User u WHERE u.name = :name")
    User findByName(@Param("name") String name);

    private static final String SQL_FIND_ALL = "SELECT id, name FROM users ORDER BY id";
}
EOF

# 运行
cargo run --features java -- parse-java -f /tmp/TestRepo.java
cargo run --features java -- parse-java -f /tmp/TestRepo.java -j
```

预期输出:
```
── [SQL] @Query (L2) ──
SELECT * FROM users WHERE status = :status
  ✓ Parsed successfully (1 statement(s))

── [JPQL] @Query (L5) ──
SELECT u FROM User u WHERE u.name = :name

── [SQL] const SQL_FIND_ALL (L8) ──
SELECT id, name FROM users ORDER BY id
  ✓ Parsed successfully (1 statement(s))

Total: 3 extraction(s) from '/tmp/TestRepo.java'
```

**Step 6: Commit**

```bash
git add src/bin/ogsql.rs
git commit -m "feat(java): add parse-java CLI subcommand"
```

---

## Task 6: 清理 + 文档更新

**Files:**
- Modify: `README.md`

**Step 1: 更新 README.md**

在 README 的 "iBatis/MyBatis XML Support" 表格之后，添加 Java SQL 提取的对应章节:

```markdown
### Java Source Code SQL Extraction / Java 源码 SQL 提取

> Feature-gated: requires `--features java` / 功能门控：需要 `--features java`

| Component | Status | Details |
|-----------|--------|---------|
| Annotation SQL / 注解SQL | ✅ Complete | `@Query`, `@NamedQuery`, `@SqlUpdate`, `@SqlQuery` |
| Method call SQL / 方法调用SQL | ✅ Complete | `createNativeQuery`, `prepareStatement`, `jdbcTemplate.query/update` |
| Constant SQL / 常量SQL | ✅ Complete | `static final String SQL_*` with heuristic detection |
| String concatenation / 字符串拼接 | ✅ Complete | `"str1" + "str2"` → merged SQL |
| Text block support / Text block支持 | ✅ Complete | Java 15+ `""" ... """` with indentation stripping |
| JPQL vs SQL detection / JPQL区分 | ✅ Complete | `nativeQuery=true` flag + annotation type heuristics |
| CLI `parse-java` subcommand / CLI子命令 | ✅ Complete | `ogsql parse-java -f Repository.java` with JSON output |
```

在项目结构中添加:

```
│   ├── java/                # [feature: java] Java 源码 SQL 提取
│   │   ├── mod.rs           # 公共 API: extract_sql_from_java()
│   │   ├── types.rs         # ExtractedSql, SqlOrigin, JavaExtractResult
│   │   ├── error.rs         # JavaError
│   │   ├── extract.rs       # CST 遍历 + 模式匹配提取
│   │   └── tests.rs         # 单元测试
```

在 CLI Usage 中添加 `parse-java` 命令说明和示例。

在 Cargo.toml features 部分添加 `java` 的说明。

**Step 2: 运行全部测试**

```bash
cargo test --features java 2>&1
cargo test --features ibatis 2>&1
cargo test 2>&1  # 确保 default features 不 broken
```

**Step 3: Commit**

```bash
git add README.md
git commit -m "docs: add Java SQL extraction feature to README"
```

---

## 总览

| Task | 内容 | 预估行数 | 依赖 |
|------|------|---------|------|
| Task 1 | 基础设施：Cargo.toml + 模块骨架 + 类型 | ~300行 | 无 |
| Task 2 | P0 注解 SQL 提取 + 测试 | ~500行 | Task 1 |
| Task 3 | P1 方法调用 SQL 提取 + 测试 | ~200行 | Task 2 |
| Task 4 | P2 常量 SQL 提取 + 测试 | ~200行 | Task 3 |
| Task 5 | CLI 集成 | ~80行 | Task 4 |
| Task 6 | 文档更新 | ~50行 | Task 5 |

**总计:** ~1330 行新增代码，6 个 commit，17+ 个单元测试。
