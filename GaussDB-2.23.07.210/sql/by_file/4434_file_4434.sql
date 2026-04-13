-- 来源: 4434_file_4434.txt
-- SQL 数量: 8

select * from hypopg_create_index('create index on bmsql_customer(c_w_id)');

set enable_hypo_index = on;

explain SELECT c_discount from bmsql_customer where c_w_id = 10;

explain SELECT c_discount from bmsql_customer where c_w_id = 10;

select * from hypopg_display_index();

select * from hypopg_estimate_size(329729);

select * from hypopg_drop_index(329726);

select * from hypopg_reset_index();

