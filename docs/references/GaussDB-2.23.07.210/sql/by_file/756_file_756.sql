-- 来源: 756_file_756.txt
-- SQL 数量: 1

CREATE TABLE customer_t1 ( c_customer_sk integer , c_customer_id char ( 5 ), c_first_name char ( 6 ), c_last_name char ( 8 ) ) distribute by hash ( c_last_name );

