-- 来源: 4551_Partition Iterator.txt
-- SQL 数量: 3

EXPLAIN SELECT * FROM test_range_pt WHERE a = 3000;

SET partition_iterator_elimination = on;

EXPLAIN SELECT * FROM test_range_pt WHERE a = 3000;

