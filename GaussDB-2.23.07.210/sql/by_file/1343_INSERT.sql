-- 来源: 1343_INSERT.txt
-- SQL 数量: 14

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . reason ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

INSERT INTO tpcds . reason ( r_reason_sk , r_reason_id , r_reason_desc ) VALUES ( 0 , 'AAAAAAAAAAAAAAAA' , 'reason0' );

CREATE TABLE tpcds . reason_t2 ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

INSERT INTO tpcds . reason_t2 ( r_reason_sk , r_reason_id , r_reason_desc ) VALUES ( 1 , 'AAAAAAAABAAAAAAA' , 'reason1' );

INSERT INTO tpcds . reason_t2 VALUES ( 2 , 'AAAAAAAABAAAAAAA' , 'reason2' );

INSERT INTO tpcds . reason_t2 VALUES ( 3 , 'AAAAAAAACAAAAAAA' , 'reason3' ),( 4 , 'AAAAAAAADAAAAAAA' , 'reason4' ),( 5 , 'AAAAAAAAEAAAAAAA' , 'reason5' );

INSERT INTO tpcds . reason_t2 SELECT * FROM tpcds . reason WHERE r_reason_sk < 5 ;

CREATE UNIQUE INDEX reason_t2_u_index ON tpcds . reason_t2 ( r_reason_sk );

INSERT INTO tpcds . reason_t2 VALUES ( 5 , 'BBBBBBBBCAAAAAAA' , 'reason5' ),( 6 , 'AAAAAAAADAAAAAAA' , 'reason6' ) ON DUPLICATE KEY UPDATE r_reason_id = 'BBBBBBBBCAAAAAAA' ;

INSERT INTO tpcds . reason_t2 VALUES ( 5 , 'BBBBBBBBCAAAAAAA' , 'reason5' ) ON DUPLICATE KEY UPDATE r_reason_desc = 'reason5_new' RETURNING * ;

DROP TABLE tpcds . reason_t2 ;

DROP TABLE tpcds . reason ;

DROP SCHEMA tpcds CASCADE ;

