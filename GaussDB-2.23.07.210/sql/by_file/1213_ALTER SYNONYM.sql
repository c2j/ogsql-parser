-- 来源: 1213_ALTER SYNONYM.txt
-- SQL 数量: 11

CREATE USER sysadmin WITH SYSADMIN PASSWORD '********' ;

\ c - sysadmin --创建同义词t1。

CREATE OR REPLACE SYNONYM t1 FOR ot . t1 ;

CREATE USER u1 PASSWORD '********' ;

GRANT ALL ON SCHEMA sysadmin TO u1 ;

ALTER SYNONYM t1 OWNER TO u1 ;

DROP SYNONYM t1 ;

REVOKE ALL ON SCHEMA sysadmin FROM u1 ;

DROP USER u1 ;

\ c - init_user --删除用户sysadmin。

DROP USER sysadmin ;

