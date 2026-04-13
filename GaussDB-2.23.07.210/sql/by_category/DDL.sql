-- 类别: DDL
-- SQL 数量: 3436

-- 来源: 1000_HintQueryblock
create view v1 as select/*+ no_expand */ c1 from t1 where c1 in (select /*+ no_expand */ c1 from t2 where t2.c3=4 );

-- 来源: 1024_SQL PATCH
create table hint_t1 ( a int , b int , c int );

-- 来源: 1024_SQL PATCH
create index on hint_t1 ( a );

-- 来源: 1024_SQL PATCH
create table test_proc_patch(a int,b int);

-- 来源: 1024_SQL PATCH
create procedure mypro() as num int;

-- 来源: 1041_GUCrewrite_rule
create table t_rep(a int) distribute by replication;

-- 来源: 1041_GUCrewrite_rule
create table t_dis(a int);

-- 来源: 1047_file_1047
CREATE TABLE int_type_t1 ( IT_COL1 TINYINT, IT_COL2 TINYINT UNSIGNED );

--删除表。
-- 来源: 1047_file_1047
DROP TABLE int_type_t1;

-- 来源: 1047_file_1047
CREATE TABLE int_type_t2 ( a TINYINT , b TINYINT , c INTEGER , d INTEGER UNSIGNED , e BIGINT , f BIGINT UNSIGNED );

-- 来源: 1047_file_1047
DROP TABLE int_type_t2 ;

-- 来源: 1047_file_1047
CREATE TABLE decimal_type_t1 ( DT_COL1 DECIMAL(10,4) );

--删除表。
-- 来源: 1047_file_1047
DROP TABLE decimal_type_t1;

-- 来源: 1047_file_1047
CREATE TABLE numeric_type_t1 ( NT_COL1 NUMERIC ( 10 , 4 ) );

-- 来源: 1047_file_1047
DROP TABLE numeric_type_t1 ;

-- 来源: 1047_file_1047
CREATE TABLE smallserial_type_tab ( a SMALLSERIAL );

-- 来源: 1047_file_1047
CREATE TABLE serial_type_tab ( b SERIAL );

-- 来源: 1047_file_1047
CREATE TABLE bigserial_type_tab ( c BIGSERIAL );

-- 来源: 1047_file_1047
DROP TABLE smallserial_type_tab ;

-- 来源: 1047_file_1047
DROP TABLE serial_type_tab ;

-- 来源: 1047_file_1047
DROP TABLE bigserial_type_tab ;

-- 来源: 1047_file_1047
CREATE TABLE float_type_t2 ( FT_COL1 INTEGER , FT_COL2 FLOAT4 , FT_COL3 FLOAT8 , FT_COL4 FLOAT ( 3 ), FT_COL5 BINARY_DOUBLE , FT_COL6 DECIMAL ( 10 , 4 ), FT_COL7 INTEGER ( 6 , 3 ) ) DISTRIBUTE BY HASH ( ft_col1 );

-- 来源: 1047_file_1047
DROP TABLE float_type_t2 ;

-- 来源: 1047_file_1047
CREATE DATABASE gaussdb_m WITH dbcompatibility 'MYSQL' ;

-- 来源: 1049_file_1049
CREATE TABLE bool_type_t1 ( BT_COL1 BOOLEAN , BT_COL2 TEXT ) DISTRIBUTE BY HASH ( BT_COL2 );

-- 来源: 1049_file_1049
DROP TABLE bool_type_t1 ;

-- 来源: 1050_file_1050
CREATE TABLE varchar_maxlength_test1 (a int, b varchar, c int) DISTRIBUTE BY HASH (a);

-- 创建表，表中仅varchar一列，根据计算规则，varchar最大存储长度为1GB-85-4=
-- 来源: 1050_file_1050
CREATE TABLE varchar_maxlength_test2 (a varchar) DISTRIBUTE BY HASH (a);

-- 来源: 1050_file_1050
CREATE TABLE char_type_t1 ( CT_COL1 CHARACTER(4) )DISTRIBUTE BY HASH (CT_COL1);

--删除表。
-- 来源: 1050_file_1050
DROP TABLE char_type_t1;

-- 来源: 1050_file_1050
CREATE TABLE char_type_t2 ( CT_COL1 VARCHAR ( 5 ) ) DISTRIBUTE BY HASH ( CT_COL1 );

-- 来源: 1050_file_1050
DROP TABLE char_type_t2 ;

-- 来源: 1050_file_1050
create database gaussdb_m with dbcompatibility 'MYSQL' ;

-- 来源: 1051_file_1051
CREATE TABLE blob_type_t1 ( BT_COL1 INTEGER , BT_COL2 BLOB , BT_COL3 RAW , BT_COL4 BYTEA ) DISTRIBUTE BY REPLICATION ;

-- 来源: 1051_file_1051
DROP TABLE blob_type_t1 ;

-- 来源: 1051_file_1051
CREATE DATABASE gaussdb_m WITH dbcompatibility 'MYSQL' ;

-- 来源: 1052__
CREATE TABLE date_type_tab ( coll date );

-- 来源: 1052__
DROP TABLE date_type_tab ;

-- 来源: 1052__
CREATE TABLE time_type_tab ( da time without time zone , dai time with time zone , dfgh timestamp without time zone , dfga timestamp with time zone , vbg smalldatetime );

-- 来源: 1052__
DROP TABLE time_type_tab ;

-- 来源: 1052__
CREATE TABLE day_type_tab ( a int , b INTERVAL DAY ( 3 ) TO SECOND ( 4 ));

-- 来源: 1052__
DROP TABLE day_type_tab ;

-- 来源: 1052__
CREATE TABLE year_type_tab ( a int , b interval year ( 6 ));

-- 来源: 1052__
DROP TABLE year_type_tab ;

-- 来源: 1052__
create database gaussdb_m dbcompatibility = 'MYSQL' ;

-- 来源: 1052__
CREATE TABLE date_type_tab ( coll date );

-- 来源: 1052__
DROP TABLE date_type_tab ;

-- 来源: 1052__
CREATE TABLE realtime_type_special(col1 varchar(20), col2 date, col3 timestamp, col4 time);

--删除表。
-- 来源: 1052__
DROP TABLE realtime_type_special;

-- 来源: 1052__
CREATE TABLE reltime_type_tab ( col1 character ( 30 ), col2 reltime );

-- 来源: 1052__
DROP TABLE reltime_type_tab ;

-- 来源: 1055_file_1055
CREATE TABLE bit_type_t1 ( BT_COL1 INTEGER , BT_COL2 BIT ( 3 ), BT_COL3 BIT VARYING ( 5 ) ) DISTRIBUTE BY REPLICATION ;

-- 来源: 1055_file_1055
DROP TABLE bit_type_t1 ;

-- 来源: 1059_HLL
CREATE TABLE t1 ( id integer , set hll );

-- 来源: 1059_HLL
CREATE TABLE t2 ( id integer , set hll ( 12 , 4 ));

-- 来源: 1059_HLL
CREATE TABLE t3 ( id int , set hll ( - 1 , - 1 , 8 , - 1 ));

-- 来源: 1059_HLL
CREATE TABLE t4 ( id int , set hll ( 5 , - 1 ));

-- 来源: 1059_HLL
DROP TABLE t1 , t2 , t3 ;

-- 来源: 1059_HLL
CREATE TABLE t1 ( id integer , set hll ( 14 ));

-- 来源: 1059_HLL
DROP TABLE t1 ;

-- 来源: 1059_HLL
CREATE TABLE helloworld ( id integer , set hll );

-- 来源: 1059_HLL
DROP TABLE helloworld ;

-- 来源: 1059_HLL
CREATE TABLE facts ( date date , user_id integer );

-- 来源: 1059_HLL
create TABLE daily_uniques ( date date UNIQUE , users hll );

-- 来源: 1059_HLL
DROP TABLE facts ;

-- 来源: 1059_HLL
DROP TABLE daily_uniques ； 场景3：“插入数据不满足hll数据结构要求” 当用户给hll类型的字段插入数据的时候，必须保证插入的数据满足hll数据结构要求，如果解析后不满足就会报错。如下示例中： 插入数据'E\\1234'时，该数据不满足hll数据结构 要求 ，不能解析成功因此失败报错。 1 2 3

-- 来源: 1059_HLL
CREATE TABLE test ( id integer , set hll );

-- 来源: 1059_HLL
DROP TABLE test ;

-- 来源: 1060_file_1060
CREATE TABLE reservation (room int, during tsrange);

-- 来源: 1062_file_1062
create table t1 ( a int );

-- 来源: 1062_file_1062
CREATE OR REPLACE FUNCTION showall () RETURNS SETOF record AS $$ SELECT count ( * ) from t1 ;

-- 来源: 1062_file_1062
DROP FUNCTION showall ();

-- 来源: 1062_file_1062
drop table t1 ;

-- 来源: 1065_XML
CREATE TABLE xmltest ( id int, data xml );

-- 来源: 1065_XML
DROP TABLE xmltest;

-- 来源: 1066_XMLTYPE
CREATE TABLE xmltypetest(id int, data xmltype);

-- 来源: 1067_aclitem
CREATE TABLE table_acl (id int,priv aclitem,privs aclitem[]);

-- 来源: 1072_file_1072
CREATE DATABASE gaussdb_m WITH dbcompatibility 'MYSQL' ;

-- 来源: 1072_file_1072
CREATE TABLE test_space ( c char ( 10 ));

-- 来源: 1072_file_1072
create table test ( a text );

-- 来源: 1072_file_1072
CREATE EXTENSION pkg_bpchar_opc ;

-- 来源: 1072_file_1072
DROP EXTENSION pkg_bpchar_opc ;

-- 来源: 1072_file_1072
CREATE EXTENSION pkg_bpchar_opc ;

-- 来源: 1072_file_1072
DROP EXTENSION pkg_bpchar_opc ;

-- 来源: 1072_file_1072
CREATE EXTENSION pkg_bpchar_opc ;

-- 来源: 1072_file_1072
DROP EXTENSION pkg_bpchar_opc ;

-- 来源: 1078_file_1078
CREATE DATABASE gaussdb_m WITH dbcompatibility 'MYSQL' ;

-- 来源: 1078_file_1078
create table json_doc ( data CLOB );

-- 来源: 1078_file_1078
create or replace procedure p1 is gaussdb $ # type t1 is table of int ;

-- 来源: 1078_file_1078
create type t1 is table of int ;

-- 来源: 1082_JSON_JSONB
CREATE TYPE jpop AS (a text, b int, c bool);

-- 来源: 1082_JSON_JSONB
DROP TYPE jpop;

-- 来源: 1082_JSON_JSONB
CREATE TYPE jpop AS (a text, b int, c bool);

-- 来源: 1082_JSON_JSONB
DROP TYPE jpop;

-- 来源: 1082_JSON_JSONB
CREATE TABLE classes(name varchar, score int);

-- 来源: 1082_JSON_JSONB
DROP TABLE classes;

-- 来源: 1082_JSON_JSONB
CREATE TABLE classes(name varchar, score int);

-- 来源: 1082_JSON_JSONB
DROP TABLE classes;

-- 来源: 1083_HLL
CREATE TABLE t_id ( id int );

-- 来源: 1083_HLL
CREATE TABLE t_data ( a int , c text );

-- 来源: 1083_HLL
CREATE TABLE t_a_c_hll ( a int , c hll );

-- 来源: 1084_SEQUENCE
CREATE SEQUENCE seqDemo ;

-- 来源: 1084_SEQUENCE
DROP SEQUENCE seqDemo ;

-- 来源: 1084_SEQUENCE
CREATE SEQUENCE seq1 ;

-- 来源: 1084_SEQUENCE
DROP SEQUENCE seq1 ;

-- 来源: 1084_SEQUENCE
CREATE SEQUENCE seq1 ;

-- 来源: 1084_SEQUENCE
DROP SEQUENCE seq1 ;

-- 来源: 1084_SEQUENCE
CREATE SEQUENCE seqDemo ;

-- 来源: 1084_SEQUENCE
DROP SEQUENCE seqDemo ;

-- 来源: 1084_SEQUENCE
CREATE SEQUENCE seqDemo ;

-- 来源: 1084_SEQUENCE
DROP SEQUENCE seqDemo ;

-- 来源: 1087_file_1087
CREATE TABLE tab ( a int );

-- 来源: 1088_file_1088
CREATE OR REPLACE PROCEDURE proc IS CURSOR cur_1 IS SELECT RATIO_TO_REPORT ( sales_amount ) OVER () FROM sales_numeric ;

-- 来源: 1091_file_1091
CREATE OR REPLACE FUNCTION unnest2 ( anyarray ) RETURNS SETOF anyelement AS $$ SELECT $ 1 [ i ][ j ] FROM generate_subscripts ( $ 1 , 1 ) g1 ( i ), generate_subscripts ( $ 1 , 2 ) g2 ( j );

-- 来源: 1091_file_1091
DROP FUNCTION unnest2 ;

-- 来源: 1092_file_1092
CREATE TABLE blob_tb ( b blob , id int ) DISTRIBUTE BY REPLICATION ;

-- 来源: 1092_file_1092
DROP TABLE blob_tb ;

-- 来源: 1092_file_1092
CREATE TABLE clob_tb ( c clob , id int );

-- 来源: 1092_file_1092
DROP TABLE clob_tb ;

-- 来源: 1092_file_1092
CREATE TABLE student_demo ( name VARCHAR2 ( 20 ), grade NUMBER ( 10 , 2 ));

-- 来源: 1108_file_1108
CREATE TABLE test(a int,b int);

-- 来源: 1108_file_1108
CREATE PROCEDURE mypro1() as num int;

-- 来源: 1110_file_1110
CREATE TABLE part_tab 1 ( a int, b int ) PARTITION BY RANGE(b) ( PARTITION P1 VALUES LESS THAN(10), PARTITION P2 VALUES LESS THAN(20), PARTITION P3 VALUES LESS THAN(MAXVALUE) );

-- 来源: 1110_file_1110
CREATE TABLE subpart_tab 1 ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY RANGE (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a VALUES LESS THAN( '2' ), SUBPARTITION p_201901_b VALUES LESS THAN( '3' ) ), PARTITION p_201902 VALUES LESS THAN( '201904' ) ( SUBPARTITION p_201902_a VALUES LESS THAN( '2' ), SUBPARTITION p_201902_b VALUES LESS THAN( '3' ) ) );

-- 来源: 1110_file_1110
CREATE INDEX index_part_tab1 ON part_tab1(b) LOCAL ( PARTITION b_index1, PARTITION b_index2, PARTITION b_index 3 );

-- 来源: 1110_file_1110
CREATE INDEX idx_user_no ON subpart_tab1(user_no) LOCAL;

-- 来源: 1112_HashFunc
CREATE TYPE b1 AS ENUM ( 'good' , 'bad' , 'ugly' );

-- 来源: 1124_XML
CREATE TABLE xmltest ( id int , data xml );

-- 来源: 1126_file_1126
create extension tablefunc ;

-- 来源: 1126_file_1126
create extension tablefunc ;

-- 来源: 1126_file_1126
create extension tablefunc ;

-- 来源: 1128_SQL
create database db1 ;

-- 来源: 1128_SQL
create database db2 ;

-- 来源: 1128_SQL
create database db1 ;

-- 来源: 1132_file_1132
CREATE TABLE tpcds . case_when_t1 ( CW_COL1 INT ) DISTRIBUTE BY HASH ( CW_COL1 );

-- 来源: 1132_file_1132
DROP TABLE tpcds . case_when_t1 ;

-- 来源: 1132_file_1132
CREATE TABLE tpcds . c_tabl ( description varchar ( 10 ), short_description varchar ( 10 ), last_value varchar ( 10 )) DISTRIBUTE BY HASH ( last_value );

-- 来源: 1132_file_1132
DROP TABLE tpcds . c_tabl ;

-- 来源: 1132_file_1132
CREATE TABLE tpcds . null_if_t1 ( NI_VALUE1 VARCHAR ( 10 ), NI_VALUE2 VARCHAR ( 10 ) ) DISTRIBUTE BY HASH ( NI_VALUE1 );

-- 来源: 1132_file_1132
DROP TABLE tpcds . null_if_t1 ;

-- 来源: 1137_file_1137
CREATE TABLE Students ( name varchar ( 20 ), id int ) with ( STORAGE_TYPE = USTORE );

-- 来源: 1137_file_1137
DROP TABLE Students ;

-- 来源: 1137_file_1137
CREATE TABLE test ( a int , b int );

-- 来源: 1137_file_1137
DROP TABLE test ;

-- 来源: 1137_file_1137
CREATE TABLE test ( a int , b int );

-- 来源: 1142_file_1142
CREATE TABLE tpcds . value_storage_t1 ( VS_COL1 CHARACTER ( 20 ) ) DISTRIBUTE BY HASH ( VS_COL1 );

-- 来源: 1142_file_1142
DROP TABLE tpcds . value_storage_t1 ;

-- 来源: 1143_UNIONCASE
CREATE DATABASE oracle_1 dbcompatibility = 'ORA';

--在TD模式下，创建TD兼容模式的数据库td_1。
-- 来源: 1143_UNIONCASE
CREATE DATABASE td_1 dbcompatibility = 'TD';

--删除Oracle和TD模式的数据库。
-- 来源: 1143_UNIONCASE
DROP DATABASE oracle_1;

-- 来源: 1143_UNIONCASE
DROP DATABASE td_1;

-- 来源: 1143_UNIONCASE
CREATE DATABASE ora_1 dbcompatibility = 'A';

--删除ORA模式的数据库。
-- 来源: 1143_UNIONCASE
DROP DATABASE ora_1;

-- 来源: 1151_file_1151
DROP SCHEMA IF EXISTS tsearch CASCADE ;

-- 来源: 1151_file_1151
CREATE SCHEMA tsearch ;

-- 来源: 1151_file_1151
CREATE TABLE tsearch . pgweb ( id int , body text , title text , last_mod_date date ) with ( storage_type = ASTORE );

-- 来源: 1152_file_1152
CREATE INDEX pgweb_idx_1 ON tsearch . pgweb USING gin ( to_tsvector ( 'english' , body ));

-- 来源: 1152_file_1152
CREATE INDEX pgweb_idx_2 ON tsearch . pgweb USING gin ( to_tsvector ( 'ngram' , body ));

-- 来源: 1152_file_1152
CREATE INDEX pgweb_idx_3 ON tsearch . pgweb USING gin ( to_tsvector ( 'english' , title || ' ' || body ));

-- 来源: 1152_file_1152
ALTER TABLE tsearch . pgweb ADD COLUMN textsearchable_index_col tsvector ;

-- 来源: 1152_file_1152
CREATE INDEX textsearch_idx_4 ON tsearch . pgweb USING gin ( textsearchable_index_col );

-- 来源: 1153_file_1153
create table table1 ( c_int int , c_bigint bigint , c_varchar varchar , c_text text ) with ( orientation = row , storage_type = ASTORE );

-- 来源: 1153_file_1153
create text search configuration ts_conf_1 ( parser = POUND );

-- 来源: 1153_file_1153
create text search configuration ts_conf_2 ( parser = POUND ) with ( split_flag = '%' );

-- 来源: 1153_file_1153
create index idx1 on table1 using gin ( to_tsvector ( c_text ));

-- 来源: 1153_file_1153
create index idx2 on tscp_u_m_005_tbl using gin ( to_tsvector ( c_text ));

-- 来源: 1155_file_1155
CREATE TABLE tsearch . tt ( id int , title text , keyword text , abstract text , body text , ti tsvector );

-- 来源: 1155_file_1155
DROP TABLE tsearch . tt ;

-- 来源: 1157_file_1157
CREATE TABLE tsearch . ts_ngram ( id int , body text );

-- 来源: 1162_file_1162
CREATE TABLE tsearch . aliases ( id int , t tsquery , s tsquery );

-- 来源: 1162_file_1162
DROP TABLE tsearch . aliases ;

-- 来源: 1166_file_1166
ALTER TEXT SEARCH CONFIGURATION astro_en ADD MAPPING FOR asciiword WITH astro_syn , english_ispell , english_stem ;

-- 来源: 1168_Simple
CREATE TEXT SEARCH DICTIONARY public . simple_dict ( TEMPLATE = pg_catalog . simple , STOPWORDS = english );

-- 来源: 1168_Simple
ALTER TEXT SEARCH DICTIONARY public . simple_dict ( Accept = false );

-- 来源: 1169_Synonym
CREATE TEXT SEARCH DICTIONARY my_synonym ( TEMPLATE = synonym , SYNONYMS = my_synonyms , FILEPATH = 'file:///home/dicts/' );

-- 来源: 1169_Synonym
ALTER TEXT SEARCH CONFIGURATION english ALTER MAPPING FOR asciiword WITH my_synonym , english_stem ;

-- 来源: 1169_Synonym
ALTER TEXT SEARCH DICTIONARY my_synonym ( CASESENSITIVE = true );

-- 来源: 1169_Synonym
CREATE TEXT SEARCH DICTIONARY syn ( TEMPLATE = synonym , SYNONYMS = synonym_sample );

-- 来源: 1169_Synonym
CREATE TEXT SEARCH CONFIGURATION tst ( copy = simple );

-- 来源: 1169_Synonym
ALTER TEXT SEARCH CONFIGURATION tst ALTER MAPPING FOR asciiword WITH syn ;

-- 来源: 1170_Thesaurus
CREATE TEXT SEARCH DICTIONARY thesaurus_astro ( TEMPLATE = thesaurus , DictFile = thesaurus_astro , Dictionary = pg_catalog . english_stem , FILEPATH = 'file:///home/dicts/' );

-- 来源: 1170_Thesaurus
ALTER TEXT SEARCH CONFIGURATION russian ALTER MAPPING FOR asciiword , asciihword , hword_asciipart WITH thesaurus_astro , english_stem ;

-- 来源: 1170_Thesaurus
ALTER TEXT SEARCH DICTIONARY thesaurus_astro ( DictFile = thesaurus_astro , FILEPATH = 'file:///home/dicts/' );

-- 来源: 1171_Ispell
CREATE TEXT SEARCH DICTIONARY norwegian_ispell ( TEMPLATE = ispell , DictFile = nn_no , AffFile = nn_no , FilePath = 'file:///home/dicts' );

-- 来源: 1173_file_1173
CREATE TEXT SEARCH CONFIGURATION ts_conf ( COPY = pg_catalog . english );

-- 来源: 1173_file_1173
CREATE TEXT SEARCH DICTIONARY gs_dict ( TEMPLATE = synonym , SYNONYMS = gs_dict , FILEPATH = 'file:///home/dicts' );

-- 来源: 1173_file_1173
CREATE TEXT SEARCH DICTIONARY english_ispell ( TEMPLATE = ispell , DictFile = english , AffFile = english , StopWords = english , FILEPATH = 'file:///home/dicts' );

-- 来源: 1173_file_1173
ALTER TEXT SEARCH CONFIGURATION ts_conf ALTER MAPPING FOR asciiword , asciihword , hword_asciipart , word , hword , hword_part WITH gs_dict , english_ispell , english_stem ;

-- 来源: 1173_file_1173
ALTER TEXT SEARCH CONFIGURATION ts_conf DROP MAPPING FOR email , url , url_path , sfloat , float ;

-- 来源: 1184_ABORT
CREATE TABLE customer_demographics_t1 ( CD_DEMO_SK INTEGER NOT NULL , CD_GENDER CHAR ( 1 ) , CD_MARITAL_STATUS CHAR ( 1 ) , CD_EDUCATION_STATUS CHAR ( 20 ) , CD_PURCHASE_ESTIMATE INTEGER , CD_CREDIT_RATING CHAR ( 10 ) , CD_DEP_COUNT INTEGER , CD_DEP_EMPLOYED_COUNT INTEGER , CD_DEP_COLLEGE_COUNT INTEGER ) DISTRIBUTE BY HASH ( CD_DEMO_SK );

-- 来源: 1184_ABORT
DROP TABLE customer_demographics_t1 ;

-- 来源: 1185_ALTER APP WORKLOAD GROUP MAPPING
CREATE RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "High" );

-- 来源: 1185_ALTER APP WORKLOAD GROUP MAPPING
CREATE WORKLOAD GROUP wg_hr1 USING RESOURCE POOL pool1 ;

-- 来源: 1185_ALTER APP WORKLOAD GROUP MAPPING
CREATE APP WORKLOAD GROUP MAPPING app_wg_map1 ;

-- 来源: 1185_ALTER APP WORKLOAD GROUP MAPPING
DROP APP WORKLOAD GROUP MAPPING app_wg_map1 ;

-- 来源: 1185_ALTER APP WORKLOAD GROUP MAPPING
DROP WORKLOAD GROUP wg_hr1 ;

-- 来源: 1185_ALTER APP WORKLOAD GROUP MAPPING
DROP RESOURCE POOL pool1 ;

-- 来源: 1188_ALTER DATABASE
CREATE DATABASE testdb;

--将testdb重命名为test_db1。
-- 来源: 1188_ALTER DATABASE
ALTER DATABASE testdb RENAME TO test_db1;

-- 来源: 1188_ALTER DATABASE
ALTER DATABASE test_db1 WITH CONNECTION LIMIT 100;

-- 来源: 1188_ALTER DATABASE
CREATE USER scott PASSWORD '********';

--将test_db1的所有者修改为jim。
-- 来源: 1188_ALTER DATABASE
ALTER DATABASE test_db1 OWNER TO scott;

-- 来源: 1188_ALTER DATABASE
CREATE TABLESPACE tbs_data1 RELATIVE LOCATION 'tablespace1/tbs_data1';

--修改test_db1默认表空间。
-- 来源: 1188_ALTER DATABASE
ALTER DATABASE test_db1 SET TABLESPACE tbs_data1;

-- 来源: 1188_ALTER DATABASE
CREATE USER jack PASSWORD '********';

-- 来源: 1188_ALTER DATABASE
CREATE TABLE test_tbl1(c1 int,c2 int);

-- 来源: 1188_ALTER DATABASE
ALTER DATABASE test_db1 ENABLE PRIVATE OBJECT;

-- 来源: 1188_ALTER DATABASE
DROP TABLE public.test_tbl1;

-- 来源: 1188_ALTER DATABASE
DROP DATABASE test_db1;

-- 来源: 1188_ALTER DATABASE
DROP TABLESPACE tbs_data1;

-- 来源: 1188_ALTER DATABASE
DROP USER jack;

-- 来源: 1188_ALTER DATABASE
DROP USER scott;

-- 来源: 1189_ALTER DATABASE LINK
CREATE USER user01 WITH SYSADMIN PASSWORD '********';

-- 来源: 1189_ALTER DATABASE LINK
DROP USER USER01 CASCADE;

-- 来源: 1190_ALTER DATA SOURCE
CREATE DATA SOURCE ds_test1 ;

-- 来源: 1190_ALTER DATA SOURCE
CREATE USER user_test1 IDENTIFIED BY '********' ;

-- 来源: 1190_ALTER DATA SOURCE
ALTER USER user_test1 WITH SYSADMIN ;

-- 来源: 1190_ALTER DATA SOURCE
DROP DATA SOURCE ds_test ;

-- 来源: 1190_ALTER DATA SOURCE
DROP USER user_test1 ;

-- 来源: 1191_ALTER DEFAULT PRIVILEGES
CREATE SCHEMA tpcds ;

-- 来源: 1191_ALTER DEFAULT PRIVILEGES
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds GRANT SELECT ON TABLES TO PUBLIC ;

-- 来源: 1191_ALTER DEFAULT PRIVILEGES
CREATE USER jack PASSWORD '******' ;

-- 来源: 1191_ALTER DEFAULT PRIVILEGES
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds GRANT INSERT ON TABLES TO jack ;

-- 来源: 1191_ALTER DEFAULT PRIVILEGES
ALTER DEFAULT PRIVILEGES FOR ROLE jack IN SCHEMA tpcds GRANT INSERT ON TABLES TO jack ;

-- 来源: 1191_ALTER DEFAULT PRIVILEGES
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds REVOKE SELECT ON TABLES FROM PUBLIC ;

-- 来源: 1191_ALTER DEFAULT PRIVILEGES
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds REVOKE INSERT ON TABLES FROM jack ;

-- 来源: 1191_ALTER DEFAULT PRIVILEGES
DROP USER jack ;

-- 来源: 1191_ALTER DEFAULT PRIVILEGES
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1192_ALTER DIRECTORY
CREATE OR REPLACE DIRECTORY dir as '/tmp/' ;

-- 来源: 1192_ALTER DIRECTORY
CREATE USER jim PASSWORD '********' ;

-- 来源: 1192_ALTER DIRECTORY
ALTER DIRECTORY dir OWNER TO jim ;

-- 来源: 1192_ALTER DIRECTORY
DROP DIRECTORY dir ;

-- 来源: 1193_ALTER FOREIGN TABLE ()
CREATE SCHEMA tpcds ;

-- 来源: 1193_ALTER FOREIGN TABLE ()
CREATE FOREIGN TABLE tpcds . customer_ft ( c_customer_sk integer , c_customer_id char ( 16 ) , c_current_cdemo_sk integer , c_current_hdemo_sk integer , c_current_addr_sk integer , c_first_shipto_date_sk integer , c_first_sales_date_sk integer , c_salutation char ( 10 ) , c_first_name char ( 20 ) , c_last_name char ( 30 ) , c_preferred_cust_flag char ( 1 ) , c_birth_day integer , c_birth_month integer , c_birth_year integer , c_birth_country varchar ( 20 ) , c_login char ( 13 ) , c_email_address char ( 50 ) , c_last_review_date char ( 10 ) ) SERVER gsmpp_server OPTIONS ( location 'gsfs://10.185.179.143:5000/customer1*.dat' , FORMAT 'TEXT' , DELIMITER '|' , encoding 'utf8' , mode 'Normal' ) READ ONLY ;

-- 来源: 1193_ALTER FOREIGN TABLE ()
ALTER FOREIGN TABLE tpcds . customer_ft options ( drop mode );

-- 来源: 1193_ALTER FOREIGN TABLE ()
DROP FOREIGN TABLE tpcds . customer_ft ;

-- 来源: 1193_ALTER FOREIGN TABLE ()
DROP SCHEMA tpcds CASCADE ;

-- 创建函数
-- 来源: 1194_ALTER FUNCTION
CREATE OR REPLACE FUNCTION test_func(a int) RETURN int IS proc_var int;

-- 删除函数
-- 来源: 1194_ALTER FUNCTION
DROP FUNCTION test_func;

-- 来源: 1195_ALTER GLOBAL CONFIGURATION
ALTER GLOBAL CONFIGURATION with ( redis_is_ok = true );

-- 来源: 1195_ALTER GLOBAL CONFIGURATION
ALTER GLOBAL CONFIGURATION with ( redis_is_ok = false );

-- 来源: 1195_ALTER GLOBAL CONFIGURATION
DROP GLOBAL CONFIGURATION redis_is_ok ;

-- 来源: 1196_ALTER GROUP
CREATE GROUP super_users WITH PASSWORD "********" ;

-- 来源: 1196_ALTER GROUP
CREATE ROLE lche WITH PASSWORD "********" ;

-- 来源: 1196_ALTER GROUP
CREATE ROLE jim WITH PASSWORD "********" ;

-- 来源: 1196_ALTER GROUP
ALTER GROUP super_users ADD USER lche , jim ;

-- 来源: 1196_ALTER GROUP
ALTER GROUP super_users DROP USER jim ;

-- 来源: 1196_ALTER GROUP
ALTER GROUP super_users RENAME TO normal_users ;

-- 来源: 1196_ALTER GROUP
DROP ROLE lche , jim ;

-- 来源: 1196_ALTER GROUP
DROP GROUP normal_users ;

-- 来源: 1197_ALTER INDEX
CREATE TABLE test1(col1 int, col2 int);

-- 来源: 1197_ALTER INDEX
CREATE INDEX aa ON test1(col1);

--将索引aa重命名为idx_test1_col1。
-- 来源: 1197_ALTER INDEX
ALTER INDEX aa RENAME TO idx_test1_col1;

-- 来源: 1197_ALTER INDEX
CREATE TABLESPACE tbs_index1 RELATIVE LOCATION 'tablespace1/tbs_index1';

--修改索引idx_test1_col1的所属表空间为tbs_index1。
-- 来源: 1197_ALTER INDEX
ALTER INDEX IF EXISTS idx_test1_col1 SET TABLESPACE tbs_index1;

--修改索引idx_test1_col1 的填充因子。
-- 来源: 1197_ALTER INDEX
ALTER INDEX IF EXISTS idx_test1_col1 SET (FILLFACTOR = 70);

-- 来源: 1197_ALTER INDEX
ALTER INDEX IF EXISTS idx_test1_col1 RESET (FILLFACTOR);

-- 来源: 1197_ALTER INDEX
ALTER INDEX IF EXISTS idx_test1_col1 UNUSABLE;

--重建索引idx_test1_col1。
-- 来源: 1197_ALTER INDEX
ALTER INDEX idx_test1_col1 REBUILD;

--删除。
-- 来源: 1197_ALTER INDEX
DROP INDEX idx_test1_col1;

-- 来源: 1197_ALTER INDEX
DROP TABLE test1;

-- 来源: 1197_ALTER INDEX
DROP TABLESPACE tbs_index1;

-- 来源: 1197_ALTER INDEX
CREATE TABLE test2(col1 int, col2 int) PARTITION BY RANGE (col1)( PARTITION p1 VALUES LESS THAN (100), PARTITION p2 VALUES LESS THAN (200) );

--创建分区索引。
-- 来源: 1197_ALTER INDEX
CREATE INDEX idx_test2_col1 ON test2(col1) LOCAL( PARTITION p1, PARTITION p2 );

--重命名索引分区。
-- 来源: 1197_ALTER INDEX
ALTER INDEX idx_test2_col1 RENAME PARTITION p1 TO p1_test2_idx;

-- 来源: 1197_ALTER INDEX
ALTER INDEX idx_test2_col1 RENAME PARTITION p2 TO p2_test2_idx;

-- 来源: 1197_ALTER INDEX
CREATE TABLESPACE tbs_index2 RELATIVE LOCATION 'tablespace1/tbs_index2';

-- 来源: 1197_ALTER INDEX
CREATE TABLESPACE tbs_index3 RELATIVE LOCATION 'tablespace1/tbs_index3';

--修改索引idx_test2_col1分区的所属表空间。
-- 来源: 1197_ALTER INDEX
ALTER INDEX idx_test2_col1 MOVE PARTITION p1_test2_idx TABLESPACE tbs_index2;

-- 来源: 1197_ALTER INDEX
ALTER INDEX idx_test2_col1 MOVE PARTITION p2_test2_idx TABLESPACE tbs_index3;

--删除。
-- 来源: 1197_ALTER INDEX
DROP INDEX idx_test2_col1;

-- 来源: 1197_ALTER INDEX
DROP TABLE test2;

-- 来源: 1197_ALTER INDEX
DROP TABLESPACE tbs_index2;

-- 来源: 1197_ALTER INDEX
DROP TABLESPACE tbs_index3;

-- 来源: 1200_ALTER MASKING POLICY
CREATE USER dev_mask PASSWORD '********' ;

-- 来源: 1200_ALTER MASKING POLICY
CREATE USER bob_mask PASSWORD '********' ;

-- 来源: 1200_ALTER MASKING POLICY
CREATE TABLE tb_for_masking ( col1 text , col2 text , col3 text );

-- 来源: 1200_ALTER MASKING POLICY
CREATE RESOURCE LABEL mask_lb1 ADD COLUMN ( tb_for_masking . col1 );

-- 来源: 1200_ALTER MASKING POLICY
CREATE RESOURCE LABEL mask_lb2 ADD COLUMN ( tb_for_masking . col2 );

-- 来源: 1200_ALTER MASKING POLICY
CREATE MASKING POLICY maskpol1 maskall ON LABEL ( mask_lb1 );

-- 来源: 1200_ALTER MASKING POLICY
ALTER MASKING POLICY maskpol1 COMMENTS 'masking policy for tb_for_masking.col1' ;

-- 来源: 1200_ALTER MASKING POLICY
ALTER MASKING POLICY maskpol1 ADD randommasking ON LABEL ( mask_lb2 );

-- 来源: 1200_ALTER MASKING POLICY
ALTER MASKING POLICY maskpol1 REMOVE randommasking ON LABEL ( mask_lb2 );

-- 来源: 1200_ALTER MASKING POLICY
ALTER MASKING POLICY maskpol1 MODIFY randommasking ON LABEL ( mask_lb1 );

-- 来源: 1200_ALTER MASKING POLICY
ALTER MASKING POLICY maskpol1 MODIFY ( FILTER ON ROLES ( dev_mask , bob_mask ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' ));

-- 来源: 1200_ALTER MASKING POLICY
ALTER MASKING POLICY maskpol1 DROP FILTER ;

-- 来源: 1200_ALTER MASKING POLICY
ALTER MASKING POLICY maskpol1 DISABLE ;

-- 来源: 1200_ALTER MASKING POLICY
DROP MASKING POLICY maskpol1 ;

-- 来源: 1200_ALTER MASKING POLICY
DROP RESOURCE LABEL mask_lb1 , mask_lb2 ;

-- 来源: 1200_ALTER MASKING POLICY
DROP TABLE tb_for_masking ;

-- 来源: 1200_ALTER MASKING POLICY
DROP USER dev_mask , bob_mask ;

-- 来源: 1201_ALTER MATERIALIZED VIEW
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建全量物化视图。
-- 来源: 1201_ALTER MATERIALIZED VIEW
CREATE MATERIALIZED VIEW foo AS SELECT * FROM my_table;

--把物化视图foo重命名为bar。
-- 来源: 1201_ALTER MATERIALIZED VIEW
ALTER MATERIALIZED VIEW foo RENAME TO bar;

--删除全量物化视图。
-- 来源: 1201_ALTER MATERIALIZED VIEW
DROP MATERIALIZED VIEW bar;

--删除表my_table。
-- 来源: 1201_ALTER MATERIALIZED VIEW
DROP TABLE my_table;

-- 创建包
-- 来源: 1204_ALTER PACKAGE
CREATE OR REPLACE PACKAGE TEST_PKG AS pkg_var int := 1;

-- 来源: 1204_ALTER PACKAGE
CREATE OR REPLACE PACKAGE BODY TEST_PKG AS PROCEDURE test_pkg_proc(var int) IS BEGIN pkg_var := 1;

-- 重编译包
-- 来源: 1204_ALTER PACKAGE
ALTER PACKAGE test_pkg COMPILE;

-- 删除包
-- 来源: 1204_ALTER PACKAGE
DROP PACKAGE TEST_PKG;

-- 来源: 1205_ALTER RESOURCE LABEL
CREATE TABLE table_for_label ( col1 int , col2 text );

-- 来源: 1205_ALTER RESOURCE LABEL
CREATE RESOURCE LABEL table_label ADD COLUMN ( table_for_label . col1 );

-- 来源: 1205_ALTER RESOURCE LABEL
ALTER RESOURCE LABEL table_label ADD COLUMN ( table_for_label . col2 );

-- 来源: 1205_ALTER RESOURCE LABEL
ALTER RESOURCE LABEL table_label REMOVE COLUMN ( table_for_label . col1 );

-- 来源: 1205_ALTER RESOURCE LABEL
DROP RESOURCE LABEL table_label ;

-- 来源: 1205_ALTER RESOURCE LABEL
DROP TABLE table_for_label ;

-- 来源: 1206_ALTER RESOURCE POOL
CREATE RESOURCE POOL pool1 ;

-- 来源: 1206_ALTER RESOURCE POOL
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "High" );

-- 来源: 1206_ALTER RESOURCE POOL
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "class1:Low" );

-- 来源: 1206_ALTER RESOURCE POOL
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "class1:wg1" );

-- 来源: 1206_ALTER RESOURCE POOL
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "class1:wg2:3" );

-- 来源: 1206_ALTER RESOURCE POOL
DROP RESOURCE POOL pool1 ;

-- 来源: 1208_ALTER ROW LEVEL SECURITY POLICY
CREATE TABLE all_data ( id int , role varchar ( 100 ), data varchar ( 100 ));

-- 来源: 1208_ALTER ROW LEVEL SECURITY POLICY
CREATE ROLE alice WITH PASSWORD "********" ;

-- 来源: 1208_ALTER ROW LEVEL SECURITY POLICY
CREATE ROLE bob WITH PASSWORD "********" ;

-- 来源: 1208_ALTER ROW LEVEL SECURITY POLICY
DROP ROLE alice , bob ;

-- 来源: 1208_ALTER ROW LEVEL SECURITY POLICY
DROP TABLE all_data ;

-- 来源: 1209_ALTER SCHEMA
CREATE DATABASE test1 WITH DBCOMPATIBILITY = 'MYSQL' ;

-- 来源: 1209_ALTER SCHEMA
CREATE SCHEMA ds ;

-- 来源: 1209_ALTER SCHEMA
ALTER SCHEMA ds RENAME TO ds_new ;

-- 来源: 1209_ALTER SCHEMA
CREATE USER jack PASSWORD '********' ;

-- 来源: 1209_ALTER SCHEMA
ALTER SCHEMA ds_new OWNER TO jack ;

-- 来源: 1209_ALTER SCHEMA
CREATE SCHEMA sch ;

-- 来源: 1209_ALTER SCHEMA
ALTER SCHEMA sch CHARACTER SET utf8mb4 COLLATE utf8mb4_bin ;

-- 来源: 1209_ALTER SCHEMA
DROP SCHEMA ds_new ;

-- 来源: 1209_ALTER SCHEMA
DROP SCHEMA sch ;

-- 来源: 1209_ALTER SCHEMA
DROP USER jack ;

-- 来源: 1209_ALTER SCHEMA
DROP DATABASE test1 ;

-- 来源: 1210_ALTER SEQUENCE
CREATE SEQUENCE serial START 101 ;

-- 来源: 1210_ALTER SEQUENCE
CREATE TABLE t1 ( c1 bigint default nextval ( 'serial' ));

-- 来源: 1210_ALTER SEQUENCE
ALTER SEQUENCE serial OWNED BY t1 . c1 ;

-- 来源: 1210_ALTER SEQUENCE
DROP SEQUENCE serial CASCADE ;

-- 来源: 1210_ALTER SEQUENCE
DROP TABLE t1 ;

-- 来源: 1211_ALTER SERVER
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw ;

-- 来源: 1211_ALTER SERVER
ALTER SERVER my_server RENAME TO my_server_1 ;

-- 来源: 1211_ALTER SERVER
DROP SERVER my_server_1 ;

-- 来源: 1212_ALTER SESSION
CREATE SCHEMA ds ;

-- 来源: 1212_ALTER SESSION
ALTER SESSION SET NAMES 'UTF8' ;

-- 来源: 1212_ALTER SESSION
ALTER SESSION SET CURRENT_SCHEMA TO tpcds ;

-- 来源: 1212_ALTER SESSION
ALTER SESSION SET XML OPTION DOCUMENT ;

-- 来源: 1212_ALTER SESSION
CREATE ROLE joe WITH PASSWORD '********' ;

-- 来源: 1212_ALTER SESSION
ALTER SESSION SET SESSION AUTHORIZATION joe PASSWORD '********' ;

-- 来源: 1212_ALTER SESSION
DROP SCHEMA ds ;

-- 来源: 1212_ALTER SESSION
DROP ROLE joe ;

-- 来源: 1212_ALTER SESSION
ALTER SESSION SET TRANSACTION READ ONLY ;

-- 来源: 1213_ALTER SYNONYM
CREATE USER sysadmin WITH SYSADMIN PASSWORD '********' ;

-- 来源: 1213_ALTER SYNONYM
CREATE OR REPLACE SYNONYM t1 FOR ot . t1 ;

-- 来源: 1213_ALTER SYNONYM
CREATE USER u1 PASSWORD '********' ;

-- 来源: 1213_ALTER SYNONYM
ALTER SYNONYM t1 OWNER TO u1 ;

-- 来源: 1213_ALTER SYNONYM
DROP SYNONYM t1 ;

-- 来源: 1213_ALTER SYNONYM
DROP USER u1 ;

-- 来源: 1213_ALTER SYNONYM
DROP USER sysadmin ;

-- 来源: 1214_ALTER SYSTEM KILL SESSION
ALTER SYSTEM KILL SESSION '140469417232128,0' IMMEDIATE ;

-- 来源: 1215_ALTER TABLE
CREATE TABLE aa(c1 int, c2 int);

-- 来源: 1215_ALTER TABLE
ALTER TABLE IF EXISTS aa RENAME TO test_alt1;

-- 来源: 1215_ALTER TABLE
CREATE SCHEMA test_schema;

--把表test_alt1的所属模式修改为test_schema。
-- 来源: 1215_ALTER TABLE
ALTER TABLE test_alt1 SET SCHEMA test_schema;

-- 来源: 1215_ALTER TABLE
CREATE USER test_user PASSWORD '********';

-- 修改test_alt1表的所有者为test_user;
-- 来源: 1215_ALTER TABLE
ALTER TABLE IF EXISTS test_schema.test_alt1 OWNER TO test_user;

-- 来源: 1215_ALTER TABLE
CREATE TABLESPACE tbs_data1 RELATIVE LOCATION 'tablespace1/tbs_data1';

--修改test_alt1表的空间为tbs_data1。
-- 来源: 1215_ALTER TABLE
ALTER TABLE test_schema.test_alt1 SET TABLESPACE tbs_data1;

--删除。
-- 来源: 1215_ALTER TABLE
DROP TABLE test_schema.test_alt1;

-- 来源: 1215_ALTER TABLE
DROP TABLESPACE tbs_data1;

-- 来源: 1215_ALTER TABLE
DROP SCHEMA test_schema;

-- 来源: 1215_ALTER TABLE
DROP USER test_user;

-- 来源: 1215_ALTER TABLE
CREATE TABLE test_alt2(c1 INT,c2 INT);

-- 修改列名
-- 来源: 1215_ALTER TABLE
ALTER TABLE test_alt2 RENAME c1 TO id;

-- 来源: 1215_ALTER TABLE
ALTER TABLE test_alt2 RENAME COLUMN c2 to areaid;

-- 来源: 1215_ALTER TABLE
ALTER TABLE IF EXISTS test_alt2 ADD COLUMN name VARCHAR(20);

-- 来源: 1215_ALTER TABLE
ALTER TABLE test_alt2 MODIFY name VARCHAR(50);

-- 来源: 1215_ALTER TABLE
ALTER TABLE test_alt2 ALTER COLUMN name TYPE VARCHAR(25);

-- 来源: 1215_ALTER TABLE
ALTER TABLE test_alt2 DROP COLUMN areaid;

--修改test_alt2表中name字段的存储模式。
-- 来源: 1215_ALTER TABLE
ALTER TABLE test_alt2 ALTER COLUMN name SET STORAGE PLAIN;

--删除。
-- 来源: 1215_ALTER TABLE
DROP TABLE test_alt2;

-- 来源: 1215_ALTER TABLE
CREATE TABLE test_alt3(pid INT, areaid CHAR(5), name VARCHAR(20));

--为pid添加非空约束。
-- 来源: 1215_ALTER TABLE
ALTER TABLE test_alt3 MODIFY pid NOT NULL;

-- 来源: 1215_ALTER TABLE
ALTER TABLE test_alt3 MODIFY pid NULL;

-- 来源: 1215_ALTER TABLE
ALTER TABLE test_alt3 ALTER COLUMN areaid SET DEFAULT '00000';

-- 来源: 1215_ALTER TABLE
ALTER TABLE test_alt3 ALTER COLUMN areaid DROP DEFAULT;

-- 来源: 1215_ALTER TABLE
ALTER TABLE test_alt3 ADD CONSTRAINT pk_test3_pid PRIMARY KEY (pid);

-- 来源: 1215_ALTER TABLE
CREATE TABLE test_alt4(c1 INT, c2 INT);

--建索引。
-- 来源: 1215_ALTER TABLE
CREATE UNIQUE INDEX pk_test4_c1 ON test_alt4(c1);

--添加约束时关联已经创建的索引。
-- 来源: 1215_ALTER TABLE
ALTER TABLE test_alt4 ADD CONSTRAINT pk_test4_c1 PRIMARY KEY USING INDEX pk_test4_c1;

--删除。
-- 来源: 1215_ALTER TABLE
DROP TABLE test_alt4;

-- 来源: 1215_ALTER TABLE
ALTER TABLE test_alt3 DROP CONSTRAINT IF EXISTS pk_test3_pid;

--删除。
-- 来源: 1215_ALTER TABLE
DROP TABLE test_alt3;

-- 来源: 1218_ALTER TEXT SEARCH CONFIGURATION
CREATE TEXT SEARCH CONFIGURATION english_1 ( parser = default );

-- 来源: 1218_ALTER TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION english_1 ADD MAPPING FOR word WITH simple , english_stem ;

-- 来源: 1218_ALTER TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION english_1 ADD MAPPING FOR email WITH english_stem , french_stem ;

-- 来源: 1218_ALTER TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION english_1 ALTER MAPPING REPLACE french_stem with german_stem ;

-- 来源: 1218_ALTER TEXT SEARCH CONFIGURATION
DROP TEXT SEARCH CONFIGURATION english_1 ;

-- 来源: 1219_ALTER TEXT SEARCH DICTIONARY
CREATE TEXT SEARCH DICTIONARY my_dict ( TEMPLATE = Simple );

-- 来源: 1219_ALTER TEXT SEARCH DICTIONARY
ALTER TEXT SEARCH DICTIONARY my_dict ( Accept = true );

-- 来源: 1219_ALTER TEXT SEARCH DICTIONARY
ALTER TEXT SEARCH DICTIONARY my_dict ( Accept );

-- 来源: 1219_ALTER TEXT SEARCH DICTIONARY
ALTER TEXT SEARCH DICTIONARY my_dict ( dummy );

-- 来源: 1219_ALTER TEXT SEARCH DICTIONARY
DROP TEXT SEARCH DICTIONARY my_dict ;

-- 来源: 1223_ALTER VIEW
CREATE TABLE test_tb1(col1 INT,col2 INT);

--创建视图。
-- 来源: 1223_ALTER VIEW
CREATE VIEW abc AS SELECT * FROM test_tb1;

--重命名视图。
-- 来源: 1223_ALTER VIEW
ALTER VIEW IF EXISTS abc RENAME TO test_v1;

-- 来源: 1223_ALTER VIEW
CREATE ROLE role_test PASSWORD '********';

--修改视图所有者。
-- 来源: 1223_ALTER VIEW
ALTER VIEW IF EXISTS test_v1 OWNER TO role_test;

-- 来源: 1223_ALTER VIEW
CREATE SCHEMA tcpds;

--修改视图所属模式。
-- 来源: 1223_ALTER VIEW
ALTER VIEW test_v1 SET SCHEMA tcpds;

-- 来源: 1223_ALTER VIEW
ALTER VIEW tcpds.test_v1 SET (security_barrier = TRUE);

--重置视图选项。
-- 来源: 1223_ALTER VIEW
ALTER VIEW tcpds.test_v1 RESET (security_barrier);

--删除视图test_v1。
-- 来源: 1223_ALTER VIEW
DROP VIEW tcpds.test_v1;

--删除表test_tb1。
-- 来源: 1223_ALTER VIEW
DROP TABLE test_tb1;

--删除用户。
-- 来源: 1223_ALTER VIEW
DROP ROLE role_test;

--删除schema。
-- 来源: 1223_ALTER VIEW
DROP SCHEMA tcpds;

-- 来源: 1224_ALTER WORKLOAD GROUP
CREATE RESOURCE POOL pool1 ;

-- 来源: 1224_ALTER WORKLOAD GROUP
CREATE WORKLOAD GROUP group1 ;

-- 来源: 1224_ALTER WORKLOAD GROUP
ALTER WORKLOAD GROUP group1 USING RESOURCE POOL pool1 WITH ( ACT_STATEMENTS = 10 );

-- 来源: 1224_ALTER WORKLOAD GROUP
DROP WORKLOAD GROUP group1 ;

-- 来源: 1224_ALTER WORKLOAD GROUP
DROP RESOURCE POOL pool1 ;

-- 来源: 1225_ANALYZE _ ANALYSE
CREATE TABLE customer_info ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL , WR_REFUNDED_CUSTOMER_SK INTEGER ) DISTRIBUTE BY HASH ( WR_ITEM_SK );

-- 来源: 1225_ANALYZE _ ANALYSE
CREATE TABLE customer_par ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL , WR_REFUNDED_CUSTOMER_SK INTEGER ) DISTRIBUTE BY HASH ( WR_ITEM_SK ) PARTITION BY RANGE ( WR_RETURNED_DATE_SK ) ( PARTITION P1 VALUES LESS THAN ( 2452275 ), PARTITION P2 VALUES LESS THAN ( 2452640 ), PARTITION P3 VALUES LESS THAN ( 2453000 ), PARTITION P4 VALUES LESS THAN ( MAXVALUE ) ) ENABLE ROW MOVEMENT ;

-- 来源: 1225_ANALYZE _ ANALYSE
DROP TABLE customer_info ;

-- 来源: 1225_ANALYZE _ ANALYSE
DROP TABLE customer_par ;

-- 来源: 1229_CALL
CREATE FUNCTION func_add_sql ( num1 integer , num2 integer ) RETURN integer AS BEGIN RETURN num1 + num2 ;

-- 来源: 1229_CALL
DROP FUNCTION func_add_sql ;

-- 来源: 1229_CALL
CREATE FUNCTION func_increment_sql ( num1 IN integer , num2 IN integer , res OUT integer ) RETURN integer AS BEGIN res : = num1 + num2 ;

-- 来源: 1229_CALL
DROP FUNCTION func_increment_sql ;

-- 来源: 1231_CLEAN CONNECTION
CREATE DATABASE test_clean_connection ;

-- 来源: 1231_CLEAN CONNECTION
CREATE USER jack PASSWORD '********' ;

-- 来源: 1231_CLEAN CONNECTION
CREATE DATABASE testdb ;

-- 来源: 1231_CLEAN CONNECTION
DROP DATABASE testdb ;

-- 来源: 1231_CLEAN CONNECTION
DROP USER jack ;

-- 来源: 1231_CLEAN CONNECTION
DROP DATABASE test_clean_connection ;

-- 来源: 1233_CLUSTER
CREATE TABLE test_c1 ( id int , name varchar ( 20 ));

-- 来源: 1233_CLUSTER
CREATE INDEX idx_test_c1_id ON test_c1 ( id );

-- 来源: 1233_CLUSTER
DROP TABLE test_c1 ;

-- 来源: 1233_CLUSTER
CREATE TABLE test(col1 int,CONSTRAINT pk_test PRIMARY KEY (col1));

-- 删除
-- 来源: 1233_CLUSTER
DROP TABLE test;

-- 来源: 1233_CLUSTER
CREATE TABLE test_c2(id int, info varchar(4)) PARTITION BY RANGE (id)( PARTITION p1 VALUES LESS THAN (11), PARTITION p2 VALUES LESS THAN (21) );

-- 来源: 1233_CLUSTER
CREATE INDEX idx_test_c2_id1 ON test_c2(id);

-- 删除
-- 来源: 1233_CLUSTER
DROP TABLE test_c2;

-- 来源: 1234_COMMENT
CREATE TABLE emp( empno varchar(7), ename varchar(50), job varchar(50), mgr varchar(7), deptno int );

--表添加注释
-- 来源: 1234_COMMENT
COMMENT ON TABLE emp IS '部门表';

--字段添加注释
-- 来源: 1234_COMMENT
COMMENT ON COLUMN emp.empno IS '员工编号';

-- 来源: 1234_COMMENT
COMMENT ON COLUMN emp.ename IS '员工姓名';

-- 来源: 1234_COMMENT
COMMENT ON COLUMN emp.job IS '职务';

-- 来源: 1234_COMMENT
COMMENT ON COLUMN emp.mgr IS '上司编号';

-- 来源: 1234_COMMENT
COMMENT ON COLUMN emp.deptno IS '部门编号';

--删除
-- 来源: 1234_COMMENT
DROP TABLE emp;

-- 来源: 1235_COMMIT _ END
CREATE SCHEMA tpcds ;

-- 来源: 1235_COMMIT _ END
CREATE TABLE tpcds . customer_demographics_t2 ( CD_DEMO_SK INTEGER NOT NULL , CD_GENDER CHAR ( 1 ) , CD_MARITAL_STATUS CHAR ( 1 ) , CD_EDUCATION_STATUS CHAR ( 20 ) , CD_PURCHASE_ESTIMATE INTEGER , CD_CREDIT_RATING CHAR ( 10 ) , CD_DEP_COUNT INTEGER , CD_DEP_EMPLOYED_COUNT INTEGER , CD_DEP_COLLEGE_COUNT INTEGER ) DISTRIBUTE BY HASH ( CD_DEMO_SK );

-- 来源: 1235_COMMIT _ END
DROP TABLE tpcds . customer_demographics_t2 ;

-- 来源: 1235_COMMIT _ END
DROP SCHEMA tpcds ;

-- 来源: 1236_COMMIT PREPARED
CREATE TABLE item1 ( id int );

-- 来源: 1236_COMMIT PREPARED
DROP TABLE item1 ;

-- 来源: 1237_COPY
CREATE SCHEMA tpcds ;

-- 来源: 1237_COPY
CREATE TABLE tpcds . ship_mode ( SM_SHIP_MODE_SK INTEGER NOT NULL , SM_SHIP_MODE_ID CHAR ( 16 ) NOT NULL , SM_TYPE CHAR ( 30 ) , SM_CODE CHAR ( 10 ) , SM_CARRIER CHAR ( 20 ) , SM_CONTRACT CHAR ( 20 ) ) DISTRIBUTE BY HASH ( SM_SHIP_MODE_SK );

-- 来源: 1237_COPY
CREATE TABLE tpcds . ship_mode_t1 ( SM_SHIP_MODE_SK INTEGER NOT NULL , SM_SHIP_MODE_ID CHAR ( 16 ) NOT NULL , SM_TYPE CHAR ( 30 ) , SM_CODE CHAR ( 10 ) , SM_CARRIER CHAR ( 20 ) , SM_CONTRACT CHAR ( 20 ) ) DISTRIBUTE BY HASH ( SM_SHIP_MODE_SK );

-- 来源: 1237_COPY
DROP TABLE tpcds . ship_mode ;

-- 来源: 1237_COPY
DROP TABLE tpcds . ship_mode_t1 ;

-- 来源: 1237_COPY
DROP SCHEMA tpcds ;

-- 来源: 1238_CREATE APP WORKLOAD GROUP MAPPING
CREATE RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "High" );

-- 来源: 1238_CREATE APP WORKLOAD GROUP MAPPING
CREATE WORKLOAD GROUP group1 USING RESOURCE POOL pool1 ;

-- 来源: 1238_CREATE APP WORKLOAD GROUP MAPPING
CREATE APP WORKLOAD GROUP MAPPING app_wg_map1 WITH ( WORKLOAD_GPNAME = group1 );

-- 来源: 1238_CREATE APP WORKLOAD GROUP MAPPING
CREATE APP WORKLOAD GROUP MAPPING app_wg_map2 ;

-- 来源: 1238_CREATE APP WORKLOAD GROUP MAPPING
DROP APP WORKLOAD GROUP MAPPING app_wg_map1 ;

-- 来源: 1238_CREATE APP WORKLOAD GROUP MAPPING
DROP APP WORKLOAD GROUP MAPPING app_wg_map2 ;

-- 来源: 1238_CREATE APP WORKLOAD GROUP MAPPING
DROP WORKLOAD GROUP group1 ;

-- 来源: 1238_CREATE APP WORKLOAD GROUP MAPPING
DROP RESOURCE POOL pool1 ;

-- 来源: 1239_CREATE AUDIT POLICY
CREATE USER dev_audit PASSWORD '*********' ;

-- 来源: 1239_CREATE AUDIT POLICY
CREATE USER bob_audit PASSWORD '*********' ;

-- 来源: 1239_CREATE AUDIT POLICY
CREATE TABLE tb_for_audit ( col1 text , col2 text , col3 text );

-- 来源: 1239_CREATE AUDIT POLICY
CREATE RESOURCE LABEL adt_lb0 ADD TABLE ( tb_for_audit );

-- 来源: 1239_CREATE AUDIT POLICY
CREATE AUDIT POLICY adt1 PRIVILEGES CREATE ;

-- 来源: 1239_CREATE AUDIT POLICY
CREATE AUDIT POLICY adt2 ACCESS SELECT ;

-- 来源: 1239_CREATE AUDIT POLICY
CREATE AUDIT POLICY adt3 PRIVILEGES CREATE ON LABEL ( adt_lb0 ) FILTER ON ROLES ( dev_audit , bob_audit );

-- 来源: 1239_CREATE AUDIT POLICY
CREATE AUDIT POLICY adt4 ACCESS SELECT ON LABEL ( adt_lb0 ), INSERT ON LABEL ( adt_lb0 ), DELETE FILTER ON ROLES ( dev_audit , bob_audit ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' );

-- 来源: 1239_CREATE AUDIT POLICY
ALTER AUDIT POLICY adt4 REMOVE ACCESS ( SELECT ON LABEL ( adt_lb0 ));

-- 来源: 1239_CREATE AUDIT POLICY
DROP AUDIT POLICY adt1 , adt2 , adt3 , adt4 ;

-- 来源: 1239_CREATE AUDIT POLICY
DROP RESOURCE LABEL adt_lb0 ;

-- 来源: 1239_CREATE AUDIT POLICY
DROP TABLE tb_for_audit ;

-- 来源: 1239_CREATE AUDIT POLICY
DROP USER dev_audit , bob_audit ;

-- 来源: 1240_CREATE BARRIER
CREATE BARRIER 'barrier1' ;

-- 来源: 1242_CREATE DATABASE
CREATE USER jim PASSWORD '********';

--创建一个GBK编码的数据库testdb1。
-- 来源: 1242_CREATE DATABASE
CREATE DATABASE testdb1 ENCODING 'GBK' template = template0;

-- 来源: 1242_CREATE DATABASE
CREATE DATABASE testdb2 OWNER jim DBCOMPATIBILITY = 'ORA';

--创建兼容ORA格式的数据库并指定时区。
-- 来源: 1242_CREATE DATABASE
CREATE DATABASE testdb3 DBCOMPATIBILITY 'ORA' DBTIMEZONE='+08:00';

-- 来源: 1242_CREATE DATABASE
DROP DATABASE testdb1;

-- 来源: 1242_CREATE DATABASE
DROP DATABASE testdb2;

-- 来源: 1242_CREATE DATABASE
DROP DATABASE testdb3;

-- 来源: 1243_CREATE DATABASE LINK
CREATE USER user01 WITH SYSADMIN PASSWORD '********';

-- 来源: 1243_CREATE DATABASE LINK
CREATE DATABASE LINK private_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

-- 来源: 1243_CREATE DATABASE LINK
DROP DATABASE LINK private_dblink;

-- 来源: 1243_CREATE DATABASE LINK
DROP USER user01 CASCADE;

-- 来源: 1244_CREATE DATA SOURCE
CREATE DATA SOURCE ds_test1 ;

-- 来源: 1244_CREATE DATA SOURCE
CREATE DATA SOURCE ds_test2 TYPE 'MPPDB' VERSION NULL ;

-- 来源: 1244_CREATE DATA SOURCE
CREATE DATA SOURCE ds_test3 OPTIONS ( dsn 'GaussDB' , encoding 'utf8' );

-- 来源: 1244_CREATE DATA SOURCE
CREATE DATA SOURCE ds_test4 TYPE 'unknown' VERSION '11.2.3' OPTIONS ( dsn 'GaussDB' , username 'userid' , password '********' , encoding '' );

-- 来源: 1244_CREATE DATA SOURCE
DROP DATA SOURCE ds_test1 ;

-- 来源: 1244_CREATE DATA SOURCE
DROP DATA SOURCE ds_test2 ;

-- 来源: 1244_CREATE DATA SOURCE
DROP DATA SOURCE ds_test3 ;

-- 来源: 1244_CREATE DATA SOURCE
DROP DATA SOURCE ds_test4 ;

-- 来源: 1245_CREATE DIRECTORY
CREATE OR REPLACE DIRECTORY dir AS '/tmp/' ;

-- 来源: 1245_CREATE DIRECTORY
DROP DIRECTORY dir ;

-- 来源: 1246_CREATE EXTENSION
CREATE EXTENSION IF NOT EXISTS security_plugin;

-- 来源: 1247_CREATE FOREIGN TABLE ()
CREATE FOREIGN TABLE foreign_HR_staffS ( staff_ID NUMBER ( 6 ) , FIRST_NAME VARCHAR2 ( 20 ), LAST_NAME VARCHAR2 ( 25 ), EMAIL VARCHAR2 ( 25 ), PHONE_NUMBER VARCHAR2 ( 20 ), HIRE_DATE DATE , employment_ID VARCHAR2 ( 10 ), SALARY NUMBER ( 8 , 2 ), COMMISSION_PCT NUMBER ( 2 , 2 ), MANAGER_ID NUMBER ( 6 ), section_ID NUMBER ( 4 ) ) SERVER gsmpp_server OPTIONS ( location 'gsfs://192.168.0.90:5000/* | gsfs://192.168.0.91:5000/*' , format 'TEXT' , delimiter E '\x20' , null '' ) WITH err_HR_staffS ;

-- 来源: 1247_CREATE FOREIGN TABLE ()
CREATE FOREIGN TABLE foreign_HR_staffS_ft3 ( staff_ID NUMBER ( 6 ) , FIRST_NAME VARCHAR2 ( 20 ), LAST_NAME VARCHAR2 ( 25 ), EMAIL VARCHAR2 ( 25 ), PHONE_NUMBER VARCHAR2 ( 20 ), HIRE_DATE DATE , employment_ID VARCHAR2 ( 10 ), SALARY NUMBER ( 8 , 2 ), COMMISSION_PCT NUMBER ( 2 , 2 ), MANAGER_ID NUMBER ( 6 ), section_ID NUMBER ( 4 ) ) SERVER gsmpp_server OPTIONS ( location 'gsfs://192.168.0.90:5000/* | gsfs://192.168.0.91:5000/*' , format 'TEXT' , delimiter E '\x20' , null '' , reject_limit '2' ) WITH err_HR_staffS_ft3 ;

-- 来源: 1247_CREATE FOREIGN TABLE ()
CREATE FOREIGN TABLE foreign_HR_staffS_ft1 ( staff_ID NUMBER ( 6 ) , FIRST_NAME VARCHAR2 ( 20 ), LAST_NAME VARCHAR2 ( 25 ), EMAIL VARCHAR2 ( 25 ), PHONE_NUMBER VARCHAR2 ( 20 ), HIRE_DATE DATE , employment_ID VARCHAR2 ( 10 ), SALARY NUMBER ( 8 , 2 ), COMMISSION_PCT NUMBER ( 2 , 2 ), MANAGER_ID NUMBER ( 6 ), section_ID NUMBER ( 4 ) ) SERVER gsmpp_server OPTIONS ( location 'file:///input_data/*' , format 'csv' , mode 'private' , delimiter ',' ) WITH err_HR_staffS_ft1 ;

-- 来源: 1247_CREATE FOREIGN TABLE ()
CREATE FOREIGN TABLE foreign_HR_staffS_ft2 ( staff_ID NUMBER ( 6 ) , FIRST_NAME VARCHAR2 ( 20 ), LAST_NAME VARCHAR2 ( 25 ), EMAIL VARCHAR2 ( 25 ), PHONE_NUMBER VARCHAR2 ( 20 ), HIRE_DATE DATE , employment_ID VARCHAR2 ( 10 ), SALARY NUMBER ( 8 , 2 ), COMMISSION_PCT NUMBER ( 2 , 2 ), MANAGER_ID NUMBER ( 6 ), section_ID NUMBER ( 4 ) ) SERVER gsmpp_server OPTIONS ( location 'file:///output_data/' , format 'csv' , delimiter '|' , header 'on' ) WRITE ONLY ;

-- 来源: 1247_CREATE FOREIGN TABLE ()
DROP FOREIGN TABLE foreign_HR_staffS ;

-- 来源: 1247_CREATE FOREIGN TABLE ()
DROP FOREIGN TABLE foreign_HR_staffS_ft1 ;

-- 来源: 1247_CREATE FOREIGN TABLE ()
DROP FOREIGN TABLE foreign_HR_staffS_ft2 ;

-- 来源: 1247_CREATE FOREIGN TABLE ()
DROP FOREIGN TABLE foreign_HR_staffS_ft3 ;

-- 来源: 1248_CREATE FUNCTION
CREATE DATABASE ora_compatible_db DBCOMPATIBILITY 'ORA' ;

-- 来源: 1248_CREATE FUNCTION
CREATE FUNCTION func_add_sql ( integer , integer ) RETURNS integer AS 'select $1 + $2;

-- 来源: 1248_CREATE FUNCTION
CREATE OR REPLACE FUNCTION func_increment_plsql ( i integer ) RETURNS integer AS $$ BEGIN RETURN i + 1 ;

-- 来源: 1248_CREATE FUNCTION
CREATE OR REPLACE FUNCTION compute ( i int , out result_1 bigint , out result_2 bigint ) RETURNS SETOF RECORD AS $$ BEGIN result_1 = i + 1 ;

-- 来源: 1248_CREATE FUNCTION
CREATE FUNCTION func_dup_sql ( in int , out f1 int , out f2 text ) AS $$ SELECT $ 1 , CAST ( $ 1 AS text ) || ' is text' $$ LANGUAGE SQL ;

-- 来源: 1248_CREATE FUNCTION
CREATE FUNCTION func_add_sql2 ( num1 integer , num2 integer ) RETURN integer AS BEGIN RETURN num1 + num2 ;

-- 来源: 1248_CREATE FUNCTION
ALTER FUNCTION func_add_sql2 ( INTEGER , INTEGER ) IMMUTABLE ;

-- 来源: 1248_CREATE FUNCTION
ALTER FUNCTION func_add_sql2 ( INTEGER , INTEGER ) RENAME TO add_two_number ;

-- 来源: 1248_CREATE FUNCTION
CREATE USER jim PASSWORD '********' ;

-- 来源: 1248_CREATE FUNCTION
ALTER FUNCTION add_two_number ( INTEGER , INTEGER ) OWNER TO jim ;

-- 来源: 1248_CREATE FUNCTION
DROP FUNCTION func_add_sql ;

-- 来源: 1248_CREATE FUNCTION
DROP FUNCTION func_increment_plsql ;

-- 来源: 1248_CREATE FUNCTION
DROP FUNCTION compute ;

-- 来源: 1248_CREATE FUNCTION
DROP FUNCTION func_dup_sql ;

-- 来源: 1248_CREATE FUNCTION
DROP FUNCTION add_two_number ;

-- 来源: 1248_CREATE FUNCTION
DROP USER jim ;

-- 来源: 1248_CREATE FUNCTION
CREATE TYPE rec AS ( c1 int , c2 int );

-- 来源: 1248_CREATE FUNCTION
CREATE OR REPLACE FUNCTION func ( a in out rec , b in out int ) RETURN int AS BEGIN a . c1 : = 100 ;

-- 来源: 1248_CREATE FUNCTION
DROP FUNCTION func ;

-- 来源: 1248_CREATE FUNCTION
DROP TYPE rec ;

-- 来源: 1248_CREATE FUNCTION
CREATE OR REPLACE FUNCTION func_001 ( a in out date , b in out date ) --#add in & inout #defult value RETURN integer AS BEGIN raise info '%' , a ;

-- 来源: 1248_CREATE FUNCTION
CREATE OR REPLACE FUNCTION func_001 ( a in out INT , b in out date ) --#add in & inout #defult value RETURN INT AS BEGIN raise info '%' , a ;

-- 来源: 1248_CREATE FUNCTION
DROP FUNCTION func_001 ;

-- 来源: 1249_CREATE GLOBAL INDEX
CREATE TABLE test1 ( c1 int , c2 int , c3 int );

-- 来源: 1249_CREATE GLOBAL INDEX
CREATE GLOBAL INDEX idx_gsi_1 ON test1 ( c2 ) CONTAINING ( c3 ) DISTRIBUTE BY HASH ( c2 );

-- 来源: 1249_CREATE GLOBAL INDEX
CREATE TABLE test2 ( c1 int , c2 int , c3 int );

-- 来源: 1249_CREATE GLOBAL INDEX
CREATE GLOBAL INDEX idx_gsi_2 ON test2 ( c2 ) CONTAINING ( c3 ) ;

-- 来源: 1249_CREATE GLOBAL INDEX
CREATE TABLE test3 ( c1 int , c2 int , c3 int );

-- 来源: 1249_CREATE GLOBAL INDEX
CREATE GLOBAL UNIQUE INDEX idx_gsi_3 ON test3 ( c2 ) DISTRIBUTE BY HASH ( c2 );

-- 来源: 1249_CREATE GLOBAL INDEX
DROP INDEX idx_gsi_1 ;

-- 来源: 1249_CREATE GLOBAL INDEX
DROP INDEX idx_gsi_2 ;

-- 来源: 1249_CREATE GLOBAL INDEX
DROP INDEX idx_gsi_3 ;

-- 来源: 1249_CREATE GLOBAL INDEX
DROP TABLE test1 ;

-- 来源: 1249_CREATE GLOBAL INDEX
DROP TABLE test2 ;

-- 来源: 1249_CREATE GLOBAL INDEX
DROP TABLE test3 ;

-- 来源: 1250_CREATE GROUP
CREATE GROUP my_group PASSWORD '********';

--删除组。
-- 来源: 1250_CREATE GROUP
DROP GROUP my_group;

-- 来源: 1251_CREATE INCREMENTAL MATERIALIZED VIEW
CREATE TABLE my_table (c1 int, c2 int);

--创建增量物化视图。
-- 来源: 1251_CREATE INCREMENTAL MATERIALIZED VIEW
CREATE INCREMENTAL MATERIALIZED VIEW my_imv AS SELECT * FROM my_table;

--删除增量物化视图。
-- 来源: 1251_CREATE INCREMENTAL MATERIALIZED VIEW
DROP MATERIALIZED VIEW my_imv;

--删除普通表my_table。
-- 来源: 1251_CREATE INCREMENTAL MATERIALIZED VIEW
DROP TABLE my_table;

-- 来源: 1252_CREATE INDEX
CREATE TABLE tbl_test1( id int, --用户id name varchar(50), --用户姓名 postcode char(6) --邮编 );

--创建表空间tbs_index1。
-- 来源: 1252_CREATE INDEX
CREATE TABLESPACE tbs_index1 RELATIVE LOCATION 'test_tablespace/tbs_index1';

--为表tbl_test1创建索引idx_test1指定表空间。
-- 来源: 1252_CREATE INDEX
CREATE INDEX idx_test1 ON tbl_test1(name) TABLESPACE tbs_index1;

--删除索引。
-- 来源: 1252_CREATE INDEX
DROP INDEX idx_test1;

--删除表空间
-- 来源: 1252_CREATE INDEX
DROP TABLESPACE tbs_index1;

-- 来源: 1252_CREATE INDEX
CREATE UNIQUE INDEX idx_test2 ON tbl_test1(id);

--删除索引。
-- 来源: 1252_CREATE INDEX
DROP INDEX idx_test2;

-- 来源: 1252_CREATE INDEX
CREATE INDEX idx_test3 ON tbl_test1(substr(postcode,2));

--删除索引。
-- 来源: 1252_CREATE INDEX
DROP INDEX idx_test3;

-- 来源: 1252_CREATE INDEX
CREATE INDEX idx_test4 ON tbl_test1(id) WHERE id IS NOT NULL;

-- 删除索引。
-- 来源: 1252_CREATE INDEX
DROP INDEX idx_test4;

-- 删除表
-- 来源: 1252_CREATE INDEX
DROP TABLE tbl_test1;

-- 来源: 1252_CREATE INDEX
CREATE TABLE student(id int, name va(20)) PARTITION BY RANGE (id) ( PARTITION p1 VALUES LESS THAN (200), PARTITION pmax VALUES LESS THAN (MAXVALUE) );

--创建LOCAL分区索引不指定索引分区的名称。
-- 来源: 1252_CREATE INDEX
CREATE INDEX idx_student1 ON student(id) LOCAL;

--删除LOCAL分区索引。
-- 来源: 1252_CREATE INDEX
DROP INDEX idx_student1;

--创建GLOBAL索引。
-- 来源: 1252_CREATE INDEX
CREATE INDEX idx_student2 ON student(name) GLOBAL;

--删除GLOBAL分区索引。
-- 来源: 1252_CREATE INDEX
DROP INDEX idx_student2;

--删除表。
-- 来源: 1252_CREATE INDEX
DROP TABLE student;

-- 来源: 1254_CREATE MASKING POLICY
CREATE USER dev_mask PASSWORD '********' ;

-- 来源: 1254_CREATE MASKING POLICY
CREATE USER bob_mask PASSWORD '********' ;

-- 来源: 1254_CREATE MASKING POLICY
CREATE TABLE tb_for_masking ( idx int , col1 text , col2 text , col3 text , col4 text , col5 text , col6 text , col7 text , col8 text );

-- 来源: 1254_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb1 ADD COLUMN ( tb_for_masking . col1 );

-- 来源: 1254_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb2 ADD COLUMN ( tb_for_masking . col2 );

-- 来源: 1254_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb3 ADD COLUMN ( tb_for_masking . col3 );

-- 来源: 1254_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb4 ADD COLUMN ( tb_for_masking . col4 );

-- 来源: 1254_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb5 ADD COLUMN ( tb_for_masking . col5 );

-- 来源: 1254_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb6 ADD COLUMN ( tb_for_masking . col6 );

-- 来源: 1254_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb7 ADD COLUMN ( tb_for_masking . col7 );

-- 来源: 1254_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb8 ADD COLUMN ( tb_for_masking . col8 );

-- 来源: 1254_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol1 maskall ON LABEL ( mask_lb1 );

-- 来源: 1254_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol2 alldigitsmasking ON LABEL ( mask_lb2 );

-- 来源: 1254_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol3 basicemailmasking ON LABEL ( mask_lb3 );

-- 来源: 1254_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol4 fullemailmasking ON LABEL ( mask_lb4 );

-- 来源: 1254_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol5 creditcardmasking ON LABEL ( mask_lb5 );

-- 来源: 1254_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol6 shufflemasking ON LABEL ( mask_lb6 );

-- 来源: 1254_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol7 regexpmasking ( '[\d+]' , '*' , 2 , 9 ) ON LABEL ( mask_lb7 );

-- 来源: 1254_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol8 randommasking ON LABEL ( mask_lb8 ) FILTER ON ROLES ( dev_mask , bob_mask ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' );

-- 来源: 1254_CREATE MASKING POLICY
DROP MASKING POLICY maskpol1 , maskpol2 , maskpol3 , maskpol4 , maskpol5 , maskpol6 , maskpol7 , maskpol8 ;

-- 来源: 1254_CREATE MASKING POLICY
DROP RESOURCE LABEL mask_lb1 , mask_lb2 , mask_lb3 , mask_lb4 , mask_lb5 , mask_lb6 , mask_lb7 , mask_lb8 ;

-- 来源: 1254_CREATE MASKING POLICY
DROP TABLE tb_for_masking ;

-- 来源: 1254_CREATE MASKING POLICY
DROP USER dev_mask , bob_mask ;

-- 来源: 1255_CREATE MATERIALIZED VIEW
CREATE TABLE my_table (c1 int, c2 int);

--创建全量物化视图。
-- 来源: 1255_CREATE MATERIALIZED VIEW
CREATE MATERIALIZED VIEW my_mv AS SELECT * FROM my_table;

--删除全量物化视图。
-- 来源: 1255_CREATE MATERIALIZED VIEW
DROP MATERIALIZED VIEW my_mv;

--删除普通表my_table。
-- 来源: 1255_CREATE MATERIALIZED VIEW
DROP TABLE my_table;

-- 来源: 1257_CREATE NODE
CREATE NODE datanode1 WITH( TYPE = datanode, PREFERRED = false );

-- 来源: 1257_CREATE NODE
CREATE NODE datanode2 WITH( TYPE = datanode, PREFERRED = false );

-- 将datanode1设为preferred DN。
-- 来源: 1257_CREATE NODE
ALTER NODE datanode1 WITH(preferred = true);

-- 删除集群节点。
-- 来源: 1257_CREATE NODE
DROP NODE datanode1;

-- 来源: 1257_CREATE NODE
DROP NODE datanode2;

-- 创建node group，用上一步中查询到的真实节点名称替换dn_6001_6002_6003。
-- 来源: 1258_CREATE NODE GROUP
CREATE NODE GROUP test_group WITH ( dn_6001_6002_6003 );

-- 删除node group。
-- 来源: 1258_CREATE NODE GROUP
DROP NODE GROUP test_group;

-- 来源: 1259_CREATE PACKAGE
CREATE DATABASE ora_compat_db DBCOMPATIBILITY 'ORA';

-- 来源: 1259_CREATE PACKAGE
CREATE OR REPLACE PACKAGE emp_bonus IS var1 INT:=1;

-- 来源: 1259_CREATE PACKAGE
DROP PACKAGE emp_bonus;

-- 来源: 1259_CREATE PACKAGE
DROP TABLE IF EXISTS test1;

--创建包头
-- 来源: 1259_CREATE PACKAGE
CREATE OR REPLACE PACKAGE emp_bonus IS var1 INT:=1;

--创建包体
-- 来源: 1259_CREATE PACKAGE
CREATE OR REPLACE PACKAGE BODY emp_bonus IS var3 INT:=3;

-- 来源: 1259_CREATE PACKAGE
ALTER PACKAGE emp_bonus OWNER TO omm;

-- 来源: 1259_CREATE PACKAGE
DROP TABLE IF EXISTS test1;

-- 来源: 1259_CREATE PACKAGE
DROP TABLE IF EXISTS test1;

-- 来源: 1259_CREATE PACKAGE
DROP TABLE IF EXISTS test1;

--删除PACKAGE。
-- 来源: 1259_CREATE PACKAGE
DROP PACKAGE emp_bonus;

--删除数据库。
-- 来源: 1259_CREATE PACKAGE
DROP DATABASE ora_compat_db;

-- 来源: 1260_CREATE PROCEDURE
CREATE OR REPLACE PROCEDURE prc_add ( param1 IN INTEGER , param2 IN OUT INTEGER ) AS BEGIN param2 : = param1 + param2 ;

-- 来源: 1260_CREATE PROCEDURE
CREATE OR REPLACE PROCEDURE pro_variadic ( var1 VARCHAR2 ( 10 ) DEFAULT 'hello!' , var4 VARIADIC int4 []) AS BEGIN dbe_output . print_line ( var1 );

-- 来源: 1260_CREATE PROCEDURE
CREATE TABLE tb1 ( a integer );

-- 来源: 1260_CREATE PROCEDURE
CREATE PROCEDURE insert_data ( v integer ) SECURITY INVOKER AS BEGIN INSERT INTO tb1 VALUES ( v );

-- 来源: 1260_CREATE PROCEDURE
DROP PROCEDURE prc_add ;

-- 来源: 1260_CREATE PROCEDURE
DROP PROCEDURE pro_variadic ;

-- 来源: 1260_CREATE PROCEDURE
DROP PROCEDURE insert_data ;

-- 来源: 1260_CREATE PROCEDURE
DROP TABLE tb1 ;

-- 来源: 1261_CREATE RESOURCE LABEL
CREATE TABLE tb_for_label ( col1 text , col2 text , col3 text );

-- 来源: 1261_CREATE RESOURCE LABEL
CREATE SCHEMA schema_for_label ;

-- 来源: 1261_CREATE RESOURCE LABEL
CREATE VIEW view_for_label AS SELECT 1 ;

-- 来源: 1261_CREATE RESOURCE LABEL
CREATE FUNCTION func_for_label RETURNS TEXT AS $$ SELECT col1 FROM tb_for_label ;

-- 来源: 1261_CREATE RESOURCE LABEL
CREATE RESOURCE LABEL IF NOT EXISTS table_label add TABLE ( public . tb_for_label );

-- 来源: 1261_CREATE RESOURCE LABEL
CREATE RESOURCE LABEL IF NOT EXISTS column_label add COLUMN ( public . tb_for_label . col1 );

-- 来源: 1261_CREATE RESOURCE LABEL
CREATE RESOURCE LABEL IF NOT EXISTS schema_label add SCHEMA ( schema_for_label );

-- 来源: 1261_CREATE RESOURCE LABEL
CREATE RESOURCE LABEL IF NOT EXISTS view_label add VIEW ( view_for_label );

-- 来源: 1261_CREATE RESOURCE LABEL
CREATE RESOURCE LABEL IF NOT EXISTS func_label add FUNCTION ( func_for_label );

-- 来源: 1261_CREATE RESOURCE LABEL
DROP RESOURCE LABEL func_label , view_label , schema_label , column_label , table_label ;

-- 来源: 1261_CREATE RESOURCE LABEL
DROP FUNCTION func_for_label ;

-- 来源: 1261_CREATE RESOURCE LABEL
DROP VIEW view_for_label ;

-- 来源: 1261_CREATE RESOURCE LABEL
DROP SCHEMA schema_for_label ;

-- 来源: 1261_CREATE RESOURCE LABEL
DROP TABLE tb_for_label ;

-- 来源: 1262_CREATE RESOURCE POOL
CREATE RESOURCE POOL pool1 ;

-- 来源: 1262_CREATE RESOURCE POOL
CREATE RESOURCE POOL pool2 WITH ( CONTROL_GROUP = "High" );

-- 来源: 1262_CREATE RESOURCE POOL
CREATE RESOURCE POOL pool3 WITH ( CONTROL_GROUP = "class1:Low" );

-- 来源: 1262_CREATE RESOURCE POOL
CREATE RESOURCE POOL pool4 WITH ( CONTROL_GROUP = "class1:wg1" );

-- 来源: 1262_CREATE RESOURCE POOL
CREATE RESOURCE POOL pool5 WITH ( CONTROL_GROUP = "class1:wg2:3" );

-- 来源: 1262_CREATE RESOURCE POOL
DROP RESOURCE POOL pool1 ;

-- 来源: 1262_CREATE RESOURCE POOL
DROP RESOURCE POOL pool2 ;

-- 来源: 1262_CREATE RESOURCE POOL
DROP RESOURCE POOL pool3 ;

-- 来源: 1262_CREATE RESOURCE POOL
DROP RESOURCE POOL pool4 ;

-- 来源: 1262_CREATE RESOURCE POOL
DROP RESOURCE POOL pool5 ;

-- 来源: 1263_CREATE ROLE
CREATE ROLE manager IDENTIFIED BY '********' ;

-- 来源: 1263_CREATE ROLE
CREATE ROLE miriam WITH LOGIN PASSWORD '********' VALID BEGIN '2015-01-01' VALID UNTIL '2026-01-01' ;

-- 来源: 1263_CREATE ROLE
ALTER ROLE manager IDENTIFIED BY '**********' REPLACE '********' ;

-- 来源: 1263_CREATE ROLE
ALTER ROLE manager SYSADMIN ;

-- 来源: 1263_CREATE ROLE
DROP ROLE manager ;

-- 来源: 1263_CREATE ROLE
DROP GROUP miriam ;

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
CREATE USER alice PASSWORD '*********' ;

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
CREATE USER bob PASSWORD '*********' ;

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
CREATE TABLE public . all_data ( id int , role varchar ( 100 ), data varchar ( 100 ));

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
ALTER TABLE all_data ENABLE ROW LEVEL SECURITY ;

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
ALTER USER alice LOGIN ;

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
DROP TABLE public . all_data ;

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
DROP USER alice , bob ;

-- 来源: 1265_CREATE SCHEMA
CREATE ROLE role1 IDENTIFIED BY '********' ;

-- 来源: 1265_CREATE SCHEMA
CREATE SCHEMA AUTHORIZATION role1 CREATE TABLE films ( title text , release date , awards text []) CREATE VIEW winners AS SELECT title , release FROM films WHERE awards IS NOT NULL ;

-- 来源: 1265_CREATE SCHEMA
DROP SCHEMA role1 CASCADE ;

-- 来源: 1265_CREATE SCHEMA
DROP USER role1 CASCADE ;

-- 来源: 1266_CREATE SECURITY LABEL
CREATE SECURITY LABEL sec_label 'L1:G4' ;

-- 来源: 1266_CREATE SECURITY LABEL
DROP SECURITY LABEL sec_label ;

-- 来源: 1267_CREATE SEQUENCE
CREATE SEQUENCE seq1 START 101 INCREMENT 10 ;

-- 来源: 1267_CREATE SEQUENCE
DROP SEQUENCE seq1 ;

-- 来源: 1267_CREATE SEQUENCE
CREATE TABLE test1 ( id int PRIMARY KEY , name varchar ( 20 ));

-- 来源: 1267_CREATE SEQUENCE
CREATE SEQUENCE test_seq2 START 1 NO CYCLE OWNED BY test1 . id ;

-- 来源: 1267_CREATE SEQUENCE
ALTER TABLE test1 ALTER COLUMN id SET DEFAULT nextval ( 'test_seq2' :: regclass );

-- 来源: 1267_CREATE SEQUENCE
DROP SEQUENCE test_seq2 CASCADE ;

-- 来源: 1267_CREATE SEQUENCE
DROP TABLE test1 ;

-- 来源: 1268_CREATE SERVER
CREATE SERVER my_server FOREIGN DATA WRAPPER file_fdw ;

-- 来源: 1268_CREATE SERVER
DROP SERVER my_server ;

-- 来源: 1268_CREATE SERVER
CREATE SERVER server_remote FOREIGN DATA WRAPPER GC_FDW OPTIONS ( address '10.146.187.231:8000,10.180.157.130:8000' , dbname 'test' , username 'test' , password '********' );

-- 来源: 1268_CREATE SERVER
DROP SERVER server_remote ;

-- 来源: 1269_CREATE SYNONYM
CREATE SCHEMA ot ;

-- 来源: 1269_CREATE SYNONYM
CREATE TABLE ot . t1 ( id int , name varchar2 ( 10 )) DISTRIBUTE BY hash ( id );

-- 来源: 1269_CREATE SYNONYM
CREATE OR REPLACE SYNONYM t1 FOR ot . t1 ;

-- 来源: 1269_CREATE SYNONYM
CREATE SYNONYM v1 FOR ot . v_t1 ;

-- 来源: 1269_CREATE SYNONYM
CREATE VIEW ot . v_t1 AS SELECT * FROM ot . t1 ;

-- 来源: 1269_CREATE SYNONYM
CREATE OR REPLACE FUNCTION ot . add ( a integer , b integer ) RETURNS integer AS $$ SELECT $ 1 + $ 2 $$ LANGUAGE sql ;

-- 来源: 1269_CREATE SYNONYM
CREATE OR REPLACE FUNCTION ot . add ( a decimal ( 5 , 2 ), b decimal ( 5 , 2 )) RETURNS decimal ( 5 , 2 ) AS $$ SELECT $ 1 + $ 2 $$ LANGUAGE sql ;

-- 来源: 1269_CREATE SYNONYM
CREATE OR REPLACE SYNONYM add FOR ot . add ;

-- 来源: 1269_CREATE SYNONYM
CREATE PROCEDURE ot . register ( n_id integer , n_name varchar2 ( 10 )) SECURITY INVOKER AS BEGIN INSERT INTO ot . t1 VALUES ( n_id , n_name );

-- 来源: 1269_CREATE SYNONYM
CREATE OR REPLACE SYNONYM register FOR ot . register ;

-- 来源: 1269_CREATE SYNONYM
DROP SYNONYM t1 ;

-- 来源: 1269_CREATE SYNONYM
DROP SYNONYM IF EXISTS v1 ;

-- 来源: 1269_CREATE SYNONYM
DROP SYNONYM IF EXISTS add ;

-- 来源: 1269_CREATE SYNONYM
DROP SYNONYM register ;

-- 来源: 1269_CREATE SYNONYM
DROP SCHEMA ot CASCADE ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t1 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t2 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ), W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t2 ;

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t1 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t3 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) DEFAULT 'GA' , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t3 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t4 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) UNIQUE DEFERRABLE , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t4 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t5 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), UNIQUE ( W_WAREHOUSE_NAME ) WITH ( fillfactor = 70 ) );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t5 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t6 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) UNIQUE , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) WITH ( fillfactor = 70 );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t6 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE UNLOGGED TABLE tpcds . warehouse_t7 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t7 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TEMPORARY TABLE warehouse_t24 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- 来源: 1270_CREATE TABLE
CREATE TEMPORARY TABLE warehouse_t25 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) ON COMMIT DELETE ROWS ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE IF NOT EXISTS tpcds . warehouse_t8 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t8 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLESPACE DS_TABLESPACE1 RELATIVE LOCATION 'tablespace/tablespace_1' ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t9 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) TABLESPACE DS_TABLESPACE1 ;

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t9 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t10 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) UNIQUE USING INDEX TABLESPACE DS_TABLESPACE1 , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t10 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t11 ( W_WAREHOUSE_SK INTEGER PRIMARY KEY , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t11 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t12 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), PRIMARY KEY ( W_WAREHOUSE_SK ) );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t12 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t13 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), CONSTRAINT W_CSTR_KEY1 PRIMARY KEY ( W_WAREHOUSE_SK ) );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t13 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t13_1 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), CONSTRAINT PRIMARY KEY USING BTREE ( W_WAREHOUSE_SK DESC ) );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t13_1 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t14 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), CONSTRAINT W_CSTR_KEY2 PRIMARY KEY ( W_WAREHOUSE_SK , W_WAREHOUSE_ID ) );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t14 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t19 ( W_WAREHOUSE_SK INTEGER PRIMARY KEY CHECK ( W_WAREHOUSE_SK > 0 ), W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) CHECK ( W_WAREHOUSE_NAME IS NOT NULL ), W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t20 ( W_WAREHOUSE_SK INTEGER PRIMARY KEY , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) CHECK ( W_WAREHOUSE_NAME IS NOT NULL ), W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), CONSTRAINT W_CONSTR_KEY2 CHECK ( W_WAREHOUSE_SK > 0 AND W_WAREHOUSE_NAME IS NOT NULL ) );

-- 来源: 1270_CREATE TABLE
ALTER TABLE tpcds . warehouse_t19 ADD W_GOODS_CATEGORY varchar ( 30 );

-- 来源: 1270_CREATE TABLE
ALTER TABLE tpcds . warehouse_t19 ADD CONSTRAINT W_CONSTR_KEY4 CHECK ( W_STATE IS NOT NULL );

-- 来源: 1270_CREATE TABLE
ALTER TABLE tpcds . warehouse_t19 ALTER COLUMN W_GOODS_CATEGORY TYPE varchar ( 80 ), ALTER COLUMN W_STREET_NAME TYPE varchar ( 100 );

-- 来源: 1270_CREATE TABLE
ALTER TABLE tpcds . warehouse_t19 MODIFY ( W_GOODS_CATEGORY varchar ( 30 ), W_STREET_NAME varchar ( 60 ));

-- 来源: 1270_CREATE TABLE
ALTER TABLE tpcds . warehouse_t19 ALTER COLUMN W_GOODS_CATEGORY SET NOT NULL ;

-- 来源: 1270_CREATE TABLE
ALTER TABLE tpcds . warehouse_t19 ALTER COLUMN W_GOODS_CATEGORY DROP NOT NULL ;

-- 来源: 1270_CREATE TABLE
ALTER TABLE tpcds . warehouse_t19 SET TABLESPACE PG_DEFAULT ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA joe ;

-- 来源: 1270_CREATE TABLE
ALTER TABLE tpcds . warehouse_t19 SET SCHEMA joe ;

-- 来源: 1270_CREATE TABLE
ALTER TABLE joe . warehouse_t19 RENAME TO warehouse_t23 ;

-- 来源: 1270_CREATE TABLE
ALTER TABLE joe . warehouse_t23 DROP COLUMN W_STREET_NAME ;

-- 来源: 1270_CREATE TABLE
DROP TABLESPACE DS_TABLESPACE1 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA IF EXISTS joe CASCADE ;

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t20 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t21 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) DISTRIBUTE BY REPLICATION ;

-- 来源: 1270_CREATE TABLE
ALTER TABLE tpcds . warehouse_t21 SET ( primarynode = on );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t21 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t22 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), CONSTRAINT W_CONSTR_KEY3 UNIQUE ( W_WAREHOUSE_SK ) ) DISTRIBUTE BY HASH ( W_WAREHOUSE_SK );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t22 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t26 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) DISTRIBUTE BY RANGE ( W_WAREHOUSE_ID ) ( SLICE s1 VALUES LESS THAN ( 10 ) DATANODE datanode1 , SLICE s2 VALUES LESS THAN ( 20 ) DATANODE datanode2 , SLICE s3 VALUES LESS THAN ( 30 ) DATANODE datanode3 , SLICE s4 VALUES LESS THAN ( MAXVALUE ) DATANODE datanode4 );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t26 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE lrt_range ( f_int1 int , f_int2 int , f_varchar1 varchar2 ( 100 )) distribute by range ( f_int1 , f_int2 ) ( slice s1 values less than ( 100 , 100 ) datanode ( datanode1 , datanode2 ), slice s2 values less than ( 200 , 200 ) datanode datanode2 , slice s3 values less than ( 300 , 300 ) datanode datanode2 , slice s4 values less than ( maxvalue , maxvalue ) datanode ( datanode1 , datanode2 ) );

-- 来源: 1270_CREATE TABLE
CREATE TABLE t_news ( county varchar ( 30 ), year varchar ( 60 ), name varchar ( 60 ), age int , news text ) distribute by list ( county , year ) ( slice s1 values (( 'china' , '2020' ),( 'china' , '2021' )) datanode ( datanode1 , datanode2 ), slice s2 values (( 'china' , '2022' ),( 'china' , '2023' ),( 'china' , '2024' )) datanode ( datanode1 , datanode2 ), slice s3 values (( 'china' , '2025' )) datanode ( datanode1 , datanode2 ), slice s4 values (( 'canada' , '2021' )) datanode datanode1 , slice s5 values (( 'canada' , '2022' )) datanode datanode2 , slice s6 values (( 'canada' , '2023' )) datanode datanode1 , slice s7 values (( 'uk' , '2021' )) datanode datanode1 , slice s8 values (( 'uk' , '2022' )) datanode datanode2 , slice s9 values (( 'uk' , '2023' )) datanode datanode1 , slice s0 values ( default ) datanode ( datanode1 , datanode2 ) );

-- 来源: 1270_CREATE TABLE
CREATE TABLE t_ran1 ( c1 int , c2 int , c3 int , c4 int , c5 int ) distribute by range ( c1 , c2 ) ( SLICE s1 VALUES LESS THAN ( 10 , 10 ) DATANODE datanode1 , SLICE s2 VALUES LESS THAN ( 10 , 20 ) DATANODE datanode2 , SLICE s3 VALUES LESS THAN ( 20 , 10 ) DATANODE datanode3 );

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t27 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) DISTRIBUTE BY RANGE ( W_WAREHOUSE_ID ) SLICE REFERENCES warehouse_t26 ;

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t27 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE tpcds . warehouse_t28 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) DISTRIBUTE BY LIST ( W_COUNTRY ) ( SLICE s1 VALUES ( 'USA' ) DATANODE datanode1 , SLICE s2 VALUES ( 'CANADA' ) DATANODE datanode2 , SLICE s3 VALUES ( 'UK' ) DATANODE datanode3 , SLICE s4 VALUES ( DEFAULT ) DATANODE datanode4 );

-- 来源: 1270_CREATE TABLE
DROP TABLE tpcds . warehouse_t28 ;

-- 来源: 1270_CREATE TABLE
DROP SCHEMA tpcds ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE creditcard_info ( id_number int , name text encrypted with ( column_encryption_key = ImgCEK , encryption_type = DETERMINISTIC ), credit_card varchar ( 19 ) encrypted with ( column_encryption_key = ImgCEK1 , encryption_type = DETERMINISTIC ));

-- 来源: 1270_CREATE TABLE
CREATE TABLE t1 ( c1 text , c2 text charset utf8mb4 collate utf8mb4_unicode_ci ) charset utf8mb4 collate utf8mb4_bin ;

-- 来源: 1270_CREATE TABLE
ALTER TABLE t1 charset utf8mb4 collate utf8mb4_general_ci ;

-- 来源: 1270_CREATE TABLE
ALTER TABLE t1 add c3 varchar ( 20 ) charset utf8mb4 collate utf8mb4_bin ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE t1_on_update ( TS0 TIMESTAMP ON UPDATE CURRENT_TIMESTAMP , TS1 TIMESTAMP ON UPDATE CURRENT_TIMESTAMP () , TS2 TIMESTAMP ( 6 ) ON UPDATE CURRENT_TIMESTAMP ( 6 ) , DT0 DATETIME ON UPDATE LOCALTIMESTAMP , DT1 DATETIME ON UPDATE NOW () , IN0 INT ) DISTRIBUTE BY HASH ( IN0 );

-- 来源: 1270_CREATE TABLE
ALTER TABLE t1_on_update ADD TS3 timestamp ON UPDATE CURRENT_TIMESTAMP ;

-- 来源: 1270_CREATE TABLE
CREATE DATABASE ilmtabledb with dbcompatibility = 'ORA' ;

-- 来源: 1270_CREATE TABLE
ALTER DATABASE ilmtabledb TO GROUP GROUP1 ;

-- 来源: 1270_CREATE TABLE
CREATE TABLE ilm_table ( a int ) WITH ( STORAGE_TYPE = ASTORE ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION ON ( a != 0 );

-- 来源: 1271_CREATE TABLESPACE
CREATE TABLESPACE ds_location1 RELATIVE LOCATION 'test_tablespace/test_tablespace_1' ;

-- 来源: 1271_CREATE TABLESPACE
CREATE ROLE joe IDENTIFIED BY '********' ;

-- 来源: 1271_CREATE TABLESPACE
CREATE ROLE jay IDENTIFIED BY '********' ;

-- 来源: 1271_CREATE TABLESPACE
CREATE TABLESPACE ds_location2 OWNER joe RELATIVE LOCATION 'test_tablespace/test_tablespace_2' ;

-- 来源: 1271_CREATE TABLESPACE
ALTER TABLESPACE ds_location1 RENAME TO ds_location3 ;

-- 来源: 1271_CREATE TABLESPACE
ALTER TABLESPACE ds_location2 OWNER TO jay ;

-- 来源: 1271_CREATE TABLESPACE
DROP TABLESPACE ds_location2 ;

-- 来源: 1271_CREATE TABLESPACE
DROP TABLESPACE ds_location3 ;

-- 来源: 1271_CREATE TABLESPACE
DROP ROLE joe ;

-- 来源: 1271_CREATE TABLESPACE
DROP ROLE jay ;

-- 来源: 1272_CREATE TABLE AS
CREATE TABLE test1(col1 int PRIMARY KEY,col2 varchar(10));

-- 创建test2表并向表中插入上面查询的数据。
-- 来源: 1272_CREATE TABLE AS
CREATE TABLE test2 AS SELECT * FROM test1 WHERE col1 < 100;

-- 来源: 1272_CREATE TABLE AS
CREATE TABLE test3(c1,c2) AS SELECT * FROM test1;

-- 删除。
-- 来源: 1272_CREATE TABLE AS
DROP TABLE test1,test2,test3;

-- 来源: 1272_CREATE TABLE AS
CREATE DATABASE ilmtabledb WITH dbcompatibility = 'ORA' ;

-- 来源: 1272_CREATE TABLE AS
ALTER DATABASE SET ILM = on ;

-- 来源: 1272_CREATE TABLE AS
CREATE TABLE old_table ( a int );

-- 来源: 1272_CREATE TABLE AS
CREATE TABLE ilm_table ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION AS ( SELECT * FROM old_table );

-- 来源: 1272_CREATE TABLE AS
DROP TABLE old_table , ilm_table ;

-- 来源: 1272_CREATE TABLE AS
DROP DATABASE ilmtabledb ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE SCHEMA tpcds ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLE tpcds . web_returns ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLE tpcds . web_returns_p1 ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL , WR_REFUNDED_CUSTOMER_SK INTEGER , WR_REFUNDED_CDEMO_SK INTEGER , WR_REFUNDED_HDEMO_SK INTEGER , WR_REFUNDED_ADDR_SK INTEGER , WR_RETURNING_CUSTOMER_SK INTEGER , WR_RETURNING_CDEMO_SK INTEGER , WR_RETURNING_HDEMO_SK INTEGER , WR_RETURNING_ADDR_SK INTEGER , WR_WEB_PAGE_SK INTEGER , WR_REASON_SK INTEGER , WR_ORDER_NUMBER BIGINT NOT NULL , WR_RETURN_QUANTITY INTEGER , WR_RETURN_AMT DECIMAL ( 7 , 2 ) , WR_RETURN_TAX DECIMAL ( 7 , 2 ) , WR_RETURN_AMT_INC_TAX DECIMAL ( 7 , 2 ) , WR_FEE DECIMAL ( 7 , 2 ) , WR_RETURN_SHIP_COST DECIMAL ( 7 , 2 ) , WR_REFUNDED_CASH DECIMAL ( 7 , 2 ) , WR_REVERSED_CHARGE DECIMAL ( 7 , 2 ) , WR_ACCOUNT_CREDIT DECIMAL ( 7 , 2 ) , WR_NET_LOSS DECIMAL ( 7 , 2 ) ) DISTRIBUTE BY HASH ( WR_ITEM_SK ) PARTITION BY RANGE ( WR_RETURNED_DATE_SK ) ( PARTITION P1 VALUES LESS THAN ( 2450815 ), PARTITION P2 VALUES LESS THAN ( 2451179 ), PARTITION P3 VALUES LESS THAN ( 2451544 ), PARTITION P4 VALUES LESS THAN ( 2451910 ), PARTITION P5 VALUES LESS THAN ( 2452275 ), PARTITION P6 VALUES LESS THAN ( 2452640 ), PARTITION P7 VALUES LESS THAN ( 2453005 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) );

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . web_returns_p1 DROP PARTITION P8 ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . web_returns_p1 ADD PARTITION P8 VALUES LESS THAN ( 2453105 );

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . web_returns_p1 ADD PARTITION P9 VALUES LESS THAN ( MAXVALUE );

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . web_returns_p1 DROP PARTITION FOR ( 2453005 );

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . web_returns_p1 RENAME PARTITION P7 TO P10 ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . web_returns_p1 RENAME PARTITION FOR ( 2452639 ) TO P11 ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLE tpcds . web_returns_p1 ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLE tpcds . web_returns ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1/tablespace_1' ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLESPACE example2 RELATIVE LOCATION 'tablespace2/tablespace_2' ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLESPACE example3 RELATIVE LOCATION 'tablespace3/tablespace_3' ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLESPACE example4 RELATIVE LOCATION 'tablespace4/tablespace_4' ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE SCHEMA tpcds ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLE tpcds . web_returns_p2 ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL , WR_REFUNDED_CUSTOMER_SK INTEGER , WR_REFUNDED_CDEMO_SK INTEGER , WR_REFUNDED_HDEMO_SK INTEGER , WR_REFUNDED_ADDR_SK INTEGER , WR_RETURNING_CUSTOMER_SK INTEGER , WR_RETURNING_CDEMO_SK INTEGER , WR_RETURNING_HDEMO_SK INTEGER , WR_RETURNING_ADDR_SK INTEGER , WR_WEB_PAGE_SK INTEGER , WR_REASON_SK INTEGER , WR_ORDER_NUMBER BIGINT NOT NULL , WR_RETURN_QUANTITY INTEGER , WR_RETURN_AMT DECIMAL ( 7 , 2 ) , WR_RETURN_TAX DECIMAL ( 7 , 2 ) , WR_RETURN_AMT_INC_TAX DECIMAL ( 7 , 2 ) , WR_FEE DECIMAL ( 7 , 2 ) , WR_RETURN_SHIP_COST DECIMAL ( 7 , 2 ) , WR_REFUNDED_CASH DECIMAL ( 7 , 2 ) , WR_REVERSED_CHARGE DECIMAL ( 7 , 2 ) , WR_ACCOUNT_CREDIT DECIMAL ( 7 , 2 ) , WR_NET_LOSS DECIMAL ( 7 , 2 ) ) TABLESPACE example1 DISTRIBUTE BY HASH ( WR_ITEM_SK ) PARTITION BY RANGE ( WR_RETURNED_DATE_SK ) ( PARTITION P1 VALUES LESS THAN ( 2450815 ), PARTITION P2 VALUES LESS THAN ( 2451179 ), PARTITION P3 VALUES LESS THAN ( 2451544 ), PARTITION P4 VALUES LESS THAN ( 2451910 ), PARTITION P5 VALUES LESS THAN ( 2452275 ), PARTITION P6 VALUES LESS THAN ( 2452640 ), PARTITION P7 VALUES LESS THAN ( 2453005 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) TABLESPACE example2 ) ENABLE ROW MOVEMENT ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLE tpcds . web_returns_p3 ( LIKE tpcds . web_returns_p2 INCLUDING PARTITION );

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P1 TABLESPACE example2 ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P2 TABLESPACE example3 ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . web_returns_p2 SPLIT PARTITION P8 AT ( 2453010 ) INTO ( PARTITION P9 , PARTITION P10 );

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . web_returns_p2 MERGE PARTITIONS P6 , P7 INTO PARTITION P8 ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . web_returns_p2 DISABLE ROW MOVEMENT ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLE tpcds . web_returns_p1 ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLE tpcds . web_returns_p2 ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLE tpcds . web_returns_p3 ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLESPACE example1 ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLESPACE example2 ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLESPACE example3 ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLESPACE example4 ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLESPACE startend_tbs1 LOCATION '/home/omm/startend_tbs1' ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLESPACE startend_tbs2 LOCATION '/home/omm/startend_tbs2' ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLESPACE startend_tbs3 LOCATION '/home/omm/startend_tbs3' ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLESPACE startend_tbs4 LOCATION '/home/omm/startend_tbs4' ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE SCHEMA tpcds ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLE tpcds . startend_pt ( c1 INT , c2 INT ) TABLESPACE startend_tbs1 DISTRIBUTE BY HASH ( c1 ) PARTITION BY RANGE ( c2 ) ( PARTITION p1 START ( 1 ) END ( 1000 ) EVERY ( 200 ) TABLESPACE startend_tbs2 , PARTITION p2 END ( 2000 ), PARTITION p3 START ( 2000 ) END ( 2500 ) TABLESPACE startend_tbs3 , PARTITION p4 START ( 2500 ), PARTITION p5 START ( 3000 ) END ( 5000 ) EVERY ( 1000 ) TABLESPACE startend_tbs4 ) ENABLE ROW MOVEMENT ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . startend_pt ADD PARTITION p6 START ( 5000 ) END ( 6000 ) EVERY ( 300 ) TABLESPACE startend_tbs4 ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . startend_pt ADD PARTITION p7 END ( MAXVALUE );

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . startend_pt RENAME PARTITION p7 TO p8 ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . startend_pt DROP PARTITION p8 ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . startend_pt RENAME PARTITION FOR ( 5950 ) TO p71 ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . startend_pt SPLIT PARTITION FOR ( 4500 ) INTO ( PARTITION q1 START ( 4000 ) END ( 5000 ) EVERY ( 250 ) TABLESPACE startend_tbs3 );

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE tpcds . startend_pt MOVE PARTITION p2 TABLESPACE startend_tbs4 ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLE tpcds . startend_pt ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLESPACE startend_tbs1 ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLESPACE startend_tbs2 ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLESPACE startend_tbs3 ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLESPACE startend_tbs4 ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLE test_list ( col1 int , col2 int ) partition by list ( col1 ) ( partition p1 values ( 2000 ), partition p2 values ( 3000 ), partition p3 values ( 4000 ), partition p4 values ( 5000 ) );

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE test_list add partition p5 values ( 6000 );

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLE t1 ( col1 int , col2 int );

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE test_list exchange partition ( p1 ) with table t1 ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER TABLE test_list truncate partition p2 ;

-- 来源: 1273_CREATE TABLE PARTITION
alter table test_list drop partition p5 ;

-- 来源: 1273_CREATE TABLE PARTITION
alter table test_list merge partitions p1 , p2 into partition p2 ;

-- 来源: 1273_CREATE TABLE PARTITION
alter table test_list split partition p2 values ( 2000 ) into ( partition p1 , partition p2 );

-- 来源: 1273_CREATE TABLE PARTITION
drop table test_list ;

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLE t1 ;

-- 来源: 1273_CREATE TABLE PARTITION
create table test_hash ( col1 int , col2 int ) partition by hash ( col1 ) ( partition p1 , partition p2 );

-- 来源: 1273_CREATE TABLE PARTITION
create table t1 ( col1 int , col2 int );

-- 来源: 1273_CREATE TABLE PARTITION
alter table test_hash exchange partition ( p1 ) with table t1 ;

-- 来源: 1273_CREATE TABLE PARTITION
alter table test_hash truncate partition p2 ;

-- 来源: 1273_CREATE TABLE PARTITION
drop table test_hash ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLE t_multi_keys_list ( a int , b varchar ( 4 ), c int ) PARTITION BY LIST ( a , b ) ( PARTITION p1 VALUES ( ( 0 , NULL ) ), PARTITION p2 VALUES ( ( 0 , '1' ), ( 0 , '2' ), ( 0 , '3' ), ( 1 , '1' ), ( 1 , '2' ) ), PARTITION p3 VALUES ( ( NULL , '0' ), ( 2 , '1' ) ), PARTITION p4 VALUES ( ( 3 , '2' ), ( NULL , NULL ) ), PARTITION pd VALUES ( DEFAULT ) );

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLE t_multi_keys_list ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER DATABASE set ilm = on ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLE ilm_part ( a int ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION , PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLE ilm_part ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER DATABASE set ilm = on ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLE ilm_part ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER DATABASE set ilm = on ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLE ilm_part ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER DATABASE set ilm = on ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLE ilm_part ;

-- 来源: 1273_CREATE TABLE PARTITION
ALTER DATABASE set ilm = on ;

-- 来源: 1273_CREATE TABLE PARTITION
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- 来源: 1273_CREATE TABLE PARTITION
DROP TABLE ilm_part ;

-- 来源: 1274_CREATE TEXT SEARCH CONFIGURATION
CREATE TEXT SEARCH CONFIGURATION ngram2 ( parser = ngram ) WITH ( gram_size = 2 , grapsymbol_ignore = false );

-- 来源: 1274_CREATE TEXT SEARCH CONFIGURATION
CREATE TEXT SEARCH CONFIGURATION ngram3 ( copy = ngram2 ) WITH ( gram_size = 2 , grapsymbol_ignore = false );

-- 来源: 1274_CREATE TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION ngram2 ADD MAPPING FOR multisymbol WITH simple ;

-- 来源: 1274_CREATE TEXT SEARCH CONFIGURATION
CREATE USER joe IDENTIFIED BY '********' ;

-- 来源: 1274_CREATE TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION ngram2 OWNER TO joe ;

-- 来源: 1274_CREATE TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION ngram2 SET SCHEMA joe ;

-- 来源: 1274_CREATE TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION joe . ngram2 RENAME TO ngram_2 ;

-- 来源: 1274_CREATE TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION joe . ngram_2 DROP MAPPING IF EXISTS FOR multisymbol ;

-- 来源: 1274_CREATE TEXT SEARCH CONFIGURATION
DROP TEXT SEARCH CONFIGURATION joe . ngram_2 ;

-- 来源: 1274_CREATE TEXT SEARCH CONFIGURATION
DROP TEXT SEARCH CONFIGURATION ngram3 ;

-- 来源: 1274_CREATE TEXT SEARCH CONFIGURATION
DROP SCHEMA IF EXISTS joe CASCADE ;

-- 来源: 1274_CREATE TEXT SEARCH CONFIGURATION
DROP ROLE IF EXISTS joe ;

-- 来源: 1276_CREATE TRIGGER
CREATE TABLE test_trigger_src_tbl ( id1 INT , id2 INT , id3 INT );

-- 来源: 1276_CREATE TRIGGER
CREATE TABLE test_trigger_des_tbl ( id1 INT , id2 INT , id3 INT );

-- 来源: 1276_CREATE TRIGGER
CREATE OR REPLACE FUNCTION tri_insert_func () RETURNS TRIGGER AS $$ DECLARE BEGIN INSERT INTO test_trigger_des_tbl VALUES ( NEW . id1 , NEW . id2 , NEW . id3 );

-- 来源: 1276_CREATE TRIGGER
CREATE OR REPLACE FUNCTION tri_update_func () RETURNS TRIGGER AS $$ DECLARE BEGIN UPDATE test_trigger_des_tbl SET id3 = NEW . id3 WHERE id1 = OLD . id1 ;

-- 来源: 1276_CREATE TRIGGER
CREATE OR REPLACE FUNCTION tri_delete_func () RETURNS TRIGGER AS $$ DECLARE BEGIN DELETE FROM test_trigger_des_tbl WHERE id1 = OLD . id1 ;

-- 来源: 1276_CREATE TRIGGER
CREATE TRIGGER insert_trigger BEFORE INSERT ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_insert_func ();

-- 来源: 1276_CREATE TRIGGER
CREATE TRIGGER update_trigger AFTER UPDATE ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_update_func ();

-- 来源: 1276_CREATE TRIGGER
CREATE TRIGGER delete_trigger BEFORE DELETE ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_delete_func ();

-- 来源: 1276_CREATE TRIGGER
ALTER TRIGGER delete_trigger ON test_trigger_src_tbl RENAME TO delete_trigger_renamed ;

-- 来源: 1276_CREATE TRIGGER
ALTER TABLE test_trigger_src_tbl DISABLE TRIGGER insert_trigger ;

-- 来源: 1276_CREATE TRIGGER
ALTER TABLE test_trigger_src_tbl DISABLE TRIGGER ALL ;

-- 来源: 1276_CREATE TRIGGER
DROP TRIGGER insert_trigger ON test_trigger_src_tbl ;

-- 来源: 1276_CREATE TRIGGER
DROP TRIGGER update_trigger ON test_trigger_src_tbl ;

-- 来源: 1276_CREATE TRIGGER
DROP TRIGGER delete_trigger_renamed ON test_trigger_src_tbl ;

-- 来源: 1276_CREATE TRIGGER
DROP FUNCTION tri_insert_func ();

-- 来源: 1276_CREATE TRIGGER
DROP FUNCTION tri_update_func ();

-- 来源: 1276_CREATE TRIGGER
DROP FUNCTION tri_delete_func ();

-- 来源: 1276_CREATE TRIGGER
DROP TABLE test_trigger_src_tbl ;

-- 来源: 1276_CREATE TRIGGER
DROP TABLE test_trigger_des_tbl ;

-- 来源: 1277_CREATE TYPE
CREATE TYPE compfoo AS ( f1 int , f2 text );

-- 来源: 1277_CREATE TYPE
CREATE TABLE t1_compfoo ( a int , b compfoo );

-- 来源: 1277_CREATE TYPE
CREATE TABLE t2_compfoo ( a int , b compfoo );

-- 来源: 1277_CREATE TYPE
ALTER TYPE compfoo RENAME TO compfoo1 ;

-- 来源: 1277_CREATE TYPE
CREATE USER usr1 PASSWORD '********' ;

-- 来源: 1277_CREATE TYPE
ALTER TYPE compfoo1 OWNER TO usr1 ;

-- 来源: 1277_CREATE TYPE
ALTER TYPE compfoo1 SET SCHEMA usr1 ;

-- 来源: 1277_CREATE TYPE
ALTER TYPE usr1 . compfoo1 ADD ATTRIBUTE f3 int ;

-- 来源: 1277_CREATE TYPE
DROP TYPE usr1 . compfoo1 CASCADE ;

-- 来源: 1277_CREATE TYPE
DROP TABLE t1_compfoo ;

-- 来源: 1277_CREATE TYPE
DROP TABLE t2_compfoo ;

-- 来源: 1277_CREATE TYPE
DROP SCHEMA usr1 ;

-- 来源: 1277_CREATE TYPE
DROP USER usr1 ;

-- 来源: 1277_CREATE TYPE
CREATE TYPE bugstatus AS ENUM ( 'create' , 'modify' , 'closed' );

-- 来源: 1277_CREATE TYPE
ALTER TYPE bugstatus ADD VALUE IF NOT EXISTS 'regress' BEFORE 'closed' ;

-- 来源: 1277_CREATE TYPE
ALTER TYPE bugstatus RENAME VALUE 'create' TO 'new' ;

-- 来源: 1277_CREATE TYPE
CREATE TYPE bugstatus_table AS TABLE OF bugstatus ;

-- 来源: 1277_CREATE TYPE
CREATE TYPE complex ;

-- 来源: 1277_CREATE TYPE
CREATE FUNCTION complex_in ( cstring ) RETURNS complex AS 'filename' LANGUAGE C IMMUTABLE STRICT not fenced ;

-- 来源: 1277_CREATE TYPE
CREATE FUNCTION complex_out ( complex ) RETURNS cstring AS 'filename' LANGUAGE C IMMUTABLE STRICT not fenced ;

-- 来源: 1277_CREATE TYPE
CREATE FUNCTION complex_recv ( internal ) RETURNS complex AS 'filename' LANGUAGE C IMMUTABLE STRICT not fenced ;

-- 来源: 1277_CREATE TYPE
CREATE FUNCTION complex_send ( complex ) RETURNS bytea AS 'filename' LANGUAGE C IMMUTABLE STRICT not fenced ;

-- 来源: 1277_CREATE TYPE
CREATE TYPE complex ( internallength = 16 , input = complex_in , output = complex_out , receive = complex_recv , send = complex_send , alignment = double );

-- 来源: 1277_CREATE TYPE
DROP TYPE complex ;

-- 来源: 1277_CREATE TYPE
DROP FUNCTION complex_send ;

-- 来源: 1277_CREATE TYPE
DROP FUNCTION complex_recv ;

-- 来源: 1277_CREATE TYPE
DROP FUNCTION complex_out ;

-- 来源: 1277_CREATE TYPE
DROP FUNCTION complex_in ;

-- 来源: 1277_CREATE TYPE
DROP TYPE bugstatus_table ;

-- 来源: 1277_CREATE TYPE
DROP TYPE bugstatus CASCADE ;

-- 来源: 1278_CREATE USER
CREATE USER jim PASSWORD '********';

--下面语句与上面的等价。
-- 来源: 1278_CREATE USER
CREATE USER kim IDENTIFIED BY '********';

--如果创建有“创建数据库”权限的用户，则需要加CREATEDB关键字。
-- 来源: 1278_CREATE USER
CREATE USER dim CREATEDB PASSWORD '********';

--将用户jim的登录密码由********修改为**********。
-- 来源: 1278_CREATE USER
ALTER USER jim IDENTIFIED BY '**********' REPLACE '********';

--为用户jim追加CREATEROLE权限。
-- 来源: 1278_CREATE USER
ALTER USER jim CREATEROLE;

--锁定jim账户。
-- 来源: 1278_CREATE USER
ALTER USER jim ACCOUNT LOCK;

--删除用户。
-- 来源: 1278_CREATE USER
DROP USER kim CASCADE;

-- 来源: 1278_CREATE USER
DROP USER jim CASCADE;

-- 来源: 1278_CREATE USER
DROP USER dim CASCADE;

-- 来源: 1279_CREATE VIEW
CREATE TABLE test_tb1(col1 int, col2 int);

--创建一个col1小于5的视图。
-- 来源: 1279_CREATE VIEW
CREATE VIEW test_v1 AS SELECT * FROM test_tb1 WHERE col1 < 3;

--删除表和视图。
-- 来源: 1279_CREATE VIEW
DROP VIEW test_v1;

-- 来源: 1279_CREATE VIEW
DROP TABLE test_tb1;

-- 来源: 1279_CREATE VIEW
CREATE TABLE test_tb2(c1 int, c2 int);

-- 来源: 1279_CREATE VIEW
CREATE TEMP VIEW test_v2 AS SELECT * FROM test_tb2;

--删除视图和表。
-- 来源: 1279_CREATE VIEW
DROP VIEW test_v2 ;

-- 来源: 1279_CREATE VIEW
DROP TABLE test_tb2;

-- 来源: 1280_CREATE WORKLOAD GROUP
CREATE WORKLOAD GROUP wg_name1 ;

-- 来源: 1280_CREATE WORKLOAD GROUP
CREATE RESOURCE POOL pool1 ;

-- 来源: 1280_CREATE WORKLOAD GROUP
CREATE WORKLOAD GROUP wg_name2 USING RESOURCE POOL pool1 ;

-- 来源: 1280_CREATE WORKLOAD GROUP
CREATE WORKLOAD GROUP wg_name3 USING RESOURCE POOL pool1 WITH ( ACT_STATEMENTS = 10 );

-- 来源: 1280_CREATE WORKLOAD GROUP
DROP WORKLOAD GROUP wg_name1 ;

-- 来源: 1280_CREATE WORKLOAD GROUP
DROP WORKLOAD GROUP wg_name2 ;

-- 来源: 1280_CREATE WORKLOAD GROUP
DROP WORKLOAD GROUP wg_name3 ;

-- 来源: 1280_CREATE WORKLOAD GROUP
DROP RESOURCE POOL pool1 ;

-- 来源: 1281_CREATE WEAK PASSWORD DICTIONARY
CREATE WEAK PASSWORD DICTIONARY WITH VALUES ('********');

--向gs_global_config系统表中插入多个弱口令。
-- 来源: 1281_CREATE WEAK PASSWORD DICTIONARY
CREATE WEAK PASSWORD DICTIONARY WITH VALUES ('********'),('********');

--清空gs_global_config系统表中所有弱口令。
-- 来源: 1281_CREATE WEAK PASSWORD DICTIONARY
DROP WEAK PASSWORD DICTIONARY;

-- 来源: 1286_DELETE
CREATE SCHEMA tpcds ;

-- 来源: 1286_DELETE
CREATE TABLE tpcds . customer_address ( ca_address_sk INTEGER NOT NULL , ca_address_id CHARACTER ( 16 ) NOT NULL , ca_street_number INTEGER , ca_street_name CHARACTER ( 20 ) );

-- 来源: 1286_DELETE
CREATE TABLE tpcds . customer_address_bak AS TABLE tpcds . customer_address ;

-- 来源: 1286_DELETE
DROP TABLE tpcds . customer_address_bak ;

-- 来源: 1286_DELETE
DROP TABLE tpcds . customer_address ;

-- 来源: 1286_DELETE
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1287_DO
CREATE USER webuser PASSWORD '********' ;

-- 来源: 1287_DO
DROP USER webuser CASCADE ;

-- 来源: 1292_DROP DATA SOURCE
CREATE DATA SOURCE ds_tst1 ;

-- 来源: 1292_DROP DATA SOURCE
DROP DATA SOURCE ds_tst1 CASCADE ;

-- 来源: 1292_DROP DATA SOURCE
DROP DATA SOURCE IF EXISTS ds_tst1 RESTRICT ;

-- 来源: 1293_DROP DIRECTORY
CREATE OR REPLACE DIRECTORY dir as '/tmp/' ;

-- 来源: 1293_DROP DIRECTORY
DROP DIRECTORY dir ;

-- 来源: 1300_DROP MASKING POLICY
DROP MASKING POLICY IF EXISTS maskpol1 ;

-- 来源: 1300_DROP MASKING POLICY
DROP MASKING POLICY IF EXISTS maskpol1 , maskpol2 , maskpol3 ;

-- 来源: 1301_DROP MATERIALIZED VIEW
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建名为my_mv的物化视图。
-- 来源: 1301_DROP MATERIALIZED VIEW
CREATE MATERIALIZED VIEW my_mv AS SELECT * FROM my_table;

--删除名为my_mv的物化视图。
-- 来源: 1301_DROP MATERIALIZED VIEW
DROP MATERIALIZED VIEW my_mv;

--删除表。
-- 来源: 1301_DROP MATERIALIZED VIEW
DROP TABLE my_table;

-- 来源: 1305_DROP OWNED
CREATE USER jim PASSWORD '********' ;

-- 来源: 1305_DROP OWNED
DROP OWNED BY jim ;

-- 来源: 1305_DROP OWNED
DROP USER jim ;

-- 来源: 1306_DROP PACKAGE
CREATE DATABASE ora_compat_db DBCOMPATIBILITY 'ORA';

--创建PACKAGE。
-- 来源: 1306_DROP PACKAGE
CREATE OR REPLACE PACKAGE emp_bonus IS var1 INT:=1;

--删除PACKAGE。
-- 来源: 1306_DROP PACKAGE
DROP PACKAGE emp_bonus;

-- 来源: 1306_DROP PACKAGE
DROP DATABASE ora_compat_db;

-- 来源: 1311_DROP ROW LEVEL SECURITY POLICY
CREATE TABLE all_data ( id int , role varchar ( 100 ), data varchar ( 100 ));

-- 来源: 1311_DROP ROW LEVEL SECURITY POLICY
DROP TABLE all_data ;

-- 来源: 1313_DROP SECURITY LABEL
CREATE SECURITY LABEL sec_label 'L1:G4' ;

-- 来源: 1313_DROP SECURITY LABEL
DROP SECURITY LABEL sec_label ;

-- 来源: 1314_DROP SEQUENCE
CREATE SEQUENCE serial START 101 ;

-- 来源: 1314_DROP SEQUENCE
DROP SEQUENCE serial ;

-- 来源: 1315_DROP SERVER
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

--删除my_server。
-- 来源: 1315_DROP SERVER
DROP SERVER my_server;

-- 来源: 1320_DROP TEXT SEARCH DICTIONARY
CREATE TEXT SEARCH DICTIONARY english ( TEMPLATE = simple );

-- 来源: 1320_DROP TEXT SEARCH DICTIONARY
DROP TEXT SEARCH DICTIONARY english ;

-- 来源: 1328_EXECUTE
CREATE SCHEMA tpcds ;

-- 来源: 1328_EXECUTE
CREATE TABLE tpcds . reason ( CD_DEMO_SK INTEGER NOT NULL , CD_GENDER character ( 16 ) , CD_MARITAL_STATUS character ( 100 ) );

-- 来源: 1328_EXECUTE
CREATE TABLE tpcds . reason_t1 AS TABLE tpcds . reason ;

-- 来源: 1328_EXECUTE
DROP TABLE tpcds . reason ;

-- 来源: 1328_EXECUTE
DROP TABLE tpcds . reason_t1 ;

-- 来源: 1328_EXECUTE
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1329_EXECUTE DIRECT
CREATE SCHEMA tpcds ;

-- 来源: 1329_EXECUTE DIRECT
CREATE TABLE tpcds . customer_address ( ca_address_sk INTEGER NOT NULL , ca_address_id CHARACTER ( 16 ) NOT NULL );

-- 来源: 1329_EXECUTE DIRECT
DROP TABLE tpcds . customer_address ;

-- 来源: 1329_EXECUTE DIRECT
DROP SCHEMA tpcds ;

-- 来源: 1332_EXPLAIN
CREATE SCHEMA tpcds ;

-- 来源: 1332_EXPLAIN
CREATE TABLE tpcds . customer_address ( ca_address_sk INTEGER NOT NULL , ca_address_id CHARACTER ( 16 ) NOT NULL );

-- 来源: 1332_EXPLAIN
CREATE TABLE tpcds . customer_address_p1 AS TABLE tpcds . customer_address ;

-- 来源: 1332_EXPLAIN
DROP TABLE tpcds . customer_address_p1 ;

-- 来源: 1332_EXPLAIN
DROP TABLE tpcds . customer_address ;

-- 来源: 1332_EXPLAIN
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1333_EXPLAIN PLAN
CREATE TABLE foo1 ( f1 int , f2 text , f3 text []);

-- 来源: 1333_EXPLAIN PLAN
CREATE TABLE foo2 ( f1 int , f2 text , f3 text []);

-- 来源: 1333_EXPLAIN PLAN
DROP TABLE foo1 ;

-- 来源: 1333_EXPLAIN PLAN
DROP TABLE foo2 ;

-- 来源: 1333_EXPLAIN PLAN
CREATE TABLE pt_t1 ( a integer , b int , c int ) WITH ( autovacuum_enabled = off ) DISTRIBUTE hash ( c );

-- 来源: 1333_EXPLAIN PLAN
CREATE TABLE pt_t1 ( a int , b int , c int ) WITH ( autovacuum_enabled = off ) DISTRIBUTE hash ( c );

-- 来源: 1333_EXPLAIN PLAN
DROP TABLE pt_t1 ;

-- 来源: 1333_EXPLAIN PLAN
DROP TABLE pg_t2 ;

-- 来源: 1333_EXPLAIN PLAN
CREATE TABLE chinamap ( id integer , pid integer , name text ) DISTRIBUTE BY hash ( id );

-- 来源: 1333_EXPLAIN PLAN
DROP TABLE chinamap ;

-- 来源: 1335_FETCH
CREATE SCHEMA tpcds ;

-- 来源: 1335_FETCH
CREATE TABLE tpcds . customer_address ( ca_address_sk INTEGER NOT NULL , ca_address_id CHARACTER ( 16 ) NOT NULL , ca_street_number INTEGER , ca_street_name CHARACTER ( 20 ) );

-- 来源: 1335_FETCH
DROP TABLE tpcds . customer_address ;

-- 来源: 1335_FETCH
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1337_GRANT
CREATE USER joe PASSWORD 'xxxxxxxxxx' ;

-- 来源: 1337_GRANT
CREATE SCHEMA tpcds ;

-- 来源: 1337_GRANT
CREATE TABLE tpcds . reason ( r_reason_sk INTEGER NOT NULL , r_reason_id CHAR ( 16 ) NOT NULL , r_reason_desc VARCHAR ( 20 ) );

-- 来源: 1337_GRANT
CREATE DATABASE testdb ;

-- 来源: 1337_GRANT
CREATE ROLE tpcds_manager PASSWORD 'xxxxxxxxxx' ;

-- 来源: 1337_GRANT
CREATE TABLESPACE tpcds_tbspc RELATIVE LOCATION 'tablespace/tablespace_1' ;

-- 来源: 1337_GRANT
CREATE or replace FUNCTION tpcds.fun1() RETURN boolean AS BEGIN SELECT current_user;

-- 来源: 1337_GRANT
CREATE ROLE manager PASSWORD 'xxxxxxxxxxx' ;

-- 来源: 1337_GRANT
CREATE ROLE senior_manager PASSWORD 'xxxxxxxxxxx' ;

-- 来源: 1337_GRANT
DROP USER manager ;

-- 来源: 1337_GRANT
DROP DATABASE testdb ;

-- 来源: 1343_INSERT
CREATE SCHEMA tpcds ;

-- 来源: 1343_INSERT
CREATE TABLE tpcds . reason ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

-- 来源: 1343_INSERT
CREATE TABLE tpcds . reason_t2 ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

-- 来源: 1343_INSERT
CREATE UNIQUE INDEX reason_t2_u_index ON tpcds . reason_t2 ( r_reason_sk );

-- 来源: 1343_INSERT
DROP TABLE tpcds . reason_t2 ;

-- 来源: 1343_INSERT
DROP TABLE tpcds . reason ;

-- 来源: 1343_INSERT
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1345_LOCK
CREATE SCHEMA tpcds ;

-- 来源: 1345_LOCK
CREATE TABLE tpcds . reason ( r_reason_sk INTEGER NOT NULL , r_reason_id CHAR ( 16 ) NOT NULL , r_reason_desc INTEGER );

-- 来源: 1345_LOCK
CREATE TABLE tpcds . reason_t1 AS TABLE tpcds . reason ;

-- 来源: 1345_LOCK
DROP TABLE tpcds . reason_t1 ;

-- 来源: 1345_LOCK
DROP TABLE tpcds . reason ;

-- 来源: 1345_LOCK
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1349_MERGE INTO
CREATE TABLE products ( product_id INTEGER , product_name VARCHAR2 ( 60 ), category VARCHAR2 ( 60 ) );

-- 来源: 1349_MERGE INTO
CREATE TABLE newproducts ( product_id INTEGER , product_name VARCHAR2 ( 60 ), category VARCHAR2 ( 60 ) );

-- 来源: 1349_MERGE INTO
DROP TABLE products ;

-- 来源: 1349_MERGE INTO
DROP TABLE newproducts ;

-- 来源: 1350_MOVE
CREATE SCHEMA tpcds ;

-- 来源: 1350_MOVE
CREATE TABLE tpcds . reason ( r_reason_sk INTEGER NOT NULL , r_reason_id CHAR ( 16 ) NOT NULL , r_reason_desc VARCHAR ( 40 ) );

-- 来源: 1350_MOVE
DROP TABLE tpcds . reason ;

-- 来源: 1350_MOVE
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1355_PURGE
CREATE ROLE tpcds IDENTIFIED BY '*********';

-- 创建表空间reason_table_space。
-- 来源: 1355_PURGE
CREATE TABLESPACE REASON_TABLE_SPACE1 owner tpcds RELATIVE location 'tablespace/tsp_reason1';

-- 创建SCHEMA。
-- 来源: 1355_PURGE
CREATE SCHEMA tpcds;

-- 在表空间创建表tpcds.reason_t1。
-- 来源: 1355_PURGE
CREATE TABLE tpcds.reason_t1 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) with(storage_type=ustore) tablespace reason_table_space1;

-- 在表空间创建表tpcds.reason_t2。
-- 来源: 1355_PURGE
CREATE TABLE tpcds.reason_t2 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) with(storage_type=ustore) tablespace reason_table_space1;

-- 在表空间创建表tpcds.reason_t3。
-- 来源: 1355_PURGE
CREATE TABLE tpcds.reason_t3 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) with(storage_type=ustore) tablespace reason_table_space1;

-- 对表tpcds.reason_t1创建索引。
-- 来源: 1355_PURGE
CREATE INDEX index_t1 on tpcds.reason_t1(r_reason_id);

-- 来源: 1355_PURGE
DROP TABLE tpcds.reason_t1;

-- 来源: 1355_PURGE
DROP TABLE tpcds.reason_t2;

-- 来源: 1355_PURGE
DROP TABLE tpcds.reason_t3;

-- 删除SCHEMA。
-- 来源: 1355_PURGE
DROP SCHEMA tpcds CASCADE;

-- 来源: 1357_REASSIGN OWNED
CREATE USER jim PASSWORD '********' ;

-- 来源: 1357_REASSIGN OWNED
CREATE USER tom PASSWORD '********' ;

-- 来源: 1357_REASSIGN OWNED
DROP USER jim , tom CASCADE ;

-- 来源: 1358_REFRESH INCREMENTAL MATERIALIZED VIEW
CREATE TABLE my_table (c1 int, c2 int);

--创建增量物化视图。
-- 来源: 1358_REFRESH INCREMENTAL MATERIALIZED VIEW
CREATE INCREMENTAL MATERIALIZED VIEW my_imv AS SELECT * FROM my_table;

--删除增量物化视图。
-- 来源: 1358_REFRESH INCREMENTAL MATERIALIZED VIEW
DROP MATERIALIZED VIEW my_imv;

--删除表my_table。
-- 来源: 1358_REFRESH INCREMENTAL MATERIALIZED VIEW
DROP TABLE my_table;

-- 来源: 1359_REFRESH MATERIALIZED VIEW
CREATE TABLE my_table (c1 int, c2 int);

--创建全量物化视图。
-- 来源: 1359_REFRESH MATERIALIZED VIEW
CREATE MATERIALIZED VIEW my_mv AS SELECT * FROM my_table;

--创建增量物化视图。
-- 来源: 1359_REFRESH MATERIALIZED VIEW
CREATE INCREMENTAL MATERIALIZED VIEW my_imv AS SELECT * FROM my_table;

--删除增量物化视图。
-- 来源: 1359_REFRESH MATERIALIZED VIEW
DROP MATERIALIZED VIEW my_imv;

--删除全量物化视图。
-- 来源: 1359_REFRESH MATERIALIZED VIEW
DROP MATERIALIZED VIEW my_mv;

--删除表my_table。
-- 来源: 1359_REFRESH MATERIALIZED VIEW
DROP TABLE my_table;

-- 来源: 1360_REINDEX
CREATE SCHEMA tpcds ;

-- 来源: 1360_REINDEX
CREATE TABLE tpcds . customer ( c_customer_sk INTEGER NOT NULL , c_customer_id CHAR ( 16 ) NOT NULL );

-- 来源: 1360_REINDEX
CREATE TABLE tpcds . customer_t1 ( c_customer_sk integer not null , c_customer_id char ( 16 ) not null , c_current_cdemo_sk integer , c_current_hdemo_sk integer , c_current_addr_sk integer , c_first_shipto_date_sk integer , c_first_sales_date_sk integer , c_salutation char ( 10 ) , c_first_name char ( 20 ) , c_last_name char ( 30 ) , c_preferred_cust_flag char ( 1 ) , c_birth_day integer , c_birth_month integer , c_birth_year integer , c_birth_country varchar ( 20 ) , c_login char ( 13 ) , c_email_address char ( 50 ) , c_last_review_date char ( 10 ) ) WITH ( orientation = row );

-- 来源: 1360_REINDEX
CREATE INDEX tpcds_customer_index1 ON tpcds . customer_t1 ( c_customer_sk );

-- 来源: 1360_REINDEX
DROP TABLE tpcds . customer_t1 ;

-- 来源: 1360_REINDEX
DROP TABLE tpcds . customer ;

-- 来源: 1360_REINDEX
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1361_RELEASE SAVEPOINT
CREATE SCHEMA tpcds ;

-- 来源: 1361_RELEASE SAVEPOINT
CREATE TABLE tpcds . table1 ( a int );

-- 来源: 1361_RELEASE SAVEPOINT
DROP TABLE tpcds . table1 ;

-- 来源: 1361_RELEASE SAVEPOINT
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1362_REPLACE
CREATE TABLE test(f1 int primary key, f2 int, f3 int);

-- 来源: 1362_REPLACE
DROP TABLE test;

-- 来源: 1369_SAVEPOINT
CREATE TABLE table1 ( a int );

-- 来源: 1369_SAVEPOINT
DROP TABLE table1 ;

-- 来源: 1369_SAVEPOINT
CREATE TABLE table2 ( a int );

-- 来源: 1369_SAVEPOINT
DROP TABLE table2 ;

-- 来源: 1370_SECURITY LABEL ON
CREATE SECURITY LABEL sec_label 'L1:G4' ;

-- 来源: 1370_SECURITY LABEL ON
CREATE TABLE tbl ( c1 int , c2 int );

-- 来源: 1370_SECURITY LABEL ON
CREATE USER bob WITH PASSWORD '********' ;

-- 来源: 1370_SECURITY LABEL ON
DROP SECURITY LABEL sec_label ;

-- 来源: 1370_SECURITY LABEL ON
DROP TABLE tbl ;

-- 来源: 1370_SECURITY LABEL ON
DROP USER bob ;

-- 来源: 1371_SELECT
CREATE SCHEMA tpcds ;

-- 来源: 1371_SELECT
CREATE TABLE tpcds . reason ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

-- 来源: 1371_SELECT
CREATE TABLE tpcds . reason_p ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) ) PARTITION BY RANGE ( r_reason_sk ) ( partition P_05_BEFORE values less than ( 05 ), partition P_15 values less than ( 15 ), partition P_25 values less than ( 25 ), partition P_35 values less than ( 35 ), partition P_45_AFTER values less than ( MAXVALUE ) );

-- 来源: 1371_SELECT
CREATE TABLE tpcds . store_returns ( sr_item_sk int , sr_customer_id varchar ( 50 ), sr_customer_sk int );

-- 来源: 1371_SELECT
CREATE TABLE tpcds . customer ( c_item_sk int , c_customer_id varchar ( 50 ), c_customer_sk int );

-- 来源: 1371_SELECT
DROP TABLE tpcds.reason_p;

-- 来源: 1371_SELECT
CREATE DATABASE pivot_db dbcompatibility ' ORA ';

-- 来源: 1371_SELECT
DROP DATABASE pivot_db;

-- 来源: 1371_SELECT
CREATE TABLE skiplocked_astore(id int, info text) WITH (storage_type=astore);

-- 来源: 1371_SELECT
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1372_SELECT INTO
CREATE SCHEMA tpcds ;

-- 来源: 1372_SELECT INTO
CREATE TABLE tpcds . reason ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

-- 来源: 1372_SELECT INTO
DROP TABLE tpcds . reason_t1 , tpcds . reason ;

-- 来源: 1372_SELECT INTO
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1375_SET ROLE
CREATE ROLE paul IDENTIFIED BY '********' ;

-- 来源: 1375_SET ROLE
DROP USER paul ;

-- 来源: 1376_SET SESSION AUTHORIZATION
CREATE ROLE paul IDENTIFIED BY '********' ;

-- 来源: 1376_SET SESSION AUTHORIZATION
DROP USER paul ;

-- 来源: 1380_START TRANSACTION
CREATE SCHEMA tpcds ;

-- 来源: 1380_START TRANSACTION
CREATE TABLE tpcds . reason ( c1 int , c2 int );

-- 来源: 1380_START TRANSACTION
DROP TABLE tpcds . reason ;

-- 来源: 1380_START TRANSACTION
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1382_TIMECAPSULE TABLE
CREATE SCHEMA tpcds;

-- 删除表tpcds.reason_t2。
-- 来源: 1382_TIMECAPSULE TABLE
DROP TABLE IF EXISTS tpcds.reason_t2;

-- 创建表tpcds.reason_t2。
-- 来源: 1382_TIMECAPSULE TABLE
CREATE TABLE tpcds.reason_t2 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) )with(storage_type = ustore);

--删除表tpcds.reason_t2。
-- 来源: 1382_TIMECAPSULE TABLE
DROP TABLE tpcds.reason_t2;

-- 来源: 1382_TIMECAPSULE TABLE
DROP SCHEMA tpcds CASCADE;

-- 来源: 1383_TRUNCATE
CREATE SCHEMA tpcds ;

-- 来源: 1383_TRUNCATE
CREATE TABLE tpcds . reason ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

-- 来源: 1383_TRUNCATE
CREATE TABLE tpcds . reason_t1 AS TABLE tpcds . reason ;

-- 来源: 1383_TRUNCATE
DROP TABLE tpcds . reason_t1 ;

-- 来源: 1383_TRUNCATE
CREATE TABLE tpcds . reason_p ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) ) PARTITION BY RANGE ( r_reason_sk ) ( partition p_05_before values less than ( 05 ), partition p_15 values less than ( 15 ), partition p_25 values less than ( 25 ), partition p_35 values less than ( 35 ), partition p_45_after values less than ( MAXVALUE ) );

-- 来源: 1383_TRUNCATE
ALTER TABLE tpcds . reason_p TRUNCATE PARTITION p_05_before ;

-- 来源: 1383_TRUNCATE
ALTER TABLE tpcds . reason_p TRUNCATE PARTITION for ( 13 );

-- 来源: 1383_TRUNCATE
DROP TABLE tpcds . reason_p ;

-- 来源: 1383_TRUNCATE
DROP TABLE tpcds . reason ;

-- 来源: 1383_TRUNCATE
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1385_UPDATE
CREATE TABLE tbl_test1(id int, info varchar(10));

-- 删除tbl_test1表。
-- 来源: 1385_UPDATE
DROP TABLE tbl_test1;

-- 来源: 1385_UPDATE
CREATE TABLE test_grade ( sid int, --学号 name varchar(50), --姓名 score char, --成绩 examtime date, --考试时间 last_exam boolean --是否是最后一次考试 );

--删除。
-- 来源: 1385_UPDATE
DROP TABLE test_grade;

-- 来源: 1387_VACUUM
CREATE SCHEMA tpcds ;

-- 来源: 1387_VACUUM
CREATE TABLE tpcds . reason ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

-- 来源: 1387_VACUUM
CREATE UNIQUE INDEX ds_reason_index1 ON tpcds . reason ( r_reason_sk );

-- 来源: 1387_VACUUM
DROP INDEX tpcds . ds_reason_index1 CASCADE ;

-- 来源: 1387_VACUUM
DROP TABLE tpcds . reason ;

-- 来源: 1387_VACUUM
DROP SCHEMA tpcds CASCADE ;

-- 来源: 1420_file_1420
CREATE OR REPLACE PROCEDURE array_proc AS DECLARE TYPE ARRAY_INTEGER IS VARRAY ( 1024 ) OF INTEGER ;

-- 来源: 1420_file_1420
DROP PROCEDURE array_proc ;

-- 来源: 1423_file_1423
CREATE OR REPLACE PROCEDURE table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER ;

-- 来源: 1423_file_1423
DROP PROCEDURE table_proc ;

-- 来源: 1423_file_1423
CREATE OR REPLACE PROCEDURE nest_table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER ;

-- 来源: 1423_file_1423
DROP PROCEDURE nest_table_proc ;

-- 来源: 1423_file_1423
CREATE OR REPLACE PROCEDURE index_table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER INDEX BY INTEGER ;

-- 来源: 1423_file_1423
DROP PROCEDURE index_table_proc ;

-- 来源: 1423_file_1423
CREATE OR REPLACE PROCEDURE nest_table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER INDEX BY INTEGER ;

-- 来源: 1423_file_1423
DROP PROCEDURE nest_table_proc ;

-- 来源: 1424_file_1424
create or replace procedure p1 () gaussdb -# as gaussdb $ # type t1 is table of int ;

-- 来源: 1424_file_1424
drop procedure if exists p1 ();

-- 来源: 1424_file_1424
create or replace procedure p1 () is gaussdb $ # type rec is record ( c1 int , c2 int );

-- 来源: 1424_file_1424
drop procedure if exists p1 ();

-- 来源: 1424_file_1424
create or replace procedure p1 () gaussdb -# as gaussdb $ # type t1 is table of int index by int ;

-- 来源: 1424_file_1424
drop procedure if exists p1 ();

-- 来源: 1424_file_1424
create or replace procedure p1 () is gaussdb $ # type rec is record ( c1 int , c2 int );

-- 来源: 1424_file_1424
drop procedure if exists p1 ();

-- 来源: 1425_record
create table emp_rec ( gaussdb ( # empno numeric ( 4 , 0 ) not null , gaussdb ( # ename varchar ( 10 ) gaussdb ( # );

-- 来源: 1425_record
CREATE OR REPLACE FUNCTION regress_record ( p_w VARCHAR2 ) RETURNS VARCHAR2 AS $$ gaussdb $ # DECLARE gaussdb $ # --声明一个record类型. gaussdb $ # type rec_type is record ( name varchar2 ( 100 ), epno int );

-- 来源: 1425_record
DROP FUNCTION regress_record ;

-- 来源: 1425_record
DROP TABLE emp_rec ;

-- 来源: 1425_record
create type rec_type is ( c1 int , c2 int );

-- 来源: 1425_record
create or replace function func ( a in int ) return rec_type is gaussdb $ # r rec_type ;

-- 来源: 1425_record
drop function func ;

-- 来源: 1425_record
drop type rec_type ;

-- 来源: 1425_record
create or replace function func ( a out int ) return record is gaussdb $ # type rc is record ( c1 int , c2 int );

-- 来源: 1425_record
drop function func ;

-- 来源: 1435_file_1435
DROP TABLE IF EXISTS customers;

-- 来源: 1435_file_1435
CREATE TABLE customers(id int,name varchar);

-- 来源: 1436_file_1436
CREATE SCHEMA hr ;

-- 来源: 1436_file_1436
CREATE TABLE staffs ( section_id INTEGER , salary INTEGER );

-- 来源: 1436_file_1436
CREATE OR REPLACE PROCEDURE proc_staffs ( section NUMBER ( 6 ), salary_sum out NUMBER ( 8 , 2 ), staffs_count out INTEGER ) IS BEGIN SELECT sum ( salary ), count ( * ) INTO salary_sum , staffs_count FROM hr . staffs where section_id = section ;

-- 来源: 1436_file_1436
CREATE OR REPLACE PROCEDURE proc_return AS v_num NUMBER ( 8 , 2 );

-- 来源: 1436_file_1436
DROP PROCEDURE proc_staffs ;

-- 来源: 1436_file_1436
DROP PROCEDURE proc_return ;

-- 来源: 1436_file_1436
CREATE OR REPLACE FUNCTION func_return returns void language plpgsql AS $$ DECLARE v_num INTEGER : = 1 ;

-- 来源: 1436_file_1436
DROP FUNCTION func_return ;

-- 来源: 1436_file_1436
DROP SCHEMA hr CASCADE ;

-- 来源: 1438_file_1438
DROP SCHEMA IF EXISTS hr CASCADE ;

-- 来源: 1438_file_1438
CREATE SCHEMA hr ;

-- 来源: 1438_file_1438
CREATE TABLE staffs ( staff_id NUMBER , first_name VARCHAR2 , salary NUMBER );

-- 来源: 1438_file_1438
CREATE OR REPLACE PROCEDURE dynamic_proc AS staff_id NUMBER ( 6 ) : = 200 ;

-- 来源: 1438_file_1438
DROP PROCEDURE dynamic_proc ;

-- 来源: 1438_file_1438
CREATE SCHEMA hr ;

-- 来源: 1438_file_1438
CREATE TABLE staffs ( section_id NUMBER , first_name VARCHAR2 , phone_number VARCHAR2 , salary NUMBER );

-- 来源: 1439_file_1439
CREATE TABLE sections_t1 ( section NUMBER ( 4 ) , section_name VARCHAR2 ( 30 ), manager_id NUMBER ( 6 ), place_id NUMBER ( 4 ) ) DISTRIBUTE BY hash ( manager_id );

-- 来源: 1439_file_1439
DROP TABLE sections_t1 ;

-- 来源: 1440_file_1440
CREATE OR REPLACE PROCEDURE proc_add ( param1 in INTEGER , param2 out INTEGER , param3 in INTEGER ) AS BEGIN param2 : = param1 + param3 ;

-- 来源: 1440_file_1440
DROP PROCEDURE proc_add ;

-- 来源: 1441_file_1441
DROP SCHEMA IF EXISTS hr CASCADE ;

-- 来源: 1441_file_1441
CREATE SCHEMA hr ;

-- 来源: 1441_file_1441
CREATE TABLE staffs ( staff_id NUMBER , first_name VARCHAR2 , salary NUMBER );

-- 来源: 1441_file_1441
CREATE OR REPLACE PROCEDURE dynamic_proc AS staff_id NUMBER ( 6 ) : = 200 ;

-- 来源: 1441_file_1441
DROP PROCEDURE dynamic_proc ;

-- 来源: 1445_RETURN NEXTRETURN QUERY
DROP TABLE t1 ;

-- 来源: 1445_RETURN NEXTRETURN QUERY
CREATE TABLE t1 ( a int );

-- 来源: 1445_RETURN NEXTRETURN QUERY
CREATE OR REPLACE FUNCTION fun_for_return_next () RETURNS SETOF t1 AS $$ DECLARE r t1 % ROWTYPE ;

-- 来源: 1445_RETURN NEXTRETURN QUERY
CREATE OR REPLACE FUNCTION fun_for_return_query () RETURNS SETOF t1 AS $$ DECLARE r t1 % ROWTYPE ;

-- 来源: 1446_file_1446
CREATE OR REPLACE PROCEDURE proc_control_structure ( i in integer ) AS BEGIN IF i > 0 THEN raise info 'i:% is greater than 0. ' , i ;

-- 来源: 1446_file_1446
DROP PROCEDURE proc_control_structure ;

-- 来源: 1447_file_1447
CREATE OR REPLACE PROCEDURE proc_loop ( i in integer , count out integer ) AS BEGIN count : = 0 ;

-- 来源: 1447_file_1447
CREATE TABLE integertable ( c1 integer ) DISTRIBUTE BY hash ( c1 );

-- 来源: 1447_file_1447
CREATE OR REPLACE PROCEDURE proc_while_loop ( maxval in integer ) AS DECLARE i int : = 1 ;

-- 来源: 1447_file_1447
DROP PROCEDURE proc_while_loop ;

-- 来源: 1447_file_1447
DROP TABLE integertable ;

-- 来源: 1447_file_1447
CREATE OR REPLACE PROCEDURE proc_for_loop () AS BEGIN FOR I IN 0 .. 5 LOOP DBE_OUTPUT . PRINT_LINE ( 'It is ' || to_char ( I ) || ' time;

-- 来源: 1447_file_1447
DROP PROCEDURE proc_for_loop ;

-- 来源: 1447_file_1447
CREATE OR REPLACE PROCEDURE proc_for_loop_query () AS record VARCHAR2 ( 50 );

-- 来源: 1447_file_1447
DROP PROCEDURE proc_for_loop_query ;

-- 来源: 1447_file_1447
CREATE TABLE TEST_t1 ( title NUMBER ( 6 ), did VARCHAR2 ( 20 ), data_period VARCHAR2 ( 25 ), kind VARCHAR2 ( 25 ), interval VARCHAR2 ( 20 ), time DATE , isModified VARCHAR2 ( 10 ) ) DISTRIBUTE BY hash ( did );

-- 来源: 1447_file_1447
CREATE OR REPLACE PROCEDURE proc_forall () AS BEGIN FORALL i IN 100 .. 120 update TEST_t1 set title = title + 100 * i ;

-- 来源: 1447_file_1447
DROP PROCEDURE proc_forall ;

-- 来源: 1447_file_1447
DROP TABLE TEST_t1 ;

-- 来源: 1448_file_1448
CREATE OR REPLACE PROCEDURE proc_case_branch ( pi_result in integer , pi_return out integer ) AS BEGIN CASE pi_result WHEN 1 THEN pi_return : = 111 ;

-- 来源: 1448_file_1448
DROP PROCEDURE proc_case_branch ;

-- 来源: 1450_file_1450
CREATE TABLE mytab ( id INT , firstname VARCHAR ( 20 ), lastname VARCHAR ( 20 )) DISTRIBUTE BY hash ( id );

-- 来源: 1450_file_1450
CREATE FUNCTION fun_exp () RETURNS INT AS $$ DECLARE x INT : = 0 ;

-- 来源: 1450_file_1450
DROP FUNCTION fun_exp ();

-- 来源: 1450_file_1450
DROP TABLE mytab ;

-- 来源: 1450_file_1450
CREATE TABLE db ( a INT , b TEXT );

-- 来源: 1450_file_1450
CREATE FUNCTION merge_db ( key INT , data TEXT ) RETURNS VOID AS $$ BEGIN LOOP --第一次尝试更新key UPDATE db SET b = data WHERE a = key ;

-- 来源: 1450_file_1450
DROP FUNCTION merge_db ;

-- 来源: 1450_file_1450
DROP TABLE db ;

-- 来源: 1451_GOTO
CREATE OR REPLACE PROCEDURE GOTO_test () AS DECLARE v1 int ;

-- 来源: 1452_file_1452
DROP TABLE IF EXISTS EXAMPLE1;

-- 来源: 1452_file_1452
CREATE TABLE EXAMPLE1(COL1 INT);

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE() AS BEGIN FOR i IN 0..20 LOOP INSERT INTO EXAMPLE1(COL1) VALUES (i);

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE TEST_COMMIT_INSERT_EXCEPTION_ROLLBACK() AS BEGIN DROP TABLE IF EXISTS TEST_COMMIT;

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE TEST_COMMIT2() IS BEGIN DROP TABLE IF EXISTS TEST_COMMIT;

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE GUC_ROLLBACK() AS BEGIN SET enable_force_vector_engine = on;

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE1() AS BEGIN INSERT INTO EXAMPLE1 VALUES(1);

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE2() AS BEGIN INSERT INTO EXAMPLE1 VALUES(2);

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE3() AS BEGIN INSERT INTO EXAMPLE1 VALUES(1);

-- 来源: 1452_file_1452
CREATE OR REPLACE FUNCTION FUNCTION_EXAMPLE1() RETURN INT AS EXP INT;

-- 来源: 1452_file_1452
CREATE OR REPLACE FUNCTION FUNCTION_TRI_EXAMPLE2() RETURN TRIGGER AS EXP INT;

-- 来源: 1452_file_1452
CREATE TRIGGER TRIGGER_EXAMPLE AFTER DELETE ON EXAMPLE1 FOR EACH ROW EXECUTE PROCEDURE FUNCTION_TRI_EXAMPLE2();

-- 来源: 1452_file_1452
DROP TABLE IF EXISTS EXAMPLE1;

-- 来源: 1452_file_1452
CREATE TABLE EXAMPLE1(COL1 INT);

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE1() IMMUTABLE AS EXP INT;

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE2(EXP_OUT OUT INT) AS EXP INT;

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE3() AS BEGIN FOR i IN 0..20 LOOP INSERT INTO EXAMPLE1 (col1) VALUES (i);

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE4() SET ARRAY_NULLS TO "ON" AS BEGIN FOR i IN 0..20 LOOP INSERT INTO EXAMPLE1 (col1) VALUES (i);

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE5(INTIN IN INT, INTOUT OUT INT) AS BEGIN INTOUT := INTIN + 1;

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE6() AS CURSOR CURSOR1(EXPIN INT) IS SELECT TRANSACTION_EXAMPLE5(EXPIN);

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE exec_func1() AS BEGIN CREATE TABLE TEST_exec(A INT);

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE exec_func2() AS BEGIN EXECUTE exec_func1();

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE exec_func3(RET_NUM OUT INT) AS BEGIN RET_NUM := 1+1;

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE exec_func4(ADD_NUM IN INT) AS SUM_NUM INT;

-- 来源: 1452_file_1452
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE3() AS BEGIN INSERT INTO EXAMPLE1 VALUES(2);

-- 来源: 1458_file_1458
drop schema if exists hr cascade;

-- 来源: 1458_file_1458
create schema hr;

-- 来源: 1458_file_1458
drop table if exists sections;

-- 来源: 1458_file_1458
drop table if exists staffs;

-- 来源: 1458_file_1458
drop table if exists department;

--创建部门表
-- 来源: 1458_file_1458
create table sections( section_name varchar(100), place_id int, section_id int );

--创建员工表
-- 来源: 1458_file_1458
create table staffs( staff_id number(6), salary number(8,2), section_id int, first_name varchar(20) );

--创建部门表
-- 来源: 1458_file_1458
create table department( section_id int );

-- 来源: 1458_file_1458
CREATE OR REPLACE PROCEDURE cursor_proc1 () AS DECLARE DEPT_NAME VARCHAR ( 100 );

-- 来源: 1458_file_1458
DROP PROCEDURE cursor_proc1 ;

-- 来源: 1458_file_1458
CREATE TABLE hr . staffs_t1 AS TABLE hr . staffs ;

-- 来源: 1458_file_1458
CREATE OR REPLACE PROCEDURE cursor_proc2 () AS DECLARE V_EMPNO NUMBER ( 6 );

-- 来源: 1458_file_1458
DROP PROCEDURE cursor_proc2 ;

-- 来源: 1458_file_1458
DROP TABLE hr . staffs_t1 ;

-- 来源: 1458_file_1458
CREATE OR REPLACE PROCEDURE proc_sys_ref ( O OUT SYS_REFCURSOR ) IS C1 SYS_REFCURSOR ;

-- 来源: 1458_file_1458
DROP PROCEDURE proc_sys_ref ;

-- 来源: 1459_file_1459
CREATE OR REPLACE PROCEDURE proc_cursor3 () AS DECLARE V_DEPTNO NUMBER ( 4 ) : = 100 ;

-- 来源: 1459_file_1459
DROP PROCEDURE proc_cursor3 ;

-- 来源: 1460_file_1460
CREATE TABLE integerTable1 ( A INTEGER ) DISTRIBUTE BY hash ( A );

-- 来源: 1460_file_1460
CREATE TABLE integerTable2 ( B INTEGER ) DISTRIBUTE BY hash ( B );

-- 来源: 1460_file_1460
DROP TABLE integerTable1 ;

-- 来源: 1460_file_1460
DROP TABLE integerTable2 ;

-- 来源: 1468_DBE_COMPRESSION
create database ilmtabledb with dbcompatibility = 'ORA' ;

-- 来源: 1468_DBE_COMPRESSION
ALTER DATABASE set ilm = on ;

-- 来源: 1468_DBE_COMPRESSION
CREATE user user1 IDENTIFIED BY 'Gauss_123' ;

-- 来源: 1468_DBE_COMPRESSION
CREATE TABLE TEST_DATA ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) with ( storage_type = astore ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

-- 来源: 1468_DBE_COMPRESSION
create database ilmtabledb with dbcompatibility = 'ORA' ;

-- 来源: 1468_DBE_COMPRESSION
alter database set ilm = on ;

-- 来源: 1468_DBE_COMPRESSION
CREATE user user1 IDENTIFIED BY 'Gauss_1234' ;

-- 来源: 1468_DBE_COMPRESSION
CREATE TABLE TEST_DATA ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

-- 来源: 1470_DBE_ILM
CREATE DATABASE ilmtabledb with dbcompatibility = 'ORA' ;

-- 来源: 1470_DBE_ILM
ALTER DATABASE set ilm = on ;

-- 来源: 1470_DBE_ILM
CREATE Schema ILM_DATA ;

-- 来源: 1470_DBE_ILM
CREATE SEQUENCE ILM_DATA . ORDER_TABLE_SE_ORDER_ID MINVALUE 1 ;

-- 来源: 1470_DBE_ILM
CREATE OR REPLACE PROCEDURE ILM_DATA . ORDER_TABLE_CREATE_DATA ( NUM INTEGER ) IS BEGIN FOR X IN 1 .. NUM LOOP INSERT INTO ORDER_TABLE VALUES ( ORDER_TABLE_SE_ORDER_ID . nextval , '零食大礼包A' , NOW ());

-- 来源: 1470_DBE_ILM
CREATE TABLE ILM_DATA . ORDER_TABLE ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) WITH ( STORAGE_TYPE = ASTORE ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

-- 来源: 1477_DBE_SCHEDULER
create user test1 identified by '*********';

-- 来源: 1477_DBE_SCHEDULER
drop user test1;

-- 来源: 1477_DBE_SCHEDULER
create user user1 password '1*s*****';

-- 来源: 1477_DBE_SCHEDULER
drop user user1;

-- 来源: 1477_DBE_SCHEDULER
create user user1 password '1*s*****';

-- 来源: 1477_DBE_SCHEDULER
drop user user1;

-- 来源: 1477_DBE_SCHEDULER
CREATE OR REPLACE PROCEDURE pr1(calendar_str text) as DECLARE start_date timestamp with time zone;

-- 来源: 1485_Retry
CREATE OR REPLACE PROCEDURE retry_basic ( IN x INT ) AS BEGIN INSERT INTO t1 ( a ) VALUES ( x );

-- 来源: 1489_file_1489
CREATE TABLE test_trigger_des_tbl(id1 int, id2 int, id3 int);

-- 来源: 1489_file_1489
CREATE OR REPLACE FUNCTION tri_insert_func() RETURNS TRIGGER AS $$ DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 1489_file_1489
DROP TABLE test_trigger_des_tbl;

-- 来源: 1489_file_1489
CREATE TABLE t1(a INT ,b TEXT);

-- 来源: 1489_file_1489
DROP TABLE t1;

-- 来源: 1489_file_1489
CREATE TABLE sections(section_id INT);

-- 来源: 1489_file_1489
CREATE OR REPLACE PROCEDURE proc_sys_ref(OUT c1 REFCURSOR) IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 1489_file_1489
CREATE OR REPLACE PROCEDURE proc_sys_call() AS DECLARE c1 SYS_REFCURSOR;

-- 来源: 1489_file_1489
CREATE OR REPLACE PROCEDURE proc_sys_ref(IN c1 REFCURSOR) IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 1489_file_1489
CREATE OR REPLACE PROCEDURE proc_sys_call() AS DECLARE c1 SYS_REFCURSOR;

-- 来源: 1489_file_1489
DROP PROCEDURE IF EXISTS proc_sys_ref;

-- 来源: 1489_file_1489
CREATE OR REPLACE function proc_sys_ref() RETURN SYS_REFCURSOR IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 1489_file_1489
CREATE OR REPLACE FUNCTION proc_sys_ref(c1 OUT SYS_REFCURSOR) RETURN SYS_REFCURSOR IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 1489_file_1489
CREATE OR REPLACE PROCEDURE proc_sys_ref(OUT c1 REFCURSOR) IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 1489_file_1489
DROP PROCEDURE proc_sys_ref;

-- 来源: 1489_file_1489
DROP TABLE sections;

-- 来源: 1489_file_1489
CREATE OR REPLACE PROCEDURE autonomous_test_in_p_116(num1 INT ) IMMUTABLE AS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 1489_file_1489
CREATE OR REPLACE PROCEDURE autonomous_test_in_p_117(num1 INT ) STABLE AS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 1489_file_1489
CREATE TABLE test_lock (id INT,a DATE);

-- 来源: 1489_file_1489
CREATE OR REPLACE FUNCTION autonomous_test_lock(num1 INT,num2 INT) RETURNS INTEGER LANGUAGE plpgsql AS $$ DECLARE num3 INT := 4;

-- 来源: 1489_file_1489
DROP TABLE test_lock;

-- 来源: 1489_file_1489
CREATE OR REPLACE FUNCTION auto_func() RETURN RECORD AS DECLARE TYPE rec_type IS RECORD(c1 INT, c2 INT);

-- 来源: 1489_file_1489
CREATE OR REPLACE PROCEDURE auto_func(r INT) AS DECLARE a INT;

-- 来源: 1489_file_1489
CREATE OR REPLACE FUNCTION test_set() RETURN SETOF INT AS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 1490_file_1490
CREATE TABLE t2(a INT, b INT);

-- 来源: 1490_file_1490
CREATE OR REPLACE PROCEDURE autonomous_4(a INT, b INT) AS DECLARE num3 INT := a;

-- 来源: 1490_file_1490
CREATE OR REPLACE PROCEDURE autonomous_5(a INT, b INT) AS DECLARE BEGIN DBE_OUTPUT.PRINT_LINE('JUST NO USE CALL.');

-- 来源: 1490_file_1490
DROP TABLE t2;

-- 来源: 1491_file_1491
CREATE TABLE t1(a INT ,B TEXT);

-- 来源: 1491_file_1491
DROP TABLE t1;

-- 来源: 1492_file_1492
CREATE TABLE t4(a INT, b INT, c TEXT);

-- 来源: 1492_file_1492
CREATE OR REPLACE FUNCTION autonomous_32(a INT ,b INT ,c TEXT) RETURN INT AS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 1492_file_1492
CREATE OR REPLACE FUNCTION autonomous_33(num1 INT) RETURN INT AS DECLARE num3 INT := 220;

-- 来源: 1492_file_1492
DROP TABLE t4;

-- 来源: 1493_PACKAGE
create database test DBCOMPATIBILITY = 'ORA';

-- 来源: 1493_PACKAGE
CREATE TABLE t2(a INT, b INT);

-- 来源: 1493_PACKAGE
CREATE OR REPLACE PACKAGE autonomous_pkg AS PROCEDURE autonomous_4(a INT, b INT);

-- 来源: 1493_PACKAGE
CREATE OR REPLACE PACKAGE BODY autonomous_pkg AS PROCEDURE autonomous_4(a INT, b INT) AS DECLARE num3 INT := a;

-- 来源: 1493_PACKAGE
CREATE OR REPLACE PROCEDURE autonomous_5(a INT, b INT) AS DECLARE va INT;

-- 来源: 1493_PACKAGE
DROP TABLE t2;

-- 来源: 2292_DBE_PLDEBUGGER Schema
CREATE OR REPLACE PROCEDURE test_debug ( IN x INT ) AS BEGIN INSERT INTO t1 ( a ) VALUES ( x );

-- 来源: 2366_file_2366
create table test1 ( c1 int , c2 varchar );

-- 来源: 2366_file_2366
create table tab_1(col1 varchar(3));

-- 来源: 2366_file_2366
create table tab_2(col2 char(3));

-- 来源: 2366_file_2366
create or replace function test(f1 int, f2 int default 20, f3 int, f4 int default 40, f5 int default 50) return int gaussdb -# as gaussdb $# begin gaussdb $# raise info 'f1:%',f1;

-- 来源: 2366_file_2366
create or replace function test(f1 int, f2 int default 20, f3 int, f4 int default 40, f5 int default 50) return int gaussdb -# as gaussdb $# begin gaussdb $# raise info 'f1:%',f1;

-- 来源: 2366_file_2366
CREATE OR REPLACE PROCEDURE out_param_test1(m in int, v inout varchar2,v1 inout varchar2) is gaussdb$# begin gaussdb$# v := 'aaaddd';

-- 来源: 2366_file_2366
CREATE OR REPLACE PROCEDURE call_out_param_test1 is gaussdb$# v varchar2(5) := 'aabbb';

-- 来源: 2366_file_2366
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of varchar(5);

-- 来源: 2366_file_2366
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of varchar(5);

-- 来源: 2366_file_2366
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of int index by varchar(5);

-- 来源: 2366_file_2366
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of int index by varchar(5);

-- 来源: 2366_file_2366
create or replace function proc_test return varchar2 as gaussdb$# begin gaussdb$# return '1';

-- 来源: 2366_file_2366
create or replace procedure proc_test as gaussdb$# begin gaussdb$# null;

-- 来源: 2366_file_2366
create or replace procedure proc_test as gaussdb$# begin gaussdb$# null;

-- 来源: 2366_file_2366
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- 来源: 2366_file_2366
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- 来源: 2366_file_2366
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- 来源: 2366_file_2366
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- 来源: 2366_file_2366
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- 来源: 2366_file_2366
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- 来源: 2366_file_2366
create table test(c1 varchar2);

-- 来源: 2366_file_2366
create user plsql_rollback1 password 'huawei@123';

-- 来源: 2366_file_2366
create user plsql_rollback2 password 'huawei@123';

-- 来源: 2366_file_2366
create or replace procedure plsql_rollback1.p1 () authid definer as gaussdb$# va int;

-- 来源: 2431_file_2431
create table a(id int, value int);

-- 来源: 2431_file_2431
create table a(id int primary key, value int);

-- 来源: 2436_file_2436
CREATE USER sysadmin WITH SYSADMIN password "********" ;

-- 来源: 2436_file_2436
ALTER USER joe SYSADMIN ;

-- 来源: 2436_file_2436
CREATE USER createrole WITH CREATEROLE password "********" ;

-- 来源: 2436_file_2436
ALTER USER joe CREATEROLE ;

-- 来源: 2436_file_2436
CREATE USER auditadmin WITH AUDITADMIN password "********" ;

-- 来源: 2436_file_2436
ALTER USER joe AUDITADMIN ;

-- 来源: 2436_file_2436
CREATE USER monadmin WITH MONADMIN password "********" ;

-- 来源: 2436_file_2436
ALTER USER joe MONADMIN ;

-- 来源: 2436_file_2436
CREATE USER opradmin WITH OPRADMIN password "********" ;

-- 来源: 2436_file_2436
ALTER USER joe OPRADMIN ;

-- 来源: 2436_file_2436
CREATE USER poladmin WITH POLADMIN password "********" ;

-- 来源: 2436_file_2436
ALTER USER joe POLADMIN ;

-- 来源: 2438_file_2438
CREATE USER joe WITH CREATEDB PASSWORD "********" ;

-- 来源: 2438_file_2438
CREATE USER user_persistence WITH PERSISTENCE IDENTIFIED BY "********" ;

-- 来源: 2441_file_2441
CREATE ROLE lily WITH CREATEDB PASSWORD "********" ;

-- 来源: 2442_file_2442
CREATE USER alice PASSWORD '********' ;

-- 来源: 2442_file_2442
CREATE USER bob PASSWORD '********' ;

-- 来源: 2442_file_2442
CREATE USER peter PASSWORD '********' ;

-- 来源: 2442_file_2442
CREATE TABLE all_data ( id int , role varchar ( 100 ), data varchar ( 100 ));

-- 来源: 2442_file_2442
ALTER TABLE all_data ENABLE ROW LEVEL SECURITY ;

-- 来源: 2449_file_2449
CREATE USER joe WITH PASSWORD "********" ;

-- 来源: 2450_file_2450
CREATE DATABASE db_tpcc ;

-- 来源: 2450_file_2450
CREATE DATABASE db_tpcc WITH TABLESPACE = hr_local ;

-- 来源: 2450_file_2450
ALTER DATABASE db_tpcc SET search_path TO pa_catalog , public ;

-- 来源: 2450_file_2450
ALTER DATABASE db_tpcc RENAME TO human_tpcds ;

-- 来源: 2450_file_2450
DROP DATABASE human_tpcds ;

-- 来源: 2451_file_2451
CREATE TABLE customer_t1 ( state_ID CHAR ( 2 ), state_NAME VARCHAR2 ( 40 ), area_ID NUMBER );

-- 来源: 2451_file_2451
DROP TABLE customer_t1 ;

-- 来源: 2451_file_2451
CREATE TABLE customer_t2 ( state_ID CHAR ( 2 ), state_NAME VARCHAR2 ( 40 ), area_ID NUMBER ) WITH ( ORIENTATION = COLUMN );

-- 来源: 2451_file_2451
DROP TABLE customer_t2 ;

-- 来源: 2452_file_2452
CREATE USER jack IDENTIFIED BY '********' ;

-- 来源: 2452_file_2452
CREATE TABLESPACE fastspace RELATIVE LOCATION 'tablespace/tablespace_1' ;

-- 来源: 2452_file_2452
CREATE TABLE foo ( i int ) TABLESPACE fastspace ;

-- 来源: 2452_file_2452
CREATE TABLE foo2 ( i int );

-- 来源: 2452_file_2452
ALTER TABLESPACE fastspace RENAME TO fspace ;

-- 来源: 2452_file_2452
DROP USER jack CASCADE ;

-- 来源: 2452_file_2452
DROP TABLE foo ;

-- 来源: 2452_file_2452
DROP TABLE foo2 ;

-- 来源: 2452_file_2452
DROP TABLESPACE fspace ;

-- 来源: 2454_file_2454
CREATE TABLE customer_t1 ( c_customer_sk integer , c_customer_id char ( 5 ), c_first_name char ( 6 ), c_last_name char ( 8 ) );

-- 来源: 2455_file_2455
CREATE TABLE table1 ( id int , a char ( 6 ), b varchar ( 6 ), c varchar ( 6 ));

-- 来源: 2455_file_2455
CREATE TABLE table2 ( id int , a char ( 20 ), b varchar ( 20 ), c varchar ( 20 ));

-- 来源: 2455_file_2455
CREATE TABLE customer_t2 ( c_customer_sk integer , c_customer_id char ( 5 ), c_first_name char ( 6 ), c_last_name char ( 8 ) );

-- 来源: 2455_file_2455
DROP TABLE customer_t2 CASCADE ;

-- 来源: 2458_file_2458
DROP TABLE customer_t1 ;

-- 来源: 2459_file_2459
CREATE TABLE public.search_table_t1(a int);

-- 来源: 2459_file_2459
CREATE TABLE public.search_table_t2(b int);

-- 来源: 2459_file_2459
CREATE TABLE public.search_table_t3(c int);

-- 来源: 2459_file_2459
CREATE TABLE public.search_table_t4(d int);

-- 来源: 2459_file_2459
CREATE TABLE public.search_table_t5(e int);

-- 来源: 2461_schema
CREATE SCHEMA myschema ;

-- 来源: 2461_schema
CREATE SCHEMA myschema AUTHORIZATION omm ;

-- 来源: 2461_schema
CREATE TABLE myschema . mytable ( id int , name varchar ( 20 ));

-- 来源: 2461_schema
CREATE USER jack IDENTIFIED BY '********' ;

-- 来源: 2461_schema
DROP SCHEMA IF EXISTS nullschema ;

-- 来源: 2461_schema
DROP SCHEMA myschema CASCADE ;

-- 来源: 2461_schema
DROP USER jack ;

-- 来源: 2462_file_2462
CREATE TABLE tpcds . customer_address ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) ) ENABLE ROW MOVEMENT ;

-- 来源: 2462_file_2462
CREATE TABLE tpcds . web_returns_p2 ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) ) ENABLE ROW MOVEMENT ;

-- 来源: 2462_file_2462
ALTER TABLE tpcds . web_returns_p2 DISABLE ROW MOVEMENT ;

-- 来源: 2462_file_2462
ALTER TABLE tpcds . web_returns_p2 DROP PARTITION P8 ;

-- 来源: 2462_file_2462
ALTER TABLE tpcds . web_returns_p2 ADD PARTITION P8 VALUES LESS THAN ( MAXVALUE );

-- 来源: 2462_file_2462
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION P8 TO P_9 ;

-- 来源: 2462_file_2462
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION FOR ( 40000 ) TO P8 ;

-- 来源: 2462_file_2462
DROP TABLE tpcds . customer_address ;

-- 来源: 2462_file_2462
DROP TABLE tpcds . web_returns_p2 ;

-- 来源: 2462_file_2462
CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1/tablespace_1' ;

-- 来源: 2462_file_2462
CREATE TABLESPACE example2 RELATIVE LOCATION 'tablespace2/tablespace_2' ;

-- 来源: 2462_file_2462
CREATE TABLESPACE example3 RELATIVE LOCATION 'tablespace3/tablespace_3' ;

-- 来源: 2462_file_2462
CREATE TABLESPACE example4 RELATIVE LOCATION 'tablespace4/tablespace_4' ;

-- 来源: 2462_file_2462
CREATE TABLE tpcds . customer_address ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) TABLESPACE example1 PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) TABLESPACE example2 ) ENABLE ROW MOVEMENT ;

-- 来源: 2462_file_2462
CREATE TABLE tpcds . web_returns_p2 ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) TABLESPACE example1 PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) TABLESPACE example2 ) ENABLE ROW MOVEMENT ;

-- 来源: 2462_file_2462
ALTER TABLE tpcds . web_returns_p2 DISABLE ROW MOVEMENT ;

-- 来源: 2462_file_2462
ALTER TABLE tpcds . web_returns_p2 DROP PARTITION P8 ;

-- 来源: 2462_file_2462
ALTER TABLE tpcds . web_returns_p2 ADD PARTITION P8 VALUES LESS THAN ( MAXVALUE );

-- 来源: 2462_file_2462
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION P8 TO P_9 ;

-- 来源: 2462_file_2462
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION FOR ( 40000 ) TO P8 ;

-- 来源: 2462_file_2462
ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P6 TABLESPACE example3 ;

-- 来源: 2462_file_2462
ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P4 TABLESPACE example4 ;

-- 来源: 2462_file_2462
DROP TABLE tpcds . web_returns_p2 ;

-- 来源: 2462_file_2462
DROP TABLESPACE example1 ;

-- 来源: 2462_file_2462
DROP TABLESPACE example2 ;

-- 来源: 2462_file_2462
DROP TABLESPACE example3 ;

-- 来源: 2462_file_2462
DROP TABLESPACE example4 ;

-- 来源: 2463_file_2463
CREATE INDEX tpcds_web_returns_p2_index1 ON tpcds . web_returns_p2 ( ca_address_id ) LOCAL ;

-- 来源: 2463_file_2463
CREATE INDEX tpcds_web_returns_p2_index2 ON tpcds . web_returns_p2 ( ca_address_sk ) LOCAL ( PARTITION web_returns_p2_P1_index , PARTITION web_returns_p2_P2_index TABLESPACE example3 , PARTITION web_returns_p2_P3_index TABLESPACE example4 , PARTITION web_returns_p2_P4_index , PARTITION web_returns_p2_P5_index , PARTITION web_returns_p2_P6_index , PARTITION web_returns_p2_P7_index , PARTITION web_returns_p2_P8_index ) TABLESPACE example2 ;

-- 来源: 2463_file_2463
ALTER INDEX tpcds . tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P2_index TABLESPACE example1 ;

-- 来源: 2463_file_2463
ALTER INDEX tpcds . tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P3_index TABLESPACE example2 ;

-- 来源: 2463_file_2463
ALTER INDEX tpcds . tpcds_web_returns_p2_index2 RENAME PARTITION web_returns_p2_P8_index TO web_returns_p2_P8_index_new ;

-- 来源: 2463_file_2463
DROP INDEX tpcds . tpcds_web_returns_p2_index1 ;

-- 来源: 2463_file_2463
DROP INDEX tpcds . tpcds_web_returns_p2_index2 ;

-- 来源: 2463_file_2463
CREATE TABLE tpcds . customer_address_bak AS TABLE tpcds . customer_address ;

-- 来源: 2463_file_2463
CREATE INDEX index_wr_returned_date_sk ON tpcds . customer_address_bak ( ca_address_sk );

-- 来源: 2463_file_2463
CREATE UNIQUE INDEX ds_ship_mode_t1_index1 ON tpcds. ship_mode_t1(SM_SHIP_MODE_SK);

-- 来源: 2463_file_2463
CREATE INDEX more_column_index ON tpcds . customer_address_bak ( ca_address_sk , ca_street_number );

-- 来源: 2463_file_2463
CREATE INDEX part_index ON tpcds . customer_address_bak ( ca_address_sk ) WHERE ca_address_sk = 5050 ;

-- 来源: 2463_file_2463
CREATE INDEX para_index ON tpcds . customer_address_bak ( trunc ( ca_street_number ));

-- 来源: 2463_file_2463
DROP TABLE tpcds . customer_address_bak ;

-- 来源: 2464_file_2464
CREATE OR REPLACE VIEW MyView AS SELECT * FROM tpcds . web_returns WHERE trunc ( wr_refunded_cash ) > 10000 ;

-- 来源: 2464_file_2464
DROP VIEW MyView ;

-- 来源: 2465_file_2465
CREATE TABLE T1 ( id serial , name text );

-- 来源: 2465_file_2465
CREATE SEQUENCE seq1 cache 100 ;

-- 来源: 2465_file_2465
CREATE TABLE T2 ( id int not null default nextval ( 'seq1' ), name text );

-- 来源: 2465_file_2465
ALTER SEQUENCE seq1 OWNED BY T2 . id ;

-- 来源: 2466_file_2466
CREATE TABLE test ( id int , time date );

-- 来源: 2466_file_2466
CREATE OR REPLACE PROCEDURE PRC_JOB_1 () AS N_NUM integer : = 1 ;

-- 来源: 2702_file_2702
create index idx on t1 ( c1 );

-- 来源: 2707_HintQueryblock
create view v1 as select/*+ no_expand */ c1 from t1 where c1 in (select /*+ no_expand */ c1 from t2 where t2.c3=4 );

-- 来源: 2731_SQL PATCH
create table hint_t1 ( a int , b int , c int );

-- 来源: 2731_SQL PATCH
create index on hint_t1 ( a );

-- 来源: 2731_SQL PATCH
create table test_proc_patch(a int,b int);

-- 来源: 2731_SQL PATCH
create index test_a on test_proc_patch(a);

-- 来源: 2731_SQL PATCH
create procedure mypro() as num int;

-- 来源: 2733_GUCrewrite_rule
create table t1(c1 int,c2 int);

-- 来源: 2733_GUCrewrite_rule
create table t2(c1 int,c2 int);

-- 来源: 2733_GUCrewrite_rule
create table t (a int, b int, c int, d int);

-- 来源: 2733_GUCrewrite_rule
create table t1 (a int, b int, c int, d int);

-- 来源: 2733_GUCrewrite_rule
create table t1(a int, b varchar, c int, d int);

-- 来源: 2733_GUCrewrite_rule
create table t2(a int, b varchar, c int, d int);

-- 来源: 2743_file_2743
CREATE TABLE int_type_t1 ( IT_COL1 TINYINT , IT_COL2 TINYINT UNSIGNED );

-- 来源: 2743_file_2743
DROP TABLE int_type_t1 ;

-- 来源: 2743_file_2743
CREATE TABLE int_type_t2 ( a TINYINT , b TINYINT , c INTEGER , d INTEGER UNSIGNED , e BIGINT , f BIGINT UNSIGNED );

-- 来源: 2743_file_2743
DROP TABLE int_type_t2 ;

-- 来源: 2743_file_2743
CREATE TABLE decimal_type_t1 ( DT_COL1 DECIMAL(10,4) );

--删除表。
-- 来源: 2743_file_2743
DROP TABLE decimal_type_t1;

-- 来源: 2743_file_2743
CREATE TABLE numeric_type_t1 ( NT_COL1 NUMERIC ( 10 , 4 ) );

-- 来源: 2743_file_2743
DROP TABLE numeric_type_t1 ;

-- 来源: 2743_file_2743
CREATE TABLE smallserial_type_tab ( a SMALLSERIAL );

-- 来源: 2743_file_2743
CREATE TABLE serial_type_tab ( b SERIAL );

-- 来源: 2743_file_2743
CREATE TABLE bigserial_type_tab ( c BIGSERIAL );

-- 来源: 2743_file_2743
CREATE TABLE largeserial_type_tab ( c LARGESERIAL );

-- 来源: 2743_file_2743
DROP TABLE smallserial_type_tab ;

-- 来源: 2743_file_2743
DROP TABLE serial_type_tab ;

-- 来源: 2743_file_2743
DROP TABLE bigserial_type_tab ;

-- 来源: 2743_file_2743
CREATE TABLE float_type_t2 ( FT_COL1 INTEGER , FT_COL2 FLOAT4 , FT_COL3 FLOAT8 , FT_COL4 FLOAT ( 3 ), FT_COL5 BINARY_DOUBLE , FT_COL6 DECIMAL ( 10 , 4 ), FT_COL7 INTEGER ( 6 , 3 ) );

-- 来源: 2743_file_2743
DROP TABLE float_type_t2 ;

-- 来源: 2743_file_2743
CREATE DATABASE gaussdb_m WITH dbcompatibility 'B' ;

-- 来源: 2745_file_2745
CREATE TABLE bool_type_t1 ( BT_COL1 BOOLEAN , BT_COL2 TEXT );

-- 来源: 2745_file_2745
DROP TABLE bool_type_t1 ;

-- 来源: 2746_file_2746
CREATE TABLE varchar_maxlength_test1 (a int, b varchar, c int);

-- 创建表，表中仅varchar一列，根据计算规则，varchar最大存储长度为1GB-85-4=
-- 来源: 2746_file_2746
CREATE TABLE varchar_maxlength_test2 (a varchar);

-- 来源: 2746_file_2746
CREATE TABLE char_type_t1 ( CT_COL1 CHARACTER(4) );

--删除表。
-- 来源: 2746_file_2746
DROP TABLE char_type_t1;

--创建表。
-- 来源: 2746_file_2746
CREATE TABLE char_type_t2 ( CT_COL1 VARCHAR(5) );

--删除数据。
-- 来源: 2746_file_2746
DROP TABLE char_type_t2;

-- 来源: 2746_file_2746
create database gaussdb_m with dbcompatibility 'b';

-- 来源: 2747_file_2747
CREATE TABLE blob_type_t1 ( BT_COL1 INTEGER , BT_COL2 BLOB , BT_COL3 RAW , BT_COL4 BYTEA ) ;

-- 来源: 2747_file_2747
DROP TABLE blob_type_t1 ;

-- 来源: 2747_file_2747
CREATE DATABASE gaussdb_m WITH dbcompatibility 'B' ;

-- 来源: 2748__
CREATE TABLE date_type_tab ( coll date );

-- 来源: 2748__
DROP TABLE date_type_tab ;

-- 来源: 2748__
CREATE TABLE time_type_tab ( da time without time zone , dai time with time zone , dfgh timestamp without time zone , dfga timestamp with time zone , vbg smalldatetime );

-- 来源: 2748__
DROP TABLE time_type_tab ;

-- 来源: 2748__
CREATE TABLE day_type_tab ( a int , b INTERVAL DAY ( 3 ) TO SECOND ( 4 ));

-- 来源: 2748__
DROP TABLE day_type_tab ;

-- 来源: 2748__
CREATE TABLE year_type_tab ( a int , b interval year ( 6 ));

-- 来源: 2748__
DROP TABLE year_type_tab ;

-- 来源: 2748__
create database gaussdb_m dbcompatibility = 'B' ;

-- 来源: 2748__
CREATE TABLE date_type_tab ( coll date );

-- 来源: 2748__
DROP TABLE date_type_tab ;

-- 来源: 2748__
CREATE TABLE realtime_type_special(col1 varchar(20), col2 date, col3 timestamp, col4 time);

--删除表。
-- 来源: 2748__
DROP TABLE realtime_type_special;

-- 来源: 2748__
CREATE TABLE reltime_type_tab ( col1 character ( 30 ), col2 reltime );

-- 来源: 2748__
DROP TABLE reltime_type_tab ;

-- 来源: 2751_file_2751
CREATE TABLE bit_type_t1 ( BT_COL1 INTEGER , BT_COL2 BIT ( 3 ), BT_COL3 BIT VARYING ( 5 ) ) ;

-- 来源: 2751_file_2751
DROP TABLE bit_type_t1 ;

-- 来源: 2755_HLL
CREATE TABLE t1 ( id integer , set hll );

-- 来源: 2755_HLL
CREATE TABLE t2 ( id integer , set hll ( 12 , 4 ));

-- 来源: 2755_HLL
CREATE TABLE t3 ( id int , set hll ( - 1 , - 1 , 8 , - 1 ));

-- 来源: 2755_HLL
CREATE TABLE t4 ( id int , set hll ( 5 , - 1 ));

-- 来源: 2755_HLL
DROP TABLE t1 , t2 , t3 ;

-- 来源: 2755_HLL
CREATE TABLE t1 ( id integer , set hll ( 14 ));

-- 来源: 2755_HLL
DROP TABLE t1 ;

-- 来源: 2755_HLL
CREATE TABLE helloworld ( id integer , set hll );

-- 来源: 2755_HLL
DROP TABLE helloworld ;

-- 来源: 2755_HLL
CREATE TABLE facts ( date date , user_id integer );

-- 来源: 2755_HLL
CREATE TABLE daily_uniques ( date date UNIQUE , users hll );

-- 来源: 2755_HLL
DROP TABLE facts ;

-- 来源: 2755_HLL
DROP TABLE daily_uniques ;

-- 来源: 2755_HLL
CREATE TABLE test ( id integer , set hll );

-- 来源: 2755_HLL
DROP TABLE test ;

-- 来源: 2756_file_2756
CREATE TABLE reservation (room int, during tsrange);

-- 来源: 2756_file_2756
DROP TABLE reservation;

-- 来源: 2756_file_2756
CREATE TYPE floatrange AS RANGE ( subtype = float8, subtype_diff = float8mi );

-- 来源: 2756_file_2756
CREATE FUNCTION time_subtype_diff(x time, y time) RETURNS float8 AS 'SELECT EXTRACT(EPOCH FROM (x - y))' LANGUAGE sql STRICT IMMUTABLE;

-- 来源: 2756_file_2756
CREATE TYPE timerange AS RANGE ( subtype = time, subtype_diff = time_subtype_diff );

-- 来源: 2758_file_2758
create table t1 ( a int );

-- 来源: 2758_file_2758
CREATE OR REPLACE FUNCTION showall () RETURNS SETOF record AS $$ SELECT count ( * ) from t1 ;

-- 来源: 2758_file_2758
DROP FUNCTION showall ();

-- 来源: 2758_file_2758
drop table t1 ;

-- 来源: 2760_XML
CREATE TABLE xmltest ( id int, data xml );

-- 来源: 2760_XML
DROP TABLE xmltest;

-- 来源: 2761_XMLTYPE
CREATE TABLE xmltypetest(id int, data xmltype);

-- 来源: 2763_SET
CREATE TABLE employee ( name text, site SET('beijing','shanghai','nanjing','wuhan') );

-- 来源: 2763_SET
DROP TABLE employee;

-- 来源: 2764_aclitem
CREATE TABLE table_acl (id int,priv aclitem,privs aclitem[]);

-- 来源: 2769_file_2769
CREATE DATABASE gaussdb_m WITH dbcompatibility 'B' ;

-- 来源: 2769_file_2769
CREATE TABLE test_space ( c char ( 10 ));

-- 来源: 2769_file_2769
create table test ( a text );

-- 来源: 2769_file_2769
CREATE EXTENSION pkg_bpchar_opc ;

-- 来源: 2769_file_2769
DROP EXTENSION pkg_bpchar_opc ;

-- 来源: 2769_file_2769
CREATE EXTENSION pkg_bpchar_opc ;

-- 来源: 2769_file_2769
DROP EXTENSION pkg_bpchar_opc ;

-- 来源: 2769_file_2769
CREATE EXTENSION pkg_bpchar_opc ;

-- 来源: 2769_file_2769
DROP EXTENSION pkg_bpchar_opc ;

-- 来源: 2774_file_2774
CREATE USER JIM PASSWORD '*********' ;

-- 来源: 2774_file_2774
CREATE DATABASE testdb3 OWNER JIM DBCOMPATIBILITY = 'B' ;

-- 来源: 2774_file_2774
CREATE USER JIM PASSWORD '*********' ;

-- 来源: 2774_file_2774
CREATE DATABASE testdb3 OWNER JIM DBCOMPATIBILITY = 'B' ;

-- 来源: 2775_file_2775
CREATE DATABASE gaussdb_m WITH dbcompatibility 'b' ;

-- 来源: 2775_file_2775
create table json_doc ( data CLOB );

-- 来源: 2775_file_2775
create or replace procedure p1 is gaussdb $ # type t1 is table of int ;

-- 来源: 2775_file_2775
create type t1 is table of int ;

-- 来源: 2775_file_2775
create or replace package pkg1 is gaussdb $ # type t1 is table of int index by int ;

-- 来源: 2775_file_2775
create or replace package body pkg1 is gaussdb $ # procedure p1 () is gaussdb $ # v1 t1 : = t1 ( 1 => 1 , 2 => 2 , 3 => 3 );

-- 来源: 2779_JSON_JSONB
CREATE TYPE jpop AS (a text, b int, c bool);

-- 来源: 2779_JSON_JSONB
DROP TYPE jpop;

-- 来源: 2779_JSON_JSONB
CREATE TYPE jpop AS (a text, b int, c bool);

-- 来源: 2779_JSON_JSONB
DROP TYPE jpop;

-- 来源: 2779_JSON_JSONB
CREATE TABLE classes(name varchar, score int);

-- 来源: 2779_JSON_JSONB
DROP TABLE classes;

-- 来源: 2779_JSON_JSONB
CREATE TABLE classes(name varchar, score int);

-- 来源: 2779_JSON_JSONB
DROP TABLE classes;

-- 来源: 2780_HLL
CREATE TABLE t_id ( id int );

-- 来源: 2780_HLL
CREATE TABLE t_data ( a int , c text );

-- 来源: 2780_HLL
CREATE TABLE t_a_c_hll ( a int , c hll );

-- 来源: 2781_SEQUENCE
CREATE SEQUENCE seqDemo ;

-- 来源: 2781_SEQUENCE
DROP SEQUENCE seqDemo ;

-- 来源: 2781_SEQUENCE
CREATE SEQUENCE seq1 ;

-- 来源: 2781_SEQUENCE
DROP SEQUENCE seq1 ;

-- 来源: 2781_SEQUENCE
CREATE SEQUENCE seq1 ;

-- 来源: 2781_SEQUENCE
DROP SEQUENCE seq1 ;

-- 来源: 2781_SEQUENCE
CREATE SEQUENCE seqDemo ;

-- 来源: 2781_SEQUENCE
DROP SEQUENCE seqDemo ;

-- 来源: 2781_SEQUENCE
CREATE SEQUENCE seqDemo ;

-- 来源: 2781_SEQUENCE
DROP SEQUENCE seqDemo ;

-- 来源: 2784_file_2784
CREATE TABLE tab ( a int );

-- 来源: 2785_file_2785
CREATE OR REPLACE PROCEDURE proc IS CURSOR cur_1 IS SELECT RATIO_TO_REPORT ( sales_amount ) OVER () FROM sales_numeric ;

-- 来源: 2788_file_2788
CREATE OR REPLACE FUNCTION unnest2 ( anyarray ) RETURNS SETOF anyelement AS $$ SELECT $ 1 [ i ][ j ] FROM generate_subscripts ( $ 1 , 1 ) g1 ( i ), generate_subscripts ( $ 1 , 2 ) g2 ( j );

-- 来源: 2788_file_2788
DROP FUNCTION unnest2 ;

-- 来源: 2789_file_2789
CREATE TABLE blob_tb ( b blob , id int );

-- 来源: 2789_file_2789
DROP TABLE blob_tb ;

-- 来源: 2789_file_2789
CREATE TABLE clob_tb ( c clob , id int );

-- 来源: 2789_file_2789
DROP TABLE clob_tb ;

-- 来源: 2789_file_2789
CREATE TABLE student_demo ( name VARCHAR2 ( 20 ), grade NUMBER ( 10 , 2 ));

-- 来源: 2790_file_2790
CREATE TABLE sales (prod_id NUMBER(6), cust_id NUMBER, time_id DATE, channel_id CHAR(1), promo_id NUMBER(6), quantity_sold NUMBER(3), amount_sold NUMBER(10,2) ) PARTITION BY RANGE( time_id) INTERVAL('1 day') ( partition p1 VALUES LESS THAN ('2019-02-01 00:00:00'), partition p2 VALUES LESS THAN ('2019-02-02 00:00:00') );

-- 来源: 2790_file_2790
create index index_sales on sales(prod_id) local (PARTITION idx_p1 ,PARTITION idx_p2);

-- 来源: 2801_file_2801
CREATE TABLE t1(a int, b int);

-- 来源: 2801_file_2801
CREATE TABLE t2(a int, b int GENERATED ALWAYS AS (a + 1) STORED);

-- 来源: 2804_file_2804
CREATE TABLE test(a int,b int);

-- 来源: 2804_file_2804
CREATE PROCEDURE mypro1() as num int;

-- 来源: 2808_file_2808
CREATE TABLE part_tab 1 ( a int, b int ) PARTITION BY RANGE(b) ( PARTITION P1 VALUES LESS THAN(10), PARTITION P2 VALUES LESS THAN(20), PARTITION P3 VALUES LESS THAN(MAXVALUE) );

-- 来源: 2808_file_2808
CREATE TABLE subpart_tab 1 ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY RANGE (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a VALUES LESS THAN( '2' ), SUBPARTITION p_201901_b VALUES LESS THAN( '3' ) ), PARTITION p_201902 VALUES LESS THAN( '201904' ) ( SUBPARTITION p_201902_a VALUES LESS THAN( '2' ), SUBPARTITION p_201902_b VALUES LESS THAN( '3' ) ) );

-- 来源: 2808_file_2808
CREATE INDEX index_part_tab1 ON part_tab1(b) LOCAL ( PARTITION b_index1, PARTITION b_index2, PARTITION b_index 3 );

-- 来源: 2808_file_2808
CREATE INDEX idx_user_no ON subpart_tab1(user_no) LOCAL;

-- 来源: 2810_HashFunc
CREATE TYPE b1 AS ENUM ( 'good' , 'bad' , 'ugly' );

-- 来源: 2821_XML
CREATE TABLE xmltest ( id int , data xml );

-- 来源: 2821_XML
CREATE SCHEMA testxmlschema ;

-- 来源: 2821_XML
CREATE TABLE testxmlschema . test1 ( a int , b text );

-- 来源: 2821_XML
create database test ;

-- 来源: 2824_file_2824
create extension tablefunc ;

-- 来源: 2824_file_2824
create extension tablefunc ;

-- 来源: 2824_file_2824
create extension tablefunc ;

-- 来源: 2826_SQL
create database db1 ;

-- 来源: 2826_SQL
create database db2 ;

-- 来源: 2826_SQL
create database db1 ;

-- 来源: 2830_file_2830
CREATE TABLE tpcds . case_when_t1 ( CW_COL1 INT );

-- 来源: 2830_file_2830
DROP TABLE tpcds . case_when_t1 ;

-- 来源: 2830_file_2830
CREATE TABLE tpcds . c_tabl ( description varchar ( 10 ), short_description varchar ( 10 ), last_value varchar ( 10 )) ;

-- 来源: 2830_file_2830
DROP TABLE tpcds . c_tabl ;

-- 来源: 2830_file_2830
CREATE TABLE tpcds . null_if_t1 ( NI_VALUE1 VARCHAR ( 10 ), NI_VALUE2 VARCHAR ( 10 ) );

-- 来源: 2830_file_2830
DROP TABLE tpcds . null_if_t1 ;

-- 来源: 2835_file_2835
CREATE TABLE Students ( name varchar ( 20 ), id int ) with ( STORAGE_TYPE = USTORE );

-- 来源: 2835_file_2835
drop table Students ;

-- 来源: 2835_file_2835
create table test ( a int , b int );

-- 来源: 2835_file_2835
drop table test ;

-- 来源: 2840_file_2840
CREATE TABLE tpcds . value_storage_t1 ( VS_COL1 CHARACTER ( 20 ) );

-- 来源: 2840_file_2840
DROP TABLE tpcds . value_storage_t1 ;

-- 来源: 2841_UNIONCASE
CREATE DATABASE a_1 dbcompatibility = 'A' ;

-- 来源: 2841_UNIONCASE
CREATE DATABASE td_1 dbcompatibility = 'C' ;

-- 来源: 2841_UNIONCASE
DROP DATABASE a_1 ;

-- 来源: 2841_UNIONCASE
DROP DATABASE td_1 ;

-- 来源: 2841_UNIONCASE
CREATE DATABASE ora_1 dbcompatibility = 'A';

--删除ORA模式的数据库。
-- 来源: 2841_UNIONCASE
DROP DATABASE ora_1;

-- 来源: 2849_file_2849
DROP SCHEMA IF EXISTS tsearch CASCADE ;

-- 来源: 2849_file_2849
CREATE SCHEMA tsearch ;

-- 来源: 2849_file_2849
CREATE TABLE tsearch . pgweb ( id int , body text , title text , last_mod_date date ) with ( storage_type = ASTORE );

-- 来源: 2850_file_2850
CREATE INDEX pgweb_idx_1 ON tsearch . pgweb USING gin ( to_tsvector ( 'english' , body ));

-- 来源: 2850_file_2850
CREATE INDEX pgweb_idx_2 ON tsearch . pgweb USING gin ( to_tsvector ( 'ngram' , body ));

-- 来源: 2850_file_2850
CREATE INDEX pgweb_idx_3 ON tsearch . pgweb USING gin ( to_tsvector ( 'english' , title || ' ' || body ));

-- 来源: 2850_file_2850
ALTER TABLE tsearch . pgweb ADD COLUMN textsearchable_index_col tsvector ;

-- 来源: 2850_file_2850
CREATE INDEX textsearch_idx_4 ON tsearch . pgweb USING gin ( textsearchable_index_col );

-- 来源: 2851_file_2851
create table table1 ( c_int int , c_bigint bigint , c_varchar varchar , c_text text ) with ( orientation = row , storage_type = ASTORE );

-- 来源: 2851_file_2851
create text search configuration ts_conf_1 ( parser = POUND );

-- 来源: 2851_file_2851
create text search configuration ts_conf_2 ( parser = POUND ) with ( split_flag = '%' );

-- 来源: 2851_file_2851
create index idx1 on table1 using gin ( to_tsvector ( c_text ));

-- 来源: 2851_file_2851
create index idx2 on table1 using gin ( to_tsvector ( c_text ));

-- 来源: 2853_file_2853
CREATE TABLE tsearch . tt ( id int , title text , keyword text , abstract text , body text , ti tsvector );

-- 来源: 2853_file_2853
DROP TABLE tsearch . tt ;

-- 来源: 2855_file_2855
CREATE TABLE tsearch . ts_ngram ( id int , body text );

-- 来源: 2860_file_2860
CREATE TABLE tsearch . aliases ( id int , t tsquery , s tsquery );

-- 来源: 2860_file_2860
DROP TABLE tsearch . aliases ;

-- 来源: 2864_file_2864
ALTER TEXT SEARCH CONFIGURATION astro_en ADD MAPPING FOR asciiword WITH astro_syn , english_ispell , english_stem ;

-- 来源: 2866_Simple
CREATE TEXT SEARCH DICTIONARY public . simple_dict ( TEMPLATE = pg_catalog . simple , STOPWORDS = english );

-- 来源: 2866_Simple
ALTER TEXT SEARCH DICTIONARY public . simple_dict ( Accept = false );

-- 来源: 2867_Synonym
CREATE TEXT SEARCH DICTIONARY my_synonym ( TEMPLATE = synonym , SYNONYMS = my_synonyms , FILEPATH = 'file:///home/dicts/' );

-- 来源: 2867_Synonym
ALTER TEXT SEARCH CONFIGURATION english ALTER MAPPING FOR asciiword WITH my_synonym , english_stem ;

-- 来源: 2867_Synonym
ALTER TEXT SEARCH DICTIONARY my_synonym ( CASESENSITIVE = true );

-- 来源: 2867_Synonym
CREATE TEXT SEARCH DICTIONARY syn ( TEMPLATE = synonym , SYNONYMS = synonym_sample );

-- 来源: 2867_Synonym
CREATE TEXT SEARCH CONFIGURATION tst ( copy = simple );

-- 来源: 2867_Synonym
ALTER TEXT SEARCH CONFIGURATION tst ALTER MAPPING FOR asciiword WITH syn ;

-- 来源: 2868_Thesaurus
CREATE TEXT SEARCH DICTIONARY thesaurus_astro ( TEMPLATE = thesaurus , DictFile = thesaurus_astro , Dictionary = pg_catalog . english_stem , FILEPATH = 'file:///home/dicts/' );

-- 来源: 2868_Thesaurus
ALTER TEXT SEARCH CONFIGURATION russian ALTER MAPPING FOR asciiword , asciihword , hword_asciipart WITH thesaurus_astro , english_stem ;

-- 来源: 2868_Thesaurus
ALTER TEXT SEARCH DICTIONARY thesaurus_astro ( DictFile = thesaurus_astro , FILEPATH = 'file:///home/dicts/' );

-- 来源: 2869_Ispell
CREATE TEXT SEARCH DICTIONARY norwegian_ispell ( TEMPLATE = ispell , DictFile = nn_no , AffFile = nn_no , FilePath = 'file:///home/dicts' );

-- 来源: 2871_file_2871
CREATE TEXT SEARCH CONFIGURATION ts_conf ( COPY = pg_catalog . english );

-- 来源: 2871_file_2871
CREATE TEXT SEARCH DICTIONARY gs_dict ( TEMPLATE = synonym , SYNONYMS = gs_dict , FILEPATH = 'file:///home/dicts' );

-- 来源: 2871_file_2871
CREATE TEXT SEARCH DICTIONARY english_ispell ( TEMPLATE = ispell , DictFile = english , AffFile = english , StopWords = english , FILEPATH = 'file:///home/dicts' );

-- 来源: 2871_file_2871
ALTER TEXT SEARCH CONFIGURATION ts_conf ALTER MAPPING FOR asciiword , asciihword , hword_asciipart , word , hword , hword_part WITH gs_dict , english_ispell , english_stem ;

-- 来源: 2871_file_2871
ALTER TEXT SEARCH CONFIGURATION ts_conf DROP MAPPING FOR email , url , url_path , sfloat , float ;

-- 来源: 2882_ABORT
CREATE TABLE customer_demographics_t1 ( CD_DEMO_SK INTEGER NOT NULL, CD_GENDER CHAR(1) , CD_MARITAL_STATUS CHAR(1) , CD_EDUCATION_STATUS CHAR(20) , CD_PURCHASE_ESTIMATE INTEGER , CD_CREDIT_RATING CHAR(10) , CD_DEP_COUNT INTEGER , CD_DEP_EMPLOYED_COUNT INTEGER , CD_DEP_COLLEGE_COUNT INTEGER ) ;

--删除表。
-- 来源: 2882_ABORT
DROP TABLE customer_demographics_t1;

-- 来源: 2883_ALTER AGGREGATE
CREATE OR REPLACE FUNCTION int_add(int,int) returns int as $BODY$ declare begin return $1 + $2;

--创建用户joe。
-- 来源: 2883_ALTER AGGREGATE
CREATE USER joe PASSWORD ' ******** ';

--创建SCHEMA。
-- 来源: 2883_ALTER AGGREGATE
CREATE SCHEMA myschema;

--删除SCHEMA,用户及相关函数。
-- 来源: 2883_ALTER AGGREGATE
DROP SCHEMA myschema CASCADE;

-- 来源: 2883_ALTER AGGREGATE
DROP USER joe;

-- 来源: 2883_ALTER AGGREGATE
DROP FUNCTION int_add(int,int);

-- 来源: 2885_ALTER DATABASE
CREATE DATABASE testdb;

--将testdb重命名为test_db1。
-- 来源: 2885_ALTER DATABASE
ALTER DATABASE testdb RENAME TO test_db1;

-- 来源: 2885_ALTER DATABASE
ALTER DATABASE test_db1 WITH CONNECTION LIMIT 100;

-- 来源: 2885_ALTER DATABASE
CREATE USER scott PASSWORD '********';

--将test_db1的所有者修改为jim。
-- 来源: 2885_ALTER DATABASE
ALTER DATABASE test_db1 OWNER TO scott;

-- 来源: 2885_ALTER DATABASE
CREATE TABLESPACE tbs_data1 RELATIVE LOCATION 'tablespace1/tbs_data1';

--修改test_db1默认表空间。
-- 来源: 2885_ALTER DATABASE
ALTER DATABASE test_db1 SET TABLESPACE tbs_data1;

-- 来源: 2885_ALTER DATABASE
CREATE USER jack PASSWORD '********';

-- 来源: 2885_ALTER DATABASE
CREATE TABLE test_tbl1(c1 int,c2 int);

-- 来源: 2885_ALTER DATABASE
ALTER DATABASE test_db1 ENABLE PRIVATE OBJECT;

-- 来源: 2885_ALTER DATABASE
DROP TABLE public.test_tbl1;

-- 来源: 2885_ALTER DATABASE
DROP DATABASE test_db1;

-- 来源: 2885_ALTER DATABASE
DROP TABLESPACE tbs_data1;

-- 来源: 2885_ALTER DATABASE
DROP USER jack;

-- 来源: 2885_ALTER DATABASE
DROP USER scott;

-- 来源: 2886_ALTER DATABASE LINK
CREATE USER user01 WITH SYSADMIN PASSWORD '********';

--创建普通用户
-- 来源: 2886_ALTER DATABASE LINK
CREATE USER user2 PASSWORD '********';

-- 来源: 2886_ALTER DATABASE LINK
DROP USER user01 CASCADE;

-- 来源: 2886_ALTER DATABASE LINK
DROP USER user02;

-- 来源: 2887_ALTER DATA SOURCE
CREATE DATA SOURCE ds_test1;

--创建用户和修改所有者。
-- 来源: 2887_ALTER DATA SOURCE
CREATE USER user_test1 IDENTIFIED BY '********';

-- 来源: 2887_ALTER DATA SOURCE
ALTER USER user_test1 WITH SYSADMIN;

--删除Data Source和user对象。
-- 来源: 2887_ALTER DATA SOURCE
DROP DATA SOURCE ds_test;

-- 来源: 2887_ALTER DATA SOURCE
DROP USER user_test1;

-- 来源: 2888_ALTER DEFAULT PRIVILEGES
CREATE SCHEMA tpcds;

--将创建在模式tpcds里的所有表（和视图）的SELECT权限授予每一个用户。
-- 来源: 2888_ALTER DEFAULT PRIVILEGES
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds GRANT SELECT ON TABLES TO PUBLIC;

--创建用户普通用户jack。
-- 来源: 2888_ALTER DEFAULT PRIVILEGES
CREATE USER jack PASSWORD ' ******** ';

--将tpcds下的所有表的插入权限授予用户jack。
-- 来源: 2888_ALTER DEFAULT PRIVILEGES
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds GRANT INSERT ON TABLES TO jack;

-- 来源: 2888_ALTER DEFAULT PRIVILEGES
ALTER DEFAULT PRIVILEGES FOR ROLE jack IN SCHEMA tpcds GRANT INSERT ON TABLES TO jack;

--撤销上述权限。
-- 来源: 2888_ALTER DEFAULT PRIVILEGES
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds REVOKE SELECT ON TABLES FROM PUBLIC;

-- 来源: 2888_ALTER DEFAULT PRIVILEGES
ALTER DEFAULT PRIVILEGES IN SCHEMA tpcds REVOKE INSERT ON TABLES FROM jack;

--删除用户jack。
-- 来源: 2888_ALTER DEFAULT PRIVILEGES
DROP USER jack CASCADE;

--删除SCHEMA。
-- 来源: 2888_ALTER DEFAULT PRIVILEGES
DROP SCHEMA tpcds;

-- 来源: 2889_ALTER DIRECTORY
CREATE OR REPLACE DIRECTORY dir as '/tmp/';

--创建用户
-- 来源: 2889_ALTER DIRECTORY
CREATE USER jim PASSWORD '********';

--修改目录的owner。
-- 来源: 2889_ALTER DIRECTORY
ALTER DIRECTORY dir OWNER TO jim;

--删除目录。
-- 来源: 2889_ALTER DIRECTORY
DROP DIRECTORY dir;

-- 来源: 2892_ALTER FOREIGN TABLE
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

--创建外表
-- 来源: 2892_ALTER FOREIGN TABLE
CREATE FOREIGN TABLE foreign_tbl (col1 text) SERVER my_server OPTIONS (logtype 'pg_log');

--修改外表属性
-- 来源: 2892_ALTER FOREIGN TABLE
ALTER FOREIGN TABLE foreign_tbl OPTIONS (ADD latest_files '2');

-- 来源: 2892_ALTER FOREIGN TABLE
ALTER FOREIGN TABLE foreign_tbl OPTIONS ( SET latest_files '5');

-- 来源: 2892_ALTER FOREIGN TABLE
ALTER FOREIGN TABLE foreign_tbl OPTIONS ( DROP latest_files);

-- 来源: 2892_ALTER FOREIGN TABLE
DROP FOREIGN TABLE foreign_tbl;

-- 来源: 2892_ALTER FOREIGN TABLE
DROP SERVER my_server;

-- 创建函数
-- 来源: 2893_ALTER FUNCTION
CREATE OR REPLACE FUNCTION test_func(a int) RETURN int IS proc_var int;

-- 删除函数
-- 来源: 2893_ALTER FUNCTION
DROP FUNCTION test_func;

-- 来源: 2894_ALTER GLOBAL CONFIGURATION
ALTER GLOBAL CONFIGURATION with ( redis_is_ok = true );

-- 来源: 2894_ALTER GLOBAL CONFIGURATION
ALTER GLOBAL CONFIGURATION with ( redis_is_ok = false );

-- 来源: 2894_ALTER GLOBAL CONFIGURATION
DROP GLOBAL CONFIGURATION redis_is_ok ;

-- 来源: 2896_ALTER INDEX
CREATE TABLE test1(col1 int, col2 int);

-- 来源: 2896_ALTER INDEX
CREATE INDEX aa ON test1(col1);

--将索引aa重命名为idx_test1_col1。
-- 来源: 2896_ALTER INDEX
ALTER INDEX aa RENAME TO idx_test1_col1;

-- 来源: 2896_ALTER INDEX
CREATE TABLESPACE tbs_index1 RELATIVE LOCATION 'tablespace1/tbs_index1';

--修改索引idx_test1_col1的所属表空间为tbs_index1。
-- 来源: 2896_ALTER INDEX
ALTER INDEX IF EXISTS idx_test1_col1 SET TABLESPACE tbs_index1;

--修改索引idx_test1_col1 的填充因子。
-- 来源: 2896_ALTER INDEX
ALTER INDEX IF EXISTS idx_test1_col1 SET (FILLFACTOR = 70);

-- 来源: 2896_ALTER INDEX
ALTER INDEX IF EXISTS idx_test1_col1 RESET (FILLFACTOR);

-- 来源: 2896_ALTER INDEX
ALTER INDEX IF EXISTS idx_test1_col1 UNUSABLE;

--重建索引idx_test1_col1。
-- 来源: 2896_ALTER INDEX
ALTER INDEX idx_test1_col1 REBUILD;

--删除。
-- 来源: 2896_ALTER INDEX
DROP INDEX idx_test1_col1;

-- 来源: 2896_ALTER INDEX
DROP TABLE test1;

-- 来源: 2896_ALTER INDEX
DROP TABLESPACE tbs_index1;

-- 来源: 2896_ALTER INDEX
CREATE TABLE test2(col1 int, col2 int) PARTITION BY RANGE (col1)( PARTITION p1 VALUES LESS THAN (100), PARTITION p2 VALUES LESS THAN (200) );

--创建分区索引。
-- 来源: 2896_ALTER INDEX
CREATE INDEX idx_test2_col1 ON test2(col1) LOCAL( PARTITION p1, PARTITION p2 );

--重命名索引分区。
-- 来源: 2896_ALTER INDEX
ALTER INDEX idx_test2_col1 RENAME PARTITION p1 TO p1_test2_idx;

-- 来源: 2896_ALTER INDEX
ALTER INDEX idx_test2_col1 RENAME PARTITION p2 TO p2_test2_idx;

-- 来源: 2896_ALTER INDEX
CREATE TABLESPACE tbs_index2 RELATIVE LOCATION 'tablespace1/tbs_index2';

-- 来源: 2896_ALTER INDEX
CREATE TABLESPACE tbs_index3 RELATIVE LOCATION 'tablespace1/tbs_index3';

--修改索引idx_test2_col1分区的所属表空间。
-- 来源: 2896_ALTER INDEX
ALTER INDEX idx_test2_col1 MOVE PARTITION p1_test2_idx TABLESPACE tbs_index2;

-- 来源: 2896_ALTER INDEX
ALTER INDEX idx_test2_col1 MOVE PARTITION p2_test2_idx TABLESPACE tbs_index3;

--删除。
-- 来源: 2896_ALTER INDEX
DROP INDEX idx_test2_col1;

-- 来源: 2896_ALTER INDEX
DROP TABLE test2;

-- 来源: 2896_ALTER INDEX
DROP TABLESPACE tbs_index2;

-- 来源: 2896_ALTER INDEX
DROP TABLESPACE tbs_index3;

-- 来源: 2899_ALTER MASKING POLICY
CREATE USER dev_mask PASSWORD '********' ;

-- 来源: 2899_ALTER MASKING POLICY
CREATE USER bob_mask PASSWORD '********' ;

-- 来源: 2899_ALTER MASKING POLICY
CREATE TABLE tb_for_masking ( col1 text , col2 text , col3 text );

-- 来源: 2899_ALTER MASKING POLICY
CREATE RESOURCE LABEL mask_lb1 ADD COLUMN ( tb_for_masking . col1 );

-- 来源: 2899_ALTER MASKING POLICY
CREATE RESOURCE LABEL mask_lb2 ADD COLUMN ( tb_for_masking . col2 );

-- 来源: 2899_ALTER MASKING POLICY
CREATE MASKING POLICY maskpol1 maskall ON LABEL ( mask_lb1 );

-- 来源: 2899_ALTER MASKING POLICY
ALTER MASKING POLICY maskpol1 COMMENTS 'masking policy for tb_for_masking.col1' ;

-- 来源: 2899_ALTER MASKING POLICY
ALTER MASKING POLICY maskpol1 ADD randommasking ON LABEL ( mask_lb2 );

-- 来源: 2899_ALTER MASKING POLICY
ALTER MASKING POLICY maskpol1 REMOVE randommasking ON LABEL ( mask_lb2 );

-- 来源: 2899_ALTER MASKING POLICY
ALTER MASKING POLICY maskpol1 MODIFY randommasking ON LABEL ( mask_lb1 );

-- 来源: 2899_ALTER MASKING POLICY
ALTER MASKING POLICY maskpol1 MODIFY ( FILTER ON ROLES ( dev_mask , bob_mask ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' ));

-- 来源: 2899_ALTER MASKING POLICY
ALTER MASKING POLICY maskpol1 DROP FILTER ;

-- 来源: 2899_ALTER MASKING POLICY
ALTER MASKING POLICY maskpol1 DISABLE ;

-- 来源: 2899_ALTER MASKING POLICY
DROP MASKING POLICY maskpol1 ;

-- 来源: 2899_ALTER MASKING POLICY
DROP RESOURCE LABEL mask_lb1 , mask_lb2 ;

-- 来源: 2899_ALTER MASKING POLICY
DROP TABLE tb_for_masking ;

-- 来源: 2899_ALTER MASKING POLICY
DROP USER dev_mask , bob_mask ;

-- 来源: 2900_ALTER MATERIALIZED VIEW
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建全量物化视图。
-- 来源: 2900_ALTER MATERIALIZED VIEW
CREATE MATERIALIZED VIEW foo AS SELECT * FROM my_table;

--把物化视图foo重命名为bar。
-- 来源: 2900_ALTER MATERIALIZED VIEW
ALTER MATERIALIZED VIEW foo RENAME TO bar;

--删除全量物化视图。
-- 来源: 2900_ALTER MATERIALIZED VIEW
DROP MATERIALIZED VIEW bar;

--删除表my_table。
-- 来源: 2900_ALTER MATERIALIZED VIEW
DROP TABLE my_table;

-- 来源: 2903_ALTER PACKAGE
CREATE DATABASE ora_compatible_db DBCOMPATIBILITY 'ORA';

-- 创建包
-- 来源: 2903_ALTER PACKAGE
CREATE OR REPLACE PACKAGE test_pkg AS pkg_var int := 1;

-- 来源: 2903_ALTER PACKAGE
CREATE OR REPLACE PACKAGE body test_pkg AS procedure test_pkg_proc(var int) IS BEGIN pkg_var := 1;

-- 重编译包
-- 来源: 2903_ALTER PACKAGE
ALTER PACKAGE test_pkg COMPILE;

--删除包
-- 来源: 2903_ALTER PACKAGE
DROP PACKAGE test_pkg;

-- 创建存储过程
-- 来源: 2904_ALTER PROCEDURE
CREATE OR REPLACE PROCEDURE test_proc(a int) IS proc_var int;

-- 删除存储过程
-- 来源: 2904_ALTER PROCEDURE
DROP PROCEDURE test_proc;

-- 来源: 2905_ALTER RESOURCE LABEL
CREATE TABLE table_for_label ( col1 int , col2 text );

-- 来源: 2905_ALTER RESOURCE LABEL
CREATE RESOURCE LABEL table_label ADD COLUMN ( table_for_label . col1 );

-- 来源: 2905_ALTER RESOURCE LABEL
ALTER RESOURCE LABEL table_label ADD COLUMN ( table_for_label . col2 );

-- 来源: 2905_ALTER RESOURCE LABEL
ALTER RESOURCE LABEL table_label REMOVE COLUMN ( table_for_label . col1 );

-- 来源: 2905_ALTER RESOURCE LABEL
DROP RESOURCE LABEL table_label ;

-- 来源: 2905_ALTER RESOURCE LABEL
DROP TABLE table_for_label ;

-- 来源: 2906_ALTER RESOURCE POOL
CREATE RESOURCE POOL pool1 ;

-- 来源: 2906_ALTER RESOURCE POOL
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "High" );

-- 来源: 2906_ALTER RESOURCE POOL
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "class1:Low" );

-- 来源: 2906_ALTER RESOURCE POOL
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "class1:wg1" );

-- 来源: 2906_ALTER RESOURCE POOL
ALTER RESOURCE POOL pool1 WITH ( CONTROL_GROUP = "class1:wg2:3" );

-- 来源: 2906_ALTER RESOURCE POOL
DROP RESOURCE POOL pool1 ;

-- 来源: 2908_ALTER ROW LEVEL SECURITY POLICY
CREATE TABLE all_data(id int, role varchar(100), data varchar(100));

--创建用户alice, bob。
-- 来源: 2908_ALTER ROW LEVEL SECURITY POLICY
CREATE ROLE alice WITH PASSWORD "********";

-- 来源: 2908_ALTER ROW LEVEL SECURITY POLICY
CREATE ROLE bob WITH PASSWORD "********";

--删除用户alice, bob。
-- 来源: 2908_ALTER ROW LEVEL SECURITY POLICY
DROP ROLE alice, bob;

--删除数据表all_data。
-- 来源: 2908_ALTER ROW LEVEL SECURITY POLICY
DROP TABLE all_data;

-- 来源: 2909_ALTER SCHEMA
CREATE DATABASE test1 WITH DBCOMPATIBILITY = 'b';

--创建模式ds。
-- 来源: 2909_ALTER SCHEMA
CREATE SCHEMA ds;

--将当前模式ds更名为ds_new。
-- 来源: 2909_ALTER SCHEMA
ALTER SCHEMA ds RENAME TO ds_new;

--创建用户jack。
-- 来源: 2909_ALTER SCHEMA
CREATE USER jack PASSWORD ' ******** ';

--将DS_NEW的所有者修改为jack。
-- 来源: 2909_ALTER SCHEMA
ALTER SCHEMA ds_new OWNER TO jack;

--将sch的默认字符集修改为utf8mb4，默认字符序修改为utf8mb4_bin。仅在B模式下（即sql_compatibility='B'）支持该语法。
-- 来源: 2909_ALTER SCHEMA
CREATE SCHEMA sch;

-- 来源: 2909_ALTER SCHEMA
ALTER SCHEMA sch CHARACTER SET utf8mb4 COLLATE utf8mb4_bin;

--删除模式ds_new和sch。
-- 来源: 2909_ALTER SCHEMA
DROP SCHEMA ds_new;

-- 来源: 2909_ALTER SCHEMA
DROP SCHEMA sch;

--删除用户jack。
-- 来源: 2909_ALTER SCHEMA
DROP USER jack;

-- 来源: 2909_ALTER SCHEMA
DROP DATABASE test1;

-- 来源: 2910_ALTER SEQUENCE
CREATE SEQUENCE serial START 101;

--创建一个表,定义默认值。
-- 来源: 2910_ALTER SEQUENCE
CREATE TABLE t1(c1 bigint default nextval('serial'));

--将序列serial的归属列变为T1.C1。
-- 来源: 2910_ALTER SEQUENCE
ALTER SEQUENCE serial OWNED BY t1.c1;

--删除序列和表。
-- 来源: 2910_ALTER SEQUENCE
DROP SEQUENCE serial CASCADE;

-- 来源: 2910_ALTER SEQUENCE
DROP TABLE t1;

-- 来源: 2911_ALTER SERVER
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

--修改外部服务的名称。
-- 来源: 2911_ALTER SERVER
ALTER SERVER my_server RENAME TO my_server_1;

--删除my_server_1。
-- 来源: 2911_ALTER SERVER
DROP SERVER my_server_1;

-- 来源: 2912_ALTER SESSION
CREATE SCHEMA ds;

--设置当前会话的字符编码为UTF8。
-- 来源: 2912_ALTER SESSION
ALTER SESSION SET NAMES 'UTF8';

--设置当前模式。
-- 来源: 2912_ALTER SESSION
ALTER SESSION SET CURRENT_SCHEMA TO tpcds;

--设置XML OPTION为DOCUMENT。
-- 来源: 2912_ALTER SESSION
ALTER SESSION SET XML OPTION DOCUMENT;

--创建角色joe，并设置会话的角色为joe。
-- 来源: 2912_ALTER SESSION
CREATE ROLE joe WITH PASSWORD ' ******** ';

-- 来源: 2912_ALTER SESSION
ALTER SESSION SET SESSION AUTHORIZATION joe PASSWORD ' ******** ';

--删除ds模式。
-- 来源: 2912_ALTER SESSION
DROP SCHEMA ds;

--删除joe。
-- 来源: 2912_ALTER SESSION
DROP ROLE joe;

-- 来源: 2912_ALTER SESSION
ALTER SESSION SET TRANSACTION READ ONLY;

-- 来源: 2914_ALTER SYNONYM
CREATE USER sysadmin WITH SYSADMIN PASSWORD '********';

--创建同义词t1。
-- 来源: 2914_ALTER SYNONYM
CREATE OR REPLACE SYNONYM t1 FOR ot.t1;

--创建新用户u1。
-- 来源: 2914_ALTER SYNONYM
CREATE USER u1 PASSWORD '********';

--修改同义词t1的owner为u1。
-- 来源: 2914_ALTER SYNONYM
ALTER SYNONYM t1 OWNER TO u1;

--删除同义词t1。
-- 来源: 2914_ALTER SYNONYM
DROP SYNONYM t1;

--删除用户u1。
-- 来源: 2914_ALTER SYNONYM
DROP USER u1;

--删除用户sysadmin。
-- 来源: 2914_ALTER SYNONYM
DROP USER sysadmin;

--结束SID为140131075880720的会话。
-- 来源: 2915_ALTER SYSTEM KILL SESSION
ALTER SYSTEM KILL SESSION '140131075880720,0' IMMEDIATE;

-- 来源: 2916_ALTER TABLE
CREATE TABLE aa(c1 int, c2 int);

-- 来源: 2916_ALTER TABLE
ALTER TABLE IF EXISTS aa RENAME TO test_alt1;

-- 来源: 2916_ALTER TABLE
CREATE SCHEMA test_schema;

--把表test_alt1的所属模式修改为test_schema。
-- 来源: 2916_ALTER TABLE
ALTER TABLE test_alt1 SET SCHEMA test_schema;

-- 来源: 2916_ALTER TABLE
CREATE USER test_user PASSWORD 'XXXXXXXXXX';

-- 修改test_alt1表的所有者为test_user;
-- 来源: 2916_ALTER TABLE
ALTER TABLE IF EXISTS test_schema.test_alt1 OWNER TO test_user;

-- 来源: 2916_ALTER TABLE
CREATE TABLESPACE tbs_data1 RELATIVE LOCATION 'tablespace1/tbs_data1';

--修改test_alt1表的空间为tbs_data1。
-- 来源: 2916_ALTER TABLE
ALTER TABLE test_schema.test_alt1 SET TABLESPACE tbs_data1;

--删除。
-- 来源: 2916_ALTER TABLE
DROP TABLE test_schema.test_alt1;

-- 来源: 2916_ALTER TABLE
DROP TABLESPACE tbs_data1;

-- 来源: 2916_ALTER TABLE
DROP SCHEMA test_schema;

-- 来源: 2916_ALTER TABLE
DROP USER test_user;

-- 来源: 2916_ALTER TABLE
CREATE TABLE test_alt2(c1 INT,c2 INT);

-- 修改列名
-- 来源: 2916_ALTER TABLE
ALTER TABLE test_alt2 RENAME c1 TO id;

-- 来源: 2916_ALTER TABLE
ALTER TABLE test_alt2 RENAME COLUMN c2 to areaid;

-- 来源: 2916_ALTER TABLE
ALTER TABLE IF EXISTS test_alt2 ADD COLUMN name VARCHAR(20);

-- 来源: 2916_ALTER TABLE
ALTER TABLE test_alt2 MODIFY name VARCHAR(50);

-- 来源: 2916_ALTER TABLE
ALTER TABLE test_alt2 ALTER COLUMN name TYPE VARCHAR(25);

-- 来源: 2916_ALTER TABLE
ALTER TABLE test_alt2 DROP COLUMN areaid;

--修改test_alt2表中name字段的存储模式。
-- 来源: 2916_ALTER TABLE
ALTER TABLE test_alt2 ALTER COLUMN name SET STORAGE PLAIN;

--删除。
-- 来源: 2916_ALTER TABLE
DROP TABLE test_alt2;

-- 来源: 2916_ALTER TABLE
CREATE DATABASE test DBCOMPATIBILITY 'B';

-- 来源: 2916_ALTER TABLE
CREATE TABLE tbl_test(id int, name varchar(20));

--修改tbl_test表中字段name类型，并指定位置到最前面。
-- 来源: 2916_ALTER TABLE
ALTER TABLE tbl_test MODIFY COLUMN name varchar(25) FIRST;

--修改tbl_test字段name的类型，并指定位置在id字段的后面。
-- 来源: 2916_ALTER TABLE
ALTER TABLE tbl_test MODIFY COLUMN name varchar(10) AFTER id;

--删除表tbl_test。
-- 来源: 2916_ALTER TABLE
DROP TABLE tbl_test;

-- 来源: 2916_ALTER TABLE
DROP DATABASE test;

-- 来源: 2916_ALTER TABLE
CREATE TABLE test_alt3(pid INT, areaid CHAR(5), name VARCHAR(20));

--为pid添加非空约束。
-- 来源: 2916_ALTER TABLE
ALTER TABLE test_alt3 MODIFY pid NOT NULL;

-- 来源: 2916_ALTER TABLE
ALTER TABLE test_alt3 MODIFY pid NULL;

-- 来源: 2916_ALTER TABLE
ALTER TABLE test_alt3 ALTER COLUMN areaid SET DEFAULT '00000';

-- 来源: 2916_ALTER TABLE
ALTER TABLE test_alt3 ALTER COLUMN areaid DROP DEFAULT;

-- 来源: 2916_ALTER TABLE
ALTER TABLE test_alt3 ADD CONSTRAINT pk_test3_pid PRIMARY KEY (pid);

-- 来源: 2916_ALTER TABLE
CREATE TABLE test_alt4(c1 INT, c2 INT);

--建索引。
-- 来源: 2916_ALTER TABLE
CREATE UNIQUE INDEX pk_test4_c1 ON test_alt4(c1);

--添加约束时关联已经创建的索引。
-- 来源: 2916_ALTER TABLE
ALTER TABLE test_alt4 ADD CONSTRAINT pk_test4_c1 PRIMARY KEY USING INDEX pk_test4_c1;

--删除。
-- 来源: 2916_ALTER TABLE
DROP TABLE test_alt4;

-- 来源: 2916_ALTER TABLE
ALTER TABLE test_alt3 DROP CONSTRAINT IF EXISTS pk_test3_pid;

--删除。
-- 来源: 2916_ALTER TABLE
DROP TABLE test_alt3;

-- 来源: 2920_ALTER TEXT SEARCH CONFIGURATION
CREATE TEXT SEARCH CONFIGURATION english_1 (parser=default);

--增加文本搜索配置字串类型映射语法。
-- 来源: 2920_ALTER TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION english_1 ADD MAPPING FOR word WITH simple,english_stem;

--增加文本搜索配置字串类型映射语法。
-- 来源: 2920_ALTER TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION english_1 ADD MAPPING FOR email WITH english_stem, french_stem;

--修改文本搜索配置字串类型映射语法。
-- 来源: 2920_ALTER TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION english_1 ALTER MAPPING REPLACE french_stem with german_stem;

--删除文本搜索配置。
-- 来源: 2920_ALTER TEXT SEARCH CONFIGURATION
DROP TEXT SEARCH CONFIGURATION english_1;

-- 来源: 2921_ALTER TEXT SEARCH DICTIONARY
CREATE TEXT SEARCH DICTIONARY my_dict ( TEMPLATE = Simple );

--更改Simple类型字典，将非停用词设置为已识别，其他参数保持不变。
-- 来源: 2921_ALTER TEXT SEARCH DICTIONARY
ALTER TEXT SEARCH DICTIONARY my_dict ( Accept = true );

--更改Simple类型字典，重置Accept参数。
-- 来源: 2921_ALTER TEXT SEARCH DICTIONARY
ALTER TEXT SEARCH DICTIONARY my_dict ( Accept );

--更新词典定义，不实际更改任何内容。
-- 来源: 2921_ALTER TEXT SEARCH DICTIONARY
ALTER TEXT SEARCH DICTIONARY my_dict ( dummy );

--删除字典my_dict。
-- 来源: 2921_ALTER TEXT SEARCH DICTIONARY
DROP TEXT SEARCH DICTIONARY my_dict;

-- 来源: 2926_ALTER VIEW
CREATE TABLE test_tb1(col1 INT,col2 INT);

--创建视图。
-- 来源: 2926_ALTER VIEW
CREATE VIEW abc AS SELECT * FROM test_tb1;

--重命名视图。
-- 来源: 2926_ALTER VIEW
ALTER VIEW IF EXISTS abc RENAME TO test_v1;

-- 来源: 2926_ALTER VIEW
CREATE ROLE role_test PASSWORD '********';

--修改视图所有者。
-- 来源: 2926_ALTER VIEW
ALTER VIEW IF EXISTS test_v1 OWNER TO role_test;

-- 来源: 2926_ALTER VIEW
CREATE SCHEMA tcpds;

--修改视图所属模式。
-- 来源: 2926_ALTER VIEW
ALTER VIEW test_v1 SET SCHEMA tcpds;

-- 来源: 2926_ALTER VIEW
ALTER VIEW tcpds.test_v1 SET (security_barrier = TRUE);

--重置视图选项。
-- 来源: 2926_ALTER VIEW
ALTER VIEW tcpds.test_v1 RESET (security_barrier);

--删除视图test_v1。
-- 来源: 2926_ALTER VIEW
DROP VIEW tcpds.test_v1;

--删除表test_tb1。
-- 来源: 2926_ALTER VIEW
DROP TABLE test_tb1;

--删除用户。
-- 来源: 2926_ALTER VIEW
DROP ROLE role_test;

--删除schema。
-- 来源: 2926_ALTER VIEW
DROP SCHEMA tcpds;

-- 来源: 2927_ANALYZE _ ANALYSE
CREATE TABLE customer_info ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL, WR_REFUNDED_CUSTOMER_SK INTEGER );

-- 来源: 2927_ANALYZE _ ANALYSE
CREATE TABLE customer_par ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL, WR_REFUNDED_CUSTOMER_SK INTEGER ) PARTITION BY RANGE(WR_RETURNED_DATE_SK) ( PARTITION P1 VALUES LESS THAN(2452275), PARTITION P2 VALUES LESS THAN(2452640), PARTITION P3 VALUES LESS THAN(2453000), PARTITION P4 VALUES LESS THAN(MAXVALUE) ) ENABLE ROW MOVEMENT;

-- 来源: 2927_ANALYZE _ ANALYSE
DROP TABLE customer_info;

-- 来源: 2927_ANALYZE _ ANALYSE
DROP TABLE customer_par;

-- 来源: 2931_CALL
CREATE FUNCTION func_add_sql(num1 integer, num2 integer) RETURN integer AS BEGIN RETURN num1 + num2;

--删除函数。
-- 来源: 2931_CALL
DROP FUNCTION func_add_sql;

--创建带出参的函数。
-- 来源: 2931_CALL
CREATE FUNCTION func_increment_sql(num1 IN integer, num2 IN integer, res OUT integer) RETURN integer AS BEGIN res := num1 + num2;

--删除函数。
-- 来源: 2931_CALL
DROP FUNCTION func_increment_sql;

-- 来源: 2933_CLEAN CONNECTION
CREATE DATABASE test_clean_connection ;

-- 来源: 2933_CLEAN CONNECTION
CREATE USER jack PASSWORD '********' ;

-- 来源: 2933_CLEAN CONNECTION
DROP USER jack ;

-- 来源: 2933_CLEAN CONNECTION
DROP DATABASE test_clean_connection ;

-- 来源: 2935_CLUSTER
CREATE TABLE test_c1 ( id int , name varchar ( 20 ));

-- 来源: 2935_CLUSTER
CREATE INDEX idx_test_c1_id ON test_c1 ( id );

-- 来源: 2935_CLUSTER
DROP TABLE test_c1 ;

-- 来源: 2935_CLUSTER
CREATE TABLE test(col1 int,CONSTRAINT pk_test PRIMARY KEY (col1));

-- 删除
-- 来源: 2935_CLUSTER
DROP TABLE test;

-- 来源: 2935_CLUSTER
CREATE TABLE test_c2(id int, info varchar(4)) PARTITION BY RANGE (id)( PARTITION p1 VALUES LESS THAN (11), PARTITION p2 VALUES LESS THAN (21) );

-- 来源: 2935_CLUSTER
CREATE INDEX idx_test_c2_id1 ON test_c2(id);

-- 删除
-- 来源: 2935_CLUSTER
DROP TABLE test_c2;

-- 来源: 2936_COMMENT
CREATE TABLE emp( empno varchar(7), ename varchar(50), job varchar(50), mgr varchar(7), deptno int );

--表添加注释
-- 来源: 2936_COMMENT
COMMENT ON TABLE emp IS '部门表';

--字段添加注释
-- 来源: 2936_COMMENT
COMMENT ON COLUMN emp.empno IS '员工编号';

-- 来源: 2936_COMMENT
COMMENT ON COLUMN emp.ename IS '员工姓名';

-- 来源: 2936_COMMENT
COMMENT ON COLUMN emp.job IS '职务';

-- 来源: 2936_COMMENT
COMMENT ON COLUMN emp.mgr IS '上司编号';

-- 来源: 2936_COMMENT
COMMENT ON COLUMN emp.deptno IS '部门编号';

--删除
-- 来源: 2936_COMMENT
DROP TABLE emp;

-- 来源: 2937_COMMIT _ END
CREATE SCHEMA tpcds;

--创建表。
-- 来源: 2937_COMMIT _ END
CREATE TABLE tpcds. customer_demographics_t2 ( CD_DEMO_SK INTEGER NOT NULL, CD_GENDER CHAR(1) , CD_MARITAL_STATUS CHAR(1) , CD_EDUCATION_STATUS CHAR(20) , CD_PURCHASE_ESTIMATE INTEGER , CD_CREDIT_RATING CHAR(10) , CD_DEP_COUNT INTEGER , CD_DEP_EMPLOYED_COUNT INTEGER , CD_DEP_COLLEGE_COUNT INTEGER ) ;

--删除表 tpcds. customer_demographics_t2。
-- 来源: 2937_COMMIT _ END
DROP TABLE tpcds. customer_demographics_t2;

-- 删除SCHEMA。
-- 来源: 2937_COMMIT _ END
DROP SCHEMA tpcds;

--创建表。
-- 来源: 2938_COMMIT PREPARED
CREATE TABLE item1(id int);

--删除表。
-- 来源: 2938_COMMIT PREPARED
DROP TABLE item1;

-- 来源: 2939_COPY
CREATE SCHEMA tpcds;

--创建 tpcds. ship_mode表。
-- 来源: 2939_COPY
CREATE TABLE tpcds. ship_mode ( SM_SHIP_MODE_SK INTEGER NOT NULL, SM_SHIP_MODE_ID CHAR(16) NOT NULL, SM_TYPE CHAR(30) , SM_CODE CHAR(10) , SM_CARRIER CHAR(20) , SM_CONTRACT CHAR(20) ) ;

--创建 tpcds. ship_mode_t1表。
-- 来源: 2939_COPY
CREATE TABLE tpcds. ship_mode_t1 ( SM_SHIP_MODE_SK INTEGER NOT NULL, SM_SHIP_MODE_ID CHAR(16) NOT NULL, SM_TYPE CHAR(30) , SM_CODE CHAR(10) , SM_CARRIER CHAR(20) , SM_CONTRACT CHAR(20) ) ;

--删除表和SCHEMA。
-- 来源: 2939_COPY
DROP TABLE tpcds. ship_mode;

-- 来源: 2939_COPY
DROP TABLE tpcds. ship_mode_t1;

-- 来源: 2939_COPY
DROP SCHEMA tpcds;

-- 来源: 2940_CREATE AGGREGATE
CREATE OR REPLACE FUNCTION int_add(int,int) returns int as $BODY$ declare begin return $1 + $2;

-- 创建测试表和添加数据
-- 来源: 2940_CREATE AGGREGATE
CREATE TABLE test_sum(a int,b int,c int);

-- 删除自定义函数
-- 来源: 2940_CREATE AGGREGATE
DROP FUNCTION int_add(int,int);

-- 删除测试表
-- 来源: 2940_CREATE AGGREGATE
DROP TABLE test_sum;

-- 来源: 2941_CREATE AUDIT POLICY
CREATE USER dev_audit PASSWORD '********' ;

-- 来源: 2941_CREATE AUDIT POLICY
CREATE USER bob_audit PASSWORD '********' ;

-- 来源: 2941_CREATE AUDIT POLICY
CREATE TABLE tb_for_audit ( col1 text , col2 text , col3 text );

-- 来源: 2941_CREATE AUDIT POLICY
CREATE RESOURCE LABEL adt_lb0 ADD TABLE ( tb_for_audit );

-- 来源: 2941_CREATE AUDIT POLICY
CREATE AUDIT POLICY adt1 PRIVILEGES CREATE ;

-- 来源: 2941_CREATE AUDIT POLICY
CREATE AUDIT POLICY adt2 ACCESS SELECT ;

-- 来源: 2941_CREATE AUDIT POLICY
CREATE AUDIT POLICY adt3 PRIVILEGES CREATE ON LABEL ( adt_lb0 ) FILTER ON ROLES ( dev_audit , bob_audit );

-- 来源: 2941_CREATE AUDIT POLICY
CREATE AUDIT POLICY adt4 ACCESS SELECT ON LABEL ( adt_lb0 ), INSERT ON LABEL ( adt_lb0 ), DELETE FILTER ON ROLES ( dev_audit , bob_audit ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' );

-- 来源: 2941_CREATE AUDIT POLICY
ALTER AUDIT POLICY adt4 REMOVE ACCESS ( SELECT ON LABEL ( adt_lb0 ));

-- 来源: 2941_CREATE AUDIT POLICY
DROP AUDIT POLICY adt1 , adt2 , adt3 , adt4 ;

-- 来源: 2941_CREATE AUDIT POLICY
DROP RESOURCE LABEL adt_lb0 ;

-- 来源: 2941_CREATE AUDIT POLICY
DROP TABLE tb_for_audit ;

-- 来源: 2941_CREATE AUDIT POLICY
DROP USER dev_audit , bob_audit ;

-- 来源: 2942_CREATE CAST
CREATE OR REPLACE FUNCTION double_to_timestamp(double precision) RETURNS TIMESTAMP WITH TIME ZONE AS $$ SELECT to_timestamp($1);

-- 来源: 2944_CREATE DATABASE
CREATE USER jim PASSWORD '********';

--创建一个GBK编码的数据库testdb1。
-- 来源: 2944_CREATE DATABASE
CREATE DATABASE testdb1 ENCODING 'GBK' template = template0;

-- 来源: 2944_CREATE DATABASE
CREATE DATABASE testdb2 OWNER jim DBCOMPATIBILITY = 'A';

--创建兼容A格式的数据库并指定时区。
-- 来源: 2944_CREATE DATABASE
CREATE DATABASE testdb3 DBCOMPATIBILITY 'A' DBTIMEZONE='+08:00';

-- 来源: 2945_CREATE DATABASE LINK
CREATE USER user01 WITH SYSADMIN PASSWORD '********';

--创建私有dblink
-- 来源: 2945_CREATE DATABASE LINK
CREATE DATABASE LINK private_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

--删除私有dblink
-- 来源: 2945_CREATE DATABASE LINK
DROP DATABASE LINK private_dblink;

-- 来源: 2945_CREATE DATABASE LINK
DROP USER user01 CASCADE;

-- 来源: 2946_CREATE DATA SOURCE
CREATE DATA SOURCE ds_test1;

--创建一个Data Source对象，含TYPE信息，VERSION为NULL。
-- 来源: 2946_CREATE DATA SOURCE
CREATE DATA SOURCE ds_test2 TYPE 'MPPDB' VERSION NULL;

--创建一个Data Source对象，仅含OPTIONS。
-- 来源: 2946_CREATE DATA SOURCE
CREATE DATA SOURCE ds_test3 OPTIONS (dsn ' GaussDB ', encoding 'utf8');

--创建一个Data Source对象，含TYPE, VERSION, OPTIONS。
-- 来源: 2946_CREATE DATA SOURCE
CREATE DATA SOURCE ds_test4 TYPE 'unknown' VERSION '11.2.3' OPTIONS (dsn ' GaussDB ', username 'userid', password '********', encoding '');

--删除Data Source对象。
-- 来源: 2946_CREATE DATA SOURCE
DROP DATA SOURCE ds_test1;

-- 来源: 2946_CREATE DATA SOURCE
DROP DATA SOURCE ds_test2;

-- 来源: 2946_CREATE DATA SOURCE
DROP DATA SOURCE ds_test3;

-- 来源: 2946_CREATE DATA SOURCE
DROP DATA SOURCE ds_test4;

-- 来源: 2947_CREATE DIRECTORY
CREATE OR REPLACE DIRECTORY dir AS '/tmp/';

--删除目录。
-- 来源: 2947_CREATE DIRECTORY
DROP DIRECTORY dir;

-- 来源: 2948_CREATE EVENT
CREATE DATABASE test_event WITH DBCOMPATIBILITY = 'b';

-- 来源: 2948_CREATE EVENT
CREATE TABLE t_ev(num int);

--删除表。
-- 来源: 2948_CREATE EVENT
DROP TABLE t_ev;

-- 来源: 2948_CREATE EVENT
DROP DATABASE test_event;

-- 来源: 2949_CREATE EXTENSION
CREATE EXTENSION IF NOT EXISTS security_plugin;

-- 来源: 2949_CREATE EXTENSION
DROP EXTENSION security_plugin;

-- 来源: 2950_CREATE FOREIGN TABLE
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

--创建外表
-- 来源: 2950_CREATE FOREIGN TABLE
CREATE FOREIGN TABLE foreign_tbl (col1 text) SERVER my_server OPTIONS (logtype 'pg_log');

--删除外表
-- 来源: 2950_CREATE FOREIGN TABLE
DROP FOREIGN TABLE foreign_tbl;

--删除server
-- 来源: 2950_CREATE FOREIGN TABLE
DROP SERVER my_server;

-- 来源: 2951_CREATE FUNCTION
CREATE FUNCTION func_add_sql(integer, integer) RETURNS integer AS 'select $1 + $2;

--利用参数名用 plpgsql 自增一个整数。
-- 来源: 2951_CREATE FUNCTION
CREATE OR REPLACE FUNCTION func_increment_plsql(i integer) RETURNS integer AS $$ BEGIN RETURN i + 1;

--返回RECORD类型
-- 来源: 2951_CREATE FUNCTION
CREATE OR REPLACE FUNCTION func_increment_sql(i int, out result_1 bigint, out result_2 bigint) RETURNS SETOF RECORD AS $$ BEGIN result_1 = i + 1;

--返回一个包含多个输出参数的记录。
-- 来源: 2951_CREATE FUNCTION
CREATE FUNCTION func_dup_sql(in int, out f1 int, out f2 text) AS $$ SELECT $1, CAST($1 AS text) || ' is text' $$ LANGUAGE SQL;

--计算两个整数的和，并返回结果。如果输入为null，则返回null。
-- 来源: 2951_CREATE FUNCTION
CREATE FUNCTION func_add_sql2(num1 integer, num2 integer) RETURN integer AS BEGIN RETURN num1 + num2;

--修改函数func_add_sql2的执行规则为IMMUTABLE，即参数不变时返回相同结果。
-- 来源: 2951_CREATE FUNCTION
ALTER FUNCTION func_add_sql2(INTEGER, INTEGER) IMMUTABLE;

--将函数func_add_sql2的名称修改为add_two_number。
-- 来源: 2951_CREATE FUNCTION
ALTER FUNCTION func_add_sql2(INTEGER, INTEGER) RENAME TO add_two_number;

--创建jim用户。
-- 来源: 2951_CREATE FUNCTION
CREATE USER jim PASSWORD '********';

--将函数add_two_number的所有者改为 jim 。
-- 来源: 2951_CREATE FUNCTION
ALTER FUNCTION add_two_number(INTEGER, INTEGER) OWNER TO jim ;

--删除函数。
-- 来源: 2951_CREATE FUNCTION
DROP FUNCTION func_add_sql;

-- 来源: 2951_CREATE FUNCTION
DROP FUNCTION func_increment_plsql;

-- 来源: 2951_CREATE FUNCTION
DROP FUNCTION func_increment_sql;

-- 来源: 2951_CREATE FUNCTION
DROP FUNCTION func_dup_sql;

-- 来源: 2951_CREATE FUNCTION
DROP FUNCTION add_two_number;

--删除jim用户
-- 来源: 2951_CREATE FUNCTION
DROP USER jim;

--创建函数
-- 来源: 2951_CREATE FUNCTION
CREATE OR REPLACE FUNCTION func1(in a integer, out b integer) RETURNS int AS $$ DECLARE c int;

--删除函数func
-- 来源: 2951_CREATE FUNCTION
DROP FUNCTION func1;

-- 不打开参数set behavior_compat_options = 'proc_outparam_override'时，被匿名块或存储过程直接调用的函数的OUT、IN OUT出参不能使用复合类型，并且RETURN值会被当做OUT出参的第一个值导致调用失败
-- 来源: 2951_CREATE FUNCTION
CREATE TYPE rec as(c1 int, c2 int);

-- 来源: 2951_CREATE FUNCTION
CREATE OR REPLACE FUNCTION func(a in out rec, b in out int) RETURN int AS BEGIN a.c1:=100;

-- 来源: 2951_CREATE FUNCTION
DROP FUNCTION func;

-- 来源: 2951_CREATE FUNCTION
DROP TYPE rec;

--以下示例只有当数据库兼容模式为A时可以执行
-- 来源: 2951_CREATE FUNCTION
CREATE OR REPLACE PACKAGE pkg_type AS type table_of_index_int is table of integer index by integer;

--创建一个返回table of integer index by integer类型结果的函数
-- 来源: 2951_CREATE FUNCTION
CREATE OR REPLACE FUNCTION func_001(a in out pkg_type.table_of_index_int, b in out pkg_type.table_of_index_var) --#add in & inout #defult value RETURN pkg_type.table_of_index_int AS table_of_index_int_val pkg_type.table_of_index_int;

--创建一个含有IN/OUT类型参数的函数
-- 来源: 2951_CREATE FUNCTION
CREATE OR REPLACE FUNCTION func_001(a in out date, b in out date) --#add in & inout #defult value RETURN integer AS BEGIN raise info '%', a;

--创建一个含有IN/OUT类型参数的函数
-- 来源: 2951_CREATE FUNCTION
CREATE OR REPLACE FUNCTION func_001(a in out INT, b in out date) --#add in & inout #defult value RETURN INT AS BEGIN raise info '%', a;

--删除函数
-- 来源: 2951_CREATE FUNCTION
DROP FUNCTION func_001;

--删除package
-- 来源: 2951_CREATE FUNCTION
DROP PACKAGE pkg_type;

-- 来源: 2953_CREATE GROUP
CREATE GROUP super_users WITH PASSWORD "********";

--创建用户。
-- 来源: 2953_CREATE GROUP
CREATE ROLE lche WITH PASSWORD "********";

--创建用户。
-- 来源: 2953_CREATE GROUP
CREATE ROLE jim WITH PASSWORD "********";

--向用户组中添加用户。
-- 来源: 2953_CREATE GROUP
ALTER GROUP super_users ADD USER lche, jim;

--从用户组中删除用户。
-- 来源: 2953_CREATE GROUP
ALTER GROUP super_users DROP USER jim;

--修改用户组的名称。
-- 来源: 2953_CREATE GROUP
ALTER GROUP super_users RENAME TO normal_users;

--删除用户。
-- 来源: 2953_CREATE GROUP
DROP ROLE lche, jim;

--删除用户组。
-- 来源: 2953_CREATE GROUP
DROP GROUP normal_users;

-- 来源: 2954_CREATE INCREMENTAL MATERIALIZED VIEW
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建增量物化视图。
-- 来源: 2954_CREATE INCREMENTAL MATERIALIZED VIEW
CREATE INCREMENTAL MATERIALIZED VIEW my_imv AS SELECT * FROM my_table;

--删除增量物化视图。
-- 来源: 2954_CREATE INCREMENTAL MATERIALIZED VIEW
DROP MATERIALIZED VIEW my_imv;

--删除普通表my_table。
-- 来源: 2954_CREATE INCREMENTAL MATERIALIZED VIEW
DROP TABLE my_table;

-- 来源: 2955_CREATE INDEX
CREATE TABLE tbl_test1( id int, --用户id name varchar(50), --用户姓名 postcode char(6) --邮编 );

--创建表空间tbs_index1。
-- 来源: 2955_CREATE INDEX
CREATE TABLESPACE tbs_index1 RELATIVE LOCATION 'test_tablespace/tbs_index1';

--为表tbl_test1创建索引idx_test1指定表空间。
-- 来源: 2955_CREATE INDEX
CREATE INDEX idx_test1 ON tbl_test1(name) TABLESPACE tbs_index1;

--删除索引。
-- 来源: 2955_CREATE INDEX
DROP INDEX idx_test1;

--删除表空间
-- 来源: 2955_CREATE INDEX
DROP TABLESPACE tbs_index1;

-- 来源: 2955_CREATE INDEX
CREATE UNIQUE INDEX idx_test2 ON tbl_test1(id);

--删除索引。
-- 来源: 2955_CREATE INDEX
DROP INDEX idx_test2;

-- 来源: 2955_CREATE INDEX
CREATE INDEX idx_test3 ON tbl_test1(substr(postcode,2));

--删除索引。
-- 来源: 2955_CREATE INDEX
DROP INDEX idx_test3;

-- 来源: 2955_CREATE INDEX
CREATE INDEX idx_test4 ON tbl_test1(id) WHERE id IS NOT NULL;

-- 删除索引。
-- 来源: 2955_CREATE INDEX
DROP INDEX idx_test4;

-- 删除表
-- 来源: 2955_CREATE INDEX
DROP TABLE tbl_test1;

-- 来源: 2955_CREATE INDEX
CREATE TABLE student(id int, name varchar(20)) PARTITION BY RANGE (id) ( PARTITION p1 VALUES LESS THAN (200), PARTITION pmax VALUES LESS THAN (MAXVALUE) );

--创建LOCAL分区索引不指定索引分区的名称。
-- 来源: 2955_CREATE INDEX
CREATE INDEX idx_student1 ON student(id) LOCAL;

--删除LOCAL分区索引。
-- 来源: 2955_CREATE INDEX
DROP INDEX idx_student1;

--创建GLOBAL索引。
-- 来源: 2955_CREATE INDEX
CREATE INDEX idx_student2 ON student(name) GLOBAL;

--删除GLOBAL分区索引。
-- 来源: 2955_CREATE INDEX
DROP INDEX idx_student2;

--删除表。
-- 来源: 2955_CREATE INDEX
DROP TABLE student;

-- 来源: 2957_CREATE MASKING POLICY
CREATE USER dev_mask PASSWORD '********' ;

-- 来源: 2957_CREATE MASKING POLICY
CREATE USER bob_mask PASSWORD '********' ;

-- 来源: 2957_CREATE MASKING POLICY
CREATE TABLE tb_for_masking ( idx int , col1 text , col2 text , col3 text , col4 text , col5 text , col6 text , col7 text , col8 text );

-- 来源: 2957_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb1 ADD COLUMN ( tb_for_masking . col1 );

-- 来源: 2957_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb2 ADD COLUMN ( tb_for_masking . col2 );

-- 来源: 2957_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb3 ADD COLUMN ( tb_for_masking . col3 );

-- 来源: 2957_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb4 ADD COLUMN ( tb_for_masking . col4 );

-- 来源: 2957_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb5 ADD COLUMN ( tb_for_masking . col5 );

-- 来源: 2957_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb6 ADD COLUMN ( tb_for_masking . col6 );

-- 来源: 2957_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb7 ADD COLUMN ( tb_for_masking . col7 );

-- 来源: 2957_CREATE MASKING POLICY
CREATE RESOURCE LABEL mask_lb8 ADD COLUMN ( tb_for_masking . col8 );

-- 来源: 2957_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol1 maskall ON LABEL ( mask_lb1 );

-- 来源: 2957_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol2 alldigitsmasking ON LABEL ( mask_lb2 );

-- 来源: 2957_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol3 basicemailmasking ON LABEL ( mask_lb3 );

-- 来源: 2957_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol4 fullemailmasking ON LABEL ( mask_lb4 );

-- 来源: 2957_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol5 creditcardmasking ON LABEL ( mask_lb5 );

-- 来源: 2957_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol6 shufflemasking ON LABEL ( mask_lb6 );

-- 来源: 2957_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol7 regexpmasking ( '[\d+]' , '*' , 2 , 9 ) ON LABEL ( mask_lb7 );

-- 来源: 2957_CREATE MASKING POLICY
CREATE MASKING POLICY maskpol8 randommasking ON LABEL ( mask_lb8 ) FILTER ON ROLES ( dev_mask , bob_mask ), APP ( gsql ), IP ( '10.20.30.40' , '127.0.0.0/24' );

-- 来源: 2957_CREATE MASKING POLICY
DROP MASKING POLICY maskpol1 , maskpol2 , maskpol3 , maskpol4 , maskpol5 , maskpol6 , maskpol7 , maskpol8 ;

-- 来源: 2957_CREATE MASKING POLICY
DROP RESOURCE LABEL mask_lb1 , mask_lb2 , mask_lb3 , mask_lb4 , mask_lb5 , mask_lb6 , mask_lb7 , mask_lb8 ;

-- 来源: 2957_CREATE MASKING POLICY
DROP TABLE tb_for_masking ;

-- 来源: 2957_CREATE MASKING POLICY
DROP USER dev_mask , bob_mask ;

-- 来源: 2958_CREATE MATERIALIZED VIEW
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建全量物化视图。
-- 来源: 2958_CREATE MATERIALIZED VIEW
CREATE MATERIALIZED VIEW my_mv AS SELECT * FROM my_table;

--删除全量物化视图。
-- 来源: 2958_CREATE MATERIALIZED VIEW
DROP MATERIALIZED VIEW my_mv;

--删除普通表my_table。
-- 来源: 2958_CREATE MATERIALIZED VIEW
DROP TABLE my_table;

-- 来源: 2959_CREATE MODEL
CREATE TABLE houses ( id INTEGER, tax INTEGER, bedroom INTEGER, bath DOUBLE PRECISION, price INTEGER, size INTEGER, lot INTEGER, mark text );

--训练模型
-- 来源: 2959_CREATE MODEL
CREATE MODEL price_model USING logistic_regression FEATURES size, lot TARGET mark FROM HOUSES WITH learning_rate=0.88, max_iterations=default;

--删除模型
-- 来源: 2959_CREATE MODEL
DROP MODEL price_model;

--删除表
-- 来源: 2959_CREATE MODEL
DROP TABLE houses;

-- 来源: 2962_CREATE PACKAGE
CREATE OR REPLACE PACKAGE emp_bonus IS var1 INT:=1;

-- 来源: 2962_CREATE PACKAGE
DROP TABLE IF EXISTS test1;

-- 来源: 2962_CREATE PACKAGE
CREATE OR REPLACE PACKAGE BODY emp_bonus IS var3 INT:=3;

-- 来源: 2962_CREATE PACKAGE
ALTER PACKAGE emp_bonus OWNER TO omm;

-- 来源: 2962_CREATE PACKAGE
DROP TABLE IF EXISTS test1;

-- 来源: 2962_CREATE PACKAGE
DROP TABLE IF EXISTS test1;

-- 来源: 2962_CREATE PACKAGE
DROP TABLE IF EXISTS test1;

-- 来源: 2962_CREATE PACKAGE
DROP PACKAGE emp_bonus;

-- 来源: 2963_CREATE PROCEDURE
CREATE OR REPLACE PROCEDURE prc_add ( param1 IN INTEGER , param2 IN OUT INTEGER ) AS BEGIN param2 : = param1 + param2 ;

-- 来源: 2963_CREATE PROCEDURE
CREATE OR REPLACE PROCEDURE pro_variadic ( var1 VARCHAR2 ( 10 ) DEFAULT 'hello!' , var4 VARIADIC int4 []) AS BEGIN dbe_output . print_line ( var1 );

-- 来源: 2963_CREATE PROCEDURE
CREATE TABLE tb1 ( a integer );

-- 来源: 2963_CREATE PROCEDURE
CREATE PROCEDURE insert_data ( v integer ) SECURITY INVOKER AS BEGIN INSERT INTO tb1 VALUES ( v );

-- 来源: 2963_CREATE PROCEDURE
CREATE OR REPLACE PROCEDURE package_func_overload ( col int , col2 out varchar ) package as declare col_type text ;

-- 来源: 2963_CREATE PROCEDURE
DROP PROCEDURE prc_add ;

-- 来源: 2963_CREATE PROCEDURE
DROP PROCEDURE pro_variadic ;

-- 来源: 2963_CREATE PROCEDURE
DROP PROCEDURE insert_data ;

-- 来源: 2963_CREATE PROCEDURE
DROP PROCEDURE package_func_overload ;

-- 来源: 2963_CREATE PROCEDURE
DROP TABLE tb1 ;

-- 来源: 2964_CREATE PUBLICATION
CREATE TABLE users (c1 int, c2 int);

-- 来源: 2964_CREATE PUBLICATION
CREATE TABLE departments (c1 int, c2 int);

-- 来源: 2964_CREATE PUBLICATION
CREATE TABLE mydata (c1 int, c2 int);

-- 来源: 2964_CREATE PUBLICATION
CREATE TABLE mydata2 (c1 int, c2 int);

--删除表。
-- 来源: 2964_CREATE PUBLICATION
DROP TABLE users;

-- 来源: 2964_CREATE PUBLICATION
DROP TABLE departments;

-- 来源: 2964_CREATE PUBLICATION
DROP TABLE mydata;

-- 来源: 2964_CREATE PUBLICATION
DROP TABLE mydata2;

-- 来源: 2965_CREATE RESOURCE LABEL
CREATE TABLE tb_for_label ( col1 text , col2 text , col3 text );

-- 来源: 2965_CREATE RESOURCE LABEL
CREATE SCHEMA schema_for_label ;

-- 来源: 2965_CREATE RESOURCE LABEL
CREATE VIEW view_for_label AS SELECT 1 ;

-- 来源: 2965_CREATE RESOURCE LABEL
CREATE FUNCTION func_for_label RETURNS TEXT AS $$ SELECT col1 FROM tb_for_label ;

-- 来源: 2965_CREATE RESOURCE LABEL
CREATE RESOURCE LABEL IF NOT EXISTS table_label add TABLE ( public . tb_for_label );

-- 来源: 2965_CREATE RESOURCE LABEL
CREATE RESOURCE LABEL IF NOT EXISTS column_label add COLUMN ( public . tb_for_label . col1 );

-- 来源: 2965_CREATE RESOURCE LABEL
CREATE RESOURCE LABEL IF NOT EXISTS schema_label add SCHEMA ( schema_for_label );

-- 来源: 2965_CREATE RESOURCE LABEL
CREATE RESOURCE LABEL IF NOT EXISTS view_label add VIEW ( view_for_label );

-- 来源: 2965_CREATE RESOURCE LABEL
CREATE RESOURCE LABEL IF NOT EXISTS func_label add FUNCTION ( func_for_label );

-- 来源: 2965_CREATE RESOURCE LABEL
DROP RESOURCE LABEL func_label , view_label , schema_label , column_label , table_label ;

-- 来源: 2965_CREATE RESOURCE LABEL
DROP FUNCTION func_for_label ;

-- 来源: 2965_CREATE RESOURCE LABEL
DROP VIEW view_for_label ;

-- 来源: 2965_CREATE RESOURCE LABEL
DROP SCHEMA schema_for_label ;

-- 来源: 2965_CREATE RESOURCE LABEL
DROP TABLE tb_for_label ;

-- 来源: 2966_CREATE RESOURCE POOL
CREATE RESOURCE POOL pool1 ;

-- 来源: 2966_CREATE RESOURCE POOL
CREATE RESOURCE POOL pool2 WITH ( CONTROL_GROUP = "High" );

-- 来源: 2966_CREATE RESOURCE POOL
CREATE RESOURCE POOL pool3 WITH ( CONTROL_GROUP = "class1:Low" );

-- 来源: 2966_CREATE RESOURCE POOL
CREATE RESOURCE POOL pool4 WITH ( CONTROL_GROUP = "class1:wg1" );

-- 来源: 2966_CREATE RESOURCE POOL
CREATE RESOURCE POOL pool5 WITH ( CONTROL_GROUP = "class1:wg2:3" );

-- 来源: 2966_CREATE RESOURCE POOL
DROP RESOURCE POOL pool1 ;

-- 来源: 2966_CREATE RESOURCE POOL
DROP RESOURCE POOL pool2 ;

-- 来源: 2966_CREATE RESOURCE POOL
DROP RESOURCE POOL pool3 ;

-- 来源: 2966_CREATE RESOURCE POOL
DROP RESOURCE POOL pool4 ;

-- 来源: 2966_CREATE RESOURCE POOL
DROP RESOURCE POOL pool5 ;

-- 来源: 2967_CREATE ROLE
CREATE ROLE manager IDENTIFIED BY ' ******** ';

--创建一个角色，从2015年1月1日开始生效，到2026年1月1日失效。
-- 来源: 2967_CREATE ROLE
CREATE ROLE miriam WITH LOGIN PASSWORD ' ******** ' VALID BEGIN '2015-01-01' VALID UNTIL '2026-01-01';

--修改角色manager的密码为********。
-- 来源: 2967_CREATE ROLE
ALTER ROLE manager IDENTIFIED BY '********' REPLACE ' ********** ';

--修改角色manager为系统管理员。
-- 来源: 2967_CREATE ROLE
ALTER ROLE manager SYSADMIN;

--删除角色manager。
-- 来源: 2967_CREATE ROLE
DROP ROLE manager;

--删除角色miriam。
-- 来源: 2967_CREATE ROLE
DROP GROUP miriam;

-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
CREATE USER alice PASSWORD '********';

--创建用户bob。
-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
CREATE USER bob PASSWORD '********';

--创建数据表all_data。
-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
CREATE TABLE public.all_data(id int, role varchar(100), data varchar(100));

--打开行访问控制策略开关。
-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
ALTER TABLE all_data ENABLE ROW LEVEL SECURITY;

--删除数据表all_data。
-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
DROP TABLE public.all_data;

--删除用户alice, bob。
-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
DROP USER alice, bob;

-- 来源: 2969_CREATE RULE
CREATE TABLE def_test ( c1 int4 DEFAULT 5, c2 text DEFAULT 'initial_default' );

-- 来源: 2969_CREATE RULE
CREATE VIEW def_view_test AS SELECT * FROM def_test;

--删除表def_test、视图def_view_test。
-- 来源: 2969_CREATE RULE
DROP VIEW def_view_test;

-- 来源: 2969_CREATE RULE
DROP TABLE def_test;

-- 来源: 2970_CREATE SCHEMA
CREATE DATABASE test1 WITH DBCOMPATIBILITY = 'b';

--创建一个角色role1。
-- 来源: 2970_CREATE SCHEMA
CREATE ROLE role1 IDENTIFIED BY ' ******** ';

-- 为用户role1创建一个同名schema，子命令创建的表films和winners的拥有者为role1。
-- 来源: 2970_CREATE SCHEMA
CREATE SCHEMA AUTHORIZATION role1 CREATE TABLE films (title text, release date, awards text[]) CREATE VIEW winners AS SELECT title, release FROM films WHERE awards IS NOT NULL;

-- 创建一个schema ds，指定schema的默认字符集为utf8mb4，默认字符序为utf8mb4_bin。仅在B模式下（即sql_compatibility='B'）支持该语法。
-- 来源: 2970_CREATE SCHEMA
CREATE SCHEMA ds CHARACTER SET utf8mb4 COLLATE utf8mb4_bin;

--删除schema。
-- 来源: 2970_CREATE SCHEMA
DROP SCHEMA role1 CASCADE;

-- 来源: 2970_CREATE SCHEMA
DROP SCHEMA ds CASCADE;

--删除用户。
-- 来源: 2970_CREATE SCHEMA
DROP USER role1 CASCADE;

-- 来源: 2970_CREATE SCHEMA
DROP DATABASE test1;

-- 来源: 2971_CREATE SECURITY LABEL
CREATE SECURITY LABEL sec_label 'L1:G4' ;

-- 来源: 2971_CREATE SECURITY LABEL
DROP SECURITY LABEL sec_label ;

-- 来源: 2972_CREATE SEQUENCE
CREATE SEQUENCE seq1 START 101 INCREMENT 10 ;

-- 来源: 2972_CREATE SEQUENCE
DROP SEQUENCE seq1 ;

-- 来源: 2972_CREATE SEQUENCE
CREATE TABLE test1 ( id int PRIMARY KEY , name varchar ( 20 ));

-- 来源: 2972_CREATE SEQUENCE
CREATE SEQUENCE test_seq2 START 1 NO CYCLE OWNED BY test1 . id ;

-- 来源: 2972_CREATE SEQUENCE
ALTER TABLE test1 ALTER COLUMN id SET DEFAULT nextval ( 'test_seq2' :: regclass );

-- 来源: 2972_CREATE SEQUENCE
DROP SEQUENCE test_seq2 CASCADE ;

-- 来源: 2972_CREATE SEQUENCE
DROP TABLE test1 ;

-- 来源: 2973_CREATE SERVER
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

--删除my_server。
-- 来源: 2973_CREATE SERVER
DROP SERVER my_server;

-- 来源: 2974_CREATE SUBSCRIPTION
CREATE TABLE users (c1 int, c2 int);

-- 来源: 2974_CREATE SUBSCRIPTION
CREATE TABLE departments (c1 int, c2 int);

-- 来源: 2974_CREATE SUBSCRIPTION
CREATE TABLE mydata (c1 int, c2 int);

--删除表。
-- 来源: 2974_CREATE SUBSCRIPTION
DROP TABLE users;

-- 来源: 2974_CREATE SUBSCRIPTION
DROP TABLE departments;

-- 来源: 2974_CREATE SUBSCRIPTION
DROP TABLE mydata;

-- 来源: 2975_CREATE SYNONYM
CREATE SCHEMA ot;

--创建表ot.t1及其同义词t1。
-- 来源: 2975_CREATE SYNONYM
CREATE TABLE ot.t1(id int, name varchar2(10));

-- 来源: 2975_CREATE SYNONYM
CREATE OR REPLACE SYNONYM t1 FOR ot.t1;

--创建同义词v1及其关联视图ot.v_t1。
-- 来源: 2975_CREATE SYNONYM
CREATE SYNONYM v1 FOR ot.v_t1;

-- 来源: 2975_CREATE SYNONYM
CREATE VIEW ot.v_t1 AS SELECT * FROM ot.t1;

--创建重载函数ot.add及其同义词add。
-- 来源: 2975_CREATE SYNONYM
CREATE OR REPLACE FUNCTION ot.add(a integer, b integer) RETURNS integer AS $$ SELECT $1 + $2 $$ LANGUAGE sql;

-- 来源: 2975_CREATE SYNONYM
CREATE OR REPLACE FUNCTION ot.add(a decimal(5,2), b decimal(5,2)) RETURNS decimal(5,2) AS $$ SELECT $1 + $2 $$ LANGUAGE sql;

-- 来源: 2975_CREATE SYNONYM
CREATE OR REPLACE SYNONYM add FOR ot.add;

--创建存储过程ot.register及其同义词register。
-- 来源: 2975_CREATE SYNONYM
CREATE PROCEDURE ot.register(n_id integer, n_name varchar2(10)) SECURITY INVOKER AS BEGIN INSERT INTO ot.t1 VALUES(n_id, n_name);

-- 来源: 2975_CREATE SYNONYM
CREATE OR REPLACE SYNONYM register FOR ot.register;

--删除同义词。
-- 来源: 2975_CREATE SYNONYM
DROP SYNONYM t1;

-- 来源: 2975_CREATE SYNONYM
DROP SYNONYM IF EXISTS v1;

-- 来源: 2975_CREATE SYNONYM
DROP SYNONYM IF EXISTS add;

-- 来源: 2975_CREATE SYNONYM
DROP SYNONYM register;

-- 来源: 2975_CREATE SYNONYM
DROP SCHEMA ot CASCADE;

-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t1 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t2 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60), W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t2;

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t1;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t3 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) DEFAULT 'GA', W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t3;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--创建表，并在事务结束时检查W_WAREHOUSE_NAME字段是否有重复。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t4 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) UNIQUE DEFERRABLE, W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t4;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t5 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2), UNIQUE(W_WAREHOUSE_NAME) WITH(fillfactor=70) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t5;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--或者用下面的语法。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t6 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) UNIQUE, W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) ) WITH(fillfactor=70);

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t6;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--创建表，并指定该表数据不写入预写日志。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE UNLOGGED TABLE tpcds. warehouse_t7 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t7;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--创建表临时表。
-- 来源: 2976_CREATE TABLE
CREATE TEMPORARY TABLE warehouse_t24 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

--创建本地临时表，并指定提交事务时删除该临时表数据。
-- 来源: 2976_CREATE TABLE
CREATE TEMPORARY TABLE warehouse_t25 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) ) ON COMMIT DELETE ROWS;

--创建全局临时表，并指定会话结束时删除该临时表数据，当前Ustore存储引擎不支持全局临时表。
-- 来源: 2976_CREATE TABLE
CREATE GLOBAL TEMPORARY TABLE gtt1 ( ID INTEGER NOT NULL, NAME CHAR(16) NOT NULL, ADDRESS VARCHAR(50) , POSTCODE CHAR(6) ) ON COMMIT PRESERVE ROWS;

--创建表时，不希望因为表已存在而报错。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE IF NOT EXISTS tpcds. warehouse_t8 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t8;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--创建普通表空间。
-- 来源: 2976_CREATE TABLE
CREATE TABLESPACE DS_TABLESPACE1 RELATIVE LOCATION 'tablespace/tablespace_1';

--创建表时，指定表空间。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t9 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) ) TABLESPACE DS_TABLESPACE1;

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t9;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--创建表时，单独指定W_WAREHOUSE_NAME的索引表空间。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t10 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) UNIQUE USING INDEX TABLESPACE DS_TABLESPACE1, W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t10;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t11 ( W_WAREHOUSE_SK INTEGER PRIMARY KEY, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t11;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

---或是用下面的语法，效果完全一样。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t12 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2), PRIMARY KEY(W_WAREHOUSE_SK) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t12;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--或是用下面的语法，指定约束的名称。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t13 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2), CONSTRAINT W_CSTR_KEY1 PRIMARY KEY(W_WAREHOUSE_SK) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t13;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--创建一个有主键约束并且指定约束类型及排序方式的表。
-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t13_1 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2), CONSTRAINT PRIMARY KEY USING BTREE (W_WAREHOUSE_SK DESC) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t13_1;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--创建一个有复合主键约束的表。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t14 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2), CONSTRAINT W_CSTR_KEY2 PRIMARY KEY(W_WAREHOUSE_SK, W_WAREHOUSE_ID) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t14;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--定义一个检查列约束。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t19 ( W_WAREHOUSE_SK INTEGER PRIMARY KEY CHECK (W_WAREHOUSE_SK > 0), W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) CHECK (W_WAREHOUSE_NAME IS NOT NULL), W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t20 ( W_WAREHOUSE_SK INTEGER PRIMARY KEY, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) CHECK (W_WAREHOUSE_NAME IS NOT NULL), W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2), CONSTRAINT W_CONSTR_KEY2 CHECK(W_WAREHOUSE_SK > 0 AND W_WAREHOUSE_NAME IS NOT NULL) );

--向 tpcds. warehouse_t19表中增加一个varchar列。
-- 来源: 2976_CREATE TABLE
ALTER TABLE tpcds. warehouse_t19 ADD W_GOODS_CATEGORY varchar(30);

--给 tpcds. warehouse_t19表增加一个检查约束。
-- 来源: 2976_CREATE TABLE
ALTER TABLE tpcds. warehouse_t19 ADD CONSTRAINT W_CONSTR_KEY4 CHECK (W_STATE IS NOT NULL);

--在一个操作中改变两个现存字段的类型。
-- 来源: 2976_CREATE TABLE
ALTER TABLE tpcds. warehouse_t19 ALTER COLUMN W_GOODS_CATEGORY TYPE varchar(80), ALTER COLUMN W_STREET_NAME TYPE varchar(100);

--此语句与上面语句等效。
-- 来源: 2976_CREATE TABLE
ALTER TABLE tpcds. warehouse_t19 MODIFY (W_GOODS_CATEGORY varchar(30), W_STREET_NAME varchar(60));

--给一个已存在字段添加非空约束。
-- 来源: 2976_CREATE TABLE
ALTER TABLE tpcds. warehouse_t19 ALTER COLUMN W_GOODS_CATEGORY SET NOT NULL;

--移除已存在字段的非空约束。
-- 来源: 2976_CREATE TABLE
ALTER TABLE tpcds. warehouse_t19 ALTER COLUMN W_GOODS_CATEGORY DROP NOT NULL;

--将表移动到另一个表空间。
-- 来源: 2976_CREATE TABLE
ALTER TABLE tpcds. warehouse_t19 SET TABLESPACE PG_DEFAULT;

--创建模式joe。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA joe;

--将表移动到另一个模式中。
-- 来源: 2976_CREATE TABLE
ALTER TABLE tpcds. warehouse_t19 SET SCHEMA joe;

--重命名已存在的表。
-- 来源: 2976_CREATE TABLE
ALTER TABLE joe.warehouse_t19 RENAME TO warehouse_t23;

--从warehouse_t23表中删除一个字段。
-- 来源: 2976_CREATE TABLE
ALTER TABLE joe.warehouse_t23 DROP COLUMN W_STREET_NAME;

--删除表空间、模式joe
-- 来源: 2976_CREATE TABLE
DROP TABLESPACE DS_TABLESPACE1;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA IF EXISTS joe CASCADE;

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t20;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--定义一个有ALWAYS属性IDENTITY列的表。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t26 ( W_WAREHOUSE_SK INTEGER GENERATED ALWAYS AS IDENTITY , W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t26;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--定义一个有BY DEFAULT属性IDENTITY列的表。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t27 ( W_WAREHOUSE_SK INTEGER GENERATED BY DEFAULT AS IDENTITY (INCREMENT BY 10 MINVALUE 200 SCALE) , W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t27;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--定义一个有BY DEFAULT ON NULL属性IDENTITY列的表。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds. warehouse_t28 ( W_WAREHOUSE_SK INTEGER GENERATED BY DEFAULT ON NULL AS IDENTITY (START WITH 10 MAXVALUE 200 CYCLE) , W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t28;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--创建一个有外键约束的表。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds.city_t23 ( W_CITY VARCHAR(60) PRIMARY KEY, W_ADDRESS TEXT );

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds.warehouse_t23 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) REFERENCES tpcds.city_t23(W_CITY), W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.city_t23;

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t23;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--或是用下面的语法，效果完全一样。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds.warehouse_t23 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) , FOREIGN KEY(W_CITY) REFERENCES tpcds.city_t23(W_CITY) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t23;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

--或是用下面的语法，指定约束的名称。
-- 来源: 2976_CREATE TABLE
CREATE SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE tpcds.warehouse_t23 ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) , CONSTRAINT W_FORE_KEY1 FOREIGN KEY(W_CITY) REFERENCES tpcds.city_t23(W_CITY) );

-- 来源: 2976_CREATE TABLE
DROP TABLE tpcds.warehouse_t23;

-- 来源: 2976_CREATE TABLE
DROP SCHEMA tpcds;

-- 来源: 2976_CREATE TABLE
CREATE TABLE t1(C1 VARCHAR(20) CHARSET utf8mb4 COLLATE utf8mb4_unicode_ci) CHARSET = utf8mb4 COLLATE = utf8mb4_ bin ;

-- 来源: 2976_CREATE TABLE
ALTER TABLE t1 charset utf8mb4 collate utf8mb4_general_ci;

-- 为t1表新增字段c3，并设置字段的字符集为utf8mb4，字符序为utf8mb4_bin
-- 来源: 2976_CREATE TABLE
ALTER TABLE t1 add c3 varchar(20) charset utf8mb4 collate utf8mb4_bin;

-- 创建开启ILM的表
-- 来源: 2976_CREATE TABLE
ALTER DATABASE SET ILM=ON;

-- 来源: 2976_CREATE TABLE
CREATE TABLE ilm_table (a int) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION ON (a != 0);

-- 来源: 2976_CREATE TABLE
CREATE TABLE ilm_table (a int);

-- 来源: 2976_CREATE TABLE
ALTER TABLE ilm_table ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION;

-- 来源: 2976_CREATE TABLE
ALTER TABLE ilm_table ILM DISABLE_ALL;

-- 来源: 2976_CREATE TABLE
ALTER TABLE ilm_table ILM ENABLE_ALL;

-- 来源: 2976_CREATE TABLE
ALTER TABLE ilm_table ILM DELETE_ALL;

-- 创建B模式数据
-- 来源: 2976_CREATE TABLE
CREATE DATABASE test_on_update DBCOMPATIBILITY 'b';

-- 创建 t1_on_update 表，设置ON UPDATE属性
-- 来源: 2976_CREATE TABLE
CREATE TABLE t1_on_update ( TS0 TIMESTAMP ON UPDATE CURRENT_TIMESTAMP , TS1 TIMESTAMP ON UPDATE CURRENT_TIMESTAMP() , TS2 TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6) , DT0 DATETIME ON UPDATE LOCALTIMESTAMP , DT1 DATETIME ON UPDATE NOW()

-- 来源: 2977_CREATE TABLE AS
CREATE TABLE test1(col1 int PRIMARY KEY,col2 varchar(10));

-- 创建test2表并向表中插入上面查询的数据。
-- 来源: 2977_CREATE TABLE AS
CREATE TABLE test2 AS SELECT * FROM test1 WHERE col1 < 100;

-- 来源: 2977_CREATE TABLE AS
CREATE TABLE test3(c1,c2) AS SELECT * FROM test1;

-- 删除。
-- 来源: 2977_CREATE TABLE AS
DROP TABLE test1,test2,test3;

-- 来源: 2978_CREATE TABLE PARTITION
CREATE SCHEMA tpcds;

--创建表 tpcds. web_returns。
-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLE tpcds. web_returns ( W_WAREHOUSE_SK INTEGER NOT NULL, W_WAREHOUSE_ID CHAR(16) NOT NULL, W_WAREHOUSE_NAME VARCHAR(20) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR(10) , W_STREET_NAME VARCHAR(60) , W_STREET_TYPE CHAR(15) , W_SUITE_NUMBER CHAR(10) , W_CITY VARCHAR(60) , W_COUNTY VARCHAR(30) , W_STATE CHAR(2) , W_ZIP CHAR(10) , W_COUNTRY VARCHAR(20) , W_GMT_OFFSET DECIMAL(5,2) );

--创建分区表 tpcds. web_returns_p1。
-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLE tpcds. web_returns_p1 ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL, WR_REFUNDED_CUSTOMER_SK INTEGER , WR_REFUNDED_CDEMO_SK INTEGER , WR_REFUNDED_HDEMO_SK INTEGER , WR_REFUNDED_ADDR_SK INTEGER , WR_RETURNING_CUSTOMER_SK INTEGER , WR_RETURNING_CDEMO_SK INTEGER , WR_RETURNING_HDEMO_SK INTEGER , WR_RETURNING_ADDR_SK INTEGER , WR_WEB_PAGE_SK INTEGER , WR_REASON_SK INTEGER , WR_ORDER_NUMBER BIGINT NOT NULL, WR_RETURN_QUANTITY INTEGER , WR_RETURN_AMT DECIMAL(7,2) , WR_RETURN_TAX DECIMAL(7,2) , WR_RETURN_AMT_INC_TAX DECIMAL(7,2) , WR_FEE DECIMAL(7,2) , WR_RETURN_SHIP_COST DECIMAL(7,2) , WR_REFUNDED_CASH DECIMAL(7,2) , WR_REVERSED_CHARGE DECIMAL(7,2) , WR_ACCOUNT_CREDIT DECIMAL(7,2) , WR_NET_LOSS DECIMAL(7,2) ) PARTITION BY RANGE(WR_RETURNED_DATE_SK) ( PARTITION P1 VALUES LESS THAN(2450815), PARTITION P2 VALUES LESS THAN(2451179), PARTITION P3 VALUES LESS THAN(2451544), PARTITION P4 VALUES LESS THAN(2451910), PARTITION P5 VALUES LESS THAN(2452275), PARTITION P6 VALUES LESS THAN(2452640), PARTITION P7 VALUES LESS THAN(2453005), PARTITION P8 VALUES LESS THAN(MAXVALUE) );

--删除分区P8。
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds. web_returns_p1 DROP PARTITION P8;

--增加分区WR_RETURNED_DATE_SK介于2453005和2453105之间。
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds. web_returns_p1 ADD PARTITION P8 VALUES LESS THAN (2453105);

--增加分区WR_RETURNED_DATE_SK介于2453105和MAXVALUE之间。
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds. web_returns_p1 ADD PARTITION P9 VALUES LESS THAN (MAXVALUE);

-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds. web_returns_p1 RENAME PARTITION P7 TO P10;

--分区P6重命名为P11。
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds. web_returns_p1 RENAME PARTITION FOR (2452639) TO P11;

--删除表tpcds.web_returns_p1。
-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLE tpcds.web_returns_p1;

--删除表tpcds.web_returns。
-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLE tpcds.web_returns;

--删除SCHEMA。
-- 来源: 2978_CREATE TABLE PARTITION
DROP SCHEMA tpcds CASCADE;

--创建开启ILM策略的表ilm_part并分区
-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLE ilm_part (a int) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION PARTITION BY RANGE (a) ( PARTITION p1 VALUES LESS THAN (10), PARTITION p2 VALUES LESS THAN (20), PARTITION p3 VALUES LESS THAN (30));

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1/tablespace_1';

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLESPACE example2 RELATIVE LOCATION 'tablespace2/tablespace_2';

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLESPACE example3 RELATIVE LOCATION 'tablespace3/tablespace_3';

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLESPACE example4 RELATIVE LOCATION 'tablespace4/tablespace_4';

--创建SCHEMA。
-- 来源: 2978_CREATE TABLE PARTITION
CREATE SCHEMA tpcds;

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLE tpcds. web_returns_p2 ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL, WR_REFUNDED_CUSTOMER_SK INTEGER , WR_REFUNDED_CDEMO_SK INTEGER , WR_REFUNDED_HDEMO_SK INTEGER , WR_REFUNDED_ADDR_SK INTEGER , WR_RETURNING_CUSTOMER_SK INTEGER , WR_RETURNING_CDEMO_SK INTEGER , WR_RETURNING_HDEMO_SK INTEGER , WR_RETURNING_ADDR_SK INTEGER , WR_WEB_PAGE_SK INTEGER , WR_REASON_SK INTEGER , WR_ORDER_NUMBER BIGINT NOT NULL, WR_RETURN_QUANTITY INTEGER , WR_RETURN_AMT DECIMAL(7,2) , WR_RETURN_TAX DECIMAL(7,2) , WR_RETURN_AMT_INC_TAX DECIMAL(7,2) , WR_FEE DECIMAL(7,2) , WR_RETURN_SHIP_COST DECIMAL(7,2) , WR_REFUNDED_CASH DECIMAL(7,2) , WR_REVERSED_CHARGE DECIMAL(7,2) , WR_ACCOUNT_CREDIT DECIMAL(7,2) , WR_NET_LOSS DECIMAL(7,2) ) TABLESPACE example1 PARTITION BY RANGE(WR_RETURNED_DATE_SK) ( PARTITION P1 VALUES LESS THAN(2450815), PARTITION P2 VALUES LESS THAN(2451179), PARTITION P3 VALUES LESS THAN(2451544), PARTITION P4 VALUES LESS THAN(2451910), PARTITION P5 VALUES LESS THAN(2452275), PARTITION P6 VALUES LESS THAN(2452640), PARTITION P7 VALUES LESS THAN(2453005), PARTITION P8 VALUES LESS THAN(MAXVALUE) TABLESPACE example2 ) ENABLE ROW MOVEMENT;

--以like方式创建一个分区表。
-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLE tpcds. web_returns_p3 (LIKE tpcds. web_returns_p2 INCLUDING PARTITION);

--修改分区P1的表空间为example2。
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds. web_returns_p2 MOVE PARTITION P1 TABLESPACE example2;

--修改分区P2的表空间为example3。
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds. web_returns_p2 MOVE PARTITION P2 TABLESPACE example3;

--以2453010为分割点切分P8。
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds. web_returns_p2 SPLIT PARTITION P8 AT (2453010) INTO ( PARTITION P9, PARTITION P10 );

--将P6，P7合并为一个分区。
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds. web_returns_p2 MERGE PARTITIONS P6, P7 INTO PARTITION P8;

--修改分区表迁移属性。
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds. web_returns_p2 DISABLE ROW MOVEMENT;

--删除表和表空间。
-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLE tpcds. web_returns_p1;

-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLE tpcds. web_returns_p2;

-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLE tpcds. web_returns_p3;

-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLESPACE example1;

-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLESPACE example2;

-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLESPACE example3;

-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLESPACE example4;

--删除SCHEMA。
-- 来源: 2978_CREATE TABLE PARTITION
DROP SCHEMA tpcds CASCADE;

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLESPACE startend_tbs1 LOCATION '/home/ omm /startend_tbs1';

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLESPACE startend_tbs2 LOCATION '/home/ omm /startend_tbs2';

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLESPACE startend_tbs3 LOCATION '/home/ omm /startend_tbs3';

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLESPACE startend_tbs4 LOCATION '/home/ omm /startend_tbs4';

-- 创建临时schema
-- 来源: 2978_CREATE TABLE PARTITION
CREATE SCHEMA tpcds;

-- 创建分区表，分区键是integer类型
-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLE tpcds.startend_pt (c1 INT, c2 INT) TABLESPACE startend_tbs1 PARTITION BY RANGE (c2) ( PARTITION p1 START(1) END(1000) EVERY(200) TABLESPACE startend_tbs2, PARTITION p2 END(2000), PARTITION p3 START(2000) END(2500) TABLESPACE startend_tbs3, PARTITION p4 START(2500), PARTITION p5 START(3000) END(5000) EVERY(1000) TABLESPACE startend_tbs4 ) ENABLE ROW MOVEMENT;

-- 增加分区: [5000, 5300), [5300, 5600), [5600, 5900), [5900, 6000)
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds.startend_pt ADD PARTITION p6 START(5000) END(6000) EVERY(300) TABLESPACE startend_tbs4;

-- 增加MAXVALUE分区: p
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds.startend_pt ADD PARTITION p7 END(MAXVALUE);

-- 重命名分区p7为p
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds.startend_pt RENAME PARTITION p7 TO p8;

-- 删除分区p
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds.startend_pt DROP PARTITION p8;

-- 重命名5950所在的分区为：p
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds.startend_pt RENAME PARTITION FOR(5950) TO p71;

-- 分裂4500所在的分区[4000, 5000)
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds.startend_pt SPLIT PARTITION FOR(4500) INTO(PARTITION q1 START(4000) END(5000) EVERY(250) TABLESPACE startend_tbs3);

-- 修改分区p2的表空间为startend_tbs
-- 来源: 2978_CREATE TABLE PARTITION
ALTER TABLE tpcds.startend_pt MOVE PARTITION p2 TABLESPACE startend_tbs4;

-- 删除表和表空间
-- 来源: 2978_CREATE TABLE PARTITION
DROP SCHEMA tpcds CASCADE;

-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLESPACE startend_tbs1;

-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLESPACE startend_tbs2;

-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLESPACE startend_tbs3;

-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLESPACE startend_tbs4;

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLE sales ( prod_id NUMBER ( 6 ), cust_id NUMBER , time_id DATE , channel_id CHAR ( 1 ), promo_id NUMBER ( 6 ), quantity_sold NUMBER ( 3 ), amount_sold NUMBER ( 10 , 2 ) ) PARTITION BY RANGE ( time_id ) INTERVAL ( '1 day' ) ( PARTITION p1 VALUES LESS THAN ( '2019-02-01 00:00:00' ), PARTITION p2 VALUES LESS THAN ( '2019-02-02 00:00:00' ) );

-- 来源: 2978_CREATE TABLE PARTITION
create table test_list ( col1 int , col2 int ) partition by list ( col1 ) ( partition p1 values ( 2000 ), partition p2 values ( 3000 ), partition p3 values ( 4000 ), partition p4 values ( 5000 ) );

-- 来源: 2978_CREATE TABLE PARTITION
alter table test_list add partition p5 values ( 6000 );

-- 来源: 2978_CREATE TABLE PARTITION
create table t1 ( col1 int , col2 int );

-- 来源: 2978_CREATE TABLE PARTITION
alter table test_list exchange partition ( p1 ) with table t1 ;

-- 来源: 2978_CREATE TABLE PARTITION
alter table test_list truncate partition p2 ;

-- 来源: 2978_CREATE TABLE PARTITION
alter table test_list drop partition p5 ;

-- 来源: 2978_CREATE TABLE PARTITION
alter table test_list merge partitions p1 , p2 into partition p2 ;

-- 来源: 2978_CREATE TABLE PARTITION
alter table test_list split partition p2 values ( 2000 ) into ( partition p1 , partition p2 );

-- 来源: 2978_CREATE TABLE PARTITION
drop table test_list ;

-- 来源: 2978_CREATE TABLE PARTITION
create table test_hash ( col1 int , col2 int ) partition by hash ( col1 ) ( partition p1 , partition p2 );

-- 来源: 2978_CREATE TABLE PARTITION
create table t1 ( col1 int , col2 int );

-- 来源: 2978_CREATE TABLE PARTITION
alter table test_hash exchange partition ( p1 ) with table t1 ;

-- 来源: 2978_CREATE TABLE PARTITION
alter table test_hash truncate partition p2 ;

-- 来源: 2978_CREATE TABLE PARTITION
drop table test_hash ;

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLE t_multi_keys_list ( a int , b varchar ( 4 ), c int ) PARTITION BY LIST ( a , b ) ( PARTITION p1 VALUES ( ( 0 , NULL ) ), PARTITION p2 VALUES ( ( 0 , '1' ), ( 0 , '2' ), ( 0 , '3' ), ( 1 , '1' ), ( 1 , '2' ) ), PARTITION p3 VALUES ( ( NULL , '0' ), ( 2 , '1' ) ), PARTITION p4 VALUES ( ( 3 , '2' ), ( NULL , NULL ) ), PARTITION pd VALUES ( DEFAULT ) );

-- 来源: 2978_CREATE TABLE PARTITION
ALTER DATABASE set ilm = on ;

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLE ilm_part ( a int ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION , PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLE ilm_part ;

-- 来源: 2978_CREATE TABLE PARTITION
ALTER DATABASE set ilm = on ;

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLE ilm_part ;

-- 来源: 2978_CREATE TABLE PARTITION
ALTER DATABASE set ilm = on ;

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLE ilm_part ;

-- 来源: 2978_CREATE TABLE PARTITION
ALTER DATABASE set ilm = on ;

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLE ilm_part ;

-- 来源: 2978_CREATE TABLE PARTITION
ALTER DATABASE set ilm = on ;

-- 来源: 2978_CREATE TABLE PARTITION
CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

-- 来源: 2978_CREATE TABLE PARTITION
DROP TABLE ilm_part ;

-- 来源: 2979_CREATE TABLESPACE
CREATE TABLESPACE ds_location1 RELATIVE LOCATION 'tablespace/tablespace_1';

--创建用户joe。
-- 来源: 2979_CREATE TABLESPACE
CREATE ROLE joe IDENTIFIED BY ' ******** ';

--创建用户jay。
-- 来源: 2979_CREATE TABLESPACE
CREATE ROLE jay IDENTIFIED BY ' ******** ';

--创建表空间，且所有者指定为用户joe。
-- 来源: 2979_CREATE TABLESPACE
CREATE TABLESPACE ds_location2 OWNER joe RELATIVE LOCATION 'tablespace/tablespace_2';

--把表空间ds_location1重命名为ds_location3。
-- 来源: 2979_CREATE TABLESPACE
ALTER TABLESPACE ds_location1 RENAME TO ds_location3;

--改变表空间ds_location2的所有者。
-- 来源: 2979_CREATE TABLESPACE
ALTER TABLESPACE ds_location2 OWNER TO jay;

--删除表空间。
-- 来源: 2979_CREATE TABLESPACE
DROP TABLESPACE ds_location2;

-- 来源: 2979_CREATE TABLESPACE
DROP TABLESPACE ds_location3;

--删除用户。
-- 来源: 2979_CREATE TABLESPACE
DROP ROLE joe;

-- 来源: 2979_CREATE TABLESPACE
DROP ROLE jay;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE list_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY LIST ( month_code ) SUBPARTITION BY LIST ( dept_code ) ( PARTITION p_201901 VALUES ( '201902' ) ( SUBPARTITION p_201901_a VALUES ( '1' ), SUBPARTITION p_201901_b VALUES ( '2' ) ), PARTITION p_201902 VALUES ( '201903' ) ( SUBPARTITION p_201902_a VALUES ( '1' ), SUBPARTITION p_201902_b VALUES ( '2' ) ) );

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE list_list ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE list_hash ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY LIST ( month_code ) SUBPARTITION BY HASH ( dept_code ) ( PARTITION p_201901 VALUES ( '201902' ) ( SUBPARTITION p_201901_a , SUBPARTITION p_201901_b ), PARTITION p_201902 VALUES ( '201903' ) ( SUBPARTITION p_201902_a , SUBPARTITION p_201902_b ) );

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE list_hash ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE list_range ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY LIST ( month_code ) SUBPARTITION BY RANGE ( dept_code ) ( PARTITION p_201901 VALUES ( '201902' ) ( SUBPARTITION p_201901_a VALUES less than ( '4' ), SUBPARTITION p_201901_b VALUES less than ( '6' ) ), PARTITION p_201902 VALUES ( '201903' ) ( SUBPARTITION p_201902_a VALUES less than ( '3' ), SUBPARTITION p_201902_b VALUES less than ( '6' ) ) );

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE list_range ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE range_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE ( month_code ) SUBPARTITION BY LIST ( dept_code ) ( PARTITION p_201901 VALUES LESS THAN ( '201903' ) ( SUBPARTITION p_201901_a VALUES ( '1' ), SUBPARTITION p_201901_b VALUES ( '2' ) ), PARTITION p_201902 VALUES LESS THAN ( '201904' ) ( SUBPARTITION p_201902_a VALUES ( '1' ), SUBPARTITION p_201902_b VALUES ( '2' ) ) );

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE range_list ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE range_hash ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE ( month_code ) SUBPARTITION BY HASH ( dept_code ) ( PARTITION p_201901 VALUES LESS THAN ( '201903' ) ( SUBPARTITION p_201901_a , SUBPARTITION p_201901_b ), PARTITION p_201902 VALUES LESS THAN ( '201904' ) ( SUBPARTITION p_201902_a , SUBPARTITION p_201902_b ) );

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE range_hash ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE range_range ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE ( month_code ) SUBPARTITION BY RANGE ( dept_code ) ( PARTITION p_201901 VALUES LESS THAN ( '201903' ) ( SUBPARTITION p_201901_a VALUES LESS THAN ( '2' ), SUBPARTITION p_201901_b VALUES LESS THAN ( '3' ) ), PARTITION p_201902 VALUES LESS THAN ( '201904' ) ( SUBPARTITION p_201902_a VALUES LESS THAN ( '2' ), SUBPARTITION p_201902_b VALUES LESS THAN ( '3' ) ) );

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE range_range ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE hash_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY hash ( month_code ) SUBPARTITION BY LIST ( dept_code ) ( PARTITION p_201901 ( SUBPARTITION p_201901_a VALUES ( '1' ), SUBPARTITION p_201901_b VALUES ( '2' ) ), PARTITION p_201902 ( SUBPARTITION p_201902_a VALUES ( '1' ), SUBPARTITION p_201902_b VALUES ( '2' ) ) );

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE hash_list ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE hash_hash ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY hash ( month_code ) SUBPARTITION BY hash ( dept_code ) ( PARTITION p_201901 ( SUBPARTITION p_201901_a , SUBPARTITION p_201901_b ), PARTITION p_201902 ( SUBPARTITION p_201902_a , SUBPARTITION p_201902_b ) );

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE hash_hash ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE hash_range ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY hash ( month_code ) SUBPARTITION BY range ( dept_code ) ( PARTITION p_201901 ( SUBPARTITION p_201901_a VALUES LESS THAN ( '2' ), SUBPARTITION p_201901_b VALUES LESS THAN ( '3' ) ), PARTITION p_201902 ( SUBPARTITION p_201902_a VALUES LESS THAN ( '2' ), SUBPARTITION p_201902_b VALUES LESS THAN ( '3' ) ) );

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE hash_range ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE range_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY LIST (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a VALUES ('1'), SUBPARTITION p_201901_b VALUES ('2') ), PARTITION p_201902 VALUES LESS THAN( '201910' ) ( SUBPARTITION p_201902_a VALUES ('1'), SUBPARTITION p_201902_b VALUES ('2') ) );

--参数sql_compatibility='B'时，可指定多分区删除数据
-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE DATABASE db dbcompatibility 'B';

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE range_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY LIST (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a VALUES ('1'), SUBPARTITION p_201901_b VALUES ('2') ), PARTITION p_201902 VALUES LESS THAN( '201910' ) ( SUBPARTITION p_201902_a VALUES ('1'), SUBPARTITION p_201902_b VALUES ('2') ) );

--删除数据库
-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP DATABASE db;

--指定分区merge into数据
-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE newrange_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY LIST (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a VALUES ('1'), SUBPARTITION p_201901_b VALUES ('2') ), PARTITION p_201902 VALUES LESS THAN( '201910' ) ( SUBPARTITION p_201902_a VALUES ('1'), SUBPARTITION p_201902_b VALUES ('2') ) );

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE range_list;

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE newrange_list;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE list_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY LIST ( month_code ) SUBPARTITION BY LIST ( dept_code ) ( PARTITION p_201901 VALUES ( '201902' ) ( SUBPARTITION p_201901_a VALUES ( '1' ), SUBPARTITION p_201901_b VALUES ( default ) ), PARTITION p_201902 VALUES ( '201903' ) ( SUBPARTITION p_201902_a VALUES ( '1' ), SUBPARTITION p_201902_b VALUES ( '2' ) ) );

-- 来源: 2980_CREATE TABLE SUBPARTITION
alter table list_list truncate partition p_201901 ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
alter table list_list truncate partition p_201902 ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
alter table list_list truncate subpartition p_201901_a ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
alter table list_list truncate subpartition p_201901_b ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
alter table list_list truncate subpartition p_201902_a ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
alter table list_list truncate subpartition p_201902_b ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE list_list ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE list_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY LIST ( month_code ) SUBPARTITION BY LIST ( dept_code ) ( PARTITION p_201901 VALUES ( '201902' ) ( SUBPARTITION p_201901_a VALUES ( '1' ), SUBPARTITION p_201901_b VALUES ( default ) ), PARTITION p_201902 VALUES ( '201903' ) ( SUBPARTITION p_201902_a VALUES ( '1' ), SUBPARTITION p_201902_b VALUES ( default ) ) );

-- 来源: 2980_CREATE TABLE SUBPARTITION
alter table list_list split subpartition p_201901_b VALUES ( 2 ) into ( subpartition p_201901_b , subpartition p_201901_c );

-- 来源: 2980_CREATE TABLE SUBPARTITION
alter table list_list split subpartition p_201902_b VALUES ( 3 ) into ( subpartition p_201902_b , subpartition p_201902_c );

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE list_list ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
ALTER DATABASE set ilm = on ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE ilm_subpart ( a int , b int ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION PARTITION BY RANGE ( a ) SUBPARTITION BY RANGE ( b ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ( SUBPARTITION p1_s1 VALUES LESS THAN ( 10 ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION , SUBPARTITION p1_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p1_s3 VALUES LESS THAN ( 30 )), PARTITION p2 VALUES LESS THAN ( 20 ) ( SUBPARTITION p2_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p2_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p2_s3 VALUES LESS THAN ( 30 )), PARTITION p3 VALUES LESS THAN ( 30 ) ( SUBPARTITION p3_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p3_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p3_s3 VALUES LESS THAN ( 30 )));

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE ilm_subpart ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
ALTER DATABASE set ilm = on ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE ilm_subpart ( a int , b int ) PARTITION BY RANGE ( a ) SUBPARTITION BY RANGE ( b ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ( SUBPARTITION p1_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p1_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p1_s3 VALUES LESS THAN ( 30 )), PARTITION p2 VALUES LESS THAN ( 20 ) ( SUBPARTITION p2_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p2_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p2_s3 VALUES LESS THAN ( 30 )), PARTITION p3 VALUES LESS THAN ( 30 ) ( SUBPARTITION p3_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p3_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p3_s3 VALUES LESS THAN ( 30 )));

-- 来源: 2980_CREATE TABLE SUBPARTITION
ALTER TABLE ilm_subpart MODIFY SUBPARTITION p2_s1 ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
ALTER TABLE ilm_subpart MODIFY SUBPARTITION p2_s1 ILM DISABLE_ALL ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
ALTER TABLE ilm_subpart MODIFY SUBPARTITION p2_s1 ILM ENABLE_ALL ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
ALTER TABLE ilm_subpart MODIFY SUBPARTITION p2_s1 ILM DELETE_ALL ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE ilm_subpart ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
ALTER DATABASE set ilm = on ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE ilm_subpart ( a int , b int ) PARTITION BY RANGE ( a ) SUBPARTITION BY RANGE ( b ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ( SUBPARTITION p1_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p1_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p1_s3 VALUES LESS THAN ( 30 )), PARTITION p2 VALUES LESS THAN ( 20 ) ( SUBPARTITION p2_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p2_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p2_s3 VALUES LESS THAN ( 30 )), PARTITION p3 VALUES LESS THAN ( 30 ) ( SUBPARTITION p3_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p3_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p3_s3 VALUES LESS THAN ( 30 )));

-- 来源: 2980_CREATE TABLE SUBPARTITION
ALTER TABLE ilm_subpart MODIFY PARTITION p2 ADD SUBPARTITION p2_s4 VALUES LESS THAN ( 40 ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE ilm_subpart ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
ALTER DATABASE set ilm = on ;

-- 来源: 2980_CREATE TABLE SUBPARTITION
CREATE TABLE ilm_subpart ( a int , b int ) PARTITION BY RANGE ( a ) SUBPARTITION BY RANGE ( b ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ( SUBPARTITION p1_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p1_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p1_s3 VALUES LESS THAN ( 30 )), PARTITION p2 VALUES LESS THAN ( 20 ) ( SUBPARTITION p2_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p2_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p2_s3 VALUES LESS THAN ( 30 )), PARTITION p3 VALUES LESS THAN ( 30 ) ( SUBPARTITION p3_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p3_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p3_s3 VALUES LESS THAN ( 30 )));

-- 来源: 2980_CREATE TABLE SUBPARTITION
ALTER TABLE ilm_subpart SPLIT SUBPARTITION p1_s2 AT ( '15' ) INTO ( SUBPARTITION p1_s2_1 ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION , SUBPARTITION p1_s2_2 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
DROP TABLE ilm_subpart ;

-- 来源: 2981_CREATE TEXT SEARCH CONFIGURATION
CREATE TEXT SEARCH CONFIGURATION ngram2 (parser=ngram) WITH (gram_size = 2, grapsymbol_ignore = false);

--创建文本搜索配置。
-- 来源: 2981_CREATE TEXT SEARCH CONFIGURATION
CREATE TEXT SEARCH CONFIGURATION ngram3 (copy=ngram2) WITH (gram_size = 2, grapsymbol_ignore = false);

--添加类型映射。
-- 来源: 2981_CREATE TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION ngram2 ADD MAPPING FOR multisymbol WITH simple;

--创建用户joe。
-- 来源: 2981_CREATE TEXT SEARCH CONFIGURATION
CREATE USER joe IDENTIFIED BY ' ******** ';

--修改文本搜索配置的所有者。
-- 来源: 2981_CREATE TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION ngram2 OWNER TO joe;

--修改文本搜索配置的schema。
-- 来源: 2981_CREATE TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION ngram2 SET SCHEMA joe;

--重命名文本搜索配置。
-- 来源: 2981_CREATE TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION joe.ngram2 RENAME TO ngram_2;

--删除类型映射。
-- 来源: 2981_CREATE TEXT SEARCH CONFIGURATION
ALTER TEXT SEARCH CONFIGURATION joe.ngram_2 DROP MAPPING IF EXISTS FOR multisymbol;

--删除文本搜索配置。
-- 来源: 2981_CREATE TEXT SEARCH CONFIGURATION
DROP TEXT SEARCH CONFIGURATION joe.ngram_2;

-- 来源: 2981_CREATE TEXT SEARCH CONFIGURATION
DROP TEXT SEARCH CONFIGURATION ngram3;

--删除Schema及用户joe。
-- 来源: 2981_CREATE TEXT SEARCH CONFIGURATION
DROP SCHEMA IF EXISTS joe CASCADE;

-- 来源: 2981_CREATE TEXT SEARCH CONFIGURATION
DROP ROLE IF EXISTS joe;

-- 来源: 2983_CREATE TRIGGER
CREATE TABLE test_trigger_src_tbl(id1 INT, id2 INT, id3 INT);

-- 来源: 2983_CREATE TRIGGER
CREATE TABLE test_trigger_des_tbl(id1 INT, id2 INT, id3 INT);

--创建触发器函数
-- 来源: 2983_CREATE TRIGGER
CREATE OR REPLACE FUNCTION tri_insert_func() RETURNS TRIGGER AS $$ DECLARE BEGIN INSERT INTO test_trigger_des_tbl VALUES(NEW.id1, NEW.id2, NEW.id3);

-- 来源: 2983_CREATE TRIGGER
CREATE OR REPLACE FUNCTION tri_update_func() RETURNS TRIGGER AS $$ DECLARE BEGIN UPDATE test_trigger_des_tbl SET id3 = NEW.id3 WHERE id1=OLD.id1;

-- 来源: 2983_CREATE TRIGGER
CREATE OR REPLACE FUNCTION TRI_DELETE_FUNC() RETURNS TRIGGER AS $$ DECLARE BEGIN DELETE FROM test_trigger_des_tbl WHERE id1=OLD.id1;

--创建INSERT触发器
-- 来源: 2983_CREATE TRIGGER
CREATE TRIGGER insert_trigger BEFORE INSERT ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_insert_func();

--创建UPDATE触发器
-- 来源: 2983_CREATE TRIGGER
CREATE TRIGGER update_trigger AFTER UPDATE ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_update_func();

--创建DELETE触发器
-- 来源: 2983_CREATE TRIGGER
CREATE TRIGGER delete_trigger BEFORE DELETE ON test_trigger_src_tbl FOR EACH ROW EXECUTE PROCEDURE tri_delete_func();

--修改触发器
-- 来源: 2983_CREATE TRIGGER
ALTER TRIGGER delete_trigger ON test_trigger_src_tbl RENAME TO delete_trigger_renamed;

--禁用insert_trigger触发器
-- 来源: 2983_CREATE TRIGGER
ALTER TABLE test_trigger_src_tbl DISABLE TRIGGER insert_trigger;

--禁用当前表上所有触发器
-- 来源: 2983_CREATE TRIGGER
ALTER TABLE test_trigger_src_tbl DISABLE TRIGGER ALL;

--删除触发器
-- 来源: 2983_CREATE TRIGGER
DROP TRIGGER insert_trigger ON test_trigger_src_tbl;

-- 来源: 2983_CREATE TRIGGER
DROP TRIGGER update_trigger ON test_trigger_src_tbl;

-- 来源: 2983_CREATE TRIGGER
DROP TRIGGER delete_trigger_renamed ON test_trigger_src_tbl;

--删除函数。
-- 来源: 2983_CREATE TRIGGER
DROP FUNCTION tri_insert_func();

-- 来源: 2983_CREATE TRIGGER
DROP FUNCTION tri_update_func();

-- 来源: 2983_CREATE TRIGGER
DROP FUNCTION tri_delete_func();

--删除源表及触发表。
-- 来源: 2983_CREATE TRIGGER
DROP TABLE test_trigger_src_tbl;

-- 来源: 2983_CREATE TRIGGER
DROP TABLE test_trigger_des_tbl;

-- 来源: 2984_CREATE TYPE
CREATE TYPE compfoo AS (f1 int, f2 text);

-- 来源: 2984_CREATE TYPE
CREATE TABLE t1_compfoo(a int, b compfoo);

-- 来源: 2984_CREATE TYPE
CREATE TABLE t2_compfoo(a int, b compfoo);

--重命名数据类型。
-- 来源: 2984_CREATE TYPE
ALTER TYPE compfoo RENAME TO compfoo1;

--要改变一个用户定义类型compfoo1的所有者为usr1。
-- 来源: 2984_CREATE TYPE
CREATE USER usr1 PASSWORD ' ******** ';

-- 来源: 2984_CREATE TYPE
ALTER TYPE compfoo1 OWNER TO usr1;

--把用户定义类型compfoo1的模式改变为usr1。
-- 来源: 2984_CREATE TYPE
ALTER TYPE compfoo1 SET SCHEMA usr1;

--给一个数据类型增加一个新的属性。
-- 来源: 2984_CREATE TYPE
ALTER TYPE usr1.compfoo1 ADD ATTRIBUTE f3 int;

--删除compfoo1类型。
-- 来源: 2984_CREATE TYPE
DROP TYPE usr1.compfoo1 CASCADE;

--删除相关表和用户。
-- 来源: 2984_CREATE TYPE
DROP TABLE t1_compfoo;

-- 来源: 2984_CREATE TYPE
DROP TABLE t2_compfoo;

-- 来源: 2984_CREATE TYPE
DROP SCHEMA usr1;

-- 来源: 2984_CREATE TYPE
DROP USER usr1;

--创建一个枚举类型。
-- 来源: 2984_CREATE TYPE
CREATE TYPE bugstatus AS ENUM ('create', 'modify', 'closed');

--添加一个标签值。
-- 来源: 2984_CREATE TYPE
ALTER TYPE bugstatus ADD VALUE IF NOT EXISTS 'regress' BEFORE 'closed';

--重命名一个标签值。
-- 来源: 2984_CREATE TYPE
ALTER TYPE bugstatus RENAME VALUE 'create' TO 'new';

--创建一个集合类型
-- 来源: 2984_CREATE TYPE
CREATE TYPE bugstatus_table AS TABLE OF bugstatus;

--删除集合类型及枚举类型。
-- 来源: 2984_CREATE TYPE
DROP TYPE bugstatus_table;

-- 来源: 2984_CREATE TYPE
DROP TYPE bugstatus CASCADE;

-- 来源: 2985_CREATE USER
CREATE USER jim PASSWORD ' ******** ';

--下面语句与上面的等价。
-- 来源: 2985_CREATE USER
CREATE USER kim IDENTIFIED BY ' ******** ';

--如果创建有“创建数据库”权限的用户，则需要加CREATEDB关键字。
-- 来源: 2985_CREATE USER
CREATE USER dim CREATEDB PASSWORD ' ******** ';

--将用户jim的登录密码由 ******** 修改为**********。
-- 来源: 2985_CREATE USER
ALTER USER jim IDENTIFIED BY '**********' REPLACE ' ******** ';

--为用户jim追加CREATEROLE权限。
-- 来源: 2985_CREATE USER
ALTER USER jim CREATEROLE;

--将enable_seqscan的值设置为on， 设置成功后，在下一会话中生效。
-- 来源: 2985_CREATE USER
ALTER USER jim SET enable_seqscan TO on;

--重置jim的enable_seqscan参数。
-- 来源: 2985_CREATE USER
ALTER USER jim RESET enable_seqscan;

--锁定jim账户。
-- 来源: 2985_CREATE USER
ALTER USER jim ACCOUNT LOCK;

--删除用户。
-- 来源: 2985_CREATE USER
DROP USER kim CASCADE;

-- 来源: 2985_CREATE USER
DROP USER jim CASCADE;

-- 来源: 2985_CREATE USER
DROP USER dim CASCADE;

-- 来源: 2986_CREATE USER MAPPING
CREATE ROLE bob PASSWORD '********';

-- 创建外部服务器
-- 来源: 2986_CREATE USER MAPPING
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

-- 创建USER MAPPING。
-- 来源: 2986_CREATE USER MAPPING
CREATE USER MAPPING FOR bob SERVER my_server OPTIONS (user 'bob', password '********');

-- 修改USER MAPPING。
-- 来源: 2986_CREATE USER MAPPING
ALTER USER MAPPING FOR bob SERVER my_server OPTIONS (SET password '********');

-- 删除USER MAPPING。
-- 来源: 2986_CREATE USER MAPPING
DROP USER MAPPING FOR bob SERVER my_server;

-- 删除外部服务器。
-- 来源: 2986_CREATE USER MAPPING
DROP SERVER my_server;

-- 删除角色。
-- 来源: 2986_CREATE USER MAPPING
DROP ROLE bob;

-- 来源: 2987_CREATE VIEW
CREATE TABLE test_tb1(col1 int, col2 int);

--创建一个col1小于5的视图。
-- 来源: 2987_CREATE VIEW
CREATE VIEW test_v1 AS SELECT * FROM test_tb1 WHERE col1 < 3;

--删除表和视图。
-- 来源: 2987_CREATE VIEW
DROP VIEW test_v1;

-- 来源: 2987_CREATE VIEW
DROP TABLE test_tb1;

-- 来源: 2987_CREATE VIEW
CREATE TABLE test_tb2(c1 int, c2 int);

-- 来源: 2987_CREATE VIEW
CREATE TEMP VIEW test_v2 AS SELECT * FROM test_tb2;

--删除视图和表。
-- 来源: 2987_CREATE VIEW
DROP VIEW test_v2 ;

-- 来源: 2987_CREATE VIEW
DROP TABLE test_tb2;

-- 来源: 2988_CREATE WEAK PASSWORD DICTIONARY
CREATE WEAK PASSWORD DICTIONARY WITH VALUES ('password1');

--向gs_global_config系统表中插入多个弱口令。
-- 来源: 2988_CREATE WEAK PASSWORD DICTIONARY
CREATE WEAK PASSWORD DICTIONARY WITH VALUES ('password2'),('password3');

--清空gs_global_config系统表中所有弱口令。
-- 来源: 2988_CREATE WEAK PASSWORD DICTIONARY
DROP WEAK PASSWORD DICTIONARY;

-- 来源: 2993_DELETE
CREATE SCHEMA tpcds;

--创建表tpcds.customer_address。
-- 来源: 2993_DELETE
CREATE TABLE tpcds.customer_address ( ca_address_sk INTEGER NOT NULL, ca_address_id CHARACTER(16) NOT NULL, ca_street_number INTEGER , ca_street_name CHARACTER (20) );

--创建表 tpcds. customer_address_bak。
-- 来源: 2993_DELETE
CREATE TABLE tpcds. customer_address_bak AS TABLE tpcds. customer_address;

--删除 tpcds. customer_address_bak表。
-- 来源: 2993_DELETE
DROP TABLE tpcds. customer_address_bak;

--删除tpcds.customer_address表。
-- 来源: 2993_DELETE
DROP TABLE tpcds.customer_address;

--删除SCHEMA。
-- 来源: 2993_DELETE
DROP SCHEMA tpcds CASCADE;

-- 来源: 2994_DO
CREATE USER webuser PASSWORD ' ******** ';

--删除用户webuser。
-- 来源: 2994_DO
DROP USER webuser CASCADE;

-- 来源: 2995_DROP AGGREGATE
CREATE OR REPLACE FUNCTION int_add(int,int) returns int as $BODY$ declare begin return $1 + $2;

-- 删除自定义函数
-- 来源: 2995_DROP AGGREGATE
DROP FUNCTION int_add(int,int);

-- 来源: 3000_DROP DATA SOURCE
CREATE DATA SOURCE ds_tst1;

--删除Data Source对象。
-- 来源: 3000_DROP DATA SOURCE
DROP DATA SOURCE ds_tst1 CASCADE;

-- 来源: 3000_DROP DATA SOURCE
DROP DATA SOURCE IF EXISTS ds_tst1 RESTRICT;

-- 来源: 3001_DROP DIRECTORY
CREATE OR REPLACE DIRECTORY dir as '/tmp/';

--删除目录。
-- 来源: 3001_DROP DIRECTORY
DROP DIRECTORY dir;

-- 来源: 3003_DROP EXTENSION
DROP EXTENSION plpgsql ;

-- 来源: 3010_DROP MASKING POLICY
DROP MASKING POLICY IF EXISTS maskpol1 ;

-- 来源: 3010_DROP MASKING POLICY
DROP MASKING POLICY IF EXISTS maskpol1 , maskpol2 , maskpol3 ;

-- 来源: 3011_DROP MATERIALIZED VIEW
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建名为my_mv的物化视图。
-- 来源: 3011_DROP MATERIALIZED VIEW
CREATE MATERIALIZED VIEW my_mv AS SELECT * FROM my_table;

--删除名为my_mv的物化视图。
-- 来源: 3011_DROP MATERIALIZED VIEW
DROP MATERIALIZED VIEW my_mv;

--删除表。
-- 来源: 3011_DROP MATERIALIZED VIEW
DROP TABLE my_table;

-- 来源: 3014_DROP OWNED
CREATE USER jim PASSWORD '********' ;

-- 来源: 3014_DROP OWNED
DROP OWNED BY jim ;

-- 来源: 3014_DROP OWNED
DROP USER jim ;

-- 来源: 3015_DROP PACKAGE
CREATE OR REPLACE PACKAGE PCK1 IS a int;

--删除PACKAGE。
-- 来源: 3015_DROP PACKAGE
DROP PACKAGE PCK1;

-- 来源: 3020_DROP ROW LEVEL SECURITY POLICY
CREATE TABLE all_data(id int, role varchar(100), data varchar(100));

--删除数据表all_data。
-- 来源: 3020_DROP ROW LEVEL SECURITY POLICY
DROP TABLE all_data;

-- 来源: 3021_DROP RULE
CREATE TABLE def_test ( c1 int4 DEFAULT 5, c2 text DEFAULT 'initial_default' );

-- 来源: 3021_DROP RULE
CREATE VIEW def_view_test AS SELECT * FROM def_test;

--删除表def_test、视图def_view_test
-- 来源: 3021_DROP RULE
DROP VIEW def_view_test;

-- 来源: 3021_DROP RULE
DROP TABLE def_test;

-- 来源: 3024_DROP SECURITY LABEL
CREATE SECURITY LABEL sec_label 'L1:G4' ;

-- 来源: 3024_DROP SECURITY LABEL
DROP SECURITY LABEL sec_label ;

-- 来源: 3025_DROP SEQUENCE
CREATE SEQUENCE serial START 101;

--删除序列。
-- 来源: 3025_DROP SEQUENCE
DROP SEQUENCE serial;

-- 来源: 3026_DROP SERVER
CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

--删除my_server。
-- 来源: 3026_DROP SERVER
DROP SERVER my_server;

-- 来源: 3032_DROP TEXT SEARCH DICTIONARY
CREATE TEXT SEARCH DICTIONARY english ( TEMPLATE = simple );

--删除词典english。
-- 来源: 3032_DROP TEXT SEARCH DICTIONARY
DROP TEXT SEARCH DICTIONARY english;

-- 来源: 3040_EXECUTE
CREATE SCHEMA tpcds;

--创建表reason。
-- 来源: 3040_EXECUTE
CREATE TABLE tpcds. reason ( CD_DEMO_SK INTEGER NOT NULL, CD_GENDER character(16) , CD_MARITAL_STATUS character(100) );

--创建表reason_t1。
-- 来源: 3040_EXECUTE
CREATE TABLE tpcds. reason_t1 AS TABLE tpcds.reason;

--删除表reason和reason_t1。
-- 来源: 3040_EXECUTE
DROP TABLE tpcds. reason;

-- 来源: 3040_EXECUTE
DROP TABLE tpcds. reason_t1;

--删除SCHEMA。
-- 来源: 3040_EXECUTE
DROP SCHEMA tpcds CASCADE;

-- 来源: 3043_EXPLAIN
CREATE TABLE student(id int, name char(20));

-- 来源: 3043_EXPLAIN
CREATE SCHEMA tpcds;

--创建表tpcds.customer_address。
-- 来源: 3043_EXPLAIN
CREATE TABLE tpcds.customer_address ( ca_address_sk INTEGER NOT NULL, ca_address_id CHARACTER(16) NOT NULL );

--创建一个表 tpcds. customer_address_p1。
-- 来源: 3043_EXPLAIN
CREATE TABLE tpcds. customer_address_p1 AS TABLE tpcds. customer_address;

--创建一个二级分区表。
-- 来源: 3043_EXPLAIN
CREATE TABLE range_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY LIST (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a values ('1'), SUBPARTITION p_201901_b values ('2') ), PARTITION p_201902 VALUES LESS THAN( '201910' ) ( SUBPARTITION p_201902_a values ('1'), SUBPARTITION p_201902_b values ('2') ) );

--删除表 tpcds. customer_address_p1。
-- 来源: 3043_EXPLAIN
DROP TABLE tpcds. customer_address_p1;

--删除表tpcds.customer_address。
-- 来源: 3043_EXPLAIN
DROP TABLE tpcds.customer_address;

--删除表range_list。
-- 来源: 3043_EXPLAIN
DROP TABLE range_list;

--删除SCHEMA。
-- 来源: 3043_EXPLAIN
DROP SCHEMA tpcds CASCADE;

-- 来源: 3043_EXPLAIN
CREATE TABLE tb_a(c1 int);

-- 来源: 3043_EXPLAIN
CREATE TABLE tb_b AS SELECT * FROM tb_a;

--删除表tb_a，tb_b。
-- 来源: 3043_EXPLAIN
DROP TABLE tb_a;

-- 来源: 3043_EXPLAIN
DROP TABLE tb_b;

-- 来源: 3044_EXPLAIN PLAN
CREATE TABLE foo1(f1 int, f2 text, f3 text[]);

-- 来源: 3044_EXPLAIN PLAN
CREATE TABLE foo2(f1 int, f2 text, f3 text[]);

-- 来源: 3044_EXPLAIN PLAN
DROP TABLE foo1 ;

-- 来源: 3044_EXPLAIN PLAN
DROP TABLE foo2 ;

-- 来源: 3046_FETCH
CREATE SCHEMA tpcds;

--创建表tpcds.customer_address。
-- 来源: 3046_FETCH
CREATE TABLE tpcds.customer_address ( ca_address_sk INTEGER NOT NULL, ca_address_id CHARACTER(16) NOT NULL, ca_street_number INTEGER , ca_street_name CHARACTER (20) );

--删除表tpcds.customer_address。
-- 来源: 3046_FETCH
DROP TABLE tpcds.customer_address;

--删除SCHEMA。
-- 来源: 3046_FETCH
DROP SCHEMA tpcds CASCADE;

-- 来源: 3048_GRANT
CREATE USER joe PASSWORD ' ******** ';

-- 来源: 3048_GRANT
CREATE SCHEMA tpcds;

-- 来源: 3048_GRANT
CREATE TABLE tpcds.reason ( r_reason_sk INTEGER NOT NULL, r_reason_id CHAR(16) NOT NULL, r_reason_desc VARCHAR(20) );

-- 来源: 3048_GRANT
CREATE DATABASE testdb;

-- 来源: 3048_GRANT
CREATE ROLE tpcds_manager PASSWORD ' ******** ';

-- 来源: 3048_GRANT
CREATE TABLESPACE tpcds_tbspc RELATIVE LOCATION 'tablespace/tablespace_1';

-- 来源: 3048_GRANT
CREATE or replace FUNCTION tpcds.fun1() RETURN boolean AS BEGIN SELECT current_user;

-- 来源: 3048_GRANT
CREATE ROLE manager PASSWORD ' ******** ';

-- 来源: 3048_GRANT
CREATE ROLE senior_manager PASSWORD ' ******** ';

-- 来源: 3048_GRANT
DROP USER manager;

-- 来源: 3048_GRANT
DROP DATABASE testdb;

-- 来源: 3054_INSERT
CREATE SCHEMA tpcds;

--创建表tpcds.reason。
-- 来源: 3054_INSERT
CREATE TABLE tpcds.reason ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--创建表 tpcds. reason_t2。
-- 来源: 3054_INSERT
CREATE TABLE tpcds. reason_t2 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--对表创建唯一索引。
-- 来源: 3054_INSERT
CREATE UNIQUE INDEX reason_t2_u_index ON tpcds. reason_t2(r_reason_sk);

--删除表 tpcds. reason_t2。
-- 来源: 3054_INSERT
DROP TABLE tpcds. reason_t2;

--删除表tpcds.reason。
-- 来源: 3054_INSERT
DROP TABLE tpcds.reason;

--删除SCHEMA。
-- 来源: 3054_INSERT
DROP SCHEMA tpcds CASCADE;

-- 来源: 3056_LOCK
CREATE SCHEMA tpcds;

--创建表tpcds.reason。
-- 来源: 3056_LOCK
CREATE TABLE tpcds.reason ( r_reason_sk INTEGER NOT NULL, r_reason_id CHAR(16) NOT NULL, r_reason_desc INTEGER );

--在执行删除操作时对一个有主键的表进行 SHARE ROW EXCLUSIVE 锁。
-- 来源: 3056_LOCK
CREATE TABLE tpcds. reason_t1 AS TABLE tpcds. reason;

--删除表 tpcds. reason_t1。
-- 来源: 3056_LOCK
DROP TABLE tpcds. reason_t1;

--删除表。
-- 来源: 3056_LOCK
DROP TABLE tpcds.reason;

--删除SCHEMA。
-- 来源: 3056_LOCK
DROP SCHEMA tpcds CASCADE;

-- 来源: 3060_MERGE INTO
CREATE TABLE products ( product_id INTEGER, product_name VARCHAR2(60), category VARCHAR2(60) );

-- 来源: 3060_MERGE INTO
CREATE TABLE newproducts ( product_id INTEGER, product_name VARCHAR2(60), category VARCHAR2(60) );

-- 删除表
-- 来源: 3060_MERGE INTO
DROP TABLE products;

-- 来源: 3060_MERGE INTO
DROP TABLE newproducts;

-- 来源: 3061_MOVE
CREATE SCHEMA tpcds;

--创建表tpcds.reason。
-- 来源: 3061_MOVE
CREATE TABLE tpcds.reason ( r_reason_sk INTEGER NOT NULL, r_reason_id CHAR(16) NOT NULL, r_reason_desc VARCHAR(40) );

--删除表。
-- 来源: 3061_MOVE
DROP TABLE tpcds.reason;

--删除SCHEMA。
-- 来源: 3061_MOVE
DROP SCHEMA tpcds CASCADE;

-- 来源: 3063_PREDICT BY
CREATE TABLE houses ( id INTEGER, tax INTEGER, bedroom INTEGER, bath DOUBLE PRECISION, price INTEGER, size INTEGER, lot INTEGER, mark text );

--训练模型
-- 来源: 3063_PREDICT BY
CREATE MODEL price_model USING logistic_regression FEATURES size, lot TARGET mark FROM HOUSES WITH learning_rate=0.88, max_iterations=default;

--删除模型
-- 来源: 3063_PREDICT BY
DROP MODEL price_model;

--删除表
-- 来源: 3063_PREDICT BY
DROP TABLE houses;

-- 来源: 3066_PURGE
CREATE ROLE tpcds IDENTIFIED BY '*********';

-- 创建表空间reason_table_space
-- 来源: 3066_PURGE
CREATE TABLESPACE REASON_TABLE_SPACE1 owner tpcds RELATIVE location 'tablespace/tsp_reason1';

-- 创建SCHEMA。
-- 来源: 3066_PURGE
CREATE SCHEMA tpcds;

-- 在表空间创建表tpcds.reason_t
-- 来源: 3066_PURGE
CREATE TABLE tpcds.reason_t1 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) tablespace reason_table_space1;

-- 在表空间创建表tpcds.reason_t
-- 来源: 3066_PURGE
CREATE TABLE tpcds.reason_t2 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) tablespace reason_table_space1;

-- 在表空间创建表tpcds.reason_t
-- 来源: 3066_PURGE
CREATE TABLE tpcds.reason_t3 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) tablespace reason_table_space1;

-- 对表tpcds.reason_t1创建索引
-- 来源: 3066_PURGE
CREATE INDEX index_t1 on tpcds.reason_t1(r_reason_id);

-- 来源: 3066_PURGE
DROP TABLE tpcds.reason_t1;

-- 来源: 3066_PURGE
DROP TABLE tpcds.reason_t2;

-- 来源: 3066_PURGE
DROP TABLE tpcds.reason_t3;

-- 删除SCHEMA。
-- 来源: 3066_PURGE
DROP SCHEMA tpcds CASCADE;

-- 来源: 3068_REASSIGN OWNED
CREATE USER jim PASSWORD '********' ;

-- 来源: 3068_REASSIGN OWNED
CREATE USER tom PASSWORD '********' ;

-- 来源: 3068_REASSIGN OWNED
DROP USER jim , tom CASCADE ;

-- 来源: 3069_REFRESH INCREMENTAL MATERIALIZED VIEW
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建增量物化视图。
-- 来源: 3069_REFRESH INCREMENTAL MATERIALIZED VIEW
CREATE INCREMENTAL MATERIALIZED VIEW my_imv AS SELECT * FROM my_table;

--删除增量物化视图。
-- 来源: 3069_REFRESH INCREMENTAL MATERIALIZED VIEW
DROP MATERIALIZED VIEW my_imv;

--删除表my_table。
-- 来源: 3069_REFRESH INCREMENTAL MATERIALIZED VIEW
DROP TABLE my_table;

-- 来源: 3070_REFRESH MATERIALIZED VIEW
CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建全量物化视图。
-- 来源: 3070_REFRESH MATERIALIZED VIEW
CREATE MATERIALIZED VIEW my_mv AS SELECT * FROM my_table;

--创建增量物化视图。
-- 来源: 3070_REFRESH MATERIALIZED VIEW
CREATE INCREMENTAL MATERIALIZED VIEW my_imv AS SELECT * FROM my_table;

--删除增量物化视图。
-- 来源: 3070_REFRESH MATERIALIZED VIEW
DROP MATERIALIZED VIEW my_imv;

--删除全量物化视图。
-- 来源: 3070_REFRESH MATERIALIZED VIEW
DROP MATERIALIZED VIEW my_mv;

--删除表my_table。
-- 来源: 3070_REFRESH MATERIALIZED VIEW
DROP TABLE my_table;

-- 来源: 3071_REINDEX
CREATE SCHEMA tpcds;

--创建表tpcds. customer。
-- 来源: 3071_REINDEX
CREATE TABLE tpcds.customer ( c_customer_sk INTEGER NOT NULL, c_customer_id CHAR(16) NOT NULL );

--创建一个行存表 tpcds. customer_t1，并在 tpcds. customer_t1表上的c_customer_sk字段创建索引。
-- 来源: 3071_REINDEX
CREATE TABLE tpcds. customer_t1 ( c_customer_sk integer not null, c_customer_id char(16) not null, c_current_cdemo_sk integer , c_current_hdemo_sk integer , c_current_addr_sk integer , c_first_shipto_date_sk integer , c_first_sales_date_sk integer , c_salutation char(10) , c_first_name char(20) , c_last_name char(30) , c_preferred_cust_flag char(1) , c_birth_day integer , c_birth_month integer , c_birth_year integer , c_birth_country varchar(20) , c_login char(13) , c_email_address char(50) , c_last_review_date char(10) ) WITH (orientation = row);

-- 来源: 3071_REINDEX
CREATE INDEX tpcds_customer_index1 ON tpcds. customer_t1 (c_customer_sk);

--删除 tpcds. customer_t1表。
-- 来源: 3071_REINDEX
DROP TABLE tpcds. customer_t1;

--删除表。
-- 来源: 3071_REINDEX
DROP TABLE tpcds.customer;

--删除SCHEMA。
-- 来源: 3071_REINDEX
DROP SCHEMA tpcds CASCADE;

-- 来源: 3072_RELEASE SAVEPOINT
CREATE SCHEMA tpcds;

--创建一个新表。
-- 来源: 3072_RELEASE SAVEPOINT
CREATE TABLE tpcds. table1(a int);

--删除表。
-- 来源: 3072_RELEASE SAVEPOINT
DROP TABLE tpcds. table1;

--删除SCHEMA。
-- 来源: 3072_RELEASE SAVEPOINT
DROP SCHEMA tpcds CASCADE;

-- 来源: 3073_REPLACE
CREATE TABLE test(f1 int primary key, f2 int, f3 int);

-- 来源: 3073_REPLACE
DROP TABLE test;

-- 来源: 3080_SAVEPOINT
CREATE TABLE table1(a int);

--删除表。
-- 来源: 3080_SAVEPOINT
DROP TABLE table1;

--创建一个新表。
-- 来源: 3080_SAVEPOINT
CREATE TABLE table2(a int);

--删除表。
-- 来源: 3080_SAVEPOINT
DROP TABLE table2;

-- 来源: 3081_SECURITY LABEL ON
CREATE SECURITY LABEL sec_label 'L1:G4' ;

-- 来源: 3081_SECURITY LABEL ON
CREATE TABLE tbl ( c1 int , c2 int );

-- 来源: 3081_SECURITY LABEL ON
CREATE USER bob WITH PASSWORD '********' ;

-- 来源: 3081_SECURITY LABEL ON
DROP SECURITY LABEL sec_label ;

-- 来源: 3081_SECURITY LABEL ON
DROP TABLE tbl ;

-- 来源: 3081_SECURITY LABEL ON
DROP USER bob ;

--创建自定义变量
-- 来源: 3082_SELECT
CREATE DATABASE user_var dbcompatibility 'b';

--删除数据库
-- 来源: 3082_SELECT
DROP DATABASE user_var;

-- 来源: 3082_SELECT
CREATE TABLE test(name varchar, id int, fatherid int);

-- 来源: 3082_SELECT
CREATE SCHEMA tpcds;

--创建表tpcds.reason。
-- 来源: 3082_SELECT
CREATE TABLE tpcds.reason ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--创建分区表 tpcds. reason_p
-- 来源: 3082_SELECT
CREATE TABLE tpcds. reason_p ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) PARTITION BY RANGE (r_reason_sk) ( partition P_05_BEFORE values less than (05), partition P_15 values less than (15), partition P_25 values less than (25), partition P_35 values less than (35), partition P_45_AFTER values less than (MAXVALUE) );

--创建表store_returns、customer
-- 来源: 3082_SELECT
CREATE TABLE tpcds.store_returns (sr_item_sk int, sr_customer_id varchar(50),sr_customer_sk int);

-- 来源: 3082_SELECT
CREATE TABLE tpcds.customer (c_item_sk int, c_customer_id varchar(50),c_customer_sk int);

--删除表。
-- 来源: 3082_SELECT
DROP TABLE tpcds. reason_p;

--闪回查询示例，使用闪回功能需要设置undo_retention_time参数
--创建表tpcds.time_table
-- 来源: 3082_SELECT
CREATE TABLE tpcds.time_table(idx integer, snaptime timestamp, snapcsn bigint, timeDesc character(100));

-- 来源: 3082_SELECT
DROP TABLE t ;

--UNPIVOT子句示例：将表p1的math列和phy列转置为（class，score）行
-- 来源: 3082_SELECT
CREATE TABLE p1(id int, math int, phy int);

--PIVOT子句示例：将表p2的（class，score）行转置为'MATH'列和 'PHY'列
-- 来源: 3082_SELECT
CREATE TABLE p2(id int, class varchar(10), score int);

-- 来源: 3082_SELECT
DROP TABLE p1;

-- 来源: 3082_SELECT
DROP TABLE p2;

--SKIP LOCKED示例
--step 1:创建astore表并插入数据
-- 来源: 3082_SELECT
CREATE TABLE skiplocked_astore(id int, info text) WITH (storage_type=astore);

--删除表。
-- 来源: 3082_SELECT
DROP TABLE tpcds.reason;

--删除SCHEMA。
-- 来源: 3082_SELECT
DROP SCHEMA tpcds CASCADE;

-- 来源: 3083_SELECT INTO
CREATE SCHEMA tpcds;

--创建表tpcds.reason。
-- 来源: 3083_SELECT INTO
CREATE TABLE tpcds.reason ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--删除 tpcds. reason_t1表。
-- 来源: 3083_SELECT INTO
DROP TABLE tpcds. reason_t1;

--删除表。
-- 来源: 3083_SELECT INTO
DROP TABLE tpcds.reason;

--删除SCHEMA。
-- 来源: 3083_SELECT INTO
DROP SCHEMA tpcds CASCADE;

--SET自定义用户变量的功能。
-- 来源: 3084_SET
CREATE DATABASE user_var dbcompatibility 'b';

--删除数据库。
-- 来源: 3084_SET
DROP DATABASE user_var;

-- 来源: 3084_SET
CREATE DATABASE test_set dbcompatibility 'B';

--删除数据库。
-- 来源: 3084_SET
DROP DATABASE test_set;

-- 来源: 3086_SET ROLE
CREATE ROLE paul IDENTIFIED BY ' ******** ';

--删除用户。
-- 来源: 3086_SET ROLE
DROP USER paul;

-- 来源: 3087_SET SESSION AUTHORIZATION
CREATE ROLE paul IDENTIFIED BY ' ******** ';

--删除用户。
-- 来源: 3087_SET SESSION AUTHORIZATION
DROP USER paul;

-- 来源: 3088_SET TRANSACTION
CREATE DATABASE mysql_compatible_db DBCOMPATIBILITY 'B';

-- 来源: 3088_SET TRANSACTION
DROP DATABASE mysql_compatible_db;

-- 来源: 3091_SHRINK
CREATE TABLE row_compression ( id int ) WITH (compresstype=2, compress_chunk_size = 512, compress_level = 1);

--删除表
-- 来源: 3091_SHRINK
DROP TABLE row_compression;

-- 来源: 3093_SNAPSHOT
CREATE TABLE t1 (id int, name varchar);

-- 来源: 3093_SNAPSHOT
DROP TABLE t1;

-- 来源: 3094_START TRANSACTION
CREATE SCHEMA tpcds;

--创建表 tpcds. reason。
-- 来源: 3094_START TRANSACTION
CREATE TABLE tpcds. reason (c1 int, c2 int);

--删除表 tpcds. reason。
-- 来源: 3094_START TRANSACTION
DROP TABLE tpcds. reason;

--删除SCHEMA。
-- 来源: 3094_START TRANSACTION
DROP SCHEMA tpcds;

-- 来源: 3096_TIMECAPSULE TABLE
CREATE SCHEMA tpcds;

-- 删除表tpcds.reason_t
-- 来源: 3096_TIMECAPSULE TABLE
DROP TABLE IF EXISTS tpcds.reason_t2;

-- 创建表tpcds.reason_t
-- 来源: 3096_TIMECAPSULE TABLE
CREATE TABLE tpcds.reason_t2 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) )with(storage_type = ustore);

--删除表tpcds.reason_t
-- 来源: 3096_TIMECAPSULE TABLE
DROP TABLE tpcds.reason_t2;

-- 来源: 3096_TIMECAPSULE TABLE
DROP SCHEMA tpcds CASCADE;

-- 来源: 3097_TRUNCATE
CREATE SCHEMA tpcds;

--创建表tpcds.reason。
-- 来源: 3097_TRUNCATE
CREATE TABLE tpcds.reason ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--创建表。
-- 来源: 3097_TRUNCATE
CREATE TABLE tpcds. reason_t1 AS TABLE tpcds. reason;

--删除表。
-- 来源: 3097_TRUNCATE
DROP TABLE tpcds. reason_t1;

-- 来源: 3097_TRUNCATE
CREATE TABLE tpcds. reason_p ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) )PARTITION BY RANGE (r_reason_sk) ( partition p_05_before values less than (05), partition p_15 values less than (15), partition p_25 values less than (25), partition p_35 values less than (35), partition p_45_after values less than (MAXVALUE) );

--清空分区p_05_before。
-- 来源: 3097_TRUNCATE
ALTER TABLE tpcds. reason_p TRUNCATE PARTITION p_05_before;

--清空分区p_15。
-- 来源: 3097_TRUNCATE
ALTER TABLE tpcds. reason_p TRUNCATE PARTITION for (15);

--删除表。
-- 来源: 3097_TRUNCATE
DROP TABLE tpcds. reason_p;

--删除表。
-- 来源: 3097_TRUNCATE
DROP TABLE tpcds.reason;

--删除SCHEMA。
-- 来源: 3097_TRUNCATE
DROP SCHEMA tpcds CASCADE;

-- 来源: 3099_UPDATE
CREATE TABLE tbl_test1(id int, info varchar(10));

-- 删除tbl_test1表。
-- 来源: 3099_UPDATE
DROP TABLE tbl_test1;

-- 来源: 3099_UPDATE
CREATE TABLE test_grade ( sid int, --学号 name varchar(50), --姓名 score char, --成绩 examtime date, --考试时间 last_exam boolean --是否是最后一次考试 );

--删除。
-- 来源: 3099_UPDATE
DROP TABLE test_grade;

-- 来源: 3101_VACUUM
CREATE SCHEMA tpcds;

--创建表tpcds.reason。
-- 来源: 3101_VACUUM
CREATE TABLE tpcds.reason ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--在表 tpcds. reason上创建索引。
-- 来源: 3101_VACUUM
CREATE UNIQUE INDEX ds_reason_index1 ON tpcds. reason(r_reason_sk);

--删除索引。
-- 来源: 3101_VACUUM
DROP INDEX tpcds.ds_reason_index1 CASCADE;

-- 来源: 3101_VACUUM
DROP TABLE tpcds. reason;

-- 来源: 3101_VACUUM
DROP SCHEMA tpcds CASCADE;

-- 来源: 3130_file_3130
CREATE OR REPLACE PROCEDURE array_proc AS DECLARE TYPE ARRAY_INTEGER IS VARRAY ( 1024 ) OF INTEGER ;

-- 来源: 3130_file_3130
DROP PROCEDURE array_proc ;

-- 来源: 3133_file_3133
CREATE OR REPLACE PROCEDURE table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER ;

-- 来源: 3133_file_3133
DROP PROCEDURE table_proc ;

-- 来源: 3133_file_3133
CREATE OR REPLACE PROCEDURE nest_table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER ;

-- 来源: 3133_file_3133
DROP PROCEDURE nest_table_proc ;

-- 来源: 3133_file_3133
CREATE OR REPLACE PROCEDURE index_table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER INDEX BY INTEGER ;

-- 来源: 3133_file_3133
DROP PROCEDURE index_table_proc ;

-- 来源: 3133_file_3133
CREATE OR REPLACE PROCEDURE nest_table_proc AS DECLARE TYPE TABLE_INTEGER IS TABLE OF INTEGER INDEX BY INTEGER ;

-- 来源: 3133_file_3133
DROP PROCEDURE nest_table_proc ;

-- 来源: 3134_file_3134
create or replace procedure p1 () gaussdb -# as gaussdb $ # type t1 is table of int ;

-- 来源: 3134_file_3134
drop procedure if exists p1 ();

-- 来源: 3134_file_3134
create or replace procedure p1 () is gaussdb $ # type rec is record ( c1 int , c2 int );

-- 来源: 3134_file_3134
drop procedure if exists p1 ();

-- 来源: 3134_file_3134
create or replace procedure p1 () gaussdb -# as gaussdb $ # type t1 is table of int index by int ;

-- 来源: 3134_file_3134
drop procedure if exists p1 ();

-- 来源: 3134_file_3134
create or replace procedure p1 () is gaussdb $ # type rec is record ( c1 int , c2 int );

-- 来源: 3134_file_3134
drop procedure if exists p1 ();

-- 来源: 3135_record
create table emp_rec ( gaussdb ( # empno numeric ( 4 , 0 ) not null , gaussdb ( # ename varchar ( 10 ) gaussdb ( # );

-- 来源: 3135_record
CREATE OR REPLACE FUNCTION regress_record ( p_w VARCHAR2 ) RETURNS VARCHAR2 AS $$ gaussdb $ # DECLARE gaussdb $ # --声明一个record类型. gaussdb $ # type rec_type is record ( name varchar2 ( 100 ), epno int );

-- 来源: 3135_record
DROP FUNCTION regress_record ;

-- 来源: 3135_record
DROP TABLE emp_rec ;

-- 来源: 3135_record
create type rec_type is ( c1 int , c2 int );

-- 来源: 3135_record
create or replace function func ( a in int ) return rec_type is gaussdb $ # r rec_type ;

-- 来源: 3135_record
drop function func ;

-- 来源: 3135_record
drop type rec_type ;

-- 来源: 3135_record
create or replace function func ( a out int ) return record is gaussdb $ # type rc is record ( c1 int , c2 int );

-- 来源: 3135_record
drop function func ;

-- 来源: 3145_file_3145
CREATE TYPE o1 AS ( a int , b int );

-- 来源: 3145_file_3145
DROP TABLE IF EXISTS customers;

-- 来源: 3145_file_3145
CREATE TABLE test(a integer);

-- 来源: 3145_file_3145
CREATE OR REPLACE FUNCTION check_test() RETURNS integer language plpgsql AS $function$ DECLARE b integer;

-- 来源: 3146_file_3146
CREATE SCHEMA hr ;

-- 来源: 3146_file_3146
CREATE TABLE staffs ( section_id INTEGER , salary INTEGER );

-- 来源: 3146_file_3146
CREATE OR REPLACE PROCEDURE proc_staffs ( section NUMBER ( 6 ), salary_sum out NUMBER ( 8 , 2 ), staffs_count out INTEGER ) IS BEGIN SELECT sum ( salary ), count ( * ) INTO salary_sum , staffs_count FROM hr . staffs where section_id = section ;

-- 来源: 3146_file_3146
CREATE OR REPLACE PROCEDURE proc_return AS v_num NUMBER ( 8 , 2 );

-- 来源: 3146_file_3146
DROP PROCEDURE proc_staffs ;

-- 来源: 3146_file_3146
DROP PROCEDURE proc_return ;

-- 来源: 3148_file_3148
DROP SCHEMA IF EXISTS hr CASCADE;

-- 来源: 3148_file_3148
CREATE SCHEMA hr;

-- 来源: 3148_file_3148
CREATE TABLE staffs ( staff_id NUMBER, first_name VARCHAR2, salary NUMBER );

--传递并检索值（INTO子句用在USING子句前）：
-- 来源: 3148_file_3148
CREATE OR REPLACE PROCEDURE dynamic_proc AS staff_id NUMBER(6) := 200;

--删除存储过程
-- 来源: 3148_file_3148
DROP PROCEDURE dynamic_proc;

-- 来源: 3148_file_3148
CREATE SCHEMA hr;

-- 来源: 3148_file_3148
CREATE TABLE staffs ( section_id NUMBER, first_name VARCHAR2, phone_number VARCHAR2, salary NUMBER );

-- 来源: 3149_file_3149
CREATE TABLE sections_t1 ( section NUMBER ( 4 ) , section_name VARCHAR2 ( 30 ), manager_id NUMBER ( 6 ), place_id NUMBER ( 4 ) );

-- 来源: 3149_file_3149
DROP TABLE sections_t1 ;

-- 来源: 3150_file_3150
CREATE OR REPLACE PROCEDURE proc_add ( param1 in INTEGER, param2 out INTEGER, param3 in INTEGER ) AS BEGIN param2:= param1 + param3;

--删除存储过程
-- 来源: 3150_file_3150
DROP PROCEDURE proc_add;

-- 来源: 3151_file_3151
DROP SCHEMA IF EXISTS hr CASCADE;

-- 来源: 3151_file_3151
CREATE SCHEMA hr;

-- 来源: 3151_file_3151
CREATE TABLE staffs ( staff_id NUMBER, first_name VARCHAR2, salary NUMBER );

--创建存储过程dynamic_proc
-- 来源: 3151_file_3151
CREATE OR REPLACE PROCEDURE dynamic_proc AS staff_id NUMBER(6) := 200;

--删除存储过程
-- 来源: 3151_file_3151
DROP PROCEDURE dynamic_proc;

-- 来源: 3155_RETURN NEXTRETURN QUERY
DROP TABLE t1 ;

-- 来源: 3155_RETURN NEXTRETURN QUERY
CREATE TABLE t1 ( a int );

-- 来源: 3155_RETURN NEXTRETURN QUERY
CREATE OR REPLACE FUNCTION fun_for_return_next () RETURNS SETOF t1 AS $$ DECLARE r t1 % ROWTYPE ;

-- 来源: 3155_RETURN NEXTRETURN QUERY
CREATE OR REPLACE FUNCTION fun_for_return_query () RETURNS SETOF t1 AS $$ DECLARE r t1 % ROWTYPE ;

-- 来源: 3156_file_3156
CREATE OR REPLACE PROCEDURE proc_control_structure ( i in integer ) AS BEGIN IF i > 0 THEN raise info 'i:% is greater than 0. ' , i ;

-- 来源: 3156_file_3156
DROP PROCEDURE proc_control_structure ;

-- 来源: 3157_file_3157
CREATE OR REPLACE PROCEDURE proc_loop ( i in integer , count out integer ) AS BEGIN count : = 0 ;

-- 来源: 3157_file_3157
CREATE TABLE integertable ( c1 integer ) ;

-- 来源: 3157_file_3157
CREATE OR REPLACE PROCEDURE proc_while_loop ( maxval in integer ) AS DECLARE i int : = 1 ;

-- 来源: 3157_file_3157
DROP PROCEDURE proc_while_loop ;

-- 来源: 3157_file_3157
DROP TABLE integertable ;

-- 来源: 3157_file_3157
CREATE OR REPLACE PROCEDURE proc_for_loop () AS BEGIN FOR I IN 0 .. 5 LOOP DBE_OUTPUT . PRINT_LINE ( 'It is ' || to_char ( I ) || ' time;

-- 来源: 3157_file_3157
DROP PROCEDURE proc_for_loop ;

-- 来源: 3157_file_3157
CREATE OR REPLACE PROCEDURE proc_for_loop_query () AS record VARCHAR2 ( 50 );

-- 来源: 3157_file_3157
DROP PROCEDURE proc_for_loop_query ;

-- 来源: 3157_file_3157
CREATE TABLE hdfs_t1 ( title NUMBER ( 6 ), did VARCHAR2 ( 20 ), data_period VARCHAR2 ( 25 ), kind VARCHAR2 ( 25 ), interval VARCHAR2 ( 20 ), time DATE , isModified VARCHAR2 ( 10 ) );

-- 来源: 3157_file_3157
CREATE OR REPLACE PROCEDURE proc_forall () AS BEGIN FORALL i IN 100 .. 120 update hdfs_t1 set title = title + 100 * i ;

-- 来源: 3157_file_3157
DROP PROCEDURE proc_forall ;

-- 来源: 3157_file_3157
DROP TABLE hdfs_t1 ;

-- 来源: 3158_file_3158
CREATE OR REPLACE PROCEDURE proc_case_branch ( pi_result in integer , pi_return out integer ) AS BEGIN CASE pi_result WHEN 1 THEN pi_return : = 111 ;

-- 来源: 3158_file_3158
DROP PROCEDURE proc_case_branch ;

-- 来源: 3160_file_3160
CREATE TABLE mytab ( id INT , firstname VARCHAR ( 20 ), lastname VARCHAR ( 20 )) ;

-- 来源: 3160_file_3160
CREATE FUNCTION fun_exp () RETURNS INT AS $$ DECLARE x INT : = 0 ;

-- 来源: 3160_file_3160
DROP FUNCTION fun_exp ();

-- 来源: 3160_file_3160
DROP TABLE mytab ;

-- 来源: 3160_file_3160
CREATE TABLE db ( a INT , b TEXT );

-- 来源: 3160_file_3160
CREATE FUNCTION merge_db ( key INT , data TEXT ) RETURNS VOID AS $$ BEGIN LOOP --第一次尝试更新key UPDATE db SET b = data WHERE a = key ;

-- 来源: 3160_file_3160
DROP FUNCTION merge_db ;

-- 来源: 3160_file_3160
DROP TABLE db ;

-- 来源: 3161_GOTO
CREATE OR REPLACE PROCEDURE GOTO_test () AS DECLARE v1 int ;

-- 来源: 3162_file_3162
DROP TABLE IF EXISTS EXAMPLE1;

-- 来源: 3162_file_3162
CREATE TABLE EXAMPLE1(COL1 INT);

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE() AS BEGIN FOR i IN 0..20 LOOP INSERT INTO EXAMPLE1(COL1) VALUES (i);

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE TEST_COMMIT_INSERT_EXCEPTION_ROLLBACK() AS BEGIN DROP TABLE IF EXISTS TEST_COMMIT;

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE TEST_COMMIT2() IS BEGIN DROP TABLE IF EXISTS TEST_COMMIT;

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE exec_func3(RET_NUM OUT INT) AS BEGIN RET_NUM := 1+1;

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE exec_func4(ADD_NUM IN INT) AS SUM_NUM INT;

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE GUC_ROLLBACK() AS BEGIN SET enable_force_vector_engine = on;

-- 来源: 3162_file_3162
CREATE OR REPLACE FUNCTION FUNCTION_EXAMPLE1() RETURN INT AS EXP INT;

-- 来源: 3162_file_3162
CREATE OR REPLACE FUNCTION FUNCTION_EXAMPLE2() RETURN INT AS EXP INT;

-- 来源: 3162_file_3162
CREATE OR REPLACE FUNCTION FUNCTION_TRI_EXAMPLE2() RETURN TRIGGER AS EXP INT;

-- 来源: 3162_file_3162
CREATE TRIGGER TRIGGER_EXAMPLE AFTER DELETE ON EXAMPLE1 FOR EACH ROW EXECUTE PROCEDURE FUNCTION_TRI_EXAMPLE2();

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE1() IMMUTABLE AS EXP INT;

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE2(EXP_OUT OUT INT) AS EXP INT;

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE3() AS BEGIN FOR i IN 0..20 LOOP INSERT INTO EXAMPLE1 (col1) VALUES (i);

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE4() SET ARRAY_NULLS TO "ON" AS BEGIN FOR i IN 0..20 LOOP INSERT INTO EXAMPLE1 (col1) VALUES (i);

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE5(INTIN IN INT, INTOUT OUT INT) AS BEGIN INTOUT := INTIN + 1;

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE TRANSACTION_EXAMPLE6() AS CURSOR CURSOR1(EXPIN INT) IS SELECT TRANSACTION_EXAMPLE5(EXPIN);

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE exec_func1() AS BEGIN CREATE TABLE TEST_exec(A INT);

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE exec_func2() AS BEGIN EXECUTE exec_func1();

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE1() AS BEGIN INSERT INTO EXAMPLE1 VALUES(1);

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE2() AS BEGIN INSERT INTO EXAMPLE1 VALUES(2);

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE3() AS BEGIN INSERT INTO EXAMPLE1 VALUES(2);

-- 来源: 3162_file_3162
CREATE OR REPLACE PROCEDURE STP_SAVEPOINT_EXAMPLE4() AS BEGIN INSERT INTO EXAMPLE1 VALUES(1);

-- 来源: 3168_file_3168
DROP SCHEMA IF EXISTS hr CASCADE;

-- 来源: 3168_file_3168
CREATE SCHEMA hr;

-- 来源: 3168_file_3168
DROP TABLE IF EXISTS sections;

-- 来源: 3168_file_3168
DROP TABLE IF EXISTS staffs;

-- 来源: 3168_file_3168
DROP TABLE IF EXISTS department;

--创建部门表
-- 来源: 3168_file_3168
CREATE TABLE sections( section_name varchar(100), place_id int, section_id int );

--创建员工表
-- 来源: 3168_file_3168
CREATE TABLE staffs( staff_id number(6), salary number(8,2), section_id int, first_name varchar(20) );

--创建部门表
-- 来源: 3168_file_3168
CREATE TABLE department( section_id int );

-- 来源: 3168_file_3168
CREATE OR REPLACE PROCEDURE cursor_proc1 () AS DECLARE DEPT_NAME VARCHAR ( 100 );

-- 来源: 3168_file_3168
DROP PROCEDURE cursor_proc1 ;

-- 来源: 3168_file_3168
CREATE TABLE hr . staffs_t1 AS TABLE hr . staffs ;

-- 来源: 3168_file_3168
CREATE OR REPLACE PROCEDURE cursor_proc2 () AS DECLARE V_EMPNO NUMBER ( 6 );

-- 来源: 3168_file_3168
DROP PROCEDURE cursor_proc2 ;

-- 来源: 3168_file_3168
DROP TABLE hr . staffs_t1 ;

-- 来源: 3168_file_3168
CREATE OR REPLACE PROCEDURE proc_sys_ref ( O OUT SYS_REFCURSOR ) IS C1 SYS_REFCURSOR ;

-- 来源: 3168_file_3168
DROP PROCEDURE proc_sys_ref ;

-- 来源: 3169_file_3169
CREATE OR REPLACE PROCEDURE proc_cursor3 () AS DECLARE V_DEPTNO NUMBER ( 4 ) : = 100 ;

-- 来源: 3169_file_3169
DROP PROCEDURE proc_cursor3 ;

-- 来源: 3170_file_3170
CREATE TABLE integerTable1 ( A INTEGER );

-- 来源: 3170_file_3170
CREATE TABLE integerTable2 ( B INTEGER );

-- 来源: 3170_file_3170
DROP TABLE integerTable1 ;

-- 来源: 3170_file_3170
DROP TABLE integerTable2 ;

-- 来源: 3178_DBE_COMPRESSION
alter database set ilm = on ;

-- 来源: 3178_DBE_COMPRESSION
CREATE user user1 IDENTIFIED BY 'Gauss_zzy123' ;

-- 来源: 3178_DBE_COMPRESSION
CREATE user user2 IDENTIFIED BY 'Gauss_zzy123' ;

-- 来源: 3178_DBE_COMPRESSION
CREATE TABLE TEST_DATA ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

-- 来源: 3180_DBE_HEAT_MAP
ALTER DATABASE set ilm = on ;

-- 来源: 3180_DBE_HEAT_MAP
CREATE Schema HEAT_MAP_DATA ;

-- 来源: 3180_DBE_HEAT_MAP
CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1' ;

-- 来源: 3180_DBE_HEAT_MAP
CREATE TABLE HEAT_MAP_DATA . heat_map_table ( id INT , value TEXT ) TABLESPACE example1 ;

-- 来源: 3181_DBE_ILM
ALTER DATABASE set ilm = on ;

-- 来源: 3181_DBE_ILM
CREATE Schema ILM_DATA ;

-- 来源: 3181_DBE_ILM
CREATE SEQUENCE ILM_DATA . ORDER_TABLE_SE_ORDER_ID MINVALUE 1 ;

-- 来源: 3181_DBE_ILM
CREATE OR REPLACE PROCEDURE ILM_DATA . ORDER_TABLE_CREATE_DATA ( NUM INTEGER ) IS BEGIN FOR X IN 1 .. NUM LOOP INSERT INTO ORDER_TABLE VALUES ( ORDER_TABLE_SE_ORDER_ID . nextval , '零食大礼包A' , NOW ());

-- 来源: 3181_DBE_ILM
CREATE TABLE ILM_DATA . ORDER_TABLE ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) WITH ( STORAGE_TYPE = ASTORE ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

-- 来源: 3186_DBE_PROFILER
DROP TABLE IF EXISTS t1 ;

-- 来源: 3186_DBE_PROFILER
CREATE TABLE t1 ( i int );

-- 来源: 3186_DBE_PROFILER
CREATE OR REPLACE PROCEDURE p1 () AS sql_stmt varchar2 ( 200 );

-- 来源: 3186_DBE_PROFILER
CREATE OR REPLACE PROCEDURE p2 () AS BEGIN p1 ();

-- 来源: 3186_DBE_PROFILER
CREATE OR REPLACE PROCEDURE p3 () AS BEGIN p2 ();

-- 来源: 3186_DBE_PROFILER
DROP TABLE t1 ;

-- 来源: 3186_DBE_PROFILER
CREATE TABLE t2 ( a int , b int );

-- 来源: 3186_DBE_PROFILER
CREATE OR REPLACE PROCEDURE autonomous ( a int , b int ) AS DECLARE num3 int : = a ;

-- 来源: 3186_DBE_PROFILER
CREATE OR REPLACE PROCEDURE autonomous_1 ( a int , b int ) AS DECLARE BEGIN dbe_output . print_line ( 'just no use call.' );

-- 来源: 3186_DBE_PROFILER
DROP TABLE t2 ;

-- 来源: 3189_DBE_SCHEDULER
create user test1 identified by '*********';

-- 来源: 3189_DBE_SCHEDULER
drop user test1;

-- 来源: 3189_DBE_SCHEDULER
create user user1 password '1*s*****';

-- 来源: 3189_DBE_SCHEDULER
drop user user1;

-- 来源: 3189_DBE_SCHEDULER
create user user1 password '1*s*****';

-- 来源: 3189_DBE_SCHEDULER
drop user user1;

-- 来源: 3189_DBE_SCHEDULER
CREATE OR REPLACE PROCEDURE pr1(calendar_str text) as DECLARE start_date timestamp with time zone;

-- 来源: 3192_DBE_STATS
CREATE SCHEMA dbe_stats_lock;

-- 来源: 3192_DBE_STATS
CREATE TABLE t1(a int,b int);

-- 删除表、删除命名空间
-- 来源: 3192_DBE_STATS
DROP TABLE t1;

-- 来源: 3192_DBE_STATS
DROP SCHEMA dbe_stats_lock;

-- 来源: 3192_DBE_STATS
CREATE SCHEMA dbe_stats_lock;

-- 来源: 3192_DBE_STATS
CREATE TABLE upart_table(a int, b int, c int) PARTITION BY RANGE(a) ( PARTITION p1 VALUES LESS THAN(1200), PARTITION p2 VALUES LESS THAN(2400), PARTITION p3 VALUES LESS THAN(MAXVALUE) );

-- 删除表、命名空间
-- 来源: 3192_DBE_STATS
DROP TABLE upart_table;

-- 来源: 3192_DBE_STATS
DROP SCHEMA dbe_stats_lock;

-- 来源: 3192_DBE_STATS
CREATE SCHEMA dbe_stats_lock;

-- 来源: 3192_DBE_STATS
CREATE TABLE t1(a int,b int);

-- 删除表、命名空间
-- 来源: 3192_DBE_STATS
DROP TABLE t1;

-- 来源: 3192_DBE_STATS
DROP SCHEMA dbe_stats_lock;

-- 来源: 3192_DBE_STATS
CREATE SCHEMA dbe_stats_restore;

-- 来源: 3192_DBE_STATS
CREATE TABLE t1(a int, b int);

-- 删除表、删除命名空间
-- 来源: 3192_DBE_STATS
DROP TABLE t1;

-- 来源: 3192_DBE_STATS
DROP SCHEMA dbe_stats_restore;

-- 来源: 3192_DBE_STATS
CREATE SCHEMA dbe_stats_restore;

-- 来源: 3192_DBE_STATS
CREATE TABLE t1(a int, b int);

-- 删除表、命名空间
-- 来源: 3192_DBE_STATS
DROP TABLE t1;

-- 来源: 3192_DBE_STATS
DROP SCHEMA dbe_stats_restore;

-- 来源: 3192_DBE_STATS
CREATE SCHEMA dbe_stats_restore;

-- 来源: 3192_DBE_STATS
CREATE TABLE t1(a int, b int);

-- 删除表、命名空间
-- 来源: 3192_DBE_STATS
DROP TABLE t1;

-- 来源: 3192_DBE_STATS
DROP SCHEMA dbe_stats_restore;

-- 来源: 3192_DBE_STATS
CREATE SCHEMA dbe_stats_purge;

-- 来源: 3192_DBE_STATS
CREATE TABLE t1(a int, b int);

-- 删除表、命名空间
-- 来源: 3192_DBE_STATS
DROP TABLE t1;

-- 来源: 3192_DBE_STATS
DROP SCHEMA dbe_stats_purge;

-- 来源: 3198_Retry
CREATE OR REPLACE PROCEDURE retry_basic ( IN x INT ) AS BEGIN INSERT INTO t1 ( a ) VALUES ( x );

-- 来源: 3202_file_3202
CREATE TABLE test_trigger_des_tbl(id1 int, id2 int, id3 int);

-- 来源: 3202_file_3202
CREATE OR REPLACE FUNCTION tri_insert_func() RETURNS TRIGGER AS $$ DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 3202_file_3202
DROP TABLE test_trigger_des_tbl;

-- 来源: 3202_file_3202
CREATE TABLE t1(a INT ,b TEXT);

-- 来源: 3202_file_3202
DROP TABLE t1;

-- 来源: 3202_file_3202
CREATE TABLE sections(section_id INT);

-- 来源: 3202_file_3202
CREATE OR REPLACE PROCEDURE proc_sys_ref(OUT c1 REFCURSOR) IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 3202_file_3202
CREATE OR REPLACE PROCEDURE proc_sys_call() AS DECLARE c1 SYS_REFCURSOR;

-- 来源: 3202_file_3202
DROP PROCEDURE proc_sys_ref;

-- 来源: 3202_file_3202
DROP PROCEDURE proc_sys_call;

-- 来源: 3202_file_3202
CREATE OR REPLACE PROCEDURE proc_sys_ref(IN c1 REFCURSOR) IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 3202_file_3202
CREATE OR REPLACE PROCEDURE proc_sys_call() AS DECLARE c1 SYS_REFCURSOR;

-- 来源: 3202_file_3202
DROP PROCEDURE proc_sys_ref;

-- 来源: 3202_file_3202
DROP PROCEDURE IF EXISTS proc_sys_ref;

-- 来源: 3202_file_3202
CREATE OR REPLACE FUNCTION proc_sys_ref() RETURN SYS_REFCURSOR IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 3202_file_3202
DROP FUNCTION IF EXISTS proc_sys_ref;

-- 来源: 3202_file_3202
CREATE OR REPLACE FUNCTION proc_sys_ref(C1 out SYS_REFCURSOR) return SYS_REFCURSOR IS gaussdb$# declare gaussdb$# PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 3202_file_3202
CREATE OR REPLACE PROCEDURE proc_sys_ref(OUT c1 REFCURSOR) IS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 3202_file_3202
DROP PROCEDURE proc_sys_ref;

-- 来源: 3202_file_3202
DROP TABLE sections;

-- 来源: 3202_file_3202
CREATE TABLE test_in (id INT,a DATE);

-- 来源: 3202_file_3202
CREATE OR REPLACE FUNCTION autonomous_out() RETURNS RECORD LANGUAGE PLPGSQL AS $$ DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 3202_file_3202
CREATE TYPE rec IS (e1 INTEGER, e2 VARCHAR2);

-- 来源: 3202_file_3202
CREATE OR REPLACE FUNCTION func(ele3 INOUT VARCHAR2) RETURN rec AS i INTEGER;

-- 来源: 3202_file_3202
DROP TABLE test_in;

-- 来源: 3202_file_3202
DROP FUNCTION autonomous_out;

-- 来源: 3202_file_3202
DROP FUNCTION func;

-- 来源: 3202_file_3202
CREATE OR REPLACE PROCEDURE auto_func(r INT) AS DECLARE a INT;

-- 来源: 3202_file_3202
DROP FUNCTION auto_func;

-- 来源: 3202_file_3202
CREATE TABLE test_in (id INT,a DATE);

-- 来源: 3202_file_3202
CREATE TABLE test_main (id INT,a DATE);

-- 来源: 3202_file_3202
CREATE OR REPLACE FUNCTION autonomous_f_022(num1 INT) RETURNS SETOF test_in LANGUAGE PLPGSQL AS $$ DECLARE count INT :=3;

-- 来源: 3202_file_3202
DROP TABLE test_main;

-- 来源: 3202_file_3202
DROP TABLE test_in;

-- 来源: 3203_file_3203
CREATE TABLE t2(a INT, b INT);

-- 来源: 3203_file_3203
CREATE OR REPLACE PROCEDURE autonomous_4(a INT, b INT) AS DECLARE num3 INT := a;

-- 来源: 3203_file_3203
CREATE OR REPLACE PROCEDURE autonomous_5(a INT, b INT) AS DECLARE BEGIN DBE_OUTPUT.PRINT_LINE('JUST NO USE CALL.');

-- 来源: 3203_file_3203
DROP TABLE t2;

-- 来源: 3203_file_3203
DROP PROCEDURE autonomous_4;

-- 来源: 3203_file_3203
DROP PROCEDURE autonomous_5;

-- 来源: 3204_file_3204
CREATE TABLE t1(a INT ,B TEXT);

-- 来源: 3204_file_3204
DROP TABLE t1;

-- 来源: 3205_file_3205
CREATE TABLE t4(a INT, b INT, c TEXT);

-- 来源: 3205_file_3205
CREATE OR REPLACE FUNCTION autonomous_32(a INT ,b INT ,c TEXT) RETURN INT AS DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 3205_file_3205
CREATE OR REPLACE FUNCTION autonomous_33(num1 INT) RETURN INT AS DECLARE num3 INT := 220;

-- 来源: 3205_file_3205
DROP TABLE t4;

-- 来源: 3205_file_3205
DROP FUNCTION autonomous_32;

-- 来源: 3205_file_3205
DROP FUNCTION autonomous_32;

-- 来源: 3206_Package
CREATE TABLE t2(a INT, b INT);

-- 来源: 3206_Package
CREATE OR REPLACE PACKAGE autonomous_pkg AS PROCEDURE autonomous_4(a INT, b INT);

-- 来源: 3206_Package
CREATE OR REPLACE PACKAGE BODY autonomous_pkg AS PROCEDURE autonomous_4(a INT, b INT) AS DECLARE num3 INT := a;

-- 来源: 3206_Package
CREATE OR REPLACE PROCEDURE autonomous_5(a INT, b INT) AS DECLARE va INT;

-- 来源: 3206_Package
DROP TABLE t2;

-- 来源: 3929_DBE_PLDEBUGGER Schema
CREATE OR REPLACE PROCEDURE test_debug ( IN x INT ) AS BEGIN INSERT INTO t1 ( a ) VALUES ( x );

-- 来源: 3978_file_3978
ALTER DATABASE dbname SET paraname TO value ;

-- 来源: 3978_file_3978
ALTER USER username SET paraname TO value ;

-- 来源: 3978_file_3978
ALTER DATABASE postgres SET explain_perf_mode TO pretty ;

-- 来源: 3978_file_3978
ALTER USER omm SET explain_perf_mode TO pretty ;

-- 来源: 4027_file_4027
create table test1 ( c1 int , c2 varchar );

-- 来源: 4027_file_4027
create table tb_test(c1 int,c2 varchar2,c3 varchar2);

-- 来源: 4027_file_4027
create or replace view v_test as select rownum from tb_test;

-- 来源: 4027_file_4027
create or replace view v_test1 as select rownum from tb_test;

-- 来源: 4027_file_4027
create table tab_1(col1 varchar(3));

-- 来源: 4027_file_4027
create table tab_2(col2 char(3));

-- 来源: 4027_file_4027
create or replace function test(f1 int, f2 int default 20, f3 int, f4 int default 40, f5 int default 50) return int gaussdb -# as gaussdb $# begin gaussdb $# raise info 'f1:%',f1;

-- 来源: 4027_file_4027
create or replace function test(f1 int, f2 int default 20, f3 int, f4 int default 40, f5 int default 50) return int gaussdb -# as gaussdb $# begin gaussdb $# raise info 'f1:%',f1;

-- 来源: 4027_file_4027
CREATE OR REPLACE PROCEDURE out_param_test1(m in int, v inout varchar2,v1 inout varchar2) is gaussdb$# begin gaussdb$# v := 'aaaddd';

-- 来源: 4027_file_4027
CREATE OR REPLACE PROCEDURE call_out_param_test1 is gaussdb$# v varchar2(5) := 'aabbb';

-- 来源: 4027_file_4027
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of varchar(5);

-- 来源: 4027_file_4027
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of varchar(5);

-- 来源: 4027_file_4027
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of int index by varchar(5);

-- 来源: 4027_file_4027
CREATE OR REPLACE procedure p1 is gaussdb$# type t1 is table of int index by varchar(5);

-- 来源: 4027_file_4027
create or replace function proc_test return varchar2 as gaussdb$# begin gaussdb$# return '1';

-- 来源: 4027_file_4027
create or replace procedure proc_test as gaussdb$# begin gaussdb$# null;

-- 来源: 4027_file_4027
create or replace procedure proc_test as gaussdb$# begin gaussdb$# null;

-- 来源: 4027_file_4027
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- 来源: 4027_file_4027
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- 来源: 4027_file_4027
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- 来源: 4027_file_4027
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- 来源: 4027_file_4027
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- 来源: 4027_file_4027
create or replace procedure p1 is gaussdb$# type t1 is table of int;

-- 来源: 4027_file_4027
create table test(c1 varchar2);

-- 来源: 4027_file_4027
create user plsql_rollback1 PASSWORD '********';

-- 来源: 4027_file_4027
create user plsql_rollback2 PASSWORD '********';

-- 来源: 4027_file_4027
create or replace procedure plsql_rollback1.p1 () authid definer as gaussdb$# va int;

-- 来源: 4027_file_4027
CREATE SCHEMA sch1;

-- 来源: 4027_file_4027
CREATE PACKAGE pck1 IS PROCEDURE sch1.pck1();

-- 创建全量物化视图
-- 来源: 4273_file_4273
CREATE MATERIALIZED VIEW mv AS select count(*) from t1;

-- 删除物化视图，删除表
-- 来源: 4273_file_4273
DROP MATERIALIZED VIEW mv;

-- 来源: 4273_file_4273
DROP TABLE t1;

-- 创建增量物化视图
-- 来源: 4277_file_4277
CREATE INCREMENTAL MATERIALIZED VIEW mv AS SELECT * FROM t1;

-- 删除物化视图，删除表
-- 来源: 4277_file_4277
DROP MATERIALIZED VIEW mv;

-- 来源: 4277_file_4277
DROP TABLE t1;

-- 来源: 4280_gsql
CREATE TABLE creditcard_info ( id_number int, name text encrypted with (column_encryption_key = cek1, encryption_type = DETERMINISTIC), credit_card varchar(19) encrypted with (column_encryption_key = cek1, encryption_type = DETERMINISTIC));

-- 向表中新增一列加密列
-- 来源: 4280_gsql
ALTER TABLE creditcard_info ADD COLUMN age int ENCRYPTED WITH (COLUMN_ENCRYPTION_KEY = cek1, ENCRYPTION_TYPE = DETERMINISTIC);

-- 从表中删除一列加密列
-- 来源: 4280_gsql
ALTER TABLE creditcard_info DROP COLUMN age;

-- 来源: 4280_gsql
DROP TABLE creditcard_info;

-- 来源: 4283__
CREATE TABLE creditcard_info ( id_number int , name text , credit_card varchar ( 19 ) encrypted with ( column_encryption_key = cek1 , encryption_type = DETERMINISTIC ) ) with ( orientation = row ) distribute by hash ( id_number );

-- 来源: 4283__
CREATE FUNCTION f_encrypt_in_sql ( val1 text , val2 varchar ( 19 )) RETURNS text AS 'SELECT name from creditcard_info where name=$1 or credit_card=$2 LIMIT 1' LANGUAGE SQL ;

-- 来源: 4283__
CREATE FUNCTION f_encrypt_in_plpgsql ( val1 text , val2 varchar ( 19 ), OUT c text ) AS $$ BEGIN SELECT into c name from creditcard_info where name = $ 1 or credit_card = $ 2 LIMIT 1 ;

-- 来源: 4284_file_4284
CREATE TABLE t1 (c1 INT, c2 TEXT) WITH (enable_tde = on);

-- 来源: 4284_file_4284
CREATE TABLE t2 (c1 INT, c2 TEXT) WITH (enable_tde = on, encrypt_algo = 'SM4_CTR');

-- 来源: 4284_file_4284
ALTER TABLE t1 ENCRYPTION KEY ROTATION;

-- 来源: 4284_file_4284
ALTER TABLE t1 SET (enable_tde = off);

-- 来源: 4284_file_4284
ALTER TABLE t1 SET (enable_tde = on);

-- 来源: 4284_file_4284
DROP TABLE t1;

-- 来源: 4286_file_4286
CREATE SECURITY LABEL label1 'L1:G2,G4' ;

-- 来源: 4286_file_4286
CREATE SECURITY LABEL label2 'L2:G2-G4' ;

-- 来源: 4286_file_4286
CREATE SECURITY LABEL label3 'L3:G1-G5' ;

-- 来源: 4286_file_4286
DROP SECURITY LABEL label1 ;

-- 来源: 4286_file_4286
DROP SECURITY LABEL label2 ;

-- 来源: 4286_file_4286
DROP SECURITY LABEL label3 ;

-- 来源: 4318_PBE
create sequence seq;

-- 来源: 4319_file_4319
CREATE TABLE t3(c1 TEXT, c2 INT);

-- 来源: 4320_file_4320
CREATE INDEX tpcds_web_returns_p2_index1 ON web_returns_p2 (ca_address_id) LOCAL;

-- 来源: 4320_file_4320
CREATE INDEX tpcds_web_returns_p2_index2 ON web_returns_p2 (ca_address_sk) LOCAL ( PARTITION web_returns_p2_P1_index, PARTITION web_returns_p2_P2_index TABLESPACE example3, PARTITION web_returns_p2_P3_index TABLESPACE example4, PARTITION web_returns_p2_P4_index, PARTITION web_returns_p2_P5_index, PARTITION web_returns_p2_P6_index, PARTITION web_returns_p2_P7_index, PARTITION web_returns_p2_P8_index ) TABLESPACE example2;

-- 来源: 4320_file_4320
CREATE INDEX tpcds_web_returns_p2_global_index ON web_returns_p2 (ca_street_number) GLOBAL;

-- 来源: 4320_file_4320
CREATE INDEX tpcds_web_returns_for_p1 ON web_returns_p2 (ca_address_id) LOCAL(partition ind_part for p1);

-- 来源: 4320_file_4320
CREATE INDEX tpcds_web_returns_for_p2 ON web_returns_p2 (ca_address_id) LOCAL(partition ind_part for (5000));

-- 来源: 4320_file_4320
ALTER INDEX tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P2_index TABLESPACE example1;

-- 来源: 4320_file_4320
ALTER INDEX tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P3_index TABLESPACE example2;

-- 来源: 4320_file_4320
ALTER INDEX tpcds_web_returns_p2_index2 RENAME PARTITION web_returns_p2_P8_index TO web_returns_p2_P8_index_new;

-- 来源: 4320_file_4320
DROP INDEX tpcds_web_returns_p2_index1;

-- 来源: 4322_file_4322
create table t1_range_int ( c1 int, c2 int, c3 int, c4 int ) partition by range(c1) ( partition range_p00 values less than(10), partition range_p01 values less than(20), partition range_p02 values less than(30), partition range_p03 values less than(40), partition range_p04 values less than(50) );

-- 来源: 4322_file_4322
ALTER TABLE t1_range_int ADD STATISTICS ((c2, c3));

-- 来源: 4322_file_4322
create index t1_range_int_index on t1_range_int(text(c1)) local;

-- 来源: 4351_file_4351
CREATE TABLE TEST(a int);

-- 来源: 4351_file_4351
CREATE TABLE TEST1(a int) with(orientation=row, storage_type=ustore);

-- 来源: 4351_file_4351
CREATE TABLE TEST2(a int) with(orientation=row, storage_type=astore);

-- 来源: 4351_file_4351
create table test4(a int) with(orientation=row);

-- 来源: 4363_Ustore
CREATE TABLE ustore_table(a INT PRIMARY KEY, b CHAR (20)) WITH (STORAGE_TYPE=USTORE);

-- 来源: 4363_Ustore
drop table ustore_table;

-- 来源: 4363_Ustore
CREATE INDEX UB_tree_index ON test(a);

-- 来源: 4363_Ustore
drop index ub_tree_index;

-- 来源: 4365_init_td
CREATE TABLE test1(name varchar) WITH(storage_type = ustore, init_td=2);

-- 来源: 4365_init_td
ALTER TABLE test1 SET(init_td=8);

-- 来源: 4365_init_td
DROP TABLE test1;

-- 来源: 4366_fillfactor
create table test(a int) with(fillfactor=100);

-- 来源: 4366_fillfactor
alter table test set(fillfactor=92);

-- 来源: 4366_fillfactor
drop table test;

-- 来源: 4393_file_4393
DROP TABLE IF EXISTS "public".flashtest;

-- 来源: 4393_file_4393
CREATE TABLE "public".flashtest (col1 INT,col2 TEXT) with(storage_type=ustore);

-- 来源: 4393_file_4393
drop TABLE IF EXISTS "public".flashtest;

-- 来源: 4394_file_4394
DROP TABLE IF EXISTS "public".flashtest;

-- 来源: 4394_file_4394
CREATE TABLE "public".flashtest (col1 INT,col2 TEXT) with(storage_type=ustore);

-- 来源: 4394_file_4394
drop TABLE IF EXISTS "public".flashtest;

-- 来源: 4395_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4395_DROP_TRUNCATE
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- 来源: 4395_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4395_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4395_DROP_TRUNCATE
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- 来源: 4395_DROP_TRUNCATE
create index flashtest_index on flashtest(id);

-- 来源: 4395_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4395_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4395_DROP_TRUNCATE
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- 来源: 4395_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4395_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4395_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4395_DROP_TRUNCATE
drop table if EXISTS flashtest_rename;

-- 来源: 4395_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4395_DROP_TRUNCATE
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- 来源: 4395_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4407_file_4407
ALTER DATABASE SET ilm = on;

-- 来源: 4407_file_4407
CREATE TABLE ilm_table_1 (col1 int, col2 text) ilm add policy row store compress advanced row after 3 days of no modification on (col1 < 1000);

-- 来源: 4407_file_4407
CREATE TABLE ilm_table_2 (col1 int, col2 text);

-- 来源: 4407_file_4407
ALTER TABLE ilm_table_2 ilm add policy row store compress advanced row after 3 days of no modification;

-- 来源: 4409_TIPS
DROP TABLE IF EXISTS ILM_TABLE;

-- 来源: 4409_TIPS
CREATE TABLE ILM_TABLE(a int);

-- 来源: 4409_TIPS
ALTER TABLE ILM_TABLE ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION;

-- 来源: 4409_TIPS
DROP TABLE IF EXISTS ILM_TABLE;

-- 来源: 4409_TIPS
CREATE TABLE ILM_TABLE(a int);

-- 来源: 4409_TIPS
ALTER TABLE ILM_TABLE ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION;

-- 来源: 4409_TIPS
ALTER DATABASE set ilm = on ;

-- 来源: 4409_TIPS
CREATE user user1 IDENTIFIED BY '********' ;

-- 来源: 4409_TIPS
CREATE user user2 IDENTIFIED BY '********' ;

-- 来源: 4409_TIPS
CREATE TABLE TEST_DATA ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

-- 来源: 4409_TIPS
ALTER DATABASE set ilm = on ;

-- 来源: 4409_TIPS
CREATE Schema HEAT_MAP_DATA ;

-- 来源: 4409_TIPS
CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1' ;

-- 来源: 4409_TIPS
CREATE TABLE HEAT_MAP_DATA . heat_map_table ( id INT , value TEXT ) TABLESPACE example1 ;

-- 来源: 4493_file_4493
CREATE TABLE t1(c1 int, c2 int);

--创建全量物化视图。
-- 来源: 4493_file_4493
CREATE MATERIALIZED VIEW mv AS select count(*) from t1;

--删除物化视图，删除表。
-- 来源: 4493_file_4493
DROP MATERIALIZED VIEW mv;

-- 来源: 4493_file_4493
DROP TABLE t1;

-- 来源: 4497_file_4497
CREATE TABLE t1(c1 int, c2 int);

--创建增量物化视图。
-- 来源: 4497_file_4497
CREATE INCREMENTAL MATERIALIZED VIEW mv AS SELECT * FROM t1;

--删除物化视图，删除表。
-- 来源: 4497_file_4497
DROP MATERIALIZED VIEW mv;

-- 来源: 4497_file_4497
DROP TABLE t1;

-- 来源: 4500_gsql
CREATE TABLE creditcard_info ( id_number int, name text encrypted with (column_encryption_key = cek1, encryption_type = DETERMINISTIC), credit_card varchar(19) encrypted with (column_encryption_key = cek1, encryption_type = DETERMINISTIC));

-- 向表中新增一列加密列
-- 来源: 4500_gsql
ALTER TABLE creditcard_info ADD COLUMN age int ENCRYPTED WITH (COLUMN_ENCRYPTION_KEY = cek1, ENCRYPTION_TYPE = DETERMINISTIC);

-- 从表中删除一列加密列
-- 来源: 4500_gsql
ALTER TABLE creditcard_info DROP COLUMN age;

-- 来源: 4500_gsql
DROP TABLE creditcard_info;

-- 来源: 4504__
CREATE TABLE creditcard_info ( id_number int , name text , credit_card varchar ( 19 ) encrypted with ( column_encryption_key = cek1 , encryption_type = DETERMINISTIC ) ) with ( orientation = row );

-- 来源: 4504__
CREATE FUNCTION f_encrypt_in_sql ( val1 text , val2 varchar ( 19 )) RETURNS text AS 'SELECT name from creditcard_info where name=$1 or credit_card=$2 LIMIT 1' LANGUAGE SQL ;

-- 来源: 4504__
CREATE FUNCTION f_encrypt_in_plpgsql ( val1 text , val2 varchar ( 19 ), OUT c text ) AS $$ BEGIN SELECT into c name from creditcard_info where name = $ 1 or credit_card = $ 2 LIMIT 1 ;

-- 来源: 4507_gsql
CREATE TABLE contacts ( id int unique , credit float8 encrypted with ( column_encryption_key = cek1 , encryption_type = DETERMINISTIC ), name text encrypted with ( column_encryption_key = cek1 , encryption_type = DETERMINISTIC ));

-- 来源: 4507_gsql
CREATE TABLE contacts_plain ( id int unique , credit float8 , name text );

-- 来源: 4507_gsql
DROP TABLE contacts, contacts_plain;

-- 来源: 4511_file_4511
CREATE TABLE t1 (c1 INT, c2 TEXT) WITH (enable_tde = on);

-- 来源: 4511_file_4511
CREATE TABLE t2 (c1 INT, c2 TEXT) WITH (enable_tde = on, encrypt_algo = 'SM4_CTR');

-- 来源: 4511_file_4511
ALTER TABLE t1 ENCRYPTION KEY ROTATION;

-- 来源: 4511_file_4511
ALTER TABLE t1 SET (enable_tde = off);

-- 来源: 4511_file_4511
ALTER TABLE t1 SET (enable_tde = on);

-- 来源: 4511_file_4511
DROP TABLE t1;

-- 来源: 4513_file_4513
CREATE SECURITY LABEL label1 'L1:G2,G4' ;

-- 来源: 4513_file_4513
CREATE SECURITY LABEL label2 'L2:G2-G4' ;

-- 来源: 4513_file_4513
CREATE SECURITY LABEL label3 'L3:G1-G5' ;

-- 来源: 4513_file_4513
DROP SECURITY LABEL label1 ;

-- 来源: 4513_file_4513
DROP SECURITY LABEL label2 ;

-- 来源: 4513_file_4513
DROP SECURITY LABEL label3 ;

-- 来源: 4522_DDL
CREATE TABLE test_create_table_partition2 (c1 INT, c2 INT) PARTITION BY RANGE (c2) ( PARTITION p1 START(1) END(1000) EVERY(200) , PARTITION p2 END(2000), PARTITION p3 START(2000) END(2500), PARTITION p4 START(2500), PARTITION p5 START(3000) END(5000) EVERY(1000) );

-- 来源: 4522_DDL
CREATE TABLE test_create_table_partition2 (c1 INT, c2 INT) PARTITION BY RANGE (c2) ( PARTITION p1_0 VALUES LESS THAN ('1'), PARTITION p1_1 VALUES LESS THAN ('201'), PARTITION p1_2 VALUES LESS THAN ('401'), PARTITION p1_3 VALUES LESS THAN ('601'), PARTITION p1_4 VALUES LESS THAN ('801'), PARTITION p1_5 VALUES LESS THAN ('1000'), PARTITION p2 VALUES LESS THAN ('2000'), PARTITION p3 VALUES LESS THAN ('2500'), PARTITION p4 VALUES LESS THAN ('3000'), PARTITION p5_1 VALUES LESS THAN ('4000'), PARTITION p5_2 VALUES LESS THAN ('5000') );

-- 来源: 4522_DDL
CREATE TABLE IF NOT EXISTS tb5 (c1 int,c2 int) with (ORIENTATION=ROW, STORAGE_TYPE=USTORE);

-- 来源: 4522_DDL
ALTER TABLE IF EXISTS tb5 * ADD COLUMN IF NOT EXISTS c2 char(5) after c1;

-- 来源: 4522_DDL
ALTER TABLE IF EXISTS public.tb5 ADD COLUMN IF NOT EXISTS c2 pg_catalog.int4 AFTER c1;

-- 来源: 4522_DDL
ALTER TABLE IF EXISTS tb5 * ADD COLUMN IF NOT EXISTS c2 char(5) after c1, ADD COLUMN IF NOT EXISTS c3 char(5) after c1;

-- 来源: 4522_DDL
ALTER TABLE IF EXISTS public.tb5 ADD COLUMN IF NOT EXISTS c2 pg_catalog.int4 AFTER c1, ADD COLUMN IF NOT EXISTS c3 pg_catalog.bpchar(5) AFTER c1;

-- 来源: 4522_DDL
ALTER TABLE IF EXISTS tb5 * ADD COLUMN c2 char(5) after c1, ADD COLUMN IF NOT EXISTS c4 int after c1;

-- 来源: 4522_DDL
ALTER TABLE tbl_28 ADD COLUMN b1 TIMESTAMP DEFAULT NOW();

-- 来源: 4522_DDL
ALTER TABLE tbl_28 ADD COLUMN b2 INT DEFAULT RANDOM();

-- 来源: 4522_DDL
ALTER TABLE tbl_28 ADD COLUMN b3 INT DEFAULT ABS(1);

-- 来源: 4522_DDL
CREATE TABLE IF NOT EXISTS tb1 (c1 time without time zone ON UPDATE CURRENT_TIMESTAMP) with (ORIENTATION=ROW, STORAGE_TYPE=USTORE);

-- B兼容模式下修改表，列字段添加ON UPDATE事件
-- 来源: 4522_DDL
ALTER TABLE IF EXISTS ONLY tb2 MODIFY COLUMN c2 time without time zone ON UPDATE LOCALTIMESTAMP;

-- 来源: 4522_DDL
CREATE TABLE IF NOT EXISTS public.tb1 (c1 TIME) WITH (orientation = 'row', storage_type = 'ustore', compression = 'no') NOCOMPRESS;

-- 来源: 4522_DDL
ALTER TABLE IF EXISTS ONLY public.tb2 MODIFY COLUMN c2 TIME;

-- 来源: 4522_DDL
CREATE TABLE IF NOT EXISTS tb3 (c1 int) with (storage_type=USTORE,ORIENTATION=ROW) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 7 day OF NO MODIFICATION;

-- 来源: 4522_DDL
CREATE TABLE IF NOT EXISTS public.tb3 (c1 pg_catalog.int4) WITH (storage_type = 'ustore', orientation = 'row', compression = 'no') NOCOMPRESS;

-- 来源: 4522_DDL
CREATE TABLE IF NOT EXISTS tb6 (c1 integer comment 'Mysql兼容注释语法') with (ORIENTATION=ROW, STORAGE_TYPE=USTORE);

-- 来源: 4522_DDL
CREATE TABLE IF NOT EXISTS public.tb6 (c1 pg_catalog.int4) WITH (storage_type = 'ustore', orientation = 'row', compression = 'no') NOCOMPRESS;

-- 来源: 4522_DDL
CREATE TABLE mix_tran_t4(id int);

-- 来源: 4522_DDL
CREATE TABLE mix_tran_t5(id int);

-- 来源: 4522_DDL
CREATE TABLE mix_tran_t6(id int);

-- 来源: 4522_DDL
CREATE TABLE mix_tran_t7(id int);

-- 来源: 4522_DDL
CREATE TABLE mix_tran_t8(id int);

-- 来源: 4522_DDL
CREATE TABLE mix_tran_t7(id int);

-- 来源: 4522_DDL
CREATE TYPE compfoo AS (f1 int, f2 text);

-- 来源: 4522_DDL
CREATE TABLE mix_tran_t8(id int);

-- 来源: 4522_DDL
CREATE TYPE compfoo AS (f1 int, f2 text);

-- 来源: 4522_DDL
CREATE TYPE compfoo AS (f1 int, f2 text);

-- 来源: 4522_DDL
CREATE TABLE mix_tran_t9(id int);

-- 来源: 4522_DDL
CREATE OR REPLACE PACKAGE ldp_pkg1 IS var1 int : = 1 ;

-- 来源: 4548_PBE
create sequence seq;

-- 来源: 4549_file_4549
CREATE TABLE t1 (c1 INT, c2 INT) PARTITION BY RANGE (c1) ( PARTITION p1 VALUES LESS THAN(10), PARTITION p2 VALUES LESS THAN(20), PARTITION p3 VALUES LESS THAN(MAXVALUE) );

-- 来源: 4549_file_4549
CREATE TABLE t3(c1 TEXT, c2 INT);

--INSERT为子查询，无法执行FastPath优化，走标准执行器模块
-- 来源: 4554_file_4554
create table test_1(col1 int, col3 text);

-- 来源: 4555_file_4555
CREATE INDEX tpcds_web_returns_p2_index1 ON web_returns_p2 (ca_address_id) LOCAL;

-- 来源: 4555_file_4555
CREATE INDEX tpcds_web_returns_p2_index2 ON web_returns_p2 (ca_address_sk) LOCAL ( PARTITION web_returns_p2_P1_index, PARTITION web_returns_p2_P2_index TABLESPACE example3, PARTITION web_returns_p2_P3_index TABLESPACE example4, PARTITION web_returns_p2_P4_index, PARTITION web_returns_p2_P5_index, PARTITION web_returns_p2_P6_index, PARTITION web_returns_p2_P7_index, PARTITION web_returns_p2_P8_index ) TABLESPACE example2;

-- 来源: 4555_file_4555
CREATE INDEX tpcds_web_returns_p2_global_index ON web_returns_p2 (ca_street_number) GLOBAL;

-- 来源: 4555_file_4555
CREATE INDEX tpcds_web_returns_for_p1 ON web_returns_p2 (ca_address_id) LOCAL(partition ind_part for p1);

-- 来源: 4555_file_4555
CREATE INDEX tpcds_web_returns_for_p2 ON web_returns_p2 (ca_address_id) LOCAL(partition ind_part for (5000));

-- 来源: 4555_file_4555
ALTER INDEX tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P2_index TABLESPACE example1;

-- 来源: 4555_file_4555
ALTER INDEX tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P3_index TABLESPACE example2;

-- 来源: 4555_file_4555
ALTER INDEX tpcds_web_returns_p2_index2 RENAME PARTITION web_returns_p2_P8_index TO web_returns_p2_P8_index_new;

-- 来源: 4555_file_4555
DROP INDEX tpcds_web_returns_p2_index1;

-- 来源: 4557_file_4557
create table t1_range_int ( c1 int, c2 int, c3 int, c4 int ) partition by range(c1) ( partition range_p00 values less than(10), partition range_p01 values less than(20), partition range_p02 values less than(30), partition range_p03 values less than(40), partition range_p04 values less than(50) );

-- 来源: 4557_file_4557
ALTER TABLE t1_range_int ADD STATISTICS ((c2, c3));

-- 来源: 4557_file_4557
create index t1_range_int_index on t1_range_int(text(c1)) local;

-- 来源: 4611_file_4611
CREATE TABLE TEST(a int);

-- 来源: 4611_file_4611
CREATE TABLE TEST1(a int) with(orientation=row, storage_type=ustore);

-- 来源: 4611_file_4611
CREATE TABLE TEST2(a int) with(orientation=row, storage_type=astore);

-- 来源: 4611_file_4611
create table test4(a int) with(orientation=row);

-- 来源: 4623_Ustore
CREATE TABLE ustore_table(a INT PRIMARY KEY, b CHAR (20)) WITH (STORAGE_TYPE=USTORE);

-- 来源: 4623_Ustore
drop table ustore_table;

-- 来源: 4623_Ustore
CREATE INDEX UB_tree_index ON test(a);

-- 来源: 4623_Ustore
drop index ub_tree_index;

-- 来源: 4625_init_td
create table test1(name varchar) with(storage_type = ustore, init_td=2);

-- 来源: 4625_init_td
alter table test1 set(init_td=8);

-- 来源: 4625_init_td
drop table test1;

-- 来源: 4626_fillfactor
create table test(a int) with(fillfactor=100);

-- 来源: 4626_fillfactor
alter table test set(fillfactor=92);

-- 来源: 4626_fillfactor
drop table test;

-- 来源: 4653_file_4653
drop TABLE IF EXISTS "public".flashtest;

-- 来源: 4653_file_4653
CREATE TABLE "public".flashtest (col1 INT,col2 TEXT) with(storage_type=ustore);

-- 来源: 4653_file_4653
drop TABLE IF EXISTS "public".flashtest;

-- 来源: 4654_file_4654
drop TABLE IF EXISTS "public".flashtest;

-- 来源: 4654_file_4654
CREATE TABLE "public".flashtest (col1 INT,col2 TEXT) with(storage_type=ustore);

-- 来源: 4654_file_4654
drop TABLE IF EXISTS "public".flashtest;

-- 来源: 4655_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4655_DROP_TRUNCATE
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- 来源: 4655_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4655_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4655_DROP_TRUNCATE
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- 来源: 4655_DROP_TRUNCATE
create index flashtest_index on flashtest(id);

-- 来源: 4655_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4655_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4655_DROP_TRUNCATE
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- 来源: 4655_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4655_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4655_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4655_DROP_TRUNCATE
drop table if EXISTS flashtest_rename;

-- 来源: 4655_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4655_DROP_TRUNCATE
create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

-- 来源: 4655_DROP_TRUNCATE
drop table if EXISTS flashtest;

-- 来源: 4667_file_4667
ALTER DATABASE SET ilm = on;

-- 来源: 4667_file_4667
CREATE TABLE ilm_table_1 (col1 int, col2 text) ilm add policy row store compress advanced row after 3 days of no modification on (col1 < 1000);

-- 来源: 4667_file_4667
CREATE TABLE ilm_table_2 (col1 int, col2 text);

-- 来源: 4667_file_4667
ALTER TABLE ilm_table_2 ilm add policy row store compress advanced row after 3 days of no modification;

-- 来源: 4669_TIPS
DROP TABLE IF EXISTS ILM_TABLE;

-- 来源: 4669_TIPS
CREATE TABLE ILM_TABLE(a int);

-- 来源: 4669_TIPS
ALTER TABLE ILM_TABLE ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION;

-- 来源: 4669_TIPS
DROP TABLE IF EXISTS ILM_TABLE;

-- 来源: 4669_TIPS
CREATE TABLE ILM_TABLE(a int);

-- 来源: 4669_TIPS
ALTER TABLE ILM_TABLE ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION;

-- 来源: 4669_TIPS
ALTER DATABASE set ilm = on ;

-- 来源: 4669_TIPS
CREATE user user1 IDENTIFIED BY '********' ;

-- 来源: 4669_TIPS
CREATE user user2 IDENTIFIED BY '********' ;

-- 来源: 4669_TIPS
CREATE TABLE TEST_DATA ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

-- 来源: 4669_TIPS
ALTER DATABASE set ilm = on ;

-- 来源: 4669_TIPS
CREATE Schema HEAT_MAP_DATA ;

-- 来源: 4669_TIPS
CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1' ;

-- 来源: 4669_TIPS
CREATE TABLE HEAT_MAP_DATA . heat_map_table ( id INT , value TEXT ) TABLESPACE example1 ;

-- 来源: 4733_DB4AI
CREATE MODEL iris_classification_model USING xgboost_regression_logistic FEATURES sepal_length, sepal_width,petal_length,petal_width TARGET target_type < 2 FROM tb_iris_1 WITH nthread=4, max_depth=8;

-- 来源: 4733_DB4AI
CREATE MODEL patient_linear_regression USING linear_regression FEATURES second_attack,treatment TARGET trait_anxiety FROM patients WITH optimizer='aa';

-- 来源: 4733_DB4AI
CREATE MODEL patient_linear_regression USING linear_regression FEATURES second_attack,treatment TARGET trait_anxiety FROM patients;

-- 来源: 4733_DB4AI
CREATE MODEL patient_linear_regression USING linear_regression FEATURES * TARGET trait_anxiety FROM patients;

-----------------------------------------------------------------------------------------------------------------------
-- 来源: 4733_DB4AI
CREATE MODEL patient_linear_regression USING linear_regression FEATURES second_attack,treatment TARGET * FROM patients;

-- 来源: 4733_DB4AI
CREATE MODEL patient_linear_regression USING linear_regression FEATURES second_attack,treatment FROM patients;

-- 来源: 4733_DB4AI
CREATE MODEL ecoli_svmc USING multiclass FEATURES f1, f2, f3, f4, f5, f6, f7 TARGET cat FROM (SELECT * FROM db4ai_ecoli WHERE cat='cp');

-- 来源: 4733_DB4AI
create model iris_classification_model using xgboost_regression_logistic features message_regular target error_level from error_code;

-- 来源: 4733_DB4AI
CREATE MODEL ecoli_svmc USING multiclass FEATURES f1, f2, f3, f4, f5, f6, f7, cat TARGET cat FROM db4ai_ecoli ;

-- 来源: 5279_drop user
drop user test1 cascade;

-- 来源: 5279_drop user
drop user test1 cascade;

-- 来源: 5295_file_5295
alter foreign table gs_pg_log_ft options (set master_only 'false');

-- 来源: 5295_file_5295
alter foreign table gs_profile_log_ft options (set latest_files '10');

-- 来源: 5779_file_5779
CREATE TABLE HR . areaS ( gaussdb ( # area_ID NUMBER , gaussdb ( # area_NAME VARCHAR2 ( 25 ) gaussdb ( # ) tablespace EXAMPLE ;

-- 来源: 5839_gs_rescue
create table original(col1 integer);

-- 来源: 5839_gs_rescue
create table amend(col1 integer,col2 integer default 0);

-- 来源: 5892_file_5892
CREATE TABLE HR . areaS ( gaussdb ( # area_ID NUMBER , gaussdb ( # area_NAME VARCHAR2 ( 25 ) gaussdb -# ) tablespace EXAMPLE ;

-- 来源: 5949_gs_rescue
create table original(col1 integer);

-- 来源: 5949_gs_rescue
create table amend(col1 integer,col2 integer default 0);

-- 来源: 733_file_733
create table a(id int, value int);

-- 来源: 733_file_733
create table a(id int primary key, value int);

-- 来源: 738_file_738
CREATE USER sysadmin WITH SYSADMIN password "********" ;

-- 来源: 738_file_738
ALTER USER joe SYSADMIN ;

-- 来源: 738_file_738
CREATE USER createrole WITH CREATEROLE password "********" ;

-- 来源: 738_file_738
ALTER USER joe CREATEROLE ;

-- 来源: 738_file_738
CREATE USER auditadmin WITH AUDITADMIN password "********" ;

-- 来源: 738_file_738
ALTER USER joe AUDITADMIN ;

-- 来源: 738_file_738
CREATE USER monadmin WITH MONADMIN password "********" ;

-- 来源: 738_file_738
ALTER USER joe MONADMIN ;

-- 来源: 738_file_738
CREATE USER opradmin WITH OPRADMIN password "********" ;

-- 来源: 738_file_738
ALTER USER joe OPRADMIN ;

-- 来源: 738_file_738
CREATE USER poladmin WITH POLADMIN password "********" ;

-- 来源: 738_file_738
ALTER USER joe POLADMIN ;

-- 来源: 740_file_740
CREATE USER joe WITH CREATEDB PASSWORD "********" ;

-- 来源: 740_file_740
CREATE USER user_persistence WITH PERSISTENCE IDENTIFIED BY "********" ;

-- 来源: 743_file_743
CREATE ROLE lily WITH CREATEDB PASSWORD "********" ;

-- 来源: 744_file_744
CREATE USER alice PASSWORD '********' ;

-- 来源: 744_file_744
CREATE USER bob PASSWORD '********' ;

-- 来源: 744_file_744
CREATE USER peter PASSWORD '********' ;

-- 来源: 744_file_744
CREATE TABLE all_data ( id int , role varchar ( 100 ), data varchar ( 100 ));

-- 来源: 744_file_744
ALTER TABLE all_data ENABLE ROW LEVEL SECURITY ;

-- 来源: 751_file_751
CREATE USER joe WITH PASSWORD "********" ;

-- 来源: 752_file_752
CREATE DATABASE db_tpcds ;

-- 来源: 752_file_752
CREATE DATABASE db_tpcds WITH TABLESPACE = hr_local ;

-- 来源: 752_file_752
ALTER DATABASE db_tpcds RENAME TO human_tpcds ;

-- 来源: 752_file_752
DROP DATABASE human_tpcds ;

-- 来源: 753_file_753
CREATE TABLE customer_t1 ( state_ID CHAR ( 2 ), state_NAME VARCHAR2 ( 40 ), area_ID NUMBER );

-- 来源: 753_file_753
DROP TABLE customer_t1 ;

-- 来源: 753_file_753
CREATE TABLE customer_t2 ( state_ID CHAR ( 2 ), state_NAME VARCHAR2 ( 40 ), area_ID NUMBER ) WITH ( ORIENTATION = COLUMN );

-- 来源: 753_file_753
DROP TABLE customer_t2 ;

-- 来源: 754_file_754
CREATE TABLESPACE ds_location1 LOCATION '/pg_location/mount1/path1' MAXSIZE '500G' ;

-- 来源: 754_file_754
CREATE USER jack IDENTIFIED BY '********' ;

-- 来源: 754_file_754
CREATE TABLESPACE fastspace RELATIVE LOCATION 'my_tablespace/tablespace1' ;

-- 来源: 754_file_754
CREATE TABLE foo ( i int ) TABLESPACE fastspace ;

-- 来源: 754_file_754
CREATE TABLE foo2 ( i int );

-- 来源: 754_file_754
ALTER TABLESPACE fastspace RENAME TO fspace ;

-- 来源: 754_file_754
DROP USER jack CASCADE ;

-- 来源: 754_file_754
DROP TABLE foo ;

-- 来源: 754_file_754
DROP TABLE foo2 ;

-- 来源: 754_file_754
DROP TABLESPACE fspace ;

-- 来源: 756_file_756
CREATE TABLE customer_t1 ( c_customer_sk integer , c_customer_id char ( 5 ), c_first_name char ( 6 ), c_last_name char ( 8 ) ) distribute by hash ( c_last_name );

-- 来源: 757_file_757
CREATE TABLE table1 ( id int , a char ( 6 ), b varchar ( 6 ), c varchar ( 6 ));

-- 来源: 757_file_757
CREATE TABLE table2 ( id int , a char ( 20 ), b varchar ( 20 ), c varchar ( 20 ));

-- 来源: 757_file_757
CREATE TABLE customer_t2 ( c_customer_sk integer , c_customer_id char ( 5 ), c_first_name char ( 6 ), c_last_name char ( 8 ) );

-- 来源: 757_file_757
DROP TABLE customer_t2 CASCADE ;

-- 来源: 760_file_760
DROP TABLE customer_t1 ;

-- 来源: 761_file_761
CREATE TABLE public.search_table_t1(a int) distribute by hash(a);

-- 来源: 761_file_761
CREATE TABLE public.search_table_t2(b int) distribute by hash(b);

-- 来源: 761_file_761
CREATE TABLE public.search_table_t3(c int) distribute by hash(c);

-- 来源: 761_file_761
CREATE TABLE public.search_table_t4(d int) distribute by hash(d);

-- 来源: 761_file_761
CREATE TABLE public.search_table_t5(e int) distribute by hash(e);

-- 来源: 763_schema
CREATE SCHEMA myschema ;

-- 来源: 763_schema
CREATE SCHEMA myschema AUTHORIZATION omm ;

-- 来源: 763_schema
CREATE TABLE myschema . mytable ( id int , name varchar ( 20 ));

-- 来源: 763_schema
CREATE USER jack IDENTIFIED BY '********' ;

-- 来源: 763_schema
DROP SCHEMA IF EXISTS nullschema ;

-- 来源: 763_schema
DROP SCHEMA myschema CASCADE ;

-- 来源: 763_schema
DROP USER jack ;

-- 来源: 764_file_764
CREATE TABLE tpcds . customer_address ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) DISTRIBUTE BY HASH ( ca_address_sk ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) ) ENABLE ROW MOVEMENT ;

-- 来源: 764_file_764
CREATE TABLE tpcds . web_returns_p2 ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) DISTRIBUTE BY HASH ( ca_address_sk ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) ) ENABLE ROW MOVEMENT ;

-- 来源: 764_file_764
ALTER TABLE tpcds . web_returns_p2 DISABLE ROW MOVEMENT ;

-- 来源: 764_file_764
ALTER TABLE tpcds . web_returns_p2 DROP PARTITION P8 ;

-- 来源: 764_file_764
ALTER TABLE tpcds . web_returns_p2 ADD PARTITION P8 VALUES LESS THAN ( MAXVALUE );

-- 来源: 764_file_764
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION P8 TO P_9 ;

-- 来源: 764_file_764
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION FOR ( 40000 ) TO P8 ;

-- 来源: 764_file_764
DROP TABLE tpcds . customer_address ;

-- 来源: 764_file_764
DROP TABLE tpcds . web_returns_p2 ;

-- 来源: 764_file_764
CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1/tablespace_1' ;

-- 来源: 764_file_764
CREATE TABLESPACE example2 RELATIVE LOCATION 'tablespace2/tablespace_2' ;

-- 来源: 764_file_764
CREATE TABLESPACE example3 RELATIVE LOCATION 'tablespace3/tablespace_3' ;

-- 来源: 764_file_764
CREATE TABLESPACE example4 RELATIVE LOCATION 'tablespace4/tablespace_4' ;

-- 来源: 764_file_764
CREATE TABLE tpcds . customer_address ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) TABLESPACE example1 DISTRIBUTE BY HASH ( ca_address_sk ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) TABLESPACE example2 ) ENABLE ROW MOVEMENT ;

-- 来源: 764_file_764
CREATE TABLE tpcds . web_returns_p2 ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) TABLESPACE example1 DISTRIBUTE BY HASH ( ca_address_sk ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) TABLESPACE example2 ) ENABLE ROW MOVEMENT ;

-- 来源: 764_file_764
ALTER TABLE tpcds . web_returns_p2 DISABLE ROW MOVEMENT ;

-- 来源: 764_file_764
ALTER TABLE tpcds . web_returns_p2 DROP PARTITION P8 ;

-- 来源: 764_file_764
ALTER TABLE tpcds . web_returns_p2 ADD PARTITION P8 VALUES LESS THAN ( MAXVALUE );

-- 来源: 764_file_764
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION P8 TO P_9 ;

-- 来源: 764_file_764
ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION FOR ( 40000 ) TO P8 ;

-- 来源: 764_file_764
ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P6 TABLESPACE example3 ;

-- 来源: 764_file_764
ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P4 TABLESPACE example4 ;

-- 来源: 764_file_764
DROP TABLE tpcds . customer_address ;

-- 来源: 764_file_764
DROP TABLE tpcds . web_returns_p2 ;

-- 来源: 764_file_764
DROP TABLESPACE example1 ;

-- 来源: 764_file_764
DROP TABLESPACE example2 ;

-- 来源: 764_file_764
DROP TABLESPACE example3 ;

-- 来源: 764_file_764
DROP TABLESPACE example4 ;

-- 来源: 765_file_765
CREATE INDEX tpcds_web_returns_p2_index1 ON tpcds . web_returns_p2 ( ca_address_id ) LOCAL ;

-- 来源: 765_file_765
CREATE INDEX tpcds_web_returns_p2_index2 ON tpcds . web_returns_p2 ( ca_address_sk ) LOCAL ( PARTITION web_returns_p2_P1_index , PARTITION web_returns_p2_P2_index TABLESPACE example3 , PARTITION web_returns_p2_P3_index TABLESPACE example4 , PARTITION web_returns_p2_P4_index , PARTITION web_returns_p2_P5_index , PARTITION web_returns_p2_P6_index , PARTITION web_returns_p2_P7_index , PARTITION web_returns_p2_P8_index ) TABLESPACE example2 ;

-- 来源: 765_file_765
ALTER INDEX tpcds . tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P2_index TABLESPACE example1 ;

-- 来源: 765_file_765
ALTER INDEX tpcds . tpcds_web_returns_p2_index2 MOVE PARTITION web_returns_p2_P3_index TABLESPACE example2 ;

-- 来源: 765_file_765
ALTER INDEX tpcds . tpcds_web_returns_p2_index2 RENAME PARTITION web_returns_p2_P8_index TO web_returns_p2_P8_index_new ;

-- 来源: 765_file_765
DROP INDEX tpcds . tpcds_web_returns_p2_index1 ;

-- 来源: 765_file_765
DROP INDEX tpcds . tpcds_web_returns_p2_index2 ;

-- 来源: 765_file_765
CREATE TABLE tpcds . customer_address_bak AS TABLE tpcds . customer_address ;

-- 来源: 765_file_765
CREATE INDEX index_wr_returned_date_sk ON tpcds . customer_address_bak ( ca_address_sk );

-- 来源: 765_file_765
CREATE UNIQUE INDEX ds_ship_mode_t1_index1 ON tpcds. ship_mode_t1(SM_SHIP_MODE_SK);

-- 来源: 765_file_765
CREATE INDEX more_column_index ON tpcds . customer_address_bak ( ca_address_sk , ca_street_number );

-- 来源: 765_file_765
CREATE INDEX part_index ON tpcds . customer_address_bak ( ca_address_sk ) WHERE ca_address_sk = 5050 ;

-- 来源: 765_file_765
CREATE INDEX para_index ON tpcds . customer_address_bak ( trunc ( ca_street_number ));

-- 来源: 765_file_765
DROP TABLE tpcds . customer_address_bak ;

-- 来源: 766_file_766
CREATE OR REPLACE VIEW MyView AS SELECT * FROM tpcds . web_returns WHERE trunc ( wr_refunded_cash ) > 10000 ;

-- 来源: 766_file_766
DROP VIEW MyView ;

-- 来源: 767_file_767
CREATE TABLE T1 ( id serial , name text );

-- 来源: 767_file_767
CREATE SEQUENCE seq1 cache 100 ;

-- 来源: 767_file_767
CREATE TABLE T2 ( id int not null default nextval ( 'seq1' ), name text );

-- 来源: 767_file_767
ALTER SEQUENCE seq1 OWNED BY T2 . id ;

-- 来源: 767_file_767
CREATE SEQUENCE newSeq1 ;

-- 来源: 767_file_767
CREATE TABLE newT1 ( id int not null default nextval ( 'newSeq1' ), name text );

-- 来源: 768_file_768
CREATE TABLE test ( id int , time date );

-- 来源: 768_file_768
CREATE OR REPLACE PROCEDURE PRC_JOB_1 () AS N_NUM integer : = 1 ;

-- 来源: 991_file_991
create table t(c1 int, c2 int, c3 int)distribute by hash(c1);

-- 来源: 991_file_991
create table t1(c1 int, c2 int, c3 int)distribute by hash(c1);

-- 来源: 991_file_991
CREATE NODE GROUP ng WITH(datanode1, datanode2, datanode3, datanode4, datanode5, datanode6);

-- 来源: 991_file_991
CREATE TABLE t1(a int, b int, c int) DISTRIBUTE BY HASH(a) TO GROUP ng;

-- 来源: 991_file_991
CREATE TABLE t2(a int, b int, c int) DISTRIBUTE BY HASH(a) TO GROUP ng;

-- 来源: 991_file_991
DROP TABLE t1 ;

-- 来源: 991_file_991
DROP TABLE t2 ;

-- 来源: 991_file_991
DROP NODE GROUP ng ;

-- 来源: 991_file_991
CREATE TABLE CUSTOMER1 ( C_CUSTKEY BIGINT NOT NULL , C_NAME VARCHAR ( 25 ) NOT NULL , C_ADDRESS VARCHAR ( 40 ) NOT NULL , C_NATIONKEY INT NOT NULL , C_PHONE CHAR ( 15 ) NOT NULL , C_ACCTBAL DECIMAL ( 15 , 2 ) NOT NULL , C_MKTSEGMENT CHAR ( 10 ) NOT NULL , C_COMMENT VARCHAR ( 117 ) NOT NULL ) DISTRIBUTE BY hash ( C_CUSTKEY );

-- 来源: 991_file_991
CREATE TABLE test_stream ( a int , b float );

-- 来源: 991_file_991
CREATE TABLE sal_emp ( c1 integer [] ) DISTRIBUTE BY replication ;

-- 来源: 994_file_994
create index idx on t1 ( c2 );

