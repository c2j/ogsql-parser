-- 来源: 1196_ALTER GROUP.txt
-- SQL 数量: 8

CREATE GROUP super_users WITH PASSWORD "********" ;

CREATE ROLE lche WITH PASSWORD "********" ;

CREATE ROLE jim WITH PASSWORD "********" ;

ALTER GROUP super_users ADD USER lche , jim ;

ALTER GROUP super_users DROP USER jim ;

ALTER GROUP super_users RENAME TO normal_users ;

DROP ROLE lche , jim ;

DROP GROUP normal_users ;

