-- 来源: 1157_file_1157.txt
-- SQL 数量: 8

SELECT id , title , ts_rank_cd ( to_tsvector ( body ), query ) AS rank FROM tsearch . pgweb , to_tsquery ( 'america' ) query WHERE query @@ to_tsvector ( body ) ORDER BY rank DESC LIMIT 10 ;

SELECT id , title , ts_rank_cd ( to_tsvector ( body ), query , 32 /* rank/(rank+1) */ ) AS rank FROM tsearch . pgweb , to_tsquery ( 'america' ) query WHERE query @@ to_tsvector ( body ) ORDER BY rank DESC LIMIT 10 ;

CREATE TABLE tsearch . ts_ngram ( id int , body text );

INSERT INTO tsearch . ts_ngram VALUES ( 1 , '中文' );

INSERT INTO tsearch . ts_ngram VALUES ( 2 , '中文检索' );

INSERT INTO tsearch . ts_ngram VALUES ( 3 , '检索中文' );

SELECT id , body , ts_rank_cd ( to_tsvector ( 'ngram' , body ), query ) AS rank FROM tsearch . ts_ngram , to_tsquery ( '中文' ) query WHERE query @@ to_tsvector ( body );

SELECT id , body , ts_rank_cd ( to_tsvector ( 'ngram' , body ), query ) AS rank FROM tsearch . ts_ngram , to_tsquery ( '中文' ) query WHERE query @@ to_tsvector ( 'ngram' , body );

