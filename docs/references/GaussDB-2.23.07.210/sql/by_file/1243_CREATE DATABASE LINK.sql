-- 来源: 1243_CREATE DATABASE LINK.txt
-- SQL 数量: 8

CREATE USER user01 WITH SYSADMIN PASSWORD '********';

SET ROLE user01 PASSWORD '********';

CREATE DATABASE LINK private_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

DROP DATABASE LINK private_dblink;

CREATE PUBLIC DATABASE LINK public_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

DROP PUBLIC DATABASE LINK public_dblink;

RESET ROLE;

DROP USER user01 CASCADE;

