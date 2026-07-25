-- 来源: 3151_file_3151.txt
-- SQL 数量: 10

DROP SCHEMA IF EXISTS hr CASCADE;

CREATE SCHEMA hr;

SET CURRENT_SCHEMA = hr;

CREATE TABLE staffs ( staff_id NUMBER, first_name VARCHAR2, salary NUMBER );

INSERT INTO staffs VALUES (200, 'mike', 5800);

INSERT INTO staffs VALUES (201, 'lily', 3000);

INSERT INTO staffs VALUES (202, 'john', 4400);

--创建存储过程dynamic_proc
CREATE OR REPLACE PROCEDURE dynamic_proc AS staff_id NUMBER(6) := 200;

--调用存储过程
CALL dynamic_proc();

--删除存储过程
DROP PROCEDURE dynamic_proc;

