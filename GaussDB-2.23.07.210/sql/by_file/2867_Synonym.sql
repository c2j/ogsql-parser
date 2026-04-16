-- 来源: 2867_Synonym.txt
-- SQL 数量: 16

SELECT * FROM ts_debug ( 'english' , 'Paris' );

CREATE TEXT SEARCH DICTIONARY my_synonym ( TEMPLATE = synonym , SYNONYMS = my_synonyms , FILEPATH = 'file:///home/dicts/' );

ALTER TEXT SEARCH CONFIGURATION english ALTER MAPPING FOR asciiword WITH my_synonym , english_stem ;

SELECT * FROM ts_debug ( 'english' , 'Paris' );

SELECT * FROM ts_debug ( 'english' , 'paris' );

ALTER TEXT SEARCH DICTIONARY my_synonym ( CASESENSITIVE = true );

SELECT * FROM ts_debug ( 'english' , 'Paris' );

SELECT * FROM ts_debug ( 'english' , 'paris' );

CREATE TEXT SEARCH DICTIONARY syn ( TEMPLATE = synonym , SYNONYMS = synonym_sample );

SELECT ts_lexize ( 'syn' , 'indices' );

CREATE TEXT SEARCH CONFIGURATION tst ( copy = simple );

ALTER TEXT SEARCH CONFIGURATION tst ALTER MAPPING FOR asciiword WITH syn ;

SELECT to_tsvector ( 'tst' , 'indices' );

SELECT to_tsquery ( 'tst' , 'indices' );

SELECT 'indexes are very useful' :: tsvector ;

SELECT 'indexes are very useful' :: tsvector @@ to_tsquery ( 'tst' , 'indices' );

