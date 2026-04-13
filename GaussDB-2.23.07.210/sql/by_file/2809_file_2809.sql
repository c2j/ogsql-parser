-- 来源: 2809_file_2809.txt
-- SQL 数量: 3

select pg_get_triggerdef(oid) from pg_trigger;

select pg_get_triggerdef(oid,true) from pg_trigger;

select pg_get_triggerdef(oid,false) from pg_trigger;

