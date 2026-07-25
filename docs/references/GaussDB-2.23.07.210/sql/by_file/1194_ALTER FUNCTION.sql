-- 来源: 1194_ALTER FUNCTION.txt
-- SQL 数量: 5

SET behavior_compat_options ='plpgsql_dependency';

-- 创建函数
CREATE OR REPLACE FUNCTION test_func(a int) RETURN int IS proc_var int;

-- 用函数名重编译函数
ALTER PROCEDURE test_func COMPILE;

-- 用函数带类型签名重编译存储过程
ALTER PROCEDURE test_func(int) COMPILE;

-- 删除函数
DROP FUNCTION test_func;

