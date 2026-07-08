-- description: Multiple ORDER BY items all in SELECT list with GROUP BY
-- nowarn: R012
SELECT a, b, c FROM t GROUP BY a, b, c ORDER BY a, b DESC, c;
