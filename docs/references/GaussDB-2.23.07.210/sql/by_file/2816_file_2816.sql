-- 来源: 2816_file_2816.txt
-- SQL 数量: 2

select * , sys_connect_by_path ( name , '-' ) from connect_table start with id = 1 connect by prior id = pid ;

select * , connect_by_root ( name ) from connect_table start with id = 1 connect by prior id = pid ;

