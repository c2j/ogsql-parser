-- 来源: 2728_Outline Hint.txt
-- SQL 数量: 4

EXPLAIN (OUTLINE ON, COSTS OFF) SELECT * FROM t1 JOIN t2 ON t1.a = t2.a;

EXPLAIN (OUTLINE ON, COSTS OFF) SELECT /*+ BEGIN_OUTLINE_DATA HashJoin(@"sel$1" t1@"sel$1" t2@"sel$1") Leading(@"sel$1" (t1@"sel$1" t2@"sel$1")) TableScan(@"sel$1" t1@"sel$1") TableScan(@"sel$1" t2@"sel$1") VERSION("1.0.0") END_OUTLINE_DATA */ * FROM t1 JOIN t2 ON t1.a = t2.a;

EXPLAIN (OUTLINE ON, COSTS OFF) SELECT /*+ NestLoop(@"sel$1" t1@"sel$1" t2@"sel$1") Leading(@"sel$1" (t1@"sel$1" t2@"sel$1")) TableScan(@"sel$1" t1@"sel$1") TableScan(@"sel$1" t2@"sel$1") */ * FROM t1 JOIN t2 ON t1.a = t2.a;

EXPLAIN (OUTLINE ON, COSTS OFF) SELECT /*+ BEGIN_OUTLINE_DATA NestLoop(@"sel$1" t1@"sel$1" t2@"sel$1") Leading(@"sel$1" (t1@"sel$1" t2@"sel$1")) TableScan(@"sel$1" t1@"sel$1") TableScan(@"sel$1" t2@"sel$1") VERSION("1.0.0") END_OUTLINE_DATA */ * from t1 join t2 on t1.a = t2.a;

