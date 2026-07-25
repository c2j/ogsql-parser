-- 来源: 1350_MOVE.txt
-- SQL 数量: 11

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . reason ( r_reason_sk INTEGER NOT NULL , r_reason_id CHAR ( 16 ) NOT NULL , r_reason_desc VARCHAR ( 40 ) );

INSERT INTO tpcds . reason VALUES ( 1 , 'AAAAAAAABAAAAAAA' , 'Xxxxxxxxx' ),( 2 , 'AAAAAAAACAAAAAAA' , 'Xxxxxxxxx' ),( 3 , 'AAAAAAAADAAAAAAA' , ' Xxxxxxxxx' ),( 4 , 'AAAAAAAAEAAAAAAA' , 'Not the product that was ordered' ),( 5 , 'AAAAAAAAFAAAAAAA' , 'Parts missing' ),( 6 , 'AAAAAAAAGAAAAAAA' , 'Does not work with a product that I have' ),( 7 , 'AAAAAAAAHAAAAAAA' , 'Gift exchange' );

START TRANSACTION ;

CURSOR cursor1 FOR SELECT * FROM tpcds . reason ;

MOVE FORWARD 3 FROM cursor1 ;

FETCH 4 FROM cursor1 ;

CLOSE cursor1 ;

END ;

DROP TABLE tpcds . reason ;

DROP SCHEMA tpcds CASCADE ;

