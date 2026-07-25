-- 来源: 2980_CREATE TABLE SUBPARTITION.txt
-- SQL 数量: 202

CREATE TABLE list_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY LIST ( month_code ) SUBPARTITION BY LIST ( dept_code ) ( PARTITION p_201901 VALUES ( '201902' ) ( SUBPARTITION p_201901_a VALUES ( '1' ), SUBPARTITION p_201901_b VALUES ( '2' ) ), PARTITION p_201902 VALUES ( '201903' ) ( SUBPARTITION p_201902_a VALUES ( '1' ), SUBPARTITION p_201902_b VALUES ( '2' ) ) );

INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO list_list VALUES ( '201902' , '2' , '1' , 1 );

INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

INSERT INTO list_list VALUES ( '201903' , '1' , '1' , 1 );

INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

select * from list_list ;

DROP TABLE list_list ;

CREATE TABLE list_hash ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY LIST ( month_code ) SUBPARTITION BY HASH ( dept_code ) ( PARTITION p_201901 VALUES ( '201902' ) ( SUBPARTITION p_201901_a , SUBPARTITION p_201901_b ), PARTITION p_201902 VALUES ( '201903' ) ( SUBPARTITION p_201902_a , SUBPARTITION p_201902_b ) );

INSERT INTO list_hash VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO list_hash VALUES ( '201902' , '2' , '1' , 1 );

INSERT INTO list_hash VALUES ( '201902' , '3' , '1' , 1 );

INSERT INTO list_hash VALUES ( '201903' , '4' , '1' , 1 );

INSERT INTO list_hash VALUES ( '201903' , '5' , '1' , 1 );

INSERT INTO list_hash VALUES ( '201903' , '6' , '1' , 1 );

select * from list_hash ;

DROP TABLE list_hash ;

CREATE TABLE list_range ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY LIST ( month_code ) SUBPARTITION BY RANGE ( dept_code ) ( PARTITION p_201901 VALUES ( '201902' ) ( SUBPARTITION p_201901_a VALUES less than ( '4' ), SUBPARTITION p_201901_b VALUES less than ( '6' ) ), PARTITION p_201902 VALUES ( '201903' ) ( SUBPARTITION p_201902_a VALUES less than ( '3' ), SUBPARTITION p_201902_b VALUES less than ( '6' ) ) );

INSERT INTO list_range VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO list_range VALUES ( '201902' , '2' , '1' , 1 );

INSERT INTO list_range VALUES ( '201902' , '3' , '1' , 1 );

INSERT INTO list_range VALUES ( '201903' , '4' , '1' , 1 );

INSERT INTO list_range VALUES ( '201903' , '5' , '1' , 1 );

INSERT INTO list_range VALUES ( '201903' , '6' , '1' , 1 );

select * from list_range ;

DROP TABLE list_range ;

CREATE TABLE range_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE ( month_code ) SUBPARTITION BY LIST ( dept_code ) ( PARTITION p_201901 VALUES LESS THAN ( '201903' ) ( SUBPARTITION p_201901_a VALUES ( '1' ), SUBPARTITION p_201901_b VALUES ( '2' ) ), PARTITION p_201902 VALUES LESS THAN ( '201904' ) ( SUBPARTITION p_201902_a VALUES ( '1' ), SUBPARTITION p_201902_b VALUES ( '2' ) ) );

INSERT INTO range_list VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO range_list VALUES ( '201902' , '2' , '1' , 1 );

INSERT INTO range_list VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO range_list VALUES ( '201903' , '2' , '1' , 1 );

INSERT INTO range_list VALUES ( '201903' , '1' , '1' , 1 );

INSERT INTO range_list VALUES ( '201903' , '2' , '1' , 1 );

select * from range_list ;

DROP TABLE range_list ;

CREATE TABLE range_hash ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE ( month_code ) SUBPARTITION BY HASH ( dept_code ) ( PARTITION p_201901 VALUES LESS THAN ( '201903' ) ( SUBPARTITION p_201901_a , SUBPARTITION p_201901_b ), PARTITION p_201902 VALUES LESS THAN ( '201904' ) ( SUBPARTITION p_201902_a , SUBPARTITION p_201902_b ) );

INSERT INTO range_hash VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO range_hash VALUES ( '201902' , '2' , '1' , 1 );

INSERT INTO range_hash VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO range_hash VALUES ( '201903' , '2' , '1' , 1 );

INSERT INTO range_hash VALUES ( '201903' , '1' , '1' , 1 );

INSERT INTO range_hash VALUES ( '201903' , '2' , '1' , 1 );

select * from range_hash ;

DROP TABLE range_hash ;

CREATE TABLE range_range ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE ( month_code ) SUBPARTITION BY RANGE ( dept_code ) ( PARTITION p_201901 VALUES LESS THAN ( '201903' ) ( SUBPARTITION p_201901_a VALUES LESS THAN ( '2' ), SUBPARTITION p_201901_b VALUES LESS THAN ( '3' ) ), PARTITION p_201902 VALUES LESS THAN ( '201904' ) ( SUBPARTITION p_201902_a VALUES LESS THAN ( '2' ), SUBPARTITION p_201902_b VALUES LESS THAN ( '3' ) ) );

INSERT INTO range_range VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO range_range VALUES ( '201902' , '2' , '1' , 1 );

INSERT INTO range_range VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO range_range VALUES ( '201903' , '2' , '1' , 1 );

INSERT INTO range_range VALUES ( '201903' , '1' , '1' , 1 );

INSERT INTO range_range VALUES ( '201903' , '2' , '1' , 1 );

select * from range_range ;

DROP TABLE range_range ;

CREATE TABLE hash_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY hash ( month_code ) SUBPARTITION BY LIST ( dept_code ) ( PARTITION p_201901 ( SUBPARTITION p_201901_a VALUES ( '1' ), SUBPARTITION p_201901_b VALUES ( '2' ) ), PARTITION p_201902 ( SUBPARTITION p_201902_a VALUES ( '1' ), SUBPARTITION p_201902_b VALUES ( '2' ) ) );

INSERT INTO hash_list VALUES ( '201901' , '1' , '1' , 1 );

INSERT INTO hash_list VALUES ( '201901' , '2' , '1' , 1 );

INSERT INTO hash_list VALUES ( '201901' , '1' , '1' , 1 );

INSERT INTO hash_list VALUES ( '201903' , '2' , '1' , 1 );

INSERT INTO hash_list VALUES ( '201903' , '1' , '1' , 1 );

INSERT INTO hash_list VALUES ( '201903' , '2' , '1' , 1 );

select * from hash_list ;

DROP TABLE hash_list ;

CREATE TABLE hash_hash ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY hash ( month_code ) SUBPARTITION BY hash ( dept_code ) ( PARTITION p_201901 ( SUBPARTITION p_201901_a , SUBPARTITION p_201901_b ), PARTITION p_201902 ( SUBPARTITION p_201902_a , SUBPARTITION p_201902_b ) );

INSERT INTO hash_hash VALUES ( '201901' , '1' , '1' , 1 );

INSERT INTO hash_hash VALUES ( '201901' , '2' , '1' , 1 );

INSERT INTO hash_hash VALUES ( '201901' , '1' , '1' , 1 );

INSERT INTO hash_hash VALUES ( '201903' , '2' , '1' , 1 );

INSERT INTO hash_hash VALUES ( '201903' , '1' , '1' , 1 );

INSERT INTO hash_hash VALUES ( '201903' , '2' , '1' , 1 );

select * from hash_hash ;

DROP TABLE hash_hash ;

CREATE TABLE hash_range ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY hash ( month_code ) SUBPARTITION BY range ( dept_code ) ( PARTITION p_201901 ( SUBPARTITION p_201901_a VALUES LESS THAN ( '2' ), SUBPARTITION p_201901_b VALUES LESS THAN ( '3' ) ), PARTITION p_201902 ( SUBPARTITION p_201902_a VALUES LESS THAN ( '2' ), SUBPARTITION p_201902_b VALUES LESS THAN ( '3' ) ) );

INSERT INTO hash_range VALUES ( '201901' , '1' , '1' , 1 );

INSERT INTO hash_range VALUES ( '201901' , '2' , '1' , 1 );

INSERT INTO hash_range VALUES ( '201901' , '1' , '1' , 1 );

INSERT INTO hash_range VALUES ( '201903' , '2' , '1' , 1 );

INSERT INTO hash_range VALUES ( '201903' , '1' , '1' , 1 );

INSERT INTO hash_range VALUES ( '201903' , '2' , '1' , 1 );

select * from hash_range ;

DROP TABLE hash_range ;

CREATE TABLE range_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY LIST (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a VALUES ('1'), SUBPARTITION p_201901_b VALUES ('2') ), PARTITION p_201902 VALUES LESS THAN( '201910' ) ( SUBPARTITION p_201902_a VALUES ('1'), SUBPARTITION p_201902_b VALUES ('2') ) );

--指定一级分区插入数据
INSERT INTO range_list partition (p_201901) VALUES('201902', '1', '1', 1);

--实际分区和指定分区不一致，报错
INSERT INTO range_list partition (p_201902) VALUES('201902', '1', '1', 1);

--指定二级分区插入数据
INSERT INTO range_list subpartition (p_201901_a) VALUES('201902', '1', '1', 1);

--实际分区和指定分区不一致，报错
INSERT INTO range_list subpartition (p_201901_b) VALUES('201902', '1', '1', 1);

INSERT INTO range_list partition for ('201902') VALUES('201902', '1', '1', 1);

INSERT INTO range_list subpartition for ('201902','1') VALUES('201902', '1', '1', 1);

--指定分区查询数据
select * from range_list partition (p_201901);

select * from range_list subpartition (p_201901_a);

select * from range_list partition for ('201902');

select * from range_list subpartition for ('201902','1');

--指定分区更新数据
update range_list partition (p_201901) set user_no = '2';

select * from range_list;

update range_list subpartition (p_201901_a) set user_no = '3';

select * from range_list;

update range_list partition for ('201902') set user_no = '4';

select * from range_list;

update range_list subpartition for ('201902','2') set user_no = '5';

select *from range_list;

select * from range_list;

--指定分区删除数据
delete from range_list partition (p_201901);

delete from range_list partition for ('201903');

delete from range_list subpartition (p_201901_a);

delete from range_list subpartition for ('201903','2');

--参数sql_compatibility='B'时，可指定多分区删除数据
CREATE DATABASE db dbcompatibility 'B';

CREATE TABLE range_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY LIST (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a VALUES ('1'), SUBPARTITION p_201901_b VALUES ('2') ), PARTITION p_201902 VALUES LESS THAN( '201910' ) ( SUBPARTITION p_201902_a VALUES ('1'), SUBPARTITION p_201902_b VALUES ('2') ) );

delete from range_list as t partition (p_201901_a, p_201901);

--删除数据库
DROP DATABASE db;

--指定分区insert数据
INSERT INTO range_list partition (p_201901) VALUES('201902', '1', '1', 1) ON DUPLICATE KEY UPDATE sales_amt = 5;

INSERT INTO range_list subpartition (p_201901_a) VALUES('201902', '1', '1', 1) ON DUPLICATE KEY UPDATE sales_amt = 10;

INSERT INTO range_list partition for ('201902') VALUES('201902', '1', '1', 1) ON DUPLICATE KEY UPDATE sales_amt = 30;

INSERT INTO range_list subpartition for ('201902','1') VALUES('201902', '1', '1', 1) ON DUPLICATE KEY UPDATE sales_amt = 40;

select * from range_list;

--指定分区merge into数据
CREATE TABLE newrange_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY RANGE (month_code) SUBPARTITION BY LIST (dept_code) ( PARTITION p_201901 VALUES LESS THAN( '201903' ) ( SUBPARTITION p_201901_a VALUES ('1'), SUBPARTITION p_201901_b VALUES ('2') ), PARTITION p_201902 VALUES LESS THAN( '201910' ) ( SUBPARTITION p_201902_a VALUES ('1'), SUBPARTITION p_201902_b VALUES ('2') ) );

INSERT INTO newrange_list VALUES('201902', '1', '1', 1);

INSERT INTO newrange_list VALUES('201903', '1', '1', 2);

MERGE INTO range_list partition (p_201901) p USING newrange_list partition (p_201901) np ON p.month_code= np.month_code WHEN MATCHED THEN UPDATE SET dept_code = np.dept_code, user_no = np.user_no, sales_amt = np.sales_amt WHEN NOT MATCHED THEN INSERT VALUES (np.month_code, np.dept_code, np.user_no, np.sales_amt);

select * from range_list;

MERGE INTO range_list partition for ('201901') p USING newrange_list partition for ('201901') np ON p.month_code= np.month_code WHEN MATCHED THEN UPDATE SET dept_code = np.dept_code, user_no = np.user_no, sales_amt = np.sales_amt WHEN NOT MATCHED THEN INSERT VALUES (np.month_code, np.dept_code, np.user_no, np.sales_amt);

select * from range_list;

MERGE INTO range_list subpartition (p_201901_a) p USING newrange_list subpartition (p_201901_a) np ON p.month_code= np.month_code WHEN MATCHED THEN UPDATE SET dept_code = np.dept_code, user_no = np.user_no, sales_amt = np.sales_amt WHEN NOT MATCHED THEN INSERT VALUES (np.month_code, np.dept_code, np.user_no, np.sales_amt);

select * from range_list;

MERGE INTO range_list subpartition for ('201901', '1') p USING newrange_list subpartition for ('201901', '1') np ON p.month_code= np.month_code WHEN MATCHED THEN UPDATE SET dept_code = np.dept_code, user_no = np.user_no, sales_amt = np.sales_amt WHEN NOT MATCHED THEN INSERT VALUES (np.month_code, np.dept_code, np.user_no, np.sales_amt);

select * from range_list;

DROP TABLE range_list;

DROP TABLE newrange_list;

CREATE TABLE list_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY LIST ( month_code ) SUBPARTITION BY LIST ( dept_code ) ( PARTITION p_201901 VALUES ( '201902' ) ( SUBPARTITION p_201901_a VALUES ( '1' ), SUBPARTITION p_201901_b VALUES ( default ) ), PARTITION p_201902 VALUES ( '201903' ) ( SUBPARTITION p_201902_a VALUES ( '1' ), SUBPARTITION p_201902_b VALUES ( '2' ) ) );

INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO list_list VALUES ( '201902' , '2' , '1' , 1 );

INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

INSERT INTO list_list VALUES ( '201903' , '1' , '1' , 1 );

INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

select * from list_list ;

select * from list_list partition ( p_201901 );

alter table list_list truncate partition p_201901 ;

select * from list_list partition ( p_201901 );

select * from list_list partition ( p_201902 );

alter table list_list truncate partition p_201902 ;

select * from list_list partition ( p_201902 );

select * from list_list ;

INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO list_list VALUES ( '201902' , '2' , '1' , 1 );

INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

INSERT INTO list_list VALUES ( '201903' , '1' , '1' , 1 );

INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

select * from list_list subpartition ( p_201901_a );

alter table list_list truncate subpartition p_201901_a ;

select * from list_list subpartition ( p_201901_a );

select * from list_list subpartition ( p_201901_b );

alter table list_list truncate subpartition p_201901_b ;

select * from list_list subpartition ( p_201901_b );

select * from list_list subpartition ( p_201902_a );

alter table list_list truncate subpartition p_201902_a ;

select * from list_list subpartition ( p_201902_a );

select * from list_list subpartition ( p_201902_b );

alter table list_list truncate subpartition p_201902_b ;

select * from list_list subpartition ( p_201902_b );

select * from list_list ;

DROP TABLE list_list ;

CREATE TABLE list_list ( month_code VARCHAR2 ( 30 ) NOT NULL , dept_code VARCHAR2 ( 30 ) NOT NULL , user_no VARCHAR2 ( 30 ) NOT NULL , sales_amt int ) PARTITION BY LIST ( month_code ) SUBPARTITION BY LIST ( dept_code ) ( PARTITION p_201901 VALUES ( '201902' ) ( SUBPARTITION p_201901_a VALUES ( '1' ), SUBPARTITION p_201901_b VALUES ( default ) ), PARTITION p_201902 VALUES ( '201903' ) ( SUBPARTITION p_201902_a VALUES ( '1' ), SUBPARTITION p_201902_b VALUES ( default ) ) );

INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO list_list VALUES ( '201902' , '2' , '1' , 1 );

INSERT INTO list_list VALUES ( '201902' , '1' , '1' , 1 );

INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

INSERT INTO list_list VALUES ( '201903' , '1' , '1' , 1 );

INSERT INTO list_list VALUES ( '201903' , '2' , '1' , 1 );

select * from list_list ;

select * from list_list subpartition ( p_201901_a );

select * from list_list subpartition ( p_201901_b );

alter table list_list split subpartition p_201901_b VALUES ( 2 ) into ( subpartition p_201901_b , subpartition p_201901_c );

select * from list_list subpartition ( p_201901_a );

select * from list_list subpartition ( p_201901_b );

select * from list_list subpartition ( p_201901_c );

select * from list_list partition ( p_201901 );

select * from list_list subpartition ( p_201902_a );

select * from list_list subpartition ( p_201902_b );

alter table list_list split subpartition p_201902_b VALUES ( 3 ) into ( subpartition p_201902_b , subpartition p_201902_c );

select * from list_list subpartition ( p_201902_a );

select * from list_list subpartition ( p_201902_b );

select * from list_list subpartition ( p_201902_c );

DROP TABLE list_list ;

ALTER DATABASE set ilm = on ;

CREATE TABLE ilm_subpart ( a int , b int ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION PARTITION BY RANGE ( a ) SUBPARTITION BY RANGE ( b ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ( SUBPARTITION p1_s1 VALUES LESS THAN ( 10 ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION , SUBPARTITION p1_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p1_s3 VALUES LESS THAN ( 30 )), PARTITION p2 VALUES LESS THAN ( 20 ) ( SUBPARTITION p2_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p2_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p2_s3 VALUES LESS THAN ( 30 )), PARTITION p3 VALUES LESS THAN ( 30 ) ( SUBPARTITION p3_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p3_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p3_s3 VALUES LESS THAN ( 30 )));

DROP TABLE ilm_subpart ;

ALTER DATABASE set ilm = on ;

CREATE TABLE ilm_subpart ( a int , b int ) PARTITION BY RANGE ( a ) SUBPARTITION BY RANGE ( b ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ( SUBPARTITION p1_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p1_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p1_s3 VALUES LESS THAN ( 30 )), PARTITION p2 VALUES LESS THAN ( 20 ) ( SUBPARTITION p2_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p2_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p2_s3 VALUES LESS THAN ( 30 )), PARTITION p3 VALUES LESS THAN ( 30 ) ( SUBPARTITION p3_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p3_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p3_s3 VALUES LESS THAN ( 30 )));

ALTER TABLE ilm_subpart MODIFY SUBPARTITION p2_s1 ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION ;

ALTER TABLE ilm_subpart MODIFY SUBPARTITION p2_s1 ILM DISABLE_ALL ;

ALTER TABLE ilm_subpart MODIFY SUBPARTITION p2_s1 ILM ENABLE_ALL ;

ALTER TABLE ilm_subpart MODIFY SUBPARTITION p2_s1 ILM DELETE_ALL ;

DROP TABLE ilm_subpart ;

ALTER DATABASE set ilm = on ;

CREATE TABLE ilm_subpart ( a int , b int ) PARTITION BY RANGE ( a ) SUBPARTITION BY RANGE ( b ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ( SUBPARTITION p1_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p1_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p1_s3 VALUES LESS THAN ( 30 )), PARTITION p2 VALUES LESS THAN ( 20 ) ( SUBPARTITION p2_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p2_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p2_s3 VALUES LESS THAN ( 30 )), PARTITION p3 VALUES LESS THAN ( 30 ) ( SUBPARTITION p3_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p3_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p3_s3 VALUES LESS THAN ( 30 )));

ALTER TABLE ilm_subpart MODIFY PARTITION p2 ADD SUBPARTITION p2_s4 VALUES LESS THAN ( 40 ) ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION ;

DROP TABLE ilm_subpart ;

ALTER DATABASE set ilm = on ;

CREATE TABLE ilm_subpart ( a int , b int ) PARTITION BY RANGE ( a ) SUBPARTITION BY RANGE ( b ) ( PARTITION p1 VALUES LESS THAN ( 10 ) ( SUBPARTITION p1_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p1_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p1_s3 VALUES LESS THAN ( 30 )), PARTITION p2 VALUES LESS THAN ( 20 ) ( SUBPARTITION p2_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p2_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p2_s3 VALUES LESS THAN ( 30 )), PARTITION p3 VALUES LESS THAN ( 30 ) ( SUBPARTITION p3_s1 VALUES LESS THAN ( 10 ), SUBPARTITION p3_s2 VALUES LESS THAN ( 20 ), SUBPARTITION p3_s3 VALUES LESS THAN ( 30 )));

ALTER TABLE ilm_subpart SPLIT SUBPARTITION p1_s2 AT ( '15' ) INTO ( SUBPARTITION p1_s2_1 ILM ADD POLICY ROW STORE COMPRESS ADVANCED ROW AFTER 3 MONTHS OF NO MODIFICATION , SUBPARTITION p1_s2_2 );

DROP TABLE ilm_subpart ;

