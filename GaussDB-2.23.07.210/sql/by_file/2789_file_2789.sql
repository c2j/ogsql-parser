-- 来源: 2789_file_2789.txt
-- SQL 数量: 30

SELECT coalesce ( NULL , 'hello' );

SELECT decode ( 'A' , 'A' , 1 , 'B' , 2 , 0 );

SELECT nullif ( 'hello' , 'world' );

SELECT nullif ( '1234' :: VARCHAR , 123 :: INT4 );

SELECT nullif ( '1234' :: VARCHAR , '2012-12-24' :: DATE );

SELECT nullif ( 1 :: bit , '1' :: MONEY );

SELECT nvl ( 'hello' , 'world' );

SELECT nvl2 ( 'hello' , 'world' , 'other' );

SELECT greatest ( 1 * 2 , 2 - 3 , 4 - 1 );

SELECT greatest ( 'HARRY' , 'HARRIOT' , 'HAROLD' );

SELECT least ( 1 * 2 , 2 - 3 , 4 - 1 );

SELECT least ( 'HARRY' , 'HARRIOT' , 'HAROLD' );

CREATE TABLE blob_tb ( b blob , id int );

INSERT INTO blob_tb VALUES ( empty_blob (), 1 );

DROP TABLE blob_tb ;

CREATE TABLE clob_tb ( c clob , id int );

INSERT INTO clob_tb VALUES ( empty_clob (), 1 );

DROP TABLE clob_tb ;

CREATE TABLE student_demo ( name VARCHAR2 ( 20 ), grade NUMBER ( 10 , 2 ));

INSERT INTO student_demo VALUES ( 'name0' , 0 );

INSERT INTO student_demo VALUES ( 'name1' , 1 );

INSERT INTO student_demo VALUES ( 'name2' , 2 );

SELECT * FROM student_demo WHERE LNNVL ( name = 'name1' );

SELECT isnull ( null );

SELECT isnull ( 1 );

select if ( 2 > 3 , 'true' , 'false' );

select if ( null , 'not null' , 'is null' );

select ifnull ( '' , null ) is null as a ;

select ifnull ( null , null ) is null as a ;

select ifnull ( null , 'A' ) as a ;

