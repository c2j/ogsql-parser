-- 来源: 2933_CLEAN CONNECTION.txt
-- SQL 数量: 7

CREATE DATABASE test_clean_connection ;

CREATE USER jack PASSWORD '********' ;

CLEAN CONNECTION TO ALL FOR DATABASE template1 TO USER jack ;

CLEAN CONNECTION TO ALL TO USER jack ;

CLEAN CONNECTION TO ALL FORCE FOR DATABASE test_clean_connection ;

DROP USER jack ;

DROP DATABASE test_clean_connection ;

