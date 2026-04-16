-- 来源: 2955_CREATE INDEX.txt
-- SQL 数量: 21

CREATE TABLE tbl_test1( id int, --用户id name varchar(50), --用户姓名 postcode char(6) --邮编 );

--创建表空间tbs_index1。
CREATE TABLESPACE tbs_index1 RELATIVE LOCATION 'test_tablespace/tbs_index1';

--为表tbl_test1创建索引idx_test1指定表空间。
CREATE INDEX idx_test1 ON tbl_test1(name) TABLESPACE tbs_index1;

--查询索引idx_test1信息。
SELECT indexname,tablename,tablespace FROM pg_indexes WHERE indexname = 'idx_test1';

--删除索引。
DROP INDEX idx_test1;

--删除表空间
DROP TABLESPACE tbs_index1;

CREATE UNIQUE INDEX idx_test2 ON tbl_test1(id);

--删除索引。
DROP INDEX idx_test2;

CREATE INDEX idx_test3 ON tbl_test1(substr(postcode,2));

--删除索引。
DROP INDEX idx_test3;

CREATE INDEX idx_test4 ON tbl_test1(id) WHERE id IS NOT NULL;

-- 删除索引。
DROP INDEX idx_test4;

-- 删除表
DROP TABLE tbl_test1;

CREATE TABLE student(id int, name varchar(20)) PARTITION BY RANGE (id) ( PARTITION p1 VALUES LESS THAN (200), PARTITION pmax VALUES LESS THAN (MAXVALUE) );

--创建LOCAL分区索引不指定索引分区的名称。
CREATE INDEX idx_student1 ON student(id) LOCAL;

--查看索引分区信息，发现LOC索引分区数和表的分区数一致。
SELECT relname FROM pg_partition WHERE parentid = 'idx_student1'::regclass;

--删除LOCAL分区索引。
DROP INDEX idx_student1;

--创建GLOBAL索引。
CREATE INDEX idx_student2 ON student(name) GLOBAL;

--查看索引分区信息，发现GLOBAL索引分区数和表的分区数不一致。
SELECT relname FROM pg_partition WHERE parentid = 'idx_student2'::regclass;

--删除GLOBAL分区索引。
DROP INDEX idx_student2;

--删除表。
DROP TABLE student;

