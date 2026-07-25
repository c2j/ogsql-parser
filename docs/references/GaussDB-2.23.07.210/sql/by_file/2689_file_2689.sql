-- 来源: 2689_file_2689.txt
-- SQL 数量: 2

EXPLAIN SELECT * FROM t1,t2 WHERE t1.c1 = t2.c2;

explain performance select sum(t2.c1) from t1,t2 where t1.c1=t2.c2 group by t1.c2;

