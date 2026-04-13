-- 来源: 1193_ALTER FOREIGN TABLE ().txt
-- SQL 数量: 5

CREATE SCHEMA tpcds ;

CREATE FOREIGN TABLE tpcds . customer_ft ( c_customer_sk integer , c_customer_id char ( 16 ) , c_current_cdemo_sk integer , c_current_hdemo_sk integer , c_current_addr_sk integer , c_first_shipto_date_sk integer , c_first_sales_date_sk integer , c_salutation char ( 10 ) , c_first_name char ( 20 ) , c_last_name char ( 30 ) , c_preferred_cust_flag char ( 1 ) , c_birth_day integer , c_birth_month integer , c_birth_year integer , c_birth_country varchar ( 20 ) , c_login char ( 13 ) , c_email_address char ( 50 ) , c_last_review_date char ( 10 ) ) SERVER gsmpp_server OPTIONS ( location 'gsfs://10.185.179.143:5000/customer1*.dat' , FORMAT 'TEXT' , DELIMITER '|' , encoding 'utf8' , mode 'Normal' ) READ ONLY ;

ALTER FOREIGN TABLE tpcds . customer_ft options ( drop mode );

DROP FOREIGN TABLE tpcds . customer_ft ;

DROP SCHEMA tpcds CASCADE ;

