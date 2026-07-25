-- 来源: 5279_drop user.txt
-- SQL 数量: 3

drop user test1 cascade;

select d.datname,s.classid,s.objid from pg_roles r join pg_shdepend s on r.oid=s.refobjid join pg_database d on s.dbid=d.oid where rolname=' test1 ';

drop user test1 cascade;

