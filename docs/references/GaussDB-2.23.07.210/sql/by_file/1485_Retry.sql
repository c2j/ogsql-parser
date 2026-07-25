-- 来源: 1485_Retry.txt
-- SQL 数量: 2

CREATE OR REPLACE PROCEDURE retry_basic ( IN x INT ) AS BEGIN INSERT INTO t1 ( a ) VALUES ( x );

CALL retry_basic ( 1 );

