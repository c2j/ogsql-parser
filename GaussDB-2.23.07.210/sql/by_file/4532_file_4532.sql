-- 来源: 4532_file_4532.txt
-- SQL 数量: 2

--查询t1_hash分区类型
SELECT oid, relname, parttype FROM pg_class WHERE relname = 't1_hash';

--查询t1_hash的分区信息
SELECT oid, relname, parttype, parentid FROM pg_partition WHERE parentid = 16685;

