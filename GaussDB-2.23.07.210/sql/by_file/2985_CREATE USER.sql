-- 来源: 2985_CREATE USER.txt
-- SQL 数量: 11

CREATE USER jim PASSWORD ' ******** ';

--下面语句与上面的等价。
CREATE USER kim IDENTIFIED BY ' ******** ';

--如果创建有“创建数据库”权限的用户，则需要加CREATEDB关键字。
CREATE USER dim CREATEDB PASSWORD ' ******** ';

--将用户jim的登录密码由 ******** 修改为**********。
ALTER USER jim IDENTIFIED BY '**********' REPLACE ' ******** ';

--为用户jim追加CREATEROLE权限。
ALTER USER jim CREATEROLE;

--将enable_seqscan的值设置为on， 设置成功后，在下一会话中生效。
ALTER USER jim SET enable_seqscan TO on;

--重置jim的enable_seqscan参数。
ALTER USER jim RESET enable_seqscan;

--锁定jim账户。
ALTER USER jim ACCOUNT LOCK;

--删除用户。
DROP USER kim CASCADE;

DROP USER jim CASCADE;

DROP USER dim CASCADE;

