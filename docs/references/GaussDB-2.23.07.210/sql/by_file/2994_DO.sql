-- 来源: 2994_DO.txt
-- SQL 数量: 3

CREATE USER webuser PASSWORD ' ******** ';

--授予用户webuser对模式tpcds下视图的所有操作权限。
DO $$DECLARE r record;

--删除用户webuser。
DROP USER webuser CASCADE;

