-- 来源: 2802_file_2802.txt
-- SQL 数量: 16

select * from gs_seg_dump_page('pg_default', 1, 1024, 4157);

select * from gs_seg_dump_page(16788, 1024, 0);

select * from gs_seg_get_spc_location('pg_default', 1024, 4157, 0);

select * from gs_seg_get_spc_location(24578,1024,0);

select * from gs_seg_get_location(4157);

select * from gs_seg_get_segment_layout();

select * from gs_seg_get_datafile_layout();

select * from gs_seg_get_slice_layout(1,1024, 0);

select * from gs_seg_get_segment('pg_default', 1024, 4157);

select * from gs_seg_get_segment(16768, 1024);

select * from gs_seg_get_extents('pg_default', 1024, 4157);

select * from gs_seg_get_extents(16768, 1024);

select * from gs_seg_free_spc_remain_segment('pg_default', 1, 4159);

select * from gs_seg_free_spc_remain_extent('pg_default', 1, 0, 4159);

select * from gs_seg_get_datafiles();

select * from gs_seg_get_spc_extents('pg_default', 1,1024, 0);

