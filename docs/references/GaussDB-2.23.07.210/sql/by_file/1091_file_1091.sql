-- 来源: 1091_file_1091.txt
-- SQL 数量: 9

SELECT * FROM generate_series ( 2 , 4 );

SELECT * FROM generate_series ( 5 , 1 , - 2 );

SELECT * FROM generate_series ( 4 , 3 );

SELECT current_date + s . a AS dates FROM generate_series ( 0 , 14 , 7 ) AS s ( a );

SELECT * FROM generate_series ( '2008-03-01 00:00' :: timestamp , '2008-03-04 12:00' , '10 hours' );

SELECT generate_subscripts ( '{NULL,1,NULL,2}' :: int [], 1 ) AS s ;

CREATE OR REPLACE FUNCTION unnest2 ( anyarray ) RETURNS SETOF anyelement AS $$ SELECT $ 1 [ i ][ j ] FROM generate_subscripts ( $ 1 , 1 ) g1 ( i ), generate_subscripts ( $ 1 , 2 ) g2 ( j );

SELECT * FROM unnest2 ( ARRAY [[ 1 , 2 ],[ 3 , 4 ]]);

DROP FUNCTION unnest2 ;

