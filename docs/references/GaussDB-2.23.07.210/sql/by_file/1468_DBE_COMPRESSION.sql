-- 来源: 1468_DBE_COMPRESSION.txt
-- SQL 数量: 16

create database ilmtabledb with dbcompatibility = 'ORA' ;

\ c ilmtabledb

ALTER DATABASE set ilm = on ;

CREATE user user1 IDENTIFIED BY 'Gauss_123' ;

SET ROLE user1 PASSWORD 'Gauss_123' ;

CREATE TABLE TEST_DATA ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) with ( storage_type = astore ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

INSERT INTO TEST_DATA VALUES ( 1 , '零食大礼包A' , NOW ());

DECLARE o_blkcnt_cmp integer ;

create database ilmtabledb with dbcompatibility = 'ORA' ;

\ c ilmtabledb

alter database set ilm = on ;

CREATE user user1 IDENTIFIED BY 'Gauss_1234' ;

SET ROLE user1 PASSWORD 'Gauss_1234' ;

CREATE TABLE TEST_DATA ( ORDER_ID INT , GOODS_NAME TEXT , CREATE_TIME TIMESTAMP ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 1 DAYS OF NO MODIFICATION ;

INSERT INTO TEST_DATA VALUES ( 1 , '零食大礼包A' , NOW ());

SELECT DBE_COMPRESSION . GET_COMPRESSION_TYPE ( 'user1' , 'test_data' , '(0,1)' , NULL );

