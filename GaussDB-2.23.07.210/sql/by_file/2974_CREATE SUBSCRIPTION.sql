-- 来源: 2974_CREATE SUBSCRIPTION.txt
-- SQL 数量: 15

CREATE TABLE users (c1 int, c2 int);

CREATE TABLE departments (c1 int, c2 int);

CREATE TABLE mydata (c1 int, c2 int);

--创建一个发布，发布两个表中所有更改。
CREATE PUBLICATION mypublication FOR TABLE users, departments;

--创建一个发布，只发布一个表中的INSERT操作。
CREATE PUBLICATION insert_only FOR TABLE mydata WITH (publish = 'insert');

--创建一个到远程服务器的订阅，复制发布mypublication和insert_only中的表，并在提交时立即开始复制。其中ip和port为发布端的ip地址和端口，user、dbname、password为发布端数据库的用户信息。
CREATE SUBSCRIPTION mysub CONNECTION 'host=192.168.11.11 port=5432 user=foo dbname=foodb password=********' PUBLICATION mypublication, insert_only;

--创建一个到远程服务器的订阅，复制insert_only发布中的表， 并且不开始复制直到稍后启用复制。其中ip和port为发布端的ip地址和端口，user、dbname、password为发布端数据库的用户信息。
CREATE SUBSCRIPTION mysub CONNECTION 'host=192.168.11.11 port=5432 user=foo dbname=foodb password=********' PUBLICATION insert_only WITH (enabled = false);

--修改订阅的连接信息。其中ip和port为发布端的ip地址和端口，user、dbname、password为发布端数据库的用户信息。
ALTER SUBSCRIPTION mysub CONNECTION 'host=192.168.11.11 port=5432 user=foo dbname=foodb password=********';

--激活订阅。
ALTER SUBSCRIPTION mysub SET(enabled=true);

--删除订阅。
DROP SUBSCRIPTION mysub;

--删除发布。
DROP PUBLICATION insert_only;

DROP PUBLICATION mypublication;

--删除表。
DROP TABLE users;

DROP TABLE departments;

DROP TABLE mydata;

