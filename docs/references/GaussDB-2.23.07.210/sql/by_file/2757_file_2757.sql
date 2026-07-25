-- 来源: 2757_file_2757.txt
-- SQL 数量: 2

SELECT oid FROM pg_class WHERE relname = 'pg_type' ;

SELECT attrelid , attname , atttypid , attstattarget FROM pg_attribute WHERE attrelid = 'pg_type' :: REGCLASS ;

