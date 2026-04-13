-- 来源: 1081_file_1081.txt
-- SQL 数量: 30

SELECT to_tsvector ( 'fat cats ate rats' ) @@ to_tsquery ( 'cat & rat' ) AS RESULT ;

SELECT to_tsvector ( 'fat cats ate rats' ) @@@ to_tsquery ( 'cat & rat' ) AS RESULT ;

SELECT 'a:1 b:2' :: tsvector || 'c:1 d:2 b:3' :: tsvector AS RESULT ;

SELECT 'fat | rat' :: tsquery && 'cat' :: tsquery AS RESULT ;

SELECT 'fat | rat' :: tsquery || 'cat' :: tsquery AS RESULT ;

SELECT !! 'cat' :: tsquery AS RESULT ;

SELECT 'cat' :: tsquery @> 'cat & rat' :: tsquery AS RESULT ;

SELECT 'cat' :: tsquery <@ 'cat & rat' :: tsquery AS RESULT ;

SELECT get_current_ts_config ();

SELECT length ( 'fat:2,4 cat:3 rat:5A' :: tsvector );

SELECT numnode ( '(fat & rat) | cat' :: tsquery );

SELECT plainto_tsquery ( 'english' , 'The Fat Rats' );

SELECT querytree ( 'foo & ! bar' :: tsquery );

SELECT setweight ( 'fat:2,4 cat:3 rat:5B' :: tsvector , 'A' );

SELECT strip ( 'fat:2,4 cat:3 rat:5A' :: tsvector );

SELECT to_tsquery ( 'english' , 'The & Fat & Rats' );

SELECT to_tsvector ( 'english' , 'The Fat Rats' );

SELECT to_tsvector_for_batch ( 'english' , 'The Fat Rats' );

SELECT ts_headline ( 'x y z' , 'z' :: tsquery );

SELECT ts_rank ( 'hello world' :: tsvector , 'world' :: tsquery );

SELECT ts_rank_cd ( 'hello world' :: tsvector , 'world' :: tsquery );

SELECT ts_rewrite ( 'a & b' :: tsquery , 'a' :: tsquery , 'foo|bar' :: tsquery );

SELECT ts_rewrite ( 'world' :: tsquery , 'select ''world''::tsquery, ''hello''::tsquery' );

SELECT ts_debug ( 'english' , 'The Brightest supernovaes' );

SELECT ts_lexize ( 'english_stem' , 'stars' );

SELECT ts_parse ( 'default' , 'foo - bar' );

SELECT ts_parse ( 3722 , 'foo - bar' );

SELECT ts_token_type ( 'default' );

SELECT ts_token_type ( 3722 );

SELECT ts_stat ( 'select ''hello world''::tsvector' );

