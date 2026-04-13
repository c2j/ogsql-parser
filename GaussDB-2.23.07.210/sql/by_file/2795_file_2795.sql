-- 来源: 2795_file_2795.txt
-- SQL 数量: 2

SELECT pg_start_backup ( 'label_goes_here' );

SELECT * FROM pg_xlogfile_name_offset ( pg_stop_backup ());

