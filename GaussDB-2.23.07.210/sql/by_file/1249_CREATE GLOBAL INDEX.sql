-- 来源: 1249_CREATE GLOBAL INDEX.txt
-- SQL 数量: 12

CREATE TABLE test1 ( c1 int , c2 int , c3 int );

CREATE GLOBAL INDEX idx_gsi_1 ON test1 ( c2 ) CONTAINING ( c3 ) DISTRIBUTE BY HASH ( c2 );

CREATE TABLE test2 ( c1 int , c2 int , c3 int );

CREATE GLOBAL INDEX idx_gsi_2 ON test2 ( c2 ) CONTAINING ( c3 ) ;

CREATE TABLE test3 ( c1 int , c2 int , c3 int );

CREATE GLOBAL UNIQUE INDEX idx_gsi_3 ON test3 ( c2 ) DISTRIBUTE BY HASH ( c2 );

DROP INDEX idx_gsi_1 ;

DROP INDEX idx_gsi_2 ;

DROP INDEX idx_gsi_3 ;

DROP TABLE test1 ;

DROP TABLE test2 ;

DROP TABLE test3 ;

