-- 来源: 1059_HLL.txt
-- SQL 数量: 37

CREATE TABLE t1 ( id integer , set hll );

\ d t1 Table "public.t1" Column | Type | Modifiers --------+---------+----------- id | integer | set | hll | -- 创建hll类型的表，指定前两个入参，后两个采用默认值。

CREATE TABLE t2 ( id integer , set hll ( 12 , 4 ));

\ d t2 Table "public.t2" Column | Type | Modifiers --------+----------------+----------- id | integer | set | hll ( 12 , 4 , 12 , 0 ) | --创建hll类型的表，指定第三个入参，其余采用默认值。

CREATE TABLE t3 ( id int , set hll ( - 1 , - 1 , 8 , - 1 ));

\ d t3 Table "public.t3" Column | Type | Modifiers --------+----------------+----------- id | integer | set | hll ( 14 , 10 , 8 , 0 ) | --创建hll类型的表，指定入参不合法报错。

CREATE TABLE t4 ( id int , set hll ( 5 , - 1 ));

DROP TABLE t1 , t2 , t3 ;

CREATE TABLE t1 ( id integer , set hll ( 14 ));

INSERT INTO t1 VALUES ( 1 , hll_empty ( 14 , - 1 ));

INSERT INTO t1 ( id , set ) VALUES ( 1 , hll_empty ( 14 , 5 ));

DROP TABLE t1 ;

CREATE TABLE helloworld ( id integer , set hll );

INSERT INTO helloworld ( id , set ) VALUES ( 1 , hll_empty ());

UPDATE helloworld SET set = hll_add ( set , hll_hash_integer ( 12345 )) WHERE id = 1 ;

UPDATE helloworld SET set = hll_add ( set , hll_hash_text ( 'hello world' )) WHERE id = 1 ;

SELECT hll_cardinality ( set ) FROM helloworld WHERE id = 1 ;

DROP TABLE helloworld ;

CREATE TABLE facts ( date date , user_id integer );

INSERT INTO facts VALUES ( '2019-02-20' , generate_series ( 1 , 100 ));

INSERT INTO facts VALUES ( '2019-02-21' , generate_series ( 1 , 200 ));

INSERT INTO facts VALUES ( '2019-02-22' , generate_series ( 1 , 300 ));

INSERT INTO facts VALUES ( '2019-02-23' , generate_series ( 1 , 400 ));

INSERT INTO facts VALUES ( '2019-02-24' , generate_series ( 1 , 500 ));

INSERT INTO facts VALUES ( '2019-02-25' , generate_series ( 1 , 600 ));

INSERT INTO facts VALUES ( '2019-02-26' , generate_series ( 1 , 700 ));

INSERT INTO facts VALUES ( '2019-02-27' , generate_series ( 1 , 800 ));

create TABLE daily_uniques ( date date UNIQUE , users hll );

INSERT INTO daily_uniques ( date , users ) SELECT date , hll_add_agg ( hll_hash_integer ( user_id )) FROM facts GROUP BY 1 ;

SELECT date , hll_cardinality ( users ) FROM daily_uniques ORDER BY date ;

SELECT hll_cardinality ( hll_union_agg ( users )) FROM daily_uniques WHERE date >= '2019-02-20' :: date AND date <= '2019-02-26' :: date ;

SELECT date , ( # hll_union_agg ( users ) OVER two_days ) - # users AS lost_uniques FROM daily_uniques WINDOW two_days AS ( ORDER BY date ASC ROWS 1 PRECEDING );

DROP TABLE facts ;

DROP TABLE daily_uniques ； 场景3：“插入数据不满足hll数据结构要求” 当用户给hll类型的字段插入数据的时候，必须保证插入的数据满足hll数据结构要求，如果解析后不满足就会报错。如下示例中： 插入数据'E\\1234'时，该数据不满足hll数据结构 要求 ，不能解析成功因此失败报错。 1 2 3

CREATE TABLE test ( id integer , set hll );

INSERT INTO test VALUES ( 1 , 'E\\1234' );

DROP TABLE test ;

