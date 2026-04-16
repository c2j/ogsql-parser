-- 来源: 4277_file_4277.txt
-- SQL 数量: 9

-- 创建增量物化视图
CREATE INCREMENTAL MATERIALIZED VIEW mv AS SELECT * FROM t1;

-- 插入数据
INSERT INTO t1 VALUES(3, 3);

-- 增量刷新物化视图
REFRESH INCREMENTAL MATERIALIZED VIEW mv;

-- 查询物化视图结果
SELECT * FROM mv;

-- 插入数据
INSERT INTO t1 VALUES(4, 4);

-- 全量刷新物化视图
REFRESH MATERIALIZED VIEW mv;

-- 查询物化视图结果
select * from mv;

-- 删除物化视图，删除表
DROP MATERIALIZED VIEW mv;

DROP TABLE t1;

