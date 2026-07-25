-- 来源: 3097_TRUNCATE.txt
-- SQL 数量: 14

CREATE SCHEMA tpcds;

--创建表tpcds.reason。
CREATE TABLE tpcds.reason ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--向表中插入多条记录。
INSERT INTO tpcds.reason values(1,'AAAAAAAABAAAAAAA','reason 1'),(5,'AAAAAAAABAAAAAAA','reason 2'),(15,'AAAAAAAABAAAAAAA','reason 3'),(25,'AAAAAAAABAAAAAAA','reason 4'),(35,'AAAAAAAABAAAAAAA','reason 5'),(45,'AAAAAAAACAAAAAAA','reason 6'),(55,'AAAAAAAACAAAAAAA','reason 7');

--创建表。
CREATE TABLE tpcds. reason_t1 AS TABLE tpcds. reason;

--清空表 tpcds. reason_t1。
TRUNCATE TABLE tpcds. reason_t1;

--删除表。
DROP TABLE tpcds. reason_t1;

CREATE TABLE tpcds. reason_p ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) )PARTITION BY RANGE (r_reason_sk) ( partition p_05_before values less than (05), partition p_15 values less than (15), partition p_25 values less than (25), partition p_35 values less than (35), partition p_45_after values less than (MAXVALUE) );

--插入数据。
INSERT INTO tpcds. reason_p SELECT * FROM tpcds. reason;

--清空分区p_05_before。
ALTER TABLE tpcds. reason_p TRUNCATE PARTITION p_05_before;

--清空分区p_15。
ALTER TABLE tpcds. reason_p TRUNCATE PARTITION for (15);

--清空分区表。
TRUNCATE TABLE tpcds. reason_p;

--删除表。
DROP TABLE tpcds. reason_p;

--删除表。
DROP TABLE tpcds.reason;

--删除SCHEMA。
DROP SCHEMA tpcds CASCADE;

