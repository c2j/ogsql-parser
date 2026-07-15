-- description: issue #292 WITH CTE before UPDATE must survive format
-- format-contains: with low_earners as
-- format-contains: update
WITH low_earners AS (SELECT emp_id FROM employees WHERE salary < 5000)
UPDATE employees SET salary = salary * 1.1 WHERE emp_id IN (SELECT emp_id FROM low_earners);
