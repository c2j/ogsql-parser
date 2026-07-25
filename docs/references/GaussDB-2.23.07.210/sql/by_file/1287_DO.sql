-- 来源: 1287_DO.txt
-- SQL 数量: 3

CREATE USER webuser PASSWORD '********' ;

DO $$ DECLARE r record ;

DROP USER webuser CASCADE ;

