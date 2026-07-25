-- 来源: 2733_GUCrewrite_rule.txt
-- SQL 数量: 23

set rewrite_rule='none';

create table t1(c1 int,c2 int);

create table t2(c1 int,c2 int);

explain (verbose on, costs off) select c1,(select avg(c2) from t2 where t2.c2=t1.c2) from t1 where t1.c1<100 order by t1.c2;

set rewrite_rule='intargetlist';

explain (verbose on, costs off) select c1,(select avg(c2) from t2 where t2.c2=t1.c2) from t1 where t1.c1<100 order by t1.c2;

set rewrite_rule='uniquecheck';

explain verbose select t1.c1 from t1 where t1.c1 = (select t2.c1 from t2 where t1.c1=t2.c1);

select * from t1 order by c2;

select * from t2 order by c2;

select t1.c1 from t1 where t1.c1 = (select t2.c1 from t2 where t1.c1=t2.c2) ;

set rewrite_rule='uniquecheck';

select t1.c1 from t1 where t1.c1 = (select t2.c1 from t2 where t1.c1=t2.c2) ;

set rewrite_rule =none;

create table t (a int, b int, c int, d int);

create table t1 (a int, b int, c int, d int);

explain (costs off) select t.b, sum(cc) from (select b, sum(c) as cc from t1 group by b) s1, t where s1.b=t.b group by t.b order by 1,2;

set rewrite_rule = lazyagg;

explain (costs off) select t.b, sum(cc) from (select b, sum(c) as cc from t1 group by b) s1, t where s1.b=t.b group by t.b order by 1,2;

create table t1(a int, b varchar, c int, d int);

create table t2(a int, b varchar, c int, d int);

set rewrite_rule = none;

explain (costs off) select t1 from t1 where t1.b = 10 and t1.c < (select sum(c) from t2 where t1.a = t2.a);

