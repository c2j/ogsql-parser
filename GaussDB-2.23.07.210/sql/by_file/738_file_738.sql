-- 来源: 738_file_738.txt
-- SQL 数量: 12

CREATE USER sysadmin WITH SYSADMIN password "********" ;

ALTER USER joe SYSADMIN ;

CREATE USER createrole WITH CREATEROLE password "********" ;

ALTER USER joe CREATEROLE ;

CREATE USER auditadmin WITH AUDITADMIN password "********" ;

ALTER USER joe AUDITADMIN ;

CREATE USER monadmin WITH MONADMIN password "********" ;

ALTER USER joe MONADMIN ;

CREATE USER opradmin WITH OPRADMIN password "********" ;

ALTER USER joe OPRADMIN ;

CREATE USER poladmin WITH POLADMIN password "********" ;

ALTER USER joe POLADMIN ;

