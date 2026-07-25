# iBatis XML Mapper SQL 解析 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 为 ogsql-parser 新增 iBatis/MyBatis XML mapper 文件解析能力：从 XML 中提取 SQL 语句、建模动态 SQL 元素、解析 `<include>` 片段引用，最终将提取的 SQL 馈入现有 Parser 管线得到结构化 AST。

**Architecture:** 新增 `src/ibatis/` 模块（与 `analyzer/` 平级），作为独立的前置预处理层。XML → SqlNode 树 → 扁平化 SQL 字符串 → 现有 `Parser::parse_sql()` → AST。不修改任何现有核心模块代码。通过 Cargo feature gate `ibatis` 控制，依赖 `quick-xml` 做流式 XML 解析。

**Tech Stack:** Rust 2021, quick-xml (streaming/event-based), serde, thiserror

**Design Decisions (confirmed with user):**
- `#{param}` → `?`, `${expr}` → `__IBATIS_DOLLAR_expr__` 占位符
- Feature-gated: `ibatis = ["dep:quick-xml"]`
- v1 仅支持同文件 `<include>` 解析

**Anti-Slop Rules (from Metis pre-analysis):**
- ❌ 禁止 `trim_text(true)` — SQL 空白有意义
- ❌ 禁止 trait 层次结构 / `dyn SqlNode` / builder pattern / visitor pattern
- ❌ 禁止 OGNL 表达式求值
- ❌ 禁止建模 `<resultMap>` / `<cache>` / `<parameterMap>`
- ❌ 禁止 async
- ✅ 使用 flat enum + match（跟随 `src/ast/mod.rs` 模式）
- ✅ 所有公开类型 derive `Debug, Clone, Serialize, Deserialize`
- ✅ 错误收集模式（跟随 `src/parser/mod.rs`）

---

## 模块结构

```
src/ibatis/
├── mod.rs          # 公开 API + 模块导出 (~200 行)
├── types.rs        # 数据模型: SqlNode, MapperStatement 等 (~250 行)
├── error.rs        # IbatisError (~60 行)
├── parser.rs       # XML 解析: bytes → MapperFile (~500 行)
├── resolver.rs     # <include> 片段解析 (~200 行)
├── flatten.rs      # SqlNode 树 → 扁平 SQL 字符串 (~350 行)
└── tests.rs        # 单元测试 (~600 行)
```

---

## Task 1: 基础设施 — Cargo.toml + 模块骨架 + 错误类型

**Files:**
- Modify: `Cargo.toml`
- Create: `src/ibatis/mod.rs`
- Create: `src/ibatis/error.rs`
- Create: `src/ibatis/types.rs`
- Modify: `src/lib.rs`

**Step 1: 在 Cargo.toml 中添加 quick-xml 依赖和 ibatis feature**

在 `Cargo.toml` 中添加:

```toml
# 在 [dependencies] 末尾添加:
quick-xml = { version = "0.37", optional = true }

# 在 [features] 中修改:
[features]
default = ["cli"]
cli = ["dep:clap"]
ibatis = ["dep:quick-xml"]      # 新增
serve = ["cli", "dep:axum", "dep:tokio", "dep:tower-http", "dep:utoipa"]
tui = ["cli", "dep:ratatui", "dep:crossterm"]
full = ["cli", "ibatis", "serve", "tui"]  # 添加 ibatis 到 full
```

> **注意**: 使用 `0.37` 而非最新 `0.39`，因为 0.37+ 的 API 已稳定，且更可能被广泛安装。实际版本号以 `cargo search quick-xml` 最新稳定版为准。

**Step 2: 创建 `src/ibatis/error.rs` — 错误类型**

```rust
//! iBatis XML 解析错误类型。

use crate::parser::ParserError;

/// iBatis XML mapper 解析过程中可能产生的错误。
#[derive(Debug, Clone, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum IbatisError {
    /// XML 格式错误
    #[error("XML parse error at line {line}: {message}")]
    XmlError {
        line: usize,
        message: String,
    },

    /// 找不到引用的 SQL 片段
    #[error("unknown sql fragment: {refid}")]
    UnknownFragment {
        refid: String,
    },

    /// 循环引用检测
    #[error("circular include detected: {chain}")]
    CircularInclude {
        chain: Vec<String>,
    },

    /// 必需属性缺失
    #[error("missing required attribute '{attribute}' on <{element}>")]
    MissingAttribute {
        element: String,
        attribute: String,
    },

    /// mapper 文件为空或没有有效内容
    #[error("empty mapper: no statements found")]
    EmptyMapper,

    /// SQL 解析错误（由核心 Parser 产生）
    #[error("SQL parse error: {0}")]
    SqlParseError(ParserError),
}

// 不实现 From<ParserError> — 避免自动转换，保持显式错误处理
```

**Step 3: 创建 `src/ibatis/types.rs` — 数据模型**

```rust
//! iBatis/MyBatis mapper 数据模型。

/// 一个完整的 mapper XML 文件解析结果。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MapperFile {
    /// mapper 的 namespace 属性
    pub namespace: String,
    /// SQL 片段定义 (<sql id="...">)
    pub fragments: Vec<SqlFragment>,
    /// SQL 语句 (<select>/<insert>/<update>/<delete>)
    pub statements: Vec<MapperStatement>,
}

/// 一个 SQL 片段 (<sql id="...">...</sql>)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SqlFragment {
    /// 片段 ID
    pub id: String,
    /// 片段内容（已解析的 SqlNode 树）
    pub body: SqlNode,
}

/// 一个 SQL 语句 (<select>/<insert>/<update>/<delete>)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MapperStatement {
    /// 语句类型
    pub kind: StatementKind,
    /// 语句 ID
    pub id: String,
    /// parameterType 属性（可选）
    pub parameter_type: Option<String>,
    /// resultType / resultMap 属性（可选）
    pub result_type: Option<String>,
    /// 语句体（已解析的 SqlNode 树，<include> 已展开）
    pub body: SqlNode,
}

/// SQL 语句类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StatementKind {
    Select,
    Insert,
    Update,
    Delete,
}

/// iBatis 动态 SQL 节点树。
///
/// 这是 iBatis XML 中 SQL 内容的中间表示。
/// 每个节点对应一段纯文本或一个动态 SQL 元素。
/// 采用 flat enum 设计（跟随 ast/mod.rs 模式）。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SqlNode {
    /// 纯 SQL 文本片段
    Text {
        content: String,
    },

    /// #{param} PreparedStatement 参数占位符
    Parameter {
        name: String,
    },

    /// ${expr} 原始字符串替换点
    RawExpr {
        expr: String,
    },

    /// <if test="...">...</if>
    If {
        test: String,
        children: Vec<SqlNode>,
    },

    /// <choose> 包含多个 <when> 和一个可选 <otherwise>
    Choose {
        /// Vec<(Option<test>, children)> — when 的 test 是 Some，otherwise 是 None
        branches: Vec<(Option<String>, Vec<SqlNode>)>,
    },

    /// <where>...</where> — 自动处理前缀 AND/OR
    Where {
        children: Vec<SqlNode>,
    },

    /// <set>...</set> — 自动处理尾部逗号
    Set {
        children: Vec<SqlNode>,
    },

    /// <trim prefix="..." suffix="..." prefixOverrides="..." suffixOverrides="...">
    Trim {
        prefix: Option<String>,
        suffix: Option<String>,
        prefix_overrides: Option<String>,
        suffix_overrides: Option<String>,
        children: Vec<SqlNode>,
    },

    /// <foreach collection="..." item="..." index="..." open="..." separator="..." close="...">
    ForEach {
        collection: String,
        item: String,
        index: Option<String>,
        open: Option<String>,
        separator: Option<String>,
        close: Option<String>,
        children: Vec<SqlNode>,
    },

    /// <bind name="..." value="..."/>
    Bind {
        name: String,
        value: String,
    },
}

/// 扁平化提取结果：一条 SQL 语句的可能变体。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FlattenedStatement {
    /// 所属 mapper statement 的 ID
    pub statement_id: String,
    /// 语句类型
    pub kind: StatementKind,
    /// 提取的扁平 SQL 字符串（#{→?}, ${→占位符）
    pub sql: String,
    /// 是否包含动态 SQL 元素（意味着可能存在其他变体）
    pub has_dynamic_elements: bool,
}

/// 完整解析结果：mapper 文件 → 解析后的语句列表 + SQL 解析结果。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParsedMapper {
    /// mapper namespace
    pub namespace: String,
    /// 每个语句的解析结果
    pub statements: Vec<ParsedStatement>,
    /// 收集的非致命错误
    pub errors: Vec<crate::ibatis::error::IbatisError>,
}

/// 单个语句的完整解析结果。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParsedStatement {
    /// 语句 ID
    pub id: String,
    /// 语句类型
    pub kind: StatementKind,
    /// 扁平化后的 SQL 文本
    pub flat_sql: String,
    /// 是否含动态元素
    pub has_dynamic_elements: bool,
    /// 核心 parser 解析结果 (StatementInfo + errors)
    pub parse_result: Option<(Vec<crate::ast::StatementInfo>, Vec<crate::parser::ParserError>)>,
}
```

**Step 4: 创建 `src/ibatis/mod.rs` — 模块骨架**

```rust
//! iBatis/MyBatis XML mapper 文件解析支持。
//!
//! 从 XML mapper 文件中提取 SQL 语句，建模动态 SQL 元素，
//! 并将提取的 SQL 馈入核心 Parser 得到结构化 AST。
//!
//! # Example
//!
//! ```ignore
//! use ogsql_parser::ibatis::parse_mapper_bytes;
//!
//! let xml = br#"<?xml version="1.0" encoding="UTF-8"?>
//! <mapper namespace="com.example.UserMapper">
//!   <select id="findUser" resultType="User">
//!     SELECT * FROM users WHERE id = #{id}
//!   </select>
//! </mapper>"#;
//!
//! let result = parse_mapper_bytes(xml);
//! ```

pub mod error;
pub mod flatten;
pub mod parser;
pub mod resolver;
pub mod types;

pub use error::IbatisError;
pub use types::{
    FlattenedStatement, MapperFile, MapperStatement, ParsedMapper, ParsedStatement, SqlFragment,
    SqlNode, StatementKind,
};

/// 从 XML 字节解析 mapper 文件。
///
/// 完整管线：XML → MapperFile (SqlNode 树) → <include> 解析 → 扁平化 SQL → Parser。
pub fn parse_mapper_bytes(xml: &[u8]) -> ParsedMapper {
    let mut errors = Vec::new();

    // Phase 1: XML 解析 → MapperFile
    let mapper_file = match parser::parse_xml(xml) {
        Ok(m) => m,
        Err(e) => {
            return ParsedMapper {
                namespace: String::new(),
                statements: Vec::new(),
                errors: vec![e],
            };
        }
    };

    // Phase 2: <include> 解析
    let mapper_file = match resolver::resolve_includes(&mapper_file) {
        Ok(m) => m,
        Err(e) => {
            errors.push(e);
            return ParsedMapper {
                namespace: mapper_file.namespace,
                statements: Vec::new(),
                errors,
            };
        }
    };

    // Phase 3+4: 扁平化 SQL 并解析
    let mut statements = Vec::new();
    for stmt in &mapper_file.statements {
        let flat_sql = flatten::flatten_sql(&stmt.body);

        let has_dynamic = has_dynamic_elements(&stmt.body);
        let parse_result = if !flat_sql.trim().is_empty() {
            Some(crate::parser::Parser::parse_sql(&flat_sql))
        } else {
            None
        };

        statements.push(ParsedStatement {
            id: stmt.id.clone(),
            kind: stmt.kind,
            flat_sql,
            has_dynamic: has_dynamic,
            parse_result,
        });
    }

    if statements.is_empty() && errors.is_empty() {
        errors.push(IbatisError::EmptyMapper);
    }

    ParsedMapper {
        namespace: mapper_file.namespace,
        statements,
        errors,
    }
}

/// 检查 SqlNode 树是否包含动态 SQL 元素。
fn has_dynamic_elements(node: &SqlNode) -> bool {
    match node {
        SqlNode::Text { .. } | SqlNode::Parameter { .. } | SqlNode::RawExpr { .. } => false,
        SqlNode::If { .. }
        | SqlNode::Choose { .. }
        | SqlNode::Where { .. }
        | SqlNode::Set { .. }
        | SqlNode::Trim { .. }
        | SqlNode::ForEach { .. }
        | SqlNode::Bind { .. } => true,
    }
}

#[cfg(test)]
mod tests;
```

**Step 5: 在 `src/lib.rs` 中添加 ibatis 模块（feature-gated）**

在 `src/lib.rs` 末尾添加:

```rust
#[cfg(feature = "ibatis")]
pub mod ibatis;
```

**Step 6: 创建 `src/ibatis/parser.rs` — 占位骨架**

```rust
//! iBatis XML mapper 文件解析器。
//!
//! 使用 quick-xml 的流式 event-based API 解析 XML。

use crate::ibatis::error::IbatisError;
use crate::ibatis::types::{MapperFile, SqlFragment, SqlNode, MapperStatement, StatementKind};

/// 解析 iBatis XML mapper 文件。
///
/// 输入: XML 文件的原始字节（quick-xml 会自动处理编码声明）。
/// 输出: 解析后的 MapperFile 结构。
pub fn parse_xml(xml: &[u8]) -> Result<MapperFile, IbatisError> {
    // Task 2 中实现
    todo!("Task 2")
}
```

**Step 7: 创建 `src/ibatis/resolver.rs` — 占位骨架**

```rust
//! <include> 片段解析。
//!
//! 解析 <sql id="..."> 片段引用，内联展开到引用点。

use crate::ibatis::error::IbatisError;
use crate::ibatis::types::MapperFile;

/// 解析 mapper 文件中所有 <include refid="..."/> 引用。
///
/// v1 仅支持同文件内的片段引用。
pub fn resolve_includes(mapper: &MapperFile) -> Result<MapperFile, IbatisError> {
    // Task 3 中实现
    todo!("Task 3")
}
```

**Step 8: 创建 `src/ibatis/flatten.rs` — 占位骨架**

```rust
//! SqlNode 树 → 扁平 SQL 字符串。
//!
//! 遍历 SqlNode 树，将动态 SQL 元素转换为具体 SQL 文本。
//! - #{param} → ?
//! - ${expr} → __IBATIS_DOLLAR_expr__

use crate::ibatis::types::SqlNode;

/// 默认占位符前缀
const DOLLAR_PREFIX: &str = "__IBATIS_DOLLAR_";
const DOLLAR_SUFFIX: &str = "__";

/// 将 SqlNode 树扁平化为 SQL 字符串。
///
/// 策略:
/// - 纯文本: 原样保留
/// - #{param}: 替换为 ?
/// - ${expr}: 替换为 __IBATIS_DOLLAR_expr__
/// - <if>/<choose>/<when>/<otherwise>: 取"最完整"分支（所有条件满足）
/// - <where>: 生成 WHERE + 去除前缀 AND/OR
/// - <set>: 生成 SET + 去除尾部逗号
/// - <trim>: 应用 prefix/suffix + overrides
/// - <foreach>: 用 open/close/separator 模板展开一次迭代
/// - <bind>: 忽略（无法求值 OGNL）
pub fn flatten_sql(node: &SqlNode) -> String {
    // Task 4 中实现
    todo!("Task 4")
}
```

**Step 9: 创建 `src/ibatis/tests.rs` — 初始占位**

```rust
//! iBatis XML 解析测试。

// Task 2 开始添加测试
```

**Step 10: 验证编译通过**

Run: `cargo check --features ibatis`
Expected: 编译成功（有 todo!() 会导致 panic，但类型检查通过）

**Step 11: Commit**

```bash
git add Cargo.toml src/lib.rs src/ibatis/
git commit -m "feat: scaffold ibatis module with feature gate"
```

---

## Task 2: XML 结构解析 — bytes → MapperFile

**目标:** 使用 quick-xml 解析 XML，提取 `<mapper>`, `<sql>`, `<select>/<insert>/<update>/<delete>` 的属性和文本内容。动态元素暂作为原始文本处理（不建模为 SqlNode 子节点），Phase 2+ 再处理。

**Files:**
- Modify: `src/ibatis/parser.rs`
- Modify: `src/ibatis/tests.rs`

**关键设计点:**
- 使用 `quick_xml::Reader`（不用 `NsReader`，MyBatis XML 不使用 XML namespace）
- **绝不**调用 `trim_text(true)` — SQL 空白有意义
- `<resultMap>`, `<cache>`, `<cache-ref>`, `<parameterMap>` 跳过（读到匹配的 `</>` 即可）
- `<selectKey>` 作为 `<insert>` 内的特殊子元素，其文本归入 insert 的 body
- 文本中的 `#{}` 和 `${}` 先作为原始文本保留，Task 4 再处理

**Step 1: 写 `parser.rs` 的完整实现**

```rust
//! iBatis XML mapper 文件解析器。

use std::collections::HashMap;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::ibatis::error::IbatisError;
use crate::ibatis::types::{MapperFile, MapperStatement, SqlFragment, SqlNode, StatementKind};

/// SQL 语句元素名称。
const SQL_STATEMENT_TAGS: &[&str] = &["select", "insert", "update", "delete"];

/// 需要"吃掉"内部内容的非 SQL 元素（不建模）。
const SKIP_CONTENT_TAGS: &[&str] = &[
    "resultMap",
    "cache",
    "cache-ref",
    "parameterMap",
    "resultMap",
];

/// SQL 片段定义元素。
const SQL_FRAGMENT_TAG: &str = "sql";

/// 动态 SQL 元素名称。
const DYNAMIC_TAGS: &[&str] = &[
    "if",
    "choose",
    "when",
    "otherwise",
    "where",
    "set",
    "trim",
    "foreach",
    "bind",
    "include",
];

/// 判断元素名是否为动态 SQL 元素。
fn is_dynamic_tag(tag: &[u8]) -> bool {
    DYNAMIC_TAGS.iter().any(|t| tag.as_ref().eq_ignore_ascii_case(t.as_bytes()))
}

/// 判断元素名是否为需要跳过内容的元素。
fn is_skip_tag(tag: &[u8]) -> bool {
    SKIP_CONTENT_TAGS.iter().any(|t| tag.as_ref().eq_ignore_ascii_case(t.as_bytes()))
}

/// 判断元素名是否为 SQL 语句类型。
fn statement_kind(tag: &[u8]) -> Option<StatementKind> {
    if tag.as_ref().eq_ignore_ascii_case(b"select") {
        Some(StatementKind::Select)
    } else if tag.as_ref().eq_ignore_ascii_case(b"insert") {
        Some(StatementKind::Insert)
    } else if tag.as_ref().eq_ignore_ascii_case(b"update") {
        Some(StatementKind::Update)
    } else if tag.as_ref().eq_ignore_ascii_case(b"delete") {
        Some(StatementKind::Delete)
    } else {
        None
    }
}

/// 解析 iBatis XML mapper 文件。
pub fn parse_xml(xml: &[u8]) -> Result<MapperFile, IbatisError> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(false); // 关键: 保留 SQL 空白

    let mut buf = Vec::new();
    let mut namespace = String::new();
    let mut fragments: Vec<SqlFragment> = Vec::new();
    let mut statements: Vec<MapperStatement> = Vec::new();

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag = e.local_name();
                if tag.as_ref().eq_ignore_ascii_case(b"mapper") {
                    namespace = get_attr(&e, "namespace").unwrap_or_default();
                } else if tag.as_ref().eq_ignore_ascii_case(SQL_FRAGMENT_TAG.as_bytes()) {
                    let id = get_attr(&e, "id").unwrap_or_default();
                    let body_text = read_text_content(&mut reader, b"sql");
                    fragments.push(SqlFragment {
                        id,
                        body: SqlNode::Text { content: body_text },
                    });
                } else if let Some(kind) = statement_kind(tag.as_ref()) {
                    let id = get_attr(&e, "id").unwrap_or_default();
                    let parameter_type = get_attr(&e, "parameterType");
                    let result_type = get_attr(&e, "resultType").or_else(|| get_attr(&e, "resultMap"));
                    let body_text = read_statement_content(&mut reader, tag.as_ref());
                    statements.push(MapperStatement {
                        kind,
                        id,
                        parameter_type,
                        result_type,
                        body: SqlNode::Text { content: body_text },
                    });
                } else if is_skip_tag(tag.as_ref()) {
                    // 跳过非 SQL 元素内容
                    skip_content(&mut reader, tag.as_ref());
                }
                // 其他未知开始标签 — 忽略
            }
            Ok(Event::Empty(e)) => {
                let tag = e.local_name();
                if tag.as_ref().eq_ignore_ascii_case(SQL_FRAGMENT_TAG.as_bytes()) {
                    let id = get_attr(&e, "id").unwrap_or_default();
                    fragments.push(SqlFragment {
                        id,
                        body: SqlNode::Text { content: String::new() },
                    });
                }
                // 其他自闭合标签（如 <include refid="..."/>）在 Task 3 处理
            }
            Ok(Event::Eof) => break,
            Ok(_) => {} // Text, End, CData 等在顶层忽略
            Err(e) => {
                let line = reader.error_position();
                return Err(IbatisError::XmlError {
                    line: line,
                    message: e.to_string(),
                });
            }
        }
    }

    Ok(MapperFile {
        namespace,
        fragments,
        statements,
    })
}

/// 读取元素的纯文本内容，直到匹配的结束标签。
///
/// 保留内部空白。不解析动态子元素（Phase 1）。
fn read_text_content(reader: &mut Reader<&[u8]>, end_tag: &[u8]) -> String {
    let mut content = String::new();
    let mut depth: u32 = 1;
    let mut buf = Vec::new();

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => {
                // quick-xml 已自动处理 XML 实体反转义 (&lt; → <)
                content.push_str(&e.unescape().unwrap_or_default());
            }
            Ok(Event::CData(e)) => {
                // CDATA 内容原样保留
                content.push_str(&String::from_utf8_lossy(e.as_ref()));
            }
            Ok(Event::Start(e)) => {
                depth += 1;
                // 内部子元素开始 — 暂时重建 XML 标签（Phase 3 会替换为 SqlNode 解析）
                let tag_name = String::from_utf8_lossy(e.local_name().as_ref());
                let attrs = format_attributes(&e);
                content.push_str(&format!("<{}{}>", tag_name, attrs));
            }
            Ok(Event::End(e)) => {
                depth -= 1;
                if depth == 0 && e.local_name().as_ref().eq_ignore_ascii_case(end_tag) {
                    break;
                }
                let tag_name = String::from_utf8_lossy(e.local_name().as_ref());
                content.push_str(&format!("</{}>", tag_name));
            }
            Ok(Event::Empty(e)) => {
                // 自闭合标签
                let tag_name = String::from_utf8_lossy(e.local_name().as_ref());
                let attrs = format_attributes(&e);
                content.push_str(&format!("<{}{}/>", tag_name, attrs));
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
    }
    content
}

/// 读取 SQL 语句内容。
///
/// 与 read_text_content 类似，但对 <selectKey>（insert 的特殊子元素）
/// 做特殊处理：其内容也归入 insert body。
fn read_statement_content(reader: &mut Reader<&[u8]>, end_tag: &[u8]) -> String {
    // Phase 1: 与 read_text_content 逻辑相同
    // <selectKey> 的内容作为文本的一部分保留
    read_text_content(reader, end_tag)
}

/// 跳过元素的所有内容（用于 resultMap, cache 等非 SQL 元素）。
fn skip_content(reader: &mut Reader<&[u8]>, end_tag: &[u8]) {
    let mut depth: u32 = 1;
    let mut buf = Vec::new();

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(_)) => depth += 1,
            Ok(Event::End(e)) => {
                depth -= 1;
                if depth == 0 && e.local_name().as_ref().eq_ignore_ascii_case(end_tag) {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
    }
}

/// 从元素中获取属性值。
fn get_attr<E: quick_xml::events::attributes::Attrs + ?Sized>(
    element: &quick_xml::events::BytesStart<'_>,
    name: &str,
) -> Option<String> {
    element.attributes().find_map(|a| {
        a.ok().and_then(|a| {
            if a.key.as_ref().eq_ignore_ascii_case(name.as_bytes()) {
                Some(String::from_utf8_lossy(&a.value).into_owned())
            } else {
                None
            }
        })
    })
}

/// 格式化元素属性为 XML 属性字符串。
fn format_attributes(element: &quick_xml::events::BytesStart<'_>) -> String {
    let mut result = String::new();
    for attr in element.attributes().flatten() {
        let key = String::from_utf8_lossy(&attr.key);
        let value = String::from_utf8_lossy(&attr.value);
        result.push_str(&format!(" {}=\"{}\"", key, value));
    }
    result
}
```

**Step 2: 写测试**

在 `src/ibatis/tests.rs` 中:

```rust
use super::*;

fn parse_simple_mapper() -> MapperFile {
    let xml = br#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE mapper PUBLIC "-//mybatis.org//DTD Mapper 3.0//EN" "http://mybatis.org/dtd/mybatis-3-mapper.dtd">
<mapper namespace="com.example.UserMapper">
    <sql id="baseColumns">id, name, email</sql>

    <select id="findById" parameterType="int" resultType="User">
        SELECT <include refid="baseColumns"/>
        FROM users
        WHERE id = #{id}
    </select>

    <insert id="insertUser" parameterType="User">
        INSERT INTO users (name, email) VALUES (#{name}, #{email})
    </insert>

    <update id="updateName">
        UPDATE users SET name = #{name} WHERE id = #{id}
    </update>

    <delete id="deleteById">
        DELETE FROM users WHERE id = #{id}
    </delete>
</mapper>"#;

    parser::parse_xml(xml).unwrap()
}

#[test]
fn test_parse_mapper_namespace() {
    let mapper = parse_simple_mapper();
    assert_eq!(mapper.namespace, "com.example.UserMapper");
}

#[test]
fn test_parse_fragments() {
    let mapper = parse_simple_mapper();
    assert_eq!(mapper.fragments.len(), 1);
    assert_eq!(mapper.fragments[0].id, "baseColumns");
    // Phase 1: body 是原始文本
    if let SqlNode::Text { content } = &mapper.fragments[0].body {
        assert!(content.contains("id, name, email"));
    } else {
        panic!("expected Text node");
    }
}

#[test]
fn test_parse_statements_count() {
    let mapper = parse_simple_mapper();
    assert_eq!(mapper.statements.len(), 4);
}

#[test]
fn test_parse_select_statement() {
    let mapper = parse_simple_mapper();
    let select = mapper.statements.iter().find(|s| s.id == "findById").unwrap();
    assert_eq!(select.kind, StatementKind::Select);
    assert_eq!(select.parameter_type.as_deref(), Some("int"));
    assert_eq!(select.result_type.as_deref(), Some("User"));
}

#[test]
fn test_parse_insert_statement() {
    let mapper = parse_simple_mapper();
    let insert = mapper.statements.iter().find(|s| s.id == "insertUser").unwrap();
    assert_eq!(insert.kind, StatementKind::Insert);
    assert_eq!(insert.parameter_type.as_deref(), Some("User"));
}

#[test]
fn test_parse_update_statement() {
    let mapper = parse_simple_mapper();
    let update = mapper.statements.iter().find(|s| s.id == "updateName").unwrap();
    assert_eq!(update.kind, StatementKind::Update);
}

#[test]
fn test_parse_delete_statement() {
    let mapper = parse_simple_mapper();
    let delete = mapper.statements.iter().find(|s| s.id == "deleteById").unwrap();
    assert_eq!(delete.kind, StatementKind::Delete);
}

#[test]
fn test_skip_result_map() {
    let xml = br#"<mapper namespace="test">
        <resultMap id="userMap" type="User">
            <id column="id" property="id"/>
            <result column="name" property="name"/>
        </resultMap>
        <select id="findAll">SELECT * FROM users</select>
    </mapper>"#;
    let mapper = parser::parse_xml(xml).unwrap();
    assert_eq!(mapper.statements.len(), 1);
}

#[test]
fn test_empty_mapper() {
    let xml = br#"<mapper namespace="empty"></mapper>"#;
    let mapper = parser::parse_xml(xml).unwrap();
    assert_eq!(mapper.namespace, "empty");
    assert!(mapper.statements.is_empty());
    assert!(mapper.fragments.is_empty());
}

#[test]
fn test_invalid_xml() {
    let xml = br#"this is not xml at all"#;
    let result = parser::parse_xml(xml);
    assert!(result.is_err());
    match result.unwrap_err() {
        IbatisError::XmlError { message, .. } => {
            assert!(!message.is_empty());
        }
        e => panic!("expected XmlError, got {:?}", e),
    }
}

#[test]
fn test_preserves_whitespace_in_sql() {
    let xml = br#"<mapper namespace="test">
    <select id="ws">
        SELECT   id,    name
        FROM     users
        WHERE    id = 1
    </select>
</mapper>"#;
    let mapper = parser::parse_xml(xml).unwrap();
    let stmt = &mapper.statements[0];
    if let SqlNode::Text { content } = &stmt.body {
        // 空白必须保留（不是 trimmed）
        assert!(content.contains("  id,    name"));
    } else {
        panic!("expected Text node");
    }
}
```

**Step 3: 运行测试**

Run: `cargo test --features ibatis --lib ibatis::tests`
Expected: 所有测试通过

**Step 4: Commit**

```bash
git add src/ibatis/
git commit -m "feat(ibatis): implement XML structure parsing — bytes to MapperFile"
```

---

## Task 3: `<include>` 片段解析

**目标:** 解析 `<include refid="..."/>` 引用，将 `<sql>` 片段内容内联展开到引用位置。仅支持同文件片段引用。检测循环引用。

**Files:**
- Modify: `src/ibatis/parser.rs` — 将 `<include refid="..."/>` 从文本重建改为识别
- Modify: `src/ibatis/resolver.rs` — 完整实现
- Modify: `src/ibatis/tests.rs` — 新增测试

**关键设计点:**
- Phase 1 中，body 是 `SqlNode::Text { content: "原始 XML 文本" }`
- resolver 需要在文本中找到 `<include refid="..."/>` 并替换为对应片段的文本
- 检测循环引用：维护一个 visited-set
- `<property>` 子元素暂不支持（v2）

**Step 1: 实现 `resolver.rs`**

```rust
//! <include> 片段解析。

use std::collections::{HashMap, HashSet};

use crate::ibatis::error::IbatisError;
use crate::ibatis::types::{MapperFile, SqlFragment, SqlNode};

/// 解析 mapper 文件中所有 <include refid="..."/> 引用。
///
/// 在 Phase 1 中，statement 和 fragment 的 body 都是 SqlNode::Text。
/// 解析逻辑是在文本中找到 `<include refid="xxx"/>` 并替换为对应片段的文本内容。
///
/// 仅支持同文件片段引用（refid 不含 `.`）。
pub fn resolve_includes(mapper: &MapperFile) -> Result<MapperFile, IbatisError> {
    // 构建片段查找表
    let fragment_map: HashMap<&str, &SqlNode> = mapper
        .fragments
        .iter()
        .map(|f| (f.id.as_str(), &f.body))
        .collect();

    // 解析每个 statement 的 body
    let mut resolved_statements = mapper.statements.clone();
    for stmt in &mut resolved_statements {
        let visited = HashSet::new();
        stmt.body = resolve_node(&stmt.body, &fragment_map, visited)?;
    }

    // 片段本身也可能引用其他片段
    let mut resolved_fragments = mapper.fragments.clone();
    for frag in &mut resolved_fragments {
        let visited = HashSet::new();
        frag.body = resolve_node(&frag.body, &fragment_map, visited)?;
    }

    Ok(MapperFile {
        namespace: mapper.namespace.clone(),
        fragments: resolved_fragments,
        statements: resolved_statements,
    })
}

/// 递归解析 SqlNode 中的 <include> 引用。
fn resolve_node(
    node: &SqlNode,
    fragments: &HashMap<&str, &SqlNode>,
    mut visited: HashSet<String>,
) -> Result<SqlNode, IbatisError> {
    match node {
        SqlNode::Text { content } => {
            // 在文本中查找并替换 <include refid="..."/>
            let resolved = resolve_includes_in_text(content, fragments, &mut visited)?;
            Ok(SqlNode::Text { content: resolved })
        }
        // 其他节点类型在 Phase 3（动态 SQL 树解析）后需要递归处理
        // Phase 1 中只有 Text 节点
        other => Ok(other.clone()),
    }
}

/// 在文本字符串中查找所有 `<include refid="xxx"/>` 并替换。
fn resolve_includes_in_text(
    text: &str,
    fragments: &HashMap<&str, &SqlNode>,
    visited: &mut HashSet<String>,
) -> Result<String, IbatisError> {
    let mut result = String::with_capacity(text.len());
    let mut pos = 0;

    while pos < text.len() {
        // 查找下一个 <include
        if let Some(start) = text[pos..].find("<include") {
            let abs_start = pos + start;
            result.push_str(&text[pos..abs_start]);

            // 找到 refid 属性值
            let rest = &text[abs_start..];
            if let Some(refid) = extract_refid(rest) {
                // 检测循环引用
                if visited.contains(&refid) {
                    let mut chain: Vec<String> = visited.iter().cloned().collect();
                    chain.push(refid.clone());
                    return Err(IbatisError::CircularInclude { chain });
                }

                // 查找片段
                let fragment_body = fragments
                    .get(refid.as_str())
                    .ok_or_else(|| IbatisError::UnknownFragment {
                        refid: refid.clone(),
                    })?;

                // 递归展开（片段可能引用其他片段）
                visited.insert(refid.clone());
                let expanded_text = get_text_content(fragment_body);
                let expanded = resolve_includes_in_text(&expanded_text, fragments, visited)?;
                visited.remove(&refid);

                result.push_str(&expanded);

                // 跳过整个 <include ... /> 标签
                if let Some(end) = rest.find("/>").or_else(|| rest.find(">")) {
                    pos = abs_start + end + 2;
                    if rest[..end + 2].contains("/>") {
                        pos = abs_start + end + 2;
                    } else {
                        pos = abs_start + end + 1;
                    }
                } else {
                    pos = text.len();
                }
            } else {
                // 无 refid 的 include — 原样保留
                result.push_str("<include");
                pos = abs_start + 8;
            }
        } else {
            result.push_str(&text[pos..]);
            break;
        }
    }

    Ok(result)
}

/// 从 `<include refid="xxx".../>` 标签中提取 refid 值。
fn extract_refid(tag_text: &str) -> Option<String> {
    // 查找 refid="..." 或 refid='...'
    let patterns = ["refid=\"", "refid='"];
    for pattern in patterns {
        if let Some(start) = tag_text.find(pattern) {
            let value_start = start + pattern.len();
            let quote_char = pattern.chars().last().unwrap();
            if let Some(end) = tag_text[value_start..].find(quote_char) {
                return Some(tag_text[value_start..value_start + end].to_string());
            }
        }
    }
    None
}

/// 从 SqlNode 获取纯文本内容。
fn get_text_content(node: &SqlNode) -> String {
    match node {
        SqlNode::Text { content } => content.clone(),
        _ => String::new(),
    }
}
```

**Step 2: 写测试**

在 `src/ibatis/tests.rs` 中追加:

```rust
// ── Include Resolution Tests ──

#[test]
fn test_include_resolution_basic() {
    let xml = br#"<mapper namespace="test">
        <sql id="cols">id, name, email</sql>
        <select id="findAll">SELECT <include refid="cols"/> FROM users</select>
    </mapper>"#;
    let mapper = parser::parse_xml(xml).unwrap();
    let resolved = resolver::resolve_includes(&mapper).unwrap();
    let stmt = resolved.statements.iter().find(|s| s.id == "findAll").unwrap();
    if let SqlNode::Text { content } = &stmt.body {
        assert!(content.contains("id, name, email"), "include should be expanded, got: {}", content);
        assert!(content.contains("SELECT"), "should contain SELECT");
        assert!(content.contains("FROM users"), "should contain FROM users");
    } else {
        panic!("expected Text node");
    }
}

#[test]
fn test_include_resolution_chained() {
    // 片段 A 引用片段 B
    let xml = br#"<mapper namespace="test">
        <sql id="table">users</sql>
        <sql id="cols">id, name FROM <include refid="table"/></sql>
        <select id="find">SELECT <include refid="cols"/></select>
    </mapper>"#;
    let mapper = parser::parse_xml(xml).unwrap();
    let resolved = resolver::resolve_includes(&mapper).unwrap();
    let stmt = &resolved.statements[0];
    if let SqlNode::Text { content } = &stmt.body {
        assert!(content.contains("users"), "chained include should expand, got: {}", content);
    } else {
        panic!("expected Text node");
    }
}

#[test]
fn test_include_unknown_fragment() {
    let xml = br#"<mapper namespace="test">
        <select id="find">SELECT <include refid="nonexistent"/> FROM users</select>
    </mapper>"#;
    let mapper = parser::parse_xml(xml).unwrap();
    let result = resolver::resolve_includes(&mapper);
    assert!(result.is_err());
    match result.unwrap_err() {
        IbatisError::UnknownFragment { refid } => {
            assert_eq!(refid, "nonexistent");
        }
        e => panic!("expected UnknownFragment, got {:?}", e),
    }
}

#[test]
fn test_include_circular_detection() {
    let xml = br#"<mapper namespace="test">
        <sql id="a"><include refid="b"/></sql>
        <sql id="b"><include refid="a"/></sql>
        <select id="find">SELECT <include refid="a"/></select>
    </mapper>"#;
    let mapper = parser::parse_xml(xml).unwrap();
    let result = resolver::resolve_includes(&mapper);
    assert!(result.is_err());
    match result.unwrap_err() {
        IbatisError::CircularInclude { chain } => {
            assert!(!chain.is_empty(), "chain should not be empty");
        }
        e => panic!("expected CircularInclude, got {:?}", e),
    }
}

#[test]
fn test_no_includes() {
    let xml = br#"<mapper namespace="test">
        <select id="find">SELECT 1</select>
    </mapper>"#;
    let mapper = parser::parse_xml(xml).unwrap();
    let resolved = resolver::resolve_includes(&mapper).unwrap();
    if let SqlNode::Text { content } = &resolved.statements[0].body {
        assert_eq!(content.trim(), "SELECT 1");
    }
}
```

**Step 3: 运行测试**

Run: `cargo test --features ibatis --lib ibatis::tests`
Expected: 所有测试通过

**Step 4: Commit**

```bash
git add src/ibatis/
git commit -m "feat(ibatis): implement <include> fragment resolution with cycle detection"
```

---

## Task 4: SQL 扁平化 — SqlNode::Text → 纯 SQL 字符串

**目标:** 将 `SqlNode::Text` 中的 `#{param}` 替换为 `?`，`${expr}` 替换为 `__IBATIS_DOLLAR_expr__`，生成可传给 `Parser::parse_sql()` 的纯 SQL 字符串。

**Files:**
- Modify: `src/ibatis/flatten.rs`
- Modify: `src/ibatis/tests.rs`

**关键设计点:**
- Phase 1 中只有 `SqlNode::Text` 节点，动态元素还是原始 XML 文本
- 对文本做参数替换：`#{...}` → `?`, `${...}` → `__IBATIS_DOLLAR_...__`
- 需要正确处理字符串字面量中的 `#{}`（不应替换）
- `\${` 应该被视为转义，不替换（MyBatis 的 GenericTokenParser 行为）

**Step 1: 实现 `flatten.rs`**

```rust
//! SqlNode 树 → 扁平 SQL 字符串。

use crate::ibatis::types::SqlNode;

/// 默认占位符前缀
const DOLLAR_PREFIX: &str = "__IBATIS_DOLLAR_";
const DOLLAR_SUFFIX: &str = "__";

/// 将 SqlNode 树扁平化为 SQL 字符串。
pub fn flatten_sql(node: &SqlNode) -> String {
    match node {
        SqlNode::Text { content } => replace_params(content),
        _ => String::new(), // 其他类型在 Phase 3 后实现
    }
}

/// 替换 SQL 文本中的参数占位符。
///
/// - `#{param}` → `?`
/// - `${expr}` → `__IBATIS_DOLLAR_expr__`
/// - `\${...}` → 原样保留 `${...}`（反斜杠转义）
/// - 位于 SQL 字符串字面量 `'...'` 内的 `#{}` 和 `${}` 不替换
fn replace_params(sql: &str) -> String {
    let mut result = String::with_capacity(sql.len());
    let chars: Vec<char> = sql.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut in_string = false;

    while i < len {
        let c = chars[i];

        // 跟踪 SQL 字符串字面量状态
        if c == '\'' && !in_string {
            in_string = true;
            result.push(c);
            i += 1;
            continue;
        }
        if c == '\'' && in_string {
            // 检查是否为 escaped quote ''
            if i + 1 < len && chars[i + 1] == '\'' {
                result.push_str("''");
                i += 2;
                continue;
            }
            in_string = false;
            result.push(c);
            i += 1;
            continue;
        }

        if in_string {
            result.push(c);
            i += 1;
            continue;
        }

        // 处理 #{
        if c == '#' && i + 1 < len && chars[i + 1] == '{' {
            if let Some(end) = find_closing_brace(&chars, i + 2) {
                // 跳过可能存在的类型注解 #{name,javaType=...}
                let param_content: String = chars[i + 2..end].iter().collect();
                let param_name = param_content.split(',').next().unwrap_or("").trim();
                let _ = param_name; // 未来可用于报告
                result.push('?');
                i = end + 1;
                continue;
            }
        }

        // 处理 ${
        if c == '$' && i + 1 < len && chars[i + 1] == '{' {
            if let Some(end) = find_closing_brace(&chars, i + 2) {
                let expr: String = chars[i + 2..end].iter().collect();
                result.push_str(DOLLAR_PREFIX);
                result.push_str(&expr);
                result.push_str(DOLLAR_SUFFIX);
                i = end + 1;
                continue;
            }
        }

        // 处理转义 \${
        if c == '\\' && i + 2 < len && chars[i + 1] == '$' && chars[i + 2] == '{' {
            result.push_str("${");
            i += 3;
            continue;
        }

        result.push(c);
        i += 1;
    }

    result
}

/// 从位置 start 开始查找匹配的 `}`，考虑嵌套 `{}`。
fn find_closing_brace(chars: &[char], start: usize) -> Option<usize> {
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

#[cfg(test)]
mod param_tests {
    use super::*;

    #[test]
    fn test_hash_param() {
        assert_eq!(replace_params("WHERE id = #{id}"), "WHERE id = ?");
    }

    #[test]
    fn test_dollar_param() {
        assert_eq!(replace_params("ORDER BY ${col}"), format!("ORDER BY {}col{}", DOLLAR_PREFIX, DOLLAR_SUFFIX));
    }

    #[test]
    fn test_mixed_params() {
        assert_eq!(
            replace_params("WHERE id = #{id} AND name = #{name}"),
            "WHERE id = ? AND name = ?"
        );
    }

    #[test]
    fn test_param_in_string_literal_not_replaced() {
        assert_eq!(
            replace_params("WHERE name = '#{not_a_param}'"),
            "WHERE name = '#{not_a_param}'"
        );
    }

    #[test]
    fn test_dollar_in_string_literal_not_replaced() {
        assert_eq!(
            replace_params("WHERE name = '${not_a_param}'"),
            "WHERE name = '${not_a_param}'"
        );
    }

    #[test]
    fn test_escaped_dollar() {
        assert_eq!(
            replace_params("WHERE text = '\\${literal}'"),
            "WHERE text = '${literal}'"
        );
    }

    #[test]
    fn test_param_with_type_annotation() {
        assert_eq!(
            replace_params("#{price,javaType=double,jdbcType=NUMERIC}"),
            "?"
        );
    }

    #[test]
    fn test_nested_braces() {
        assert_eq!(
            replace_params("#{item}"),
            "?"
        );
    }

    #[test]
    fn test_no_params() {
        assert_eq!(replace_params("SELECT 1"), "SELECT 1");
    }
}
```

**Step 2: 运行测试**

Run: `cargo test --features ibatis --lib ibatis::flatten::param_tests`
Expected: 全部通过

**Step 3: Commit**

```bash
git add src/ibatis/
git commit -m "feat(ibatis): implement parameter replacement #{→?}, ${→placeholder"
```

---

## Task 5: 端到端管线集成 — parse_mapper_bytes

**目标:** 将 `parse_mapper_bytes()` 中的 TODO 替换为真实实现，走通完整管线：XML → MapperFile → include 解析 → 扁平化 → Parser::parse_sql() → AST。

**Files:**
- Modify: `src/ibatis/mod.rs` — 将 todo!() 替换为真实实现（已在 Task 1 Step 4 中写好，只需确保 flatten_sql 已实现）
- Modify: `src/ibatis/tests.rs` — 端到端测试

**Step 1: 确认 `src/ibatis/mod.rs` 中 `parse_mapper_bytes` 的实现**

确认 `parse_mapper_bytes` 函数调用 `parser::parse_xml` → `resolver::resolve_includes` → `flatten::flatten_sql` → `Parser::parse_sql` 的完整链路（Task 1 Step 4 中已写好）。

**Step 2: 写端到端测试**

在 `src/ibatis/tests.rs` 中追加:

```rust
// ── End-to-End Pipeline Tests ──

#[test]
fn test_e2e_simple_select() {
    let xml = br#"<mapper namespace="com.example.UserMapper">
        <select id="findById">SELECT id, name FROM users WHERE id = #{id}</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert_eq!(result.namespace, "com.example.UserMapper");
    assert_eq!(result.statements.len(), 1);
    assert!(result.errors.is_empty());

    let stmt = &result.statements[0];
    assert_eq!(stmt.id, "findById");
    assert_eq!(stmt.kind, StatementKind::Select);
    assert!(stmt.flat_sql.contains("SELECT"));
    assert!(stmt.flat_sql.contains("?"));

    // 验证核心 parser 成功解析
    if let Some((infos, errors)) = &stmt.parse_result {
        assert!(errors.is_empty(), "parser errors: {:?}", errors);
        assert_eq!(infos.len(), 1);
    } else {
        panic!("expected parse result");
    }
}

#[test]
fn test_e2e_insert() {
    let xml = br#"<mapper namespace="test">
        <insert id="insertUser">INSERT INTO users (name) VALUES (#{name})</insert>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    let stmt = &result.statements[0];
    assert_eq!(stmt.kind, StatementKind::Insert);
    assert!(stmt.flat_sql.contains("INSERT INTO"));
}

#[test]
fn test_e2e_with_fragment() {
    let xml = br#"<mapper namespace="test">
        <sql id="cols">id, name</sql>
        <select id="findAll">SELECT <include refid="cols"/> FROM users</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.errors.is_empty());
    let stmt = &result.statements[0];
    // include 应该被展开
    assert!(stmt.flat_sql.contains("id, name"), "got: {}", stmt.flat_sql);
    assert!(stmt.flat_sql.contains("FROM users"));
}

#[test]
fn test_e2e_dollar_param_placeholder() {
    let xml = br#"<mapper namespace="test">
        <select id="dynamicOrder">SELECT * FROM users ORDER BY ${column}</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    let stmt = &result.statements[0];
    assert!(stmt.flat_sql.contains("__IBATIS_DOLLAR_column__"));
}

#[test]
fn test_e2e_empty_mapper() {
    let xml = br#"<mapper namespace="empty"></mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.statements.is_empty());
    // 应该有一个 EmptyMapper 错误
    assert!(result.errors.iter().any(|e| matches!(e, IbatisError::EmptyMapper)));
}

#[test]
fn test_e2e_multiple_statements() {
    let xml = br#"<mapper namespace="test">
        <select id="find">SELECT * FROM users WHERE id = #{id}</select>
        <insert id="add">INSERT INTO users (name) VALUES (#{name})</insert>
        <update id="update">UPDATE users SET name = #{name} WHERE id = #{id}</update>
        <delete id="remove">DELETE FROM users WHERE id = #{id}</delete>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert_eq!(result.statements.len(), 4);
    assert_eq!(result.statements[0].kind, StatementKind::Select);
    assert_eq!(result.statements[1].kind, StatementKind::Insert);
    assert_eq!(result.statements[2].kind, StatementKind::Update);
    assert_eq!(result.statements[3].kind, StatementKind::Delete);
}
```

**Step 3: 运行测试**

Run: `cargo test --features ibatis --lib ibatis::tests`
Expected: 全部通过

**Step 4: Commit**

```bash
git add src/ibatis/
git commit -m "feat(ibatis): end-to-end pipeline — XML to AST"
```

---

## Task 6: 动态 SQL 树解析 — XML 动态元素 → SqlNode 子节点

**目标:** 在 XML 解析阶段，将 `<if>`, `<choose>/<when>/<otherwise>`, `<where>`, `<set>`, `<trim>`, `<foreach>`, `<bind>` 等动态元素解析为 `SqlNode` 的对应变体，而不再是原始 XML 文本。

**Files:**
- Modify: `src/ibatis/parser.rs` — 重写 `read_text_content` 为 `read_node_tree`，递归解析动态元素
- Modify: `src/ibatis/resolver.rs` — 适配新的 SqlNode 树结构
- Modify: `src/ibatis/flatten.rs` — 实现所有 SqlNode 变体的扁平化逻辑
- Modify: `src/ibatis/tests.rs` — 动态 SQL 测试

**关键设计点:**
- 用 `read_node_tree()` 替换 `read_text_content()`
- 遇到动态元素标签时递归解析子节点
- 纯文本和 CDATA 归入 `SqlNode::Text`
- `#{...}` 和 `${...}` 在文本中识别为独立的 `SqlNode::Parameter` 和 `SqlNode::RawExpr`
- `<include refid="..."/>` 解析为特殊的中间节点，由 resolver 处理

**Step 1: 在 `parser.rs` 中实现 `read_node_tree`**

在 `parser.rs` 中新增核心函数:

```rust
/// 递归解析元素内容为 SqlNode 子节点列表。
fn read_node_tree(reader: &mut Reader<&[u8]>, end_tag: &[u8]) -> Vec<SqlNode> {
    let mut nodes = Vec::new();
    let mut text_buf = String::new();
    let mut buf = Vec::new();

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap_or_default();
                text_buf.push_str(&text);
            }
            Ok(Event::CData(e)) => {
                text_buf.push_str(&String::from_utf8_lossy(e.as_ref()));
            }
            Ok(Event::Start(e)) => {
                // 先 flush 积累的文本
                flush_text_nodes(&mut text_buf, &mut nodes);

                let tag = e.local_name();
                if let Some(node) = parse_dynamic_element(reader, tag.as_ref(), &e) {
                    nodes.push(node);
                } else if tag.as_ref().eq_ignore_ascii_case(b"include") {
                    // <include> 在这里作为 Start 不应出现（应该是 Empty）
                    // 但以防万一
                } else {
                    // 未知元素 — 重建 XML 并作为文本
                    let tag_name = String::from_utf8_lossy(tag.as_ref()).to_string();
                    let attrs = format_attributes(&e);
                    let children = read_node_tree(reader, tag.as_ref());
                    let mut content = format!("<{}{}>", tag_name, attrs);
                    for child in &children {
                        content.push_str(&node_to_raw_text(child));
                    }
                    content.push_str(&format!("</{}>", tag_name));
                    text_buf.push_str(&content);
                }
            }
            Ok(Event::Empty(e)) => {
                flush_text_nodes(&mut text_buf, &mut nodes);

                let tag = e.local_name();
                if tag.as_ref().eq_ignore_ascii_case(b"include") {
                    if let Some(refid) = get_attr(&e, "refid") {
                        nodes.push(make_include_node(refid));
                    }
                } else if tag.as_ref().eq_ignore_ascii_case(b"bind") {
                    let name = get_attr(&e, "name").unwrap_or_default();
                    let value = get_attr(&e, "value").unwrap_or_default();
                    nodes.push(SqlNode::Bind { name, value });
                } else {
                    // 未知自闭合标签 — 作为文本
                    let tag_name = String::from_utf8_lossy(tag.as_ref()).to_string();
                    let attrs = format_attributes(&e);
                    text_buf.push_str(&format!("<{}{}/>", tag_name, attrs));
                }
            }
            Ok(Event::End(e)) => {
                if e.local_name().as_ref().eq_ignore_ascii_case(end_tag) {
                    flush_text_nodes(&mut text_buf, &mut nodes);
                    break;
                }
            }
            Ok(Event::Eof) => {
                flush_text_nodes(&mut text_buf, &mut nodes);
                break;
            }
            _ => {}
        }
    }

    nodes
}

/// 将积累的文本拆分为 Text / Parameter / RawExpr 节点。
fn flush_text_nodes(text_buf: &mut String, nodes: &mut Vec<SqlNode>) {
    if text_buf.is_empty() {
        return;
    }
    let text = std::mem::take(text_buf);
    // 拆分 #{...} 和 ${...} 为独立节点
    let parsed = parse_text_to_nodes(&text);
    nodes.extend(parsed);
}

/// 将文本拆分为混合节点列表: Text / Parameter / RawExpr。
fn parse_text_to_nodes(text: &str) -> Vec<SqlNode> {
    let mut nodes = Vec::new();
    let mut current_text = String::new();
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // 检测 #{
        if chars[i] == '#' && i + 1 < len && chars[i + 1] == '{' {
            if let Some(end) = find_closing_brace(&chars, i + 2) {
                // flush 纯文本
                if !current_text.is_empty() {
                    nodes.push(SqlNode::Text {
                        content: std::mem::take(&mut current_text),
                    });
                }
                let param: String = chars[i + 2..end].iter().collect();
                let name = param.split(',').next().unwrap_or("").trim().to_string();
                nodes.push(SqlNode::Parameter { name });
                i = end + 1;
                continue;
            }
        }

        // 检测 ${
        if chars[i] == '$' && i + 1 < len && chars[i + 1] == '{' {
            if let Some(end) = find_closing_brace(&chars, i + 2) {
                if !current_text.is_empty() {
                    nodes.push(SqlNode::Text {
                        content: std::mem::take(&mut current_text),
                    });
                }
                let expr: String = chars[i + 2..end].iter().collect();
                nodes.push(SqlNode::RawExpr { expr });
                i = end + 1;
                continue;
            }
        }

        current_text.push(chars[i]);
        i += 1;
    }

    if !current_text.is_empty() {
        nodes.push(SqlNode::Text {
            content: current_text,
        });
    }

    nodes
}

/// 查找匹配的 `}`，考虑嵌套。
fn find_closing_brace(chars: &[char], start: usize) -> Option<usize> {
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

/// 解析动态 SQL 元素。
fn parse_dynamic_element(
    reader: &mut Reader<&[u8]>,
    tag: &[u8],
    element: &quick_xml::events::BytesStart<'_>,
) -> Option<SqlNode> {
    if tag.eq_ignore_ascii_case(b"if") {
        let test = get_attr(element, "test").unwrap_or_default();
        let children = read_node_tree(reader, b"if");
        Some(SqlNode::If { test, children })
    } else if tag.eq_ignore_ascii_case(b"where") {
        let children = read_node_tree(reader, b"where");
        Some(SqlNode::Where { children })
    } else if tag.eq_ignore_ascii_case(b"set") {
        let children = read_node_tree(reader, b"set");
        Some(SqlNode::Set { children })
    } else if tag.eq_ignore_ascii_case(b"trim") {
        let prefix = get_attr(element, "prefix");
        let suffix = get_attr(element, "suffix");
        let prefix_overrides = get_attr(element, "prefixOverrides");
        let suffix_overrides = get_attr(element, "suffixOverrides");
        let children = read_node_tree(reader, b"trim");
        Some(SqlNode::Trim {
            prefix,
            suffix,
            prefix_overrides,
            suffix_overrides,
            children,
        })
    } else if tag.eq_ignore_ascii_case(b"foreach") {
        let collection = get_attr(element, "collection").unwrap_or_default();
        let item = get_attr(element, "item").unwrap_or("item").to_string();
        let index = get_attr(element, "index");
        let open = get_attr(element, "open");
        let separator = get_attr(element, "separator");
        let close = get_attr(element, "close");
        let children = read_node_tree(reader, b"foreach");
        Some(SqlNode::ForEach {
            collection,
            item,
            index,
            open,
            separator,
            close,
            children,
        })
    } else if tag.eq_ignore_ascii_case(b"choose") {
        let children = read_node_tree(reader, b"choose");
        let branches = parse_choose_branches(children);
        Some(SqlNode::Choose { branches })
    } else if tag.eq_ignore_ascii_case(b"when") {
        // <when> 应该只在 <choose> 内部，不应单独遇到
        // 如果单独遇到，当作 <if> 处理
        let test = get_attr(element, "test").unwrap_or_default();
        let children = read_node_tree(reader, b"when");
        Some(SqlNode::If { test, children })
    } else if tag.eq_ignore_ascii_case(b"otherwise") {
        let children = read_node_tree(reader, b"otherwise");
        Some(SqlNode::If {
            test: String::new(), // always true
            children,
        })
    } else {
        None
    }
}

/// 从 <choose> 的子节点中提取 when/otherwise 分支。
fn parse_choose_branches(children: Vec<SqlNode>) -> Vec<(Option<String>, Vec<SqlNode>)> {
    let mut branches = Vec::new();
    for child in children {
        match child {
            SqlNode::If { test, children } => {
                if test.is_empty() {
                    // otherwise — test 为空表示无条件的默认分支
                    branches.push((None, children));
                } else {
                    branches.push((Some(test), children));
                }
            }
            _ => {
                // 纯文本节点 — 附加到最后一个分支（如果有）
                if let Some(last) = branches.last_mut() {
                    last.1.push(child);
                }
            }
        }
    }
    branches
}

/// 创建一个 <include> 的中间表示节点。
/// 在 include 解析之前使用，解析后会被替换。
fn make_include_node(refid: String) -> SqlNode {
    // 使用 Text 节点保存 refid 信息，resolver 会识别并替换
    SqlNode::Text {
        content: format!("<include refid=\"{}\"/>", refid),
    }
}

/// 将 SqlNode 转换回原始 XML 文本（用于未知元素的 fallback）。
fn node_to_raw_text(node: &SqlNode) -> String {
    match node {
        SqlNode::Text { content } => content.clone(),
        SqlNode::Parameter { name } => format!("#{{{}}}", name),
        SqlNode::RawExpr { expr } => format!("${{{}}}", expr),
        SqlNode::If { test, children } => {
            let inner: String = children.iter().map(node_to_raw_text).collect();
            format!("<if test=\"{}\">{}</if>", test, inner)
        }
        SqlNode::Where { children } => {
            let inner: String = children.iter().map(node_to_raw_text).collect();
            format!("<where>{}</where>", inner)
        }
        SqlNode::Set { children } => {
            let inner: String = children.iter().map(node_to_raw_text).collect();
            format!("<set>{}</set>", inner)
        }
        SqlNode::Trim { prefix, suffix, prefix_overrides, suffix_overrides, children } => {
            let inner: String = children.iter().map(node_to_raw_text).collect();
            let mut attrs = String::new();
            if let Some(p) = prefix { attrs.push_str(&format!(" prefix=\"{}\"", p)); }
            if let Some(s) = suffix { attrs.push_str(&format!(" suffix=\"{}\"", s)); }
            if let Some(po) = prefix_overrides { attrs.push_str(&format!(" prefixOverrides=\"{}\"", po)); }
            if let Some(so) = suffix_overrides { attrs.push_str(&format!(" suffixOverrides=\"{}\"", so)); }
            format!("<trim{}>{}</trim>", attrs, inner)
        }
        SqlNode::ForEach { collection, item, index, open, separator, close, children } => {
            let inner: String = children.iter().map(node_to_raw_text).collect();
            let mut attrs = format!(" collection=\"{}\" item=\"{}\"", collection, item);
            if let Some(i) = index { attrs.push_str(&format!(" index=\"{}\"", i)); }
            if let Some(o) = open { attrs.push_str(&format!(" open=\"{}\"", o)); }
            if let Some(s) = separator { attrs.push_str(&format!(" separator=\"{}\"", s)); }
            if let Some(c) = close { attrs.push_str(&format!(" close=\"{}\"", c)); }
            format!("<foreach{}>{}</foreach>", attrs, inner)
        }
        SqlNode::Choose { branches } => {
            let mut inner = String::from("<choose>");
            for (test, children) in branches {
                let branch_text: String = children.iter().map(node_to_raw_text).collect();
                if let Some(t) = test {
                    inner.push_str(&format!("<when test=\"{}\">{}</when>", t, branch_text));
                } else {
                    inner.push_str(&format!("<otherwise>{}</otherwise>", branch_text));
                }
            }
            inner.push_str("</choose>");
            inner
        }
        SqlNode::Bind { name, value } => format!("<bind name=\"{}\" value=\"{}\"/>", name, value),
    }
}
```

然后修改 `parse_xml` 中的 SQL 语句和片段解析，使用 `read_node_tree` 替换 `read_text_content`:

```rust
// 在 parse_xml() 中，SQL 语句解析处修改为:
} else if let Some(kind) = statement_kind(tag.as_ref()) {
    let id = get_attr(&e, "id").unwrap_or_default();
    let parameter_type = get_attr(&e, "parameterType");
    let result_type = get_attr(&e, "resultType").or_else(|| get_attr(&e, "resultMap"));
    let children = read_node_tree(&mut reader, tag.as_ref());
    // 将子节点列表合并为单个 SqlNode
    let body = merge_children(children);
    statements.push(MapperStatement {
        kind,
        id,
        parameter_type,
        result_type,
        body,
    });
}

// 同样修改 fragment 解析:
} else if tag.as_ref().eq_ignore_ascii_case(SQL_FRAGMENT_TAG.as_bytes()) {
    let id = get_attr(&e, "id").unwrap_or_default();
    let children = read_node_tree(&mut reader, b"sql");
    let body = merge_children(children);
    fragments.push(SqlFragment { id, body });
}
```

新增辅助函数:

```rust
/// 将多个子节点合并为单个 SqlNode。
/// 0 个 → Text 空; 1 个 → 直接返回; 多个 → 不合并，保留为顶层 Vec（后续扁平化时处理）
fn merge_children(children: Vec<SqlNode>) -> SqlNode {
    match children.len() {
        0 => SqlNode::Text { content: String::new() },
        1 => children.into_iter().next().unwrap(),
        _ => {
            // 多个子节点时，用一个包装节点
            // 实际上我们可以添加一个 Sequence 变体，或者直接在 body 中存储 Vec<SqlNode>
            // 为保持简单，我们将多个节点拼接为文本（包含 #{} 和 ${} 标记）
            let mut content = String::new();
            for child in &children {
                content.push_str(&node_to_raw_text(child));
            }
            SqlNode::Text { content }
        }
    }
}
```

> **注意**: `merge_children` 的多节点情况是个妥协。更优雅的做法是在 `MapperStatement` 中将 `body: SqlNode` 改为 `body: Vec<SqlNode>`。但这会波及太多类型定义。可以在后续优化中修改。

**Step 2: 更新 `resolver.rs` 以处理新的 SqlNode 结构**

```rust
//! <include> 片段解析。

use std::collections::{HashMap, HashSet};

use crate::ibatis::error::IbatisError;
use crate::ibatis::types::{MapperFile, SqlNode};

pub fn resolve_includes(mapper: &MapperFile) -> Result<MapperFile, IbatisError> {
    let fragment_map: HashMap<&str, &SqlNode> = mapper
        .fragments
        .iter()
        .map(|f| (f.id.as_str(), &f.body))
        .collect();

    let mut resolved_statements = mapper.statements.clone();
    for stmt in &mut resolved_statements {
        let visited = HashSet::new();
        stmt.body = resolve_node(&stmt.body, &fragment_map, visited)?;
    }

    let mut resolved_fragments = mapper.fragments.clone();
    for frag in &mut resolved_fragments {
        let visited = HashSet::new();
        frag.body = resolve_node(&frag.body, &fragment_map, visited)?;
    }

    Ok(MapperFile {
        namespace: mapper.namespace.clone(),
        fragments: resolved_fragments,
        statements: resolved_statements,
    })
}

fn resolve_node(
    node: &SqlNode,
    fragments: &HashMap<&str, &SqlNode>,
    mut visited: HashSet<String>,
) -> Result<SqlNode, IbatisError> {
    match node {
        SqlNode::Text { content } => {
            // 查找 <include refid="..."/>
            if content.contains("<include") {
                let resolved = resolve_includes_in_text(content, fragments, &mut visited)?;
                Ok(SqlNode::Text { content: resolved })
            } else {
                Ok(node.clone())
            }
        }
        SqlNode::If { test, children } => {
            let resolved_children = resolve_children(children, fragments, &mut visited)?;
            Ok(SqlNode::If {
                test: test.clone(),
                children: resolved_children,
            })
        }
        SqlNode::Choose { branches } => {
            let mut resolved_branches = Vec::new();
            for (test, children) in branches {
                let resolved_children = resolve_children(children, fragments, &mut visited)?;
                resolved_branches.push((test.clone(), resolved_children));
            }
            Ok(SqlNode::Choose {
                branches: resolved_branches,
            })
        }
        SqlNode::Where { children } => {
            let resolved_children = resolve_children(children, fragments, &mut visited)?;
            Ok(SqlNode::Where {
                children: resolved_children,
            })
        }
        SqlNode::Set { children } => {
            let resolved_children = resolve_children(children, fragments, &mut visited)?;
            Ok(SqlNode::Set {
                children: resolved_children,
            })
        }
        SqlNode::Trim { prefix, suffix, prefix_overrides, suffix_overrides, children } => {
            let resolved_children = resolve_children(children, fragments, &mut visited)?;
            Ok(SqlNode::Trim {
                prefix: prefix.clone(),
                suffix: suffix.clone(),
                prefix_overrides: prefix_overrides.clone(),
                suffix_overrides: suffix_overrides.clone(),
                children: resolved_children,
            })
        }
        SqlNode::ForEach { collection, item, index, open, separator, close, children } => {
            let resolved_children = resolve_children(children, fragments, &mut visited)?;
            Ok(SqlNode::ForEach {
                collection: collection.clone(),
                item: item.clone(),
                index: index.clone(),
                open: open.clone(),
                separator: separator.clone(),
                close: close.clone(),
                children: resolved_children,
            })
        }
        // Parameter, RawExpr, Bind 无需处理
        other => Ok(other.clone()),
    }
}

fn resolve_children(
    children: &[SqlNode],
    fragments: &HashMap<&str, &SqlNode>,
    visited: &mut HashSet<String>,
) -> Result<Vec<SqlNode>, IbatisError> {
    children
        .iter()
        .map(|c| resolve_node(c, fragments, visited.clone()))
        .collect()
}

fn resolve_includes_in_text(
    text: &str,
    fragments: &HashMap<&str, &SqlNode>,
    visited: &mut HashSet<String>,
) -> Result<String, IbatisError> {
    let mut result = String::with_capacity(text.len());
    let mut pos = 0;

    while pos < text.len() {
        if let Some(start) = text[pos..].find("<include") {
            let abs_start = pos + start;
            result.push_str(&text[pos..abs_start]);

            let rest = &text[abs_start..];
            if let Some(refid) = extract_refid(rest) {
                if visited.contains(&refid) {
                    let mut chain: Vec<String> = visited.iter().cloned().collect();
                    chain.push(refid.clone());
                    return Err(IbatisError::CircularInclude { chain });
                }

                let fragment_body = fragments
                    .get(refid.as_str())
                    .ok_or_else(|| IbatisError::UnknownFragment {
                        refid: refid.clone(),
                    })?;

                visited.insert(refid.clone());
                let expanded_text = node_to_flat_text(fragment_body);
                let expanded = resolve_includes_in_text(&expanded_text, fragments, visited)?;
                visited.remove(&refid);

                result.push_str(&expanded);

                // 跳过 <include ... /> 标签
                let tag_end = if let Some(e) = rest.find("/>") {
                    abs_start + e + 2
                } else if let Some(e) = rest.find(">") {
                    abs_start + e + 1
                } else {
                    text.len()
                };
                pos = tag_end;
            } else {
                result.push_str("<include");
                pos = abs_start + 8;
            }
        } else {
            result.push_str(&text[pos..]);
            break;
        }
    }

    Ok(result)
}

fn extract_refid(tag_text: &str) -> Option<String> {
    let patterns = ["refid=\"", "refid='"];
    for pattern in patterns {
        if let Some(start) = tag_text.find(pattern) {
            let value_start = start + pattern.len();
            let quote_char = pattern.chars().last().unwrap();
            if let Some(end) = tag_text[value_start..].find(quote_char) {
                return Some(tag_text[value_start..value_start + end].to_string());
            }
        }
    }
    None
}

/// 将 SqlNode 递归展平为纯文本（含 #{}/${} 标记）。
fn node_to_flat_text(node: &SqlNode) -> String {
    match node {
        SqlNode::Text { content } => content.clone(),
        SqlNode::Parameter { name } => format!("#{{{}}}", name),
        SqlNode::RawExpr { expr } => format!("${{{}}}", expr),
        SqlNode::If { children, .. } => children.iter().map(node_to_flat_text).collect(),
        SqlNode::Choose { branches } => {
            branches
                .iter()
                .flat_map(|(_, children)| children.iter().map(node_to_flat_text))
                .collect()
        }
        SqlNode::Where { children } => children.iter().map(node_to_flat_text).collect(),
        SqlNode::Set { children } => children.iter().map(node_to_flat_text).collect(),
        SqlNode::Trim { children, .. } => children.iter().map(node_to_flat_text).collect(),
        SqlNode::ForEach { children, .. } => children.iter().map(node_to_flat_text).collect(),
        SqlNode::Bind { .. } => String::new(),
    }
}
```

**Step 3: 更新 `flatten.rs` — 处理所有 SqlNode 变体**

```rust
//! SqlNode 树 → 扁平 SQL 字符串。

use crate::ibatis::types::SqlNode;

const DOLLAR_PREFIX: &str = "__IBATIS_DOLLAR_";
const DOLLAR_SUFFIX: &str = "__";

pub fn flatten_sql(node: &SqlNode) -> String {
    match node {
        SqlNode::Text { content } => replace_params(content),
        SqlNode::Parameter { .. } => "?".to_string(),
        SqlNode::RawExpr { expr } => format!("{}{}{}", DOLLAR_PREFIX, expr, DOLLAR_SUFFIX),
        SqlNode::If { children, .. } => {
            // "最完整"策略: 取所有分支的内容
            flatten_children(children)
        }
        SqlNode::Choose { branches } => {
            // 取第一个 when 分支（最可能的路径）
            if let Some((_, first_branch)) = branches.first() {
                flatten_children(first_branch)
            } else {
                String::new()
            }
        }
        SqlNode::Where { children } => {
            let content = flatten_children(children);
            apply_trim(&content, Some("WHERE"), None, Some(&["AND ", "OR ", "AND\n", "OR\n"]), None)
        }
        SqlNode::Set { children } => {
            let content = flatten_children(children);
            apply_trim(&content, Some("SET"), None, None, Some(","))
        }
        SqlNode::Trim { prefix, suffix, prefix_overrides, suffix_overrides, children } => {
            let content = flatten_children(children);
            apply_trim(&content, prefix.as_deref(), suffix.as_deref(), prefix_overrides.as_deref().map(|s| s as &str), suffix_overrides.as_deref())
        }
        SqlNode::ForEach { open, separator, close, children, .. } => {
            let content = flatten_children(children);
            let sep = separator.as_deref().unwrap_or("");
            let open_str = open.as_deref().unwrap_or("");
            let close_str = close.as_deref().unwrap_or("");
            // 单次迭代展开
            format!("{}{}{}", open_str, content, close_str)
        }
        SqlNode::Bind { .. } => String::new(),
    }
}

fn flatten_children(children: &[SqlNode]) -> String {
    children.iter().map(flatten_sql).collect()
}

/// 应用 trim 逻辑：添加前缀/后缀，移除前缀/后缀覆盖。
fn apply_trim(
    content: &str,
    prefix: Option<&str>,
    suffix: Option<&str>,
    prefix_overrides: Option<&str>,
    suffix_overrides: Option<&str>,
) -> String {
    let mut result = content.trim().to_string();

    // 移除前缀覆盖（大小写不敏感）
    if let Some(overrides) = prefix_overrides {
        let parts: Vec<&str> = overrides.split('|').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        for part in &parts {
            while result.len() >= part.len()
                && result[..part.len()].eq_ignore_ascii_case(part)
            {
                result = result[part.len()..].to_string();
            }
            // 也检查去除前导空白后的
            let trimmed = result.trim_start();
            if trimmed.len() >= part.len()
                && trimmed[..part.len()].eq_ignore_ascii_case(part)
            {
                result = trimmed[part.len()..].to_string();
            }
        }
    }

    // 移除后缀覆盖
    if let Some(overrides) = suffix_overrides {
        let parts: Vec<&str> = overrides.split('|').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        for part in &parts {
            while result.len() >= part.len()
                && result[result.len() - part.len()..].eq_ignore_ascii_case(part)
            {
                result.truncate(result.len() - part.len());
            }
        }
    }

    result = result.trim().to_string();

    // 添加前缀和后缀
    let mut final_result = String::new();
    if let Some(p) = prefix {
        if !result.is_empty() {
            final_result.push_str(p);
            final_result.push(' ');
        }
    }
    final_result.push_str(&result);
    if let Some(s) = suffix {
        if !result.is_empty() {
            final_result.push(' ');
            final_result.push_str(s);
        }
    }

    final_result
}

/// 替换 SQL 文本中的参数占位符。
fn replace_params(sql: &str) -> String {
    let mut result = String::with_capacity(sql.len());
    let chars: Vec<char> = sql.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut in_string = false;

    while i < len {
        let c = chars[i];

        if c == '\'' && !in_string {
            in_string = true;
            result.push(c);
            i += 1;
            continue;
        }
        if c == '\'' && in_string {
            if i + 1 < len && chars[i + 1] == '\'' {
                result.push_str("''");
                i += 2;
                continue;
            }
            in_string = false;
            result.push(c);
            i += 1;
            continue;
        }

        if in_string {
            result.push(c);
            i += 1;
            continue;
        }

        if c == '#' && i + 1 < len && chars[i + 1] == '{' {
            if let Some(end) = find_closing_brace(&chars, i + 2) {
                result.push('?');
                i = end + 1;
                continue;
            }
        }

        if c == '$' && i + 1 < len && chars[i + 1] == '{' {
            if let Some(end) = find_closing_brace(&chars, i + 2) {
                let expr: String = chars[i + 2..end].iter().collect();
                result.push_str(DOLLAR_PREFIX);
                result.push_str(&expr);
                result.push_str(DOLLAR_SUFFIX);
                i = end + 1;
                continue;
            }
        }

        result.push(c);
        i += 1;
    }

    result
}

fn find_closing_brace(chars: &[char], start: usize) -> Option<usize> {
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
```

**Step 4: 写动态 SQL 测试**

在 `src/ibatis/tests.rs` 中追加:

```rust
// ── Dynamic SQL Tests ──

#[test]
fn test_dynamic_if() {
    let xml = br#"<mapper namespace="test">
        <select id="findUser">
            SELECT * FROM users
            <where>
                <if test="name != null">AND name = #{name}</if>
                <if test="age != null">AND age = #{age}</if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    let stmt = &result.statements[0];
    assert!(stmt.flat_sql.contains("SELECT * FROM users"));
    assert!(stmt.flat_sql.contains("WHERE"), "should have WHERE, got: {}", stmt.flat_sql);
    assert!(stmt.has_dynamic_elements);
}

#[test]
fn test_dynamic_where_strips_leading_and() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <if test="name != null">AND name = #{name}</if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    let sql = &result.statements[0].flat_sql;
    // WHERE 后不应该紧跟 AND
    assert!(!sql.contains("WHERE AND"), "got: {}", sql);
    assert!(sql.contains("WHERE"), "should have WHERE, got: {}", sql);
}

#[test]
fn test_dynamic_set_strips_trailing_comma() {
    let xml = br#"<mapper namespace="test">
        <update id="updateUser">
            UPDATE users
            <set>
                <if test="name != null">name = #{name},</if>
                <if test="email != null">email = #{email},</if>
            </set>
            WHERE id = #{id}
        </update>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    let sql = &result.statements[0].flat_sql;
    assert!(sql.contains("SET"), "should have SET, got: {}", sql);
    // SET 后面不应该有连续逗号
    assert!(!sql.contains(",,"), "should not have double comma, got: {}", sql);
}

#[test]
fn test_dynamic_foreach() {
    let xml = br#"<mapper namespace="test">
        <select id="findByIds">
            SELECT * FROM users WHERE id IN
            <foreach collection="ids" item="id" open="(" separator="," close=")">
                #{id}
            </foreach>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    let sql = &result.statements[0].flat_sql;
    assert!(sql.contains("IN"), "should have IN, got: {}", sql);
}

#[test]
fn test_dynamic_choose() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <choose>
                    <when test="id != null">AND id = #{id}</when>
                    <otherwise>AND status = 'ACTIVE'</otherwise>
                </choose>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    assert!(result.statements[0].has_dynamic_elements);
}
```

**Step 5: 运行测试**

Run: `cargo test --features ibatis --lib ibatis::tests`
Expected: 全部通过

**Step 6: Commit**

```bash
git add src/ibatis/
git commit -m "feat(ibatis): dynamic SQL tree parsing — if/where/set/foreach/choose/trim"
```

---

## Task 7: CLI 集成 — `parse-xml` 子命令

**目标:** 在 CLI 中添加 `parse-xml` 子命令，支持从文件或 stdin 读取 XML mapper 文件并输出解析结果。

**Files:**
- Modify: `src/bin/ogsql.rs`

**Step 1: 在 `src/bin/ogsql.rs` 中添加 `ParseXml` 命令**

在 `Commands` enum 中添加:

```rust
#[cfg(feature = "ibatis")]
/// Parse iBatis/MyBatis XML mapper file / 解析 iBatis XML mapper 文件
#[command(name = "parse-xml")]
ParseXml,
```

在 `main()` 的 match 中添加:

```rust
#[cfg(feature = "ibatis")]
Commands::ParseXml => cmd_parse_xml(&cli),
```

添加 `cmd_parse_xml` 函数:

```rust
#[cfg(feature = "ibatis")]
fn cmd_parse_xml(cli: &Cli) {
    let input = read_input(cli.file.as_deref());
    let result = ogsql_parser::ibatis::parse_mapper_bytes(input.as_bytes());

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        if !result.errors.is_empty() {
            eprintln!("{} error(s):", result.errors.len());
            for e in &result.errors {
                eprintln!("  {}", e);
            }
        }

        for stmt in &result.statements {
            println!("── {} ({:?}) ──", stmt.id, stmt.kind);
            println!("{}", stmt.flat_sql.trim());
            if stmt.has_dynamic_elements {
                println!("  [contains dynamic SQL elements]");
            }
            if let Some((infos, errors)) = &stmt.parse_result {
                if !errors.is_empty() {
                    eprintln!("  {} parse error(s):", errors.len());
                    for e in errors {
                        eprintln!("    {}", e);
                    }
                } else {
                    println!("  ✓ Parsed successfully ({} statement(s))", infos.len());
                }
            }
            println!();
        }

        println!("Total: {} statement(s) in namespace '{}'", result.statements.len(), result.namespace);
    }
}
```

**Step 2: 测试 CLI**

Run: `echo '<mapper namespace="test"><select id="s">SELECT 1</select></mapper>' | cargo run --features ibatis -- parse-xml`
Expected: 输出解析结果，包含 `s (Select)` 和 `SELECT 1`

Run: `echo '<mapper namespace="test"><select id="s">SELECT 1</select></mapper>' | cargo run --features ibatis -- parse-xml -j`
Expected: JSON 格式输出

**Step 3: Commit**

```bash
git add src/bin/ogsql.rs
git commit -m "feat(ibatis): add parse-xml CLI subcommand"
```

---

## Task 8: 综合测试 + 文档更新

**目标:** 确保所有功能正常工作，更新 README。

**Files:**
- Modify: `README.md` — 添加 iBatis 支持说明
- 全部测试通过

**Step 1: 运行完整测试套件**

Run: `cargo test --features ibatis`
Expected: 所有测试通过（包括现有的 230+ 核心解析器测试）

**Step 2: 验证 feature gate 正确性**

Run: `cargo test`（不带 `--features ibatis`）
Expected: 编译通过，核心测试通过，ibatis 相关测试不运行

Run: `cargo test --features full`
Expected: 全部测试通过

**Step 3: 更新 README.md**

在 README 中添加 iBatis XML 支持部分（跟随现有双语风格）:

在 **Phase 4: DDL** 之后添加:

```markdown
### Phase 5.5: iBatis XML Support / iBatis XML 支持

| Component | Status | Details |
|-----------|--------|---------|
| XML Mapper Parsing / XML Mapper 解析 | ✅ Complete | MyBatis/iBatis XML mapper file parsing |
| Dynamic SQL Elements / 动态 SQL 元素 | ✅ Complete | if, choose, where, set, trim, foreach, bind |
| Include Resolution / 片段引用解析 | ✅ Complete | Same-file <sql> fragment expansion with cycle detection |
| Parameter Handling / 参数处理 | ✅ Complete | #{} → ? (PreparedStatement), ${} → placeholder identifier |
| CLI Support / CLI 支持 | ✅ Complete | `ogsql parse-xml` subcommand (requires `ibatis` feature) |
```

在 CLI Usage 部分添加:

```markdown
#### Parse iBatis XML Mapper / 解析 iBatis XML Mapper

\```bash
# Parse mapper file
ogsql -f UserMapper.xml parse-xml

# Parse with JSON output
ogsql -f UserMapper.xml parse-xml -j

# From stdin
cat UserMapper.xml | ogsql parse-xml
\```
```

在 Build 部分更新:

```markdown
\```bash
# With iBatis XML support
cargo build --release --features ibatis

# All features (includes ibatis)
cargo build --release --features full
\```
```

**Step 4: Final commit**

```bash
git add .
git commit -m "docs: add iBatis XML support documentation to README"
```

---

## 验收标准汇总

| # | 验收条件 | 验证命令 |
|---|----------|---------|
| 1 | 基础 XML 解析：提取 namespace, fragments, statements | `cargo test --features ibatis --lib ibatis::tests::test_parse_mapper_namespace` |
| 2 | 所有 4 种语句类型正确识别 | `cargo test --features ibatis --lib ibatis::tests::test_parse_statements_count` |
| 3 | `<include>` 片段展开 | `cargo test --features ibatis --lib ibatis::tests::test_include_resolution_basic` |
| 4 | 循环引用检测 | `cargo test --features ibatis --lib ibatis::tests::test_include_circular_detection` |
| 5 | 参数替换 `#{→?}, ${→占位符}` | `cargo test --features ibatis --lib ibatis::flatten::param_tests` |
| 6 | 端到端管线：XML → AST | `cargo test --features ibatis --lib ibatis::tests::test_e2e_simple_select` |
| 7 | 动态 SQL: if/where/set/foreach/choose | `cargo test --features ibatis --lib ibatis::tests::test_dynamic_if` |
| 8 | CLI: `parse-xml` 子命令 | `echo '<mapper namespace="t"><select id="s">SELECT 1</select></mapper>' \| cargo run --features ibatis -- parse-xml` |
| 9 | Feature gate 正确：不带 feature 不编译 ibatis | `cargo test` (无 ibatis feature) |
| 10 | 空白保留 | `cargo test --features ibatis --lib ibatis::tests::test_preserves_whitespace_in_sql` |
| 11 | 非SQL元素跳过 | `cargo test --features ibatis --lib ibatis::tests::test_skip_result_map` |
| 12 | 错误 XML 报告 | `cargo test --features ibatis --lib ibatis::tests::test_invalid_xml` |

---

## 不在 v1 范围内的功能（明确排除）

- ❌ 跨文件 `<include>` 引用解析
- ❌ OGNL 表达式求值
- ❌ `<resultMap>` / `<cache>` / `<parameterMap>` 深度建模
- ❌ `<property>` 参数化片段
- ❌ MyBatis annotation (`@Select` 等) 支持
- ❌ MyBatis Plus / TK Mapper 扩展元素
- ❌ SQL 变体穷举（指数级分支展开）
- ❌ XML → SQL → SqlNode → XML 往返转换
