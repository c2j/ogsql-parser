-- Issue: N/A
-- Description: Basic extract-sql scenarios: procedures with SELECT/INSERT/UPDATE/DELETE, variable replacement, INTO clauses
-- Expect: parse success

-- ============================================
-- 1. Simple SELECT in procedure
-- ============================================
CREATE OR REPLACE PROCEDURE test_select_proc(
    p_id IN INTEGER,
    p_name IN VARCHAR
) AS
BEGIN
    SELECT * FROM users WHERE id = p_id AND name = p_name;
END;
/

-- ============================================
-- 2. INSERT in procedure
-- ============================================
CREATE OR REPLACE PROCEDURE test_insert_proc(
    p_name VARCHAR,
    p_age INTEGER
) AS
BEGIN
    INSERT INTO users (name, age) VALUES (p_name, p_age);
END;
/

-- ============================================
-- 3. UPDATE in procedure
-- ============================================
CREATE OR REPLACE PROCEDURE test_update_proc(
    p_id INTEGER,
    p_status VARCHAR
) AS
BEGIN
    UPDATE users SET status = p_status WHERE id = p_id;
END;
/

-- ============================================
-- 4. DELETE in procedure
-- ============================================
CREATE OR REPLACE PROCEDURE test_delete_proc(
    p_id INTEGER
) AS
BEGIN
    DELETE FROM users WHERE id = p_id;
END;
/

-- ============================================
-- 5. SELECT INTO with local variable
-- ============================================
CREATE OR REPLACE PROCEDURE test_select_into_proc(
    p_id INTEGER
) AS
    v_name VARCHAR(100);
    v_count INTEGER;
BEGIN
    SELECT name INTO v_name FROM users WHERE id = p_id;
    SELECT COUNT(*) INTO v_count FROM orders WHERE user_id = p_id;
END;
/

-- ============================================
-- 6. Multiple SQL statements
-- ============================================
CREATE OR REPLACE PROCEDURE test_multiple_stmts(
    p_account_id INTEGER
) AS
    v_frozen VARCHAR(1);
BEGIN
    SELECT frozen_flag INTO v_frozen FROM accounts WHERE account_id = p_account_id;
    IF v_frozen = 'Y' THEN
        UPDATE accounts SET frozen_flag = 'N' WHERE account_id = p_account_id;
    END IF;
    INSERT INTO audit_log (account_id, action) VALUES (p_account_id, 'UNFREEZE');
END;
/

-- ============================================
-- 7. Different parameter modes
-- ============================================
CREATE OR REPLACE PROCEDURE test_param_modes(
    p_in IN INTEGER,
    p_out OUT VARCHAR,
    p_inout INOUT INTEGER
) AS
BEGIN
    SELECT name INTO p_out FROM users WHERE id = p_in;
    p_inout := p_inout * 2;
    INSERT INTO logs (msg) VALUES ('processed ' || p_in);
END;
/

-- ============================================
-- 8. Procedure with no parameters
-- ============================================
CREATE OR REPLACE PROCEDURE test_no_params() AS
    v_count INTEGER;
BEGIN
    SELECT COUNT(*) INTO v_count FROM users;
    INSERT INTO stats (total_users) VALUES (v_count);
END;
/

-- ============================================
-- 9. Variable substitution with underscore names
-- ============================================
CREATE OR REPLACE PROCEDURE test_underscore_vars(
    p_user_id INTEGER,
    p_created_by VARCHAR
) AS
    v_total_count INTEGER;
    v_last_login TIMESTAMP;
BEGIN
    SELECT COUNT(*) INTO v_total_count FROM users WHERE created_by = p_created_by;
    SELECT login_time INTO v_last_login FROM sessions WHERE user_id = p_user_id;
    UPDATE users SET last_login = v_last_login WHERE id = p_user_id;
END;
/

-- ============================================
-- 10. MERGE statement in procedure
-- ============================================
CREATE OR REPLACE PROCEDURE test_merge_proc(
    p_id INTEGER,
    p_name VARCHAR,
    p_age INTEGER
) AS
BEGIN
    MERGE INTO users t USING (SELECT p_id AS id, p_name AS name, p_age AS age) s
    ON (t.id = s.id)
    WHEN MATCHED THEN UPDATE SET t.name = s.name, t.age = s.age
    WHEN NOT MATCHED THEN INSERT (id, name, age) VALUES (s.id, s.name, s.age);
END;
/

-- ============================================
-- 11. WITH (CTE) in procedure
-- ============================================
CREATE OR REPLACE PROCEDURE test_cte_proc(
    p_dept_id INTEGER
) AS
    v_count INTEGER;
BEGIN
    WITH dept_users AS (
        SELECT id, name FROM users WHERE dept_id = p_dept_id
    )
    SELECT COUNT(*) INTO v_count FROM dept_users;
END;
/

-- ============================================
-- 12. Anonymous block (BEGIN ... END)
-- ============================================
DO $$
DECLARE
    v_count INTEGER;
    v_msg VARCHAR := 'hello';
BEGIN
    SELECT COUNT(*) INTO v_count FROM users;
    INSERT INTO logs (msg, cnt) VALUES (v_msg, v_count);
END;
$$;
