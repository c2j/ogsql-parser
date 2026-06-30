-- Issue: N/A
-- Description: IF/ELSE/ELSIF and CASE branches with SQL in each branch
-- Expect: snapshot
-- Command: parse --extract-sql

-- ============================================
-- 1. IF-ELSE with SQL in both branches
-- ============================================
CREATE OR REPLACE PROCEDURE test_if_else_sql(
    p_flag VARCHAR,
    p_id INTEGER
) AS
    v_name VARCHAR(100);
BEGIN
    IF p_flag = 'A' THEN
        SELECT name INTO v_name FROM users WHERE id = p_id AND type = 'A';
    ELSE
        SELECT name INTO v_name FROM users WHERE id = p_id AND type = 'B';
    END IF;
    INSERT INTO audit_log (user_id, name_found) VALUES (p_id, v_name);
END;
/

-- ============================================
-- 2. IF-ELSIF-ELSE with SQL in all branches
-- ============================================
CREATE OR REPLACE PROCEDURE test_if_elsif_sql(
    p_action VARCHAR,
    p_id INTEGER,
    p_value VARCHAR
) AS
BEGIN
    IF p_action = 'INSERT' THEN
        INSERT INTO data_table (id, value) VALUES (p_id, p_value);
    ELSIF p_action = 'UPDATE' THEN
        UPDATE data_table SET value = p_value WHERE id = p_id;
    ELSIF p_action = 'DELETE' THEN
        DELETE FROM data_table WHERE id = p_id;
    ELSE
        SELECT value FROM data_table WHERE id = p_id;
    END IF;
END;
/

-- ============================================
-- 3. Nested IF with SQL
-- ============================================
CREATE OR REPLACE PROCEDURE test_nested_if_sql(
    p_type VARCHAR,
    p_status VARCHAR,
    p_id INTEGER
) AS
    v_result VARCHAR(100);
BEGIN
    IF p_type = 'USER' THEN
        IF p_status = 'ACTIVE' THEN
            SELECT name INTO v_result FROM users WHERE id = p_id;
        ELSE
            SELECT name INTO v_result FROM users_archive WHERE id = p_id;
        END IF;
        INSERT INTO logs (msg) VALUES ('User result: ' || v_result);
    ELSE
        SELECT dept_name INTO v_result FROM departments WHERE id = p_id;
    END IF;
END;
/

-- ============================================
-- 4. CASE-WHEN with SQL in each case
-- ============================================
CREATE OR REPLACE PROCEDURE test_case_sql(
    p_op VARCHAR,
    p_table_id INTEGER
) AS
    v_count INTEGER;
BEGIN
    CASE p_op
        WHEN 'COUNT_ALL' THEN
            SELECT COUNT(*) INTO v_count FROM users;
        WHEN 'COUNT_ACTIVE' THEN
            SELECT COUNT(*) INTO v_count FROM users WHERE status = 'active';
        WHEN 'COUNT_BY_ID' THEN
            SELECT COUNT(*) INTO v_count FROM orders WHERE user_id = p_table_id;
        ELSE
            v_count := 0;
    END CASE;
    INSERT INTO stats (op, result) VALUES (p_op, v_count);
END;
/

-- ============================================
-- 5. Searched CASE with SQL
-- ============================================
CREATE OR REPLACE PROCEDURE test_searched_case_sql(
    p_score INTEGER,
    p_user_id INTEGER
) AS
    v_grade VARCHAR(10);
BEGIN
    CASE
        WHEN p_score >= 90 THEN
            SELECT 'A' INTO v_grade FROM dual;
            UPDATE users SET grade = 'A' WHERE id = p_user_id;
        WHEN p_score >= 80 THEN
            SELECT 'B' INTO v_grade FROM dual;
            UPDATE users SET grade = 'B' WHERE id = p_user_id;
        WHEN p_score >= 70 THEN
            SELECT 'C' INTO v_grade FROM dual;
            UPDATE users SET grade = 'C' WHERE id = p_user_id;
        ELSE
            SELECT 'F' INTO v_grade FROM dual;
            UPDATE users SET grade = 'F' WHERE id = p_user_id;
    END CASE;
END;
/

-- ============================================
-- 6. IF with SQL and EXECUTE in branches
-- ============================================
CREATE OR REPLACE PROCEDURE test_if_execute_sql(
    p_dynamic_flag VARCHAR,
    p_table VARCHAR,
    p_id INTEGER
) AS
    v_result VARCHAR(100);
BEGIN
    IF p_dynamic_flag = 'Y' THEN
        EXECUTE IMMEDIATE 'SELECT name FROM ' || p_table || ' WHERE id = ' || p_id INTO v_result;
    ELSE
        SELECT name INTO v_result FROM main_table WHERE id = p_id;
    END IF;
    INSERT INTO result_log (result) VALUES (v_result);
END;
/

-- ============================================
-- 7. IF with SQL referencing cursor variable
-- ============================================
CREATE OR REPLACE PROCEDURE test_if_cursor_sql(
    p_use_cache VARCHAR,
    p_user_id INTEGER,
    p_cursor OUT SYS_REFCURSOR
) AS
    v_sql VARCHAR(1000);
BEGIN
    IF p_use_cache = 'Y' THEN
        OPEN p_cursor FOR SELECT * FROM cache_users WHERE user_id = p_user_id;
    ELSE
        v_sql := 'SELECT * FROM users WHERE user_id = ' || p_user_id;
        OPEN p_cursor FOR EXECUTE v_sql;
    END IF;
END;
/

-- ============================================
-- 8. IF-ELSE with DELETE from different tables
-- ============================================
CREATE OR REPLACE PROCEDURE test_if_delete_tables(
    p_table_type VARCHAR,
    p_id INTEGER
) AS
BEGIN
    IF p_table_type = 'MAIN' THEN
        DELETE FROM main_data WHERE id = p_id;
    ELSIF p_table_type = 'TEMP' THEN
        DELETE FROM temp_data WHERE id = p_id;
    ELSE
        DELETE FROM archive_data WHERE id = p_id;
    END IF;
END;
/
