-- 来源: 3046_FETCH.txt
-- SQL 数量: 18

CREATE SCHEMA tpcds;

--创建表tpcds.customer_address。
CREATE TABLE tpcds.customer_address ( ca_address_sk INTEGER NOT NULL, ca_address_id CHARACTER(16) NOT NULL, ca_street_number INTEGER , ca_street_name CHARACTER (20) );

--向表中插入多条记录。
INSERT INTO tpcds.customer_address VALUES (1, 'AAAAAAAABAAAAAAA', '18', 'Jackson'),(2, 'AAAAAAAACAAAAAAA', '362', 'Washington 6th'),(3, 'AAAAAAAADAAAAAAA', '585', 'Dogwood Washington');

--SELECT语句，用一个游标读取一个表。开始一个事务。
START TRANSACTION;

--建立一个名为cursor1的游标。
CURSOR cursor1 FOR SELECT * FROM tpcds. customer_address ORDER BY 1;

--抓取头3行到游标cursor1里。
FETCH FORWARD 3 FROM cursor1;

--关闭游标并提交事务。
CLOSE cursor1;

--VALUES子句，用一个游标读取VALUES子句中的内容。开始一个事务。
START TRANSACTION;

--建立一个名为cursor2的游标。
CURSOR cursor2 FOR VALUES(1,2),(0,3) ORDER BY 1;

--抓取头2行到游标cursor2里。
FETCH FORWARD 2 FROM cursor2;

--关闭游标并提交事务。
CLOSE cursor2;

--WITH HOLD游标的使用，开启事务。
START TRANSACTION;

--创建一个with hold游标。
DECLARE cursor1 CURSOR WITH HOLD FOR SELECT * FROM tpcds. customer_address ORDER BY 1;

--抓取头2行到游标cursor1里。
FETCH FORWARD 2 FROM cursor1;

--抓取下一行到游标cursor1里。
FETCH FORWARD 1 FROM cursor1;

--关闭游标。
CLOSE cursor1;

--删除表tpcds.customer_address。
DROP TABLE tpcds.customer_address;

--删除SCHEMA。
DROP SCHEMA tpcds CASCADE;

