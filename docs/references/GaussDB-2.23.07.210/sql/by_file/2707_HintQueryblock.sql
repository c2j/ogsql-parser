-- 来源: 2707_HintQueryblock.txt
-- SQL 数量: 8

set explain_perf_mode = pretty;

explain (blockname on,costs off) select * from t1, (select c1 from t2 group by c1) sub1 where t1.c1 = sub1.c1;

explain (blockname on,costs off) select /*+indexscan(@sel$2 t2) tablescan(t1)*/ * from t1, (select c1 from t2 group by c1) sub1 where t1.c1 = sub1.c1;

explain (blockname on,costs off) select * from t2, (select c1 from t1 where t1.c3 = 2) sub1 where t2.c1 = sub1.c1;

explain (blockname on,costs off) select * from t2, (select /*+ no_expand*/ c1 from t1 where t1.c3 = 2) sub1 where t2.c1 = sub1.c1;

explain (blockname on,costs off) select/*+ indexscan(@sel$2 t1)*/ * from t2, (select c1 from t1 where t1.c3 = 2) sub1 where t2.c1 = sub1.c1;

create view v1 as select/*+ no_expand */ c1 from t1 where c1 in (select /*+ no_expand */ c1 from t2 where t2.c3=4 );

explain (blockname on,costs off) select * from v1;

