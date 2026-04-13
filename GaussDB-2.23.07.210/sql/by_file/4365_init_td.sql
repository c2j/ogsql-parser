-- 来源: 4365_init_td.txt
-- SQL 数量: 4

CREATE TABLE test1(name varchar) WITH(storage_type = ustore, init_td=2);

ALTER TABLE test1 SET(init_td=8);

SELECT * FROM pg_thread_wait_status;

DROP TABLE test1;

