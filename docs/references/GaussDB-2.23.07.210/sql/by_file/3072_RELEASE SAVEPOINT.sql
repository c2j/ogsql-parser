-- 来源: 3072_RELEASE SAVEPOINT.txt
-- SQL 数量: 11

CREATE SCHEMA tpcds;

--创建一个新表。
CREATE TABLE tpcds. table1(a int);

--开启事务。
START TRANSACTION;

--插入数据。
INSERT INTO tpcds. table1 VALUES (3);

--建立保存点。
SAVEPOINT my_savepoint;

--插入数据。
INSERT INTO tpcds. table1 VALUES (4);

--删除保存点。
RELEASE SAVEPOINT my_savepoint;

--提交事务。
COMMIT;

--查询表的内容，会同时看到3和4。
SELECT * FROM tpcds. table1;

--删除表。
DROP TABLE tpcds. table1;

--删除SCHEMA。
DROP SCHEMA tpcds CASCADE;

