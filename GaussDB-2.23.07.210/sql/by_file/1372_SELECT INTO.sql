-- 来源: 1372_SELECT INTO.txt
-- SQL 数量: 6

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . reason ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

INSERT INTO tpcds . reason values ( 1 , 'AAAAAAAABAAAAAAA' , 'reason 1' ),( 2 , 'AAAAAAAABAAAAAAA' , 'reason 2' ),( 3 , 'AAAAAAAABAAAAAAA' , 'reason 3' ),( 4 , 'AAAAAAAABAAAAAAA' , 'reason 4' ),( 4 , 'AAAAAAAABAAAAAAA' , 'reason 5' ),( 4 , 'AAAAAAAACAAAAAAA' , 'reason 6' ),( 5 , 'AAAAAAAACAAAAAAA' , 'reason 7' );

SELECT * INTO tpcds . reason_t1 FROM tpcds . reason WHERE r_reason_sk < 5 ;

DROP TABLE tpcds . reason_t1 , tpcds . reason ;

DROP SCHEMA tpcds CASCADE ;

