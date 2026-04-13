-- 来源: 1270_CREATE TABLE.txt
-- SQL 数量: 145

SELECT a.count,b.node_name FROM (SELECT count(*) AS count,xc_node_id FROM tablename GROUP BY xc_node_id) a, pgxc_node b WHERE a.xc_node_id=b.node_id ORDER BY a.count DESC;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t1 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

CREATE TABLE tpcds . warehouse_t2 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ), W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

DROP TABLE tpcds . warehouse_t2 ;

DROP TABLE tpcds . warehouse_t1 ;

DROP SCHEMA tpcds ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t3 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) DEFAULT 'GA' , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

DROP TABLE tpcds . warehouse_t3 ;

DROP SCHEMA tpcds ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t4 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) UNIQUE DEFERRABLE , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

DROP TABLE tpcds . warehouse_t4 ;

DROP SCHEMA tpcds ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t5 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), UNIQUE ( W_WAREHOUSE_NAME ) WITH ( fillfactor = 70 ) );

DROP TABLE tpcds . warehouse_t5 ;

DROP SCHEMA tpcds ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t6 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) UNIQUE , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) WITH ( fillfactor = 70 );

DROP TABLE tpcds . warehouse_t6 ;

DROP SCHEMA tpcds ;

CREATE SCHEMA tpcds ;

CREATE UNLOGGED TABLE tpcds . warehouse_t7 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_

DROP TABLE tpcds . warehouse_t7 ;

DROP SCHEMA tpcds ;

CREATE TEMPORARY TABLE warehouse_t24 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

CREATE TEMPORARY TABLE warehouse_t25 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) ON COMMIT DELETE ROWS ;

CREATE SCHEMA tpcds ;

CREATE TABLE IF NOT EXISTS tpcds . warehouse_t8 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

DROP TABLE tpcds . warehouse_t8 ;

DROP SCHEMA tpcds ;

CREATE TABLESPACE DS_TABLESPACE1 RELATIVE LOCATION 'tablespace/tablespace_1' ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t9 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) TABLESPACE DS_TABLESPACE1 ;

DROP TABLE tpcds . warehouse_t9 ;

DROP SCHEMA tpcds ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t10 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) UNIQUE USING INDEX TABLESPACE DS_TABLESPACE1 , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

DROP TABLE tpcds . warehouse_t10 ;

DROP SCHEMA tpcds ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t11 ( W_WAREHOUSE_SK INTEGER PRIMARY KEY , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

DROP TABLE tpcds . warehouse_t11 ;

DROP SCHEMA tpcds ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t12 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), PRIMARY KEY ( W_WAREHOUSE_SK ) );

DROP TABLE tpcds . warehouse_t12 ;

DROP SCHEMA tpcds ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t13 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), CONSTRAINT W_CSTR_KEY1 PRIMARY KEY ( W_WAREHOUSE_SK ) );

DROP TABLE tpcds . warehouse_t13 ;

DROP SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t13_1 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), CONSTRAINT PRIMARY KEY USING BTREE ( W_WAREHOUSE_SK DESC ) );

DROP TABLE tpcds . warehouse_t13_1 ;

DROP SCHEMA tpcds ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t14 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), CONSTRAINT W_CSTR_KEY2 PRIMARY KEY ( W_WAREHOUSE_SK , W_WAREHOUSE_ID ) );

DROP TABLE tpcds . warehouse_t14 ;

DROP SCHEMA tpcds ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t19 ( W_WAREHOUSE_SK INTEGER PRIMARY KEY CHECK ( W_WAREHOUSE_SK > 0 ), W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) CHECK ( W_WAREHOUSE_NAME IS NOT NULL ), W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

CREATE TABLE tpcds . warehouse_t20 ( W_WAREHOUSE_SK INTEGER PRIMARY KEY , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) CHECK ( W_WAREHOUSE_NAME IS NOT NULL ), W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), CONSTRAINT W_CONSTR_KEY2 CHECK ( W_WAREHOUSE_SK > 0 AND W_WAREHOUSE_NAME IS NOT NULL ) );

ALTER TABLE tpcds . warehouse_t19 ADD W_GOODS_CATEGORY varchar ( 30 );

ALTER TABLE tpcds . warehouse_t19 ADD CONSTRAINT W_CONSTR_KEY4 CHECK ( W_STATE IS NOT NULL );

ALTER TABLE tpcds . warehouse_t19 ALTER COLUMN W_GOODS_CATEGORY TYPE varchar ( 80 ), ALTER COLUMN W_STREET_NAME TYPE varchar ( 100 );

ALTER TABLE tpcds . warehouse_t19 MODIFY ( W_GOODS_CATEGORY varchar ( 30 ), W_STREET_NAME varchar ( 60 ));

ALTER TABLE tpcds . warehouse_t19 ALTER COLUMN W_GOODS_CATEGORY SET NOT NULL ;

ALTER TABLE tpcds . warehouse_t19 ALTER COLUMN W_GOODS_CATEGORY DROP NOT NULL ;

ALTER TABLE tpcds . warehouse_t19 SET TABLESPACE PG_DEFAULT ;

CREATE SCHEMA joe ;

ALTER TABLE tpcds . warehouse_t19 SET SCHEMA joe ;

ALTER TABLE joe . warehouse_t19 RENAME TO warehouse_t23 ;

ALTER TABLE joe . warehouse_t23 DROP COLUMN W_STREET_NAME ;

DROP TABLESPACE DS_TABLESPACE1 ;

DROP SCHEMA IF EXISTS joe CASCADE ;

DROP TABLE tpcds . warehouse_t20 ;

DROP SCHEMA tpcds ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t21 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) DISTRIBUTE BY REPLICATION ;

ALTER TABLE tpcds . warehouse_t21 SET ( primarynode = on );

\ d + tpcds . warehouse_t21 Table "tpcds.warehouse_t21" Column | Type | Modifiers | Storage | Stats target | Description -------------------+-----------------------+-----------+----------+--------------+------------- w_warehouse_sk | integer | not null | plain | | w_warehouse_id | character ( 16 ) | not null | extended | | w_warehouse_name | character varying ( 20 ) | | extended | | w_warehouse_sq_ft | integer | | plain | | w_street_number | character ( 10 ) | | extended | | w_street_name | character varying ( 60 ) | | extended | | w_street_type | character ( 15 ) | | extended | | w_suite_number | character ( 10 ) | | extended | | w_city | character varying ( 60 ) | | extended | | w_county | character varying ( 30 ) | | extended | | w_state | character ( 2 ) | | extended | | w_zip | character ( 10 ) | | extended | | w_country | character varying ( 20 ) | | extended | | w_gmt_offset | numeric ( 5 , 2 ) | | main | | Has OIDs : no Distribute By : REPLICATION Location Nodes : ALL DATANODES Options : orientation = row , logical_repl_node =- 1 , compression = no , primarynode = on

DROP TABLE tpcds . warehouse_t21 ;

DROP SCHEMA tpcds ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t22 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ), CONSTRAINT W_CONSTR_KEY3 UNIQUE ( W_WAREHOUSE_SK ) ) DISTRIBUTE BY HASH ( W_WAREHOUSE_SK );

DROP TABLE tpcds . warehouse_t22 ;

DROP SCHEMA tpcds ;

SELECT node_name FROM pgxc_node ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t26 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) DISTRIBUTE BY RANGE ( W_WAREHOUSE_ID ) ( SLICE s1 VALUES LESS THAN ( 10 ) DATANODE datanode1 , SLICE s2 VALUES LESS THAN ( 20 ) DATANODE datanode2 , SLICE s3 VALUES LESS THAN ( 30 ) DATANODE datanode3 , SLICE s4 VALUES LESS THAN ( MAXVALUE ) DATANODE datanode4 );

DROP TABLE tpcds . warehouse_t26 ;

DROP SCHEMA tpcds ;

CREATE TABLE lrt_range ( f_int1 int , f_int2 int , f_varchar1 varchar2 ( 100 )) distribute by range ( f_int1 , f_int2 ) ( slice s1 values less than ( 100 , 100 ) datanode ( datanode1 , datanode2 ), slice s2 values less than ( 200 , 200 ) datanode datanode2 , slice s3 values less than ( 300 , 300 ) datanode datanode2 , slice s4 values less than ( maxvalue , maxvalue ) datanode ( datanode1 , datanode2 ) );

INSERT INTO lrt_range VALUES ( generate_series ( 1 , 4 ), generate_series ( 1 , 4 ));

SELECT node_name , node_type , node_id FROM pgxc_node ;

SELECT xc_node_id , * FROM lrt_range ;

CREATE TABLE t_news ( county varchar ( 30 ), year varchar ( 60 ), name varchar ( 60 ), age int , news text ) distribute by list ( county , year ) ( slice s1 values (( 'china' , '2020' ),( 'china' , '2021' )) datanode ( datanode1 , datanode2 ), slice s2 values (( 'china' , '2022' ),( 'china' , '2023' ),( 'china' , '2024' )) datanode ( datanode1 , datanode2 ), slice s3 values (( 'china' , '2025' )) datanode ( datanode1 , datanode2 ), slice s4 values (( 'canada' , '2021' )) datanode datanode1 , slice s5 values (( 'canada' , '2022' )) datanode datanode2 , slice s6 values (( 'canada' , '2023' )) datanode datanode1 , slice s7 values (( 'uk' , '2021' )) datanode datanode1 , slice s8 values (( 'uk' , '2022' )) datanode datanode2 , slice s9 values (( 'uk' , '2023' )) datanode datanode1 , slice s0 values ( default ) datanode ( datanode1 , datanode2 ) );

INSERT INTO t_news values ( 'china' , '2020' , '张三' , 21 );

INSERT INTO t_news values ( 'china' , '2021' , '张三' , 21 );

INSERT INTO t_news values ( 'china' , '2022' , '张三' , 21 );

INSERT INTO t_news values ( 'china' , '2023' , '张三' , 21 );

INSERT INTO t_news values ( 'china' , '2024' , '张三' , 21 );

INSERT INTO t_news values ( 'china' , '2025' , '张三' , 21 );

SELECT node_name , node_type , node_id FROM pgxc_node ;

SELECT xc_node_id , * FROM t_news ;

DELETE FROM t_news ;

INSERT INTO t_news values ( 'Japan' , '2020' , '赵六' , 18 ),( 'Japan' , '2021' , '赵六' , 19 ),( 'Japan' , '2022' , '赵六' , 20 ),( 'Japan' , '2027' , '赵六' , 21 );

SELECT xc_node_id , * FROM t_news ;

CREATE TABLE t_ran1 ( c1 int , c2 int , c3 int , c4 int , c5 int ) distribute by range ( c1 , c2 ) ( SLICE s1 VALUES LESS THAN ( 10 , 10 ) DATANODE datanode1 , SLICE s2 VALUES LESS THAN ( 10 , 20 ) DATANODE datanode2 , SLICE s3 VALUES LESS THAN ( 20 , 10 ) DATANODE datanode3 );

INSERT INTO t_ran1 values ( 9 , 5 , 'a' );

INSERT INTO t_ran1 values ( 9 , 20 , 'a' );

INSERT INTO t_ran1 values ( 9 , 21 , 'a' );

INSERT INTO t_ran1 values ( 10 , 5 , 'a' );

INSERT INTO t_ran1 values ( 10 , 15 , 'a' );

INSERT INTO t_ran1 values ( 10 , 20 , 'a' );

INSERT INTO t_ran1 values ( 10 , 21 , 'a' );

INSERT INTO t_ran1 values ( 11 , 5 , 'a' );

INSERT INTO t_ran1 values ( 11 , 20 , 'a' );

INSERT INTO t_ran1 values ( 11 , 21 , 'a' );

SELECT node_name , node_type , node_id FROM pgxc_node ;

SELECT xc_node_id , * FROM t_ran1 ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t27 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) DISTRIBUTE BY RANGE ( W_WAREHOUSE_ID ) SLICE REFERENCES warehouse_t26 ;

DROP TABLE tpcds . warehouse_t27 ;

DROP SCHEMA tpcds ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . warehouse_t28 ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) ) DISTRIBUTE BY LIST ( W_COUNTRY ) ( SLICE s1 VALUES ( 'USA' ) DATANODE datanode1 , SLICE s2 VALUES ( 'CANADA' ) DATANODE datanode2 , SLICE s3 VALUES ( 'UK' ) DATANODE datanode3 , SLICE s4 VALUES ( DEFAULT ) DATANODE datanode4 );

DROP TABLE tpcds . warehouse_t28 ;

DROP SCHEMA tpcds ;

CREATE TABLE creditcard_info ( id_number int , name text encrypted with ( column_encryption_key = ImgCEK , encryption_type = DETERMINISTIC ), credit_card varchar ( 19 ) encrypted with ( column_encryption_key = ImgCEK1 , encryption_type = DETERMINISTIC ));

show server_encoding ;

show sql_compatibility ;

CREATE TABLE t1 ( c1 text , c2 text charset utf8mb4 collate utf8mb4_unicode_ci ) charset utf8mb4 collate utf8mb4_bin ;

ALTER TABLE t1 charset utf8mb4 collate utf8mb4_general_ci ;

ALTER TABLE t1 add c3 varchar ( 20 ) charset utf8mb4 collate utf8mb4_bin ;

SET b_format_version = '5.7' ;

SET b_format_dev_version = 's1' ;

CREATE TABLE t1_on_update ( TS0 TIMESTAMP ON UPDATE CURRENT_TIMESTAMP , TS1 TIMESTAMP ON UPDATE CURRENT_TIMESTAMP () , TS2 TIMESTAMP ( 6 ) ON UPDATE CURRENT_TIMESTAMP ( 6 ) , DT0 DATETIME ON UPDATE LOCALTIMESTAMP , DT1 DATETIME ON UPDATE NOW () , IN0 INT ) DISTRIBUTE BY HASH ( IN0 );

ALTER TABLE t1_on_update ADD TS3 timestamp ON UPDATE CURRENT_TIMESTAMP ;

CREATE DATABASE ilmtabledb with dbcompatibility = 'ORA' ;

\ c ilmtabledb

ALTER DATABASE ilmtabledb TO GROUP GROUP1 ;

CREATE TABLE ilm_table ( a int ) WITH ( STORAGE_TYPE = ASTORE ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION ON ( a != 0 );

