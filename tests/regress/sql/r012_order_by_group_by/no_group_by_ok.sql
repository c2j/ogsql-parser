-- description: ORDER BY without GROUP BY should NOT trigger R012
-- nowarn: R012
SELECT a FROM t ORDER BY a;
