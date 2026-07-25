-- 来源: 2841_UNIONCASE.txt
-- SQL 数量: 11

SELECT text 'a' AS "text" UNION SELECT 'b' ;

SELECT 1 . 2 AS "numeric" UNION SELECT 1 ;

SELECT 1 AS "real" UNION SELECT CAST ( '2.2' AS REAL );

CREATE DATABASE a_1 dbcompatibility = 'A' ;

\ c a_1 --创建表t1。 a_1 =# CREATE TABLE t1 ( a int , b varchar ( 10 ));

CREATE DATABASE td_1 dbcompatibility = 'C' ;

\ c td_1 --创建表t2。 td_1 =# CREATE TABLE t2 ( a int , b varchar ( 10 ));

DROP DATABASE a_1 ;

DROP DATABASE td_1 ;

CREATE DATABASE ora_1 dbcompatibility = 'A';

--删除ORA模式的数据库。
DROP DATABASE ora_1;

