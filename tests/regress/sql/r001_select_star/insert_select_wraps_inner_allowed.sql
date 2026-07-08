-- description: INSERT with outer * wrapping explicit inner should NOT warn
-- nowarn: R001
INSERT INTO t SELECT * FROM (SELECT id, name FROM s) sub;
