-- 来源: 994_file_994.txt
-- SQL 数量: 10

explain ( analyze on , costs off ) select * from t1 where c2 = 10004 ;

create index idx on t1 ( c2 );

explain ( analyze on , costs off ) select * from t1 where c2 = 10004 ;

explain analyze select count(*) from t2,t1 where t1.c1=t2.c2;

set enable_mergejoin=off;

set enable_nestloop=off;

explain analyze select count(*) from t2,t1 where t1.c1=t2.c2;

explain analyze select count(*) from t1 group by c2;

set enable_sort=off;

explain analyze select count(*) from t1 group by c2;

