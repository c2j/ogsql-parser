-- 来源: 1328_EXECUTE.txt
-- SQL 数量: 9

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . reason ( CD_DEMO_SK INTEGER NOT NULL , CD_GENDER character ( 16 ) , CD_MARITAL_STATUS character ( 100 ) );

INSERT INTO tpcds . reason VALUES ( 51 , 'AAAAAAAADDAAAAAA' , 'reason 51' );

CREATE TABLE tpcds . reason_t1 AS TABLE tpcds . reason ;

PREPARE insert_reason ( integer , character ( 16 ), character ( 100 )) AS INSERT INTO tpcds . reason_t1 VALUES ( $ 1 , $ 2 , $ 3 );

EXECUTE insert_reason ( 52 , 'AAAAAAAADDAAAAAA' , 'reason 52' );

DROP TABLE tpcds . reason ;

DROP TABLE tpcds . reason_t1 ;

DROP SCHEMA tpcds CASCADE ;

