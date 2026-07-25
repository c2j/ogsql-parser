-- 来源: 3043_EXPLAIN.txt
-- SQL 数量: 26

CREATE TABLE student(id int, name char(20));

EXPLAIN (NODE true) INSERT INTO student VALUES(5,'a'),(6,'b');

EXPLAIN (NUM_NODES true) INSERT INTO student VALUES(5,'a'),(6,'b');

CREATE SCHEMA tpcds;

--创建表tpcds.customer_address。
CREATE TABLE tpcds.customer_address ( ca_address_sk INTEGER NOT NULL, ca_address_id CHARACTER(16) NOT NULL );

--向表中插入多条记录。
INSERT INTO tpcds.customer_address VALUES (5000, 'AAAAAAAABAAAAAAA'),(10000, 'AAAAAAAACAAAAAAA');

--创建一个表 tpcds. customer_address_p1。
CREATE TABLE tpcds. customer_address_p1 AS TABLE tpcds. customer_address;

--修改explain_perf_mode为normal。
SET explain_perf_mode=normal;

--显示表简单查询的执行计划。
EXPLAIN SELECT * FROM tpcds. customer_address_p1;

--以JSON格式输出的执行计划（explain_perf_mode为normal时）。
EXPLAIN(FORMAT JSON) SELECT * FROM tpcds. customer_address_p1;

--如果有一个索引，当使用一个带索引WHERE条件的查询，可能会显示一个不同的计划。
EXPLAIN SELECT * FROM tpcds. customer_address_p1 WHERE ca_address_sk=10000;

--以YAML格式输出的执行计划（explain_perf_mode为normal时）。
EXPLAIN(FORMAT YAML) SELECT * FROM tpcds. customer_address_p1 WHERE ca_address_sk=10000;

--禁止开销估计的执行计划。
EXPLAIN(COSTS FALSE) SELECT * FROM tpcds. customer_address_p1 WHERE ca_address_sk=10000;

--带有聚集函数查询的执行计划。
EXPLAIN SELECT SUM(ca_address_sk) FROM tpcds. customer_address_p1 WHERE ca_address_sk<10000;

--创建一个二级分区表。
CREATE TABLE range_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY LIST (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a values ('1'), SUBPARTITION p_201901_b values ('2') ), PARTITION p_201902 VALUES LESS THAN( '201910' ) ( SUBPARTITION p_201902_a values ('1'), SUBPARTITION p_201902_b values ('2') ) );

--执行带有二级分区表的查询语句。
--Iterations 和 Sub Iterations分别标识遍历了几个一级分区和二级分区。
--Selected Partitions标识哪些一级分区被实际扫描，Selected Subpartitions: (p:s)标识第p个一级分区下s个二级分区被实际扫描，如果一级分区下所有二级分区都被扫描则s显示为ALL。
EXPLAIN SELECT * FROM range_list WHERE dept_code = '1';

--删除表 tpcds. customer_address_p1。
DROP TABLE tpcds. customer_address_p1;

--删除表tpcds.customer_address。
DROP TABLE tpcds.customer_address;

--删除表range_list。
DROP TABLE range_list;

--删除SCHEMA。
DROP SCHEMA tpcds CASCADE;

CREATE TABLE tb_a(c1 int);

INSERT INTO tb_a VALUES(1),(2),(3);

CREATE TABLE tb_b AS SELECT * FROM tb_a;

EXPLAIN (OPTEVAL on )SELECT * FROM tb_a a, tb_b b WHERE a.c1=b.c1 AND a.c1=1;

--删除表tb_a，tb_b。
DROP TABLE tb_a;

DROP TABLE tb_b;

