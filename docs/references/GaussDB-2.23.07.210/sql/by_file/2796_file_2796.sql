-- 来源: 2796_file_2796.txt
-- SQL 数量: 3

select * from pg_create_physical_replication_slot_extern ( 'uuid' , false , 'OBS;

select * from pg_create_physical_replication_slot_extern ( 'uuid' , false , 'NAS;

select gs_set_obs_delete_location('0/54000000');

