-- 来源: 4319_file_4319.txt
-- SQL 数量: 9

EXPLAIN (VERBOSE ON, COSTS OFF) SELECT /*+ nestloop(t1 t2) indexscan(t1) indexscan(t2) */ * FROM t2 JOIN t1 ON t1.c1 = t2.c1;

EXPLAIN (VERBOSE ON, COSTS OFF) SELECT /*+ nestloop(t1 t2) indexscan(t1) indexscan(t2) */ * FROM t2 JOIN t1 ON t1.c1 < t2.c1;

EXPLAIN (VERBOSE ON, COSTS OFF) SELECT /*+ nestloop(t1 t2) indexscan(t1) indexscan(t2) */ * FROM t2 JOIN t1 ON t1.c1 < t2.c1;

EXPLAIN (VERBOSE ON, COSTS OFF) SELECT /*+ nestloop(t1 t2) indexscan(t1) indexscan(t2) */ * FROM t2 JOIN t1 ON t1.c1 = t2.c1 AND t1.c2 = 2;

set enable_seqscan=off;

EXPLAIN (VERBOSE ON, COSTS OFF) SELECT /*+ nestloop(t1 t2) */ * FROM t2 JOIN t1 ON t1.c1 = t2.c1 OR t1.c2 = 2;

CREATE TABLE t3(c1 TEXT, c2 INT);

EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 JOIN t3 ON t1.c1 = t3.c1;

EXPLAIN (VERBOSE ON, COSTS OFF) SELECT * FROM t1 JOIN t3 ON t1.c1 = LENGTHB(t3.c1);

