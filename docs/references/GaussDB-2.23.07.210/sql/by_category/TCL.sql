-- 类别: TCL
-- SQL 数量: 158

-- 来源: 1110_file_1110
BEGIN;

-- 来源: 1184_ABORT
START TRANSACTION ;

-- 来源: 1184_ABORT
ABORT ;

-- 来源: 1212_ALTER SESSION
START TRANSACTION ;

-- 来源: 1227_BEGIN
BEGIN dbe_output . print_line ( 'Hello' );

-- 来源: 1235_COMMIT _ END
START TRANSACTION ;

-- 来源: 1235_COMMIT _ END
COMMIT ;

-- 来源: 1236_COMMIT PREPARED
BEGIN ;

-- 来源: 1236_COMMIT PREPARED
PREPARE TRANSACTION 'trans_test' ;

-- 来源: 1236_COMMIT PREPARED
COMMIT PREPARED 'trans_test' ;

--匿名块里调用package存储过程
-- 来源: 1259_CREATE PACKAGE
BEGIN emp_bonus.testpro1(1);

-- 来源: 1335_FETCH
START TRANSACTION ;

-- 来源: 1335_FETCH
START TRANSACTION ;

-- 来源: 1335_FETCH
START TRANSACTION ;

-- 来源: 1345_LOCK
START TRANSACTION ;

-- 来源: 1345_LOCK
COMMIT ;

-- 来源: 1346_LOCK BUCKETS
START TRANSACTION ;

-- 来源: 1346_LOCK BUCKETS
COMMIT ;

-- 来源: 1350_MOVE
START TRANSACTION ;

-- 来源: 1361_RELEASE SAVEPOINT
START TRANSACTION ;

-- 来源: 1361_RELEASE SAVEPOINT
SAVEPOINT my_savepoint ;

-- 来源: 1361_RELEASE SAVEPOINT
RELEASE SAVEPOINT my_savepoint ;

-- 来源: 1361_RELEASE SAVEPOINT
COMMIT ;

-- 来源: 1365_ROLLBACK
START TRANSACTION ;

-- 来源: 1365_ROLLBACK
ROLLBACK ;

-- 来源: 1367_ROLLBACK TO SAVEPOINT
START TRANSACTION ;

-- 来源: 1367_ROLLBACK TO SAVEPOINT
SAVEPOINT my_savepoint ;

-- 来源: 1367_ROLLBACK TO SAVEPOINT
ROLLBACK TO SAVEPOINT my_savepoint ;

-- 来源: 1367_ROLLBACK TO SAVEPOINT
SAVEPOINT foo ;

-- 来源: 1367_ROLLBACK TO SAVEPOINT
ROLLBACK TO SAVEPOINT foo ;

-- 来源: 1367_ROLLBACK TO SAVEPOINT
RELEASE SAVEPOINT my_savepoint ;

-- 来源: 1367_ROLLBACK TO SAVEPOINT
COMMIT ;

-- 来源: 1369_SAVEPOINT
START TRANSACTION ;

-- 来源: 1369_SAVEPOINT
SAVEPOINT my_savepoint ;

-- 来源: 1369_SAVEPOINT
ROLLBACK TO SAVEPOINT my_savepoint ;

-- 来源: 1369_SAVEPOINT
COMMIT ;

-- 来源: 1369_SAVEPOINT
START TRANSACTION ;

-- 来源: 1369_SAVEPOINT
SAVEPOINT my_savepoint ;

-- 来源: 1369_SAVEPOINT
RELEASE SAVEPOINT my_savepoint ;

-- 来源: 1369_SAVEPOINT
COMMIT ;

-- 来源: 1371_SELECT
BEGIN ;

-- 来源: 1377_SET TRANSACTION
START TRANSACTION ;

-- 来源: 1377_SET TRANSACTION
COMMIT ;

-- 来源: 1380_START TRANSACTION
START TRANSACTION ;

-- 来源: 1380_START TRANSACTION
BEGIN ;

-- 来源: 1380_START TRANSACTION
START TRANSACTION ISOLATION LEVEL READ COMMITTED READ WRITE ;

-- 来源: 1380_START TRANSACTION
COMMIT ;

-- 来源: 1428_file_1428
BEGIN NULL ;

-- 来源: 1428_file_1428
BEGIN dbe_output . print_line ( 'hello world!' );

-- 来源: 1452_file_1452
BEGIN;

-- 来源: 1452_file_1452
BEGIN;

-- 来源: 1452_file_1452
COMMIT;

-- 来源: 1452_file_1452
BEGIN;

-- 来源: 1452_file_1452
ROLLBACK TO SAVEPOINT s1;

-- 来源: 1452_file_1452
COMMIT;

-- 来源: 1452_file_1452
BEGIN;

-- 来源: 1460_file_1460
BEGIN FOR ROW_TRANS IN SELECT first_name FROM hr . staffs LOOP DBE_OUTPUT . PRINT_LINE ( ROW_TRANS . first_name );

-- 来源: 1480_DBE_TASK
BEGIN gaussdb $ # DBE_TASK . ID_SUBMIT ( 12345 , 'insert_msg_statistic1;

-- 来源: 1480_DBE_TASK
BEGIN gaussdb $ # DBE_TASK . CHANGE ( gaussdb $ # job => 101 , gaussdb $ # what => 'insert into t2 values (2);

-- 来源: 1489_file_1489
begin;

-- 来源: 1489_file_1489
START TRANSACTION;

-- 来源: 1491_file_1491
START TRANSACTION;

-- 来源: 1491_file_1491
ROLLBACK;

-- 来源: 2431_file_2431
start transaction isolation level repeatable read;

-- 来源: 2431_file_2431
start transaction isolation level repeatable read;

-- 来源: 2431_file_2431
commit;

-- 来源: 2431_file_2431
commit;

-- 来源: 2431_file_2431
start transaction isolation level repeatable read;

-- 来源: 2431_file_2431
start transaction isolation level repeatable read;

-- 来源: 2431_file_2431
commit;

-- 来源: 2808_file_2808
BEGIN;

--开启事务。
-- 来源: 2882_ABORT
START TRANSACTION;

--终止事务，上面所执行的更新会被撤销掉。
-- 来源: 2882_ABORT
ABORT;

--开启事务,设置事务级别
-- 来源: 2912_ALTER SESSION
START TRANSACTION;

-- 来源: 2929_BEGIN
BEGIN gaussdb $ # dbe_output . print_line ( 'Hello' );

--开启事务。
-- 来源: 2937_COMMIT _ END
START TRANSACTION;

--提交事务，让所有更改永久化。
-- 来源: 2937_COMMIT _ END
COMMIT;

-- 来源: 2938_COMMIT PREPARED
BEGIN;

--准备标识符为的trans_test的事务。
-- 来源: 2938_COMMIT PREPARED
PREPARE TRANSACTION 'trans_test';

--提交标识符为的trans_test的事务。
-- 来源: 2938_COMMIT PREPARED
COMMIT PREPARED 'trans_test';

--匿名块里调用PACKAGE存储过程
-- 来源: 2962_CREATE PACKAGE
BEGIN emp_bonus.testpro1(1);

--SELECT语句，用一个游标读取一个表。开始一个事务。
-- 来源: 3046_FETCH
START TRANSACTION;

--VALUES子句，用一个游标读取VALUES子句中的内容。开始一个事务。
-- 来源: 3046_FETCH
START TRANSACTION;

--WITH HOLD游标的使用，开启事务。
-- 来源: 3046_FETCH
START TRANSACTION;

-- 来源: 3056_LOCK
START TRANSACTION;

-- 来源: 3056_LOCK
COMMIT;

--开始一个事务。
-- 来源: 3061_MOVE
START TRANSACTION;

-- 来源: 3065_PREPARE TRANSACTION
BEGIN;

--准备标识符为的trans_test的事务。
-- 来源: 3065_PREPARE TRANSACTION
PREPARE TRANSACTION 'trans_test';

--取消标识符为的trans_test的事务。
-- 来源: 3065_PREPARE TRANSACTION
ROLLBACK PREPARED 'trans_test';

--开启事务。
-- 来源: 3072_RELEASE SAVEPOINT
START TRANSACTION;

--建立保存点。
-- 来源: 3072_RELEASE SAVEPOINT
SAVEPOINT my_savepoint;

--删除保存点。
-- 来源: 3072_RELEASE SAVEPOINT
RELEASE SAVEPOINT my_savepoint;

--提交事务。
-- 来源: 3072_RELEASE SAVEPOINT
COMMIT;

-- 来源: 3076_ROLLBACK
START TRANSACTION;

--取消所有更改
-- 来源: 3076_ROLLBACK
ROLLBACK;

-- 来源: 3077_ROLLBACK PREPARED
BEGIN;

--准备标识符为的trans_test的事务。
-- 来源: 3077_ROLLBACK PREPARED
PREPARE TRANSACTION 'trans_test';

--取消标识符为的trans_test的事务。
-- 来源: 3077_ROLLBACK PREPARED
ROLLBACK PREPARED 'trans_test';

-- 来源: 3078_ROLLBACK TO SAVEPOINT
START TRANSACTION;

-- 来源: 3078_ROLLBACK TO SAVEPOINT
SAVEPOINT my_savepoint;

-- 来源: 3078_ROLLBACK TO SAVEPOINT
ROLLBACK TO SAVEPOINT my_savepoint;

-- 来源: 3078_ROLLBACK TO SAVEPOINT
SAVEPOINT foo;

-- 来源: 3078_ROLLBACK TO SAVEPOINT
ROLLBACK TO SAVEPOINT foo;

-- 来源: 3078_ROLLBACK TO SAVEPOINT
RELEASE SAVEPOINT my_savepoint;

-- 来源: 3078_ROLLBACK TO SAVEPOINT
COMMIT;

--开启事务。
-- 来源: 3080_SAVEPOINT
START TRANSACTION;

--建立保存点。
-- 来源: 3080_SAVEPOINT
SAVEPOINT my_savepoint;

--回滚保存点。
-- 来源: 3080_SAVEPOINT
ROLLBACK TO SAVEPOINT my_savepoint;

--提交事务。
-- 来源: 3080_SAVEPOINT
COMMIT;

--开启事务。
-- 来源: 3080_SAVEPOINT
START TRANSACTION;

--建立保存点。
-- 来源: 3080_SAVEPOINT
SAVEPOINT my_savepoint;

--回滚保存点。
-- 来源: 3080_SAVEPOINT
RELEASE SAVEPOINT my_savepoint;

--提交事务。
-- 来源: 3080_SAVEPOINT
COMMIT;

--step 2:session1开启事务通过UPDATE锁住skiplocked_astore中id等于1的行
-- 来源: 3082_SELECT
BEGIN;

--开启一个事务，设置事务的隔离级别为READ COMMITTED，访问模式为READ ONLY。
-- 来源: 3088_SET TRANSACTION
START TRANSACTION;

-- 来源: 3088_SET TRANSACTION
COMMIT;

--以默认方式启动事务。
-- 来源: 3094_START TRANSACTION
START TRANSACTION;

--以默认方式启动事务。
-- 来源: 3094_START TRANSACTION
BEGIN;

--以隔离级别为READ COMMITTED，读/写方式启动事务。
-- 来源: 3094_START TRANSACTION
START TRANSACTION ISOLATION LEVEL READ COMMITTED READ WRITE;

-- 来源: 3094_START TRANSACTION
COMMIT;

-- 来源: 3138_file_3138
BEGIN NULL ;

-- 来源: 3138_file_3138
BEGIN dbe_output . print_line ( 'hello world!' );

-- 来源: 3162_file_3162
BEGIN;

-- 来源: 3162_file_3162
BEGIN;

-- 来源: 3162_file_3162
BEGIN;

-- 来源: 3162_file_3162
BEGIN;

-- 来源: 3170_file_3170
BEGIN FOR ROW_TRANS IN SELECT first_name FROM hr . staffs LOOP DBE_OUTPUT . PRINT_LINE ( ROW_TRANS . first_name );

-- 来源: 3193_DBE_TASK
BEGIN gaussdb $ # DBE_TASK . ID_SUBMIT ( 12345 , 'insert_msg_statistic1;

-- 来源: 3193_DBE_TASK
BEGIN gaussdb $ # DBE_TASK . CHANGE ( gaussdb $ # job => 101 , gaussdb $ # what => 'insert into t2 values (2);

-- 来源: 3202_file_3202
begin;

-- 来源: 3204_file_3204
START TRANSACTION;

-- 来源: 3204_file_3204
ROLLBACK;

-- 来源: 4407_file_4407
BEGIN DBE_ILM.EXECUTE_ILM(OWNER => 'public', OBJECT_NAME => 'ilm_table_1', TASK_ID => v_taskid, SUBOBJECT_NAME => NULL, POLICY_NAME => 'ALL POLICIES', EXECUTION_MODE => 2);

-- 来源: 4522_DDL
BEGIN;

-- 来源: 4522_DDL
COMMIT;

-- 只反解析第一句和第三句SQL语句
-- 来源: 4522_DDL
BEGIN;

-- 来源: 4522_DDL
COMMIT;

-- 只反解析第一句和第二句SQL语句
-- 来源: 4522_DDL
BEGIN;

-- 来源: 4522_DDL
COMMIT;

-- 全反解析
-- 来源: 4522_DDL
BEGIN;

-- 来源: 4522_DDL
COMMIT;

-- 只反解析第一句和第三句SQL语句
-- 来源: 4522_DDL
BEGIN;

-- 来源: 4522_DDL
COMMIT;

-- 全反解析
-- 来源: 4522_DDL
BEGIN;

-- 来源: 4522_DDL
COMMIT;

-- 只反解析第一句SQL语句
-- 来源: 4522_DDL
BEGIN;

-- 来源: 4522_DDL
COMMIT;

-- 只反解析第一句和第三句SQL语句
-- 来源: 4522_DDL
BEGIN;

-- 来源: 4522_DDL
COMMIT;

-- 来源: 4667_file_4667
BEGIN DBE_ILM.EXECUTE_ILM(OWNER => 'public', OBJECT_NAME => 'ilm_table_1', TASK_ID => v_taskid, SUBOBJECT_NAME => NULL, POLICY_NAME => 'ALL POLICIES', EXECUTION_MODE => 2);

-- 来源: 733_file_733
start transaction isolation level repeatable read;

-- 来源: 733_file_733
start transaction isolation level repeatable read;

-- 来源: 733_file_733
commit;

-- 来源: 733_file_733
commit;

-- 来源: 733_file_733
start transaction isolation level repeatable read;

-- 来源: 733_file_733
start transaction isolation level repeatable read;

-- 来源: 733_file_733
commit;

