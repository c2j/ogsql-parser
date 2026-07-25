-- 来源: 2887_ALTER DATA SOURCE.txt
-- SQL 数量: 11

CREATE DATA SOURCE ds_test1;

--修改名称。
ALTER DATA SOURCE ds_test1 RENAME TO ds_test;

--创建用户和修改所有者。
CREATE USER user_test1 IDENTIFIED BY '********';

ALTER USER user_test1 WITH SYSADMIN;

ALTER DATA SOURCE ds_test OWNER TO user_test1;

--修改TYPE和VERSION。
ALTER DATA SOURCE ds_test TYPE 'MPPDB_TYPE' VERSION 'XXX';

--添加字段。
ALTER DATA SOURCE ds_test OPTIONS (add dsn ' gaussdb ', username 'test_user');

--修改字段。
ALTER DATA SOURCE ds_test OPTIONS (set dsn 'unknown');

--删除字段。
ALTER DATA SOURCE ds_test OPTIONS (drop username);

--删除Data Source和user对象。
DROP DATA SOURCE ds_test;

DROP USER user_test1;

