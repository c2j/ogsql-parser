-- 来源: 2979_CREATE TABLESPACE.txt
-- SQL 数量: 10

CREATE TABLESPACE ds_location1 RELATIVE LOCATION 'tablespace/tablespace_1';

--创建用户joe。
CREATE ROLE joe IDENTIFIED BY ' ******** ';

--创建用户jay。
CREATE ROLE jay IDENTIFIED BY ' ******** ';

--创建表空间，且所有者指定为用户joe。
CREATE TABLESPACE ds_location2 OWNER joe RELATIVE LOCATION 'tablespace/tablespace_2';

--把表空间ds_location1重命名为ds_location3。
ALTER TABLESPACE ds_location1 RENAME TO ds_location3;

--改变表空间ds_location2的所有者。
ALTER TABLESPACE ds_location2 OWNER TO jay;

--删除表空间。
DROP TABLESPACE ds_location2;

DROP TABLESPACE ds_location3;

--删除用户。
DROP ROLE joe;

DROP ROLE jay;

