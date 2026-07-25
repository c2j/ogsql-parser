-- 来源: 1369_SAVEPOINT.txt
-- SQL 数量: 19

CREATE TABLE table1 ( a int );

START TRANSACTION ;

INSERT INTO table1 VALUES ( 1 );

SAVEPOINT my_savepoint ;

INSERT INTO table1 VALUES ( 2 );

ROLLBACK TO SAVEPOINT my_savepoint ;

INSERT INTO table1 VALUES ( 3 );

COMMIT ;

SELECT * FROM table1 ;

DROP TABLE table1 ;

CREATE TABLE table2 ( a int );

START TRANSACTION ;

INSERT INTO table2 VALUES ( 3 );

SAVEPOINT my_savepoint ;

INSERT INTO table2 VALUES ( 4 );

RELEASE SAVEPOINT my_savepoint ;

COMMIT ;

SELECT * FROM table2 ;

DROP TABLE table2 ;

