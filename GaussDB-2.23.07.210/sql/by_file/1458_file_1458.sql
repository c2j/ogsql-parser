-- 来源: 1458_file_1458.txt
-- SQL 数量: 23

drop schema if exists hr cascade;

create schema hr;

set current_schema = hr;

drop table if exists sections;

drop table if exists staffs;

drop table if exists department;

--创建部门表
create table sections( section_name varchar(100), place_id int, section_id int );

insert into sections values ('hr',1,1);

--创建员工表
create table staffs( staff_id number(6), salary number(8,2), section_id int, first_name varchar(20) );

insert into staffs values (1,100,1,'Tom');

--创建部门表
create table department( section_id int );

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

DROP PROCEDURE proc_sys_ref ;

