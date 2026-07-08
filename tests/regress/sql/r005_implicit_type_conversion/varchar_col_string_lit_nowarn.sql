-- description: varchar column vs string literal is same-family, should NOT trigger R005
-- schema: t.name=varchar(100)
-- nowarn: R005
SELECT * FROM t WHERE name = 'abc';
