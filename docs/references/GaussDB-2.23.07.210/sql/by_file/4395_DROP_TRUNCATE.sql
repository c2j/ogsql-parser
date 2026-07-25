-- 来源: 4395_DROP_TRUNCATE.txt
-- SQL 数量: 59

select * from gs_recyclebin;

drop table if EXISTS flashtest;

select * from gs_recyclebin;

create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

insert into flashtest values(1, 'A');

select * from flashtest;

drop table if EXISTS flashtest;

select * from gs_recyclebin;

select * from flashtest;

PURGE TABLE flashtest;

select * from gs_recyclebin;

drop table if EXISTS flashtest;

create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

create index flashtest_index on flashtest(id);

drop table if EXISTS flashtest;

select * from gs_recyclebin;

PURGE index flashtest_index;

select * from gs_recyclebin;

PURGE RECYCLEBIN;

select * from gs_recyclebin;

drop table if EXISTS flashtest;

create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

insert into flashtest values(1, 'A');

select * from flashtest;

drop table if EXISTS flashtest;

select * from gs_recyclebin;

select * from flashtest;

timecapsule table flashtest to before drop;

select * from flashtest;

select * from gs_recyclebin;

drop table if EXISTS flashtest;

select * from flashtest;

select * from gs_recyclebin;

timecapsule table "BIN$31C14EB48DC$9B4E$0==$0" to before drop;

select * from gs_recyclebin;

select * from flashtest;

drop table if EXISTS flashtest;

select * from gs_recyclebin;

select * from flashtest;

timecapsule table flashtest to before drop rename to flashtest_rename;

select * from flashtest;

select * from flashtest_rename;

select * from gs_recyclebin;

drop table if EXISTS flashtest_rename;

PURGE RECYCLEBIN;

select * from gs_recyclebin;

drop table if EXISTS flashtest;

create table if not EXISTS flashtest(id int, name text) with (storage_type = ustore);

insert into flashtest values(1, 'A');

select * from flashtest;

truncate table flashtest;

select * from gs_recyclebin;

select * from flashtest;

timecapsule table flashtest to before truncate;

select * from flashtest;

select * from gs_recyclebin;

drop table if EXISTS flashtest;

PURGE RECYCLEBIN;

select * from gs_recyclebin;

