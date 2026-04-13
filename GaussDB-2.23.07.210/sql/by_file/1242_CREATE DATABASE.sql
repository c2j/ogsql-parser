-- 来源: 1242_CREATE DATABASE.txt
-- SQL 数量: 12

CREATE USER jim PASSWORD '********';

--创建一个GBK编码的数据库testdb1。
CREATE DATABASE testdb1 ENCODING 'GBK' template = template0;

--查看数据库testdb1信息。
SELECT datname,pg_encoding_to_char(encoding) FROM pg_database WHERE datname = 'testdb1';

CREATE DATABASE testdb2 OWNER jim DBCOMPATIBILITY = 'ORA';

--查看testdb2信息。
SELECT t1.datname,t2.usename,t1.datcompatibility FROM pg_database t1,pg_user t2 WHERE t1.datname = 'testdb2' AND t1.datdba=t2.usesysid;

SET a_format_version='10c';

SET a_format_dev_version='s2';

--创建兼容ORA格式的数据库并指定时区。
CREATE DATABASE testdb3 DBCOMPATIBILITY 'ORA' DBTIMEZONE='+08:00';

--查看testdb3信息。
SELECT datname,datcompatibility,dattimezone FROM pg_database WHERE datname = 'testdb3';

DROP DATABASE testdb1;

DROP DATABASE testdb2;

DROP DATABASE testdb3;

