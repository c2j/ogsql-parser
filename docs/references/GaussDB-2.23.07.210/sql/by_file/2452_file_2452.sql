-- 来源: 2452_file_2452.txt
-- SQL 数量: 13

CREATE USER jack IDENTIFIED BY '********' ;

CREATE TABLESPACE fastspace RELATIVE LOCATION 'tablespace/tablespace_1' ;

GRANT CREATE ON TABLESPACE fastspace TO jack ;

CREATE TABLE foo ( i int ) TABLESPACE fastspace ;

SET default_tablespace = 'fastspace' ;

CREATE TABLE foo2 ( i int );

SELECT spcname FROM pg_tablespace ;

SELECT PG_TABLESPACE_SIZE ( 'example' );

ALTER TABLESPACE fastspace RENAME TO fspace ;

DROP USER jack CASCADE ;

DROP TABLE foo ;

DROP TABLE foo2 ;

DROP TABLESPACE fspace ;

