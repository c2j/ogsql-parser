-- 来源: 2969_CREATE RULE.txt
-- SQL 数量: 6

CREATE TABLE def_test ( c1 int4 DEFAULT 5, c2 text DEFAULT 'initial_default' );

CREATE VIEW def_view_test AS SELECT * FROM def_test;

--创建RULE def_view_test_ins。
CREATE RULE def_view_test_ins AS ON INSERT TO def_view_test DO INSTEAD INSERT INTO def_test SELECT new.*;

--删除RULE def_view_test_ins。
DROP RULE def_view_test_ins ON def_view_test;

--删除表def_test、视图def_view_test。
DROP VIEW def_view_test;

DROP TABLE def_test;

