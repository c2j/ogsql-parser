-- 来源: 758_file_758.txt
-- SQL 数量: 3

UPDATE customer_t1 SET c_customer_sk = 9876 WHERE c_customer_sk = 9527 ;

UPDATE customer_t1 SET c_customer_sk = c_customer_sk + 100 ;

UPDATE customer_t1 SET c_customer_id = 'Admin' , c_first_name = 'Local' WHERE c_customer_sk = 4421 ;

