-- 来源: 4303_file_4303.txt
-- SQL 数量: 1

--查询t1_hash分区类型
SELECT relname, parttype FROM pg_class WHERE relname = 't1_hash';

