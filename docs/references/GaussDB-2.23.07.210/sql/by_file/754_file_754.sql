-- 来源: 754_file_754.txt
-- SQL 数量: 14

CREATE TABLESPACE ds_location1 LOCATION '/pg_location/mount1/path1' MAXSIZE '500G' ;

CREATE USER jack IDENTIFIED BY '********' ;

CREATE TABLESPACE fastspace RELATIVE LOCATION 'my_tablespace/tablespace1' ;

GRANT CREATE ON TABLESPACE fastspace TO jack ;

CREATE TABLE foo ( i int ) TABLESPACE fastspace ;

SET default_tablespace = 'fastspace' ;

CREATE TABLE foo2 ( i int );

SELECT spcname FROM pg_tablespace ;

SELECT PG_TABLESPACE_SIZE ( 'fastspace' );

ALTER TABLESPACE fastspace RENAME TO fspace ;

DROP USER jack CASCADE ;

DROP TABLE foo ;

DROP TABLE foo2 ;

DROP TABLESPACE fspace ;

