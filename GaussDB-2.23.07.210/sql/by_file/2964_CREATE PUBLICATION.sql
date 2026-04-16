-- 来源: 2964_CREATE PUBLICATION.txt
-- SQL 数量: 16

CREATE TABLE users (c1 int, c2 int);

CREATE TABLE departments (c1 int, c2 int);

CREATE TABLE mydata (c1 int, c2 int);

CREATE TABLE mydata2 (c1 int, c2 int);

--创建一个发布，发布两个表中所有更改。
CREATE PUBLICATION mypublication FOR TABLE users, departments;

--创建一个发布，发布所有表中的所有更改。
CREATE PUBLICATION alltables FOR ALL TABLES;

--创建一个发布，只发布一个表中的INSERT操作。
CREATE PUBLICATION insert_only FOR TABLE mydata WITH (publish = 'insert');

--修改发布的动作。
ALTER PUBLICATION insert_only SET (publish='insert,update,delete');

--向发布中添加表。
ALTER PUBLICATION insert_only ADD TABLE mydata2;

--删除发布。
DROP PUBLICATION insert_only;

DROP PUBLICATION alltables;

DROP PUBLICATION mypublication;

--删除表。
DROP TABLE users;

DROP TABLE departments;

DROP TABLE mydata;

DROP TABLE mydata2;

