-- 来源: 3070_REFRESH MATERIALIZED VIEW.txt
-- SQL 数量: 9

CREATE TABLE my_table (c1 int, c2 int) WITH(STORAGE_TYPE=ASTORE);

--创建全量物化视图。
CREATE MATERIALIZED VIEW my_mv AS SELECT * FROM my_table;

--创建增量物化视图。
CREATE INCREMENTAL MATERIALIZED VIEW my_imv AS SELECT * FROM my_table;

--基表写入数据。
INSERT INTO my_table VALUES(1,1),(2,2);

--对全量物化视图my_mv进行全量刷新。
REFRESH MATERIALIZED VIEW my_mv;

--对增量物化视图my_imv进行全量刷新。
REFRESH MATERIALIZED VIEW my_imv;

--删除增量物化视图。
DROP MATERIALIZED VIEW my_imv;

--删除全量物化视图。
DROP MATERIALIZED VIEW my_mv;

--删除表my_table。
DROP TABLE my_table;

