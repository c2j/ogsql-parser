-- 来源: 1104_file_1104.txt
-- SQL 数量: 16

select * from pg_create_logical_replication_slot('slot_lsn','mppdb_decoding',0);

select * from pg_create_logical_replication_slot('slot_csn','mppdb_decoding',1);

select * from pg_logical_slot_peek_changes('slot_lsn',NULL,4096,'skip-empty-xacts','on');

select * from pg_logical_slot_peek_changes('slot_csn',NULL,4096,'skip-empty-xacts','on');

select * from pg_get_replication_slots();

select * from pg_get_replication_slots();

select * from pg_logical_get_area_changes('0/502E418', NULL, NULL, 'sql_decoding', NULL);

select * from gs_get_parallel_decode_status();

select * from gs_get_slot_decoded_wal_time('replication_slot');

select * from gs_logical_parallel_decode_status('replication_slot');

select * from gs_logical_parallel_decode_status('replication_slot');

select * from gs_logical_parallel_decode_reset_status('replication_slot');

select * from gs_logical_parallel_decode_status('replication_slot');

select * from gs_get_parallel_decode_thread_info();

SELECT * FROM gs_get_distribute_decode_status();

SELECT * FROM gs_get_distribute_decode_status_detail();

