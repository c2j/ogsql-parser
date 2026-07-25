-- 来源: 2948_CREATE EVENT.txt
-- SQL 数量: 11

CREATE DATABASE test_event WITH DBCOMPATIBILITY = 'b';

CREATE TABLE t_ev(num int);

--创建一个执行一次的定时任务。
CREATE EVENT IF NOT EXISTS event_e1 ON SCHEDULE AT sysdate + interval 5 second + interval 33 minute DISABLE DO insert into t_ev values(0);

--创建一个每隔一分钟执行一次的定时任务。
CREATE EVENT IF NOT EXISTS event_e2 ON SCHEDULE EVERY 1 minute DO insert into t_ev values(1);

--修改定时任务状态和待执行语句。
ALTER EVENT event_e1 ENABLE DO select 1;

--修改定时任务名。
ALTER EVENT event_e1 RENAME TO event_ee;

--查看定时任务。
SHOW EVENTS;

--删除定时任务。
DROP EVENT event_e1;

DROP EVENT event_e2;

--删除表。
DROP TABLE t_ev;

DROP DATABASE test_event;

