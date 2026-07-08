-- description: complex WHERE with AND, one cross-family (age=string) triggers R005
-- schema: t.name=varchar(100), t.age=integer
-- warn: R005
SELECT * FROM t WHERE name = 'abc' AND age = '30';
