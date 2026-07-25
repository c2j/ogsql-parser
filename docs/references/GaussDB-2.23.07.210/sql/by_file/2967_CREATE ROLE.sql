-- 来源: 2967_CREATE ROLE.txt
-- SQL 数量: 6

CREATE ROLE manager IDENTIFIED BY ' ******** ';

--创建一个角色，从2015年1月1日开始生效，到2026年1月1日失效。
CREATE ROLE miriam WITH LOGIN PASSWORD ' ******** ' VALID BEGIN '2015-01-01' VALID UNTIL '2026-01-01';

--修改角色manager的密码为********。
ALTER ROLE manager IDENTIFIED BY '********' REPLACE ' ********** ';

--修改角色manager为系统管理员。
ALTER ROLE manager SYSADMIN;

--删除角色manager。
DROP ROLE manager;

--删除角色miriam。
DROP GROUP miriam;

