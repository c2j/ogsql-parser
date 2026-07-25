-- 来源: 4493_file_4493.txt
-- SQL 数量: 10

CREATE TABLE t1(c1 int, c2 int);

INSERT INTO t1 VALUES(1, 1);

INSERT INTO t1 VALUES(2, 2);

--创建全量物化视图。
CREATE MATERIALIZED VIEW mv AS select count(*) from t1;

--查询物化视图结果。
SELECT * FROM mv;

--向物化视图中基表插入数据。
INSERT INTO t1 VALUES(3, 3);

--对全量物化视图做全量刷新。
REFRESH MATERIALIZED VIEW mv;

--查询物化视图结果。
SELECT * FROM mv;

--删除物化视图，删除表。
DROP MATERIALIZED VIEW mv;

DROP TABLE t1;

