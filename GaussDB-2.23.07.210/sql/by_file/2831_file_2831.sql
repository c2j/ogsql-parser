-- 来源: 2831_file_2831.txt
-- SQL 数量: 4

SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE EXISTS ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom = store_returns . sr_reason_sk and sr_customer_sk < 10 );

SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE sr_customer_sk IN ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom < 10 );

SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE sr_customer_sk < ANY ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom < 10 );

SELECT sr_reason_sk , sr_customer_sk FROM tpcds . store_returns WHERE sr_customer_sk < all ( SELECT d_dom FROM tpcds . date_dim WHERE d_dom < 10 );

