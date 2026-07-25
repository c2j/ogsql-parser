-- 来源: 1471_DBE_ILM_ADMIN.txt
-- SQL 数量: 4

DBE_ILM_ADMIN . DISABLE_ILM ();

DBE_ILM_ADMIN . ENABLE_ILM ();

CALL DBE_ILM_ADMIN . CUSTOMIZE_ILM ( 1 , 15 );

select * from gs_adm_ilmparameters ;

