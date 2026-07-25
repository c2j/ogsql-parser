-- 来源: 4625_init_td.txt
-- SQL 数量: 4

create table test1(name varchar) with(storage_type = ustore, init_td=2);

alter table test1 set(init_td=8);

select * from pg_thread_wait_status;

drop table test1;

