-- 来源: 1223_ALTER VIEW.txt
-- SQL 数量: 13

CREATE TABLE test_tb1(col1 INT,col2 INT);

--创建视图。
CREATE VIEW abc AS SELECT * FROM test_tb1;

--重命名视图。
ALTER VIEW IF EXISTS abc RENAME TO test_v1;

CREATE ROLE role_test PASSWORD '********';

--修改视图所有者。
ALTER VIEW IF EXISTS test_v1 OWNER TO role_test;

CREATE SCHEMA tcpds;

--修改视图所属模式。
ALTER VIEW test_v1 SET SCHEMA tcpds;

ALTER VIEW tcpds.test_v1 SET (security_barrier = TRUE);

--重置视图选项。
ALTER VIEW tcpds.test_v1 RESET (security_barrier);

--删除视图test_v1。
DROP VIEW tcpds.test_v1;

--删除表test_tb1。
DROP TABLE test_tb1;

--删除用户。
DROP ROLE role_test;

--删除schema。
DROP SCHEMA tcpds;

