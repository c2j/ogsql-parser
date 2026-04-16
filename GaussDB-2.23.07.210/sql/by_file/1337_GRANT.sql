-- 来源: 1337_GRANT.txt
-- SQL 数量: 25

CREATE USER joe PASSWORD 'xxxxxxxxxx' ;

GRANT ALL PRIVILEGES TO joe ;

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . reason ( r_reason_sk INTEGER NOT NULL , r_reason_id CHAR ( 16 ) NOT NULL , r_reason_desc VARCHAR ( 20 ) );

REVOKE ALL PRIVILEGES FROM joe ;

GRANT USAGE ON SCHEMA tpcds TO joe ;

GRANT ALL PRIVILEGES ON tpcds . reason TO joe ;

GRANT select ( r_reason_sk , r_reason_id , r_reason_desc ), update ( r_reason_desc ) ON tpcds . reason TO joe ;

GRANT select ( r_reason_sk , r_reason_id ) ON tpcds . reason TO joe WITH GRANT OPTION ;

CREATE DATABASE testdb ;

GRANT create , connect on database testdb TO joe WITH GRANT OPTION ;

CREATE ROLE tpcds_manager PASSWORD 'xxxxxxxxxx' ;

GRANT USAGE , CREATE ON SCHEMA tpcds TO tpcds_manager ;

CREATE TABLESPACE tpcds_tbspc RELATIVE LOCATION 'tablespace/tablespace_1' ;

GRANT ALL ON TABLESPACE tpcds_tbspc TO joe ;

CREATE or replace FUNCTION tpcds.fun1() RETURN boolean AS BEGIN SELECT current_user;

GRANT ALTER ON FUNCTION tpcds.fun1() TO joe;

CREATE ROLE manager PASSWORD 'xxxxxxxxxxx' ;

GRANT joe TO manager WITH ADMIN OPTION ;

CREATE ROLE senior_manager PASSWORD 'xxxxxxxxxxx' ;

GRANT manager TO senior_manager ;

REVOKE joe FROM manager ;

REVOKE manager FROM senior_manager ;

DROP USER manager ;

DROP DATABASE testdb ;

