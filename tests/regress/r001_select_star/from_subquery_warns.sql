-- description: SELECT * in FROM subquery should warn
-- warn: R001
SELECT id FROM (SELECT * FROM t1) sub;
