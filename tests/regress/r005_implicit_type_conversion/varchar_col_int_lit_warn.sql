-- description: varchar column vs integer literal is cross-family, should trigger R005
-- schema: t.name=varchar(100)
-- warn: R005
SELECT * FROM t WHERE name = 123;
