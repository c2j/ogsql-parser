-- 来源: 2846_file_2846.txt
-- SQL 数量: 4

SELECT 'a fat cat sat on a mat and ate a fat rat' :: tsvector @@ 'cat & rat' :: tsquery AS RESULT ;

SELECT 'fat & cow' :: tsquery @@ 'a fat cat sat on a mat and ate a fat rat' :: tsvector AS RESULT ;

SELECT to_tsvector ( 'fat cats ate fat rats' ) @@ to_tsquery ( 'fat & rat' ) AS RESULT ;

SELECT 'fat cats ate fat rats' :: tsvector @@ to_tsquery ( 'fat & rat' ) AS RESULT ;

