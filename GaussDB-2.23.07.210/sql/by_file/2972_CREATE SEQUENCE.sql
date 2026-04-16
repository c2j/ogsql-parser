-- 来源: 2972_CREATE SEQUENCE.txt
-- SQL 数量: 11

CREATE SEQUENCE seq1 START 101 INCREMENT 10 ;

SELECT nextval ( 'seq1' );

SELECT nextval ( 'seq1' );

DROP SEQUENCE seq1 ;

CREATE TABLE test1 ( id int PRIMARY KEY , name varchar ( 20 ));

CREATE SEQUENCE test_seq2 START 1 NO CYCLE OWNED BY test1 . id ;

ALTER TABLE test1 ALTER COLUMN id SET DEFAULT nextval ( 'test_seq2' :: regclass );

INSERT INTO test1 ( name ) values ( 'Joe' ),( 'Scott' ),( 'Ben' );

SELECT * FROM test1 ;

DROP SEQUENCE test_seq2 CASCADE ;

DROP TABLE test1 ;

