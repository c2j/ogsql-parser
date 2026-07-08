-- description: ORDER BY without DISTINCT should NOT trigger R011
-- nowarn: R011
SELECT a FROM t ORDER BY b;
