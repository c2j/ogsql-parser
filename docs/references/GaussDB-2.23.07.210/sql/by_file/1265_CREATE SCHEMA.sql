-- 来源: 1265_CREATE SCHEMA.txt
-- SQL 数量: 4

CREATE ROLE role1 IDENTIFIED BY '********' ;

CREATE SCHEMA AUTHORIZATION role1 CREATE TABLE films ( title text , release date , awards text []) CREATE VIEW winners AS SELECT title , release FROM films WHERE awards IS NOT NULL ;

DROP SCHEMA role1 CASCADE ;

DROP USER role1 CASCADE ;

