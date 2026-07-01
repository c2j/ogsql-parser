-- Issue: N/A
-- Description: Package spec cursor SQL extraction — cursors defined in PACKAGE spec with SELECT
-- Expect: snapshot
-- Command: parse --extract-sql

-- ============================================
-- Package spec with cursor declarations
-- ============================================
CREATE OR REPLACE PACKAGE pkg_cursor_sql AS
    -- Simple cursor with basic SELECT
    CURSOR cur_active_users IS
        SELECT id, name, email FROM users WHERE status = 'active';

    -- Cursor with JOIN
    CURSOR cur_user_orders(p_user_id INTEGER) IS
        SELECT u.name, o.order_id, o.amount
        FROM users u
        INNER JOIN orders o ON u.id = o.user_id
        WHERE u.id = p_user_id;

    -- Cursor with parameterized WHERE and ORDER BY
    CURSOR cur_dept_employees(p_dept_id INTEGER) IS
        SELECT id, name, salary
        FROM employees
        WHERE dept_id = p_dept_id
        ORDER BY salary DESC;

    -- Cursor with subquery
    CURSOR cur_high_value_users IS
        SELECT id, name FROM users
        WHERE id IN (
            SELECT user_id FROM orders
            WHERE amount > 1000
            GROUP BY user_id
        );

    -- Cursor with CTE (WITH clause)
    CURSOR cur_dept_summary IS
        WITH dept_stats AS (
            SELECT dept_id, COUNT(*) AS emp_count, AVG(salary) AS avg_sal
            FROM employees GROUP BY dept_id
        )
        SELECT d.name, ds.emp_count, ds.avg_sal
        FROM departments d
        JOIN dept_stats ds ON d.id = ds.dept_id;

    PROCEDURE get_active_users(p_cursor OUT SYS_REFCURSOR);
    FUNCTION count_orders(p_user_id INTEGER) RETURN INTEGER;
END pkg_cursor_sql;
/

-- ============================================
-- Package body (minimal, just for reference)
-- ============================================
CREATE OR REPLACE PACKAGE BODY pkg_cursor_sql AS
    PROCEDURE get_active_users(p_cursor OUT SYS_REFCURSOR) AS
    BEGIN
        OPEN p_cursor FOR SELECT id, name, email FROM users WHERE status = 'active';
    END get_active_users;

    FUNCTION count_orders(p_user_id INTEGER) RETURN INTEGER AS
        v_count INTEGER;
    BEGIN
        SELECT COUNT(*) INTO v_count FROM orders WHERE user_id = p_user_id;
        RETURN v_count;
    END count_orders;
END pkg_cursor_sql;
/
