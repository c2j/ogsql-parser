-- 来源: 1383_TRUNCATE.txt
-- SQL 数量: 14

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . reason ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

INSERT INTO tpcds . reason values ( 1 , 'AAAAAAAABAAAAAAA' , 'reason 1' ),( 5 , 'AAAAAAAABAAAAAAA' , 'reason 2' ),( 15 , 'AAAAAAAABAAAAAAA' , 'reason 3' ),( 25 , 'AAAAAAAABAAAAAAA' , 'reason 4' ),( 35 , 'AAAAAAAABAAAAAAA' , 'reason 5' ),( 45 , 'AAAAAAAACAAAAAAA' , 'reason 6' ),( 55 , 'AAAAAAAACAAAAAAA' , 'reason 7' );

CREATE TABLE tpcds . reason_t1 AS TABLE tpcds . reason ;

TRUNCATE TABLE tpcds . reason_t1 ;

DROP TABLE tpcds . reason_t1 ;

CREATE TABLE tpcds . reason_p ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) ) PARTITION BY RANGE ( r_reason_sk ) ( partition p_05_before values less than ( 05 ), partition p_15 values less than ( 15 ), partition p_25 values less than ( 25 ), partition p_35 values less than ( 35 ), partition p_45_after values less than ( MAXVALUE ) );

INSERT INTO tpcds . reason_p SELECT * FROM tpcds . reason ;

ALTER TABLE tpcds . reason_p TRUNCATE PARTITION p_05_before ;

ALTER TABLE tpcds . reason_p TRUNCATE PARTITION for ( 13 );

TRUNCATE TABLE tpcds . reason_p ;

DROP TABLE tpcds . reason_p ;

DROP TABLE tpcds . reason ;

DROP SCHEMA tpcds CASCADE ;

