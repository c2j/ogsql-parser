-- 来源: 1020_aggHint.txt
-- SQL 数量: 2

explain (costs off) select c1 from t2 where c1 in( select /*+ use_hash_agg */ t1.c1 from t1,t3 where t1.c1=t3.c1 group by 1);

explain (costs off) select c1 from t2 where c1 in( select /*+ use_sort_agg */ t1.c1 from t1,t3 where t1.c1=t3.c1 group by 1);

