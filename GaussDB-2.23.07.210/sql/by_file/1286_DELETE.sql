-- 来源: 1286_DELETE.txt
-- SQL 数量: 9

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . customer_address ( ca_address_sk INTEGER NOT NULL , ca_address_id CHARACTER ( 16 ) NOT NULL , ca_street_number INTEGER , ca_street_name CHARACTER ( 20 ) );

INSERT INTO tpcds . customer_address VALUES ( 1 , 'AAAAAAAABAAAAAAA' , '18' , 'Jackson' ),( 10000 , 'AAAAAAAACAAAAAAA' , '362' , 'Washington 6th' ),( 15000 , 'AAAAAAAADAAAAAAA' , '585' , 'Dogwood Washington' );

CREATE TABLE tpcds . customer_address_bak AS TABLE tpcds . customer_address ;

DELETE FROM tpcds . customer_address_bak WHERE ca_address_sk < 14888 ;

DELETE FROM tpcds . customer_address_bak ;

DROP TABLE tpcds . customer_address_bak ;

DROP TABLE tpcds . customer_address ;

DROP SCHEMA tpcds CASCADE ;

