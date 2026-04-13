-- 来源: 3083_SELECT INTO.txt
-- SQL 数量: 7

CREATE SCHEMA tpcds;

--创建表tpcds.reason。
CREATE TABLE tpcds.reason ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--向表中插入多条记录。
INSERT INTO tpcds.reason values(1,'AAAAAAAABAAAAAAA','reason 1'),(2,'AAAAAAAABAAAAAAA','reason 2'),(3,'AAAAAAAABAAAAAAA','reason 3'),(4,'AAAAAAAABAAAAAAA','reason 4'),(4,'AAAAAAAABAAAAAAA','reason 5'),(4,'AAAAAAAACAAAAAAA','reason 6'),(5,'AAAAAAAACAAAAAAA','reason 7');

--将 tpcds. reason表中r_reason_sk小于5的值加入到新建表中。
SELECT * INTO tpcds. reason_t1 FROM tpcds. reason WHERE r_reason_sk < 5;

--删除 tpcds. reason_t1表。
DROP TABLE tpcds. reason_t1;

--删除表。
DROP TABLE tpcds.reason;

--删除SCHEMA。
DROP SCHEMA tpcds CASCADE;

