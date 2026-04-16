-- 来源: 1271_CREATE TABLESPACE.txt
-- SQL 数量: 10

CREATE TABLESPACE ds_location1 RELATIVE LOCATION 'test_tablespace/test_tablespace_1' ;

CREATE ROLE joe IDENTIFIED BY '********' ;

CREATE ROLE jay IDENTIFIED BY '********' ;

CREATE TABLESPACE ds_location2 OWNER joe RELATIVE LOCATION 'test_tablespace/test_tablespace_2' ;

ALTER TABLESPACE ds_location1 RENAME TO ds_location3 ;

ALTER TABLESPACE ds_location2 OWNER TO jay ;

DROP TABLESPACE ds_location2 ;

DROP TABLESPACE ds_location3 ;

DROP ROLE joe ;

DROP ROLE jay ;

