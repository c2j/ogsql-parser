-- description: Some ORDER BY items missing from SELECT with GROUP BY
-- warn: R012
SELECT a, b FROM t GROUP BY a, b ORDER BY a, c;
