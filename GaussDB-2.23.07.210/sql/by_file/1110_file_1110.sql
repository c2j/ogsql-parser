-- 来源: 1110_file_1110.txt
-- SQL 数量: 70

select pg_stat_get_role_name(10);

select * from pg_stat_get_activity(139881386280704);

select * from gs_stat_get_hotkeys_info () order by count , hash_value ;

select * from gs_stat_clean_hotkeys ();

select * from global_stat_get_hotkeys_info () order by count , hash_value ;

select * from global_stat_clean_hotkeys ();

SELECT pg_backend_pid ();

SELECT pg_stat_get_backend_pid ( 1 );

select * from gs_stack ( 139663481165568 );

select * from gs_stack ();

SELECT * FROM gs_perf_start ( 10 , 100 );

SELECT * FROM gs_perf_query () WHERE overhead > 2 AND level < 10 ;

SELECT * FROM gs_perf_clean ();

select sessionid from pg_stat_activity where usename = 'testuser';

select * from gs_session_all_settings(788861) where name = 'work_mem';

select * from gs_session_all_settings() where name = 'work_mem';

select * from gs_local_wal_preparse_statistics();

select * from gs_hot_standby_space_info();

SELECT * FROM exrto_file_read_stat();

SELECT * FROM gs_exrto_recycle_info();

SELECT * FROM gs_stat_get_db_conflict_all(12738);

SELECT * FROM gs_redo_stat_info();

SELECT * FROM gs_recovery_conflict_waitevent_info();

SELECT * FROM gs_display_delay_ddl_info();

CREATE TABLE part_tab 1 ( a int, b int ) PARTITION BY RANGE(b) ( PARTITION P1 VALUES LESS THAN(10), PARTITION P2 VALUES LESS THAN(20), PARTITION P3 VALUES LESS THAN(MAXVALUE) );

CREATE TABLE subpart_tab 1 ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY RANGE (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a VALUES LESS THAN( '2' ), SUBPARTITION p_201901_b VALUES LESS THAN( '3' ) ), PARTITION p_201902 VALUES LESS THAN( '201904' ) ( SUBPARTITION p_201902_a VALUES LESS THAN( '2' ), SUBPARTITION p_201902_b VALUES LESS THAN( '3' ) ) );

CREATE INDEX index_part_tab1 ON part_tab1(b) LOCAL ( PARTITION b_index1, PARTITION b_index2, PARTITION b_index 3 );

CREATE INDEX idx_user_no ON subpart_tab1(user_no) LOCAL;

INSERT INTO part_tab1 VALUES(1, 1);

INSERT INTO part_tab1 VALUES(1, 11);

INSERT INTO part_tab1 VALUES(1, 21);

UPDATE part_tab1 SET a = 2 WHERE b = 1;

UPDATE part_tab1 SET a = 3 WHERE b = 11;

UPDATE /*+ indexscan(part_tab1) */ part_tab1 SET a = 4 WHERE b = 21;

DELETE FROM part_tab1;

ANALYZE part_tab1;

VACUUM part_tab1;

INSERT INTO subpart_tab1 VALUES('201902', '1', '1', 1);

INSERT INTO subpart_tab1 VALUES('201902', '2', '2', 1);

INSERT INTO subpart_tab1 VALUES('201903', '1', '3', 1);

INSERT INTO subpart_tab1 VALUES('201903', '2', '4', 1);

UPDATE subpart_tab1 SET sales_amt = 2 WHERE user_no='1';

UPDATE subpart_tab1 SET sales_amt = 3 WHERE user_no='2';

UPDATE subpart_tab1 SET sales_amt = 4 WHERE user_no='3';

UPDATE /*+ indexscan(subpart_tab1) */ subpart_tab1 SET sales_amt = 5 WHERE user_no='4';

DELETE FROM subpart_tab1;

ANALYZE subpart_tab1;

VACUUM subpart_tab1;

SELECT * FROM gs_stat_all_partitions;

SELECT * FROM gs_statio_all_partitions;

SELECT * FROM gs_stat_get_partition_stats(16952);

BEGIN;

INSERT INTO part_tab1 VALUES(1, 1);

INSERT INTO part_tab1 VALUES(1, 11);

INSERT INTO part_tab1 VALUES(1, 21);

UPDATE part_tab1 SET a = 2 WHERE b = 1;

UPDATE part_tab1 SET a = 3 WHERE b = 11;

UPDATE /*+ indexscan(part_tab1) */ part_tab1 SET a = 4 WHERE b = 21;

DELETE FROM part_tab1;

INSERT INTO subpart_tab1 VALUES('201902', '1', '1', 1);

INSERT INTO subpart_tab1 VALUES('201902', '2', '2', 1);

INSERT INTO subpart_tab1 VALUES('201903', '1', '3', 1);

INSERT INTO subpart_tab1 VALUES('201903', '2', '4', 1);

UPDATE subpart_tab1 SET sales_amt = 2 WHERE user_no='1';

UPDATE subpart_tab1 SET sales_amt = 3 WHERE user_no='2';

UPDATE subpart_tab1 SET sales_amt = 4 WHERE user_no='3';

UPDATE /*+ indexscan(subpart_tab1) */ subpart_tab1 SET sales_amt = 5 WHERE user_no='4';

DELETE FROM subpart_tab1;

SELECT * FROM gs_stat_xact_all_partitions;

SELECT * FROM gs_stat_get_xact_partition_stats(16952);

