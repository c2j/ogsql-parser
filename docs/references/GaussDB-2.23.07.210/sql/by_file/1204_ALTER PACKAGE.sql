-- 来源: 1204_ALTER PACKAGE.txt
-- SQL 数量: 6

SET behavior_compat_options ='plpgsql_dependency';

-- 创建包
CREATE OR REPLACE PACKAGE TEST_PKG AS pkg_var int := 1;

CREATE OR REPLACE PACKAGE BODY TEST_PKG AS PROCEDURE test_pkg_proc(var int) IS BEGIN pkg_var := 1;

-- 重编译包
ALTER PACKAGE test_pkg COMPILE;

-- 删除包
DROP PACKAGE TEST_PKG;

-- 关闭依赖功能
SET behavior_compat_options = '';

