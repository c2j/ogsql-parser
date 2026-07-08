-- description: procedure with varchar2 variable assigned via complex CASE WHEN expression with subquery-style conditions
CREATE OR REPLACE PROCEDURE test_complex_case(
    p_dept_id IN INTEGER,
    p_mode    IN VARCHAR2
) AS
    v_action VARCHAR2(4000) := CASE
        WHEN p_dept_id IS NULL AND p_mode = 'VIEW' THEN 'SELECT * FROM employees'
        WHEN p_dept_id IS NOT NULL AND p_mode = 'VIEW' THEN 'SELECT * FROM employees WHERE dept_id = ' || p_dept_id
        WHEN p_mode = 'UPDATE' THEN 'UPDATE employees SET updated_at = SYSDATE'
        WHEN p_mode IN ('DELETE', 'PURGE') THEN 'DELETE FROM employees WHERE dept_id = ' || p_dept_id
        WHEN p_dept_id BETWEEN 1 AND 100 AND p_mode = 'REPORT' THEN 'EXEC report_by_dept(' || p_dept_id || ')'
        ELSE 'INVALID_OPERATION'
    END;
BEGIN
    RAISE NOTICE 'Action: %', v_action;
END;
