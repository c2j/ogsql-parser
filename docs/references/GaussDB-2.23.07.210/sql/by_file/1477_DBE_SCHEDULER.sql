-- 来源: 1477_DBE_SCHEDULER.txt
-- SQL 数量: 112

CALL DBE_SCHEDULER . create_program ( 'program1' , 'STORED_PROCEDURE' , 'select pg_sleep(1);

CALL DBE_SCHEDULER . create_schedule ( 'schedule1' , NULL , 'sysdate' , NULL , 'test' );

CALL DBE_SCHEDULER . create_job ( job_name => 'job1' , program_name => 'program1' , schedule_name => 'schedule1' );

CALL DBE_SCHEDULER . drop_job ( 'job1' , true , false , 'STOP_ON_FIRST_ERROR' );

CALL DBE_SCHEDULER . drop_schedule ( 'schedule1' );

CALL DBE_SCHEDULER . drop_program ( 'program1' , false );

CALL DBE_SCHEDULER . create_program ( 'program1' , 'STORED_PROCEDURE' , 'select pg_sleep(1);

CALL DBE_SCHEDULER . create_schedule ( 'schedule1' , NULL , 'sysdate' , NULL , 'test' );

CALL DBE_SCHEDULER . create_job ( job_name => 'job1' , program_name => 'program1' , schedule_name => 'schedule1' );

CALL DBE_SCHEDULER . drop_job ( 'job1' , true , false , 'STOP_ON_FIRST_ERROR' );

CALL DBE_SCHEDULER . drop_schedule ( 'schedule1' );

CALL DBE_SCHEDULER . drop_program ( 'program1' , false );

CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

CALL DBE_SCHEDULER.create_job('job1', 'program1', '2021-07-20', 'interval ''3 minute''', '2121-07-20', 'DEFAULT_JOB_CLASS', false, false,'test', 'style', NULL, NULL);

CALL DBE_SCHEDULER.drop_single_job('job1', false, false);

CALL DBE_SCHEDULER.drop_program('program1', false);

CALL DBE_SCHEDULER . create_program ( 'program1' , 'STORED_PROCEDURE' , 'select pg_sleep(1);

CALL DBE_SCHEDULER . set_attribute ( 'program1' , 'number_of_arguments' , 0 );

CALL DBE_SCHEDULER . set_attribute ( 'program1' , 'program_type' , 'STORED_PROCEDURE' );

CALL DBE_SCHEDULER . drop_program ( 'program1' , false );

SELECT dbe_scheduler . create_job ( 'job1' , 'PLSQL_BLOCK' , 'begin insert into test1 values(12);

CALL DBE_SCHEDULER . run_job ( 'job1' , false );

CALL DBE_SCHEDULER . drop_job ( 'job1' , true , false , 'STOP_ON_FIRST_ERROR' );

SELECT dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

CALL DBE_SCHEDULER.run_backend_job('job1');

CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

create user test1 identified by '*********';

select DBE_SCHEDULER.create_credential('cre_1', 'test1', '*********');

select DBE_SCHEDULER.create_job(job_name=>'job1', job_type=>'EXTERNAL_SCRIPT', job_action=>'/usr/bin/pwd', enabled=>true, auto_drop=>false, credential_name => 'cre_1');

CALL DBE_SCHEDULER.run_foreground_job('job1');

CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

CALL DBE_SCHEDULER.drop_credential('cre_1', false);

drop user test1;

SELECT dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

CALL DBE_SCHEDULER.stop_job('job1', true, 'STOP_ON_FIRST_ERROR');

CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

SELECT dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

CALL DBE_SCHEDULER.stop_single_job('job1', true);

CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

CALL DBE_SCHEDULER.generate_job_name();

CALL DBE_SCHEDULER.generate_job_name();

CALL DBE_SCHEDULER.generate_job_name('job');

CALL DBE_SCHEDULER.generate_job_name('job');

CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

CALL DBE_SCHEDULER.drop_program('program1', false);

CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

CALL DBE_SCHEDULER.define_program_argument('program1', 1, 'pa1', 'type1', false);

CALL DBE_SCHEDULER.define_program_argument('program1', 1, 'pa1', 'type1', 'value1', false);

CALL DBE_SCHEDULER.drop_program('program1', false);

CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

CALL DBE_SCHEDULER.drop_program('program1', false);

CALL DBE_SCHEDULER.create_program('program1', 'STORED_PROCEDURE', 'select pg_sleep(1);

CALL DBE_SCHEDULER.drop_single_program('program1', false);

CALL dbe_scheduler.create_job('job1','EXTERNAL_SCRIPT','begin insert into test1 values(12);

CALL DBE_SCHEDULER.set_job_argument_value('job1', 1, 'value1');

CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

CALL DBE_SCHEDULER.create_schedule('schedule1', sysdate, 'sysdate + 3 / (24 * 60 * 60)', null, 'test1');

CALL DBE_SCHEDULER.create_schedule('schedule2', sysdate, 'FREQ=DAILY;

CALL DBE_SCHEDULER.create_schedule('schedule3', sysdate, 'FREQ=DAILY;

CALL DBE_SCHEDULER.drop_schedule('schedule1');

CALL DBE_SCHEDULER.drop_schedule('schedule2', false);

CALL DBE_SCHEDULER.drop_schedule('schedule3', true);

CALL DBE_SCHEDULER.create_schedule('schedule1', sysdate, 'sysdate + 3 / (24 * 60 * 60)', null, 'test1');

CALL DBE_SCHEDULER.create_schedule('schedule2', sysdate, 'FREQ=DAILY;

CALL DBE_SCHEDULER.create_schedule('schedule3', sysdate, 'FREQ=DAILY;

CALL DBE_SCHEDULER.drop_schedule('schedule1');

CALL DBE_SCHEDULER.drop_schedule('schedule2', false);

CALL DBE_SCHEDULER.drop_schedule('schedule3', true);

CALL DBE_SCHEDULER.create_schedule('schedule1', sysdate, 'sysdate + 3 / (24 * 60 * 60)', null, 'test1');

CALL DBE_SCHEDULER.create_schedule('schedule2', sysdate, 'FREQ=DAILY;

CALL DBE_SCHEDULER.create_schedule('schedule3', sysdate, 'FREQ=DAILY;

CALL DBE_SCHEDULER.drop_single_schedule('schedule1');

CALL DBE_SCHEDULER.drop_single_schedule('schedule2', false);

CALL DBE_SCHEDULER.drop_single_schedule('schedule3', true);

CALL DBE_SCHEDULER.create_job_class(job_class_name => 'jc1', resource_consumer_group => '123');

CALL DBE_SCHEDULER.drop_job_class('jc1', false);

CALL DBE_SCHEDULER.create_job_class(job_class_name => 'jc1', resource_consumer_group => '123');

CALL DBE_SCHEDULER.drop_job_class('jc1', false);

CALL DBE_SCHEDULER.create_job_class(job_class_name => 'jc1', resource_consumer_group => '123');

CALL DBE_SCHEDULER.drop_single_job_class('jc1', false);

create user user1 password '1*s*****';

CALL DBE_SCHEDULER.grant_user_authorization('user1', 'create job');

drop user user1;

create user user1 password '1*s*****';

CALL DBE_SCHEDULER.grant_user_authorization('user1', 'create job');

CALL DBE_SCHEDULER.revoke_user_authorization('user1', 'create job');

drop user user1;

CALL DBE_SCHEDULER.create_credential('cre_1', 'user1', '');

CALL DBE_SCHEDULER.drop_credential('cre_1', false);

CALL DBE_SCHEDULER.create_credential('cre_1', 'user1', '');

CALL DBE_SCHEDULER.drop_credential('cre_1', false);

CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

CALL DBE_SCHEDULER.create_program('program1', 'stored_procedure', 'insert into tb_job_test(key) values(null);

CALL DBE_SCHEDULER.enable('job1');

CALL DBE_SCHEDULER.enable('program1', 'STOP_ON_FIRST_ERROR');

CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

CALL DBE_SCHEDULER.drop_program('program1', false);

CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

CALL DBE_SCHEDULER.enable_single('job1');

CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

CALL DBE_SCHEDULER.create_program('program1', 'stored_procedure', 'insert into tb_job_test(key) values(null);

CALL DBE_SCHEDULER.disable('job1');

CALL DBE_SCHEDULER.disable('program1', false, 'STOP_ON_FIRST_ERROR');

CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

CALL DBE_SCHEDULER.drop_program('program1', false);

CALL dbe_scheduler.create_job('job1','PLSQL_BLOCK','begin insert into test1 values(12);

CALL DBE_SCHEDULER.disable_single('job1', false);

CALL DBE_SCHEDULER.drop_job('job1', true, false, 'STOP_ON_FIRST_ERROR');

CALL DBE_SCHEDULER.eval_calendar_string('FREQ=DAILY;

CREATE OR REPLACE PROCEDURE pr1(calendar_str text) as DECLARE start_date timestamp with time zone;

CALL pr1('FREQ=hourly;

