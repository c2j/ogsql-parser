-- 来源: 3080_SAVEPOINT.txt
-- SQL 数量: 19

CREATE TABLE table1(a int);

--开启事务。
START TRANSACTION;

--插入数据。
INSERT INTO table1 VALUES (1);

--建立保存点。
SAVEPOINT my_savepoint;

--插入数据。
INSERT INTO table1 VALUES (2);

--回滚保存点。
ROLLBACK TO SAVEPOINT my_savepoint;

--插入数据。
INSERT INTO table1 VALUES (3);

--提交事务。
COMMIT;

--查询表的内容，会同时看到1和3,不能看到2，因为2被回滚。
SELECT * FROM table1;

--删除表。
DROP TABLE table1;

--创建一个新表。
CREATE TABLE table2(a int);

--开启事务。
START TRANSACTION;

--插入数据。
INSERT INTO table2 VALUES (3);

--建立保存点。
SAVEPOINT my_savepoint;

--插入数据。
INSERT INTO table2 VALUES (4);

--回滚保存点。
RELEASE SAVEPOINT my_savepoint;

--提交事务。
COMMIT;

--查询表的内容，会同时看到3和4。
SELECT * FROM table2;

--删除表。
DROP TABLE table2;

