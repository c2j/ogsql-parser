-- Issue: N/A
-- Description: Package body with multiple procedures containing SQL extraction
-- Expect: snapshot
-- Command: parse --extract-sql

CREATE OR REPLACE PACKAGE BODY pkg_user_mgmt AS
    PROCEDURE create_user(
        p_name VARCHAR,
        p_email VARCHAR,
        p_dept_id INTEGER
    ) AS
        v_user_id INTEGER;
    BEGIN
        INSERT INTO users (name, email, dept_id) VALUES (p_name, p_email, p_dept_id);
        INSERT INTO user_audit (user_id, action, action_time) VALUES (v_user_id, 'CREATE', CURRENT_TIMESTAMP);
    END create_user;
    PROCEDURE update_user_status(
        p_user_id INTEGER,
        p_status VARCHAR
    ) AS
        v_old_status VARCHAR(20);
    BEGIN
        SELECT status INTO v_old_status FROM users WHERE id = p_user_id;
        UPDATE users SET status = p_status WHERE id = p_user_id;
        INSERT INTO status_change_log (user_id, old_status, new_status) VALUES (p_user_id, v_old_status, p_status);
    END update_user_status;
    PROCEDURE get_users_by_dept(
        p_dept_id INTEGER,
        p_cursor OUT SYS_REFCURSOR
    ) AS
    BEGIN
        OPEN p_cursor FOR SELECT id, name, email, status FROM users WHERE dept_id = p_dept_id ORDER BY name;
    END get_users_by_dept;
    PROCEDURE bulk_archive_users(
        p_days_inactive INTEGER
    ) AS
        v_count INTEGER;
        v_sql VARCHAR(4000);
    BEGIN
        SELECT COUNT(*) INTO v_count FROM users WHERE status = 'active';
        v_sql := 'INSERT INTO users_archive SELECT * FROM users WHERE status = ''inactive''';
        EXECUTE IMMEDIATE v_sql;
        DELETE FROM users WHERE status = 'inactive';
        INSERT INTO archive_log (count_archived) VALUES (v_count);
    END bulk_archive_users;
    FUNCTION count_active_users(
        p_dept_id INTEGER DEFAULT NULL
    ) RETURN INTEGER AS
        v_count INTEGER;
    BEGIN
        IF p_dept_id IS NULL THEN
            SELECT COUNT(*) INTO v_count FROM users WHERE status = 'active';
        ELSE
            SELECT COUNT(*) INTO v_count FROM users WHERE status = 'active' AND dept_id = p_dept_id;
        END IF;
        RETURN v_count;
    END count_active_users;
END pkg_user_mgmt;
/
