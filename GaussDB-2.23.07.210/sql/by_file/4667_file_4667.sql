-- 来源: 4667_file_4667.txt
-- SQL 数量: 18

ALTER DATABASE SET ilm = on;

List of relations Schema | Name | Type | Owner | Storage

SELECT a.oid, a.relname from pg_class a inner join pg_namespace b on a.relnamespace = b.oid WHERE (a.relname = 'gsilmpolicy_seq' OR a.relname = 'gsilmtask_seq') AND b.nspname = 'public';

CREATE TABLE ilm_table_1 (col1 int, col2 text) ilm add policy row store compress advanced row after 3 days of no modification on (col1 < 1000);

CREATE TABLE ilm_table_2 (col1 int, col2 text);

ALTER TABLE ilm_table_2 ilm add policy row store compress advanced row after 3 days of no modification;

SELECT * FROM gs_my_ilmpolicies;

SELECT * FROM gs_my_ilmdatamovementpolicies;

SELECT * FROM gs_my_ilmobjects;

CALL DBE_ILM_ADMIN.CUSTOMIZE_ILM(11, 1);

INSERT INTO ilm_table_1 select *, 'test_data' FROM generate_series(1, 10000);

DECLARE v_taskid number;

BEGIN DBE_ILM.EXECUTE_ILM(OWNER => 'public', OBJECT_NAME => 'ilm_table_1', TASK_ID => v_taskid, SUBOBJECT_NAME => NULL, POLICY_NAME => 'ALL POLICIES', EXECUTION_MODE => 2);

SELECT * FROM gs_my_ilmtasks;

SELECT * FROM gs_my_ilmevaluationdetails;

SELECT * FROM gs_my_ilmresults;

DECLARE V_HOUR INT := 22;

SELECT * FROM gs_adm_ilmparameters;

