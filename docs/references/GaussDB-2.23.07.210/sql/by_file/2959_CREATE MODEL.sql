-- 来源: 2959_CREATE MODEL.txt
-- SQL 数量: 5

CREATE TABLE houses ( id INTEGER, tax INTEGER, bedroom INTEGER, bath DOUBLE PRECISION, price INTEGER, size INTEGER, lot INTEGER, mark text );

--插入训练数据
INSERT INTO houses(id, tax, bedroom, bath, price, size, lot, mark) VALUES (1,590,2,1,50000,770,22100,'a+'), (2,1050,3,2,85000,1410,12000,'a+'), (3,20,2,1,22500,1060,3500,'a-'), (4,870,2,2,90000,1300,17500,'a+'), (5,1320,3,2,133000,1500,30000,'a+'), (6,1350,2,1,90500,850,25700,'a-'), (7,2790,3,2.5,260000,2130,25000,'a+'), (8,680,2,1,142500,1170,22000,'a-'), (9,1840,3,2,160000,1500,19000,'a+'), (10,3680,4,2,240000,2790,20000,'a-'), (11,1660,3,1,87000,1030,17500,'a+'), (12,1620,3,2,118500,1250,20000,'a-'), (13,3100,3,2,140000,1760,38000,'a+'), (14,2090,2,3,148000,1550,14000,'a-'), (15,650,3,1.5,65000,1450,12000,'a-');

--训练模型
CREATE MODEL price_model USING logistic_regression FEATURES size, lot TARGET mark FROM HOUSES WITH learning_rate=0.88, max_iterations=default;

--删除模型
DROP MODEL price_model;

--删除表
DROP TABLE houses;

