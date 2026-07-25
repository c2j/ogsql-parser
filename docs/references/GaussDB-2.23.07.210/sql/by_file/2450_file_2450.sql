-- 来源: 2450_file_2450.txt
-- SQL 数量: 8

CREATE DATABASE db_tpcc ;

CREATE DATABASE db_tpcc WITH TABLESPACE = hr_local ;

gsql ((GaussDB Kernel XXX.X.XXX build f521c606) compiled at 2021-09-16 14:55:22 commit 2935 last mr 6385 release) Non-SSL connection (SSL connection is recommended when requiring high-security) Type "help" for help. db_tpcc=> 查看数据库 使用\l元命令查看数据库系统的数据库列表。

\ l 使用如下命令通过系统表pg_database查询数据库列表。

SELECT datname FROM pg_database ;

ALTER DATABASE db_tpcc SET search_path TO pa_catalog , public ;

ALTER DATABASE db_tpcc RENAME TO human_tpcds ;

DROP DATABASE human_tpcds ;

