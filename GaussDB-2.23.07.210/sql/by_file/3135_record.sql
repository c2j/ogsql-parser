-- 来源: 3135_record.txt
-- SQL 数量: 17

create table emp_rec ( gaussdb ( # empno numeric ( 4 , 0 ) not null , gaussdb ( # ename varchar ( 10 ) gaussdb ( # );

insert into emp_rec values ( 111 , 'aaa' ), ( 222 , 'bbb' ), ( 333 , 'ccc' );

\ d emp_rec Table "public.emp_rec" Column | Type | Modifiers --------+-----------------------+----------- empno | numeric ( 4 , 0 ) | not null ename | character varying ( 10 ) | --演示在函数中对record进行操作。

CREATE OR REPLACE FUNCTION regress_record ( p_w VARCHAR2 ) RETURNS VARCHAR2 AS $$ gaussdb $ # DECLARE gaussdb $ # --声明一个record类型. gaussdb $ # type rec_type is record ( name varchar2 ( 100 ), epno int );

CALL regress_record ( 'abc' );

DROP FUNCTION regress_record ;

DROP TABLE emp_rec ;

create type rec_type is ( c1 int , c2 int );

set behavior_compat_options = 'proc_outparam_override' ;

create or replace function func ( a in int ) return rec_type is gaussdb $ # r rec_type ;

call func ( 0 );

drop function func ;

drop type rec_type ;

set behavior_compat_options = 'proc_outparam_override' ;

create or replace function func ( a out int ) return record is gaussdb $ # type rc is record ( c1 int , c2 int );

call func ( 1 );

drop function func ;

