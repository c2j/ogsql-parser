-- 来源: 2724_bitmapscanHint.txt
-- SQL 数量: 1

explain(costs off) select /*+ BitmapScan(t1 it1 it3)*/* from t1 where (t1.c1 = 5 or t1.c2=6) or (t1.c3=3 or t1.c2=7);

