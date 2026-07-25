-- 来源: 1155_file_1155.txt
-- SQL 数量: 5

SELECT to_tsvector ( 'english' , 'a fat cat sat on a mat - it ate a fat rats' );

CREATE TABLE tsearch . tt ( id int , title text , keyword text , abstract text , body text , ti tsvector );

INSERT INTO tsearch . tt ( id , title , keyword , abstract , body ) VALUES ( 1 , 'China' , 'Beijing' , 'China' , 'China, officially the People''s Republic of China (PRC), located in Asia, is the world''s most populous state.' );

UPDATE tsearch . tt SET ti = setweight ( to_tsvector ( coalesce ( title , '' )), 'A' ) || setweight ( to_tsvector ( coalesce ( keyword , '' )), 'B' ) || setweight ( to_tsvector ( coalesce ( abstract , '' )), 'C' ) || setweight ( to_tsvector ( coalesce ( body , '' )), 'D' );

DROP TABLE tsearch . tt ;

