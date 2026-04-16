-- 来源: 1259_CREATE PACKAGE.txt
-- SQL 数量: 15

CREATE DATABASE ora_compat_db DBCOMPATIBILITY 'ORA';

CREATE OR REPLACE PACKAGE emp_bonus IS var1 INT:=1;

DROP PACKAGE emp_bonus;

DROP TABLE IF EXISTS test1;

--创建包头
CREATE OR REPLACE PACKAGE emp_bonus IS var1 INT:=1;

--创建包体
CREATE OR REPLACE PACKAGE BODY emp_bonus IS var3 INT:=3;

ALTER PACKAGE emp_bonus OWNER TO omm;

--将PACKAGE emp_bonus的所属者改为omm 调用PACKAGE示例
CALL emp_bonus.testpro1(1);

DROP TABLE IF EXISTS test1;

SELECT emp_bonus.testpro1(1);

DROP TABLE IF EXISTS test1;

--匿名块里调用package存储过程
BEGIN emp_bonus.testpro1(1);

DROP TABLE IF EXISTS test1;

--删除PACKAGE。
DROP PACKAGE emp_bonus;

--删除数据库。
DROP DATABASE ora_compat_db;

