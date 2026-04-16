-- 来源: 2830_file_2830.txt
-- SQL 数量: 23

CREATE TABLE tpcds . case_when_t1 ( CW_COL1 INT );

INSERT INTO tpcds . case_when_t1 VALUES ( 1 ), ( 2 ), ( 3 );

SELECT * FROM tpcds . case_when_t1 ;

SELECT CW_COL1 , CASE WHEN CW_COL1 = 1 THEN 'one' WHEN CW_COL1 = 2 THEN 'two' ELSE 'other' END FROM tpcds . case_when_t1 ORDER BY 1 ;

DROP TABLE tpcds . case_when_t1 ;

SELECT DECODE ( 'A' , 'A' , 1 , 'B' , 2 , 0 );

CREATE TABLE tpcds . c_tabl ( description varchar ( 10 ), short_description varchar ( 10 ), last_value varchar ( 10 )) ;

INSERT INTO tpcds . c_tabl VALUES ( 'abc' , 'efg' , '123' );

INSERT INTO tpcds . c_tabl VALUES ( NULL , 'efg' , '123' );

INSERT INTO tpcds . c_tabl VALUES ( NULL , NULL , '123' );

SELECT description , short_description , last_value , COALESCE ( description , short_description , last_value ) FROM tpcds . c_tabl ORDER BY 1 , 2 , 3 , 4 ;

DROP TABLE tpcds . c_tabl ;

SELECT COALESCE ( NULL , 'Hello World' );

CREATE TABLE tpcds . null_if_t1 ( NI_VALUE1 VARCHAR ( 10 ), NI_VALUE2 VARCHAR ( 10 ) );

INSERT INTO tpcds . null_if_t1 VALUES ( 'abc' , 'abc' );

INSERT INTO tpcds . null_if_t1 VALUES ( 'abc' , 'efg' );

SELECT NI_VALUE1 , NI_VALUE2 , NULLIF ( NI_VALUE1 , NI_VALUE2 ) FROM tpcds . null_if_t1 ORDER BY 1 , 2 , 3 ;

DROP TABLE tpcds . null_if_t1 ;

SELECT NULLIF ( 'Hello' , 'Hello World' );

SELECT greatest ( 9000 , 155555 , 2 . 01 );

SELECT least ( 9000 , 2 );

SELECT nvl ( null , 1 );

SELECT nvl ( 'Hello World' , 1 );

