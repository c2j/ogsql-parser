-- 来源: 1446_file_1446.txt
-- SQL 数量: 7

DECLARE v_user_id integer default 1 ;

DECLARE v_user_id integer default 0 ;

DECLARE v_user_id integer default 1 ;

DECLARE v_user_id integer default NULL ;

CREATE OR REPLACE PROCEDURE proc_control_structure ( i in integer ) AS BEGIN IF i > 0 THEN raise info 'i:% is greater than 0. ' , i ;

CALL proc_control_structure ( 3 );

DROP PROCEDURE proc_control_structure ;

