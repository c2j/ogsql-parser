-- 来源: 2850_file_2850.txt
-- SQL 数量: 7

CREATE INDEX pgweb_idx_1 ON tsearch . pgweb USING gin ( to_tsvector ( 'english' , body ));

CREATE INDEX pgweb_idx_2 ON tsearch . pgweb USING gin ( to_tsvector ( 'ngram' , body ));

CREATE INDEX pgweb_idx_3 ON tsearch . pgweb USING gin ( to_tsvector ( 'english' , title || ' ' || body ));

ALTER TABLE tsearch . pgweb ADD COLUMN textsearchable_index_col tsvector ;

UPDATE tsearch . pgweb SET textsearchable_index_col = to_tsvector ( 'english' , coalesce ( title , '' ) || ' ' || coalesce ( body , '' ));

CREATE INDEX textsearch_idx_4 ON tsearch . pgweb USING gin ( textsearchable_index_col );

SELECT title FROM tsearch . pgweb WHERE textsearchable_index_col @@ to_tsquery ( 'north & america' ) ORDER BY last_mod_date DESC LIMIT 10 ;

