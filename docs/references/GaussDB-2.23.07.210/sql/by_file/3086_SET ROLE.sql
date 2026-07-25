-- 来源: 3086_SET ROLE.txt
-- SQL 数量: 3

CREATE ROLE paul IDENTIFIED BY ' ******** ';

--设置当前用户为paul。
SET ROLE paul PASSWORD ' ******** ';

--删除用户。
DROP USER paul;

