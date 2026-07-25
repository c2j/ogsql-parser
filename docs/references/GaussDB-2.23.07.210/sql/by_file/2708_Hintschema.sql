-- 来源: 2708_Hintschema.txt
-- SQL 数量: 2

explain(blockname on,costs off) select /*+ indexscan(t1)*/ * from t1, (select c2 from t1 where c1=1) tt1 where t1.c1 = tt1.c2;

explain(blockname on,costs off) select /*+ indexscan(t1@sel$2)*/ * from t1, (select c2 from t1 where c1=1) tt1 where t1.c1 = tt1.c2;

