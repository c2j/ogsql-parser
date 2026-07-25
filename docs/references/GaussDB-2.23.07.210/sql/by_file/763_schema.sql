-- 来源: 763_schema.txt
-- SQL 数量: 14

CREATE SCHEMA myschema ;

CREATE SCHEMA myschema AUTHORIZATION omm ;

CREATE TABLE myschema . mytable ( id int , name varchar ( 20 ));

SELECT * FROM myschema . mytable ;

SHOW SEARCH_PATH ;

SET SEARCH_PATH TO myschema , public ;

REVOKE CREATE ON SCHEMA public FROM PUBLIC ;

SELECT current_schema ();

CREATE USER jack IDENTIFIED BY '********' ;

GRANT USAGE ON schema myschema TO jack ;

REVOKE USAGE ON schema myschema FROM jack ;

DROP SCHEMA IF EXISTS nullschema ;

DROP SCHEMA myschema CASCADE ;

DROP USER jack ;

