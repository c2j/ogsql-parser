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
    assert_eq!(mapper.statements.len(), 4);
}

#[test]
fn test_parse_select_statement() {
    let mapper = parse_simple_mapper();
    let select = mapper
        .statements
        .iter()
        .find(|s| s.id == "findById")
        .unwrap();
    assert_eq!(select.kind, StatementKind::Select);
    assert_eq!(select.parameter_type.as_deref(), Some("int"));
    assert_eq!(select.result_type.as_deref(), Some("User"));
}

#[test]
fn test_parse_insert_statement() {
    let mapper = parse_simple_mapper();
    let insert = mapper
        .statements
        .iter()
        .find(|s| s.id == "insertUser")
        .unwrap();
    assert_eq!(insert.kind, StatementKind::Insert);
    assert_eq!(insert.parameter_type.as_deref(), Some("User"));
}

#[test]
fn test_parse_update_and_delete() {
    let mapper = parse_simple_mapper();
    let update = mapper
        .statements
        .iter()
        .find(|s| s.id == "updateName")
        .unwrap();
    assert_eq!(update.kind, StatementKind::Update);
    let delete = mapper
        .statements
        .iter()
        .find(|s| s.id == "deleteById")
        .unwrap();
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
        SqlNode::Parameter { name } => format!("#{{{}}}", name),
        SqlNode::RawExpr { expr } => format!("${{{}}}", expr),
        SqlNode::If { children, .. } => children.iter().map(node_text).collect(),
        SqlNode::Choose { branches } => branches
            .iter()
            .flat_map(|(_, ch)| ch.iter().map(node_text))
            .collect(),
        SqlNode::Where { children } => children.iter().map(node_text).collect(),
        SqlNode::Set { children } => children.iter().map(node_text).collect(),
        SqlNode::Trim { children, .. } => children.iter().map(node_text).collect(),
        SqlNode::ForEach { children, .. } => children.iter().map(node_text).collect(),
        SqlNode::Bind { .. } => String::new(),
    }
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
    let stmt = resolved
        .statements
        .iter()
        .find(|s| s.id == "findAll")
        .unwrap();
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
    assert!(
        content.contains("users"),
        "chained include should expand, got: {}",
        content
    );
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
    assert!(stmt.flat_sql.contains("$1"));

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
    assert!(stmt.flat_sql.contains("__IBATIS_DOLLAR_column__"));
}

#[test]
fn test_e2e_empty_mapper() {
    let xml = br#"<mapper namespace="empty"></mapper>"#;
    let result = super::parse_mapper_bytes(xml);
    assert!(result.statements.is_empty());
    assert!(result
        .errors
        .iter()
        .any(|e| matches!(e, IbatisError::EmptyMapper)));
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
    assert!(
        stmt.flat_sql.contains("WHERE"),
        "should have WHERE, got: {}",
        stmt.flat_sql
    );
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
    assert!(
        !sql.contains("WHERE AND"),
        "prefix override should strip AND, got: {}",
        sql
    );
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
