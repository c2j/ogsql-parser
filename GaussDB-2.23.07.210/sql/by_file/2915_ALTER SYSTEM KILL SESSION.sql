-- 来源: 2915_ALTER SYSTEM KILL SESSION.txt
-- SQL 数量: 2

SELECT sa.sessionid AS sid,0::integer AS serial#,ad.rolname AS username FROM pg_stat_get_activity(NULL) AS sa LEFT JOIN pg_authid ad ON(sa.usesysid = ad.oid)WHERE sa.application_name <> 'JobScheduler';

--结束SID为140131075880720的会话。
ALTER SYSTEM KILL SESSION '140131075880720,0' IMMEDIATE;

