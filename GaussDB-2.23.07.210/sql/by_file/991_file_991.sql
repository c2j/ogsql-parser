-- 来源: 991_file_991.txt
-- SQL 数量: 33

explain select * from t where c1 > 1;

explain select * from t limit 1;

explain select sum(c1), count(*) from t;

create table t(c1 int, c2 int, c3 int)distribute by hash(c1);

create table t1(c1 int, c2 int, c3 int)distribute by hash(c1);

explain select * from t1 join t on t.c1 = t1.c1;

explain select * from t1 join t on t.c1 = t1.c2;

CREATE NODE GROUP ng WITH(datanode1, datanode2, datanode3, datanode4, datanode5, datanode6);

CREATE TABLE t1(a int, b int, c int) DISTRIBUTE BY HASH(a) TO GROUP ng;

CREATE TABLE t2(a int, b int, c int) DISTRIBUTE BY HASH(a) TO GROUP ng;

EXPLAIN (COSTS OFF) SELECT * FROM t1 UNION ALL SELECT * FROM t2;

EXPLAIN (COSTS OFF) SELECT * FROM t1 UNION SELECT * FROM t2;

EXPLAIN (COSTS OFF) SELECT * FROM t1 WHERE a = 1 UNION SELECT * FROM t2 WHERE a = 1;

EXPLAIN (COSTS OFF) SELECT * FROM t1 WHERE a = 1;

EXPLAIN (COSTS OFF) SELECT * FROM t2 WHERE a = 3;

EXPLAIN (COSTS OFF) SELECT * FROM t1 WHERE a = 1 UNION SELECT * FROM t2 WHERE a = 3;

EXPLAIN ( COSTS OFF ) WITH cte AS ( SELECT * FROM t1 ) SELECT * FROM cte ;

EXPLAIN ( COSTS OFF ) WITH cte AS ( SELECT * FROM t1 WHERE a = 1 ) SELECT * FROM cte ;

EXPLAIN ( COSTS OFF ) WITH cte AS ( SELECT * FROM t1 ORDER BY a ) SELECT * FROM cte ;

EXPLAIN ( COSTS OFF ) WITH RECURSIVE cte AS ( SELECT * FROM t1 UNION ALL SELECT t2 . * FROM t2 , cte WHERE cte . a = t2 . a ) SELECT * FROM cte ;

EXPLAIN ( COSTS OFF ) WITH RECURSIVE cte AS ( SELECT * FROM t1 WHERE a = 1 UNION ALL SELECT t2 . * FROM t2 , cte WHERE cte . a = t2 . a AND t2 . a = 1 ) SELECT * FROM cte ;

EXPLAIN ( COSTS OFF ) WITH RECURSIVE cte AS ( SELECT * FROM t1 UNION SELECT t2 . * FROM t2 , cte WHERE cte . a = t2 . a ) SELECT * FROM cte ;

DROP TABLE t1 ;

DROP TABLE t2 ;

DROP NODE GROUP ng ;

CREATE TABLE CUSTOMER1 ( C_CUSTKEY BIGINT NOT NULL , C_NAME VARCHAR ( 25 ) NOT NULL , C_ADDRESS VARCHAR ( 40 ) NOT NULL , C_NATIONKEY INT NOT NULL , C_PHONE CHAR ( 15 ) NOT NULL , C_ACCTBAL DECIMAL ( 15 , 2 ) NOT NULL , C_MKTSEGMENT CHAR ( 10 ) NOT NULL , C_COMMENT VARCHAR ( 117 ) NOT NULL ) DISTRIBUTE BY hash ( C_CUSTKEY );

CREATE TABLE test_stream ( a int , b float );

CREATE TABLE sal_emp ( c1 integer [] ) DISTRIBUTE BY replication ;

explain update customer1 set C_NAME = 'a' returning c_name ;

explain verbose select count ( c_custkey order by c_custkey ) from customer1 ;

explain verbose select count ( distinct b ) from test_stream ;

explain verbose select distinct on ( c_custkey ) c_custkey from customer1 order by c_custkey ;

explain verbose select array [ c_custkey , 1 ] from customer1 order by c_custkey ;

