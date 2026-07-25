-- 来源: 1184_ABORT.txt
-- SQL 数量: 7

CREATE TABLE customer_demographics_t1 ( CD_DEMO_SK INTEGER NOT NULL , CD_GENDER CHAR ( 1 ) , CD_MARITAL_STATUS CHAR ( 1 ) , CD_EDUCATION_STATUS CHAR ( 20 ) , CD_PURCHASE_ESTIMATE INTEGER , CD_CREDIT_RATING CHAR ( 10 ) , CD_DEP_COUNT INTEGER , CD_DEP_EMPLOYED_COUNT INTEGER , CD_DEP_COLLEGE_COUNT INTEGER ) DISTRIBUTE BY HASH ( CD_DEMO_SK );

INSERT INTO customer_demographics_t1 VALUES ( 1920801 , 'M' , 'U' , 'DOCTOR DEGREE' , 200 , 'GOOD' , 1 , 0 , 0 );

START TRANSACTION ;

UPDATE customer_demographics_t1 SET cd_education_status = 'Unknown' ;

ABORT ;

SELECT * FROM customer_demographics_t1 WHERE cd_demo_sk = 1920801 ;

DROP TABLE customer_demographics_t1 ;

