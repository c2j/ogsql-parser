-- description: DISTINCT without ORDER BY should NOT trigger R011
-- nowarn: R011
SELECT DISTINCT a FROM t;
