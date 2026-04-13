-- 来源: 3145_file_3145.txt
-- SQL 数量: 7

CREATE TYPE o1 AS ( a int , b int );

DECLARE TYPE r1 is VARRAY ( 10 ) of o1 ;

DROP TABLE IF EXISTS customers;

DECLARE type id_list is varray(6) of customers.id%type;

CREATE TABLE test(a integer);

CREATE OR REPLACE FUNCTION check_test() RETURNS integer language plpgsql AS $function$ DECLARE b integer;

SELECT check_test();

