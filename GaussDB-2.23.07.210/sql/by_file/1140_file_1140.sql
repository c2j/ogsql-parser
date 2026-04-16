-- 来源: 1140_file_1140.txt
-- SQL 数量: 6

SELECT 40 ! AS "40 factorial" ;

SELECT CAST ( 40 AS bigint ) ! AS "40 factorial" ;

SELECT text 'abc' || 'def' AS "text and unknown" ;

SELECT 'abc' || 'def' AS "unspecified" ;

SELECT @ '-4.5' AS "abs" ;

SELECT array [ 1 , 2 ] <@ '{1,2,3}' as "is subset" ;

