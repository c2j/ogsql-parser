-- 来源: 4552_Merge Append.txt
-- SQL 数量: 3

EXPLAIN ANALYZE SELECT * FROM test_range_pt WHERE b >10 AND b < 5000 ORDER BY b LIMIT 10;

--关闭分区表Merge Append算子
SET sql_beta_feature = 'disable_merge_append_partition';

EXPLAIN ANALYZE SELECT * FROM test_range_pt WHERE b >10 AND b < 5000 ORDER BY b LIMIT 10;

