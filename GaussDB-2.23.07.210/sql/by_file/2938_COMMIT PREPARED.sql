-- 来源: 2938_COMMIT PREPARED.txt
-- SQL 数量: 5

BEGIN;

--准备标识符为的trans_test的事务。
PREPARE TRANSACTION 'trans_test';

--创建表。
CREATE TABLE item1(id int);

--提交标识符为的trans_test的事务。
COMMIT PREPARED 'trans_test';

--删除表。
DROP TABLE item1;

