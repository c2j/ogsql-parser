-- 来源: 2950_CREATE FOREIGN TABLE.txt
-- SQL 数量: 4

CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

--创建外表
CREATE FOREIGN TABLE foreign_tbl (col1 text) SERVER my_server OPTIONS (logtype 'pg_log');

--删除外表
DROP FOREIGN TABLE foreign_tbl;

--删除server
DROP SERVER my_server;

