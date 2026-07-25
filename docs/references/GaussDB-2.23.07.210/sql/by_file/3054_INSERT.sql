-- 来源: 3054_INSERT.txt
-- SQL 数量: 14

CREATE SCHEMA tpcds;

--创建表tpcds.reason。
CREATE TABLE tpcds.reason ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--向表中插入一条记录。
INSERT INTO tpcds.reason(r_reason_sk, r_reason_id, r_reason_desc) VALUES (0, 'AAAAAAAAAAAAAAAA', 'reason0');

--创建表 tpcds. reason_t2。
CREATE TABLE tpcds. reason_t2 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--向表中插入一条记录。
INSERT INTO tpcds. reason_t2(r_reason_sk, r_reason_id, r_reason_desc) VALUES (1, 'AAAAAAAABAAAAAAA', 'reason1');

--向表中插入一条记录，和上一条语法等效。
INSERT INTO tpcds. reason_t2 VALUES (2, 'AAAAAAAABAAAAAAA', 'reason2');

--向表中插入多条记录。
INSERT INTO tpcds. reason_t2 VALUES (3, 'AAAAAAAACAAAAAAA','reason3'),(4, 'AAAAAAAADAAAAAAA', 'reason4'),(5, 'AAAAAAAAEAAAAAAA','reason5');

--向表中插入 tpcds. reason中r_reason_sk小于5的记录。
INSERT INTO tpcds. reason_t2 SELECT * FROM tpcds. reason WHERE r_reason_sk <5;

--对表创建唯一索引。
CREATE UNIQUE INDEX reason_t2_u_index ON tpcds. reason_t2(r_reason_sk);

--向表中插入多条记录，如果冲突则更新冲突数据行中r_reason_id字段为'BBBBBBBBCAAAAAAA'。
INSERT INTO tpcds. reason_t2 VALUES (5, 'BBBBBBBBCAAAAAAA','reason5'),(6, 'AAAAAAAADAAAAAAA', 'reason6') ON DUPLICATE KEY UPDATE r_reason_id = 'BBBBBBBBCAAAAAAA';

--更新已有记录并返回
INSERT INTO tpcds. reason_t2 VALUES ( 5, 'BBBBBBBBCAAAAAAA','reason5') ON DUPLICATE KEY UPDATE r_reason_desc='reason5_new' RETURNING *;

--删除表 tpcds. reason_t2。
DROP TABLE tpcds. reason_t2;

--删除表tpcds.reason。
DROP TABLE tpcds.reason;

--删除SCHEMA。
DROP SCHEMA tpcds CASCADE;

