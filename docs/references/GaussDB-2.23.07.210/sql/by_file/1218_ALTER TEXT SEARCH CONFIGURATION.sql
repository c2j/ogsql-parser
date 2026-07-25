-- 来源: 1218_ALTER TEXT SEARCH CONFIGURATION.txt
-- SQL 数量: 7

CREATE TEXT SEARCH CONFIGURATION english_1 ( parser = default );

ALTER TEXT SEARCH CONFIGURATION english_1 ADD MAPPING FOR word WITH simple , english_stem ;

ALTER TEXT SEARCH CONFIGURATION english_1 ADD MAPPING FOR email WITH english_stem , french_stem ;

SELECT b . cfgname , a . maptokentype , a . mapseqno , a . mapdict , c . dictname FROM pg_ts_config_map a , pg_ts_config b , pg_ts_dict c WHERE a . mapcfg = b . oid AND a . mapdict = c . oid AND b . cfgname = 'english_1' ORDER BY 1 , 2 , 3 , 4 , 5 ;

ALTER TEXT SEARCH CONFIGURATION english_1 ALTER MAPPING REPLACE french_stem with german_stem ;

SELECT b . cfgname , a . maptokentype , a . mapseqno , a . mapdict , c . dictname FROM pg_ts_config_map a , pg_ts_config b , pg_ts_dict c WHERE a . mapcfg = b . oid AND a . mapdict = c . oid AND b . cfgname = 'english_1' ORDER BY 1 , 2 , 3 , 4 , 5 ;

DROP TEXT SEARCH CONFIGURATION english_1 ;

