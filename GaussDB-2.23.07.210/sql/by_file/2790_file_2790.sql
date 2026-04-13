-- 来源: 2790_file_2790.txt
-- SQL 数量: 61

SELECT current_query ();

SELECT current_schema ();

SELECT current_schemas ( true );

SELECT database ();

SELECT current_user ;

SELECT definer_current_user ();

SELECT pg_current_sessionid ();

select pg_current_sessid();

SELECT pg_current_userid();

SELECT working_version_num ();

select tablespace_oid_name ( 1663 );

SELECT inet_client_addr ();

SELECT inet_client_port ();

SELECT inet_server_addr ();

SELECT inet_server_port ();

SELECT pg_backend_pid ();

SELECT pg_conf_load_time ();

SELECT pg_my_temp_schema ();

SELECT pg_is_other_temp_schema ( 25356 );

SELECT pg_listening_channels ();

SELECT pg_postmaster_start_time ();

select * from pg_get_ruledef(24828);

select sessionid2pid ( sessid :: cstring ) from gs_session_stat limit 2 ;

SELECT session_context ( 'USERENV' , 'CURRENT_SCHEMA' );

SELECT pg_trigger_depth ();

SELECT session_user ;

SELECT user ;

select getpgusername ();

select getdatabaseencoding ();

select version();

select opengauss_version ();

select gs_deployment ();

SELECT get_hostname ();

SELECT get_nodename ();

SELECT get_nodeinfo ( 'node_type' );

SELECT get_nodeinfo ( 'node_name' );

SELECT get_schema_oid ( 'public' );

SELECT has_table_privilege ( 'tpcds.web_site' , 'select' );

SELECT has_table_privilege ( 'omm' , 'tpcds.web_site' , 'select,INSERT WITH GRANT OPTION ' );

SELECT relname FROM pg_class WHERE pg_table_is_visible ( oid );

SELECT format_type (( SELECT oid FROM pg_type WHERE typname = 'varchar' ), 10 );

select pg_check_authid(1);

select * from pg_get_functiondef(598);

select * from pg_get_indexdef(16416);

CREATE TABLE sales (prod_id NUMBER(6), cust_id NUMBER, time_id DATE, channel_id CHAR(1), promo_id NUMBER(6), quantity_sold NUMBER(3), amount_sold NUMBER(10,2) ) PARTITION BY RANGE( time_id) INTERVAL('1 day') ( partition p1 VALUES LESS THAN ('2019-02-01 00:00:00'), partition p2 VALUES LESS THAN ('2019-02-02 00:00:00') );

create index index_sales on sales(prod_id) local (PARTITION idx_p1 ,PARTITION idx_p2);

INSERT INTO sales VALUES(1, 12, '2019-02-05 00:00:00', 'a', 1, 1, 1);

select oid from pg_class where relname = 'index_sales';

select * from pg_get_indexdef(24632, true);

select * from pg_get_indexdef(24632, false);

select * from pg_get_indexdef(16416, 0, false);

select * from pg_get_indexdef(16416, 1, false);

select pg_check_authid(20);

select * from pg_get_tabledef(16384);

select * from pg_get_tabledef('t1');

SELECT pg_typeof ( 33 );

SELECT typlen FROM pg_type WHERE oid = pg_typeof ( 33 );

SELECT collation for ( description ) FROM pg_description LIMIT 1 ;

select * from pg_get_serial_sequence('t1', 'c1');

select * from pg_sequence_parameters(16420);

select * from gs_get_kernel_info();

