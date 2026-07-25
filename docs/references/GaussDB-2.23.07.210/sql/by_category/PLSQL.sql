-- 类别: PLSQL
-- SQL 数量: 634

-- 来源: 1024_SQL PATCH
call mypro();

-- 来源: 1024_SQL PATCH
call mypro();

-- 来源: 1077_file_1077
CALL tinterval ( abstime 'May 10, 1947 23:59:12' , abstime 'Mon May 1 00:30:30 1995' );

-- 来源: 1078_file_1078
call p1 ();

-- 来源: 1088_file_1088
CALL PROC ();

-- 来源: 1108_file_1108
CALL mypro1();

-- 来源: 1112_HashFunc
call hashenum ( 'good' :: b1 );

-- 来源: 1124_XML
declare xmldata xml ;

-- 来源: 1124_XML
declare xmldata xml;

-- 来源: 1125_XMLTYPE
declare a xmltype ;

-- 来源: 1125_XMLTYPE
declare xmltype_clob clob ;

-- 来源: 1125_XMLTYPE
declare xmltype_blob blob ;

-- 来源: 1125_XMLTYPE
declare xmltype_clob clob ;

-- 来源: 1125_XMLTYPE
declare xmltype_blob blob ;

-- 来源: 1229_CALL
CALL func_add_sql ( 1 , 3 );

-- 来源: 1229_CALL
CALL func_add_sql ( num1 => 1 , num2 => 3 );

-- 来源: 1229_CALL
CALL func_add_sql ( num2 : = 2 , num1 : = 3 );

-- 来源: 1229_CALL
CALL func_increment_sql ( 1 , 2 , 1 );

-- 来源: 1229_CALL
DECLARE res int ;

-- 来源: 1248_CREATE FUNCTION
DECLARE r rec ;

-- 来源: 1248_CREATE FUNCTION
DECLARE date1 date : = '2022-02-02' ;

-- 来源: 1248_CREATE FUNCTION
DECLARE date1 int : = 1 ;

--将PACKAGE emp_bonus的所属者改为omm 调用PACKAGE示例
-- 来源: 1259_CREATE PACKAGE
CALL emp_bonus.testpro1(1);

-- 来源: 1260_CREATE PROCEDURE
CALL insert_data ( 1 );

-- 来源: 1269_CREATE SYNONYM
CALL register ( 3 , 'mia' );

-- 来源: 1287_DO
DO $$ DECLARE r record ;

-- 来源: 1335_FETCH
DECLARE cursor1 CURSOR WITH HOLD FOR SELECT * FROM tpcds . customer_address ORDER BY 1 ;

-- 来源: 1367_ROLLBACK TO SAVEPOINT
DECLARE foo CURSOR FOR SELECT 1 UNION SELECT 2 ;

-- 来源: 1420_file_1420
CALL array_proc ();

-- 来源: 1421_file_1421
declare type array_integer is varray(10) of integer;

-- 来源: 1421_file_1421
declare type array_integer is varray(10) of integer;

-- 来源: 1421_file_1421
declare type array_integer is varray(10) of integer;

-- 来源: 1421_file_1421
declare type array_integer is varray(10) of integer;

-- 来源: 1421_file_1421
declare type array_integer is varray(10) of integer;

-- n大于数组元素个数, 清空数组元素
-- 来源: 1421_file_1421
declare type array_integer is varray(10) of integer;

-- 来源: 1421_file_1421
declare type array_integer is varray(10) of integer;

-- 数组未初始化
-- 来源: 1421_file_1421
declare type array_integer is varray(10) of integer;

-- 来源: 1421_file_1421
declare type array_integer is varray(10) of integer;

-- 来源: 1421_file_1421
declare type array_integer is varray(10) of integer;

-- 来源: 1421_file_1421
declare type array_integer is varray(10) of integer;

-- 来源: 1421_file_1421
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回NULL
-- 来源: 1421_file_1421
declare type varr is varray(10) of varchar(3);

-- 来源: 1421_file_1421
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回NULL
-- 来源: 1421_file_1421
declare type varr is varray(10) of varchar(3);

-- 来源: 1421_file_1421
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回NULL
-- 来源: 1421_file_1421
declare type varr is varray(10) of varchar(3);

-- 下标越界，大于数组范围
-- 来源: 1421_file_1421
declare type varr is varray(10) of varchar(3);

-- 来源: 1421_file_1421
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回NULL
-- 来源: 1421_file_1421
declare type varr is varray(10) of varchar(3);

-- 下标越界，大于数组范围
-- 来源: 1421_file_1421
declare type varr is varray(10) of varchar(3);

-- 来源: 1421_file_1421
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回false
-- 来源: 1421_file_1421
declare type varr is varray(10) of varchar(3);

-- 来源: 1423_file_1423
CALL table_proc ();

-- 来源: 1423_file_1423
CALL nest_table_proc ();

-- 来源: 1423_file_1423
CALL index_table_proc ();

-- 来源: 1423_file_1423
CALL nest_table_proc ();

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type nest is table of varchar2;

-- 来源: 1424_file_1424
declare type nest is table of varchar2 index by varchar2;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type t1 is table of int index by varchar;

-- 来源: 1424_file_1424
declare type t1 is table of int index by varchar;

-- 来源: 1424_file_1424
declare type t1 is table of int index by varchar;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type t1 is table of int index by int;

-- 来源: 1424_file_1424
declare type t1 is table of int index by varchar;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type t1 is table of int index by int;

-- 来源: 1424_file_1424
declare type t1 is table of int index by varchar;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type t1 is table of int index by varchar;

-- 来源: 1424_file_1424
declare type t1 is table of int index by varchar;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type t1 is table of int index by varchar;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
declare type t1 is table of int index by int;

-- 来源: 1424_file_1424
declare type nest is table of int;

-- 来源: 1424_file_1424
call p1 ();

-- 来源: 1424_file_1424
call p1 ();

-- 来源: 1424_file_1424
call p1 ();

-- 来源: 1424_file_1424
call p1 ();

-- 来源: 1425_record
CALL regress_record ( 'abc' );

-- 来源: 1425_record
call func ( 0 );

-- 来源: 1425_record
call func ( 1 );

-- 来源: 1428_file_1428
DECLARE my_var VARCHAR2 ( 30 );

-- 来源: 1434_file_1434
DECLARE emp_id INTEGER : = 7788 ;

-- 来源: 1434_file_1434
DECLARE emp_id INTEGER : = 7788 ;

-- 来源: 1435_file_1435
DECLARE my_id integer;

-- 来源: 1435_file_1435
DECLARE type id_list is varray(6) of customers.id%type;

-- 来源: 1436_file_1436
CALL proc_return ();

-- 来源: 1436_file_1436
CALL func_return ();

-- 来源: 1438_file_1438
DECLARE staff_count VARCHAR2 ( 20 );

-- 来源: 1438_file_1438
CALL dynamic_proc ();

-- 来源: 1438_file_1438
DECLARE name VARCHAR2 ( 20 );

-- 来源: 1439_file_1439
DECLARE section NUMBER ( 4 ) : = 280 ;

-- 来源: 1440_file_1440
DECLARE input1 INTEGER : = 1 ;

-- 来源: 1441_file_1441
CALL dynamic_proc ();

-- 来源: 1445_RETURN NEXTRETURN QUERY
call fun_for_return_next ();

-- 来源: 1445_RETURN NEXTRETURN QUERY
call fun_for_return_query ();

-- 来源: 1446_file_1446
DECLARE v_user_id integer default 1 ;

-- 来源: 1446_file_1446
DECLARE v_user_id integer default 0 ;

-- 来源: 1446_file_1446
DECLARE v_user_id integer default 1 ;

-- 来源: 1446_file_1446
DECLARE v_user_id integer default NULL ;

-- 来源: 1446_file_1446
CALL proc_control_structure ( 3 );

-- 来源: 1447_file_1447
CALL proc_loop ( 10 , 5 );

-- 来源: 1447_file_1447
CALL proc_while_loop ( 10 );

-- 来源: 1447_file_1447
CALL proc_for_loop ();

-- 来源: 1447_file_1447
CALL proc_for_loop_query ();

-- 来源: 1447_file_1447
CALL proc_forall ();

-- 来源: 1448_file_1448
CALL proc_case_branch ( 3 , 0 );

-- 来源: 1449_file_1449
DECLARE v_num integer default NULL;

-- 来源: 1450_file_1450
call fun_exp ();

-- 来源: 1451_GOTO
call GOTO_test ();

-- 来源: 1452_file_1452
call TRANSACTION_EXAMPLE();

-- 来源: 1452_file_1452
call TEST_COMMIT_INSERT_EXCEPTION_ROLLBACK();

-- 来源: 1452_file_1452
call TEST_COMMIT2();

-- 来源: 1452_file_1452
call GUC_ROLLBACK();

-- 来源: 1452_file_1452
call STP_SAVEPOINT_EXAMPLE1();

-- 来源: 1452_file_1452
CALL STP_SAVEPOINT_EXAMPLE2();

-- 来源: 1452_file_1452
CALL STP_SAVEPOINT_EXAMPLE3();

-- 来源: 1452_file_1452
call FUNCTION_EXAMPLE1();

-- 来源: 1452_file_1452
CALL TRANSACTION_EXAMPLE1();

-- 来源: 1452_file_1452
CALL TRANSACTION_EXAMPLE2(100);

-- 来源: 1452_file_1452
CALL TRANSACTION_EXAMPLE3();

-- 来源: 1452_file_1452
CALL TRANSACTION_EXAMPLE4();

-- 来源: 1452_file_1452
CALL TRANSACTION_EXAMPLE6();

-- 来源: 1452_file_1452
CALL exec_func2();

-- 来源: 1452_file_1452
CALL exec_func4(1);

-- 来源: 1458_file_1458
CALL cursor_proc1 ();

-- 来源: 1458_file_1458
CALL cursor_proc2 ();

-- 来源: 1458_file_1458
DECLARE C1 SYS_REFCURSOR ;

-- 来源: 1459_file_1459
CALL proc_cursor3 ();

-- 来源: 1468_DBE_COMPRESSION
DECLARE o_blkcnt_cmp integer ;

-- 来源: 1470_DBE_ILM
CALL DBE_ILM . STOP_ILM ( - 1 , true , NULL );

-- 来源: 1471_DBE_ILM_ADMIN
CALL DBE_ILM_ADMIN . CUSTOMIZE_ILM ( 1 , 15 );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . create_program ( 'program1' , 'STORED_PROCEDURE' , 'select pg_sleep(1);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . create_schedule ( 'schedule1' , NULL , 'sysdate' , NULL , 'test' );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . create_job ( job_name => 'job1' , program_name => 'program1' , schedule_name => 'schedule1' );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_job ( 'job1' , true , false , 'STOP_ON_FIRST_ERROR' );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_schedule ( 'schedule1' );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_program ( 'program1' , false );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . create_program ( 'program1' , 'STORED_PROCEDURE' , 'select pg_sleep(1);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . create_schedule ( 'schedule1' , NULL , 'sysdate' , NULL , 'test' );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . create_job ( job_name => 'job1' , program_name => 'program1' , schedule_name => 'schedule1' );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_job ( 'job1' , true , false , 'STOP_ON_FIRST_ERROR' );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_schedule ( 'schedule1' );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_program ( 'program1' , false );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_job('job1', 'program1', '2021-07-20', 'interval ''3 minute''', '2121-07-20', 'DEFAULT_JOB_CLASS', false, false,'test', 'style', NULL, NULL);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_single_job('job1', false, false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_program('program1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . create_program ( 'program1' , 'STORED_PROCEDURE' , 'select pg_sleep(1);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . set_attribute ( 'program1' , 'number_of_arguments' , 0 );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . set_attribute ( 'program1' , 'program_type' , 'STORED_PROCEDURE' );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_program ( 'program1' , false );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . run_job ( 'job1' , false );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_job ( 'job1' , true , false , 'STOP_ON_FIRST_ERROR' );

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.run_backend_job('job1');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.run_foreground_job('job1');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_credential('cre_1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.stop_job('job1', true, 'STOP_ON_FIRST_ERROR');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.stop_single_job('job1', true);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.generate_job_name();

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.generate_job_name();

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.generate_job_name('job');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.generate_job_name('job');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_program('program1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.define_program_argument('program1', 1, 'pa1', 'type1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.define_program_argument('program1', 1, 'pa1', 'type1', 'value1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_program('program1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_program('program1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_single_program('program1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL dbe_scheduler.create_job('job1','EXTERNAL_SCRIPT','begin insert into test1 values(12);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.set_job_argument_value('job1', 1, 'value1');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule1', sysdate, 'sysdate + 3 / (24 * 60 * 60)', null, 'test1');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule2', sysdate, 'FREQ=DAILY;

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule3', sysdate, 'FREQ=DAILY;

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_schedule('schedule1');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_schedule('schedule2', false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_schedule('schedule3', true);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule1', sysdate, 'sysdate + 3 / (24 * 60 * 60)', null, 'test1');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule2', sysdate, 'FREQ=DAILY;

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule3', sysdate, 'FREQ=DAILY;

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_schedule('schedule1');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_schedule('schedule2', false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_schedule('schedule3', true);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule1', sysdate, 'sysdate + 3 / (24 * 60 * 60)', null, 'test1');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule2', sysdate, 'FREQ=DAILY;

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule3', sysdate, 'FREQ=DAILY;

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_single_schedule('schedule1');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_single_schedule('schedule2', false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_single_schedule('schedule3', true);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_job_class(job_class_name => 'jc1', resource_consumer_group => '123');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job_class('jc1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_job_class(job_class_name => 'jc1', resource_consumer_group => '123');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job_class('jc1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_job_class(job_class_name => 'jc1', resource_consumer_group => '123');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_single_job_class('jc1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.grant_user_authorization('user1', 'create job');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.grant_user_authorization('user1', 'create job');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.revoke_user_authorization('user1', 'create job');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_credential('cre_1', 'user1', '');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_credential('cre_1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_credential('cre_1', 'user1', '');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_credential('cre_1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_program('program1', 'stored_procedure', 'insert into tb_job_test(key) values(null);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.enable('job1');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.enable('program1', 'STOP_ON_FIRST_ERROR');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_program('program1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.enable_single('job1');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 1477_DBE_SCHEDULER
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_program('program1', 'stored_procedure', 'insert into tb_job_test(key) values(null);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.disable('job1');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.disable('program1', false, 'STOP_ON_FIRST_ERROR');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_program('program1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.disable_single('job1', false);

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 1477_DBE_SCHEDULER
CALL DBE_SCHEDULER.eval_calendar_string('FREQ=DAILY;

-- 来源: 1477_DBE_SCHEDULER
CALL pr1('FREQ=hourly;

-- 来源: 1480_DBE_TASK
DECLARE gaussdb -# jobid int ;

-- 来源: 1480_DBE_TASK
DECLARE gaussdb -# id integer ;

-- 来源: 1480_DBE_TASK
CALL dbe_task . id_submit ( 101 , 'insert_msg_statistic1;

-- 来源: 1480_DBE_TASK
CALL dbe_task.cancel(101);

-- 来源: 1480_DBE_TASK
CALL dbe_task . id_submit ( 101 , 'insert_msg_statistic1;

-- 来源: 1480_DBE_TASK
CALL dbe_task . finish ( 101 , true );

-- 来源: 1480_DBE_TASK
CALL dbe_task . finish ( 101 , false , sysdate );

-- 来源: 1480_DBE_TASK
CALL dbe_task . update ( 101 , 'call userproc();

-- 来源: 1480_DBE_TASK
CALL dbe_task . update ( 101 , 'insert into tbl_a values(sysdate);

-- 来源: 1480_DBE_TASK
CALL dbe_task . content ( 101 , 'call userproc();

-- 来源: 1480_DBE_TASK
CALL dbe_task . content ( 101 , 'insert into tbl_a values(sysdate);

-- 来源: 1480_DBE_TASK
CALL dbe_task . next_time ( 101 , sysdate );

-- 来源: 1480_DBE_TASK
CALL dbe_task . interval ( 101 , 'sysdate + 1.0/1440' );

-- 来源: 1480_DBE_TASK
CALL dbe_task . cancel ( 101 );

-- 来源: 1485_Retry
CALL retry_basic ( 1 );

-- 来源: 1489_file_1489
DECLARE

-- 来源: 1489_file_1489
CALL proc_sys_call();

-- 来源: 1489_file_1489
CALL proc_sys_call();

-- 来源: 1489_file_1489
call proc_sys_ref(null);

-- 来源: 1489_file_1489
CALL autonomous_test_lock(1,1);

-- 来源: 1489_file_1489
call auto_func(1);

-- 来源: 1489_file_1489
CALL test_set();

-- 来源: 1491_file_1491
DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 2292_DBE_PLDEBUGGER Schema
call test_debug ( 1 );

-- 来源: 2366_file_2366
CALL call_out_param_test1();

-- 来源: 2366_file_2366
CALL p1();

-- 来源: 2366_file_2366
CALL p1();

-- 来源: 2366_file_2366
CALL p1();

-- 来源: 2366_file_2366
CALL p1();

-- 来源: 2366_file_2366
call p1();

-- 来源: 2366_file_2366
call p1();

-- 来源: 2366_file_2366
call p1();

-- 来源: 2366_file_2366
call p1();

-- 来源: 2366_file_2366
call p1();

-- 来源: 2366_file_2366
call p1();

-- 来源: 2466_file_2466
call dbe_task . submit ( 'call public.prc_job_1();

-- 来源: 2466_file_2466
call dbe_task . id_submit ( 1 , 'call public.prc_job_1();

-- 来源: 2466_file_2466
call dbe_task . finish ( 1 , true );

-- 来源: 2466_file_2466
call dbe_task . finish ( 1 , false );

-- 来源: 2466_file_2466
call dbe_task . next_date ( 1 , sysdate + 1 . 0 / 24 );

-- 来源: 2466_file_2466
call dbe_task . interval ( 1 , 'sysdate + 1.0/24' );

-- 来源: 2466_file_2466
call dbe_task . content ( 1 , 'insert into public.test values(333, sysdate+5);

-- 来源: 2466_file_2466
call dbe_task . update ( 1 , 'call public.prc_job_1();

-- 来源: 2466_file_2466
call dbe_task . cancel ( 1 );

-- 来源: 2731_SQL PATCH
call mypro();

-- 来源: 2731_SQL PATCH
call mypro();

-- 来源: 2774_file_2774
CALL tinterval ( abstime 'May 10, 1947 23:59:12' , abstime 'Mon May 1 00:30:30 1995' );

-- 来源: 2775_file_2775
call p1 ();

-- 来源: 2775_file_2775
call pkg1 . p1 ();

-- 来源: 2782_file_2782
call f1 ();

-- 来源: 2782_file_2782
call f1 ();

-- 来源: 2785_file_2785
CALL PROC ();

-- 来源: 2804_file_2804
CALL mypro1();

-- 来源: 2810_HashFunc
call hashenum ( 'good' :: b1 );

-- 来源: 2821_XML
DECLARE xmldata xml ;

-- 来源: 2821_XML
DECLARE xmldata xml;

-- 来源: 2822_XMLTYPE
declare a xmltype ;

-- 来源: 2822_XMLTYPE
declare xmltype_clob clob ;

-- 来源: 2822_XMLTYPE
declare xmltype_blob blob ;

-- 来源: 2822_XMLTYPE
declare xmltype_clob clob ;

-- 来源: 2822_XMLTYPE
declare xmltype_blob blob ;

-- 来源: 2823_Global Plsql Cache
call pg_catalog.invalidate_plsql_object('public','pkg1','package');

--按参数值传递。
-- 来源: 2931_CALL
CALL func_add_sql(1, 3);

--使用命名标记法传参。
-- 来源: 2931_CALL
CALL func_add_sql(num1 => 1,num2 => 3);

-- 来源: 2931_CALL
CALL func_add_sql(num2 := 2, num1 := 3);

--出参传入常量。
-- 来源: 2931_CALL
CALL func_increment_sql(1,2,1);

--同时返回return和出参
-- 来源: 2951_CREATE FUNCTION
DECLARE result integer;

--不支持左赋值表达式
-- 来源: 2951_CREATE FUNCTION
DECLARE result integer;

--存储过程中不支持out/inout传入常量
-- 来源: 2951_CREATE FUNCTION
DECLARE result integer;

--存储过程中支持out/inout传入变量
-- 来源: 2951_CREATE FUNCTION
DECLARE result integer;

-- 来源: 2951_CREATE FUNCTION
DECLARE r rec;

-- 来源: 2951_CREATE FUNCTION
DECLARE table_of_index_int_val pkg_type.table_of_index_int;

-- 来源: 2951_CREATE FUNCTION
DECLARE date1 date := '2022-02-02';

-- 来源: 2951_CREATE FUNCTION
DECLARE date1 int := 1;

--将PACKAGE emp_bonus的所属者改为omm 调用PACKAGE示例
-- 来源: 2962_CREATE PACKAGE
CALL emp_bonus.testpro1(1);

-- 来源: 2963_CREATE PROCEDURE
CALL insert_data ( 1 );

--使用同义词register，调用存储过程。
-- 来源: 2975_CREATE SYNONYM
CALL register(3,'mia');

--授予用户webuser对模式tpcds下视图的所有操作权限。
-- 来源: 2994_DO
DO $$DECLARE r record;

-- 来源: 3021_DROP RULE
DO INSTEAD INSERT INTO def_test SELECT new.*;

--创建一个with hold游标。
-- 来源: 3046_FETCH
DECLARE cursor1 CURSOR WITH HOLD FOR SELECT * FROM tpcds. customer_address ORDER BY 1;

--游标位置不受保存点回滚的影响。
-- 来源: 3078_ROLLBACK TO SAVEPOINT
DECLARE foo CURSOR FOR SELECT 1 UNION SELECT 2;

-- 来源: 3130_file_3130
CALL array_proc ();

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- n大于数组元素个数，清空数组元素
-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 数组未初始化
-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type array_integer is varray(10) of integer;

-- 来源: 3131_file_3131
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回NULL
-- 来源: 3131_file_3131
declare type varr is varray(10) of varchar(3);

-- 来源: 3131_file_3131
declare type varr is varray(10) of varchar(3);

-- 来源: 3131_file_3131
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回NULL
-- 来源: 3131_file_3131
declare type varr is varray(10) of varchar(3);

-- 来源: 3131_file_3131
declare type varr is varray(10) of varchar(3);

-- 来源: 3131_file_3131
declare type varr is varray(10) of varchar(3);

-- 下标越界，大于数组范围
-- 来源: 3131_file_3131
declare type varr is varray(10) of varchar(3);

-- 来源: 3131_file_3131
declare type varr is varray(10) of varchar(3);

-- 来源: 3131_file_3131
declare type varr is varray(10) of varchar(3);

-- 下标越界，大于数组范围
-- 来源: 3131_file_3131
declare type varr is varray(10) of varchar(3);

-- 来源: 3131_file_3131
declare type varr is varray(10) of varchar(3);

-- 来源: 3131_file_3131
declare type varr is varray(10) of varchar(3);

-- 数组未初始化返回false
-- 来源: 3131_file_3131
declare type varr is varray(10) of varchar(3);

-- 来源: 3133_file_3133
CALL table_proc ();

-- 来源: 3133_file_3133
CALL nest_table_proc ();

-- 来源: 3133_file_3133
CALL index_table_proc ();

-- 来源: 3133_file_3133
CALL nest_table_proc ();

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type nest is table of varchar2;

-- 来源: 3134_file_3134
declare type nest is table of varchar2 index by varchar2;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type t1 is table of int index by varchar;

-- 来源: 3134_file_3134
declare type t1 is table of int index by varchar;

-- 来源: 3134_file_3134
declare type t1 is table of int index by varchar;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type t1 is table of int index by int;

-- 来源: 3134_file_3134
declare type t1 is table of int index by varchar;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type t1 is table of int index by int;

-- 来源: 3134_file_3134
declare type t1 is table of int index by varchar;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type t1 is table of int index by varchar;

-- 来源: 3134_file_3134
declare type t1 is table of int index by varchar;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type t1 is table of int index by varchar;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
declare type t1 is table of int index by int;

-- 来源: 3134_file_3134
declare type nest is table of int;

-- 来源: 3134_file_3134
call p1 ();

-- 来源: 3134_file_3134
call p1 ();

-- 来源: 3134_file_3134
call p1 ();

-- 来源: 3134_file_3134
call p1 ();

-- 来源: 3135_record
CALL regress_record ( 'abc' );

-- 来源: 3135_record
call func ( 0 );

-- 来源: 3135_record
call func ( 1 );

-- 来源: 3138_file_3138
DECLARE my_var VARCHAR2 ( 30 );

-- 来源: 3144_file_3144
DECLARE emp_id INTEGER : = 7788 ;

-- 来源: 3144_file_3144
DECLARE emp_id INTEGER :=7788;

-- 来源: 3145_file_3145
DECLARE TYPE r1 is VARRAY ( 10 ) of o1 ;

-- 来源: 3145_file_3145
DECLARE type id_list is varray(6) of customers.id%type;

-- 来源: 3146_file_3146
CALL proc_return ();

--从动态语句检索值（INTO 子句）：
-- 来源: 3148_file_3148
DECLARE staff_count VARCHAR2(20);

--调用存储过程
-- 来源: 3148_file_3148
CALL dynamic_proc();

-- 来源: 3148_file_3148
DECLARE name VARCHAR2(20);

-- 来源: 3149_file_3149
DECLARE section NUMBER ( 4 ) : = 280 ;

-- 来源: 3150_file_3150
DECLARE input1 INTEGER:=1;

--调用存储过程
-- 来源: 3151_file_3151
CALL dynamic_proc();

-- 来源: 3155_RETURN NEXTRETURN QUERY
call fun_for_return_next ();

-- 来源: 3155_RETURN NEXTRETURN QUERY
call fun_for_return_query ();

-- 来源: 3156_file_3156
DECLARE v_user_id integer default 1 ;

-- 来源: 3156_file_3156
DECLARE v_user_id integer default 1 ;

-- 来源: 3156_file_3156
DECLARE v_user_id integer default 1 ;

-- 来源: 3156_file_3156
DECLARE v_user_id integer default NULL ;

-- 来源: 3156_file_3156
CALL proc_control_structure ( 3 );

-- 来源: 3157_file_3157
CALL proc_loop ( 10 , 5 );

-- 来源: 3157_file_3157
CALL proc_while_loop ( 10 );

-- 来源: 3157_file_3157
CALL proc_for_loop ();

-- 来源: 3157_file_3157
CALL proc_for_loop_query ();

-- 来源: 3157_file_3157
CALL proc_forall ();

-- 来源: 3158_file_3158
CALL proc_case_branch ( 3 , 0 );

-- 来源: 3159_file_3159
DECLARE v_num integer default NULL;

-- 来源: 3160_file_3160
call fun_exp ();

-- 来源: 3161_GOTO
call GOTO_test ();

-- 来源: 3162_file_3162
call TRANSACTION_EXAMPLE();

-- 来源: 3162_file_3162
call TEST_COMMIT_INSERT_EXCEPTION_ROLLBACK();

-- 来源: 3162_file_3162
call TEST_COMMIT2();

-- 来源: 3162_file_3162
call exec_func3('');

-- 来源: 3162_file_3162
call exec_func4();

-- 来源: 3162_file_3162
call GUC_ROLLBACK();

-- 来源: 3162_file_3162
call STP_SAVEPOINT_EXAMPLE1();

-- 来源: 3168_file_3168
CALL cursor_proc1 ();

-- 来源: 3168_file_3168
CALL cursor_proc2 ();

-- 来源: 3168_file_3168
DECLARE C1 SYS_REFCURSOR ;

-- 来源: 3169_file_3169
CALL proc_cursor3 ();

-- 来源: 3170_file_3170
DECLARE CURSOR C1 IS SELECT A FROM integerTable1 ;

-- 来源: 3181_DBE_ILM
CALL DBE_ILM . STOP_ILM ( - 1 , true , NULL );

-- 来源: 3182_DBE_ILM_ADMIN
CALL DBE_ILM_ADMIN . CUSTOMIZE_ILM ( 1 , 15 );

-- 来源: 3186_DBE_PROFILER
CALL p3 ();

-- 来源: 3186_DBE_PROFILER
CALL autonomous ( 11 , 22 );

-- 来源: 3186_DBE_PROFILER
CALL autonomous_1 ( 11 , 22 );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . create_program ( 'program1' , 'STORED_PROCEDURE' , 'select pg_sleep(1);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . create_schedule ( 'schedule1' , NULL , 'sysdate' , NULL , 'test' );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . create_job ( job_name => 'job1' , program_name => 'program1' , schedule_name => 'schedule1' );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_job ( 'job1' , true , false , 'STOP_ON_FIRST_ERROR' );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_schedule ( 'schedule1' );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_program ( 'program1' , false );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . create_program ( 'program1' , 'STORED_PROCEDURE' , 'select pg_sleep(1);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . create_schedule ( 'schedule1' , NULL , 'sysdate' , NULL , 'test' );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . create_job ( job_name => 'job1' , program_name => 'program1' , schedule_name => 'schedule1' );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_job ( 'job1' , true , false , 'STOP_ON_FIRST_ERROR' );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_schedule ( 'schedule1' );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_program ( 'program1' , false );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_job('job1', 'program1', '2021-07-20', 'interval ''3 minute''', '2121-07-20', 'DEFAULT_JOB_CLASS', false, false,'test', 'style', NULL, NULL);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_single_job('job1', false, false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_program('program1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . create_program ( 'program1' , 'STORED_PROCEDURE' , 'select pg_sleep(1);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . set_attribute ( 'program1' , 'number_of_arguments' , 0 );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . set_attribute ( 'program1' , 'program_type' , 'STORED_PROCEDURE' );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_program ( 'program1' , false );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . run_job ( 'job1' , false );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER . drop_job ( 'job1' , true , false , 'STOP_ON_FIRST_ERROR' );

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.run_backend_job('job1');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.run_foreground_job('job1');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_credential('cre_1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.stop_job('job1', true, 'STOP_ON_FIRST_ERROR');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.stop_single_job('job1', true);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.generate_job_name();

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.generate_job_name();

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.generate_job_name('job');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.generate_job_name('job');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_program('program1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.define_program_argument('program1', 1, 'pa1', 'type1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.define_program_argument('program1', 1, 'pa1', 'type1', 'value1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_program('program1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_program('program1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_single_program('program1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL dbe_scheduler.create_job('job1','EXTERNAL_SCRIPT','begin insert into test1 values(12);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.set_job_argument_value('job1', 1, 'value1');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule1', sysdate, 'sysdate + 3 / (24 * 60 * 60)', null, 'test1');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule2', sysdate, 'FREQ=DAILY;

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule3', sysdate, 'FREQ=DAILY;

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_schedule('schedule1');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_schedule('schedule2', false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_schedule('schedule3', true);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule1', sysdate, 'sysdate + 3 / (24 * 60 * 60)', null, 'test1');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule2', sysdate, 'FREQ=DAILY;

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule3', sysdate, 'FREQ=DAILY;

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_schedule('schedule1');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_schedule('schedule2', false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_schedule('schedule3', true);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule1', sysdate, 'sysdate + 3 / (24 * 60 * 60)', null, 'test1');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule2', sysdate, 'FREQ=DAILY;

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_schedule('schedule3', sysdate, 'FREQ=DAILY;

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_single_schedule('schedule1');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_single_schedule('schedule2', false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_single_schedule('schedule3', true);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_job_class(job_class_name => 'jc1', resource_consumer_group => '123');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job_class('jc1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_job_class(job_class_name => 'jc1', resource_consumer_group => '123');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job_class('jc1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_job_class(job_class_name => 'jc1', resource_consumer_group => '123');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_single_job_class('jc1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.grant_user_authorization('user1', 'create job');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.grant_user_authorization('user1', 'create job');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.revoke_user_authorization('user1', 'create job');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_credential('cre_1', 'user1', '');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_credential('cre_1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_credential('cre_1', 'user1', '');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_credential('cre_1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_program('program1', 'stored_procedure', 'insert into tb_job_test(key) values(null);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.enable('job1');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.enable('program1', 'STOP_ON_FIRST_ERROR');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_program('program1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.enable_single('job1');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 3189_DBE_SCHEDULER
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.create_program('program1', 'stored_procedure', 'insert into tb_job_test(key) values(null);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.disable('job1');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.disable('program1', false, 'STOP_ON_FIRST_ERROR');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_program('program1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.disable_single('job1', false);

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

-- 来源: 3189_DBE_SCHEDULER
CALL DBE_SCHEDULER.eval_calendar_string('FREQ=DAILY;

-- 来源: 3189_DBE_SCHEDULER
CALL pr1('FREQ=hourly;

-- 锁定表，查看其锁定状态
-- 来源: 3192_DBE_STATS
CALL DBE_STATS.LOCK_TABLE_STATS(ownname=>'dbe_stats_lock',tabname=>'t1');

-- 锁定一个分区，其他分区及表不受影响
-- 来源: 3192_DBE_STATS
CALL DBE_STATS.LOCK_PARTITION_STATS(ownname=>'dbe_stats_lock',tabname=>'upart_table',partname=>'p1');

-- 锁定列后，查看列的锁定状态
-- 来源: 3192_DBE_STATS
CALL DBE_STATS.LOCK_COLUMN_STATS(ownname=>'dbe_stats_lock',tabname=>'t1',colname=>'a');

-- 来源: 3192_DBE_STATS
CALL DBE_STATS.LOCK_SCHEMA_STATS(ownname=>'dbe_stats_lock');

-- 来源: 3192_DBE_STATS
CALL DBE_STATS.UNLOCK_TABLE_STATS(ownname=>'dbe_stats_lock',tabname=>'t1');

-- 来源: 3192_DBE_STATS
CALL DBE_STATS.UNLOCK_PARTITION_STATS(ownname=>'dbe_stats_lock',tabname=>'upart_table',partname=>'p1');

-- 来源: 3192_DBE_STATS
CALL DBE_STATS.UNLOCK_COLUMN_STATS(ownname=>'dbe_stats_lock',tabname=>'t1',colname=>'a');

-- 来源: 3192_DBE_STATS
CALL DBE_STATS.UNLOCK_SCHEMA_STATS(ownname=>'dbe_stats_lock');

-- 回退到最早的统计信息，查看系统表
-- 来源: 3192_DBE_STATS
CALL DBE_STATS.RESTORE_TABLE_STATS(ownname=>'dbe_stats_restore',tabname=>'t1',as_of_timestamp=>((SELECT MIN(reltimestamp) FROM GS_TABLESTATS_HISTORY WHERE relname='t1') + INTERVAL '1 second'));

-- 来源: 3192_DBE_STATS
CALL DBE_STATS.RESTORE_PARTITION_STATS(ownname=>'dbe_stats_restore',tabname=>'t1',partname=>'p1',as_of_timestamp=>((SELECT MIN(reltimestamp) FROM GS_TABLESTATS_HISTORY WHERE relname='t1') + INTERVAL '1 second'));

-- 回退到时间较早的时间节点，查询系统表中的统计信息
-- 来源: 3192_DBE_STATS
CALL DBE_STATS.RESTORE_COLUMN_STATS(ownname=>'dbe_stats_restore',tabname=>'t1',colname=>'a',as_of_timestamp=>((SELECT MIN(reltimestamp) FROM GS_TABLESTATS_HISTORY WHERE relname='t1') + INTERVAL '1 second'));

-- 来源: 3192_DBE_STATS
CALL DBE_STATS.RESTORE_SCHEMA_STATS(ownname=>'dbe_stats_restore',as_of_timestamp=>((SELECT MIN(reltimestamp) FROM GS_TABLESTATS_HISTORY WHERE relname='t1') + INTERVAL '1 second'));

-- 清除时间较早的历史统计信息，查看历史表
-- 来源: 3192_DBE_STATS
CALL DBE_STATS.PURGE_STATS(before_timestamp=>((SELECT MIN(reltimestamp) FROM GS_TABLESTATS_HISTORY WHERE relname='t1') + INTERVAL '1 second'));

-- 来源: 3192_DBE_STATS
CALL DBE_STATS.GET_STATS_HISTORY_RETENTION();

-- 来源: 3192_DBE_STATS
CALL DBE_STATS.GET_STATS_HISTORY_AVAILABILITY();

-- 来源: 3193_DBE_TASK
DECLARE gaussdb -# jobid int ;

-- 来源: 3193_DBE_TASK
DECLARE gaussdb -# id integer ;

-- 来源: 3193_DBE_TASK
CALL dbe_task . id_submit ( 101 , 'insert_msg_statistic1;

-- 来源: 3193_DBE_TASK
CALL dbe_task.cancel(101);

-- 来源: 3193_DBE_TASK
CALL dbe_task . id_submit ( 101 , 'insert_msg_statistic1;

-- 来源: 3193_DBE_TASK
CALL dbe_task . finish ( 101 , true );

-- 来源: 3193_DBE_TASK
CALL dbe_task . update ( 101 , 'call userproc();

-- 来源: 3193_DBE_TASK
CALL dbe_task . update ( 101 , 'insert into tbl_a values(sysdate);

-- 来源: 3193_DBE_TASK
CALL dbe_task . content ( 101 , 'call userproc();

-- 来源: 3193_DBE_TASK
CALL dbe_task . content ( 101 , 'insert into tbl_a values(sysdate);

-- 来源: 3193_DBE_TASK
CALL dbe_task . next_time ( 101 , sysdate );

-- 来源: 3193_DBE_TASK
CALL dbe_task . interval ( 101 , 'sysdate + 1.0/1440' );

-- 来源: 3193_DBE_TASK
CALL dbe_task . cancel ( 101 );

-- 来源: 3198_Retry
CALL retry_basic ( 1 );

-- 来源: 3202_file_3202
DECLARE

-- 来源: 3202_file_3202
CALL proc_sys_call();

-- 来源: 3202_file_3202
CALL proc_sys_call();

-- 来源: 3202_file_3202
call proc_sys_ref(null);

-- 来源: 3202_file_3202
CALL func(1);

-- 来源: 3202_file_3202
call auto_func(1);

-- 来源: 3204_file_3204
DECLARE PRAGMA AUTONOMOUS_TRANSACTION;

-- 来源: 3929_DBE_PLDEBUGGER Schema
call test_debug ( 1 );

-- 来源: 4027_file_4027
CALL call_out_param_test1();

-- 来源: 4027_file_4027
CALL p1();

-- 来源: 4027_file_4027
CALL p1();

-- 来源: 4027_file_4027
CALL p1();

-- 来源: 4027_file_4027
CALL p1();

-- 来源: 4027_file_4027
call p1();

-- 来源: 4027_file_4027
call p1();

-- 来源: 4027_file_4027
call p1();

-- 来源: 4027_file_4027
call p1();

-- 来源: 4027_file_4027
call p1();

-- 来源: 4027_file_4027
call p1();

-- 来源: 4407_file_4407
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(11, 1);

-- 来源: 4407_file_4407
DECLARE v_taskid number;

-- 来源: 4407_file_4407
DECLARE V_HOUR INT := 22;

-- 来源: 4409_TIPS
CALL DBE_ILM_ADMIN.DISABLE_ILM();

-- 来源: 4409_TIPS
CALL DBE_ILM_ADMIN.ENABLE_ILM();

-- 来源: 4409_TIPS
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(11, 1);

-- 来源: 4409_TIPS
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(12, 10);

-- 来源: 4409_TIPS
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(1, 1);

-- 来源: 4409_TIPS
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(13, 512);

-- 来源: 4667_file_4667
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(11, 1);

-- 来源: 4667_file_4667
DECLARE v_taskid number;

-- 来源: 4667_file_4667
DECLARE V_HOUR INT := 22;

-- 来源: 4669_TIPS
CALL DBE_ILM_ADMIN.DISABLE_ILM();

-- 来源: 4669_TIPS
CALL DBE_ILM_ADMIN.ENABLE_ILM();

-- 来源: 4669_TIPS
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(11， 1);

-- 来源: 4669_TIPS
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(12, 10);

-- 来源: 4669_TIPS
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(1, 1);

-- 来源: 4669_TIPS
CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(13, 512);

-- 来源: 768_file_768
call dbe_task . submit ( 'call public.prc_job_1();

-- 来源: 768_file_768
call dbe_task . id_submit ( 1 , 'call public.prc_job_1();

-- 来源: 768_file_768
call dbe_task . finish ( 1 , true );

-- 来源: 768_file_768
call dbe_task . finish ( 1 , false );

-- 来源: 768_file_768
call dbe_task . next_date ( 1 , sysdate + 1 . 0 / 24 );

-- 来源: 768_file_768
call dbe_task . interval ( 1 , 'sysdate + 1.0/24' );

-- 来源: 768_file_768
call dbe_task . content ( 1 , 'insert into public.test values(333, sysdate+5);

-- 来源: 768_file_768
call dbe_task . update ( 1 , 'call public.prc_job_1();

-- 来源: 768_file_768
call dbe_task . cancel ( 1 );

