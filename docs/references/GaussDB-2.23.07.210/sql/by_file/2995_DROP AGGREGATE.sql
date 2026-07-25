-- 来源: 2995_DROP AGGREGATE.txt
-- SQL 数量: 4

CREATE OR REPLACE FUNCTION int_add(int,int) returns int as $BODY$ declare begin return $1 + $2;

-- 创建聚合函数。
CREATE AGGREGATE myavg (int) ( sfunc = int_add, stype = int, initcond = '0' );

--将integer类型的聚合函数myavg删除。
DROP AGGREGATE myavg(integer);

-- 删除自定义函数
DROP FUNCTION int_add(int,int);

