-- 来源: 2748__.txt
-- SQL 数量: 52

CREATE TABLE date_type_tab ( coll date );

INSERT INTO date_type_tab VALUES ( date '12-10-2010' );

SELECT * FROM date_type_tab ;

DROP TABLE date_type_tab ;

CREATE TABLE time_type_tab ( da time without time zone , dai time with time zone , dfgh timestamp without time zone , dfga timestamp with time zone , vbg smalldatetime );

INSERT INTO time_type_tab VALUES ( '21:21:21' , '21:21:21 pst' , '2010-12-12' , '2013-12-11 pst' , '2003-04-12 04:05:06' );

SELECT * FROM time_type_tab ;

DROP TABLE time_type_tab ;

CREATE TABLE day_type_tab ( a int , b INTERVAL DAY ( 3 ) TO SECOND ( 4 ));

INSERT INTO day_type_tab VALUES ( 1 , INTERVAL '3' DAY );

SELECT * FROM day_type_tab ;

DROP TABLE day_type_tab ;

CREATE TABLE year_type_tab ( a int , b interval year ( 6 ));

INSERT INTO year_type_tab VALUES ( 1 , interval '2' year );

SELECT * FROM year_type_tab ;

DROP TABLE year_type_tab ;

create database gaussdb_m dbcompatibility = 'B' ;

CREATE TABLE date_type_tab ( coll date );

INSERT INTO date_type_tab VALUES ( date '12-10-2010' );

SELECT * FROM date_type_tab ;

SHOW datestyle ;

SET datestyle = 'YMD' ;

INSERT INTO date_type_tab VALUES ( date '2010-12-11' );

SELECT * FROM date_type_tab ;

DROP TABLE date_type_tab ;

SELECT time '04:05:06' ;

SELECT time '04:05:06 PST' ;

SELECT time with time zone '04:05:06 PST' ;

CREATE TABLE realtime_type_special(col1 varchar(20), col2 date, col3 timestamp, col4 time);

--插入数据。
INSERT INTO realtime_type_special VALUES('epoch', 'epoch', 'epoch', NULL);

INSERT INTO realtime_type_special VALUES('now', 'now', 'now', 'now');

INSERT INTO realtime_type_special VALUES('today', 'today', 'today', NULL);

INSERT INTO realtime_type_special VALUES('tomorrow', 'tomorrow', 'tomorrow', NULL);

INSERT INTO realtime_type_special VALUES('yesterday', 'yesterday', 'yesterday', NULL);

--查看数据。
SELECT * FROM realtime_type_special;

SELECT * FROM realtime_type_special WHERE col3 < 'infinity';

SELECT * FROM realtime_type_special WHERE col3 > '-infinity';

SELECT * FROM realtime_type_special WHERE col3 > 'now';

SELECT * FROM realtime_type_special WHERE col3 = 'today';

SELECT * FROM realtime_type_special WHERE col3 = 'tomorrow';

SELECT * FROM realtime_type_special WHERE col3 > 'yesterday';

SELECT TIME 'allballs';

--删除表。
DROP TABLE realtime_type_special;

CREATE TABLE reltime_type_tab ( col1 character ( 30 ), col2 reltime );

INSERT INTO reltime_type_tab VALUES ( '90' , '90' );

INSERT INTO reltime_type_tab VALUES ( '-366' , '-366' );

INSERT INTO reltime_type_tab VALUES ( '1975.25' , '1975.25' );

INSERT INTO reltime_type_tab VALUES ( '-2 YEARS +5 MONTHS 10 DAYS' , '-2 YEARS +5 MONTHS 10 DAYS' );

INSERT INTO reltime_type_tab VALUES ( '30 DAYS 12:00:00' , '30 DAYS 12:00:00' );

INSERT INTO reltime_type_tab VALUES ( 'P-1.1Y10M' , 'P-1.1Y10M' );

SELECT * FROM reltime_type_tab ;

DROP TABLE reltime_type_tab ;

