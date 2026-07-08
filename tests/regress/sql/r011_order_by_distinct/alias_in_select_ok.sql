-- description: ORDER BY alias matches SELECT alias with DISTINCT
-- nowarn: R011
SELECT DISTINCT a AS x FROM t ORDER BY x;
