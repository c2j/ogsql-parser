-- description: GROUP BY without ORDER BY should NOT trigger R012
-- nowarn: R012
SELECT a FROM t GROUP BY a;
