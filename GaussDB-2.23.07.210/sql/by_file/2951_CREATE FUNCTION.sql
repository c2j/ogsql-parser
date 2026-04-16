-- 来源: 2951_CREATE FUNCTION.txt
-- SQL 数量: 37

CREATE FUNCTION func_add_sql(integer, integer) RETURNS integer AS 'select $1 + $2;

--利用参数名用 plpgsql 自增一个整数。
CREATE OR REPLACE FUNCTION func_increment_plsql(i integer) RETURNS integer AS $$ BEGIN RETURN i + 1;

--返回RECORD类型
CREATE OR REPLACE FUNCTION func_increment_sql(i int, out result_1 bigint, out result_2 bigint) RETURNS SETOF RECORD AS $$ BEGIN result_1 = i + 1;

--返回一个包含多个输出参数的记录。
CREATE FUNCTION func_dup_sql(in int, out f1 int, out f2 text) AS $$ SELECT $1, CAST($1 AS text) || ' is text' $$ LANGUAGE SQL;

-- 调用func_dup_sql函数
SELECT * FROM func_dup_sql(42);

--计算两个整数的和，并返回结果。如果输入为null，则返回null。
CREATE FUNCTION func_add_sql2(num1 integer, num2 integer) RETURN integer AS BEGIN RETURN num1 + num2;

--修改函数func_add_sql2的执行规则为IMMUTABLE，即参数不变时返回相同结果。
ALTER FUNCTION func_add_sql2(INTEGER, INTEGER) IMMUTABLE;

--将函数func_add_sql2的名称修改为add_two_number。
ALTER FUNCTION func_add_sql2(INTEGER, INTEGER) RENAME TO add_two_number;

--创建jim用户。
CREATE USER jim PASSWORD '********';

--将函数add_two_number的所有者改为 jim 。
ALTER FUNCTION add_two_number(INTEGER, INTEGER) OWNER TO jim ;

--删除函数。
DROP FUNCTION func_add_sql;

DROP FUNCTION func_increment_plsql;

DROP FUNCTION func_increment_sql;

DROP FUNCTION func_dup_sql;

DROP FUNCTION add_two_number;

--删除jim用户
DROP USER jim;

--设置参数
SET behavior_compat_options='proc_outparam_override';

--创建函数
CREATE OR REPLACE FUNCTION func1(in a integer, out b integer) RETURNS int AS $$ DECLARE c int;

--同时返回return和出参
DECLARE result integer;

--不支持左赋值表达式
DECLARE result integer;

--存储过程中不支持out/inout传入常量
DECLARE result integer;

--存储过程中支持out/inout传入变量
DECLARE result integer;

--删除函数func
DROP FUNCTION func1;

-- 不打开参数set behavior_compat_options = 'proc_outparam_override'时，被匿名块或存储过程直接调用的函数的OUT、IN OUT出参不能使用复合类型，并且RETURN值会被当做OUT出参的第一个值导致调用失败
CREATE TYPE rec as(c1 int, c2 int);

CREATE OR REPLACE FUNCTION func(a in out rec, b in out int) RETURN int AS BEGIN a.c1:=100;

DECLARE r rec;

DROP FUNCTION func;

DROP TYPE rec;

--以下示例只有当数据库兼容模式为A时可以执行
CREATE OR REPLACE PACKAGE pkg_type AS type table_of_index_int is table of integer index by integer;

--创建一个返回table of integer index by integer类型结果的函数
CREATE OR REPLACE FUNCTION func_001(a in out pkg_type.table_of_index_int, b in out pkg_type.table_of_index_var) --#add in & inout #defult value RETURN pkg_type.table_of_index_int AS table_of_index_int_val pkg_type.table_of_index_int;

DECLARE table_of_index_int_val pkg_type.table_of_index_int;

--创建一个含有IN/OUT类型参数的函数
CREATE OR REPLACE FUNCTION func_001(a in out date, b in out date) --#add in & inout #defult value RETURN integer AS BEGIN raise info '%', a;

DECLARE date1 date := '2022-02-02';

--创建一个含有IN/OUT类型参数的函数
CREATE OR REPLACE FUNCTION func_001(a in out INT, b in out date) --#add in & inout #defult value RETURN INT AS BEGIN raise info '%', a;

DECLARE date1 int := 1;

--删除函数
DROP FUNCTION func_001;

--删除package
DROP PACKAGE pkg_type;

