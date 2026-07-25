-- 来源: 4626_fillfactor.txt
-- SQL 数量: 3

create table test(a int) with(fillfactor=100);

alter table test set(fillfactor=92);

drop table test;

