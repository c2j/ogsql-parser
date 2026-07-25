-- 来源: 5949_gs_rescue.txt
-- SQL 数量: 5

create table original(col1 integer);

copy original from '/data2/file01';

create table amend(col1 integer,col2 integer default 0);

copy amend from '/data2/file02';

insert into amend(col1) select * from original;

