-- 来源: 3148_file_3148.txt
-- SQL 数量: 17

DROP SCHEMA IF EXISTS hr CASCADE;

CREATE SCHEMA hr;

SET CURRENT_SCHEMA = hr;

CREATE TABLE staffs ( staff_id NUMBER, first_name VARCHAR2, salary NUMBER );

INSERT INTO staffs VALUES (200, 'mike', 5800);

INSERT INTO staffs VALUES (201, 'lily', 3000);

INSERT INTO staffs VALUES (202, 'john', 4400);

--从动态语句检索值（INTO 子句）：
DECLARE staff_count VARCHAR2(20);

--传递并检索值（INTO子句用在USING子句前）：
CREATE OR REPLACE PROCEDURE dynamic_proc AS staff_id NUMBER(6) := 200;

--调用存储过程
CALL dynamic_proc();

--删除存储过程
DROP PROCEDURE dynamic_proc;

CREATE SCHEMA hr;

SET CURRENT_SCHEMA = hr;

CREATE TABLE staffs ( section_id NUMBER, first_name VARCHAR2, phone_number VARCHAR2, salary NUMBER );

INSERT INTO staffs VALUES (30, 'mike', '13567829252', 5800);

INSERT INTO staffs VALUES (40, 'john', '17896354637', 4000);

DECLARE name VARCHAR2(20);

