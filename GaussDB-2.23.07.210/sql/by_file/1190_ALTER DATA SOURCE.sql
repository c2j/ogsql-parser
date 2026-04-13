-- 来源: 1190_ALTER DATA SOURCE.txt
-- SQL 数量: 11

CREATE DATA SOURCE ds_test1 ;

ALTER DATA SOURCE ds_test1 RENAME TO ds_test ;

CREATE USER user_test1 IDENTIFIED BY '********' ;

ALTER USER user_test1 WITH SYSADMIN ;

ALTER DATA SOURCE ds_test OWNER TO user_test1 ;

ALTER DATA SOURCE ds_test TYPE 'MPPDB_TYPE' VERSION 'XXX' ;

ALTER DATA SOURCE ds_test OPTIONS ( add dsn 'mppdb' , username 'test_user' );

ALTER DATA SOURCE ds_test OPTIONS ( set dsn 'unknown' );

ALTER DATA SOURCE ds_test OPTIONS ( drop username );

DROP DATA SOURCE ds_test ;

DROP USER user_test1 ;

