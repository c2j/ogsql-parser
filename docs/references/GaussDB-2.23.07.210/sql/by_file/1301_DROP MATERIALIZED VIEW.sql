-- 来源: 1301_DROP MATERIALIZED VIEW.txt
-- SQL 数量: 4

CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建名为my_mv的物化视图。
CREATE MATERIALIZED VIEW my_mv AS SELECT * FROM my_table;

--删除名为my_mv的物化视图。
DROP MATERIALIZED VIEW my_mv;

--删除表。
DROP TABLE my_table;

