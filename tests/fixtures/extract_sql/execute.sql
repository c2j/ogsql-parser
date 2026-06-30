-- Issue: N/A
-- Description: EXECUTE IMMEDIATE scenarios: string literal, dollar-quoted, concatenation, USING clause, INTO clause
-- Expect: snapshot
-- Command: parse --extract-sql

-- ============================================
-- 1. EXECUTE IMMEDIATE with simple string
-- ============================================
CREATE OR REPLACE PROCEDURE test_execute_string(
    p_table_name VARCHAR
) AS
BEGIN
    EXECUTE IMMEDIATE 'SELECT COUNT(*) FROM ' || p_table_name;
END;
/

-- ============================================
-- 2. EXECUTE IMMEDIATE with dollar-quoted string
-- ============================================
CREATE OR REPLACE PROCEDURE test_execute_dollar(
    p_id INTEGER
) AS
BEGIN
    EXECUTE $$UPDATE users SET status = 'active' WHERE id = $$ || p_id;
END;
/

-- ============================================
-- 3. EXECUTE IMMEDIATE with USING clause
-- ============================================
CREATE OR REPLACE PROCEDURE test_execute_using(
    p_id INTEGER,
    p_name VARCHAR
) AS
BEGIN
    EXECUTE IMMEDIATE 'UPDATE users SET name = $2 WHERE id = $1' USING p_id, p_name;
END;
/

-- ============================================
-- 4. EXECUTE IMMEDIATE with INTO clause
-- ============================================
CREATE OR REPLACE PROCEDURE test_execute_into(
    p_id INTEGER
) AS
    v_name VARCHAR(100);
    v_age INTEGER;
BEGIN
    EXECUTE IMMEDIATE 'SELECT name, age FROM users WHERE id = ' || p_id INTO v_name, v_age;
END;
/

-- ============================================
-- 5. EXECUTE with pre-built SQL variable (simple assignment)
-- ============================================
CREATE OR REPLACE PROCEDURE test_execute_var_trace(
    p_id INTEGER
) AS
    v_sql VARCHAR(1000);
    v_count INTEGER;
BEGIN
    v_sql := 'SELECT COUNT(*) FROM users WHERE id = ' || p_id;
    EXECUTE IMMEDIATE v_sql INTO v_count;
END;
/

-- ============================================
-- 6. EXECUTE with multi-step SQL construction
-- ============================================
CREATE OR REPLACE PROCEDURE test_execute_multi_step(
    p_table VARCHAR,
    p_column VARCHAR,
    p_value VARCHAR
) AS
    v_sql VARCHAR(4000);
    v_count INTEGER;
BEGIN
    v_sql := 'SELECT COUNT(*) FROM ' || p_table;
    v_sql := v_sql || ' WHERE ' || p_column || ' = ''' || p_value || '''';
    EXECUTE IMMEDIATE v_sql INTO v_count;
END;
/

-- ============================================
-- 7. EXECUTE with complex concatenation
-- ============================================
CREATE OR REPLACE PROCEDURE test_execute_complex_concat(
    p_schema VARCHAR,
    p_table VARCHAR,
    p_id INTEGER
) AS
    v_sql VARCHAR(4000);
    v_result VARCHAR(100);
BEGIN
    v_sql := 'SELECT name FROM ' || p_schema || '.' || p_table || ' WHERE id = ' || p_id;
    EXECUTE IMMEDIATE v_sql INTO v_result;
END;
/

-- ============================================
-- 8. EXECUTE with DELETE
-- ============================================
CREATE OR REPLACE PROCEDURE test_execute_delete(
    p_table VARCHAR,
    p_id INTEGER
) AS
BEGIN
    EXECUTE IMMEDIATE 'DELETE FROM ' || p_table || ' WHERE id = ' || p_id;
END;
/

-- ============================================
-- 9. EXECUTE with INSERT from SELECT
-- ============================================
CREATE OR REPLACE PROCEDURE test_execute_insert_select(
    p_source_table VARCHAR,
    p_target_table VARCHAR
) AS
BEGIN
    EXECUTE IMMEDIATE 'INSERT INTO ' || p_target_table || ' SELECT * FROM ' || p_source_table;
END;
/

-- ============================================
-- 10. EXECUTE with bulk operation (FORALL equivalent via dynamic)
-- ============================================
CREATE OR REPLACE PROCEDURE test_execute_bulk(
    p_table VARCHAR,
    p_batch_size INTEGER
) AS
    v_sql VARCHAR(4000);
BEGIN
    v_sql := 'DELETE FROM ' || p_table || ' WHERE ROWNUM <= ' || p_batch_size;
    EXECUTE IMMEDIATE v_sql;
END;
/

-- ============================================
-- 11. EXECUTE using nested variable trace
-- ============================================
CREATE OR REPLACE PROCEDURE test_execute_nested_trace(
    p_condition VARCHAR
) AS
    v_base_sql VARCHAR(2000);
    v_full_sql VARCHAR(4000);
    v_count INTEGER;
BEGIN
    v_base_sql := 'SELECT COUNT(*) FROM users WHERE 1=1';
    v_full_sql := v_base_sql || ' AND ' || p_condition;
    EXECUTE IMMEDIATE v_full_sql INTO v_count;
END;
/

-- ============================================
-- 12. EXECUTE with function call in SQL
-- ============================================
CREATE OR REPLACE PROCEDURE test_execute_function(
    p_user_id INTEGER
) AS
    v_sql VARCHAR(1000);
    v_result VARCHAR(100);
BEGIN
    v_sql := 'SELECT UPPER(name) FROM users WHERE id = ' || p_user_id;
    EXECUTE IMMEDIATE v_sql INTO v_result;
END;
/
