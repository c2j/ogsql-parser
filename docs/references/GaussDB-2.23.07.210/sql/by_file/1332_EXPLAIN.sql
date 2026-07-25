-- 来源: 1332_EXPLAIN.txt
-- SQL 数量: 14

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . customer_address ( ca_address_sk INTEGER NOT NULL , ca_address_id CHARACTER ( 16 ) NOT NULL );

INSERT INTO tpcds . customer_address VALUES ( 5000 , 'AAAAAAAABAAAAAAA' ),( 10000 , 'AAAAAAAACAAAAAAA' );

CREATE TABLE tpcds . customer_address_p1 AS TABLE tpcds . customer_address ;

SET explain_perf_mode = normal ;

EXPLAIN SELECT * FROM tpcds . customer_address_p1 ;

EXPLAIN ( FORMAT JSON ) SELECT * FROM tpcds . customer_address_p1 ;

EXPLAIN SELECT * FROM tpcds . customer_address_p1 WHERE ca_address_sk = 10000 ;

EXPLAIN ( FORMAT YAML ) SELECT * FROM tpcds . customer_address_p1 WHERE ca_address_sk = 10000 ;

EXPLAIN ( COSTS FALSE ) SELECT * FROM tpcds . customer_address_p1 WHERE ca_address_sk = 10000 ;

EXPLAIN SELECT SUM ( ca_address_sk ) FROM tpcds . customer_address_p1 WHERE ca_address_sk < 10000 ;

DROP TABLE tpcds . customer_address_p1 ;

DROP TABLE tpcds . customer_address ;

DROP SCHEMA tpcds CASCADE ;

