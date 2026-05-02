//! Constants used across the Java SQL extraction module.

pub(super) const STRING_BUILDER_TYPES: &[&str] = &["StringBuilder", "StringBuffer"];

pub(super) const SQL_ANNOTATIONS: &[&str] = &[
    "Query",
    "NamedQuery",
    "SqlUpdate",
    "SqlQuery",
    "Modifying",
    "Select",
    "Insert",
    "Update",
    "Delete",
    "NamedNativeQuery",
    "SqlBatch",
];

pub(super) const SQL_METHOD_UNAMBIGUOUS: &[&str] = &[
    "createNativeQuery",
    "createQuery",
    "prepareStatement",
    "prepareCall",
    "executeQuery",
    "executeUpdate",
    "executeProcedure",
    "queryForObject",
    "queryForList",
    "queryForMap",
    "batchUpdate",
];

pub(super) const SQL_METHOD_AMBIGUOUS: &[&str] = &["query", "update", "execute"];

pub(super) const SQL_KEYWORDS: &[&str] = &[
    "SELECT ",
    "INSERT ",
    "UPDATE ",
    "DELETE ",
    "WITH ",
    "CREATE ",
    "ALTER ",
    "DROP ",
    "MERGE ",
    "TRUNCATE ",
    "CALL ",
    "{CALL ",
];

pub(super) const SQL_NAME_PATTERN: &str = "SQL";

pub(super) const SQL_STATEMENT_PREFIXES: &[&str] = &[
    "SELECT ",
    "INSERT ",
    "UPDATE ",
    "DELETE ",
    "WITH ",
    "CREATE ",
    "ALTER ",
    "DROP ",
    "MERGE ",
    "TRUNCATE ",
    "CALL ",
    "select ",
    "insert ",
    "update ",
    "delete ",
    "with ",
    "create ",
    "alter ",
    "drop ",
    "merge ",
    "truncate ",
    "call ",
];
