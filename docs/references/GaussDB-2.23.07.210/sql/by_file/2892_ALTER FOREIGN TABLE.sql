-- 来源: 2892_ALTER FOREIGN TABLE.txt
-- SQL 数量: 7

CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

--创建外表
CREATE FOREIGN TABLE foreign_tbl (col1 text) SERVER my_server OPTIONS (logtype 'pg_log');

--修改外表属性
ALTER FOREIGN TABLE foreign_tbl OPTIONS (ADD latest_files '2');

ALTER FOREIGN TABLE foreign_tbl OPTIONS ( SET latest_files '5');

ALTER FOREIGN TABLE foreign_tbl OPTIONS ( DROP latest_files);

DROP FOREIGN TABLE foreign_tbl;

DROP SERVER my_server;

