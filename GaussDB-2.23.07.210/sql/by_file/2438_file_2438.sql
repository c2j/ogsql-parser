-- 来源: 2438_file_2438.txt
-- SQL 数量: 4

CREATE USER joe WITH CREATEDB PASSWORD "********" ;

SELECT * FROM pg_user ;

SELECT * FROM pg_authid ;

CREATE USER user_persistence WITH PERSISTENCE IDENTIFIED BY "********" ;

