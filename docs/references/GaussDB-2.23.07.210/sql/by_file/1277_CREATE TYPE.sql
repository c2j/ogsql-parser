-- 来源: 1277_CREATE TYPE.txt
-- SQL 数量: 34

CREATE TYPE compfoo AS ( f1 int , f2 text );

CREATE TABLE t1_compfoo ( a int , b compfoo );

CREATE TABLE t2_compfoo ( a int , b compfoo );

INSERT INTO t1_compfoo values ( 1 ,( 1 , 'demo' ));

INSERT INTO t2_compfoo select * from t1_compfoo ;

SELECT ( b ). f1 FROM t1_compfoo ;

SELECT * FROM t1_compfoo t1 join t2_compfoo t2 on ( t1 . b ). f1 = ( t1 . b ). f1 ;

ALTER TYPE compfoo RENAME TO compfoo1 ;

CREATE USER usr1 PASSWORD '********' ;

ALTER TYPE compfoo1 OWNER TO usr1 ;

ALTER TYPE compfoo1 SET SCHEMA usr1 ;

ALTER TYPE usr1 . compfoo1 ADD ATTRIBUTE f3 int ;

DROP TYPE usr1 . compfoo1 CASCADE ;

DROP TABLE t1_compfoo ;

DROP TABLE t2_compfoo ;

DROP SCHEMA usr1 ;

DROP USER usr1 ;

CREATE TYPE bugstatus AS ENUM ( 'create' , 'modify' , 'closed' );

ALTER TYPE bugstatus ADD VALUE IF NOT EXISTS 'regress' BEFORE 'closed' ;

ALTER TYPE bugstatus RENAME VALUE 'create' TO 'new' ;

CREATE TYPE bugstatus_table AS TABLE OF bugstatus ;

CREATE TYPE complex ;

CREATE FUNCTION complex_in ( cstring ) RETURNS complex AS 'filename' LANGUAGE C IMMUTABLE STRICT not fenced ;

CREATE FUNCTION complex_out ( complex ) RETURNS cstring AS 'filename' LANGUAGE C IMMUTABLE STRICT not fenced ;

CREATE FUNCTION complex_recv ( internal ) RETURNS complex AS 'filename' LANGUAGE C IMMUTABLE STRICT not fenced ;

CREATE FUNCTION complex_send ( complex ) RETURNS bytea AS 'filename' LANGUAGE C IMMUTABLE STRICT not fenced ;

CREATE TYPE complex ( internallength = 16 , input = complex_in , output = complex_out , receive = complex_recv , send = complex_send , alignment = double );

DROP TYPE complex ;

DROP FUNCTION complex_send ;

DROP FUNCTION complex_recv ;

DROP FUNCTION complex_out ;

DROP FUNCTION complex_in ;

DROP TYPE bugstatus_table ;

DROP TYPE bugstatus CASCADE ;

