-- 来源: 1211_ALTER SERVER.txt
-- SQL 数量: 3

CREATE SERVER my_server FOREIGN DATA WRAPPER log_fdw ;

ALTER SERVER my_server RENAME TO my_server_1 ;

DROP SERVER my_server_1 ;

