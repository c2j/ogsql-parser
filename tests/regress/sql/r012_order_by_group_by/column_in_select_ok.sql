-- description: ORDER BY column is in SELECT list with GROUP BY
-- nowarn: R012
SELECT a, b FROM t GROUP BY a, b ORDER BY a;
