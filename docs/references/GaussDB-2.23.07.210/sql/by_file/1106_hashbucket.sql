-- 来源: 1106_hashbucket.txt
-- SQL 数量: 17

SELECT * FROM gs_redis_get_plan(16388, 16417);

SELECT * FROM gs_redis_get_bucket_statistics();

SELECT gs_redis_set_distributed_db('gaussdb');

SELECT * FROM gs_redis_hashbucket_update_segment_header(16388, 16417);

SELECT * FROM gs_redis_local_get_segment_header('mytable', '256');

SELECT * FROM gs_redis_local_update_segment_header('mytable', '4294967295,4294967295,4294967295,4294967295,....');

SELECT * FROM gs_redis_hashbucket_update_inverse_pointer('0,1,2,3,4,5,6,7,8,9,10','datanode1','datanode3');

SELECT * FROM gs_redis_hashbucket_update_inverse_pointer('0,1,2,3,4,5,6,7,8,9,10','datanode1','datanode3');

SELECT * FROM gs_redis_local_update_inverse_pointer('mytable', '4294967295,4294967295,4294967295,4294967295,....','1 2 3');

SELECT * FROM gs_redis_local_set_hashbucket_frozenxid();

SELECT * FROM gs_redis_set_hashbucket_frozenxid(16388, 16417);

SELECT * FROM gs_redis_set_nextxid('15268817');

SELECT * FROM gs_redis_set_csn('15268817');

SELECT * FROM gs_redis_check_bucket_flush('{datanode1， datanode2}');

SELECT * FROM gs_redis_show_bucketxid('1 2 3');

SELECT * FROM gs_redis_drop_bucket_files(16388, 16417);

SELECT * FROM gs_redis_local_drop_bucket_files('1 2 3', 3);

