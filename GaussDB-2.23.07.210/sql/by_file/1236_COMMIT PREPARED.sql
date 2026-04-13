-- 来源: 1236_COMMIT PREPARED.txt
-- SQL 数量: 5

BEGIN ;

PREPARE TRANSACTION 'trans_test' ;

CREATE TABLE item1 ( id int );

COMMIT PREPARED 'trans_test' ;

DROP TABLE item1 ;

