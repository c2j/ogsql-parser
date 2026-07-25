-- 来源: 1056_file_1056.txt
-- SQL 数量: 15

SELECT 'a fat cat sat on a mat and ate a fat rat' :: tsvector ;

SELECT $$ the lexeme ' ' contains spaces $$ :: tsvector ;

SELECT $$ the lexeme 'Joe''s' contains a quote $$ :: tsvector ;

SELECT 'a:1 fat:2 cat:3 sat:4 on:5 a:6 mat:7 and:8 ate:9 a:10 fat:11 rat:12' :: tsvector ;

SELECT 'a:1A fat:2B,4C cat:5D' :: tsvector ;

SELECT 'The Fat Rats' :: tsvector ;

SELECT to_tsvector ( 'english' , 'The Fat Rats' );

SELECT 'fat & rat' :: tsquery ;

SELECT 'fat & (rat | cat)' :: tsquery ;

SELECT 'fat & rat & ! cat' :: tsquery ;

SELECT 'fat:ab & cat' :: tsquery ;

SELECT 'super:*' :: tsquery ;

SELECT to_tsvector ( 'seriousness' ) @@ to_tsquery ( 'series:*' ) AS RESULT ;

SELECT to_tsquery ( 'series:*' );

SELECT to_tsquery ( 'Fat:ab & Cats' );

