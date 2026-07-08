-- description: UNION branches each have standalone SELECT * — should warn
-- warn: R001
-- warn: P001
SELECT * FROM t1 UNION SELECT * FROM t2;
