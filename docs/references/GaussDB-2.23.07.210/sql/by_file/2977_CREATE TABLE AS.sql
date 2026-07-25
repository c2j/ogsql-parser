-- 来源: 2977_CREATE TABLE AS.txt
-- SQL 数量: 6

CREATE TABLE test1(col1 int PRIMARY KEY,col2 varchar(10));

INSERT INTO test1 VALUES (1,'col1'),(101,'col101');

-- 查询表中col1<100的数据。
SELECT * FROM test1 WHERE col1 < 100;

-- 创建test2表并向表中插入上面查询的数据。
CREATE TABLE test2 AS SELECT * FROM test1 WHERE col1 < 100;

CREATE TABLE test3(c1,c2) AS SELECT * FROM test1;

-- 删除。
DROP TABLE test1,test2,test3;

