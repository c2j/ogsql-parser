-- 来源: 2984_CREATE TYPE.txt
-- SQL 数量: 23

CREATE TYPE compfoo AS (f1 int, f2 text);

CREATE TABLE t1_compfoo(a int, b compfoo);

CREATE TABLE t2_compfoo(a int, b compfoo);

INSERT INTO t1_compfoo values(1,(1,'demo'));

INSERT INTO t2_compfoo select * from t1_compfoo;

SELECT (b).f1 FROM t1_compfoo;

SELECT * FROM t1_compfoo t1 join t2_compfoo t2 on (t1.b).f1=(t1.b).f1;

--重命名数据类型。
ALTER TYPE compfoo RENAME TO compfoo1;

--要改变一个用户定义类型compfoo1的所有者为usr1。
CREATE USER usr1 PASSWORD ' ******** ';

ALTER TYPE compfoo1 OWNER TO usr1;

--把用户定义类型compfoo1的模式改变为usr1。
ALTER TYPE compfoo1 SET SCHEMA usr1;

--给一个数据类型增加一个新的属性。
ALTER TYPE usr1.compfoo1 ADD ATTRIBUTE f3 int;

--删除compfoo1类型。
DROP TYPE usr1.compfoo1 CASCADE;

--删除相关表和用户。
DROP TABLE t1_compfoo;

DROP TABLE t2_compfoo;

DROP SCHEMA usr1;

DROP USER usr1;

--创建一个枚举类型。
CREATE TYPE bugstatus AS ENUM ('create', 'modify', 'closed');

--添加一个标签值。
ALTER TYPE bugstatus ADD VALUE IF NOT EXISTS 'regress' BEFORE 'closed';

--重命名一个标签值。
ALTER TYPE bugstatus RENAME VALUE 'create' TO 'new';

--创建一个集合类型
CREATE TYPE bugstatus_table AS TABLE OF bugstatus;

--删除集合类型及枚举类型。
DROP TYPE bugstatus_table;

DROP TYPE bugstatus CASCADE;

