-- 来源: 1162_file_1162.txt
-- SQL 数量: 8

SELECT ts_rewrite ( 'a & b' :: tsquery , 'a' :: tsquery , 'c' :: tsquery );

CREATE TABLE tsearch . aliases ( id int , t tsquery , s tsquery );

INSERT INTO tsearch . aliases VALUES ( 1 , to_tsquery ( 'supernovae' ), to_tsquery ( 'supernovae|sn' ));

SELECT ts_rewrite ( to_tsquery ( 'supernovae & crab' ), 'SELECT t, s FROM tsearch.aliases' );

UPDATE tsearch . aliases SET s = to_tsquery ( 'supernovae|sn & !nebulae' ) WHERE t = to_tsquery ( 'supernovae' );

SELECT ts_rewrite ( to_tsquery ( 'supernovae & crab' ), 'SELECT t, s FROM tsearch.aliases' );

SELECT ts_rewrite ( 'a & b' :: tsquery , 'SELECT t,s FROM tsearch.aliases WHERE ''a & b''::tsquery @> t' );

DROP TABLE tsearch . aliases ;

