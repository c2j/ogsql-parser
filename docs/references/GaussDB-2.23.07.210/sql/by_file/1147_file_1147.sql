-- 来源: 1147_file_1147.txt
-- SQL 数量: 1

SELECT d_dow || '-' || d_dom || '-' || d_fy_week_seq AS identify_serials FROM tpcds . date_dim WHERE d_fy_week_seq = 1 ;

