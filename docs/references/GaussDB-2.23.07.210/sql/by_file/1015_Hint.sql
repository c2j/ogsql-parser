-- 来源: 1015_Hint.txt
-- SQL 数量: 4

deallocate all;

prepare p1 as insert /*+ no_gpc */ into t1 select c1,c2 from t2 where c1=$1;

execute p1(3);

select * from dbe_perf.global_plancache_status where schema_name='public' order by 1,2;

