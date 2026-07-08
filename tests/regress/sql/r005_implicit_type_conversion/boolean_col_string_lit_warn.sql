-- description: boolean column vs string literal is cross-family, should trigger R005
-- schema: t.active=boolean
-- warn: R005
SELECT * FROM t WHERE active = 'true';
