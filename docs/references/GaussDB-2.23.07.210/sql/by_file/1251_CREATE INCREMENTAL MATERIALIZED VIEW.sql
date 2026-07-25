-- 来源: 1251_CREATE INCREMENTAL MATERIALIZED VIEW.txt
-- SQL 数量: 6

CREATE TABLE my_table (c1 int, c2 int);

--创建增量物化视图。
CREATE INCREMENTAL MATERIALIZED VIEW my_imv AS SELECT * FROM my_table;

--基表写入数据。
INSERT INTO my_table VALUES(1,1),(2,2);

--对增量物化视图my_imv进行增量刷新。
REFRESH INCREMENTAL MATERIALIZED VIEW my_imv;

--删除增量物化视图。
DROP MATERIALIZED VIEW my_imv;

--删除普通表my_table。
DROP TABLE my_table;

