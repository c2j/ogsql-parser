-- 来源: 1209_ALTER SCHEMA.txt
-- SQL 数量: 13

CREATE DATABASE test1 WITH DBCOMPATIBILITY = 'MYSQL' ;

\ c test1 --创建模式ds。

CREATE SCHEMA ds ;

ALTER SCHEMA ds RENAME TO ds_new ;

CREATE USER jack PASSWORD '********' ;

ALTER SCHEMA ds_new OWNER TO jack ;

CREATE SCHEMA sch ;

ALTER SCHEMA sch CHARACTER SET utf8mb4 COLLATE utf8mb4_bin ;

DROP SCHEMA ds_new ;

DROP SCHEMA sch ;

DROP USER jack ;

\ c postgres

DROP DATABASE test1 ;

