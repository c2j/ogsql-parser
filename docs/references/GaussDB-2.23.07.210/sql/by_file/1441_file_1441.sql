-- 来源: 1441_file_1441.txt
-- SQL 数量: 10

DROP SCHEMA IF EXISTS hr CASCADE ;

CREATE SCHEMA hr ;

SET CURRENT_SCHEMA = hr ;

CREATE TABLE staffs ( staff_id NUMBER , first_name VARCHAR2 , salary NUMBER );

INSERT INTO staffs VALUES ( 200 , 'mike' , 5800 );

INSERT INTO staffs VALUES ( 201 , 'lily' , 3000 );

INSERT INTO staffs VALUES ( 202 , 'john' , 4400 );

CREATE OR REPLACE PROCEDURE dynamic_proc AS staff_id NUMBER ( 6 ) : = 200 ;

CALL dynamic_proc ();

DROP PROCEDURE dynamic_proc ;

