-- 来源: 1156_file_1156.txt
-- SQL 数量: 5

SELECT to_tsquery ( 'english' , 'The & Fat & Rats' );

SELECT to_tsquery ( 'english' , 'Fat | Rats:AB' );

SELECT to_tsquery ( 'supern:*A & star:A*B' );

SELECT plainto_tsquery ( 'english' , 'The Fat Rats' );

SELECT plainto_tsquery ( 'english' , 'The Fat & Rats:C' );

