-- 来源: 1011_Plan Hint.txt
-- SQL 数量: 2

EXPLAIN ANALYZE SELECT avg ( netpaid ) FROM ( select c_last_name , c_first_name , s_store_name , ca_state , s_state , i_color , i_current_price , i_manager_id , i_units , i_size , sum ( ss_sales_price ) netpaid FROM store_sales , store_returns , store , item , customer , customer_address WHERE ss_ticket_number = sr_ticket_number AND ss_item_sk = sr_item_sk AND ss_customer_sk = c_customer_sk AND ss_item_sk = i_item_sk AND ss_store_sk = s_store_sk AND c_birth_country = upper ( ca_country ) AND s_zip = ca_zip AND s_market_id = 7 GROUP BY c_last_name , c_first_name , s_store_name , ca_state , s_state , i_color , i_current_price , i_manager_id , i_units , i_size );

EXPLAIN ANALYZE SELECT sum ( l_extendedprice ) / 7 . 0 AS avg_yearly FROM lineitem , part WHERE p_partkey = l_partkey AND p_brand = 'Brand#23' AND p_container = 'MED BOX' AND l_quantity < ( SELECT 0 . 2 * avg ( l_quantity ) FROM lineitem WHERE l_partkey = p_partkey );

