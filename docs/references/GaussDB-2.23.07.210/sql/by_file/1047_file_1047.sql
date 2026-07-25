-- 来源: 1047_file_1047.txt
-- SQL 数量: 37

CREATE TABLE int_type_t1 ( IT_COL1 TINYINT, IT_COL2 TINYINT UNSIGNED );

--插入数据。
INSERT INTO int_type_t1 VALUES(10,20);

--查看数据。
SELECT * FROM int_type_t1;

--删除表。
DROP TABLE int_type_t1;

CREATE TABLE int_type_t2 ( a TINYINT , b TINYINT , c INTEGER , d INTEGER UNSIGNED , e BIGINT , f BIGINT UNSIGNED );

INSERT INTO int_type_t2 VALUES ( 100 , 10 , 1000 , 10000 , 200 , 2000 );

SELECT * FROM int_type_t2 ;

DROP TABLE int_type_t2 ;

CREATE TABLE decimal_type_t1 ( DT_COL1 DECIMAL(10,4) );

--插入数据。
INSERT INTO decimal_type_t1 VALUES(123456.122331);

--查询表中的数据。
SELECT * FROM decimal_type_t1;

--删除表。
DROP TABLE decimal_type_t1;

CREATE TABLE numeric_type_t1 ( NT_COL1 NUMERIC ( 10 , 4 ) );

INSERT INTO numeric_type_t1 VALUES ( 123456 . 12354 );

SELECT * FROM numeric_type_t1 ;

DROP TABLE numeric_type_t1 ;

CREATE TABLE smallserial_type_tab ( a SMALLSERIAL );

INSERT INTO smallserial_type_tab VALUES ( default );

INSERT INTO smallserial_type_tab VALUES ( default );

SELECT * FROM smallserial_type_tab ;

CREATE TABLE serial_type_tab ( b SERIAL );

INSERT INTO serial_type_tab VALUES ( default );

INSERT INTO serial_type_tab VALUES ( default );

SELECT * FROM serial_type_tab ;

CREATE TABLE bigserial_type_tab ( c BIGSERIAL );

INSERT INTO bigserial_type_tab VALUES ( default );

INSERT INTO bigserial_type_tab VALUES ( default );

SELECT * FROM bigserial_type_tab ;

DROP TABLE smallserial_type_tab ;

DROP TABLE serial_type_tab ;

DROP TABLE bigserial_type_tab ;

CREATE TABLE float_type_t2 ( FT_COL1 INTEGER , FT_COL2 FLOAT4 , FT_COL3 FLOAT8 , FT_COL4 FLOAT ( 3 ), FT_COL5 BINARY_DOUBLE , FT_COL6 DECIMAL ( 10 , 4 ), FT_COL7 INTEGER ( 6 , 3 ) ) DISTRIBUTE BY HASH ( ft_col1 );

INSERT INTO float_type_t2 VALUES ( 10 , 10 . 365456 , 123456 . 1234 , 10 . 3214 , 321 . 321 , 123 . 123654 , 123 . 123654 );

SELECT * FROM float_type_t2 ;

DROP TABLE float_type_t2 ;

CREATE DATABASE gaussdb_m WITH dbcompatibility 'MYSQL' ;

\ c gaussdb_m --设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

