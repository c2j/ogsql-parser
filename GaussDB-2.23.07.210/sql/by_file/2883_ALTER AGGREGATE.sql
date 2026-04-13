-- 来源: 2883_ALTER AGGREGATE.txt
-- SQL 数量: 10

CREATE OR REPLACE FUNCTION int_add(int,int) returns int as $BODY$ declare begin return $1 + $2;

-- 创建聚合函数。
CREATE AGGREGATE myavg (int) ( sfunc = int_add, stype = int, initcond = '0' );

--把一个接受integer 类型参数的聚合函数myavg重命名为 my_average 。
ALTER AGGREGATE myavg(integer) RENAME TO my_average;

--创建用户joe。
CREATE USER joe PASSWORD ' ******** ';

--把一个接受integer 类型参数的聚合函数myavg的所有者改为joe 。
ALTER AGGREGATE my_average(integer) OWNER TO joe;

--创建SCHEMA。
CREATE SCHEMA myschema;

--把一个接受integer 类型参数的聚合函数myavg移动到模式myschema里。
ALTER AGGREGATE my_average(integer) SET SCHEMA myschema;

--删除SCHEMA,用户及相关函数。
DROP SCHEMA myschema CASCADE;

DROP USER joe;

DROP FUNCTION int_add(int,int);

