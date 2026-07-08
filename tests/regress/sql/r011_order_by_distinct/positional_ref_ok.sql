-- description: ORDER BY positional reference is valid with DISTINCT
-- nowarn: R011
SELECT DISTINCT a, b FROM t ORDER BY 1;
