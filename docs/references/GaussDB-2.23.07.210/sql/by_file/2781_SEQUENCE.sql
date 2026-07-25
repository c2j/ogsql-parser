-- 来源: 2781_SEQUENCE.txt
-- SQL 数量: 23

CREATE SEQUENCE seqDemo ;

SELECT nextval ( 'seqDemo' );

SELECT seqDemo . nextval ;

DROP SEQUENCE seqDemo ;

CREATE SEQUENCE seq1 ;

SELECT nextval ( 'seq1' );

SELECT currval ( 'seq1' );

SELECT seq1 . currval ;

DROP SEQUENCE seq1 ;

CREATE SEQUENCE seq1 ;

SELECT nextval ( 'seq1' );

SELECT lastval ();

DROP SEQUENCE seq1 ;

CREATE SEQUENCE seqDemo ;

SELECT nextval ( 'seqDemo' );

SELECT setval ( 'seqDemo' , 5 );

DROP SEQUENCE seqDemo ;

CREATE SEQUENCE seqDemo ;

SELECT nextval ( 'seqDemo' );

SELECT setval ( 'seqDemo' , 5 , true );

DROP SEQUENCE seqDemo ;

SELECT last_insert_id ( 100 );

SELECT last_insert_id ();

