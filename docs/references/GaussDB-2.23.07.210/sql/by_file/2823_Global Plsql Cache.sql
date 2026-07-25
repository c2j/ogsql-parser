-- 来源: 2823_Global Plsql Cache.txt
-- SQL 数量: 4

select * from gs_glc_memory_detail where type='func' or type='pkg';

select invalidate_plsql_object('public','f3','function');

call pg_catalog.invalidate_plsql_object('public','pkg1','package');

select invalidate_plsql_object();

