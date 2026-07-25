-- 来源: 2962_CREATE PACKAGE.txt
-- SQL 数量: 11

CREATE OR REPLACE PACKAGE emp_bonus IS var1 INT:=1;

DROP TABLE IF EXISTS test1;

CREATE OR REPLACE PACKAGE BODY emp_bonus IS var3 INT:=3;

ALTER PACKAGE emp_bonus OWNER TO omm;

--将PACKAGE emp_bonus的所属者改为omm 调用PACKAGE示例
CALL emp_bonus.testpro1(1);

DROP TABLE IF EXISTS test1;

SELECT emp_bonus.testpro1(1);

DROP TABLE IF EXISTS test1;

--匿名块里调用PACKAGE存储过程
BEGIN emp_bonus.testpro1(1);

DROP TABLE IF EXISTS test1;

DROP PACKAGE emp_bonus;

