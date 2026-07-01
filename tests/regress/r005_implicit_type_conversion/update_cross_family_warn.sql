-- description: UPDATE WHERE integer col vs string literal is cross-family, should trigger R005
-- schema: t.id=integer
-- warn: R005
UPDATE t SET x = 1 WHERE id = 'abc';
