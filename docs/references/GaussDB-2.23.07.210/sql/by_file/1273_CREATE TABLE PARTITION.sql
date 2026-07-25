-- 来源: 1273_CREATE TABLE PARTITION.txt
-- SQL 数量: 119

CREATE SCHEMA tpcds ;

SET CURRENT_SCHEMA TO tpcds ;

CREATE TABLE tpcds . web_returns ( W_WAREHOUSE_SK INTEGER NOT NULL , W_WAREHOUSE_ID CHAR ( 16 ) NOT NULL , W_WAREHOUSE_NAME VARCHAR ( 20 ) , W_WAREHOUSE_SQ_FT INTEGER , W_STREET_NUMBER CHAR ( 10 ) , W_STREET_NAME VARCHAR ( 60 ) , W_STREET_TYPE CHAR ( 15 ) , W_SUITE_NUMBER CHAR ( 10 ) , W_CITY VARCHAR ( 60 ) , W_COUNTY VARCHAR ( 30 ) , W_STATE CHAR ( 2 ) , W_ZIP CHAR ( 10 ) , W_COUNTRY VARCHAR ( 20 ) , W_GMT_OFFSET DECIMAL ( 5 , 2 ) );

CREATE TABLE tpcds . web_returns_p1 ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL , WR_REFUNDED_CUSTOMER_SK INTEGER , WR_REFUNDED_CDEMO_SK INTEGER , WR_REFUNDED_HDEMO_SK INTEGER , WR_REFUNDED_ADDR_SK INTEGER , WR_RETURNING_CUSTOMER_SK INTEGER , WR_RETURNING_CDEMO_SK INTEGER , WR_RETURNING_HDEMO_SK INTEGER , WR_RETURNING_ADDR_SK INTEGER , WR_WEB_PAGE_SK INTEGER , WR_REASON_SK INTEGER , WR_ORDER_NUMBER BIGINT NOT NULL , WR_RETURN_QUANTITY INTEGER , WR_RETURN_AMT DECIMAL ( 7 , 2 ) , WR_RETURN_TAX DECIMAL ( 7 , 2 ) , WR_RETURN_AMT_INC_TAX DECIMAL ( 7 , 2 ) , WR_FEE DECIMAL ( 7 , 2 ) , WR_RETURN_SHIP_COST DECIMAL ( 7 , 2 ) , WR_REFUNDED_CASH DECIMAL ( 7 , 2 ) , WR_REVERSED_CHARGE DECIMAL ( 7 , 2 ) , WR_ACCOUNT_CREDIT DECIMAL ( 7 , 2 ) , WR_NET_LOSS DECIMAL ( 7 , 2 ) ) DISTRIBUTE BY HASH ( WR_ITEM_SK ) PARTITION BY RANGE ( WR_RETURNED_DATE_SK ) ( PARTITION P1 VALUES LESS THAN ( 2450815 ), PARTITION P2 VALUES LESS THAN ( 2451179 ), PARTITION P3 VALUES LESS THAN ( 2451544 ), PARTITION P4 VALUES LESS THAN ( 2451910 ), PARTITION P5 VALUES LESS THAN ( 2452275 ), PARTITION P6 VALUES LESS THAN ( 2452640 ), PARTITION P7 VALUES LESS THAN ( 2453005 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) );

INSERT INTO tpcds . web_returns_p1 SELECT * FROM tpcds . web_returns ;

ALTER TABLE tpcds . web_returns_p1 DROP PARTITION P8 ;

ALTER TABLE tpcds . web_returns_p1 ADD PARTITION P8 VALUES LESS THAN ( 2453105 );

ALTER TABLE tpcds . web_returns_p1 ADD PARTITION P9 VALUES LESS THAN ( MAXVALUE );

ALTER TABLE tpcds . web_returns_p1 DROP PARTITION FOR ( 2453005 );

ALTER TABLE tpcds . web_returns_p1 RENAME PARTITION P7 TO P10 ;

ALTER TABLE tpcds . web_returns_p1 RENAME PARTITION FOR ( 2452639 ) TO P11 ;

SELECT count ( * ) FROM tpcds . web_returns_p1 PARTITION ( P10 );

SELECT COUNT ( * ) FROM tpcds . web_returns_p1 PARTITION FOR ( 2450815 );

DROP TABLE tpcds . web_returns_p1 ;

DROP TABLE tpcds . web_returns ;

DROP SCHEMA tpcds CASCADE ;

CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1/tablespace_1' ;

CREATE TABLESPACE example2 RELATIVE LOCATION 'tablespace2/tablespace_2' ;

CREATE TABLESPACE example3 RELATIVE LOCATION 'tablespace3/tablespace_3' ;

CREATE TABLESPACE example4 RELATIVE LOCATION 'tablespace4/tablespace_4' ;

CREATE SCHEMA tpcds ;

SET CURRENT_SCHEMA TO tpcds ;

CREATE TABLE tpcds . web_returns_p2 ( WR_RETURNED_DATE_SK INTEGER , WR_RETURNED_TIME_SK INTEGER , WR_ITEM_SK INTEGER NOT NULL , WR_REFUNDED_CUSTOMER_SK INTEGER , WR_REFUNDED_CDEMO_SK INTEGER , WR_REFUNDED_HDEMO_SK INTEGER , WR_REFUNDED_ADDR_SK INTEGER , WR_RETURNING_CUSTOMER_SK INTEGER , WR_RETURNING_CDEMO_SK INTEGER , WR_RETURNING_HDEMO_SK INTEGER , WR_RETURNING_ADDR_SK INTEGER , WR_WEB_PAGE_SK INTEGER , WR_REASON_SK INTEGER , WR_ORDER_NUMBER BIGINT NOT NULL , WR_RETURN_QUANTITY INTEGER , WR_RETURN_AMT DECIMAL ( 7 , 2 ) , WR_RETURN_TAX DECIMAL ( 7 , 2 ) , WR_RETURN_AMT_INC_TAX DECIMAL ( 7 , 2 ) , WR_FEE DECIMAL ( 7 , 2 ) , WR_RETURN_SHIP_COST DECIMAL ( 7 , 2 ) , WR_REFUNDED_CASH DECIMAL ( 7 , 2 ) , WR_REVERSED_CHARGE DECIMAL ( 7 , 2 ) , WR_ACCOUNT_CREDIT DECIMAL ( 7 , 2 ) , WR_NET_LOSS DECIMAL ( 7 , 2 ) ) TABLESPACE example1 DISTRIBUTE BY HASH ( WR_ITEM_SK ) PARTITION BY RANGE ( WR_RETURNED_DATE_SK ) ( PARTITION P1 VALUES LESS THAN ( 2450815 ), PARTITION P2 VALUES LESS THAN ( 2451179 ), PARTITION P3 VALUES LESS THAN ( 2451544 ), PARTITION P4 VALUES LESS THAN ( 2451910 ), PARTITION P5 VALUES LESS THAN ( 2452275 ), PARTITION P6 VALUES LESS THAN ( 2452640 ), PARTITION P7 VALUES LESS THAN ( 2453005 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) TABLESPACE example2 ) ENABLE ROW MOVEMENT ;

CREATE TABLE tpcds . web_returns_p3 ( LIKE tpcds . web_returns_p2 INCLUDING PARTITION );

ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P1 TABLESPACE example2 ;

ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P2 TABLESPACE example3 ;

ALTER TABLE tpcds . web_returns_p2 SPLIT PARTITION P8 AT ( 2453010 ) INTO ( PARTITION P9 , PARTITION P10 );

ALTER TABLE tpcds . web_returns_p2 MERGE PARTITIONS P6 , P7 INTO PARTITION P8 ;

ALTER TABLE tpcds . web_returns_p2 DISABLE ROW MOVEMENT ;

DROP TABLE tpcds . web_returns_p1 ;

DROP TABLE tpcds . web_returns_p2 ;

DROP TABLE tpcds . web_returns_p3 ;

DROP SCHEMA tpcds CASCADE ;

DROP TABLESPACE example1 ;

DROP TABLESPACE example2 ;

DROP TABLESPACE example3 ;

DROP TABLESPACE example4 ;

CREATE TABLESPACE startend_tbs1 LOCATION '/home/omm/startend_tbs1' ;

CREATE TABLESPACE startend_tbs2 LOCATION '/home/omm/startend_tbs2' ;

CREATE TABLESPACE startend_tbs3 LOCATION '/home/omm/startend_tbs3' ;

CREATE TABLESPACE startend_tbs4 LOCATION '/home/omm/startend_tbs4' ;

CREATE SCHEMA tpcds ;

SET CURRENT_SCHEMA TO tpcds ;

CREATE TABLE tpcds . startend_pt ( c1 INT , c2 INT ) TABLESPACE startend_tbs1 DISTRIBUTE BY HASH ( c1 ) PARTITION BY RANGE ( c2 ) ( PARTITION p1 START ( 1 ) END ( 1000 ) EVERY ( 200 ) TABLESPACE startend_tbs2 , PARTITION p2 END ( 2000 ), PARTITION p3 START ( 2000 ) END ( 2500 ) TABLESPACE startend_tbs3 , PARTITION p4 START ( 2500 ), PARTITION p5 START ( 3000 ) END ( 5000 ) EVERY ( 1000 ) TABLESPACE startend_tbs4 ) ENABLE ROW MOVEMENT ;

SELECT relname , boundaries , spcname FROM pg_partition p JOIN pg_tablespace t ON p . reltablespace = t . oid and p . parentid = 'tpcds.startend_pt' :: regclass ORDER BY 1 ;

INSERT INTO tpcds . startend_pt VALUES ( GENERATE_SERIES ( 0 , 4999 ), GENERATE_SERIES ( 0 , 4999 ));

SELECT COUNT ( * ) FROM tpcds . startend_pt PARTITION FOR ( 0 );

SELECT COUNT ( * ) FROM tpcds . startend_pt PARTITION ( p3 );

ALTER TABLE tpcds . startend_pt ADD PARTITION p6 START ( 5000 ) END ( 6000 ) EVERY ( 300 ) TABLESPACE startend_tbs4 ;

ALTER TABLE tpcds . startend_pt ADD PARTITION p7 END ( MAXVALUE );

ALTER TABLE tpcds . startend_pt RENAME PARTITION p7 TO p8 ;

ALTER TABLE tpcds . startend_pt DROP PARTITION p8 ;

ALTER TABLE tpcds . startend_pt RENAME PARTITION FOR ( 5950 ) TO p71 ;

ALTER TABLE tpcds . startend_pt SPLIT PARTITION FOR ( 4500 ) INTO ( PARTITION q1 START ( 4000 ) END ( 5000 ) EVERY ( 250 ) TABLESPACE startend_tbs3 );

ALTER TABLE tpcds . startend_pt MOVE PARTITION p2 TABLESPACE startend_tbs4 ;

SELECT relname , boundaries , spcname FROM pg_partition p JOIN pg_tablespace t ON p . reltablespace = t . oid and p . parentid = 'tpcds.startend_pt' :: regclass ORDER BY 1 ;

DROP TABLE tpcds . startend_pt ;

DROP SCHEMA tpcds CASCADE ;

DROP TABLESPACE startend_tbs1 ;

DROP TABLESPACE startend_tbs2 ;

DROP TABLESPACE startend_tbs3 ;

DROP TABLESPACE startend_tbs4 ;

CREATE TABLE test_list ( col1 int , col2 int ) partition by list ( col1 ) ( partition p1 values ( 2000 ), partition p2 values ( 3000 ), partition p3 values ( 4000 ), partition p4 values ( 5000 ) );

INSERT INTO test_list VALUES ( 2000 , 2000 );

INSERT INTO test_list VALUES ( 3000 , 3000 );

SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

INSERT INTO test_list VALUES ( 6000 , 6000 );

ALTER TABLE test_list add partition p5 values ( 6000 );

SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

INSERT INTO test_list VALUES ( 6000 , 6000 );

CREATE TABLE t1 ( col1 int , col2 int );

SELECT * FROM test_list partition ( p1 );

ALTER TABLE test_list exchange partition ( p1 ) with table t1 ;

SELECT * FROM test_list partition ( p1 );

SELECT * FROM t1 ;

SELECT * FROM test_list partition ( p2 );

ALTER TABLE test_list truncate partition p2 ;

SELECT * FROM test_list partition ( p2 );

alter table test_list drop partition p5 ;

SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

INSERT INTO test_list VALUES ( 6000 , 6000 );

alter table test_list merge partitions p1 , p2 into partition p2 ;

SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

alter table test_list split partition p2 values ( 2000 ) into ( partition p1 , partition p2 );

SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_list' AND t1 . parttype = 'p' ;

drop table test_list ;

DROP TABLE t1 ;

create table test_hash ( col1 int , col2 int ) partition by hash ( col1 ) ( partition p1 , partition p2 );

INSERT INTO test_hash VALUES ( 1 , 1 );

INSERT INTO test_hash VALUES ( 2 , 2 );

INSERT INTO test_hash VALUES ( 3 , 3 );

INSERT INTO test_hash VALUES ( 4 , 4 );

SELECT t1 . relname , partstrategy , boundaries FROM pg_partition t1 , pg_class t2 WHERE t1 . parentid = t2 . oid AND t2 . relname = 'test_hash' AND t1 . parttype = 'p' ;

select * from test_hash partition ( p1 );

select * from test_hash partition ( p2 );

create table t1 ( col1 int , col2 int );

alter table test_hash exchange partition ( p1 ) with table t1 ;

select * from test_hash partition ( p1 );

select * from t1 ;

alter table test_hash truncate partition p2 ;

select * from test_hash partition ( p2 );

drop table test_hash ;

CREATE TABLE t_multi_keys_list ( a int , b varchar ( 4 ), c int ) PARTITION BY LIST ( a , b ) ( PARTITION p1 VALUES ( ( 0 , NULL ) ), PARTITION p2 VALUES ( ( 0 , '1' ), ( 0 , '2' ), ( 0 , '3' ), ( 1 , '1' ), ( 1 , '2' ) ), PARTITION p3 VALUES ( ( NULL , '0' ), ( 2 , '1' ) ), PARTITION p4 VALUES ( ( 3 , '2' ), ( NULL , NULL ) ), PARTITION pd VALUES ( DEFAULT ) );

DROP TABLE t_multi_keys_list ;

ALTER DATABASE set ilm = on ;

CREATE TABLE ilm_part ( a int ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION , PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

DROP TABLE ilm_part ;

ALTER DATABASE set ilm = on ;

CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

DROP TABLE ilm_part ;

ALTER DATABASE set ilm = on ;

CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

DROP TABLE ilm_part ;

ALTER DATABASE set ilm = on ;

CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

DROP TABLE ilm_part ;

ALTER DATABASE set ilm = on ;

CREATE TABLE ilm_part ( a int ) PARTITION BY RANGE ( a ) ( PARTITION p1 VALUES LESS THAN ( 10 ), PARTITION p2 VALUES LESS THAN ( 20 ), PARTITION p3 VALUES LESS THAN ( 30 ));

DROP TABLE ilm_part ;

