-- 来源: 2805_Undo.txt
-- SQL 数量: 12

select * from gs_global_config where name like '%undostoragetype%';

select * from gs_stat_undo(true);

select * from gs_stat_undo(false);

select * from gs_undo_meta_dump_zone(-1,true);

select * from gs_undo_translot_dump_slot(-1,true);

select * from gs_undo_translot_dump_xid('15758',false);

select * from gs_undo_dump_record('0000000000000042');

select * from gs_undo_dump_xid('15779');

select * from gs_verify_undo_record('urp', 24, 24, 1);

select * from gs_verify_undo_record('zone', 0, 2, 1);

select * from gs_verify_undo_slot('zone', 0, 2, 1);

select * from gs_verify_undo_meta('all', 0, 2, 1);

