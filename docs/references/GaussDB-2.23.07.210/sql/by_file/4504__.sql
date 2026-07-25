-- 来源: 4504__.txt
-- SQL 数量: 7

CREATE TABLE creditcard_info ( id_number int , name text , credit_card varchar ( 19 ) encrypted with ( column_encryption_key = cek1 , encryption_type = DETERMINISTIC ) ) with ( orientation = row );

insert into creditcard_info values ( 1 , 'Avi' , '1234567890123456' );

insert into creditcard_info values ( 2 , 'Eli' , '2345678901234567' );

CREATE FUNCTION f_encrypt_in_sql ( val1 text , val2 varchar ( 19 )) RETURNS text AS 'SELECT name from creditcard_info where name=$1 or credit_card=$2 LIMIT 1' LANGUAGE SQL ;

CREATE FUNCTION f_encrypt_in_plpgsql ( val1 text , val2 varchar ( 19 ), OUT c text ) AS $$ BEGIN SELECT into c name from creditcard_info where name = $ 1 or credit_card = $ 2 LIMIT 1 ;

SELECT f_encrypt_in_sql ( 'Avi' , '1234567890123456' );

SELECT f_encrypt_in_plpgsql ( 'Avi' , val2 => '1234567890123456' );

