-- 来源: 2431_file_2431.txt
-- SQL 数量: 23

create table a(id int, value int);

insert into a values(1,4);

insert into a values(2,4);

start transaction isolation level repeatable read;

select * from a;

update a set value = 6 where id = 1;

select * from a;

start transaction isolation level repeatable read;

select * from a;

update a set value = 6 where id = 2;

select * from a;

commit;

commit;

select * from a;

create table a(id int primary key, value int);

insert into a values(1,10);

start transaction isolation level repeatable read;

delete a where id = 1;

start transaction isolation level repeatable read;

select * from a;

commit;

insert into a values(1, 100);

select * from a;

