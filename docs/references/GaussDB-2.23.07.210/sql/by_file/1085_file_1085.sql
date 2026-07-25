-- 来源: 1085_file_1085.txt
-- SQL 数量: 51

SELECT ARRAY [ 1 . 1 , 2 . 1 , 3 . 1 ]:: int [] = ARRAY [ 1 , 2 , 3 ] AS RESULT ;

SELECT ARRAY [ 1 , 2 , 3 ] <> ARRAY [ 1 , 2 , 4 ] AS RESULT ;

SELECT ARRAY [ 1 , 2 , 3 ] < ARRAY [ 1 , 2 , 4 ] AS RESULT ;

SELECT ARRAY [ 1 , 4 , 3 ] > ARRAY [ 1 , 2 , 4 ] AS RESULT ;

SELECT ARRAY [ 1 , 2 , 3 ] <= ARRAY [ 1 , 2 , 3 ] AS RESULT ;

SELECT ARRAY [ 1 , 4 , 3 ] >= ARRAY [ 1 , 4 , 3 ] AS RESULT ;

SELECT ARRAY [ 1 , 4 , 3 ] @> ARRAY [ 3 , 1 ] AS RESULT ;

SELECT ARRAY [ 2 , 7 ] <@ ARRAY [ 1 , 7 , 4 , 2 , 6 ] AS RESULT ;

SELECT ARRAY [ 1 , 4 , 3 ] && ARRAY [ 2 , 1 ] AS RESULT ;

SELECT ARRAY [ 1 , 2 , 3 ] || ARRAY [ 4 , 5 , 6 ] AS RESULT ;

SELECT ARRAY [ 1 , 2 , 3 ] || ARRAY [[ 4 , 5 , 6 ],[ 7 , 8 , 9 ]] AS RESULT ;

SELECT 3 || ARRAY [ 4 , 5 , 6 ] AS RESULT ;

SELECT ARRAY [ 4 , 5 , 6 ] || 7 AS RESULT ;

SELECT array_append ( ARRAY [ 1 , 2 ], 3 ) AS RESULT ;

SELECT array_prepend ( 1 , ARRAY [ 2 , 3 ]) AS RESULT ;

SELECT array_cat ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 4 , 5 ]) AS RESULT ;

SELECT array_cat ( ARRAY [[ 1 , 2 ],[ 4 , 5 ]], ARRAY [ 6 , 7 ]) AS RESULT ;

SELECT array_union ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

SELECT array_union ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

SELECT array_union_distinct ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

SELECT array_union_distinct ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

SELECT array_intersect ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

SELECT array_intersect ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

SELECT array_intersect_distinct ( ARRAY [ 1 , 2 , 2 ], ARRAY [ 2 , 2 , 4 , 5 ]) AS RESULT ;

SELECT array_intersect_distinct ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

SELECT array_except ( ARRAY [ 1 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

SELECT array_except ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

SELECT array_except ( NULL , ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

SELECT array_except_distinct ( ARRAY [ 1 , 2 , 2 , 3 ], ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

SELECT array_except_distinct ( ARRAY [ 1 , 2 , 3 ], NULL ) AS RESULT ;

SELECT array_except_distinct ( NULL , ARRAY [ 3 , 4 , 5 ]) AS RESULT ;

SELECT array_ndims ( ARRAY [[ 1 , 2 , 3 ], [ 4 , 5 , 6 ]]) AS RESULT ;

SELECT array_dims ( ARRAY [[ 1 , 2 , 3 ], [ 4 , 5 , 6 ]]) AS RESULT ;

SELECT array_length ( array [ 1 , 2 , 3 ], 1 ) AS RESULT ;

SELECT array_lower ( '[0:2]={1,2,3}' :: int [], 1 ) AS RESULT ;

SELECT array_sort ( ARRAY [ 5 , 1 , 3 , 6 , 2 , 7 ]) AS RESULT ;

SELECT array_upper ( ARRAY [ 1 , 8 , 3 , 7 ], 1 ) AS RESULT ;

SELECT array_to_string ( ARRAY [ 1 , 2 , 3 , NULL , 5 ], ',' , '*' ) AS RESULT ;

SELECT array_delete(ARRAY[1,8,3,7]) AS RESULT;

SELECT array_deleteidx(ARRAY[1,2,3,4,5], 1) AS RESULT;

SELECT array_extendnull(ARRAY[1,8,3,7],1) AS RESULT;

SELECT array_extendnull(ARRAY[1,8,3,7],2,2) AS RESULT;

SELECT array_trim(ARRAY[1,8,3,7],1) AS RESULT;

SELECT array_exists(ARRAY[1,8,3,7],1) AS RESULT;

SELECT array_next(ARRAY[1,8,3,7],1) AS RESULT;

SELECT array_prior(ARRAY[1,8,3,7],2) AS RESULT;

SELECT string_to_array ( 'xx~^~yy~^~zz' , '~^~' , 'yy' ) AS RESULT ;

SELECT string_to_array ( 'xx~^~yy~^~zz' , '~^~' , 'y' ) AS RESULT ;

SELECT unnest ( ARRAY [ 1 , 2 ]) AS RESULT ;

SELECT cardinality(array[[1, 2], [3, 4]]);

SELECT array_positions(array[1, 2, 3, 1], 1) AS RESULT;

