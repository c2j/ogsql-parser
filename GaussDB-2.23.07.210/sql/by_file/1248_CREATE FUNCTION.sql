-- 来源: 1248_CREATE FUNCTION.txt
-- SQL 数量: 28

CREATE DATABASE ora_compatible_db DBCOMPATIBILITY 'ORA' ;

\ c ora_compatible_db --定义函数为SQL查询。

CREATE FUNCTION func_add_sql ( integer , integer ) RETURNS integer AS 'select $1 + $2;

CREATE OR REPLACE FUNCTION func_increment_plsql ( i integer ) RETURNS integer AS $$ BEGIN RETURN i + 1 ;

CREATE OR REPLACE FUNCTION compute ( i int , out result_1 bigint , out result_2 bigint ) RETURNS SETOF RECORD AS $$ BEGIN result_1 = i + 1 ;

CREATE FUNCTION func_dup_sql ( in int , out f1 int , out f2 text ) AS $$ SELECT $ 1 , CAST ( $ 1 AS text ) || ' is text' $$ LANGUAGE SQL ;

SELECT * FROM func_dup_sql ( 42 );

CREATE FUNCTION func_add_sql2 ( num1 integer , num2 integer ) RETURN integer AS BEGIN RETURN num1 + num2 ;

ALTER FUNCTION func_add_sql2 ( INTEGER , INTEGER ) IMMUTABLE ;

ALTER FUNCTION func_add_sql2 ( INTEGER , INTEGER ) RENAME TO add_two_number ;

CREATE USER jim PASSWORD '********' ;

ALTER FUNCTION add_two_number ( INTEGER , INTEGER ) OWNER TO jim ;

DROP FUNCTION func_add_sql ;

DROP FUNCTION func_increment_plsql ;

DROP FUNCTION compute ;

DROP FUNCTION func_dup_sql ;

DROP FUNCTION add_two_number ;

DROP USER jim ;

CREATE TYPE rec AS ( c1 int , c2 int );

CREATE OR REPLACE FUNCTION func ( a in out rec , b in out int ) RETURN int AS BEGIN a . c1 : = 100 ;

DECLARE r rec ;

DROP FUNCTION func ;

DROP TYPE rec ;

CREATE OR REPLACE FUNCTION func_001 ( a in out date , b in out date ) --#add in & inout #defult value RETURN integer AS BEGIN raise info '%' , a ;

DECLARE date1 date : = '2022-02-02' ;

CREATE OR REPLACE FUNCTION func_001 ( a in out INT , b in out date ) --#add in & inout #defult value RETURN INT AS BEGIN raise info '%' , a ;

DECLARE date1 int : = 1 ;

DROP FUNCTION func_001 ;

