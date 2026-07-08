-- description: DELETE WHERE integer col vs string literal is cross-family, should trigger R005
-- schema: t.id=integer
-- warn: R005
DELETE FROM t WHERE id = 'abc';
