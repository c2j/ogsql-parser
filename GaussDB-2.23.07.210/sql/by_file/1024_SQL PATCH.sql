-- 来源: 1024_SQL PATCH.txt
-- SQL 数量: 32

create table hint_t1 ( a int , b int , c int );

create index on hint_t1 ( a );

insert into hint_t1 values ( 1 , 1 , 1 );

analyze hint_t1 ;

set track_stmt_stat_level = 'L1,L1' ;

set enable_fast_query_shipping = off ;

set explain_perf_mode = normal ;

select * from hint_t1 where hint_t1 . a = 1 ;

\ x --切换扩展显示模式，便于观察计划 Expanded display is on .

select unique_query_id , query , query_plan from dbe_perf . statement_history where query like '%hint_t1%' ;

\ x --关闭扩展显示模式

select * from dbe_sql_util . create_hint_sql_patch ( 'patch1' , 3929365485 , 'indexscan(hint_t1)' );

set track_stmt_stat_level = 'L1,L1' ;

set enable_fast_query_shipping = off ;

explain select * from hint_t1 where hint_t1 . a = 1 ;

select * from hint_t1 where hint_t1 . a = 1 ;

\ x Expanded display is on .

select unique_query_id , query , query_plan from dbe_perf . statement_history where query like '%hint_t1%' ;

select * from dbe_sql_util.drop_sql_patch('patch1');

select * from dbe_sql_util.create_abort_sql_patch('patch2', 3929365485);

select * from hint_t1 t1 where t1.a = 1;

create table test_proc_patch(a int,b int);

insert into test_proc_patch values(1,2);

create procedure mypro() as num int;

set track_stmt_stat_level = 'L0,L1';

select b from test_proc_patch where a = 1;

call mypro();

select unique_query_id, query, query_plan, parent_unique_sql_id from dbe_perf.statement_history where query like '%call mypro();

select * from dbe_sql_util.create_abort_sql_patch('patch1',2859505004,2502737203);

select patch_name,unique_sql_id,parent_unique_sql_id,enable,abort,hint_string from gs_sql_patch where patch_name = 'patch1';

select b from test_proc_patch where a = 1;

call mypro();

