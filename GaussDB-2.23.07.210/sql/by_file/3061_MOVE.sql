-- 来源: 3061_MOVE.txt
-- SQL 数量: 10

CREATE SCHEMA tpcds;

--创建表tpcds.reason。
CREATE TABLE tpcds.reason ( r_reason_sk INTEGER NOT NULL, r_reason_id CHAR(16) NOT NULL, r_reason_desc VARCHAR(40) );

--向表中插入多条记录。
INSERT INTO tpcds.reason VALUES (1, 'AAAAAAAABAAAAAAA', 'Xxxxxxxxx'),(2, 'AAAAAAAACAAAAAAA', ' Xxxxxxxxx'),(3, 'AAAAAAAADAAAAAAA', ' Xxxxxxxxx'),(4, 'AAAAAAAAEAAAAAAA', 'Not the product that was ordered'),(5, 'AAAAAAAAFAAAAAAA', 'Parts missing'),(6, 'AAAAAAAAGAAAAAAA', 'Does not work with a product that I have'),(7, 'AAAAAAAAHAAAAAAA', 'Gift exchange');

--开始一个事务。
START TRANSACTION;

--定义一个名为cursor1的游标。
CURSOR cursor1 FOR SELECT * FROM tpcds. reason;

--忽略游标cursor1的前3行。
MOVE FORWARD 3 FROM cursor1;

--抓取游标cursor1的前4行。
FETCH 4 FROM cursor1;

--关闭游标。
CLOSE cursor1;

--删除表。
DROP TABLE tpcds.reason;

--删除SCHEMA。
DROP SCHEMA tpcds CASCADE;

