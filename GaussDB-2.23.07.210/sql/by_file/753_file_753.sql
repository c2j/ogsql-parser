-- 来源: 753_file_753.txt
-- SQL 数量: 4

CREATE TABLE customer_t1 ( state_ID CHAR ( 2 ), state_NAME VARCHAR2 ( 40 ), area_ID NUMBER );

DROP TABLE customer_t1 ;

CREATE TABLE customer_t2 ( state_ID CHAR ( 2 ), state_NAME VARCHAR2 ( 40 ), area_ID NUMBER ) WITH ( ORIENTATION = COLUMN );

DROP TABLE customer_t2 ;

