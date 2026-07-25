-- 来源: 759_file_759.txt
-- SQL 数量: 8

SELECT * FROM pg_tables ;

\ d + customer_t1 ;

SELECT count ( * ) FROM customer_t1 ;

SELECT * FROM customer_t1 ;

SELECT c_customer_sk FROM customer_t1 ;

SELECT DISTINCT ( c_customer_sk ) FROM customer_t1 ;

SELECT * FROM customer_t1 WHERE c_customer_sk = 3869 ;

SELECT * FROM customer_t1 ORDER BY c_customer_sk ;

