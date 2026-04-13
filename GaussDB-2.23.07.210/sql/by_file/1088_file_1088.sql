-- 来源: 1088_file_1088.txt
-- SQL 数量: 15

SELECT d_moy , d_fy_week_seq , rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

SELECT d_moy , d_fy_week_seq , Row_number () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

SELECT d_moy , d_fy_week_seq , dense_rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

SELECT d_moy , d_fy_week_seq , percent_rank () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

SELECT d_moy , d_fy_week_seq , cume_dist () OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim e_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

SELECT d_moy , d_fy_week_seq , ntile ( 3 ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

SELECT d_moy , d_fy_week_seq , lag ( d_moy , 3 , null ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

SELECT d_moy , d_fy_week_seq , lead ( d_fy_week_seq , 2 ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

SELECT d_moy , d_fy_week_seq , first_value ( d_fy_week_seq ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 7 ORDER BY 1 , 2 ;

SELECT d_moy , d_fy_week_seq , last_value ( d_moy ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 6 ORDER BY 1 , 2 ;

SELECT d_moy , d_fy_week_seq , nth_value ( d_fy_week_seq , 6 ) OVER ( PARTITION BY d_moy ORDER BY d_fy_week_seq ) FROM tpcds . date_dim WHERE d_moy < 4 AND d_fy_week_seq < 6 ORDER BY 1 , 2 ;

SELECT sales_group , sales_id , sales_amount , RATIO_TO_REPORT ( sales_amount ) OVER ( PARTITION BY sales_group ) FROM sales_int8 ORDER BY sales_id ;

SELECT sales_group , sales_id , sales_amount , TO_CHAR ( RATIO_TO_REPORT ( sales_amount ) OVER (), '$999eeee' ) FROM sales ORDER BY sales_id ;

CREATE OR REPLACE PROCEDURE proc IS CURSOR cur_1 IS SELECT RATIO_TO_REPORT ( sales_amount ) OVER () FROM sales_numeric ;

CALL PROC ();

