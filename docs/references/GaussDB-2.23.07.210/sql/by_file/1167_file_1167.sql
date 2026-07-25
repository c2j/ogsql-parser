-- 来源: 1167_file_1167.txt
-- SQL 数量: 3

SELECT to_tsvector ( 'english' , 'in the list of stop words' );

SELECT ts_rank_cd ( to_tsvector ( 'english' , 'in the list of stop words' ), to_tsquery ( 'list & stop' ));

SELECT ts_rank_cd ( to_tsvector ( 'english' , 'list stop words' ), to_tsquery ( 'list & stop' ));

