-- 来源: 2776_file_2776.txt
-- SQL 数量: 62

SELECT box '((0,0),(1,1))' + point '(2.0,0)' AS RESULT ;

SELECT box '((0,0),(1,1))' - point '(2.0,0)' AS RESULT ;

SELECT box '((0,0),(1,1))' * point '(2.0,0)' AS RESULT ;

SELECT box '((0,0),(2,2))' / point '(2.0,0)' AS RESULT ;

SELECT box '((1,-1),(-1,1))' # box '((1,1),(-2,-2))' AS RESULT ;

SELECT # path '((1,0),(0,1),(-1,0))' AS RESULT ;

SELECT @-@ path '((0,0),(1,0))' AS RESULT ;

SELECT @@ circle '((0,0),10)' AS RESULT ;

SELECT circle '((0,0),1)' <-> circle '((5,0),1)' AS RESULT ;

SELECT box '((0,0),(1,1))' && box '((0,0),(2,2))' AS RESULT ;

SELECT circle '((0,0),1)' << circle '((5,0),1)' AS RESULT ;

SELECT circle '((5,0),1)' >> circle '((0,0),1)' AS RESULT ;

SELECT box '((0,0),(1,1))' &< box '((0,0),(2,2))' AS RESULT ;

SELECT box '((0,0),(3,3))' &> box '((0,0),(2,2))' AS RESULT ;

SELECT box '((0,0),(3,3))' <<| box '((3,4),(5,5))' AS RESULT ;

SELECT box '((3,4),(5,5))' |>> box '((0,0),(3,3))' AS RESULT ;

SELECT box '((0,0),(1,1))' &<| box '((0,0),(2,2))' AS RESULT ;

SELECT box '((0,0),(3,3))' |&> box '((0,0),(2,2))' AS RESULT ;

SELECT box '((0,0),(-3,-3))' <^ box '((0,0),(2,2))' AS RESULT ;

SELECT box '((0,0),(2,2))' >^ box '((0,0),(-3,-3))' AS RESULT ;

SELECT lseg '((-1,0),(1,0))' ?# box '((-2,-2),(2,2))' AS RESULT ;

SELECT ?- lseg '((-1,0),(1,0))' AS RESULT ;

SELECT point '(1,0)' ?- point '(0,0)' AS RESULT ;

SELECT ?| lseg '((-1,0),(1,0))' AS RESULT ;

SELECT point '(0,1)' ?| point '(0,0)' AS RESULT ;

SELECT lseg '((0,0),(0,1))' ?-| lseg '((0,0),(1,0))' AS RESULT ;

SELECT lseg '((-1,0),(1,0))' ?|| lseg '((-1,2),(1,2))' AS RESULT ;

SELECT circle '((0,0),2)' @> point '(1,1)' AS RESULT ;

SELECT point '(1,1)' <@ circle '((0,0),2)' AS RESULT ;

SELECT polygon '((0,0),(1,1))' ~= polygon '((1,1),(0,0))' AS RESULT ;

SELECT area ( box '((0,0),(1,1))' ) AS RESULT ;

SELECT center ( box '((0,0),(1,2))' ) AS RESULT ;

SELECT diameter ( circle '((0,0),2.0)' ) AS RESULT ;

SELECT height ( box '((0,0),(1,1))' ) AS RESULT ;

SELECT isclosed ( path '((0,0),(1,1),(2,0))' ) AS RESULT ;

SELECT isopen ( path '[(0,0),(1,1),(2,0)]' ) AS RESULT ;

SELECT length ( path '((-1,0),(1,0))' ) AS RESULT ;

SELECT npoints ( path '[(0,0),(1,1),(2,0)]' ) AS RESULT ;

SELECT npoints ( polygon '((1,1),(0,0))' ) AS RESULT ;

SELECT pclose ( path '[(0,0),(1,1),(2,0)]' ) AS RESULT ;

SELECT popen ( path '((0,0),(1,1),(2,0))' ) AS RESULT ;

SELECT radius ( circle '((0,0),2.0)' ) AS RESULT ;

SELECT width ( box '((0,0),(1,1))' ) AS RESULT ;

SELECT box ( circle '((0,0),2.0)' ) AS RESULT ;

SELECT box ( point '(0,0)' , point '(1,1)' ) AS RESULT ;

SELECT box ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

SELECT circle ( box '((0,0),(1,1))' ) AS RESULT ;

SELECT circle ( point '(0,0)' , 2 . 0 ) AS RESULT ;

SELECT circle ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

SELECT lseg ( box '((-1,0),(1,0))' ) AS RESULT ;

SELECT lseg ( point '(-1,0)' , point '(1,0)' ) AS RESULT ;

SELECT slope(point '(1,1)', point '(0,0)') AS RESULT;

SELECT path ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

SELECT point ( 23 . 4 , - 44 . 5 ) AS RESULT ;

SELECT point ( box '((-1,0),(1,0))' ) AS RESULT ;

SELECT point ( circle '((0,0),2.0)' ) AS RESULT ;

SELECT point ( lseg '((-1,0),(1,0))' ) AS RESULT ;

SELECT point ( polygon '((0,0),(1,1),(2,0))' ) AS RESULT ;

SELECT polygon ( box '((0,0),(1,1))' ) AS RESULT ;

SELECT polygon ( circle '((0,0),2.0)' ) AS RESULT ;

SELECT polygon ( 12 , circle '((0,0),2.0)' ) AS RESULT ;

SELECT polygon ( path '((0,0),(1,1),(2,0))' ) AS RESULT ;

