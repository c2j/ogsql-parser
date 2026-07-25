-- 类别: DML_DELETE
-- SQL 数量: 45

-- 来源: 1110_file_1110
DELETE FROM part_tab1;

-- 来源: 1110_file_1110
DELETE FROM subpart_tab1;

-- 来源: 1110_file_1110
DELETE FROM part_tab1;

-- 来源: 1110_file_1110
DELETE FROM subpart_tab1;

-- 来源: 1124_XML
DELETE FROM XMLTEST ;

-- 来源: 1270_CREATE TABLE
DELETE FROM t_news ;

-- 来源: 1276_CREATE TRIGGER
DELETE FROM test_trigger_src_tbl WHERE id1 = 100 ;

-- 来源: 1286_DELETE
DELETE FROM tpcds . customer_address_bak WHERE ca_address_sk < 14888 ;

-- 来源: 1286_DELETE
DELETE FROM tpcds . customer_address_bak ;

-- 来源: 1333_EXPLAIN PLAN
DELETE FROM plan_table WHERE STATEMENT_ID = 'TPCH-Q4' ;

-- 来源: 1345_LOCK
DELETE FROM tpcds . reason_t1 WHERE r_reason_desc IN ( SELECT r_reason_desc FROM tpcds . reason_t1 WHERE r_reason_sk < 6 );

-- 来源: 1345_LOCK
DELETE FROM tpcds . reason_t1 WHERE r_reason_sk = 7 ;

-- 来源: 1452_file_1452
DELETE FROM EXAMPLE1;

-- 来源: 2366_file_2366
delete from tab_1;

-- 来源: 2431_file_2431
delete a where id = 1;

-- 来源: 2458_file_2458
DELETE FROM customer_t1 WHERE c_customer_sk = 3869 ;

-- 来源: 2458_file_2458
DELETE FROM customer_t1 ;

-- 来源: 2801_file_2801
DELETE FROM t1;

-- 来源: 2801_file_2801
DELETE FROM t2;

-- 来源: 2808_file_2808
DELETE FROM part_tab1;

-- 来源: 2808_file_2808
DELETE FROM subpart_tab1;

-- 来源: 2808_file_2808
DELETE FROM part_tab1;

-- 来源: 2808_file_2808
DELETE FROM subpart_tab1;

-- 来源: 2821_XML
DELETE FROM XMLTEST ;

--指定分区删除数据
-- 来源: 2980_CREATE TABLE SUBPARTITION
delete from range_list partition (p_201901);

-- 来源: 2980_CREATE TABLE SUBPARTITION
delete from range_list partition for ('201903');

-- 来源: 2980_CREATE TABLE SUBPARTITION
delete from range_list subpartition (p_201901_a);

-- 来源: 2980_CREATE TABLE SUBPARTITION
delete from range_list subpartition for ('201903','2');

-- 来源: 2980_CREATE TABLE SUBPARTITION
delete from range_list as t partition (p_201901_a, p_201901);

--执行DELETE触发事件并检查触发结果
-- 来源: 2983_CREATE TRIGGER
DELETE FROM test_trigger_src_tbl WHERE id1=100;

--删除 tpcds. customer_address_bak中ca_address_sk大于14888的职员。
-- 来源: 2993_DELETE
DELETE FROM tpcds. customer_address_bak WHERE ca_address_sk > 14888;

--同时删除 tpcds. customer_address和 tpcds. customer_address_bak中ca_address_sk小于50的职员。
-- 来源: 2993_DELETE
DELETE FROM tpcds. customer_address a, tpcds. customer_address_bak b where a.ca_address_sk = b.ca_address_sk and a.ca_address_sk < 50;

--删除 tpcds. customer_address_bak中所有数据。
-- 来源: 2993_DELETE
DELETE FROM tpcds. customer_address_bak;

-- 来源: 3044_EXPLAIN PLAN
DELETE FROM plan_table WHERE STATEMENT_ID = 'TPCH-Q4' ;

-- 来源: 3056_LOCK
DELETE FROM tpcds. reason_t1 WHERE r_reason_desc IN(SELECT r_reason_desc FROM tpcds. reason_t1 WHERE r_reason_sk < 6 );

-- 来源: 3056_LOCK
DELETE FROM tpcds. reason_t1 WHERE r_reason_sk = 7;

-- 来源: 3082_SELECT
DELETE tpcds.time_table;

-- 来源: 3162_file_3162
DELETE FROM EXAMPLE1;

-- 来源: 4027_file_4027
delete from tab_1;

-- 来源: 4313_DQL_DML
DELETE FROM list_02 PARTITION (p_list_5);

-- 来源: 4507_gsql
delete from contacts;

-- 来源: 4543_DQL_DML
DELETE FROM list_list_02 PARTITION (p_list_5);

-- 来源: 733_file_733
delete a where id = 1;

-- 来源: 760_file_760
DELETE FROM customer_t1 WHERE c_customer_sk = 3869 ;

-- 来源: 760_file_760
DELETE FROM customer_t1 ;

