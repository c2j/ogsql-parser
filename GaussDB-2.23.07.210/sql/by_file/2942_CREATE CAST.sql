-- 来源: 2942_CREATE CAST.txt
-- SQL 数量: 3

CREATE OR REPLACE FUNCTION double_to_timestamp(double precision) RETURNS TIMESTAMP WITH TIME ZONE AS $$ SELECT to_timestamp($1);

--创建类型转换
CREATE CAST(double precision AS timestamp with time zone) WITH FUNCTION double_to_timestamp(double precision) AS IMPLICIT;

--删除类型转换
DROP CAST (double precision AS timestamp with time zone);

