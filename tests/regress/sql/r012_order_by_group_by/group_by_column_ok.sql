-- description: ORDER BY references a column in GROUP BY but not in SELECT
-- nowarn: R012
SELECT a FROM t1 GROUP BY a, b ORDER BY b
