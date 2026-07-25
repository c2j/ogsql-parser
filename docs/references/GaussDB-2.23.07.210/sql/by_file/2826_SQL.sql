-- 来源: 2826_SQL.txt
-- SQL 数量: 10

select gs_add_workload_rule ( 'sqlid' , 'rule for one query' , '' , now (), '' , 20 , '{id=32413214}' );

create database db1 ;

create database db2 ;

select gs_add_workload_rule ( 'select' , 'rule for select' , '{db1, db2}' , '' , '' , 100 , '{tb1, tb2}' );

select gs_add_workload_rule ( 'resource' , 'rule for resource' , '{}' , '' , '' , 20 , '{cpu-80}' );

create database db1 ;

select gs_update_workload_rule ( 2 , 'rule for select 2' , '{db1}' , now (), '' , 50 , '{tb1}' );

select gs_delete_workload_rule ( 3 );

select * from gs_get_workload_rule_stat ( 1 );

select * from gs_get_workload_rule_stat ( - 1 );

