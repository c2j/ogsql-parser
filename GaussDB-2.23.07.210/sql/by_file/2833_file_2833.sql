-- 来源: 2833_file_2833.txt
-- SQL 数量: 14

SELECT ROW ( 1 , 2 , NULL ) < ROW ( 1 , 3 , 0 ) AS RESULT ;

select ( 4 , 5 , 6 ) > ( 3 , 2 , 1 ) as result ;

select ( 4 , 1 , 1 ) > ( 3 , 2 , 1 ) as result ;

select ( 'test' , 'data' ) > ( 'data' , 'data' ) as result ;

select ( 4 , 1 , 1 ) > ( 3 , 2 , null ) as result ;

select ( null , 1 , 1 ) > ( 3 , 2 , 1 ) as result ;

select ( null , 5 , 6 ) > ( null , 5 , 6 ) as result ;

select ( 4 , 5 , 6 ) > ( 4 , 5 , 6 ) as result ;

select ( 2 , 2 , 5 ) >= ( 2 , 2 , 3 ) as result ;

select ( 2 , 2 , 1 ) <= ( 2 , 2 , 3 ) as result ;

select ( 1 , 2 , 3 ) = ( 1 , 2 , 3 ) as result ;

select ( 1 , 2 , 3 ) <> ( 2 , 2 , 3 ) as result ;

select ( 2 , 2 , 3 ) <> ( 2 , 2 , null ) as result ;

select ( null , 5 , 6 ) <> ( null , 5 , 6 ) as result ;

