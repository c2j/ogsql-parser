-- 来源: 2904_ALTER PROCEDURE.txt
-- SQL 数量: 5

SET behavior_compat_options ='plpgsql_dependency';

-- 创建存储过程
CREATE OR REPLACE PROCEDURE test_proc(a int) IS proc_var int;

-- 用存储过程名重编译存储过程
ALTER PROCEDURE test_proc COMPILE;

-- 用存储过程带类型签名重编译存储过程
ALTER PROCEDURE test_proc(int) COMPILE;

-- 删除存储过程
DROP PROCEDURE test_proc;

