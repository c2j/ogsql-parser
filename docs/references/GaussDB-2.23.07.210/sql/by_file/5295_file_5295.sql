-- 来源: 5295_file_5295.txt
-- SQL 数量: 3

select gs_create_log_tables();

alter foreign table gs_pg_log_ft options (set master_only 'false');

alter foreign table gs_profile_log_ft options (set latest_files '10');

