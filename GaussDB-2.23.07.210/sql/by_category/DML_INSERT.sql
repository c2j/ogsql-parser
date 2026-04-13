-- 类别: DML_INSERT
-- SQL 数量: 775

-- 来源: 1024_SQL PATCH
insert into hint_t1 values ( 1 , 1 , 1 );

-- 来源: 1024_SQL PATCH
insert into test_proc_patch values(1,2);

--插入数据。
-- 来源: 1047_file_1047
INSERT INTO int_type_t1 VALUES(10,20);

-- 来源: 1047_file_1047
INSERT INTO int_type_t2 VALUES ( 100 , 10 , 1000 , 10000 , 200 , 2000 );

--插入数据。
-- 来源: 1047_file_1047
INSERT INTO decimal_type_t1 VALUES(123456.122331);

-- 来源: 1047_file_1047
INSERT INTO numeric_type_t1 VALUES ( 123456 . 12354 );

-- 来源: 1047_file_1047
INSERT INTO smallserial_type_tab VALUES ( default );

-- 来源: 1047_file_1047
INSERT INTO smallserial_type_tab VALUES ( default );

-- 来源: 1047_file_1047
INSERT INTO serial_type_tab VALUES ( default );

-- 来源: 1047_file_1047
INSERT INTO serial_type_tab VALUES ( default );

-- 来源: 1047_file_1047
INSERT INTO bigserial_type_tab VALUES ( default );

-- 来源: 1047_file_1047
INSERT INTO bigserial_type_tab VALUES ( default );

-- 来源: 1047_file_1047
INSERT INTO float_type_t2 VALUES ( 10 , 10 . 365456 , 123456 . 1234 , 10 . 3214 , 321 . 321 , 123 . 123654 , 123 . 123654 );

-- 来源: 1049_file_1049
INSERT INTO bool_type_t1 VALUES ( TRUE , 'sic est' );

-- 来源: 1049_file_1049
INSERT INTO bool_type_t1 VALUES ( FALSE , 'non est' );

-- varchar为1073741728，超过规定长度，插入失败
-- 来源: 1050_file_1050
insert into varchar_maxlength_test1 values(1, repeat('a', 1073741728), 1);

-- varchar为1073741727，长度符合要求，插入成功
-- 来源: 1050_file_1050
insert into varchar_maxlength_test1 values(1, repeat('a', 1073741727), 1);

--插入数据。
-- 来源: 1050_file_1050
INSERT INTO char_type_t1 VALUES ('ok');

-- 来源: 1050_file_1050
INSERT INTO char_type_t2 VALUES ( 'ok' );

-- 来源: 1050_file_1050
INSERT INTO char_type_t2 VALUES ( 'good' );

-- 来源: 1050_file_1050
INSERT INTO char_type_t2 VALUES ( 'too long' );

-- 来源: 1050_file_1050
INSERT INTO char_type_t2 VALUES ( 'too long' :: varchar ( 5 ));

-- 来源: 1051_file_1051
INSERT INTO blob_type_t1 VALUES ( 10 , empty_blob (), HEXTORAW ( 'DEADBEEF' ), E '\\xDEADBEEF' );

-- 来源: 1052__
INSERT INTO date_type_tab VALUES ( date '12-10-2010' );

-- 来源: 1052__
INSERT INTO time_type_tab VALUES ( '21:21:21' , '21:21:21 pst' , '2010-12-12' , '2013-12-11 pst' , '2003-04-12 04:05:06' );

-- 来源: 1052__
INSERT INTO day_type_tab VALUES ( 1 , INTERVAL '3' DAY );

-- 来源: 1052__
INSERT INTO year_type_tab VALUES ( 1 , interval '2' year );

-- 来源: 1052__
INSERT INTO date_type_tab VALUES ( date '12-10-2010' );

-- 来源: 1052__
INSERT INTO date_type_tab VALUES ( date '2010-12-11' );

--插入数据。
-- 来源: 1052__
INSERT INTO realtime_type_special VALUES('epoch', 'epoch', 'epoch', NULL);

-- 来源: 1052__
INSERT INTO realtime_type_special VALUES('now', 'now', 'now', 'now');

-- 来源: 1052__
INSERT INTO realtime_type_special VALUES('today', 'today', 'today', NULL);

-- 来源: 1052__
INSERT INTO realtime_type_special VALUES('tomorrow', 'tomorrow', 'tomorrow', NULL);

-- 来源: 1052__
INSERT INTO realtime_type_special VALUES('yesterday', 'yesterday', 'yesterday', NULL);

-- 来源: 1052__
INSERT INTO reltime_type_tab VALUES ( '90' , '90' );

-- 来源: 1052__
INSERT INTO reltime_type_tab VALUES ( '-366' , '-366' );

-- 来源: 1052__
INSERT INTO reltime_type_tab VALUES ( '1975.25' , '1975.25' );

-- 来源: 1052__
INSERT INTO reltime_type_tab VALUES ( '-2 YEARS +5 MONTHS 10 DAYS' , '-2 YEARS +5 MONTHS 10 DAYS' );

-- 来源: 1052__
INSERT INTO reltime_type_tab VALUES ( '30 DAYS 12:00:00' , '30 DAYS 12:00:00' );

-- 来源: 1052__
INSERT INTO reltime_type_tab VALUES ( 'P-1.1Y10M' , 'P-1.1Y10M' );

-- 来源: 1055_file_1055
INSERT INTO bit_type_t1 VALUES ( 1 , B '101' , B '00' );

-- 来源: 1055_file_1055
INSERT INTO bit_type_t1 VALUES ( 2 , B '10' , B '101' );

-- 来源: 1055_file_1055
INSERT INTO bit_type_t1 VALUES ( 2 , B '10' :: bit ( 3 ), B '101' );

-- 来源: 1059_HLL
INSERT INTO t1 VALUES ( 1 , hll_empty ( 14 , - 1 ));

-- 来源: 1059_HLL
INSERT INTO t1 ( id , set ) VALUES ( 1 , hll_empty ( 14 , 5 ));

-- 来源: 1059_HLL
INSERT INTO helloworld ( id , set ) VALUES ( 1 , hll_empty ());

-- 来源: 1059_HLL
INSERT INTO facts VALUES ( '2019-02-20' , generate_series ( 1 , 100 ));

-- 来源: 1059_HLL
INSERT INTO facts VALUES ( '2019-02-21' , generate_series ( 1 , 200 ));

-- 来源: 1059_HLL
INSERT INTO facts VALUES ( '2019-02-22' , generate_series ( 1 , 300 ));

-- 来源: 1059_HLL
INSERT INTO facts VALUES ( '2019-02-23' , generate_series ( 1 , 400 ));

-- 来源: 1059_HLL
INSERT INTO facts VALUES ( '2019-02-24' , generate_series ( 1 , 500 ));

-- 来源: 1059_HLL
INSERT INTO facts VALUES ( '2019-02-25' , generate_series ( 1 , 600 ));

-- 来源: 1059_HLL
INSERT INTO facts VALUES ( '2019-02-26' , generate_series ( 1 , 700 ));

-- 来源: 1059_HLL
INSERT INTO facts VALUES ( '2019-02-27' , generate_series ( 1 , 800 ));

-- 来源: 1059_HLL
INSERT INTO daily_uniques ( date , users ) SELECT date , hll_add_agg ( hll_hash_integer ( user_id )) FROM facts GROUP BY 1 ;

-- 来源: 1059_HLL
INSERT INTO test VALUES ( 1 , 'E\\1234' );

-- 来源: 1060_file_1060
INSERT INTO reservation VALUES (1108, '[2010-01-01 14:30, 2010-01-01 15:30)');

-- 来源: 1062_file_1062
insert into t1 values ( 1 ),( 2 );

-- 来源: 1065_XML
INSERT INTO xmltest VALUES (1, 'one');

-- 来源: 1065_XML
INSERT INTO xmltest VALUES (2, 'two');

-- 来源: 1066_XMLTYPE
INSERT INTO xmltypetest VALUES (1, '<ss/>');

-- 来源: 1066_XMLTYPE
INSERT INTO xmltypetest VALUES (2, '<xx/>');

-- 来源: 1067_aclitem
INSERT INTO table_acl VALUES (1,'user1=arw/omm','{omm=d/user2,omm=w/omm}');

-- 来源: 1067_aclitem
INSERT INTO table_acl VALUES (2,'user1=aw/omm','{omm=d/user2}');

-- 来源: 1072_file_1072
INSERT INTO test_space values ( 'a' );

-- 来源: 1072_file_1072
insert into test ( a ) values ( 'abC 不' );

-- 来源: 1072_file_1072
insert into test ( a ) values ( 'abC 啊' );

-- 来源: 1072_file_1072
insert into test ( a ) values ( 'abc 啊' );

-- 来源: 1078_file_1078
insert into json_doc values ( '{"name":"a"}' );

-- 来源: 1082_JSON_JSONB
INSERT INTO classes VALUES('A',2);

-- 来源: 1082_JSON_JSONB
INSERT INTO classes VALUES('A',3);

-- 来源: 1082_JSON_JSONB
INSERT INTO classes VALUES('D',5);

-- 来源: 1082_JSON_JSONB
INSERT INTO classes VALUES('D',null);

-- 来源: 1082_JSON_JSONB
INSERT INTO classes VALUES('A',2);

-- 来源: 1082_JSON_JSONB
INSERT INTO classes VALUES('A',3);

-- 来源: 1082_JSON_JSONB
INSERT INTO classes VALUES('D',5);

-- 来源: 1082_JSON_JSONB
INSERT INTO classes VALUES('D',null);

-- 来源: 1083_HLL
INSERT INTO t_id VALUES ( generate_series ( 1 , 500 ));

-- 来源: 1083_HLL
INSERT INTO t_data SELECT mod ( id , 2 ), id FROM t_id ;

-- 来源: 1083_HLL
INSERT INTO t_a_c_hll SELECT a , hll_add_agg ( hll_hash_text ( c )) FROM t_data GROUP BY a ;

-- 来源: 1087_file_1087
INSERT INTO tab values ( 1 );

-- 来源: 1087_file_1087
INSERT INTO tab values ( 2 );

-- 来源: 1092_file_1092
INSERT INTO blob_tb VALUES ( empty_blob (), 1 );

-- 来源: 1092_file_1092
INSERT INTO clob_tb VALUES ( empty_clob (), 1 );

-- 来源: 1092_file_1092
INSERT INTO student_demo VALUES ( 'name0' , 0 );

-- 来源: 1092_file_1092
INSERT INTO student_demo VALUES ( 'name1' , 1 );

-- 来源: 1092_file_1092
INSERT INTO student_demo VALUES ( 'name2' , 2 );

-- 来源: 1108_file_1108
INSERT INTO test values(1,1);

-- 来源: 1110_file_1110
INSERT INTO part_tab1 VALUES(1, 1);

-- 来源: 1110_file_1110
INSERT INTO part_tab1 VALUES(1, 11);

-- 来源: 1110_file_1110
INSERT INTO part_tab1 VALUES(1, 21);

-- 来源: 1110_file_1110
INSERT INTO subpart_tab1 VALUES('201902', '1', '1', 1);

-- 来源: 1110_file_1110
INSERT INTO subpart_tab1 VALUES('201902', '2', '2', 1);

-- 来源: 1110_file_1110
INSERT INTO subpart_tab1 VALUES('201903', '1', '3', 1);

-- 来源: 1110_file_1110
INSERT INTO subpart_tab1 VALUES('201903', '2', '4', 1);

-- 来源: 1110_file_1110
INSERT INTO part_tab1 VALUES(1, 1);

-- 来源: 1110_file_1110
INSERT INTO part_tab1 VALUES(1, 11);

-- 来源: 1110_file_1110
INSERT INTO part_tab1 VALUES(1, 21);

-- 来源: 1110_file_1110
INSERT INTO subpart_tab1 VALUES('201902', '1', '1', 1);

-- 来源: 1110_file_1110
INSERT INTO subpart_tab1 VALUES('201902', '2', '2', 1);

-- 来源: 1110_file_1110
INSERT INTO subpart_tab1 VALUES('201903', '1', '3', 1);

-- 来源: 1110_file_1110
INSERT INTO subpart_tab1 VALUES('201903', '2', '4', 1);

-- 来源: 1124_XML
INSERT INTO xmltest VALUES ( 1 , '<value>one</value>' );

-- 来源: 1124_XML
INSERT INTO xmltest VALUES ( 2 , '<value>two</value>' );

-- 来源: 1124_XML
INSERT INTO xmltest VALUES ( 1 , '<?xml version="1.0" encoding="GBK"?><value>one</value>' );

-- 来源: 1124_XML
INSERT INTO xmltest VALUES ( 2 , '<?xml version="1.0" encoding="GBK"?><value>two</value>' );

-- 来源: 1132_file_1132
INSERT INTO tpcds . case_when_t1 VALUES ( 1 ), ( 2 ), ( 3 );

-- 来源: 1132_file_1132
INSERT INTO tpcds . c_tabl VALUES ( 'abc' , 'efg' , '123' );

-- 来源: 1132_file_1132
INSERT INTO tpcds . c_tabl VALUES ( NULL , 'efg' , '123' );

-- 来源: 1132_file_1132
INSERT INTO tpcds . c_tabl VALUES ( NULL , NULL , '123' );

-- 来源: 1132_file_1132
INSERT INTO tpcds . null_if_t1 VALUES ( 'abc' , 'abc' );

-- 来源: 1132_file_1132
INSERT INTO tpcds . null_if_t1 VALUES ( 'abc' , 'efg' );

-- 来源: 1137_file_1137
INSERT INTO Students VALUES ( 'Jack' , 35 );

-- 来源: 1137_file_1137
INSERT INTO Students VALUES ( 'Leon' , 15 );

-- 来源: 1137_file_1137
INSERT INTO Students VALUES ( 'James' , 24 );

-- 来源: 1137_file_1137
INSERT INTO Students VALUES ( 'Taker' , 81 );

-- 来源: 1137_file_1137
INSERT INTO Students VALUES ( 'Mary' , 25 );

-- 来源: 1137_file_1137
INSERT INTO Students VALUES ( 'Rose' , 64 );

-- 来源: 1137_file_1137
INSERT INTO Students VALUES ( 'Perl' , 18 );

-- 来源: 1137_file_1137
INSERT INTO Students VALUES ( 'Under' , 57 );

-- 来源: 1137_file_1137
INSERT INTO Students VALUES ( 'Angel' , 101 );

-- 来源: 1137_file_1137
INSERT INTO Students VALUES ( 'Frank' , 20 );

-- 来源: 1137_file_1137
INSERT INTO Students VALUES ( 'Charlie' , 40 );

-- 来源: 1137_file_1137
INSERT INTO test SELECT generate_series , generate_series FROM generate_series ( 1 , 10 );

-- 来源: 1137_file_1137
INSERT INTO test VALUES ( generate_series ( 1 , 10 ), generate_series ( 1 , 10 ));

-- 来源: 1142_file_1142
INSERT INTO tpcds . value_storage_t1 VALUES ( 'abcdef' );

-- 来源: 1151_file_1151
INSERT INTO tsearch . pgweb VALUES ( 1 , 'China, officially the People''s Republic of China (PRC), located in Asia, is the world''s most populous state.' , 'China' , '2010-1-1' );

-- 来源: 1151_file_1151
INSERT INTO tsearch . pgweb VALUES ( 2 , 'America is a rock band, formed in England in 1970 by multi-instrumentalists Dewey Bunnell, Dan Peek, and Gerry Beckley.' , 'America' , '2010-1-1' );

-- 来源: 1151_file_1151
INSERT INTO tsearch . pgweb VALUES ( 3 , 'England is a country that is part of the United Kingdom. It shares land borders with Scotland to the north and Wales to the west.' , 'England' , '2010-1-1' );

-- 来源: 1151_file_1151
INSERT INTO tsearch . pgweb VALUES ( 4 , 'Australia, officially the Commonwealth of Australia, is a country comprising the mainland of the Australian continent, the island of Tasmania, and numerous smaller islands.' , 'Australia' , '2010-1-1' );

-- 来源: 1151_file_1151
INSERT INTO tsearch . pgweb VALUES ( 6 , 'Japan is an island country in East Asia.' , 'Japan' , '2010-1-1' );

-- 来源: 1151_file_1151
INSERT INTO tsearch . pgweb VALUES ( 7 , 'Germany, officially the Federal Republic of Germany, is a sovereign state and federal parliamentary republic in central-western Europe.' , 'Germany' , '2010-1-1' );

-- 来源: 1151_file_1151
INSERT INTO tsearch . pgweb VALUES ( 8 , 'France, is a sovereign state comprising territory in western Europe and several overseas regions and territories.' , 'France' , '2010-1-1' );

-- 来源: 1151_file_1151
INSERT INTO tsearch . pgweb VALUES ( 9 , 'Italy officially the Italian Republic, is a unitary parliamentary republic in Europe.' , 'Italy' , '2010-1-1' );

-- 来源: 1151_file_1151
INSERT INTO tsearch . pgweb VALUES ( 10 , 'India, officially the Republic of India, is a country in South Asia.' , 'India' , '2010-1-1' );

-- 来源: 1151_file_1151
INSERT INTO tsearch . pgweb VALUES ( 11 , 'Brazil, officially the Federative Republic of Brazil, is the largest country in both South America and Latin America.' , 'Brazil' , '2010-1-1' );

-- 来源: 1151_file_1151
INSERT INTO tsearch . pgweb VALUES ( 12 , 'Canada is a country in the northern half of North America.' , 'Canada' , '2010-1-1' );

-- 来源: 1151_file_1151
INSERT INTO tsearch . pgweb VALUES ( 13 , 'Mexico, officially the United Mexican States, is a federal republic in the southern part of North America.' , 'Mexico' , '2010-1-1' );

-- 来源: 1155_file_1155
INSERT INTO tsearch . tt ( id , title , keyword , abstract , body ) VALUES ( 1 , 'China' , 'Beijing' , 'China' , 'China, officially the People''s Republic of China (PRC), located in Asia, is the world''s most populous state.' );

-- 来源: 1157_file_1157
INSERT INTO tsearch . ts_ngram VALUES ( 1 , '中文' );

-- 来源: 1157_file_1157
INSERT INTO tsearch . ts_ngram VALUES ( 2 , '中文检索' );

-- 来源: 1157_file_1157
INSERT INTO tsearch . ts_ngram VALUES ( 3 , '检索中文' );

-- 来源: 1162_file_1162
INSERT INTO tsearch . aliases VALUES ( 1 , to_tsquery ( 'supernovae' ), to_tsquery ( 'supernovae|sn' ));

-- 来源: 1184_ABORT
INSERT INTO customer_demographics_t1 VALUES ( 1920801 , 'M' , 'U' , 'DOCTOR DEGREE' , 200 , 'GOOD' , 1 , 0 , 0 );

-- 来源: 1233_CLUSTER
INSERT INTO test_c1 VALUES ( 3 , 'Joe' ),( 1 , 'Jack' ),( 2 , 'Scott' );

-- 来源: 1233_CLUSTER
INSERT INTO test_c2 VALUES (6,'ABBB'),(2,'ABAB'),(9,'AAAA');

-- 来源: 1233_CLUSTER
INSERT INTO test_c2 VALUES (11,'AAAB'),(19,'BBBA'),(16,'BABA');

-- 来源: 1235_COMMIT _ END
INSERT INTO tpcds . customer_demographics_t2 VALUES ( 1 , 'M' , 'U' , 'DOCTOR DEGREE' , 1200 , 'GOOD' , 1 , 0 , 0 );

-- 来源: 1235_COMMIT _ END
INSERT INTO tpcds . customer_demographics_t2 VALUES ( 2 , 'F' , 'U' , 'MASTER DEGREE' , 300 , 'BAD' , 1 , 0 , 0 );

-- 来源: 1237_COPY
INSERT INTO tpcds . ship_mode VALUES ( 1 , 'a' , 'b' , 'c' , 'd' , 'e' );

--基表写入数据。
-- 来源: 1251_CREATE INCREMENTAL MATERIALIZED VIEW
INSERT INTO my_table VALUES(1,1),(2,2);

-- 来源: 1254_CREATE MASKING POLICY
INSERT INTO tb_for_masking VALUES ( 1 , '9876543210' , 'usr321usr' , 'abc@huawei.com' , 'abc@huawei.com' , '1234-4567-7890-0123' , 'abcdef 123456 ui 323 jsfd321 j3k2l3' , '4880-9898-4545-2525' , 'this is a llt case' );

-- 来源: 1254_CREATE MASKING POLICY
INSERT INTO tb_for_masking VALUES ( 2 , '0123456789' , 'lltc123llt' , 'abc@gmail.com' , 'abc@gmail.com' , '9876-5432-1012-3456' , '1234 abcd ef 56 gh78ijk90lm' , '4856-7654-1234-9865' , 'this,is.a!LLT?case' );

--基表写入数据。
-- 来源: 1255_CREATE MATERIALIZED VIEW
INSERT INTO my_table VALUES(1,1),(2,2);

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
INSERT INTO all_data VALUES ( 1 , 'alice' , 'alice data' );

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
INSERT INTO all_data VALUES ( 2 , 'bob' , 'bob data' );

-- 来源: 1264_CREATE ROW LEVEL SECURITY POLICY
INSERT INTO all_data VALUES ( 3 , 'peter' , 'peter data' );

-- 来源: 1267_CREATE SEQUENCE
INSERT INTO test1 ( name ) values ( 'Joe' ),( 'Scott' ),( 'Ben' );

-- 来源: 1269_CREATE SYNONYM
INSERT INTO t1 VALUES ( 1 , 'ada' ), ( 2 , 'bob' );

-- 来源: 1270_CREATE TABLE
INSERT INTO lrt_range VALUES ( generate_series ( 1 , 4 ), generate_series ( 1 , 4 ));

-- 来源: 1270_CREATE TABLE
INSERT INTO t_news values ( 'china' , '2020' , '张三' , 21 );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_news values ( 'china' , '2021' , '张三' , 21 );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_news values ( 'china' , '2022' , '张三' , 21 );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_news values ( 'china' , '2023' , '张三' , 21 );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_news values ( 'china' , '2024' , '张三' , 21 );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_news values ( 'china' , '2025' , '张三' , 21 );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_news values ( 'Japan' , '2020' , '赵六' , 18 ),( 'Japan' , '2021' , '赵六' , 19 ),( 'Japan' , '2022' , '赵六' , 20 ),( 'Japan' , '2027' , '赵六' , 21 );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_ran1 values ( 9 , 5 , 'a' );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_ran1 values ( 9 , 20 , 'a' );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_ran1 values ( 9 , 21 , 'a' );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_ran1 values ( 10 , 5 , 'a' );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_ran1 values ( 10 , 15 , 'a' );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_ran1 values ( 10 , 20 , 'a' );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_ran1 values ( 10 , 21 , 'a' );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_ran1 values ( 11 , 5 , 'a' );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_ran1 values ( 11 , 20 , 'a' );

-- 来源: 1270_CREATE TABLE
INSERT INTO t_ran1 values ( 11 , 21 , 'a' );

-- 来源: 1272_CREATE TABLE AS
INSERT INTO test1 VALUES (1,'col1'),(101,'col101');

-- 来源: 1273_CREATE TABLE PARTITION
INSERT INTO tpcds . web_returns_p1 SELECT * FROM tpcds . web_returns ;

-- 来源: 1273_CREATE TABLE PARTITION
INSERT INTO tpcds . startend_pt VALUES ( GENERATE_SERIES ( 0 , 4999 ), GENERATE_SERIES ( 0 , 4999 ));

-- 来源: 1273_CREATE TABLE PARTITION
INSERT INTO test_list VALUES ( 2000 , 2000 );

-- 来源: 1273_CREATE TABLE PARTITION
INSERT INTO test_list VALUES ( 3000 , 3000 );

-- 来源: 1273_CREATE TABLE PARTITION
INSERT INTO test_list VALUES ( 6000 , 6000 );

-- 来源: 1273_CREATE TABLE PARTITION
INSERT INTO test_list VALUES ( 6000 , 6000 );

-- 来源: 1273_CREATE TABLE PARTITION
INSERT INTO test_list VALUES ( 6000 , 6000 );

-- 来源: 1273_CREATE TABLE PARTITION
INSERT INTO test_hash VALUES ( 1 , 1 );

-- 来源: 1273_CREATE TABLE PARTITION
INSERT INTO test_hash VALUES ( 2 , 2 );

-- 来源: 1273_CREATE TABLE PARTITION
INSERT INTO test_hash VALUES ( 3 , 3 );

-- 来源: 1273_CREATE TABLE PARTITION
INSERT INTO test_hash VALUES ( 4 , 4 );

-- 来源: 1276_CREATE TRIGGER
INSERT INTO test_trigger_src_tbl VALUES ( 100 , 200 , 300 );

-- 来源: 1277_CREATE TYPE
INSERT INTO t1_compfoo values ( 1 ,( 1 , 'demo' ));

-- 来源: 1277_CREATE TYPE
INSERT INTO t2_compfoo select * from t1_compfoo ;

-- 来源: 1279_CREATE VIEW
INSERT INTO test_tb1 VALUES (generate_series(1,100),generate_series(1,100));

-- 来源: 1286_DELETE
INSERT INTO tpcds . customer_address VALUES ( 1 , 'AAAAAAAABAAAAAAA' , '18' , 'Jackson' ),( 10000 , 'AAAAAAAACAAAAAAA' , '362' , 'Washington 6th' ),( 15000 , 'AAAAAAAADAAAAAAA' , '585' , 'Dogwood Washington' );

-- 来源: 1328_EXECUTE
INSERT INTO tpcds . reason VALUES ( 51 , 'AAAAAAAADDAAAAAA' , 'reason 51' );

-- 来源: 1332_EXPLAIN
INSERT INTO tpcds . customer_address VALUES ( 5000 , 'AAAAAAAABAAAAAAA' ),( 10000 , 'AAAAAAAACAAAAAAA' );

-- 来源: 1335_FETCH
INSERT INTO tpcds . customer_address VALUES ( 1 , 'AAAAAAAABAAAAAAA' , '18' , 'Jackson' ),( 2 , 'AAAAAAAACAAAAAAA' , '362' , 'Washington 6th' ),( 3 , 'AAAAAAAADAAAAAAA' , '585' , 'Dogwood Washington' );

-- 来源: 1343_INSERT
INSERT INTO tpcds . reason ( r_reason_sk , r_reason_id , r_reason_desc ) VALUES ( 0 , 'AAAAAAAAAAAAAAAA' , 'reason0' );

-- 来源: 1343_INSERT
INSERT INTO tpcds . reason_t2 ( r_reason_sk , r_reason_id , r_reason_desc ) VALUES ( 1 , 'AAAAAAAABAAAAAAA' , 'reason1' );

-- 来源: 1343_INSERT
INSERT INTO tpcds . reason_t2 VALUES ( 2 , 'AAAAAAAABAAAAAAA' , 'reason2' );

-- 来源: 1343_INSERT
INSERT INTO tpcds . reason_t2 VALUES ( 3 , 'AAAAAAAACAAAAAAA' , 'reason3' ),( 4 , 'AAAAAAAADAAAAAAA' , 'reason4' ),( 5 , 'AAAAAAAAEAAAAAAA' , 'reason5' );

-- 来源: 1343_INSERT
INSERT INTO tpcds . reason_t2 SELECT * FROM tpcds . reason WHERE r_reason_sk < 5 ;

-- 来源: 1343_INSERT
INSERT INTO tpcds . reason_t2 VALUES ( 5 , 'BBBBBBBBCAAAAAAA' , 'reason5' ),( 6 , 'AAAAAAAADAAAAAAA' , 'reason6' ) ON DUPLICATE KEY UPDATE r_reason_id = 'BBBBBBBBCAAAAAAA' ;

-- 来源: 1343_INSERT
INSERT INTO tpcds . reason_t2 VALUES ( 5 , 'BBBBBBBBCAAAAAAA' , 'reason5' ) ON DUPLICATE KEY UPDATE r_reason_desc = 'reason5_new' RETURNING * ;

-- 来源: 1345_LOCK
INSERT INTO tpcds . reason VALUES ( 1 , 'AAAAAAAABAAAAAAA' , '18' ),( 5 , 'AAAAAAAACAAAAAAA' , '362' ),( 7 , 'AAAAAAAADAAAAAAA' , '585' );

-- 来源: 1349_MERGE INTO
INSERT INTO products VALUES ( 1501 , 'vivitar 35mm' , 'electrncs' );

-- 来源: 1349_MERGE INTO
INSERT INTO products VALUES ( 1502 , 'olympus is50' , 'electrncs' );

-- 来源: 1349_MERGE INTO
INSERT INTO products VALUES ( 1600 , 'play gym' , 'toys' );

-- 来源: 1349_MERGE INTO
INSERT INTO products VALUES ( 1601 , 'lamaze' , 'toys' );

-- 来源: 1349_MERGE INTO
INSERT INTO products VALUES ( 1666 , 'harry potter' , 'dvd' );

-- 来源: 1349_MERGE INTO
INSERT INTO newproducts VALUES ( 1502 , 'olympus camera' , 'electrncs' );

-- 来源: 1349_MERGE INTO
INSERT INTO newproducts VALUES ( 1601 , 'lamaze' , 'toys' );

-- 来源: 1349_MERGE INTO
INSERT INTO newproducts VALUES ( 1666 , 'harry potter' , 'toys' );

-- 来源: 1349_MERGE INTO
INSERT INTO newproducts VALUES ( 1700 , 'wait interface' , 'books' );

-- 来源: 1350_MOVE
INSERT INTO tpcds . reason VALUES ( 1 , 'AAAAAAAABAAAAAAA' , 'Xxxxxxxxx' ),( 2 , 'AAAAAAAACAAAAAAA' , 'Xxxxxxxxx' ),( 3 , 'AAAAAAAADAAAAAAA' , ' Xxxxxxxxx' ),( 4 , 'AAAAAAAAEAAAAAAA' , 'Not the product that was ordered' ),( 5 , 'AAAAAAAAFAAAAAAA' , 'Parts missing' ),( 6 , 'AAAAAAAAGAAAAAAA' , 'Does not work with a product that I have' ),( 7 , 'AAAAAAAAHAAAAAAA' , 'Gift exchange' );

--基表写入数据。
-- 来源: 1358_REFRESH INCREMENTAL MATERIALIZED VIEW
INSERT INTO my_table VALUES(1,1),(2,2);

--基表写入数据。
-- 来源: 1359_REFRESH MATERIALIZED VIEW
INSERT INTO my_table VALUES(1,1),(2,2);

-- 来源: 1360_REINDEX
INSERT INTO tpcds . customer VALUES ( 1 , 'AAAAAAAABAAAAAAA' ),( 5 , 'AAAAAAAACAAAAAAA' ),( 10 , 'AAAAAAAADAAAAAAA' );

-- 来源: 1360_REINDEX
INSERT INTO tpcds . customer_t1 SELECT * FROM tpcds . customer WHERE c_customer_sk < 10 ;

-- 来源: 1361_RELEASE SAVEPOINT
INSERT INTO tpcds . table1 VALUES ( 3 );

-- 来源: 1361_RELEASE SAVEPOINT
INSERT INTO tpcds . table1 VALUES ( 4 );

--插入数据。
-- 来源: 1362_REPLACE
INSERT INTO test VALUES(1, 1, 1), (2, 2, 2), (3, 3, 3);

-- 来源: 1369_SAVEPOINT
INSERT INTO table1 VALUES ( 1 );

-- 来源: 1369_SAVEPOINT
INSERT INTO table1 VALUES ( 2 );

-- 来源: 1369_SAVEPOINT
INSERT INTO table1 VALUES ( 3 );

-- 来源: 1369_SAVEPOINT
INSERT INTO table2 VALUES ( 3 );

-- 来源: 1369_SAVEPOINT
INSERT INTO table2 VALUES ( 4 );

-- 来源: 1371_SELECT
INSERT INTO tpcds . reason values ( 3 , 'AAAAAAAABAAAAAAA' , 'reason 1' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 2' ),( 4 , 'AAAAAAAABAAAAAAA' , 'reason 3' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 4' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 5' ),( 20 , 'AAAAAAAACAAAAAAA' , 'N%reason 6' ),( 30 , 'AAAAAAAACAAAAAAA' , 'W%reason 7' );

-- 来源: 1371_SELECT
INSERT INTO tpcds . reason_p values ( 3 , 'AAAAAAAABAAAAAAA' , 'reason 1' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 2' ),( 4 , 'AAAAAAAABAAAAAAA' , 'reason 3' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 4' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 5' ),( 20 , 'AAAAAAAACAAAAAAA' , 'reason 6' ),( 30 , 'AAAAAAAACAAAAAAA' , 'reason 7' );

-- 来源: 1371_SELECT
INSERT INTO skiplocked_astore VALUES (1, ' abc '), (2, ' bcd '), (3, ' cdf '),(3, ' dfg ' );

-- 来源: 1372_SELECT INTO
INSERT INTO tpcds . reason values ( 1 , 'AAAAAAAABAAAAAAA' , 'reason 1' ),( 2 , 'AAAAAAAABAAAAAAA' , 'reason 2' ),( 3 , 'AAAAAAAABAAAAAAA' , 'reason 3' ),( 4 , 'AAAAAAAABAAAAAAA' , 'reason 4' ),( 4 , 'AAAAAAAABAAAAAAA' , 'reason 5' ),( 4 , 'AAAAAAAACAAAAAAA' , 'reason 6' ),( 5 , 'AAAAAAAACAAAAAAA' , 'reason 7' );

--向表tpcds.reason_t2中插入记录。
-- 来源: 1382_TIMECAPSULE TABLE
INSERT INTO tpcds.reason_t2 VALUES (1, 'AA', 'reason1'),(2, 'AB', 'reason2'),(3, 'AC', 'reason3');

-- 来源: 1383_TRUNCATE
INSERT INTO tpcds . reason values ( 1 , 'AAAAAAAABAAAAAAA' , 'reason 1' ),( 5 , 'AAAAAAAABAAAAAAA' , 'reason 2' ),( 15 , 'AAAAAAAABAAAAAAA' , 'reason 3' ),( 25 , 'AAAAAAAABAAAAAAA' , 'reason 4' ),( 35 , 'AAAAAAAABAAAAAAA' , 'reason 5' ),( 45 , 'AAAAAAAACAAAAAAA' , 'reason 6' ),( 55 , 'AAAAAAAACAAAAAAA' , 'reason 7' );

-- 来源: 1383_TRUNCATE
INSERT INTO tpcds . reason_p SELECT * FROM tpcds . reason ;

-- 来源: 1385_UPDATE
INSERT INTO tbl_test1 VALUES (1, 'A'), (2, 'B');

--插入数据。
-- 来源: 1385_UPDATE
INSERT INTO test_grade VALUES (1,'Scott','A','2008-07-08',1),(2,'Ben','D','2008-07-08',1),(3,'Jack','D','2008-07-08',1);

-- 来源: 1387_VACUUM
INSERT INTO tpcds . reason values ( 1 , 'AAAAAAAABAAAAAAA' , 'reason 1' ),( 2 , 'AAAAAAAABAAAAAAA' , 'reason 2' );

-- 来源: 1425_record
insert into emp_rec values ( 111 , 'aaa' ), ( 222 , 'bbb' ), ( 333 , 'ccc' );

-- 来源: 1435_file_1435
INSERT INTO customers VALUES(1,'ab');

-- 来源: 1436_file_1436
INSERT INTO staffs VALUES ( 30 , 10 );

-- 来源: 1436_file_1436
INSERT INTO staffs VALUES ( 30 , 20 );

-- 来源: 1438_file_1438
INSERT INTO staffs VALUES ( 200 , 'mike' , 5800 );

-- 来源: 1438_file_1438
INSERT INTO staffs VALUES ( 201 , 'lily' , 3000 );

-- 来源: 1438_file_1438
INSERT INTO staffs VALUES ( 202 , 'john' , 4400 );

-- 来源: 1438_file_1438
INSERT INTO staffs VALUES ( 30 , 'mike' , '13567829252' , 5800 );

-- 来源: 1438_file_1438
INSERT INTO staffs VALUES ( 40 , 'john' , '17896354637' , 4000 );

-- 来源: 1441_file_1441
INSERT INTO staffs VALUES ( 200 , 'mike' , 5800 );

-- 来源: 1441_file_1441
INSERT INTO staffs VALUES ( 201 , 'lily' , 3000 );

-- 来源: 1441_file_1441
INSERT INTO staffs VALUES ( 202 , 'john' , 4400 );

-- 来源: 1445_RETURN NEXTRETURN QUERY
INSERT INTO t1 VALUES ( 1 ),( 10 );

-- 来源: 1447_file_1447
INSERT INTO TEST_t1 VALUES ( 8 , 'Donald' , 'OConnell' , 'DOCONNEL' , '650.507.9833' , to_date ( '21-06-1999' , 'dd-mm-yyyy' ), 'SH_CLERK' );

-- 来源: 1450_file_1450
INSERT INTO mytab ( firstname , lastname ) VALUES ( 'Tom' , 'Jones' );

-- 来源: 1452_file_1452
INSERT INTO EXAMPLE1 VALUES(3);

-- 来源: 1458_file_1458
insert into sections values ('hr',1,1);

-- 来源: 1458_file_1458
insert into staffs values (1,100,1,'Tom');

-- 来源: 1460_file_1460
INSERT INTO integerTable2 VALUES ( 2 );

-- 来源: 1468_DBE_COMPRESSION
INSERT INTO TEST_DATA VALUES ( 1 , '零食大礼包A' , NOW ());

-- 来源: 1468_DBE_COMPRESSION
INSERT INTO TEST_DATA VALUES ( 1 , '零食大礼包A' , NOW ());

-- 来源: 1489_file_1489
INSERT INTO sections VALUES(1);

-- 来源: 1489_file_1489
INSERT INTO sections VALUES(1);

-- 来源: 1489_file_1489
INSERT INTO sections VALUES(1);

-- 来源: 1489_file_1489
INSERT INTO sections VALUES(1);

-- 来源: 1489_file_1489
INSERT INTO test_lock VALUES (10,SYSDATE),(11,SYSDATE),(12,SYSDATE);

-- 来源: 1490_file_1490
INSERT INTO t2 VALUES(1,2);

-- 来源: 1491_file_1491
INSERT INTO t1 VALUES(1,'YOU WILL ROLLBACK!');

-- 来源: 1493_PACKAGE
INSERT INTO t2 VALUES(1,2);

-- 来源: 2366_file_2366
insert into test1 values ( 2 , '1.1' );

-- 来源: 2366_file_2366
insert into tab_2 values(' ');

-- 来源: 2366_file_2366
insert into tab_1 select col2 from tab_2;

-- 来源: 2366_file_2366
insert into tab_1 select col2 from tab_2;

-- 来源: 2366_file_2366
insert into test values('x');

-- 来源: 2431_file_2431
insert into a values(1,4);

-- 来源: 2431_file_2431
insert into a values(2,4);

-- 来源: 2431_file_2431
insert into a values(1,10);

-- 来源: 2431_file_2431
insert into a values(1, 100);

-- 来源: 2442_file_2442
INSERT INTO all_data VALUES ( 1 , 'alice' , 'alice data' );

-- 来源: 2442_file_2442
INSERT INTO all_data VALUES ( 2 , 'bob' , 'bob data' );

-- 来源: 2442_file_2442
INSERT INTO all_data VALUES ( 3 , 'peter' , 'peter data' );

-- 来源: 2455_file_2455
INSERT INTO table1 VALUES ( 1 , reverse ( '123ＡＡ78' ), reverse ( '123ＡＡ78' ), reverse ( '123ＡＡ78' ));

-- 来源: 2455_file_2455
INSERT INTO table1 VALUES ( 2 , reverse ( '123Ａ78' ), reverse ( '123Ａ78' ), reverse ( '123Ａ78' ));

-- 来源: 2455_file_2455
INSERT INTO table1 VALUES ( 3 , '87Ａ123' , '87Ａ123' , '87Ａ123' );

-- 来源: 2455_file_2455
INSERT INTO table2 VALUES ( 1 , reverse ( '123ＡＡ78' ), reverse ( '123ＡＡ78' ), reverse ( '123ＡＡ78' ));

-- 来源: 2455_file_2455
INSERT INTO table2 VALUES ( 2 , reverse ( '123Ａ78' ), reverse ( '123Ａ78' ), reverse ( '123Ａ78' ));

-- 来源: 2455_file_2455
INSERT INTO customer_t1 ( c_customer_sk , c_customer_id , c_first_name ) VALUES ( 3769 , 'hello' , 'Grace' );

-- 来源: 2455_file_2455
INSERT INTO customer_t1 VALUES ( 3769 , 'hello' , 'Grace' );

-- 来源: 2455_file_2455
INSERT INTO customer_t1 ( c_customer_sk , c_first_name ) VALUES ( 3769 , 'Grace' );

-- 来源: 2455_file_2455
INSERT INTO customer_t1 VALUES ( 3769 , 'hello' );

-- 来源: 2455_file_2455
INSERT INTO customer_t1 ( c_customer_sk , c_customer_id , c_first_name ) VALUES ( 3769 , 'hello' , DEFAULT );

-- 来源: 2455_file_2455
INSERT INTO customer_t1 DEFAULT VALUES ;

-- 来源: 2455_file_2455
INSERT INTO customer_t1 ( c_customer_sk , c_customer_id , c_first_name ) VALUES ( 6885 , 'maps' , 'Joes' ), ( 4321 , 'tpcds' , 'Lily' ), ( 9527 , 'world' , 'James' );

-- 来源: 2455_file_2455
INSERT INTO customer_t2 SELECT * FROM customer_t1 ;

-- 来源: 2462_file_2462
INSERT INTO tpcds . web_returns_p2 SELECT * FROM tpcds . customer_address ;

-- 来源: 2462_file_2462
INSERT INTO tpcds . web_returns_p2 SELECT * FROM tpcds . customer_address ;

-- 来源: 2731_SQL PATCH
insert into hint_t1 values ( 1 , 1 , 1 );

-- 来源: 2731_SQL PATCH
insert into test_proc_patch values(1,2);

-- 来源: 2743_file_2743
INSERT INTO int_type_t1 VALUES ( 10 , 20 );

-- 来源: 2743_file_2743
INSERT INTO int_type_t2 VALUES ( 100 , 10 , 1000 , 10000 , 200 , 2000 );

--插入数据。
-- 来源: 2743_file_2743
INSERT INTO decimal_type_t1 VALUES(123456.122331);

-- 来源: 2743_file_2743
INSERT INTO numeric_type_t1 VALUES ( 123456 . 12354 );

-- 来源: 2743_file_2743
INSERT INTO smallserial_type_tab VALUES ( default );

-- 来源: 2743_file_2743
INSERT INTO smallserial_type_tab VALUES ( default );

-- 来源: 2743_file_2743
INSERT INTO serial_type_tab VALUES ( default );

-- 来源: 2743_file_2743
INSERT INTO serial_type_tab VALUES ( default );

-- 来源: 2743_file_2743
INSERT INTO bigserial_type_tab VALUES ( default );

-- 来源: 2743_file_2743
INSERT INTO bigserial_type_tab VALUES ( default );

-- 来源: 2743_file_2743
INSERT INTO largeserial_type_tab VALUES ( default );

-- 来源: 2743_file_2743
INSERT INTO largeserial_type_tab VALUES ( default );

-- 来源: 2743_file_2743
INSERT INTO float_type_t2 VALUES ( 10 , 10 . 365456 , 123456 . 1234 , 10 . 3214 , 321 . 321 , 123 . 123654 , 123 . 123654 );

-- 来源: 2745_file_2745
INSERT INTO bool_type_t1 VALUES ( TRUE , 'sic est' );

-- 来源: 2745_file_2745
INSERT INTO bool_type_t1 VALUES ( FALSE , 'non est' );

-- varchar为1073741728，超过规定长度，插入失败
-- 来源: 2746_file_2746
insert into varchar_maxlength_test1 values(1, repeat('a', 1073741728), 1);

-- varchar为1073741727，长度符合要求，插入成功
-- 来源: 2746_file_2746
insert into varchar_maxlength_test1 values(1, repeat('a', 1073741727), 1);

--插入数据。
-- 来源: 2746_file_2746
INSERT INTO char_type_t1 VALUES ('ok');

--插入数据。
-- 来源: 2746_file_2746
INSERT INTO char_type_t2 VALUES ('ok');

-- 来源: 2746_file_2746
INSERT INTO char_type_t2 VALUES ('good');

--插入的数据长度超过类型规定的长度报错。
-- 来源: 2746_file_2746
INSERT INTO char_type_t2 VALUES ('too long');

--明确类型的长度，超过数据类型长度后会自动截断。
-- 来源: 2746_file_2746
INSERT INTO char_type_t2 VALUES ('too long'::varchar(5));

-- 来源: 2747_file_2747
INSERT INTO blob_type_t1 VALUES ( 10 , empty_blob (), HEXTORAW ( 'DEADBEEF' ), E '\\xDEADBEEF' );

-- 来源: 2748__
INSERT INTO date_type_tab VALUES ( date '12-10-2010' );

-- 来源: 2748__
INSERT INTO time_type_tab VALUES ( '21:21:21' , '21:21:21 pst' , '2010-12-12' , '2013-12-11 pst' , '2003-04-12 04:05:06' );

-- 来源: 2748__
INSERT INTO day_type_tab VALUES ( 1 , INTERVAL '3' DAY );

-- 来源: 2748__
INSERT INTO year_type_tab VALUES ( 1 , interval '2' year );

-- 来源: 2748__
INSERT INTO date_type_tab VALUES ( date '12-10-2010' );

-- 来源: 2748__
INSERT INTO date_type_tab VALUES ( date '2010-12-11' );

--插入数据。
-- 来源: 2748__
INSERT INTO realtime_type_special VALUES('epoch', 'epoch', 'epoch', NULL);

-- 来源: 2748__
INSERT INTO realtime_type_special VALUES('now', 'now', 'now', 'now');

-- 来源: 2748__
INSERT INTO realtime_type_special VALUES('today', 'today', 'today', NULL);

-- 来源: 2748__
INSERT INTO realtime_type_special VALUES('tomorrow', 'tomorrow', 'tomorrow', NULL);

-- 来源: 2748__
INSERT INTO realtime_type_special VALUES('yesterday', 'yesterday', 'yesterday', NULL);

-- 来源: 2748__
INSERT INTO reltime_type_tab VALUES ( '90' , '90' );

-- 来源: 2748__
INSERT INTO reltime_type_tab VALUES ( '-366' , '-366' );

-- 来源: 2748__
INSERT INTO reltime_type_tab VALUES ( '1975.25' , '1975.25' );

-- 来源: 2748__
INSERT INTO reltime_type_tab VALUES ( '-2 YEARS +5 MONTHS 10 DAYS' , '-2 YEARS +5 MONTHS 10 DAYS' );

-- 来源: 2748__
INSERT INTO reltime_type_tab VALUES ( '30 DAYS 12:00:00' , '30 DAYS 12:00:00' );

-- 来源: 2748__
INSERT INTO reltime_type_tab VALUES ( 'P-1.1Y10M' , 'P-1.1Y10M' );

-- 来源: 2751_file_2751
INSERT INTO bit_type_t1 VALUES ( 1 , B '101' , B '00' );

-- 来源: 2751_file_2751
INSERT INTO bit_type_t1 VALUES ( 2 , B '10' , B '101' );

-- 来源: 2751_file_2751
INSERT INTO bit_type_t1 VALUES ( 2 , B '10' :: bit ( 3 ), B '101' );

-- 来源: 2755_HLL
INSERT INTO t1 VALUES ( 1 , hll_empty ( 14 , - 1 ));

-- 来源: 2755_HLL
INSERT INTO t1 ( id , set ) VALUES ( 1 , hll_empty ( 14 , 5 ));

-- 来源: 2755_HLL
INSERT INTO helloworld ( id , set ) VALUES ( 1 , hll_empty ());

-- 来源: 2755_HLL
INSERT INTO facts VALUES ( '2019-02-20' , generate_series ( 1 , 100 ));

-- 来源: 2755_HLL
INSERT INTO facts VALUES ( '2019-02-21' , generate_series ( 1 , 200 ));

-- 来源: 2755_HLL
INSERT INTO facts VALUES ( '2019-02-22' , generate_series ( 1 , 300 ));

-- 来源: 2755_HLL
INSERT INTO facts VALUES ( '2019-02-23' , generate_series ( 1 , 400 ));

-- 来源: 2755_HLL
INSERT INTO facts VALUES ( '2019-02-24' , generate_series ( 1 , 500 ));

-- 来源: 2755_HLL
INSERT INTO facts VALUES ( '2019-02-25' , generate_series ( 1 , 600 ));

-- 来源: 2755_HLL
INSERT INTO facts values ( '2019-02-26' , generate_series ( 1 , 700 ));

-- 来源: 2755_HLL
INSERT INTO facts VALUES ( '2019-02-27' , generate_series ( 1 , 800 ));

-- 来源: 2755_HLL
INSERT INTO daily_uniques ( date , users ) SELECT date , hll_add_agg ( hll_hash_integer ( user_id )) FROM facts GROUP BY 1 ;

-- 来源: 2755_HLL
INSERT INTO test VALUES ( 1 , 'E\\1234' );

-- 来源: 2756_file_2756
INSERT INTO reservation VALUES (1108, '[2010-01-01 14:30, 2010-01-01 15:30)');

-- 来源: 2758_file_2758
insert into t1 values ( 1 ),( 2 );

-- 来源: 2760_XML
INSERT INTO xmltest VALUES (1, 'one');

-- 来源: 2760_XML
INSERT INTO xmltest VALUES (2, 'two');

-- 来源: 2761_XMLTYPE
INSERT INTO xmltypetest VALUES (1, '<ss/>');

-- 来源: 2761_XMLTYPE
INSERT INTO xmltypetest VALUES (2, '<xx/>');

-- 来源: 2763_SET
INSERT INTO employee values('zhangsan', 'nanjing,beijing');

-- 来源: 2763_SET
INSERT INTO employee VALUES ('zhangsan', 'hangzhou');

-- 来源: 2763_SET
INSERT INTO employee values('lisi', 9);

-- 来源: 2764_aclitem
INSERT INTO table_acl VALUES (1,'user1=arw/omm','{omm=d/user2,omm=w/omm}');

-- 来源: 2764_aclitem
INSERT INTO table_acl VALUES (2,'user1=aw/omm','{omm=d/user2}');

-- 来源: 2769_file_2769
INSERT INTO test_space values ( 'a' );

-- 来源: 2769_file_2769
insert into test ( a ) values ( 'abC 不' );

-- 来源: 2769_file_2769
insert into test ( a ) values ( 'abC 啊' );

-- 来源: 2769_file_2769
insert into test ( a ) values ( 'abc 啊' );

-- 来源: 2775_file_2775
insert into json_doc values ( '{"name":"a"}' );

-- 来源: 2779_JSON_JSONB
INSERT INTO classes VALUES('A',2);

-- 来源: 2779_JSON_JSONB
INSERT INTO classes VALUES('A',3);

-- 来源: 2779_JSON_JSONB
INSERT INTO classes VALUES('D',5);

-- 来源: 2779_JSON_JSONB
INSERT INTO classes VALUES('D',null);

-- 来源: 2779_JSON_JSONB
INSERT INTO classes VALUES('A',2);

-- 来源: 2779_JSON_JSONB
INSERT INTO classes VALUES('A',3);

-- 来源: 2779_JSON_JSONB
INSERT INTO classes VALUES('D',5);

-- 来源: 2779_JSON_JSONB
INSERT INTO classes VALUES('D',null);

-- 来源: 2780_HLL
INSERT INTO t_id VALUES ( generate_series ( 1 , 500 ));

-- 来源: 2780_HLL
INSERT INTO t_data SELECT mod ( id , 2 ), id FROM t_id ;

-- 来源: 2780_HLL
INSERT INTO t_a_c_hll SELECT a , hll_add_agg ( hll_hash_text ( c )) FROM t_data GROUP BY a ;

-- 来源: 2784_file_2784
INSERT INTO tab values ( 1 );

-- 来源: 2784_file_2784
INSERT INTO tab values ( 2 );

-- 来源: 2789_file_2789
INSERT INTO blob_tb VALUES ( empty_blob (), 1 );

-- 来源: 2789_file_2789
INSERT INTO clob_tb VALUES ( empty_clob (), 1 );

-- 来源: 2789_file_2789
INSERT INTO student_demo VALUES ( 'name0' , 0 );

-- 来源: 2789_file_2789
INSERT INTO student_demo VALUES ( 'name1' , 1 );

-- 来源: 2789_file_2789
INSERT INTO student_demo VALUES ( 'name2' , 2 );

-- 来源: 2790_file_2790
INSERT INTO sales VALUES(1, 12, '2019-02-05 00:00:00', 'a', 1, 1, 1);

-- 来源: 2801_file_2801
INSERT INTO t1 VALUES(1,1);

-- 来源: 2801_file_2801
INSERT INTO t2(a) VALUES(1);

-- 来源: 2804_file_2804
INSERT INTO test values(1,1);

-- 来源: 2808_file_2808
INSERT INTO part_tab1 VALUES(1, 1);

-- 来源: 2808_file_2808
INSERT INTO part_tab1 VALUES(1, 11);

-- 来源: 2808_file_2808
INSERT INTO part_tab1 VALUES(1, 21);

-- 来源: 2808_file_2808
INSERT INTO subpart_tab1 VALUES('201902', '1', '1', 1);

-- 来源: 2808_file_2808
INSERT INTO subpart_tab1 VALUES('201902', '2', '2', 1);

-- 来源: 2808_file_2808
INSERT INTO subpart_tab1 VALUES('201903', '1', '3', 1);

-- 来源: 2808_file_2808
INSERT INTO subpart_tab1 VALUES('201903', '2', '4', 1);

-- 来源: 2808_file_2808
INSERT INTO part_tab1 VALUES(1, 1);

-- 来源: 2808_file_2808
INSERT INTO part_tab1 VALUES(1, 11);

-- 来源: 2808_file_2808
INSERT INTO part_tab1 VALUES(1, 21);

-- 来源: 2808_file_2808
INSERT INTO subpart_tab1 VALUES('201902', '1', '1', 1);

-- 来源: 2808_file_2808
INSERT INTO subpart_tab1 VALUES('201902', '2', '2', 1);

-- 来源: 2808_file_2808
INSERT INTO subpart_tab1 VALUES('201903', '1', '3', 1);

-- 来源: 2808_file_2808
INSERT INTO subpart_tab1 VALUES('201903', '2', '4', 1);

-- 来源: 2821_XML
INSERT INTO xmltest VALUES ( 1 , '<value>one</value>' );

-- 来源: 2821_XML
INSERT INTO xmltest VALUES ( 2 , '<value>two</value>' );

-- 来源: 2821_XML
INSERT INTO xmltest VALUES ( 1 , '<?xml version="1.0" encoding="GBK"?><value>one</value>' );

-- 来源: 2821_XML
INSERT INTO xmltest VALUES ( 2 , '<?xml version="1.0" encoding="GBK"?><value>two</value>' );

-- 来源: 2821_XML
INSERT INTO testxmlschema . test1 VALUES ( 1 , 'one' ), ( 2 , 'two' ), ( - 1 , null );

-- 来源: 2830_file_2830
INSERT INTO tpcds . case_when_t1 VALUES ( 1 ), ( 2 ), ( 3 );

-- 来源: 2830_file_2830
INSERT INTO tpcds . c_tabl VALUES ( 'abc' , 'efg' , '123' );

-- 来源: 2830_file_2830
INSERT INTO tpcds . c_tabl VALUES ( NULL , 'efg' , '123' );

-- 来源: 2830_file_2830
INSERT INTO tpcds . c_tabl VALUES ( NULL , NULL , '123' );

-- 来源: 2830_file_2830
INSERT INTO tpcds . null_if_t1 VALUES ( 'abc' , 'abc' );

-- 来源: 2830_file_2830
INSERT INTO tpcds . null_if_t1 VALUES ( 'abc' , 'efg' );

-- 来源: 2835_file_2835
INSERT INTO Students VALUES ( 'Jack' , 35 );

-- 来源: 2835_file_2835
INSERT INTO Students VALUES ( 'Leon' , 15 );

-- 来源: 2835_file_2835
INSERT INTO Students VALUES ( 'James' , 24 );

-- 来源: 2835_file_2835
INSERT INTO Students VALUES ( 'Taker' , 81 );

-- 来源: 2835_file_2835
INSERT INTO Students VALUES ( 'Mary' , 25 );

-- 来源: 2835_file_2835
INSERT INTO Students VALUES ( 'Rose' , 64 );

-- 来源: 2835_file_2835
INSERT INTO Students VALUES ( 'Perl' , 18 );

-- 来源: 2835_file_2835
INSERT INTO Students VALUES ( 'Under' , 57 );

-- 来源: 2835_file_2835
INSERT INTO Students VALUES ( 'Angel' , 101 );

-- 来源: 2835_file_2835
INSERT INTO Students VALUES ( 'Frank' , 20 );

-- 来源: 2835_file_2835
INSERT INTO Students VALUES ( 'Charlie' , 40 );

-- 来源: 2835_file_2835
insert into test select generate_series , generate_series from generate_series ( 1 , 10 );

-- 来源: 2840_file_2840
INSERT INTO tpcds . value_storage_t1 VALUES ( 'abcdef' );

-- 来源: 2849_file_2849
INSERT INTO tsearch . pgweb VALUES ( 1 , 'China, officially the People''s Republic of China (PRC), located in Asia, is the world''s most populous state.' , 'China' , '2010-1-1' );

-- 来源: 2849_file_2849
INSERT INTO tsearch . pgweb VALUES ( 2 , 'America is a rock band, formed in England in 1970 by multi-instrumentalists Dewey Bunnell, Dan Peek, and Gerry Beckley.' , 'America' , '2010-1-1' );

-- 来源: 2849_file_2849
INSERT INTO tsearch . pgweb VALUES ( 3 , 'England is a country that is part of the United Kingdom. It shares land borders with Scotland to the north and Wales to the west.' , 'England' , '2010-1-1' );

-- 来源: 2849_file_2849
INSERT INTO tsearch . pgweb VALUES ( 4 , 'Australia, officially the Commonwealth of Australia, is a country comprising the mainland of the Australian continent, the island of Tasmania, and numerous smaller islands.' , 'Australia' , '2010-1-1' );

-- 来源: 2849_file_2849
INSERT INTO tsearch . pgweb VALUES ( 6 , 'Japan is an island country in East Asia.' , 'Japan' , '2010-1-1' );

-- 来源: 2849_file_2849
INSERT INTO tsearch . pgweb VALUES ( 7 , 'Germany, officially the Federal Republic of Germany, is a sovereign state and federal parliamentary republic in central-western Europe.' , 'Germany' , '2010-1-1' );

-- 来源: 2849_file_2849
INSERT INTO tsearch . pgweb VALUES ( 8 , 'France, is a sovereign state comprising territory in western Europe and several overseas regions and territories.' , 'France' , '2010-1-1' );

-- 来源: 2849_file_2849
INSERT INTO tsearch . pgweb VALUES ( 9 , 'Italy officially the Italian Republic, is a unitary parliamentary republic in Europe.' , 'Italy' , '2010-1-1' );

-- 来源: 2849_file_2849
INSERT INTO tsearch . pgweb VALUES ( 10 , 'India, officially the Republic of India, is a country in South Asia.' , 'India' , '2010-1-1' );

-- 来源: 2849_file_2849
INSERT INTO tsearch . pgweb VALUES ( 11 , 'Brazil, officially the Federative Republic of Brazil, is the largest country in both South America and Latin America.' , 'Brazil' , '2010-1-1' );

-- 来源: 2849_file_2849
INSERT INTO tsearch . pgweb VALUES ( 12 , 'Canada is a country in the northern half of North America.' , 'Canada' , '2010-1-1' );

-- 来源: 2849_file_2849
INSERT INTO tsearch . pgweb VALUES ( 13 , 'Mexico, officially the United Mexican States, is a federal republic in the southern part of North America.' , 'Mexico' , '2010-1-1' );

-- 来源: 2853_file_2853
INSERT INTO tsearch . tt ( id , title , keyword , abstract , body ) VALUES ( 1 , 'China' , 'Beijing' , 'China' , 'China, officially the People''s Republic of China (PRC), located in Asia, is the world''s most populous state.' );

-- 来源: 2855_file_2855
INSERT INTO tsearch . ts_ngram VALUES ( 1 , '中文' );

-- 来源: 2855_file_2855
INSERT INTO tsearch . ts_ngram VALUES ( 2 , '中文检索' );

-- 来源: 2855_file_2855
INSERT INTO tsearch . ts_ngram VALUES ( 3 , '检索中文' );

-- 来源: 2860_file_2860
INSERT INTO tsearch . aliases VALUES ( 1 , to_tsquery ( 'supernovae' ), to_tsquery ( 'supernovae|sn' ));

--插入记录。
-- 来源: 2882_ABORT
INSERT INTO customer_demographics_t1 VALUES(1920801,'M', 'U', 'DOCTOR DEGREE', 200, 'GOOD', 1, 0,0);

-- 来源: 2935_CLUSTER
INSERT INTO test_c1 VALUES ( 3 , 'Joe' ),( 1 , 'Jack' ),( 2 , 'Scott' );

-- 来源: 2935_CLUSTER
INSERT INTO test_c2 VALUES (6,'ABBB'),(2,'ABAB'),(9,'AAAA');

-- 来源: 2935_CLUSTER
INSERT INTO test_c2 VALUES (11,'AAAB'),(19,'BBBA'),(16,'BABA');

--插入数据。
-- 来源: 2937_COMMIT _ END
INSERT INTO tpcds. customer_demographics_t2 VALUES(1,'M', 'U', 'DOCTOR DEGREE', 1200, 'GOOD', 1, 0, 0);

-- 来源: 2937_COMMIT _ END
INSERT INTO tpcds. customer_demographics_t2 VALUES(2,'F', 'U', 'MASTER DEGREE', 300, 'BAD', 1, 0, 0);

--向 tpcds. ship_mode表插入一条数据。
-- 来源: 2939_COPY
INSERT INTO tpcds. ship_mode VALUES (1,'a','b','c','d','e');

-- 来源: 2940_CREATE AGGREGATE
INSERT INTO test_sum VALUES(1,2),(2,3),(3,4),(4,5);

--基表写入数据。
-- 来源: 2954_CREATE INCREMENTAL MATERIALIZED VIEW
INSERT INTO my_table VALUES(1,1),(2,2);

-- 来源: 2957_CREATE MASKING POLICY
INSERT INTO tb_for_masking VALUES ( 1 , '9876543210' , 'usr321usr' , 'abc@huawei.com' , 'abc@huawei.com' , '1234-4567-7890-0123' , 'abcdef 123456 ui 323 jsfd321 j3k2l3' , '4880-9898-4545-2525' , 'this is a llt case' );

-- 来源: 2957_CREATE MASKING POLICY
INSERT INTO tb_for_masking VALUES ( 2 , '0123456789' , 'lltc123llt' , 'abc@gmail.com' , 'abc@gmail.com' , '9876-5432-1012-3456' , '1234 abcd ef 56 gh78ijk90lm' , '4856-7654-1234-9865' , 'this,is.a!LLT?case' );

--基表写入数据。
-- 来源: 2958_CREATE MATERIALIZED VIEW
INSERT INTO my_table VALUES(1,1),(2,2);

--插入训练数据
-- 来源: 2959_CREATE MODEL
INSERT INTO houses(id, tax, bedroom, bath, price, size, lot, mark) VALUES (1,590,2,1,50000,770,22100,'a+'), (2,1050,3,2,85000,1410,12000,'a+'), (3,20,2,1,22500,1060,3500,'a-'), (4,870,2,2,90000,1300,17500,'a+'), (5,1320,3,2,133000,1500,30000,'a+'), (6,1350,2,1,90500,850,25700,'a-'), (7,2790,3,2.5,260000,2130,25000,'a+'), (8,680,2,1,142500,1170,22000,'a-'), (9,1840,3,2,160000,1500,19000,'a+'), (10,3680,4,2,240000,2790,20000,'a-'), (11,1660,3,1,87000,1030,17500,'a+'), (12,1620,3,2,118500,1250,20000,'a-'), (13,3100,3,2,140000,1760,38000,'a+'), (14,2090,2,3,148000,1550,14000,'a-'), (15,650,3,1.5,65000,1450,12000,'a-');

--向数据表插入数据。
-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
INSERT INTO all_data VALUES(1, 'alice', 'alice data');

-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
INSERT INTO all_data VALUES(2, 'bob', 'bob data');

-- 来源: 2968_CREATE ROW LEVEL SECURITY POLICY
INSERT INTO all_data VALUES(3, 'peter', 'peter data');

-- 来源: 2972_CREATE SEQUENCE
INSERT INTO test1 ( name ) values ( 'Joe' ),( 'Scott' ),( 'Ben' );

-- 来源: 2975_CREATE SYNONYM
INSERT INTO t1 VALUES (1, 'ada'), (2, 'bob');

-- 来源: 2977_CREATE TABLE AS
INSERT INTO test1 VALUES (1,'col1'),(101,'col101');

--从示例数据表导入数据。
-- 来源: 2978_CREATE TABLE PARTITION
INSERT INTO tpcds. web_returns_p1 SELECT * FROM tpcds. web_returns;

-- 导入数据，查看分区数据量
-- 来源: 2978_CREATE TABLE PARTITION
INSERT INTO tpcds.startend_pt VALUES (GENERATE_SERIES(0, 4999), GENERATE_SERIES(0, 4999));

-- 来源: 2978_CREATE TABLE PARTITION
INSERT INTO sales VALUES ( 1 , 12 , '2019-01-10 00:00:00' , 'a' , 1 , 1 , 1 );

-- 来源: 2978_CREATE TABLE PARTITION
INSERT INTO sales VALUES ( 1 , 12 , '2019-02-01 00:00:00' , 'a' , 1 , 1 , 1 );

-- 来源: 2978_CREATE TABLE PARTITION
INSERT INTO sales VALUES ( 1 , 12 , '2019-02-05 00:00:00' , 'a' , 1 , 1 , 1 );

-- 来源: 2978_CREATE TABLE PARTITION
INSERT INTO sales VALUES ( 1 , 12 , '2019-02-03 00:00:00' , 'a' , 1 , 1 , 1 );

-- 来源: 2978_CREATE TABLE PARTITION
INSERT INTO test_list VALUES ( 2000 , 2000 );

-- 来源: 2978_CREATE TABLE PARTITION
INSERT INTO test_list VALUES ( 3000 , 3000 );

-- 来源: 2978_CREATE TABLE PARTITION
INSERT INTO test_list VALUES ( 6000 , 6000 );

-- 来源: 2978_CREATE TABLE PARTITION
INSERT INTO test_list VALUES ( 6000 , 6000 );

-- 来源: 2978_CREATE TABLE PARTITION
INSERT INTO test_list VALUES ( 6000 , 6000 );

-- 来源: 2978_CREATE TABLE PARTITION
INSERT INTO test_hash VALUES ( 1 , 1 );

-- 来源: 2978_CREATE TABLE PARTITION
INSERT INTO test_hash VALUES ( 2 , 2 );

-- 来源: 2978_CREATE TABLE PARTITION
INSERT INTO test_hash VALUES ( 3 , 3 );

-- 来源: 2978_CREATE TABLE PARTITION
INSERT INTO test_hash VALUES ( 4 , 4 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201902' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201903' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_hash VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_hash VALUES ( '201902' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_hash VALUES ( '201902' , '3' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_hash VALUES ( '201903' , '4' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_hash VALUES ( '201903' , '5' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_hash VALUES ( '201903' , '6' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_range VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_range VALUES ( '201902' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_range VALUES ( '201902' , '3' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_range VALUES ( '201903' , '4' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_range VALUES ( '201903' , '5' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_range VALUES ( '201903' , '6' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list VALUES ( '201902' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list VALUES ( '201903' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_hash VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_hash VALUES ( '201902' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_hash VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_hash VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_hash VALUES ( '201903' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_hash VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_range VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_range VALUES ( '201902' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_range VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_range VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_range VALUES ( '201903' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_range VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_list VALUES ( '201901' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_list VALUES ( '201901' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_list VALUES ( '201901' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_list VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_list VALUES ( '201903' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_list VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_hash VALUES ( '201901' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_hash VALUES ( '201901' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_hash VALUES ( '201901' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_hash VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_hash VALUES ( '201903' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_hash VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_range VALUES ( '201901' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_range VALUES ( '201901' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_range VALUES ( '201901' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_range VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_range VALUES ( '201903' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO hash_range VALUES ( '201903' , '2' , '1' , 1 );

--指定一级分区插入数据
-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list partition (p_201901) VALUES('201902', '1', '1', 1);

--实际分区和指定分区不一致，报错
-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list partition (p_201902) VALUES('201902', '1', '1', 1);

--指定二级分区插入数据
-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list subpartition (p_201901_a) VALUES('201902', '1', '1', 1);

--实际分区和指定分区不一致，报错
-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list subpartition (p_201901_b) VALUES('201902', '1', '1', 1);

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list partition for ('201902') VALUES('201902', '1', '1', 1);

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list subpartition for ('201902','1') VALUES('201902', '1', '1', 1);

--指定分区insert数据
-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list partition (p_201901) VALUES('201902', '1', '1', 1) ON DUPLICATE KEY UPDATE sales_amt = 5;

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list subpartition (p_201901_a) VALUES('201902', '1', '1', 1) ON DUPLICATE KEY UPDATE sales_amt = 10;

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list partition for ('201902') VALUES('201902', '1', '1', 1) ON DUPLICATE KEY UPDATE sales_amt = 30;

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO range_list subpartition for ('201902','1') VALUES('201902', '1', '1', 1) ON DUPLICATE KEY UPDATE sales_amt = 40;

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO newrange_list VALUES('201902', '1', '1', 1);

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO newrange_list VALUES('201903', '1', '1', 2);

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201902' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201903' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201902' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201903' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201902' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201903' , '1' , '1' , 1 );

-- 来源: 2980_CREATE TABLE SUBPARTITION
INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

--执行INSERT触发事件并检查触发结果
-- 来源: 2983_CREATE TRIGGER
INSERT INTO test_trigger_src_tbl VALUES(100,200,300);

-- 来源: 2984_CREATE TYPE
INSERT INTO t1_compfoo values(1,(1,'demo'));

-- 来源: 2984_CREATE TYPE
INSERT INTO t2_compfoo select * from t1_compfoo;

-- 来源: 2987_CREATE VIEW
INSERT INTO test_tb1 VALUES (generate_series(1,100),generate_series(1,100));

--向表中插入多条记录。
-- 来源: 2993_DELETE
INSERT INTO tpcds.customer_address VALUES (1, 'AAAAAAAABAAAAAAA', '18', 'Jackson'),(10000, 'AAAAAAAACAAAAAAA', '362', 'Washington 6th'),(15000, 'AAAAAAAADAAAAAAA', '585', 'Dogwood Washington');

--插入数据。
-- 来源: 3040_EXECUTE
INSERT INTO tpcds. reason VALUES(51, 'AAAAAAAADDAAAAAA', 'reason 51');

--向表中插入多条记录。
-- 来源: 3043_EXPLAIN
INSERT INTO tpcds.customer_address VALUES (5000, 'AAAAAAAABAAAAAAA'),(10000, 'AAAAAAAACAAAAAAA');

-- 来源: 3043_EXPLAIN
INSERT INTO tb_a VALUES(1),(2),(3);

--向表中插入多条记录。
-- 来源: 3046_FETCH
INSERT INTO tpcds.customer_address VALUES (1, 'AAAAAAAABAAAAAAA', '18', 'Jackson'),(2, 'AAAAAAAACAAAAAAA', '362', 'Washington 6th'),(3, 'AAAAAAAADAAAAAAA', '585', 'Dogwood Washington');

--向表中插入一条记录。
-- 来源: 3054_INSERT
INSERT INTO tpcds.reason(r_reason_sk, r_reason_id, r_reason_desc) VALUES (0, 'AAAAAAAAAAAAAAAA', 'reason0');

--向表中插入一条记录。
-- 来源: 3054_INSERT
INSERT INTO tpcds. reason_t2(r_reason_sk, r_reason_id, r_reason_desc) VALUES (1, 'AAAAAAAABAAAAAAA', 'reason1');

--向表中插入一条记录，和上一条语法等效。
-- 来源: 3054_INSERT
INSERT INTO tpcds. reason_t2 VALUES (2, 'AAAAAAAABAAAAAAA', 'reason2');

--向表中插入多条记录。
-- 来源: 3054_INSERT
INSERT INTO tpcds. reason_t2 VALUES (3, 'AAAAAAAACAAAAAAA','reason3'),(4, 'AAAAAAAADAAAAAAA', 'reason4'),(5, 'AAAAAAAAEAAAAAAA','reason5');

--向表中插入 tpcds. reason中r_reason_sk小于5的记录。
-- 来源: 3054_INSERT
INSERT INTO tpcds. reason_t2 SELECT * FROM tpcds. reason WHERE r_reason_sk <5;

--向表中插入多条记录，如果冲突则更新冲突数据行中r_reason_id字段为'BBBBBBBBCAAAAAAA'。
-- 来源: 3054_INSERT
INSERT INTO tpcds. reason_t2 VALUES (5, 'BBBBBBBBCAAAAAAA','reason5'),(6, 'AAAAAAAADAAAAAAA', 'reason6') ON DUPLICATE KEY UPDATE r_reason_id = 'BBBBBBBBCAAAAAAA';

--更新已有记录并返回
-- 来源: 3054_INSERT
INSERT INTO tpcds. reason_t2 VALUES ( 5, 'BBBBBBBBCAAAAAAA','reason5') ON DUPLICATE KEY UPDATE r_reason_desc='reason5_new' RETURNING *;

--向表中插入多条记录。
-- 来源: 3056_LOCK
INSERT INTO tpcds.reason VALUES (1, 'AAAAAAAABAAAAAAA', '18'),(5, 'AAAAAAAACAAAAAAA', '362'),(7, 'AAAAAAAADAAAAAAA', '585');

-- 来源: 3060_MERGE INTO
INSERT INTO products VALUES (1501, 'vivitar 35mm', 'electrncs');

-- 来源: 3060_MERGE INTO
INSERT INTO products VALUES (1502, 'olympus is50', 'electrncs');

-- 来源: 3060_MERGE INTO
INSERT INTO products VALUES (1600, 'play gym', 'toys');

-- 来源: 3060_MERGE INTO
INSERT INTO products VALUES (1601, 'lamaze', 'toys');

-- 来源: 3060_MERGE INTO
INSERT INTO products VALUES (1666, 'harry potter', 'dvd');

-- 来源: 3060_MERGE INTO
INSERT INTO newproducts VALUES (1502, 'olympus camera', 'electrncs');

-- 来源: 3060_MERGE INTO
INSERT INTO newproducts VALUES (1601, 'lamaze', 'toys');

-- 来源: 3060_MERGE INTO
INSERT INTO newproducts VALUES (1666, 'harry potter', 'toys');

-- 来源: 3060_MERGE INTO
INSERT INTO newproducts VALUES (1700, 'wait interface', 'books');

--向表中插入多条记录。
-- 来源: 3061_MOVE
INSERT INTO tpcds.reason VALUES (1, 'AAAAAAAABAAAAAAA', 'Xxxxxxxxx'),(2, 'AAAAAAAACAAAAAAA', ' Xxxxxxxxx'),(3, 'AAAAAAAADAAAAAAA', ' Xxxxxxxxx'),(4, 'AAAAAAAAEAAAAAAA', 'Not the product that was ordered'),(5, 'AAAAAAAAFAAAAAAA', 'Parts missing'),(6, 'AAAAAAAAGAAAAAAA', 'Does not work with a product that I have'),(7, 'AAAAAAAAHAAAAAAA', 'Gift exchange');

--插入训练数据
-- 来源: 3063_PREDICT BY
INSERT INTO houses(id, tax, bedroom, bath, price, size, lot, mark) VALUES (1,590,2,1,50000,770,22100,'a+'), (2,1050,3,2,85000,1410,12000,'a+'), (3,20,2,1,22500,1060,3500,'a-'), (4,870,2,2,90000,1300,17500,'a+'), (5,1320,3,2,133000,1500,30000,'a+'), (6,1350,2,1,90500,850,25700,'a-'), (7,2790,3,2.5,260000,2130,25000,'a+'), (8,680,2,1,142500,1170,22000,'a-'), (9,1840,3,2,160000,1500,19000,'a+'), (10,3680,4,2,240000,2790,20000,'a-'), (11,1660,3,1,87000,1030,17500,'a+'), (12,1620,3,2,118500,1250,20000,'a-'), (13,3100,3,2,140000,1760,38000,'a+'), (14,2090,2,3,148000,1550,14000,'a-'), (15,650,3,1.5,65000,1450,12000,'a-');

--基表写入数据。
-- 来源: 3069_REFRESH INCREMENTAL MATERIALIZED VIEW
INSERT INTO my_table VALUES(1,1),(2,2);

--基表写入数据。
-- 来源: 3070_REFRESH MATERIALIZED VIEW
INSERT INTO my_table VALUES(1,1),(2,2);

--向表中插入多条记录。
-- 来源: 3071_REINDEX
INSERT INTO tpcds.customer VALUES (1, 'AAAAAAAABAAAAAAA'),(5, 'AAAAAAAACAAAAAAA'),(10, 'AAAAAAAADAAAAAAA');

-- 来源: 3071_REINDEX
INSERT INTO tpcds. customer_t1 SELECT * FROM tpcds. customer WHERE c_customer_sk < 10;

--插入数据。
-- 来源: 3072_RELEASE SAVEPOINT
INSERT INTO tpcds. table1 VALUES (3);

--插入数据。
-- 来源: 3072_RELEASE SAVEPOINT
INSERT INTO tpcds. table1 VALUES (4);

--插入数据。
-- 来源: 3073_REPLACE
INSERT INTO test VALUES(1, 1, 1), (2, 2, 2), (3, 3, 3);

--插入数据。
-- 来源: 3080_SAVEPOINT
INSERT INTO table1 VALUES (1);

--插入数据。
-- 来源: 3080_SAVEPOINT
INSERT INTO table1 VALUES (2);

--插入数据。
-- 来源: 3080_SAVEPOINT
INSERT INTO table1 VALUES (3);

--插入数据。
-- 来源: 3080_SAVEPOINT
INSERT INTO table2 VALUES (3);

--插入数据。
-- 来源: 3080_SAVEPOINT
INSERT INTO table2 VALUES (4);

-- 来源: 3082_SELECT
INSERT INTO test VALUES('A', 1, 0), ('B', 2, 1),('C',3,1),('D',4,1),('E',5,2);

--向表中插入多条记录。
-- 来源: 3082_SELECT
INSERT INTO tpcds.reason values(3,'AAAAAAAABAAAAAAA','reason 1'),(10,'AAAAAAAABAAAAAAA','reason 2'),(4,'AAAAAAAABAAAAAAA','reason 3'),(10,'AAAAAAAABAAAAAAA','reason 4'),(10,'AAAAAAAABAAAAAAA','reason 5'),(20,'AAAAAAAACAAAAAAA','N%reason 6'),(30,'AAAAAAAACAAAAAAA','W%reason 7');

--插入数据。
-- 来源: 3082_SELECT
INSERT INTO tpcds. reason_p values(3,'AAAAAAAABAAAAAAA','reason 1'),(10,'AAAAAAAABAAAAAAA','reason 2'),(4,'AAAAAAAABAAAAAAA','reason 3'),(10,'AAAAAAAABAAAAAAA','reason 4'),(10,'AAAAAAAABAAAAAAA','reason 5'),(20,'AAAAAAAACAAAAAAA','reason 6'),(30,'AAAAAAAACAAAAAAA','reason 7');

--向表tpcds.time_table中插入记录
-- 来源: 3082_SELECT
INSERT INTO tpcds.time_table select 1, now(),int8in(xidout(next_csn)), 'time1' from gs_get_next_xid_csn();

-- 来源: 3082_SELECT
INSERT INTO tpcds.time_table select 2, now(),int8in(xidout(next_csn)), 'time2' from gs_get_next_xid_csn();

-- 来源: 3082_SELECT
INSERT INTO tpcds.time_table select 3, now(),int8in(xidout(next_csn)), 'time3' from gs_get_next_xid_csn();

-- 来源: 3082_SELECT
INSERT INTO tpcds.time_table select 4, now(),int8in(xidout(next_csn)), 'time4' from gs_get_next_xid_csn();

-- 来源: 3082_SELECT
INSERT INTO p1 values(1,20,30);

-- 来源: 3082_SELECT
INSERT INTO p1 values(2,30,40);

-- 来源: 3082_SELECT
INSERT INTO p1 values(3,40,50);

-- 来源: 3082_SELECT
INSERT INTO p2 SELECT * FROM p1 UNPIVOT(score FOR class IN(math, phy));

-- 来源: 3082_SELECT
INSERT INTO skiplocked_astore VALUES (1, 'abc'), (2, 'bcd'), (3, 'cdf'),(3, 'dfg');

--向表中插入多条记录。
-- 来源: 3083_SELECT INTO
INSERT INTO tpcds.reason values(1,'AAAAAAAABAAAAAAA','reason 1'),(2,'AAAAAAAABAAAAAAA','reason 2'),(3,'AAAAAAAABAAAAAAA','reason 3'),(4,'AAAAAAAABAAAAAAA','reason 4'),(4,'AAAAAAAABAAAAAAA','reason 5'),(4,'AAAAAAAACAAAAAAA','reason 6'),(5,'AAAAAAAACAAAAAAA','reason 7');

--插入数据
-- 来源: 3091_SHRINK
INSERT INTO row_compression SELECT generate_series(1,1000);

-- 来源: 3093_SNAPSHOT
INSERT INTO t1 VALUES (1, 'zhangsan');

-- 来源: 3093_SNAPSHOT
INSERT INTO t1 VALUES (2, 'lisi');

-- 来源: 3093_SNAPSHOT
INSERT INTO t1 VALUES (3, 'wangwu');

-- 来源: 3093_SNAPSHOT
INSERT INTO t1 VALUES (4, 'lisa');

-- 来源: 3093_SNAPSHOT
INSERT INTO t1 VALUES (5, 'jack');

--向表tpcds.reason_t2中插入记录
-- 来源: 3096_TIMECAPSULE TABLE
INSERT INTO tpcds.reason_t2 VALUES (1, 'AA', 'reason1'),(2, 'AB', 'reason2'),(3, 'AC', 'reason3');

--向表中插入多条记录。
-- 来源: 3097_TRUNCATE
INSERT INTO tpcds.reason values(1,'AAAAAAAABAAAAAAA','reason 1'),(5,'AAAAAAAABAAAAAAA','reason 2'),(15,'AAAAAAAABAAAAAAA','reason 3'),(25,'AAAAAAAABAAAAAAA','reason 4'),(35,'AAAAAAAABAAAAAAA','reason 5'),(45,'AAAAAAAACAAAAAAA','reason 6'),(55,'AAAAAAAACAAAAAAA','reason 7');

--插入数据。
-- 来源: 3097_TRUNCATE
INSERT INTO tpcds. reason_p SELECT * FROM tpcds. reason;

-- 来源: 3099_UPDATE
INSERT INTO tbl_test1 VALUES (1, 'A'), (2, 'B');

--插入数据。
-- 来源: 3099_UPDATE
INSERT INTO test_grade VALUES (1,'Scott','A','2008-07-08',1),(2,'Ben','D','2008-07-08',1),(3,'Jack','D','2008-07-08',1);

--向表中插入多条记录。
-- 来源: 3101_VACUUM
INSERT INTO tpcds.reason values(1,'AAAAAAAABAAAAAAA','reason 1'),(2,'AAAAAAAABAAAAAAA','reason 2');

-- 来源: 3135_record
insert into emp_rec values ( 111 , 'aaa' ), ( 222 , 'bbb' ), ( 333 , 'ccc' );

-- 来源: 3146_file_3146
INSERT INTO staffs VALUES ( 30 , 10 );

-- 来源: 3146_file_3146
INSERT INTO staffs VALUES ( 30 , 20 );

-- 来源: 3148_file_3148
INSERT INTO staffs VALUES (200, 'mike', 5800);

-- 来源: 3148_file_3148
INSERT INTO staffs VALUES (201, 'lily', 3000);

-- 来源: 3148_file_3148
INSERT INTO staffs VALUES (202, 'john', 4400);

-- 来源: 3148_file_3148
INSERT INTO staffs VALUES (30, 'mike', '13567829252', 5800);

-- 来源: 3148_file_3148
INSERT INTO staffs VALUES (40, 'john', '17896354637', 4000);

-- 来源: 3151_file_3151
INSERT INTO staffs VALUES (200, 'mike', 5800);

-- 来源: 3151_file_3151
INSERT INTO staffs VALUES (201, 'lily', 3000);

-- 来源: 3151_file_3151
INSERT INTO staffs VALUES (202, 'john', 4400);

-- 来源: 3155_RETURN NEXTRETURN QUERY
INSERT INTO t1 VALUES ( 1 ),( 10 );

-- 来源: 3157_file_3157
INSERT INTO hdfs_t1 VALUES ( 8 , 'Donald' , 'OConnell' , 'DOCONNEL' , '650.507.9833' , to_date ( '21-06-1999' , 'dd-mm-yyyy' ), 'SH_CLERK' );

-- 来源: 3160_file_3160
INSERT INTO mytab ( firstname , lastname ) VALUES ( 'Tom' , 'Jones' );

-- 来源: 3168_file_3168
INSERT INTO sections VALUES ('hr',1,1);

-- 来源: 3168_file_3168
INSERT INTO staffs VALUES (1,100,1,'Tom');

-- 来源: 3170_file_3170
INSERT INTO integerTable2 VALUES ( 2 );

-- 来源: 3180_DBE_HEAT_MAP
INSERT INTO HEAT_MAP_DATA . heat_map_table VALUES ( 1 , 'test_data_row_1' );

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(generate_series(1,100),1);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(1,1);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(1,1);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(1,1);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(2,2);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(2,2);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(2,2);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(1,1);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(1,1);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(1,1);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(2,2);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(2,2);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(2,2);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(1,1);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(1,1);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(1,1);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(2,2);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(2,2);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(2,2);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(1,1);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(1,1);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(1,1);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(2,2);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(2,2);

-- 来源: 3192_DBE_STATS
INSERT INTO t1 VALUES(2,2);

-- 来源: 3202_file_3202
INSERT INTO sections VALUES(1);

-- 来源: 3202_file_3202
INSERT INTO sections VALUES(1);

-- 来源: 3202_file_3202
INSERT INTO sections VALUES(1);

-- 来源: 3202_file_3202
INSERT INTO sections VALUES(1);

-- 来源: 3202_file_3202
INSERT INTO test_main VALUES (1111,'2021-01-01'),(2222,'2021-02-02');

-- 来源: 3203_file_3203
INSERT INTO t2 VALUES(1,2);

-- 来源: 3204_file_3204
INSERT INTO t1 VALUES(1,'YOU WILL ROLLBACK!');

-- 来源: 3206_Package
INSERT INTO t2 VALUES(1,2);

-- 来源: 4027_file_4027
insert into test1 values ( 2 , '1.1' );

-- 来源: 4027_file_4027
insert into tb_test values(1,'a','b');

-- 来源: 4027_file_4027
insert into tab_2 values(' ');

-- 来源: 4027_file_4027
insert into tab_1 select col2 from tab_2;

-- 来源: 4027_file_4027
insert into tab_1 select col2 from tab_2;

-- 来源: 4027_file_4027
insert into test values('x');

-- 再次向物化视图中基表插入数据
-- 来源: 4273_file_4273
INSERT INTO t1 VALUES(3, 3);

-- 插入数据
-- 来源: 4277_file_4277
INSERT INTO t1 VALUES(3, 3);

-- 插入数据
-- 来源: 4277_file_4277
INSERT INTO t1 VALUES(4, 4);

-- 来源: 4280_gsql
INSERT INTO creditcard_info VALUES (1,'joe','6217986500001288393');

-- 来源: 4280_gsql
INSERT INTO creditcard_info VALUES (2, 'joy','6219985678349800033');

-- 来源: 4283__
insert into creditcard_info values ( 1 , 'Avi' , '1234567890123456' );

-- 来源: 4283__
insert into creditcard_info values ( 2 , 'Eli' , '2345678901234567' );

-- 来源: 4284_file_4284
INSERT INTO t1 VALUES (1, 'tde plain 123');

-- 指定分区p_list_7插入数据，由于数据不符合该分区约束，插入报错
-- 来源: 4313_DQL_DML
INSERT INTO list_02 PARTITION (p_list_7) VALUES(null, 'cherry', 'cherry data');

-- 来源: 4322_file_4322
insert into t1_range_int select v,v,v,v from generate_series(0, 49) as v;

-- 来源: 4393_file_4393
INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

-- 来源: 4394_file_4394
INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

-- 来源: 4394_file_4394
INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

-- 来源: 4394_file_4394
INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

-- 来源: 4395_DROP_TRUNCATE
insert into flashtest values(1, 'A');

-- 来源: 4395_DROP_TRUNCATE
insert into flashtest values(1, 'A');

-- 来源: 4395_DROP_TRUNCATE
insert into flashtest values(1, 'A');

-- 来源: 4407_file_4407
INSERT INTO ilm_table_1 select *, 'test_data' FROM generate_series(1, 10000);

-- 来源: 4409_TIPS
INSERT INTO HEAT_MAP_DATA . heat_map_table VALUES ( 1 , 'test_data_row_1' );

-- 来源: 4493_file_4493
INSERT INTO t1 VALUES(1, 1);

-- 来源: 4493_file_4493
INSERT INTO t1 VALUES(2, 2);

--向物化视图中基表插入数据。
-- 来源: 4493_file_4493
INSERT INTO t1 VALUES(3, 3);

-- 来源: 4497_file_4497
INSERT INTO t1 VALUES(1, 1);

-- 来源: 4497_file_4497
INSERT INTO t1 VALUES(2, 2);

--插入数据。
-- 来源: 4497_file_4497
INSERT INTO t1 VALUES(3, 3);

--插入数据。
-- 来源: 4497_file_4497
INSERT INTO t1 VALUES(4, 4);

-- 来源: 4500_gsql
INSERT INTO creditcard_info VALUES (1,'joe','6217986500001288393');

-- 来源: 4500_gsql
INSERT INTO creditcard_info VALUES (2, 'joy','6219985678349800033');

-- 来源: 4504__
insert into creditcard_info values ( 1 , 'Avi' , '1234567890123456' );

-- 来源: 4504__
insert into creditcard_info values ( 2 , 'Eli' , '2345678901234567' );

-- 来源: 4507_gsql
INSERT INTO contacts VALUES ( 1 , 8000 , 'zhangsan' );

-- 来源: 4507_gsql
INSERT INTO contacts VALUES ( 2 , 7056 . 6 , 'lisi' );

-- 来源: 4507_gsql
INSERT INTO contacts VALUES ( 3 , 16050 , 'wangwu' );

-- 来源: 4507_gsql
insert into contacts_plain (id, name) select id, ce_decrypt_deterministic(name, (select column_key_distributed_id from gs_column_keys where column_key_name=' cek1 ')) from contacts;

-- 来源: 4507_gsql
insert into contacts (id, name) select id, ce_encrypt_deterministic(name, (select column_key_distributed_id from gs_column_keys where column_key_name=' cek1 ')) from contacts_plain;

-- 来源: 4511_file_4511
INSERT INTO t1 VALUES (1, 'tde plain 123');

-- 来源: 4522_DDL
INSERT INTO test1(col1) values(1);

-- 来源: 4522_DDL
INSERT INTO mix_tran_t4 VALUES(111);

-- 来源: 4522_DDL
INSERT INTO mix_tran_t4 VALUES(111);

-- 来源: 4522_DDL
INSERT INTO mix_tran_t4 VALUES(111);

-- 来源: 4522_DDL
INSERT INTO mix_tran_t4 VALUES(111);

-- 来源: 4522_DDL
INSERT INTO mix_tran_t4 VALUES(111);

-- 来源: 4522_DDL
INSERT INTO mix_tran_t4 VALUES(111);

-- 来源: 4522_DDL
INSERT INTO mix_tran_t4 VALUES(111);

-- 来源: 4522_DDL
INSERT INTO mix_tran_t4 VALUES(111);

-- 来源: 4522_DDL
INSERT INTO mix_tran_t4 VALUES(111);

-- 来源: 4522_DDL
INSERT INTO mix_tran_t4 VALUES(111);

-- 指定分区p_list_7_1插入数据，由于数据不符合该分区约束，插入报错
-- 来源: 4543_DQL_DML
INSERT INTO list_list_02 SUBPARTITION (p_list_7_1) VALUES(null, 'cherry', 'cherry data');

-- 来源: 4557_file_4557
insert into t1_range_int select v,v,v,v from generate_series(0, 49) as v;

-- 来源: 4653_file_4653
INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

-- 来源: 4654_file_4654
INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

-- 来源: 4654_file_4654
INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

-- 来源: 4655_DROP_TRUNCATE
insert into flashtest values(1, 'A');

-- 来源: 4655_DROP_TRUNCATE
insert into flashtest values(1, 'A');

-- 来源: 4655_DROP_TRUNCATE
insert into flashtest values(1, 'A');

-- 来源: 4667_file_4667
INSERT INTO ilm_table_1 select *, 'test_data' FROM generate_series(1, 10000);

-- 来源: 4669_TIPS
INSERT INTO HEAT_MAP_DATA . heat_map_table VALUES ( 1 , 'test_data_row_1' );

-- 来源: 5779_file_5779
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 1 , 'Europe' );

-- 来源: 5779_file_5779
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 2 , 'Americas' );

-- 来源: 5779_file_5779
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 3 , 'Asia' );

-- 来源: 5779_file_5779
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 4 , 'Middle East and Africa' );

-- 来源: 5839_gs_rescue
insert into amend(col1) select * from original;

-- 来源: 5892_file_5892
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 1 , 'Europe' );

-- 来源: 5892_file_5892
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 2 , 'Americas' );

-- 来源: 5892_file_5892
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 3 , 'Asia' );

-- 来源: 5892_file_5892
INSERT INTO HR . areaS ( area_ID , area_NAME ) VALUES ( 4 , 'Middle East and Africa' );

-- 来源: 5949_gs_rescue
insert into amend(col1) select * from original;

-- 来源: 733_file_733
insert into a values(1,4);

-- 来源: 733_file_733
insert into a values(2,4);

-- 来源: 733_file_733
insert into a values(1,10);

-- 来源: 733_file_733
insert into a values(1, 100);

-- 来源: 744_file_744
INSERT INTO all_data VALUES ( 1 , 'alice' , 'alice data' );

-- 来源: 744_file_744
INSERT INTO all_data VALUES ( 2 , 'bob' , 'bob data' );

-- 来源: 744_file_744
INSERT INTO all_data VALUES ( 3 , 'peter' , 'peter data' );

-- 来源: 757_file_757
INSERT INTO table1 VALUES ( 1 , reverse ( '123AAA78' ), reverse ( '123AA78' ), reverse ( '123AA78' ));

-- 来源: 757_file_757
INSERT INTO table1 VALUES ( 2 , reverse ( '123A78' ), reverse ( '123A78' ), reverse ( '123A78' ));

-- 来源: 757_file_757
INSERT INTO table1 VALUES ( 3 , '87A123' , '87A123' , '87A123' );

-- 来源: 757_file_757
INSERT INTO table2 VALUES ( 1 , reverse ( '123AA78' ), reverse ( '123AA78' ), reverse ( '123AA78' ));

-- 来源: 757_file_757
INSERT INTO table2 VALUES ( 2 , reverse ( '123A78' ), reverse ( '123A78' ), reverse ( '123A78' ));

-- 来源: 757_file_757
INSERT INTO customer_t1 ( c_customer_sk , c_customer_id , c_first_name ) VALUES ( 3769 , 'hello' , 'Grace' );

-- 来源: 757_file_757
INSERT INTO customer_t1 VALUES ( 3769 , 'hello' , 'Grace' );

-- 来源: 757_file_757
INSERT INTO customer_t1 ( c_customer_sk , c_first_name ) VALUES ( 3769 , 'Grace' );

-- 来源: 757_file_757
INSERT INTO customer_t1 VALUES ( 3769 , 'hello' );

-- 来源: 757_file_757
INSERT INTO customer_t1 ( c_customer_sk , c_customer_id , c_first_name ) VALUES ( 3769 , 'hello' , DEFAULT );

-- 来源: 757_file_757
INSERT INTO customer_t1 DEFAULT VALUES ;

-- 来源: 757_file_757
INSERT INTO customer_t1 ( c_customer_sk , c_customer_id , c_first_name ) VALUES ( 6885 , 'maps' , 'Joes' ), ( 4321 , 'tpcds' , 'Lily' ), ( 9527 , 'world' , 'James' );

-- 来源: 757_file_757
INSERT INTO customer_t2 SELECT * FROM customer_t1 ;

-- 来源: 764_file_764
INSERT INTO tpcds . web_returns_p2 SELECT * FROM tpcds . customer_address ;

-- 来源: 764_file_764
INSERT INTO tpcds . web_returns_p2 SELECT * FROM tpcds . customer_address ;

-- 来源: 767_file_767
INSERT INTO newT1 ( name ) SELECT name FROM T1 ;

-- 来源: 767_file_767
INSERT INTO newT1 ( id , name ) SELECT id , name FROM T1 ;

