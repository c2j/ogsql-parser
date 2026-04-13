-- 来源: 1214_ALTER SYSTEM KILL SESSION.txt
-- SQL 数量: 2

SELECT sid , serial # , username FROM dv_sessions WHERE sid IN ( SELECT pid FROM pg_stat_activity );

ALTER SYSTEM KILL SESSION '140469417232128,0' IMMEDIATE ;

