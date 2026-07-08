-- description: outer SELECT * with explicit inner query should NOT warn
-- nowarn: R001
SELECT * FROM (SELECT id, name FROM t1) sub;
