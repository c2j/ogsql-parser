-- 来源: 2993_DELETE.txt
-- SQL 数量: 10

CREATE SCHEMA tpcds;

--创建表tpcds.customer_address。
CREATE TABLE tpcds.customer_address ( ca_address_sk INTEGER NOT NULL, ca_address_id CHARACTER(16) NOT NULL, ca_street_number INTEGER , ca_street_name CHARACTER (20) );

--向表中插入多条记录。
INSERT INTO tpcds.customer_address VALUES (1, 'AAAAAAAABAAAAAAA', '18', 'Jackson'),(10000, 'AAAAAAAACAAAAAAA', '362', 'Washington 6th'),(15000, 'AAAAAAAADAAAAAAA', '585', 'Dogwood Washington');

--创建表 tpcds. customer_address_bak。
CREATE TABLE tpcds. customer_address_bak AS TABLE tpcds. customer_address;

--删除 tpcds. customer_address_bak中ca_address_sk大于14888的职员。
DELETE FROM tpcds. customer_address_bak WHERE ca_address_sk > 14888;

--同时删除 tpcds. customer_address和 tpcds. customer_address_bak中ca_address_sk小于50的职员。
DELETE FROM tpcds. customer_address a, tpcds. customer_address_bak b where a.ca_address_sk = b.ca_address_sk and a.ca_address_sk < 50;

--删除 tpcds. customer_address_bak中所有数据。
DELETE FROM tpcds. customer_address_bak;

--删除 tpcds. customer_address_bak表。
DROP TABLE tpcds. customer_address_bak;

--删除tpcds.customer_address表。
DROP TABLE tpcds.customer_address;

--删除SCHEMA。
DROP SCHEMA tpcds CASCADE;

