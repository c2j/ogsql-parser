-- description: SELECT * with GROUP BY should skip R012 (no false positive)
-- nowarn: R012
SELECT * FROM t GROUP BY a ORDER BY b;
