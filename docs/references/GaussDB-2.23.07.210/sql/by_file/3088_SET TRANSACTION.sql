-- 来源: 3088_SET TRANSACTION.txt
-- SQL 数量: 9

CREATE DATABASE mysql_compatible_db DBCOMPATIBILITY 'B';

--开启一个事务，设置事务的隔离级别为READ COMMITTED，访问模式为READ ONLY。
START TRANSACTION;

SET LOCAL TRANSACTION ISOLATION LEVEL READ COMMITTED READ ONLY;

COMMIT;

--设置当前会话的事务隔离级别、读写模式。
--在sql_compatibility = 'B'场景下,b_format_behavior_compat_options设置为set_session_transaction。
SET SESSION TRANSACTION ISOLATION LEVEL READ COMMITTED;

SET SESSION TRANSACTION READ ONLY;

--给sql_compatibility = 'B'的数据库设置全局会话的事务隔离级别、读写模式(当前只能在sql_compatibility = 'B'场景下)。
SET GLOBAL TRANSACTION ISOLATION LEVEL READ COMMITTED;

SET GLOBAL TRANSACTION READ ONLY;

DROP DATABASE mysql_compatible_db;

