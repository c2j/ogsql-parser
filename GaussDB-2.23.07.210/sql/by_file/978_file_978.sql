-- 来源: 978_file_978.txt
-- SQL 数量: 5

EXPLAIN SELECT * FROM t1,t2 WHERE t1.c1 = t2.c2;

explain select c1 , count ( 1 ) from t1 group by c1 ;

set enable_fast_query_shipping=off;

explain select c1,count(1) from t1 group by c1;

explain performance select count(1) from t1;

