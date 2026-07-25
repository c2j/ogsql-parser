-- 来源: 1201_ALTER MATERIALIZED VIEW.txt
-- SQL 数量: 5

CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建全量物化视图。
CREATE MATERIALIZED VIEW foo AS SELECT * FROM my_table;

--把物化视图foo重命名为bar。
ALTER MATERIALIZED VIEW foo RENAME TO bar;

--删除全量物化视图。
DROP MATERIALIZED VIEW bar;

--删除表my_table。
DROP TABLE my_table;

