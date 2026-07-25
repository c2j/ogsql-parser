-- 来源: 2931_CALL.txt
-- SQL 数量: 8

CREATE FUNCTION func_add_sql(num1 integer, num2 integer) RETURN integer AS BEGIN RETURN num1 + num2;

--按参数值传递。
CALL func_add_sql(1, 3);

--使用命名标记法传参。
CALL func_add_sql(num1 => 1,num2 => 3);

CALL func_add_sql(num2 := 2, num1 := 3);

--删除函数。
DROP FUNCTION func_add_sql;

--创建带出参的函数。
CREATE FUNCTION func_increment_sql(num1 IN integer, num2 IN integer, res OUT integer) RETURN integer AS BEGIN res := num1 + num2;

--出参传入常量。
CALL func_increment_sql(1,2,1);

--删除函数。
DROP FUNCTION func_increment_sql;

