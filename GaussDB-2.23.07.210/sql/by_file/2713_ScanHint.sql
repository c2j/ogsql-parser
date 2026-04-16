-- 来源: 2713_ScanHint.txt
-- SQL 数量: 2

-- 使用索引
EXPLAIN SELECT /*+ gsi(gsi_test gsi_test_idx) */ * FROM gsi_test where b = 1;

EXPLAIN SELECT /*+ gsitable(gsi_test gsi_test_idx) */ * FROM gsi_test where b = 1;

