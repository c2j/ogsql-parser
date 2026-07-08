-- description: deeply nested SELECT * — some inner * should still warn even when outer wraps
-- split: semicolon
-- warn: R001
SELECT * FROM (SELECT * FROM (SELECT id, name FROM t1) a) b;
SELECT * FROM (SELECT id FROM t1) sub;
SELECT id FROM (SELECT * FROM t1) sub;
