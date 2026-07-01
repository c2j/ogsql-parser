-- description: standalone INSERT ... SELECT * should warn
-- warn: R001
INSERT INTO t SELECT * FROM s;
