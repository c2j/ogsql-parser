-- 来源: 2720_Hint.txt
-- SQL 数量: 5

explain (costs off) select /*+nestloop_index(t1,(t2 t3)) */* from t1,t2,t3 where t1.c1 = t2.c1 and t1.c2 = t3.c2;

explain (costs off) select /*+NestLoop_Index(t1,it1) */* from t1,t2 where t1.c1 = t2.c1;

SET rewrite_rule = 'predpushforce' ;

EXPLAIN SELECT * FROM t1, t2 WHERE t1.c1 = t2.c1;

EXPLAIN SELECT /*+predpush_same_level(t1, t2)*/ * FROM t1, t2 WHERE t1.c1 = t2.c1;

