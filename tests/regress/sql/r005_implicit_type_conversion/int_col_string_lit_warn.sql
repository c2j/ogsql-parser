-- description: integer column vs string literal is cross-family, should trigger R005
-- schema: t.age=integer
-- warn: R005
SELECT * FROM t WHERE age = '30';
