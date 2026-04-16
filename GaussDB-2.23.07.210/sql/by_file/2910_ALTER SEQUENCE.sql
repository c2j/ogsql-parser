-- 来源: 2910_ALTER SEQUENCE.txt
-- SQL 数量: 5

CREATE SEQUENCE serial START 101;

--创建一个表,定义默认值。
CREATE TABLE t1(c1 bigint default nextval('serial'));

--将序列serial的归属列变为T1.C1。
ALTER SEQUENCE serial OWNED BY t1.c1;

--删除序列和表。
DROP SEQUENCE serial CASCADE;

DROP TABLE t1;

