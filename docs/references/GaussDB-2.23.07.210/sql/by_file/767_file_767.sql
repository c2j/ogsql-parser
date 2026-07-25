-- 来源: 767_file_767.txt
-- SQL 数量: 9

CREATE TABLE T1 ( id serial , name text );

CREATE SEQUENCE seq1 cache 100 ;

CREATE TABLE T2 ( id int not null default nextval ( 'seq1' ), name text );

ALTER SEQUENCE seq1 OWNED BY T2 . id ;

CREATE SEQUENCE newSeq1 ;

CREATE TABLE newT1 ( id int not null default nextval ( 'newSeq1' ), name text );

INSERT INTO newT1 ( name ) SELECT name FROM T1 ;

INSERT INTO newT1 ( id , name ) SELECT id , name FROM T1 ;

SELECT SETVAL ( 'newSeq1' , 10000 );

