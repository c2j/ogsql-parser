-- 来源: 2937_COMMIT _ END.txt
-- SQL 数量: 9

CREATE SCHEMA tpcds;

--创建表。
CREATE TABLE tpcds. customer_demographics_t2 ( CD_DEMO_SK INTEGER NOT NULL, CD_GENDER CHAR(1) , CD_MARITAL_STATUS CHAR(1) , CD_EDUCATION_STATUS CHAR(20) , CD_PURCHASE_ESTIMATE INTEGER , CD_CREDIT_RATING CHAR(10) , CD_DEP_COUNT INTEGER , CD_DEP_EMPLOYED_COUNT INTEGER , CD_DEP_COLLEGE_COUNT INTEGER ) ;

--开启事务。
START TRANSACTION;

--插入数据。
INSERT INTO tpcds. customer_demographics_t2 VALUES(1,'M', 'U', 'DOCTOR DEGREE', 1200, 'GOOD', 1, 0, 0);

INSERT INTO tpcds. customer_demographics_t2 VALUES(2,'F', 'U', 'MASTER DEGREE', 300, 'BAD', 1, 0, 0);

--提交事务，让所有更改永久化。
COMMIT;

--查询数据。
SELECT * FROM tpcds. customer_demographics_t2;

--删除表 tpcds. customer_demographics_t2。
DROP TABLE tpcds. customer_demographics_t2;

-- 删除SCHEMA。
DROP SCHEMA tpcds;

