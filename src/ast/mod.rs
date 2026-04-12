pub mod plpgsql;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTableStatement {
    pub temporary: bool,
    pub unlogged: bool,
    pub if_not_exists: bool,
    pub name: ObjectName,
    pub columns: Vec<ColumnDef>,
    pub constraints: Vec<TableConstraint>,
    pub inherits: Vec<ObjectName>,
    pub partition_by: Option<PartitionClause>,
    pub tablespace: Option<String>,
    pub on_commit: Option<OnCommitAction>,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PartitionClause {
    Range { column: ObjectName },
    List { column: ObjectName },
    Hash { column: ObjectName },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OnCommitAction {
    PreserveRows,
    DeleteRows,
    Drop,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
    pub constraints: Vec<ColumnConstraint>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DataType {
    Boolean,
    SmallInt,
    Integer,
    BigInt,
    Real,
    Double,
    Numeric(Option<u32>, Option<u32>),
    Char(Option<u32>),
    Varchar(Option<u32>),
    Text,
    Bytea,
    Timestamp(Option<u32>, Option<TimeZoneInfo>),
    Timestamptz(Option<u32>),
    Date,
    Time(Option<u32>, Option<TimeZoneInfo>),
    Interval,
    Json,
    Jsonb,
    Uuid,
    Bit(Option<u32>),
    Varbit(Option<u32>),
    Custom(ObjectName),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TimeZoneInfo {
    WithTimeZone,
    WithoutTimeZone,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ColumnConstraint {
    NotNull,
    Null,
    Default(Expr),
    Unique,
    PrimaryKey,
    Check(Expr),
    References(ObjectName, Vec<String>),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TableConstraint {
    PrimaryKey(Vec<String>),
    Unique(Vec<String>),
    Check(Expr),
    ForeignKey {
        columns: Vec<String>,
        ref_table: ObjectName,
        ref_columns: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterTableStatement {
    pub if_exists: bool,
    pub name: ObjectName,
    pub actions: Vec<AlterTableAction>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlterTableAction {
    AddColumn(ColumnDef),
    DropColumn {
        name: String,
        if_exists: bool,
        cascade: bool,
    },
    AlterColumn {
        name: String,
        action: AlterColumnAction,
    },
    AddConstraint {
        name: Option<String>,
        constraint: TableConstraint,
    },
    DropConstraint {
        name: String,
        if_exists: bool,
        cascade: bool,
    },
    RenameColumn {
        old: String,
        new: String,
    },
    RenameTo {
        new_name: String,
    },
    OwnerTo {
        owner: String,
    },
    SetSchema {
        schema: String,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlterColumnAction {
    SetDataType(DataType),
    SetDefault(Expr),
    DropDefault,
    SetNotNull,
    DropNotNull,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DropStatement {
    pub object_type: ObjectType,
    pub if_exists: bool,
    pub names: Vec<ObjectName>,
    pub cascade: bool,
    pub purge: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ObjectType {
    Table,
    Index,
    Sequence,
    View,
    Schema,
    Database,
    Tablespace,
    Function,
    Procedure,
    Trigger,
    Extension,
    MaterializedView,
    ForeignTable,
    ForeignServer,
    Fdw,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateIndexStatement {
    pub unique: bool,
    pub if_not_exists: bool,
    pub concurrent: bool,
    pub name: Option<String>,
    pub table: ObjectName,
    pub columns: Vec<IndexColumn>,
    pub tablespace: Option<String>,
    pub where_clause: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IndexColumn {
    pub name: String,
    pub asc: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateSequenceStatement {
    pub if_not_exists: bool,
    pub name: ObjectName,
    pub start: Option<Expr>,
    pub increment: Option<Expr>,
    pub min_value: Option<Expr>,
    pub max_value: Option<Expr>,
    pub cache: Option<Expr>,
    pub cycle: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TruncateStatement {
    pub tables: Vec<ObjectName>,
    pub cascade: bool,
    pub restart_identity: bool,
}

// ========== CREATE TYPE ==========

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTypeStatement {
    pub name: ObjectName,
    pub type_kind: TypeKind,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TypeKind {
    Composite { attributes: Vec<TypeAttribute> },
    Enum { labels: Vec<String> },
    Base { options: Vec<(String, String)> },
    Shell,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TypeAttribute {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Statement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
    Merge(MergeStatement),
    CreateTable(CreateTableStatement),
    AlterTable(AlterTableStatement),
    Drop(DropStatement),
    Truncate(TruncateStatement),
    CreateIndex(CreateIndexStatement),
    CreateSchema(CreateSchemaStatement),
    CreateDatabase(CreateDatabaseStatement),
    CreateTablespace(CreateTablespaceStatement),
    CreateFunction(CreateFunctionStatement),
    CreateProcedure(CreateProcedureStatement),
    CreateType(CreateTypeStatement),
    AlterIndex(AlterIndexStatement),
    CreatePackage(CreatePackageStatement),
    CreateView(CreateViewStatement),
    CreateMaterializedView(CreateMaterializedViewStatement),
    CreateSequence(CreateSequenceStatement),
    CreateTrigger(CreateTriggerStatement),
    CreateExtension(CreateExtensionStatement),
    CreateRole(CreateRoleStatement),
    CreateUser(CreateUserStatement),
    CreateGroup(CreateGroupStatement),
    Grant(GrantStatement),
    Revoke(RevokeStatement),
    Transaction(TransactionStatement),
    Copy(CopyStatement),
    Explain(ExplainStatement),
    Vacuum(VacuumStatement),
    VariableSet(VariableSetStatement),
    VariableShow(VariableShowStatement),
    VariableReset(VariableResetStatement),
    Do(DoStatement),
    Call(CallFuncStatement),
    Prepare(PrepareStatement),
    Execute(ExecuteStatement),
    Deallocate(DeallocateStatement),
    Comment(CommentStatement),
    Lock(LockStatement),
    DeclareCursor(DeclareCursorStatement),
    ClosePortal(ClosePortalStatement),
    Fetch(FetchStatement),
    Checkpoint,
    Discard(DiscardStatement),
    Cluster(ClusterStatement),
    Reindex(ReindexStatement),
    Listen(ListenStatement),
    Notify(NotifyStatement),
    Unlisten(UnlistenStatement),
    Rule(RuleStatement),
    DropRule(DropStatement),
    CreateCast(CreateCastStatement),
    CreateConversion(CreateConversionStatement),
    CreateDomain(CreateDomainStatement),
    AlterDomain(AlterDomainStatement),
    CreateForeignTable(CreateForeignTableStatement),
    CreateForeignServer(CreateForeignServerStatement),
    CreateFdw(CreateFdwStatement),
    CreatePublication(CreatePublicationStatement),
    CreateSubscription(CreateSubscriptionStatement),
    CreateSynonym(CreateSynonymStatement),
    CreateModel(CreateModelStatement),
    CreateAm(CreateAmStatement),
    CreateDirectory(CreateDirectoryStatement),
    CreateNode(CreateNodeStatement),
    CreateNodeGroup(CreateNodeGroupStatement),
    CreateResourcePool(CreateResourcePoolStatement),
    CreateWorkloadGroup(CreateWorkloadGroupStatement),
    CreateAuditPolicy(CreateAuditPolicyStatement),
    CreateMaskingPolicy(CreateMaskingPolicyStatement),
    CreateRlsPolicy(CreateRlsPolicyStatement),
    CreateDataSource(CreateDataSourceStatement),
    CreateEvent(CreateEventStatement),
    CreateOpClass(CreateOpClassStatement),
    CreateOpFamily(CreateOpFamilyStatement),
    CreateContQuery(CreateContQueryStatement),
    CreateStream(CreateStreamStatement),
    CreateKey(CreateKeyStatement),
    CreatePackageBody(CreatePackageBodyStatement),
    AlterFunction(AlterFunctionStatement),
    AlterProcedure(AlterProcedureStatement),
    AlterSchema(AlterSchemaStatement),
    AlterDatabase(AlterDatabaseStatement),
    AlterRole(AlterRoleStatement),
    AlterUser(AlterUserStatement),
    AlterGroup(AlterGroupStatement),
    AlterSequence(AlterSequenceStatement),
    AlterExtension(AlterExtensionStatement),
    AlterCompositeType(AlterCompositeTypeStatement),
    AlterView(AlterViewStatement),
    AlterTrigger(AlterTriggerStatement),
    AlterForeignTable(AlterForeignTableStatement),
    AlterForeignServer(AlterForeignServerStatement),
    AlterFdw(AlterFdwStatement),
    AlterPublication(AlterPublicationStatement),
    AlterSubscription(AlterSubscriptionStatement),
    AlterNode(AlterNodeStatement),
    AlterNodeGroup(AlterNodeGroupStatement),
    AlterResourcePool(AlterResourcePoolStatement),
    AlterWorkloadGroup(AlterWorkloadGroupStatement),
    AlterAuditPolicy(AlterAuditPolicyStatement),
    AlterMaskingPolicy(AlterMaskingPolicyStatement),
    AlterRlsPolicy(AlterRlsPolicyStatement),
    AlterDataSource(AlterDataSourceStatement),
    AlterEvent(AlterEventStatement),
    AlterOpFamily(AlterOpFamilyStatement),
    AlterGlobalConfig(AlterGlobalConfigStatement),
    RefreshMaterializedView(RefreshMatViewStatement),
    Shutdown(ShutdownStatement),
    Barrier(BarrierStatement),
    Purge(PurgeStatement),
    TimeCapsule(TimeCapsuleStatement),
    Snapshot(SnapshotStatement),
    Shrink(ShrinkStatement),
    Verify(VerifyStatement),
    CleanConn(CleanConnStatement),
    Compile(CompileStatement),
    GetDiag(GetDiagStatement),
    ShowEvent(ShowEventStatement),
    AnonyBlock(AnonyBlockStatement),
    RemovePackage(RemovePackageStatement),
    SecLabel(SecLabelStatement),
    CreateWeakPasswordDictionary,
    DropWeakPasswordDictionary,
    CreatePolicyLabel(CreatePolicyLabelStatement),
    AlterPolicyLabel(AlterPolicyLabelStatement),
    DropPolicyLabel(DropPolicyLabelStatement),
    GrantRole(GrantRoleStatement),
    RevokeRole(RevokeRoleStatement),
    Analyze(AnalyzeStatement),
    Empty,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatementInfo {
    pub sql_text: String,
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
    #[serde(flatten)]
    pub statement: Statement,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GroupByItem {
    Expr(Expr),
    GroupingSets(Vec<Vec<Expr>>),
    Rollup(Vec<Expr>),
    Cube(Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SelectStatement {
    pub hints: Vec<String>,
    pub with: Option<WithClause>,
    pub distinct: bool,
    pub targets: Vec<SelectTarget>,
    /// PL/pgSQL extension: `SELECT ... INTO var1, var2 FROM ...`
    pub into_targets: Option<Vec<SelectTarget>>,
    pub from: Vec<TableRef>,
    pub where_clause: Option<Expr>,
    pub group_by: Vec<GroupByItem>,
    pub having: Option<Expr>,
    pub order_by: Vec<OrderByItem>,
    pub limit: Option<Expr>,
    pub offset: Option<Expr>,
    pub fetch: Option<FetchClause>,
    pub lock_clause: Option<LockClause>,
    pub set_operation: Option<SetOperation>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FetchClause {
    pub count: Option<Expr>,
    pub with_ties: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LockClause {
    Update {
        tables: Vec<ObjectName>,
        nowait: bool,
        skip_locked: bool,
    },
    Share {
        tables: Vec<ObjectName>,
        nowait: bool,
        skip_locked: bool,
    },
    NoKeyUpdate {
        tables: Vec<ObjectName>,
        nowait: bool,
        skip_locked: bool,
    },
    KeyShare {
        tables: Vec<ObjectName>,
        nowait: bool,
        skip_locked: bool,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SetOperation {
    Union {
        all: bool,
        right: Box<SelectStatement>,
    },
    Intersect {
        all: bool,
        right: Box<SelectStatement>,
    },
    Except {
        all: bool,
        right: Box<SelectStatement>,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithClause {
    pub recursive: bool,
    pub ctes: Vec<Cte>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Cte {
    pub name: String,
    pub columns: Vec<String>,
    pub query: Box<SelectStatement>,
    /// None = default, Some(true) = MATERIALIZED, Some(false) = NOT MATERIALIZED
    pub materialized: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SelectTarget {
    Expr(Expr, Option<String>),
    Star(Option<String>),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TableRef {
    Table {
        name: ObjectName,
        alias: Option<String>,
    },
    FunctionCall {
        name: ObjectName,
        args: Vec<Expr>,
        alias: Option<String>,
    },
    Subquery {
        query: Box<SelectStatement>,
        alias: Option<String>,
    },
    Join {
        left: Box<TableRef>,
        right: Box<TableRef>,
        join_type: JoinType,
        condition: Option<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderByItem {
    pub expr: Expr,
    pub asc: Option<bool>,
    pub nulls_first: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Expr {
    Literal(Literal),
    ColumnRef(ObjectName),
    BinaryOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    UnaryOp {
        op: String,
        expr: Box<Expr>,
    },
    FunctionCall {
        name: ObjectName,
        args: Vec<Expr>,
        distinct: bool,
        over: Option<WindowSpec>,
    },
    Case {
        operand: Option<Box<Expr>>,
        whens: Vec<WhenClause>,
        else_expr: Option<Box<Expr>>,
    },
    Between {
        expr: Box<Expr>,
        low: Box<Expr>,
        high: Box<Expr>,
        negated: bool,
    },
    InList {
        expr: Box<Expr>,
        list: Vec<Expr>,
        negated: bool,
    },
    InSubquery {
        expr: Box<Expr>,
        subquery: Box<SelectStatement>,
        negated: bool,
    },
    Exists(Box<SelectStatement>),
    Subquery(Box<SelectStatement>),
    IsNull {
        expr: Box<Expr>,
        negated: bool,
    },
    TypeCast {
        expr: Box<Expr>,
        type_name: DataType,
    },
    Parameter(i32),
    Array(Vec<Expr>),
    Parenthesized(Box<Expr>),
    Default,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WhenClause {
    pub condition: Expr,
    pub result: Expr,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Literal {
    Integer(i64),
    Float(String),
    String(String),
    EscapeString(String),
    BitString(String),
    HexString(String),
    NationalString(String),
    DollarString { tag: Option<String>, body: String },
    Boolean(bool),
    Null,
}

pub type ObjectName = Vec<String>;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WindowSpec {
    pub window_name: Option<String>,
    pub partition_by: Vec<Expr>,
    pub order_by: Vec<OrderByItem>,
    pub frame: Option<WindowFrame>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WindowFrameMode {
    Rows,
    Range,
    Groups,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WindowFrameDirection {
    UnboundedPreceding,
    UnboundedFollowing,
    CurrentRow,
    Preceding(i64),
    Following(i64),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WindowFrame {
    pub mode: WindowFrameMode,
    pub start: Option<WindowFrameBound>,
    pub end: Option<WindowFrameBound>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WindowFrameBound {
    pub direction: WindowFrameDirection,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InsertStatement {
    pub hints: Vec<String>,
    pub table: ObjectName,
    pub columns: Vec<String>,
    pub source: InsertSource,
    pub on_conflict: Option<OnConflictAction>,
    pub returning: Vec<SelectTarget>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OnConflictAction {
    Nothing,
    Update {
        target: Option<OnConflictTarget>,
        assignments: Vec<UpdateAssignment>,
        where_clause: Option<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OnConflictTarget {
    Columns(Vec<String>),
    OnConstraint(String),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum InsertSource {
    Values(Vec<Vec<Expr>>),
    Select(Box<SelectStatement>),
    DefaultValues,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateStatement {
    pub hints: Vec<String>,
    pub tables: Vec<TableRef>,
    pub assignments: Vec<UpdateAssignment>,
    pub from: Vec<TableRef>,
    pub where_clause: Option<Expr>,
    pub returning: Vec<SelectTarget>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateAssignment {
    pub column: ObjectName,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleteStatement {
    pub hints: Vec<String>,
    pub tables: Vec<TableRef>,
    pub using: Vec<TableRef>,
    pub where_clause: Option<Expr>,
    pub returning: Vec<SelectTarget>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MergeStatement {
    pub hints: Vec<String>,
    pub target: TableRef,
    pub source: TableRef,
    pub on_condition: Expr,
    pub when_clauses: Vec<MergeWhenClause>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MergeWhenClause {
    pub matched: bool,
    pub action: MergeAction,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MergeAction {
    Update(Vec<UpdateAssignment>),
    Delete,
    Insert {
        columns: Vec<String>,
        values: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransactionStatement {
    pub kind: TransactionKind,
    pub modes: Vec<TransactionMode>,
    pub savepoint_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TransactionKind {
    Begin,
    Commit,
    Rollback,
    Savepoint,
    ReleaseSavepoint,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TransactionMode {
    IsolationLevel(IsolationLevel),
    ReadOnly,
    ReadWrite,
    Deferrable,
    NotDeferrable,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum IsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Serializable,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VariableSetStatement {
    pub local: bool,
    pub session: bool,
    pub name: String,
    pub value: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VariableShowStatement {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VariableResetStatement {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DiscardStatement {
    pub target: DiscardTarget,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DiscardTarget {
    All,
    Plans,
    Sequences,
    Temp,
}

// ── COPY statement ──

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CopyOption {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CopyStatement {
    pub relation: Option<ObjectName>,
    pub query: Option<SelectStatement>,
    pub columns: Vec<String>,
    pub is_from: bool,
    pub filename: Option<String>,
    pub is_program: bool,
    pub options: Vec<CopyOption>,
}

// ── EXPLAIN statement ──

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExplainOption {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExplainStatement {
    pub analyze: bool,
    pub verbose: bool,
    pub performance: bool,
    pub plan: bool,
    pub statement_id: Option<String>,
    pub options: Vec<ExplainOption>,
    pub query: Box<Statement>,
}

// ── CALL statement ──

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CallArg {
    Positional(Expr),
    Named {
        name: String,
        arg: Expr,
        uses_arrow: bool,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CallFuncStatement {
    pub func_name: ObjectName,
    pub args: Vec<CallArg>,
}

macro_rules! stub_struct {
    ($($name:ident),+ $(,)?) => {
        $(
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
            pub struct $name { pub _stub: () }
        )+
    };
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateViewStatement {
    pub replace: bool,
    pub temporary: bool,
    pub recursive: bool,
    pub name: ObjectName,
    pub columns: Vec<String>,
    pub query: Box<SelectStatement>,
    pub check_option: Option<CheckOption>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CheckOption {
    Local,
    Cascaded,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateSchemaStatement {
    pub if_not_exists: bool,
    pub name: Option<String>,
    pub authorization: Option<String>,
    pub elements: Vec<SchemaElement>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SchemaElement {
    Table(CreateTableStatement),
    Index(CreateIndexStatement),
    View(CreateViewStatement),
    Sequence(CreateSequenceStatement),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateDatabaseStatement {
    pub name: String,
    pub owner: Option<String>,
    pub template: Option<String>,
    pub encoding: Option<String>,
    pub locale: Option<String>,
    pub lc_collate: Option<String>,
    pub lc_ctype: Option<String>,
    pub tablespace: Option<String>,
    pub allow_connections: Option<bool>,
    pub connection_limit: Option<i32>,
    pub is_template: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTablespaceStatement {
    pub name: String,
    pub owner: Option<String>,
    pub location: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnonyBlockStatement {
    pub block: crate::ast::plpgsql::PlBlock,
}

pub mod visitor;

macro_rules! stub_struct {
    ($($name:ident),+ $(,)?) => {
        $(
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
            pub struct $name { pub _stub: () }
        )+
    };
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateForeignTableStatement {
    pub name: ObjectName,
    pub columns: Vec<ColumnDef>,
    pub server_name: String,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateForeignServerStatement {
    pub name: String,
    pub server_type: Option<String>,
    pub version: Option<String>,
    pub fdw_name: String,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateFdwStatement {
    pub name: String,
    pub handler: Option<String>,
    pub validator: Option<String>,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePublicationStatement {
    pub name: String,
    pub tables: Vec<ObjectName>,
    pub all_tables: bool,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateSubscriptionStatement {
    pub name: String,
    pub connection: String,
    pub publications: Vec<String>,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateNodeStatement {
    pub name: String,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateNodeGroupStatement {
    pub name: String,
    pub nodes: Vec<String>,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateResourcePoolStatement {
    pub name: String,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateWorkloadGroupStatement {
    pub name: String,
    pub pool_name: Option<String>,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateAuditPolicyStatement {
    pub name: String,
    pub policy_type: String,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMaskingPolicyStatement {
    pub name: String,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateRlsPolicyStatement {
    pub name: String,
    pub table: ObjectName,
    pub permissive: bool,
    pub using_expr: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RoutineParam {
    pub name: String,
    pub mode: Option<String>,
    pub data_type: String,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateFunctionStatement {
    pub replace: bool,
    pub name: ObjectName,
    pub parameters: Vec<RoutineParam>,
    pub return_type: Option<String>,
    pub options: String,
    pub block: Option<crate::ast::plpgsql::PlBlock>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateProcedureStatement {
    pub replace: bool,
    pub name: ObjectName,
    pub parameters: Vec<RoutineParam>,
    pub options: String,
    pub block: Option<crate::ast::plpgsql::PlBlock>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PackageAuthid {
    CurrentUser,
    Definer,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePackageStatement {
    pub replace: bool,
    pub name: ObjectName,
    pub authid: Option<PackageAuthid>,
    pub items: Vec<PackageItem>,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreatePackageBodyStatement {
    pub replace: bool,
    pub name: ObjectName,
    pub items: Vec<PackageItem>,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PackageItem {
    Procedure(PackageProcedure),
    Function(PackageFunction),
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PackageProcedure {
    pub name: ObjectName,
    pub parameters: Vec<RoutineParam>,
    pub block: Option<crate::ast::plpgsql::PlBlock>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PackageFunction {
    pub name: ObjectName,
    pub parameters: Vec<RoutineParam>,
    pub return_type: Option<String>,
    pub block: Option<crate::ast::plpgsql::PlBlock>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateExtensionStatement {
    pub replace: bool,
    pub if_not_exists: bool,
    pub name: String,
    pub schema: Option<String>,
    pub version: Option<String>,
    pub cascade: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateDomainStatement {
    pub name: ObjectName,
    pub data_type: DataType,
    pub default_value: Option<Expr>,
    pub not_null: bool,
    pub check: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CastMethod {
    WithFunction(String),
    WithoutFunction,
    WithInout,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CastContext {
    Implicit,
    Assignment,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateCastStatement {
    pub source_type: DataType,
    pub target_type: DataType,
    pub method: CastMethod,
    pub context: Option<CastContext>,
}

// ========== Wave 6: GRANT / REVOKE ==========

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GrantStatement {
    pub privileges: Vec<Privilege>,
    pub target: GrantTarget,
    pub grantees: Vec<String>,
    pub with_grant_option: bool,
    pub granted_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Privilege {
    All,
    Select,
    Insert,
    Update,
    Delete,
    Usage,
    Create,
    Connect,
    Temporary,
    Execute,
    Trigger,
    References,
    Alter,
    Drop,
    Comment,
    Index,
    Vacuum,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GrantTarget {
    Table(Vec<ObjectName>),
    Schema(Vec<String>),
    Database(Vec<String>),
    Function(Vec<ObjectName>),
    Sequence(Vec<ObjectName>),
    AllTablesInSchema(Vec<String>),
    AllFunctionsInSchema(Vec<String>),
    AllSequencesInSchema(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RevokeStatement {
    pub privileges: Vec<Privilege>,
    pub target: GrantTarget,
    pub grantees: Vec<String>,
    pub cascade: bool,
    pub granted_by: Option<String>,
}

// ========== Wave 8: CREATE TRIGGER + MATERIALIZED VIEW ==========

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateTriggerStatement {
    pub name: String,
    pub or_replace: bool,
    pub constraint: bool,
    pub table: ObjectName,
    pub events: Vec<TriggerEvent>,
    pub for_each: TriggerForEach,
    pub when: Option<Expr>,
    pub func_name: ObjectName,
    pub func_args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TriggerEvent {
    Insert,
    Update,
    UpdateOf(Vec<String>),
    Delete,
    Truncate,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TriggerForEach {
    Row,
    Statement,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMaterializedViewStatement {
    pub if_not_exists: bool,
    pub name: ObjectName,
    pub columns: Vec<String>,
    pub query: Box<SelectStatement>,
    pub tablespace: Option<String>,
    pub with_data: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefreshMatViewStatement {
    pub concurrent: bool,
    pub name: ObjectName,
}

// ========== Wave 9: VACUUM / ANALYZE / COMMENT ON / LOCK TABLE ==========

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VacuumStatement {
    pub full: bool,
    pub verbose: bool,
    pub analyze: bool,
    pub freeze: bool,
    pub tables: Vec<VacuumTarget>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VacuumTarget {
    pub name: ObjectName,
    pub columns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnalyzeStatement {
    pub verbose: bool,
    pub tables: Vec<VacuumTarget>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CommentStatement {
    pub object_type: String,
    pub name: ObjectName,
    pub comment: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LockStatement {
    pub tables: Vec<ObjectName>,
    pub mode: String,
    pub nowait: bool,
}

// ========== Wave 10: PREPARE / EXECUTE / DEALLOCATE / DO ==========

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrepareStatement {
    pub name: String,
    pub data_types: Vec<String>,
    pub statement: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed_statement: Option<Box<Statement>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExecuteStatement {
    pub name: String,
    pub params: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeallocateStatement {
    pub name: Option<String>,
    pub all: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DoStatement {
    pub language: Option<String>,
    pub code: String,
    pub block: Option<crate::ast::plpgsql::PlBlock>,
}

// ========== Wave 11: ALTER DATABASE/SCHEMA/SEQUENCE/FUNCTION/ROLE/USER/SYSTEM ==========

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterDatabaseStatement {
    pub name: String,
    pub action: AlterDatabaseAction,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlterDatabaseAction {
    Set { parameter: String, value: String },
    Reset { parameter: String },
    RenameTo { new_name: String },
    OwnerTo { owner: String },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterSchemaStatement {
    pub name: String,
    pub action: AlterSchemaAction,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlterSchemaAction {
    RenameTo { new_name: String },
    OwnerTo { owner: String },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterSequenceStatement {
    pub name: ObjectName,
    pub options: Vec<SequenceOption>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SequenceOption {
    IncrementBy(i64),
    MinValue(Option<i64>),
    MaxValue(Option<i64>),
    StartWith(i64),
    Restart(bool),
    Cache(i64),
    Cycle(bool),
    NoCycle,
    OwnedBy { owner: ObjectName },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterFunctionStatement {
    pub name: ObjectName,
    pub action: AlterFunctionAction,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlterFunctionAction {
    RenameTo { new_name: String },
    OwnerTo { owner: String },
    SetSchema { schema: String },
    Set { parameter: String, value: String },
    Reset { parameter: String },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterProcedureStatement {
    pub name: ObjectName,
    pub action: AlterFunctionAction,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterRoleStatement {
    pub name: String,
    pub options: Vec<(String, Option<String>)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterUserStatement {
    pub name: String,
    pub options: Vec<(String, Option<String>)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterGroupStatement {
    pub name: String,
    pub action: AlterGroupAction,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlterGroupAction {
    AddUser(String),
    DropUser(String),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterGlobalConfigStatement {
    pub action: AlterGlobalConfigAction,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlterGlobalConfigAction {
    Set { parameter: String, value: String },
    Reset { parameter: String },
}

// ========== Wave 12: CURSOR / LISTEN / NOTIFY / RULE / CLUSTER / REINDEX ==========

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeclareCursorStatement {
    pub name: String,
    pub binary: bool,
    pub scroll: bool,
    pub hold: bool,
    pub query: Box<SelectStatement>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FetchStatement {
    pub direction: FetchDirection,
    pub cursor_name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FetchDirection {
    Next,
    Prior,
    First,
    Last,
    Absolute(i64),
    Relative(i64),
    ForwardAll,
    BackwardAll,
    Forward(i64),
    Backward(i64),
    Count(i64),
    All,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClosePortalStatement {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListenStatement {
    pub channel: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotifyStatement {
    pub channel: String,
    pub payload: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnlistenStatement {
    pub channel: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RuleStatement {
    pub name: String,
    pub table: ObjectName,
    pub event: RuleEvent,
    pub condition: Option<Expr>,
    pub instead: bool,
    pub actions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed_actions: Option<Vec<Statement>>,
}

use std::fmt;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RuleEvent {
    Select,
    Insert,
    Update,
    Delete,
}

impl fmt::Display for RuleEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleEvent::Select => write!(f, "SELECT"),
            RuleEvent::Insert => write!(f, "INSERT"),
            RuleEvent::Update => write!(f, "UPDATE"),
            RuleEvent::Delete => write!(f, "DELETE"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClusterStatement {
    pub table: Option<ObjectName>,
    pub verbose: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReindexStatement {
    pub target: ReindexTarget,
    pub verbose: bool,
    pub concurrent: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ReindexTarget {
    Table(ObjectName),
    Index(ObjectName),
    Schema(String),
    Database(String),
    System,
}

// ========== CREATE TYPE ==========

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterIndexStatement {
    pub if_exists: bool,
    pub name: ObjectName,
    pub action: AlterIndexAction,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlterIndexAction {
    RenameTo(String),
    SetTablespace(String),
    Set(Vec<(String, String)>),
    Reset(Vec<String>),
    NoOp,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlterTypeAction {
    AddAttribute {
        name: String,
        data_type: String,
        cascade: bool,
    },
    DropAttribute {
        name: String,
        if_exists: bool,
        cascade: bool,
    },
    RenameAttribute {
        old_name: String,
        new_name: String,
        cascade: bool,
    },
    RenameTo(String),
    SetSchema(String),
    OwnerTo(String),
    AddEnumValue {
        value: String,
        before: Option<String>,
        after: Option<String>,
    },
    RenameEnumValue {
        old_value: String,
        new_value: String,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterCompositeTypeStatement {
    pub name: ObjectName,
    pub action: AlterTypeAction,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AlterViewAction {
    RenameTo(String),
    Set(Vec<(String, String)>),
    Reset(Vec<String>),
    SetSchema(String),
    OwnerTo(String),
    AlterColumnDefault {
        column: String,
        set_default: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterViewStatement {
    pub name: ObjectName,
    pub action: AlterViewAction,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterTriggerStatement {
    pub name: String,
    pub table: ObjectName,
    pub new_name: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AlterExtensionStatement {
    pub name: String,
    pub action: String,
}

// ========== Remaining stubs (not yet implemented) ==========

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RoleOption {
    Superuser(bool),
    CreateDb(bool),
    CreateRole(bool),
    Inherit(bool),
    Login(bool),
    Replication(bool),
    BypassRls(bool),
    ConnectionLimit(i64),
    EncryptedPassword(String),
    UnencryptedPassword(String),
    ValidUntil(String),
    InRole(Vec<String>),
    Role(Vec<String>),
    Admin(Vec<String>),
    User(Vec<String>),
    Sysid(i64),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateRoleStatement {
    pub name: String,
    pub options: Vec<RoleOption>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateUserStatement {
    pub name: String,
    pub options: Vec<RoleOption>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateGroupStatement {
    pub name: String,
    pub options: Vec<RoleOption>,
}

stub_struct!(
    DropDatabaseStatement,
    DropTablespaceStatement,
    DropRuleStatement,
    CreateConversionStatement,
    AlterDomainStatement,
    CreateSynonymStatement,
    CreateModelStatement,
    CreateAmStatement,
    CreateDirectoryStatement,
    CreateDataSourceStatement,
    CreateEventStatement,
    CreateOpClassStatement,
    CreateOpFamilyStatement,
    CreateContQueryStatement,
    CreateStreamStatement,
    CreateKeyStatement,
    AlterForeignTableStatement,
    AlterForeignServerStatement,
    AlterFdwStatement,
    AlterPublicationStatement,
    AlterSubscriptionStatement,
    AlterNodeStatement,
    AlterNodeGroupStatement,
    AlterResourcePoolStatement,
    AlterWorkloadGroupStatement,
    AlterAuditPolicyStatement,
    AlterMaskingPolicyStatement,
    AlterRlsPolicyStatement,
    AlterDataSourceStatement,
    AlterEventStatement,
    AlterOpFamilyStatement,
    ShutdownStatement,
    BarrierStatement,
    PurgeStatement,
    TimeCapsuleStatement,
    SnapshotStatement,
    ShrinkStatement,
    VerifyStatement,
    CleanConnStatement,
    CompileStatement,
    GetDiagStatement,
    ShowEventStatement,
    RemovePackageStatement,
    SecLabelStatement,
    CreatePolicyLabelStatement,
    AlterPolicyLabelStatement,
    DropPolicyLabelStatement,
);

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GrantRoleStatement {
    pub roles: Vec<String>,
    pub grantees: Vec<String>,
    pub with_admin_option: bool,
    pub granted_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RevokeRoleStatement {
    pub roles: Vec<String>,
    pub grantees: Vec<String>,
    pub granted_by: Option<String>,
    pub cascade: bool,
}
