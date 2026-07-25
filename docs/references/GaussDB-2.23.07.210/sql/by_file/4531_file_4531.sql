-- 来源: 4531_file_4531.txt
-- SQL 数量: 2

--查询t1_hash分区类型
SELECT relname, parttype FROM pg_class WHERE relname = 't1_hash';

--查询t1_sub_rr分区类型
SELECT relname, parttype FROM pg_class WHERE relname = 't1_sub_rr';

