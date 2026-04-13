-- 来源: 764_file_764.txt
-- SQL 数量: 34

CREATE TABLE tpcds . customer_address ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) DISTRIBUTE BY HASH ( ca_address_sk ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) ) ENABLE ROW MOVEMENT ;

CREATE TABLE tpcds . web_returns_p2 ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) DISTRIBUTE BY HASH ( ca_address_sk ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) ) ENABLE ROW MOVEMENT ;

INSERT INTO tpcds . web_returns_p2 SELECT * FROM tpcds . customer_address ;

ALTER TABLE tpcds . web_returns_p2 DISABLE ROW MOVEMENT ;

ALTER TABLE tpcds . web_returns_p2 DROP PARTITION P8 ;

ALTER TABLE tpcds . web_returns_p2 ADD PARTITION P8 VALUES LESS THAN ( MAXVALUE );

ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION P8 TO P_9 ;

ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION FOR ( 40000 ) TO P8 ;

SELECT * FROM tpcds . web_returns_p2 PARTITION ( P6 );

SELECT * FROM tpcds . web_returns_p2 PARTITION FOR ( 35888 );

DROP TABLE tpcds . customer_address ;

DROP TABLE tpcds . web_returns_p2 ;

CREATE TABLESPACE example1 RELATIVE LOCATION 'tablespace1/tablespace_1' ;

CREATE TABLESPACE example2 RELATIVE LOCATION 'tablespace2/tablespace_2' ;

CREATE TABLESPACE example3 RELATIVE LOCATION 'tablespace3/tablespace_3' ;

CREATE TABLESPACE example4 RELATIVE LOCATION 'tablespace4/tablespace_4' ;

CREATE TABLE tpcds . customer_address ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) TABLESPACE example1 DISTRIBUTE BY HASH ( ca_address_sk ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) TABLESPACE example2 ) ENABLE ROW MOVEMENT ;

CREATE TABLE tpcds . web_returns_p2 ( ca_address_sk integer NOT NULL , ca_address_id character ( 16 ) NOT NULL , ca_street_number character ( 10 ) , ca_street_name character varying ( 60 ) , ca_street_type character ( 15 ) , ca_suite_number character ( 10 ) , ca_city character varying ( 60 ) , ca_county character varying ( 30 ) , ca_state character ( 2 ) , ca_zip character ( 10 ) , ca_country character varying ( 20 ) , ca_gmt_offset numeric ( 5 , 2 ) , ca_location_type character ( 20 ) ) TABLESPACE example1 DISTRIBUTE BY HASH ( ca_address_sk ) PARTITION BY RANGE ( ca_address_sk ) ( PARTITION P1 VALUES LESS THAN ( 5000 ), PARTITION P2 VALUES LESS THAN ( 10000 ), PARTITION P3 VALUES LESS THAN ( 15000 ), PARTITION P4 VALUES LESS THAN ( 20000 ), PARTITION P5 VALUES LESS THAN ( 25000 ), PARTITION P6 VALUES LESS THAN ( 30000 ), PARTITION P7 VALUES LESS THAN ( 40000 ), PARTITION P8 VALUES LESS THAN ( MAXVALUE ) TABLESPACE example2 ) ENABLE ROW MOVEMENT ;

INSERT INTO tpcds . web_returns_p2 SELECT * FROM tpcds . customer_address ;

ALTER TABLE tpcds . web_returns_p2 DISABLE ROW MOVEMENT ;

ALTER TABLE tpcds . web_returns_p2 DROP PARTITION P8 ;

ALTER TABLE tpcds . web_returns_p2 ADD PARTITION P8 VALUES LESS THAN ( MAXVALUE );

ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION P8 TO P_9 ;

ALTER TABLE tpcds . web_returns_p2 RENAME PARTITION FOR ( 40000 ) TO P8 ;

ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P6 TABLESPACE example3 ;

ALTER TABLE tpcds . web_returns_p2 MOVE PARTITION P4 TABLESPACE example4 ;

SELECT * FROM tpcds . web_returns_p2 PARTITION ( P6 );

SELECT * FROM tpcds . web_returns_p2 PARTITION FOR ( 35888 );

DROP TABLE tpcds . customer_address ;

DROP TABLE tpcds . web_returns_p2 ;

DROP TABLESPACE example1 ;

DROP TABLESPACE example2 ;

DROP TABLESPACE example3 ;

DROP TABLESPACE example4 ;

