-- 来源: 2770_file_2770.txt
-- SQL 数量: 11

SELECT octet_length ( E 'jo\\000se' :: bytea ) AS RESULT ;

SELECT overlay ( E 'Th\\000omas' :: bytea placing E '\\002\\003' :: bytea from 2 for 3 ) AS RESULT ;

SELECT position ( E '\\000om' :: bytea in E 'Th\\000omas' :: bytea ) AS RESULT ;

SELECT substring ( E 'Th\\000omas' :: bytea from 2 for 3 ) AS RESULT ;

select substr ( E 'Th\\000omas' :: bytea , 2 , 3 ) as result ;

SELECT trim ( E '\\000' :: bytea from E '\\000Tom\\000' :: bytea ) AS RESULT ;

SELECT btrim ( E '\\000trim\\000' :: bytea , E '\\000' :: bytea ) AS RESULT ;

SELECT get_bit ( E 'Th\\000omas' :: bytea , 45 ) AS RESULT ;

SELECT get_byte ( E 'Th\\000omas' :: bytea , 4 ) AS RESULT ;

SELECT set_bit ( E 'Th\\000omas' :: bytea , 45 , 0 ) AS RESULT ;

SELECT set_byte ( E 'Th\\000omas' :: bytea , 4 , 64 ) AS RESULT ;

