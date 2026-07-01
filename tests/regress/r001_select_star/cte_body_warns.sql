-- description: SELECT * in CTE body should warn
-- warn: R001
WITH cte AS (SELECT * FROM t1) SELECT id FROM cte;
