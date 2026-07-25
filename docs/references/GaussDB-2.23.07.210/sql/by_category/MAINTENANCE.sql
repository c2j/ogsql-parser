-- 类别: MAINTENANCE
-- SQL 数量: 62

-- 来源: 1024_SQL PATCH
analyze hint_t1 ;

-- 来源: 1102_file_1102
analyze ;

-- 来源: 1110_file_1110
ANALYZE part_tab1;

-- 来源: 1110_file_1110
VACUUM part_tab1;

-- 来源: 1110_file_1110
ANALYZE subpart_tab1;

-- 来源: 1110_file_1110
VACUUM subpart_tab1;

-- 来源: 1225_ANALYZE _ ANALYSE
ANALYZE customer_info ;

-- 来源: 1225_ANALYZE _ ANALYSE
ANALYZE VERBOSE customer_info ;

-- 来源: 1230_CHECKPOINT
CHECKPOINT ;

-- 来源: 1233_CLUSTER
CLUSTER test_c1 USING idx_test_c1_id ;

-- 第一次聚簇排序不带USING关键字报错
-- 来源: 1233_CLUSTER
CLUSTER test;

-- 聚簇排序
-- 来源: 1233_CLUSTER
CLUSTER test USING pk_test;

--对已做过聚簇的表重新进行聚簇
-- 来源: 1233_CLUSTER
CLUSTER VERBOSE test;

-- 对分区p2进行聚簇排序
-- 来源: 1233_CLUSTER
CLUSTER test_c2 PARTITION (p2) USING idx_test_c2_id1;

-- 来源: 1345_LOCK
LOCK TABLE tpcds . reason_t1 IN SHARE ROW EXCLUSIVE MODE ;

-- 来源: 1346_LOCK BUCKETS
LOCK BUCKETS ( 0 , 1 , 2 , 3 ) IN ACCESS EXCLUSIVE MODE ;

-- 来源: 1360_REINDEX
REINDEX INDEX tpcds . tpcds_customer_index1 ;

-- 来源: 1360_REINDEX
REINDEX INDEX CONCURRENTLY tpcds . tpcds_customer_index1 ;

-- 来源: 1360_REINDEX
REINDEX TABLE tpcds . customer_t1 ;

-- 来源: 1360_REINDEX
REINDEX TABLE CONCURRENTLY tpcds . customer_t1 ;

-- 来源: 1387_VACUUM
VACUUM ( VERBOSE , ANALYZE ) tpcds . reason ;

-- 来源: 2799_file_2799
analyze ;

-- 来源: 2808_file_2808
ANALYZE part_tab1;

-- 来源: 2808_file_2808
VACUUM part_tab1;

-- 来源: 2808_file_2808
ANALYZE subpart_tab1;

-- 来源: 2808_file_2808
VACUUM subpart_tab1;

-- 来源: 2927_ANALYZE _ ANALYSE
ANALYZE customer_info;

-- 来源: 2927_ANALYZE _ ANALYSE
ANALYZE customer_par;

-- 来源: 2927_ANALYZE _ ANALYSE
ANALYZE VERBOSE customer_info;

-- 来源: 2932_CHECKPOINT
CHECKPOINT;

-- 来源: 2935_CLUSTER
CLUSTER test_c1 USING idx_test_c1_id ;

-- 第一次聚簇排序不带USING关键字报错
-- 来源: 2935_CLUSTER
CLUSTER test;

-- 聚簇排序
-- 来源: 2935_CLUSTER
CLUSTER test USING pk_test;

--对已做过聚簇的表重新进行聚簇
-- 来源: 2935_CLUSTER
CLUSTER VERBOSE test;

-- 对分区p2进行聚簇排序
-- 来源: 2935_CLUSTER
CLUSTER test_c2 PARTITION (p2) USING idx_test_c2_id1;

-- 来源: 3056_LOCK
LOCK TABLE tpcds. reason_t1 IN SHARE ROW EXCLUSIVE MODE;

--重建一个单独索引。
-- 来源: 3071_REINDEX
REINDEX INDEX tpcds. tpcds_customer_index1;

--在线重建一个单独索引。
-- 来源: 3071_REINDEX
REINDEX INDEX CONCURRENTLY tpcds. tpcds_customer_index1;

--重建表 tpcds. customer_t1上的所有索引。
-- 来源: 3071_REINDEX
REINDEX TABLE tpcds. customer_t1;

--在线重建表 tpcds. customer_t1上的所有索引。
-- 来源: 3071_REINDEX
REINDEX TABLE CONCURRENTLY tpcds. customer_t1;

--对带索引的表 tpcds. reason执行VACUUM操作。
-- 来源: 3101_VACUUM
VACUUM (VERBOSE, ANALYZE) tpcds. reason;

-- 锁定后analyze, 发生报错
-- 来源: 3192_DBE_STATS
ANALYZE t1;

-- 来源: 3192_DBE_STATS
ANALYZE t1;

-- 来源: 3192_DBE_STATS
ANALYZE t1;

-- 来源: 3192_DBE_STATS
ANALYZE t1;

-- 来源: 3192_DBE_STATS
ANALYZE t1;

-- 来源: 3192_DBE_STATS
ANALYZE t1;

-- 来源: 3192_DBE_STATS
ANALYZE t1;

-- 来源: 3192_DBE_STATS
ANALYZE t1;

-- 来源: 3192_DBE_STATS
ANALYZE t1;

-- 来源: 3192_DBE_STATS
ANALYZE t1;

-- 来源: 3192_DBE_STATS
ANALYZE;

-- 来源: 4284_file_4284
VACUUM FULL t1;

-- 来源: 4284_file_4284
VACUUM FULL t1;

-- 来源: 4322_file_4322
analyze t1_range_int with all;

-- 来源: 4322_file_4322
analyze t1_range_int with all;

-- 来源: 4322_file_4322
analyze t1_range_int with all;

-- 来源: 4511_file_4511
VACUUM FULL t1;

-- 来源: 4511_file_4511
VACUUM FULL t1;

-- 来源: 4557_file_4557
analyze t1_range_int with all;

-- 来源: 4557_file_4557
analyze t1_range_int with all;

-- 来源: 4557_file_4557
analyze t1_range_int with all;

