-- 来源: 2885_ALTER DATABASE.txt
-- SQL 数量: 20

CREATE DATABASE testdb;

--将testdb重命名为test_db1。
ALTER DATABASE testdb RENAME TO test_db1;

ALTER DATABASE test_db1 WITH CONNECTION LIMIT 100;

--查看test_db1信息。
SELECT datname,datconnlimit FROM pg_database WHERE datname = 'test_db1';

CREATE USER scott PASSWORD '********';

--将test_db1的所有者修改为jim。
ALTER DATABASE test_db1 OWNER TO scott;

--查看test_db1信息。
SELECT t1.datname, t2.usename FROM pg_database t1, pg_user t2 WHERE t1.datname='test_db1' AND t1.datdba=t2.usesysid;

CREATE TABLESPACE tbs_data1 RELATIVE LOCATION 'tablespace1/tbs_data1';

--修改test_db1默认表空间。
ALTER DATABASE test_db1 SET TABLESPACE tbs_data1;

--查看test_db1信息。
SELECT t1.datname AS database, t2.spcname AS tablespace FROM pg_database t1, pg_tablespace t2 WHERE t1.dattablespace = t2.oid AND t1.datname = 'test_db1';

CREATE USER jack PASSWORD '********';

CREATE TABLE test_tbl1(c1 int,c2 int);

SELECT tablename FROM pg_tables WHERE tablename = 'test_tbl1';

ALTER DATABASE test_db1 ENABLE PRIVATE OBJECT;

--由于隔离属性的原因，该查询只能查出0条数据。
SELECT tablename FROM pg_tables WHERE tablename = 'test_tbl1';

DROP TABLE public.test_tbl1;

DROP DATABASE test_db1;

DROP TABLESPACE tbs_data1;

DROP USER jack;

DROP USER scott;

