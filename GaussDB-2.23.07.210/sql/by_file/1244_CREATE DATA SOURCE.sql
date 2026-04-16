-- 来源: 1244_CREATE DATA SOURCE.txt
-- SQL 数量: 8

CREATE DATA SOURCE ds_test1 ;

CREATE DATA SOURCE ds_test2 TYPE 'MPPDB' VERSION NULL ;

CREATE DATA SOURCE ds_test3 OPTIONS ( dsn 'GaussDB' , encoding 'utf8' );

CREATE DATA SOURCE ds_test4 TYPE 'unknown' VERSION '11.2.3' OPTIONS ( dsn 'GaussDB' , username 'userid' , password '********' , encoding '' );

DROP DATA SOURCE ds_test1 ;

DROP DATA SOURCE ds_test2 ;

DROP DATA SOURCE ds_test3 ;

DROP DATA SOURCE ds_test4 ;

