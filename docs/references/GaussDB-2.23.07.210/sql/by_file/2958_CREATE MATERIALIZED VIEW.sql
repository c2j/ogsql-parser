-- 来源: 2958_CREATE MATERIALIZED VIEW.txt
-- SQL 数量: 6

CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建全量物化视图。
CREATE MATERIALIZED VIEW my_mv AS SELECT * FROM my_table;

--基表写入数据。
INSERT INTO my_table VALUES(1,1),(2,2);

--对全量物化视图my_mv进行全量刷新。
REFRESH MATERIALIZED VIEW my_mv;

--删除全量物化视图。
DROP MATERIALIZED VIEW my_mv;

--删除普通表my_table。
DROP TABLE my_table;

