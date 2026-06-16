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
    assert_eq!(ext.sql.trim(), "SELECT * FROM users WHERE id = __JAVA_VAR_JDBC_PARAM_1__");
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
    assert_eq!(ext.sql.trim(), "SELECT * FROM users WHERE status = __JAVA_VAR_int_status__");
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
        "INSERT INTO users (name, email) VALUES (__JAVA_VAR_String_name__, __JAVA_VAR_String_email__)"
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
    let constants: Vec<_> =
        result.extractions.iter().filter(|e| e.origin.method == ExtractionMethod::Constant).collect();
    assert_eq!(constants.len(), 2);
    assert_eq!(constants[0].sql.trim(), "SELECT id, name, email FROM users WHERE id = __JAVA_VAR_JDBC_PARAM_1__");
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
    assert_eq!(ext.sql.trim(), "SELECT * FROM users WHERE status = __JAVA_VAR_int_status__");
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
    assert_eq!(result.extractions[0].sql.trim(), "SELECT * FROM users WHERE status = __JAVA_VAR_int_status__");
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
    assert_eq!(result.extractions[0].origin.annotation_name.as_deref(), Some("NamedQuery"));
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
    assert_eq!(ext.origin.api_method_name.as_deref(), Some("createNativeQuery"));
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
    assert!(ext.sql.contains("__JAVA_VAR_String_id__"), "SQL: {}", ext.sql);
    assert_eq!(ext.origin.api_method_name.as_deref(), Some("prepareStatement"));
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
    let method_extractions: Vec<_> =
        result.extractions.iter().filter(|e| e.origin.method == ExtractionMethod::MethodCall).collect();
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
    let constants: Vec<_> =
        result.extractions.iter().filter(|e| e.origin.method == ExtractionMethod::Constant).collect();
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
    let constants: Vec<_> =
        result.extractions.iter().filter(|e| e.origin.method == ExtractionMethod::Constant).collect();
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
    let constants: Vec<_> =
        result.extractions.iter().filter(|e| e.origin.method == ExtractionMethod::Constant).collect();
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
    let constants: Vec<_> =
        result.extractions.iter().filter(|e| e.origin.method == ExtractionMethod::Constant).collect();
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
    assert_eq!(ext.sql, "select a from t where id='__JAVA_RAW_String_mail__'");
    assert!(ext.is_concatenated);
    assert!(ext
        .parse_result
        .as_ref()
        .map_or(false, |r| r.errors.iter().all(|e| matches!(e, crate::parser::ParserError::Warning { .. }))));
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
    assert_eq!(ext.sql, "select * from t where id=__JAVA_RAW_int_id__ and name='__JAVA_RAW_String_name__'");
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
        "select * from t where id=__JAVA_RAW_int_id__ and name='__JAVA_RAW_String_name__' and status='__JAVA_RAW_String_status__'"
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
    assert_eq!(result.extractions[0].sql, "select name, value, result from t1");
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
    assert!(result.extractions[1].sql.contains("update t1 set name = 'tom'"));
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
    let method_extractions: Vec<_> =
        result.extractions.iter().filter(|e| e.origin.method == ExtractionMethod::MethodCall).collect();
    assert!(method_extractions.is_empty(), "Should not extract without extra methods");

    let config = JavaExtractConfig { extra_sql_methods: vec!["findNativeQuery".to_string()], ..Default::default() };
    let result = extract_sql_from_java(java, "CustomDao.java", &config);
    let method_extractions: Vec<_> =
        result.extractions.iter().filter(|e| e.origin.method == ExtractionMethod::MethodCall).collect();
    assert_eq!(method_extractions.len(), 1);
    assert_eq!(method_extractions[0].origin.api_method_name.as_deref(), Some("findNativeQuery"));
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
    assert_eq!(result.extractions[0].sql, "SELECT * FROM users WHERE id = __JAVA_RAW_int_id__");
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
    assert!(result.extractions[0].sql.contains("__JAVA_RAW_String_table__"));
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
        "SELECT * FROM users WHERE 1=1 AND id = __JAVA_RAW_int_id__ AND name = '__JAVA_RAW_String_name__'"
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
    assert!(result.extractions[0].sql.contains("__JAVA_RAW_int_id__"));
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
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_int_id__"));
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

#[test]
fn test_mybatis_hash_placeholder_converted() {
    let java = r#"
        public interface UserMapper {
            @Select("select * from T_USERS where USER_NAME = #{username}")
            UserInfo selectUser(@Param("username") String username);
        }
    "#;
    let result = extract_sql_from_java(java, "UserMapper.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert_eq!(
        result.extractions[0].sql.trim(),
        "select * from T_USERS where USER_NAME = __JAVA_VAR_String_username__"
    );
    assert!(result.extractions[0].parse_result.is_some());
    let parse_result = result.extractions[0].parse_result.as_ref().unwrap();
    assert!(parse_result.errors.is_empty(), "Parse errors: {:?}", parse_result.errors);
}

#[test]
fn test_mybatis_dollar_placeholder_converted() {
    let java = r#"
        public interface Mapper {
            @Select("SELECT * FROM ${tableName} WHERE id = #{id}")
            List<Map> findAll(@Param("tableName") String table, @Param("id") int id);
        }
    "#;
    let result = extract_sql_from_java(java, "Mapper.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("__JAVA_RAW_tableName__"));
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_int_id__"));
}

#[test]
fn test_parse_mapper_interface_basic() {
    let source = r#"
package com.example.mapper;

public interface UserMapper {
    User findById(int id);
    List<User> findByName(String name);
    void insert(User user);
}
"#;
    let info = crate::java::parse_mapper_interface(source).unwrap();
    assert_eq!(info.fqn, "com.example.mapper.UserMapper");
    assert!(info.methods.contains_key("findById"));
    let method = &info.methods["findById"];
    assert_eq!(method.params.len(), 1);
    assert_eq!(method.params[0].name, "id");
    assert_eq!(method.params[0].java_type, "int");
}

#[test]
fn test_parse_mapper_interface_with_param_annotation() {
    let source = r#"
package com.example.mapper;

public interface UserMapper {
    List<User> search(@Param("status") int status, @Param("name") String name);
}
"#;
    let info = crate::java::parse_mapper_interface(source).unwrap();
    let method = &info.methods["search"];
    assert_eq!(method.params.len(), 2);
    assert_eq!(method.params[0].name, "status");
    assert_eq!(method.params[0].java_type, "int");
    assert_eq!(method.params[0].param_annotation, Some("status".into()));
    assert_eq!(method.params[1].name, "name");
    assert_eq!(method.params[1].java_type, "String");
}

#[test]
fn test_parse_mapper_interface_not_interface() {
    let source = "public class Foo { }";
    assert!(crate::java::parse_mapper_interface(source).is_none());
}

#[test]
fn test_parse_dto_fields() {
    let source = r#"
package com.example.model;

public class User {
    private Long id;
    private String name;
    private Integer age;
    private BigDecimal salary;
    private Date createTime;
    private boolean active;
}
"#;
    let fields = crate::java::parse_dto_fields(source);
    assert_eq!(fields.get("id").unwrap(), "Long");
    assert_eq!(fields.get("name").unwrap(), "String");
    assert_eq!(fields.get("age").unwrap(), "Integer");
    assert_eq!(fields.get("salary").unwrap(), "BigDecimal");
    assert_eq!(fields.get("createTime").unwrap(), "Date");
    assert_eq!(fields.get("active").unwrap(), "boolean");
}

// ── JDBC Parameter Type Inference Tests ──

#[test]
fn test_jdbc_setString_inference() {
    let java = r#"
        public class UserDao {
            public void insert(String seqId, String zipName) {
                String sqlSub = "INSERT INTO dat_mail_aas_attachment (SEQ_ID,FILE_NAME,FILE_CONTENT) VALUES (?,?, empty_blob())";
                PreparedStatement st = conn.prepareStatement(sqlSub);
                st.setString(1, seqId);
                st.setString(2, zipName);
                st.execute();
            }
        }
    "#;
    let result = extract_sql_from_java(java, "UserDao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    let ext = result.extractions.iter().find(|e| e.sql.contains("INSERT INTO")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_String_seqId__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_zipName__"), "SQL: {}", ext.sql);
}

#[test]
fn test_jdbc_setInt_inference() {
    let java = r#"
        public class Dao {
            public void query(int id, String name) {
                PreparedStatement ps = conn.prepareStatement("SELECT * FROM t WHERE id = ? AND name = ?");
                ps.setInt(1, id);
                ps.setString(2, name);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    let ext = result.extractions.iter().find(|e| e.sql.contains("SELECT")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_int_id__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_name__"), "SQL: {}", ext.sql);
}

#[test]
fn test_jdbc_multiple_types() {
    let java = r#"
        public class Dao {
            public void insert(String name, int age, BigDecimal salary) {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO emp (name, age, salary) VALUES (?, ?, ?)");
                ps.setString(1, name);
                ps.setInt(2, age);
                ps.setBigDecimal(3, salary);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    let ext = result.extractions.iter().find(|e| e.sql.contains("INSERT")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_String_name__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_int_age__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_BigDecimal_salary__"), "SQL: {}", ext.sql);
}

#[test]
fn test_jdbc_no_setter_falls_back_to_param_n() {
    let java = r#"
        public class Dao {
            public void query() {
                PreparedStatement ps = conn.prepareStatement("SELECT * FROM t WHERE id = ?");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    let ext = result.extractions.iter().find(|e| e.sql.contains("SELECT")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_JDBC_PARAM_1__"), "SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_String_"), "SQL: {}", ext.sql);
}

#[test]
fn test_jdbc_partial_setter_inference() {
    let java = r#"
        public class Dao {
            public void query(String name, int age) {
                PreparedStatement ps = conn.prepareStatement("SELECT * FROM t WHERE name = ? AND age = ?");
                ps.setString(1, name);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    let ext = result.extractions.iter().find(|e| e.sql.contains("SELECT")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_String_name__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_JDBC_PARAM_2__"), "SQL: {}", ext.sql);
}

#[test]
fn test_jdbc_setter_out_of_order() {
    let java = r#"
        public class Dao {
            public void query(String name, int id) {
                PreparedStatement ps = conn.prepareStatement("SELECT * FROM t WHERE id = ? AND name = ?");
                ps.setString(2, name);
                ps.setInt(1, id);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    let ext = result.extractions.iter().find(|e| e.sql.contains("SELECT")).unwrap();
    assert!(ext.sql.contains("id = __JAVA_VAR_int_id__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("name = __JAVA_VAR_String_name__"), "SQL: {}", ext.sql);
}

#[test]
fn test_jdbc_two_prepared_statements_same_method() {
    let java = r#"
        public class Dao {
            public void batch(String a, String b) {
                PreparedStatement ps1 = conn.prepareStatement("INSERT INTO t1 (v) VALUES (?)");
                ps1.setString(1, a);
                PreparedStatement ps2 = conn.prepareStatement("INSERT INTO t2 (v) VALUES (?)");
                ps2.setString(1, b);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    let insert_t1 = result.extractions.iter().find(|e| e.sql.contains("t1")).unwrap();
    let insert_t2 = result.extractions.iter().find(|e| e.sql.contains("t2")).unwrap();
    assert!(insert_t1.sql.contains("__JAVA_VAR_String_a__"), "SQL: {}", insert_t1.sql);
    assert!(insert_t2.sql.contains("__JAVA_VAR_String_b__"), "SQL: {}", insert_t2.sql);
}

#[test]
fn test_jdbc_exact_user_example() {
    let java = r#"
        public class AutoSendReport {
            public void run(String seqId, String zipName) {
                String sqlSub="INSERT INTO dat_mail_aas_attachment (SEQ_ID,FILE_NAME,FILE_CONTENT) VALUES (?,?, empty_blob())";
                PreparedStatement st = conn.prepareStatement(sqlSub);
                st.setString(1, seqId);
                st.setString(2, zipName);
                st.execute();
                conn.commit();
            }
        }
    "#;
    let result = extract_sql_from_java(java, "AutoSendReport.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    let ext = result.extractions.iter().find(|e| e.sql.contains("INSERT INTO dat_mail_aas_attachment")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_String_seqId__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_zipName__"), "SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"), "SQL: {}", ext.sql);
}

#[test]
fn test_jdbc_setter_with_literal_value() {
    let java = r#"
        public class Dao {
            public void query() {
                PreparedStatement ps = conn.prepareStatement("SELECT * FROM t WHERE status = ?");
                ps.setString(1, "active");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    let ext = result.extractions.iter().find(|e| e.sql.contains("SELECT")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_String_JDBC_PARAM_1__"), "SQL: {}", ext.sql);
}

#[test]
fn test_jdbc_sql_constant_with_prepare_statement() {
    let java = r#"
        public class Dao {
            public void query(int id) {
                String sql = "SELECT * FROM t WHERE id = ?";
                PreparedStatement ps = conn.prepareStatement(sql);
                ps.setInt(1, id);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    let ext = result.extractions.iter().find(|e| e.sql.contains("SELECT")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_int_id__"), "SQL: {}", ext.sql);
}

#[test]
fn test_jdbc_ambiguous_method_sql_not_affected() {
    let java = r#"
        public class Dao {
            public void run() {
                jdbc.query("SELECT * FROM users WHERE id = ?");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("__JAVA_VAR_JDBC_PARAM_1__"));
}

#[test]
fn test_jdbc_reused_ps_variable_both_backfilled() {
    let java = r#"
        public class Dao {
            public void batch(String seqId, String zipName, String status) {
                String sql1 = "INSERT INTO dat_mail_aas_attachment (SEQ_ID,FILE_NAME,FILE_CONTENT) VALUES (?,?, empty_blob())";
                PreparedStatement st = conn.prepareStatement(sql1);
                st.setString(1, seqId);
                st.setString(2, zipName);
                st.execute();

                String sql2 = "UPDATE dat_mail_aas_attachment SET FILE_NAME = ? WHERE SEQ_ID = ?";
                st = conn.prepareStatement(sql2);
                st.setString(1, zipName);
                st.setString(2, seqId);
                st.executeUpdate();
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    let insert_ext = result.extractions.iter().find(|e| e.sql.contains("INSERT INTO")).unwrap();
    assert!(
        insert_ext.sql.contains("__JAVA_VAR_String_seqId__"),
        "INSERT should have backfilled param 1, got: {}",
        insert_ext.sql
    );
    assert!(
        insert_ext.sql.contains("__JAVA_VAR_String_zipName__"),
        "INSERT should have backfilled param 2, got: {}",
        insert_ext.sql
    );
    assert!(
        !insert_ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"),
        "INSERT should have no unresolved placeholders, got: {}",
        insert_ext.sql
    );

    let update_ext = result.extractions.iter().find(|e| e.sql.contains("UPDATE")).unwrap();
    assert!(
        update_ext.sql.contains("__JAVA_VAR_String_zipName__"),
        "UPDATE should have backfilled param 1, got: {}",
        update_ext.sql
    );
    assert!(
        update_ext.sql.contains("__JAVA_VAR_String_seqId__"),
        "UPDATE should have backfilled param 2, got: {}",
        update_ext.sql
    );
}

#[test]
fn test_jdbc_reused_ps_variable_three_rounds() {
    let java = r#"
        public class Dao {
            public void multi(String x, String y, String z) {
                String sql1 = "INSERT INTO t1 (v) VALUES (?)";
                PreparedStatement ps = conn.prepareStatement(sql1);
                ps.setString(1, x);
                ps.execute();

                String sql2 = "INSERT INTO t2 (v) VALUES (?)";
                ps = conn.prepareStatement(sql2);
                ps.setString(1, y);
                ps.execute();

                String sql3 = "INSERT INTO t3 (v) VALUES (?)";
                ps = conn.prepareStatement(sql3);
                ps.setString(1, z);
                ps.execute();
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 3);

    let t1 = result.extractions.iter().find(|e| e.sql.contains("t1")).unwrap();
    assert!(t1.sql.contains("__JAVA_VAR_String_x__"), "t1: {}", t1.sql);

    let t2 = result.extractions.iter().find(|e| e.sql.contains("t2")).unwrap();
    assert!(t2.sql.contains("__JAVA_VAR_String_y__"), "t2: {}", t2.sql);

    let t3 = result.extractions.iter().find(|e| e.sql.contains("t3")).unwrap();
    assert!(t3.sql.contains("__JAVA_VAR_String_z__"), "t3: {}", t3.sql);
}

// ── String[] Argument Mapping Tests (Phase 1) ──

#[test]
fn test_string_array_arg_sql_constant_single_param() {
    let java = r#"
        public class Dao {
            private static final String SQL = "SELECT * FROM t WHERE id = ?";
            public void query(String nodeId) {
                DbService.executeQuery("ORACLEJDBC", SQL, new String[] {nodeId});
            }
        }
    "#;
    let config = JavaExtractConfig { extra_sql_methods: vec!["executeQuery".to_string()], ..Default::default() };
    let result = extract_sql_from_java(java, "Dao.java", &config);
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    let ext = result.extractions.iter().find(|e| e.sql.contains("SELECT")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_String_nodeId__"), "SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"), "SQL: {}", ext.sql);
}

#[test]
fn test_string_array_arg_multiple_params() {
    let java = r#"
        public class Dao {
            private static final String SQL = "SELECT * FROM t WHERE a = ? AND b = ?";
            public void query(String x, String y) {
                DbService.executeQuery("DB", SQL, new String[] {x, y});
            }
        }
    "#;
    let config = JavaExtractConfig { extra_sql_methods: vec!["executeQuery".to_string()], ..Default::default() };
    let result = extract_sql_from_java(java, "Dao.java", &config);
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    let ext = result.extractions.iter().find(|e| e.sql.contains("SELECT")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_String_x__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_y__"), "SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"), "SQL: {}", ext.sql);
}

#[test]
fn test_object_array_arg_typed_params() {
    let java = r#"
        public class Dao {
            private static final String SQL = "SELECT * FROM t WHERE id = ?";
            public void query(int id) {
                DbService.executeQuery("DB", SQL, new Object[] {id});
            }
        }
    "#;
    let config = JavaExtractConfig { extra_sql_methods: vec!["executeQuery".to_string()], ..Default::default() };
    let result = extract_sql_from_java(java, "Dao.java", &config);
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    let ext = result.extractions.iter().find(|e| e.sql.contains("SELECT")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_int_id__"), "SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"), "SQL: {}", ext.sql);
}

#[test]
fn test_inline_sql_with_inline_string_array() {
    let java = r#"
        public class Dao {
            public void query(String nodeId) {
                DbService.executeQuery("DB", "SELECT * FROM t WHERE id = ?", new String[] {nodeId});
            }
        }
    "#;
    let config = JavaExtractConfig { extra_sql_methods: vec!["executeQuery".to_string()], ..Default::default() };
    let result = extract_sql_from_java(java, "Dao.java", &config);
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    let ext = result.extractions.iter().find(|e| e.sql.contains("SELECT")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_String_nodeId__"), "SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"), "SQL: {}", ext.sql);
}

#[test]
fn test_real_world_integration_ebms_handler() {
    let java = r#"
        public class EBMSHandler {
            private static final String SWITCH_SQL = "SELECT KIND_ID FROM ebk_dic_all_kind t WHERE t.operation_kind = 'PI00168_SWITCH'";
            private static final String QUERY_MENU_SQL = "SELECT t.node_id, t.node_name, t.edition FROM par_netuser_menu_tree t WHERE t.node_id = ?";
            public int execute(String node) throws Exception {
                List list1 = DbService.executeQuery("ORACLEJDBC", SWITCH_SQL);
                List list2 = DbService.executeQuery("ORACLEJDBC", QUERY_MENU_SQL, new String[] {node});
                return 0;
            }
        }
    "#;
    let config = JavaExtractConfig { extra_sql_methods: vec!["executeQuery".to_string()], ..Default::default() };
    let result = extract_sql_from_java(java, "EBMSHandler.java", &config);
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);

    let switch_ext = result.extractions.iter().find(|e| e.sql.contains("SWITCH")).unwrap();
    assert!(
        !switch_ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"),
        "SWITCH_SQL should have no unresolved params, got: {}",
        switch_ext.sql
    );

    let menu_ext = result.extractions.iter().find(|e| e.sql.contains("node_id")).unwrap();
    assert!(
        menu_ext.sql.contains("__JAVA_VAR_String_node__"),
        "QUERY_MENU_SQL should have backfilled node param, got: {}",
        menu_ext.sql
    );
    assert!(
        !menu_ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"),
        "QUERY_MENU_SQL should have no unresolved placeholders, got: {}",
        menu_ext.sql
    );
}

// ── Cross-Method PreparedStatement Flow Tests (Phase 2) ──

#[test]
fn test_cross_method_ps_passing_literal_setters() {
    let java = r#"
        public class Dao {
            public void process(String name, String email) {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO t (name, email) VALUES (?, ?)");
                insertData(ps, name, email);
            }

            public static void insertData(PreparedStatement ps, String name, String email) {
                ps.setString(1, name);
                ps.setString(2, email);
                ps.execute();
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    let ext = result.extractions.iter().find(|e| e.sql.contains("INSERT INTO t")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_String_name__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_email__"), "SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"), "SQL: {}", ext.sql);
}

#[test]
fn test_cross_method_dynamic_loop_setter() {
    let java = r#"
        public class Dao {
            public void process(List list) throws Exception {
                PreparedStatement ps = conn.prepareStatement("INSERT INTO t (a,b,c) VALUES (?,?,?)");
                submitData(ps, list);
            }

            public static void submitData(PreparedStatement ps, List list) throws Exception {
                for (Iterator it = list.iterator(); it.hasNext();) {
                    String[] s = (String[]) it.next();
                    for (int i = 0; i < s.length; i++) {
                        ps.setString(i + 1, s[i]);
                    }
                    ps.addBatch();
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    let ext = result.extractions.iter().find(|e| e.sql.contains("INSERT INTO t")).unwrap();
    assert!(ext.sql.contains("__JAVA_VAR_String_DYNAMIC_1__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_DYNAMIC_2__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_DYNAMIC_3__"), "SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"), "SQL: {}", ext.sql);
}

#[test]
fn test_user_scenario_submit_data_cross_method() {
    let java = r#"
        public class DataProcessor {
            public static void submitData(PreparedStatement ps, List list) throws Exception {
                try {
                    for (Iterator it = list.iterator(); it.hasNext();) {
                        String[] s = (String[]) it.next();
                        for (int i = 0; i < s.length; i++) {
                            ps.setString(i + 1, s[i]);
                        }
                        ps.addBatch();
                    }
                    ps.executeBatch();
                } catch (Exception e) {
                    e.printStackTrace();
                }
            }

            public void process(List list, String accno) throws Exception {
                ps = conn.prepareStatement("insert into dat_clnt_fv_tran " +
                        "(ACCNO,BUSIDATE,SERIALNO,TRXCODE,DRCRF,SUMMARY,AMOUNT,BALANCE,RECIPACC,RECIPNAM,NOTES)" +
                        "VALUES(lpad(" + accno + ",19,0),?,?,?,?,?,?,?,?,?,?)");
                if(list.size()>0){
                    submitData(ps, list);
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "DataProcessor.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert!(ext.sql.contains("__JAVA_RAW_String_accno__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_DYNAMIC_1__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_DYNAMIC_10__"), "SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"), "SQL: {}", ext.sql);
}

#[test]
fn test_undeclared_var_in_string_concat_gets_string_type() {
    let java = r#"
        public class Foo {
            public void bar(Connection conn) throws Exception {
                PreparedStatement ps = conn.prepareStatement(
                    "SELECT * FROM t WHERE id = " + someVar + " AND name = ?");
                ps.setString(1, "x");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Foo.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert!(ext.sql.contains("__JAVA_RAW_String_someVar__"), "SQL: {}", ext.sql);
}

#[test]
fn test_cross_method_fallback_when_method_unparsed() {
    let java = r#"
        public class MailService {
            public void saveAttachment(Connection conn) throws Exception {
                List list = new ArrayList();
                PreparedStatement ps = conn.prepareStatement(
                    "insert into t (a,b,c) VALUES (?,?,?)");
                submitData(ps, list);
            }
            public static void submitData(PreparedStatement ps, List list) throws Exception {
                for (int i = 0; i < 3; i++) {
                    ps.setString(i + 1, "x");
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "MailService.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert!(ext.sql.contains("__JAVA_VAR_String_DYNAMIC_1__"), "Expected DYNAMIC fallback, got: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_String_DYNAMIC_3__"), "Expected DYNAMIC_3, got: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_JDBC_PARAM_"), "Should not have JDBC_PARAM, got: {}", ext.sql);
}

#[test]
fn test_extra_sql_var_patterns_cmd_sb_empty_init() {
    let java = r#"
        public class Dao {
            public void find(String table) {
                StringBuilder cmd = new StringBuilder();
                cmd.append("SELECT * FROM ").append(table);
            }
        }
    "#;

    let result_default = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result_default.extractions.is_empty(), "Should not extract 'cmd' StringBuilder without extra patterns");

    let config = JavaExtractConfig { extra_sql_var_patterns: vec!["CMD".to_string()], ..Default::default() };
    let result = extract_sql_from_java(java, "Dao.java", &config);
    assert_eq!(result.extractions.len(), 1);
    assert!(result.extractions[0].sql.contains("SELECT * FROM"));
    assert_eq!(result.extractions[0].origin.variable_name.as_deref(), Some("cmd"));
}

// ── Phase X: __JAVA_RAW_ prefix and SB ? placeholder fixes ──

#[test]
fn test_sb_append_with_jdbc_placeholder_parses_successfully() {
    let java = r#"
        public class Dao {
            public void search() {
                StringBuilder sql = new StringBuilder("SELECT * FROM users WHERE 1=1");
                sql.append(" AND name LIKE ?");
                sql.append(" AND age > ?");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Java errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert!(ext.sql.contains("__JAVA_VAR_JDBC_PARAM_1__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_JDBC_PARAM_2__"), "SQL: {}", ext.sql);
    // Should parse successfully (only warnings allowed)
    assert!(ext.parse_result.is_some(), "No parse result");
    let parse_result = ext.parse_result.as_ref().unwrap();
    let real_errors: Vec<_> =
        parse_result.errors.iter().filter(|e| !matches!(e, crate::parser::ParserError::Warning { .. })).collect();
    assert!(real_errors.is_empty(), "Parse errors: {:?}", real_errors);
}

#[test]
fn test_plus_eq_concat_jdbc_placeholder_parses_successfully() {
    let java = r#"
        public class Dao {
            public void search() {
                String sql = "SELECT * FROM users WHERE 1=1";
                sql += " AND name LIKE ?";
                sql += " AND age > ?";
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Java errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert!(ext.sql.contains("__JAVA_VAR_JDBC_PARAM_1__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_VAR_JDBC_PARAM_2__"), "SQL: {}", ext.sql);
    assert!(ext.parse_result.is_some(), "No parse result");
    let parse_result = ext.parse_result.as_ref().unwrap();
    let real_errors: Vec<_> =
        parse_result.errors.iter().filter(|e| !matches!(e, crate::parser::ParserError::Warning { .. })).collect();
    assert!(real_errors.is_empty(), "Parse errors: {:?}", real_errors);
}

#[test]
fn test_raw_variable_reference_uses_java_raw_prefix() {
    let java = r#"
        public class Dao {
            public void query(String table) {
                String sql = "SELECT * FROM " + table + " WHERE id = 1";
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert!(result.errors.is_empty(), "Errors: {:?}", result.errors);
    assert_eq!(result.extractions.len(), 1);
    let ext = &result.extractions[0];
    assert!(ext.sql.contains("__JAVA_RAW_String_table__"), "SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_String_table__"), "Should not use _VAR_ prefix, SQL: {}", ext.sql);
}

#[test]
fn test_sb_append_raw_variable_uses_java_raw_prefix() {
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
    let ext = &result.extractions[0];
    assert!(ext.sql.contains("__JAVA_RAW_int_id__"), "SQL: {}", ext.sql);
    assert!(ext.sql.contains("__JAVA_RAW_String_name__"), "SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_int_id__"), "Should use _RAW_ prefix, SQL: {}", ext.sql);
    assert!(!ext.sql.contains("__JAVA_VAR_String_name__"), "Should use _RAW_ prefix, SQL: {}", ext.sql);
}

#[test]
fn test_dollar_placeholder_uses_java_raw_prefix() {
    let java = r#"
        public interface Mapper {
            @Select("SELECT * FROM ${tableName} WHERE id = #{id}")
            List<Map> findAll(@Param("tableName") String table, @Param("id") int id);
        }
    "#;
    let result = extract_sql_from_java(java, "Mapper.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    assert!(
        result.extractions[0].sql.contains("__JAVA_RAW_tableName__"),
        "Dollar-brace should produce __JAVA_RAW_, SQL: {}",
        result.extractions[0].sql
    );
    assert!(
        result.extractions[0].sql.contains("__JAVA_VAR_int_id__"),
        "Hash-brace should produce __JAVA_VAR_, SQL: {}",
        result.extractions[0].sql
    );
}

#[test]
fn test_extra_sql_var_patterns_stmt_and_default_sql_still_works() {
    let java = r#"
        public class Dao {
            public void find(int id) {
                String stmt = "SELECT * FROM t WHERE id = " + id;
                String sql = "DELETE FROM temp";
            }
        }
    "#;

    let config = JavaExtractConfig { extra_sql_var_patterns: vec!["STMT".to_string()], ..Default::default() };
    let result = extract_sql_from_java(java, "Dao.java", &config);
    assert_eq!(result.extractions.len(), 2);
    let stmt_ext = result.extractions.iter().find(|e| e.origin.variable_name.as_deref() == Some("stmt")).unwrap();
    assert!(stmt_ext.sql.contains("SELECT * FROM t"));
    let sql_ext = result.extractions.iter().find(|e| e.origin.variable_name.as_deref() == Some("sql")).unwrap();
    assert!(sql_ext.sql.contains("DELETE FROM temp"));
}

// ── Cross-method evaluation (Set.of → for-loop filter → nCopies → String.join) ──

#[test]
fn test_cross_method_colpart_and_placeholders() {
    let src = r#"
import java.util.*;
public class Test {
    void foo(Map<String, Object> columnValues) {
        Set<String> allowedCols = Set.of("emp_name", "dept_id", "hire_date", "salary");
        List<String> columns = new ArrayList<>();

        for (Map.Entry<String, Object> entry : columnValues.entrySet()) {
            if (!allowedCols.contains(entry.getKey())) throw new RuntimeException();
            columns.add(entry.getKey());
        }

        String colPart = String.join(", ", columns);
        String placeholderPart = String.join(", ", Collections.nCopies(columns.size(), "?"));
        String sql = "INSERT INTO employee (" + colPart + ") VALUES (" + placeholderPart + ")";
    }
}
"#;

    let result = crate::java::extract_sql_from_java(src, "Test.java", &crate::java::JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1, "expected 1 extraction, got {:?}", result.errors);

    let sql = &result.extractions[0].sql;

    // colPart should expand to all allowedCols members
    assert!(sql.contains("emp_name"), "should expand colPart, got: {}", sql);
    assert!(sql.contains("dept_id"), "should expand colPart, got: {}", sql);
    assert!(sql.contains("hire_date"), "should expand colPart, got: {}", sql);
    assert!(sql.contains("salary"), "should expand colPart, got: {}", sql);

    // placeholderPart should expand to 4 JDBC params
    assert!(sql.contains("__JAVA_VAR_JDBC_PARAM_1__"), "expected JDBC params, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_JDBC_PARAM_2__"), "expected 4 params, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_JDBC_PARAM_3__"), "expected 4 params, got: {}", sql);
    assert!(sql.contains("__JAVA_VAR_JDBC_PARAM_4__"), "expected 4 params, got: {}", sql);

    // No __JAVA_RAW_ should survive
    assert!(!sql.contains("__JAVA_RAW_"), "unexpected __JAVA_RAW_ in: {}", sql);
}

// ── Pending String Variable Tracking Tests ──

#[test]
fn test_pending_string_var_basic_accumulation() {
    let java = r#"
        public class Dao {
            public void build(String val) {
                StringBuilder sb = new StringBuilder("INSERT INTO t VALUES ");
                String vals = "(";
                vals += "'";
                vals += val;
                vals += "'";
                sb.append(vals);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(
        sql.contains("INSERT INTO t VALUES ('__JAVA_RAW_String_val__'"),
        "expected pending var expanded, got: {}",
        sql
    );
}

#[test]
fn test_pending_string_var_binary_expr_concat() {
    let java = r#"
        public class Dao {
            public void build(String val) {
                StringBuilder sb = new StringBuilder("SELECT * FROM t WHERE name IN ");
                String vals = "(";
                vals += "'" + val + "',";
                sb.append(vals);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(
        sql.contains("SELECT * FROM t WHERE name IN ('__JAVA_RAW_String_val__',"),
        "expected binary concat expanded, got: {}",
        sql
    );
}

#[test]
fn test_pending_string_var_not_leaked_to_non_sb() {
    let java = r#"
        public class Dao {
            public void build(String val) {
                String vals = "(";
                vals += val;
                vals += ")";
                System.out.println(vals);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(
        result.extractions.len(),
        0,
        "pending var should NOT produce extraction when not appended to tracked SB"
    );
}

// ── Binary Expression in append() Tests ──

#[test]
fn test_append_binary_expression_preserves_static_parts() {
    let java = r#"
        public class Dao {
            public void build(String colName) {
                StringBuilder sql = new StringBuilder("INSERT INTO t ");
                sql.append("(" + colName + ") VALUES ");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("INSERT INTO t ("), "should preserve static prefix, got: {}", sql);
    assert!(sql.contains(") VALUES "), "should preserve ') VALUES ' suffix, got: {}", sql);
    assert!(sql.contains("__JAVA_RAW_String_colName__"), "should contain placeholder, got: {}", sql);
}

#[test]
fn test_append_binary_expression_var_plus_literal() {
    let java = r#"
        public class Dao {
            public void build(String table) {
                StringBuilder sql = new StringBuilder("SELECT * FROM ");
                sql.append(table + " WHERE id = 1");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(
        sql.contains("SELECT * FROM __JAVA_RAW_String_table__ WHERE id = 1"),
        "expected var + literal expanded, got: {}",
        sql
    );
}

// ── deleteCharAt Tests ──

#[test]
fn test_delete_char_at_literal_index() {
    let java = r#"
        public class Dao {
            public void build() {
                StringBuilder sql = new StringBuilder("SELECT *, FROM t");
                sql.deleteCharAt(8);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(!sql.contains(","), "comma at index 8 should be deleted, got: {}", sql);
    assert!(sql.contains("SELECT *"), "should still have SELECT *, got: {}", sql);
    assert!(sql.contains("FROM t"), "should still have FROM t, got: {}", sql);
}

#[test]
fn test_delete_char_at_length_minus_n() {
    let java = r#"
        public class Dao {
            public void build() {
                StringBuilder sql = new StringBuilder("SELECT * FROM t,, ");
                sql.deleteCharAt(sql.length() - 2);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.matches(',').count() == 1, "expected exactly 1 comma after deleteCharAt(length-2), got: {}", sql);
}

#[test]
fn test_delete_char_at_in_chain_before_append() {
    let java = r#"
        public class Dao {
            public void build() {
                StringBuilder sql = new StringBuilder("SELECT * FROM t,");
                sql.deleteCharAt(sql.length() - 1).append(" WHERE id = 1");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("SELECT * FROM t WHERE id = 1"), "expected comma deleted then WHERE appended, got: {}", sql);
}

// ── SB Self-Reassignment Chain Tests ──

#[test]
fn test_sb_self_reassign_delete_char_at_and_append() {
    let java = r#"
        public class Dao {
            public void build() {
                StringBuilder sql = new StringBuilder("SELECT * FROM t");
                sql.append(",");
                sql = sql.deleteCharAt(sql.length() - 1).append(";");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("SELECT * FROM t;"), "expected comma deleted and semicolon appended, got: {}", sql);
    assert!(!sql.contains("t,"), "should not have trailing comma, got: {}", sql);
}

// ── Complex Guard Test: Simplified user INSERT scenario ──

#[test]
fn test_complex_dynamic_insert_with_pending_vars_and_delete_char_at() {
    let java = r#"
        public class Dao {
            public void buildInsert(String srcTable, String val) {
                StringBuilder sqlBuilder = new StringBuilder();
                sqlBuilder.append("INSERT INTO ");
                sqlBuilder.append(srcTable + " ");
                sqlBuilder.append("(" + "col1, col2" + ") VALUES ");
                String vals = "(";
                vals += "'" + val + "',";
                vals += "'fixed'";
                vals += "), ";
                sqlBuilder.append(vals);
                sqlBuilder = sqlBuilder.deleteCharAt(sqlBuilder.length() - 2).append(";");
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(
        result.extractions.len(),
        1,
        "extractions: {:?}",
        result.extractions.iter().map(|e| &e.sql).collect::<Vec<_>>()
    );
    let sql = &result.extractions[0].sql;

    assert!(sql.starts_with("INSERT INTO"), "should start with INSERT INTO, got: {}", sql);
    assert!(sql.contains("__JAVA_RAW_String_srcTable__"), "should contain srcTable placeholder, got: {}", sql);
    assert!(sql.contains("(col1, col2)"), "should contain column list, got: {}", sql);
    assert!(sql.contains("VALUES"), "should contain VALUES, got: {}", sql);
    assert!(sql.contains("('__JAVA_RAW_String_val__'"), "should contain pending var expansion for val, got: {}", sql);
    assert!(sql.contains("'fixed'"), "should contain fixed literal, got: {}", sql);
    assert!(sql.contains(";"), "should end with semicolon, got: {}", sql);

    assert!(
        !sql.contains("__JAVA_RAW_String_vals__"),
        "should NOT have generic vals placeholder (should be expanded), got: {}",
        sql
    );
}

// ── Pending String substring() Strip Tests ──

#[test]
fn test_pending_string_substring_strip() {
    let java = r#"
        public class Dao {
            public void build() {
                StringBuilder sqlBuilder = new StringBuilder();
                sqlBuilder.append("INSERT INTO t VALUES ");
                String vals = "(";
                vals += "'a',";
                vals += "'b',";
                if (vals.endsWith(",")) vals = vals.substring(0, vals.length() - 1);
                vals += ")";
                sqlBuilder.append(vals);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(sql.contains("VALUES ('a','b')"), "trailing comma should be stripped, got: {}", sql);
}

#[test]
fn test_ends_with_guard_skips_when_no_suffix() {
    let java = r#"
        public class Dao {
            public void build() {
                StringBuilder sqlBuilder = new StringBuilder();
                sqlBuilder.append("INSERT INTO t VALUES ");
                String vals = "(NO_COMMA";
                if (vals.endsWith(",")) vals = vals.substring(0, vals.length() - 1);
                vals += ")";
                sqlBuilder.append(vals);
            }
        }
    "#;
    let result = extract_sql_from_java(java, "Dao.java", &JavaExtractConfig::default());
    assert_eq!(result.extractions.len(), 1);
    let sql = &result.extractions[0].sql;
    assert!(
        sql.contains("(NO_COMMA)"),
        "endsWith guard should skip substring when value doesn't end with suffix, got: {}",
        sql
    );
}

// ── Full User Scenario Guard Test ──

#[test]
fn test_full_dynamic_insert_partition_scenario() {
    let java = r#"
        import java.util.*;
        import java.util.stream.Collectors;

        public class PartitionDao {
            public void buildBatchInsert(String srcTable, String partionField) {
                List<String> partionFields = Arrays.asList(partionField.split(","));
                List<Map<String, String>> dataList = new ArrayList<>();
                Map<String, List<Map<String, String>>> groupList = new LinkedHashMap<>();

                for (String partitionClause : groupList.keySet()) {
                    StringBuilder sqlBuilder = new StringBuilder();
                    sqlBuilder.append("INSERT INTO  ");
                    sqlBuilder.append(srcTable + " ");

                    sqlBuilder.append("(" + dataList.get(0).keySet().stream()
                        .filter(e -> !partionFields.contains(e))
                        .collect(Collectors.joining(",")) + ") VALUES ");

                    List<Map<String, String>> groupData = groupList.get(partitionClause);
                    for (Map<String, String> data : groupData) {
                        String subStr = "(";
                        for (String key : data.keySet()) {
                            if (!partionFields.contains(key)) {
                                subStr += "'" + data.get(key) + "',";
                            }
                        }
                        if (subStr.endsWith(",")) subStr = subStr.substring(0, subStr.length() - 1);
                        subStr += "), ";
                        sqlBuilder.append(subStr);
                    }
                    sqlBuilder = sqlBuilder.deleteCharAt(sqlBuilder.length() - 2).append(";\n");
                }
            }
        }
    "#;
    let result = extract_sql_from_java(java, "PartitionDao.java", &JavaExtractConfig::default());
    assert_eq!(
        result.extractions.len(),
        1,
        "expected 1 extraction, got {}: {:?}",
        result.extractions.len(),
        result.extractions.iter().map(|e| &e.sql).collect::<Vec<_>>()
    );
    let sql = &result.extractions[0].sql;

    assert!(sql.contains("INSERT INTO"), "should contain INSERT INTO, got: {}", sql);
    assert!(sql.contains("__JAVA_RAW_String_srcTable__"), "should contain srcTable placeholder, got: {}", sql);
    assert!(sql.contains("VALUES"), "should contain VALUES keyword, got: {}", sql);

    assert!(
        sql.contains("__JAVA_RAW_String_get__") || sql.contains("__JAVA_RAW_String_data__"),
        "should contain value placeholder from subStr accumulation, got: {}",
        sql
    );

    assert!(
        !sql.contains("__RAW_String_get__',)"),
        "trailing comma inside value tuple should be stripped by substring, got: {}",
        sql
    );

    assert!(sql.contains(";\n"), "should end with semicolon-newline, got: {}", sql);

    assert!(
        !sql.contains("__JAVA_RAW_String_subStr__"),
        "should NOT have generic subStr placeholder (should be expanded), got: {}",
        sql
    );
}
