-- description: ORDER BY positional reference is valid with GROUP BY
-- nowarn: R012
SELECT a, b FROM t GROUP BY a, b ORDER BY 2;
