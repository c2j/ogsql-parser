-- description: ORDER BY column is NOT in SELECT list with DISTINCT
-- warn: R011
SELECT DISTINCT a FROM t ORDER BY b;
