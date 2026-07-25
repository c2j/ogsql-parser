-- 来源: 1360_REINDEX.txt
-- SQL 数量: 13

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . customer ( c_customer_sk INTEGER NOT NULL , c_customer_id CHAR ( 16 ) NOT NULL );

INSERT INTO tpcds . customer VALUES ( 1 , 'AAAAAAAABAAAAAAA' ),( 5 , 'AAAAAAAACAAAAAAA' ),( 10 , 'AAAAAAAADAAAAAAA' );

CREATE TABLE tpcds . customer_t1 ( c_customer_sk integer not null , c_customer_id char ( 16 ) not null , c_current_cdemo_sk integer , c_current_hdemo_sk integer , c_current_addr_sk integer , c_first_shipto_date_sk integer , c_first_sales_date_sk integer , c_salutation char ( 10 ) , c_first_name char ( 20 ) , c_last_name char ( 30 ) , c_preferred_cust_flag char ( 1 ) , c_birth_day integer , c_birth_month integer , c_birth_year integer , c_birth_country varchar ( 20 ) , c_login char ( 13 ) , c_email_address char ( 50 ) , c_last_review_date char ( 10 ) ) WITH ( orientation = row );

CREATE INDEX tpcds_customer_index1 ON tpcds . customer_t1 ( c_customer_sk );

INSERT INTO tpcds . customer_t1 SELECT * FROM tpcds . customer WHERE c_customer_sk < 10 ;

REINDEX INDEX tpcds . tpcds_customer_index1 ;

REINDEX INDEX CONCURRENTLY tpcds . tpcds_customer_index1 ;

REINDEX TABLE tpcds . customer_t1 ;

REINDEX TABLE CONCURRENTLY tpcds . customer_t1 ;

DROP TABLE tpcds . customer_t1 ;

DROP TABLE tpcds . customer ;

DROP SCHEMA tpcds CASCADE ;

