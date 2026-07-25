-- 来源: 4027_file_4027.txt
-- SQL 数量: 114

select 0 . 1231243 as a , 0 . 1231243 :: numeric as b , 0 . 1231243 :: integer ( 10 , 3 ) as c , length ( 0 . 1242343 ) as d ;

select 0 . 1231243 as a , 0 . 1231243 :: numeric as b , 0 . 1231243 :: integer ( 10 , 3 ) as c , length ( 0 . 1242343 ) as d ;

select add_months ( '2018-02-28' , 3 ) from sys_dummy ;

select add_months ( '2018-02-28' , 3 ) from sys_dummy ;

select '' AS to_number_14 , to_number ( '34,50' , '999,99' );

select '' AS to_number_14 , to_number ( '34,50' , '999,99' );

select ( - 2147483648 ):: int4 / ( - 1 ):: int4 ;

select ( - 2147483648 ):: int4 / ( - 1 ):: int4 ;

create table test1 ( c1 int , c2 varchar );

insert into test1 values ( 2 , '1.1' );

set behavior_compat_options = '' ;

select * from test1 where c2 > 1 ;

set behavior_compat_options = 'convert_string_digit_to_numeric' ;

select * from test1 where c2 > 1 ;

select length ( lpad ( '123' , 0 , '*' )) from sys_dummy ;

select length ( lpad ( '123' , 0 , '*' )) from sys_dummy ;

select concat ( variadic NULL :: int []) is NULL ;

select concat ( variadic NULL :: int []) is NULL ;

select concat ( variadic NULL :: int []) is NULL ;

set behavior_compat_options='hide_tailing_zero';

select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

set behavior_compat_options='';

select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

set behavior_compat_options='';

create table tb_test(c1 int,c2 varchar2,c3 varchar2);

insert into tb_test values(1,'a','b');

create or replace view v_test as select rownum from tb_test;

set behavior_compat_options = 'rownum_type_compat';

create or replace view v_test1 as select rownum from tb_test;

set behavior_compat_options='aformat_null_test';

select r, r is null as isnull, r is not null as isnotnull from (values (1,row(1,2)), (1,row(null,null)), (1,null), (null,row(1,2)), (null,row(null,null)), (null,null) ) r(a,b);

set behavior_compat_options='';

select r, r is null as isnull, r is not null as isnotnull from (values (1,row(1,2)), (1,row(null,null)), (1,null), (null,row(1,2)), (null,row(null,null)), (null,null) ) r(a,b);

set behavior_compat_options='';

create table tab_1(col1 varchar(3));

create table tab_2(col2 char(3));

insert into tab_2 values(' ');

insert into tab_1 select col2 from tab_2;

select * from tab_1 where col1 is null;

select * from tab_1 where col1=' ';

delete from tab_1;

set behavior_compat_options = 'char_coerce_compat';

insert into tab_1 select col2 from tab_2;

select * from tab_1 where col1 is null;

select * from tab_1 where col1=' ';

set behavior_compat_options='truncate_numeric_tail_zero';

select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

set behavior_compat_options='';

select cast(123.123 as numeric(15,10)) as a, to_char(cast(123.123 as numeric(15,10)), '999D999999');

create or replace function test(f1 int, f2 int default 20, f3 int, f4 int default 40, f5 int default 50) return int gaussdb -# as gaussdb $# begin gaussdb $# raise info 'f1:%',f1;

select test(1,2);

create or replace function test(f1 int, f2 int default 20, f3 int, f4 int default 40, f5 int default 50) return int gaussdb -# as gaussdb $# begin gaussdb $# raise info 'f1:%',f1;

select test(1,2);

select power(2,3);

select count(*) from db_ind_columns;

select count(index_name) from db_ind_columns;

SELECT left('abcde', 2);

SELECT pg_client_encoding();

set behavior_compat_options = 'enable_funcname_with_argsname';

select power(2,3);

select count(*) from db_ind_columns;

select count(index_name) from db_ind_columns;

SELECT left('abcde', 2);

SELECT pg_client_encoding();

SET behavior_compat_options='proc_outparam_override,proc_outparam_transfer_length';

CREATE OR REPLACE PROCEDURE out_param_test1(m in int, v inout varchar2,v1 inout varchar2) is gaussdb$# begin gaussdb$# v := 'aaaddd';

CREATE OR REPLACE PROCEDURE call_out_param_test1 is gaussdb$# v varchar2(5) := 'aabbb';

CALL call_out_param_test1();

CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of varchar(5);

CALL p1();

SET behavior_compat_options = 'tableof_elem_constraints';

CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of varchar(5);

CALL p1();

CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of int index by varchar(5);

CALL p1();

SET behavior_compat_options = 'tableof_elem_constraints';

CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of int index by varchar(5);

CALL p1();

set behavior_compat_options='current_sysdate';

select sysdate;

create or replace function proc_test return varchar2 as gaussdb$# begin gaussdb$# return '1';

create or replace procedure proc_test as gaussdb$# begin gaussdb$# null;

-- 设置参数后允许替换类型
set behavior_compat_options='allow_function_procedure_replace';

create or replace procedure proc_test as gaussdb$# begin gaussdb$# null;

create or replace procedure p1 is gaussdb$# type t1 is table of int;

call p1();

create or replace procedure p1 is gaussdb$# type t1 is table of int;

call p1();

create or replace procedure p1 is gaussdb$# type t1 is table of int;

call p1();

set behavior_compat_options = 'collection_exception_backcompat';

create or replace procedure p1 is gaussdb$# type t1 is table of int;

call p1();

create or replace procedure p1 is gaussdb$# type t1 is table of int;

call p1();

create or replace procedure p1 is gaussdb$# type t1 is table of int;

call p1();

set behavior_compat_options='enable_case_when_alias';

create table test(c1 varchar2);

insert into test values('x');

select decode(c1,'x','0','default') from test;

select (case c1 when 'x' then '0' else 'default' end) from test;

create user plsql_rollback1 PASSWORD '********';

create user plsql_rollback2 PASSWORD '********';

grant plsql_rollback1 to plsql_rollback2;

create or replace procedure plsql_rollback1.p1 () authid definer as gaussdb$# va int;

set session AUTHORIZATION plsql_rollback2 PASSWORD '********';

CREATE SCHEMA sch1;

CREATE PACKAGE pck1 IS PROCEDURE sch1.pck1();

select timestamp '2024-03-20 01:30:00’ at time zone 'Europe/Moscow' from dual;

set behavior_compat_options='enable_use_ora_timestamptz';

select timestamp '2024-03-20 01:30:00’ at time zone 'Europe/Moscow' from dual;

set gs_format_behavior_compat_options='allow_textconcat_null';

select 'a' || null || 'b';

