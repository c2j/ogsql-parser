-- 来源: 1387_VACUUM.txt
-- SQL 数量: 8

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . reason ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

INSERT INTO tpcds . reason values ( 1 , 'AAAAAAAABAAAAAAA' , 'reason 1' ),( 2 , 'AAAAAAAABAAAAAAA' , 'reason 2' );

CREATE UNIQUE INDEX ds_reason_index1 ON tpcds . reason ( r_reason_sk );

VACUUM ( VERBOSE , ANALYZE ) tpcds . reason ;

DROP INDEX tpcds . ds_reason_index1 CASCADE ;

DROP TABLE tpcds . reason ;

DROP SCHEMA tpcds CASCADE ;

