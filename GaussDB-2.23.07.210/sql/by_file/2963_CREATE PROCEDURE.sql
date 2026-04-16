-- 来源: 2963_CREATE PROCEDURE.txt
-- SQL 数量: 13

CREATE OR REPLACE PROCEDURE prc_add ( param1 IN INTEGER , param2 IN OUT INTEGER ) AS BEGIN param2 : = param1 + param2 ;

SELECT prc_add ( 2 , 3 );

CREATE OR REPLACE PROCEDURE pro_variadic ( var1 VARCHAR2 ( 10 ) DEFAULT 'hello!' , var4 VARIADIC int4 []) AS BEGIN dbe_output . print_line ( var1 );

SELECT pro_variadic ( var1 => 'hello' , VARIADIC var4 => array [ 1 , 2 , 3 , 4 ]);

CREATE TABLE tb1 ( a integer );

CREATE PROCEDURE insert_data ( v integer ) SECURITY INVOKER AS BEGIN INSERT INTO tb1 VALUES ( v );

CALL insert_data ( 1 );

CREATE OR REPLACE PROCEDURE package_func_overload ( col int , col2 out varchar ) package as declare col_type text ;

DROP PROCEDURE prc_add ;

DROP PROCEDURE pro_variadic ;

DROP PROCEDURE insert_data ;

DROP PROCEDURE package_func_overload ;

DROP TABLE tb1 ;

