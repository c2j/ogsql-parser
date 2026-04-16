-- 来源: 4323_file_4323.txt
-- SQL 数量: 6

select relname, relpages, reltuples from pg_partition where relname in ('id11', 'id22', 'max_id1');

select * from pg_stats where tablename ='only_fisrt_part' and partitionname ='id11';

EXPLAIN SELECT * FROM test_int4_maxvalue PARTITION(id1);

-- 查询指定分区max_id
EXPLAIN SELECT * FROM test_int4_maxvalue PARTITION(max_id);

-- 查询指定分区p_
EXPLAIN SELECT * FROM test_default PARTITION(p_1);

-- 查询指定分区p_
EXPLAIN SELECT * FROM test_default PARTITION(p_3);

