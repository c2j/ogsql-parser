-- 来源: 3120_SQL.txt
-- SQL 数量: 1

explain select * from t1 where not exists(select * from t2 where t1.c1 = t2.c1);

