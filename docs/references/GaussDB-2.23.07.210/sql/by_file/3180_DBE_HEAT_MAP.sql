-- 来源: 3180_DBE_HEAT_MAP.txt
-- SQL 数量: 7

ALTER DATABASE set ilm = on ;

CREATE Schema HEAT_MAP_DATA ;

SET current_schema = HEAT_MAP_DATA ;

CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1' ;

CREATE TABLE HEAT_MAP_DATA . heat_map_table ( id INT , value TEXT ) TABLESPACE example1 ;

INSERT INTO HEAT_MAP_DATA . heat_map_table VALUES ( 1 , 'test_data_row_1' );

SELECT * from DBE_HEAT_MAP . ROW_HEAT_MAP ( owner => 'heat_map_data' , segment_name => 'heat_map_table' , partition_name => NULL , ctid => '(0,1)' );

