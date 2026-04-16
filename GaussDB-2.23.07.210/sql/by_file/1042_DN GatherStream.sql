-- 来源: 1042_DN GatherStream.txt
-- SQL 数量: 50

set enable_broadcast=false;

set explain_perf_mode=pretty;

set enable_dngather=false;

explain select count(*) from t1, t2 where t1.b = t2.b;

set enable_dngather=true;

explain select count(*) from t1, t2 where t1.b = t2.b;

set enable_dngather=false;

explain select * from t1, t2, t3, t4 where t1.b = t2.b and t2.c = t3.c and t3.d = t4.d order by t1.a;

set enable_dngather=true;

explain select * from t1, t2, t3, t4 where t1.b = t2.b and t2.c = t3.c and t3.d = t4.d order by t1.a;

set enable_dngather=false;

explain select count(*) from t1, t2, t3, t4 where t1.b = t2.b and t2.c = t3.c and t3.d = t4.d group by t1.b order by t1.b;

set enable_dngather=true;

explain select count(*) from t1, t2, t3, t4 where t1.b = t2.b and t2.c = t3.c and t3.d = t4.d group by t1.b order by t1.b;

set explain_perf_mode=pretty;

set enable_dngather=false;

explain select count(*) from t1 group by b;

set enable_dngather=true;

explain select count(*) from t1 group by b;

set enable_dngather=false;

explain select b from t1 group by b;

set enable_dngather=true;

explain select b from t1 group by b;

set explain_perf_mode=pretty;

set enable_dngather=false;

explain select count(*) over (partition by b) a from t1;

set enable_dngather=true;

explain select count(*) over (partition by b) a from t1;

set enable_dngather=false;

explain select sum(b) over (partition by b) a from t1 group by b;

set enable_dngather=true;

explain select sum(b) over (partition by b) a from t1 group by b;

set explain_perf_mode=pretty;

set enable_broadcast=false;

set enable_dngather=false;

explain select t1.a, t2.b from t1, t2 where t1.b = t2.b union all select t3.a, t3.b from t3, t4 where t3.b = t4.b;

set enable_dngather=true;

explain select t1.a, t2.b from t1, t2 where t1.b = t2.b union all select t3.a, t3.b from t3, t4 where t3.b = t4.b;

set enable_dngather=false;

explain select t1.a, t2.b from t1, t2 where t1.b = t2.b union select t3.a, t3.b from t3, t4 where t3.b = t4.b order by a, b;

set enable_dngather=true;

explain select t1.a, t2.b from t1, t2 where t1.b = t2.b union select t3.a, t3.b from t3, t4 where t3.b = t4.b order by a, b;

set enable_dngather=false;

explain select b, count(*) from t1 group by b union all select b, count(*) from t2 group by b order by b;

set enable_dngather=true;

explain select b, count(*) from t1 group by b union all select b, count(*) from t2 group by b order by b;

set enable_dngather=false;

explain select b, count(*) from t1 group by b union all select count(distinct a) a , count(distinct b)b from t2 order by b;

set enable_dngather=true;

explain select b, count(*) from t1 group by b union all select count(distinct a) a , count(distinct b)b from t2 order by b;

