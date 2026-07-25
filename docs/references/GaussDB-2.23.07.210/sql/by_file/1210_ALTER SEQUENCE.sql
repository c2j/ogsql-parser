-- 来源: 1210_ALTER SEQUENCE.txt
-- SQL 数量: 5

CREATE SEQUENCE serial START 101 ;

CREATE TABLE t1 ( c1 bigint default nextval ( 'serial' ));

ALTER SEQUENCE serial OWNED BY t1 . c1 ;

DROP SEQUENCE serial CASCADE ;

DROP TABLE t1 ;

