-- description: ORDER BY alias matches SELECT alias with GROUP BY
-- nowarn: R012
SELECT a AS x, b FROM t GROUP BY a, b ORDER BY x;
