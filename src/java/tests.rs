use crate::java::{extract_sql_from_java, ExtractionMethod, JavaExtractConfig, SqlKind};

// ── Placeholder Conversion Tests (Phase 1.1) ──

#[test]
fn test_placeholder_question_mark_positional() {
    let java = r#"
        public class UserService {
            public User findUser(String id) {
                return em.createNativeQuery("SELECT * FROM users WHERE id = ?", User.class)
                    .getSingleResult();
            }
        }
    "#;
    let result = extract_sql_from_java(java, "UserService.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(
        ext.sql.trim(),
        "SELECT * FROM users WHERE id = __JAVA_VAR_JDBC_PARAM_1__"
    );
}

#[test]
fn test_placeholder_question_mark_multiple() {
    let java = r#"
        public class UserDao {
            public void insert(String name, String email) {
                PreparedStatement ps = conn.prepareStatement(
                    "INSERT INTO users (name, email) VALUES (?, ?)");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "UserDao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(
        ext.sql.trim(),
        "INSERT INTO users (name, email) VALUES (__JAVA_VAR_JDBC_PARAM_1__, __JAVA_VAR_JDBC_PARAM_2__)"
    );
}

#[test]
fn test_placeholder_question_mark_numbered() {
    let java = r#"
        public class UserDao {
            public void insert(String name, String email) {
                PreparedStatement ps = conn.prepareStatement(
                    "INSERT INTO users (name, email) VALUES (?1, ?2)");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "UserDao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(
        ext.sql.trim(),
        "INSERT INTO users (name, email) VALUES (__JAVA_VAR_JDBC_PARAM_1__, __JAVA_VAR_JDBC_PARAM_2__)"
    );
}

#[test]
fn test_placeholder_named_colon() {
    let java = r#"
        public interface UserRepository {
            @Query(value = "SELECT * FROM users WHERE status = :status", nativeQuery = true)
            List<User> findByStatus(@Param("status") int status);
        }
    "#;
    let result = extract_sql_from_java(java, "UserRepository.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(
        ext.sql.trim(),
        "SELECT * FROM users WHERE status = __JAVA_VAR_status__"
    );
}

#[test]
fn test_placeholder_named_multiple() {
    let java = r#"
        public interface UserDao {
            @SqlUpdate("INSERT INTO users (name, email) VALUES (:name, :email)")
            void insert(@Bind("name") String name, @Bind("email") String email);
        }
    "#;
    let result = extract_sql_from_java(java, "UserDao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(
        ext.sql.trim(),
        "INSERT INTO users (name, email) VALUES (__JAVA_VAR_name__, __JAVA_VAR_email__)"
    );
}

#[test]
fn test_placeholder_in_constant() {
    let java = r#"
        public class UserQueries {
            private static final String SQL_FIND_BY_ID =
                "SELECT id, name, email FROM users WHERE id = ?";
            private static final String SQL_INSERT =
                "INSERT INTO users (name, email) VALUES (?, ?)";
        }
    "#;
    let result = extract_sql_from_java(java, "UserQueries.java", &JavaExtractConfig::default());
    let constants: Vec<_> = result
        .extractions
        .iter()
        .filter(|e| e.origin.method == ExtractionMethod::Constant)
        .collect();
    assert_eq!(constants.len(), 2);
    assert_eq!(
        constants[0].sql.trim(),
        "SELECT id, name, email FROM users WHERE id = __JAVA_VAR_JDBC_PARAM_1__"
    );
    assert_eq!(
        constants[1].sql.trim(),
        "INSERT INTO users (name, email) VALUES (__JAVA_VAR_JDBC_PARAM_1__, __JAVA_VAR_JDBC_PARAM_2__)"
    );
}

#[test]
fn test_placeholder_inside_string_literal_preserved() {
    let java = r#"
        public class UserDao {
            public void query(String id) {
                PreparedStatement ps = conn.prepareStatement(
                    "SELECT * FROM users WHERE name = '?unknown' AND id = ?");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "UserDao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert!(ext.sql.contains("'?unknown'"), "String literal ? should be preserved");
    assert!(ext.sql.contains("__JAVA_VAR_JDBC_PARAM_1__"), "Non-literal ? should be replaced");
}

#[test]
fn test_query_annotation_native_sql() {
    let java = r#"
        public interface UserRepository {
            @Query(value = "SELECT * FROM users WHERE status = :status", nativeQuery = true)
            List<User> findByStatus(@Param("status") int status);
        }
    "#;
    let result = extract_sql_from_java(java, "UserRepository.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(ext.sql.trim(), "SELECT * FROM users WHERE status = __JAVA_VAR_status__");
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
    let result = extract_sql_from_java(java, "UserRepository.java", &JavaExtractConfig::default());
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
    let result = extract_sql_from_java(java, "UserRepository.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty());
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(
        result.extractions[0].sql.trim(),
        "SELECT * FROM users WHERE status = __JAVA_VAR_status__"
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
    let result = extract_sql_from_java(java, "UserRepository.java", &JavaExtractConfig::default());
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
    let result = extract_sql_from_java(java, "User.java", &JavaExtractConfig::default());
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
    let result = extract_sql_from_java(java, "UserDao.java", &JavaExtractConfig::default());
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
    let result = extract_sql_from_java(java, "UserRepository.java", &JavaExtractConfig::default());
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
    let result = extract_sql_from_java(java, "Calculator.java", &JavaExtractConfig::default());
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
    let result = extract_sql_from_java(java, "UserService.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(ext.sql.trim(), "SELECT * FROM users WHERE id = __JAVA_VAR_JDBC_PARAM_1__");
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
    let result = extract_sql_from_java(java, "UserService.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert!(ext.sql.contains("DELETE FROM users"));
    assert!(ext.sql.contains("__JAVA_VAR_JDBC_PARAM_1__"));
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
    let result = extract_sql_from_java(java, "UserRepository.java", &JavaExtractConfig::default());
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
    let result = extract_sql_from_java(java, "MyService.java", &JavaExtractConfig::default());
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
    let result = extract_sql_from_java(java, "UserService.java", &JavaExtractConfig::default());
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
    let result = extract_sql_from_java(java, "UserQueries.java", &JavaExtractConfig::default());
    let constants: Vec<_> = result
        .extractions
        .iter()
        .filter(|e| e.origin.method == ExtractionMethod::Constant)
        .collect();
    assert_eq!(constants.len(), 2);
    assert!(constants[0].sql.contains("SELECT"));
    assert!(constants[1].sql.contains("INSERT"));
    assert!(constants[0].sql.contains("__JAVA_VAR_JDBC_PARAM_1__"));
    assert!(constants[1].sql.contains("__JAVA_VAR_JDBC_PARAM_1__"));
    assert!(constants[1].sql.contains("__JAVA_VAR_JDBC_PARAM_2__"));
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
    let result = extract_sql_from_java(java, "UserQueries.java", &JavaExtractConfig::default());
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
    let result = extract_sql_from_java(java, "Config.java", &JavaExtractConfig::default());
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
    let result = extract_sql_from_java(java, "Q.java", &JavaExtractConfig::default());
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
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(
        ext.sql,
        "select a from t where id='__JAVA_VAR_String_mail__'"
    );
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
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(
        ext.sql,
        "select * from t where id=__JAVA_VAR_int_id__ and name='__JAVA_VAR_String_name__'"
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
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(
        ext.sql,
        "select * from t where id=__JAVA_VAR_int_id__ and name='__JAVA_VAR_String_name__' and status='__JAVA_VAR_String_status__'"
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
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
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
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.is_empty());
}

#[test]
fn test_reassignment_creates_new_extraction() {
    let java = r#"
        public class Dao {
            public void query() {
                String sql = "select name, value, result from t1";
                sql = "update t1 set name = 'tom'";
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 2);
    assert_eq!(
        result.extractions[0].sql,
        "select name, value, result from t1"
    );
    assert_eq!(result.extractions[1].sql, "update t1 set name = 'tom'");
}

#[test]
fn test_reassignment_then_concat() {
    let java = r#"
        public class Dao {
            public void query(int id) {
                String sql = "select * from t1";
                sql = "update t1 set name = 'tom'";
                sql = sql + " where id = " + id;
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 2);
    assert_eq!(result.extractions[0].sql, "select * from t1");
    assert!(result.extractions[1]
        .sql
        .contains("update t1 set name = 'tom'"));
    assert!(result.extractions[1].sql.contains("where id = "));
    assert!(result.extractions[1].is_concatenated);
}

#[test]
fn test_extra_sql_methods() {
    let java = r#"
        public class CustomDao {
            public void findUsers() {
                db.findNativeQuery("SELECT * FROM users WHERE active = 1");
            }
        }
    "#;

    let result = extract_sql_from_java(java, "CustomDao.java", &JavaExtractConfig::default());
    let method_extractions: Vec<_> = result
        .extractions
        .iter()
        .filter(|e| e.origin.method == ExtractionMethod::MethodCall)
        .collect();
    assert!(
        method_extractions.is_empty(),
        "Should not extract without extra methods"
    );

    let config = JavaExtractConfig {
        extra_sql_methods: vec!["findNativeQuery".to_string()],
    };
    let result = extract_sql_from_java(java, "CustomDao.java", &config);
    let method_extractions: Vec<_> = result
        .extractions
        .iter()
        .filter(|e| e.origin.method == ExtractionMethod::MethodCall)
        .collect();
    assert_eq!(method_extractions.len(), 1);
    assert_eq!(
        method_extractions[0].origin.api_method_name.as_deref(),
        Some("findNativeQuery")
    );
    assert!(method_extractions[0].sql.contains("SELECT * FROM users"));
}

// ── Annotation Expansion Tests (Phase 1.2) ──

#[test]
fn test_mybatis_select_annotation() {
    let java = r#"
        public interface UserMapper {
            @Select("SELECT * FROM users WHERE status = #{status}")
            List<User> findByStatus(@Param("status") String status);
        }
    "#;
    let result = extract_sql_from_java(java, "UserMapper.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert_eq!(ext.origin.annotation_name.as_deref(), Some("Select"));
    assert_eq!(ext.sql_kind, SqlKind::NativeSql);
    assert!(ext.sql.contains("SELECT * FROM users"));
}

#[test]
fn test_mybatis_insert_annotation() {
    let java = r#"
        public interface UserMapper {
            @Insert("INSERT INTO users (name, email) VALUES (#{name}, #{email})")
            void insert(@Param("name") String name, @Param("email") String email);
        }
    "#;
    let result = extract_sql_from_java(java, "UserMapper.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(result.extractions[0].origin.annotation_name.as_deref(), Some("Insert"));
}

#[test]
fn test_mybatis_update_annotation() {
    let java = r#"
        public interface UserMapper {
            @Update("UPDATE users SET name = #{name} WHERE id = #{id}")
            void updateName(@Param("id") Long id, @Param("name") String name);
        }
    "#;
    let result = extract_sql_from_java(java, "UserMapper.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(result.extractions[0].origin.annotation_name.as_deref(), Some("Update"));
}

#[test]
fn test_mybatis_delete_annotation() {
    let java = r#"
        public interface UserMapper {
            @Delete("DELETE FROM users WHERE id = #{id}")
            void delete(@Param("id") Long id);
        }
    "#;
    let result = extract_sql_from_java(java, "UserMapper.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(result.extractions[0].origin.annotation_name.as_deref(), Some("Delete"));
}

#[test]
fn test_hibernate_named_native_query() {
    let java = r#"
        @NamedNativeQuery(name = "User.findAll", query = "SELECT * FROM users")
        public class User { }
    "#;
    let result = extract_sql_from_java(java, "User.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(result.extractions[0].origin.annotation_name.as_deref(), Some("NamedNativeQuery"));
    assert_eq!(result.extractions[0].sql_kind, SqlKind::NativeSql);
}

#[test]
fn test_jdbi_sql_batch_annotation() {
    let java = r#"
        public interface UserDao {
            @SqlBatch("INSERT INTO users (name) VALUES (:name)")
            void insertAll(@Bind("name") List<String> names);
        }
    "#;
    let result = extract_sql_from_java(java, "UserDao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(result.extractions[0].origin.annotation_name.as_deref(), Some("SqlBatch"));
}

// ── Method Name Expansion Tests (Phase 1.3) ──

#[test]
fn test_spring_query_for_object() {
    let java = r#"
        public class UserDao {
            public User findById(String id) {
                return jdbcTemplate.queryForObject("SELECT * FROM users WHERE id = ?", User.class);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "UserDao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(result.extractions[0].origin.api_method_name.as_deref(), Some("queryForObject"));
    assert!(result.extractions[0].sql.contains("SELECT * FROM users"));
}

#[test]
fn test_spring_query_for_list() {
    let java = r#"
        public class UserDao {
            public List<User> findAll() {
                return jdbcTemplate.queryForList("SELECT * FROM users", User.class);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "UserDao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(result.extractions[0].origin.api_method_name.as_deref(), Some("queryForList"));
}

#[test]
fn test_spring_batch_update() {
    let java = r#"
        public class UserDao {
            public void batchInsert() {
                jdbcTemplate.batchUpdate("INSERT INTO users (name) VALUES (?)");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "UserDao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(result.extractions[0].origin.api_method_name.as_deref(), Some("batchUpdate"));
}

// ── StringBuilder Tests (Phase 2) ──

#[test]
fn test_string_builder_basic_append() {
    let java = r#"
        public class Dao {
            public void query(int id) {
                StringBuilder sql = new StringBuilder("SELECT * FROM users");
                sql.append(" WHERE id = ").append(id);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(
        result.extractions[0].sql,
        "SELECT * FROM users WHERE id = __JAVA_VAR_int_id__"
    );
    assert!(result.extractions[0].is_concatenated);
}

#[test]
fn test_string_builder_empty_init() {
    let java = r#"
        public class Dao {
            public void query(String table) {
                StringBuilder sql = new StringBuilder();
                sql.append("SELECT * FROM ").append(table);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("SELECT * FROM"));
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_String_table__"));
}

#[test]
fn test_string_builder_multi_step() {
    let java = r#"
        public class Dao {
            public void query(int id, String name) {
                StringBuilder sql = new StringBuilder("SELECT * FROM users WHERE 1=1");
                sql.append(" AND id = ").append(id);
                sql.append(" AND name = '").append(name).append("'");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(
        result.extractions[0].sql,
        "SELECT * FROM users WHERE 1=1 AND id = __JAVA_VAR_int_id__ AND name = '__JAVA_VAR_String_name__'"
    );
}

#[test]
fn test_string_builder_insert() {
    let java = r#"
        public class Dao {
            public void query(String extra) {
                StringBuilder sql = new StringBuilder("SELECT FROM users");
                sql.insert(7, "* ");
                sql.append(" WHERE 1=1");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("SELECT * FROM users"));
}

#[test]
fn test_string_builder_delete() {
    let java = r#"
        public class Dao {
            public void query() {
                StringBuilder sql = new StringBuilder("SELECT * FROM users WHERE obsolete");
                sql.delete(19, 34);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(result.extractions[0].sql.trim(), "SELECT * FROM users");
}

#[test]
fn test_string_buffer_basic() {
    let java = r#"
        public class Dao {
            public void query(int id) {
                StringBuffer sql = new StringBuffer("SELECT * FROM t");
                sql.append(" WHERE id = ").append(id);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("SELECT * FROM t"));
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_int_id__"));
}

#[test]
fn test_string_builder_method_scoped() {
    let java = r#"
        public class Dao {
            public void methodA() {
                StringBuilder sql = new StringBuilder("SELECT * FROM a");
            }
            public void methodB() {
                StringBuilder sql = new StringBuilder("SELECT * FROM b");
                sql.append(" WHERE id = 1");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 2);
    assert_eq!(result.extractions[0].sql, "SELECT * FROM a");
    assert_eq!(result.extractions[1].sql, "SELECT * FROM b WHERE id = 1");
}

#[test]
fn test_string_builder_non_sql_ignored() {
    let java = r#"
        public class Dao {
            public void build() {
                StringBuilder msg = new StringBuilder("Hello");
                msg.append(" World");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.is_empty());
}

// ── Phase 4: Test Coverage Hardening ──

#[test]
fn test_placeholder_colon_with_underscore() {
    let java = r#"
        public interface Repo {
            @Query(value = "SELECT * FROM t WHERE x = :my_param", nativeQuery = true)
            List<User> find();
        }
    "#;
    let result = extract_sql_from_java(java, "Repo.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_my_param__"));
}

#[test]
fn test_named_native_query_with_query_key() {
    let java = r#"
        @NamedNativeQuery(name = "User.findById", query = "SELECT * FROM users WHERE id = ?")
        public class User {}
    "#;
    let result = extract_sql_from_java(java, "User.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_JDBC_PARAM_1__"));
}

#[test]
fn test_native_query_false_yields_jpql() {
    let java = r#"
        public interface Repo {
            @Query(value = "SELECT u FROM User u WHERE u.name = :name", nativeQuery = false)
            List<User> findByName();
        }
    "#;
    let result = extract_sql_from_java(java, "Repo.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(result.extractions[0].sql_kind, SqlKind::Jpql);
}

#[test]
fn test_sql_query_annotation() {
    let java = r#"
        public class Dao {
            @SqlQuery("SELECT * FROM users WHERE active = 1")
            public List<User> findActive() { return null; }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("SELECT * FROM users"));
}

#[test]
fn test_execute_query() {
    let java = r#"
        public class Dao {
            public void run() {
                ResultSet rs = stmt.executeQuery("SELECT * FROM products");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("SELECT * FROM products"));
}

#[test]
fn test_execute_update() {
    let java = r#"
        public class Dao {
            public void run() {
                int n = stmt.executeUpdate("DELETE FROM temp_data");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("DELETE FROM temp_data"));
}

#[test]
fn test_ambiguous_method_with_sql_content() {
    let java = r#"
        public class Dao {
            public void run() {
                jdbc.query("SELECT * FROM users");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
}

#[test]
fn test_ambiguous_method_without_sql_content() {
    let java = r#"
        public class Dao {
            public void run() {
                obj.query("calculateMetrics");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.extractions.is_empty());
}

#[test]
fn test_interface_declaration() {
    let java = r#"
        public interface UserRepository {
            @Query("SELECT * FROM users WHERE id = :id")
            User findById(@Param("id") int id);
        }
    "#;
    let result = extract_sql_from_java(java, "UserRepository.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_id__"));
}

#[test]
fn test_text_block_indentation() {
    let java = r#"
        public class Dao {
            public void run() {
                String sql = """
                    SELECT *
                    FROM users
                    WHERE id = 1
                    """;
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("SELECT *"));
    assert!(sql.contains("FROM users"));
    assert!(sql.contains("WHERE id = 1"));
    assert!(!sql.starts_with("    "));
}

#[test]
fn test_escape_sequences_in_string() {
    let java = r#"
        public class Dao {
            public void run() {
                String sql = "SELECT * FROM users WHERE name = 'O\'Brien'";
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("O'Brien"));
}

#[test]
fn test_empty_java_file() {
    let java = "// empty file";
    let result = extract_sql_from_java(java, "Empty.java", &JavaExtractConfig::default());
    assert!(result.extractions.is_empty());
    assert!(result.errors.is_empty());
}

#[test]
fn test_syntax_error_in_java() {
    let java = "public class { }";
    let result = extract_sql_from_java(java, "Broken.java", &JavaExtractConfig::default());
    assert!(result.extractions.is_empty());
}

#[test]
fn test_placeholder_in_field_constant() {
    let java = r#"
        public class Dao {
            private static final String SQL = "SELECT * FROM users WHERE id = ?";
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_JDBC_PARAM_1__"));
}
