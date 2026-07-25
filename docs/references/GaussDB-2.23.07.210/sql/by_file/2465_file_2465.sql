-- 来源: 2465_file_2465.txt
-- SQL 数量: 4

CREATE TABLE T1 ( id serial , name text );

CREATE SEQUENCE seq1 cache 100 ;

CREATE TABLE T2 ( id int not null default nextval ( 'seq1' ), name text );

ALTER SEQUENCE seq1 OWNED BY T2 . id ;

