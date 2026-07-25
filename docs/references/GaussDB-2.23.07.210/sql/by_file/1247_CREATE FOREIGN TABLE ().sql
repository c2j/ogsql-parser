-- 来源: 1247_CREATE FOREIGN TABLE ().txt
-- SQL 数量: 8

CREATE FOREIGN TABLE foreign_HR_staffS ( staff_ID NUMBER ( 6 ) , FIRST_NAME VARCHAR2 ( 20 ), LAST_NAME VARCHAR2 ( 25 ), EMAIL VARCHAR2 ( 25 ), PHONE_NUMBER VARCHAR2 ( 20 ), HIRE_DATE DATE , employment_ID VARCHAR2 ( 10 ), SALARY NUMBER ( 8 , 2 ), COMMISSION_PCT NUMBER ( 2 , 2 ), MANAGER_ID NUMBER ( 6 ), section_ID NUMBER ( 4 ) ) SERVER gsmpp_server OPTIONS ( location 'gsfs://192.168.0.90:5000/* | gsfs://192.168.0.91:5000/*' , format 'TEXT' , delimiter E '\x20' , null '' ) WITH err_HR_staffS ;

CREATE FOREIGN TABLE foreign_HR_staffS_ft3 ( staff_ID NUMBER ( 6 ) , FIRST_NAME VARCHAR2 ( 20 ), LAST_NAME VARCHAR2 ( 25 ), EMAIL VARCHAR2 ( 25 ), PHONE_NUMBER VARCHAR2 ( 20 ), HIRE_DATE DATE , employment_ID VARCHAR2 ( 10 ), SALARY NUMBER ( 8 , 2 ), COMMISSION_PCT NUMBER ( 2 , 2 ), MANAGER_ID NUMBER ( 6 ), section_ID NUMBER ( 4 ) ) SERVER gsmpp_server OPTIONS ( location 'gsfs://192.168.0.90:5000/* | gsfs://192.168.0.91:5000/*' , format 'TEXT' , delimiter E '\x20' , null '' , reject_limit '2' ) WITH err_HR_staffS_ft3 ;

CREATE FOREIGN TABLE foreign_HR_staffS_ft1 ( staff_ID NUMBER ( 6 ) , FIRST_NAME VARCHAR2 ( 20 ), LAST_NAME VARCHAR2 ( 25 ), EMAIL VARCHAR2 ( 25 ), PHONE_NUMBER VARCHAR2 ( 20 ), HIRE_DATE DATE , employment_ID VARCHAR2 ( 10 ), SALARY NUMBER ( 8 , 2 ), COMMISSION_PCT NUMBER ( 2 , 2 ), MANAGER_ID NUMBER ( 6 ), section_ID NUMBER ( 4 ) ) SERVER gsmpp_server OPTIONS ( location 'file:///input_data/*' , format 'csv' , mode 'private' , delimiter ',' ) WITH err_HR_staffS_ft1 ;

CREATE FOREIGN TABLE foreign_HR_staffS_ft2 ( staff_ID NUMBER ( 6 ) , FIRST_NAME VARCHAR2 ( 20 ), LAST_NAME VARCHAR2 ( 25 ), EMAIL VARCHAR2 ( 25 ), PHONE_NUMBER VARCHAR2 ( 20 ), HIRE_DATE DATE , employment_ID VARCHAR2 ( 10 ), SALARY NUMBER ( 8 , 2 ), COMMISSION_PCT NUMBER ( 2 , 2 ), MANAGER_ID NUMBER ( 6 ), section_ID NUMBER ( 4 ) ) SERVER gsmpp_server OPTIONS ( location 'file:///output_data/' , format 'csv' , delimiter '|' , header 'on' ) WRITE ONLY ;

DROP FOREIGN TABLE foreign_HR_staffS ;

DROP FOREIGN TABLE foreign_HR_staffS_ft1 ;

DROP FOREIGN TABLE foreign_HR_staffS_ft2 ;

DROP FOREIGN TABLE foreign_HR_staffS_ft3 ;

