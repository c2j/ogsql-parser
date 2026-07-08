-- description: SELECT * in WHERE subquery should warn
-- warn: R001
SELECT id FROM t1 WHERE id IN (SELECT * FROM t2);
