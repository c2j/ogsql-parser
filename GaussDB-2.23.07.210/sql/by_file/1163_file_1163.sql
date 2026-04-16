-- 来源: 1163_file_1163.txt
-- SQL 数量: 2

SELECT * FROM ts_stat ( 'SELECT to_tsvector(''english'', sr_reason_sk) FROM tpcds.store_returns WHERE sr_customer_sk < 10' ) ORDER BY nentry DESC , ndoc DESC , word LIMIT 10 ;

SELECT * FROM ts_stat ( 'SELECT to_tsvector(''english'', sr_reason_sk) FROM tpcds.store_returns WHERE sr_customer_sk < 10' , 'a' ) ORDER BY nentry DESC , ndoc DESC , word LIMIT 10 ;

