-- 来源: 2909_ALTER SCHEMA.txt
-- SQL 数量: 11

CREATE DATABASE test1 WITH DBCOMPATIBILITY = 'b';

--创建模式ds。
CREATE SCHEMA ds;

--将当前模式ds更名为ds_new。
ALTER SCHEMA ds RENAME TO ds_new;

--创建用户jack。
CREATE USER jack PASSWORD ' ******** ';

--将DS_NEW的所有者修改为jack。
ALTER SCHEMA ds_new OWNER TO jack;

--将sch的默认字符集修改为utf8mb4，默认字符序修改为utf8mb4_bin。仅在B模式下（即sql_compatibility='B'）支持该语法。
CREATE SCHEMA sch;

ALTER SCHEMA sch CHARACTER SET utf8mb4 COLLATE utf8mb4_bin;

--删除模式ds_new和sch。
DROP SCHEMA ds_new;

DROP SCHEMA sch;

--删除用户jack。
DROP USER jack;

DROP DATABASE test1;

