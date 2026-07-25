-- 来源: 1355_PURGE.txt
-- SQL 数量: 18

CREATE ROLE tpcds IDENTIFIED BY '*********';

-- 创建表空间reason_table_space。
CREATE TABLESPACE REASON_TABLE_SPACE1 owner tpcds RELATIVE location 'tablespace/tsp_reason1';

-- 创建SCHEMA。
CREATE SCHEMA tpcds;

-- 在表空间创建表tpcds.reason_t1。
CREATE TABLE tpcds.reason_t1 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) with(storage_type=ustore) tablespace reason_table_space1;

-- 在表空间创建表tpcds.reason_t2。
CREATE TABLE tpcds.reason_t2 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) with(storage_type=ustore) tablespace reason_table_space1;

-- 在表空间创建表tpcds.reason_t3。
CREATE TABLE tpcds.reason_t3 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) with(storage_type=ustore) tablespace reason_table_space1;

-- 对表tpcds.reason_t1创建索引。
CREATE INDEX index_t1 on tpcds.reason_t1(r_reason_id);

DROP TABLE tpcds.reason_t1;

DROP TABLE tpcds.reason_t2;

DROP TABLE tpcds.reason_t3;

--查看回收站。
SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

--PURGE清除表。
PURGE TABLE tpcds.reason_t3;

SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

--PURGE清除索引。
PURGE INDEX tpcds.index_t1;

SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

--PURGE清除回收站所有对象。
PURGE recyclebin;

SELECT rcyname,rcyoriginname,rcytablespace FROM GS_RECYCLEBIN;

-- 删除SCHEMA。
DROP SCHEMA tpcds CASCADE;

