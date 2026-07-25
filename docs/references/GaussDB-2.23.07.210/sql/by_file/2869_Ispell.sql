-- 来源: 2869_Ispell.txt
-- SQL 数量: 2

CREATE TEXT SEARCH DICTIONARY norwegian_ispell ( TEMPLATE = ispell , DictFile = nn_no , AffFile = nn_no , FilePath = 'file:///home/dicts' );

SELECT ts_lexize ( 'norwegian_ispell' , 'sjokoladefabrikk' );

