-- 来源: 2801_file_2801.txt
-- SQL 数量: 34

select * from pg_create_logical_replication_slot('slot_lsn','mppdb_decoding',0);

select * from pg_create_logical_replication_slot('slot_csn','mppdb_decoding',1);

select * from pg_logical_slot_peek_changes('slot_lsn',NULL,4096,'skip-empty-xacts','on');

select * from pg_logical_slot_peek_changes('slot_csn',NULL,4096,'skip-empty-xacts','on');

CREATE TABLE t1(a int, b int);

SELECT pg_current_xlog_location();

INSERT INTO t1 VALUES(1,1);

UPDATE t1 SET b = 2 WHERE a = 1;

DELETE FROM t1;

SELECT * FROM pg_logical_get_area_changes('0/5ECBCD48', NULL, NULL, 'sql_decoding', NULL);

CREATE TABLE t2(a int, b int GENERATED ALWAYS AS (a + 1) STORED);

SELECT pg_current_xlog_location();

INSERT INTO t2(a) VALUES(1);

UPDATE t2 set a = 2 where a = 1;

DELETE FROM t2;

SELECT * FROM pg_logical_get_area_changes('0/5F62CFE8', NULL, NULL, 'sql_decoding', NULL, 'skip-generated-columns', 'on');

select * from pg_get_replication_slots();

select * from gs_get_parallel_decode_status();

select * from gs_get_slot_decoded_wal_time('replication_slot');

select * from gs_logical_parallel_decode_status('replication_slot');

select * from gs_logical_parallel_decode_status('replication_slot');

select * from gs_logical_parallel_decode_reset_status('replication_slot');

select * from gs_logical_parallel_decode_status('replication_slot');

select * from gs_logical_decode_start_observe('replication_slot',20,5);

select * from gs_logical_decode_start_observe('replication_slot',20,5);

select * from gs_logical_decode_stop_observe('replication_slot');

select * from gs_logical_decode_stop_observe('replication_slot');

select * from gs_logical_decode_observe_data('replication_slot');

select * from gs_logical_decode_observe('replication_slot');

select * from gs_logical_decode_observe_status('replication_slot');

select * from gs_logical_decode_observe_status('replication_slo');

select * from gs_logical_decode_stop_observe('replication_slot');

select * from gs_logical_decode_observe_status('replication_slot');

select * from gs_get_parallel_decode_thread_info();

