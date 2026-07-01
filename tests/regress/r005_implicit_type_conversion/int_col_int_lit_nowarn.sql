-- description: integer column vs integer literal is same-family, should NOT trigger R005
-- schema: t.age=integer
-- nowarn: R005
SELECT * FROM t WHERE age = 30;
