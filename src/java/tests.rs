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
    assert_eq!(
        result.extractions[0].sql.trim(),
        "SELECT * FROM users WHERE status = :status"
    );
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
    assert_eq!(
        result.extractions[0].origin.annotation_name.as_deref(),
        Some("NamedQuery")
    );
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
    assert_eq!(
        ext.origin.api_method_name.as_deref(),
        Some("createNativeQuery")
    );
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
    assert_eq!(
        ext.origin.api_method_name.as_deref(),
        Some("prepareStatement")
    );
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
    let java = r#"
        public class MyService {
            public void doSomething() {
                query("http://example.com/api");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "MyService.java");
    let method_extractions: Vec<_> = result
        .extractions
        .iter()
        .filter(|e| e.origin.method == ExtractionMethod::MethodCall)
        .collect();
    assert!(method_extractions.is_empty());
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
    let constants: Vec<_> = result
        .extractions
        .iter()
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
    let constants: Vec<_> = result
        .extractions
        .iter()
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
    let constants: Vec<_> = result
        .extractions
        .iter()
        .filter(|e| e.origin.method == ExtractionMethod::Constant)
        .collect();
    assert!(constants.is_empty());
}

#[test]
fn test_sql_constant_name_heuristic() {
    let java = r#"
        public class Q {
            private static final String DELETE_SQL = "DELETE FROM temp";
        }
    "#;
    let result = extract_sql_from_java(java, "Q.java");
    let constants: Vec<_> = result
        .extractions
        .iter()
        .filter(|e| e.origin.method == ExtractionMethod::Constant)
        .collect();
    assert_eq!(constants.len(), 1);
}

#[test]
fn test_cross_statement_concat_assign() {
    let java = r#"
        public class Dao {
            public void query(String mail) {
                String sql = "select a from t where id=";
                sql = sql + "'" + mail + "'";
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java");
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(ext.sql, "select a from t where id='__JAVA_VAR__'");
    assert!(ext.is_concatenated);
    assert!(ext.parse_result.as_ref().map_or(false, |r| r
        .errors
        .iter()
        .all(|e| matches!(e, crate::parser::ParserError::Warning { .. }))));
}

#[test]
fn test_cross_statement_concat_plus_eq() {
    let java = r#"
        public class Dao {
            public void query(int id, String name) {
                String sql = "select * from t where id=";
                sql += id;
                sql += " and name='" + name + "'";
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java");
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(
        ext.sql,
        "select * from t where id=__JAVA_VAR__ and name='__JAVA_VAR__'"
    );
    assert!(ext.is_concatenated);
}

#[test]
fn test_cross_statement_concat_multi_step() {
    let java = r#"
        public class Dao {
            public void query(int id, String name, String status) {
                String sql = "select * from t";
                sql = sql + " where id=" + id;
                sql = sql + " and name='" + name + "'";
                sql = sql + " and status='" + status + "'";
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java");
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(
        ext.sql,
        "select * from t where id=__JAVA_VAR__ and name='__JAVA_VAR__' and status='__JAVA_VAR__'"
    );
    assert!(ext.is_concatenated);
}

#[test]
fn test_cross_statement_method_scoped() {
    let java = r#"
        public class Dao {
            public void methodA() {
                String sql = "select * from a";
            }
            public void methodB() {
                String sql = "select * from b";
                sql = sql + " where id = 1";
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java");
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 2);
    assert_eq!(result.extractions[0].sql, "select * from a");
    assert_eq!(result.extractions[1].sql, "select * from b where id = 1");
}

#[test]
fn test_cross_statement_non_tracked_var_ignored() {
    let java = r#"
        public class Dao {
            public void query() {
                String msg = "hello world";
                msg = msg + "!";
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java");
    assert!(result.extractions.is_empty());
}
