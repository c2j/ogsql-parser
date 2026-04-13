-- 来源: 2911_ALTER SERVER.txt
-- SQL 数量: 3

CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw;

--修改外部服务的名称。
ALTER SERVER my_server RENAME TO my_server_1;

--删除my_server_1。
DROP SERVER my_server_1;

