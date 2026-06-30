-- Issue: N/A
-- Description: Package body procedures referencing package-level variables (defined in spec/body top-level)
-- Expect: snapshot; package-level vars currently NOT replaced as __SQL_PARAM__ placeholders
-- Command: parse --extract-sql

-- ============================================
-- Package spec: defines package-level variables, cursor, type
-- ============================================
CREATE OR REPLACE PACKAGE pkg_config AS
    g_org_id INTEGER := 0;
    g_app_name VARCHAR(100) := 'DEFAULT';
    g_batch_size INTEGER := 1000;
    g_debug_flag VARCHAR(1) := 'N';
    CURSOR cur_active_users IS SELECT id, name FROM users WHERE status = 'active';
    TYPE user_rec IS RECORD (id INTEGER, name VARCHAR(100), dept_id INTEGER);
    PROCEDURE sync_user(p_user_id INTEGER);
    PROCEDURE batch_audit(p_dept_id INTEGER);
    FUNCTION get_active_count RETURN INTEGER;
END pkg_config;
/

-- ============================================
-- Package body: procedures reference package-level variables in SQL
-- ============================================
CREATE OR REPLACE PACKAGE BODY pkg_config AS

    PROCEDURE sync_user(p_user_id INTEGER) AS
        v_name VARCHAR(100);
    BEGIN
        SELECT name INTO v_name FROM users
        WHERE id = p_user_id AND org_id = g_org_id;
        IF g_debug_flag = 'Y' THEN
            INSERT INTO debug_log (user_id, user_name, batch_size)
            VALUES (p_user_id, v_name, g_batch_size);
        END IF;
    END sync_user;
    PROCEDURE batch_audit(p_dept_id INTEGER) AS
        v_count INTEGER;
        v_sql VARCHAR(4000);
    BEGIN
        SELECT COUNT(*) INTO v_count FROM employees WHERE dept_id = p_dept_id;
        v_sql := 'UPDATE employees SET last_audit = CURRENT_TIMESTAMP WHERE org_id = ' || g_org_id;
        EXECUTE IMMEDIATE v_sql;
        INSERT INTO audit_summary (dept_id, emp_count, app_name, org_id)
        VALUES (p_dept_id, v_count, g_app_name, g_org_id);
    END batch_audit;
    FUNCTION get_active_count RETURN INTEGER AS
        v_total INTEGER;
    BEGIN
        SELECT COUNT(*) INTO v_total FROM users
        WHERE status = 'active' AND org_id = g_org_id;
        RETURN v_total;
    END get_active_count;

END pkg_config;
/
