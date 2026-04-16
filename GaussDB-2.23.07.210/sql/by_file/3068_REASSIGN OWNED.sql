-- 来源: 3068_REASSIGN OWNED.txt
-- SQL 数量: 4

CREATE USER jim PASSWORD '********' ;

CREATE USER tom PASSWORD '********' ;

REASSIGN OWNED BY jim TO tom ;

DROP USER jim , tom CASCADE ;

