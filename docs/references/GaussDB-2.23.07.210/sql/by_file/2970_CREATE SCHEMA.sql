-- 来源: 2970_CREATE SCHEMA.txt
-- SQL 数量: 8

CREATE DATABASE test1 WITH DBCOMPATIBILITY = 'b';

--创建一个角色role1。
CREATE ROLE role1 IDENTIFIED BY ' ******** ';

-- 为用户role1创建一个同名schema，子命令创建的表films和winners的拥有者为role1。
CREATE SCHEMA AUTHORIZATION role1 CREATE TABLE films (title text, release date, awards text[]) CREATE VIEW winners AS SELECT title, release FROM films WHERE awards IS NOT NULL;

-- 创建一个schema ds，指定schema的默认字符集为utf8mb4，默认字符序为utf8mb4_bin。仅在B模式下（即sql_compatibility='B'）支持该语法。
CREATE SCHEMA ds CHARACTER SET utf8mb4 COLLATE utf8mb4_bin;

--删除schema。
DROP SCHEMA role1 CASCADE;

DROP SCHEMA ds CASCADE;

--删除用户。
DROP USER role1 CASCADE;

DROP DATABASE test1;

