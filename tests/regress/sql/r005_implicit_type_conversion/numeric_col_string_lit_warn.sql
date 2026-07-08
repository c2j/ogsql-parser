-- description: numeric column vs string literal is cross-family, should trigger R005
-- schema: t.price=numeric(10,2)
-- warn: R005
SELECT * FROM t WHERE price = '99.99';
