-- 来源: 1361_RELEASE SAVEPOINT.txt
-- SQL 数量: 11

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . table1 ( a int );

START TRANSACTION ;

INSERT INTO tpcds . table1 VALUES ( 3 );

SAVEPOINT my_savepoint ;

INSERT INTO tpcds . table1 VALUES ( 4 );

RELEASE SAVEPOINT my_savepoint ;

COMMIT ;

SELECT * FROM tpcds . table1 ;

DROP TABLE tpcds . table1 ;

DROP SCHEMA tpcds CASCADE ;

