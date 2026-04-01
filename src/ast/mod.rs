#[derive(Debug, Clone, PartialEq)]
pub struct CreateTableStatement {
    pub if_not_exists: bool,
    pub name: ObjectName,
    pub columns: Vec<ColumnDef>,
    pub constraints: Vec<TableConstraint>,
    pub options: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
    pub constraints: Vec<ColumnConstraint>,
}

#[derive(Debug, Clone, PartialEq)]
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
    Custom(ObjectName),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeZoneInfo {
    WithTimeZone,
    WithoutTimeZone,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnConstraint {
    NotNull,
    Null,
    Default(Expr),
    Unique,
    PrimaryKey,
    Check(Expr),
    References(ObjectName, Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct AlterTableStatement {
    pub if_exists: bool,
    pub name: ObjectName,
    pub actions: Vec<AlterTableAction>,
}

#[derive(Debug, Clone, PartialEq)]
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
    AddConstraint(TableConstraint),
    DropConstraint {
        name: String,
        if_exists: bool,
        cascade: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlterColumnAction {
    SetDataType(DataType),
    SetDefault(Expr),
    DropDefault,
    SetNotNull,
    DropNotNull,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DropStatement {
    pub object_type: ObjectType,
    pub if_exists: bool,
    pub names: Vec<ObjectName>,
    pub cascade: bool,
    pub purge: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    Table,
    Index,
    Sequence,
    View,
    Schema,
    Database,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateIndexStatement {
    pub unique: bool,
    pub if_not_exists: bool,
    pub name: Option<String>,
    pub table: ObjectName,
    pub columns: Vec<IndexColumn>,
    pub where_clause: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexColumn {
    pub name: String,
    pub asc: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct TruncateStatement {
    pub tables: Vec<ObjectName>,
    pub cascade: bool,
    pub restart_identity: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
    Merge(MergeStatement),
    CreateTable(CreateTableStatement),
    AlterTable(AlterTableStatement),
    DropIndex(DropStatement),
    Truncate(TruncateStatement),
    CreateIndex(CreateIndexStatement),
    CreateSchema(CreateSchemaStatement),
    CreateDatabase(CreateDatabaseStatement),
    CreateTablespace(CreateTablespaceStatement),
    DropDatabase(DropDatabaseStatement),
    DropTablespace(DropTablespaceStatement),
    CreateFunction(CreateFunctionStatement),
    CreateProcedure(CreateProcedureStatement),
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
    DropRule(DropRuleStatement),
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
    DropRole(DropRoleStatement),
    DropUser(DropUserStatement),
    DropGroup(DropGroupStatement),
    DropFunction(DropFunctionStatement),
    DropSynonym(DropSynonymStatement),
    DropModel(DropModelStatement),
    DropDirectory(DropDirectoryStatement),
    DropNode(DropNodeStatement),
    DropNodeGroup(DropNodeGroupStatement),
    DropResourcePool(DropResourcePoolStatement),
    DropWorkloadGroup(DropWorkloadGroupStatement),
    DropAuditPolicy(DropAuditPolicyStatement),
    DropMaskingPolicy(DropMaskingPolicyStatement),
    DropRlsPolicy(DropRlsPolicyStatement),
    DropDataSource(DropDataSourceStatement),
    DropEvent(DropEventStatement),
    DropOpClass(DropOpClassStatement),
    DropOpFamily(DropOpFamilyStatement),
    DropSubscription(DropSubscriptionStatement),
    DropGlobalConfig(DropGlobalConfigStatement),
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

#[derive(Debug, Clone, PartialEq)]
pub struct SelectStatement {
    pub with: Option<WithClause>,
    pub distinct: bool,
    pub targets: Vec<SelectTarget>,
    pub from: Vec<TableRef>,
    pub where_clause: Option<Expr>,
    pub group_by: Vec<Expr>,
    pub having: Option<Expr>,
    pub order_by: Vec<OrderByItem>,
    pub limit: Option<Expr>,
    pub offset: Option<Expr>,
    pub set_operation: Option<SetOperation>,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct WithClause {
    pub recursive: bool,
    pub ctes: Vec<Cte>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cte {
    pub name: String,
    pub columns: Vec<String>,
    pub query: Box<SelectStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectTarget {
    Expr(Expr, Option<String>),
    Star(Option<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableRef {
    Table {
        name: ObjectName,
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

#[derive(Debug, Clone, PartialEq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderByItem {
    pub expr: Expr,
    pub asc: Option<bool>,
    pub nulls_first: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
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
        type_name: String,
    },
    Parameter(i32),
    Array(Vec<Expr>),
    Default,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhenClause {
    pub condition: Expr,
    pub result: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(String),
    String(String),
    Boolean(bool),
    Null,
}

pub type ObjectName = Vec<String>;

#[derive(Debug, Clone, PartialEq)]
pub struct InsertStatement {
    pub table: ObjectName,
    pub columns: Vec<String>,
    pub source: InsertSource,
    pub returning: Vec<SelectTarget>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InsertSource {
    Values(Vec<Vec<Expr>>),
    Select(Box<SelectStatement>),
    DefaultValues,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateStatement {
    pub tables: Vec<TableRef>,
    pub assignments: Vec<UpdateAssignment>,
    pub from: Vec<TableRef>,
    pub where_clause: Option<Expr>,
    pub returning: Vec<SelectTarget>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateAssignment {
    pub column: ObjectName,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteStatement {
    pub tables: Vec<TableRef>,
    pub using: Vec<TableRef>,
    pub where_clause: Option<Expr>,
    pub returning: Vec<SelectTarget>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MergeStatement {
    pub target: TableRef,
    pub source: TableRef,
    pub on_condition: Expr,
    pub when_clauses: Vec<MergeWhenClause>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MergeWhenClause {
    pub matched: bool,
    pub action: MergeAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MergeAction {
    Update(Vec<UpdateAssignment>),
    Delete,
    Insert {
        columns: Vec<String>,
        values: Vec<Expr>,
    },
}

macro_rules! stub_struct {
    ($($name:ident),+ $(,)?) => {
        $(
            #[derive(Debug, Clone, PartialEq)]
            pub struct $name { pub _stub: () }
        )+
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateViewStatement {
    pub replace: bool,
    pub temporary: bool,
    pub recursive: bool,
    pub name: ObjectName,
    pub columns: Vec<String>,
    pub query: Box<SelectStatement>,
    pub check_option: Option<CheckOption>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CheckOption {
    Local,
    Cascaded,
}

stub_struct!(
    CreateSchemaStatement,
    CreateDatabaseStatement,
    CreateTablespaceStatement,
    DropDatabaseStatement,
    DropTablespaceStatement,
    CreateFunctionStatement,
    CreateProcedureStatement,
    CreatePackageStatement,
    CreateMaterializedViewStatement,
    CreateTriggerStatement,
    CreateExtensionStatement,
    CreateRoleStatement,
    CreateUserStatement,
    CreateGroupStatement,
    GrantStatement,
    RevokeStatement,
    TransactionStatement,
    CopyStatement,
    ExplainStatement,
    VacuumStatement,
    VariableSetStatement,
    VariableShowStatement,
    VariableResetStatement,
    DoStatement,
    CallFuncStatement,
    PrepareStatement,
    ExecuteStatement,
    DeallocateStatement,
    CommentStatement,
    LockStatement,
    DeclareCursorStatement,
    ClosePortalStatement,
    FetchStatement,
    DiscardStatement,
    ClusterStatement,
    ReindexStatement,
    ListenStatement,
    NotifyStatement,
    UnlistenStatement,
    RuleStatement,
    DropRuleStatement,
    CreateCastStatement,
    CreateConversionStatement,
    CreateDomainStatement,
    AlterDomainStatement,
    CreateForeignTableStatement,
    CreateForeignServerStatement,
    CreateFdwStatement,
    CreatePublicationStatement,
    CreateSubscriptionStatement,
    CreateSynonymStatement,
    CreateModelStatement,
    CreateAmStatement,
    CreateDirectoryStatement,
    CreateNodeStatement,
    CreateNodeGroupStatement,
    CreateResourcePoolStatement,
    CreateWorkloadGroupStatement,
    CreateAuditPolicyStatement,
    CreateMaskingPolicyStatement,
    CreateRlsPolicyStatement,
    CreateDataSourceStatement,
    CreateEventStatement,
    CreateOpClassStatement,
    CreateOpFamilyStatement,
    CreateContQueryStatement,
    CreateStreamStatement,
    CreateKeyStatement,
    CreatePackageBodyStatement,
    AlterFunctionStatement,
    AlterProcedureStatement,
    AlterSchemaStatement,
    AlterDatabaseStatement,
    AlterRoleStatement,
    AlterUserStatement,
    AlterGroupStatement,
    AlterSequenceStatement,
    AlterExtensionStatement,
    AlterCompositeTypeStatement,
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
    AlterGlobalConfigStatement,
    DropRoleStatement,
    DropUserStatement,
    DropGroupStatement,
    DropFunctionStatement,
    DropSynonymStatement,
    DropModelStatement,
    DropDirectoryStatement,
    DropNodeStatement,
    DropNodeGroupStatement,
    DropResourcePoolStatement,
    DropWorkloadGroupStatement,
    DropAuditPolicyStatement,
    DropMaskingPolicyStatement,
    DropRlsPolicyStatement,
    DropDataSourceStatement,
    DropEventStatement,
    DropOpClassStatement,
    DropOpFamilyStatement,
    DropSubscriptionStatement,
    DropGlobalConfigStatement,
    RefreshMatViewStatement,
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
    AnonyBlockStatement,
    RemovePackageStatement,
    SecLabelStatement,
    CreatePolicyLabelStatement,
    AlterPolicyLabelStatement,
    DropPolicyLabelStatement,
    GrantRoleStatement,
    RevokeRoleStatement,
    AnalyzeStatement,
);
