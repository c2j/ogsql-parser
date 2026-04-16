-- 来源: 1371_SELECT.txt
-- SQL 数量: 45

SELECT * FROM XMLTABLE( XMLNAMESPACES('nspace1' AS "ns1", 'nspace2' AS "ns2"), -- 声明两个XML的命名空间'nspace1'和'nspace2'及对应的别名"ns1"和"ns2" '/ns1:root/*:child' -- 经row_expression从传入的数据中选取命名空间为'nspace1'的root节点，在选取其下面的所有child节点，忽略child的命名空间；其中ns1为'nspace1'的别名 PASSING xmltype( '<root xmlns="nspace1"> <child> <name>peter</name> <age>11</age> </child> <child xmlns="nspace1"> <name>qiqi</name> <age>12</age> </child> <child xmlns="nspace2"> <name>hacker</name> <age>15</age> </child> </root>') COLUMNS column FOR ORDINALITY, -- 该列为行号列 name varchar(10) path 'ns1:name', -- 从row_expression获取的每个child节点中选取命名空间为'nspace1'的name节点，并将节点中的值转换为varchar(10)返回；其中ns1为'nspace1'的别名 age int);

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . reason ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) );

INSERT INTO tpcds . reason values ( 3 , 'AAAAAAAABAAAAAAA' , 'reason 1' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 2' ),( 4 , 'AAAAAAAABAAAAAAA' , 'reason 3' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 4' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 5' ),( 20 , 'AAAAAAAACAAAAAAA' , 'N%reason 6' ),( 30 , 'AAAAAAAACAAAAAAA' , 'W%reason 7' );

WITH temp_t ( name , isdba ) AS ( SELECT usename , usesuper FROM pg_user ) SELECT * FROM temp_t ;

SELECT DISTINCT ( r_reason_sk ) FROM tpcds . reason ;

SELECT * FROM tpcds . reason LIMIT 1 ;

SELECT r_reason_desc FROM tpcds . reason ORDER BY r_reason_desc ;

SELECT a . usename , b . locktime FROM pg_user a , pg_user_status b WHERE a . usesysid = b . roloid ;

SELECT a . usename , b . locktime , a . usesuper FROM pg_user a FULL JOIN pg_user_status b on a . usesysid = b . roloid ;

SELECT r_reason_id , AVG ( r_reason_sk ) FROM tpcds . reason GROUP BY r_reason_id HAVING AVG ( r_reason_sk ) > 25 ;

SELECT r_reason_id , AVG ( r_reason_sk ) FROM tpcds . reason GROUP BY CUBE ( r_reason_id , r_reason_sk );

SELECT r_reason_id , AVG ( r_reason_sk ) FROM tpcds . reason GROUP BY GROUPING SETS (( r_reason_id , r_reason_sk ), r_reason_sk );

SELECT r_reason_sk , tpcds . reason . r_reason_desc FROM tpcds . reason WHERE tpcds . reason . r_reason_desc LIKE 'W%' UNION SELECT r_reason_sk , tpcds . reason . r_reason_desc FROM tpcds . reason WHERE tpcds . reason . r_reason_desc LIKE 'N%' ;

SELECT * FROM tpcds . reason ORDER BY NLSSORT ( r_reason_desc , 'NLS_SORT = SCHINESE_PINYIN_M' );

SELECT * FROM tpcds . reason ORDER BY NLSSORT ( r_reason_desc , 'NLS_SORT = generic_m_ci' );

CREATE TABLE tpcds . reason_p ( r_reason_sk integer , r_reason_id character ( 16 ), r_reason_desc character ( 100 ) ) PARTITION BY RANGE ( r_reason_sk ) ( partition P_05_BEFORE values less than ( 05 ), partition P_15 values less than ( 15 ), partition P_25 values less than ( 25 ), partition P_35 values less than ( 35 ), partition P_45_AFTER values less than ( MAXVALUE ) );

INSERT INTO tpcds . reason_p values ( 3 , 'AAAAAAAABAAAAAAA' , 'reason 1' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 2' ),( 4 , 'AAAAAAAABAAAAAAA' , 'reason 3' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 4' ),( 10 , 'AAAAAAAABAAAAAAA' , 'reason 5' ),( 20 , 'AAAAAAAACAAAAAAA' , 'reason 6' ),( 30 , 'AAAAAAAACAAAAAAA' , 'reason 7' );

SELECT * FROM tpcds . reason_p PARTITION ( P_05_BEFORE );

SELECT * FROM tpcds . reason_p PARTITION ( P_05_BEFORE , P_15 , P_25 ) ORDER BY 1 ;

SELECT COUNT ( * ), r_reason_id FROM tpcds . reason_p GROUP BY r_reason_id ;

SELECT * FROM tpcds . reason GROUP BY CUBE ( r_reason_id , r_reason_sk , r_reason_desc );

SELECT * FROM tpcds . reason GROUP BY GROUPING SETS (( r_reason_id , r_reason_sk ), r_reason_desc );

SELECT COUNT ( * ) c , r_reason_id FROM tpcds . reason_p GROUP BY r_reason_id HAVING c > 2 ;

SELECT COUNT ( * ), r_reason_id FROM tpcds . reason_p GROUP BY r_reason_id HAVING r_reason_id IN ( 'AAAAAAAABAAAAAAA' , 'AAAAAAAADAAAAAAA' );

SELECT * FROM tpcds . reason_p WHERE r_reason_id = 'AAAAAAAABAAAAAAA' INTERSECT SELECT * FROM tpcds . reason_p WHERE r_reason_sk < 5 ;

SELECT * FROM tpcds . reason_p WHERE r_reason_id = 'AAAAAAAABAAAAAAA' EXCEPT SELECT * FROM tpcds . reason_p WHERE r_reason_sk < 4 ;

CREATE TABLE tpcds . store_returns ( sr_item_sk int , sr_customer_id varchar ( 50 ), sr_customer_sk int );

CREATE TABLE tpcds . customer ( c_item_sk int , c_customer_id varchar ( 50 ), c_customer_sk int );

SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE t1 . sr_customer_sk = t2 . c_customer_sk ( + ) ORDER BY 1 DESC LIMIT 1 ;

SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE t1 . sr_customer_sk ( + ) = t2 . c_customer_sk ORDER BY 1 DESC LIMIT 1 ;

SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE t1 . sr_customer_sk = t2 . c_customer_sk ( + ) AND t2 . c_customer_sk ( + ) < 1 ORDER BY 1 LIMIT 1 ;

SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE NOT ( t1 . sr_customer_sk = t2 . c_customer_sk ( + ) AND t2 . c_customer_sk ( + ) < 1 );

SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE ( t1 . sr_customer_sk = t2 . c_customer_sk ( + )):: bool ;

SELECT t1 . sr_item_sk , t2 . c_customer_id FROM tpcds . store_returns t1 , tpcds . customer t2 WHERE t1 . sr_customer_sk ( + ) = t2 . c_customer_sk ( + );

DROP TABLE tpcds.reason_p;

WITH RECURSIVE t1(a) as ( select 100 ), t(n) AS ( VALUES (1) UNION ALL SELECT n+1 FROM t WHERE n < (select max(a) from t1) ) SELECT sum(n) FROM t;

CREATE DATABASE pivot_db dbcompatibility ' ORA ';

DROP DATABASE pivot_db;

CREATE TABLE skiplocked_astore(id int, info text) WITH (storage_type=astore);

INSERT INTO skiplocked_astore VALUES (1, ' abc '), (2, ' bcd '), (3, ' cdf '),(3, ' dfg ' );

BEGIN ;

SELECT * FROM skiplocked_astore WHERE id = 1 FOR UPDATE ;

SELECT * FROM skiplocked_astore FOR UPDATE SKIP LOCKED ;

DROP SCHEMA tpcds CASCADE ;

