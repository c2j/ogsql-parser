-- 来源: 1382_TIMECAPSULE TABLE.txt
-- SQL 数量: 12

CREATE SCHEMA tpcds;

-- 删除表tpcds.reason_t2。
DROP TABLE IF EXISTS tpcds.reason_t2;

-- 创建表tpcds.reason_t2。
CREATE TABLE tpcds.reason_t2 ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) )with(storage_type = ustore);

--向表tpcds.reason_t2中插入记录。
INSERT INTO tpcds.reason_t2 VALUES (1, 'AA', 'reason1'),(2, 'AB', 'reason2'),(3, 'AC', 'reason3');

--清空tpcds.reason_t2表中的数据。
TRUNCATE TABLE tpcds.reason_t2;

--查询tpcds.reason_t2表中的数据。
SELECT * FROM tpcds.reason_t2;

--执行闪回TRUNCATE。
TIMECAPSULE TABLE tpcds.reason_t2 to BEFORE TRUNCATE;

SELECT * FROM tpcds.reason_t2;

--删除表tpcds.reason_t2。
DROP TABLE tpcds.reason_t2;

--执行闪回DROP。
TIMECAPSULE TABLE tpcds.reason_t2 to BEFORE DROP;

-- 清空回收站，删除SCHEMA。
PURGE RECYCLEBIN;

DROP SCHEMA tpcds CASCADE;

