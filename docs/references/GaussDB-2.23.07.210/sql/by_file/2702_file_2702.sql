-- 来源: 2702_file_2702.txt
-- SQL 数量: 10

explain ( analyze on , costs off ) select * from t1 where c1 = 10004 ;

create index idx on t1 ( c1 );

explain ( analyze on , costs off ) select * from t1 where c1 = 10004 ;

explain analyze select count(*) from t1,t2 where t1.c1=t2.c2;

set enable_mergejoin=off;

set enable_nestloop=off;

explain analyze select count(*) from t1,t2 where t1.c1=t2.c2;

explain analyze select count(*) from t1 group by c1;

set enable_sort=off;

explain analyze select count(*) from t1 group by c1;

