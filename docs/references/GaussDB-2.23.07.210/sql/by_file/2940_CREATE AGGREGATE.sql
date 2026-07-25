-- 来源: 2940_CREATE AGGREGATE.txt
-- SQL 数量: 8

CREATE OR REPLACE FUNCTION int_add(int,int) returns int as $BODY$ declare begin return $1 + $2;

-- 创建聚合函数
CREATE AGGREGATE sum (int) ( sfunc = int_add, stype = int, initcond = '0' );

-- 创建测试表和添加数据
CREATE TABLE test_sum(a int,b int,c int);

INSERT INTO test_sum VALUES(1,2),(2,3),(3,4),(4,5);

-- 执行聚合函数
SELECT sum(a) FROM test_sum;

-- 删除聚合函数
DROP AGGREGATE sum(int);

-- 删除自定义函数
DROP FUNCTION int_add(int,int);

-- 删除测试表
DROP TABLE test_sum;

