-- 来源: 3168_file_3168.txt
-- SQL 数量: 24

DROP SCHEMA IF EXISTS hr CASCADE;

CREATE SCHEMA hr;

SET current_schema = hr;

DROP TABLE IF EXISTS sections;

DROP TABLE IF EXISTS staffs;

DROP TABLE IF EXISTS department;

--创建部门表
CREATE TABLE sections( section_name varchar(100), place_id int, section_id int );

INSERT INTO sections VALUES ('hr',1,1);

--创建员工表
CREATE TABLE staffs( staff_id number(6), salary number(8,2), section_id int, first_name varchar(20) );

INSERT INTO staffs VALUES (1,100,1,'Tom');

--创建部门表
CREATE TABLE department( section_id int );

CREATE OR REPLACE PROCEDURE cursor_proc1 () AS DECLARE DEPT_NAME VARCHAR ( 100 );

CALL cursor_proc1 ();

hr ---1 hr ---1 hr ---1 cursor_proc1 -------------- ( 1 row )

DROP PROCEDURE cursor_proc1 ;

CREATE TABLE hr . staffs_t1 AS TABLE hr . staffs ;

CREATE OR REPLACE PROCEDURE cursor_proc2 () AS DECLARE V_EMPNO NUMBER ( 6 );

CALL cursor_proc2 ();

DROP PROCEDURE cursor_proc2 ;

DROP TABLE hr . staffs_t1 ;

CREATE OR REPLACE PROCEDURE proc_sys_ref ( O OUT SYS_REFCURSOR ) IS C1 SYS_REFCURSOR ;

DECLARE C1 SYS_REFCURSOR ;

1 1 ANONYMOUS BLOCK EXECUTE --删除存储过程

DROP PROCEDURE proc_sys_ref ;

