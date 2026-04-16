-- 来源: 1074_file_1074.txt
-- SQL 数量: 12

SELECT B '10001' || B '011' AS RESULT ;

SELECT B '10001' & B '01101' AS RESULT ;

SELECT B '10001' | B '01101' AS RESULT ;

SELECT B '10001' # B '01101' AS RESULT ;

SELECT ~ B '10001' AS RESULT ;

SELECT B '10001' << 3 AS RESULT ;

SELECT B '10001' >> 2 AS RESULT ;

SELECT 44 :: bit ( 10 ) AS RESULT ;

SELECT 44 :: bit ( 3 ) AS RESULT ;

SELECT cast ( - 44 as bit ( 12 )) AS RESULT ;

SELECT '1110' :: bit ( 4 ):: integer AS RESULT ;

select substring ( '10101111' :: bit ( 8 ), 2 );

