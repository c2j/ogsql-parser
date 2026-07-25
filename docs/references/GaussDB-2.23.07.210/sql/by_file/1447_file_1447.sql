-- 来源: 1447_file_1447.txt
-- SQL 数量: 20

CREATE OR REPLACE PROCEDURE proc_loop ( i in integer , count out integer ) AS BEGIN count : = 0 ;

CALL proc_loop ( 10 , 5 );

CREATE TABLE integertable ( c1 integer ) DISTRIBUTE BY hash ( c1 );

CREATE OR REPLACE PROCEDURE proc_while_loop ( maxval in integer ) AS DECLARE i int : = 1 ;

CALL proc_while_loop ( 10 );

DROP PROCEDURE proc_while_loop ;

DROP TABLE integertable ;

CREATE OR REPLACE PROCEDURE proc_for_loop () AS BEGIN FOR I IN 0 .. 5 LOOP DBE_OUTPUT . PRINT_LINE ( 'It is ' || to_char ( I ) || ' time;

CALL proc_for_loop ();

DROP PROCEDURE proc_for_loop ;

CREATE OR REPLACE PROCEDURE proc_for_loop_query () AS record VARCHAR2 ( 50 );

CALL proc_for_loop_query ();

DROP PROCEDURE proc_for_loop_query ;

CREATE TABLE TEST_t1 ( title NUMBER ( 6 ), did VARCHAR2 ( 20 ), data_period VARCHAR2 ( 25 ), kind VARCHAR2 ( 25 ), interval VARCHAR2 ( 20 ), time DATE , isModified VARCHAR2 ( 10 ) ) DISTRIBUTE BY hash ( did );

INSERT INTO TEST_t1 VALUES ( 8 , 'Donald' , 'OConnell' , 'DOCONNEL' , '650.507.9833' , to_date ( '21-06-1999' , 'dd-mm-yyyy' ), 'SH_CLERK' );

CREATE OR REPLACE PROCEDURE proc_forall () AS BEGIN FORALL i IN 100 .. 120 update TEST_t1 set title = title + 100 * i ;

CALL proc_forall ();

SELECT * FROM TEST_t1 ;

DROP PROCEDURE proc_forall ;

DROP TABLE TEST_t1 ;

