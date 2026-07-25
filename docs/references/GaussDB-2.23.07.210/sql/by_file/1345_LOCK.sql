-- 来源: 1345_LOCK.txt
-- SQL 数量: 12

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . reason ( r_reason_sk INTEGER NOT NULL , r_reason_id CHAR ( 16 ) NOT NULL , r_reason_desc INTEGER );

INSERT INTO tpcds . reason VALUES ( 1 , 'AAAAAAAABAAAAAAA' , '18' ),( 5 , 'AAAAAAAACAAAAAAA' , '362' ),( 7 , 'AAAAAAAADAAAAAAA' , '585' );

CREATE TABLE tpcds . reason_t1 AS TABLE tpcds . reason ;

START TRANSACTION ;

LOCK TABLE tpcds . reason_t1 IN SHARE ROW EXCLUSIVE MODE ;

DELETE FROM tpcds . reason_t1 WHERE r_reason_desc IN ( SELECT r_reason_desc FROM tpcds . reason_t1 WHERE r_reason_sk < 6 );

DELETE FROM tpcds . reason_t1 WHERE r_reason_sk = 7 ;

COMMIT ;

DROP TABLE tpcds . reason_t1 ;

DROP TABLE tpcds . reason ;

DROP SCHEMA tpcds CASCADE ;

