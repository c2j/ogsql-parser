-- Issue: N/A
-- Description: FOR, WHILE, LOOP, FOREACH with SQL inside loop bodies
-- Expect: parse success

-- ============================================
-- 1. FOR loop with static SQL cursor
-- ============================================
CREATE OR REPLACE PROCEDURE test_for_cursor_sql(
    p_dept_id INTEGER
) AS
BEGIN
    FOR rec IN (
        SELECT id, name, salary FROM employees WHERE dept_id = p_dept_id
    ) LOOP
        INSERT INTO report (emp_id, emp_name, emp_salary)
        VALUES (rec.id, rec.name, rec.salary);
    END LOOP;
END;
/

-- ============================================
-- 2. FOR loop with EXECUTE (dynamic cursor)
-- ============================================
CREATE OR REPLACE PROCEDURE test_for_execute_cursor(
    p_table VARCHAR,
    p_condition VARCHAR
) AS
    v_sql VARCHAR(4000);
    v_count INTEGER := 0;
BEGIN
    v_sql := 'SELECT id, name FROM ' || p_table || ' WHERE ' || p_condition;
    FOR rec IN EXECUTE v_sql LOOP
        v_count := v_count + 1;
        INSERT INTO process_log (table_name, record_id, record_name)
        VALUES (p_table, rec.id, rec.name);
    END LOOP;
    UPDATE stats SET processed_count = v_count WHERE table_name = p_table;
END;
/

-- ============================================
-- 3. WHILE loop with SQL
-- ============================================
CREATE OR REPLACE PROCEDURE test_while_sql(
    p_batch_size INTEGER
) AS
    v_processed INTEGER := 0;
    v_total INTEGER;
    v_batch_id INTEGER := 0;
BEGIN
    SELECT COUNT(*) INTO v_total FROM pending_tasks;
    WHILE v_processed < v_total LOOP
        INSERT INTO batch_log (batch_id, start_row, batch_size)
        VALUES (v_batch_id, v_processed, p_batch_size);
        UPDATE pending_tasks SET status = 'PROCESSING' WHERE id > 0 AND status = 'PENDING';
        v_processed := v_processed + p_batch_size;
        v_batch_id := v_batch_id + 1;
    END LOOP;
    INSERT INTO completion_log (total_processed) VALUES (v_processed);
END;
/

-- ============================================
-- 4. Simple LOOP with SQL and EXIT condition
-- ============================================
CREATE OR REPLACE PROCEDURE test_loop_sql(
    p_max_iterations INTEGER
) AS
    v_counter INTEGER := 0;
    v_exists INTEGER;
BEGIN
    LOOP
        v_counter := v_counter + 1;
        SELECT COUNT(*) INTO v_exists FROM queue WHERE status = 'PENDING';
        IF v_exists = 0 THEN
            EXIT;
        END IF;
        UPDATE queue SET status = 'PROCESSING'
        WHERE id = (SELECT id FROM queue WHERE status = 'PENDING' LIMIT 1);
        INSERT INTO loop_log (iteration, remaining) VALUES (v_counter, v_exists - 1);
        IF v_counter >= p_max_iterations THEN
            EXIT;
        END IF;
    END LOOP;
END;
/

-- ============================================
-- 5. FOR loop with named cursor
-- ============================================
CREATE OR REPLACE PROCEDURE test_for_named_cursor(
    p_cursor_name VARCHAR,
    p_value INTEGER
) AS
    v_id INTEGER;
    v_name VARCHAR(100);
    cur CURSOR FOR SELECT id, name FROM items WHERE value > p_value;
BEGIN
    FOR rec IN cur LOOP
        INSERT INTO selected_items (item_id, item_name) VALUES (rec.id, rec.name);
    END LOOP;
END;
/

-- ============================================
-- 6. Nested loops with SQL
-- ============================================
CREATE OR REPLACE PROCEDURE test_nested_loops(
    p_dept_id INTEGER
) AS
    v_dept_name VARCHAR(100);
BEGIN
    FOR dept IN (SELECT id, name FROM departments WHERE id = p_dept_id) LOOP
        UPDATE dept_summary SET last_scan = CURRENT_TIMESTAMP WHERE dept_id = dept.id;
        FOR emp IN (SELECT id, name, salary FROM employees WHERE dept_id = dept.id) LOOP
            INSERT INTO salary_audit (emp_id, dept_id, emp_name, salary)
            VALUES (emp.id, dept.id, emp.name, emp.salary);
        END LOOP;
        INSERT INTO dept_scan_log (dept_id, dept_name, scan_time)
        VALUES (dept.id, dept.name, CURRENT_TIMESTAMP);
    END LOOP;
END;
/

-- ============================================
-- 7. FOREACH loop
-- ============================================
CREATE OR REPLACE PROCEDURE test_foreach_sql(
    p_ids INTEGER[]
) AS
    v_id INTEGER;
    v_name VARCHAR(100);
BEGIN
    FOREACH v_id IN ARRAY p_ids LOOP
        SELECT name INTO v_name FROM users WHERE id = v_id;
        INSERT INTO name_list (user_id, user_name) VALUES (v_id, v_name);
    END LOOP;
END;
/

-- ============================================
-- 8. FOR loop with labeled EXIT/CONTINUE
-- ============================================
CREATE OR REPLACE PROCEDURE test_labeled_loop_sql(
    p_skip_ids INTEGER[]
) AS
    v_id INTEGER;
BEGIN
    <<outer_loop>>
    FOR rec IN (SELECT id, name, type FROM products) LOOP
        IF rec.type = 'DISABLED' THEN
            CONTINUE outer_loop;
        END IF;
        INSERT INTO active_products (product_id, product_name)
        VALUES (rec.id, rec.name);
    END LOOP;
END;
/

-- ============================================
-- 9. FOR loop with INSERT SELECT
-- ============================================
CREATE OR REPLACE PROCEDURE test_for_insert_select(
    p_source_dept INTEGER,
    p_target_dept INTEGER
) AS
BEGIN
    FOR emp IN (
        SELECT id, name, salary FROM employees
        WHERE dept_id = p_source_dept AND status = 'active'
    ) LOOP
        INSERT INTO employees (id, name, salary, dept_id)
        VALUES (emp.id + 10000, emp.name, emp.salary, p_target_dept);
    END LOOP;
END;
/

-- ============================================
-- 10. FOR loop with LIMIT and OFFSET in cursor
-- ============================================
CREATE OR REPLACE PROCEDURE test_for_limited_cursor(
    p_limit INTEGER,
    p_offset INTEGER
) AS
    v_count INTEGER := 0;
BEGIN
    FOR rec IN (
        SELECT id, data FROM large_table
        ORDER BY id
        LIMIT p_limit OFFSET p_offset
    ) LOOP
        v_count := v_count + 1;
        INSERT INTO batch_results (original_id, data, batch_seq)
        VALUES (rec.id, rec.data, v_count);
    END LOOP;
END;
/
