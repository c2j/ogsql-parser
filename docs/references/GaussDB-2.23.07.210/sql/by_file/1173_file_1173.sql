-- 来源: 1173_file_1173.txt
-- SQL 数量: 9

CREATE TEXT SEARCH CONFIGURATION ts_conf ( COPY = pg_catalog . english );

CREATE TEXT SEARCH DICTIONARY gs_dict ( TEMPLATE = synonym , SYNONYMS = gs_dict , FILEPATH = 'file:///home/dicts' );

CREATE TEXT SEARCH DICTIONARY english_ispell ( TEMPLATE = ispell , DictFile = english , AffFile = english , StopWords = english , FILEPATH = 'file:///home/dicts' );

ALTER TEXT SEARCH CONFIGURATION ts_conf ALTER MAPPING FOR asciiword , asciihword , hword_asciipart , word , hword , hword_part WITH gs_dict , english_ispell , english_stem ;

ALTER TEXT SEARCH CONFIGURATION ts_conf DROP MAPPING FOR email , url , url_path , sfloat , float ;

SELECT * FROM ts_debug ( 'ts_conf' , ' GaussDB, the highly scalable, SQL compliant, open source object-relational database management system, is now undergoing beta testing of the next version of our software. ' );

\ dF + ts_conf Text search configuration "public.ts_conf" Parser : "pg_catalog.default" Token | Dictionaries -----------------+------------------------------------- asciihword | gs_dict , english_ispell , english_stem asciiword | gs_dict , english_ispell , english_stem file | simple host | simple hword | gs_dict , english_ispell , english_stem hword_asciipart | gs_dict , english_ispell , english_stem hword_numpart | simple hword_part | gs_dict , english_ispell , english_stem int | simple numhword | simple numword | simple uint | simple version | simple word | gs_dict , english_ispell , english_stem

SET default_text_search_config = 'public.ts_conf' ;

SHOW default_text_search_config ;

