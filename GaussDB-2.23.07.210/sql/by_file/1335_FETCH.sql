-- 来源: 1335_FETCH.txt
-- SQL 数量: 21

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . customer_address ( ca_address_sk INTEGER NOT NULL , ca_address_id CHARACTER ( 16 ) NOT NULL , ca_street_number INTEGER , ca_street_name CHARACTER ( 20 ) );

INSERT INTO tpcds . customer_address VALUES ( 1 , 'AAAAAAAABAAAAAAA' , '18' , 'Jackson' ),( 2 , 'AAAAAAAACAAAAAAA' , '362' , 'Washington 6th' ),( 3 , 'AAAAAAAADAAAAAAA' , '585' , 'Dogwood Washington' );

START TRANSACTION ;

CURSOR cursor1 FOR SELECT * FROM tpcds . customer_address ORDER BY 1 ;

FETCH FORWARD 3 FROM cursor1 ;

CLOSE cursor1 ;

END ;

START TRANSACTION ;

CURSOR cursor2 FOR VALUES ( 1 , 2 ),( 0 , 3 ) ORDER BY 1 ;

FETCH FORWARD 2 FROM cursor2 ;

CLOSE cursor2 ;

END ;

START TRANSACTION ;

DECLARE cursor1 CURSOR WITH HOLD FOR SELECT * FROM tpcds . customer_address ORDER BY 1 ;

FETCH FORWARD 2 FROM cursor1 ;

END ;

FETCH FORWARD 1 FROM cursor1 ;

CLOSE cursor1 ;

DROP TABLE tpcds . customer_address ;

DROP SCHEMA tpcds CASCADE ;

