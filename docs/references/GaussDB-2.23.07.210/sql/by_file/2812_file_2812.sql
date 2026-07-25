-- 来源: 2812_file_2812.txt
-- SQL 数量: 4

select * from pg_get_gtt_relstats(74069);

select * from pg_get_gtt_statistics(74069,1,''::text);

select * from pg_gtt_attached_pid(74069);

select * from pg_list_gtt_relfrozenxids();

