-- 来源: 2866_Simple.txt
-- SQL 数量: 6

CREATE TEXT SEARCH DICTIONARY public . simple_dict ( TEMPLATE = pg_catalog . simple , STOPWORDS = english );

SELECT ts_lexize ( 'public.simple_dict' , 'YeS' );

SELECT ts_lexize ( 'public.simple_dict' , 'The' );

ALTER TEXT SEARCH DICTIONARY public . simple_dict ( Accept = false );

SELECT ts_lexize ( 'public.simple_dict' , 'YeS' );

SELECT ts_lexize ( 'public.simple_dict' , 'The' );

