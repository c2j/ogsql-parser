-- 来源: 4654_file_4654.txt
-- SQL 数量: 15

drop TABLE IF EXISTS "public".flashtest;

CREATE TABLE "public".flashtest (col1 INT,col2 TEXT) with(storage_type=ustore);

select int8in(xidout(next_csn)) from gs_get_next_xid_csn();

select now();

SELECT * FROM flashtest;

INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

SELECT * FROM flashtest;

TIMECAPSULE TABLE flashtest TO TIMESTAMP to_timestamp ('2023-09-13 19:52:21.551028', 'YYYY-MM-DD HH24:MI:SS.FF');

SELECT * FROM flashtest;

select now();

INSERT INTO flashtest VALUES(1,'INSERT1'),(2,'INSERT2'),(3,'INSERT3'),(4,'INSERT4'),(5,'INSERT5'),(6,'INSERT6');

SELECT * FROM flashtest;

TIMECAPSULE TABLE flashtest TO TIMESTAMP '2023-09-13 19:54:00.641506';

SELECT * FROM flashtest;

drop TABLE IF EXISTS "public".flashtest;

