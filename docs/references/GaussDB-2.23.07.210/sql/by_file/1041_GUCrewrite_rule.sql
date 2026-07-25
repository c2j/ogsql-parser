-- 来源: 1041_GUCrewrite_rule.txt
-- SQL 数量: 14

show rewrite_rule;

explain (costs off) select * from t1, (select sum(c2), c1 from t2 group by c1) st2 where st2.c1 = t1.c1;

set rewrite_rule='predpushnormal';

explain (costs off) select * from t1, (select sum(c2), c1 from t2 group by c1) st2 where st2.c1 = t1.c1;

set rewrite_rule='predpushforce';

explain (costs off) select * from t1, (select sum(c2), c1 from t2 group by c1) st2 where st2.c1 = t1.c1;

set rewrite_rule = 'predpush';

explain (costs off) select * from t1, (select sum(c2), c1 from t2 group by c1) st2 where st2.c1 = t1.c1;

create table t_rep(a int) distribute by replication;

create table t_dis(a int);

set rewrite_rule = '';

explain (costs off) select * from t_dis where a = any(select a from t_rep) or a > 100;

set rewrite_rule = disablerep;

explain (costs off) select * from t_dis where a = any(select a from t_rep) or a > 100;

