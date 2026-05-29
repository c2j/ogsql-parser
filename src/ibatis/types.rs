//! iBatis/MyBatis mapper 数据模型。

/// 一个完整的 mapper XML 文件解析结果。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MapperFile {
    /// mapper 的 namespace 属性
    pub namespace: String,
    /// SQL 片段定义 (<sql id="...">)
    pub fragments: Vec<SqlFragment>,
    /// iBatis 2.x parameterMap 定义 (<parameterMap id="...">)
    pub parameter_maps: Vec<ParameterMapDef>,
    /// typeAlias 映射 (<typeAlias alias="account" type="testdomain.Account" />)
    pub type_aliases: Vec<(String, String)>,
    /// SQL 语句 (<select>/<insert>/<update>/<delete>)
    pub statements: Vec<MapperStatement>,
}

/// iBatis 2.x <parameterMap> 定义。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParameterMapDef {
    pub id: String,
    pub class: Option<String>,
    pub params: Vec<ParameterMapEntry>,
}

/// iBatis 2.x <parameterMap> 中的 <parameter> 元素。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParameterMapEntry {
    pub property: String,
    pub jdbc_type: Option<String>,
    pub java_type: Option<String>,
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
        java_type: Option<String>,
    },
    Include {
        refid: String,
    },
    If {
        test: String,
        prepend: Option<String>,
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
        prepend: Option<String>,
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
    pub parameter_type: Option<String>,
    pub result_type: Option<String>,
    pub flat_sql: String,
    pub parameters: Vec<ParamMeta>,
    pub has_dynamic_elements: bool,
    pub line: usize,
    pub parse_result: Option<(Vec<crate::ast::StatementInfo>, Vec<crate::parser::ParserError>)>,
}

/// MyBatis 支持的 JDBC 类型（常用子集）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum JdbcType {
    Integer,
    BigInt,
    SmallInt,
    TinyInt,
    Decimal,
    Numeric,
    Double,
    Float,
    Real,
    Char,
    VarChar,
    LongVarChar,
    NChar,
    NVarChar,
    Clob,
    NClob,
    Binary,
    VarBinary,
    Blob,
    Date,
    Time,
    Timestamp,
    Boolean,
    Null,
    Array,
    Other,
}

/// 参数类型推断来源。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum InferenceSource {
    InlineJavaType,
    InlineJdbcType,
    ParameterClass,
    JavaMethodSignature,
    JavaParamAnnotation,
    JavaDtoField,
}

/// 参数元数据。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParamMeta {
    pub name: String,
    pub jdbc_type: Option<JdbcType>,
    pub source: Option<InferenceSource>,
    pub position: usize,
    pub raw: String,
}

// ── Structured Dynamic SQL AST types (Issue #179) ──

/// XML 源码位置，用于将动态 SQL 节点溯源回 Mapper XML 文件。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct XmlSourceLocation {
    pub file_path: Option<String>,
    pub line: usize,
}

/// 保留完整动态 SQL 树形结构的单个语句解析结果。
///
/// 与 [`ParsedStatement`] 不同，此类型保留 `SqlNode` 树而不做扁平化，
/// 使调用方可以自行实现 SQL 变体展开策略（用于指纹匹配、安全审计等场景）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StructuredStatement {
    pub id: String,
    pub kind: StatementKind,
    pub parameter_type: Option<String>,
    pub result_type: Option<String>,
    /// 动态 SQL 节点树 — 本 API 的核心价值。
    /// 调用方遍历此树以枚举所有可能的 SQL 变体。
    pub body: SqlNode,
    /// 是否包含动态元素 (If / Choose / ForEach / Where / Set / Trim / Bind)
    pub has_dynamic_elements: bool,
    /// XML 文件中的源码位置
    pub location: XmlSourceLocation,
    /// 从 body 中收集的所有参数（#{param} 和 ${expr}）
    pub parameters: Vec<ParamMeta>,
}

impl StructuredStatement {
    /// 展开所有可能的 SQL 变体（受控爆炸）。
    /// 内部处理 `<where>`/`<set>`/`<trim>` 的运行时语义。
    pub fn expand_variants(&self, config: &ExpandConfig) -> Vec<ExpandedVariant> {
        crate::ibatis::expand::expand_variants(self, config)
    }

    /// 转换为 [`ParsedStatement`] 供向后兼容的调用方使用。
    /// 使用 `flat_sql`（"最完整"变体）和解析结果。
    pub fn to_parsed_statement(&self, _namespace: &str) -> ParsedStatement {
        use crate::ibatis::flatten;
        let flat_sql = flatten::flatten_sql(&self.body);
        let parse_result =
            if !flat_sql.trim().is_empty() { Some(crate::parser::Parser::parse_sql(&flat_sql)) } else { None };
        ParsedStatement {
            id: self.id.clone(),
            kind: self.kind,
            parameter_type: self.parameter_type.clone(),
            result_type: self.result_type.clone(),
            flat_sql,
            parameters: self.parameters.clone(),
            has_dynamic_elements: self.has_dynamic_elements,
            line: self.location.line,
            parse_result,
        }
    }
}

/// 保留完整动态 SQL 结构的 mapper 解析结果。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StructuredMapper {
    pub namespace: String,
    pub statements: Vec<StructuredStatement>,
    pub fragments: Vec<SqlFragment>,
    pub errors: Vec<crate::ibatis::error::IbatisError>,
}

// ── Expand API types (Issue #179 downstream requirements) ──

/// `expand_variants()` 的展开策略。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfExpandStrategy {
    IncludeOnly,
    ExcludeOnly,
    Both,
}

/// `#{param}` / `${expr}` 在展开 SQL 中的渲染方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceholderStrategy {
    PreserveInternalMarkers,
    QuestionMark,
}

/// 受控展开配置。
#[derive(Debug, Clone)]
pub struct ExpandConfig {
    pub max_depth: usize,
    pub max_variants: usize,
    pub foreach_sizes: Vec<usize>,
    pub if_strategy: IfExpandStrategy,
    pub placeholder: PlaceholderStrategy,
    /// 是否为每个展开变体生成 `parse_result`。
    ///
    /// `false`（默认）时 `ExpandedVariant::parse_result` 为 `None`，
    /// 调用方按需自行解析。`true` 时展开时对每个变体的 `sql`
    /// 调用 `Parser::parse_sql()` 并填充结果。
    pub generate_parse_results: bool,
}

impl Default for ExpandConfig {
    fn default() -> Self {
        ExpandConfig {
            max_depth: 10,
            max_variants: 100,
            foreach_sizes: vec![1, 2],
            if_strategy: IfExpandStrategy::Both,
            placeholder: PlaceholderStrategy::PreserveInternalMarkers,
            generate_parse_results: false,
        }
    }
}

/// 展开过程中每个分支决策的记录。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum BranchStep {
    If { test: String, included: bool },
    Choose { branch_index: usize },
    Foreach { collection: String, size: usize },
}

/// 展开后的单个 SQL 变体。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExpandedVariant {
    /// 展开后的完整 SQL 文本（可直接喂入 Tokenizer + Parser）。
    pub sql: String,
    /// 产生此变体的分支决策路径。
    pub branch_path: Vec<BranchStep>,
    /// 此变体中实际出现的参数（仅限当前分支组合下出现的参数）。
    pub parameters: Vec<ParamMeta>,
    /// 解析结果（可选，按需填充）。
    ///
    /// 在 `expand_variants` 中对每个变体的 `sql` 调用 `Parser::parse_sql()` 生成
    /// （需 `ExpandConfig::generate_parse_results == true`）。
    /// 调用方可从中提取 CALL 目标、表访问关系等。
    pub parse_result: Option<(Vec<crate::ast::StatementInfo>, Vec<crate::parser::ParserError>)>,
}
