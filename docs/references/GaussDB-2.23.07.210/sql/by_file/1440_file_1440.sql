-- 来源: 1440_file_1440.txt
-- SQL 数量: 3

CREATE OR REPLACE PROCEDURE proc_add ( param1 in INTEGER , param2 out INTEGER , param3 in INTEGER ) AS BEGIN param2 : = param1 + param3 ;

DECLARE input1 INTEGER : = 1 ;

DROP PROCEDURE proc_add ;

