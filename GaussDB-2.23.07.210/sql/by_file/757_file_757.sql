-- 来源: 757_file_757.txt
-- SQL 数量: 17

CREATE TABLE table1 ( id int , a char ( 6 ), b varchar ( 6 ), c varchar ( 6 ));

CREATE TABLE table2 ( id int , a char ( 20 ), b varchar ( 20 ), c varchar ( 20 ));

INSERT INTO table1 VALUES ( 1 , reverse ( '123AAA78' ), reverse ( '123AA78' ), reverse ( '123AA78' ));

INSERT INTO table1 VALUES ( 2 , reverse ( '123A78' ), reverse ( '123A78' ), reverse ( '123A78' ));

INSERT INTO table1 VALUES ( 3 , '87A123' , '87A123' , '87A123' );

INSERT INTO table2 VALUES ( 1 , reverse ( '123AA78' ), reverse ( '123AA78' ), reverse ( '123AA78' ));

INSERT INTO table2 VALUES ( 2 , reverse ( '123A78' ), reverse ( '123A78' ), reverse ( '123A78' ));

INSERT INTO customer_t1 ( c_customer_sk , c_customer_id , c_first_name ) VALUES ( 3769 , 'hello' , 'Grace' );

INSERT INTO customer_t1 VALUES ( 3769 , 'hello' , 'Grace' );

INSERT INTO customer_t1 ( c_customer_sk , c_first_name ) VALUES ( 3769 , 'Grace' );

INSERT INTO customer_t1 VALUES ( 3769 , 'hello' );

INSERT INTO customer_t1 ( c_customer_sk , c_customer_id , c_first_name ) VALUES ( 3769 , 'hello' , DEFAULT );

INSERT INTO customer_t1 DEFAULT VALUES ;

INSERT INTO customer_t1 ( c_customer_sk , c_customer_id , c_first_name ) VALUES ( 6885 , 'maps' , 'Joes' ), ( 4321 , 'tpcds' , 'Lily' ), ( 9527 , 'world' , 'James' );

CREATE TABLE customer_t2 ( c_customer_sk integer , c_customer_id char ( 5 ), c_first_name char ( 6 ), c_last_name char ( 8 ) );

INSERT INTO customer_t2 SELECT * FROM customer_t1 ;

DROP TABLE customer_t2 CASCADE ;

