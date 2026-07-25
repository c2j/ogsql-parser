-- 来源: 3101_VACUUM.txt
-- SQL 数量: 8

CREATE SCHEMA tpcds;

--创建表tpcds.reason。
CREATE TABLE tpcds.reason ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--向表中插入多条记录。
INSERT INTO tpcds.reason values(1,'AAAAAAAABAAAAAAA','reason 1'),(2,'AAAAAAAABAAAAAAA','reason 2');

--在表 tpcds. reason上创建索引。
CREATE UNIQUE INDEX ds_reason_index1 ON tpcds. reason(r_reason_sk);

--对带索引的表 tpcds. reason执行VACUUM操作。
VACUUM (VERBOSE, ANALYZE) tpcds. reason;

--删除索引。
DROP INDEX tpcds.ds_reason_index1 CASCADE;

DROP TABLE tpcds. reason;

DROP SCHEMA tpcds CASCADE;

