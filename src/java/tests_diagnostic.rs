//! Diagnostic test suite for parse-java improvement analysis.
//!
//! Covers all 10 improvement suggestions from the review, organized by category.
//! Tests that currently fail are marked with improvement-needed comments.
//! Run with: cargo test --features java tests_diagnostic -- --nocapture

use crate::java::{extract_sql_from_java, extract_sql_from_java_files, ExtractionMethod, JavaExtractConfig};

// ═══════════════════════════════════════════════════════════════
// GROUP A: looks_like_sql edge cases (Suggestion 6)
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_a1_select_box_not_sql() {
    let java = r#"
        public class Config {
            private static final String LABEL = "SELECT_BOX";
        }
    "#;
    let result = extract_sql_from_java(java, "Config.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 0,
        "SELECT_BOX is not SQL, got: {:?}", result.extractions);
}

#[test]
fn diag_a2_select_newline_false_negative() {
    let java = r#"
        public class Dao {
            public void query() {
                String sql = "select\n* from users";
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1,
        "newline-separated SQL should be detected; needs whitespace normalization in looks_like_sql");
    assert!(result.extractions[0].sql.contains("from users"));
}

#[test]
fn diag_a3_select_tab_separated() {
    let java = r#"
        public class Dao {
            public void query() {
                String sql = "SELECT\t* FROM users";
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1,
        "tab-separated SQL should be detected; needs whitespace normalization in looks_like_sql");
    assert!(result.extractions[0].sql.contains("FROM users"));
}

#[test]
fn diag_a4_english_text_not_sql() {
    let java = r#"
        public class UI {
            private String prompt = "Please SELECT one option from the menu";
        }
    "#;
    let result = extract_sql_from_java(java, "UI.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 0,
        "English prose with SQL keyword should not be extracted, got: {:?}", result.extractions);
}

// ═══════════════════════════════════════════════════════════════
// GROUP B: Field type not entering var_types (Suggestion 2)
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_b1_field_string_type_in_method_concat() {
    let java = r#"
        public class Dao {
            private String tableName = "users";
            public void query() {
                String sql = "SELECT * FROM " + tableName;
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1, "should extract SQL from field concat");
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_String_tableName__"),
        "field variable should have String type from var_types, got: {}", result.extractions[0].sql);
}

#[test]
fn diag_b2_field_int_type_in_sql() {
    let java = r#"
        public class Dao {
            private int limit = 100;
            public void query() {
                String sql = "SELECT * FROM users LIMIT " + limit;
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1, "should extract SQL with field int variable");
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_int_limit__"),
        "field int should produce __JAVA_VAR_int_limit__, not String; \
        needs field type extraction in visit_field_declaration, got: {}", result.extractions[0].sql);
}

#[test]
fn diag_b3_field_stringbuilder_in_method() {
    let java = r#"
        public class Dao {
            private StringBuilder sqlBuilder = new StringBuilder("SELECT * FROM users");
            public void query(int id) {
                sqlBuilder.append(" WHERE id = ").append(id);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "field-level StringBuilder should be tracked; \
        needs visit_field_declaration to handle StringBuilder type, got count: {}", result.extractions.len());
    let combined = result.extractions.iter().map(|e| e.sql.as_str()).collect::<String>();
    assert!(combined.contains("WHERE id ="),
        "field SB append in method should be tracked, got: {:?}", result.extractions);
}

// ═══════════════════════════════════════════════════════════════
// GROUP C: StringBuilder mixed chains (Suggestion 3)
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_c1_append_insert_append_chain() {
    let java = r#"
        public class Dao {
            public void query() {
                StringBuilder sql = new StringBuilder("SELECT  users");
                sql.append(" WHERE 1=1").insert(7, "* FROM").append(" AND active = 1");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("* FROM"),
        "insert in chained call should be applied; \
        needs handle_sb_append to recurse into insert/delete, got: {}", sql);
    assert!(sql.contains("WHERE 1=1"), "append in chain should work, got: {}", sql);
    assert!(sql.contains("AND active = 1"), "final append in chain should work, got: {}", sql);
}

#[test]
fn diag_c2_stringbuilder_replace() {
    let java = r#"
        public class Dao {
            public void query() {
                StringBuilder sql = new StringBuilder("SELECT * FROM old_table WHERE id = 1");
                sql.replace(14, 23, "users");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("users") && !sql.contains("old_table"),
        "StringBuilder.replace should be applied; \
        needs handle_sb_replace implementation, got: {}", sql);
}

#[test]
fn diag_c3_delete_then_append_chain() {
    let java = r#"
        public class Dao {
            public void query() {
                StringBuilder sql = new StringBuilder("SELECT * FROM users WHERE obsolete");
                sql.delete(19, 34).append(" WHERE id = 1");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(!sql.contains("obsolete"),
        "delete should remove 'obsolete', got: {}", sql);
    assert!(sql.contains("WHERE id = 1"),
        "append after delete in chain should work; \
        needs handle_sb_delete to propagate chain to append, got: {}", sql);
}

// ═══════════════════════════════════════════════════════════════
// GROUP D: Type inference limitations (Suggestion 5)
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_d1_constant_propagation_local() {
    let java = r#"
        public class Dao {
            public void query() {
                final String TABLE = "users";
                String sql = "SELECT * FROM " + TABLE;
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1, "should extract SQL");
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("SELECT * FROM users"),
        "final String constant should be inlined into SQL, got: {}", sql);
}

#[test]
fn diag_d2_constant_propagation_static_field() {
    let java = r#"
        public class Dao {
            private static final String TABLE = "users";
            public void query() {
                String sql = "SELECT * FROM " + TABLE;
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("SELECT * FROM users"),
        "static final field constant should be inlined, got: {}", sql);
}

#[test]
fn diag_d3_getter_method_return() {
    let java = r#"
        public class Dao {
            public void query() {
                String sql = "SELECT * FROM " + getTableName();
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1, "SQL with method call should be extracted");
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("SELECT * FROM"),
        "should extract SQL containing method call, got: {}", sql);
    assert!(!sql.contains("getTableName()") && !sql.contains("____"),
        "placeholder for getter call should be clean, got: {}", sql);
}

#[test]
fn diag_d4_field_access_type() {
    let java = r#"
        public class Dao {
            public void query(User user) {
                String sql = "SELECT * FROM users WHERE name = '" + user.name + "'";
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(!sql.contains("user.name") && !sql.contains("user_name"),
        "field access user.name should produce a clean placeholder for the field part, got: {}", sql);
}

#[test]
fn diag_d5_generic_list_get() {
    let java = r#"
        public class Dao {
            public void query(List<String> names) {
                String sql = "SELECT * FROM users WHERE name = '" + names.get(0) + "'";
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(!sql.contains("names.get(0)") && !sql.contains("names_get_0___"),
        "names.get(0) should produce a clean placeholder, got: {}", sql);
}

#[test]
fn diag_d6_int_variable_typed_concat() {
    let java = r#"
        public class Dao {
            public void query() {
                int limit = 10;
                String sql = "SELECT * FROM users LIMIT " + limit;
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_int_limit__"),
        "local int variable should produce int-typed placeholder, got: {}", result.extractions[0].sql);
}

// ═══════════════════════════════════════════════════════════════
// GROUP E: Annotation robustness (Suggestion 7)
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_e1_annotation_no_space() {
    let java = r#"
        public interface Repo {
            @Query(value="SELECT * FROM users WHERE id = :id",nativeQuery=true)
            User findById(@Param("id") int id);
        }
    "#;
    let result = extract_sql_from_java(java, "Repo.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("SELECT * FROM users"));
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_int_id__"));
}

#[test]
fn diag_e2_annotation_bare_value() {
    let java = r#"
        public interface Repo {
            @Query("SELECT * FROM users WHERE status = :status")
            List<User> findByStatus(@Param("status") String status);
        }
    "#;
    let result = extract_sql_from_java(java, "Repo.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_String_status__"));
}

#[test]
fn diag_e3_annotation_keys_reversed_order() {
    let java = r#"
        public interface Repo {
            @Query(nativeQuery = true, value = "SELECT * FROM users WHERE id = :id")
            User findById(@Param("id") int id);
        }
    "#;
    let result = extract_sql_from_java(java, "Repo.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_int_id__"));
}

// ═══════════════════════════════════════════════════════════════
// GROUP F: Lambda / method reference (Suggestion 8)
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_f1_sql_as_lambda_arg() {
    let java = r#"
        public class Dao {
            public void find() {
                jdbcTemplate.query("SELECT * FROM users",
                    (rs, rowNum) -> new User(rs.getLong("id")));
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("SELECT * FROM users"));
}

#[test]
fn diag_f2_sql_built_inside_lambda() {
    let java = r#"
        public class Dao {
            public void process(List<String> tables) {
                tables.forEach(table -> {
                    String sql = "SELECT COUNT(*) FROM " + table;
                    jdbcTemplate.query(sql, (rs, rowNum) -> rs.getInt(1));
                });
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1,
        "SQL built inside lambda body should be extracted");
    assert!(result.extractions[0].sql.contains("SELECT COUNT(*) FROM"),
        "got: {}", result.extractions[0].sql);
}

#[test]
fn diag_f3_sql_inside_anonymous_class() {
    let java = r#"
        public class Dao {
            public void process() {
                Runnable r = new Runnable() {
                    @Override
                    public void run() {
                        String sql = "SELECT * FROM audit_log WHERE ts > NOW()";
                    }
                };
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1,
        "SQL inside anonymous class should be extracted");
    assert!(result.extractions[0].sql.contains("audit_log"));
}

// ═══════════════════════════════════════════════════════════════
// GROUP G: scoped_type_identifier (Suggestion 9)
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_g1_scoped_prepared_statement() {
    let java = r#"
        public class Dao {
            public void query(String name) throws Exception {
                java.sql.PreparedStatement ps = conn.prepareStatement("SELECT * FROM t WHERE name = ?");
                ps.setString(1, name);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_String_name__"),
        "scoped PS type should still resolve setString backfill, got: {}", result.extractions[0].sql);
    assert!(!result.extractions[0].sql.contains("__JAVA_VAR_JDBC_PARAM_1__"),
        "param should be backfilled, not left as JDBC_PARAM, got: {}", result.extractions[0].sql);
}

#[test]
fn diag_g2_scoped_ps_as_method_param() {
    let java = r#"
        public class Dao {
            public void process(String name) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO t (name) VALUES (?)");
                bindAndExecute(ps, name);
            }
            public static void bindAndExecute(java.sql.PreparedStatement ps, String name) throws Exception {
                ps.setString(1, name);
                ps.execute();
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("__JAVA_VAR_String_name__"),
        "scoped_type_identifier in method param should match 'PreparedStatement' for behavior recording; \
        needs extract_type_name to handle scoped_type_identifier, got: {}", sql);
    assert!(!sql.contains("DYNAMIC"),
        "should not fall back to DYNAMIC when setter pattern is available, got: {}", sql);
}

// ═══════════════════════════════════════════════════════════════
// GROUP H: Complex / edge cases (Suggestion 10)
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_h1_ternary_operator_sql() {
    let java = r#"
        public class Dao {
            public void query(boolean isRead) {
                String sql = isRead ? "SELECT * FROM users" : "UPDATE users SET active = 0";
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "ternary operator should extract at least one branch; \
        needs handling for ternary_expression node, got count: {}", result.extractions.len());
}

#[test]
fn diag_h2_switch_statement_sql() {
    let java = r#"
        public class Dao {
            public void query(int type, String name) {
                String sql;
                switch (type) {
                    case 1:
                        sql = "SELECT * FROM users WHERE name = '" + name + "'";
                        break;
                    case 2:
                        sql = "DELETE FROM users WHERE name = '" + name + "'";
                        break;
                    default:
                        sql = "SELECT * FROM users";
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 2,
        "switch branches with SQL should be extracted; \
        needs uninitialized var tracking + switch body traversal, got count: {}",
        result.extractions.len());
    let all_sql: String = result.extractions.iter().map(|e| e.sql.as_str()).collect();
    assert!(all_sql.contains("SELECT") || all_sql.contains("DELETE"),
        "should contain SELECT or DELETE SQL, got: {:?}", result.extractions);
}

#[test]
fn diag_h3_nested_class_field_sql() {
    let java = r#"
        public class Outer {
            class Inner {
                private static final String SQL = "SELECT * FROM inner_table";
                public void run() {
                    String sql = SQL + " WHERE id = 1";
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Outer.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 2,
        "nested class: should extract both field SQL and method concat; \
        got count: {}, extractions: {:?}", result.extractions.len(), result.extractions);
    let all_sql: String = result.extractions.iter().map(|e| e.sql.as_str()).collect();
    assert!(all_sql.contains("inner_table"),
        "should contain inner_table, got: {:?}", result.extractions);
    assert!(all_sql.contains("WHERE id = 1"),
        "method body concat should extend field SQL, got: {:?}", result.extractions);
}

#[test]
fn diag_h4_try_catch_sql() {
    let java = r#"
        public class Dao {
            public void query() {
                try {
                    String sql = "SELECT * FROM users";
                } catch (Exception e) {
                    String sql = "INSERT INTO error_log (msg) VALUES ('query failed')";
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 2, "both try and catch SQL should be extracted");
    let all_sql: String = result.extractions.iter().map(|e| e.sql.as_str()).collect();
    assert!(all_sql.contains("SELECT * FROM users"), "try block SQL, got: {:?}", result.extractions);
    assert!(all_sql.contains("INSERT INTO error_log"), "catch block SQL, got: {:?}", result.extractions);
}

#[test]
fn diag_h5_multiple_classes_same_file() {
    let java = r#"
        public class UserDao {
            public void insert(String name) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO users (name) VALUES (?)");
                PsHelper.bindName(ps, name);
            }
        }
        class PsHelper {
            public static void bindName(PreparedStatement ps, String name) throws Exception {
                ps.setString(1, name);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Multi.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("__JAVA_VAR_String_name__"),
        "cross-class PS passing should resolve setters; \
        needs method_behaviors shared across class declarations in same file, got: {}", sql);
    assert!(!sql.contains("DYNAMIC"),
        "should not fall back to DYNAMIC when literal setter pattern exists, got: {}", sql);
}

#[test]
fn diag_h6_if_else_branches() {
    let java = r#"
        public class Dao {
            public void query(boolean active, String name) {
                String sql;
                if (active) {
                    sql = "SELECT * FROM users WHERE active = 1 AND name = '" + name + "'";
                } else {
                    sql = "SELECT * FROM users WHERE active = 0";
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "if-else branches with SQL should be extracted; \
        needs uninitialized var tracking so 'String sql;' registers sql as tracked, \
        got count: {}", result.extractions.len());
}

// ═══════════════════════════════════════════════════════════════
// GROUP I: Cross-file simulation (Suggestion 4)
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_i1_two_classes_cross_method() {
    let java = r#"
        public class OrderDao {
            public void insertOrder(String orderId, String product) throws Exception {
                PreparedStatement ps = conn.prepareStatement(
                    "INSERT INTO orders (id, product) VALUES (?, ?)");
                OrderHelper.bindOrderParams(ps, orderId, product);
            }
        }
        class OrderHelper {
            public static void bindOrderParams(PreparedStatement ps, String orderId, String product) throws Exception {
                ps.setString(1, orderId);
                ps.setString(2, product);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Order.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("__JAVA_VAR_String_orderId__"),
        "cross-class PS should resolve typed params; \
        needs method_behaviors shared across class declarations, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_String_product__"),
        "product param should also be typed, got: {}", sql);
    assert!(!sql.contains("DYNAMIC"),
        "should not fall back to DYNAMIC, got: {}", sql);
}

#[test]
fn diag_i2_same_class_cross_method() {
    let java = r#"
        public class Dao {
            public void batch(String a, String b, String c) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO t (x,y,z) VALUES (?,?,?)");
                bindAll(ps, a, b, c);
            }
            private static void bindAll(PreparedStatement ps, String a, String b, String c) throws Exception {
                ps.setString(1, a);
                ps.setString(2, b);
                ps.setString(3, c);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("__JAVA_VAR_String_a__"), "param a, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_String_b__"), "param b, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_String_c__"), "param c, got: {}", sql);
    assert!(!sql.contains("__JAVA_VAR_JDBC_PARAM_"), "no unresolved, got: {}", sql);
}

// ═══════════════════════════════════════════════════════════════
// GROUP J: Combinatorial / real-world complex scenarios
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_j1_sb_with_jdbc_setters_and_cross_method() {
    let java = r#"
        public class ReportDao {
            private static final String BASE_SQL = "SELECT * FROM report_data";
            public void generate(String dept, String startDate, String endDate) throws Exception {
                StringBuilder sql = new StringBuilder(BASE_SQL);
                sql.append(" WHERE dept = ?");
                sql.append(" AND report_date BETWEEN ? AND ?");
                PreparedStatement ps = conn.prepareStatement(sql.toString());
                bindReportParams(ps, dept, startDate, endDate);
            }
            private static void bindReportParams(PreparedStatement ps, String dept, String start, String end) throws Exception {
                ps.setString(1, dept);
                ps.setString(2, start);
                ps.setString(3, end);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Report.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "should extract at least the SB-assembled SQL, got count: {}", result.extractions.len());
    let all_sql: String = result.extractions.iter().map(|e| e.sql.as_str()).collect();
    assert!(all_sql.contains("WHERE dept"),
        "SB appends should appear in final SQL, got: {:?}", result.extractions);
    assert!(all_sql.contains("__JAVA_VAR_String_dept__"),
        "cross-method JDBC setter should backfill typed params, got: {:?}", result.extractions);
}

#[test]
fn diag_j2_annotation_plus_method_call_plus_constant() {
    let java = r#"
        public interface UserRepo {
            @Select("SELECT * FROM users WHERE id = #{id}")
            User findById(@Param("id") int id);
        }
        class UserDao {
            private static final String SQL_INSERT = "INSERT INTO users (name) VALUES (?)";
            public void insert(String name) throws Exception {
                PreparedStatement ps = conn.prepareStatement(SQL_INSERT);
                ps.setString(1, name);
            }
            public void search(String keyword) {
                jdbcTemplate.query("SELECT * FROM users WHERE name LIKE '%' || ? || '%'",
                    (rs, rowNum) -> new User());
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Mixed.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 3,
        "should extract from annotation + constant + method call, got: {}", result.extractions.len());
    let methods: Vec<_> = result.extractions.iter().map(|e| e.origin.method).collect();
    assert!(methods.contains(&ExtractionMethod::Annotation), "should have annotation extraction");
    assert!(methods.contains(&ExtractionMethod::Constant), "should have constant extraction");
    assert!(methods.contains(&ExtractionMethod::MethodCall), "should have method call extraction");
}

#[test]
fn diag_j3_mybatis_dynamic_sql_pattern() {
    let java = r#"
        public interface Mapper {
            @Select("SELECT * FROM ${tableName} WHERE status = #{status} AND id IN (${ids})")
            List<Map> findDynamic(@Param("tableName") String table, @Param("status") int status, @Param("ids") String ids);
        }
    "#;
    let result = extract_sql_from_java(java, "Mapper.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("__JAVA_VAR_tableName__"), "dollar-brace raw placeholder, got: {sql}");
    assert!(sql.contains("__JAVA_VAR_int_status__"), "hash-brace typed placeholder, got: {sql}");
    assert!(sql.contains("__JAVA_VAR_String_ids__"), "dollar-brace raw placeholder for ids, got: {sql}");
}

#[test]
fn diag_j4_complex_sb_with_variable_parts() {
    let java = r#"
        public class Dao {
            public void search(String table, String status, int limit) {
                StringBuilder sql = new StringBuilder("SELECT * FROM ");
                sql.append(table);
                sql.append(" WHERE 1=1");
                sql.append(" AND status = '").append(status).append("'");
                sql.append(" LIMIT ").append(limit);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("SELECT * FROM"), "base SB, got: {}", sql);
    assert!(sql.contains("WHERE 1=1"), "append, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_String_table__"), "table var, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_String_status__"), "status var, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_int_limit__"), "limit var, got: {}", sql);
}

#[test]
fn diag_j5_text_block_with_params() {
    let java = r#"
        public class Dao {
            public void query(String name, int age) throws Exception {
                String sql = """
                    SELECT id, name, age
                    FROM users
                    WHERE name = ? AND age > ?
                    ORDER BY name
                    """;
                PreparedStatement ps = conn.prepareStatement(sql);
                ps.setString(1, name);
                ps.setInt(2, age);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_String_name__"),
        "name param, got: {}", result.extractions[0].sql);
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_int_age__"),
        "age param, got: {}", result.extractions[0].sql);
}

#[test]
fn diag_j6_string_concat_with_method_call_value() {
    let java = r#"
        public class Dao {
            public void query() {
                String sql = "SELECT * FROM " + getTable() + " WHERE id = 1";
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("SELECT * FROM"),
        "SQL with method call should be extracted, got: {}", result.extractions[0].sql);
}

#[test]
fn diag_j7_sb_concat_with_ps_cross_method_complex() {
    let java = r#"
        public class ComplexDao {
            private static final String BASE_QUERY = "SELECT u.id, u.name, d.dept_name";
            public void search(String name, String dept, int minAge) throws Exception {
                StringBuilder sql = new StringBuilder(BASE_QUERY);
                sql.append(" FROM users u JOIN departments d ON u.dept_id = d.id");
                sql.append(" WHERE u.name LIKE ?");
                sql.append(" AND d.dept_name = ?");
                sql.append(" AND u.age >= ?");
                PreparedStatement ps = conn.prepareStatement(sql.toString());
                SearchHelper.bindParams(ps, name, dept, minAge);
            }
        }
        class SearchHelper {
            public static void bindParams(PreparedStatement ps, String name, String dept, int minAge) throws Exception {
                ps.setString(1, "%" + name + "%");
                ps.setString(2, dept);
                ps.setInt(3, minAge);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Complex.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1);
    let all_sql: String = result.extractions.iter().map(|e| e.sql.as_str()).collect();
    assert!(all_sql.contains("WHERE u.name LIKE"),
        "SB appends with JDBC params should be assembled, got: {:?}", result.extractions);
    assert!(all_sql.contains("__JAVA_VAR_String_name__"),
        "cross-class method should resolve name param, got: {:?}", result.extractions);
    assert!(all_sql.contains("__JAVA_VAR_String_dept__"),
        "cross-class method should resolve dept param, got: {:?}", result.extractions);
    assert!(all_sql.contains("__JAVA_VAR_int_minAge__"),
        "cross-class method should resolve minAge param, got: {:?}", result.extractions);
}

#[test]
fn diag_j8_overloaded_method_name() {
    let java = r#"
        public class Dao {
            public void insertUser(String name) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO users (name) VALUES (?)");
                Helper.bind(ps, name);
            }
            public void insertOrder(String orderId, int amount) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO orders (id, amount) VALUES (?, ?)");
                Helper.bind(ps, orderId, amount);
            }
        }
        class Helper {
            public static void bind(PreparedStatement ps, String name) throws Exception {
                ps.setString(1, name);
            }
            public static void bind(PreparedStatement ps, String orderId, int amount) throws Exception {
                ps.setString(1, orderId);
                ps.setInt(2, amount);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Overload.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 2,
        "should extract both INSERT statements, got: {}", result.extractions.len());
    let user_sql = result.extractions.iter().find(|e| e.sql.contains("users"));
    let order_sql = result.extractions.iter().find(|e| e.sql.contains("orders"));
    assert!(user_sql.is_some(), "should extract INSERT INTO users");
    assert!(order_sql.is_some(), "should extract INSERT INTO orders");
    let user = user_sql.unwrap();
    assert!(user.sql.contains("__JAVA_VAR_String_name__"),
        "user insert: overloaded bind(String) should resolve, \
        needs methodBehaviors keyed by signature not just name, got: {}", user.sql);
    let order = order_sql.unwrap();
    assert!(order.sql.contains("__JAVA_VAR_String_orderId__"),
        "order insert: overloaded bind(String,int) should resolve, got: {}", order.sql);
    assert!(order.sql.contains("__JAVA_VAR_int_amount__"),
        "order insert: amount param, got: {}", order.sql);
}

// ═══════════════════════════════════════════════════════════════
// GROUP K: Control flow — if/else variations
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_k1_if_else_both_sql() {
    let java = r#"
        public class Dao {
            public void query(boolean flag) {
                String sql;
                if (flag) {
                    sql = "SELECT * FROM users WHERE active = 1";
                } else {
                    sql = "SELECT * FROM users WHERE active = 0";
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 2,
        "both if/else branches should produce extractions, got: {}", result.extractions.len());
    let all: String = result.extractions.iter().map(|e| e.sql.as_str()).collect();
    assert!(all.contains("active = 1"), "if branch, got: {:?}", result.extractions);
    assert!(all.contains("active = 0"), "else branch, got: {:?}", result.extractions);
}

#[test]
fn diag_k2_if_else_chain() {
    let java = r#"
        public class Dao {
            public void query(String type) {
                String sql;
                if ("user".equals(type)) {
                    sql = "SELECT * FROM users";
                } else if ("order".equals(type)) {
                    sql = "SELECT * FROM orders";
                } else {
                    sql = "SELECT * FROM products";
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 3,
        "all 3 branches should produce extractions, got: {}", result.extractions.len());
}

#[test]
fn diag_k3_nested_if_sql() {
    let java = r#"
        public class Dao {
            public void query(boolean admin, boolean active) {
                String sql = "SELECT * FROM users";
                if (admin) {
                    if (active) {
                        sql = sql + " WHERE active = 1 AND role = 'admin'";
                    } else {
                        sql = sql + " WHERE role = 'admin'";
                    }
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "nested if should extract SQL, got: {}", result.extractions.len());
    let all: String = result.extractions.iter().map(|e| e.sql.as_str()).collect();
    assert!(all.contains("SELECT * FROM users"), "base SQL, got: {:?}", result.extractions);
}

#[test]
fn diag_k4_if_with_concat_var() {
    let java = r#"
        public class Dao {
            public void query(String table, boolean condition) {
                String sql;
                if (condition) {
                    sql = "SELECT * FROM " + table + " WHERE active = 1";
                } else {
                    sql = "SELECT * FROM " + table;
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 2,
        "both branches with concat should extract, got: {}", result.extractions.len());
    let all: String = result.extractions.iter().map(|e| e.sql.as_str()).collect();
    assert!(all.contains("WHERE active = 1"), "conditional branch, got: {:?}", result.extractions);
}

#[test]
fn diag_k5_if_else_sb_append() {
    let java = r#"
        public class Dao {
            public void query(boolean includeDeleted) {
                StringBuilder sql = new StringBuilder("SELECT * FROM users");
                if (includeDeleted) {
                    sql.append(" WHERE 1=1");
                } else {
                    sql.append(" WHERE active = 1");
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "SB inside if/else should produce extraction, got: {}", result.extractions.len());
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("SELECT * FROM users"), "base SB, got: {}", sql);
}

#[test]
fn diag_k6_only_if_no_else_sql() {
    let java = r#"
        public class Dao {
            public void query(boolean filter) {
                String sql = "SELECT * FROM users";
                if (filter) {
                    sql += " WHERE active = 1";
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "if-only branch with += should extract, got: {}", result.extractions.len());
}

// ═══════════════════════════════════════════════════════════════
// GROUP L: Control flow — for loops
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_l1_sql_inside_for_body() {
    let java = r#"
        public class Dao {
            public void batch() {
                for (int i = 0; i < 10; i++) {
                    String sql = "INSERT INTO log (msg) VALUES ('batch')";
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "SQL inside for body should be extracted, got: {}", result.extractions.len());
    assert!(result.extractions[0].sql.contains("INSERT INTO log"));
}

#[test]
fn diag_l2_sb_append_in_for() {
    let java = r#"
        public class Dao {
            public void buildQuery(String[] columns) {
                StringBuilder sql = new StringBuilder("SELECT ");
                for (int i = 0; i < columns.length; i++) {
                    sql.append(columns[i]);
                    if (i < columns.length - 1) {
                        sql.append(", ");
                    }
                }
                sql.append(" FROM users");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "SB assembled in for loop should extract, got: {}", result.extractions.len());
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("SELECT "), "base SELECT, got: {}", sql);
    assert!(sql.contains("FROM users"), "append after loop, got: {}", sql);
}

#[test]
fn diag_l3_for_each_sql() {
    let java = r#"
        public class Dao {
            public void process(List<String> tables) {
                for (String table : tables) {
                    String sql = "SELECT COUNT(*) FROM " + table;
                    jdbcTemplate.query(sql, (rs, rowNum) -> rs.getInt(1));
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "SQL inside for-each should be extracted, got: {}", result.extractions.len());
    let all: String = result.extractions.iter().map(|e| e.sql.as_str()).collect();
    assert!(all.contains("SELECT COUNT(*) FROM"), "for-each SQL, got: {:?}", result.extractions);
}

#[test]
fn diag_l4_for_with_ps_setter() {
    let java = r#"
        public class Dao {
            public void batch(String[] names) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO users (name) VALUES (?)");
                for (int i = 0; i < names.length; i++) {
                    ps.setString(1, names[i]);
                    ps.addBatch();
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("INSERT INTO users"), "base PS SQL, got: {}", sql);
}

#[test]
fn diag_l5_nested_for_sb_in() {
    let java = r#"
        public class Dao {
            public void buildInClause(List<String> ids) {
                StringBuilder sql = new StringBuilder("SELECT * FROM users WHERE id IN (");
                for (int i = 0; i < ids.size(); i++) {
                    if (i > 0) sql.append(",");
                    sql.append("?");
                }
                sql.append(")");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "nested for building IN clause should extract, got: {}", result.extractions.len());
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("SELECT * FROM users WHERE id IN"), "IN clause base, got: {}", sql);
}

#[test]
fn diag_l6_while_loop_sql() {
    let java = r#"
        public class Dao {
            public void process(ResultSet rs) throws Exception {
                while (rs.next()) {
                    String name = rs.getString("name");
                    String sql = "INSERT INTO audit (name) VALUES ('" + name + "')";
                    jdbcTemplate.execute(sql);
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "SQL inside while should be extracted, got: {}", result.extractions.len());
}

#[test]
fn diag_l7_do_while_sql() {
    let java = r#"
        public class Dao {
            public void retry() {
                String sql;
                int attempts = 0;
                do {
                    sql = "SELECT * FROM status WHERE id = 1";
                    attempts++;
                } while (attempts < 3);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "SQL inside do-while should be extracted, got: {}", result.extractions.len());
    assert!(result.extractions[0].sql.contains("SELECT * FROM status"));
}

// ═══════════════════════════════════════════════════════════════
// GROUP M: Control flow — switch / switch-expression
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_m1_switch_multiple_cases() {
    let java = r#"
        public class Dao {
            public void query(int type) {
                String sql;
                switch (type) {
                    case 1:
                        sql = "SELECT * FROM users";
                        break;
                    case 2:
                        sql = "SELECT * FROM orders";
                        break;
                    case 3:
                        sql = "SELECT * FROM products";
                        break;
                    default:
                        sql = "SELECT 1";
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 4,
        "all 4 switch branches should extract, got: {}", result.extractions.len());
    let all: String = result.extractions.iter().map(|e| e.sql.as_str()).collect();
    assert!(all.contains("users"), "case 1, got: {:?}", result.extractions);
    assert!(all.contains("orders"), "case 2, got: {:?}", result.extractions);
    assert!(all.contains("products"), "case 3, got: {:?}", result.extractions);
    assert!(all.contains("SELECT 1"), "default, got: {:?}", result.extractions);
}

#[test]
fn diag_m2_switch_fallthrough() {
    let java = r#"
        public class Dao {
            public void query(int type) {
                String sql;
                switch (type) {
                    case 1:
                    case 2:
                        sql = "SELECT * FROM users";
                        break;
                    default:
                        sql = "SELECT * FROM all_data";
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 2,
        "switch fallthrough cases should extract, got: {}", result.extractions.len());
}

#[test]
fn diag_m3_switch_with_sb() {
    let java = r#"
        public class Dao {
            public void query(String mode) {
                StringBuilder sql = new StringBuilder("SELECT * FROM users");
                switch (mode) {
                    case "active":
                        sql.append(" WHERE active = 1");
                        break;
                    case "admin":
                        sql.append(" WHERE role = 'admin'");
                        break;
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "SB inside switch should extract, got: {}", result.extractions.len());
}

#[test]
fn diag_m4_switch_expression_java14() {
    let java = r#"
        public class Dao {
            public void query(String type) {
                String sql = switch (type) {
                    case "user" -> "SELECT * FROM users";
                    case "order" -> "SELECT * FROM orders";
                    default -> "SELECT 1";
                };
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "Java 14 switch expression (arrow syntax) should extract at least one SQL; \
        currently fails because tree-sitter switch_expression node is not traversed, got: {}", result.extractions.len());
}

#[test]
fn diag_m5_switch_with_string_concat() {
    let java = r#"
        public class Dao {
            public void query(int op, String table) {
                String sql;
                switch (op) {
                    case 1:
                        sql = "SELECT * FROM " + table;
                        break;
                    case 2:
                        sql = "DELETE FROM " + table;
                        break;
                    default:
                        sql = "SELECT 1";
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 2,
        "switch with concatenated SQL should extract, got: {}", result.extractions.len());
}

// ═══════════════════════════════════════════════════════════════
// GROUP N: Control flow — try/catch/finally
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_n1_try_catch_finally() {
    let java = r#"
        public class Dao {
            public void query() throws Exception {
                try {
                    String sql = "SELECT * FROM users";
                    jdbcTemplate.query(sql, (rs, rn) -> rs.getInt(1));
                } catch (SQLException e) {
                    String sql = "INSERT INTO error_log (msg) VALUES ('query failed')";
                    jdbcTemplate.execute(sql);
                } finally {
                    String sql = "DELETE FROM temp WHERE created < NOW()";
                    jdbcTemplate.execute(sql);
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 3,
        "try + catch + finally should each extract, got: {}", result.extractions.len());
    let all: String = result.extractions.iter().map(|e| e.sql.as_str()).collect();
    assert!(all.contains("SELECT * FROM users"), "try block");
    assert!(all.contains("INSERT INTO error_log"), "catch block");
    assert!(all.contains("DELETE FROM temp"), "finally block");
}

#[test]
fn diag_n2_try_with_resources_sql() {
    let java = r#"
        public class Dao {
            public void query(String name) throws Exception {
                try (PreparedStatement ps = conn.prepareStatement("SELECT * FROM users WHERE name = ?")) {
                    ps.setString(1, name);
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "SQL in try-with-resources should extract, got: {}", result.extractions.len());
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("SELECT * FROM users WHERE name ="), "PS SQL, got: {}", sql);
}

#[test]
fn diag_n3_nested_try_catch() {
    let java = r#"
        public class Dao {
            public void complex() {
                try {
                    String sql1 = "SELECT * FROM users";
                    try {
                        String sql2 = "INSERT INTO audit (action) VALUES ('read')";
                    } catch (Exception e) {
                        String sql3 = "INSERT INTO error_log (msg) VALUES ('inner fail')";
                    }
                } catch (Exception e) {
                    String sql4 = "INSERT INTO error_log (msg) VALUES ('outer fail')";
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 4,
        "all 4 nested try/catch SQLs should extract, got: {}", result.extractions.len());
}

#[test]
fn diag_n4_multi_catch() {
    let java = r#"
        public class Dao {
            public void query() {
                try {
                    String sql = "SELECT * FROM users";
                } catch (IOException | SQLException e) {
                    String sql = "INSERT INTO error_log (msg) VALUES ('multi-catch')";
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 2,
        "try + multi-catch should extract, got: {}", result.extractions.len());
}

// ═══════════════════════════════════════════════════════════════
// GROUP O: Control flow — combinations / real-world patterns
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_o1_if_inside_for_sb() {
    let java = r#"
        public class Dao {
            public void buildQuery(String table, boolean[] flags) {
                StringBuilder sql = new StringBuilder("SELECT * FROM ");
                sql.append(table);
                for (int i = 0; i < flags.length; i++) {
                    if (flags[i]) {
                        sql.append(" WHERE condition_").append(i);
                    }
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "if inside for with SB should extract, got: {}", result.extractions.len());
}

#[test]
fn diag_o2_sb_conditional_where() {
    let java = r#"
        public class Dao {
            public void search(String name, String dept, boolean activeOnly) {
                StringBuilder sql = new StringBuilder("SELECT * FROM users WHERE 1=1");
                if (name != null) {
                    sql.append(" AND name = '").append(name).append("'");
                }
                if (dept != null) {
                    sql.append(" AND dept = '").append(dept).append("'");
                }
                if (activeOnly) {
                    sql.append(" AND active = 1");
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "conditional WHERE building should extract base SQL, got: {}", result.extractions.len());
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("WHERE 1=1"), "base WHERE clause, got: {}", sql);
}

#[test]
fn diag_o3_switch_in_for() {
    let java = r#"
        public class Dao {
            public void multiOp(String[] ops) throws Exception {
                for (String op : ops) {
                    PreparedStatement ps;
                    switch (op) {
                        case "select":
                            ps = conn.prepareStatement("SELECT * FROM data");
                            break;
                        case "delete":
                            ps = conn.prepareStatement("DELETE FROM data WHERE id = 0");
                            break;
                        default:
                            ps = conn.prepareStatement("SELECT 1");
                    }
                    ps.execute();
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 3,
        "switch inside for should extract each branch, got: {}", result.extractions.len());
}

#[test]
fn diag_o4_for_inside_try_inside_if() {
    let java = r#"
        public class Dao {
            public void batch(String[] names) {
                if (names != null && names.length > 0) {
                    try {
                        String sql = "INSERT INTO users (name) VALUES (?)";
                        PreparedStatement ps = conn.prepareStatement(sql);
                        for (String name : names) {
                            ps.setString(1, name);
                            ps.addBatch();
                        }
                    } catch (Exception e) {
                        String sql = "INSERT INTO error_log (msg) VALUES ('batch failed')";
                    }
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 2,
        "nested if > try > for should extract, got: {}", result.extractions.len());
}

#[test]
fn diag_o5_labeled_break_in_for() {
    let java = r#"
        public class Dao {
            public void search(List<String> tables) {
                outer:
                for (String table : tables) {
                    String sql = "SELECT COUNT(*) FROM " + table;
                    if (table.equals("stop")) break outer;
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "SQL inside labeled for should extract, got: {}", result.extractions.len());
}

#[test]
fn diag_o6_synchronized_block_sql() {
    let java = r#"
        public class Dao {
            public void query() {
                synchronized (this) {
                    String sql = "SELECT * FROM locked_table WHERE id = 1";
                    jdbcTemplate.query(sql, (rs, rn) -> rs.getInt(1));
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "SQL inside synchronized block should extract, got: {}", result.extractions.len());
}

#[test]
fn diag_o7_dynamic_where_builder() {
    let java = r#"
        public class Dao {
            public void search(String name, Integer age, String dept) {
                StringBuilder sql = new StringBuilder("SELECT * FROM users WHERE 1=1");
                if (name != null && !name.isEmpty()) {
                    sql.append(" AND name LIKE ?");
                }
                if (age != null) {
                    sql.append(" AND age > ?");
                }
                if (dept != null) {
                    sql.append(" AND dept = ?");
                }
                sql.append(" ORDER BY id DESC LIMIT 100");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("SELECT * FROM users"), "base, got: {}", sql);
    assert!(sql.contains("ORDER BY id DESC"), "final append, got: {}", sql);
}

#[test]
fn diag_o8_batch_insert_with_for_and_cross_method() {
    let java = r#"
        public class BatchDao {
            public void insertAll(List<User> users) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO users (name, email) VALUES (?, ?)");
                for (User u : users) {
                    BatchHelper.bindUser(ps, u.name, u.email);
                    ps.addBatch();
                }
            }
        }
        class BatchHelper {
            public static void bindUser(PreparedStatement ps, String name, String email) throws Exception {
                ps.setString(1, name);
                ps.setString(2, email);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Batch.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("INSERT INTO users"), "batch PS SQL, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_String_name__"), "cross-method param, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_String_email__"), "cross-method param, got: {}", sql);
}

#[test]
fn diag_o9_conditional_order_by() {
    let java = r#"
        public class Dao {
            public void query(String sortBy, boolean ascending) {
                StringBuilder sql = new StringBuilder("SELECT * FROM users");
                if (sortBy != null) {
                    sql.append(" ORDER BY ").append(sortBy);
                    if (ascending) {
                        sql.append(" ASC");
                    } else {
                        sql.append(" DESC");
                    }
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("SELECT * FROM users"), "base, got: {}", sql);
}

#[test]
fn diag_o10_return_sql_from_if() {
    let java = r#"
        public class Dao {
            public String getQuery(String type) {
                if ("simple".equals(type)) {
                    return "SELECT id FROM users";
                } else {
                    return "SELECT u.id, u.name, d.dept FROM users u JOIN dept d ON u.dept_id = d.id";
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 2,
        "return SQL from both if/else branches should extract; \
        currently fails because return_statement only recurses children but the string_literal \
        inside return is not visited as a top-level declaration/assignment, got: {}", result.extractions.len());
}

#[test]
fn diag_o11_break_in_switch_with_sql() {
    let java = r#"
        public class Dao {
            public void process(int action, String id) throws Exception {
                PreparedStatement ps;
                switch (action) {
                    case 1:
                        ps = conn.prepareStatement("SELECT * FROM data WHERE id = ?");
                        ps.setString(1, id);
                        break;
                    case 2:
                        ps = conn.prepareStatement("DELETE FROM data WHERE id = ?");
                        ps.setString(1, id);
                        break;
                    default:
                        return;
                }
                ps.execute();
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 2,
        "switch with break and PS should extract, got: {}", result.extractions.len());
}

#[test]
fn diag_o12_complex_daopattern() {
    let java = r#"
        public class UserDao {
            private static final String BASE_SQL = "SELECT id, name, email FROM users";
            
            public List<User> search(UserQuery q) throws Exception {
                StringBuilder sql = new StringBuilder(BASE_SQL);
                sql.append(" WHERE 1=1");
                if (q.getName() != null) {
                    sql.append(" AND name LIKE ?");
                }
                if (q.getMinAge() > 0) {
                    sql.append(" AND age >= ?");
                }
                if (q.getDept() != null) {
                    sql.append(" AND dept = ?");
                }
                
                PreparedStatement ps = conn.prepareStatement(sql.toString());
                int idx = 1;
                if (q.getName() != null) {
                    ps.setString(idx++, "%" + q.getName() + "%");
                }
                if (q.getMinAge() > 0) {
                    ps.setInt(idx++, q.getMinAge());
                }
                if (q.getDept() != null) {
                    ps.setString(idx++, q.getDept());
                }
                return mapResults(ps.executeQuery());
            }
        }
    "#;
    let result = extract_sql_from_java(java, "UserDao.java", &JavaExtractConfig::default());
    assert!(result.extractions.len() >= 1,
        "complex DAO pattern should extract SQL, got: {}", result.extractions.len());
    let all_sql: String = result.extractions.iter().map(|e| e.sql.as_str()).collect();
    assert!(all_sql.contains("SELECT id, name, email FROM users"), "base, got: {:?}", result.extractions);
    assert!(all_sql.contains("WHERE 1=1"), "WHERE clause, got: {:?}", result.extractions);
}

// ═══════════════════════════════════════════════════════════════
// GROUP P: Cross-file — method behavior sharing across files
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_p1_cross_file_ps_binding() {
    let helper_java = r#"
        public class PsHelper {
            public static void bindName(PreparedStatement ps, String name) throws Exception {
                ps.setString(1, name);
            }
        }
    "#;
    let dao_java = r#"
        public class UserDao {
            public void insert(String name) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO users (name) VALUES (?)");
                PsHelper.bindName(ps, name);
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[("PsHelper.java", helper_java), ("UserDao.java", dao_java)],
        &JavaExtractConfig::default(),
    );
    let dao = &results[1];
    assert!(dao.extractions.len() >= 1,
        "should extract INSERT from UserDao, got: {}", dao.extractions.len());
    let sql = &dao.extractions[0].sql;
    assert!(sql.contains("__JAVA_VAR_String_name__"),
        "cross-file PS binding should resolve setter; \
        needs method_behaviors shared across files, got: {}", sql);
    assert!(!sql.contains("DYNAMIC"),
        "should not fall back to DYNAMIC, got: {}", sql);
}

#[test]
fn diag_p2_cross_file_multi_param_binding() {
    let helper_java = r#"
        class OrderHelper {
            public static void bindOrder(PreparedStatement ps, String orderId, String product, int qty) throws Exception {
                ps.setString(1, orderId);
                ps.setString(2, product);
                ps.setInt(3, qty);
            }
        }
    "#;
    let dao_java = r#"
        public class OrderDao {
            public void create(String orderId, String product, int qty) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO orders (id, product, qty) VALUES (?, ?, ?)");
                OrderHelper.bindOrder(ps, orderId, product, qty);
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[("OrderHelper.java", helper_java), ("OrderDao.java", dao_java)],
        &JavaExtractConfig::default(),
    );
    let dao = &results[1];
    assert!(dao.extractions.len() >= 1);
    let sql = &dao.extractions[0].sql;
    assert!(sql.contains("__JAVA_VAR_String_orderId__"),
        "orderId from cross-file helper, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_String_product__"),
        "product from cross-file helper, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_int_qty__"),
        "qty from cross-file helper, got: {}", sql);
}

#[test]
fn diag_p3_cross_file_overloaded_methods() {
    let helper_java = r#"
        class Helper {
            public static void bind(PreparedStatement ps, String name) throws Exception {
                ps.setString(1, name);
            }
            public static void bind(PreparedStatement ps, String orderId, int amount) throws Exception {
                ps.setString(1, orderId);
                ps.setInt(2, amount);
            }
        }
    "#;
    let dao_java = r#"
        public class Dao {
            public void insertUser(String name) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO users (name) VALUES (?)");
                Helper.bind(ps, name);
            }
            public void insertOrder(String orderId, int amount) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO orders (id, amount) VALUES (?, ?)");
                Helper.bind(ps, orderId, amount);
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[("Helper.java", helper_java), ("Dao.java", dao_java)],
        &JavaExtractConfig::default(),
    );
    let dao = &results[1];
    assert!(dao.extractions.len() >= 2,
        "should extract both INSERTs, got: {}", dao.extractions.len());
    let user_sql = dao.extractions.iter().find(|e| e.sql.contains("users"));
    let order_sql = dao.extractions.iter().find(|e| e.sql.contains("orders"));
    assert!(user_sql.is_some(), "INSERT INTO users");
    assert!(order_sql.is_some(), "INSERT INTO orders");
    assert!(user_sql.unwrap().sql.contains("__JAVA_VAR_String_name__"),
        "user insert: cross-file overloaded bind(String), got: {}", user_sql.unwrap().sql);
    assert!(order_sql.unwrap().sql.contains("__JAVA_VAR_String_orderId__"),
        "order insert: cross-file overloaded bind(String,int), got: {}", order_sql.unwrap().sql);
}

#[test]
fn diag_p4_cross_file_sb_with_constant() {
    let constants_java = r#"
        public class SqlConstants {
            public static final String USER_QUERY = "SELECT id, name FROM users";
            public static final String ORDER_QUERY = "SELECT id, total FROM orders";
        }
    "#;
    let dao_java = r#"
        public class UserDao {
            public void query() {
                String sql = SqlConstants.USER_QUERY + " WHERE active = 1";
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[("SqlConstants.java", constants_java), ("UserDao.java", dao_java)],
        &JavaExtractConfig::default(),
    );
    let dao = &results[1];
    assert!(dao.extractions.len() >= 1,
        "cross-file constant in concat should extract, got: {}", dao.extractions.len());
    let sql = &dao.extractions[0].sql;
    assert!(sql.contains("SELECT id, name FROM users"),
        "constant value should be inlined, got: {}", sql);
    assert!(sql.contains("WHERE active = 1"),
        "local concat should be preserved, got: {}", sql);
}

#[test]
fn diag_p5_cross_file_sb_init_with_constant() {
    let constants_java = r#"
        public class SqlConstants {
            public static final String BASE_SQL = "SELECT u.id, u.name FROM users u";
        }
    "#;
    let dao_java = r#"
        public class UserDao {
            public void search(String name) {
                StringBuilder sql = new StringBuilder(SqlConstants.BASE_SQL);
                sql.append(" WHERE u.name LIKE ?");
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[("SqlConstants.java", constants_java), ("UserDao.java", dao_java)],
        &JavaExtractConfig::default(),
    );
    let dao = &results[1];
    assert!(dao.extractions.len() >= 1,
        "SB initialized with cross-file constant should extract, got: {}", dao.extractions.len());
    let sql = &dao.extractions[0].sql;
    assert!(sql.contains("SELECT u.id, u.name FROM users u"),
        "constant value as SB init, got: {}", sql);
    assert!(sql.contains("WHERE u.name LIKE ?"),
        "append should be tracked, got: {}", sql);
}

#[test]
fn diag_p6_cross_file_annotation_plus_constant() {
    let constants_java = r#"
        public class SqlConstants {
            public static final String FIND_ACTIVE = "SELECT * FROM users WHERE active = 1";
        }
    "#;
    let repo_java = r#"
        public interface UserRepo {
            @Select("SELECT * FROM users WHERE id = #{id}")
            User findById(@Param("id") int id);
        }
    "#;
    let dao_java = r#"
        class UserDao {
            public void findActive() {
                String sql = SqlConstants.FIND_ACTIVE;
                jdbcTemplate.query(sql, (rs, rn) -> new User());
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[
            ("SqlConstants.java", constants_java),
            ("UserRepo.java", repo_java),
            ("UserDao.java", dao_java),
        ],
        &JavaExtractConfig::default(),
    );
    let const_count = results[0].extractions.len();
    let repo_count = results[1].extractions.len();
    let dao_count = results[2].extractions.len();
    assert!(const_count >= 1, "constants file should extract, got: {}", const_count);
    assert!(repo_count >= 1, "repo annotations should extract, got: {}", repo_count);
    assert!(dao_count >= 1, "dao using cross-file constant should extract, got: {}", dao_count);
}

#[test]
fn diag_p7_cross_file_ps_through_service_layer() {
    let helper_java = r#"
        class JdbcHelper {
            public static void bindParams(PreparedStatement ps, String a, String b) throws Exception {
                ps.setString(1, a);
                ps.setString(2, b);
            }
        }
    "#;
    let service_java = r#"
        class UserService {
            public void save(String name, String email) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO users (name, email) VALUES (?, ?)");
                JdbcHelper.bindParams(ps, name, email);
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[("JdbcHelper.java", helper_java), ("UserService.java", service_java)],
        &JavaExtractConfig::default(),
    );
    let svc = &results[1];
    assert!(svc.extractions.len() >= 1);
    let sql = &svc.extractions[0].sql;
    assert!(sql.contains("__JAVA_VAR_String_name__"),
        "cross-file binding through service layer, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_String_email__"),
        "email param, got: {}", sql);
}

#[test]
fn diag_p8_cross_file_three_layer_chain() {
    let binder_java = r#"
        class ParamBinder {
            public static void bindUser(PreparedStatement ps, String name, int age) throws Exception {
                ps.setString(1, name);
                ps.setInt(2, age);
            }
        }
    "#;
    let service_java = r#"
        class UserService {
            public static void process(PreparedStatement ps, String name, int age) throws Exception {
                ParamBinder.bindUser(ps, name, age);
            }
        }
    "#;
    let dao_java = r#"
        public class UserDao {
            public void insert(String name, int age) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO users (name, age) VALUES (?, ?)");
                UserService.process(ps, name, age);
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[
            ("ParamBinder.java", binder_java),
            ("UserService.java", service_java),
            ("UserDao.java", dao_java),
        ],
        &JavaExtractConfig::default(),
    );
    let dao = &results[2];
    assert!(dao.extractions.len() >= 1);
    let sql = &dao.extractions[0].sql;
    assert!(sql.contains("__JAVA_VAR_String_name__"),
        "3-layer cross-file binding: name, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_int_age__"),
        "3-layer cross-file binding: age, got: {}", sql);
}

#[test]
fn diag_p9_cross_file_dynamic_loop_setter() {
    let helper_java = r#"
        class DataHelper {
            public static void submitData(PreparedStatement ps, List data) throws Exception {
                for (Iterator it = data.iterator(); it.hasNext();) {
                    String[] s = (String[]) it.next();
                    for (int i = 0; i < s.length; i++) {
                        ps.setString(i + 1, s[i]);
                    }
                    ps.addBatch();
                }
            }
        }
    "#;
    let dao_java = r#"
        public class Dao {
            public void process(List list) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO t (a,b,c) VALUES (?,?,?)");
                DataHelper.submitData(ps, list);
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[("DataHelper.java", helper_java), ("Dao.java", dao_java)],
        &JavaExtractConfig::default(),
    );
    let dao = &results[1];
    assert!(dao.extractions.len() >= 1);
    let sql = &dao.extractions[0].sql;
    assert!(sql.contains("__JAVA_VAR_String_DYNAMIC_"),
        "cross-file dynamic loop setter should produce DYNAMIC placeholders, got: {}", sql);
}

#[test]
fn diag_p10_cross_file_reverse_order() {
    let dao_java = r#"
        public class UserDao {
            public void insert(String name) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO users (name) VALUES (?)");
                Binder.bind(ps, name);
            }
        }
    "#;
    let binder_java = r#"
        class Binder {
            public static void bind(PreparedStatement ps, String name) throws Exception {
                ps.setString(1, name);
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[("UserDao.java", dao_java), ("Binder.java", binder_java)],
        &JavaExtractConfig::default(),
    );
    let dao = &results[0];
    assert!(dao.extractions.len() >= 1,
        "Dao processed BEFORE Binder: method_behaviors from Binder not yet available, got: {}", dao.extractions.len());
    let sql = &dao.extractions[0].sql;
    assert!(sql.contains("INSERT INTO users"), "base SQL, got: {}", sql);
}

#[test]
fn diag_p11_cross_file_sb_with_ps_and_constant() {
    let constants_java = r#"
        public class Sql {
            public static final String BASE = "SELECT * FROM report_data";
        }
    "#;
    let helper_java = r#"
        class ReportHelper {
            public static void bind(PreparedStatement ps, String dept, String start, String end) throws Exception {
                ps.setString(1, dept);
                ps.setString(2, start);
                ps.setString(3, end);
            }
        }
    "#;
    let dao_java = r#"
        public class ReportDao {
            public void generate(String dept, String start, String end) throws Exception {
                StringBuilder sql = new StringBuilder(Sql.BASE);
                sql.append(" WHERE dept = ?");
                sql.append(" AND report_date BETWEEN ? AND ?");
                PreparedStatement ps = conn.prepareStatement(sql.toString());
                ReportHelper.bind(ps, dept, start, end);
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[
            ("Sql.java", constants_java),
            ("ReportHelper.java", helper_java),
            ("ReportDao.java", dao_java),
        ],
        &JavaExtractConfig::default(),
    );
    let dao = &results[2];
    assert!(dao.extractions.len() >= 1);
    let sql = &dao.extractions[0].sql;
    assert!(sql.contains("SELECT * FROM report_data"), "constant in SB init, got: {}", sql);
    assert!(sql.contains("WHERE dept"), "append, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_String_dept__"),
        "cross-file binding, got: {}", sql);
}

#[test]
fn diag_p12_cross_file_inheritance_field_sql() {
    let base_java = r#"
        public class BaseDao {
            protected static final String TABLE = "users";
            protected String buildSelect() { return "SELECT * FROM " + TABLE; }
        }
    "#;
    let dao_java = r#"
        public class UserDao extends BaseDao {
            public void query() {
                String sql = buildSelect() + " WHERE active = 1";
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[("BaseDao.java", base_java), ("UserDao.java", dao_java)],
        &JavaExtractConfig::default(),
    );
    let dao = &results[1];
    assert!(dao.extractions.len() >= 1,
        "child class using inherited method should extract SQL, got: {}", dao.extractions.len());
}

// ═══════════════════════════════════════════════════════════════
// GROUP Q: Cross-file — edge cases and failure modes
// ═══════════════════════════════════════════════════════════════

#[test]
fn diag_q1_cross_file_nonexistent_method() {
    let dao_java = r#"
        public class Dao {
            public void insert(String name) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO users (name) VALUES (?)");
                NonexistentHelper.bind(ps, name);
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[("Dao.java", dao_java)],
        &JavaExtractConfig::default(),
    );
    let dao = &results[0];
    assert!(dao.extractions.len() >= 1,
        "should still extract SQL even if helper method not found, got: {}", dao.extractions.len());
    let sql = &dao.extractions[0].sql;
    assert!(sql.contains("INSERT INTO users"), "base SQL, got: {}", sql);
}

#[test]
fn diag_q2_cross_file_empty_helper() {
    let helper_java = r#"
        public class EmptyHelper {
            public static void doNothing() {}
        }
    "#;
    let dao_java = r#"
        public class Dao {
            public void query() {
                String sql = "SELECT * FROM users";
                jdbcTemplate.query(sql, (rs, rn) -> rs.getInt(1));
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[("EmptyHelper.java", helper_java), ("Dao.java", dao_java)],
        &JavaExtractConfig::default(),
    );
    let dao = &results[1];
    assert!(dao.extractions.len() >= 1,
        "unrelated helper file should not interfere, got: {}", dao.extractions.len());
}

#[test]
fn diag_q3_cross_file_constant_only_used_in_sb() {
    let constants_java = r#"
        public class Tables {
            public static final String USER_TABLE = "users";
            public static final String ORDER_TABLE = "orders";
        }
    "#;
    let dao_java = r#"
        public class Dao {
            public void query() {
                StringBuilder sql = new StringBuilder("SELECT * FROM ");
                sql.append(Tables.USER_TABLE);
                sql.append(" WHERE active = 1");
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[("Tables.java", constants_java), ("Dao.java", dao_java)],
        &JavaExtractConfig::default(),
    );
    let dao = &results[1];
    assert!(dao.extractions.len() >= 1,
        "cross-file constant as SB append arg should inline, got: {}", dao.extractions.len());
    let sql = &dao.extractions[0].sql;
    assert!(sql.contains("SELECT * FROM"),
        "base, got: {}", sql);
    assert!(sql.contains("WHERE active = 1"),
        "append, got: {}", sql);
}

#[test]
fn diag_q4_cross_file_same_class_name_different_files() {
    let helper1_java = r#"
        class Binder {
            public static void bind(PreparedStatement ps, String name) throws Exception {
                ps.setString(1, name);
            }
        }
    "#;
    let helper2_java = r#"
        class Binder {
            public static void bind(PreparedStatement ps, String name, int age) throws Exception {
                ps.setString(1, name);
                ps.setInt(2, age);
            }
        }
    "#;
    let dao_java = r#"
        public class Dao {
            public void insert(String name) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO t (name) VALUES (?)");
                Binder.bind(ps, name);
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[
            ("Binder1.java", helper1_java),
            ("Binder2.java", helper2_java),
            ("Dao.java", dao_java),
        ],
        &JavaExtractConfig::default(),
    );
    let dao = &results[2];
    assert!(dao.extractions.len() >= 1);
    let sql = &dao.extractions[0].sql;
    assert!(sql.contains("INSERT INTO t"), "base SQL, got: {}", sql);
}

#[test]
fn diag_q5_cross_file_concat_with_concat_value_in_setter() {
    let helper_java = r#"
        class SearchBinder {
            public static void bind(PreparedStatement ps, String name, String dept) throws Exception {
                ps.setString(1, "%" + name + "%");
                ps.setString(2, dept);
            }
        }
    "#;
    let dao_java = r#"
        public class SearchDao {
            public void search(String name, String dept) throws Exception {
                PreparedStatement ps = conn.prepareStatement(
                    "SELECT * FROM users WHERE name LIKE ? AND dept = ?");
                SearchBinder.bind(ps, name, dept);
            }
        }
    "#;
    let results = extract_sql_from_java_files(
        &[("SearchBinder.java", helper_java), ("SearchDao.java", dao_java)],
        &JavaExtractConfig::default(),
    );
    let dao = &results[1];
    assert!(dao.extractions.len() >= 1);
    let sql = &dao.extractions[0].sql;
    assert!(sql.contains("__JAVA_VAR_String_name__"),
        "concat value in cross-file setter should resolve, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_String_dept__"),
        "simple value in cross-file setter, got: {}", sql);
}


