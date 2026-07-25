-- 类别: OTHER
-- SQL 数量: 412

-- 来源: 1024_SQL PATCH
\ x --切换扩展显示模式，便于观察计划 Expanded display is on .

-- 来源: 1024_SQL PATCH
\ x --关闭扩展显示模式

-- 来源: 1024_SQL PATCH
\ x Expanded display is on .

-- 来源: 1047_file_1047
\ c gaussdb_m --设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

-- 来源: 1050_file_1050
\ c gaussdb_m -- 设置兼容版本控制参数 gaussdb_m =# set b_format_version = '5.7' ;

-- 来源: 1051_file_1051
\ c gaussdb_m --设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

-- 来源: 1059_HLL
\ d t1 Table "public.t1" Column | Type | Modifiers --------+---------+----------- id | integer | set | hll | -- 创建hll类型的表，指定前两个入参，后两个采用默认值。

-- 来源: 1059_HLL
\ d t2 Table "public.t2" Column | Type | Modifiers --------+----------------+----------- id | integer | set | hll ( 12 , 4 , 12 , 0 ) | --创建hll类型的表，指定第三个入参，其余采用默认值。

-- 来源: 1059_HLL
\ d t3 Table "public.t3" Column | Type | Modifiers --------+----------------+----------- id | integer | set | hll ( 14 , 10 , 8 , 0 ) | --创建hll类型的表，指定入参不合法报错。

-- 来源: 1072_file_1072
\ c gaussdb_m -- 设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

-- 来源: 1078_file_1078
\ c gaussdb_m -- 设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

-- 来源: 1173_file_1173
\ dF + ts_conf Text search configuration "public.ts_conf" Parser : "pg_catalog.default" Token | Dictionaries -----------------+------------------------------------- asciihword | gs_dict , english_ispell , english_stem asciiword | gs_dict , english_ispell , english_stem file | simple host | simple hword | gs_dict , english_ispell , english_stem hword_asciipart | gs_dict , english_ispell , english_stem hword_numpart | simple hword_part | gs_dict , english_ispell , english_stem int | simple numhword | simple numword | simple uint | simple version | simple word | gs_dict , english_ispell , english_stem

-- 来源: 1185_ALTER APP WORKLOAD GROUP MAPPING
ALTER APP WORKLOAD GROUP MAPPING app_wg_map1 WITH ( WORKLOAD_GPNAME = wg_hr1 );

--创建公共dblink
-- 来源: 1189_ALTER DATABASE LINK
CREATE PUBLIC DATABASE LINK public_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

-- 修改dblink对象信息
-- 来源: 1189_ALTER DATABASE LINK
ALTER PUBLIC DATABASE LINK public_dblink CONNECT TO 'user2' IDENTIFIED BY '********';

-- 来源: 1189_ALTER DATABASE LINK
DROP PUBLIC DATABASE LINK public_dblink;

-- 来源: 1190_ALTER DATA SOURCE
ALTER DATA SOURCE ds_test1 RENAME TO ds_test ;

-- 来源: 1190_ALTER DATA SOURCE
ALTER DATA SOURCE ds_test OWNER TO user_test1 ;

-- 来源: 1190_ALTER DATA SOURCE
ALTER DATA SOURCE ds_test TYPE 'MPPDB_TYPE' VERSION 'XXX' ;

-- 来源: 1190_ALTER DATA SOURCE
ALTER DATA SOURCE ds_test OPTIONS ( add dsn 'mppdb' , username 'test_user' );

-- 来源: 1190_ALTER DATA SOURCE
ALTER DATA SOURCE ds_test OPTIONS ( set dsn 'unknown' );

-- 来源: 1190_ALTER DATA SOURCE
ALTER DATA SOURCE ds_test OPTIONS ( drop username );

-- 用函数名重编译函数
-- 来源: 1194_ALTER FUNCTION
ALTER PROCEDURE test_func COMPILE;

-- 用函数带类型签名重编译存储过程
-- 来源: 1194_ALTER FUNCTION
ALTER PROCEDURE test_func(int) COMPILE;

-- 来源: 1208_ALTER ROW LEVEL SECURITY POLICY
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING ( role = CURRENT_USER );

-- 来源: 1208_ALTER ROW LEVEL SECURITY POLICY
\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_rls" FOR ALL TO public USING ((( role ):: name = "current_user" ())) Has OIDs : no Distribute By : HASH ( id ) Location Nodes : ALL DATANODES Options : orientation = row , compression = no --修改行访问控制all_data_rls的名称。

-- 来源: 1208_ALTER ROW LEVEL SECURITY POLICY
ALTER ROW LEVEL SECURITY POLICY all_data_rls ON all_data RENAME TO all_data_new_rls ;

-- 来源: 1208_ALTER ROW LEVEL SECURITY POLICY
ALTER ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data TO alice , bob ;

-- 来源: 1208_ALTER ROW LEVEL SECURITY POLICY
\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_new_rls" FOR ALL TO alice , bob USING ((( role ):: name = "current_user" ())) Has OIDs : no Distribute By : HASH ( id ) Location Nodes : ALL DATANODES Options : orientation = row , compression = no , enable_rowsecurity = true --修改行访问控制策略表达式。

-- 来源: 1208_ALTER ROW LEVEL SECURITY POLICY
ALTER ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data USING ( id > 100 AND role = current_user );

-- 来源: 1208_ALTER ROW LEVEL SECURITY POLICY
\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_new_rls" FOR ALL TO alice , bob USING ((( id > 100 ) AND (( role ):: name = "current_user" ()))) Has OIDs : no Distribute By : HASH ( id ) Location Nodes : ALL DATANODES Options : orientation = row , compression = no , enable_rowsecurity = true --删除访问控制策略。

-- 来源: 1208_ALTER ROW LEVEL SECURITY POLICY
DROP ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data ;

-- 来源: 1209_ALTER SCHEMA
\ c test1 --创建模式ds。

-- 来源: 1209_ALTER SCHEMA
\ c postgres

-- 来源: 1212_ALTER SESSION
END ;

-- 来源: 1213_ALTER SYNONYM
\ c - sysadmin --创建同义词t1。

-- 来源: 1213_ALTER SYNONYM
\ c - init_user --删除用户sysadmin。

-- 来源: 1231_CLEAN CONNECTION
CLEAN CONNECTION TO NODE ( dn_6001_6002 , dn_6003_6004 ) FOR DATABASE template1 ;

-- 来源: 1231_CLEAN CONNECTION
CLEAN CONNECTION TO NODE ( dn_6001_6002 ) TO USER jack ;

-- 来源: 1231_CLEAN CONNECTION
CLEAN CONNECTION TO ALL FORCE FOR DATABASE testdb ;

-- 来源: 1243_CREATE DATABASE LINK
CREATE PUBLIC DATABASE LINK public_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

-- 来源: 1243_CREATE DATABASE LINK
DROP PUBLIC DATABASE LINK public_dblink;

-- 来源: 1248_CREATE FUNCTION
\ c ora_compatible_db --定义函数为SQL查询。

--对增量物化视图my_imv进行增量刷新。
-- 来源: 1251_CREATE INCREMENTAL MATERIALIZED VIEW
REFRESH INCREMENTAL MATERIALIZED VIEW my_imv;

--对全量物化视图my_mv进行全量刷新。
-- 来源: 1255_CREATE MATERIALIZED VIEW
REFRESH MATERIALIZED VIEW my_mv;

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING ( role = CURRENT_USER );

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_rls" FOR ALL TO public USING ((( role ):: name = "current_user" ())) Has OIDs : no Distribute By : HASH ( id ) Location Nodes : ALL DATANODES Options : orientation = row , compression = no , enable_rowsecurity = true --当前用户执行SELECT操作。

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
DROP ROW LEVEL SECURITY POLICY all_data_rls ON all_data ;

-- 来源: 1270_CREATE TABLE
\ d + tpcds . warehouse_t21 Table "tpcds.warehouse_t21" Column | Type | Modifiers | Storage | Stats target | Description -------------------+-----------------------+-----------+----------+--------------+------------- w_warehouse_sk | integer | not null | plain | | w_warehouse_id | character ( 16 ) | not null | extended | | w_warehouse_name | character varying ( 20 ) | | extended | | w_warehouse_sq_ft | integer | | plain | | w_street_number | character ( 10 ) | | extended | | w_street_name | character varying ( 60 ) | | extended | | w_street_type | character ( 15 ) | | extended | | w_suite_number | character ( 10 ) | | extended | | w_city | character varying ( 60 ) | | extended | | w_county | character varying ( 30 ) | | extended | | w_state | character ( 2 ) | | extended | | w_zip | character ( 10 ) | | extended | | w_country | character varying ( 20 ) | | extended | | w_gmt_offset | numeric ( 5 , 2 ) | | main | | Has OIDs : no Distribute By : REPLICATION Location Nodes : ALL DATANODES Options : orientation = row , logical_repl_node =- 1 , compression = no , primarynode = on

-- 来源: 1270_CREATE TABLE
\ c ilmtabledb

-- 来源: 1272_CREATE TABLE AS
\ c ilmtabledb --开启数据库ILM特性

-- 来源: 1272_CREATE TABLE AS
\ c postgres

-- 来源: 1311_DROP ROW LEVEL SECURITY POLICY
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING ( role = CURRENT_USER );

-- 来源: 1311_DROP ROW LEVEL SECURITY POLICY
DROP ROW LEVEL SECURITY POLICY all_data_rls ON all_data ;

-- 来源: 1330_EXPDP DATABASE
EXPDP DATABASE test LOCATION = '/data1/expdp/database';

-- 来源: 1331_EXPDP TABLE
EXPDP TABLE test_t LOCATION = '/data1/expdp/table0';

-- 来源: 1335_FETCH
CURSOR cursor1 FOR SELECT * FROM tpcds . customer_address ORDER BY 1 ;

-- 来源: 1335_FETCH
END ;

-- 来源: 1335_FETCH
CURSOR cursor2 FOR VALUES ( 1 , 2 ),( 0 , 3 ) ORDER BY 1 ;

-- 来源: 1335_FETCH
END ;

-- 来源: 1335_FETCH
END ;

-- 来源: 1339_IMPDP DATABASE CREATE
IMPDP DATABASE test CREATE SOURCE = '/data1/impdp/database' OWNER = admin;

-- 来源: 1340_IMPDP RECOVER
IMPDP DATABASE RECOVER SOURCE = '/data1/impdp/database' owner=admin;

-- 来源: 1341_IMPDP TABLE
IMPDP TABLE SOURCE = '/data1/impdp/table0' OWNER=admin;

-- 来源: 1342_IMPDP TABLE PREPARE
IMPDP TABLE PREPARE SOURCE = '/data1/impdp/table0' OWNER=admin;

-- 来源: 1348_MARK BUCKETS
MARK BUCKETS ( 0 , 1 , 2 , 3 ) FINISH FROM datanode1 TO datanode3 ;

-- 来源: 1350_MOVE
CURSOR cursor1 FOR SELECT * FROM tpcds . reason ;

-- 来源: 1350_MOVE
MOVE FORWARD 3 FROM cursor1 ;

-- 来源: 1350_MOVE
END ;

--PURGE清除表。
-- 来源: 1355_PURGE
PURGE TABLE tpcds.reason_t3;

--PURGE清除索引。
-- 来源: 1355_PURGE
PURGE INDEX tpcds.index_t1;

--PURGE清除回收站所有对象。
-- 来源: 1355_PURGE
PURGE recyclebin;

-- 来源: 1357_REASSIGN OWNED
REASSIGN OWNED BY jim TO tom ;

--对增量物化视图my_imv进行增量刷新。
-- 来源: 1358_REFRESH INCREMENTAL MATERIALIZED VIEW
REFRESH INCREMENTAL MATERIALIZED VIEW my_imv;

--对全量物化视图my_mv进行全量刷新。
-- 来源: 1359_REFRESH MATERIALIZED VIEW
REFRESH MATERIALIZED VIEW my_mv;

--对增量物化视图my_imv进行全量刷新。
-- 来源: 1359_REFRESH MATERIALIZED VIEW
REFRESH MATERIALIZED VIEW my_imv;

--值替换插入数据。
-- 来源: 1362_REPLACE
REPLACE INTO test VALUES(1, 11, 11);

--查询替换插入数据。
-- 来源: 1362_REPLACE
REPLACE INTO test SELECT 2, 22, 22;

--设置指定字段替换插入数据。
-- 来源: 1362_REPLACE
REPLACE INTO test SET f1 = f1 + 3, f2 = f1 * 10 + 3, f3 = f2;

-- 来源: 1370_SECURITY LABEL ON
SECURITY LABEL ON ROLE bob IS 'sec_label' ;

-- 来源: 1370_SECURITY LABEL ON
SECURITY LABEL ON TABLE tbl IS 'sec_label' ;

-- 来源: 1370_SECURITY LABEL ON
SECURITY LABEL ON COLUMN tbl . c1 IS 'sec_label' ;

-- 来源: 1370_SECURITY LABEL ON
SECURITY LABEL ON ROLE bob IS NULL ;

-- 来源: 1370_SECURITY LABEL ON
SECURITY LABEL ON TABLE tbl IS NULL ;

-- 来源: 1370_SECURITY LABEL ON
SECURITY LABEL ON COLUMN tbl . c1 IS NULL ;

-- 来源: 1379_SHUTDOWN
SHUTDOWN;

--使用fast模式关闭当前数据库节点。
-- 来源: 1379_SHUTDOWN
SHUTDOWN FAST;

-- 来源: 1380_START TRANSACTION
END ;

-- 来源: 1380_START TRANSACTION
END ;

--执行闪回TRUNCATE。
-- 来源: 1382_TIMECAPSULE TABLE
TIMECAPSULE TABLE tpcds.reason_t2 to BEFORE TRUNCATE;

--执行闪回DROP。
-- 来源: 1382_TIMECAPSULE TABLE
TIMECAPSULE TABLE tpcds.reason_t2 to BEFORE DROP;

-- 清空回收站，删除SCHEMA。
-- 来源: 1382_TIMECAPSULE TABLE
PURGE RECYCLEBIN;

-- 来源: 1425_record
\ d emp_rec Table "public.emp_rec" Column | Type | Modifiers --------+-----------------------+----------- empno | numeric ( 4 , 0 ) | not null ename | character varying ( 10 ) | --演示在函数中对record进行操作。

-- 来源: 1458_file_1458
hr ---1 hr ---1 hr ---1 cursor_proc1 -------------- ( 1 row )

-- 来源: 1460_file_1460
Tom ANONYMOUS BLOCK EXECUTE --创建表

-- 来源: 1468_DBE_COMPRESSION
\ c ilmtabledb

-- 来源: 1468_DBE_COMPRESSION
\ c ilmtabledb

-- 来源: 1470_DBE_ILM
\ c ilmtabledb

-- 来源: 1471_DBE_ILM_ADMIN
DBE_ILM_ADMIN . DISABLE_ILM ();

-- 来源: 1471_DBE_ILM_ADMIN
DBE_ILM_ADMIN . ENABLE_ILM ();

-- 来源: 2317_file_2317
\ q 修改 集群 所有CN 的最大连接数。 gs_guc set -Z coordinator -N all -I all -c "max_connections = 800" 重启 集群 。 gs_om -t stop && gs_om -t start 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看最大连接数。 1 2 3 4

-- 来源: 2317_file_2317
\ q 修改 集群 所有CN 的客户端认证最长时间。 gs_guc reload -Z coordinator -N all -I all -c "authentication_timeout = 59s" 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看客户端认证的最长时间。 1 2 3 4

-- 来源: 2317_file_2317
\ q 修改 集群 所有 CN和DN 的最大连接数。 gs_guc set -Z coordinator -Z datanode -N all -I all -c "max_connections = 500" 重启 集群 。 gs_om -t stop gs_om -t start 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看最大连接数。 1 2 3 4

-- 来源: 2317_file_2317
\ q 修改 集群 所有 CN和DN 的客户端认证最长时间。 gs_guc reload -Z coordinator -Z datanode -N all -I all -c "authentication_timeout = 30s" 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看客户端认证的最长时间。 1 2 3 4

-- 来源: 2442_file_2442
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING ( role = CURRENT_USER );

-- 来源: 2442_file_2442
\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_rls" FOR ALL TO public USING ((( role ):: name = "current_user" ())) Has OIDs : no Location Nodes : ALL DATANODES Options : orientation = row , compression = no , enable_rowsecurity = true --切换至用户alice，执行SQL"SELECT * FROM public.all_data"

-- 来源: 2450_file_2450
gsql ((GaussDB Kernel XXX.X.XXX build f521c606) compiled at 2021-09-16 14:55:22 commit 2935 last mr 6385 release) Non-SSL connection (SSL connection is recommended when requiring high-security) Type "help" for help. db_tpcc=> 查看数据库 使用\l元命令查看数据库系统的数据库列表。

-- 来源: 2450_file_2450
\ l 使用如下命令通过系统表pg_database查询数据库列表。

-- 来源: 2457_file_2457
\ d + customer_t1 ;

-- 来源: 2463_file_2463
\ di + tpcds . tpcds_web_returns_p2_index2 删除索引 1

-- 来源: 2464_file_2464
\ d + MyView View "PG_CATALOG.MyView" Column | Type | Modifiers | Storage | Description ----------+-----------------------+-----------+----------+------------- USERNAME | CHARACTER VARYING ( 64 ) | | extended | View definition : SELECT PG_AUTHID . ROLNAME :: CHARACTER VARYING ( 64 ) AS USERNAME FROM PG_AUTHID ;

-- 来源: 2731_SQL PATCH
\ x --切换扩展显示模式，便于观察计划 Expanded display is on .

-- 来源: 2731_SQL PATCH
\ x Expanded display is off .

-- 来源: 2731_SQL PATCH
\ x Expanded display is on .

-- 来源: 2743_file_2743
\ c gaussdb_m --设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

-- 来源: 2747_file_2747
\ c gaussdb_m --设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

-- 来源: 2755_HLL
\ d t1 Table "public.t1" Column | Type | Modifiers --------+---------+----------- id | integer | set | hll | -- 创建hll类型的表，指定前两个入参，后两个采用默认值。

-- 来源: 2755_HLL
\ d t2 Table "public.t2" Column | Type | Modifiers --------+----------------+----------- id | integer | set | hll ( 12 , 4 , 12 , 0 ) | --创建hll类型的表，指定第三个入参，其余采用默认值。

-- 来源: 2755_HLL
\ d t3 Table "public.t3" Column | Type | Modifiers --------+----------------+----------- id | integer | set | hll ( 14 , 10 , 8 , 0 ) | --创建hll类型的表，指定入参不合法报错。

-- 来源: 2769_file_2769
\ c gaussdb_m -- 设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

-- 来源: 2774_file_2774
\ c testdb3 testdb3 =# SET b_format_dev_version = 's1' ;

-- 来源: 2774_file_2774
\ c testdb3 testdb3 =# SET b_format_dev_version = 's1' ;

-- 来源: 2775_file_2775
\ c gaussdb_m -- 设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

-- 来源: 2821_XML
CURSOR xc WITH HOLD FOR SELECT * FROM testxmlschema . test1 ORDER BY 1 , 2 ;

-- 来源: 2841_UNIONCASE
\ c a_1 --创建表t1。 a_1 =# CREATE TABLE t1 ( a int , b varchar ( 10 ));

-- 来源: 2841_UNIONCASE
\ c td_1 --创建表t2。 td_1 =# CREATE TABLE t2 ( a int , b varchar ( 10 ));

-- 来源: 2871_file_2871
\ dF + ts_conf Text search configuration "public.ts_conf" Parser : "pg_catalog.default" Token | Dictionaries -----------------+------------------------------------- asciihword | gs_dict , english_ispell , english_stem asciiword | gs_dict , english_ispell , english_stem file | simple host | simple hword | gs_dict , english_ispell , english_stem hword_asciipart | gs_dict , english_ispell , english_stem hword_numpart | simple hword_part | gs_dict , english_ispell , english_stem int | simple numhword | simple numword | simple uint | simple version | simple word | gs_dict , english_ispell , english_stem

-- 创建聚合函数。
-- 来源: 2883_ALTER AGGREGATE
CREATE AGGREGATE myavg (int) ( sfunc = int_add, stype = int, initcond = '0' );

--把一个接受integer 类型参数的聚合函数myavg重命名为 my_average 。
-- 来源: 2883_ALTER AGGREGATE
ALTER AGGREGATE myavg(integer) RENAME TO my_average;

--把一个接受integer 类型参数的聚合函数myavg的所有者改为joe 。
-- 来源: 2883_ALTER AGGREGATE
ALTER AGGREGATE my_average(integer) OWNER TO joe;

--把一个接受integer 类型参数的聚合函数myavg移动到模式myschema里。
-- 来源: 2883_ALTER AGGREGATE
ALTER AGGREGATE my_average(integer) SET SCHEMA myschema;

--创建公共dblink
-- 来源: 2886_ALTER DATABASE LINK
CREATE PUBLIC DATABASE LINK public_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

-- 修改dblink对象信息
-- 来源: 2886_ALTER DATABASE LINK
ALTER PUBLIC DATABASE LINK public_dblink CONNECT TO 'user2' IDENTIFIED BY '********';

--删除公共dblink
-- 来源: 2886_ALTER DATABASE LINK
DROP PUBLIC DATABASE LINK public_dblink;

--修改名称。
-- 来源: 2887_ALTER DATA SOURCE
ALTER DATA SOURCE ds_test1 RENAME TO ds_test;

-- 来源: 2887_ALTER DATA SOURCE
ALTER DATA SOURCE ds_test OWNER TO user_test1;

--修改TYPE和VERSION。
-- 来源: 2887_ALTER DATA SOURCE
ALTER DATA SOURCE ds_test TYPE 'MPPDB_TYPE' VERSION 'XXX';

--添加字段。
-- 来源: 2887_ALTER DATA SOURCE
ALTER DATA SOURCE ds_test OPTIONS (add dsn ' gaussdb ', username 'test_user');

--修改字段。
-- 来源: 2887_ALTER DATA SOURCE
ALTER DATA SOURCE ds_test OPTIONS (set dsn 'unknown');

--删除字段。
-- 来源: 2887_ALTER DATA SOURCE
ALTER DATA SOURCE ds_test OPTIONS (drop username);

-- 用函数名重编译函数
-- 来源: 2893_ALTER FUNCTION
ALTER PROCEDURE test_func COMPILE;

-- 用函数带类型签名重编译存储过程
-- 来源: 2893_ALTER FUNCTION
ALTER PROCEDURE test_func(int) COMPILE;

-- 来源: 2901_ALTER OPERATOR
ALTER OPERATOR @@ (text, text) OWNER TO omm;

-- 用存储过程名重编译存储过程
-- 来源: 2904_ALTER PROCEDURE
ALTER PROCEDURE test_proc COMPILE;

-- 用存储过程带类型签名重编译存储过程
-- 来源: 2904_ALTER PROCEDURE
ALTER PROCEDURE test_proc(int) COMPILE;

--创建行访问控制策略，当前用户只能查看用户自身的数据。
-- 来源: 2908_ALTER ROW LEVEL SECURITY POLICY
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING(role = CURRENT_USER);

--修改行访问控制all_data_rls的名称。
-- 来源: 2908_ALTER ROW LEVEL SECURITY POLICY
ALTER ROW LEVEL SECURITY POLICY all_data_rls ON all_data RENAME TO all_data_new_rls;

--修改行访问控制策略影响的用户。
-- 来源: 2908_ALTER ROW LEVEL SECURITY POLICY
ALTER ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data TO alice, bob;

--修改行访问控制策略表达式。
-- 来源: 2908_ALTER ROW LEVEL SECURITY POLICY
ALTER ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data USING (id > 100 AND role = current_user);

--删除访问控制策略。
-- 来源: 2908_ALTER ROW LEVEL SECURITY POLICY
DROP ROW LEVEL SECURITY POLICY all_data_new_rls ON all_data;

-- 来源: 2933_CLEAN CONNECTION
CLEAN CONNECTION TO ALL FOR DATABASE template1 TO USER jack ;

-- 来源: 2933_CLEAN CONNECTION
CLEAN CONNECTION TO ALL TO USER jack ;

-- 来源: 2933_CLEAN CONNECTION
CLEAN CONNECTION TO ALL FORCE FOR DATABASE test_clean_connection ;

-- 创建聚合函数
-- 来源: 2940_CREATE AGGREGATE
CREATE AGGREGATE sum (int) ( sfunc = int_add, stype = int, initcond = '0' );

-- 删除聚合函数
-- 来源: 2940_CREATE AGGREGATE
DROP AGGREGATE sum(int);

--创建类型转换
-- 来源: 2942_CREATE CAST
CREATE CAST(double precision AS timestamp with time zone) WITH FUNCTION double_to_timestamp(double precision) AS IMPLICIT;

--删除类型转换
-- 来源: 2942_CREATE CAST
DROP CAST (double precision AS timestamp with time zone);

--创建公共dblink
-- 来源: 2945_CREATE DATABASE LINK
CREATE PUBLIC DATABASE LINK public_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

--删除公共dblink
-- 来源: 2945_CREATE DATABASE LINK
DROP PUBLIC DATABASE LINK public_dblink;

--创建一个执行一次的定时任务。
-- 来源: 2948_CREATE EVENT
CREATE EVENT IF NOT EXISTS event_e1 ON SCHEDULE AT sysdate + interval 5 second + interval 33 minute DISABLE DO insert into t_ev values(0);

--创建一个每隔一分钟执行一次的定时任务。
-- 来源: 2948_CREATE EVENT
CREATE EVENT IF NOT EXISTS event_e2 ON SCHEDULE EVERY 1 minute DO insert into t_ev values(1);

--修改定时任务状态和待执行语句。
-- 来源: 2948_CREATE EVENT
ALTER EVENT event_e1 ENABLE DO select 1;

--修改定时任务名。
-- 来源: 2948_CREATE EVENT
ALTER EVENT event_e1 RENAME TO event_ee;

--删除定时任务。
-- 来源: 2948_CREATE EVENT
DROP EVENT event_e1;

-- 来源: 2948_CREATE EVENT
DROP EVENT event_e2;

--对增量物化视图my_imv进行增量刷新。
-- 来源: 2954_CREATE INCREMENTAL MATERIALIZED VIEW
REFRESH INCREMENTAL MATERIALIZED VIEW my_imv;

--对全量物化视图my_mv进行全量刷新。
-- 来源: 2958_CREATE MATERIALIZED VIEW
REFRESH MATERIALIZED VIEW my_mv;

--创建一个发布，发布两个表中所有更改。
-- 来源: 2964_CREATE PUBLICATION
CREATE PUBLICATION mypublication FOR TABLE users, departments;

--创建一个发布，发布所有表中的所有更改。
-- 来源: 2964_CREATE PUBLICATION
CREATE PUBLICATION alltables FOR ALL TABLES;

--创建一个发布，只发布一个表中的INSERT操作。
-- 来源: 2964_CREATE PUBLICATION
CREATE PUBLICATION insert_only FOR TABLE mydata WITH (publish = 'insert');

--修改发布的动作。
-- 来源: 2964_CREATE PUBLICATION
ALTER PUBLICATION insert_only SET (publish='insert,update,delete');

--向发布中添加表。
-- 来源: 2964_CREATE PUBLICATION
ALTER PUBLICATION insert_only ADD TABLE mydata2;

--删除发布。
-- 来源: 2964_CREATE PUBLICATION
DROP PUBLICATION insert_only;

-- 来源: 2964_CREATE PUBLICATION
DROP PUBLICATION alltables;

-- 来源: 2964_CREATE PUBLICATION
DROP PUBLICATION mypublication;

--创建行访问控制策略，当前用户只能查看用户自身的数据。
-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING(role = CURRENT_USER);

--删除行访问控制策略。
-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
DROP ROW LEVEL SECURITY POLICY all_data_rls ON all_data;

--创建RULE def_view_test_ins。
-- 来源: 2969_CREATE RULE
CREATE RULE def_view_test_ins AS ON INSERT TO def_view_test DO INSTEAD INSERT INTO def_test SELECT new.*;

--删除RULE def_view_test_ins。
-- 来源: 2969_CREATE RULE
DROP RULE def_view_test_ins ON def_view_test;

--创建一个发布，发布两个表中所有更改。
-- 来源: 2974_CREATE SUBSCRIPTION
CREATE PUBLICATION mypublication FOR TABLE users, departments;

--创建一个发布，只发布一个表中的INSERT操作。
-- 来源: 2974_CREATE SUBSCRIPTION
CREATE PUBLICATION insert_only FOR TABLE mydata WITH (publish = 'insert');

--创建一个到远程服务器的订阅，复制发布mypublication和insert_only中的表，并在提交时立即开始复制。其中ip和port为发布端的ip地址和端口，user、dbname、password为发布端数据库的用户信息。
-- 来源: 2974_CREATE SUBSCRIPTION
CREATE SUBSCRIPTION mysub CONNECTION 'host=192.168.11.11 port=5432 user=foo dbname=foodb password=********' PUBLICATION mypublication, insert_only;

--创建一个到远程服务器的订阅，复制insert_only发布中的表， 并且不开始复制直到稍后启用复制。其中ip和port为发布端的ip地址和端口，user、dbname、password为发布端数据库的用户信息。
-- 来源: 2974_CREATE SUBSCRIPTION
CREATE SUBSCRIPTION mysub CONNECTION 'host=192.168.11.11 port=5432 user=foo dbname=foodb password=********' PUBLICATION insert_only WITH (enabled = false);

--修改订阅的连接信息。其中ip和port为发布端的ip地址和端口，user、dbname、password为发布端数据库的用户信息。
-- 来源: 2974_CREATE SUBSCRIPTION
ALTER SUBSCRIPTION mysub CONNECTION 'host=192.168.11.11 port=5432 user=foo dbname=foodb password=********';

--激活订阅。
-- 来源: 2974_CREATE SUBSCRIPTION
ALTER SUBSCRIPTION mysub SET(enabled=true);

--删除订阅。
-- 来源: 2974_CREATE SUBSCRIPTION
DROP SUBSCRIPTION mysub;

--删除发布。
-- 来源: 2974_CREATE SUBSCRIPTION
DROP PUBLICATION insert_only;

-- 来源: 2974_CREATE SUBSCRIPTION
DROP PUBLICATION mypublication;

-- 创建聚合函数。
-- 来源: 2995_DROP AGGREGATE
CREATE AGGREGATE myavg (int) ( sfunc = int_add, stype = int, initcond = '0' );

--将integer类型的聚合函数myavg删除。
-- 来源: 2995_DROP AGGREGATE
DROP AGGREGATE myavg(integer);

--创建行访问控制策略。
-- 来源: 3020_DROP ROW LEVEL SECURITY POLICY
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING(role = CURRENT_USER);

--删除行访问控制策略。
-- 来源: 3020_DROP ROW LEVEL SECURITY POLICY
DROP ROW LEVEL SECURITY POLICY all_data_rls ON all_data;

--创建RULE def_view_test_ins
-- 来源: 3021_DROP RULE
CREATE RULE def_view_test_ins AS

-- 来源: 3021_DROP RULE
ON INSERT TO def_view_test

--删除RULE def_view_test_ins
-- 来源: 3021_DROP RULE
DROP RULE def_view_test_ins ON def_view_test;

-- 来源: 3041_EXPDP DATABASE
EXPDP DATABASE test LOCATION = '/data1/expdp/database';

-- 来源: 3042_EXPDP TABLE
EXPDP TABLE test_t LOCATION = '/data1/expdp/table0';

--建立一个名为cursor1的游标。
-- 来源: 3046_FETCH
CURSOR cursor1 FOR SELECT * FROM tpcds. customer_address ORDER BY 1;

--建立一个名为cursor2的游标。
-- 来源: 3046_FETCH
CURSOR cursor2 FOR VALUES(1,2),(0,3) ORDER BY 1;

-- 来源: 3050_IMPDP DATABASE CREATE
IMPDP DATABASE test CREATE SOURCE = '/data1/impdp/database' OWNER=admin;

-- 来源: 3051_IMPDP RECOVER
IMPDP DATABASE RECOVER SOURCE = '/data1/impdp/database' OWNER=admin;

-- 来源: 3052_IMPDP TABLE
IMPDP TABLE SOURCE = '/data1/impdp/table0' OWNER=admin;

-- 来源: 3053_IMPDP TABLE PREPARE
IMPDP TABLE PREPARE SOURCE = '/data1/impdp/table0' OWNER=admin;

--定义一个名为cursor1的游标。
-- 来源: 3061_MOVE
CURSOR cursor1 FOR SELECT * FROM tpcds. reason;

--忽略游标cursor1的前3行。
-- 来源: 3061_MOVE
MOVE FORWARD 3 FROM cursor1;

--PURGE清除表
-- 来源: 3066_PURGE
PURGE TABLE tpcds.reason_t3;

--PURGE清除索引
-- 来源: 3066_PURGE
PURGE INDEX tpcds.index_t1;

--PURGE清除回收站所有对象
-- 来源: 3066_PURGE
PURGE recyclebin;

-- 来源: 3068_REASSIGN OWNED
REASSIGN OWNED BY jim TO tom ;

--对增量物化视图my_imv进行增量刷新。
-- 来源: 3069_REFRESH INCREMENTAL MATERIALIZED VIEW
REFRESH INCREMENTAL MATERIALIZED VIEW my_imv;

--对全量物化视图my_mv进行全量刷新。
-- 来源: 3070_REFRESH MATERIALIZED VIEW
REFRESH MATERIALIZED VIEW my_mv;

--对增量物化视图my_imv进行全量刷新。
-- 来源: 3070_REFRESH MATERIALIZED VIEW
REFRESH MATERIALIZED VIEW my_imv;

--值替换插入数据。
-- 来源: 3073_REPLACE
REPLACE INTO test VALUES(1, 11, 11);

--查询替换插入数据。
-- 来源: 3073_REPLACE
REPLACE INTO test SELECT 2, 22, 22;

--设置指定字段替换插入数据。
-- 来源: 3073_REPLACE
REPLACE INTO test SET f1 = f1 + 3, f2 = f1 * 10 + 3, f3 = f2;

-- 来源: 3081_SECURITY LABEL ON
SECURITY LABEL ON ROLE bob IS 'sec_label' ;

-- 来源: 3081_SECURITY LABEL ON
SECURITY LABEL ON TABLE tbl IS 'sec_label' ;

-- 来源: 3081_SECURITY LABEL ON
SECURITY LABEL ON COLUMN tbl . c1 IS 'sec_label' ;

-- 来源: 3081_SECURITY LABEL ON
SECURITY LABEL ON ROLE bob IS NULL ;

-- 来源: 3081_SECURITY LABEL ON
SECURITY LABEL ON TABLE tbl IS NULL ;

-- 来源: 3081_SECURITY LABEL ON
SECURITY LABEL ON COLUMN tbl . c1 IS NULL ;

--shrink整理
-- 来源: 3091_SHRINK
SHRINK TABLE row_compression;

-- 来源: 3092_SHUTDOWN
SHUTDOWN;

--使用fast模式关闭当前数据库节点。
-- 来源: 3092_SHUTDOWN
SHUTDOWN FAST;

-- 来源: 3093_SNAPSHOT
CREATE SNAPSHOT s1@1.0 comment is 'first version' AS SELECT * FROM t1;

-- 来源: 3093_SNAPSHOT
CREATE SNAPSHOT s1@2.0 FROM @1.0 comment is 'inherits from @1.0' USING (INSERT VALUES(6, 'john'), (7, 'tim');

-- 来源: 3093_SNAPSHOT
SAMPLE SNAPSHOT s1@2.0 stratify by name as nick at ratio .5;

-- 来源: 3093_SNAPSHOT
PURGE SNAPSHOT s1@2.0;

-- 来源: 3093_SNAPSHOT
PURGE SNAPSHOT s1nick@2.0;

-- 来源: 3093_SNAPSHOT
PURGE SNAPSHOT s1@1.0;

--执行闪回TRUNCATE
-- 来源: 3096_TIMECAPSULE TABLE
TIMECAPSULE TABLE tpcds.reason_t2 to BEFORE TRUNCATE;

--执行闪回DROP
-- 来源: 3096_TIMECAPSULE TABLE
TIMECAPSULE TABLE tpcds.reason_t2 to BEFORE DROP;

-- 清空回收站，删除SCHEMA。
-- 来源: 3096_TIMECAPSULE TABLE
PURGE RECYCLEBIN;

-- 来源: 3135_record
\ d emp_rec Table "public.emp_rec" Column | Type | Modifiers --------+-----------------------+----------- empno | numeric ( 4 , 0 ) | not null ename | character varying ( 10 ) | --演示在函数中对record进行操作。

-- 来源: 3168_file_3168
hr ---1 hr ---1 hr ---1 cursor_proc1 -------------- ( 1 row )

-- 来源: 3168_file_3168
1 1 ANONYMOUS BLOCK EXECUTE --删除存储过程

-- 来源: 3170_file_3170
Tom ANONYMOUS BLOCK EXECUTE --创建表

-- 来源: 3182_DBE_ILM_ADMIN
DBE_ILM_ADMIN . DISABLE_ILM ();

-- 来源: 3182_DBE_ILM_ADMIN
DBE_ILM_ADMIN . ENABLE_ILM ();

-- 来源: 3978_file_3978
\ q 修改 GaussDB 数据库主节点的最大连接数。 gs_guc set -Z datanode -N all -I all -c "max_connections = 800" 重启 数据库 。 gs_om -t stop && gs_om -t start 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看最大连接数。 1 2 3 4

-- 来源: 3978_file_3978
\ q 修改数据库主节点的客户端认证最长时间。 gs_guc reload -Z datanode -N all -I all -c "authentication_timeout = 59s" 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看客户端认证的最长时间。 1 2 3 4

-- 来源: 3978_file_3978
\ q 修改 GaussDB 数据库节点的最大连接数。 gs_guc set -Z datanode -N all -I all -c "max_connections = 500" 重启 数据库 。 gs_om -t stop gs_om -t start 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看最大连接数。 1 2 3 4

-- 来源: 3978_file_3978
\ q 修改 GaussDB 数据库节点的客户端认证最长时间。 gs_guc reload -Z datanode -N all -I all -c "authentication_timeout = 30s" 连接数据库，具体操作请参考《开发者指南》中“数据库使用入门 > 连接数据库 > 使用gsql连接”章节。 查看客户端认证的最长时间。 1 2 3 4

-- 对全量物化视图做全量刷新
-- 来源: 4273_file_4273
REFRESH MATERIALIZED VIEW mv;

-- 增量刷新物化视图
-- 来源: 4277_file_4277
REFRESH INCREMENTAL MATERIALIZED VIEW mv;

-- 全量刷新物化视图
-- 来源: 4277_file_4277
REFRESH MATERIALIZED VIEW mv;

-- 来源: 4280_gsql
CREATE CLIENT MASTER KEY cmk1 WITH ( KEY_STORE = hcs_kms , KEY_PATH = '{KMS服务器地址}/{密钥ID}', ALGORITHM = AES_256);

-- 来源: 4280_gsql
CREATE COLUMN ENCRYPTION KEY cek1 WITH VALUES (CLIENT_MASTER_KEY = cmk1, ALGORITHM = AES_256_GCM);

-- 删除列密钥
-- 来源: 4280_gsql
DROP COLUMN ENCRYPTION KEY cek1;

-- 删除主密钥
-- 来源: 4280_gsql
DROP CLIENT MASTER KEY cmk1;

-- 来源: 4287_file_4287
SECURITY LABEL ON USER user1 is 'label1' ;

-- 来源: 4287_file_4287
SECURITY LABEL ON USER user2 is 'label3' ;

-- 来源: 4287_file_4287
SECURITY LABEL ON TABLE tbl is 'label2' ;

-- 来源: 4394_file_4394
TIMECAPSULE TABLE flashtest TO CSN 79352065;

-- 来源: 4394_file_4394
TIMECAPSULE TABLE flashtest TO TIMESTAMP to_timestamp ('2023-09-13 19:52:21.551028', 'YYYY-MM-DD HH24:MI:SS.FF');

-- 来源: 4394_file_4394
TIMECAPSULE TABLE flashtest TO TIMESTAMP '2023-09-13 19:54:00.641506';

-- 来源: 4395_DROP_TRUNCATE
PURGE TABLE flashtest;

-- 来源: 4395_DROP_TRUNCATE
PURGE index flashtest_index;

-- 来源: 4395_DROP_TRUNCATE
PURGE RECYCLEBIN;

-- 来源: 4395_DROP_TRUNCATE
timecapsule table flashtest to before drop;

-- 来源: 4395_DROP_TRUNCATE
timecapsule table "BIN$31C14EB48DC$9B4E$0==$0" to before drop;

-- 来源: 4395_DROP_TRUNCATE
timecapsule table flashtest to before drop rename to flashtest_rename;

-- 来源: 4395_DROP_TRUNCATE
PURGE RECYCLEBIN;

-- 来源: 4395_DROP_TRUNCATE
timecapsule table flashtest to before truncate;

-- 来源: 4395_DROP_TRUNCATE
PURGE RECYCLEBIN;

-- 来源: 4407_file_4407
List of relations Schema | Name | Type | Owner | Storage

-- 来源: 4409_TIPS
DBE_ILM.STOP_ILM (task_id => V_TASK, p_drop_running_Jobs => FALSE, p_Jobname => V_JOBNAME);

-- 来源: 4409_TIPS
DBE_COMPRESSION.GET_COMPRESSION_RATIO ( scratchtbsname IN VARCHAR2, ownname IN VARCHAR2, objname IN VARCHAR2, subobjname IN VARCHAR2, comptype IN NUMBER, blkcnt_cmp OUT PLS_INTEGER, blkcnt_uncmp OUT PLS_INTEGER, row_cmp OUT PLS_INTEGER, row_uncmp OUT PLS_INTEGER, cmp_ratio OUT NUMBER, comptype_str OUT VARCHAR2, sample_ratio IN INTEGER DEFAULT 20, objtype IN PLS_INTEGER DEFAULT OBJTYPE_TABLE);

-- 来源: 4409_TIPS
DBE_HEAT_MAP.ROW_HEAT_MAP( owner IN VARCHAR2, segment_name IN VARCHAR2, partition_name IN VARCHAR2 DEFAULT NULL, ctid IN VARCHAR2,);

--对全量物化视图做全量刷新。
-- 来源: 4493_file_4493
REFRESH MATERIALIZED VIEW mv;

--增量刷新物化视图。
-- 来源: 4497_file_4497
REFRESH INCREMENTAL MATERIALIZED VIEW mv;

--全量刷新物化视图。
-- 来源: 4497_file_4497
REFRESH MATERIALIZED VIEW mv;

-- 来源: 4500_gsql
CREATE CLIENT MASTER KEY cmk1 WITH ( KEY_STORE = hcs_kms , KEY_PATH = '{KMS服务器地址}/{密钥ID}', ALGORITHM = AES_256);

-- 来源: 4500_gsql
CREATE COLUMN ENCRYPTION KEY cek1 WITH VALUES (CLIENT_MASTER_KEY = cmk1, ALGORITHM = AES_256_GCM);

-- 删除列密钥
-- 来源: 4500_gsql
DROP COLUMN ENCRYPTION KEY cek1;

-- 删除主密钥
-- 来源: 4500_gsql
DROP CLIENT MASTER KEY cmk1;

-- 来源: 4507_gsql
CREATE CLIENT MASTER KEY cmk1 WITH ( KEY_STORE = hcs_kms , KEY_PATH = '{KMS服务器地址}/{密钥ID}', ALGORITHM = AES_256);

-- 来源: 4507_gsql
CREATE COLUMN ENCRYPTION KEY cek1 WITH VALUES (CLIENT_MASTER_KEY = cmk1, ALGORITHM = AES_256_GCM);

-- 来源: 4507_gsql
DROP COLUMN ENCRYPTION KEY cek1;

-- 来源: 4507_gsql
DROP CLIENT MASTER KEY cmk1;

-- 来源: 4514_file_4514
SECURITY LABEL ON USER user1 is 'label1' ;

-- 来源: 4514_file_4514
SECURITY LABEL ON USER user2 is 'label3' ;

-- 来源: 4514_file_4514
SECURITY LABEL ON TABLE tbl is 'label2' ;

-- 来源: 4522_DDL
GAINT ALL PRIVILEGES to u01;

-- 来源: 4522_DDL
gs_guc set -Z datanode -D $node_dir -c "wal_level = logical" 其中，$node_dir为数据库节点路径，用户可根据实际情况替换。 使用如下命令连接数据库。

-- 来源: 4522_DDL
gsql -d gaussdb -p 20000 -r 其中，gaussdb为需要连接的数据库名称，20000为数据库端口号，用户可根据实际情况替换。 创建名称为slot1的逻辑复制槽。 1 2 3 4

-- 来源: 4654_file_4654
TIMECAPSULE TABLE flashtest TO TIMESTAMP to_timestamp ('2023-09-13 19:52:21.551028', 'YYYY-MM-DD HH24:MI:SS.FF');

-- 来源: 4654_file_4654
TIMECAPSULE TABLE flashtest TO TIMESTAMP '2023-09-13 19:54:00.641506';

-- 来源: 4655_DROP_TRUNCATE
PURGE TABLE flashtest;

-- 来源: 4655_DROP_TRUNCATE
PURGE index flashtest_index;

-- 来源: 4655_DROP_TRUNCATE
PURGE RECYCLEBIN;

-- 来源: 4655_DROP_TRUNCATE
timecapsule table flashtest to before drop;

-- 来源: 4655_DROP_TRUNCATE
timecapsule table "BIN$31C14EB48DC$9B4E$0==$0" to before drop;

-- 来源: 4655_DROP_TRUNCATE
timecapsule table flashtest to before drop rename to flashtest_rename;

-- 来源: 4655_DROP_TRUNCATE
PURGE RECYCLEBIN;

-- 来源: 4655_DROP_TRUNCATE
timecapsule table flashtest to before truncate;

-- 来源: 4655_DROP_TRUNCATE
PURGE RECYCLEBIN;

-- 来源: 4667_file_4667
List of relations Schema | Name | Type | Owner | Storage

-- 来源: 4669_TIPS
DBE_ILM.STOP_ILM (task_id => V_TASK, p_drop_running_Jobs => FALSE, p_Jobname => V_JOBNAME);

-- 来源: 4669_TIPS
DBE_COMPRESSION.GET_COMPRESSION_RATIO ( scratchtbsname IN VARCHAR2, ownname IN VARCHAR2, objname IN VARCHAR2, subobjname IN VARCHAR2, comptype IN NUMBER, blkcnt_cmp OUT PLS_INTEGER, blkcnt_uncmp OUT PLS_INTEGER, row_cmp OUT PLS_INTEGER, row_uncmp OUT PLS_INTEGER, cmp_ratio OUT NUMBER, comptype_str OUT VARCHAR2, sample_ratio IN INTEGER DEFAULT 20, objtype IN PLS_INTEGER DEFAULT OBJTYPE_TABLE);

-- 来源: 4669_TIPS
DBE_HEAT_MAP.ROW_HEAT_MAP( owner IN VARCHAR2, segment_name IN VARCHAR2, partition_name IN VARCHAR2 DEFAULT NULL, ctid IN VARCHAR2,);

-- 来源: 5778_gsql
\ set foo bar 要引用变量的值，在变量前面加冒号。例如查看变量的值： 1

-- 来源: 5778_gsql
\ echo : foo bar 这种变量的引用方法适用于规则的SQL语句和除\copy、\ef、\help、\sf、\!以外的元命令。 gsql预定义了一些特殊变量，同时也规划了变量的取值。为了保证和后续版本最大限度地兼容，请避免以其他目的使用这些变量。所有特殊变量见 表2 。 所有特殊变量都由大写字母、数字和下划线组成。 要查看特殊变量的默认值，请使用元命令 \echo : varname （例如\echo :DBNAME）。 表2 特殊变量设置 变量 设置方法 变量说明 DBNAME \set DBNAME dbname 当前连接的数据库的名称。每次连接数据库时都会被重新设置。 ECHO \set ECHO all | queries 如果设置为all，只显示查询信息。等效于使用gsql连接数据库时指定-a参数。 如果设置为queries，显示命令行和查询信息。等效于使用gsql连接数据库时指定-e参数。 ECHO_HIDDEN \set ECHO_HIDDEN on | off | noexec 当使用元命令查询数据库信息（例如\dg）时，此变量的取值决定了查询的行为： 设置为on，先显示元命令实际调用的查询语句，然后显示查询结果。等效于使用gsql连接数据库时指定-E参数。 设置为off，则只显示查询结果。 设置为noexec，则只显示查询信息，不执行查询操作。 ENCODING \set ENCODING encoding 当前客户端的字符集编码。 FETCH_COUNT \set FETCH_COUNT variable 如果该变量的值为大于0的整数，假设为n，则执行SELECT语句时每次从结果集中取n行到缓存并显示到屏幕。 如果不设置此变量，或设置的值小于等于0，则执行SELECT语句时一次性把结果都取到缓存。 说明： 设置合理的变量值，将减少内存使用量。一般来说，设为100到1000之间的值比较合理。 HISTCONTROL \set HISTCONTROL ignorespace | ignoredups | ignoreboth | none ignorespace：以空格开始的行将不会写入历史列表。 ignoredups：与以前历史记录里匹配的行不会写入历史记录。 ignoreboth、none或者其他值：所有以交互模式读入的行都被保存到历史列表。 说明： none表示不设置HISTCONTROL。 HISTFILE \set HISTFILE filename 此文件用于存储历史名列表。缺省值是~/.bash_history。 HISTSIZE \set HISTSIZE size 保存在历史命令里命令的个数。缺省值是500。 HOST \set HOST hostname 已连接的数据库主机名称。 IGNOREEOF \set IGNOREEOF variable 若设置此变量为数值，假设为10，则在gsql中输入的前9次EOF字符（通常是Ctrl+C）都会被忽略，在第10次按Ctrl+C才能退出gsql程序。 若设置此变量为非数值，则缺省为10。 若删除此变量，则向交互的gsql会话发送一个EOF终止应用。 LASTOID \set LASTOID oid 最后影响的oid值，即为从一条INSERT或lo_import命令返回的值。此变量只保证在下一条SQL语句的结果显示之前有效。 ON_ERROR_ROLLBACK \set ON_ERROR_ROLLBACK on | interactive | off 如果是on，当一个事务块里的语句产生错误的时候，这个错误将被忽略而事务继续。 如果是interactive，这样的错误只是在交互的会话里忽略。 如果是off（缺省），事务块里一个语句生成的错误将会回滚整个事务。on_error_rollback-on模式是通过在一个事务块的每个命令前隐含地发出一个SAVEPOINT的方式来工作的，在发生错误的时候回滚到该事务块。 ON_ERROR_STOP \set ON_ERROR_STOP on | off on：命令执行错误时会立即停止，在交互模式下，gsql会立即返回已执行命令的结果。 off（缺省）：命令执行错误时将会跳过错误继续执行。 PORT \set PORT port 正连接数据库的端口号。 USER \set USER username 当前用于连接的数据库用户。 VERBOSITY \set VERBOSITY terse | default | verbose 这个选项可以设置为值terse、default、verbose之一以控制错误报告的冗余行。 terse：仅返回严重且主要的错误文本以及文本位置（一般适合于单行错误信息）。 default：返回严重且主要的错误文本及其位置，还包括详细的错误细节、错误提示（可能会跨越多行）。 verbose：返回所有的错误信息。 SQL代换 像元命令的参数一样，gsql变量的一个关键特性是可以把gsql变量替换成正规的SQL语句。此外，gsql还提供为变量更换新的别名或其他标识符等功能。使用SQL代换方式替换一个变量的值可在变量前加冒号。例如： 1 2 3 4 5 6 7 8

-- 来源: 5778_gsql
\ set foo 'HR.areaS'

-- 来源: 5778_gsql
\ set PROMPT2 TEST

-- 来源: 5778_gsql
\ set PROMPT3 '>>>>'

-- 来源: 5779_file_5779
\ l List of databases Name | Owner | Encoding | Collate | Ctype | Access privileges ----------------+----------+-----------+---------+-------+----------------------- human_resource | omm | SQL_ASCII | C | C | postgres | omm | SQL_ASCII | C | C | template0 | omm | SQL_ASCII | C | C | = c / omm + | | | | | omm = CTc / omm template1 | omm | SQL_ASCII | C | C | = c / omm + | | | | | omm = CTc / omm human_staff | omm | SQL_ASCII | C | C | ( 5 rows ) 更多gsql元命令请参见 元命令参考 。 示例 以把一个查询分成多行输入为例。注意提示符的变化： 1 2 3 4

-- 来源: 5779_file_5779
\ d HR . areaS Table "hr.areas" Column | Type | Modifiers -----------+-----------------------+----------- area_id | numeric | not null area_name | character varying ( 25 ) | 向HR.areaS表插入四行数据： 1 2 3 4 5 6 7

-- 来源: 5779_file_5779
\ set PROMPT1 '%n@%m %~%R%#' omm @ [ local ]

-- 来源: 5779_file_5779
查看表： 1 2 3 4 5 6 7 8 omm @ [ local ]

-- 来源: 5779_file_5779
\ pset border 2 Border style is 2 . omm @ [ local ]

-- 来源: 5779_file_5779
\ pset border 0 Border style is 0 . omm @ [ local ]

-- 来源: 5779_file_5779
\ a \ t \ x Output format is unaligned . Showing only tuples . Expanded display is on . omm @ [ local ]

-- 来源: 5779_file_5779
父主题： gsql

-- 来源: 5780_file_5780
\ h Available help : ABORT ALTER AGGREGATE ALTER APP WORKLOAD GROUP ... ... 例如，查看CREATE DATABASE命令的参数可使用下面的命令： 1 2 3 4 5 6 7 8 9 10 11 12

-- 来源: 5780_file_5780
\ help CREATE DATABASE Command : CREATE DATABASE Description : create a new database Syntax : CREATE DATABASE database_name [ [ WITH ] { [ OWNER [ = ] user_name ] | [ TEMPLATE [ = ] template ] | [ ENCODING [ = ] encoding ] | [ LC_COLLATE [ = ] lc_collate ] | [ LC_CTYPE [ = ] lc_ctype ] | [ DBCOMPATIBILITY [ = ] compatibility_type ] | [ TABLESPACE [ = ] tablespace_name ] | [ CONNECTION LIMIT [ = ] connlimit ] } [...] ];

-- 来源: 5780_file_5780
\ ? General \ copyright show GaussDB Kernel usage and distribution terms \ g [ FILE ] or ;

-- 来源: 5782_file_5782
\ d \d[S+] NAME 列出指定表、视图和索引的结构。 - 假设存在表a，列出指定表a的结构。

-- 来源: 5782_file_5782
\ dtable + a \d+ [PATTERN] 列出所有表、视图和索引。 如果声明了PATTERN，只显示名称匹配PATTERN的表、视图和索引。 列出所有名称以f开头的表、视图和索引。

-- 来源: 5782_file_5782
\ d + f * \da[S] [PATTERN] 列出所有可用的聚集函数，以及它们操作的数据类型和返回值类型。 如果声明了PATTERN，只显示名称匹配PATTERN的聚集函数。 列出所有名称以f开头可用的聚集函数，以及它们操作的数据类型和返回值类型。

-- 来源: 5782_file_5782
\ da f * \db[+] [PATTERN] 列出所有可用的表空间。 如果声明了PATTERN，只显示名称匹配PATTERN的表空间。 列出所有名称以p开头的可用表空间。

-- 来源: 5782_file_5782
\ db p * \dc[S+] [PATTERN] 列出所有字符集之间的可用转换。 如果声明了PATTERN，只显示名称匹配PATTERN的转换。 列出所有字符集之间的可用转换。

-- 来源: 5782_file_5782
\ dc * \dC[+] [PATTERN] 列出所有类型转换。 PATTERN需要使用实际类型名，不能使用别名。 如果声明了PATTERN，只显示名称匹配PATTERN的转换。 列出所有名称以c开头的类型转换。

-- 来源: 5782_file_5782
\ dC c * \dd[S] [PATTERN] 显示所有匹配PATTERN的描述。 如果没有给出参数，则显示所有可视对象。“对象”包括：聚集、函数、操作符、类型、关系(表、视图、索引、序列、大对象)、规则。 列出所有可视对象。

-- 来源: 5782_file_5782
\ dd \ddp [PATTERN] 显示所有默认的使用权限。 如果指定了PATTERN，只显示名称匹配PATTERN的使用权限。 列出所有默认的使用权限。

-- 来源: 5782_file_5782
\ ddp \dD[S+] [PATTERN] 列出所有可用域。 如果声明了PATTERN，只显示名称匹配PATTERN的域。 列出所有可用域。

-- 来源: 5782_file_5782
\ dD \det[+] [PATTERN] 列出所有的外部表。 如果声明了PATTERN，只显示名称匹配PATTERN的表。 列出所有的外部表。

-- 来源: 5782_file_5782
\ det \des[+] [PATTERN] 列出所有的外部服务器。 如果声明了PATTERN，只显示名称匹配PATTERN的服务器。 列出所有的外部服务器。

-- 来源: 5782_file_5782
\ des \deu[+] [PATTERN] 列出用户映射信息。 如果声明了PATTERN，只显示名称匹配PATTERN的信息。 列出用户映射信息。

-- 来源: 5782_file_5782
\ deu \dew[+] [PATTERN] 列出封装的外部数据。 如果声明了PATTERN，只显示名称匹配PATTERN的数据。 列出封装的外部数据。

-- 来源: 5782_file_5782
\ dew \df[antw][S+] [PATTERN] 列出所有可用函数，以及它们的参数和返回的数据类型。a代表聚集函数，n代表普通函数，t代表触发器，w代表窗口函数。 如果声明了PATTERN，只显示名称匹配PATTERN的函数。 列出所有可用函数，以及它们的参数和返回的数据类型。

-- 来源: 5782_file_5782
\ df \dF[+] [PATTERN] 列出所有的文本搜索配置信息。 如果声明了PATTERN，只显示名称匹配PATTERN的配置信息。 列出所有的文本搜索配置信息。

-- 来源: 5782_file_5782
\ dF + \dFd[+] [PATTERN] 列出所有的文本搜索字典。 如果声明了PATTERN，只显示名称匹配PATTERN的字典。 列出所有的文本搜索字典。

-- 来源: 5782_file_5782
\ dFd \dFp[+] [PATTERN] 列出所有的文本搜索分析器。 如果声明了PATTERN，只显示名称匹配PATTERN的分析器。 列出所有的文本搜索分析器。

-- 来源: 5782_file_5782
\ dFp \dFt[+] [PATTERN] 列出所有的文本搜索模板。 如果声明了PATTERN，只显示名称匹配PATTERN的模板。 列出所有的文本搜索模板。

-- 来源: 5782_file_5782
\ dFt \dg[+] [PATTERN] 列出所有数据库角色。 说明： 因为用户和群组的概念被统一为角色，所以这个命令等价于\du。为了和以前兼容，所以保留两个命令。 如果指定了PATTERN，只显示名称匹配PATTERN的角色。 列出名称为“j?e”所有数据库角色（“?”表示任一字符）。

-- 来源: 5782_file_5782
\ dg j ? e \dl \lo_list的别名，显示一个大对象的列表。 - 列出所有的大对象。

-- 来源: 5782_file_5782
\ dl \dL[S+] [PATTERN] 列出可用的程序语言。 如果指定了PATTERN，只列出名称匹配PATTERN的语言。 列出可用的程序语言。

-- 来源: 5782_file_5782
\ dL \dm[S+] [PATTERN] 列出物化视图。 如果指定了PATTERN，只列出名称匹配PATTERN的物化视图。 列出物化视图。

-- 来源: 5782_file_5782
\ dm \dn[S+] [PATTERN] 列出所有模式（名称空间）。如果向命令追加+，会列出每个模式相关的权限及描述。 如果声明了PATTERN，只列出名称匹配PATTERN的模式名。缺省时，只列出用户创建的模式。 列出所有名称以d开头的模式以及相关信息。

-- 来源: 5782_file_5782
\ dn + d * \do[S] [PATTERN] 列出所有可用的操作符，以及它们的操作数和返回的数据类型。 如果声明了PATTERN，只列出名称匹配PATTERN的操作符。缺省时，只列出用户创建的操作符。 列出所有可用的操作符，以及它们的操作数和返回的数据类型。

-- 来源: 5782_file_5782
\ do \dO[S+] [PATTERN] 列出排序规则。 如果声明了PATTERN，只列出名称匹配PATTERN的规则。缺省时，只列出用户创建的规则。 列出排序规则。

-- 来源: 5782_file_5782
\ dO \dp [PATTERN] 列出一列可用的表、视图以及相关的权限信息。 \dp显示结果如下： rolename=xxxx/yyyy --赋予一个角色的权限 =xxxx/yyyy --赋予public的权限 xxxx表示赋予的权限，yyyy表示授予这个权限的角色。权限的参数说明请参见 表5 。 如果指定了PATTERN，只列出名称匹配PATTERN的表、视图。 列出一列可用的表、视图以及相关的权限信息。

-- 来源: 5782_file_5782
\ dp \drds [PATTERN1 [PATTERN2]] 列出所有修改过的配置参数。这些设置可以是针对角色的、针对数据库的或者同时针对两者的。PATTERN1和PATTERN2表示要列出的角色PATTERN和数据库PATTERN。 如果声明了PATTERN，只列出名称匹配PATTERN的规则。缺省或指定*时，则会列出所有设置。 列出数据库所有修改过的配置参数。

-- 来源: 5782_file_5782
\ drds * dbname \dT[S+] [PATTERN] 列出所有的数据类型。 如果指定了PATTERN，只列出名称匹配PATTERN的类型。 列出所有的数据类型。

-- 来源: 5782_file_5782
\ dT \du[+] [PATTERN] 列出所有数据库角色。 说明： 因为用户和群组的概念被统一为角色，所以这个命令等价于\dg。为了和以前兼容，所以保留两个命令。 如果指定了PATTERN，则只列出名称匹配PATTERN的角色。 列出所有数据库角色。

-- 来源: 5782_file_5782
\ du \dE[S+] [PATTERN] \di[S+] [PATTERN] \ds[S+] [PATTERN] \dt[S+] [PATTERN] \dv[S+] [PATTERN] 这一组命令，字母E，i，s，t和v分别代表着外部表，索引，序列，表和视图。可以以任意顺序指定其中一个或者它们的组合来列出这些对象。例如：\dit列出所有的索引和表。在命令名称后面追加+，则每一个对象的物理尺寸以及相关的描述也会被列出。 如果指定了PATTERN，只列出名称匹配该PATTERN的对象。默认情况下只会显示用户创建的对象。通过PATTERN或者S修饰符可以把系统对象包括在内。 列出所有的索引和视图。

-- 来源: 5782_file_5782
\ div \dx[+] [PATTERN] 列出安装数据库的扩展信息。 如果指定了PATTERN，则只列出名称匹配PATTERN的扩展信息。 列出安装数据库的扩展信息。

-- 来源: 5782_file_5782
\ dx \l[+] 列出服务器上所有数据库的名称、所有者、字符集编码以及使用权限。 - 列出服务器上所有数据库的名称、所有者、字符集编码以及使用权限。

-- 来源: 5782_file_5782
\ l \sf[+] FUNCNAME 显示函数的定义。 说明： 对于带圆括号的函数名，需要在函数名两端添加双引号，并且在双引号后面加上参数类型列表。参数类型列表两端添加圆括号。 - 假设存在函数function_a和函数名带圆括号的函数func()name，列出函数的定义。 1 2

-- 来源: 5782_file_5782
\ sf function_a

-- 来源: 5782_file_5782
\ sf "func()name" ( argtype1 , argtype2 ) \z [PATTERN] 列出数据库中所有表、视图和序列，以及它们相关的访问特权。 如果给出任何pattern ，则被当成一个正则表达式，只显示匹配的表、视图、序列。 列出数据库中所有表、视图和序列，以及它们相关的访问特权。

-- 来源: 5782_file_5782
\ z 表5 权限的参数说明 参数 参数说明 r SELECT：允许对指定的表、视图读取数据。 w UPDATE：允许对指定表更新字段。 a INSERT：允许对指定表插入数据。 d DELETE：允许删除指定表中的数据。 D TRUNCATE：允许清理指定表中的数据。 x REFERENCES：允许创建外键约束，分布式场景暂不支持。 t TRIGGER：允许在指定表上创建触发器。 X EXECUTE：允许使用指定的函数，以及利用这些函数实现的操作符。 U USAGE： 对于过程语言，允许用户在创建函数时，指定过程语言。 对于模式，允许访问包含在指定模式中的对象。 对于序列，允许使用nextval函数。 C CREATE： 对于数据库，允许在该数据库里创建新的模式。 对于模式，允许在该模式中创建新的对象。 对于表空间，允许在其中创建表，以及允许创建数据库和模式的时候把该表空间指定为其缺省表空间。 c CONNECT：允许用户连接到指定的数据库。 T TEMPORARY：允许创建临时表。 A ALTER：允许用户修改指定对象的属性。 P DROP：允许用户删除指定的对象。 m COMMENT：允许用户定义或修改指定对象的注释。 i INDEX：允许用户在指定表上创建索引。 v VACUUM：允许用户对指定的表执行ANALYZE和VACUUM操作。 * 给前面权限的授权选项。 表6 格式化元命令 参数 参数说明 \a 对齐模式和非对齐模式之间的切换。 \C [STRING] 把正在打印的表的标题设置为一个查询的结果或者取消这样的设置。 \f [STRING] 对于不对齐的查询输出，显示或者设置域分隔符。 \H 若当前模式为文本格式，则切换为HTML输出格式。 若当前模式为HTML格式，则切换回文本格式。 \pset NAME [VALUE] 设置影响查询结果表输出的选项。NAME的取值见 表7 。 \t [on|off] 切换输出的字段名的信息和行计数脚注。 \T [STRING] 指定在使用HTML输出格式时放在table标签里的属性。如果参数为空，不设置。 \x [on|off|auto] 切换扩展行格式。 表7 可调节的打印选项 选项 选项说明 取值范围 border value必须是一个数字。通常这个数字越大，表的边界就越宽线就越多，但是这个取决于特定的格式。 在HTML格式下，取值范围为大于0的整数。 在其他格式下，取值范围： 0：无边框 1：内部分隔线 2：台架 expanded (或x) 在正常和扩展格式之间切换。 当打开扩展格式时，查询结果用两列显示，字段名称在左、数据在右。这个模式在数据无法放进通常的“水平”模式的屏幕时很有用。 在正常格式下，当查询输出的格式比屏幕宽时，用扩展格式。正常格式只对aligned和wrapped格式有用。 fieldsep 声明域分隔符来实现非对齐输出。这样就可以创建其他程序希望的制表符或逗号分隔的输出。要设置制表符域分隔符，键入\pset fieldsep '\t'。缺省域分隔符是'|'(竖条符)。 - fieldsep_zero 声明域分隔符来实现非对齐输出到零字节。 - footer 用来切换脚注。 - format 设置输出格式。允许使用唯一缩写（这意味着一个字母就够了）。 取值范围： unaligned：写一行的所有列在一条直线上中，当前活动字段分隔符分隔。 aligned：此格式是标准的，可读性好的文本输出。 wrapped：类似aligned，但是包装跨行的宽数据值，使其适应目标字段的宽度输出。 html：把表输出为可用于文档里的对应标记语言。输出不是完整的文档。 latex：把表输出为可用于文档里的对应标记语言。输出不是完整的文档。 troff-ms：把表输出为可用于文档里的对应标记语言。输出不是完整的文档。 null 打印一个字符串，用来代替一个null值。 缺省是什么都不打印，这样很容易和空字符串混淆。 numericlocale 切换分隔小数点左边的数值的区域相关的分组符号。 on：显示指定的分隔符。 off：不显示分隔符。 忽略此参数，显示默认的分隔符。 pager 控制查询和gsql帮助输出的分页器。如果设置了环境变量PAGER，输出将被指向到指定程序，否则使用系统缺省。 on：当输出到终端且不适合屏幕显示时，使用分页器。 off：不使用分页器。 always：当输出到终端无论是否符合屏幕显示时，都使用分页器。 recordsep 声明在非对齐输出格式时的记录分隔符。 - recordsep_zero 声明在非对齐输出到零字节时的记录分隔符。 - tableattr（或T） 声明放在html输出格式中HTML table标签的属性（例如：cellpadding或bgcolor）。注意：这里可能不需要声明border，因为已经在\pset border里用过了。如果没有给出value，则不设置表的属性。 - title 为随后打印的表设置标题。这个可以用于给输出一个描述性标签。如果没有给出value，不设置标题。 - tuples_only（或者t） 在完全显示和只显示实际的表数据之间切换。完全显示将输出像列头、标题、各种脚注等信息。在tuples_only模式下，只显示实际的表数据。 - feedback 切换是否输出结果行数。 - 表8 连接元命令 参数 参数说明 取值范围 \c[onnect] [DBNAME|- USER|- HOST|- PORT|-] 连接到一个新的数据库。当数据库名称长度超过63个字节时，默认前63个字节有效，连接到前63个字节对应的数据库，但是gsql的命令提示符中显示的数据库对象名仍为截断前的名称。 说明： 重新建立连接时，如果切换数据库登录用户，将可能会出现交互式输入，要求输入新用户的连接密码。该密码最长长度为999字节，受限于GUC参数password_max_length的最大值。 - \encoding [ENCODING] 设置客户端字符编码格式。 不带参数时，显示当前的编码格式。 \conninfo 输出当前连接的数据库的信息。 - 表9 操作系统元命令 参数 参数说明 取值范围 \cd [DIR] 切换当前的工作目录。 绝对路径或相对路径，且满足操作系统路径命名规则。 \setenv NAME [VALUE] 设置环境变量NAME为VALUE，如果没有给出VALUE值，则不设置环境变量。 - \timing [on|off] 以毫秒为单位显示每条SQL语句的执行时间（不包括屏显打印时间）。 on表示打开显示。 off表示关闭显示。 \! [COMMAND] 返回到一个单独的Unix shell或者执行Unix命令COMMAND。 - 表10 变量元命令 参数 参数说明 \prompt [TEXT] NAME 提示用户用文本格式来指定变量名称。 \set [NAME [VALUE]] 设置内部变量NAME为VALUE或者如果给出了多于一个值，设置为所有这些值的连接结果。如果没有给出第二个参数，只设变量不设值。 有一些常用变量被gsql特殊对待，它们是一些选项设置，通常所有特殊对待的变量都是由大写字母组成(可能还有数字和下划线)。 表11 是一个所有特殊对待的变量列表。 \unset NAME 不设置（或删除）gsql变量名。 表11 \set常用命令 名称 命令说明 取值范围 \set VERBOSITY value 这个选项可以设置为值default，verbose，terse之一以控制错误报告的冗余行。 value取值范围：default，verbose，terse \set ON_ERROR_STOP value 如果设置了这个变量，脚本处理将马上停止。如果该脚本是从另外一个脚本调用的，那个脚本也会按同样的方式停止。如果最外层的脚本不是从一次交互的gsql会话中调用的而是用-f选项调用的，gsql将返回错误代码3，以示这个情况与致命错误条件的区别（错误代码为1）。 value取值范围为：on/off，true/false，yes/no，1/0 \set AUTOCOMMIT [on|off] 设置当前gsql连接的自动提交行为，on为打开自动提交，off为关闭自动提交。默认情况下，gsql连接处于自动提交模式，每个单独的语句都被隐式提交。如果基于性能或者其它方面考虑，需要关闭自动提交时，需要用户自己显示的发出COMMIT命令来保证事务的提交。例如，在指定的业务SQL执行完之后发送COMMIT语句显式提交，特别是gsql客户端退出之前务必保证所有的事务已经提交。 说明： gsql默认使用自动提交模式，若关闭自动提交，将会导致后面执行的语句都受到隐式事务包裹，数据库中不支持在事务中执行的语句不能在此模式下执行。 on表示打开自动提交。 off表示关闭自动提交。 表12 大对象元命令 参数 参数说明 \lo_list 显示一个目前存储在该数据库里的所有 GaussDB 大对象和提供给他们的注释。 表13 全密态元命令 参数 参数说明 \send_token 全密态功能，传输密钥到服务端缓存，只在开启内存解密逃生通道的情况下使用。 \st 全密态功能，传输密钥到服务端缓存，只在开启内存解密逃生通道的情况下使用。 \clear_token 全密态功能，销毁服务端缓存的密钥，只在开启内存解密逃生通道的情况下使用。 \ct 全密态功能，销毁服务端缓存的密钥，只在开启内存解密逃生通道的情况下使用。 \key_info KEY_INFO 在全密态数据库特性中，用于设置用于访问外部密钥管理者的参数。 分布式暂不支持 全密态内存解密逃生通道 。 PATTERN 很多\d命令都可以用一个PATTERN参数来指定要被显示的对象名称。在最简单的情况下，PATTERN正好就是该对象的准确名称。在PATTERN中的字符通常会被变成小写形式（就像在SQL名称中那样），例如\dt FOO将会显示名为foo的表。就像在SQL名称中那样，把PATTERN放在双引号中可以阻止它被转换成小写形式。如果需要在一个PATTERN中包括一个真正的双引号字符，则需要把它写成两个相邻的双引号，这同样是符合SQL引用标识符的规则。例如，\dt "FOO""BAR"将显示名为FOO"BAR（不是foo"bar）的表。和普通的SQL名称规则不同，不能只在PATTERN的一部分周围放上双引号，例如\dt FOO"FOO"BAR将会显示名为fooFOObar的表。 不使用PATTERN参数时，\d命令会显示当前schema搜索路径中可见的全部对象——这等价于用*作为PATTERN。所谓对象可见是指可以直接用名称引用该对象，而不需要用schema来进行限定。要查看数据库中所有的对象而不管它们的可见性，可以把*.*用作PATTERN。 如果放在一个PATTERN中，*将匹配任意字符序列（包括空序列），而?会匹配任意的单个字符（这种记号方法就像 Unix shell 的文件名PATTERN一样）。例如，\dt int*会显示名称以int开始的表。但是如果被放在双引号内，*和?就会失去这些特殊含义而变成普通的字符。 包含一个点号（.）的PATTERN被解释为一个schema名称模式后面跟上一个对象名称模式。例如，\dt foo*.*bar*会显示名称以foo开始的schema中所有名称包括bar的表。如果没有出现点号，那么模式将只匹配当前schema搜索路径中可见的对象。同样，双引号内的点号会失去其特殊含义并且变成普通的字符。 高级用户可以使用字符类等正则表达式记法，如[0-9]可以匹配任意数字。所有的正则表达式特殊字符都按照POSIX正则表达式所说的工作。以下字符除外： .会按照上面所说的作为一种分隔符。 *会被翻译成正则表达式记号.*。 ?会被翻译成.。 $则按字面意思匹配。 根据需要，可以通过书写?、( R +|)、( R |)和 R ?来分别模拟PATTERN字符.、 R *和 R ?。$不需要作为一个正则表达式字符，因为PATTERN必须匹配整个名称，而不是像正则表达式的常规用法那样解释（换句话说，$会被自动地追加到PATTERN上）。如果不希望该PATTERN的匹配位置被固定，可以在开头或者结尾写上*。注意在双引号内，所有的正则表达式特殊字符会失去其特殊含义并且按照其字面意思进行匹配。另外，在操作符名称PATTERN中（即\do的PATTERN参数），正则表达式特殊字符也按照字面意思进行匹配。

-- 来源: 5891_gsql
\ set foo bar 要引用变量的值，在变量前面加冒号。例如查看变量的值： 1

-- 来源: 5891_gsql
\ echo : foo bar 这种变量的引用方法适用于规则的SQL语句和除\copy、\ef、\help、\sf、\!以外的元命令。 gsql预定义了一些特殊变量，同时也规划了变量的取值。为了保证和后续版本最大限度地兼容，请避免以其他目的使用这些变量。所有特殊变量见 表2 。 所有特殊变量都由大写字母、数字和下划线组成。 要查看特殊变量的默认值，请使用元命令 \echo : varname （例如\echo :DBNAME）。 表2 特殊变量设置 变量 设置方法 变量说明 DBNAME \set DBNAME dbname 当前连接的数据库的名称。每次连接数据库时都会被重新设置。 ECHO \set ECHO all | queries 如果设置为all，只显示查询信息。等效于使用gsql连接数据库时指定-a参数。 如果设置为queries，显示命令行和查询信息。等效于使用gsql连接数据库时指定-e参数。 ECHO_HIDDEN \set ECHO_HIDDEN on | off | noexec 当使用元命令查询数据库信息（例如\dg）时，此变量的取值决定了查询的行为： 设置为on，先显示元命令实际调用的查询语句，然后显示查询结果。等效于使用gsql连接数据库时指定-E参数。 设置为off，则只显示查询结果。 设置为noexec，则只显示查询信息，不执行查询操作。 ENCODING \set ENCODING encoding 当前客户端的字符集编码。 FETCH_COUNT \set FETCH_COUNT variable 如果该变量的值为大于0的整数，假设为n，则执行SELECT语句时每次从结果集中取n行到缓存并显示到屏幕。 如果不设置此变量，或设置的值小于等于0，则执行SELECT语句时一次性把结果都取到缓存。 说明： 设置合理的变量值，将减少内存使用量。一般来说，设为100到1000之间的值比较合理。 HISTCONTROL \set HISTCONTROL ignorespace | ignoredups | ignoreboth | none ignorespace：以空格开始的行将不会写入历史列表。 ignoredups：与以前历史记录里匹配的行不会写入历史记录。 ignoreboth、none或者其他值：所有以交互模式读入的行都被保存到历史列表。 说明： none表示不设置HISTCONTROL。 HISTFILE \set HISTFILE filename 此文件用于存储历史名列表。缺省值是~/.bash_history。 HISTSIZE \set HISTSIZE size 保存在历史命令里命令的个数。缺省值是500。 HOST \set HOST hostname 已连接的数据库主机名称。 IGNOREEOF \set IGNOREEOF variable 若设置此变量为数值，假设为10，则在gsql中输入的前9次EOF字符（通常是Ctrl+C）都会被忽略，在第10次按Ctrl+C才能退出gsql程序。 若设置此变量为非数值，则缺省为10。 若删除此变量，则向交互的gsql会话发送一个EOF终止应用。 LASTOID \set LASTOID oid 最后影响的oid值，即为从一条INSERT或lo_import命令返回的值。此变量只保证在下一条SQL语句的结果显示之前有效。 ON_ERROR_ROLLBACK \set ON_ERROR_ROLLBACK on | interactive | off 如果是on，当一个事务块里的语句产生错误的时候，这个错误将被忽略而事务继续。 如果是interactive，这样的错误只是在交互的会话里忽略。 如果是off（缺省），事务块里一个语句生成的错误将会回滚整个事务。on_error_rollback-on模式是通过在一个事务块的每个命令前隐含地发出一个SAVEPOINT的方式来工作的，在发生错误的时候回滚到该事务块。 ON_ERROR_STOP \set ON_ERROR_STOP on | off on：命令执行错误时会立即停止，在交互模式下，gsql会立即返回已执行命令的结果。 off（缺省）：命令执行错误时将会跳过错误继续执行。 PORT \set PORT port 正连接数据库的端口号。 USER \set USER username 当前用于连接的数据库用户。 VERBOSITY \set VERBOSITY terse | default | verbose 这个选项可以设置为值terse、default、verbose之一以控制错误报告的冗余行。 terse：仅返回严重且主要的错误文本以及文本位置（一般适合于单行错误信息）。 default：返回严重且主要的错误文本及其位置，还包括详细的错误细节、错误提示（可能会跨越多行）。 verbose：返回所有的错误信息。 SQL代换 像元命令的参数一样，gsql变量的一个关键特性是可以把gsql变量替换成正规的SQL语句。此外，gsql还提供为变量更换新的别名或其他标识符等功能。使用SQL代换方式替换一个变量的值可在变量前加冒号。例如： 1 2 3 4 5 6 7 8

-- 来源: 5891_gsql
\ set foo 'HR.areaS'

-- 来源: 5891_gsql
\ set PROMPT2 TEST

-- 来源: 5891_gsql
\ set PROMPT3 '>>>>'

-- 来源: 5892_file_5892
\ l List of databases Name | Owner | Encoding | Collate | Ctype | Access privileges ----------------+----------+-----------+---------+-------+----------------------- human_resource | omm | SQL_ASCII | C | C | postgres | omm | SQL_ASCII | C | C | template0 | omm | SQL_ASCII | C | C | = c / omm + | | | | | omm = CTc / omm template1 | omm | SQL_ASCII | C | C | = c / omm + | | | | | omm = CTc / omm human_staff | omm | SQL_ASCII | C | C | ( 5 rows ) 更多gsql元命令请参见 元命令参考 。 示例 以把一个查询分成多行输入为例。注意提示符的变化： 1 2 3 4

-- 来源: 5892_file_5892
\ d HR . areaS Table "hr.areas" Column | Type | Modifiers -----------+-----------------------+----------- area_id | numeric | not null area_name | character varying ( 25 ) | 向HR.areaS表插入四行数据： 1 2 3 4 5 6 7

-- 来源: 5892_file_5892
\ set PROMPT1 '%n@%m %~%R%#' omm @ [ local ]

-- 来源: 5892_file_5892
查看表： 1 2 3 4 5 6 7 8 omm @ [ local ]

-- 来源: 5892_file_5892
\ pset border 2 Border style is 2 . omm @ [ local ]

-- 来源: 5892_file_5892
\ pset border 0 Border style is 0 . omm @ [ local ]

-- 来源: 5892_file_5892
\ a \ t \ x Output format is unaligned . Showing only tuples . Expanded display is on . omm @ [ local ]

-- 来源: 5892_file_5892
父主题： gsql

-- 来源: 5893_file_5893
\ h Available help : ABORT ALTER AGGREGATE ALTER APP WORKLOAD GROUP ... ... 例如，查看CREATE DATABASE命令的参数可使用下面的命令： 1 2 3 4 5 6 7 8 9 10 11 12

-- 来源: 5893_file_5893
\ help CREATE DATABASE Command : CREATE DATABASE Description : create a new database Syntax : CREATE DATABASE database_name [ [ WITH ] { [ OWNER [ = ] user_name ] | [ TEMPLATE [ = ] template ] | [ ENCODING [ = ] encoding ] | [ LC_COLLATE [ = ] lc_collate ] | [ LC_CTYPE [ = ] lc_ctype ] | [ DBCOMPATIBILITY [ = ] compatibility_type ] | [ TABLESPACE [ = ] tablespace_name ] | [ CONNECTION LIMIT [ = ] connlimit ] } [...] ];

-- 来源: 5893_file_5893
\ ? General \ copyright show GaussDB Kernel usage and distribution terms \ g [ FILE ] or ;

-- 来源: 5895_file_5895
\ d \d[S+] NAME 列出指定表、视图和索引的结构。 - 假设存在表a，列出指定表a的结构。

-- 来源: 5895_file_5895
\ dtable + a \d+ [PATTERN] 列出所有表、视图和索引。 如果声明了PATTERN，只显示名称匹配PATTERN的表、视图和索引。 列出所有名称以f开头的表、视图和索引。

-- 来源: 5895_file_5895
\ d + f * \da[S] [PATTERN] 列出所有可用的聚集函数，以及它们操作的数据类型和返回值类型。 如果声明了PATTERN，只显示名称匹配PATTERN的聚集函数。 列出所有名称以f开头可用的聚集函数，以及它们操作的数据类型和返回值类型。

-- 来源: 5895_file_5895
\ da f * \db[+] [PATTERN] 列出所有可用的表空间。 如果声明了PATTERN，只显示名称匹配PATTERN的表空间。 列出所有名称以p开头的可用表空间。

-- 来源: 5895_file_5895
\ db p * \dc[S+] [PATTERN] 列出所有字符集之间的可用转换。 如果声明了PATTERN，只显示名称匹配PATTERN的转换。 列出所有字符集之间的可用转换。

-- 来源: 5895_file_5895
\ dc * \dC[+] [PATTERN] 列出所有类型转换。 PATTERN需要使用实际类型名，不能使用别名。 如果声明了PATTERN，只显示名称匹配PATTERN的转换。 列出所有名称以c开头的类型转换。

-- 来源: 5895_file_5895
\ dC c * \dd[S] [PATTERN] 显示所有匹配PATTERN的描述。 如果没有给出参数，则显示所有可视对象。“对象”包括：聚集、函数、操作符、类型、关系(表、视图、索引、序列、大对象)、规则。 列出所有可视对象。

-- 来源: 5895_file_5895
\ dd \ddp [PATTERN] 显示所有默认的使用权限。 如果指定了PATTERN，只显示名称匹配PATTERN的使用权限。 列出所有默认的使用权限。

-- 来源: 5895_file_5895
\ ddp \dD[S+] [PATTERN] 列出所有可用域。 如果声明了PATTERN，只显示名称匹配PATTERN的域。 列出所有可用域。

-- 来源: 5895_file_5895
\ dD \det[+] [PATTERN] 列出所有的外部表。 如果声明了PATTERN，只显示名称匹配PATTERN的表。 列出所有的外部表。

-- 来源: 5895_file_5895
\ det \des[+] [PATTERN] 列出所有的外部服务器。 如果声明了PATTERN，只显示名称匹配PATTERN的服务器。 列出所有的外部服务器。

-- 来源: 5895_file_5895
\ des \deu[+] [PATTERN] 列出用户映射信息。 如果声明了PATTERN，只显示名称匹配PATTERN的信息。 列出用户映射信息。

-- 来源: 5895_file_5895
\ deu \dew[+] [PATTERN] 列出封装的外部数据。 如果声明了PATTERN，只显示名称匹配PATTERN的数据。 列出封装的外部数据。

-- 来源: 5895_file_5895
\ dew \df[antw][S+] [PATTERN] 列出所有可用函数，以及它们的参数和返回的数据类型。a代表聚集函数，n代表普通函数，t代表触发器，w代表窗口函数。 如果声明了PATTERN，只显示名称匹配PATTERN的函数。 列出所有可用函数，以及它们的参数和返回的数据类型。

-- 来源: 5895_file_5895
\ df \dF[+] [PATTERN] 列出所有的文本搜索配置信息。 如果声明了PATTERN，只显示名称匹配PATTERN的配置信息。 列出所有的文本搜索配置信息。

-- 来源: 5895_file_5895
\ dF + \dFd[+] [PATTERN] 列出所有的文本搜索字典。 如果声明了PATTERN，只显示名称匹配PATTERN的字典。 列出所有的文本搜索字典。

-- 来源: 5895_file_5895
\ dFd \dFp[+] [PATTERN] 列出所有的文本搜索分析器。 如果声明了PATTERN，只显示名称匹配PATTERN的分析器。 列出所有的文本搜索分析器。

-- 来源: 5895_file_5895
\ dFp \dFt[+] [PATTERN] 列出所有的文本搜索模板。 如果声明了PATTERN，只显示名称匹配PATTERN的模板。 列出所有的文本搜索模板。

-- 来源: 5895_file_5895
\ dFt \dg[+] [PATTERN] 列出所有数据库角色。 说明： 因为用户和群组的概念被统一为角色，所以这个命令等价于\du。为了和以前兼容，所以保留两个命令。 如果指定了PATTERN，只显示名称匹配PATTERN的角色。 列出名称为“j?e”所有数据库角色（“?”表示任一字符）。

-- 来源: 5895_file_5895
\ dg j ? e \dl \lo_list的别名，显示一个大对象的列表。 - 列出所有的大对象。

-- 来源: 5895_file_5895
\ dl \dL[S+] [PATTERN] 列出可用的程序语言。 如果指定了PATTERN，只列出名称匹配PATTERN的语言。 列出可用的程序语言。

-- 来源: 5895_file_5895
\ dL \dm[S+] [PATTERN] 列出物化视图。 如果指定了PATTERN，只列出名称匹配PATTERN的物化视图。 列出物化视图。

-- 来源: 5895_file_5895
\ dm \dn[S+] [PATTERN] 列出所有模式（名称空间）。如果向命令追加+，会列出每个模式相关的权限及描述。 如果声明了PATTERN，只列出名称匹配PATTERN的模式名。缺省时，只列出用户创建的模式。 列出所有名称以d开头的模式以及相关信息。

-- 来源: 5895_file_5895
\ dn + d * \do[S] [PATTERN] 列出所有可用的操作符，以及它们的操作数和返回的数据类型。 如果声明了PATTERN，只列出名称匹配PATTERN的操作符。缺省时，只列出用户创建的操作符。 列出所有可用的操作符，以及它们的操作数和返回的数据类型。

-- 来源: 5895_file_5895
\ do \dO[S+] [PATTERN] 列出排序规则。 如果声明了PATTERN，只列出名称匹配PATTERN的规则。缺省时，只列出用户创建的规则。 列出排序规则。

-- 来源: 5895_file_5895
\ dO \dp [PATTERN] 列出一列可用的表、视图以及相关的权限信息。 \dp显示结果如下： rolename=xxxx/yyyy --赋予一个角色的权限 =xxxx/yyyy --赋予public的权限 xxxx表示赋予的权限，yyyy表示授予这个权限的角色。权限的参数说明请参见 表5 。 如果指定了PATTERN，只列出名称匹配PATTERN的表、视图。 列出一列可用的表、视图以及相关的权限信息。

-- 来源: 5895_file_5895
\ dp \drds [PATTERN1 [PATTERN2]] 列出所有修改过的配置参数。这些设置可以是针对角色的、针对数据库的或者同时针对两者的。PATTERN1和PATTERN2表示要列出的角色PATTERN和数据库PATTERN。 如果声明了PATTERN，只列出名称匹配PATTERN的规则。缺省或指定*时，则会列出所有设置。 列出数据库所有修改过的配置参数。

-- 来源: 5895_file_5895
\ drds * dbname \dT[S+] [PATTERN] 列出所有的数据类型。 如果指定了PATTERN，只列出名称匹配PATTERN的类型。 列出所有的数据类型。

-- 来源: 5895_file_5895
\ dT \du[+] [PATTERN] 列出所有数据库角色。 说明： 因为用户和群组的概念被统一为角色，所以这个命令等价于\dg。为了和以前兼容，所以保留两个命令。 如果指定了PATTERN，则只列出名称匹配PATTERN的角色。 列出所有数据库角色。

-- 来源: 5895_file_5895
\ du \dE[S+] [PATTERN] \di[S+] [PATTERN] \ds[S+] [PATTERN] \dt[S+] [PATTERN] \dv[S+] [PATTERN] 这一组命令，字母E，i，s，t和v分别代表着外部表，索引，序列，表和视图。可以以任意顺序指定其中一个或者它们的组合来列出这些对象。例如：\dit列出所有的索引和表。在命令名称后面追加+，则每一个对象的物理尺寸以及相关的描述也会被列出。 如果指定了PATTERN，只列出名称匹配该PATTERN的对象。默认情况下只会显示用户创建的对象。通过PATTERN或者S修饰符可以把系统对象包括在内。 列出所有的索引和视图。

-- 来源: 5895_file_5895
\ div \dx[+] [PATTERN] 列出安装数据库的扩展信息。 如果指定了PATTERN，则只列出名称匹配PATTERN的扩展信息。 列出安装数据库的扩展信息。

-- 来源: 5895_file_5895
\ dx \l[+] 列出服务器上所有数据库的名称、所有者、字符集编码以及使用权限。 - 列出服务器上所有数据库的名称、所有者、字符集编码以及使用权限。

-- 来源: 5895_file_5895
\ l \sf[+] FUNCNAME 显示函数的定义。 说明： 对于带圆括号的函数名，需要在函数名两端添加双引号，并且在双引号后面加上参数类型列表。参数类型列表两端添加圆括号。 - 假设存在函数function_a和函数名带圆括号的函数func()name，列出函数的定义。 1 2

-- 来源: 5895_file_5895
\ sf function_a

-- 来源: 5895_file_5895
\ sf "func()name" ( argtype1 , argtype2 ) \z [PATTERN] 列出数据库中所有表、视图和序列，以及它们相关的访问特权。 如果给出任何pattern ，则被当成一个正则表达式，只显示匹配的表、视图、序列。 列出数据库中所有表、视图和序列，以及它们相关的访问特权。

-- 来源: 5895_file_5895
\ z 表5 权限的参数说明 参数 参数说明 r SELECT：允许对指定的表、视图读取数据。 w UPDATE：允许对指定表更新字段。 a INSERT：允许对指定表插入数据。 d DELETE：允许删除指定表中的数据。 D TRUNCATE：允许清理指定表中的数据。 x REFERENCES：允许创建外键约束。 t TRIGGER：允许在指定表上创建触发器。 X EXECUTE：允许使用指定的函数，以及利用这些函数实现的操作符。 U USAGE： 对于过程语言，允许用户在创建函数时，指定过程语言。 对于模式，允许访问包含在指定模式中的对象。 对于序列，允许使用nextval函数。 C CREATE： 对于数据库，允许在该数据库里创建新的模式。 对于模式，允许在该模式中创建新的对象。 对于表空间，允许在其中创建表，以及允许创建数据库和模式的时候把该表空间指定为其缺省表空间。 c CONNECT：允许用户连接到指定的数据库。 T TEMPORARY：允许创建临时表。 A ALTER：允许用户修改指定对象的属性。 P DROP：允许用户删除指定的对象。 m COMMENT：允许用户定义或修改指定对象的注释。 i INDEX：允许用户在指定表上创建索引。 v VACUUM：允许用户对指定的表执行ANALYZE和VACUUM操作。 * 给前面权限的授权选项。 表6 格式化元命令 参数 参数说明 \a 对齐模式和非对齐模式之间的切换。 \C [STRING] 把正在打印的表的标题设置为一个查询的结果或者取消这样的设置。 \f [STRING] 对于不对齐的查询输出，显示或者设置域分隔符。 \H 若当前模式为文本格式，则切换为HTML输出格式。 若当前模式为HTML格式，则切换回文本格式。 \pset NAME [VALUE] 设置影响查询结果表输出的选项。NAME的取值见 表7 。 \t [on|off] 切换输出的字段名的信息和行计数脚注。 \T [STRING] 指定在使用HTML输出格式时放在table标签里的属性。如果参数为空，不设置。 \x [on|off|auto] 切换扩展行格式。 表7 可调节的打印选项 选项 选项说明 取值范围 border value必须是一个数字。通常这个数字越大，表的边界就越宽线就越多，但是这个取决于特定的格式。 在HTML格式下，取值范围为大于0的整数。 在其他格式下，取值范围： 0：无边框 1：内部分隔线 2：台架 expanded (或x) 在正常和扩展格式之间切换。 当打开扩展格式时，查询结果用两列显示，字段名称在左、数据在右。这个模式在数据无法放进通常的“水平”模式的屏幕时很有用。 在正常格式下，当查询输出的格式比屏幕宽时，用扩展格式。正常格式只对aligned和wrapped格式有用。 fieldsep 声明域分隔符来实现非对齐输出。这样就可以创建其他程序希望的制表符或逗号分隔的输出。要设置制表符域分隔符，键入\pset fieldsep '\t'。缺省域分隔符是 '|' (竖条符)。 - fieldsep_zero 声明域分隔符来实现非对齐输出到零字节。 - footer 用来切换脚注。 - format 设置输出格式。允许使用唯一缩写（这意味着一个字母就够了）。 取值范围： unaligned：写一行的所有列在一条直线上中，当前活动字段分隔符分隔。 aligned：此格式是标准的，可读性好的文本输出。 wrapped：类似aligned，但是包装跨行的宽数据值，使其适应目标字段的宽度输出。 html：把表输出为可用于文档里的对应标记语言。输出不是完整的文档。 latex：把表输出为可用于文档里的对应标记语言。输出不是完整的文档。 troff-ms：把表输出为可用于文档里的对应标记语言。输出不是完整的文档。 null 打印一个字符串，用来代替一个null值。 缺省是什么都不打印，这样很容易和空字符串混淆。 numericlocale 切换分隔小数点左边的数值的区域相关的分组符号。 on：显示指定的分隔符。 off：不显示分隔符。 忽略此参数，显示默认的分隔符。 pager 控制查询和gsql帮助输出的分页器。如果设置了环境变量PAGER，输出将被指向到指定程序，否则使用系统缺省。 on：当输出到终端且不适合屏幕显示时，使用分页器。 off：不使用分页器。 always：当输出到终端无论是否符合屏幕显示时，都使用分页器。 recordsep 声明在非对齐输出格式时的记录分隔符。 - recordsep_zero 声明在非对齐输出到零字节时的记录分隔符。 - tableattr（或T） 声明放在html输出格式中HTML table标签的属性（例如：cellpadding或bgcolor）。注意：这里可能不需要声明border，因为已经在\pset border里用过了。如果没有给出value，则不设置表的属性。 - title 为随后打印的表设置标题。这个可以用于给输出一个描述性标签。如果没有给出value，不设置标题。 - tuples_only （或者t） 在完全显示和只显示实际的表数据之间切换。完全显示将输出像列头、标题、各种脚注等信息。在tuples_only模式下，只显示实际的表数据。 - feedback 切换是否输出结果行数 - 表8 连接元命令 参数 参数说明 取值范围 \c[onnect] [DBNAME|- USER|- HOST|- PORT|-] 连接到一个新的数据库。当数据库名称长度超过63个字节时，默认前63个字节有效，连接到前63个字节对应的数据库，但是gsql的命令提示符中显示的数据库对象名仍为截断前的名称。 说明： 重新建立连接时，如果切换数据库登录用户，将可能会出现交互式输入，要求输入新用户的连接密码。该密码最长长度为999字节，受限于GUC参数password_max_length的最大值。 - \encoding [ENCODING] 设置客户端字符编码格式。 不带参数时，显示当前的编码格式。 \conninfo 输出当前连接的数据库的信息。 - 表9 操作系统元命令 参数 参数说明 取值范围 \cd [DIR] 切换当前的工作目录。 绝对路径或相对路径，且满足操作系统路径命名规则。 \setenv NAME [VALUE] 设置环境变量NAME为VALUE，如果没有给出VALUE值，则不设置环境变量。 - \timing [on|off] 以毫秒为单位显示每条SQL语句的执行时间 （不包括屏显打印时间） 。 on表示打开显示。 off表示关闭显示。 \! [COMMAND] 返回到一个单独的Unix shell或者执行Unix命令COMMAND。 - 表10 变量元命令 参数 参数说明 \prompt [TEXT] NAME 提示用户用文本格式来指定变量名称。 \set [NAME [VALUE]] 设置内部变量NAME为VALUE或者如果给出了多于一个值，设置为所有这些值的连接结果。如果没有给出第二个参数，只设变量不设值。 有一些常用变量被gsql特殊对待，它们是一些选项设置，通常所有特殊对待的变量都是由大写字母组成(可能还有数字和下划线)。 表11 是一个所有特殊对待的变量列表。 \unset NAME 不设置（或删除）gsql变量名。 表11 \set常用命令 名称 命令说明 取值范围 \set VERBOSITY value 这个选项可以设置为值default，verbose，terse之一以控制错误报告的冗余行。 value取值范围：default， verbose，terse \set ON_ERROR_STOP value 如果设置了这个变量，脚本处理将马上停止。如果该脚本是从另外一个脚本调用的，那个脚本也会按同样的方式停止。如果最外层的脚本不是从一次交互的gsql会话中调用的而是用-f选项调用的，gsql将返回错误代码3，以示这个情况与致命错误条件的区别（错误代码为1）。 value取值范围为：on/off，true/false，yes/no，1/0 \set AUTOCOMMIT [on|off] 设置当前gsql连接的自动提交行为，on为打开自动提交，off为关闭自动提交。默认情况下，gsql连接处于自动提交模式，每个单独的语句都被隐式提交。如果基于性能或者其它方面考虑，需要关闭自动提交时，需要用户自己显示的发出COMMIT命令来保证事务的提交。例如，在指定的业务SQL执行完之后发送COMMIT语句显式提交，特别是gsql客户端退出之前务必保证所有的事务已经提交。 说明： gsql默认使用自动提交模式，若关闭自动提交，将会导致后面执行的语句都受到隐式事务包裹，数据库中不支持在事务中执行的语句不能在此模式下执行。 on表示打开自动提交。 off表示关闭自动提交。 表12 大对象元命令 参数 参数说明 \lo_list 显示一个目前存储在该数据库里的所有 GaussDB 大对象和提供给他们的注释。 表13 全密态元命令 参数 参数说明 \send_token 全密态功能，传输密钥到服务端缓存，只在开启内存解密逃生通道的情况下使用。 \st 全密态功能，传输密钥到服务端缓存，只在开启内存解密逃生通道的情况下使用。 \clear_token 全密态功能，销毁服务端缓存的密钥，只在开启内存解密逃生通道的情况下使用。 \ct 全密态功能，销毁服务端缓存的密钥，只在开启内存解密逃生通道的情况下使用。 \key_info KEY_INFO 在全密态数据库特性中，用于设置访问外部密钥管理者的参数。 PATTERN 很多\d命令都可以用一个PATTERN参数来指定要被显示的对象名称。在最简单的情况下，PATTERN正好就是该对象的准确名称。在PATTERN中的字符通常会被变成小写形式（就像在SQL名称中那样），例如\dt FOO将会显示名为foo的表。就像在SQL名称中那样，把PATTERN放在双引号中可以阻止它被转换成小写形式。如果需要在一个PATTERN中包括一个真正的双引号字符，则需要把它写成两个相邻的双引号，这同样是符合SQL引用标识符的规则。例如，\dt "FOO""BAR"将显示名为FOO"BAR（不是foo"bar）的表。和普通的SQL名称规则不同，不能只在PATTERN的一部分周围放上双引号，例如\dt FOO"FOO"BAR将会显示名为fooFOObar的表。 不使用PATTERN参数时，\d命令会显示当前schema搜索路径中可见的全部对象——这等价于用*作为PATTERN。所谓对象可见是指可以直接用名称引用该对象，而不需要用schema来进行限定。要查看数据库中所有的对象而不管它们的可见性，可以把*.*用作PATTERN。 如果放在一个PATTERN中，*将匹配任意字符序列（包括空序列），而?会匹配任意的单个字符（这种记号方法就像 Unix shell 的文件名PATTERN一样）。例如，\dt int*会显示名称以int开始的表。但是如果被放在双引号内，*和?就会失去这些特殊含义而变成普通的字符。 包含一个点号（.）的PATTERN被解释为一个schema名称模式后面跟上一个对象名称模式。例如，\dt foo*.*bar*会显示名称以foo开始的schema中所有名称包括bar的表。如果没有出现点号，那么模式将只匹配当前schema搜索路径中可见的对象。同样，双引号内的点号会失去其特殊含义并且变成普通的字符。 高级用户可以使用字符类等正则表达式记法，如[0-9]可以匹配任意数字。所有的正则表达式特殊字符都按照POSIX正则表达式所说的工作。以下字符除外： .会按照上面所说的作为一种分隔符。 *会被翻译成正则表达式记号.*。 ?会被翻译成.。 $则按字面意思匹配。 根据需要，可以通过书写?、( R +|)、( R |)和 R ?来分别模拟PATTERN字符.、 R *和 R ?。$不需要作为一个正则表达式字符，因为PATTERN必须匹配整个名称，而不是像正则表达式的常规用法那样解释（换句话说，$会被自动地追加到PATTERN上）。如果不希望该PATTERN的匹配位置被固定，可以在开头或者结尾写上*。注意在双引号内，所有的正则表达式特殊字符会失去其特殊含义并且按照其字面意思进行匹配。另外，在操作符名称PATTERN中（即\do的PATTERN参数），正则表达式特殊字符也按照字面意思进行匹配。 DELIMITER 更改SQL语句之间分隔符命令，分隔符默认值为“;

-- 来源: 744_file_744
CREATE ROW LEVEL SECURITY POLICY all_data_rls ON all_data USING ( role = CURRENT_USER );

-- 来源: 744_file_744
\ d + all_data Table "public.all_data" Column | Type | Modifiers | Storage | Stats target | Description --------+------------------------+-----------+----------+--------------+------------- id | integer | | plain | | role | character varying ( 100 ) | | extended | | data | character varying ( 100 ) | | extended | | Row Level Security Policies : POLICY "all_data_rls" FOR ALL TO public USING ((( role ):: name = "current_user" ())) Has OIDs : no Distribute By : HASH ( id ) Location Nodes : ALL DATANODES Options : orientation = row , compression = no , enable_rowsecurity = true --切换至用户alice，执行SQL"SELECT * FROM public.all_data"

-- 来源: 752_file_752
gsql((GaussDB Kernel XXX.XXX.XXX build f521c606) compiled at 2021-09-16 14:55:22 commit 2935 last mr 6385 release) Non-SSL connection (SSL connection is recommended when requiring high-security) Type "help" for help. db_tpcds=> 查看数据库 使用\l元命令查看数据库系统的数据库列表。

-- 来源: 752_file_752
\ l 使用如下命令通过系统表pg_database查询数据库列表。

-- 来源: 759_file_759
\ d + customer_t1 ;

-- 来源: 765_file_765
\ di + tpcds . tpcds_web_returns_p2_index2 删除索引 1

-- 来源: 766_file_766
\ d + MyView View "PG_CATALOG.MyView" Column | Type | Modifiers | Storage | Description ----------+-----------------------+-----------+----------+------------- USERNAME | CHARACTER VARYING ( 64 ) | | extended | View definition : SELECT PG_AUTHID . ROLNAME :: CHARACTER VARYING ( 64 ) AS USERNAME FROM PG_AUTHID ;

