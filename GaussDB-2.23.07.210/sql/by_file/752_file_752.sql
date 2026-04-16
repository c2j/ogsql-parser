-- 来源: 752_file_752.txt
-- SQL 数量: 7

CREATE DATABASE db_tpcds ;

CREATE DATABASE db_tpcds WITH TABLESPACE = hr_local ;

gsql((GaussDB Kernel XXX.XXX.XXX build f521c606) compiled at 2021-09-16 14:55:22 commit 2935 last mr 6385 release) Non-SSL connection (SSL connection is recommended when requiring high-security) Type "help" for help. db_tpcds=> 查看数据库 使用\l元命令查看数据库系统的数据库列表。

\ l 使用如下命令通过系统表pg_database查询数据库列表。

SELECT datname FROM pg_database ;

ALTER DATABASE db_tpcds RENAME TO human_tpcds ;

DROP DATABASE human_tpcds ;

