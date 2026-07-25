-- 来源: 1279_CREATE VIEW.txt
-- SQL 数量: 10

CREATE TABLE test_tb1(col1 int, col2 int);

INSERT INTO test_tb1 VALUES (generate_series(1,100),generate_series(1,100));

--创建一个col1小于5的视图。
CREATE VIEW test_v1 AS SELECT * FROM test_tb1 WHERE col1 < 3;

--查看视图。
SELECT * FROM test_v1;

--删除表和视图。
DROP VIEW test_v1;

DROP TABLE test_tb1;

CREATE TABLE test_tb2(c1 int, c2 int);

CREATE TEMP VIEW test_v2 AS SELECT * FROM test_tb2;

--删除视图和表。
DROP VIEW test_v2 ;

DROP TABLE test_tb2;

