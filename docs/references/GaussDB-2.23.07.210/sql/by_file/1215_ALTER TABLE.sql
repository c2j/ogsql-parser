-- 来源: 1215_ALTER TABLE.txt
-- SQL 数量: 36

CREATE TABLE aa(c1 int, c2 int);

ALTER TABLE IF EXISTS aa RENAME TO test_alt1;

CREATE SCHEMA test_schema;

--把表test_alt1的所属模式修改为test_schema。
ALTER TABLE test_alt1 SET SCHEMA test_schema;

--查询表信息。
SELECT schemaname,tablename FROM pg_tables WHERE tablename = 'test_alt1';

CREATE USER test_user PASSWORD '********';

-- 修改test_alt1表的所有者为test_user;
ALTER TABLE IF EXISTS test_schema.test_alt1 OWNER TO test_user;

-- 查看
SELECT tablename, schemaname, tableowner FROM pg_tables WHERE tablename = 'test_alt1';

CREATE TABLESPACE tbs_data1 RELATIVE LOCATION 'tablespace1/tbs_data1';

--修改test_alt1表的空间为tbs_data1。
ALTER TABLE test_schema.test_alt1 SET TABLESPACE tbs_data1;

-- 查看。
SELECT tablename, tablespace FROM pg_tables WHERE tablename = 'test_alt1';

--删除。
DROP TABLE test_schema.test_alt1;

DROP TABLESPACE tbs_data1;

DROP SCHEMA test_schema;

DROP USER test_user;

CREATE TABLE test_alt2(c1 INT,c2 INT);

-- 修改列名
ALTER TABLE test_alt2 RENAME c1 TO id;

ALTER TABLE test_alt2 RENAME COLUMN c2 to areaid;

ALTER TABLE IF EXISTS test_alt2 ADD COLUMN name VARCHAR(20);

ALTER TABLE test_alt2 MODIFY name VARCHAR(50);

ALTER TABLE test_alt2 ALTER COLUMN name TYPE VARCHAR(25);

ALTER TABLE test_alt2 DROP COLUMN areaid;

--修改test_alt2表中name字段的存储模式。
ALTER TABLE test_alt2 ALTER COLUMN name SET STORAGE PLAIN;

--删除。
DROP TABLE test_alt2;

CREATE TABLE test_alt3(pid INT, areaid CHAR(5), name VARCHAR(20));

--为pid添加非空约束。
ALTER TABLE test_alt3 MODIFY pid NOT NULL;

ALTER TABLE test_alt3 MODIFY pid NULL;

ALTER TABLE test_alt3 ALTER COLUMN areaid SET DEFAULT '00000';

ALTER TABLE test_alt3 ALTER COLUMN areaid DROP DEFAULT;

ALTER TABLE test_alt3 ADD CONSTRAINT pk_test3_pid PRIMARY KEY (pid);

CREATE TABLE test_alt4(c1 INT, c2 INT);

--建索引。
CREATE UNIQUE INDEX pk_test4_c1 ON test_alt4(c1);

--添加约束时关联已经创建的索引。
ALTER TABLE test_alt4 ADD CONSTRAINT pk_test4_c1 PRIMARY KEY USING INDEX pk_test4_c1;

--删除。
DROP TABLE test_alt4;

ALTER TABLE test_alt3 DROP CONSTRAINT IF EXISTS pk_test3_pid;

--删除。
DROP TABLE test_alt3;

