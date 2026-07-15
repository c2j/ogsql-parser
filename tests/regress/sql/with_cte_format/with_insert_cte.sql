-- description: issue #292 control: WITH CTE before INSERT is preserved by format
-- format-contains: with targets as
-- format-contains: insert
WITH targets AS (SELECT emp_id FROM employees)
INSERT INTO archive SELECT * FROM targets;
