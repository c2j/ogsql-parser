-- 来源: 1237_COPY.txt
-- SQL 数量: 16

CREATE SCHEMA tpcds ;

CREATE TABLE tpcds . ship_mode ( SM_SHIP_MODE_SK INTEGER NOT NULL , SM_SHIP_MODE_ID CHAR ( 16 ) NOT NULL , SM_TYPE CHAR ( 30 ) , SM_CODE CHAR ( 10 ) , SM_CARRIER CHAR ( 20 ) , SM_CONTRACT CHAR ( 20 ) ) DISTRIBUTE BY HASH ( SM_SHIP_MODE_SK );

INSERT INTO tpcds . ship_mode VALUES ( 1 , 'a' , 'b' , 'c' , 'd' , 'e' );

COPY tpcds . ship_mode TO '/home/omm/ds_ship_mode.dat' ;

COPY tpcds . ship_mode TO STDOUT ;

COPY tpcds . ship_mode TO STDOUT WITH ( delimiter ',' , encoding 'utf8' );

COPY tpcds . ship_mode TO STDOUT WITH ( format 'CSV' , force_quote ( SM_SHIP_MODE_SK ));

CREATE TABLE tpcds . ship_mode_t1 ( SM_SHIP_MODE_SK INTEGER NOT NULL , SM_SHIP_MODE_ID CHAR ( 16 ) NOT NULL , SM_TYPE CHAR ( 30 ) , SM_CODE CHAR ( 10 ) , SM_CARRIER CHAR ( 20 ) , SM_CONTRACT CHAR ( 20 ) ) DISTRIBUTE BY HASH ( SM_SHIP_MODE_SK );

COPY tpcds . ship_mode_t1 FROM STDIN ;

COPY tpcds . ship_mode_t1 FROM '/home/omm/ds_ship_mode.dat' ;

COPY tpcds . ship_mode_t1 FROM '/home/omm/ds_ship_mode.dat' TRANSFORM ( SM_TYPE AS LEFT ( SM_TYPE , 10 ));

COPY tpcds . ship_mode_t1 FROM '/home/omm/ds_ship_mode.dat' WITH ( format 'text' , delimiter E '\t' , ignore_extra_data 'true' , noescaping 'true' );

COPY tpcds . ship_mode_t1 FROM '/home/omm/ds_ship_mode.dat' FIXED FORMATTER ( SM_SHIP_MODE_SK ( 0 , 2 ), SM_SHIP_MODE_ID ( 2 , 16 ), SM_TYPE ( 18 , 30 ), SM_CODE ( 50 , 10 ), SM_CARRIER ( 61 , 20 ), SM_CONTRACT ( 82 , 20 )) header ignore_extra_data ;

DROP TABLE tpcds . ship_mode ;

DROP TABLE tpcds . ship_mode_t1 ;

DROP SCHEMA tpcds ;

