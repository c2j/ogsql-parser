-- 来源: 1435_file_1435.txt
-- SQL 数量: 5

DROP TABLE IF EXISTS customers;

CREATE TABLE customers(id int,name varchar);

INSERT INTO customers VALUES(1,'ab');

DECLARE my_id integer;

DECLARE type id_list is varray(6) of customers.id%type;

