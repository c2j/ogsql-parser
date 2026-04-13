-- 来源: 1096_file_1096.txt
-- SQL 数量: 8

SELECT pg_ls_dir ( './' );

SELECT pg_read_file ( 'postmaster.pid' , 0 , 100 );

SELECT convert_from ( pg_read_binary_file ( 'filename' ), 'UTF8' );

SELECT * FROM pg_stat_file ( 'filename' );

SELECT ( pg_stat_file ( 'filename' )). modification ;

SELECT convert_from ( pg_read_binary_file ( 'postmaster.pid' ), 'UTF8' );

SELECT * FROM pg_stat_file ( 'postmaster.pid' );

SELECT ( pg_stat_file ( 'postmaster.pid' )). modification ;

