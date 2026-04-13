-- 来源: 1075_file_1075.txt
-- SQL 数量: 15

SELECT 'abc' LIKE 'abc' AS RESULT ;

SELECT 'abc' LIKE 'a%' AS RESULT ;

SELECT 'abc' LIKE '_b_' AS RESULT ;

SELECT 'abc' LIKE 'c' AS RESULT ;

SELECT 'abc' SIMILAR TO 'abc' AS RESULT ;

SELECT 'abc' SIMILAR TO 'a' AS RESULT ;

SELECT 'abc' SIMILAR TO '%(b|d)%' AS RESULT ;

SELECT 'abc' SIMILAR TO '(b|c)%' AS RESULT ;

SELECT 'abc' ~ 'Abc' AS RESULT ;

SELECT 'abc' ~* 'Abc' AS RESULT ;

SELECT 'abc' !~ 'Abc' AS RESULT ;

SELECT 'abc' !~* 'Abc' AS RESULT ;

SELECT 'abc' ~ '^a' AS RESULT ;

SELECT 'abc' ~ '(b|d)' AS RESULT ;

SELECT 'abc' ~ '^(b|c)' AS RESULT ;

