-- 来源: 2835_file_2835.txt
-- SQL 数量: 25

CREATE TABLE Students ( name varchar ( 20 ), id int ) with ( STORAGE_TYPE = USTORE );

INSERT INTO Students VALUES ( 'Jack' , 35 );

INSERT INTO Students VALUES ( 'Leon' , 15 );

INSERT INTO Students VALUES ( 'James' , 24 );

INSERT INTO Students VALUES ( 'Taker' , 81 );

INSERT INTO Students VALUES ( 'Mary' , 25 );

INSERT INTO Students VALUES ( 'Rose' , 64 );

INSERT INTO Students VALUES ( 'Perl' , 18 );

INSERT INTO Students VALUES ( 'Under' , 57 );

INSERT INTO Students VALUES ( 'Angel' , 101 );

INSERT INTO Students VALUES ( 'Frank' , 20 );

INSERT INTO Students VALUES ( 'Charlie' , 40 );

SELECT * FROM Students WHERE rownum <= 10 ;

SELECT * FROM Students WHERE rownum < 5 order by 1 ;

SELECT rownum , * FROM ( SELECT * FROM Students order by 1 ) WHERE rownum <= 2 ;

SELECT * FROM Students WHERE rownum > 1 ;

SELECT * FROM Students ;

update Students set id = id + 5 WHERE rownum < 4 ;

SELECT * FROM Students ;

drop table Students ;

create table test ( a int , b int );

insert into test select generate_series , generate_series from generate_series ( 1 , 10 );

EXPLAIN SELECT a , rownum FROM test group by a , rownum having rownum < 5 ;

EXPLAIN SELECT * FROM ( SELECT * FROM test WHERE rownum < 5 ) WHERE b < 5 ;

drop table test ;

