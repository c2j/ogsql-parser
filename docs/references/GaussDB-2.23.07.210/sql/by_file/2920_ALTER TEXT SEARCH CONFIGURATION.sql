-- 来源: 2920_ALTER TEXT SEARCH CONFIGURATION.txt
-- SQL 数量: 7

CREATE TEXT SEARCH CONFIGURATION english_1 (parser=default);

--增加文本搜索配置字串类型映射语法。
ALTER TEXT SEARCH CONFIGURATION english_1 ADD MAPPING FOR word WITH simple,english_stem;

--增加文本搜索配置字串类型映射语法。
ALTER TEXT SEARCH CONFIGURATION english_1 ADD MAPPING FOR email WITH english_stem, french_stem;

--查询文本搜索配置相关信息。
SELECT b.cfgname,a.maptokentype,a.mapseqno,a.mapdict,c.dictname FROM pg_ts_config_map a,pg_ts_config b, pg_ts_dict c WHERE a.mapcfg=b.oid AND a.mapdict=c.oid AND b.cfgname='english_1' ORDER BY 1,2,3,4,5;

--修改文本搜索配置字串类型映射语法。
ALTER TEXT SEARCH CONFIGURATION english_1 ALTER MAPPING REPLACE french_stem with german_stem;

--查询文本搜索配置相关信息。
SELECT b.cfgname,a.maptokentype,a.mapseqno,a.mapdict,c.dictname FROM pg_ts_config_map a,pg_ts_config b, pg_ts_dict c WHERE a.mapcfg=b.oid AND a.mapdict=c.oid AND b.cfgname='english_1' ORDER BY 1,2,3,4,5;

--删除文本搜索配置。
DROP TEXT SEARCH CONFIGURATION english_1;

