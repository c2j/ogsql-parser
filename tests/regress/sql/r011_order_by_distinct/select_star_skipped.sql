-- description: SELECT * with DISTINCT should skip R011 (no false positive)
-- nowarn: R011
SELECT DISTINCT * FROM t ORDER BY a;
