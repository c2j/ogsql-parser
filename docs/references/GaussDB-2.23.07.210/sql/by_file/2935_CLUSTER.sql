-- 来源: 2935_CLUSTER.txt
-- SQL 数量: 20

CREATE TABLE test_c1 ( id int , name varchar ( 20 ));

CREATE INDEX idx_test_c1_id ON test_c1 ( id );

INSERT INTO test_c1 VALUES ( 3 , 'Joe' ),( 1 , 'Jack' ),( 2 , 'Scott' );

SELECT * FROM test_c1 ;

CLUSTER test_c1 USING idx_test_c1_id ;

SELECT * FROM test_c1 ;

DROP TABLE test_c1 ;

CREATE TABLE test(col1 int,CONSTRAINT pk_test PRIMARY KEY (col1));

-- 第一次聚簇排序不带USING关键字报错
CLUSTER test;

-- 聚簇排序
CLUSTER test USING pk_test;

--对已做过聚簇的表重新进行聚簇
CLUSTER VERBOSE test;

-- 删除
DROP TABLE test;

CREATE TABLE test_c2(id int, info varchar(4)) PARTITION BY RANGE (id)( PARTITION p1 VALUES LESS THAN (11), PARTITION p2 VALUES LESS THAN (21) );

CREATE INDEX idx_test_c2_id1 ON test_c2(id);

INSERT INTO test_c2 VALUES (6,'ABBB'),(2,'ABAB'),(9,'AAAA');

INSERT INTO test_c2 VALUES (11,'AAAB'),(19,'BBBA'),(16,'BABA');

-- 查看
SELECT * FROM test_c2;

-- 对分区p2进行聚簇排序
CLUSTER test_c2 PARTITION (p2) USING idx_test_c2_id1;

-- 查看
SELECT * FROM test_c2;

-- 删除
DROP TABLE test_c2;

