-- 来源: 2882_ABORT.txt
-- SQL 数量: 7

CREATE TABLE customer_demographics_t1 ( CD_DEMO_SK INTEGER NOT NULL, CD_GENDER CHAR(1) , CD_MARITAL_STATUS CHAR(1) , CD_EDUCATION_STATUS CHAR(20) , CD_PURCHASE_ESTIMATE INTEGER , CD_CREDIT_RATING CHAR(10) , CD_DEP_COUNT INTEGER , CD_DEP_EMPLOYED_COUNT INTEGER , CD_DEP_COLLEGE_COUNT INTEGER ) ;

--插入记录。
INSERT INTO customer_demographics_t1 VALUES(1920801,'M', 'U', 'DOCTOR DEGREE', 200, 'GOOD', 1, 0,0);

--开启事务。
START TRANSACTION;

--更新字段值。
UPDATE customer_demographics_t1 SET cd_education_status= 'Unknown';

--终止事务，上面所执行的更新会被撤销掉。
ABORT;

--查询数据。
SELECT * FROM customer_demographics_t1 WHERE cd_demo_sk = 1920801;

--删除表。
DROP TABLE customer_demographics_t1;

