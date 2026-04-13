-- 来源: 4511_file_4511.txt
-- SQL 数量: 13

SHOW enable_tde;

show tde_key_info;

CREATE TABLE t1 (c1 INT, c2 TEXT) WITH (enable_tde = on);

CREATE TABLE t2 (c1 INT, c2 TEXT) WITH (enable_tde = on, encrypt_algo = 'SM4_CTR');

SELECT relname,reloptions FROM pg_class WHERE relname = 't1';

INSERT INTO t1 VALUES (1, 'tde plain 123');

SELECT * FROM t1;

ALTER TABLE t1 ENCRYPTION KEY ROTATION;

ALTER TABLE t1 SET (enable_tde = off);

VACUUM FULL t1;

ALTER TABLE t1 SET (enable_tde = on);

VACUUM FULL t1;

DROP TABLE t1;

