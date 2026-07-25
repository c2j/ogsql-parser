-- 来源: 1086_file_1086.txt
-- SQL 数量: 29

SELECT int4range ( 1 , 5 ) = '[1,4]' :: int4range AS RESULT ;

SELECT numrange ( 1 . 1 , 2 . 2 ) <> numrange ( 1 . 1 , 2 . 3 ) AS RESULT ;

SELECT int4range ( 1 , 10 ) < int4range ( 2 , 3 ) AS RESULT ;

SELECT int4range ( 1 , 10 ) > int4range ( 1 , 5 ) AS RESULT ;

SELECT numrange ( 1 . 1 , 2 . 2 ) <= numrange ( 1 . 1 , 2 . 2 ) AS RESULT ;

SELECT numrange ( 1 . 1 , 2 . 2 ) >= numrange ( 1 . 1 , 2 . 0 ) AS RESULT ;

SELECT int4range ( 2 , 4 ) @> int4range ( 2 , 3 ) AS RESULT ;

SELECT '[2011-01-01,2011-03-01)' :: tsrange @> '2011-01-10' :: timestamp AS RESULT ;

SELECT int4range ( 2 , 4 ) <@ int4range ( 1 , 7 ) AS RESULT ;

SELECT 42 <@ int4range ( 1 , 7 ) AS RESULT ;

SELECT int8range ( 3 , 7 ) && int8range ( 4 , 12 ) AS RESULT ;

SELECT int8range ( 1 , 10 ) << int8range ( 100 , 110 ) AS RESULT ;

SELECT int8range ( 50 , 60 ) >> int8range ( 20 , 30 ) AS RESULT ;

SELECT int8range ( 1 , 20 ) &< int8range ( 18 , 20 ) AS RESULT ;

SELECT int8range ( 7 , 20 ) &> int8range ( 5 , 10 ) AS RESULT ;

SELECT numrange ( 1 . 1 , 2 . 2 ) -|- numrange ( 2 . 2 , 3 . 3 ) AS RESULT ;

SELECT numrange ( 5 , 15 ) + numrange ( 10 , 20 ) AS RESULT ;

SELECT int8range ( 5 , 15 ) * int8range ( 10 , 20 ) AS RESULT ;

SELECT int8range ( 5 , 15 ) - int8range ( 10 , 20 ) AS RESULT ;

SELECT numrange ( 1 . 1 , 2 . 2 ) AS RESULT ;

SELECT numrange ( 1 . 1 , 2 . 2 , '()' ) AS RESULT ;

SELECT lower ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

SELECT upper ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

SELECT isempty ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

SELECT lower_inc ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

SELECT upper_inc ( numrange ( 1 . 1 , 2 . 2 )) AS RESULT ;

SELECT lower_inf ( '(,)' :: daterange ) AS RESULT ;

SELECT upper_inf ( '(,)' :: daterange ) AS RESULT ;

SELECT elem_contained_by_range ( '2' , numrange ( 1 . 1 , 2 . 2 ));

