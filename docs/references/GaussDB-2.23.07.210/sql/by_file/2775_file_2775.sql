-- 来源: 2775_file_2775.txt
-- SQL 数量: 109

SELECT cash_words ( '1.23' );

SELECT convert ( 12 . 5 , text );

SELECT cast ( '22-oct-1997' as timestamp );

SELECT cast ( '22-ocX-1997' as timestamp DEFAULT '22-oct-1997' ON CONVERSION ERROR , 'DD-Mon-YYYY' );

CREATE DATABASE gaussdb_m WITH dbcompatibility 'b' ;

\ c gaussdb_m -- 设置兼容版本控制参数 gaussdb_m =# SET b_format_version = '5.7' ;

SELECT CAST ( 12 AS UNSIGNED );

SELECT hextoraw ( '7D' );

SELECT numtoday ( 2 );

SELECT rawtohex ( '1234567' );

set a_format_dev_version='s2';

select rawtohex2('12\n?$\123/2');

set a_format_dev_version='s2';

select bit2coding('1234567890');

set a_format_dev_version='s2';

select bit4coding('1234567890');

SELECT to_blob ( '0AADD343CDBBD' :: RAW ( 10 ));

SELECT to_bigint ( '123364545554455' );

SELECT to_binary_double ( '12345678' );

SELECT to_binary_double ( '1,2,3' , '9,9,9' );

SELECT to_binary_double ( 1 e2 default 12 on conversion error );

SELECT to_binary_double ( 'aa' default 12 on conversion error );

SELECT to_binary_double ( '12-' default 10 on conversion error , '99S' );

SELECT to_binary_double ( 'aa-' default 12 on conversion error , '99S' );

SELECT to_binary_float ( '12345678' );

SELECT to_binary_float ( '1,2,3' , '9,9,9' );

SELECT to_binary_float ( 1 e2 default 12 on conversion error );

SELECT to_binary_float ( 'aa' default 12 on conversion error );

SELECT to_binary_float ( '12-' default 10 on conversion error , '99S' );

SELECT to_binary_float ( 'aa-' default 12 on conversion error , '99S' );

SELECT to_char ( current_timestamp , 'HH12:MI:SS' );

SELECT to_char ( current_timestamp , 'FMHH12:FMMI:FMSS' );

SELECT to_char ( 125 . 8 :: real , '999D99' );

SELECT to_char ( 1485 , '9,999' );

SELECT to_char ( 1148 . 5 , '9,999.999' );

SELECT to_char ( 148 . 5 , '990999.909' );

SELECT to_char ( 123 , 'XXX' );

SELECT to_char ( interval '15h 2m 12s' , 'HH24:MI:SS' );

SELECT to_char ( 125 , '999' );

select to_char ( site ) from employee ;

SELECT to_char ( - 125 . 8 , '999D99S' );

SELECT to_char ( '01110' );

SELECT to_char ( current_timestamp , 'HH12:MI:SS' );

SELECT to_nchar ( current_timestamp , 'HH12:MI:SS' );

SELECT to_nchar ( current_timestamp , 'FMHH12:FMMI:FMSS' );

SELECT to_nchar ( 125 . 8 :: real , '999D99' );

SELECT to_nchar ( 1485 , '9,999' );

SELECT to_nchar ( 1148 . 5 , '9,999.999' );

SELECT to_nchar ( 148 . 5 , '990999.909' );

SELECT to_nchar ( 123 , 'XXX' );

SELECT to_nchar ( interval '15h 2m 12s' , 'HH24:MI:SS' );

SELECT to_nchar ( 125 , '999' );

SELECT to_nchar ( - 125 . 8 , '999D99S' );

SELECT to_nchar ( '01110' );

SELECT to_nchar ( current_timestamp , 'HH12:MI:SS' );

SELECT to_clob ( 'ABCDEF' :: RAW ( 10 ));

SELECT to_clob ( 'hello111' :: CHAR ( 15 ));

SELECT to_clob ( 'gauss123' :: NCHAR ( 10 ));

SELECT to_clob ( 'gauss234' :: VARCHAR ( 10 ));

SELECT to_clob ( 'gauss345' :: VARCHAR2 ( 10 ));

SELECT to_clob ( 'gauss456' :: NVARCHAR2 ( 10 ));

SELECT to_clob ( 'World222!' :: TEXT );

SELECT to_date ( '2015-08-14' );

SELECT to_date ( '05 Dec 2000' , 'DD Mon YYYY' );

SELECT to_date ( '2015-08-14' );

SELECT to_date ( '05 Dec 2000' , 'DD Mon YYYY' );

set a_format_version='10c';

set a_format_dev_version='s1';

show nls_timestamp_format;

select to_date('12-jan-2022' default '12-apr-2022' on conversion error);

select to_date('12-ja-2022' default '12-apr-2022' on conversion error);

select to_date('2022-12-12' default '2022-01-01' on conversion error, 'yyyy-mm-dd');

SELECT to_number ( '12,454.8-' , '99G999D9S' );

SELECT to_number ( '12,454.8-' , '99G999D9S' );

select to_number ( '1e2' );

select to_number ( '123.456' );

select to_number ( '123' , '999' );

select to_number ( '123-' , '999MI' );

select to_number ( '123' default '456-' on conversion error , '999MI' );

SELECT to_timestamp ( 1284352323 );

SHOW nls_timestamp_format ;

SELECT to_timestamp ( '12-sep-2014' );

SELECT to_timestamp ( '12-Sep-10 14:10:10.123000' , 'DD-Mon-YY HH24:MI:SS.FF' );

SELECT to_timestamp ( '-1' , 'SYYYY' );

SELECT to_timestamp ( '98' , 'RR' );

SELECT to_timestamp ( '01' , 'RR' );

set a_format_version='10c';

set a_format_dev_version='s1';

SELECT to_timestamp('11-Sep-11' DEFAULT '12-Sep-10 14:10:10.123000' ON CONVERSION ERROR,'DD-Mon-YY HH24:MI:SS.FF');

SELECT to_timestamp('12-Sep-10 14:10:10.123000','DD-Mon-YY HH24:MI:SSXFF');

SELECT to_timestamp ( '05 Dec 2000' , 'DD Mon YYYY' );

SELECT to_timestamp_tz ( '05 DeX 2000' DEFAULT '05 Dec 2001' ON CONVERSION ERROR , 'DD Mon YYYY' );

SELECT to_timestamp_tz ( '05 DeX 2000' DEFAULT '05 Dec 2001' ON CONVERSION ERROR , 'DD Mon YYYY' , 'nls_date_language=AMERICAN' );

select to_dsinterval ( '12 1:2:3.456' );

select to_dsinterval ( 'P3DT4H5M6S' );

select to_yminterval ( '1-1' );

select to_yminterval ( 'P13Y3M4DT4H2M5S' );

create table json_doc ( data CLOB );

insert into json_doc values ( '{"name":"a"}' );

select treat ( data as json ) from json_doc ;

create or replace procedure p1 is gaussdb $ # type t1 is table of int ;

call p1 ();

create type t1 is table of int ;

select cast ( t1 ( 1 , 2 , 3 ) as int []) result ;

create or replace package pkg1 is gaussdb $ # type t1 is table of int index by int ;

create or replace package body pkg1 is gaussdb $ # procedure p1 () is gaussdb $ # v1 t1 : = t1 ( 1 => 1 , 2 => 2 , 3 => 3 );

call pkg1 . p1 ();

select indexbytableint_to_array ( pkg1 . t1 ( 1 => 1 , 2 => 2 , 3 => 3 ));

SELECT convert_to_nocase ( '12345' , 'GBK' );

