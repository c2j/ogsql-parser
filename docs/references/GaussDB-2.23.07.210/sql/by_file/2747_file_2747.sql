-- 来源: 2747_file_2747.txt
-- SQL 数量: 6

CREATE TABLE blob_type_t1 ( BT_COL1 INTEGER , BT_COL2 BLOB , BT_COL3 RAW , BT_COL4 BYTEA ) ;

INSERT INTO blob_type_t1 VALUES ( 10 , empty_blob (), HEXTORAW ( 'DEADBEEF' ), E '\\xDEADBEEF' );

SELECT * FROM blob_type_t1 ;

DROP TABLE blob_type_t1 ;

CREATE DATABASE gaussdb_m WITH dbcompatibility 'B' ;

\ c gaussdb_m --设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

