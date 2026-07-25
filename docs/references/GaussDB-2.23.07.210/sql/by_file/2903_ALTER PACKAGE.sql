-- 来源: 2903_ALTER PACKAGE.txt
-- SQL 数量: 6

CREATE DATABASE ora_compatible_db DBCOMPATIBILITY 'ORA';

-- 开启依赖功能
SET behavior_compat_options ='plpgsql_dependency';

-- 创建包
CREATE OR REPLACE PACKAGE test_pkg AS pkg_var int := 1;

CREATE OR REPLACE PACKAGE body test_pkg AS procedure test_pkg_proc(var int) IS BEGIN pkg_var := 1;

-- 重编译包
ALTER PACKAGE test_pkg COMPILE;

--删除包
DROP PACKAGE test_pkg;

