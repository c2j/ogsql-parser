-- 来源: 2774_file_2774.txt
-- SQL 数量: 197

SELECT date '2001-10-01' - '7' AS RESULT ;

SELECT date '2001-9-28' + integer '7' AS RESULT ;

SELECT date '2001-09-28' + interval '1 hour' AS RESULT ;

SELECT date '2001-09-28' + time '03:00' AS RESULT ;

SELECT interval '1 day' + interval '1 hour' AS RESULT ;

SELECT timestamp '2001-09-28 01:00' + interval '23 hours' AS RESULT ;

SELECT time '01:00' + interval '3 hours' AS RESULT ;

SELECT date '2001-10-01' - date '2001-09-28' AS RESULT ;

SELECT date '2001-10-01' - integer '7' AS RESULT ;

SELECT date '2001-09-28' - interval '1 hour' AS RESULT ;

SELECT time '05:00' - time '03:00' AS RESULT ;

SELECT time '05:00' - interval '2 hours' AS RESULT ;

SELECT timestamp '2001-09-28 23:00' - interval '23 hours' AS RESULT ;

SELECT interval '1 day' - interval '1 hour' AS RESULT ;

SELECT timestamp '2001-09-29 03:00' - timestamp '2001-09-27 12:00' AS RESULT ;

SELECT 900 * interval '1 second' AS RESULT ;

SELECT 21 * interval '1 day' AS RESULT ;

SELECT double precision '3.5' * interval '1 hour' AS RESULT ;

SELECT interval '1 hour' / double precision '1.5' AS RESULT ;

SELECT age ( timestamp '2001-04-10' , timestamp '1957-06-13' );

SELECT age ( timestamp '1957-06-13' );

SELECT clock_timestamp ();

SELECT current_date ;

SELECT current_time ;

SELECT current_timestamp ;

SELECT current_timestamp ;

SELECT current_timestamp ();

SELECT current_timestamp ( 1 );

SELECT current_timestamp ( 1 );

SELECT pg_systimestamp ();

SELECT date_part ( 'hour' , timestamp '2001-02-16 20:38:40' );

SELECT date_part ( 'month' , interval '2 years 3 months' );

SELECT date_trunc ( 'hour' , timestamp '2001-02-16 20:38:40' );

SELECT trunc ( timestamp '2001-02-16 20:38:40' );

SELECT trunc ( timestamp '2001-02-16 20:38:40' , 'hour' );

SELECT round ( timestamp '2001-02-16 20:38:40' , 'hour' );

SELECT daterange ( '2000-05-06' , '2000-08-08' );

SELECT daterange ( '2000-05-06' , '2000-08-08' , '[]' );

SELECT isfinite ( date '2001-02-16' );

SELECT isfinite ( date 'infinity' );

SELECT isfinite ( timestamp '2001-02-16 21:28:30' );

SELECT isfinite ( timestamp 'infinity' );

SELECT isfinite ( interval '4 hours' );

SELECT justify_days ( interval '35 days' );

SELECT JUSTIFY_HOURS ( INTERVAL '27 HOURS' );

SELECT JUSTIFY_INTERVAL ( INTERVAL '1 MON -1 HOUR' );

SELECT localtime AS RESULT ;

SELECT localtimestamp ;

SELECT maketime ( 8 , 15 , 26 . 53 );

SELECT maketime ( - 888 , 15 , 26 . 53 );

SELECT now ();

SELECT timenow ();

SELECT dbtimezone ;

SELECT numtodsinterval ( 100 , 'HOUR' );

SET intervalstyle = a ;

SELECT numtodsinterval ( 100 , 'HOUR' );

SELECT numtoyminterval ( 100 , 'MONTH' );

SET intervalstyle = oracle ;

SELECT numtodsinterval ( 100 , 'MONTH' );

SELECT new_time ( '1997-10-10' , 'AST' , 'EST' );

SELECT NEW_TIME ( TO_TIMESTAMP ( '10-Sep-02 14:10:10.123000' , 'DD-Mon-RR HH24:MI:SS.FF' ), 'AST' , 'PST' );

SELECT SESSIONTIMEZONE ;

SELECT LOWER ( SESSIONTIMEZONE );

SELECT SYS_EXTRACT_UTC ( TIMESTAMP '2000-03-28 11:30:00.00' );

SELECT SYS_EXTRACT_UTC ( TIMESTAMPTZ '2000-03-28 11:30:00.00 -08:00' );

SELECT TZ_OFFSET ( 'US/Pacific' );

SELECT TZ_OFFSET ( sessiontimezone );

SELECT pg_sleep ( 10 );

SELECT statement_timestamp ();

SELECT sysdate ;

SELECT current_sysdate ();

SELECT timeofday ();

SELECT transaction_timestamp ();

SELECT transaction_timestamp ();

SELECT add_months ( to_date ( '2017-5-29' , 'yyyy-mm-dd' ), 11 ) FROM sys_dummy ;

SELECT last_day ( to_date ( '2017-01-01' , 'YYYY-MM-DD' )) AS cal_result ;

SELECT months_between(to_date('2022-10-31', 'yyyy-mm-dd'), to_date('2022-09-30', 'yyyy-mm-dd'));

SELECT months_between(to_date('2022-10-30', 'yyyy-mm-dd'), to_date('2022-09-30', 'yyyy-mm-dd'));

SELECT months_between(to_date('2022-10-29', 'yyyy-mm-dd'), to_date('2022-09-30', 'yyyy-mm-dd'));

SELECT next_day ( timestamp '2017-05-25 00:00:00' , 'Sunday' ) AS cal_result ;

SELECT next_day ( timestamp '2024-01-17 00:00:00' , 7 . 9999999 );

SET a_format_version = '10c' ;

SET a_format_dev_version = 's1' ;

SELECT next_day ( timestamp '2024-01-17 00:00:00' , 7 . 9999999 );

CALL tinterval ( abstime 'May 10, 1947 23:59:12' , abstime 'Mon May 1 00:30:30 1995' );

SELECT tintervalend ( '["Sep 4, 1983 23:59:12" "Oct4, 1983 23:59:12"]' );

SELECT tintervalrel ( '["Sep 4, 1983 23:59:12" "Oct4, 1983 23:59:12"]' );

CREATE USER JIM PASSWORD '*********' ;

CREATE DATABASE testdb3 OWNER JIM DBCOMPATIBILITY = 'B' ;

\ c testdb3 testdb3 =# SET b_format_dev_version = 's1' ;

CREATE USER JIM PASSWORD '*********' ;

CREATE DATABASE testdb3 OWNER JIM DBCOMPATIBILITY = 'B' ;

\ c testdb3 testdb3 =# SET b_format_dev_version = 's1' ;

SELECT ADDDATE ( '2018-05-01' , INTERVAL 1 DAY );

SELECT ADDDATE('2018-05-01', 1);

SELECT curdate ();

SELECT curtime ( 3 );

SELECT DATE_ADD('2018-05-01', INTERVAL 1 DAY);

SELECT DATE_ADD('2018-05-01', 1);

SELECT date_format('2023-10-11 12:13:14.151617','%b %c %M %m');

SELECT DATE_SUB('2018-05-01', INTERVAL 1 YEAR);

SELECT DATE_SUB('2023-1-1', 20);

SELECT datediff('2021-11-12','2021-11-13');

SELECT day('2023-01-02');

SELECT dayofmonth('23-05-22');

SELECT dayname('2023-10-11');

SELECT dayofweek('2023-04-16');

SELECT dayofyear('2000-12-31');

SELECT extract(YEAR FROM '2023-10-11');

SELECT extract(QUARTER FROM '2023-10-11');

SELECT extract(MONTH FROM '2023-10-11');

SELECT extract(WEEK FROM '2023-10-11');

SELECT extract(DAY FROM '2023-10-11');

SELECT extract(HOUR FROM '2023-10-11 12:13:14');

SELECT from_days(36524);

SELECT from_unixtime(1111885200);

SELECT get_format(date, 'eur');

SELECT get_format(date, 'usa');

SELECT HOUR('10:10:10.1');

SELECT makedate(2000, 60);

SELECT MICROSECOND('2023-5-5 10:10:10.24485');

SELECT MINUTE(time'10:10:10');

SELECT month('2021-11-30');

SELECT monthname('2023-02-28');

SELECT period_add(202205, -12);

SELECT period_diff('202101', '202102');

SELECT SECOND('2023-5-5 10:10:10');

SELECT QUARTER('2012-1-1');

SELECT str_to_date('May 1, 2013','%M %d,%Y');

SELECT SUBDATE('2023-1-1', 20);

SELECT SUBDATE('2018-05-01', INTERVAL 1 YEAR);

SELECT subtime('2000-03-01 20:59:59', '22:58');

SELECT addtime('2000-03-01 20:59:59', '00:00:01');

SELECT TIME_FORMAT('25:30:30', '%T|%r|%H|%h|%I|%i|%S|%f|%p|%k');

SELECT time_to_sec('00:00:01');

SELECT timediff(date'2022-12-30',20221229);

SELECT TIMESTAMPADD(DAY,-2,'2022-07-27');

SELECT to_days('2000-1-1');

SELECT TO_SECONDS('2009-11-29 13:43:32');

SELECT UNIX_TIMESTAMP('2022-12-22');

SELECT utc_date();

SELECT utc_time();

SELECT utc_timestamp();

SELECT week(date'2000-01-01', 1);

SELECT week('2000-01-01', 2);

SELECT weekday('1970-01-01 12:00:00');

SELECT weekofyear('1970-05-22');

SELECT year('23-05-22');

SELECT yearweek(datetime'2000-01-01', 3);

SELECT timestamp_diff ( 'year' , '2018-01-01' , '2020-04-01' );

SELECT timestamp_diff ( 'month' , '2018-01-01' , '2020-04-01' );

SELECT timestamp_diff ( 'quarter' , '2018-01-01' , '2020-04-01' );

SELECT timestamp_diff ( 'week' , '2018-01-01' , '2020-04-01' );

SELECT timestamp_diff ( 'day' , '2018-01-01' , '2020-04-01' );

SELECT timestamp_diff ( 'hour' , '2018-01-01 10:10:10' , '2018-01-01 12:12:12' );

SELECT timestamp_diff ( 'minute' , '2018-01-01 10:10:10' , '2018-01-01 12:12:12' );

SELECT timestamp_diff ( 'second' , '2018-01-01 10:10:10' , '2018-01-01 10:12:12' );

SELECT timestamp_diff ( 'microsecond' , '2018-01-01 10:10:10' , '2018-01-01 10:12:12' );

SELECT TIMESTAMPDIFF ( YEAR , '2018-01-01' , '2020-01-01' );

SELECT TIMESTAMPDIFF ( QUARTER , '2018-01-01' , '2020-01-01' );

SELECT TIMESTAMPDIFF ( MONTH , '2018-01-01' , '2020-01-01' );

SELECT TIMESTAMPDIFF ( WEEK , '2018-01-01' , '2020-01-01' );

SELECT TIMESTAMPDIFF ( DAY , '2018-01-01' , '2020-01-01' );

SELECT TIMESTAMPDIFF ( HOUR , '2020-01-01 10:10:10' , '2020-01-01 11:11:11' );

SELECT TIMESTAMPDIFF ( MINUTE , '2020-01-01 10:10:10' , '2020-01-01 11:11:11' );

SELECT TIMESTAMPDIFF ( SECOND , '2020-01-01 10:10:10' , '2020-01-01 11:11:11' );

SELECT TIMESTAMPDIFF ( MICROSECOND , '2020-01-01 10:10:10.000000' , '2020-01-01 10:10:10.111111' );

SELECT EXTRACT ( CENTURY FROM TIMESTAMP '2000-12-16 12:21:13' );

SELECT EXTRACT ( DAY FROM TIMESTAMP '2001-02-16 20:38:40' );

SELECT EXTRACT ( DAY FROM INTERVAL '40 days 1 minute' );

SELECT EXTRACT ( DECADE FROM TIMESTAMP '2001-02-16 20:38:40' );

SELECT EXTRACT ( DOW FROM TIMESTAMP '2001-02-16 20:38:40' );

SELECT EXTRACT ( DOY FROM TIMESTAMP '2001-02-16 20:38:40' );

SELECT EXTRACT ( EPOCH FROM TIMESTAMP WITH TIME ZONE '2001-02-16 20:38:40.12-08' );

SELECT EXTRACT ( EPOCH FROM INTERVAL '5 days 3 hours' );

SELECT TIMESTAMP WITH TIME ZONE 'epoch' + 982384720 . 12 * INTERVAL '1 second' AS RESULT ;

SELECT EXTRACT ( HOUR FROM TIMESTAMP '2001-02-16 20:38:40' );

SELECT EXTRACT ( ISODOW FROM TIMESTAMP '2001-02-18 20:38:40' );

SELECT EXTRACT ( ISOYEAR FROM DATE '2006-01-01' );

SELECT EXTRACT ( WEEK FROM TIMESTAMP '2006-01-01 00:00:40' );

SELECT EXTRACT(ISOYEAR FROM DATE '2006-01-02');

SELECT EXTRACT(WEEK FROM TIMESTAMP '2006-01-02 00:00:40');

SELECT EXTRACT ( MICROSECONDS FROM TIME '17:12:28.5' );

SELECT EXTRACT ( MILLENNIUM FROM TIMESTAMP '2001-02-16 20:38:40' );

SELECT EXTRACT ( MILLISECONDS FROM TIME '17:12:28.5' );

SELECT EXTRACT ( MINUTE FROM TIMESTAMP '2001-02-16 20:38:40' );

SELECT EXTRACT ( MONTH FROM TIMESTAMP '2001-02-16 20:38:40' );

SELECT EXTRACT ( MONTH FROM INTERVAL '2 years 13 months' );

SELECT EXTRACT ( QUARTER FROM TIMESTAMP '2001-02-16 20:38:40' );

SELECT EXTRACT ( SECOND FROM TIME '17:12:28.5' );

SELECT EXTRACT ( ISOYEAR FROM DATE '2006-01-01' );

SELECT EXTRACT ( WEEK FROM TIMESTAMP '2006-01-01 00:00:40' );

SELECT EXTRACT(ISOYEAR FROM DATE '2006-01-02');

SELECT EXTRACT(WEEK FROM TIMESTAMP '2006-01-02 00:00:40');

SELECT EXTRACT ( YEAR FROM TIMESTAMP '2001-02-16 20:38:40' );

SELECT date_part ( 'day' , TIMESTAMP '2001-02-16 20:38:40' );

SELECT date_part ( 'hour' , INTERVAL '4 hours 3 minutes' );

