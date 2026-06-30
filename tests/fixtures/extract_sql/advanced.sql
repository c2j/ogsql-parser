-- Issue: N/A
-- Description: Advanced extract-sql scenarios: PERFORM, RETURNING, exception handlers, sub-blocks, FORALL
-- Expect: snapshot
-- Command: parse --extract-sql

-- ============================================
-- 1. PERFORM statement
-- ============================================
CREATE OR REPLACE PROCEDURE test_perform_sql(
    p_user_id INTEGER,
    p_amount NUMERIC
) AS
BEGIN
    PERFORM process_payment(p_user_id, p_amount);
    PERFORM update_balance(p_user_id);
    INSERT INTO payment_log (user_id, amount) VALUES (p_user_id, p_amount);
END;
/

-- ============================================
-- 2. INSERT with RETURNING
-- ============================================
CREATE OR REPLACE PROCEDURE test_insert_returning(
    p_name VARCHAR,
    p_email VARCHAR
) AS
    v_new_id INTEGER;
BEGIN
    INSERT INTO users (name, email) VALUES (p_name, p_email)
    RETURNING id INTO v_new_id;
    INSERT INTO welcome_queue (user_id, message) VALUES (v_new_id, 'Welcome');
END;
/

-- ============================================
-- 3. Exception handler with SQL
-- ============================================
CREATE OR REPLACE PROCEDURE test_exception_sql(
    p_id INTEGER,
    p_value VARCHAR
) AS
BEGIN
    INSERT INTO data_table (id, value) VALUES (p_id, p_value);
EXCEPTION
    WHEN UNIQUE_VIOLATION THEN
        UPDATE data_table SET value = p_value WHERE id = p_id;
    WHEN OTHERS THEN
        INSERT INTO error_log (error_id, error_msg, error_time) VALUES (p_id, SQLERRM, CURRENT_TIMESTAMP);
END;
/

-- ============================================
-- 4. Nested sub-blocks with SQL
-- ============================================
CREATE OR REPLACE PROCEDURE test_subblock_sql(
    p_main_id INTEGER,
    p_sub_value VARCHAR
) AS
    v_main_name VARCHAR(100);
    v_sub_count INTEGER;
BEGIN
    SELECT name INTO v_main_name FROM main_table WHERE id = p_main_id;
    BEGIN
        SELECT id INTO v_sub_count FROM sub_table WHERE main_id = p_main_id AND value = p_sub_value;
        UPDATE sub_table SET processed = 'Y' WHERE id = v_sub_count;
        INSERT INTO sub_process_log (main_id, sub_id, processed_by) VALUES (p_main_id, v_sub_count, v_main_name);
    END;
    UPDATE main_table SET last_processed = CURRENT_TIMESTAMP WHERE id = p_main_id;
END;
/

-- ============================================
-- 5. GOTO with SQL in labeled sections
-- ============================================
CREATE OR REPLACE PROCEDURE test_goto_sql(
    p_id INTEGER
) AS
    v_status VARCHAR(20);
BEGIN
    SELECT status INTO v_status FROM tasks WHERE id = p_id;
    IF v_status IS NULL THEN
        GOTO not_found;
    END IF;
    IF v_status = 'DONE' THEN
        GOTO already_done;
    END IF;
    UPDATE tasks SET status = 'PROCESSING' WHERE id = p_id;
    INSERT INTO task_log (task_id, action) VALUES (p_id, 'STARTED');
    RETURN;
    <<not_found>>
    INSERT INTO missing_tasks (task_id) VALUES (p_id);
    RETURN;
    <<already_done>>
    INSERT INTO duplicate_attempts (task_id) VALUES (p_id);
END;
/

-- ============================================
-- 6. FORALL with SQL (bulk operations)
-- ============================================
CREATE OR REPLACE PROCEDURE test_forall_sql AS
    TYPE id_array IS VARRAY(100) OF INTEGER;
    v_ids id_array := id_array(1, 2, 3, 4, 5);
    v_count INTEGER;
BEGIN
    FORALL i IN 1..v_ids.COUNT
        INSERT INTO batch_process (id) VALUES (v_ids(i));
    SELECT COUNT(*) INTO v_count FROM batch_process;
    INSERT INTO batch_summary (total_processed) VALUES (v_count);
END;
/

-- ============================================
-- 7. GET DIAGNOSTICS with SQL
-- ============================================
CREATE OR REPLACE PROCEDURE test_diagnostics_sql(
    p_id INTEGER,
    p_value VARCHAR
) AS
    v_row_count INTEGER;
BEGIN
    INSERT INTO items (id, value) VALUES (p_id, p_value);
    GET DIAGNOSTICS v_row_count = ROW_COUNT;
    IF v_row_count > 0 THEN
        INSERT INTO diag_log (item_id, rows_affected) VALUES (p_id, v_row_count);
    END IF;
END;
/

-- ============================================
-- 8. RAISE with SQL
-- ============================================
CREATE OR REPLACE PROCEDURE test_raise_sql(
    p_id INTEGER,
    p_action VARCHAR
) AS
    v_name VARCHAR(100);
BEGIN
    SELECT name INTO v_name FROM users WHERE id = p_id;
    IF p_action = 'LOG' THEN
        INSERT INTO action_log (user_id, user_name, action) VALUES (p_id, v_name, p_action);
    ELSIF p_action = 'DELETE' THEN
        DELETE FROM users WHERE id = p_id;
        INSERT INTO deletion_log (user_id, user_name) VALUES (p_id, v_name);
    END IF;
END;
/

-- ============================================
-- 9. Cursor with dynamic SQL and FETCH
-- ============================================
CREATE OR REPLACE PROCEDURE test_dynamic_cursor_sql(
    p_where_clause VARCHAR
) AS
    v_sql VARCHAR(4000);
    v_id INTEGER;
    v_data VARCHAR(1000);
    cur SYS_REFCURSOR;
BEGIN
    v_sql := 'SELECT id, data FROM dynamic_data WHERE ' || p_where_clause;
    OPEN cur FOR EXECUTE v_sql;
    LOOP
        FETCH cur INTO v_id, v_data;
        EXIT WHEN cur%NOTFOUND;
        INSERT INTO extracted_data (source_id, source_data) VALUES (v_id, v_data);
    END LOOP;
    CLOSE cur;
END;
/

-- ============================================
-- 10. Savepoint with exception and SQL
-- ============================================
CREATE OR REPLACE PROCEDURE test_savepoint_sql(
    p_from_id INTEGER,
    p_to_id INTEGER,
    p_amount NUMERIC
) AS
    v_balance NUMERIC;
BEGIN
    SELECT balance INTO v_balance FROM accounts WHERE id = p_from_id;
    IF v_balance >= p_amount THEN
        UPDATE accounts SET balance = balance - p_amount WHERE id = p_from_id;
        UPDATE accounts SET balance = balance + p_amount WHERE id = p_to_id;
        INSERT INTO transfers (from_id, to_id, amount) VALUES (p_from_id, p_to_id, p_amount);
    END IF;
EXCEPTION
    WHEN OTHERS THEN
        INSERT INTO transfer_errors (from_id, to_id, error) VALUES (p_from_id, p_to_id, SQLERRM);
END;
/
