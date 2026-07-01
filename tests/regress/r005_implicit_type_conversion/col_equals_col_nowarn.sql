-- description: column = column (no literal involved) should NOT trigger R005
-- schema: t.a=integer, t.b=integer
-- nowarn: R005
SELECT * FROM t WHERE a = b;
