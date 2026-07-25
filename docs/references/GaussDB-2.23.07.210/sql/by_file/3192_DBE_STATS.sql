-- 来源: 3192_DBE_STATS.txt
-- SQL 数量: 101

CREATE SCHEMA dbe_stats_lock;

SET CURRENT_SCHEMA=dbe_stats_lock;

CREATE TABLE t1(a int,b int);

-- 锁定表，查看其锁定状态
CALL DBE_STATS.LOCK_TABLE_STATS(ownname=>'dbe_stats_lock',tabname=>'t1');

SELECT relname,instr(reloptions::text,'stat_state=locked',1,1) <> 0 as exist_lock FROM PG_CLASS WHERE relname='t1' and relnamespace = (SELECT oid FROM PG_NAMESPACE WHERE nspname='dbe_stats_lock');

-- 锁定后analyze, 发生报错
ANALYZE t1;

-- 删除表、删除命名空间
DROP TABLE t1;

DROP SCHEMA dbe_stats_lock;

CREATE SCHEMA dbe_stats_lock;

SET CURRENT_SCHEMA=dbe_stats_lock;

CREATE TABLE upart_table(a int, b int, c int) PARTITION BY RANGE(a) ( PARTITION p1 VALUES LESS THAN(1200), PARTITION p2 VALUES LESS THAN(2400), PARTITION p3 VALUES LESS THAN(MAXVALUE) );

-- 锁定一个分区，其他分区及表不受影响
CALL DBE_STATS.LOCK_PARTITION_STATS(ownname=>'dbe_stats_lock',tabname=>'upart_table',partname=>'p1');

SELECT relname,instr(reloptions::text,'stat_state=locked',1,1) <> 0 as exist_lock FROM PG_CLASS WHERE relname='upart_table';

SELECT relname,instr(reloptions::text,'stat_state=locked',1,1) <> 0 as exist_lock FROM PG_PARTITION WHERE parentid='upart_table'::REGCLASS;

-- 删除表、命名空间
DROP TABLE upart_table;

DROP SCHEMA dbe_stats_lock;

CREATE SCHEMA dbe_stats_lock;

SET CURRENT_SCHEMA=dbe_stats_lock;

CREATE TABLE t1(a int,b int);

INSERT INTO t1 VALUES(generate_series(1,100),1);

ANALYZE t1;

-- 锁定列后，查看列的锁定状态
CALL DBE_STATS.LOCK_COLUMN_STATS(ownname=>'dbe_stats_lock',tabname=>'t1',colname=>'a');

SELECT staattnum,stastate FROM PG_STATISTIC WHERE starelid='t1'::REGCLASS;

-- 删除表、命名空间
DROP TABLE t1;

DROP SCHEMA dbe_stats_lock;

CALL DBE_STATS.LOCK_SCHEMA_STATS(ownname=>'dbe_stats_lock');

CALL DBE_STATS.UNLOCK_TABLE_STATS(ownname=>'dbe_stats_lock',tabname=>'t1');

CALL DBE_STATS.UNLOCK_PARTITION_STATS(ownname=>'dbe_stats_lock',tabname=>'upart_table',partname=>'p1');

CALL DBE_STATS.UNLOCK_COLUMN_STATS(ownname=>'dbe_stats_lock',tabname=>'t1',colname=>'a');

CALL DBE_STATS.UNLOCK_SCHEMA_STATS(ownname=>'dbe_stats_lock');

CREATE SCHEMA dbe_stats_restore;

SET CURRENT_SCHEMA=dbe_stats_restore;

CREATE TABLE t1(a int, b int);

INSERT INTO t1 VALUES(1,1);

INSERT INTO t1 VALUES(1,1);

INSERT INTO t1 VALUES(1,1);

ANALYZE t1;

INSERT INTO t1 VALUES(2,2);

INSERT INTO t1 VALUES(2,2);

INSERT INTO t1 VALUES(2,2);

ANALYZE t1;

-- 查看历史表
SELECT relname,reltuples FROM GS_TABLESTATS_HISTORY WHERE relname='t1';

-- 查看当前系统表中的统计信息
SELECT relname,reltuples FROM PG_CLASS WHERE relname='t1' AND relnamespace = (SELECT oid FROM PG_NAMESPACE WHERE nspname='dbe_stats_restore');

-- 回退到最早的统计信息，查看系统表
CALL DBE_STATS.RESTORE_TABLE_STATS(ownname=>'dbe_stats_restore',tabname=>'t1',as_of_timestamp=>((SELECT MIN(reltimestamp) FROM GS_TABLESTATS_HISTORY WHERE relname='t1') + INTERVAL '1 second'));

SELECT relname,reltuples FROM PG_CLASS WHERE relname='t1' AND relnamespace = (SELECT oid FROM PG_NAMESPACE WHERE nspname='dbe_stats_restore');

-- 删除表、删除命名空间
DROP TABLE t1;

DROP SCHEMA dbe_stats_restore;

CALL DBE_STATS.RESTORE_PARTITION_STATS(ownname=>'dbe_stats_restore',tabname=>'t1',partname=>'p1',as_of_timestamp=>((SELECT MIN(reltimestamp) FROM GS_TABLESTATS_HISTORY WHERE relname='t1') + INTERVAL '1 second'));

CREATE SCHEMA dbe_stats_restore;

SET CURRENT_SCHEMA=dbe_stats_restore;

CREATE TABLE t1(a int, b int);

INSERT INTO t1 VALUES(1,1);

INSERT INTO t1 VALUES(1,1);

INSERT INTO t1 VALUES(1,1);

ANALYZE t1;

INSERT INTO t1 VALUES(2,2);

INSERT INTO t1 VALUES(2,2);

INSERT INTO t1 VALUES(2,2);

ANALYZE t1;

-- 查看历史表里的统计信息
SELECT staattnum,stadistinct,stakind1,stanumbers1,stavalues1 FROM GS_STATISTIC_HISTORY WHERE starelid='t1'::REGCLASS ORDER BY statimestamp;

-- 查询当前系统表中的统计信息
SELECT staattnum,stadistinct,stakind1,stanumbers1,stavalues1 FROM PG_STATISTIC WHERE starelid='t1'::REGCLASS;

-- 回退到时间较早的时间节点，查询系统表中的统计信息
CALL DBE_STATS.RESTORE_COLUMN_STATS(ownname=>'dbe_stats_restore',tabname=>'t1',colname=>'a',as_of_timestamp=>((SELECT MIN(reltimestamp) FROM GS_TABLESTATS_HISTORY WHERE relname='t1') + INTERVAL '1 second'));

SELECT staattnum,stadistinct,stakind1,stanumbers1,stavalues1 FROM PG_STATISTIC WHERE starelid='t1'::REGCLASS;

-- 删除表、命名空间
DROP TABLE t1;

DROP SCHEMA dbe_stats_restore;

CREATE SCHEMA dbe_stats_restore;

SET CURRENT_SCHEMA=dbe_stats_restore;

CREATE TABLE t1(a int, b int);

INSERT INTO t1 VALUES(1,1);

INSERT INTO t1 VALUES(1,1);

INSERT INTO t1 VALUES(1,1);

ANALYZE t1;

INSERT INTO t1 VALUES(2,2);

INSERT INTO t1 VALUES(2,2);

INSERT INTO t1 VALUES(2,2);

ANALYZE t1;

SELECT relname,reltuples FROM GS_TABLESTATS_HISTORY WHERE relname='t1';

SELECT relname,reltuples FROM PG_CLASS WHERE relname='t1' AND relnamespace = (SELECT oid FROM PG_NAMESPACE WHERE nspname='dbe_stats_restore');

CALL DBE_STATS.RESTORE_SCHEMA_STATS(ownname=>'dbe_stats_restore',as_of_timestamp=>((SELECT MIN(reltimestamp) FROM GS_TABLESTATS_HISTORY WHERE relname='t1') + INTERVAL '1 second'));

SELECT relname,reltuples FROM PG_CLASS WHERE relname='t1' AND relnamespace = (SELECT oid FROM PG_NAMESPACE WHERE nspname='dbe_stats_restore');

-- 删除表、命名空间
DROP TABLE t1;

DROP SCHEMA dbe_stats_restore;

CREATE SCHEMA dbe_stats_purge;

SET CURRENT_SCHEMA=dbe_stats_purge;

CREATE TABLE t1(a int, b int);

INSERT INTO t1 VALUES(1,1);

INSERT INTO t1 VALUES(1,1);

INSERT INTO t1 VALUES(1,1);

ANALYZE t1;

INSERT INTO t1 VALUES(2,2);

INSERT INTO t1 VALUES(2,2);

INSERT INTO t1 VALUES(2,2);

ANALYZE t1;

-- 查看历史表
SELECT relname,reltuples FROM GS_TABLESTATS_HISTORY WHERE relname='t1';

-- 清除时间较早的历史统计信息，查看历史表
CALL DBE_STATS.PURGE_STATS(before_timestamp=>((SELECT MIN(reltimestamp) FROM GS_TABLESTATS_HISTORY WHERE relname='t1') + INTERVAL '1 second'));

SELECT relname,reltuples FROM GS_TABLESTATS_HISTORY WHERE relname='t1';

-- 删除表、命名空间
DROP TABLE t1;

DROP SCHEMA dbe_stats_purge;

CALL DBE_STATS.GET_STATS_HISTORY_RETENTION();

ANALYZE;

CALL DBE_STATS.GET_STATS_HISTORY_AVAILABILITY();

