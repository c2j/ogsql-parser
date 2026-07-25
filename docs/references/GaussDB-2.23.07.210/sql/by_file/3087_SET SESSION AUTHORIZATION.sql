-- 来源: 3087_SET SESSION AUTHORIZATION.txt
-- SQL 数量: 3

CREATE ROLE paul IDENTIFIED BY ' ******** ';

--设置当前用户为paul。
SET SESSION AUTHORIZATION paul password ' ******** ';

--删除用户。
DROP USER paul;

