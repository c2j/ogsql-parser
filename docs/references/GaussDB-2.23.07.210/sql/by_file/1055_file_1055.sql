-- 来源: 1055_file_1055.txt
-- SQL 数量: 6

CREATE TABLE bit_type_t1 ( BT_COL1 INTEGER , BT_COL2 BIT ( 3 ), BT_COL3 BIT VARYING ( 5 ) ) DISTRIBUTE BY REPLICATION ;

INSERT INTO bit_type_t1 VALUES ( 1 , B '101' , B '00' );

INSERT INTO bit_type_t1 VALUES ( 2 , B '10' , B '101' );

INSERT INTO bit_type_t1 VALUES ( 2 , B '10' :: bit ( 3 ), B '101' );

SELECT * FROM bit_type_t1 ;

DROP TABLE bit_type_t1 ;

