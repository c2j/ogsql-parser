-- description: issue #292 control: WITH CTE before SELECT is preserved by format
-- format-contains: with active as
-- format-contains: select
WITH active AS (SELECT emp_id FROM employees WHERE salary > 8000)
SELECT * FROM active;
