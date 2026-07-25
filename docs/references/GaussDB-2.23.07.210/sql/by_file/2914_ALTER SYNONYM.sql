-- 来源: 2914_ALTER SYNONYM.txt
-- SQL 数量: 9

CREATE USER sysadmin WITH SYSADMIN PASSWORD '********';

--创建同义词t1。
CREATE OR REPLACE SYNONYM t1 FOR ot.t1;

--创建新用户u1。
CREATE USER u1 PASSWORD '********';

--给新用户赋权限
GRANT ALL ON SCHEMA sysadmin TO u1;

--修改同义词t1的owner为u1。
ALTER SYNONYM t1 OWNER TO u1;

--删除同义词t1。
DROP SYNONYM t1;

--收回用户u1权限
REVOKE ALL ON SCHEMA sysadmin FROM u1;

--删除用户u1。
DROP USER u1;

--删除用户sysadmin。
DROP USER sysadmin;

