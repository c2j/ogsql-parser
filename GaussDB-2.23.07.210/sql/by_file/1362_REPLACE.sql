-- 来源: 1362_REPLACE.txt
-- SQL 数量: 9

CREATE TABLE test(f1 int primary key, f2 int, f3 int);

--插入数据。
INSERT INTO test VALUES(1, 1, 1), (2, 2, 2), (3, 3, 3);

--值替换插入数据。
REPLACE INTO test VALUES(1, 11, 11);

--查询值替换插入的结果
SELECT * FROM test WHERE f1 = 1;

--查询替换插入数据。
REPLACE INTO test SELECT 2, 22, 22;

SELECT * FROM test WHERE f1 = 2;

--设置指定字段替换插入数据。
REPLACE INTO test SET f1 = f1 + 3, f2 = f1 * 10 + 3, f3 = f2;

SELECT * FROM test WHERE f1 = 3;

DROP TABLE test;

