-- 类别: DML_UPDATE
-- SQL 数量: 73

-- 来源: 1059_HLL
UPDATE helloworld SET set = hll_add ( set , hll_hash_integer ( 12345 )) WHERE id = 1 ;

-- 来源: 1059_HLL
UPDATE helloworld SET set = hll_add ( set , hll_hash_text ( 'hello world' )) WHERE id = 1 ;

-- 来源: 1110_file_1110
UPDATE part_tab1 SET a = 2 WHERE b = 1;

-- 来源: 1110_file_1110
UPDATE part_tab1 SET a = 3 WHERE b = 11;

-- 来源: 1110_file_1110
UPDATE /*+ indexscan(part_tab1) */ part_tab1 SET a = 4 WHERE b = 21;

-- 来源: 1110_file_1110
UPDATE subpart_tab1 SET sales_amt = 2 WHERE user_no='1';

-- 来源: 1110_file_1110
UPDATE subpart_tab1 SET sales_amt = 3 WHERE user_no='2';

-- 来源: 1110_file_1110
UPDATE subpart_tab1 SET sales_amt = 4 WHERE user_no='3';

-- 来源: 1110_file_1110
UPDATE /*+ indexscan(subpart_tab1) */ subpart_tab1 SET sales_amt = 5 WHERE user_no='4';

-- 来源: 1110_file_1110
UPDATE part_tab1 SET a = 2 WHERE b = 1;

-- 来源: 1110_file_1110
UPDATE part_tab1 SET a = 3 WHERE b = 11;

-- 来源: 1110_file_1110
UPDATE /*+ indexscan(part_tab1) */ part_tab1 SET a = 4 WHERE b = 21;

-- 来源: 1110_file_1110
UPDATE subpart_tab1 SET sales_amt = 2 WHERE user_no='1';

-- 来源: 1110_file_1110
UPDATE subpart_tab1 SET sales_amt = 3 WHERE user_no='2';

-- 来源: 1110_file_1110
UPDATE subpart_tab1 SET sales_amt = 4 WHERE user_no='3';

-- 来源: 1110_file_1110
UPDATE /*+ indexscan(subpart_tab1) */ subpart_tab1 SET sales_amt = 5 WHERE user_no='4';

-- 来源: 1137_file_1137
update Students set id = id + 5 WHERE rownum < 4 ;

-- 来源: 1152_file_1152
UPDATE tsearch . pgweb SET textsearchable_index_col = to_tsvector ( 'english' , coalesce ( title , '' ) || ' ' || coalesce ( body , '' ));

-- 来源: 1155_file_1155
UPDATE tsearch . tt SET ti = setweight ( to_tsvector ( coalesce ( title , '' )), 'A' ) || setweight ( to_tsvector ( coalesce ( keyword , '' )), 'B' ) || setweight ( to_tsvector ( coalesce ( abstract , '' )), 'C' ) || setweight ( to_tsvector ( coalesce ( body , '' )), 'D' );

-- 来源: 1162_file_1162
UPDATE tsearch . aliases SET s = to_tsquery ( 'supernovae|sn & !nebulae' ) WHERE t = to_tsquery ( 'supernovae' );

-- 来源: 1184_ABORT
UPDATE customer_demographics_t1 SET cd_education_status = 'Unknown' ;

-- 来源: 1269_CREATE SYNONYM
UPDATE t1 SET t1 . name = 'cici' WHERE t1 . id = 2 ;

-- 来源: 1276_CREATE TRIGGER
UPDATE test_trigger_src_tbl SET id3 = 400 WHERE id1 = 100 ;

--修改tbl_test1表中所有数据的info列。
-- 来源: 1385_UPDATE
UPDATE tbl_test1 SET info = 'aa';

-- 来源: 1385_UPDATE
UPDATE tbl_test1 SET info = 'bb' WHERE id = 2;

-- 来源: 1385_UPDATE
UPDATE tbl_test1 SET info = 'ABC' WHERE id = 1 RETURNING info;

-- 来源: 1489_file_1489
UPDATE test_lock SET a=SYSDATE WHERE id =11;

-- 来源: 2431_file_2431
update a set value = 6 where id = 1;

-- 来源: 2431_file_2431
update a set value = 6 where id = 2;

-- 来源: 2456_file_2456
UPDATE customer_t1 SET c_customer_sk = 9876 WHERE c_customer_sk = 9527 ;

-- 来源: 2456_file_2456
UPDATE customer_t1 SET c_customer_sk = c_customer_sk + 100 ;

-- 来源: 2456_file_2456
UPDATE customer_t1 SET c_customer_id = 'Admin' , c_first_name = 'Local' WHERE c_customer_sk = 4421 ;

-- 来源: 2755_HLL
UPDATE helloworld SET set = hll_add ( set , hll_hash_integer ( 12345 )) WHERE id = 1 ;

-- 来源: 2755_HLL
UPDATE helloworld SET set = hll_add ( set , hll_hash_text ( 'hello world' )) WHERE id = 1 ;

-- 来源: 2801_file_2801
UPDATE t1 SET b = 2 WHERE a = 1;

-- 来源: 2801_file_2801
UPDATE t2 set a = 2 where a = 1;

-- 来源: 2808_file_2808
UPDATE part_tab1 SET a = 2 WHERE b = 1;

-- 来源: 2808_file_2808
UPDATE part_tab1 SET a = 3 WHERE b = 11;

-- 来源: 2808_file_2808
UPDATE /*+ indexscan(part_tab1) */ part_tab1 SET a = 4 WHERE b = 21;

-- 来源: 2808_file_2808
UPDATE subpart_tab1 SET sales_amt = 2 WHERE user_no='1';

-- 来源: 2808_file_2808
UPDATE subpart_tab1 SET sales_amt = 3 WHERE user_no='2';

-- 来源: 2808_file_2808
UPDATE subpart_tab1 SET sales_amt = 4 WHERE user_no='3';

-- 来源: 2808_file_2808
UPDATE /*+ indexscan(subpart_tab1) */ subpart_tab1 SET sales_amt = 5 WHERE user_no='4';

-- 来源: 2808_file_2808
UPDATE part_tab1 SET a = 2 WHERE b = 1;

-- 来源: 2808_file_2808
UPDATE part_tab1 SET a = 3 WHERE b = 11;

-- 来源: 2808_file_2808
UPDATE /*+ indexscan(part_tab1) */ part_tab1 SET a = 4 WHERE b = 21;

-- 来源: 2808_file_2808
UPDATE subpart_tab1 SET sales_amt = 2 WHERE user_no='1';

-- 来源: 2808_file_2808
UPDATE subpart_tab1 SET sales_amt = 3 WHERE user_no='2';

-- 来源: 2808_file_2808
UPDATE subpart_tab1 SET sales_amt = 4 WHERE user_no='3';

-- 来源: 2808_file_2808
UPDATE /*+ indexscan(subpart_tab1) */ subpart_tab1 SET sales_amt = 5 WHERE user_no='4';

-- 来源: 2835_file_2835
update Students set id = id + 5 WHERE rownum < 4 ;

-- 来源: 2850_file_2850
UPDATE tsearch . pgweb SET textsearchable_index_col = to_tsvector ( 'english' , coalesce ( title , '' ) || ' ' || coalesce ( body , '' ));

-- 来源: 2853_file_2853
UPDATE tsearch . tt SET ti = setweight ( to_tsvector ( coalesce ( title , '' )), 'A' ) || setweight ( to_tsvector ( coalesce ( keyword , '' )), 'B' ) || setweight ( to_tsvector ( coalesce ( abstract , '' )), 'C' ) || setweight ( to_tsvector ( coalesce ( body , '' )), 'D' );

-- 来源: 2860_file_2860
UPDATE tsearch . aliases SET s = to_tsquery ( 'supernovae|sn & !nebulae' ) WHERE t = to_tsquery ( 'supernovae' );

--更新字段值。
-- 来源: 2882_ABORT
UPDATE customer_demographics_t1 SET cd_education_status= 'Unknown';

-- 来源: 2975_CREATE SYNONYM
UPDATE t1 SET t1.name = 'cici' WHERE t1.id = 2;

--指定分区更新数据
-- 来源: 2980_CREATE TABLE SUBPARTITION
update range_list partition (p_201901) set user_no = '2';

-- 来源: 2980_CREATE TABLE SUBPARTITION
update range_list subpartition (p_201901_a) set user_no = '3';

-- 来源: 2980_CREATE TABLE SUBPARTITION
update range_list partition for ('201902') set user_no = '4';

-- 来源: 2980_CREATE TABLE SUBPARTITION
update range_list subpartition for ('201902','2') set user_no = '5';

--执行UPDATE触发事件并检查触发结果
-- 来源: 2983_CREATE TRIGGER
UPDATE test_trigger_src_tbl SET id3=400 WHERE id1=100;

--修改tbl_test1表中所有数据的info列。
-- 来源: 3099_UPDATE
UPDATE tbl_test1 SET info = 'aa';

-- 来源: 3099_UPDATE
UPDATE tbl_test1 SET info = 'bb' WHERE id = 2;

-- 来源: 3099_UPDATE
UPDATE tbl_test1 SET info = 'ABC' WHERE id = 1 RETURNING info;

-- 更新加密表中数据
-- 来源: 4280_gsql
update creditcard_info set credit_card = '80000000011111111' where name = 'joy';

-- 将分区值100所属分区，即分区p_list_4的数据进行更新
-- 来源: 4313_DQL_DML
UPDATE list_02 PARTITION FOR (100) SET data = '';

-- 更新加密表中数据
-- 来源: 4500_gsql
update creditcard_info set credit_card = '80000000011111111' where name = 'joy';

-- 将一级分区值100所属分区的数据进行更新
-- 来源: 4543_DQL_DML
UPDATE list_list_02 PARTITION FOR (100) SET id = 1;

-- 来源: 733_file_733
update a set value = 6 where id = 1;

-- 来源: 733_file_733
update a set value = 6 where id = 2;

-- 来源: 758_file_758
UPDATE customer_t1 SET c_customer_sk = 9876 WHERE c_customer_sk = 9527 ;

-- 来源: 758_file_758
UPDATE customer_t1 SET c_customer_sk = c_customer_sk + 100 ;

-- 来源: 758_file_758
UPDATE customer_t1 SET c_customer_id = 'Admin' , c_first_name = 'Local' WHERE c_customer_sk = 4421 ;

