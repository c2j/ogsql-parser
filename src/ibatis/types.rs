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
    pub id: String,
    pub body: SqlNode,
}

/// 一个 SQL 语句 (<select>/<insert>/<update>/<delete>)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MapperStatement {
    pub kind: StatementKind,
    pub id: String,
    pub parameter_type: Option<String>,
    pub result_type: Option<String>,
    pub body: SqlNode,
    /// 标签在 XML 文件中的行号（1-based）
    pub line: usize,
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SqlNode {
    Text {
        content: String,
    },
    Parameter {
        name: String,
        java_type: Option<String>,
    },
    RawExpr {
        expr: String,
    },
    If {
        test: String,
        children: Vec<SqlNode>,
    },
    Choose {
        branches: Vec<(Option<String>, Vec<SqlNode>)>,
    },
    /// 多节点序列（SQL 文本 + 动态元素混合的顶层容器）
    Sequence {
        children: Vec<SqlNode>,
    },
    Where {
        children: Vec<SqlNode>,
    },
    Set {
        children: Vec<SqlNode>,
    },
    Trim {
        prefix: Option<String>,
        suffix: Option<String>,
        prefix_overrides: Option<String>,
        suffix_overrides: Option<String>,
        children: Vec<SqlNode>,
    },
    ForEach {
        collection: String,
        item: String,
        index: Option<String>,
        open: Option<String>,
        separator: Option<String>,
        close: Option<String>,
        children: Vec<SqlNode>,
    },
    Bind {
        name: String,
        value: String,
    },
}

/// 扁平化提取结果。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FlattenedStatement {
    pub statement_id: String,
    pub kind: StatementKind,
    pub sql: String,
    pub has_dynamic_elements: bool,
}

/// 完整解析结果。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParsedMapper {
    pub file_path: Option<String>,
    pub namespace: String,
    pub statements: Vec<ParsedStatement>,
    pub errors: Vec<crate::ibatis::error::IbatisError>,
}

/// 单个语句的完整解析结果。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParsedStatement {
    pub id: String,
    pub kind: StatementKind,
    pub flat_sql: String,
    pub has_dynamic_elements: bool,
    pub line: usize,
    pub parse_result: Option<(
        Vec<crate::ast::StatementInfo>,
        Vec<crate::parser::ParserError>,
    )>,
}
