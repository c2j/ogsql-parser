-- 来源: 1084_SEQUENCE.txt
-- SQL 数量: 23

CREATE SEQUENCE seqDemo ;

SELECT nextval ( 'seqDemo' );

SELECT seqDemo . nextval ;

DROP SEQUENCE seqDemo ;

CREATE SEQUENCE seq1 ;

SELECT nextval ( 'seq1' );

SET enable_beta_features = on ;

SELECT currval ( 'seq1' );

SELECT seq1 . currval seq1 ;

DROP SEQUENCE seq1 ;

CREATE SEQUENCE seq1 ;

SELECT nextval ( 'seq1' );

SET enable_beta_features = on ;

SELECT lastval ();

DROP SEQUENCE seq1 ;

CREATE SEQUENCE seqDemo ;

SELECT nextval ( 'seqDemo' );

SELECT setval ( 'seqDemo' , 3 );

DROP SEQUENCE seqDemo ;

CREATE SEQUENCE seqDemo ;

SELECT nextval ( 'seqDemo' );

SELECT setval ( 'seqDemo' , 5 , true );

DROP SEQUENCE seqDemo ;

