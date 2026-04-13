-- 来源: 1229_CALL.txt
-- SQL 数量: 9

CREATE FUNCTION func_add_sql ( num1 integer , num2 integer ) RETURN integer AS BEGIN RETURN num1 + num2 ;

CALL func_add_sql ( 1 , 3 );

CALL func_add_sql ( num1 => 1 , num2 => 3 );

CALL func_add_sql ( num2 : = 2 , num1 : = 3 );

DROP FUNCTION func_add_sql ;

CREATE FUNCTION func_increment_sql ( num1 IN integer , num2 IN integer , res OUT integer ) RETURN integer AS BEGIN res : = num1 + num2 ;

CALL func_increment_sql ( 1 , 2 , 1 );

DECLARE res int ;

DROP FUNCTION func_increment_sql ;

