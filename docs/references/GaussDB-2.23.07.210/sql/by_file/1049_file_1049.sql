-- 来源: 1049_file_1049.txt
-- SQL 数量: 6

CREATE TABLE bool_type_t1 ( BT_COL1 BOOLEAN , BT_COL2 TEXT ) DISTRIBUTE BY HASH ( BT_COL2 );

INSERT INTO bool_type_t1 VALUES ( TRUE , 'sic est' );

INSERT INTO bool_type_t1 VALUES ( FALSE , 'non est' );

SELECT * FROM bool_type_t1 ;

SELECT * FROM bool_type_t1 WHERE bt_col1 = 't' ;

DROP TABLE bool_type_t1 ;

