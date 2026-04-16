-- 来源: 2896_ALTER INDEX.txt
-- SQL 数量: 30

CREATE TABLE test1(col1 int, col2 int);

CREATE INDEX aa ON test1(col1);

--将索引aa重命名为idx_test1_col1。
ALTER INDEX aa RENAME TO idx_test1_col1;

--查询test1表上的索引信息。
SELECT tablename,indexname,tablespace FROM pg_indexes WHERE tablename = 'test1';

CREATE TABLESPACE tbs_index1 RELATIVE LOCATION 'tablespace1/tbs_index1';

--修改索引idx_test1_col1的所属表空间为tbs_index1。
ALTER INDEX IF EXISTS idx_test1_col1 SET TABLESPACE tbs_index1;

--查询test1表上的索引信息。
SELECT tablename,indexname,tablespace FROM pg_indexes WHERE tablename = 'test1';

--修改索引idx_test1_col1 的填充因子。
ALTER INDEX IF EXISTS idx_test1_col1 SET (FILLFACTOR = 70);

ALTER INDEX IF EXISTS idx_test1_col1 RESET (FILLFACTOR);

ALTER INDEX IF EXISTS idx_test1_col1 UNUSABLE;

--查看索引idx_test1_col1的可用性。
SELECT indisusable FROM pg_index WHERE indexrelid = 'idx_test1_col1'::regclass;

--重建索引idx_test1_col1。
ALTER INDEX idx_test1_col1 REBUILD;

--查看索引idx_test1_col1的可用性。
SELECT indisusable FROM pg_index WHERE indexrelid = 'idx_test1_col1'::regclass;

--删除。
DROP INDEX idx_test1_col1;

DROP TABLE test1;

DROP TABLESPACE tbs_index1;

CREATE TABLE test2(col1 int, col2 int) PARTITION BY RANGE (col1)( PARTITION p1 VALUES LESS THAN (100), PARTITION p2 VALUES LESS THAN (200) );

--创建分区索引。
CREATE INDEX idx_test2_col1 ON test2(col1) LOCAL( PARTITION p1, PARTITION p2 );

--重命名索引分区。
ALTER INDEX idx_test2_col1 RENAME PARTITION p1 TO p1_test2_idx;

ALTER INDEX idx_test2_col1 RENAME PARTITION p2 TO p2_test2_idx;

--查询索引idx_test2_col1分区的名称。
SELECT relname FROM pg_partition WHERE parentid = 'idx_test2_col1'::regclass;

CREATE TABLESPACE tbs_index2 RELATIVE LOCATION 'tablespace1/tbs_index2';

CREATE TABLESPACE tbs_index3 RELATIVE LOCATION 'tablespace1/tbs_index3';

--修改索引idx_test2_col1分区的所属表空间。
ALTER INDEX idx_test2_col1 MOVE PARTITION p1_test2_idx TABLESPACE tbs_index2;

ALTER INDEX idx_test2_col1 MOVE PARTITION p2_test2_idx TABLESPACE tbs_index3;

--查询索引idx_test2_col1分区的所属表空间。
SELECT t1.relname index_name, t2.spcname tablespace_name FROM pg_partition t1, pg_tablespace t2 WHERE t1.parentid = 'idx_test2_col1'::regclass AND t1.reltablespace = t2.oid;

--删除。
DROP INDEX idx_test2_col1;

DROP TABLE test2;

DROP TABLESPACE tbs_index2;

DROP TABLESPACE tbs_index3;

