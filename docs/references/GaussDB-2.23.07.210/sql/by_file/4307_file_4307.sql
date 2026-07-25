-- 来源: 4307_file_4307.txt
-- SQL 数量: 4

SELECT * FROM range_sales PARTITION (p1);

SELECT * FROM range_sales PARTITION (p2);

SELECT * FROM range_sales PARTITION (p3);

-- 查看分区表信息
SELECT relname, boundaries, spcname FROM pg_partition p JOIN pg_tablespace t ON p.reltablespace=t.oid and p.parentid='tpcds.startend_pt'::regclass ORDER BY 1;

