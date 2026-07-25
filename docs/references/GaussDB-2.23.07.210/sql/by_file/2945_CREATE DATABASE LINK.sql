-- 来源: 2945_CREATE DATABASE LINK.txt
-- SQL 数量: 8

CREATE USER user01 WITH SYSADMIN PASSWORD '********';

SET ROLE user01 PASSWORD '********';

--创建私有dblink
CREATE DATABASE LINK private_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

--删除私有dblink
DROP DATABASE LINK private_dblink;

--创建公共dblink
CREATE PUBLIC DATABASE LINK public_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

--删除公共dblink
DROP PUBLIC DATABASE LINK public_dblink;

--删除创建出的用户
RESET ROLE;

DROP USER user01 CASCADE;

