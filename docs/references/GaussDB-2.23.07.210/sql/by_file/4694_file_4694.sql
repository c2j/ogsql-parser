-- 来源: 4694_file_4694.txt
-- SQL 数量: 5

select * from hypopg_create_index('create index on bmsql_customer(c_w_id)');

set enable_hypo_index = on;

explain SELECT c_discount from bmsql_customer where c_w_id = 10;

explain SELECT c_discount from bmsql_customer where c_w_id = 10;

select * from hypopg_display_index();

