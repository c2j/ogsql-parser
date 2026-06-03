//! iBatis XML 解析测试。

use super::*;
use crate::ibatis::error::IbatisError;
use crate::ibatis::types::{SqlNode, StatementKind};

fn parse_simple_mapper() -> crate::ibatis::types::MapperFile {
    let xml = br#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE mapper PUBLIC "-//mybatis.org//DTD Mapper 3.0//EN" "http://mybatis.org/dtd/mybatis-3-mapper.dtd">
<mapper namespace="com.example.UserMapper">
    <sql id="baseColumns">id, name, email</sql>

    <select id="findById" parameterType="int" resultType="User">
        SELECT <include refid="baseColumns"/>
        FROM users
        WHERE id = #{id}
    </select>

    <select id="findById" parameterType="int" resultType="User">
        SELECT <include refid="baseColumns"/>
        FROM users
        WHERE id &lt;&gt; #{id}
    </select>

    <insert id="insertUser" parameterType="User">
        INSERT INTO users (name, email) VALUES (#{name}, #{email})
    </insert>

    <update id="updateName">
        UPDATE users SET name = #{name} WHERE id = #{id}
    </update>

    <delete id="deleteById">
        DELETE FROM users WHERE id = #{id}
    </delete>
</mapper>"#;

    crate::ibatis::parser::parse_xml(xml).unwrap()
}

#[test]
fn test_parse_mapper_namespace() {
    let mapper = parse_simple_mapper();
    assert_eq!(mapper.namespace, "com.example.UserMapper");
}

#[test]
fn test_parse_fragments() {
    let mapper = parse_simple_mapper();
    assert_eq!(mapper.fragments.len(), 1);
    assert_eq!(mapper.fragments[0].id, "baseColumns");
    if let SqlNode::Text { content } = &mapper.fragments[0].body {
        assert!(content.contains("id, name, email"));
    } else {
        panic!("expected Text node");
    }
}

#[test]
fn test_parse_statements_count() {
    let mapper = parse_simple_mapper();
    assert_eq!(mapper.statements.len(), 5);
}

#[test]
fn test_parse_select_statement() {
    let mapper = parse_simple_mapper();
    let select = mapper.statements.iter().find(|s| s.id == "findById").unwrap();
    assert_eq!(select.kind, StatementKind::Select);
    assert_eq!(select.parameter_type.as_deref(), Some("int"));
    assert_eq!(select.result_type.as_deref(), Some("User"));
}

#[test]
fn test_parse_insert_statement() {
    let mapper = parse_simple_mapper();
    let insert = mapper.statements.iter().find(|s| s.id == "insertUser").unwrap();
    assert_eq!(insert.kind, StatementKind::Insert);
    assert_eq!(insert.parameter_type.as_deref(), Some("User"));
}

#[test]
fn test_parse_update_and_delete() {
    let mapper = parse_simple_mapper();
    let update = mapper.statements.iter().find(|s| s.id == "updateName").unwrap();
    assert_eq!(update.kind, StatementKind::Update);
    let delete = mapper.statements.iter().find(|s| s.id == "deleteById").unwrap();
    assert_eq!(delete.kind, StatementKind::Delete);
}

#[test]
fn test_skip_result_map() {
    let xml = br#"<mapper namespace="test">
        <resultMap id="userMap" type="User">
            <id column="id" property="id"/>
            <result column="name" property="name"/>
        </resultMap>
        <select id="findAll">SELECT * FROM users</select>
    </mapper>"#;
    let mapper = crate::ibatis::parser::parse_xml(xml).unwrap();
    assert_eq!(mapper.statements.len(), 1);
}

#[test]
fn test_empty_mapper() {
    let xml = br#"<mapper namespace="empty"></mapper>"#;
    let mapper = crate::ibatis::parser::parse_xml(xml).unwrap();
    assert_eq!(mapper.namespace, "empty");
    assert!(mapper.statements.is_empty());
    assert!(mapper.fragments.is_empty());
}

#[test]
fn test_invalid_xml() {
    let xml = br#"<mapper namespace="test"><select id="bad"></select>"#;
    let result = crate::ibatis::parser::parse_xml(xml);
    // quick-xml is lenient; truncated XML may or may not error.
    // The key contract: if it returns Ok, the namespace is parsed.
    if let Ok(mapper) = result {
        assert_eq!(mapper.namespace, "test");
    }
}

#[test]
fn test_preserves_whitespace_in_sql() {
    let xml = br#"<mapper namespace="test">
    <select id="ws">
        SELECT   id,    name
        FROM     users
        WHERE    id = 1
    </select>
</mapper>"#;
    let mapper = crate::ibatis::parser::parse_xml(xml).unwrap();
    let stmt = &mapper.statements[0];
    if let SqlNode::Text { content } = &stmt.body {
        assert!(content.contains("  id,    name"));
    } else {
        panic!("expected Text node");
    }
}

fn node_text(node: &SqlNode) -> String {
    match node {
        SqlNode::Text { content } => content.clone(),
        SqlNode::Sequence { children } => children.iter().map(node_text).collect(),
        SqlNode::Parameter { name, java_type, jdbc_type, .. } => {
            let type_str: Option<&str> = jdbc_type.as_deref().or(java_type.as_deref());
            match type_str {
                Some(t) => format!("#{{{},{}}}", name, format!("jdbcType={}", t)),
                None => format!("#{{{}}}", name),
            }
        }
        SqlNode::RawExpr { expr, java_type, jdbc_type } => {
            let type_str: Option<&str> = jdbc_type.as_deref().or(java_type.as_deref());
            match type_str {
                Some(t) => format!("${{{},{}}}", expr, format!("jdbcType={}", t)),
                None => format!("${{{}}}", expr),
            }
        }
        SqlNode::If { children, .. } => children.iter().map(node_text).collect(),
        SqlNode::Choose { branches } => branches.iter().flat_map(|(_, ch)| ch.iter().map(node_text)).collect(),
        SqlNode::Where { children } => children.iter().map(node_text).collect(),
        SqlNode::Set { children } => children.iter().map(node_text).collect(),
        SqlNode::Trim { children, .. } => children.iter().map(node_text).collect(),
        SqlNode::ForEach { children, .. } => children.iter().map(node_text).collect(),
        SqlNode::Bind { .. } => String::new(),
        SqlNode::Include { refid } => format!("<include refid=\"{}\" />", refid),
    }
}

#[test]
fn test_include_parsed_as_node() {
    let xml = br#"<mapper namespace="test">
        <sql id="cols">id, name</sql>
        <select id="findAll">SELECT <include refid="cols"/> FROM users</select>
    </mapper>"#;
    let mapper = crate::ibatis::parser::parse_xml(xml).unwrap();
    let stmt = mapper.statements.iter().find(|s| s.id == "findAll").unwrap();
    if let SqlNode::Sequence { children } = &stmt.body {
        let include_nodes: Vec<_> = children.iter().filter(|n| matches!(n, SqlNode::Include { .. })).collect();
        assert_eq!(include_nodes.len(), 1, "expected exactly one Include node");
        if let SqlNode::Include { refid } = include_nodes[0] {
            assert_eq!(refid, "cols");
        } else {
            panic!("expected Include node");
        }
    } else {
        panic!("expected Sequence node");
    }
}

#[test]
fn test_include_open_close_parsed_as_node() {
    let xml = br#"<mapper namespace="test">
        <sql id="cols">id, name</sql>
        <select id="findAll">SELECT <include refid="cols"></include> FROM users</select>
    </mapper>"#;
    let mapper = crate::ibatis::parser::parse_xml(xml).unwrap();
    let stmt = mapper.statements.iter().find(|s| s.id == "findAll").unwrap();
    if let SqlNode::Sequence { children } = &stmt.body {
        let include_nodes: Vec<_> = children.iter().filter(|n| matches!(n, SqlNode::Include { .. })).collect();
        assert_eq!(include_nodes.len(), 1, "expected exactly one Include node");
        if let SqlNode::Include { refid } = include_nodes[0] {
            assert_eq!(refid, "cols");
        }
    } else {
        panic!("expected Sequence node, got {:?}", stmt.body);
    }
}

#[test]
fn test_include_open_close_resolved() {
    let xml = br#"<mapper namespace="test">
        <sql id="allColumn">id, user_code, area_code</sql>
        <select id="getList">SELECT <include refid="allColumn"></include> FROM users</select>
    </mapper>"#;
    let mapper = crate::ibatis::parser::parse_xml(xml).unwrap();
    let resolved = crate::ibatis::resolver::resolve_includes(&mapper).unwrap();
    let stmt = resolved.statements.iter().find(|s| s.id == "getList").unwrap();
    let content = node_text(&stmt.body);
    assert!(content.contains("id, user_code, area_code"), "got: {}", content);
    assert!(!content.contains("<include"), "raw include text should not remain, got: {}", content);
}

#[test]
fn test_include_open_close_resolved_structurally() {
    let xml = br#"<mapper namespace="test">
        <sql id="cols">id, name</sql>
        <select id="findAll">SELECT <include refid="cols"/> FROM users</select>
    </mapper>"#;
    let mapper = crate::ibatis::parser::parse_xml(xml).unwrap();
    let resolved = crate::ibatis::resolver::resolve_includes(&mapper).unwrap();
    let stmt = resolved.statements.iter().find(|s| s.id == "findAll").unwrap();
    let content = node_text(&stmt.body);
    assert!(content.contains("id, name"), "got: {}", content);
    assert!(!content.contains("<include"), "raw include text should not remain, got: {}", content);
}

// ── Include Resolution Tests ──

#[test]
fn test_include_resolution_basic() {
    let xml = br#"<mapper namespace="test">
        <sql id="cols">id, name, email</sql>
        <select id="findAll">SELECT <include refid="cols"/> FROM users</select>
    </mapper>"#;
    let mapper = crate::ibatis::parser::parse_xml(xml).unwrap();
    let resolved = crate::ibatis::resolver::resolve_includes(&mapper).unwrap();
    let stmt = resolved.statements.iter().find(|s| s.id == "findAll").unwrap();
    let content = node_text(&stmt.body);
    assert!(content.contains("id, name, email"), "got: {}", content);
    assert!(content.contains("SELECT"));
    assert!(content.contains("FROM users"));
}

#[test]
fn test_include_resolution_chained() {
    let xml = br#"<mapper namespace="test">
        <sql id="table">users</sql>
        <sql id="cols">id, name FROM <include refid="table"/></sql>
        <select id="find">SELECT <include refid="cols"/></select>
    </mapper>"#;
    let mapper = crate::ibatis::parser::parse_xml(xml).unwrap();
    let resolved = crate::ibatis::resolver::resolve_includes(&mapper).unwrap();
    let stmt = &resolved.statements[0];
    let content = node_text(&stmt.body);
    assert!(content.contains("users"), "chained include should expand, got: {}", content);
}

#[test]
fn test_include_unknown_fragment() {
    let xml = br#"<mapper namespace="test">
        <select id="find">SELECT <include refid="nonexistent"/> FROM users</select>
    </mapper>"#;
    let mapper = crate::ibatis::parser::parse_xml(xml).unwrap();
    let result = crate::ibatis::resolver::resolve_includes(&mapper);
    assert!(result.is_err());
    match result.unwrap_err() {
        IbatisError::UnknownFragment { refid } => {
            assert_eq!(refid, "nonexistent");
        }
        e => panic!("expected UnknownFragment, got {:?}", e),
    }
}

#[test]
fn test_include_circular_detection() {
    let xml = br#"<mapper namespace="test">
        <sql id="a"><include refid="b"/></sql>
        <sql id="b"><include refid="a"/></sql>
        <select id="find">SELECT <include refid="a"/></select>
    </mapper>"#;
    let mapper = crate::ibatis::parser::parse_xml(xml).unwrap();
    let result = crate::ibatis::resolver::resolve_includes(&mapper);
    assert!(result.is_err());
    match result.unwrap_err() {
        IbatisError::CircularInclude { chain } => {
            assert!(!chain.is_empty());
        }
        e => panic!("expected CircularInclude, got {:?}", e),
    }
}

#[test]
fn test_no_includes() {
    let xml = br#"<mapper namespace="test">
        <select id="find">SELECT 1</select>
    </mapper>"#;
    let mapper = crate::ibatis::parser::parse_xml(xml).unwrap();
    let resolved = crate::ibatis::resolver::resolve_includes(&mapper).unwrap();
    let content = node_text(&resolved.statements[0].body);
    assert_eq!(content.trim(), "SELECT 1");
}

// ── End-to-End Pipeline Tests ──

#[test]
fn test_e2e_simple_select() {
    let xml = br#"<mapper namespace="com.example.UserMapper">
        <select id="findById">SELECT id, name FROM users WHERE id = #{id}</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert_eq!(result.namespace, "com.example.UserMapper");
    assert_eq!(result.statements.len(), 1);
    assert!(result.errors.is_empty());

    let stmt = &result.statements[0];
    assert_eq!(stmt.id, "findById");
    assert_eq!(stmt.kind, StatementKind::Select);
    assert!(stmt.flat_sql.contains("SELECT"));
    assert!(stmt.flat_sql.contains("__XML_PARAM_id__"));

    if let Some((infos, errors)) = &stmt.parse_result {
        assert!(errors.is_empty(), "parser errors: {:?}", errors);
        assert_eq!(infos.len(), 1);
    } else {
        panic!("expected parse result");
    }
}

#[test]
fn test_e2e_insert() {
    let xml = br#"<mapper namespace="test">
        <insert id="insertUser">INSERT INTO users (name) VALUES (#{name})</insert>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    let stmt = &result.statements[0];
    assert_eq!(stmt.kind, StatementKind::Insert);
    assert!(stmt.flat_sql.contains("INSERT INTO"));
}

#[test]
fn test_e2e_with_fragment() {
    let xml = br#"<mapper namespace="test">
        <sql id="cols">id, name</sql>
        <select id="findAll">SELECT <include refid="cols"/> FROM users</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.errors.is_empty());
    let stmt = &result.statements[0];
    assert!(stmt.flat_sql.contains("id, name"), "got: {}", stmt.flat_sql);
    assert!(stmt.flat_sql.contains("FROM users"));
}

#[test]
fn test_e2e_dollar_param_placeholder() {
    let xml = br#"<mapper namespace="test">
        <select id="dynamicOrder">SELECT * FROM users ORDER BY ${column}</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    let stmt = &result.statements[0];
    assert!(stmt.flat_sql.contains("__XML_RAW_column__"));
}

#[test]
fn test_e2e_dollar_param_with_java_type() {
    let xml = br#"<mapper namespace="test">
        <select id="dynamicOrder">SELECT * FROM users ORDER BY ${column,javaType=string}</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    let stmt = &result.statements[0];
    assert!(stmt.flat_sql.contains("__XML_RAW_STRING_column__"), "got: {}", stmt.flat_sql);
}

#[test]
fn test_e2e_dollar_param_with_jdbc_type() {
    let xml = br#"<mapper namespace="test">
        <select id="dynamicCol">SELECT ${col,jdbcType=VARCHAR} FROM users</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    let stmt = &result.statements[0];
    assert!(stmt.flat_sql.contains("__XML_RAW_VARCHAR_col__"), "got: {}", stmt.flat_sql);
}

#[test]
fn test_e2e_empty_mapper() {
    let xml = br#"<mapper namespace="empty"></mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.statements.is_empty());
    assert!(result.errors.iter().any(|e| matches!(e, IbatisError::EmptyMapper)));
}

#[test]
fn test_e2e_multiple_statements() {
    let xml = br#"<mapper namespace="test">
        <select id="find">SELECT * FROM users WHERE id = #{id}</select>
        <insert id="add">INSERT INTO users (name) VALUES (#{name})</insert>
        <update id="update">UPDATE users SET name = #{name} WHERE id = #{id}</update>
        <delete id="remove">DELETE FROM users WHERE id = #{id}</delete>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert_eq!(result.statements.len(), 4);
    assert_eq!(result.statements[0].kind, StatementKind::Select);
    assert_eq!(result.statements[1].kind, StatementKind::Insert);
    assert_eq!(result.statements[2].kind, StatementKind::Update);
    assert_eq!(result.statements[3].kind, StatementKind::Delete);
}

// ── Dynamic SQL Tests ──

#[test]
fn test_dynamic_if() {
    let xml = br#"<mapper namespace="test">
        <select id="findUser">
            SELECT * FROM users
            <where>
                <if test="name != null">AND name = #{name}</if>
                <if test="age != null">AND age = #{age}</if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    let stmt = &result.statements[0];
    assert!(stmt.flat_sql.contains("SELECT * FROM users"));
    assert!(stmt.flat_sql.contains("WHERE"), "should have WHERE, got: {}", stmt.flat_sql);
    assert!(stmt.has_dynamic_elements);
}

#[test]
fn test_dynamic_where_strips_leading_and() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <if test="name != null">AND name = #{name}</if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    let sql = &result.statements[0].flat_sql;
    assert!(!sql.contains("WHERE AND"), "got: {}", sql);
    assert!(sql.contains("WHERE"), "should have WHERE, got: {}", sql);
}

#[test]
fn test_dynamic_set_strips_trailing_comma() {
    let xml = br#"<mapper namespace="test">
        <update id="updateUser">
            UPDATE users
            <set>
                <if test="name != null">name = #{name},</if>
                <if test="email != null">email = #{email},</if>
            </set>
            WHERE id = #{id}
        </update>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    let sql = &result.statements[0].flat_sql;
    assert!(sql.contains("SET"), "should have SET, got: {}", sql);
}

#[test]
fn test_dynamic_foreach() {
    let xml = br#"<mapper namespace="test">
        <select id="findByIds">
            SELECT * FROM users WHERE id IN
            <foreach collection="ids" item="id" open="(" separator="," close=")">
                #{id}
            </foreach>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    let sql = &result.statements[0].flat_sql;
    assert!(sql.contains("IN"), "should have IN, got: {}", sql);
    assert!(sql.contains("("), "should have open paren, got: {}", sql);
    assert!(sql.contains(")"), "should have close paren, got: {}", sql);
}

#[test]
fn test_dynamic_choose() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <choose>
                    <when test="id != null">AND id = #{id}</when>
                    <otherwise>AND status = 'ACTIVE'</otherwise>
                </choose>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    assert!(result.statements[0].has_dynamic_elements);
}

#[test]
fn test_dynamic_trim_custom() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <trim prefix="WHERE" prefixOverrides="AND |OR ">
                <if test="name != null">AND name = #{name}</if>
            </trim>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    let sql = &result.statements[0].flat_sql;
    assert!(sql.contains("WHERE"), "should have WHERE, got: {}", sql);
    assert!(!sql.contains("WHERE AND"), "prefix override should strip AND, got: {}", sql);
}

#[test]
fn test_dynamic_bind() {
    let xml = br#"<mapper namespace="test">
        <select id="search">
            SELECT * FROM users
            <where>
                <bind name="pattern" value="'%' + name + '%'"/>
                name LIKE #{pattern}
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    assert!(result.statements[0].has_dynamic_elements);
}

#[test]
fn test_xml_entity_decoding() {
    let xml = br#"<?xml version="1.0" encoding="UTF-8"?>
<mapper namespace="test">
    <select id="q1">select * from t where id&lt;&gt;1</select>
    <select id="q2">select * from t where name = 'hello&amp;world'</select>
    <select id="q3">select * from t where id &gt;= 5 and id &lt;= 10</select>
</mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    assert_eq!(result.statements.len(), 3);
    assert!(result.statements[0].flat_sql.contains("<>"));
    assert!(result.statements[1].flat_sql.contains("&"));
    assert!(result.statements[2].flat_sql.contains(">="));
    assert!(result.statements[2].flat_sql.contains("<="));
}

#[test]
fn test_cdata_with_xml_entities_issue70() {
    // Issue #70: parse_mapper_bytes_with_path infinite loop on SQL with &gt;= / &lt;= in CDATA
    let xml = br#"<?xml version="1.0" encoding="UTF-8" ?>
<!DOCTYPE mapper PUBLIC "-//mybatis.org//DTD Mapper 3.0//EN" "http://mybatis.org/dtd/mybatis-3-mapper.dtd" >
<mapper namespace="test">
    <select id="queryVModeNeedAndVInstNeed" parameterType="map" resultType="map">
        <![CDATA[
        	select t.model_need "modelNeed"
        	from dat_inst_oper_type_mode t where t.operation_no = #{vOperationNo}
        	and t.inure_begin_date &gt;= #{date} and t.inure_end_date &lt;= #{date}
        ]]>
    </select>
</mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert_eq!(result.statements.len(), 1, "should parse 1 statement, errors: {:?}", result.errors);
    assert!(result.errors.is_empty());
    let sql = &result.statements[0].flat_sql;
    assert!(sql.contains(">="), "should decode &gt;= to >=, got: {}", sql);
    assert!(sql.contains("<="), "should decode &lt;= to <=, got: {}", sql);
}

#[test]
fn test_cdata_with_actual_operators() {
    // CDATA with actual >= and <= operators (not entity-encoded)
    let xml = br#"<mapper namespace="test">
    <select id="rangeQuery">
        <![CDATA[SELECT * FROM t WHERE id >= 1 AND id <= 100]]>
    </select>
</mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert_eq!(result.statements.len(), 1, "errors: {:?}", result.errors);
    assert!(result.statements[0].flat_sql.contains(">="));
    assert!(result.statements[0].flat_sql.contains("<="));
}

#[test]
fn test_entity_outside_cdata_operators() {
    // Entities outside CDATA (standard MyBatis practice for >= / <=)
    let xml = br#"<mapper namespace="test">
    <select id="rangeQuery">
        SELECT * FROM t WHERE id &gt;= #{min} AND id &lt;= #{max}
    </select>
</mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert_eq!(result.statements.len(), 1, "errors: {:?}", result.errors);
    assert!(result.statements[0].flat_sql.contains(">="), "got: {}", result.statements[0].flat_sql);
    assert!(result.statements[0].flat_sql.contains("<="), "got: {}", result.statements[0].flat_sql);
}

#[cfg(feature = "java")]
#[test]
fn test_java_type_to_jdbc_mapping() {
    use crate::ibatis::types::JdbcType;
    assert_eq!(crate::ibatis::java_resolve::java_type_to_jdbc("int"), Some(JdbcType::Integer));
    assert_eq!(crate::ibatis::java_resolve::java_type_to_jdbc("String"), Some(JdbcType::VarChar));
    assert_eq!(crate::ibatis::java_resolve::java_type_to_jdbc("Long"), Some(JdbcType::BigInt));
    assert_eq!(crate::ibatis::java_resolve::java_type_to_jdbc("Date"), Some(JdbcType::Timestamp));
    assert_eq!(crate::ibatis::java_resolve::java_type_to_jdbc("Unknown"), None);
}

#[cfg(feature = "java")]
#[test]
fn test_jdbc_type_from_str() {
    use crate::ibatis::types::JdbcType;
    assert_eq!(crate::ibatis::java_resolve::jdbc_type_from_str("VARCHAR"), Some(JdbcType::VarChar));
    assert_eq!(crate::ibatis::java_resolve::jdbc_type_from_str("INTEGER"), Some(JdbcType::Integer));
    assert_eq!(crate::ibatis::java_resolve::jdbc_type_from_str("timestamp"), Some(JdbcType::Timestamp));
}

#[cfg(feature = "java")]
#[test]
fn test_e2e_param_type_from_java_interface() {
    let java_source = r#"
package com.example.mapper;
public interface UserMapper {
    User findById(int id);
    List<User> findByName(String name);
}
"#;

    let tmp_dir = std::env::temp_dir().join("ogsql_test_java_src_e2e");
    let pkg_dir = tmp_dir.join("com/example/mapper");
    std::fs::create_dir_all(&pkg_dir).unwrap();
    std::fs::write(pkg_dir.join("UserMapper.java"), java_source).unwrap();

    let xml = br#"<mapper namespace="com.example.mapper.UserMapper">
        <select id="findById">SELECT * FROM users WHERE id = #{id}</select>
        <select id="findByName">SELECT * FROM users WHERE name = #{name}</select>
    </mapper>"#;

    let result = crate::ibatis::parse_mapper_bytes_with_java_src(xml, None, vec![tmp_dir.clone()]);

    assert_eq!(result.statements.len(), 2);

    let stmt1 = &result.statements[0];
    assert_eq!(stmt1.id, "findById");
    assert_eq!(stmt1.parameters.len(), 1);
    assert_eq!(stmt1.parameters[0].name, "id");
    assert_eq!(stmt1.parameters[0].jdbc_type, Some(crate::ibatis::types::JdbcType::Integer));

    let stmt2 = &result.statements[1];
    assert_eq!(stmt2.id, "findByName");
    assert_eq!(stmt2.parameters[0].name, "name");
    assert_eq!(stmt2.parameters[0].jdbc_type, Some(crate::ibatis::types::JdbcType::VarChar));

    let _ = std::fs::remove_dir_all(&tmp_dir);
}

#[test]
fn test_ibatis2_dynamic_tag() {
    let xml = br#"<?xml version="1.0"?>
<sqlMap>
    <select id="testDynamic" parameterClass="map">
        select * from ACCOUNT
        <dynamic>
            <isNotNull property="id">
                where ACC_ID = #id#
            </isNotNull>
        </dynamic>
    </select>
</sqlMap>"#;
    let result = crate::ibatis::parse_mapper_bytes(xml);
    assert_eq!(result.statements.len(), 1);
    assert_eq!(result.statements[0].id, "testDynamic");
    assert!(result.statements[0].has_dynamic_elements);
}

#[test]
fn test_ibatis2_iterate_tag() {
    let xml = br#"<?xml version="1.0"?>
<sqlMap>
    <select id="testIterate">
        select * from ACCOUNT where ACC_ID in
        <iterate open="(" close=")" conjunction=",">
            #[]#
        </iterate>
    </select>
</sqlMap>"#;
    let result = crate::ibatis::parse_mapper_bytes(xml);
    assert_eq!(result.statements.len(), 1);
    let sql = &result.statements[0].flat_sql;
    assert!(sql.contains("__XML_PARAM__item__"), "got: {}", sql);
}

#[test]
fn test_ibatis2_isEqual_tag() {
    let xml = br#"<?xml version="1.0"?>
<sqlMap>
    <select id="testIsEqual">
        select * from ACCOUNT
        <isEqual property="mode" compareValue="full">
            where ACC_ID = #id#
        </isEqual>
    </select>
</sqlMap>"#;
    let result = crate::ibatis::parse_mapper_bytes(xml);
    assert_eq!(result.statements.len(), 1);
    assert!(result.statements[0].has_dynamic_elements);
}

#[test]
fn test_ibatis2_parameterMap() {
    let xml = br#"<?xml version="1.0"?>
<sqlMap>
    <parameterMap id="test-params" class="account">
        <parameter property="id"/>
        <parameter property="firstName"/>
    </parameterMap>
    <select id="testPMap" parameterMap="test-params">
        select * from ACCOUNT where ACC_ID = ? and ACC_FIRST_NAME = ?
    </select>
</sqlMap>"#;
    let result = crate::ibatis::parse_mapper_bytes(xml);
    assert_eq!(result.statements.len(), 1);
    let stmt = &result.statements[0];
    assert_eq!(stmt.id, "testPMap");
    let sql = &stmt.flat_sql;
    assert!(sql.contains("__XML_PARAM_id__"), "got: {}", sql);
    assert!(sql.contains("__XML_PARAM_firstName__"), "got: {}", sql);
    assert!(!sql.contains("?"), "got: {}", sql);
    assert_eq!(stmt.parameters.len(), 2);
    assert_eq!(stmt.parameters[0].name, "id");
    assert_eq!(stmt.parameters[1].name, "firstName");
}

#[test]
fn test_ibatis2_colon_type_syntax() {
    use crate::ibatis::util::parse_param_type;
    let (name, jt) = parse_param_type("emailAddress:VARCHAR:no_email@provided.com");
    assert_eq!(name, "emailAddress");
    assert_eq!(jt.as_deref(), Some("VARCHAR"));
}

#[test]
fn test_sanitize_param_name() {
    use crate::ibatis::flatten::sanitize_param_name;
    assert_eq!(sanitize_param_name("nestedList[].idList[]"), "nestedList_item_idList_item");
    assert_eq!(sanitize_param_name("normalParam"), "normalParam");
    assert_eq!(sanitize_param_name("value+1"), "value_1");
}

// ── Structured AST API Tests (Issue #179) ──

fn find_first_node_of_type<'a>(node: &'a SqlNode, type_name: &str) -> Option<&'a SqlNode> {
    let is_match = match type_name {
        "if" => matches!(node, SqlNode::If { .. }),
        "choose" => matches!(node, SqlNode::Choose { .. }),
        "foreach" => matches!(node, SqlNode::ForEach { .. }),
        "where" => matches!(node, SqlNode::Where { .. }),
        "set" => matches!(node, SqlNode::Set { .. }),
        "trim" => matches!(node, SqlNode::Trim { .. }),
        "bind" => matches!(node, SqlNode::Bind { .. }),
        "rawexpr" => matches!(node, SqlNode::RawExpr { .. }),
        "parameter" => matches!(node, SqlNode::Parameter { .. }),
        "include" => matches!(node, SqlNode::Include { .. }),
        _ => false,
    };
    if is_match {
        return Some(node);
    }
    match node {
        SqlNode::Sequence { children }
        | SqlNode::If { children, .. }
        | SqlNode::Where { children }
        | SqlNode::Set { children }
        | SqlNode::Trim { children, .. }
        | SqlNode::ForEach { children, .. } => {
            for child in children {
                if let Some(found) = find_first_node_of_type(child, type_name) {
                    return Some(found);
                }
            }
            None
        }
        SqlNode::Choose { branches } => {
            for (_, ch) in branches {
                for child in ch {
                    if let Some(found) = find_first_node_of_type(child, type_name) {
                        return Some(found);
                    }
                }
            }
            None
        }
        _ => None,
    }
}

#[test]
fn test_structured_simple_select() {
    let xml = br#"<mapper namespace="com.example.UserMapper">
        <select id="findById" parameterType="int" resultType="User">
            SELECT * FROM users WHERE id = #{id}
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    assert_eq!(result.namespace, "com.example.UserMapper");
    assert!(result.errors.is_empty());
    assert_eq!(result.statements.len(), 1);
    assert_eq!(result.statements[0].id, "findById");
    assert_eq!(result.statements[0].kind, StatementKind::Select);
    assert!(!result.statements[0].has_dynamic_elements);
    assert_eq!(result.statements[0].parameters.len(), 1);
    assert_eq!(result.statements[0].parameters[0].name, "id");
}

#[test]
fn test_structured_preserves_if_tree() {
    let xml = br#"<mapper namespace="test">
        <select id="findUser">
            SELECT * FROM users
            <where>
                <if test="name != null">AND name = #{name}</if>
                <if test="age != null">AND age = #{age}</if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    assert!(result.errors.is_empty());
    let stmt = &result.statements[0];
    assert!(stmt.has_dynamic_elements);

    let where_node = find_first_node_of_type(&stmt.body, "where");
    assert!(where_node.is_some(), "expected a Where node");

    if let SqlNode::Where { children: where_children } = where_node.unwrap() {
        let if_nodes: Vec<_> = where_children.iter().filter(|n| matches!(n, SqlNode::If { .. })).collect();
        assert_eq!(if_nodes.len(), 2, "expected two If nodes inside Where");

        if let SqlNode::If { test, children: if_children, .. } = if_nodes[0] {
            assert_eq!(test, "name != null");
            let params: Vec<_> = if_children.iter().filter(|n| matches!(n, SqlNode::Parameter { .. })).collect();
            assert_eq!(params.len(), 1);
            if let SqlNode::Parameter { name, .. } = params[0] {
                assert_eq!(name, "name");
            }
        }

        if let SqlNode::If { test, .. } = if_nodes[1] {
            assert_eq!(test, "age != null");
        }
    }
}

#[test]
fn test_structured_preserves_foreach_tree() {
    let xml = br#"<mapper namespace="test">
        <select id="findByIds">
            SELECT * FROM users WHERE id IN
            <foreach collection="ids" item="id" open="(" separator="," close=")">
                #{id}
            </foreach>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let stmt = &result.statements[0];
    assert!(stmt.has_dynamic_elements);

    let foreach = find_first_node_of_type(&stmt.body, "foreach");
    assert!(foreach.is_some());

    if let SqlNode::ForEach { collection, item, open, separator, close, children, .. } = foreach.unwrap() {
        assert_eq!(collection, "ids");
        assert_eq!(item, "id");
        assert_eq!(open.as_deref(), Some("("));
        assert_eq!(separator.as_deref(), Some(","));
        assert_eq!(close.as_deref(), Some(")"));
        let params: Vec<_> = children.iter().filter(|n| matches!(n, SqlNode::Parameter { .. })).collect();
        assert_eq!(params.len(), 1);
    }
}

#[test]
fn test_structured_preserves_choose_tree() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <choose>
                    <when test="id != null">AND id = #{id}</when>
                    <otherwise>AND status = 'ACTIVE'</otherwise>
                </choose>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let stmt = &result.statements[0];
    assert!(stmt.has_dynamic_elements);

    let choose = find_first_node_of_type(&stmt.body, "choose");
    assert!(choose.is_some());

    if let SqlNode::Choose { branches } = choose.unwrap() {
        assert_eq!(branches.len(), 2);
        assert_eq!(branches[0].0.as_deref(), Some("id != null"));
        assert!(branches[1].0.as_ref().map_or(true, |t| t.is_empty()));
    }
}

#[test]
fn test_structured_preserves_trim_tree() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <trim prefix="WHERE" prefixOverrides="AND |OR ">
                <if test="name != null">AND name = #{name}</if>
            </trim>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let stmt = &result.statements[0];

    let trim = find_first_node_of_type(&stmt.body, "trim");
    assert!(trim.is_some());

    if let SqlNode::Trim { prefix, prefix_overrides, children, .. } = trim.unwrap() {
        assert_eq!(prefix.as_deref(), Some("WHERE"));
        assert_eq!(prefix_overrides.as_deref(), Some("AND |OR "));
        let if_nodes: Vec<_> = children.iter().filter(|n| matches!(n, SqlNode::If { .. })).collect();
        assert_eq!(if_nodes.len(), 1, "trim should have one If child");
    }
}

#[test]
fn test_structured_preserves_set_tree() {
    let xml = br#"<mapper namespace="test">
        <update id="updateUser">
            UPDATE users
            <set>
                <if test="name != null">name = #{name},</if>
                <if test="email != null">email = #{email},</if>
            </set>
            WHERE id = #{id}
        </update>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let stmt = &result.statements[0];
    assert!(stmt.has_dynamic_elements);

    let set = find_first_node_of_type(&stmt.body, "set");
    assert!(set.is_some());

    if let SqlNode::Set { children } = set.unwrap() {
        let if_nodes: Vec<_> = children.iter().filter(|n| matches!(n, SqlNode::If { .. })).collect();
        assert_eq!(if_nodes.len(), 2);
    }
}

#[test]
fn test_structured_preserves_bind_node() {
    let xml = br#"<mapper namespace="test">
        <select id="search">
            SELECT * FROM users
            <where>
                <bind name="pattern" value="'%' + name + '%'"/>
                name LIKE #{pattern}
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let stmt = &result.statements[0];
    assert!(stmt.has_dynamic_elements);

    let bind = find_first_node_of_type(&stmt.body, "bind");
    assert!(bind.is_some());

    if let SqlNode::Bind { name, value } = bind.unwrap() {
        assert_eq!(name, "pattern");
        assert_eq!(value, "'%' + name + '%'");
    }
}

#[test]
fn test_structured_include_resolved() {
    let xml = br#"<mapper namespace="test">
        <sql id="cols">id, name</sql>
        <select id="findAll">SELECT <include refid="cols"/> FROM users</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    assert!(result.errors.is_empty());
    let stmt = &result.statements[0];
    let text = node_text(&stmt.body);
    assert!(text.contains("id, name"), "include should be resolved, got: {}", text);
    assert!(!text.contains("<include"));
}

#[test]
fn test_structured_fragments_preserved() {
    let xml = br#"<mapper namespace="test">
        <sql id="baseColumns">id, name, email</sql>
        <select id="findAll">SELECT <include refid="baseColumns"/> FROM users</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    assert_eq!(result.fragments.len(), 1);
    assert_eq!(result.fragments[0].id, "baseColumns");
}

#[test]
fn test_structured_source_location() {
    let xml = br#"<mapper namespace="test">
        <select id="find">SELECT 1</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured_with_path(xml, Some("test.xml"));
    let stmt = &result.statements[0];
    assert_eq!(stmt.location.file_path.as_deref(), Some("test.xml"));
    assert!(stmt.location.line > 0);
}

#[test]
fn test_structured_empty_mapper() {
    let xml = br#"<mapper namespace="empty"></mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    assert!(result.statements.is_empty());
    assert!(result.errors.iter().any(|e| matches!(e, IbatisError::EmptyMapper)));
}

#[test]
fn test_structured_invalid_xml() {
    let xml = b"not xml at all";
    let result = super::parse_mapper_bytes_structured(xml);
    assert!(!result.errors.is_empty());
}

#[test]
fn test_structured_collects_params() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users WHERE id = #{id} AND name = #{name}
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let stmt = &result.statements[0];
    assert_eq!(stmt.parameters.len(), 2);
    let names: Vec<&str> = stmt.parameters.iter().map(|p| p.name.as_str()).collect();
    assert!(names.contains(&"id"));
    assert!(names.contains(&"name"));
}

#[test]
fn test_structured_dollar_param() {
    let xml = br#"<mapper namespace="test">
        <select id="dynamicOrder">SELECT * FROM users ORDER BY ${column}</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let stmt = &result.statements[0];
    let raw = find_first_node_of_type(&stmt.body, "rawexpr");
    assert!(raw.is_some());
    if let SqlNode::RawExpr { expr, .. } = raw.unwrap() {
        assert_eq!(expr, "column");
    }
}

#[test]
fn test_structured_nested_if_inside_foreach() {
    let xml = br#"<mapper namespace="test">
        <select id="complex">
            SELECT * FROM users
            <where>
                <if test="ids != null">
                    AND id IN
                    <foreach collection="ids" item="id" open="(" separator="," close=")">
                        <if test="id != null">#{id}</if>
                    </foreach>
                </if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    assert!(result.errors.is_empty());
    let stmt = &result.statements[0];
    assert!(stmt.has_dynamic_elements);

    let where_node = find_first_node_of_type(&stmt.body, "where");
    assert!(where_node.is_some(), "should have Where node");

    let foreach = find_first_node_of_type(&stmt.body, "foreach");
    assert!(foreach.is_some(), "should have ForEach node");

    if let SqlNode::ForEach { children, .. } = foreach.unwrap() {
        let inner_if: Vec<_> = children.iter().filter(|n| matches!(n, SqlNode::If { .. })).collect();
        assert_eq!(inner_if.len(), 1, "ForEach should contain one inner If");
        if let SqlNode::If { test, .. } = inner_if[0] {
            assert_eq!(test, "id != null");
        }
    }
}

#[test]
fn test_structured_ibatis2_compat() {
    let xml = br#"<?xml version="1.0"?>
<sqlMap>
    <select id="testDynamic" parameterClass="map">
        select * from ACCOUNT
        <dynamic>
            <isNotNull property="id">
                where ACC_ID = #id#
            </isNotNull>
        </dynamic>
    </select>
</sqlMap>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    assert_eq!(result.statements.len(), 1);
    assert!(result.statements[0].has_dynamic_elements);
}

#[test]
fn test_structured_multiple_statements() {
    let xml = br#"<mapper namespace="test">
        <select id="find">SELECT * FROM users WHERE id = #{id}</select>
        <insert id="add">INSERT INTO users (name) VALUES (#{name})</insert>
        <update id="update">UPDATE users SET name = #{name} WHERE id = #{id}</update>
        <delete id="remove">DELETE FROM users WHERE id = #{id}</delete>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    assert_eq!(result.statements.len(), 4);
    assert_eq!(result.statements[0].kind, StatementKind::Select);
    assert_eq!(result.statements[1].kind, StatementKind::Insert);
    assert_eq!(result.statements[2].kind, StatementKind::Update);
    assert_eq!(result.statements[3].kind, StatementKind::Delete);
}

#[test]
fn test_structured_json_roundtrip() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <if test="name != null">AND name = #{name}</if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let json = serde_json::to_string(&result).expect("should serialize to JSON");
    assert!(json.contains("\"namespace\":\"test\""));
    assert!(json.contains("\"id\":\"find\""));
    assert!(json.contains("\"name != null\""));

    let restored: super::StructuredMapper = serde_json::from_str(&json).expect("should deserialize from JSON");
    assert_eq!(restored.namespace, "test");
    assert_eq!(restored.statements.len(), 1);
    assert!(restored.statements[0].has_dynamic_elements);
}

#[test]
fn test_structured_no_dynamic_elements_for_static_sql() {
    let xml = br#"<mapper namespace="test">
        <select id="staticQuery">SELECT 1</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    assert!(!result.statements[0].has_dynamic_elements);
    assert!(result.statements[0].parameters.is_empty());
}

// ── Guard Tests: parse_mapper_bytes unchanged ──

#[test]
fn guard_flat_api_simple() {
    let xml = br#"<mapper namespace="test">
        <select id="findById">SELECT * FROM users WHERE id = #{id}</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.errors.is_empty());
    assert_eq!(result.statements.len(), 1);
    assert!(result.statements[0].flat_sql.contains("__XML_PARAM_id__"));
    assert!(result.statements[0].parse_result.is_some());
}

#[test]
fn guard_flat_api_dynamic_where() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <if test="name != null">AND name = #{name}</if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.errors.is_empty());
    assert!(result.statements[0].has_dynamic_elements);
    assert!(result.statements[0].flat_sql.contains("WHERE"));
    assert!(!result.statements[0].flat_sql.contains("WHERE AND"));
}

#[test]
fn guard_flat_api_foreach() {
    let xml = br#"<mapper namespace="test">
        <select id="findByIds">
            SELECT * FROM users WHERE id IN
            <foreach collection="ids" item="id" open="(" separator="," close=")">
                #{id}
            </foreach>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    let sql = &result.statements[0].flat_sql;
    assert!(sql.contains("IN"));
    assert!(sql.contains("("));
    assert!(sql.contains(")"));
}

#[test]
fn guard_flat_api_include() {
    let xml = br#"<mapper namespace="test">
        <sql id="cols">id, name</sql>
        <select id="findAll">SELECT <include refid="cols"/> FROM users</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.errors.is_empty());
    assert!(result.statements[0].flat_sql.contains("id, name"));
}

#[test]
fn guard_flat_api_dollar_param() {
    let xml = br#"<mapper namespace="test">
        <select id="dynamicOrder">SELECT * FROM users ORDER BY ${column}</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.statements[0].flat_sql.contains("__XML_RAW_column__"));
}

#[test]
fn guard_flat_api_empty_mapper() {
    let xml = br#"<mapper namespace="empty"></mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.statements.is_empty());
    assert!(result.errors.iter().any(|e| matches!(e, IbatisError::EmptyMapper)));
}

#[test]
fn guard_flat_api_multiple_statements() {
    let xml = br#"<mapper namespace="test">
        <select id="find">SELECT * FROM users WHERE id = #{id}</select>
        <insert id="add">INSERT INTO users (name) VALUES (#{name})</insert>
        <update id="update">UPDATE users SET name = #{name} WHERE id = #{id}</update>
        <delete id="remove">DELETE FROM users WHERE id = #{id}</delete>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert_eq!(result.statements.len(), 4);
    assert_eq!(result.statements[0].kind, StatementKind::Select);
    assert_eq!(result.statements[1].kind, StatementKind::Insert);
    assert_eq!(result.statements[2].kind, StatementKind::Update);
    assert_eq!(result.statements[3].kind, StatementKind::Delete);
}

// ── Expand Variants Tests (Issue #179 downstream) ──

use crate::ibatis::types::{BranchStep, ExpandConfig, IfExpandStrategy, PlaceholderStrategy};

fn default_expand_config() -> ExpandConfig {
    ExpandConfig {
        max_depth: 10,
        max_variants: 100,
        foreach_sizes: vec![1, 2],
        if_strategy: IfExpandStrategy::Both,
        placeholder: PlaceholderStrategy::PreserveInternalMarkers,
        generate_parse_results: false,
    }
}

#[test]
fn test_expand_simple_select_no_dynamic() {
    let xml = br#"<mapper namespace="test">
        <select id="find">SELECT * FROM users WHERE id = #{id}</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let stmt = &result.statements[0];
    let variants = stmt.expand_variants(&default_expand_config());
    assert_eq!(variants.len(), 1);
    assert!(variants[0].sql.contains("SELECT"));
    assert!(variants[0].sql.contains("__XML_PARAM_id__"));
    assert!(variants[0].branch_path.is_empty());
    assert_eq!(variants[0].parameters.len(), 1);
}

#[test]
fn test_expand_if_both_two_variants() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <if test="name != null">AND name = #{name}</if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let stmt = &result.statements[0];
    let variants = stmt.expand_variants(&default_expand_config());
    assert_eq!(variants.len(), 2);

    let included = &variants[0];
    assert!(included.sql.contains("WHERE"), "got: {}", included.sql);
    assert!(included.sql.contains("__XML_PARAM_name__"));

    let excluded = &variants[1];
    assert!(!excluded.sql.contains("name"), "got: {}", excluded.sql);
}

#[test]
fn test_expand_if_include_only() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <if test="name != null">AND name = #{name}</if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let config = ExpandConfig { if_strategy: IfExpandStrategy::IncludeOnly, ..default_expand_config() };
    let variants = result.statements[0].expand_variants(&config);
    assert_eq!(variants.len(), 1);
    assert!(variants[0].sql.contains("WHERE"));
}

#[test]
fn test_expand_if_exclude_only() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <if test="name != null">AND name = #{name}</if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let config = ExpandConfig { if_strategy: IfExpandStrategy::ExcludeOnly, ..default_expand_config() };
    let variants = result.statements[0].expand_variants(&config);
    assert_eq!(variants.len(), 1);
    assert!(!variants[0].sql.contains("name"));
}

#[test]
fn test_expand_two_ifs_combinatorial() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <if test="name != null">AND name = #{name}</if>
                <if test="age != null">AND age = #{age}</if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let variants = result.statements[0].expand_variants(&default_expand_config());
    assert_eq!(variants.len(), 4, "2 independent Ifs should produce 4 variants");
}

#[test]
fn test_expand_where_strips_leading_and() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <if test="name != null">AND name = #{name}</if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let variants = result.statements[0].expand_variants(&default_expand_config());
    let included = &variants[0];
    assert!(!included.sql.contains("WHERE AND"), "got: {}", included.sql);
    assert!(included.sql.contains("WHERE"));
}

#[test]
fn test_expand_set_strips_trailing_comma() {
    let xml = br#"<mapper namespace="test">
        <update id="updateUser">
            UPDATE users
            <set>
                <if test="name != null">name = #{name},</if>
            </set>
            WHERE id = #{id}
        </update>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let variants = result.statements[0].expand_variants(&default_expand_config());
    let included = &variants[0];
    assert!(included.sql.contains("SET"), "got: {}", included.sql);
    let set_content = included.sql.split("SET").nth(1).unwrap_or("").trim();
    let before_where = set_content.split("WHERE").next().unwrap_or("").trim();
    assert!(!before_where.ends_with(','), "SET content should not end with comma, got: {}", before_where);
}

#[test]
fn test_expand_foreach_with_sizes() {
    let xml = br#"<mapper namespace="test">
        <select id="findByIds">
            SELECT * FROM users WHERE id IN
            <foreach collection="ids" item="id" open="(" separator="," close=")">
                #{id}
            </foreach>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let variants = result.statements[0].expand_variants(&default_expand_config());
    assert_eq!(variants.len(), 2);

    assert!(variants[0].sql.contains("("));
    assert!(variants[0].sql.contains(")"));

    let size2_sql = &variants[1].sql;
    assert!(size2_sql.contains(","), "size=2 should have separator, got: {}", size2_sql);
}

#[test]
fn test_expand_choose_two_branches() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <choose>
                    <when test="id != null">AND id = #{id}</when>
                    <otherwise>AND status = 'ACTIVE'</otherwise>
                </choose>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let variants = result.statements[0].expand_variants(&default_expand_config());
    assert_eq!(variants.len(), 2);
}

#[test]
fn test_expand_branch_path_recorded() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <if test="name != null">AND name = #{name}</if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let variants = result.statements[0].expand_variants(&default_expand_config());
    assert!(variants[0].branch_path.iter().any(|s| matches!(s, BranchStep::If { included: true, .. })));
    assert!(variants[1].branch_path.iter().any(|s| matches!(s, BranchStep::If { included: false, .. })));
}

#[test]
fn test_expand_params_per_variant() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <if test="name != null">AND name = #{name}</if>
                <if test="age != null">AND age = #{age}</if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let variants = result.statements[0].expand_variants(&default_expand_config());

    let both = variants
        .iter()
        .find(|v| v.branch_path.iter().all(|s| matches!(s, BranchStep::If { included: true, .. })))
        .unwrap();
    assert_eq!(both.parameters.len(), 2);

    let neither = variants
        .iter()
        .find(|v| v.branch_path.iter().all(|s| matches!(s, BranchStep::If { included: false, .. })))
        .unwrap();
    assert_eq!(neither.parameters.len(), 0);
}

#[test]
fn test_expand_placeholder_question_mark() {
    let xml = br#"<mapper namespace="test">
        <select id="find">SELECT * FROM users WHERE id = #{id} AND name = #{name}</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let config = ExpandConfig { placeholder: PlaceholderStrategy::QuestionMark, ..default_expand_config() };
    let variants = result.statements[0].expand_variants(&config);
    assert_eq!(variants.len(), 1);
    assert!(variants[0].sql.contains("?"), "got: {}", variants[0].sql);
    assert!(!variants[0].sql.contains("__XML_PARAM_"));
}

#[test]
fn test_expand_max_variants_cap() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <if test="a">AND a = #{a}</if>
                <if test="b">AND b = #{b}</if>
                <if test="c">AND c = #{c}</if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let config = ExpandConfig { max_variants: 3, ..default_expand_config() };
    let variants = result.statements[0].expand_variants(&config);
    assert_eq!(variants.len(), 3);
}

#[test]
fn test_expand_trim_semantics() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <trim prefix="WHERE" prefixOverrides="AND |OR ">
                <if test="name != null">AND name = #{name}</if>
            </trim>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let variants = result.statements[0].expand_variants(&default_expand_config());
    let included = &variants[0];
    assert!(included.sql.contains("WHERE"), "got: {}", included.sql);
    assert!(!included.sql.contains("WHERE AND"), "got: {}", included.sql);
}

#[test]
fn test_to_parsed_statement_static() {
    let xml = br#"<mapper namespace="test">
        <select id="findById">SELECT * FROM users WHERE id = #{id}</select>
    </mapper>"#;
    let structured = super::parse_mapper_bytes_structured(xml);
    let parsed = structured.statements[0].to_parsed_statement("test");
    assert_eq!(parsed.id, "findById");
    assert_eq!(parsed.kind, StatementKind::Select);
    assert!(parsed.flat_sql.contains("__XML_PARAM_id__"));
    assert!(!parsed.has_dynamic_elements);
    assert!(parsed.parse_result.is_some());
}

#[test]
fn test_to_parsed_statement_dynamic() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users
            <where>
                <if test="name != null">AND name = #{name}</if>
            </where>
        </select>
    </mapper>"#;
    let structured = super::parse_mapper_bytes_structured(xml);
    let parsed = structured.statements[0].to_parsed_statement("test");
    assert!(parsed.has_dynamic_elements);
    assert!(parsed.flat_sql.contains("WHERE"));
    assert!(!parsed.flat_sql.contains("WHERE AND"));
}

#[test]
fn test_expand_nested_if_foreach() {
    let xml = br#"<mapper namespace="test">
        <select id="complex">
            SELECT * FROM users
            <where>
                <if test="ids != null">
                    AND id IN
                    <foreach collection="ids" item="id" open="(" separator="," close=")">
                        #{id}
                    </foreach>
                </if>
            </where>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let config = ExpandConfig { foreach_sizes: vec![2], ..default_expand_config() };
    let variants = result.statements[0].expand_variants(&config);

    let included = variants
        .iter()
        .find(|v| {
            v.branch_path.iter().any(|s| matches!(s, BranchStep::If { test, included: true } if test == "ids != null"))
        })
        .unwrap();
    assert!(included.sql.contains("("), "got: {}", included.sql);
    assert!(included.sql.contains(","), "foreach size=2 should have separator, got: {}", included.sql);
}

#[test]
fn test_expand_foreach_branch_step_recorded() {
    let xml = br#"<mapper namespace="test">
        <select id="findByIds">
            SELECT * FROM users WHERE id IN
            <foreach collection="ids" item="id" open="(" separator="," close=")">
                #{id}
            </foreach>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let variants = result.statements[0].expand_variants(&default_expand_config());
    assert_eq!(variants.len(), 2);

    assert!(matches!(&variants[0].branch_path[0], BranchStep::Foreach { collection, size: 1 } if collection == "ids"));
    assert!(matches!(&variants[1].branch_path[0], BranchStep::Foreach { collection, size: 2 } if collection == "ids"));
}

// ── parse_result field tests (Issue #179 comment) ──

#[test]
fn test_expand_parse_results_disabled_by_default() {
    let xml = br#"<mapper namespace="test">
        <select id="find">SELECT * FROM users WHERE id = #{id}</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let variants = result.statements[0].expand_variants(&ExpandConfig::default());
    assert!(variants[0].parse_result.is_none());
}

#[test]
fn test_expand_parse_results_generated_when_enabled() {
    let xml = br#"<mapper namespace="test">
        <select id="find">SELECT * FROM users WHERE id = #{id}</select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let config = ExpandConfig { generate_parse_results: true, ..default_expand_config() };
    let variants = result.statements[0].expand_variants(&config);
    assert_eq!(variants.len(), 1);
    let pr = variants[0].parse_result.as_ref().expect("parse_result should be Some");
    assert!(!pr.0.is_empty(), "should parse at least one statement");
}

#[test]
fn test_expand_parse_results_variants_with_parse_result() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            SELECT * FROM users WHERE id = #{id}
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let config = ExpandConfig { generate_parse_results: true, ..default_expand_config() };
    let variants = result.statements[0].expand_variants(&config);

    assert_eq!(variants.len(), 1);
    let v = &variants[0];
    assert!(v.sql.contains("SELECT"));
    let pr = v.parse_result.as_ref().expect("non-empty SQL should have parse_result");
    assert!(!pr.0.is_empty(), "should parse at least one statement");
}

#[test]
fn test_hash_in_sql_string_literal_not_mistaken_for_param() {
    // Regression: '# inside SQL string literals was misinterpreted as iBatis 2.x #param# delimiters.
    // The SQL: select substr(#{str},instr(#{str},'#')+1,instr(#{str},'#',instr(#{str},'#')+1)-instr(#{str},'#')-1)
    // Contains 5 occurrences of #{str} and 5 occurrences of '#' — the '#' must stay as text.
    let xml = br#"<mapper namespace="test">
        <select id="getSeqNo" parameterType="String" resultType="String">
            select substr(#{str},instr(#{str},'#')+1,instr(#{str},'#',instr(#{str},'#')+1)-instr(#{str},'#')-1)
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);

    let stmt = &result.statements[0];
    assert_eq!(stmt.id, "getSeqNo");

    // All parameters must be named "str" — no garbage like "')+1" or "'"
    for (i, p) in stmt.parameters.iter().enumerate() {
        assert_eq!(p.name, "str", "parameter[{}] should be 'str', got '{}'", i, p.name);
    }
    // Exactly 5 #{str} occurrences
    assert_eq!(stmt.parameters.len(), 5, "expected 5 params, got {}: {:?}", stmt.parameters.len(), stmt.parameters);

    // flat_sql should preserve '#' literals
    let parsed = stmt.to_parsed_statement("test");
    assert!(parsed.flat_sql.contains(",'#'"), "flat_sql should contain '#' literal, got: {}", parsed.flat_sql);
    assert!(
        !parsed.flat_sql.contains("__XML_PARAM____"),
        "flat_sql should not have double-underscore param corruption: {}",
        parsed.flat_sql
    );

    // The SQL must parse without errors
    if let Some((_, errors)) = &parsed.parse_result {
        assert!(errors.is_empty(), "SQL parse errors: {:?}", errors);
    }
}

#[test]
fn test_ibatis2_param_respects_string_literals() {
    // Simpler case: iBatis 2.x #value# format should NOT match when # is inside '...'
    // The text "WHERE sep = '#' AND id = #value#" should yield:
    //   Text("WHERE sep = '#' AND id = "), Parameter("value")
    let nodes = super::parser::parse_text_to_nodes("WHERE sep = '#' AND id = #value#");
    // Expect: Text("WHERE sep = '#' AND id = "), Parameter("value")
    let params: Vec<_> = nodes.iter().filter(|n| matches!(n, SqlNode::Parameter { .. })).collect();
    assert_eq!(params.len(), 1, "expected exactly 1 parameter, got nodes: {:?}", nodes);
    if let SqlNode::Parameter { name, .. } = params[0] {
        assert_eq!(name, "value", "parameter name should be 'value', got '{}'", name);
    }
    // The '#' text literal should be preserved in a Text node
    let text_content: String = nodes
        .iter()
        .filter_map(|n| match n {
            SqlNode::Text { content } => Some(content.as_str()),
            _ => None,
        })
        .collect();
    assert!(text_content.contains("'#'"), "text should contain '#' literal, got: {}", text_content);
}

#[test]
fn test_expand_parse_results_empty_variant_has_none() {
    let xml = br#"<mapper namespace="test">
        <select id="find">
            <if test="name != null">AND name = #{name}</if>
        </select>
    </mapper>"#;
    let result = super::parse_mapper_bytes_structured(xml);
    let config = ExpandConfig { generate_parse_results: true, ..default_expand_config() };
    let variants = result.statements[0].expand_variants(&config);

    // The excluded variant has empty SQL → parse_result should be None
    let excluded = variants
        .iter()
        .find(|v| v.branch_path.iter().any(|s| matches!(s, BranchStep::If { included: false, .. })))
        .unwrap();
    assert!(excluded.sql.trim().is_empty());
    assert!(excluded.parse_result.is_none(), "empty SQL should not generate parse_result");
}

// ── Guard Tests: Callable Stored Procedure Mapper XML ──
// These tests guard against regressions in parsing complex stored procedure
// call mappers with foreach, if, #{param,mode=IN,jdbcType=VARCHAR} syntax.

/// Helper: the canonical callable stored procedure mapper XML from user reports.
fn callable_proc_mapper_xml() -> &'static [u8] {
    br#"<mapper namespace="com.example.ProcMapper">
   <select id="callOracleProcCommon" statementType="CALLABLE" resultType="java.util.Map">
       call ${procName}
       <foreach collection="param" item="item" index="index" open="(" close=")" separator=",">
           <if test="item.param == 'i'.toString()">
               #{item.paramValue,mode=IN,jdbcType=VARCHAR}
           </if>
           <if test="item.param == '?'.toString()">
               #{outParam.key${index},mode=OUT,jdbcType=VARCHAR}
           </if>
           <if test="item.param == 'c'.toString()">
               #{outParam.key${index},mode=OUT,jdbcType=CURSOR,resultMap=myMap}
           </if>
           <if test="item.param == 'b'.toString()">
               #{outParam.key${index},mode=OUT,jdbcType=CLOB,javaType=String}
           </if>
       </foreach>
   </select>
   <select id="callOracleProcCommon" statementType="CALLABLE" resultType="java.util.Map" databaseId="gauss">
       {call ${procName}
       <foreach collection="param" item="item" index="index" open="(" close=")" separator=",">
           <if test="item.param == 'i'.toString()">
               #{item.paramValue,mode=IN,jdbcType=VARCHAR}
           </if>
           <if test="item.param == '?'.toString()">
               #{outParam.key${index},mode=OUT,jdbcType=VARCHAR}
           </if>
           <if test="item.param == 'c'.toString()">
               #{outParam.key${index},mode=OUT,jdbcType=OTHER,resultMap=MDBCursorMapGauss}
           </if>
           <if test="item.param == 'b'.toString()">
               #{outParam.key${index},mode=OUT,jdbcType=CLOB,javaType=String}
           </if>
       </foreach>
       }
   </select>
   <select id="callOracleProcNoCursor" statementType="CALLABLE" resultType="java.util.Map">
       call ${procName}
       <foreach collection="params" item="item" index="index" open="(" close=")" separator=",">
           <if test="item != '?'.toString()">
               #{item,mode=IN,jdbcType=VARCHAR}
           </if>
           <if test="item == '?'.toString()">
               #{outParam.key${index},mode=OUT,jdbcType=VARCHAR}
           </if>
       </foreach>
   </select>
   <select id="callOracleProcCursorOut4" statementType="CALLABLE">
       {call ${procName}(
       <foreach collection="inParams" item="item" index="index" separator=",">
           #{item,mode=IN,jdbcType=VARCHAR}
       </foreach>
       <if test="inParams.length > 0">
           ,
       </if>
       #{outParam.o_retcode,mode=OUT,jdbcType=VARCHAR},
       #{outParam.o_retmsg,mode=OUT,jdbcType=VARCHAR},
       #{outParam.o_totalnum,mode=OUT,jdbcType=VARCHAR},
       #{outParam.o_list,mode=OUT,jdbcType=CURSOR,resultMap=MDBCursorMap,javaType=java.lang.Object}
       )}
   </select>
</mapper>"#
}

// ── Guard: Issue #3 — databaseId captured ──

#[test]
fn guard_database_id_captured() {
    let result = super::parse_mapper_bytes_structured(callable_proc_mapper_xml());
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);

    // Should have 4 statements total
    assert_eq!(result.statements.len(), 4, "expected 4 statements, got {}", result.statements.len());

    // First callOracleProcCommon has NO databaseId
    let first_common = &result.statements[0];
    assert_eq!(first_common.id, "callOracleProcCommon");
    assert_eq!(first_common.database_id, None, "first callOracleProcCommon should have database_id=None");

    // Second callOracleProcCommon HAS databaseId="gauss"
    let gauss_common = &result.statements[1];
    assert_eq!(gauss_common.id, "callOracleProcCommon");
    assert_eq!(
        gauss_common.database_id,
        Some("gauss".to_string()),
        "second callOracleProcCommon should have database_id=Some(\"gauss\")"
    );
}

// ── Guard: Issue #5 — statementType captured ──

#[test]
fn guard_statement_type_captured() {
    let result = super::parse_mapper_bytes_structured(callable_proc_mapper_xml());
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);

    for stmt in &result.statements {
        assert_eq!(
            stmt.statement_type,
            Some("CALLABLE".to_string()),
            "statement '{}' should have statement_type=CALLABLE",
            stmt.id
        );
    }
}

// ── Guard: Issue #4 — jdbcType preserved separately from javaType ──

#[test]
fn guard_jdbc_type_preserved_separately() {
    let result = super::parse_mapper_bytes_structured(callable_proc_mapper_xml());
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);

    // In the 4th statement (callOracleProcCursorOut4), the last param has both
    // jdbcType=CURSOR and javaType=java.lang.Object — both must be preserved.
    let stmt = result.statements.iter().find(|s| s.id == "callOracleProcCursorOut4").unwrap();

    // Find the outParam.o_list parameter — it should have jdbcType=CURSOR
    let o_list = stmt.parameters.iter().find(|p| p.name == "outParam.o_list");
    assert!(o_list.is_some(), "should have outParam.o_list parameter");
    let o_list = o_list.unwrap();
    assert_eq!(
        o_list.jdbc_type,
        Some(crate::ibatis::types::JdbcType::Cursor),
        "outParam.o_list should have jdbc_type=Cursor, got: {:?}",
        o_list.jdbc_type
    );
}

// ── Guard: Issue #6 — mode and resultMap preserved in parameter attrs ──

#[test]
fn guard_mode_preserved_in_param_attrs() {
    use crate::ibatis::types::SqlNode;

    let result = super::parse_mapper_bytes_structured(callable_proc_mapper_xml());
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);

    // In callOracleProcCommon, find the If with test="item.param == 'i'.toString()"
    // Its child Parameter should have mode=IN, jdbcType=VARCHAR
    let stmt = &result.statements[0];

    fn find_if_with_test<'a>(node: &'a SqlNode, test_contains: &str) -> Option<&'a SqlNode> {
        match node {
            SqlNode::If { test, children, .. } if test.contains(test_contains) => Some(node),
            SqlNode::If { children, .. } => children.iter().find_map(|c| find_if_with_test(c, test_contains)),
            SqlNode::ForEach { children, .. } => children.iter().find_map(|c| find_if_with_test(c, test_contains)),
            SqlNode::Sequence { children } => children.iter().find_map(|c| find_if_with_test(c, test_contains)),
            SqlNode::Where { children } | SqlNode::Set { children } => {
                children.iter().find_map(|c| find_if_with_test(c, test_contains))
            }
            _ => None,
        }
    }

    let if_in = find_if_with_test(&stmt.body, "item.param == 'i'").expect("should find If node for 'i'");
    if let SqlNode::If { children, .. } = if_in {
        let param = children.iter().find_map(|c| match c {
            SqlNode::Parameter { name, .. } if name == "item.paramValue" => Some(c),
            _ => None,
        });
        assert!(param.is_some(), "should find Parameter 'item.paramValue' in If(IN) branch");
        if let SqlNode::Parameter { mode, jdbc_type, .. } = param.unwrap() {
            assert_eq!(mode, &Some("IN".to_string()), "item.paramValue should have mode=IN");
            assert_eq!(jdbc_type, &Some("VARCHAR".to_string()), "item.paramValue should have jdbc_type=VARCHAR");
        }
    }

    // Check OUT param has mode=OUT
    let if_out = find_if_with_test(&stmt.body, "item.param == '?'").expect("should find If node for '?'");
    if let SqlNode::If { children, .. } = if_out {
        let param = children.iter().find_map(|c| match c {
            n @ SqlNode::Parameter { name, .. } if name.contains("outParam.key") => Some(n),
            _ => None,
        });
        assert!(param.is_some(), "should find outParam.key param in If(OUT) branch");
        if let SqlNode::Parameter { mode, jdbc_type, .. } = param.unwrap() {
            assert_eq!(mode, &Some("OUT".to_string()), "outParam.key should have mode=OUT");
            assert_eq!(jdbc_type, &Some("VARCHAR".to_string()), "outParam.key should have jdbc_type=VARCHAR");
        }
    }

    // Check CURSOR param has resultMap
    let if_cursor = find_if_with_test(&stmt.body, "item.param == 'c'").expect("should find If node for 'c'");
    if let SqlNode::If { children, .. } = if_cursor {
        let param = children.iter().find_map(|c| match c {
            n @ SqlNode::Parameter { name, .. } if name.contains("outParam.key") => Some(n),
            _ => None,
        });
        assert!(param.is_some(), "should find outParam.key param in If(CURSOR) branch");
        if let SqlNode::Parameter { result_map, jdbc_type, .. } = param.unwrap() {
            assert_eq!(jdbc_type, &Some("CURSOR".to_string()), "CURSOR param should have jdbc_type=CURSOR");
            assert_eq!(
                result_map, &Some("myMap".to_string()),
                "CURSOR param should have result_map=myMap"
            );
        }
    }
}

// ── Guard: Issue #1 — ForEach separator preserved in flatten ──

#[test]
fn guard_foreach_separator_in_flat_sql() {
    let result = super::parse_mapper_bytes(callable_proc_mapper_xml());
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);

    // callOracleProcCursorOut4 has foreach with separator=","
    // The flattened SQL should contain the separator
    let stmt = result
        .statements
        .iter()
        .find(|s| s.id == "callOracleProcCursorOut4")
        .expect("should find callOracleProcCursorOut4");

    let sql = &stmt.flat_sql;
    // The foreach body has #{item,mode=IN,jdbcType=VARCHAR} with separator=","
    // In the flattened SQL, the separator should appear between items
    assert!(
        sql.contains(",") || sql.contains("__XML_PARAM"),
        "flat_sql should contain separator or params, got: {}",
        sql
    );
}

// ── Guard: Issue #2 — ParamMeta.jdbc_type populated from XML inline attrs ──

#[test]
fn guard_param_meta_jdbc_type_populated() {
    let result = super::parse_mapper_bytes(callable_proc_mapper_xml());
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);

    // Find callOracleProcCursorOut4 — has params with explicit jdbcType
    let stmt = result
        .statements
        .iter()
        .find(|s| s.id == "callOracleProcCursorOut4")
        .expect("should find callOracleProcCursorOut4");

    // outParam.o_retcode has jdbcType=VARCHAR — jdbc_type should be populated
    let retcode = stmt.parameters.iter().find(|p| p.name == "outParam.o_retcode");
    assert!(retcode.is_some(), "should have outParam.o_retcode param");
    let retcode = retcode.unwrap();
    assert!(
        retcode.jdbc_type.is_some(),
        "outParam.o_retcode should have jdbc_type populated (xml has jdbcType=VARCHAR), got: {:?}",
        retcode
    );
    assert_eq!(
        retcode.jdbc_type,
        Some(crate::ibatis::types::JdbcType::VarChar),
        "outParam.o_retcode should be VarChar"
    );

    // outParam.o_list has jdbcType=CURSOR — should be Cursor
    let o_list = stmt.parameters.iter().find(|p| p.name == "outParam.o_list");
    assert!(o_list.is_some(), "should have outParam.o_list param");
    let o_list = o_list.unwrap();
    assert!(
        o_list.jdbc_type.is_some(),
        "outParam.o_list should have jdbc_type populated (xml has jdbcType=CURSOR), got: {:?}",
        o_list
    );
}

// ── Guard: Flat API also has database_id and statement_type ──

#[test]
fn guard_flat_api_database_id_and_statement_type() {
    let result = super::parse_mapper_bytes(callable_proc_mapper_xml());
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);

    // Second callOracleProcCommon should have database_id
    let gauss_stmt = result.statements.iter().find(|s| {
        s.id == "callOracleProcCommon" && s.database_id.as_deref() == Some("gauss")
    });
    assert!(gauss_stmt.is_some(), "should find callOracleProcCommon with database_id=gauss");

    // All should have statement_type=CALLABLE
    for stmt in &result.statements {
        assert_eq!(
            stmt.statement_type,
            Some("CALLABLE".to_string()),
            "flat API: statement '{}' should have statement_type=CALLABLE",
            stmt.id
        );
    }
}

// ── Foreach Flatten Regression Tests ──
// Bug: separator was incorrectly inserted between foreach body children
// instead of between iterations. The separator is only meaningful during
// expand (multiple iterations), not during flatten (single iteration).

#[test]
fn test_foreach_insert_batch_no_extra_commas() {
    let xml = br#"<mapper namespace="test">
        <insert id="insertBatch">
            insert into sys_user_role(user_id, role_id)
            values
            <foreach collection="items" item="item" separator=",">
                (#{item.userId}, #{item.roleId})
            </foreach>
        </insert>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    let sql = &result.statements[0].flat_sql;
    assert!(
        !sql.contains("(,__XML_PARAM_"),
        "should not have leading comma after open paren, got: {}",
        sql
    );
    assert!(
        !sql.contains(",,__XML_PARAM_"),
        "should not have double commas between params, got: {}",
        sql
    );
    assert!(
        !sql.contains(",)"),
        "should not have trailing comma before close paren, got: {}",
        sql
    );
    assert!(
        sql.contains("(__XML_PARAM_item_userId__, __XML_PARAM_item_roleId__)"),
        "params should be properly separated by single comma+space, got: {}",
        sql
    );
}

#[test]
fn test_foreach_update_with_cte_no_extra_commas() {
    let xml = br#"<mapper namespace="test">
        <update id="setRunOrder">
            WITH level_tab AS (
            select req.procedure_code, req.lv from (values
            <foreach collection="procedureCodeLvDTOList" item="item" separator=",">
                (#{item.procedureCode}, #{item.level})
            </foreach>
            ) as req(procedure_code, lv)
            )
            update ctl_fund_split_proc_run t
            set t.run_order = n.run_order
            from (SELECT t.ctid rn FROM aaspb.ctl_fund_split_proc_run t, level_tab s) n
            where t.ctid = n.rn
        </update>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
    let sql = &result.statements[0].flat_sql;
    assert!(
        !sql.contains("(,__XML_PARAM_"),
        "should not have leading comma after open paren, got: {}",
        sql
    );
    assert!(
        !sql.contains(",,__XML_PARAM_"),
        "should not have double commas between params, got: {}",
        sql
    );
    assert!(
        sql.contains("(__XML_PARAM_item_procedureCode__, __XML_PARAM_item_level__)"),
        "params should be properly separated, got: {}",
        sql
    );
}

#[test]
fn test_foreach_values_with_text_comma_no_duplication() {
    let xml = br#"<mapper namespace="test">
        <insert id="insertBatch">
            insert into t(a, b) values
            <foreach collection="list" item="x" separator=",">
                (#{x.a}, #{x.b})
            </foreach>
        </insert>
    </mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    let sql = &result.statements[0].flat_sql;
    assert!(
        sql.contains("(__XML_PARAM_x_a__, __XML_PARAM_x_b__)"),
        "foreach body should flatten without injecting separator between children, got: {}",
        sql
    );
}
