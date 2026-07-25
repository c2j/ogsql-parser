-- 来源: 1093_file_1093.txt
-- SQL 数量: 64

SELECT current_query ();

SELECT current_schema ();

SELECT current_schemas ( true );

SELECT database ();

SELECT current_user ;

SELECT definer_current_user ();

SELECT pg_current_sessionid ();

select pg_current_sessid();

SELECT pg_current_userid();

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

select sessionid2pid ( sessid :: cstring ) from pv_session_stat limit 2 ;

SELECT session_context ( 'USERENV' , 'CURRENT_SCHEMA' );

SELECT pg_trigger_depth ();

SELECT opengauss_version ();

select gs_deployment ();

SELECT session_user ;

SELECT user ;

select get_shard_oids_byname ( 'datanode1' );

select getpgusername ();

select getdatabaseencoding ();

SELECT version ();

SELECT working_version_num ();

SELECT get_hostname ();

SELECT get_nodename ();

SELECT get_nodeinfo ( 'node_type' );

SELECT get_nodeinfo ( 'node_name' );

SELECT get_schema_oid ( 'public' );

SELECT pgxc_parse_clog ();

SELECT pgxc_parse_clog ( '-1' );

SELECT pgxc_prepared_xact ();

SELECT pgxc_xacts_iscommitted ( 1 );

SELECT pgxc_total_memory_detail ();

SELECT has_table_privilege ( 'tpcds.web_site' , 'select' );

SELECT has_table_privilege ( 'omm' , 'tpcds.web_site' , 'select,INSERT WITH GRANT OPTION ' );

SELECT relname FROM pg_class WHERE pg_table_is_visible ( oid );

SELECT format_type (( SELECT oid FROM pg_type WHERE typname = 'varchar' ), 10 );

select pg_check_authid(1);

select * from pg_get_functiondef(598);

select * from pg_get_indexdef(16416);

select * from pg_get_indexdef(16416, true);

select * from pg_get_indexdef(16416, 0, false);

select * from pg_get_indexdef(16416, 1, false);

select pg_check_authid(20);

select * from pg_get_tabledef(16384);

select * from pg_get_tabledef('t1');

SELECT pg_typeof ( 33 );

SELECT typlen FROM pg_type WHERE oid = pg_typeof ( 33 );

SELECT collation for ( description ) FROM pg_description LIMIT 1 ;

SELECT getdistributekey ( 'item' );

select * from pg_get_serial_sequence('t1', 'c1');

select * from pg_sequence_parameters(16420);

select pgxc_get_variable_info( );

select * from gs_get_index_status('public', 'index1');

select * from gs_get_kernel_info();

