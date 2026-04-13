-- 来源: 3065_PREPARE TRANSACTION.txt
-- SQL 数量: 3

BEGIN;

--准备标识符为的trans_test的事务。
PREPARE TRANSACTION 'trans_test';

--取消标识符为的trans_test的事务。
ROLLBACK PREPARED 'trans_test';

