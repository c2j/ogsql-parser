-- 来源: 2975_CREATE SYNONYM.txt
-- SQL 数量: 22

CREATE SCHEMA ot;

--创建表ot.t1及其同义词t1。
CREATE TABLE ot.t1(id int, name varchar2(10));

CREATE OR REPLACE SYNONYM t1 FOR ot.t1;

--使用同义词t1。
SELECT * FROM t1;

INSERT INTO t1 VALUES (1, 'ada'), (2, 'bob');

UPDATE t1 SET t1.name = 'cici' WHERE t1.id = 2;

--创建同义词v1及其关联视图ot.v_t1。
CREATE SYNONYM v1 FOR ot.v_t1;

CREATE VIEW ot.v_t1 AS SELECT * FROM ot.t1;

--使用同义词v1。
SELECT * FROM v1;

--创建重载函数ot.add及其同义词add。
CREATE OR REPLACE FUNCTION ot.add(a integer, b integer) RETURNS integer AS $$ SELECT $1 + $2 $$ LANGUAGE sql;

CREATE OR REPLACE FUNCTION ot.add(a decimal(5,2), b decimal(5,2)) RETURNS decimal(5,2) AS $$ SELECT $1 + $2 $$ LANGUAGE sql;

CREATE OR REPLACE SYNONYM add FOR ot.add;

--使用同义词add。
SELECT add(1,2);

SELECT add(1.2,2.3);

--创建存储过程ot.register及其同义词register。
CREATE PROCEDURE ot.register(n_id integer, n_name varchar2(10)) SECURITY INVOKER AS BEGIN INSERT INTO ot.t1 VALUES(n_id, n_name);

CREATE OR REPLACE SYNONYM register FOR ot.register;

--使用同义词register，调用存储过程。
CALL register(3,'mia');

--删除同义词。
DROP SYNONYM t1;

DROP SYNONYM IF EXISTS v1;

DROP SYNONYM IF EXISTS add;

DROP SYNONYM register;

DROP SCHEMA ot CASCADE;

