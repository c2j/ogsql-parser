-- description: ORDER BY positional reference out of range with DISTINCT
-- warn: R011
SELECT DISTINCT a, b FROM t ORDER BY 3;
