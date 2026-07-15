-- description: issue #292 WITH CTE before DELETE must survive format
-- format-contains: with to_delete as
-- format-contains: delete
WITH to_delete AS (SELECT emp_id FROM employees WHERE salary > 8000)
DELETE FROM emp_performance WHERE emp_id IN (SELECT emp_id FROM to_delete);
