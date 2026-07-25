-- 来源: 3082_SELECT.txt
-- SQL 数量: 71

--创建自定义变量
CREATE DATABASE user_var dbcompatibility 'b';

--删除数据库
DROP DATABASE user_var;

SELECT * FROM XMLTABLE( XMLNAMESPACES('nspace1' AS "ns1", 'nspace2' AS "ns2"), -- 声明两个XML的命名空间'nspace1'和'nspace2'及对应的别名"ns1"和"ns2" '/ns1:root/*:child' -- 经row_expression从传入的数据中选取命名空间为'nspace1'的root节点，在选取其下面的所有child节点，忽略child的命名空间；其中ns1为'nspace1'的别名 PASSING xmltype( '<root xmlns="nspace1"> <child> <name>peter</name> <age>11</age> </child> <child xmlns="nspace1"> <name>qiqi</name> <age>12</age> </child> <child xmlns="nspace2"> <name>hacker</name> <age>15</age> </child> </root>') COLUMNS column FOR ORDINALITY, -- 该列为行号列 name varchar(10) path 'ns1:name', -- 从row_expression获取的每个child节点中选取命名空间为'nspace1'的name节点，并将节点中的值转换为varchar(10)返回；其中ns1为'nspace1'的别名 age int);

CREATE TABLE test(name varchar, id int, fatherid int);

INSERT INTO test VALUES('A', 1, 0), ('B', 2, 1),('C',3,1),('D',4,1),('E',5,2);

SELECT * FROM TEST START WITH id = 1 CONNECT BY prior id = fatherid ORDER SIBLINGS BY id DESC;

CREATE SCHEMA tpcds;

--创建表tpcds.reason。
CREATE TABLE tpcds.reason ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) );

--向表中插入多条记录。
INSERT INTO tpcds.reason values(3,'AAAAAAAABAAAAAAA','reason 1'),(10,'AAAAAAAABAAAAAAA','reason 2'),(4,'AAAAAAAABAAAAAAA','reason 3'),(10,'AAAAAAAABAAAAAAA','reason 4'),(10,'AAAAAAAABAAAAAAA','reason 5'),(20,'AAAAAAAACAAAAAAA','N%reason 6'),(30,'AAAAAAAACAAAAAAA','W%reason 7');

--先通过子查询得到一张临时表temp_t，然后查询表temp_t中的所有数据。
WITH temp_t(name,isdba) AS (SELECT usename,usesuper FROM pg_user) SELECT * FROM temp_t;

--查询 tpcds. reason表的所有r_reason_sk记录，且去除重复。
SELECT DISTINCT(r_reason_sk) FROM tpcds. reason;

--LIMIT子句示例：获取表中一条记录。
SELECT * FROM tpcds. reason LIMIT 1;

--查询所有记录，且按字母升序排列。
SELECT r_reason_desc FROM tpcds. reason ORDER BY r_reason_desc;

--通过表别名，从pg_user和pg_user_status这两张表中获取数据。
SELECT a.usename,b.locktime FROM pg_user a,pg_user_status b WHERE a.usesysid=b.roloid;

--FULL JOIN子句示例：将pg_user和pg_user_status这两张表的数据进行全连接显示，即数据的合集。
SELECT a.usename,b.locktime,a.usesuper FROM pg_user a FULL JOIN pg_user_status b ON a.usesysid=b.roloid;

--GROUP BY子句示例：根据查询条件过滤，并对结果进行分组。
SELECT r_reason_id, AVG(r_reason_sk) FROM tpcds. reason GROUP BY r_reason_id HAVING AVG(r_reason_sk) > 25;

--GROUP BY CUBE子句示例：根据查询条件过滤，并对结果进行分组汇总。
SELECT r_reason_id,AVG(r_reason_sk) FROM tpcds. reason GROUP BY CUBE(r_reason_id,r_reason_sk);

--GROUP BY GROUPING SETS子句示例:根据查询条件过滤，并对结果进行分组汇总。
SELECT r_reason_id,AVG(r_reason_sk) FROM tpcds. reason GROUP BY GROUPING SETS((r_reason_id,r_reason_sk),r_reason_sk);

--UNION子句示例：将表 tpcds. reason里r_reason_desc字段中的内容以W开头和以N开头的进行合并。
SELECT r_reason_sk, tpcds. reason.r_reason_desc FROM tpcds. reason WHERE tpcds. reason.r_reason_desc LIKE 'W%' UNION SELECT r_reason_sk, tpcds. reason.r_reason_desc FROM tpcds. reason WHERE tpcds. reason.r_reason_desc LIKE 'N%';

--NLS_SORT子句示例：中文拼音排序。
SELECT * FROM tpcds. reason ORDER BY NLSSORT( r_reason_desc, 'NLS_SORT = SCHINESE_PINYIN_M');

--不区分大小写排序（可选，仅支持纯英文不区分大小写排序）:
SELECT * FROM tpcds. reason ORDER BY NLSSORT( r_reason_desc, 'NLS_SORT = generic_m_ci');

--创建分区表 tpcds. reason_p
CREATE TABLE tpcds. reason_p ( r_reason_sk integer, r_reason_id character(16), r_reason_desc character(100) ) PARTITION BY RANGE (r_reason_sk) ( partition P_05_BEFORE values less than (05), partition P_15 values less than (15), partition P_25 values less than (25), partition P_35 values less than (35), partition P_45_AFTER values less than (MAXVALUE) );

--插入数据。
INSERT INTO tpcds. reason_p values(3,'AAAAAAAABAAAAAAA','reason 1'),(10,'AAAAAAAABAAAAAAA','reason 2'),(4,'AAAAAAAABAAAAAAA','reason 3'),(10,'AAAAAAAABAAAAAAA','reason 4'),(10,'AAAAAAAABAAAAAAA','reason 5'),(20,'AAAAAAAACAAAAAAA','reason 6'),(30,'AAAAAAAACAAAAAAA','reason 7');

--PARTITION子句示例：从 tpcds. reason_p的表分区P_05_BEFORE中获取数据。
SELECT * FROM tpcds. reason_p PARTITION (P_05_BEFORE);

--PARTITION子句指定多分区示例：从 tpcds. reason_p的表分区P_05_BEFORE，P_15，P_25中获取数据。
SELECT * FROM tpcds. reason_p PARTITION (P_05_BEFORE, P_15, P_25) ORDER BY 1;

--GROUP BY子句示例：按r_reason_id分组统计 tpcds. reason_p表中的记录数。
SELECT COUNT(*),r_reason_id FROM tpcds. reason_p GROUP BY r_reason_id;

--GROUP BY CUBE子句示例：根据查询条件过滤，并对查询结果分组汇总。
SELECT * FROM tpcds. reason GROUP BY CUBE (r_reason_id,r_reason_sk,r_reason_desc);

--GROUP BY GROUPING SETS子句示例：根据查询条件过滤，并对查询结果分组汇总。
SELECT * FROM tpcds. reason GROUP BY GROUPING SETS ((r_reason_id,r_reason_sk),r_reason_desc);

--HAVING子句示例：按r_reason_id分组统计 tpcds. reason_p表中的记录，并只显示r_reason_id个数大于2的信息。
SELECT COUNT(*) c,r_reason_id FROM tpcds. reason_p GROUP BY r_reason_id HAVING c>2;

--IN子句示例：按r_reason_id分组统计 tpcds. reason_p表中的r_reason_id个数，并只显示r_reason_id值为 AAAAAAAABAAAAAAA或AAAAAAAADAAAAAAA的个数。
SELECT COUNT(*),r_reason_id FROM tpcds. reason_p GROUP BY r_reason_id HAVING r_reason_id IN('AAAAAAAABAAAAAAA','AAAAAAAADAAAAAAA');

--INTERSECT子句示例：查询r_reason_id等于AAAAAAAABAAAAAAA，并且r_reason_sk小于5的信息。
SELECT * FROM tpcds. reason_p WHERE r_reason_id='AAAAAAAABAAAAAAA' INTERSECT SELECT * FROM tpcds. reason_p WHERE r_reason_sk<5;

--EXCEPT子句示例：查询r_reason_id等于AAAAAAAABAAAAAAA，并且去除r_reason_sk小于4的信息。
SELECT * FROM tpcds. reason_p WHERE r_reason_id='AAAAAAAABAAAAAAA' EXCEPT SELECT * FROM tpcds. reason_p WHERE r_reason_sk<4;

--创建表store_returns、customer
CREATE TABLE tpcds.store_returns (sr_item_sk int, sr_customer_id varchar(50),sr_customer_sk int);

CREATE TABLE tpcds.customer (c_item_sk int, c_customer_id varchar(50),c_customer_sk int);

--通过在where子句中指定"(+)"来实现左连接。
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE t1.sr_customer_sk = t2.c_customer_sk(+) ORDER BY 1 DESC LIMIT 1;

--通过在where子句中指定"(+)"来实现右连接。
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE t1.sr_customer_sk(+) = t2.c_customer_sk ORDER BY 1 DESC LIMIT 1;

--通过在where子句中指定"(+)"来实现左连接，并且增加连接条件。
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE t1.sr_customer_sk = t2.c_customer_sk(+) AND t2.c_customer_sk(+) < 1 ORDER BY 1 LIMIT 1;

--不支持在where子句中指定"(+)"的同时使用内层嵌套AND/OR的表达式。
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE NOT(t1.sr_customer_sk = t2.c_customer_sk(+) AND t2.c_customer_sk(+) < 1);

--where子句在不支持表达式宏指定"(+)"会报错。
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE (t1.sr_customer_sk = t2.c_customer_sk(+))::bool;

--where子句在表达式的两边都指定"(+)"会报错。
SELECT t1.sr_item_sk ,t2.c_customer_id FROM tpcds.store_returns t1, tpcds.customer t2 WHERE t1.sr_customer_sk(+) = t2.c_customer_sk(+);

--删除表。
DROP TABLE tpcds. reason_p;

--闪回查询示例，使用闪回功能需要设置undo_retention_time参数
--创建表tpcds.time_table
CREATE TABLE tpcds.time_table(idx integer, snaptime timestamp, snapcsn bigint, timeDesc character(100));

--向表tpcds.time_table中插入记录
INSERT INTO tpcds.time_table select 1, now(),int8in(xidout(next_csn)), 'time1' from gs_get_next_xid_csn();

INSERT INTO tpcds.time_table select 2, now(),int8in(xidout(next_csn)), 'time2' from gs_get_next_xid_csn();

INSERT INTO tpcds.time_table select 3, now(),int8in(xidout(next_csn)), 'time3' from gs_get_next_xid_csn();

INSERT INTO tpcds.time_table select 4, now(),int8in(xidout(next_csn)), 'time4' from gs_get_next_xid_csn();

SELECT * FROM tpcds.time_table;

DELETE tpcds.time_table;

--2021-04-25 17:50:22.311176应该使用tpcds.time_table中第四条snaptime字段值
SELECT * FROM tpcds.time_table TIMECAPSULE TIMESTAMP to_timestamp('2021-04-25 17:50:22.311176','YYYY-MM-DD HH24:MI:SS.FF');

--107330 csn应该使用tpcds.time_table中第四条snapcsn字段值
SELECT * FROM tpcds.time_table TIMECAPSULE CSN 107330;

--WITH RECURSIVE查询示例：计算从1到100的累加值。
WITH RECURSIVE t1(a) AS ( SELECT 100 ), t(n) AS ( VALUES (1) UNION ALL SELECT n+1 FROM t WHERE n < (SELECT max(a) FROM t1) ) SELECT sum(n) FROM t;

DROP TABLE t ;

--UNPIVOT子句示例：将表p1的math列和phy列转置为（class，score）行
CREATE TABLE p1(id int, math int, phy int);

INSERT INTO p1 values(1,20,30);

INSERT INTO p1 values(2,30,40);

INSERT INTO p1 values(3,40,50);

SELECT * FROM p1;

SELECT * FROM p1 UNPIVOT(score FOR class IN(math, phy));

--PIVOT子句示例：将表p2的（class，score）行转置为'MATH'列和 'PHY'列
CREATE TABLE p2(id int, class varchar(10), score int);

INSERT INTO p2 SELECT * FROM p1 UNPIVOT(score FOR class IN(math, phy));

SELECT * FROM p2;

SELECT * FROM p2 PIVOT(max(score) FOR class IN ('MATH', 'PHY'));

DROP TABLE p1;

DROP TABLE p2;

--SKIP LOCKED示例
--step 1:创建astore表并插入数据
CREATE TABLE skiplocked_astore(id int, info text) WITH (storage_type=astore);

INSERT INTO skiplocked_astore VALUES (1, 'abc'), (2, 'bcd'), (3, 'cdf'),(3, 'dfg');

--step 2:session1开启事务通过UPDATE锁住skiplocked_astore中id等于1的行
BEGIN;

SELECT * FROM skiplocked_astore WHERE id = 1 FOR UPDATE;

--STEP 3:session2 使用SKIP LOCKED会跳过被锁行，仅返回加锁成功的行
SELECT * FROM skiplocked_astore FOR UPDATE SKIP LOCKED;

--删除表。
DROP TABLE tpcds.reason;

--删除SCHEMA。
DROP SCHEMA tpcds CASCADE;

