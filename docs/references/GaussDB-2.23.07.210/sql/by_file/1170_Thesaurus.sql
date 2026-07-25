-- 来源: 1170_Thesaurus.txt
-- SQL 数量: 7

CREATE TEXT SEARCH DICTIONARY thesaurus_astro ( TEMPLATE = thesaurus , DictFile = thesaurus_astro , Dictionary = pg_catalog . english_stem , FILEPATH = 'file:///home/dicts/' );

ALTER TEXT SEARCH CONFIGURATION russian ALTER MAPPING FOR asciiword , asciihword , hword_asciipart WITH thesaurus_astro , english_stem ;

SELECT plainto_tsquery ( 'russian' , 'supernova star' );

SELECT to_tsvector ( 'russian' , 'supernova star' );

SELECT to_tsquery ( 'russian' , '''supernova star''' );

ALTER TEXT SEARCH DICTIONARY thesaurus_astro ( DictFile = thesaurus_astro , FILEPATH = 'file:///home/dicts/' );

SELECT plainto_tsquery ( 'russian' , 'supernova star' );

