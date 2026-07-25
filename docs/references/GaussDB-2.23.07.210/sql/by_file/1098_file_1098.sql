-- 来源: 1098_file_1098.txt
-- SQL 数量: 2

SELECT pg_start_backup ( 'label_goes_here' , true );

SELECT * FROM pg_xlogfile_name_offset ( pg_stop_backup ());

