-- 来源: 1450_file_1450.txt
-- SQL 数量: 13

CREATE TABLE mytab ( id INT , firstname VARCHAR ( 20 ), lastname VARCHAR ( 20 )) DISTRIBUTE BY hash ( id );

INSERT INTO mytab ( firstname , lastname ) VALUES ( 'Tom' , 'Jones' );

CREATE FUNCTION fun_exp () RETURNS INT AS $$ DECLARE x INT : = 0 ;

call fun_exp ();

select * from mytab ;

DROP FUNCTION fun_exp ();

DROP TABLE mytab ;

CREATE TABLE db ( a INT , b TEXT );

CREATE FUNCTION merge_db ( key INT , data TEXT ) RETURNS VOID AS $$ BEGIN LOOP --第一次尝试更新key UPDATE db SET b = data WHERE a = key ;

SELECT merge_db ( 1 , 'david' );

SELECT merge_db ( 1 , 'dennis' );

DROP FUNCTION merge_db ;

DROP TABLE db ;

