-- 来源: 2763_SET.txt
-- SQL 数量: 7

CREATE TABLE employee ( name text, site SET('beijing','shanghai','nanjing','wuhan') );

INSERT INTO employee values('zhangsan', 'nanjing,beijing');

INSERT INTO employee VALUES ('zhangsan', 'hangzhou');

SELECT * FROM employee;

INSERT INTO employee values('lisi', 9);

SELECT * FROM employee;

DROP TABLE employee;

