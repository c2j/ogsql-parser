-- 来源: 2849_file_2849.txt
-- SQL 数量: 19

DROP SCHEMA IF EXISTS tsearch CASCADE ;

CREATE SCHEMA tsearch ;

CREATE TABLE tsearch . pgweb ( id int , body text , title text , last_mod_date date ) with ( storage_type = ASTORE );

INSERT INTO tsearch . pgweb VALUES ( 1 , 'China, officially the People''s Republic of China (PRC), located in Asia, is the world''s most populous state.' , 'China' , '2010-1-1' );

INSERT INTO tsearch . pgweb VALUES ( 2 , 'America is a rock band, formed in England in 1970 by multi-instrumentalists Dewey Bunnell, Dan Peek, and Gerry Beckley.' , 'America' , '2010-1-1' );

INSERT INTO tsearch . pgweb VALUES ( 3 , 'England is a country that is part of the United Kingdom. It shares land borders with Scotland to the north and Wales to the west.' , 'England' , '2010-1-1' );

INSERT INTO tsearch . pgweb VALUES ( 4 , 'Australia, officially the Commonwealth of Australia, is a country comprising the mainland of the Australian continent, the island of Tasmania, and numerous smaller islands.' , 'Australia' , '2010-1-1' );

INSERT INTO tsearch . pgweb VALUES ( 6 , 'Japan is an island country in East Asia.' , 'Japan' , '2010-1-1' );

INSERT INTO tsearch . pgweb VALUES ( 7 , 'Germany, officially the Federal Republic of Germany, is a sovereign state and federal parliamentary republic in central-western Europe.' , 'Germany' , '2010-1-1' );

INSERT INTO tsearch . pgweb VALUES ( 8 , 'France, is a sovereign state comprising territory in western Europe and several overseas regions and territories.' , 'France' , '2010-1-1' );

INSERT INTO tsearch . pgweb VALUES ( 9 , 'Italy officially the Italian Republic, is a unitary parliamentary republic in Europe.' , 'Italy' , '2010-1-1' );

INSERT INTO tsearch . pgweb VALUES ( 10 , 'India, officially the Republic of India, is a country in South Asia.' , 'India' , '2010-1-1' );

INSERT INTO tsearch . pgweb VALUES ( 11 , 'Brazil, officially the Federative Republic of Brazil, is the largest country in both South America and Latin America.' , 'Brazil' , '2010-1-1' );

INSERT INTO tsearch . pgweb VALUES ( 12 , 'Canada is a country in the northern half of North America.' , 'Canada' , '2010-1-1' );

INSERT INTO tsearch . pgweb VALUES ( 13 , 'Mexico, officially the United Mexican States, is a federal republic in the southern part of North America.' , 'Mexico' , '2010-1-1' );

SELECT id , body , title FROM tsearch . pgweb WHERE to_tsvector ( 'english' , body ) @@ to_tsquery ( 'english' , 'america' );

SHOW default_text_search_config ;

SELECT id , body , title FROM tsearch . pgweb WHERE to_tsvector ( body ) @@ to_tsquery ( 'america' );

SELECT title FROM tsearch . pgweb WHERE to_tsvector ( title || ' ' || body ) @@ to_tsquery ( 'north & america' ) ORDER BY last_mod_date DESC LIMIT 10 ;

