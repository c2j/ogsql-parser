-- 来源: 2981_CREATE TEXT SEARCH CONFIGURATION.txt
-- SQL 数量: 12

CREATE TEXT SEARCH CONFIGURATION ngram2 (parser=ngram) WITH (gram_size = 2, grapsymbol_ignore = false);

--创建文本搜索配置。
CREATE TEXT SEARCH CONFIGURATION ngram3 (copy=ngram2) WITH (gram_size = 2, grapsymbol_ignore = false);

--添加类型映射。
ALTER TEXT SEARCH CONFIGURATION ngram2 ADD MAPPING FOR multisymbol WITH simple;

--创建用户joe。
CREATE USER joe IDENTIFIED BY ' ******** ';

--修改文本搜索配置的所有者。
ALTER TEXT SEARCH CONFIGURATION ngram2 OWNER TO joe;

--修改文本搜索配置的schema。
ALTER TEXT SEARCH CONFIGURATION ngram2 SET SCHEMA joe;

--重命名文本搜索配置。
ALTER TEXT SEARCH CONFIGURATION joe.ngram2 RENAME TO ngram_2;

--删除类型映射。
ALTER TEXT SEARCH CONFIGURATION joe.ngram_2 DROP MAPPING IF EXISTS FOR multisymbol;

--删除文本搜索配置。
DROP TEXT SEARCH CONFIGURATION joe.ngram_2;

DROP TEXT SEARCH CONFIGURATION ngram3;

--删除Schema及用户joe。
DROP SCHEMA IF EXISTS joe CASCADE;

DROP ROLE IF EXISTS joe;

