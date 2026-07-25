-- 来源: 4653_file_4653.txt
-- SQL 数量: 13

drop TABLE IF EXISTS "public".flashtest;

CREATE TABLE "public".flashtest (col1 INT,col2 TEXT) with(storage_type=ustore);

select int8in(xidout(next_csn)) from gs_get_next_xid_csn();

select now();

INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

SELECT * FROM flashtest;

SELECT * FROM flashtest TIMECAPSULE CSN 79351682;

SELECT * FROM flashtest;

SELECT * FROM flashtest TIMECAPSULE TIMESTAMP '2023-09-13 19:35:26.011986';

SELECT * FROM flashtest;

SELECT * FROM flashtest TIMECAPSULE TIMESTAMP to_timestamp ('2023-09-13 19:35:26.011986', 'YYYY-MM-DD HH24:MI:SS.FF');

SELECT * FROM flashtest AS ft TIMECAPSULE CSN 79351682;

drop TABLE IF EXISTS "public".flashtest;

