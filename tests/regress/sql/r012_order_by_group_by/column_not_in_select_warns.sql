-- description: ORDER BY column is NOT in SELECT list with GROUP BY
-- warn: R012
SELECT a FROM t GROUP BY a ORDER BY b;
