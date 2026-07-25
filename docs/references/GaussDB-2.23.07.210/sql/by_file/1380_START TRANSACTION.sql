-- 来源: 1380_START TRANSACTION.txt
-- SQL 数量: 13

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . reason ( c1 int , c2 int );

START TRANSACTION ;

SELECT * FROM tpcds . reason ;

END ;

BEGIN ;

SELECT * FROM tpcds . reason ;

END ;

START TRANSACTION ISOLATION LEVEL READ COMMITTED READ WRITE ;

SELECT * FROM tpcds . reason ;

COMMIT ;

DROP TABLE tpcds . reason ;

DROP SCHEMA tpcds CASCADE ;

