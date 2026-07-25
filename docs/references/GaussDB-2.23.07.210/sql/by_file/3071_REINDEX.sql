-- 来源: 3071_REINDEX.txt
-- SQL 数量: 13

CREATE SCHEMA tpcds;

--创建表tpcds. customer。
CREATE TABLE tpcds.customer ( c_customer_sk INTEGER NOT NULL, c_customer_id CHAR(16) NOT NULL );

--向表中插入多条记录。
INSERT INTO tpcds.customer VALUES (1, 'AAAAAAAABAAAAAAA'),(5, 'AAAAAAAACAAAAAAA'),(10, 'AAAAAAAADAAAAAAA');

--创建一个行存表 tpcds. customer_t1，并在 tpcds. customer_t1表上的c_customer_sk字段创建索引。
CREATE TABLE tpcds. customer_t1 ( c_customer_sk integer not null, c_customer_id char(16) not null, c_current_cdemo_sk integer , c_current_hdemo_sk integer , c_current_addr_sk integer , c_first_shipto_date_sk integer , c_first_sales_date_sk integer , c_salutation char(10) , c_first_name char(20) , c_last_name char(30) , c_preferred_cust_flag char(1) , c_birth_day integer , c_birth_month integer , c_birth_year integer , c_birth_country varchar(20) , c_login char(13) , c_email_address char(50) , c_last_review_date char(10) ) WITH (orientation = row);

CREATE INDEX tpcds_customer_index1 ON tpcds. customer_t1 (c_customer_sk);

INSERT INTO tpcds. customer_t1 SELECT * FROM tpcds. customer WHERE c_customer_sk < 10;

--重建一个单独索引。
REINDEX INDEX tpcds. tpcds_customer_index1;

--在线重建一个单独索引。
REINDEX INDEX CONCURRENTLY tpcds. tpcds_customer_index1;

--重建表 tpcds. customer_t1上的所有索引。
REINDEX TABLE tpcds. customer_t1;

--在线重建表 tpcds. customer_t1上的所有索引。
REINDEX TABLE CONCURRENTLY tpcds. customer_t1;

--删除 tpcds. customer_t1表。
DROP TABLE tpcds. customer_t1;

--删除表。
DROP TABLE tpcds.customer;

--删除SCHEMA。
DROP SCHEMA tpcds CASCADE;

