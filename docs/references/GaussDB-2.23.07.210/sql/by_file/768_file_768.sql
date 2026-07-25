-- 来源: 768_file_768.txt
-- SQL 数量: 12

CREATE TABLE test ( id int , time date );

CREATE OR REPLACE PROCEDURE PRC_JOB_1 () AS N_NUM integer : = 1 ;

call dbe_task . submit ( 'call public.prc_job_1();

call dbe_task . id_submit ( 1 , 'call public.prc_job_1();

select job , dbname , start_date , last_date , this_date , next_date , broken , status , interval , failures , what from my_jobs ;

call dbe_task . finish ( 1 , true );

call dbe_task . finish ( 1 , false );

call dbe_task . next_date ( 1 , sysdate + 1 . 0 / 24 );

call dbe_task . interval ( 1 , 'sysdate + 1.0/24' );

call dbe_task . content ( 1 , 'insert into public.test values(333, sysdate+5);

call dbe_task . update ( 1 , 'call public.prc_job_1();

call dbe_task . cancel ( 1 );

