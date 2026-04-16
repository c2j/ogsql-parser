-- 来源: 1161_file_1161.txt
-- SQL 数量: 3

SELECT numnode ( plainto_tsquery ( 'the any' ));

SELECT numnode(' foo & bar ' :: tsquery );

SELECT querytree ( to_tsquery ( '!defined' ));

