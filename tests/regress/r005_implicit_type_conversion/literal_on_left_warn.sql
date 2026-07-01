-- description: literal on left, integer column on right — cross-family, should trigger R005
-- schema: t.id=integer
-- warn: R005
SELECT * FROM t WHERE 'abc' = id;
