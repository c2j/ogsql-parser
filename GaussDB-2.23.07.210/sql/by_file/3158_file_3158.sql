-- 来源: 3158_file_3158.txt
-- SQL 数量: 3

CREATE OR REPLACE PROCEDURE proc_case_branch ( pi_result in integer , pi_return out integer ) AS BEGIN CASE pi_result WHEN 1 THEN pi_return : = 111 ;

CALL proc_case_branch ( 3 , 0 );

DROP PROCEDURE proc_case_branch ;

