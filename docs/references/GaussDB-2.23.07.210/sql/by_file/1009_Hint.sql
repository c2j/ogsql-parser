-- 来源: 1009_Hint.txt
-- SQL 数量: 1

EXPLAIN (costs off) SELECT /*+PREDPUSH(pt2 st3) */ * FROM pt2, (SELECT /*+ indexscan(pt3) indexscan(pt4) */sum(pt3.b), pt3.a FROM pt3, pt4 where pt3.a = pt4.a GROUP BY pt3.a) st3 WHERE st3.a = pt2.a;

