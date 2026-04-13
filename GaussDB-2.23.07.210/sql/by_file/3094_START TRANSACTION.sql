-- 来源: 3094_START TRANSACTION.txt
-- SQL 数量: 11

CREATE SCHEMA tpcds;

--创建表 tpcds. reason。
CREATE TABLE tpcds. reason (c1 int, c2 int);

--以默认方式启动事务。
START TRANSACTION;

SELECT * FROM tpcds. reason;

--以默认方式启动事务。
BEGIN;

SELECT * FROM tpcds. reason;

--以隔离级别为READ COMMITTED，读/写方式启动事务。
START TRANSACTION ISOLATION LEVEL READ COMMITTED READ WRITE;

SELECT * FROM tpcds. reason;

COMMIT;

--删除表 tpcds. reason。
DROP TABLE tpcds. reason;

--删除SCHEMA。
DROP SCHEMA tpcds;

