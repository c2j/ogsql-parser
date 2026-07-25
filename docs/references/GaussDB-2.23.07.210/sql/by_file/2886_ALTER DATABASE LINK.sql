-- 来源: 2886_ALTER DATABASE LINK.txt
-- SQL 数量: 9

CREATE USER user01 WITH SYSADMIN PASSWORD '********';

SET ROLE user01 PASSWORD '********';

--创建公共dblink
CREATE PUBLIC DATABASE LINK public_dblink CONNECT TO 'user1' IDENTIFIED BY '********' USING (host '192.168.11.11',port '54399',dbname 'db01');

--创建普通用户
CREATE USER user2 PASSWORD '********';

-- 修改dblink对象信息
ALTER PUBLIC DATABASE LINK public_dblink CONNECT TO 'user2' IDENTIFIED BY '********';

--删除公共dblink
DROP PUBLIC DATABASE LINK public_dblink;

--删除创建出的用户
RESET ROLE;

DROP USER user01 CASCADE;

DROP USER user02;

