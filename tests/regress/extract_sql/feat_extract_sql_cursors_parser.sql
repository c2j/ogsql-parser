-- Issue: N/A
-- Description: OPEN FOR, RETURN QUERY, FETCH, cursor operations with SQL extraction
-- Expect: parse success

-- ============================================
-- 1. OPEN cursor FOR static SELECT
-- ============================================
CREATE OR REPLACE PROCEDURE test_open_for_select(
    p_dept_id INTEGER,
    p_cursor OUT SYS_REFCURSOR
) AS
BEGIN
    OPEN p_cursor FOR
        SELECT id, name, salary FROM employees WHERE dept_id = p_dept_id;
END;
/

-- ============================================
-- 2. OPEN cursor FOR EXECUTE (dynamic SQL)
-- ============================================
CREATE OR REPLACE PROCEDURE test_open_for_execute(
    p_table VARCHAR,
    p_id INTEGER,
    p_cursor OUT SYS_REFCURSOR
) AS
    v_sql VARCHAR(1000);
BEGIN
    v_sql := 'SELECT * FROM ' || p_table || ' WHERE id = ' || p_id;
    OPEN p_cursor FOR EXECUTE v_sql;
END;
/

-- ============================================
-- 3. OPEN cursor FOR EXECUTE with USING
-- ============================================
CREATE OR REPLACE PROCEDURE test_open_for_execute_using(
    p_id INTEGER,
    p_name VARCHAR,
    p_cursor OUT SYS_REFCURSOR
) AS
    v_sql VARCHAR(1000);
BEGIN
    v_sql := 'SELECT * FROM users WHERE id = $1 AND name = $2';
    OPEN p_cursor FOR EXECUTE v_sql USING p_id, p_name;
END;
/

-- ============================================
-- 4. RETURN QUERY with static SELECT
-- ============================================
CREATE OR REPLACE FUNCTION test_return_query(
    p_dept_id INTEGER
) RETURNS TABLE(id INTEGER, name VARCHAR, salary NUMERIC) AS $$
BEGIN
    RETURN QUERY
        SELECT id, name, salary FROM employees WHERE dept_id = p_dept_id;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- 5. RETURN QUERY EXECUTE
-- ============================================
CREATE OR REPLACE FUNCTION test_return_query_execute(
    p_table VARCHAR,
    p_id INTEGER
) RETURNS TABLE(id INTEGER, name VARCHAR) AS $$
DECLARE
    v_sql TEXT;
BEGIN
    v_sql := 'SELECT id, name FROM ' || p_table || ' WHERE id = ' || p_id;
    RETURN QUERY EXECUTE v_sql;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- 6. FETCH from cursor
-- ============================================
CREATE OR REPLACE PROCEDURE test_fetch_cursor(
    p_cursor_id INTEGER
) AS
    v_id INTEGER;
    v_name VARCHAR(100);
    cur CURSOR FOR SELECT id, name FROM users WHERE status = 'active';
    cur2 SYS_REFCURSOR;
BEGIN
    OPEN cur;
    LOOP
        FETCH cur INTO v_id, v_name;
        EXIT WHEN NOT FOUND;
        INSERT INTO fetch_log (user_id, user_name) VALUES (v_id, v_name);
    END LOOP;
    CLOSE cur;
    OPEN cur2 FOR SELECT id, name FROM users WHERE id = p_cursor_id;
    FETCH cur2 INTO v_id, v_name;
    INSERT INTO single_fetch_log (user_id, user_name) VALUES (v_id, v_name);
    CLOSE cur2;
END;
/

-- ============================================
-- 7. Multiple OUT REFCURSOR parameters
-- ============================================
CREATE OR REPLACE PROCEDURE test_multi_refcursor(
    p_id INTEGER,
    p_cursor1 OUT SYS_REFCURSOR,
    p_cursor2 OUT SYS_REFCURSOR
) AS
BEGIN
    OPEN p_cursor1 FOR SELECT * FROM users WHERE id = p_id;
    OPEN p_cursor2 FOR SELECT * FROM orders WHERE user_id = p_id;
END;
/

-- ============================================
-- 8. OPEN cursor FOR with complex JOIN
-- ============================================
CREATE OR REPLACE PROCEDURE test_open_for_join(
    p_user_id INTEGER,
    p_cursor OUT SYS_REFCURSOR
) AS
BEGIN
    OPEN p_cursor FOR
        SELECT u.id, u.name, o.order_id, o.amount
        FROM users u
        INNER JOIN orders o ON u.id = o.user_id
        WHERE u.id = p_user_id
        ORDER BY o.order_date DESC;
END;
/

-- ============================================
-- 9. Cursor with parameterized query
-- ============================================
CREATE OR REPLACE PROCEDURE test_cursor_with_params(
    p_min_salary NUMERIC,
    p_dept_id INTEGER
) AS
    cur CURSOR(c_salary NUMERIC, c_dept INTEGER) FOR
        SELECT id, name FROM employees
        WHERE salary >= c_salary AND dept_id = c_dept;
    v_id INTEGER;
    v_name VARCHAR;
BEGIN
    OPEN cur(p_min_salary, p_dept_id);
    LOOP
        FETCH cur INTO v_id, v_name;
        EXIT WHEN NOT FOUND;
        INSERT INTO high_salary_log (emp_id, emp_name) VALUES (v_id, v_name);
    END LOOP;
    CLOSE cur;
END;
/

-- ============================================
-- 10. RETURN QUERY with UNION
-- ============================================
CREATE OR REPLACE FUNCTION test_return_query_union(
    p_include_archived BOOLEAN
) RETURNS TABLE(id INTEGER, name VARCHAR) AS $$
BEGIN
    RETURN QUERY
        SELECT id, name FROM active_users;
    IF p_include_archived THEN
        RETURN QUERY
            SELECT id, name FROM archived_users;
    END IF;
END;
$$ LANGUAGE plpgsql;
