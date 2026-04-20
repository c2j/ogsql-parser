CREATE OR REPLACE PACKAGE BODY BIGFUND.JOB_MAP_OBJECT IS
  PROCEDURE join_by_scheduler(SELF IN OUT BIGFUND.JOB_MAP_OBJECT, p_i_job_num  IN NUMBER(38),
                                    p_i_job_type VARCHAR2 DEFAULT 'split',
                                    p_i_timeout  IN NUMBER(38) DEFAULT 100) IS
 v_entry_time    DATE := SYSDATE;
 BEGIN
   LOOP
     DECLARE
       v_status VARCHAR2(30);
     BEGIN
       SELECT 1
         INTO v_status
         FROM user_scheduler_jobs t
        WHERE t.job_name = upper(p_i_job_type || '_' || p_i_job_num);
     EXCEPTION
       WHEN OTHERS THEN
         EXIT;
     END;

     IF v_entry_time + p_i_timeout / 24 / 60 / 60 <= SYSDATE OR SYSDATE < v_entry_time THEN
       RAISE EXCEPTION '(%): %',-200001, 'job timeout';
     END IF;
     PG_SLEEP(5);
   END LOOP;
 END;


 CREATE OR REPLACE PACKAGE BODY BIGFUND.JOB_MAP_OBJECT1 IS
   PROCEDURE join_by_scheduler1(SELF IN OUT BIGFUND.JOB_MAP_OBJECT, p_i_job_num  IN NUMBER(38),
                                     p_i_job_type VARCHAR2 DEFAULT 'split',
                                     p_i_timeout  IN NUMBER(38) DEFAULT 100) IS
  v_entry_time    DATE := SYSDATE;
  BEGIN
    LOOP
      DECLARE
        v_status VARCHAR2(30);
      BEGIN
        SELECT 1
          INTO v_status
          FROM user_scheduler_jobs t
         WHERE t.job_name = upper(p_i_job_type || '_' || p_i_job_num);
      EXCEPTION
        WHEN OTHERS THEN
          EXIT;
      --END;

      IF v_entry_time + p_i_timeout / 24 / 60 / 60 <= SYSDATE OR SYSDATE < v_entry_time THEN
        RAISE EXCEPTION '(%): %',-200001, 'job timeout';
      END IF;
      PG_SLEEP(5);
    END LOOP;
  END;
