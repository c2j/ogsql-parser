-- 来源: 742_Schema.txt
-- SQL 数量: 5

SELECT s . nspname , u . usename AS nspowner FROM pg_namespace s , pg_user u WHERE nspname = 'schema_name' AND s . nspowner = u . usesysid ;

SELECT * FROM pg_namespace ;

SELECT distinct ( tablename ), schemaname from pg_tables where schemaname = 'pg_catalog' ;

SHOW SEARCH_PATH ;

SET SEARCH_PATH TO myschema , public ;

