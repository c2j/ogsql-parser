-- description: SELECT without WHERE clause should NOT trigger R005
-- schema: t.id=integer
-- nowarn: R005
SELECT * FROM t;
