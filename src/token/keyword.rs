#![allow(non_camel_case_types)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Keyword {
    ABORT_P,
    ABSOLUTE_P,
    ACCESS,
    ACCOUNT,
    ACTION,
    ADD_P,
    ADMIN,
    AFTER,
    AGGREGATE,
    ALGORITHM,
    ALL,
    ALSO,
    ALTER,
    ALWAYS,
    ASENSITIVE,
    ANALYSE,
    ANALYZE,
    AND,
    ANY,
    APP,
    APPEND,
    APPLY,
    ARCHIVE,
    ARRAY,
    AS,
    ASC,
    ASOF_P,
    ASSERTION,
    ASSIGNMENT,
    ASYMMETRIC,
    AT,
    ATTRIBUTE,
    AUDIT,
    AUTHID,
    AUTHORIZATION,
    AUTO_INCREMENT,
    AUTOEXTEND,
    AUTOMAPPED,
    BACKWARD,
    BARRIER,
    BEFORE,
    BEGIN_P,
    BEGIN_NON_ANOYBLOCK,
    BETWEEN,
    BIGINT,
    BINARY,
    BINARY_DOUBLE,
    BINARY_DOUBLE_INF,
    BINARY_DOUBLE_NAN,
    BINARY_INTEGER,
    BIT,
    BLANKS,
    BLOB_P,
    BLOCKCHAIN,
    BODY_P,
    BOOLEAN_P,
    BOTH,
    BUCKETCNT,
    BUCKETS,
    BUILD,
    BY,
    BYTE_P,
    BYTEAWITHOUTORDER,
    BYTEAWITHOUTORDERWITHEQUAL,
    CACHE,
    CALL,
    CALLED,
    CANCELABLE,
    CASCADE,
    CASCADED,
    CASE,
    CAST,
    CATALOG_P,
    CATALOG_NAME,
    CHAIN,
    CHANGE,
    CHAR_P,
    CHARACTER,
    CHARACTERISTICS,
    CHARACTERSET,
    CHARSET,
    CHECK,
    CHECKPOINT,
    CLASS,
    CLASS_ORIGIN,
    CLEAN,
    CLIENT,
    CLIENT_MASTER_KEY,
    CLIENT_MASTER_KEYS,
    CLOB,
    CLOSE,
    CLUSTER,
    COALESCE,
    COLLATE,
    COLLATION,
    COLUMN,
    COLUMN_ENCRYPTION_KEY,
    COLUMN_ENCRYPTION_KEYS,
    COLUMN_NAME,
    COLUMNS,
    COMMENT,
    COMMENTS,
    COMMIT,
    COMMITTED,
    COMPACT,
    COMPATIBLE_ILLEGAL_CHARS,
    COMPILE,
    COMPLETE,
    COMPLETION,
    COMPRESS,
    CONCURRENTLY,
    CONDITION,
    CONFIGURATION,
    CONNECT,
    CONNECTION,
    CONSISTENT,
    CONSTANT,
    CONSTRAINT,
    CONSTRAINT_CATALOG,
    CONSTRAINT_NAME,
    CONSTRAINT_SCHEMA,
    CONSTRAINTS,
    CONFLICT,
    CONSTRUCTOR,
    CONTENT_P,
    CONTINUE_P,
    CONTVIEW,
    CONVERSION_P,
    CONVERT_P,
    COORDINATOR,
    COORDINATORS,
    COPY,
    COST,
    CREATE,
    CROSS,
    CSN,
    CSV,
    CUBE,
    CURRENT_P,
    CURRENT_CATALOG,
    CURRENT_DATE,
    CURRENT_ROLE,
    CURRENT_SCHEMA,
    CURRENT_TIME,
    CURRENT_TIMESTAMP,
    CURRENT_USER,
    CURSOR,
    CURSOR_NAME,
    CYCLE,
    DATA_P,
    DATABASE,
    DATAFILE,
    DATANODE,
    DATANODES,
    DATATYPE_CL,
    DATE_P,
    DATE_FORMAT_P,
    DAY_P,
    DAY_HOUR_P,
    DAY_MINUTE_P,
    DAY_SECOND_P,
    DBCOMPATIBILITY_P,
    DEALLOCATE,
    DEC,
    DECIMAL_P,
    DECLARE,
    DECODE,
    DEFAULT,
    DEFAULTS,
    DEFERRABLE,
    DEFERRED,
    DEFINER,
    DELETE_P,
    DELIMITER,
    DELIMITERS,
    DELTA,
    DELTAMERGE,
    DENSE_RANK,
    DESC,
    DETERMINISTIC,
    DIAGNOSTICS,
    DICTIONARY,
    DIRECT,
    DIRECTORY,
    DISABLE_P,
    DISCARD,
    DISCONNECT,
    DISTINCT,
    DISTRIBUTE,
    DISTRIBUTION,
    DO,
    DOCUMENT_P,
    DOMAIN_P,
    DOUBLE_P,
    DROP,
    DUMPFILE,
    DUPLICATE,
    EACH,
    ELASTIC,
    ELSE,
    ENABLE_P,
    ENCLOSED,
    ENCODING,
    ENCRYPTED,
    ENCRYPTED_VALUE,
    ENCRYPTION,
    ENCRYPTION_TYPE,
    END_P,
    ENTITYESCAPING,
    ENDS,
    ENFORCED,
    ENUM_P,
    EOL,
    ERROR_P,
    ERRORS,
    ESCAPE,
    ESCAPED,
    ESCAPING,
    EVALNAME,
    EVENT,
    EVENTS,
    EVERY,
    EXCEPT,
    EXCHANGE,
    EXCLUDE,
    EXCLUDED,
    EXCLUDING,
    EXCLUSIVE,
    EXECUTE,
    EXISTS,
    EXPIRED_P,
    EXPLAIN,
    EXTENSION,
    EXTERNAL,
    EXTRACT,
    FALSE_P,
    FAMILY,
    FAST,
    FEATURES,
    FENCED,
    FETCH,
    FIELDS,
    FILEHEADER_P,
    FILL_MISSING_FIELDS,
    FILLER,
    FILTER,
    FINAL,
    FIRST_P,
    FIXED_P,
    FLOAT_P,
    FOLLOWING,
    FOLLOWS_P,
    FOR,
    FORCE,
    FOREIGN,
    FORMATTER,
    FORWARD,
    FREEZE,
    FROM,
    FULL,
    FUNCTION,
    FUNCTIONS,
    GENERATED,
    GET,
    GLOBAL,
    GRANT,
    GRANTED,
    GREATEST,
    GROUP_P,
    GROUPING_P,
    GROUPS,
    GROUPPARENT,
    HANDLER,
    HAVING,
    HDFSDIRECTORY,
    HEADER_P,
    HOLD,
    HOUR_P,
    HOUR_MINUTE_P,
    HOUR_SECOND_P,
    IDENTIFIED,
    IDENTITY_P,
    IF_P,
    IGNORE,
    IGNORE_EXTRA_DATA,
    ILIKE,
    IMCSTORED,
    IMMEDIATE,
    IMMUTABLE,
    IMPLICIT_P,
    IN_P,
    INCLUDE,
    INCLUDING,
    INCREMENT,
    INCREMENTAL,
    INDEX,
    INDEXES,
    INFILE,
    INFINITE_P,
    INHERIT,
    INHERITS,
    INITIAL_P,
    INITIALLY,
    INITRANS,
    INLINE_P,
    INNER_P,
    INOUT,
    INPUT_P,
    INSENSITIVE,
    INSERT,
    INSTEAD,
    INT_P,
    INTEGER,
    INTERNAL,
    INTERSECT,
    INTERVAL,
    INTO,
    INVISIBLE,
    INVOKER,
    IP,
    IS,
    ISNULL,
    ISOLATION,
    JOIN,
    JSON_EXISTS,
    KEEP,
    KEY,
    KEY_PATH,
    KEY_STORE,
    KILL,
    LABEL,
    LANGUAGE,
    LARGE_P,
    LAST_P,
    LATERAL_P,
    LC_COLLATE_P,
    LC_CTYPE_P,
    LEADING,
    LEAKPROOF,
    LEAST,
    LEFT,
    LESS,
    LEVEL,
    LIKE,
    LIMIT,
    LINES,
    LIST,
    LISTEN,
    LOAD,
    LOCAL,
    LOCALTIME,
    LOCALTIMESTAMP,
    LOCATION,
    LOCK_P,
    LOCKED,
    LOG_P,
    LOGGING,
    LOGIN_ANY,
    LOGIN_FAILURE,
    LOGIN_SUCCESS,
    LOGOUT,
    LOOP,
    MAP,
    MAPPING,
    MASKING,
    MASTER,
    MATCH,
    MATCHED,
    MATERIALIZED,
    MAXEXTENTS,
    MAXSIZE,
    MAXTRANS,
    MAXVALUE,
    MEMBER,
    MERGE,
    MESSAGE_TEXT,
    METHOD,
    MINEXTENTS,
    MINUS_P,
    MINUTE_P,
    MINUTE_SECOND_P,
    MINVALUE,
    MODE,
    MODEL,
    MODIFY_P,
    MONTH_P,
    MOVE,
    MOVEMENT,
    MYSQL_ERRNO,
    NAME_P,
    NAMES,
    NAN_P,
    NATIONAL,
    NATURAL,
    NCHAR,
    NEXT,
    NO,
    NOCOMPRESS,
    NOCYCLE,
    NOENTITYESCAPING,
    NODE,
    NOLOGGING,
    NOMAXVALUE,
    NOMINVALUE,
    NONE,
    NOT,
    NOTHING,
    NOTIFY,
    NOTNULL,
    NOVALIDATE,
    NOWAIT,
    NTH_VALUE_P,
    NULL_P,
    NULLCOLS,
    NULLIF,
    NULLS_P,
    NUMBER_P,
    NUMERIC,
    NUMSTR,
    NVARCHAR,
    NVARCHAR2,
    NVL,
    OBJECT_P,
    OF,
    OFF,
    OFFSET,
    OIDS,
    ON,
    ONLY,
    OPERATOR,
    OPTIMIZATION,
    OPTION,
    OPTIONALLY,
    OPTIONS,
    OR,
    ORDER,
    OUT_P,
    OUTER_P,
    OUTFILE,
    OVER,
    OVERLAPS,
    OVERLAY,
    OWNED,
    OWNER,
    PACKAGE,
    PACKAGES,
    PARALLEL_ENABLE,
    PARSER,
    PARTIAL,
    PARTITION,
    PARTITIONS,
    PASSING,
    PASSWORD,
    PCTFREE,
    PER_P,
    PERCENT,
    PERFORMANCE,
    PERM,
    PIPELINED,
    PLACING,
    PLAN,
    PLANS,
    POLICY,
    POOL,
    POSITION,
    PRECEDES_P,
    PRECEDING,
    PRECISION,
    PREDICT,
    PREFERRED,
    PREFIX,
    PREPARE,
    PREPARED,
    PRESERVE,
    PRIMARY,
    PRIOR,
    PRIORER,
    PRIVATE,
    PRIVILEGE,
    PRIVILEGES,
    PROCEDURAL,
    PROCEDURE,
    PROFILE,
    PUBLIC,
    PUBLICATION,
    PUBLISH,
    PURGE,
    QUERY,
    QUOTE,
    RANDOMIZED,
    RANGE,
    RATIO,
    RAW,
    READ,
    REAL,
    REASSIGN,
    REBUILD,
    RECHECK,
    RECURSIVE,
    RECYCLEBIN,
    REDISANYVALUE,
    REF,
    REFERENCES,
    REFRESH,
    REINDEX,
    REJECT_P,
    RELATIVE_P,
    RELEASE,
    RELOPTIONS,
    REMOTE_P,
    REMOVE,
    RENAME,
    REPEAT,
    REPEATABLE,
    REPLACE,
    REPLICA,
    RESET,
    RESIZE,
    RESOURCE,
    RESPECT_P,
    RESTART,
    RESTRICT,
    RESULT,
    RESULT_CACHE,
    RETURN,
    RETURNED_SQLSTATE,
    RETURNING,
    RETURNS,
    REUSE,
    REVOKE,
    RIGHT,
    ROLE,
    ROLES,
    ROLLBACK,
    ROLLUP,
    ROTATE,
    ROTATION,
    ROW,
    ROW_COUNT,
    ROWNUM,
    ROWS,
    ROWTYPE_P,
    RULE,
    SAMPLE,
    SAVEPOINT,
    SCHEDULE,
    SCHEMA,
    SCHEMA_NAME,
    SCROLL,
    SEARCH,
    SECOND_P,
    SECURITY,
    SELECT,
    SEPARATOR_P,
    SEQUENCE,
    SEQUENCES,
    SERIALIZABLE,
    SERVER,
    SESSION,
    SESSION_USER,
    SET,
    SETOF,
    SETS,
    SHARE,
    SHARE_MEMORY,
    SHIPPABLE,
    SHOW,
    SHRINK,
    SHUTDOWN,
    SIBLINGS,
    SIMILAR,
    SIMPLE,
    SIZE,
    SKIP,
    SLAVE,
    SLICE,
    SMALLDATETIME,
    SMALLDATETIME_FORMAT_P,
    SMALLINT,
    SNAPSHOT,
    SOME,
    SOURCE_P,
    SPACE,
    SPECIFICATION,
    SPILL,
    SPLIT,
    SQL_P,
    STABLE,
    STACKED_P,
    STANDALONE_P,
    START,
    STARTING,
    STARTS,
    STATEMENT,
    STATEMENT_ID,
    STATIC_P,
    STATISTICS,
    STDIN,
    STDOUT,
    STORAGE,
    STORE_P,
    STORED,
    STRATIFY,
    STREAM,
    STRICT_P,
    STRIP_P,
    SUBCLASS_ORIGIN,
    SUBPARTITION,
    SUBPARTITIONS,
    SUBSCRIPTION,
    SUBSTRING,
    SYMMETRIC,
    SYNONYM,
    SYS_REFCURSOR,
    SYSDATE,
    SYSID,
    SYSTEM_P,
    TABLE,
    TABLE_NAME,
    TABLES,
    TABLESAMPLE,
    TABLESPACE,
    TARGET,
    TEMP,
    TEMPLATE,
    TEMPORARY,
    TERMINATED,
    TEXT_P,
    THAN,
    THEN,
    TIES,
    TIME,
    TIME_FORMAT_P,
    TIMECAPSULE,
    TIMESTAMP,
    TIMESTAMP_FORMAT_P,
    TIMESTAMPDIFF,
    TIMEZONE_HOUR_P,
    TIMEZONE_MINUTE_P,
    TINYINT,
    TO,
    TRAILING,
    TRANSACTION,
    TRANSFORM,
    TREAT,
    TRIGGER,
    TRIM,
    TRUE_P,
    TRUNCATE,
    TRUSTED,
    TSFIELD,
    TSTAG,
    TSTIME,
    TYPE_P,
    TYPES_P,
    UNBOUNDED,
    UNCOMMITTED,
    UNDER,
    UNENCRYPTED,
    UNIMCSTORED,
    UNION,
    UNIQUE,
    UNKNOWN,
    UNLIMITED,
    UNLISTEN,
    UNLOCK,
    UNLOGGED,
    UNTIL,
    UNUSABLE,
    UPDATE,
    USE_P,
    USEEOF,
    USER,
    USING,
    VACUUM,
    VALID,
    VALIDATE,
    VALIDATION,
    VALIDATOR,
    VALUE_P,
    VALUES,
    VARCHAR,
    VARCHAR2,
    VARIABLES,
    VARIADIC,
    VARRAY,
    VARYING,
    VCGROUP,
    VERBOSE,
    VERIFY,
    VERSION_P,
    VIEW,
    VISIBLE,
    VOLATILE,
    WAIT,
    WARNINGS,
    WEAK,
    WELLFORMED,
    WHEN,
    WHERE,
    WHILE_P,
    WHITESPACE_P,
    WINDOW,
    WITH,
    WITHIN,
    WITHOUT,
    WORK,
    WORKLOAD,
    WRAPPER,
    WRITE,
    XML_P,
    XMLATTRIBUTES,
    XMLCONCAT,
    XMLELEMENT,
    XMLEXISTS,
    XMLFOREST,
    XMLPARSE,
    XMLPI,
    XMLROOT,
    XMLSERIALIZE,
    YEAR_P,
    YEAR_MONTH_P,
    YES_P,
    ZONE,
}

impl Keyword {
    /// Returns the SQL keyword string
    pub fn as_str(&self) -> &'static str {
        match self {
            Keyword::ABORT_P => "abort",
            Keyword::ABSOLUTE_P => "absolute",
            Keyword::ACCESS => "access",
            Keyword::ACCOUNT => "account",
            Keyword::ACTION => "action",
            Keyword::ADD_P => "add",
            Keyword::ADMIN => "admin",
            Keyword::AFTER => "after",
            Keyword::AGGREGATE => "aggregate",
            Keyword::ALGORITHM => "algorithm",
            Keyword::ALL => "all",
            Keyword::ALSO => "also",
            Keyword::ALTER => "alter",
            Keyword::ALWAYS => "always",
            Keyword::ASENSITIVE => "asensitive",
            Keyword::ANALYSE => "analyse",
            Keyword::ANALYZE => "analyze",
            Keyword::AND => "and",
            Keyword::ANY => "any",
            Keyword::APP => "app",
            Keyword::APPEND => "append",
            Keyword::APPLY => "apply",
            Keyword::ARCHIVE => "archive",
            Keyword::ARRAY => "array",
            Keyword::AS => "as",
            Keyword::ASC => "asc",
            Keyword::ASOF_P => "asof",
            Keyword::ASSERTION => "assertion",
            Keyword::ASSIGNMENT => "assignment",
            Keyword::ASYMMETRIC => "asymmetric",
            Keyword::AT => "at",
            Keyword::ATTRIBUTE => "attribute",
            Keyword::AUDIT => "audit",
            Keyword::AUTHID => "authid",
            Keyword::AUTHORIZATION => "authorization",
            Keyword::AUTO_INCREMENT => "auto_increment",
            Keyword::AUTOEXTEND => "autoextend",
            Keyword::AUTOMAPPED => "automapped",
            Keyword::BACKWARD => "backward",
            Keyword::BARRIER => "barrier",
            Keyword::BEFORE => "before",
            Keyword::BEGIN_P => "begin",
            Keyword::BEGIN_NON_ANOYBLOCK => "begin_non_anoyblock",
            Keyword::BETWEEN => "between",
            Keyword::BIGINT => "bigint",
            Keyword::BINARY => "binary",
            Keyword::BINARY_DOUBLE => "binary_double",
            Keyword::BINARY_DOUBLE_INF => "binary_double_infinity",
            Keyword::BINARY_DOUBLE_NAN => "binary_double_nan",
            Keyword::BINARY_INTEGER => "binary_integer",
            Keyword::BIT => "bit",
            Keyword::BLANKS => "blanks",
            Keyword::BLOB_P => "blob",
            Keyword::BLOCKCHAIN => "blockchain",
            Keyword::BODY_P => "body",
            Keyword::BOOLEAN_P => "boolean",
            Keyword::BOTH => "both",
            Keyword::BUCKETCNT => "bucketcnt",
            Keyword::BUCKETS => "buckets",
            Keyword::BUILD => "build",
            Keyword::BY => "by",
            Keyword::BYTE_P => "byte",
            Keyword::BYTEAWITHOUTORDER => "byteawithoutorder",
            Keyword::BYTEAWITHOUTORDERWITHEQUAL => "byteawithoutorderwithequal",
            Keyword::CACHE => "cache",
            Keyword::CALL => "call",
            Keyword::CALLED => "called",
            Keyword::CANCELABLE => "cancelable",
            Keyword::CASCADE => "cascade",
            Keyword::CASCADED => "cascaded",
            Keyword::CASE => "case",
            Keyword::CAST => "cast",
            Keyword::CATALOG_P => "catalog",
            Keyword::CATALOG_NAME => "catalog_name",
            Keyword::CHAIN => "chain",
            Keyword::CHANGE => "change",
            Keyword::CHAR_P => "char",
            Keyword::CHARACTER => "character",
            Keyword::CHARACTERISTICS => "characteristics",
            Keyword::CHARACTERSET => "characterset",
            Keyword::CHARSET => "charset",
            Keyword::CHECK => "check",
            Keyword::CHECKPOINT => "checkpoint",
            Keyword::CLASS => "class",
            Keyword::CLASS_ORIGIN => "class_origin",
            Keyword::CLEAN => "clean",
            Keyword::CLIENT => "client",
            Keyword::CLIENT_MASTER_KEY => "client_master_key",
            Keyword::CLIENT_MASTER_KEYS => "client_master_keys",
            Keyword::CLOB => "clob",
            Keyword::CLOSE => "close",
            Keyword::CLUSTER => "cluster",
            Keyword::COALESCE => "coalesce",
            Keyword::COLLATE => "collate",
            Keyword::COLLATION => "collation",
            Keyword::COLUMN => "column",
            Keyword::COLUMN_ENCRYPTION_KEY => "column_encryption_key",
            Keyword::COLUMN_ENCRYPTION_KEYS => "column_encryption_keys",
            Keyword::COLUMN_NAME => "column_name",
            Keyword::COLUMNS => "columns",
            Keyword::COMMENT => "comment",
            Keyword::COMMENTS => "comments",
            Keyword::COMMIT => "commit",
            Keyword::COMMITTED => "committed",
            Keyword::COMPACT => "compact",
            Keyword::COMPATIBLE_ILLEGAL_CHARS => "compatible_illegal_chars",
            Keyword::COMPILE => "compile",
            Keyword::COMPLETE => "complete",
            Keyword::COMPLETION => "completion",
            Keyword::COMPRESS => "compress",
            Keyword::CONCURRENTLY => "concurrently",
            Keyword::CONDITION => "condition",
            Keyword::CONFIGURATION => "configuration",
            Keyword::CONNECT => "connect",
            Keyword::CONNECTION => "connection",
            Keyword::CONSISTENT => "consistent",
            Keyword::CONSTANT => "constant",
            Keyword::CONSTRAINT => "constraint",
            Keyword::CONSTRAINT_CATALOG => "constraint_catalog",
            Keyword::CONSTRAINT_NAME => "constraint_name",
            Keyword::CONSTRAINT_SCHEMA => "constraint_schema",
            Keyword::CONFLICT => "conflict",
            Keyword::CONSTRAINTS => "constraints",
            Keyword::CONSTRUCTOR => "constructor",
            Keyword::CONTENT_P => "content",
            Keyword::CONTINUE_P => "continue",
            Keyword::CONTVIEW => "contview",
            Keyword::CONVERSION_P => "conversion",
            Keyword::CONVERT_P => "convert",
            Keyword::COORDINATOR => "coordinator",
            Keyword::COORDINATORS => "coordinators",
            Keyword::COPY => "copy",
            Keyword::COST => "cost",
            Keyword::CREATE => "create",
            Keyword::CROSS => "cross",
            Keyword::CSN => "csn",
            Keyword::CSV => "csv",
            Keyword::CUBE => "cube",
            Keyword::CURRENT_P => "current",
            Keyword::CURRENT_CATALOG => "current_catalog",
            Keyword::CURRENT_DATE => "current_date",
            Keyword::CURRENT_ROLE => "current_role",
            Keyword::CURRENT_SCHEMA => "current_schema",
            Keyword::CURRENT_TIME => "current_time",
            Keyword::CURRENT_TIMESTAMP => "current_timestamp",
            Keyword::CURRENT_USER => "current_user",
            Keyword::CURSOR => "cursor",
            Keyword::CURSOR_NAME => "cursor_name",
            Keyword::CYCLE => "cycle",
            Keyword::DATA_P => "data",
            Keyword::DATABASE => "database",
            Keyword::DATAFILE => "datafile",
            Keyword::DATANODE => "datanode",
            Keyword::DATANODES => "datanodes",
            Keyword::DATATYPE_CL => "datatype_cl",
            Keyword::DATE_P => "date",
            Keyword::DATE_FORMAT_P => "date_format",
            Keyword::DAY_P => "day",
            Keyword::DAY_HOUR_P => "day_hour",
            Keyword::DAY_MINUTE_P => "day_minute",
            Keyword::DAY_SECOND_P => "day_second",
            Keyword::DBCOMPATIBILITY_P => "dbcompatibility",
            Keyword::DEALLOCATE => "deallocate",
            Keyword::DEC => "dec",
            Keyword::DECIMAL_P => "decimal",
            Keyword::DECLARE => "declare",
            Keyword::DECODE => "decode",
            Keyword::DEFAULT => "default",
            Keyword::DEFAULTS => "defaults",
            Keyword::DEFERRABLE => "deferrable",
            Keyword::DEFERRED => "deferred",
            Keyword::DEFINER => "definer",
            Keyword::DELETE_P => "delete",
            Keyword::DELIMITER => "delimiter",
            Keyword::DELIMITERS => "delimiters",
            Keyword::DELTA => "delta",
            Keyword::DELTAMERGE => "deltamerge",
            Keyword::DENSE_RANK => "dense_rank",
            Keyword::DESC => "desc",
            Keyword::DETERMINISTIC => "deterministic",
            Keyword::DIAGNOSTICS => "diagnostics",
            Keyword::DICTIONARY => "dictionary",
            Keyword::DIRECT => "direct",
            Keyword::DIRECTORY => "directory",
            Keyword::DISABLE_P => "disable",
            Keyword::DISCARD => "discard",
            Keyword::DISCONNECT => "disconnect",
            Keyword::DISTINCT => "distinct",
            Keyword::DISTRIBUTE => "distribute",
            Keyword::DISTRIBUTION => "distribution",
            Keyword::DO => "do",
            Keyword::DOCUMENT_P => "document",
            Keyword::DOMAIN_P => "domain",
            Keyword::DOUBLE_P => "double",
            Keyword::DROP => "drop",
            Keyword::DUMPFILE => "dumpfile",
            Keyword::DUPLICATE => "duplicate",
            Keyword::EACH => "each",
            Keyword::ELASTIC => "elastic",
            Keyword::ELSE => "else",
            Keyword::ENABLE_P => "enable",
            Keyword::ENCLOSED => "enclosed",
            Keyword::ENCODING => "encoding",
            Keyword::ENCRYPTED => "encrypted",
            Keyword::ENCRYPTED_VALUE => "encrypted_value",
            Keyword::ENCRYPTION => "encryption",
            Keyword::ENCRYPTION_TYPE => "encryption_type",
            Keyword::END_P => "end",
            Keyword::ENDS => "ends",
            Keyword::ENFORCED => "enforced",
            Keyword::ENTITYESCAPING => "entityescaping",
            Keyword::ENUM_P => "enum",
            Keyword::EOL => "eol",
            Keyword::ERROR_P => "error",
            Keyword::ERRORS => "errors",
            Keyword::ESCAPE => "escape",
            Keyword::ESCAPED => "escaped",
            Keyword::ESCAPING => "escaping",
            Keyword::EVALNAME => "evalname",
            Keyword::EVENT => "event",
            Keyword::EVENTS => "events",
            Keyword::EVERY => "every",
            Keyword::EXCEPT => "except",
            Keyword::EXCHANGE => "exchange",
            Keyword::EXCLUDE => "exclude",
            Keyword::EXCLUDED => "excluded",
            Keyword::EXCLUDING => "excluding",
            Keyword::EXCLUSIVE => "exclusive",
            Keyword::EXECUTE => "execute",
            Keyword::EXISTS => "exists",
            Keyword::EXPIRED_P => "expired",
            Keyword::EXPLAIN => "explain",
            Keyword::EXTENSION => "extension",
            Keyword::EXTERNAL => "external",
            Keyword::EXTRACT => "extract",
            Keyword::FALSE_P => "false",
            Keyword::FAMILY => "family",
            Keyword::FAST => "fast",
            Keyword::FEATURES => "features",
            Keyword::FENCED => "fenced",
            Keyword::FETCH => "fetch",
            Keyword::FIELDS => "fields",
            Keyword::FILEHEADER_P => "fileheader",
            Keyword::FILL_MISSING_FIELDS => "fill_missing_fields",
            Keyword::FILLER => "filler",
            Keyword::FILTER => "filter",
            Keyword::FINAL => "final",
            Keyword::FIRST_P => "first",
            Keyword::FIXED_P => "fixed",
            Keyword::FLOAT_P => "float",
            Keyword::FOLLOWING => "following",
            Keyword::FOLLOWS_P => "follows",
            Keyword::FOR => "for",
            Keyword::FORCE => "force",
            Keyword::FOREIGN => "foreign",
            Keyword::FORMATTER => "formatter",
            Keyword::FORWARD => "forward",
            Keyword::FREEZE => "freeze",
            Keyword::FROM => "from",
            Keyword::FULL => "full",
            Keyword::FUNCTION => "function",
            Keyword::FUNCTIONS => "functions",
            Keyword::GENERATED => "generated",
            Keyword::GET => "get",
            Keyword::GLOBAL => "global",
            Keyword::GRANT => "grant",
            Keyword::GRANTED => "granted",
            Keyword::GREATEST => "greatest",
            Keyword::GROUP_P => "group",
            Keyword::GROUPING_P => "grouping",
            Keyword::GROUPS => "groups",
            Keyword::GROUPPARENT => "groupparent",
            Keyword::HANDLER => "handler",
            Keyword::HAVING => "having",
            Keyword::HDFSDIRECTORY => "hdfsdirectory",
            Keyword::HEADER_P => "header",
            Keyword::HOLD => "hold",
            Keyword::HOUR_P => "hour",
            Keyword::HOUR_MINUTE_P => "hour_minute",
            Keyword::HOUR_SECOND_P => "hour_second",
            Keyword::IDENTIFIED => "identified",
            Keyword::IDENTITY_P => "identity",
            Keyword::IF_P => "if",
            Keyword::IGNORE => "ignore",
            Keyword::IGNORE_EXTRA_DATA => "ignore_extra_data",
            Keyword::ILIKE => "ilike",
            Keyword::IMCSTORED => "imcstored",
            Keyword::IMMEDIATE => "immediate",
            Keyword::IMMUTABLE => "immutable",
            Keyword::IMPLICIT_P => "implicit",
            Keyword::IN_P => "in",
            Keyword::INCLUDE => "include",
            Keyword::INCLUDING => "including",
            Keyword::INCREMENT => "increment",
            Keyword::INCREMENTAL => "incremental",
            Keyword::INDEX => "index",
            Keyword::INDEXES => "indexes",
            Keyword::INFILE => "infile",
            Keyword::INFINITE_P => "infinite",
            Keyword::INHERIT => "inherit",
            Keyword::INHERITS => "inherits",
            Keyword::INITIAL_P => "initial",
            Keyword::INITIALLY => "initially",
            Keyword::INITRANS => "initrans",
            Keyword::INLINE_P => "inline",
            Keyword::INNER_P => "inner",
            Keyword::INOUT => "inout",
            Keyword::INPUT_P => "input",
            Keyword::INSENSITIVE => "insensitive",
            Keyword::INSERT => "insert",
            Keyword::INSTEAD => "instead",
            Keyword::INT_P => "int",
            Keyword::INTEGER => "integer",
            Keyword::INTERNAL => "internal",
            Keyword::INTERSECT => "intersect",
            Keyword::INTERVAL => "interval",
            Keyword::INTO => "into",
            Keyword::INVISIBLE => "invisible",
            Keyword::INVOKER => "invoker",
            Keyword::IP => "ip",
            Keyword::IS => "is",
            Keyword::ISNULL => "isnull",
            Keyword::ISOLATION => "isolation",
            Keyword::JOIN => "join",
            Keyword::JSON_EXISTS => "json_exists",
            Keyword::KEEP => "keep",
            Keyword::KEY => "key",
            Keyword::KEY_PATH => "key_path",
            Keyword::KEY_STORE => "key_store",
            Keyword::KILL => "kill",
            Keyword::LABEL => "label",
            Keyword::LANGUAGE => "language",
            Keyword::LARGE_P => "large",
            Keyword::LAST_P => "last",
            Keyword::LATERAL_P => "lateral",
            Keyword::LC_COLLATE_P => "lc_collate",
            Keyword::LC_CTYPE_P => "lc_ctype",
            Keyword::LEADING => "leading",
            Keyword::LEAKPROOF => "leakproof",
            Keyword::LEAST => "least",
            Keyword::LEFT => "left",
            Keyword::LESS => "less",
            Keyword::LEVEL => "level",
            Keyword::LIKE => "like",
            Keyword::LIMIT => "limit",
            Keyword::LINES => "lines",
            Keyword::LIST => "list",
            Keyword::LISTEN => "listen",
            Keyword::LOAD => "load",
            Keyword::LOCAL => "local",
            Keyword::LOCALTIME => "localtime",
            Keyword::LOCALTIMESTAMP => "localtimestamp",
            Keyword::LOCATION => "location",
            Keyword::LOCK_P => "lock",
            Keyword::LOCKED => "locked",
            Keyword::LOG_P => "log",
            Keyword::LOGGING => "logging",
            Keyword::LOGIN_ANY => "login_any",
            Keyword::LOGIN_FAILURE => "login_failure",
            Keyword::LOGIN_SUCCESS => "login_success",
            Keyword::LOGOUT => "logout",
            Keyword::LOOP => "loop",
            Keyword::MAP => "map",
            Keyword::MAPPING => "mapping",
            Keyword::MASKING => "masking",
            Keyword::MASTER => "master",
            Keyword::MATCH => "match",
            Keyword::MATCHED => "matched",
            Keyword::MATERIALIZED => "materialized",
            Keyword::MAXEXTENTS => "maxextents",
            Keyword::MAXSIZE => "maxsize",
            Keyword::MAXTRANS => "maxtrans",
            Keyword::MAXVALUE => "maxvalue",
            Keyword::MEMBER => "member",
            Keyword::MERGE => "merge",
            Keyword::MESSAGE_TEXT => "message_text",
            Keyword::METHOD => "method",
            Keyword::MINEXTENTS => "minextents",
            Keyword::MINUS_P => "minus",
            Keyword::MINUTE_P => "minute",
            Keyword::MINUTE_SECOND_P => "minute_second",
            Keyword::MINVALUE => "minvalue",
            Keyword::MODE => "mode",
            Keyword::MODEL => "model",
            Keyword::MODIFY_P => "modify",
            Keyword::MONTH_P => "month",
            Keyword::MOVE => "move",
            Keyword::MOVEMENT => "movement",
            Keyword::MYSQL_ERRNO => "mysql_errno",
            Keyword::NAME_P => "name",
            Keyword::NAMES => "names",
            Keyword::NAN_P => "nan",
            Keyword::NATIONAL => "national",
            Keyword::NATURAL => "natural",
            Keyword::NCHAR => "nchar",
            Keyword::NEXT => "next",
            Keyword::NO => "no",
            Keyword::NOCOMPRESS => "nocompress",
            Keyword::NOCYCLE => "nocycle",
            Keyword::NOENTITYESCAPING => "noentityescaping",
            Keyword::NODE => "node",
            Keyword::NOLOGGING => "nologging",
            Keyword::NOMAXVALUE => "nomaxvalue",
            Keyword::NOMINVALUE => "nominvalue",
            Keyword::NONE => "none",
            Keyword::NOT => "not",
            Keyword::NOTHING => "nothing",
            Keyword::NOTIFY => "notify",
            Keyword::NOTNULL => "notnull",
            Keyword::NOVALIDATE => "novalidate",
            Keyword::NOWAIT => "nowait",
            Keyword::NTH_VALUE_P => "nth_value",
            Keyword::NULL_P => "null",
            Keyword::NULLCOLS => "nullcols",
            Keyword::NULLIF => "nullif",
            Keyword::NULLS_P => "nulls",
            Keyword::NUMBER_P => "number",
            Keyword::NUMERIC => "numeric",
            Keyword::NUMSTR => "numstr",
            Keyword::NVARCHAR => "nvarchar",
            Keyword::NVARCHAR2 => "nvarchar2",
            Keyword::NVL => "nvl",
            Keyword::OBJECT_P => "object",
            Keyword::OF => "of",
            Keyword::OFF => "off",
            Keyword::OFFSET => "offset",
            Keyword::OIDS => "oids",
            Keyword::ON => "on",
            Keyword::ONLY => "only",
            Keyword::OPERATOR => "operator",
            Keyword::OPTIMIZATION => "optimization",
            Keyword::OPTION => "option",
            Keyword::OPTIONALLY => "optionally",
            Keyword::OPTIONS => "options",
            Keyword::OR => "or",
            Keyword::ORDER => "order",
            Keyword::OUT_P => "out",
            Keyword::OUTER_P => "outer",
            Keyword::OUTFILE => "outfile",
            Keyword::OVER => "over",
            Keyword::OVERLAPS => "overlaps",
            Keyword::OVERLAY => "overlay",
            Keyword::OWNED => "owned",
            Keyword::OWNER => "owner",
            Keyword::PACKAGE => "package",
            Keyword::PACKAGES => "packages",
            Keyword::PARALLEL_ENABLE => "parallel_enable",
            Keyword::PARSER => "parser",
            Keyword::PARTIAL => "partial",
            Keyword::PARTITION => "partition",
            Keyword::PARTITIONS => "partitions",
            Keyword::PASSING => "passing",
            Keyword::PASSWORD => "password",
            Keyword::PCTFREE => "pctfree",
            Keyword::PER_P => "per",
            Keyword::PERCENT => "percent",
            Keyword::PERFORMANCE => "performance",
            Keyword::PERM => "perm",
            Keyword::PIPELINED => "pipelined",
            Keyword::PLACING => "placing",
            Keyword::PLAN => "plan",
            Keyword::PLANS => "plans",
            Keyword::POLICY => "policy",
            Keyword::POOL => "pool",
            Keyword::POSITION => "position",
            Keyword::PRECEDES_P => "precedes",
            Keyword::PRECEDING => "preceding",
            Keyword::PRECISION => "precision",
            Keyword::PREDICT => "predict",
            Keyword::PREFERRED => "preferred",
            Keyword::PREFIX => "prefix",
            Keyword::PREPARE => "prepare",
            Keyword::PREPARED => "prepared",
            Keyword::PRESERVE => "preserve",
            Keyword::PRIMARY => "primary",
            Keyword::PRIOR => "prior",
            Keyword::PRIORER => "priorer",
            Keyword::PRIVATE => "private",
            Keyword::PRIVILEGE => "privilege",
            Keyword::PRIVILEGES => "privileges",
            Keyword::PROCEDURAL => "procedural",
            Keyword::PROCEDURE => "procedure",
            Keyword::PROFILE => "profile",
            Keyword::PUBLIC => "public",
            Keyword::PUBLICATION => "publication",
            Keyword::PUBLISH => "publish",
            Keyword::PURGE => "purge",
            Keyword::QUERY => "query",
            Keyword::QUOTE => "quote",
            Keyword::RANDOMIZED => "randomized",
            Keyword::RANGE => "range",
            Keyword::RATIO => "ratio",
            Keyword::RAW => "raw",
            Keyword::READ => "read",
            Keyword::REAL => "real",
            Keyword::REASSIGN => "reassign",
            Keyword::REBUILD => "rebuild",
            Keyword::RECHECK => "recheck",
            Keyword::RECURSIVE => "recursive",
            Keyword::RECYCLEBIN => "recyclebin",
            Keyword::REDISANYVALUE => "redisanyvalue",
            Keyword::REF => "ref",
            Keyword::REFERENCES => "references",
            Keyword::REFRESH => "refresh",
            Keyword::REINDEX => "reindex",
            Keyword::REJECT_P => "reject",
            Keyword::RELATIVE_P => "relative",
            Keyword::RELEASE => "release",
            Keyword::RELOPTIONS => "reloptions",
            Keyword::REMOTE_P => "remote",
            Keyword::REMOVE => "remove",
            Keyword::RENAME => "rename",
            Keyword::REPEAT => "repeat",
            Keyword::REPEATABLE => "repeatable",
            Keyword::REPLACE => "replace",
            Keyword::REPLICA => "replica",
            Keyword::RESET => "reset",
            Keyword::RESIZE => "resize",
            Keyword::RESOURCE => "resource",
            Keyword::RESPECT_P => "respect",
            Keyword::RESTART => "restart",
            Keyword::RESTRICT => "restrict",
            Keyword::RESULT => "result",
            Keyword::RESULT_CACHE => "result_cache",
            Keyword::RETURN => "return",
            Keyword::RETURNED_SQLSTATE => "returned_sqlstate",
            Keyword::RETURNING => "returning",
            Keyword::RETURNS => "returns",
            Keyword::REUSE => "reuse",
            Keyword::REVOKE => "revoke",
            Keyword::RIGHT => "right",
            Keyword::ROLE => "role",
            Keyword::ROLES => "roles",
            Keyword::ROLLBACK => "rollback",
            Keyword::ROLLUP => "rollup",
            Keyword::ROTATE => "rotate",
            Keyword::ROTATION => "rotation",
            Keyword::ROW => "row",
            Keyword::ROW_COUNT => "row_count",
            Keyword::ROWNUM => "rownum",
            Keyword::ROWS => "rows",
            Keyword::ROWTYPE_P => "rowtype",
            Keyword::RULE => "rule",
            Keyword::SAMPLE => "sample",
            Keyword::SAVEPOINT => "savepoint",
            Keyword::SCHEDULE => "schedule",
            Keyword::SCHEMA => "schema",
            Keyword::SCHEMA_NAME => "schema_name",
            Keyword::SCROLL => "scroll",
            Keyword::SEARCH => "search",
            Keyword::SECOND_P => "second",
            Keyword::SECURITY => "security",
            Keyword::SELECT => "select",
            Keyword::SEPARATOR_P => "separator",
            Keyword::SEQUENCE => "sequence",
            Keyword::SEQUENCES => "sequences",
            Keyword::SERIALIZABLE => "serializable",
            Keyword::SERVER => "server",
            Keyword::SESSION => "session",
            Keyword::SESSION_USER => "session_user",
            Keyword::SET => "set",
            Keyword::SETOF => "setof",
            Keyword::SETS => "sets",
            Keyword::SHARE => "share",
            Keyword::SHARE_MEMORY => "share_memory",
            Keyword::SHIPPABLE => "shippable",
            Keyword::SHOW => "show",
            Keyword::SHRINK => "shrink",
            Keyword::SHUTDOWN => "shutdown",
            Keyword::SIBLINGS => "siblings",
            Keyword::SIMILAR => "similar",
            Keyword::SIMPLE => "simple",
            Keyword::SIZE => "size",
            Keyword::SKIP => "skip",
            Keyword::SLAVE => "slave",
            Keyword::SLICE => "slice",
            Keyword::SMALLDATETIME => "smalldatetime",
            Keyword::SMALLDATETIME_FORMAT_P => "smalldatetime_format",
            Keyword::SMALLINT => "smallint",
            Keyword::SNAPSHOT => "snapshot",
            Keyword::SOME => "some",
            Keyword::SOURCE_P => "source",
            Keyword::SPACE => "space",
            Keyword::SPECIFICATION => "specification",
            Keyword::SPILL => "spill",
            Keyword::SPLIT => "split",
            Keyword::SQL_P => "sql",
            Keyword::STABLE => "stable",
            Keyword::STACKED_P => "stacked",
            Keyword::STANDALONE_P => "standalone",
            Keyword::START => "start",
            Keyword::STARTING => "starting",
            Keyword::STARTS => "starts",
            Keyword::STATEMENT => "statement",
            Keyword::STATEMENT_ID => "statement_id",
            Keyword::STATIC_P => "static",
            Keyword::STATISTICS => "statistics",
            Keyword::STDIN => "stdin",
            Keyword::STDOUT => "stdout",
            Keyword::STORAGE => "storage",
            Keyword::STORE_P => "store",
            Keyword::STORED => "stored",
            Keyword::STRATIFY => "stratify",
            Keyword::STREAM => "stream",
            Keyword::STRICT_P => "strict",
            Keyword::STRIP_P => "strip",
            Keyword::SUBCLASS_ORIGIN => "subclass_origin",
            Keyword::SUBPARTITION => "subpartition",
            Keyword::SUBPARTITIONS => "subpartitions",
            Keyword::SUBSCRIPTION => "subscription",
            Keyword::SUBSTRING => "substring",
            Keyword::SYMMETRIC => "symmetric",
            Keyword::SYNONYM => "synonym",
            Keyword::SYS_REFCURSOR => "sys_refcursor",
            Keyword::SYSDATE => "sysdate",
            Keyword::SYSID => "sysid",
            Keyword::SYSTEM_P => "system",
            Keyword::TABLE => "table",
            Keyword::TABLE_NAME => "table_name",
            Keyword::TABLES => "tables",
            Keyword::TABLESAMPLE => "tablesample",
            Keyword::TABLESPACE => "tablespace",
            Keyword::TARGET => "target",
            Keyword::TEMP => "temp",
            Keyword::TEMPLATE => "template",
            Keyword::TEMPORARY => "temporary",
            Keyword::TERMINATED => "terminated",
            Keyword::TEXT_P => "text",
            Keyword::THAN => "than",
            Keyword::THEN => "then",
            Keyword::TIES => "ties",
            Keyword::TIME => "time",
            Keyword::TIME_FORMAT_P => "time_format",
            Keyword::TIMECAPSULE => "timecapsule",
            Keyword::TIMESTAMP => "timestamp",
            Keyword::TIMESTAMP_FORMAT_P => "timestamp_format",
            Keyword::TIMESTAMPDIFF => "timestampdiff",
            Keyword::TIMEZONE_HOUR_P => "timezone_hour",
            Keyword::TIMEZONE_MINUTE_P => "timezone_minute",
            Keyword::TINYINT => "tinyint",
            Keyword::TO => "to",
            Keyword::TRAILING => "trailing",
            Keyword::TRANSACTION => "transaction",
            Keyword::TRANSFORM => "transform",
            Keyword::TREAT => "treat",
            Keyword::TRIGGER => "trigger",
            Keyword::TRIM => "trim",
            Keyword::TRUE_P => "true",
            Keyword::TRUNCATE => "truncate",
            Keyword::TRUSTED => "trusted",
            Keyword::TSFIELD => "tsfield",
            Keyword::TSTAG => "tstag",
            Keyword::TSTIME => "tstime",
            Keyword::TYPE_P => "type",
            Keyword::TYPES_P => "types",
            Keyword::UNBOUNDED => "unbounded",
            Keyword::UNCOMMITTED => "uncommitted",
            Keyword::UNDER => "under",
            Keyword::UNENCRYPTED => "unencrypted",
            Keyword::UNIMCSTORED => "unimcstored",
            Keyword::UNION => "union",
            Keyword::UNIQUE => "unique",
            Keyword::UNKNOWN => "unknown",
            Keyword::UNLIMITED => "unlimited",
            Keyword::UNLISTEN => "unlisten",
            Keyword::UNLOCK => "unlock",
            Keyword::UNLOGGED => "unlogged",
            Keyword::UNTIL => "until",
            Keyword::UNUSABLE => "unusable",
            Keyword::UPDATE => "update",
            Keyword::USE_P => "use",
            Keyword::USEEOF => "useeof",
            Keyword::USER => "user",
            Keyword::USING => "using",
            Keyword::VACUUM => "vacuum",
            Keyword::VALID => "valid",
            Keyword::VALIDATE => "validate",
            Keyword::VALIDATION => "validation",
            Keyword::VALIDATOR => "validator",
            Keyword::VALUE_P => "value",
            Keyword::VALUES => "values",
            Keyword::VARCHAR => "varchar",
            Keyword::VARCHAR2 => "varchar2",
            Keyword::VARIABLES => "variables",
            Keyword::VARIADIC => "variadic",
            Keyword::VARRAY => "varray",
            Keyword::VARYING => "varying",
            Keyword::VCGROUP => "vcgroup",
            Keyword::VERBOSE => "verbose",
            Keyword::VERIFY => "verify",
            Keyword::VERSION_P => "version",
            Keyword::VIEW => "view",
            Keyword::VISIBLE => "visible",
            Keyword::VOLATILE => "volatile",
            Keyword::WAIT => "wait",
            Keyword::WARNINGS => "warnings",
            Keyword::WEAK => "weak",
            Keyword::WELLFORMED => "wellformed",
            Keyword::WHEN => "when",
            Keyword::WHERE => "where",
            Keyword::WHILE_P => "while",
            Keyword::WHITESPACE_P => "whitespace",
            Keyword::WINDOW => "window",
            Keyword::WITH => "with",
            Keyword::WITHIN => "within",
            Keyword::WITHOUT => "without",
            Keyword::WORK => "work",
            Keyword::WORKLOAD => "workload",
            Keyword::WRAPPER => "wrapper",
            Keyword::WRITE => "write",
            Keyword::XML_P => "xml",
            Keyword::XMLATTRIBUTES => "xmlattributes",
            Keyword::XMLCONCAT => "xmlconcat",
            Keyword::XMLELEMENT => "xmlelement",
            Keyword::XMLEXISTS => "xmlexists",
            Keyword::XMLFOREST => "xmlforest",
            Keyword::XMLPARSE => "xmlparse",
            Keyword::XMLPI => "xmlpi",
            Keyword::XMLROOT => "xmlroot",
            Keyword::XMLSERIALIZE => "xmlserialize",
            Keyword::YEAR_P => "year",
            Keyword::YEAR_MONTH_P => "year_month",
            Keyword::YES_P => "yes",
            Keyword::ZONE => "zone",
        }
    }

    /// Returns the keyword category
    pub fn category(&self) -> KeywordCategory {
        match self {
            Keyword::ALL | Keyword::ANALYSE | Keyword::ANALYZE | Keyword::AND | Keyword::ANY => {
                KeywordCategory::Reserved
            }
            Keyword::ARRAY | Keyword::AS | Keyword::ASC | Keyword::ASYMMETRIC | Keyword::AUTHID => {
                KeywordCategory::Reserved
            }
            Keyword::BOTH | Keyword::BUCKETS | Keyword::CASE | Keyword::CAST | Keyword::CHECK => {
                KeywordCategory::Reserved
            }
            Keyword::COLLATE
            | Keyword::COLUMN
            | Keyword::CONSTRAINT
            | Keyword::CREATE
            | Keyword::CURRENT_CATALOG => KeywordCategory::Reserved,
            Keyword::CURRENT_DATE
            | Keyword::CURRENT_ROLE
            | Keyword::CURRENT_TIME
            | Keyword::CURRENT_TIMESTAMP
            | Keyword::CURRENT_USER => KeywordCategory::Reserved,
            Keyword::DEFAULT
            | Keyword::DEFERRABLE
            | Keyword::DESC
            | Keyword::DISTINCT
            | Keyword::DO => KeywordCategory::Reserved,
            Keyword::ELSE
            | Keyword::END_P
            | Keyword::EXCEPT
            | Keyword::EXCLUDED
            | Keyword::FALSE_P => KeywordCategory::Reserved,
            Keyword::FETCH
            | Keyword::FOR
            | Keyword::FOREIGN
            | Keyword::FROM
            | Keyword::GRANT => KeywordCategory::Reserved,
            Keyword::GROUP_P | Keyword::GROUPPARENT | Keyword::HAVING | Keyword::IN_P => {
                KeywordCategory::Reserved
            }
            Keyword::INITIALLY
            | Keyword::INTERSECT
            | Keyword::INTO
            | Keyword::IS
            | Keyword::LEADING => KeywordCategory::Reserved,
            Keyword::LESS
            | Keyword::LIMIT
            | Keyword::LOCALTIME
            | Keyword::LOCALTIMESTAMP
            | Keyword::MAXVALUE
            | Keyword::MINUS_P => KeywordCategory::Reserved,
            Keyword::MODIFY_P
            | Keyword::NOCYCLE
            | Keyword::NOT
            | Keyword::NULL_P
            | Keyword::OFFSET => KeywordCategory::Reserved,
            Keyword::ON | Keyword::ONLY | Keyword::OR | Keyword::ORDER | Keyword::PERFORMANCE => {
                KeywordCategory::Reserved
            }
            Keyword::PLACING
            | Keyword::PRIMARY
            | Keyword::PROCEDURE
            | Keyword::REFERENCES
            | Keyword::REJECT_P => KeywordCategory::Reserved,
            Keyword::RETURNING | Keyword::ROWNUM | Keyword::SELECT | Keyword::SESSION_USER => {
                KeywordCategory::Reserved
            }
            Keyword::SHRINK | Keyword::SOME | Keyword::SYMMETRIC | Keyword::SYSDATE => {
                KeywordCategory::Reserved
            }
            Keyword::TABLE | Keyword::THEN | Keyword::TO | Keyword::TRAILING | Keyword::TRUE_P => {
                KeywordCategory::Reserved
            }
            Keyword::UNION | Keyword::UNIQUE | Keyword::USER | Keyword::USING => {
                KeywordCategory::Reserved
            }
            Keyword::VARIADIC
            | Keyword::VERIFY
            | Keyword::WHEN
            | Keyword::WHERE
            | Keyword::WINDOW => KeywordCategory::Reserved,
            Keyword::WITH => KeywordCategory::Reserved,
            Keyword::BETWEEN
            | Keyword::BIGINT
            | Keyword::BINARY_DOUBLE
            | Keyword::BINARY_DOUBLE_INF
            | Keyword::BINARY_DOUBLE_NAN => KeywordCategory::ColName,
            Keyword::BINARY_INTEGER
            | Keyword::BIT
            | Keyword::BOOLEAN_P
            | Keyword::BUCKETCNT
            | Keyword::BYTEAWITHOUTORDER => KeywordCategory::ColName,
            Keyword::BYTEAWITHOUTORDERWITHEQUAL
            | Keyword::CHAR_P
            | Keyword::CHARACTER
            | Keyword::COALESCE
            | Keyword::DATE_P => KeywordCategory::ColName,
            Keyword::DEC
            | Keyword::DECIMAL_P
            | Keyword::DECODE
            | Keyword::EXISTS
            | Keyword::EXTRACT => KeywordCategory::ColName,
            Keyword::FLOAT_P
            | Keyword::GREATEST
            | Keyword::GROUPING_P
            | Keyword::INOUT
            | Keyword::INT_P => KeywordCategory::ColName,
            Keyword::INTEGER
            | Keyword::INTERVAL
            | Keyword::JSON_EXISTS
            | Keyword::LEAST
            | Keyword::NATIONAL => KeywordCategory::ColName,
            Keyword::NCHAR
            | Keyword::NONE
            | Keyword::NTH_VALUE_P
            | Keyword::NULLIF
            | Keyword::NUMBER_P => KeywordCategory::ColName,
            Keyword::NUMERIC
            | Keyword::NVARCHAR
            | Keyword::NVARCHAR2
            | Keyword::NVL
            | Keyword::OUT_P => KeywordCategory::ColName,
            Keyword::OVERLAY
            | Keyword::POSITION
            | Keyword::PRECISION
            | Keyword::REAL
            | Keyword::ROW => KeywordCategory::ColName,
            Keyword::SETOF
            | Keyword::SMALLDATETIME
            | Keyword::SMALLINT
            | Keyword::SUBSTRING
            | Keyword::TIME => KeywordCategory::ColName,
            Keyword::TIMESTAMP
            | Keyword::TIMESTAMPDIFF
            | Keyword::TINYINT
            | Keyword::TREAT
            | Keyword::TRIM => KeywordCategory::ColName,
            Keyword::VALUES
            | Keyword::VARCHAR
            | Keyword::VARCHAR2
            | Keyword::XMLATTRIBUTES
            | Keyword::XMLCONCAT => KeywordCategory::ColName,
            Keyword::XMLELEMENT
            | Keyword::XMLEXISTS
            | Keyword::XMLFOREST
            | Keyword::XMLPARSE
            | Keyword::XMLPI => KeywordCategory::ColName,
            Keyword::XMLROOT | Keyword::XMLSERIALIZE => KeywordCategory::ColName,
            Keyword::AUTHORIZATION
            | Keyword::BINARY
            | Keyword::COLLATION
            | Keyword::COMPACT
            | Keyword::CONCURRENTLY => KeywordCategory::TypeFuncName,
            Keyword::CROSS
            | Keyword::CSN
            | Keyword::CURRENT_SCHEMA
            | Keyword::DELTAMERGE
            | Keyword::FREEZE => KeywordCategory::TypeFuncName,
            Keyword::FULL
            | Keyword::HDFSDIRECTORY
            | Keyword::ILIKE
            | Keyword::INNER_P
            | Keyword::JOIN => KeywordCategory::TypeFuncName,
            Keyword::LEFT
            | Keyword::LIKE
            | Keyword::NATURAL
            | Keyword::NOTNULL
            | Keyword::OUTER_P => KeywordCategory::TypeFuncName,
            Keyword::OVERLAPS
            | Keyword::RECYCLEBIN
            | Keyword::RIGHT
            | Keyword::SIMILAR
            | Keyword::TABLESAMPLE => KeywordCategory::TypeFuncName,
            Keyword::TIMECAPSULE | Keyword::VERBOSE => KeywordCategory::TypeFuncName,
            Keyword::ABORT_P
            | Keyword::ABSOLUTE_P
            | Keyword::ACCESS
            | Keyword::ACCOUNT
            | Keyword::ACTION => KeywordCategory::Unreserved,
            Keyword::ADD_P
            | Keyword::ADMIN
            | Keyword::AFTER
            | Keyword::AGGREGATE
            | Keyword::ALGORITHM => KeywordCategory::Unreserved,
            Keyword::ALSO | Keyword::ALTER | Keyword::ALWAYS | Keyword::APP | Keyword::APPEND => {
                KeywordCategory::Unreserved
            }
            Keyword::APPLY
            | Keyword::ARCHIVE
            | Keyword::ASOF_P
            | Keyword::ASSERTION
            | Keyword::ASENSITIVE
            | Keyword::ASSIGNMENT => KeywordCategory::Unreserved,
            Keyword::AT
            | Keyword::ATTRIBUTE
            | Keyword::AUDIT
            | Keyword::AUTO_INCREMENT
            | Keyword::AUTOEXTEND => KeywordCategory::Unreserved,
            Keyword::AUTOMAPPED
            | Keyword::BACKWARD
            | Keyword::BARRIER
            | Keyword::BEFORE
            | Keyword::BEGIN_P => KeywordCategory::Unreserved,
            Keyword::BEGIN_NON_ANOYBLOCK
            | Keyword::BLANKS
            | Keyword::BLOB_P
            | Keyword::BLOCKCHAIN
            | Keyword::BODY_P => KeywordCategory::Unreserved,
            Keyword::BUILD | Keyword::BY | Keyword::BYTE_P | Keyword::CACHE | Keyword::CALL => {
                KeywordCategory::Unreserved
            }
            Keyword::CALLED
            | Keyword::CANCELABLE
            | Keyword::CASCADE
            | Keyword::CASCADED
            | Keyword::CATALOG_P => KeywordCategory::Unreserved,
            Keyword::CATALOG_NAME
            | Keyword::CHAIN
            | Keyword::CHANGE
            | Keyword::CHARACTERISTICS
            | Keyword::CHARACTERSET => KeywordCategory::Unreserved,
            Keyword::CHARSET
            | Keyword::CHECKPOINT
            | Keyword::CLASS
            | Keyword::CLASS_ORIGIN
            | Keyword::CLEAN => KeywordCategory::Unreserved,
            Keyword::CLIENT
            | Keyword::CLIENT_MASTER_KEY
            | Keyword::CLIENT_MASTER_KEYS
            | Keyword::CLOB
            | Keyword::CLOSE => KeywordCategory::Unreserved,
            Keyword::CLUSTER
            | Keyword::COLUMN_ENCRYPTION_KEY
            | Keyword::COLUMN_ENCRYPTION_KEYS
            | Keyword::COLUMN_NAME
            | Keyword::COLUMNS => KeywordCategory::Unreserved,
            Keyword::COMMENT
            | Keyword::COMMENTS
            | Keyword::COMMIT
            | Keyword::COMMITTED
            | Keyword::COMPATIBLE_ILLEGAL_CHARS => KeywordCategory::Unreserved,
            Keyword::COMPILE
            | Keyword::COMPLETE
            | Keyword::COMPLETION
            | Keyword::COMPRESS
            | Keyword::CONDITION => KeywordCategory::Unreserved,
            Keyword::CONFIGURATION
            | Keyword::CONNECT
            | Keyword::CONNECTION
            | Keyword::CONSISTENT
            | Keyword::CONSTANT => KeywordCategory::Unreserved,
            Keyword::CONFLICT
            | Keyword::CONSTRAINT_CATALOG
            | Keyword::CONSTRAINT_NAME
            | Keyword::CONSTRAINT_SCHEMA
            | Keyword::CONSTRAINTS
            | Keyword::CONSTRUCTOR => KeywordCategory::Unreserved,
            Keyword::CONTENT_P
            | Keyword::CONTINUE_P
            | Keyword::CONTVIEW
            | Keyword::CONVERSION_P
            | Keyword::CONVERT_P => KeywordCategory::Unreserved,
            Keyword::COORDINATOR
            | Keyword::COORDINATORS
            | Keyword::COPY
            | Keyword::COST
            | Keyword::CSV => KeywordCategory::Unreserved,
            Keyword::CUBE
            | Keyword::CURRENT_P
            | Keyword::CURSOR
            | Keyword::CURSOR_NAME
            | Keyword::CYCLE => KeywordCategory::Unreserved,
            Keyword::DATA_P
            | Keyword::DATABASE
            | Keyword::DATAFILE
            | Keyword::DATANODE
            | Keyword::DATANODES => KeywordCategory::Unreserved,
            Keyword::DATATYPE_CL
            | Keyword::DATE_FORMAT_P
            | Keyword::DAY_P
            | Keyword::DAY_HOUR_P
            | Keyword::DAY_MINUTE_P => KeywordCategory::Unreserved,
            Keyword::DAY_SECOND_P
            | Keyword::DBCOMPATIBILITY_P
            | Keyword::DEALLOCATE
            | Keyword::DECLARE
            | Keyword::DEFAULTS => KeywordCategory::Unreserved,
            Keyword::DEFERRED
            | Keyword::DEFINER
            | Keyword::DELETE_P
            | Keyword::DELIMITER
            | Keyword::DELIMITERS => KeywordCategory::Unreserved,
            Keyword::DELTA
            | Keyword::DENSE_RANK
            | Keyword::DETERMINISTIC
            | Keyword::DIAGNOSTICS
            | Keyword::DICTIONARY => KeywordCategory::Unreserved,
            Keyword::DIRECT
            | Keyword::DIRECTORY
            | Keyword::DISABLE_P
            | Keyword::DISCARD
            | Keyword::DISCONNECT => KeywordCategory::Unreserved,
            Keyword::DISTRIBUTE
            | Keyword::DISTRIBUTION
            | Keyword::DOCUMENT_P
            | Keyword::DOMAIN_P
            | Keyword::DOUBLE_P => KeywordCategory::Unreserved,
            Keyword::DROP
            | Keyword::DUMPFILE
            | Keyword::DUPLICATE
            | Keyword::EACH
            | Keyword::ELASTIC => KeywordCategory::Unreserved,
            Keyword::ENABLE_P
            | Keyword::ENCLOSED
            | Keyword::ENCODING
            | Keyword::ENCRYPTED
            | Keyword::ENCRYPTED_VALUE => KeywordCategory::Unreserved,
            Keyword::ENCRYPTION
            | Keyword::ENCRYPTION_TYPE
            | Keyword::ENDS
            | Keyword::ENFORCED
            | Keyword::ENTITYESCAPING
            | Keyword::ENUM_P => KeywordCategory::Unreserved,
            Keyword::EOL
            | Keyword::ERROR_P
            | Keyword::ERRORS
            | Keyword::ESCAPE
            | Keyword::ESCAPED => KeywordCategory::Unreserved,
            Keyword::ESCAPING
            | Keyword::EVALNAME
            | Keyword::EVENT
            | Keyword::EVENTS
            | Keyword::EVERY
            | Keyword::EXCHANGE => KeywordCategory::Unreserved,
            Keyword::EXCLUDE
            | Keyword::EXCLUDING
            | Keyword::EXCLUSIVE
            | Keyword::EXECUTE
            | Keyword::EXPIRED_P => KeywordCategory::Unreserved,
            Keyword::EXPLAIN
            | Keyword::EXTENSION
            | Keyword::EXTERNAL
            | Keyword::FAMILY
            | Keyword::FAST => KeywordCategory::Unreserved,
            Keyword::FEATURES
            | Keyword::FENCED
            | Keyword::FIELDS
            | Keyword::FILEHEADER_P
            | Keyword::FILL_MISSING_FIELDS => KeywordCategory::Unreserved,
            Keyword::FILLER
            | Keyword::FILTER
            | Keyword::FINAL
            | Keyword::FIRST_P
            | Keyword::FIXED_P => KeywordCategory::Unreserved,
            Keyword::FOLLOWING
            | Keyword::FOLLOWS_P
            | Keyword::FORCE
            | Keyword::FORMATTER
            | Keyword::FORWARD => KeywordCategory::Unreserved,
            Keyword::FUNCTION
            | Keyword::FUNCTIONS
            | Keyword::GENERATED
            | Keyword::GET
            | Keyword::GLOBAL
            | Keyword::GROUPS => KeywordCategory::Unreserved,
            Keyword::GRANTED
            | Keyword::HANDLER
            | Keyword::HEADER_P
            | Keyword::HOLD
            | Keyword::HOUR_P => KeywordCategory::Unreserved,
            Keyword::HOUR_MINUTE_P
            | Keyword::HOUR_SECOND_P
            | Keyword::IDENTIFIED
            | Keyword::IDENTITY_P
            | Keyword::IF_P
            | Keyword::IMCSTORED => KeywordCategory::Unreserved,
            Keyword::IGNORE
            | Keyword::IGNORE_EXTRA_DATA
            | Keyword::IMMEDIATE
            | Keyword::IMMUTABLE
            | Keyword::IMPLICIT_P => KeywordCategory::Unreserved,
            Keyword::INCLUDE
            | Keyword::INCLUDING
            | Keyword::INCREMENT
            | Keyword::INCREMENTAL
            | Keyword::INDEX => KeywordCategory::Unreserved,
            Keyword::INDEXES
            | Keyword::INFILE
            | Keyword::INFINITE_P
            | Keyword::INHERIT
            | Keyword::INHERITS => KeywordCategory::Unreserved,
            Keyword::INITIAL_P
            | Keyword::INITRANS
            | Keyword::INLINE_P
            | Keyword::INPUT_P
            | Keyword::INSENSITIVE => KeywordCategory::Unreserved,
            Keyword::INSERT
            | Keyword::INSTEAD
            | Keyword::INTERNAL
            | Keyword::INVISIBLE
            | Keyword::INVOKER => KeywordCategory::Unreserved,
            Keyword::IP | Keyword::ISNULL | Keyword::ISOLATION | Keyword::KEEP | Keyword::KEY => {
                KeywordCategory::Unreserved
            }
            Keyword::KEY_PATH
            | Keyword::KEY_STORE
            | Keyword::KILL
            | Keyword::LABEL
            | Keyword::LANGUAGE => KeywordCategory::Unreserved,
            Keyword::LARGE_P
            | Keyword::LAST_P
            | Keyword::LATERAL_P
            | Keyword::LC_COLLATE_P
            | Keyword::LC_CTYPE_P => KeywordCategory::Unreserved,
            Keyword::LEAKPROOF
            | Keyword::LEVEL
            | Keyword::LINES
            | Keyword::LIST
            | Keyword::LISTEN => KeywordCategory::Unreserved,
            Keyword::LOAD
            | Keyword::LOCAL
            | Keyword::LOCATION
            | Keyword::LOCK_P
            | Keyword::LOCKED => KeywordCategory::Unreserved,
            Keyword::LOG_P
            | Keyword::LOGGING
            | Keyword::LOGIN_ANY
            | Keyword::LOGIN_FAILURE
            | Keyword::LOGIN_SUCCESS => KeywordCategory::Unreserved,
            Keyword::LOGOUT
            | Keyword::LOOP
            | Keyword::MAP
            | Keyword::MAPPING
            | Keyword::MASKING => KeywordCategory::Unreserved,
            Keyword::MASTER
            | Keyword::MATCH
            | Keyword::MATCHED
            | Keyword::MATERIALIZED
            | Keyword::MAXEXTENTS => KeywordCategory::Unreserved,
            Keyword::MAXSIZE | Keyword::MAXTRANS | Keyword::MEMBER | Keyword::MERGE => {
                KeywordCategory::Unreserved
            }
            Keyword::MESSAGE_TEXT
            | Keyword::METHOD
            | Keyword::MINEXTENTS
            | Keyword::MINUTE_P
            | Keyword::MINUTE_SECOND_P => KeywordCategory::Unreserved,
            Keyword::MINVALUE
            | Keyword::MODE
            | Keyword::MODEL
            | Keyword::MONTH_P
            | Keyword::MOVE => KeywordCategory::Unreserved,
            Keyword::MOVEMENT
            | Keyword::MYSQL_ERRNO
            | Keyword::NAME_P
            | Keyword::NAMES
            | Keyword::NAN_P => KeywordCategory::Unreserved,
            Keyword::NEXT
            | Keyword::NO
            | Keyword::NOCOMPRESS
            | Keyword::NODE
            | Keyword::NOENTITYESCAPING
            | Keyword::NOLOGGING => KeywordCategory::Unreserved,
            Keyword::NOMAXVALUE
            | Keyword::NOMINVALUE
            | Keyword::NOTHING
            | Keyword::NOTIFY
            | Keyword::NOVALIDATE => KeywordCategory::Unreserved,
            Keyword::NOWAIT
            | Keyword::NULLCOLS
            | Keyword::NULLS_P
            | Keyword::NUMSTR
            | Keyword::OBJECT_P => KeywordCategory::Unreserved,
            Keyword::OF
            | Keyword::OFF
            | Keyword::OIDS
            | Keyword::OPERATOR
            | Keyword::OPTIMIZATION => KeywordCategory::Unreserved,
            Keyword::OPTION
            | Keyword::OPTIONALLY
            | Keyword::OPTIONS
            | Keyword::OUTFILE
            | Keyword::OVER => KeywordCategory::Unreserved,
            Keyword::OWNED
            | Keyword::OWNER
            | Keyword::PACKAGE
            | Keyword::PACKAGES
            | Keyword::PARALLEL_ENABLE => KeywordCategory::Unreserved,
            Keyword::PARSER
            | Keyword::PARTIAL
            | Keyword::PARTITION
            | Keyword::PARTITIONS
            | Keyword::PASSING => KeywordCategory::Unreserved,
            Keyword::PASSWORD
            | Keyword::PCTFREE
            | Keyword::PER_P
            | Keyword::PERCENT
            | Keyword::PERM => KeywordCategory::Unreserved,
            Keyword::PIPELINED
            | Keyword::PLAN
            | Keyword::PLANS
            | Keyword::POLICY
            | Keyword::POOL => KeywordCategory::Unreserved,
            Keyword::PRECEDES_P
            | Keyword::PRECEDING
            | Keyword::PREDICT
            | Keyword::PREFERRED
            | Keyword::PREFIX => KeywordCategory::Unreserved,
            Keyword::PREPARE
            | Keyword::PREPARED
            | Keyword::PRESERVE
            | Keyword::PRIOR
            | Keyword::PRIORER => KeywordCategory::Unreserved,
            Keyword::PRIVATE
            | Keyword::PRIVILEGE
            | Keyword::PRIVILEGES
            | Keyword::PROCEDURAL
            | Keyword::PROFILE
            | Keyword::PUBLIC => KeywordCategory::Unreserved,
            Keyword::PUBLICATION
            | Keyword::PUBLISH
            | Keyword::PURGE
            | Keyword::QUERY
            | Keyword::QUOTE => KeywordCategory::Unreserved,
            Keyword::RANDOMIZED
            | Keyword::RANGE
            | Keyword::RATIO
            | Keyword::RAW
            | Keyword::READ => KeywordCategory::Unreserved,
            Keyword::REASSIGN
            | Keyword::REBUILD
            | Keyword::RECHECK
            | Keyword::RECURSIVE
            | Keyword::REDISANYVALUE => KeywordCategory::Unreserved,
            Keyword::REF
            | Keyword::REFRESH
            | Keyword::REINDEX
            | Keyword::RELATIVE_P
            | Keyword::RELEASE => KeywordCategory::Unreserved,
            Keyword::RELOPTIONS
            | Keyword::REMOTE_P
            | Keyword::REMOVE
            | Keyword::RENAME
            | Keyword::REPEAT => KeywordCategory::Unreserved,
            Keyword::REPEATABLE
            | Keyword::REPLACE
            | Keyword::REPLICA
            | Keyword::RESET
            | Keyword::RESIZE => KeywordCategory::Unreserved,
            Keyword::RESOURCE
            | Keyword::RESPECT_P
            | Keyword::RESTART
            | Keyword::RESTRICT
            | Keyword::RESULT => KeywordCategory::Unreserved,
            Keyword::RESULT_CACHE
            | Keyword::RETURN
            | Keyword::RETURNED_SQLSTATE
            | Keyword::RETURNS
            | Keyword::REUSE => KeywordCategory::Unreserved,
            Keyword::REVOKE
            | Keyword::ROLE
            | Keyword::ROLES
            | Keyword::ROLLBACK
            | Keyword::ROLLUP => KeywordCategory::Unreserved,
            Keyword::ROTATE
            | Keyword::ROTATION
            | Keyword::ROW_COUNT
            | Keyword::ROWS
            | Keyword::ROWTYPE_P => KeywordCategory::Unreserved,
            Keyword::RULE
            | Keyword::SAMPLE
            | Keyword::SAVEPOINT
            | Keyword::SCHEDULE
            | Keyword::SCHEMA => KeywordCategory::Unreserved,
            Keyword::SCHEMA_NAME
            | Keyword::SCROLL
            | Keyword::SEARCH
            | Keyword::SECOND_P
            | Keyword::SECURITY => KeywordCategory::Unreserved,
            Keyword::SEPARATOR_P
            | Keyword::SEQUENCE
            | Keyword::SEQUENCES
            | Keyword::SERIALIZABLE
            | Keyword::SERVER => KeywordCategory::Unreserved,
            Keyword::SESSION
            | Keyword::SET
            | Keyword::SETS
            | Keyword::SHARE
            | Keyword::SHARE_MEMORY
            | Keyword::SHIPPABLE => KeywordCategory::Unreserved,
            Keyword::SHOW
            | Keyword::SHUTDOWN
            | Keyword::SIBLINGS
            | Keyword::SIMPLE
            | Keyword::SIZE => KeywordCategory::Unreserved,
            Keyword::SKIP
            | Keyword::SLAVE
            | Keyword::SLICE
            | Keyword::SMALLDATETIME_FORMAT_P
            | Keyword::SNAPSHOT => KeywordCategory::Unreserved,
            Keyword::SOURCE_P
            | Keyword::SPACE
            | Keyword::SPECIFICATION
            | Keyword::SPILL
            | Keyword::SPLIT => KeywordCategory::Unreserved,
            Keyword::SQL_P
            | Keyword::STABLE
            | Keyword::STACKED_P
            | Keyword::STANDALONE_P
            | Keyword::START => KeywordCategory::Unreserved,
            Keyword::STARTING
            | Keyword::STARTS
            | Keyword::STATEMENT
            | Keyword::STATEMENT_ID
            | Keyword::STATIC_P => KeywordCategory::Unreserved,
            Keyword::STATISTICS
            | Keyword::STDIN
            | Keyword::STDOUT
            | Keyword::STORAGE
            | Keyword::STORE_P => KeywordCategory::Unreserved,
            Keyword::STORED
            | Keyword::STRATIFY
            | Keyword::STREAM
            | Keyword::STRICT_P
            | Keyword::STRIP_P => KeywordCategory::Unreserved,
            Keyword::SUBCLASS_ORIGIN
            | Keyword::SUBPARTITION
            | Keyword::SUBPARTITIONS
            | Keyword::SUBSCRIPTION
            | Keyword::SYNONYM => KeywordCategory::Unreserved,
            Keyword::SYS_REFCURSOR
            | Keyword::SYSID
            | Keyword::SYSTEM_P
            | Keyword::TABLE_NAME
            | Keyword::TABLES => KeywordCategory::Unreserved,
            Keyword::TABLESPACE
            | Keyword::TARGET
            | Keyword::TEMP
            | Keyword::TEMPLATE
            | Keyword::TEMPORARY => KeywordCategory::Unreserved,
            Keyword::TERMINATED
            | Keyword::TEXT_P
            | Keyword::THAN
            | Keyword::TIES
            | Keyword::TIME_FORMAT_P => KeywordCategory::Unreserved,
            Keyword::TIMESTAMP_FORMAT_P
            | Keyword::TIMEZONE_HOUR_P
            | Keyword::TIMEZONE_MINUTE_P
            | Keyword::TRANSACTION
            | Keyword::TRANSFORM => KeywordCategory::Unreserved,
            Keyword::TRIGGER
            | Keyword::TRUNCATE
            | Keyword::TRUSTED
            | Keyword::TSFIELD
            | Keyword::TSTAG => KeywordCategory::Unreserved,
            Keyword::TSTIME
            | Keyword::TYPE_P
            | Keyword::TYPES_P
            | Keyword::UNBOUNDED
            | Keyword::UNCOMMITTED => KeywordCategory::Unreserved,
            Keyword::UNDER
            | Keyword::UNENCRYPTED
            | Keyword::UNIMCSTORED
            | Keyword::UNKNOWN
            | Keyword::UNLIMITED
            | Keyword::UNLISTEN => KeywordCategory::Unreserved,
            Keyword::UNLOCK
            | Keyword::UNLOGGED
            | Keyword::UNTIL
            | Keyword::UNUSABLE
            | Keyword::UPDATE => KeywordCategory::Unreserved,
            Keyword::USE_P
            | Keyword::USEEOF
            | Keyword::VACUUM
            | Keyword::VALID
            | Keyword::VALIDATE => KeywordCategory::Unreserved,
            Keyword::VALIDATION
            | Keyword::VALIDATOR
            | Keyword::VALUE_P
            | Keyword::VARIABLES
            | Keyword::VARRAY => KeywordCategory::Unreserved,
            Keyword::VARYING
            | Keyword::VCGROUP
            | Keyword::VERSION_P
            | Keyword::VIEW
            | Keyword::VISIBLE => KeywordCategory::Unreserved,
            Keyword::VOLATILE
            | Keyword::WAIT
            | Keyword::WARNINGS
            | Keyword::WEAK
            | Keyword::WELLFORMED
            | Keyword::WHILE_P => KeywordCategory::Unreserved,
            Keyword::WHITESPACE_P
            | Keyword::WITHIN
            | Keyword::WITHOUT
            | Keyword::WORK
            | Keyword::WORKLOAD => KeywordCategory::Unreserved,
            Keyword::WRAPPER
            | Keyword::WRITE
            | Keyword::XML_P
            | Keyword::YEAR_P
            | Keyword::YEAR_MONTH_P => KeywordCategory::Unreserved,
            Keyword::YES_P | Keyword::ZONE => KeywordCategory::Unreserved,
        }
    }
}

/// Lookup a keyword by its string representation (case-insensitive).
/// Uses a compile-time perfect hash function (phf) for O(1) lookup.
pub fn lookup_keyword(s: &str) -> Option<Keyword> {
    use phf::phf_map;
    static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
        "abort" => Keyword::ABORT_P,
        "absolute" => Keyword::ABSOLUTE_P,
        "access" => Keyword::ACCESS,
        "account" => Keyword::ACCOUNT,
        "action" => Keyword::ACTION,
        "add" => Keyword::ADD_P,
        "admin" => Keyword::ADMIN,
        "after" => Keyword::AFTER,
        "aggregate" => Keyword::AGGREGATE,
        "algorithm" => Keyword::ALGORITHM,
        "all" => Keyword::ALL,
        "also" => Keyword::ALSO,
        "alter" => Keyword::ALTER,
        "always" => Keyword::ALWAYS,
        "asensitive" => Keyword::ASENSITIVE,
        "analyse" => Keyword::ANALYSE,
        "analyze" => Keyword::ANALYZE,
        "and" => Keyword::AND,
        "any" => Keyword::ANY,
        "app" => Keyword::APP,
        "append" => Keyword::APPEND,
        "apply" => Keyword::APPLY,
        "archive" => Keyword::ARCHIVE,
        "array" => Keyword::ARRAY,
        "as" => Keyword::AS,
        "asc" => Keyword::ASC,
        "asof" => Keyword::ASOF_P,
        "assertion" => Keyword::ASSERTION,
        "assignment" => Keyword::ASSIGNMENT,
        "asymmetric" => Keyword::ASYMMETRIC,
        "at" => Keyword::AT,
        "attribute" => Keyword::ATTRIBUTE,
        "audit" => Keyword::AUDIT,
        "authid" => Keyword::AUTHID,
        "authorization" => Keyword::AUTHORIZATION,
        "auto_increment" => Keyword::AUTO_INCREMENT,
        "autoextend" => Keyword::AUTOEXTEND,
        "automapped" => Keyword::AUTOMAPPED,
        "backward" => Keyword::BACKWARD,
        "barrier" => Keyword::BARRIER,
        "before" => Keyword::BEFORE,
        "begin" => Keyword::BEGIN_P,
        "begin_non_anoyblock" => Keyword::BEGIN_NON_ANOYBLOCK,
        "between" => Keyword::BETWEEN,
        "bigint" => Keyword::BIGINT,
        "binary" => Keyword::BINARY,
        "binary_double" => Keyword::BINARY_DOUBLE,
        "binary_double_infinity" => Keyword::BINARY_DOUBLE_INF,
        "binary_double_nan" => Keyword::BINARY_DOUBLE_NAN,
        "binary_integer" => Keyword::BINARY_INTEGER,
        "bit" => Keyword::BIT,
        "blanks" => Keyword::BLANKS,
        "blob" => Keyword::BLOB_P,
        "blockchain" => Keyword::BLOCKCHAIN,
        "body" => Keyword::BODY_P,
        "boolean" => Keyword::BOOLEAN_P,
        "both" => Keyword::BOTH,
        "bucketcnt" => Keyword::BUCKETCNT,
        "buckets" => Keyword::BUCKETS,
        "build" => Keyword::BUILD,
        "by" => Keyword::BY,
        "byte" => Keyword::BYTE_P,
        "byteawithoutorder" => Keyword::BYTEAWITHOUTORDER,
        "byteawithoutorderwithequal" => Keyword::BYTEAWITHOUTORDERWITHEQUAL,
        "cache" => Keyword::CACHE,
        "call" => Keyword::CALL,
        "called" => Keyword::CALLED,
        "cancelable" => Keyword::CANCELABLE,
        "cascade" => Keyword::CASCADE,
        "cascaded" => Keyword::CASCADED,
        "case" => Keyword::CASE,
        "cast" => Keyword::CAST,
        "catalog" => Keyword::CATALOG_P,
        "catalog_name" => Keyword::CATALOG_NAME,
        "chain" => Keyword::CHAIN,
        "change" => Keyword::CHANGE,
        "char" => Keyword::CHAR_P,
        "character" => Keyword::CHARACTER,
        "characteristics" => Keyword::CHARACTERISTICS,
        "characterset" => Keyword::CHARACTERSET,
        "charset" => Keyword::CHARSET,
        "check" => Keyword::CHECK,
        "checkpoint" => Keyword::CHECKPOINT,
        "class" => Keyword::CLASS,
        "class_origin" => Keyword::CLASS_ORIGIN,
        "clean" => Keyword::CLEAN,
        "client" => Keyword::CLIENT,
        "client_master_key" => Keyword::CLIENT_MASTER_KEY,
        "client_master_keys" => Keyword::CLIENT_MASTER_KEYS,
        "clob" => Keyword::CLOB,
        "close" => Keyword::CLOSE,
        "cluster" => Keyword::CLUSTER,
        "coalesce" => Keyword::COALESCE,
        "collate" => Keyword::COLLATE,
        "collation" => Keyword::COLLATION,
        "column" => Keyword::COLUMN,
        "column_encryption_key" => Keyword::COLUMN_ENCRYPTION_KEY,
        "column_encryption_keys" => Keyword::COLUMN_ENCRYPTION_KEYS,
        "column_name" => Keyword::COLUMN_NAME,
        "columns" => Keyword::COLUMNS,
        "comment" => Keyword::COMMENT,
        "comments" => Keyword::COMMENTS,
        "commit" => Keyword::COMMIT,
        "committed" => Keyword::COMMITTED,
        "compact" => Keyword::COMPACT,
        "compatible_illegal_chars" => Keyword::COMPATIBLE_ILLEGAL_CHARS,
        "compile" => Keyword::COMPILE,
        "complete" => Keyword::COMPLETE,
        "completion" => Keyword::COMPLETION,
        "compress" => Keyword::COMPRESS,
        "concurrently" => Keyword::CONCURRENTLY,
        "condition" => Keyword::CONDITION,
        "configuration" => Keyword::CONFIGURATION,
        "conflict" => Keyword::CONFLICT,
        "connect" => Keyword::CONNECT,
        "connection" => Keyword::CONNECTION,
        "consistent" => Keyword::CONSISTENT,
        "constant" => Keyword::CONSTANT,
        "constraint" => Keyword::CONSTRAINT,
        "constraint_catalog" => Keyword::CONSTRAINT_CATALOG,
        "constraint_name" => Keyword::CONSTRAINT_NAME,
        "constraint_schema" => Keyword::CONSTRAINT_SCHEMA,
        "constraints" => Keyword::CONSTRAINTS,
        "constructor" => Keyword::CONSTRUCTOR,
        "content" => Keyword::CONTENT_P,
        "continue" => Keyword::CONTINUE_P,
        "contview" => Keyword::CONTVIEW,
        "conversion" => Keyword::CONVERSION_P,
        "convert" => Keyword::CONVERT_P,
        "coordinator" => Keyword::COORDINATOR,
        "coordinators" => Keyword::COORDINATORS,
        "copy" => Keyword::COPY,
        "cost" => Keyword::COST,
        "create" => Keyword::CREATE,
        "cross" => Keyword::CROSS,
        "csn" => Keyword::CSN,
        "csv" => Keyword::CSV,
        "cube" => Keyword::CUBE,
        "current" => Keyword::CURRENT_P,
        "current_catalog" => Keyword::CURRENT_CATALOG,
        "current_date" => Keyword::CURRENT_DATE,
        "current_role" => Keyword::CURRENT_ROLE,
        "current_schema" => Keyword::CURRENT_SCHEMA,
        "current_time" => Keyword::CURRENT_TIME,
        "current_timestamp" => Keyword::CURRENT_TIMESTAMP,
        "current_user" => Keyword::CURRENT_USER,
        "cursor" => Keyword::CURSOR,
        "cursor_name" => Keyword::CURSOR_NAME,
        "cycle" => Keyword::CYCLE,
        "data" => Keyword::DATA_P,
        "database" => Keyword::DATABASE,
        "datafile" => Keyword::DATAFILE,
        "datanode" => Keyword::DATANODE,
        "datanodes" => Keyword::DATANODES,
        "datatype_cl" => Keyword::DATATYPE_CL,
        "date" => Keyword::DATE_P,
        "date_format" => Keyword::DATE_FORMAT_P,
        "day" => Keyword::DAY_P,
        "day_hour" => Keyword::DAY_HOUR_P,
        "day_minute" => Keyword::DAY_MINUTE_P,
        "day_second" => Keyword::DAY_SECOND_P,
        "dbcompatibility" => Keyword::DBCOMPATIBILITY_P,
        "deallocate" => Keyword::DEALLOCATE,
        "dec" => Keyword::DEC,
        "decimal" => Keyword::DECIMAL_P,
        "declare" => Keyword::DECLARE,
        "decode" => Keyword::DECODE,
        "default" => Keyword::DEFAULT,
        "defaults" => Keyword::DEFAULTS,
        "deferrable" => Keyword::DEFERRABLE,
        "deferred" => Keyword::DEFERRED,
        "definer" => Keyword::DEFINER,
        "delete" => Keyword::DELETE_P,
        "delimiter" => Keyword::DELIMITER,
        "delimiters" => Keyword::DELIMITERS,
        "delta" => Keyword::DELTA,
        "deltamerge" => Keyword::DELTAMERGE,
        "dense_rank" => Keyword::DENSE_RANK,
        "desc" => Keyword::DESC,
        "deterministic" => Keyword::DETERMINISTIC,
        "diagnostics" => Keyword::DIAGNOSTICS,
        "dictionary" => Keyword::DICTIONARY,
        "direct" => Keyword::DIRECT,
        "directory" => Keyword::DIRECTORY,
        "disable" => Keyword::DISABLE_P,
        "discard" => Keyword::DISCARD,
        "disconnect" => Keyword::DISCONNECT,
        "distinct" => Keyword::DISTINCT,
        "distribute" => Keyword::DISTRIBUTE,
        "distribution" => Keyword::DISTRIBUTION,
        "do" => Keyword::DO,
        "document" => Keyword::DOCUMENT_P,
        "domain" => Keyword::DOMAIN_P,
        "double" => Keyword::DOUBLE_P,
        "drop" => Keyword::DROP,
        "dumpfile" => Keyword::DUMPFILE,
        "duplicate" => Keyword::DUPLICATE,
        "each" => Keyword::EACH,
        "elastic" => Keyword::ELASTIC,
        "else" => Keyword::ELSE,
        "enable" => Keyword::ENABLE_P,
        "enclosed" => Keyword::ENCLOSED,
        "encoding" => Keyword::ENCODING,
        "encrypted" => Keyword::ENCRYPTED,
        "encrypted_value" => Keyword::ENCRYPTED_VALUE,
        "encryption" => Keyword::ENCRYPTION,
        "encryption_type" => Keyword::ENCRYPTION_TYPE,
        "end" => Keyword::END_P,
        "ends" => Keyword::ENDS,
        "enforced" => Keyword::ENFORCED,
        "entityescaping" => Keyword::ENTITYESCAPING,
        "enum" => Keyword::ENUM_P,
        "eol" => Keyword::EOL,
        "error" => Keyword::ERROR_P,
        "errors" => Keyword::ERRORS,
        "escape" => Keyword::ESCAPE,
        "escaped" => Keyword::ESCAPED,
        "escaping" => Keyword::ESCAPING,
        "evalname" => Keyword::EVALNAME,
        "event" => Keyword::EVENT,
        "events" => Keyword::EVENTS,
        "every" => Keyword::EVERY,
        "except" => Keyword::EXCEPT,
        "exchange" => Keyword::EXCHANGE,
        "exclude" => Keyword::EXCLUDE,
        "excluded" => Keyword::EXCLUDED,
        "excluding" => Keyword::EXCLUDING,
        "exclusive" => Keyword::EXCLUSIVE,
        "execute" => Keyword::EXECUTE,
        "exists" => Keyword::EXISTS,
        "expired" => Keyword::EXPIRED_P,
        "explain" => Keyword::EXPLAIN,
        "extension" => Keyword::EXTENSION,
        "external" => Keyword::EXTERNAL,
        "extract" => Keyword::EXTRACT,
        "false" => Keyword::FALSE_P,
        "family" => Keyword::FAMILY,
        "fast" => Keyword::FAST,
        "features" => Keyword::FEATURES,
        "fenced" => Keyword::FENCED,
        "fetch" => Keyword::FETCH,
        "fields" => Keyword::FIELDS,
        "fileheader" => Keyword::FILEHEADER_P,
        "fill_missing_fields" => Keyword::FILL_MISSING_FIELDS,
        "filler" => Keyword::FILLER,
        "filter" => Keyword::FILTER,
        "final" => Keyword::FINAL,
        "first" => Keyword::FIRST_P,
        "fixed" => Keyword::FIXED_P,
        "float" => Keyword::FLOAT_P,
        "following" => Keyword::FOLLOWING,
        "follows" => Keyword::FOLLOWS_P,
        "for" => Keyword::FOR,
        "force" => Keyword::FORCE,
        "foreign" => Keyword::FOREIGN,
        "formatter" => Keyword::FORMATTER,
        "forward" => Keyword::FORWARD,
        "freeze" => Keyword::FREEZE,
        "from" => Keyword::FROM,
        "full" => Keyword::FULL,
        "function" => Keyword::FUNCTION,
        "functions" => Keyword::FUNCTIONS,
        "generated" => Keyword::GENERATED,
        "get" => Keyword::GET,
        "global" => Keyword::GLOBAL,
        "grant" => Keyword::GRANT,
        "granted" => Keyword::GRANTED,
        "greatest" => Keyword::GREATEST,
        "group" => Keyword::GROUP_P,
        "grouping" => Keyword::GROUPING_P,
        "groupparent" => Keyword::GROUPPARENT,
        "groups" => Keyword::GROUPS,
        "handler" => Keyword::HANDLER,
        "having" => Keyword::HAVING,
        "hdfsdirectory" => Keyword::HDFSDIRECTORY,
        "header" => Keyword::HEADER_P,
        "hold" => Keyword::HOLD,
        "hour" => Keyword::HOUR_P,
        "hour_minute" => Keyword::HOUR_MINUTE_P,
        "hour_second" => Keyword::HOUR_SECOND_P,
        "identified" => Keyword::IDENTIFIED,
        "identity" => Keyword::IDENTITY_P,
        "if" => Keyword::IF_P,
        "ignore" => Keyword::IGNORE,
        "ignore_extra_data" => Keyword::IGNORE_EXTRA_DATA,
        "ilike" => Keyword::ILIKE,
        "imcstored" => Keyword::IMCSTORED,
        "immediate" => Keyword::IMMEDIATE,
        "immutable" => Keyword::IMMUTABLE,
        "implicit" => Keyword::IMPLICIT_P,
        "in" => Keyword::IN_P,
        "include" => Keyword::INCLUDE,
        "including" => Keyword::INCLUDING,
        "increment" => Keyword::INCREMENT,
        "incremental" => Keyword::INCREMENTAL,
        "index" => Keyword::INDEX,
        "indexes" => Keyword::INDEXES,
        "infile" => Keyword::INFILE,
        "infinite" => Keyword::INFINITE_P,
        "inherit" => Keyword::INHERIT,
        "inherits" => Keyword::INHERITS,
        "initial" => Keyword::INITIAL_P,
        "initially" => Keyword::INITIALLY,
        "initrans" => Keyword::INITRANS,
        "inline" => Keyword::INLINE_P,
        "inner" => Keyword::INNER_P,
        "inout" => Keyword::INOUT,
        "input" => Keyword::INPUT_P,
        "insensitive" => Keyword::INSENSITIVE,
        "insert" => Keyword::INSERT,
        "instead" => Keyword::INSTEAD,
        "int" => Keyword::INT_P,
        "integer" => Keyword::INTEGER,
        "internal" => Keyword::INTERNAL,
        "intersect" => Keyword::INTERSECT,
        "interval" => Keyword::INTERVAL,
        "into" => Keyword::INTO,
        "invisible" => Keyword::INVISIBLE,
        "invoker" => Keyword::INVOKER,
        "ip" => Keyword::IP,
        "is" => Keyword::IS,
        "isnull" => Keyword::ISNULL,
        "isolation" => Keyword::ISOLATION,
        "join" => Keyword::JOIN,
        "json_exists" => Keyword::JSON_EXISTS,
        "keep" => Keyword::KEEP,
        "key" => Keyword::KEY,
        "key_path" => Keyword::KEY_PATH,
        "key_store" => Keyword::KEY_STORE,
        "kill" => Keyword::KILL,
        "label" => Keyword::LABEL,
        "language" => Keyword::LANGUAGE,
        "large" => Keyword::LARGE_P,
        "last" => Keyword::LAST_P,
        "lateral" => Keyword::LATERAL_P,
        "lc_collate" => Keyword::LC_COLLATE_P,
        "lc_ctype" => Keyword::LC_CTYPE_P,
        "leading" => Keyword::LEADING,
        "leakproof" => Keyword::LEAKPROOF,
        "least" => Keyword::LEAST,
        "left" => Keyword::LEFT,
        "less" => Keyword::LESS,
        "level" => Keyword::LEVEL,
        "like" => Keyword::LIKE,
        "limit" => Keyword::LIMIT,
        "lines" => Keyword::LINES,
        "list" => Keyword::LIST,
        "listen" => Keyword::LISTEN,
        "load" => Keyword::LOAD,
        "local" => Keyword::LOCAL,
        "localtime" => Keyword::LOCALTIME,
        "localtimestamp" => Keyword::LOCALTIMESTAMP,
        "location" => Keyword::LOCATION,
        "lock" => Keyword::LOCK_P,
        "locked" => Keyword::LOCKED,
        "log" => Keyword::LOG_P,
        "logging" => Keyword::LOGGING,
        "login_any" => Keyword::LOGIN_ANY,
        "login_failure" => Keyword::LOGIN_FAILURE,
        "login_success" => Keyword::LOGIN_SUCCESS,
        "logout" => Keyword::LOGOUT,
        "loop" => Keyword::LOOP,
        "map" => Keyword::MAP,
        "mapping" => Keyword::MAPPING,
        "masking" => Keyword::MASKING,
        "master" => Keyword::MASTER,
        "match" => Keyword::MATCH,
        "matched" => Keyword::MATCHED,
        "materialized" => Keyword::MATERIALIZED,
        "maxextents" => Keyword::MAXEXTENTS,
        "maxsize" => Keyword::MAXSIZE,
        "maxtrans" => Keyword::MAXTRANS,
        "maxvalue" => Keyword::MAXVALUE,
        "member" => Keyword::MEMBER,
        "merge" => Keyword::MERGE,
        "message_text" => Keyword::MESSAGE_TEXT,
        "method" => Keyword::METHOD,
        "minextents" => Keyword::MINEXTENTS,
        "minus" => Keyword::MINUS_P,
        "minute" => Keyword::MINUTE_P,
        "minute_second" => Keyword::MINUTE_SECOND_P,
        "minvalue" => Keyword::MINVALUE,
        "mode" => Keyword::MODE,
        "model" => Keyword::MODEL,
        "modify" => Keyword::MODIFY_P,
        "month" => Keyword::MONTH_P,
        "move" => Keyword::MOVE,
        "movement" => Keyword::MOVEMENT,
        "mysql_errno" => Keyword::MYSQL_ERRNO,
        "name" => Keyword::NAME_P,
        "names" => Keyword::NAMES,
        "nan" => Keyword::NAN_P,
        "national" => Keyword::NATIONAL,
        "natural" => Keyword::NATURAL,
        "nchar" => Keyword::NCHAR,
        "next" => Keyword::NEXT,
        "no" => Keyword::NO,
        "nocompress" => Keyword::NOCOMPRESS,
        "nocycle" => Keyword::NOCYCLE,
        "node" => Keyword::NODE,
        "noentityescaping" => Keyword::NOENTITYESCAPING,
        "nologging" => Keyword::NOLOGGING,
        "nomaxvalue" => Keyword::NOMAXVALUE,
        "nominvalue" => Keyword::NOMINVALUE,
        "none" => Keyword::NONE,
        "not" => Keyword::NOT,
        "nothing" => Keyword::NOTHING,
        "notify" => Keyword::NOTIFY,
        "notnull" => Keyword::NOTNULL,
        "novalidate" => Keyword::NOVALIDATE,
        "nowait" => Keyword::NOWAIT,
        "nth_value" => Keyword::NTH_VALUE_P,
        "null" => Keyword::NULL_P,
        "nullcols" => Keyword::NULLCOLS,
        "nullif" => Keyword::NULLIF,
        "nulls" => Keyword::NULLS_P,
        "number" => Keyword::NUMBER_P,
        "numeric" => Keyword::NUMERIC,
        "numstr" => Keyword::NUMSTR,
        "nvarchar" => Keyword::NVARCHAR,
        "nvarchar2" => Keyword::NVARCHAR2,
        "nvl" => Keyword::NVL,
        "object" => Keyword::OBJECT_P,
        "of" => Keyword::OF,
        "off" => Keyword::OFF,
        "offset" => Keyword::OFFSET,
        "oids" => Keyword::OIDS,
        "on" => Keyword::ON,
        "only" => Keyword::ONLY,
        "operator" => Keyword::OPERATOR,
        "optimization" => Keyword::OPTIMIZATION,
        "option" => Keyword::OPTION,
        "optionally" => Keyword::OPTIONALLY,
        "options" => Keyword::OPTIONS,
        "or" => Keyword::OR,
        "order" => Keyword::ORDER,
        "out" => Keyword::OUT_P,
        "outer" => Keyword::OUTER_P,
        "outfile" => Keyword::OUTFILE,
        "over" => Keyword::OVER,
        "overlaps" => Keyword::OVERLAPS,
        "overlay" => Keyword::OVERLAY,
        "owned" => Keyword::OWNED,
        "owner" => Keyword::OWNER,
        "package" => Keyword::PACKAGE,
        "packages" => Keyword::PACKAGES,
        "parallel_enable" => Keyword::PARALLEL_ENABLE,
        "parser" => Keyword::PARSER,
        "partial" => Keyword::PARTIAL,
        "partition" => Keyword::PARTITION,
        "partitions" => Keyword::PARTITIONS,
        "passing" => Keyword::PASSING,
        "password" => Keyword::PASSWORD,
        "pctfree" => Keyword::PCTFREE,
        "per" => Keyword::PER_P,
        "percent" => Keyword::PERCENT,
        "performance" => Keyword::PERFORMANCE,
        "perm" => Keyword::PERM,
        "pipelined" => Keyword::PIPELINED,
        "placing" => Keyword::PLACING,
        "plan" => Keyword::PLAN,
        "plans" => Keyword::PLANS,
        "policy" => Keyword::POLICY,
        "pool" => Keyword::POOL,
        "position" => Keyword::POSITION,
        "precedes" => Keyword::PRECEDES_P,
        "preceding" => Keyword::PRECEDING,
        "precision" => Keyword::PRECISION,
        "predict" => Keyword::PREDICT,
        "preferred" => Keyword::PREFERRED,
        "prefix" => Keyword::PREFIX,
        "prepare" => Keyword::PREPARE,
        "prepared" => Keyword::PREPARED,
        "preserve" => Keyword::PRESERVE,
        "primary" => Keyword::PRIMARY,
        "prior" => Keyword::PRIOR,
        "priorer" => Keyword::PRIORER,
        "private" => Keyword::PRIVATE,
        "privilege" => Keyword::PRIVILEGE,
        "privileges" => Keyword::PRIVILEGES,
        "procedural" => Keyword::PROCEDURAL,
        "procedure" => Keyword::PROCEDURE,
        "profile" => Keyword::PROFILE,
        "public" => Keyword::PUBLIC,
        "publication" => Keyword::PUBLICATION,
        "publish" => Keyword::PUBLISH,
        "purge" => Keyword::PURGE,
        "query" => Keyword::QUERY,
        "quote" => Keyword::QUOTE,
        "randomized" => Keyword::RANDOMIZED,
        "range" => Keyword::RANGE,
        "ratio" => Keyword::RATIO,
        "raw" => Keyword::RAW,
        "read" => Keyword::READ,
        "real" => Keyword::REAL,
        "reassign" => Keyword::REASSIGN,
        "rebuild" => Keyword::REBUILD,
        "recheck" => Keyword::RECHECK,
        "recursive" => Keyword::RECURSIVE,
        "recyclebin" => Keyword::RECYCLEBIN,
        "redisanyvalue" => Keyword::REDISANYVALUE,
        "ref" => Keyword::REF,
        "references" => Keyword::REFERENCES,
        "refresh" => Keyword::REFRESH,
        "reindex" => Keyword::REINDEX,
        "reject" => Keyword::REJECT_P,
        "relative" => Keyword::RELATIVE_P,
        "release" => Keyword::RELEASE,
        "reloptions" => Keyword::RELOPTIONS,
        "remote" => Keyword::REMOTE_P,
        "remove" => Keyword::REMOVE,
        "rename" => Keyword::RENAME,
        "repeat" => Keyword::REPEAT,
        "repeatable" => Keyword::REPEATABLE,
        "replace" => Keyword::REPLACE,
        "replica" => Keyword::REPLICA,
        "reset" => Keyword::RESET,
        "resize" => Keyword::RESIZE,
        "resource" => Keyword::RESOURCE,
        "respect" => Keyword::RESPECT_P,
        "restart" => Keyword::RESTART,
        "restrict" => Keyword::RESTRICT,
        "result" => Keyword::RESULT,
        "result_cache" => Keyword::RESULT_CACHE,
        "return" => Keyword::RETURN,
        "returned_sqlstate" => Keyword::RETURNED_SQLSTATE,
        "returning" => Keyword::RETURNING,
        "returns" => Keyword::RETURNS,
        "reuse" => Keyword::REUSE,
        "revoke" => Keyword::REVOKE,
        "right" => Keyword::RIGHT,
        "role" => Keyword::ROLE,
        "roles" => Keyword::ROLES,
        "rollback" => Keyword::ROLLBACK,
        "rollup" => Keyword::ROLLUP,
        "rotate" => Keyword::ROTATE,
        "rotation" => Keyword::ROTATION,
        "row" => Keyword::ROW,
        "row_count" => Keyword::ROW_COUNT,
        "rownum" => Keyword::ROWNUM,
        "rows" => Keyword::ROWS,
        "rowtype" => Keyword::ROWTYPE_P,
        "rule" => Keyword::RULE,
        "sample" => Keyword::SAMPLE,
        "savepoint" => Keyword::SAVEPOINT,
        "schedule" => Keyword::SCHEDULE,
        "schema" => Keyword::SCHEMA,
        "schema_name" => Keyword::SCHEMA_NAME,
        "scroll" => Keyword::SCROLL,
        "search" => Keyword::SEARCH,
        "second" => Keyword::SECOND_P,
        "security" => Keyword::SECURITY,
        "select" => Keyword::SELECT,
        "separator" => Keyword::SEPARATOR_P,
        "sequence" => Keyword::SEQUENCE,
        "sequences" => Keyword::SEQUENCES,
        "serializable" => Keyword::SERIALIZABLE,
        "server" => Keyword::SERVER,
        "session" => Keyword::SESSION,
        "session_user" => Keyword::SESSION_USER,
        "set" => Keyword::SET,
        "setof" => Keyword::SETOF,
        "sets" => Keyword::SETS,
        "share" => Keyword::SHARE,
        "share_memory" => Keyword::SHARE_MEMORY,
        "shippable" => Keyword::SHIPPABLE,
        "show" => Keyword::SHOW,
        "shrink" => Keyword::SHRINK,
        "shutdown" => Keyword::SHUTDOWN,
        "siblings" => Keyword::SIBLINGS,
        "similar" => Keyword::SIMILAR,
        "simple" => Keyword::SIMPLE,
        "size" => Keyword::SIZE,
        "skip" => Keyword::SKIP,
        "slave" => Keyword::SLAVE,
        "slice" => Keyword::SLICE,
        "smalldatetime" => Keyword::SMALLDATETIME,
        "smalldatetime_format" => Keyword::SMALLDATETIME_FORMAT_P,
        "smallint" => Keyword::SMALLINT,
        "snapshot" => Keyword::SNAPSHOT,
        "some" => Keyword::SOME,
        "source" => Keyword::SOURCE_P,
        "space" => Keyword::SPACE,
        "specification" => Keyword::SPECIFICATION,
        "spill" => Keyword::SPILL,
        "split" => Keyword::SPLIT,
        "sql" => Keyword::SQL_P,
        "stable" => Keyword::STABLE,
        "stacked" => Keyword::STACKED_P,
        "standalone" => Keyword::STANDALONE_P,
        "start" => Keyword::START,
        "starting" => Keyword::STARTING,
        "starts" => Keyword::STARTS,
        "statement" => Keyword::STATEMENT,
        "statement_id" => Keyword::STATEMENT_ID,
        "static" => Keyword::STATIC_P,
        "statistics" => Keyword::STATISTICS,
        "stdin" => Keyword::STDIN,
        "stdout" => Keyword::STDOUT,
        "storage" => Keyword::STORAGE,
        "store" => Keyword::STORE_P,
        "stored" => Keyword::STORED,
        "stratify" => Keyword::STRATIFY,
        "stream" => Keyword::STREAM,
        "strict" => Keyword::STRICT_P,
        "strip" => Keyword::STRIP_P,
        "subclass_origin" => Keyword::SUBCLASS_ORIGIN,
        "subpartition" => Keyword::SUBPARTITION,
        "subpartitions" => Keyword::SUBPARTITIONS,
        "subscription" => Keyword::SUBSCRIPTION,
        "substring" => Keyword::SUBSTRING,
        "symmetric" => Keyword::SYMMETRIC,
        "synonym" => Keyword::SYNONYM,
        "sys_refcursor" => Keyword::SYS_REFCURSOR,
        "sysdate" => Keyword::SYSDATE,
        "sysid" => Keyword::SYSID,
        "system" => Keyword::SYSTEM_P,
        "table" => Keyword::TABLE,
        "table_name" => Keyword::TABLE_NAME,
        "tables" => Keyword::TABLES,
        "tablesample" => Keyword::TABLESAMPLE,
        "tablespace" => Keyword::TABLESPACE,
        "target" => Keyword::TARGET,
        "temp" => Keyword::TEMP,
        "template" => Keyword::TEMPLATE,
        "temporary" => Keyword::TEMPORARY,
        "terminated" => Keyword::TERMINATED,
        "text" => Keyword::TEXT_P,
        "than" => Keyword::THAN,
        "then" => Keyword::THEN,
        "ties" => Keyword::TIES,
        "time" => Keyword::TIME,
        "time_format" => Keyword::TIME_FORMAT_P,
        "timecapsule" => Keyword::TIMECAPSULE,
        "timestamp" => Keyword::TIMESTAMP,
        "timestamp_format" => Keyword::TIMESTAMP_FORMAT_P,
        "timestampdiff" => Keyword::TIMESTAMPDIFF,
        "timezone_hour" => Keyword::TIMEZONE_HOUR_P,
        "timezone_minute" => Keyword::TIMEZONE_MINUTE_P,
        "tinyint" => Keyword::TINYINT,
        "to" => Keyword::TO,
        "trailing" => Keyword::TRAILING,
        "transaction" => Keyword::TRANSACTION,
        "transform" => Keyword::TRANSFORM,
        "treat" => Keyword::TREAT,
        "trigger" => Keyword::TRIGGER,
        "trim" => Keyword::TRIM,
        "true" => Keyword::TRUE_P,
        "truncate" => Keyword::TRUNCATE,
        "trusted" => Keyword::TRUSTED,
        "tsfield" => Keyword::TSFIELD,
        "tstag" => Keyword::TSTAG,
        "tstime" => Keyword::TSTIME,
        "type" => Keyword::TYPE_P,
        "types" => Keyword::TYPES_P,
        "unbounded" => Keyword::UNBOUNDED,
        "uncommitted" => Keyword::UNCOMMITTED,
        "under" => Keyword::UNDER,
        "unencrypted" => Keyword::UNENCRYPTED,
        "unimcstored" => Keyword::UNIMCSTORED,
        "union" => Keyword::UNION,
        "unique" => Keyword::UNIQUE,
        "unknown" => Keyword::UNKNOWN,
        "unlimited" => Keyword::UNLIMITED,
        "unlisten" => Keyword::UNLISTEN,
        "unlock" => Keyword::UNLOCK,
        "unlogged" => Keyword::UNLOGGED,
        "until" => Keyword::UNTIL,
        "unusable" => Keyword::UNUSABLE,
        "update" => Keyword::UPDATE,
        "use" => Keyword::USE_P,
        "useeof" => Keyword::USEEOF,
        "user" => Keyword::USER,
        "using" => Keyword::USING,
        "vacuum" => Keyword::VACUUM,
        "valid" => Keyword::VALID,
        "validate" => Keyword::VALIDATE,
        "validation" => Keyword::VALIDATION,
        "validator" => Keyword::VALIDATOR,
        "value" => Keyword::VALUE_P,
        "values" => Keyword::VALUES,
        "varchar" => Keyword::VARCHAR,
        "varchar2" => Keyword::VARCHAR2,
        "variables" => Keyword::VARIABLES,
        "variadic" => Keyword::VARIADIC,
        "varray" => Keyword::VARRAY,
        "varying" => Keyword::VARYING,
        "vcgroup" => Keyword::VCGROUP,
        "verbose" => Keyword::VERBOSE,
        "verify" => Keyword::VERIFY,
        "version" => Keyword::VERSION_P,
        "view" => Keyword::VIEW,
        "visible" => Keyword::VISIBLE,
        "volatile" => Keyword::VOLATILE,
        "wait" => Keyword::WAIT,
        "warnings" => Keyword::WARNINGS,
        "weak" => Keyword::WEAK,
        "wellformed" => Keyword::WELLFORMED,
        "when" => Keyword::WHEN,
        "where" => Keyword::WHERE,
        "while" => Keyword::WHILE_P,
        "whitespace" => Keyword::WHITESPACE_P,
        "window" => Keyword::WINDOW,
        "with" => Keyword::WITH,
        "within" => Keyword::WITHIN,
        "without" => Keyword::WITHOUT,
        "work" => Keyword::WORK,
        "workload" => Keyword::WORKLOAD,
        "wrapper" => Keyword::WRAPPER,
        "write" => Keyword::WRITE,
        "xml" => Keyword::XML_P,
        "xmlattributes" => Keyword::XMLATTRIBUTES,
        "xmlconcat" => Keyword::XMLCONCAT,
        "xmlelement" => Keyword::XMLELEMENT,
        "xmlexists" => Keyword::XMLEXISTS,
        "xmlforest" => Keyword::XMLFOREST,
        "xmlparse" => Keyword::XMLPARSE,
        "xmlpi" => Keyword::XMLPI,
        "xmlroot" => Keyword::XMLROOT,
        "xmlserialize" => Keyword::XMLSERIALIZE,
        "year" => Keyword::YEAR_P,
        "year_month" => Keyword::YEAR_MONTH_P,
        "yes" => Keyword::YES_P,
        "zone" => Keyword::ZONE,
    };

    // Fast path: input is already all lowercase ASCII
    if s.is_ascii() && s.bytes().all(|b| b.is_ascii_lowercase()) {
        return KEYWORDS.get(s).copied();
    }
    // General path: lowercase then lookup
    KEYWORDS.get(&*s.to_ascii_lowercase()).copied()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeywordCategory {
    Reserved,
    ColName,
    TypeFuncName,
    Unreserved,
}
