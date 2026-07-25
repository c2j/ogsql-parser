-- 来源: 1019_Hint.txt
-- SQL 数量: 1

explain (costs off) select /*+materialize_inner(t1) materialize_inner(t1 t2)*/ * from t1,t2,t3 where t1.c3 = t2.c3 and t2.c2=t3.c2 and t1.c2=5;

