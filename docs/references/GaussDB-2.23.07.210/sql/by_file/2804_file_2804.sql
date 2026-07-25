-- 来源: 2804_file_2804.txt
-- SQL 数量: 21

select pg_stat_get_env();

select gs_parse_page_bypath('base/16603/16394', -1, 'btree', false);

select gs_parse_page_bypath('base/12828/16771_vm', -1, 'vm', false);

select gs_parse_page_bypath('000000000000', 0, 'clog', false);

select gs_parse_page_bypath('base/12828/16777', -10, 'heap', false);

select * from gs_stat_space(false);

select * from gs_index_dump_read(0, 'all');

select * from gs_index_dump_read(1, 'all');

select * from gs_parse_page_bypath('base/15833/16768', 0, 'uheap', false);

select * from gs_xlogdump_bylastlsn('0/4593570', -1, 'uheap');

select * from gs_xlogdump_bylastlsn('0/4593570', 0, 'ubtree');

CREATE TABLE test(a int,b int);

INSERT INTO test values(1,1);

CREATE PROCEDURE mypro1() as num int;

SET instr_unique_sql_track_type = 'all';

SET track_stmt_stat_level = 'L0,L0';

CALL mypro1();

SET track_stmt_stat_level = 'off,L0';

SET instr_unique_sql_track_type = 'top';

SELECT query,unique_query_id,start_time,finish_time FROM dbe_perf.statement_history;

SELECT query FROM dbe_perf.get_full_sql_by_parent_id_and_timestamp(536458473,'2023-06-02 17:40:59.028144+08','2023-06-02 17:40:59.032027+08');

