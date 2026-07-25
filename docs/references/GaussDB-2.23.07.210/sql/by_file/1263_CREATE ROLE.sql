-- 来源: 1263_CREATE ROLE.txt
-- SQL 数量: 6

CREATE ROLE manager IDENTIFIED BY '********' ;

CREATE ROLE miriam WITH LOGIN PASSWORD '********' VALID BEGIN '2015-01-01' VALID UNTIL '2026-01-01' ;

ALTER ROLE manager IDENTIFIED BY '**********' REPLACE '********' ;

ALTER ROLE manager SYSADMIN ;

DROP ROLE manager ;

DROP GROUP miriam ;

