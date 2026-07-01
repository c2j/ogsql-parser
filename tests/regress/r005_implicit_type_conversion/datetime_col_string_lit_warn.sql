-- description: timestamp column vs string literal is cross-family, should trigger R005
-- schema: t.created_at=timestamp
-- warn: R005
SELECT * FROM t WHERE created_at = '2024-01-01';
