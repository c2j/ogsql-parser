-- 来源: 3093_SNAPSHOT.txt
-- SQL 数量: 14

CREATE TABLE t1 (id int, name varchar);

INSERT INTO t1 VALUES (1, 'zhangsan');

INSERT INTO t1 VALUES (2, 'lisi');

INSERT INTO t1 VALUES (3, 'wangwu');

INSERT INTO t1 VALUES (4, 'lisa');

INSERT INTO t1 VALUES (5, 'jack');

CREATE SNAPSHOT s1@1.0 comment is 'first version' AS SELECT * FROM t1;

CREATE SNAPSHOT s1@2.0 FROM @1.0 comment is 'inherits from @1.0' USING (INSERT VALUES(6, 'john'), (7, 'tim');

SELECT * FROM DB4AISHOT(s1@1.0);

SAMPLE SNAPSHOT s1@2.0 stratify by name as nick at ratio .5;

PURGE SNAPSHOT s1@2.0;

PURGE SNAPSHOT s1nick@2.0;

PURGE SNAPSHOT s1@1.0;

DROP TABLE t1;

