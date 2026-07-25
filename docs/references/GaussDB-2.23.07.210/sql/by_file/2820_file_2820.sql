-- 来源: 2820_file_2820.txt
-- SQL 数量: 22

select * from gs_verify_data_file();

select * from gs_verify_data_file(true);

select * from gs_repair_file(16554,'base/16552/24745',360);

select * from local_bad_block_info();

select * from local_clear_bad_block_info();

select * from gs_verify_and_tryrepair_page('base/16552/24745',0,false,false);

select * from gs_repair_page('base/16552/24745',0,false,60);

select gs_edit_page_bypath('base/15808/25075',0,16,'0x1FFF', 2, false, 'page');

select gs_edit_page_bypath('base/15808/25075', 0,16,'@1231!', 8, false, 'page');

select gs_edit_page_bypath('/pg_log_dir/dump/1663_15808_25075_0.editpage', 0,16,'0x1FFF', 2, true, 'page');

select * from gs_repair_page_bypath('pg_log/dump/1663_15991_16767_0.editpage', 0, 'base/15991/16767', 0, 'page');

select * from gs_repair_page_bypath('standby', 0, 'base/15990/16768', 0, 'page');

select * from gs_repair_page_bypath('init_block', 0, 'base/15990/16768', 0, 'page');

select * from gs_repair_undo_byzone(4);

select * from gs_repair_undo_byzone(78);

select * from gs_repair_undo_byzone(0);

select * from gs_verify_urq(16387, 0, 1, 'free queue');

select * from gs_verify_urq(16387, 0, 1, 'empty queue');

SELECT * FROM gs_urq_dump_stat(16387, 0);

SELECT gs_urq_dump_stat(17260,0);

select * from gs_repair_urq(16387, 0);

select * from gs_get_standby_bad_block_info();

