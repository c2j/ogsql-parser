-- 来源: 1195_ALTER GLOBAL CONFIGURATION.txt
-- SQL 数量: 6

ALTER GLOBAL CONFIGURATION with ( redis_is_ok = true );

SELECT * FROM gs_global_config ;

ALTER GLOBAL CONFIGURATION with ( redis_is_ok = false );

SELECT * FROM gs_global_config ;

DROP GLOBAL CONFIGURATION redis_is_ok ;

SELECT * FROM gs_global_config ;

